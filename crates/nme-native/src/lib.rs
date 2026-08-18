//! The NME-native AOT backend.
//!
//! This crate compiles a **restricted, statically typed core subset** of NME
//! to C, and the system C compiler turns that into native machine code. It is
//! deliberately separate from the Python compatibility backend ([`nme-core`]
//! transpiling to CPython) and from Nuitka: it is a real NME-to-native
//! compiler path, not a Python wrapper.
//!
//! The core subset in this version is small and documented:
//!
//! - `say`/`show`/`말해` of an integer expression, a finite float expression,
//!   a boolean expression, a string variable, or a string literal (one binary
//!   `+` concatenation);
//! - signed 32-bit integer literals and arithmetic with explicit overflow and
//!   zero-divisor checks;
//! - finite float literals and arithmetic as C `double` values; non-finite
//!   float literals are rejected before C emission and non-finite arithmetic
//!   results stop with a bilingual native-runtime error;
//! - escaped native string literals; embedded NUL characters are rejected
//!   because the C string runtime cannot preserve them;
//! - source comments are emitted as inert C comments, never as C directives;
//! - `set x to ...` / `x은 ...` / `x = ...` assignments of booleans, integers,
//!   finite floats, and string literals (including the Python-looking form);
//!   native string variables use checked 8192-byte buffers;
//! - value changes on an existing integer or float binding:
//!   `x add N` / `x = x + N` / `점수에 N 더해`;
//! - bindings first assigned in a possibly skipped control block must be
//!   initialized before the block or used after assignment within it;
//! - `while`/`if`/`else`/`else if` blocks over integer, finite-float, string,
//!   and boolean comparisons or truthiness, closed by `end`/`끝`, plus
//!   documented logical `and`/`or` conditions, one-line NME output and loop
//!   `break` bodies, and `times:`/`번:` loops;
//! - `break` inside a loop;
//! - functions over integer parameters with an unconditional integer `return`
//!   (recursion works); calls must name a function in the file and use its
//!   declared arity; headers use simple positional integer parameters only and
//!   definitions must be at file scope.
//!
//! Anything outside this core is rejected with a clear bilingual diagnostic;
//! it is never silently miscompiled. The rest of NME keeps running on CPython.

use std::collections::HashMap;

use nme_core::diagnostics::{Diagnostic, DiagnosticCode, Span};
use nme_core::syntax::{
    CompareOp, Condition, ConditionValue, InlineStmt, LogicalOp, NmeStmt, Value,
};
use nme_core::{lexer, parser};

use rustpython_parser::ast::{CmpOp, Constant, Expr, Operator, UnaryOp};
use rustpython_parser::Parse as _;

/// The static type the native backend tracks per variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VarType {
    Int,
    Float,
    Str,
    Bool,
    MaybeInt,
    MaybeFloat,
    MaybeStr,
    MaybeBool,
}

/// The static type of an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExprType {
    Int,
    Float,
    Str,
    Bool,
}

/// Tracks declaration insertion points for the main function and generated
/// native functions. Python bindings are function-scoped even when an
/// assignment appears inside an `if` or `while`, so declarations must not be
/// emitted inside a narrower C block.
#[derive(Debug)]
struct DeclarationSlots {
    offsets: Vec<usize>,
    active: usize,
}

impl DeclarationSlots {
    fn new(output: &str) -> Self {
        Self {
            offsets: vec![output.len()],
            active: 0,
        }
    }

    fn start_function(&mut self, output: &str) {
        self.offsets.push(output.len());
        self.active = self.offsets.len() - 1;
    }

    fn use_main(&mut self) {
        self.active = 0;
    }

    fn declare(&mut self, output: &mut String, declaration: &str) {
        let offset = self.offsets[self.active];
        output.insert_str(offset, declaration);
        let delta = declaration.len();
        for slot in &mut self.offsets {
            if *slot >= offset {
                *slot += delta;
            }
        }
    }
}

/// A native control block may assign a name on a path that does not execute.
/// Keep the pre-block types so those names can be rejected after the block
/// unless the block is statically known to run or every reachable branch
/// initializes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeBranchFlow {
    FallThrough,
    Terminated,
}

#[derive(Debug)]
struct NativeBlockFrame {
    body_depth: usize,
    bindings_before: HashMap<String, VarType>,
    is_loop: bool,
    definitely_runs: bool,
    branch_bindings: HashMap<String, VarType>,
    bindings_in_all_branches: Option<HashMap<String, VarType>>,
    has_else: bool,
    bindings_after_reachable_branch: Option<HashMap<String, VarType>>,
    reachable_branch_flow: Option<NativeBranchFlow>,
    branch_flow: NativeBranchFlow,
}

struct NativeControlContext<'a> {
    out: &'a mut String,
    open_braces: &'a mut usize,
    native_blocks: &'a mut Vec<NativeBlockFrame>,
    source: &'a str,
    span: Span,
    declared: &'a HashMap<String, VarType>,
    functions: &'a HashMap<String, usize>,
}

struct NativeInlineContext<'a> {
    source: &'a str,
    span: Span,
    declared: &'a HashMap<String, VarType>,
    functions: &'a HashMap<String, usize>,
    allow_break: bool,
}

fn concrete_type(kind: VarType) -> VarType {
    match kind {
        VarType::Int | VarType::MaybeInt => VarType::Int,
        VarType::Float | VarType::MaybeFloat => VarType::Float,
        VarType::Str | VarType::MaybeStr => VarType::Str,
        VarType::Bool | VarType::MaybeBool => VarType::Bool,
    }
}

fn maybe_type(kind: VarType) -> VarType {
    match concrete_type(kind) {
        VarType::Int => VarType::MaybeInt,
        VarType::Float => VarType::MaybeFloat,
        VarType::Str => VarType::MaybeStr,
        VarType::Bool => VarType::MaybeBool,
        VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr | VarType::MaybeBool => {
            unreachable!()
        }
    }
}

fn is_maybe_type(kind: VarType) -> bool {
    matches!(
        kind,
        VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr | VarType::MaybeBool
    )
}

fn condition_definitely_true(condition: &Condition) -> bool {
    matches!(
        condition,
        Condition::Truthy {
            value: ConditionValue::Literal(nme_core::syntax::Literal::True),
            negated: false,
        }
    )
}

fn finish_native_block(
    mut frame: NativeBlockFrame,
    declared: &mut HashMap<String, VarType>,
) -> NativeBranchFlow {
    if let Some(bindings) = frame.bindings_after_reachable_branch {
        *declared = bindings;
        return frame
            .reachable_branch_flow
            .unwrap_or(NativeBranchFlow::FallThrough);
    }
    if frame.definitely_runs {
        return frame.branch_flow;
    }
    record_branch_bindings(&mut frame, declared);
    if frame.branch_flow == NativeBranchFlow::FallThrough {
        record_completed_branch(&mut frame, declared);
    }
    let mut merged = frame.bindings_before;
    for (name, kind) in frame.branch_bindings {
        if !frame.is_loop && frame.has_else {
            if let Some(all_branch_kind) = frame
                .bindings_in_all_branches
                .as_ref()
                .and_then(|bindings| bindings.get(&name))
            {
                merged.insert(name, *all_branch_kind);
                continue;
            }
        }
        merged.entry(name).or_insert_with(|| maybe_type(kind));
    }
    *declared = merged;
    if frame.is_loop || !frame.has_else || frame.bindings_in_all_branches.is_some() {
        NativeBranchFlow::FallThrough
    } else {
        NativeBranchFlow::Terminated
    }
}

fn record_branch_bindings(frame: &mut NativeBlockFrame, declared: &HashMap<String, VarType>) {
    for (name, kind) in declared {
        frame.branch_bindings.entry(name.clone()).or_insert(*kind);
    }
}

fn record_completed_branch(frame: &mut NativeBlockFrame, declared: &HashMap<String, VarType>) {
    let completed = declared
        .iter()
        .filter(|(name, kind)| {
            let is_new = !frame.bindings_before.contains_key(*name);
            let was_maybe = frame
                .bindings_before
                .get(*name)
                .is_some_and(|before| is_maybe_type(*before));
            (is_new || was_maybe) && !is_maybe_type(**kind)
        })
        .map(|(name, kind)| (name.clone(), concrete_type(*kind)))
        .collect::<HashMap<_, _>>();
    match &mut frame.bindings_in_all_branches {
        Some(bindings) => bindings.retain(|name, kind| {
            completed
                .get(name)
                .is_some_and(|completed_kind| completed_kind == kind)
        }),
        None => frame.bindings_in_all_branches = Some(completed),
    }
}

fn reset_for_next_branch(frame: &NativeBlockFrame) -> HashMap<String, VarType> {
    let mut declared = frame.bindings_before.clone();
    for (name, kind) in &frame.branch_bindings {
        if !declared.contains_key(name) {
            declared.insert(name.clone(), maybe_type(*kind));
        }
    }
    declared
}

const PREAMBLE: &str = concat!(
    "#include <limits.h>\n",
    "#include <float.h>\n",
    "#include <stdio.h>\n",
    "#include <stdlib.h>\n",
    "#include <string.h>\n",
    "#if defined(_MSC_VER)\n",
    "#pragma warning(disable: 4505)\n",
    "#endif\n",
    "#if defined(__GNUC__) || defined(__clang__)\n",
    "#define NME_UNUSED __attribute__((unused))\n",
    "#else\n",
    "#define NME_UNUSED\n",
    "#endif\n",
    "#if INT_MAX != 2147483647 || INT_MIN != (-2147483647 - 1)\n",
    "#error \"NME native backend requires a 32-bit C int\"\n",
    "#endif\n",
    "#define NME_STRING_CAPACITY 8192\n",
    "static char nme_cat_buf[2][NME_STRING_CAPACITY];\n",
    "static int nme_cat_index;\n",
    "NME_UNUSED static void nme_integer_overflow(void) {\n",
    "    fputs(\"nme native: integer overflow / 정수 오버플로가 발생했습니다\\n\", stderr);\n",
    "    exit(1);\n",
    "}\n",
    "NME_UNUSED static void nme_integer_division_by_zero(void) {\n",
    "    fputs(\"nme native: integer modulo by zero / 정수 나머지의 제수가 0입니다\\n\", stderr);\n",
    "    exit(1);\n",
    "}\n",
    "NME_UNUSED static int nme_add_int(int left, int right) {\n",
    "    if ((right > 0 && left > INT_MAX - right)\n",
    "        || (right < 0 && left < INT_MIN - right)) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return left + right;\n",
    "}\n",
    "NME_UNUSED static int nme_sub_int(int left, int right) {\n",
    "    if ((right < 0 && left > INT_MAX + right)\n",
    "        || (right > 0 && left < INT_MIN + right)) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return left - right;\n",
    "}\n",
    "NME_UNUSED static int nme_mul_int(int left, int right) {\n",
    "    long long result = (long long)left * (long long)right;\n",
    "    if (result > INT_MAX || result < INT_MIN) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return (int)result;\n",
    "}\n",
    "NME_UNUSED static int nme_mod_int(int left, int right) {\n",
    "    if (right == 0) {\n",
    "        nme_integer_division_by_zero();\n",
    "    }\n",
    "    if (left == INT_MIN && right == -1) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return left % right;\n",
    "}\n",
    "NME_UNUSED static int nme_neg_int(int value) {\n",
    "    if (value == INT_MIN) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return -value;\n",
    "}\n",
    "NME_UNUSED static void nme_non_finite_float(void) {\n",
    "    fputs(\"nme native: non-finite float result / 유한하지 않은 실수 결과가 발생했습니다\\n\", stderr);\n",
    "    exit(1);\n",
    "}\n",
    "NME_UNUSED static double nme_float_result(double value) {\n",
    "    if (value != value || value > DBL_MAX || value < -DBL_MAX) {\n",
    "        nme_non_finite_float();\n",
    "    }\n",
    "    return value;\n",
    "}\n",
    "NME_UNUSED static double nme_add_float(double left, double right) {\n",
    "    return nme_float_result(left + right);\n",
    "}\n",
    "NME_UNUSED static double nme_sub_float(double left, double right) {\n",
    "    return nme_float_result(left - right);\n",
    "}\n",
    "NME_UNUSED static double nme_mul_float(double left, double right) {\n",
    "    return nme_float_result(left * right);\n",
    "}\n",
    "NME_UNUSED static int nme_len(const char *value) {\n",
    "    size_t length = strlen(value);\n",
    "    int count = 0;\n",
    "    for (size_t index = 0; index < length; index++) {\n",
    "        unsigned char byte = (unsigned char)value[index];\n",
    "        if ((byte & 0xc0) != 0x80) {\n",
    "            if (count == INT_MAX) {\n",
    "                nme_integer_overflow();\n",
    "            }\n",
    "            count++;\n",
    "        }\n",
    "    }\n",
    "    return count;\n",
    "}\n",
    "NME_UNUSED static void nme_string_overflow(void) {\n",
    "    fputs(\"nme native: string value exceeds 8191 bytes / 문자열 값이 8191바이트를 초과했습니다\\n\", stderr);\n",
    "    exit(1);\n",
    "}\n",
    "NME_UNUSED static void nme_copy(char *destination, size_t capacity, const char *source) {\n",
    "    size_t length = strlen(source);\n",
    "    if (length >= capacity) {\n",
    "        nme_string_overflow();\n",
    "    }\n",
    "    memmove(destination, source, length + 1);\n",
    "}\n",
    "NME_UNUSED static char *nme_cat(const char *a, const char *b) {\n",
    "    size_t a_length = strlen(a);\n",
    "    size_t b_length = strlen(b);\n",
    "    if (a_length >= NME_STRING_CAPACITY\n",
    "        || b_length >= NME_STRING_CAPACITY - a_length) {\n",
    "        nme_string_overflow();\n",
    "    }\n",
    "    char *destination = nme_cat_buf[nme_cat_index];\n",
    "    nme_cat_index = (nme_cat_index + 1) % 2;\n",
    "    memcpy(destination, a, a_length);\n",
    "    memcpy(destination + a_length, b, b_length + 1);\n",
    "    return destination;\n",
    "}\n",
);

fn native_function_header(tokens: &[lexer::Token]) -> Option<(String, Vec<String>)> {
    let name = match tokens.get(1).map(|token| &token.tok) {
        Some(rustpython_parser::Tok::Name { name }) => name.clone(),
        _ => return None,
    };
    if !matches!(
        tokens.get(2).map(|token| &token.tok),
        Some(rustpython_parser::Tok::Lpar)
    ) {
        return None;
    }
    let closing = tokens
        .iter()
        .position(|token| matches!(token.tok, rustpython_parser::Tok::Rpar))?;
    if closing + 2 != tokens.len()
        || !matches!(
            tokens.last().map(|token| &token.tok),
            Some(rustpython_parser::Tok::Colon)
        )
    {
        return None;
    }
    let parameter_tokens = &tokens[3..closing];
    let mut parameters = Vec::new();
    for (index, token) in parameter_tokens.iter().enumerate() {
        if index % 2 == 0 {
            let rustpython_parser::Tok::Name { name } = &token.tok else {
                return None;
            };
            parameters.push(name.clone());
        } else if !matches!(token.tok, rustpython_parser::Tok::Comma) {
            return None;
        }
    }
    Some((name, parameters))
}

fn native_function_signatures(
    lines: &[lexer::LogicalLine],
) -> (HashMap<String, usize>, Vec<Diagnostic>) {
    let mut functions = HashMap::new();
    let mut problems = Vec::new();
    for line in lines {
        if !matches!(
            line.tokens.first().map(|token| &token.tok),
            Some(rustpython_parser::Tok::Def)
        ) {
            continue;
        }
        let Some((name, parameters)) = native_function_header(&line.tokens) else {
            continue;
        };
        if functions.insert(name.clone(), parameters.len()).is_some() {
            problems.push(duplicate_native_function(&name, line.span));
        }
    }
    (functions, problems)
}

/// Emits prototypes before `main` so a native function may call another
/// function that appears later in the NME source. The frontend already limits
/// native functions to integer parameters and returns, so the C declarations
/// can use the same fixed signature for every accepted function.
fn native_function_prototypes(lines: &[lexer::LogicalLine]) -> String {
    let mut prototypes = String::new();
    for line in lines {
        if !matches!(
            line.tokens.first().map(|token| &token.tok),
            Some(rustpython_parser::Tok::Def)
        ) {
            continue;
        }
        let Some((name, parameters)) = native_function_header(&line.tokens) else {
            continue;
        };
        let parameter_types = if parameters.is_empty() {
            "void".to_string()
        } else {
            (0..parameters.len())
                .map(|_| "int")
                .collect::<Vec<_>>()
                .join(", ")
        };
        prototypes.push_str(&format!("int {name}({parameter_types});\n"));
    }
    prototypes
}

/// Compiles the native core subset of `source` to C source text.
///
/// On failure returns every problem found, ready to render with
/// [`nme_core::diagnostics::render_all`].
#[allow(clippy::too_many_lines)]
pub fn native_compile(source: &str) -> Result<String, Vec<Diagnostic>> {
    let lines = lexer::logical_lines(source).map_err(|problem| vec![problem])?;
    let program = parser::parse_program(source, &lines)?;
    let mut by_index = HashMap::new();
    for nme_line in &program.nme_lines {
        by_index.insert(nme_line.line_index, nme_line);
    }
    let (functions, signature_problems) = native_function_signatures(&lines);

    let mut out = String::from(PREAMBLE);
    out.push_str(&native_function_prototypes(&lines));
    out.push_str("int main(void) {\n");
    let mut open_braces = 1usize; // the `main` body
    let mut declaration_slots = DeclarationSlots::new(&out);
    let mut declared = HashMap::new();
    let mut native_blocks = Vec::<NativeBlockFrame>::new();
    let mut saved_main_scope = None;
    let mut in_function = false;
    let mut function_span = None;
    let mut function_returned = false;
    let mut problems = signature_problems;

    for (index, line) in lines.iter().enumerate() {
        let line_text = &source[line.span.start..line.span.end];
        let is_significant_line =
            !line_text.trim().is_empty() && !line_text.trim_start().starts_with('#');
        let nme_line = by_index.get(&index);
        let current_depth = nme_line
            .map_or(line.indent + program.virtual_indents[index], |nme_line| {
                line.indent + nme_line.virtual_indent
            });
        let is_branch = nme_line.is_some_and(|nme_line| {
            matches!(nme_line.stmt, NmeStmt::ElseIf { .. } | NmeStmt::Else { .. })
        });
        if !is_branch {
            while native_blocks
                .last()
                .is_some_and(|frame| current_depth < frame.body_depth)
            {
                let frame = native_blocks.pop().expect("native block frame exists");
                let block_flow = finish_native_block(frame, &mut declared);
                if block_flow == NativeBranchFlow::Terminated {
                    if let Some(parent) = native_blocks.last_mut() {
                        if !parent.is_loop {
                            parent.branch_flow = NativeBranchFlow::Terminated;
                        }
                    }
                }
            }
        }
        if in_function && line.indent == 0 && is_significant_line {
            if let Some(span) = function_span.take() {
                if !function_returned {
                    problems.push(native_function_requires_return(span));
                }
            }
            if let Some(main_scope) = saved_main_scope.take() {
                declared = main_scope;
            }
            declaration_slots.use_main();
            in_function = false;
            function_returned = false;
        }
        if let Some(nme_line) = nme_line {
            if is_branch {
                if let Some(frame) = native_blocks.last_mut() {
                    if frame.definitely_runs && frame.bindings_after_reachable_branch.is_none() {
                        frame.bindings_after_reachable_branch = Some(declared.clone());
                        frame.reachable_branch_flow = Some(frame.branch_flow);
                    }
                    record_branch_bindings(frame, &declared);
                    if frame.branch_flow == NativeBranchFlow::FallThrough {
                        record_completed_branch(frame, &declared);
                    }
                    frame.branch_flow = NativeBranchFlow::FallThrough;
                    declared = reset_for_next_branch(frame);
                }
            }
            // `else`/`else if` lines emit their own closing `}` before the
            // next branch, so the generic brace closing must not run first.
            if !is_branch {
                close_braces(&mut out, &mut open_braces, current_depth + 1);
            }
            match &nme_line.stmt {
                NmeStmt::Say { value } => {
                    if let Err(diag) = emit_say(&mut out, value, source, &declared, &functions) {
                        problems.push(diag);
                    }
                }
                NmeStmt::Set { target, value } => {
                    if let Err(diag) = emit_set(
                        &mut out,
                        &mut declaration_slots,
                        &mut declared,
                        target,
                        value,
                        source,
                        &functions,
                    ) {
                        problems.push(diag);
                    }
                }
                NmeStmt::Update {
                    target,
                    amount,
                    operation,
                } => {
                    if let Err(diag) = emit_update(
                        &mut out,
                        &mut declared,
                        target,
                        amount,
                        *operation,
                        source,
                        nme_line.span,
                        &functions,
                    ) {
                        problems.push(diag);
                    }
                }
                NmeStmt::While { condition, inline } => {
                    let mut control = NativeControlContext {
                        out: &mut out,
                        open_braces: &mut open_braces,
                        native_blocks: &mut native_blocks,
                        source,
                        span: nme_line.span,
                        declared: &declared,
                        functions: &functions,
                    };
                    if let Err(diag) = emit_native_condition_block(
                        &mut control,
                        "while",
                        condition,
                        inline.as_ref(),
                        current_depth + 1,
                        true,
                        false,
                    ) {
                        problems.push(diag);
                    }
                }
                NmeStmt::When { condition, inline } => {
                    let mut control = NativeControlContext {
                        out: &mut out,
                        open_braces: &mut open_braces,
                        native_blocks: &mut native_blocks,
                        source,
                        span: nme_line.span,
                        declared: &declared,
                        functions: &functions,
                    };
                    if let Err(diag) = emit_native_condition_block(
                        &mut control,
                        "if",
                        condition,
                        inline.as_ref(),
                        current_depth + 1,
                        false,
                        condition_definitely_true(condition),
                    ) {
                        problems.push(diag);
                    }
                }
                NmeStmt::Break => {
                    if native_blocks.iter().any(|frame| frame.is_loop) {
                        out.push_str("break;\n");
                        if let Some(frame) = native_blocks.last_mut() {
                            if !frame.is_loop {
                                frame.branch_flow = NativeBranchFlow::Terminated;
                            }
                        }
                    } else {
                        problems.push(native_break_outside_loop(nme_line.span));
                    }
                }
                NmeStmt::Times { count, inline } => {
                    let count_text = code_text(count, source);
                    match check_expr(count_text, nme_line.span, &declared, &functions) {
                        Ok((lowered, ExprType::Int)) => {
                            let header =
                                format!("for (int _nme_i = 0; _nme_i < {lowered}; _nme_i++)");
                            if let Some(inline) = inline {
                                match lower_inline_body(
                                    inline,
                                    source,
                                    &declared,
                                    &functions,
                                    nme_line.span,
                                    true,
                                ) {
                                    Ok(text) => out.push_str(&format!("{header} {text}\n")),
                                    Err(diag) => problems.push(diag),
                                }
                            } else {
                                out.push_str(&format!("{header} {{\n"));
                                open_braces += 1;
                                native_blocks.push(NativeBlockFrame {
                                    body_depth: current_depth + 1,
                                    bindings_before: declared.clone(),
                                    is_loop: true,
                                    definitely_runs: false,
                                    branch_bindings: HashMap::new(),
                                    bindings_in_all_branches: None,
                                    has_else: false,
                                    bindings_after_reachable_branch: None,
                                    reachable_branch_flow: None,
                                    branch_flow: NativeBranchFlow::FallThrough,
                                });
                            }
                        }
                        Err(diag) => problems.push(diag),
                        Ok(_) => problems.push(not_supported("this repeat count", nme_line.span)),
                    }
                }
                NmeStmt::ElseIf { condition, inline } => {
                    let inline_context = NativeInlineContext {
                        source,
                        span: nme_line.span,
                        declared: &declared,
                        functions: &functions,
                        allow_break: native_blocks.iter().any(|frame| frame.is_loop),
                    };
                    match lower_native_control(
                        "} else if",
                        Some(condition),
                        inline.as_ref(),
                        &inline_context,
                    ) {
                        Ok(text) => {
                            out.push_str(&text);
                            mark_inline_break(&mut native_blocks, inline.as_ref(), source);
                        }
                        Err(diag) => problems.push(diag),
                    }
                }
                NmeStmt::Else { inline } => {
                    let inline_context = NativeInlineContext {
                        source,
                        span: nme_line.span,
                        declared: &declared,
                        functions: &functions,
                        allow_break: native_blocks.iter().any(|frame| frame.is_loop),
                    };
                    match lower_native_control("} else", None, inline.as_ref(), &inline_context) {
                        Ok(text) => {
                            if let Some(frame) = native_blocks.last_mut() {
                                frame.has_else = true;
                            }
                            out.push_str(&text);
                            mark_inline_break(&mut native_blocks, inline.as_ref(), source);
                        }
                        Err(diag) => problems.push(diag),
                    }
                }
                NmeStmt::End => {}
                other => problems.push(unsupported_statement(other, nme_line.span)),
            }
        } else {
            // A non-NME line: blank, comment, or a Python assignment.
            let total_depth = line.indent + program.virtual_indents[index];
            close_braces(&mut out, &mut open_braces, total_depth + 1);
            let text = line_text;
            let trimmed = text.trim();
            if trimmed.is_empty() {
                out.push_str(text);
                out.push('\n');
                continue;
            }
            if trimmed.starts_with('#') {
                emit_native_comment(&mut out, text);
                continue;
            }
            let is_function_header = !in_function
                && line.indent == 0
                && matches!(
                    line.tokens.first().map(|token| &token.tok),
                    Some(rustpython_parser::Tok::Def)
                );
            let is_function_definition = matches!(
                line.tokens.first().map(|token| &token.tok),
                Some(rustpython_parser::Tok::Def)
            );
            if is_function_definition && !is_function_header {
                problems.push(native_nested_function(line.span));
                continue;
            }
            if is_function_header {
                saved_main_scope = Some(std::mem::take(&mut declared));
                in_function = true;
            }
            let output_before_line = out.len();
            let diagnostic = if !in_function
                && matches!(
                    line.tokens.first().map(|token| &token.tok),
                    Some(rustpython_parser::Tok::Return)
                ) {
                Some(native_return_outside_function(line.span))
            } else {
                emit_python_line(
                    &mut out,
                    &mut open_braces,
                    &mut declaration_slots,
                    &mut declared,
                    &line.tokens,
                    text,
                    source,
                    line.span,
                    &functions,
                )
            };
            if let Some(diag) = diagnostic {
                problems.push(diag);
            } else if is_function_header && out.len() > output_before_line {
                declaration_slots.start_function(&out);
                function_span = Some(line.span);
                function_returned = false;
            } else {
                if in_function
                    && matches!(
                        line.tokens.first().map(|token| &token.tok),
                        Some(rustpython_parser::Tok::Return)
                    )
                    && native_blocks
                        .last()
                        .is_some_and(|frame| !frame.is_loop && current_depth == frame.body_depth)
                {
                    if let Some(frame) = native_blocks.last_mut() {
                        frame.branch_flow = NativeBranchFlow::Terminated;
                    }
                }
                if in_function
                    && function_span.is_some()
                    && matches!(
                        line.tokens.first().map(|token| &token.tok),
                        Some(rustpython_parser::Tok::Return)
                    )
                    && open_braces == 2
                {
                    function_returned = true;
                }
            }
        }
    }
    if let Some(span) = function_span.take() {
        if !function_returned {
            problems.push(native_function_requires_return(span));
        }
    }
    close_braces(&mut out, &mut open_braces, 1);
    out.push_str("return 0;\n");
    close_braces(&mut out, &mut open_braces, 0);

    if problems.is_empty() {
        Ok(hoist_native_functions(&out))
    } else {
        Err(problems)
    }
}

/// Lowers a one-line inline statement (used by `3 times: say "hi"`) to C
/// text without a trailing newline.
fn lower_inline(
    stmt: &NmeStmt,
    source: &str,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
    span: Span,
    allow_break: bool,
) -> Result<String, Diagnostic> {
    match stmt {
        NmeStmt::Say { value } => {
            let mut out = String::new();
            emit_say(&mut out, value, source, declared, functions)?;
            Ok(out.trim_end().to_string())
        }
        NmeStmt::Break if allow_break => Ok("break;".to_string()),
        NmeStmt::Break => Err(native_break_outside_loop(span)),
        _ => Err(not_supported("this inline statement", span)),
    }
}

fn lower_inline_body(
    inline: &InlineStmt,
    source: &str,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
    span: Span,
    allow_break: bool,
) -> Result<String, Diagnostic> {
    match inline {
        InlineStmt::Nme(inner) => {
            lower_inline(inner, source, declared, functions, span, allow_break)
        }
        InlineStmt::Python(body_span) if is_bare_python_break(source, *body_span) => {
            if allow_break {
                Ok("break;".to_string())
            } else {
                Err(native_break_outside_loop(span))
            }
        }
        InlineStmt::Python(_) => Err(not_supported("this inline body", span)),
    }
}

fn is_bare_python_break(source: &str, span: Span) -> bool {
    source[span.start..span.end].trim() == "break"
}

fn inline_body_is_break(inline: Option<&InlineStmt>, source: &str) -> bool {
    match inline {
        Some(InlineStmt::Nme(inner)) => matches!(inner.as_ref(), NmeStmt::Break),
        Some(InlineStmt::Python(span)) => is_bare_python_break(source, *span),
        None => false,
    }
}

fn mark_inline_break(
    native_blocks: &mut [NativeBlockFrame],
    inline: Option<&InlineStmt>,
    source: &str,
) {
    if inline_body_is_break(inline, source) {
        if let Some(frame) = native_blocks.last_mut() {
            if !frame.is_loop {
                frame.branch_flow = NativeBranchFlow::Terminated;
            }
        }
    }
}

/// Lowers a native control header and, when present, its supported one-line
/// body. Inline output keeps the C block open so the next source line closes it
/// at normal virtual indentation and inline `elif`/`else` branches share the
/// same branch frame as their multi-line forms.
fn lower_native_control(
    prefix: &str,
    condition: Option<&Condition>,
    inline: Option<&InlineStmt>,
    context: &NativeInlineContext<'_>,
) -> Result<String, Diagnostic> {
    let header = if let Some(condition) = condition {
        format!(
            "{prefix} ({})",
            check_condition(
                condition,
                context.source,
                context.span,
                context.declared,
                context.functions,
            )?
        )
    } else {
        prefix.to_string()
    };
    let body = match inline {
        Some(inline) => Some(lower_inline_body(
            inline,
            context.source,
            context.declared,
            context.functions,
            context.span,
            context.allow_break,
        )?),
        None => None,
    };
    Ok(format!(
        "{header} {{\n{}",
        body.map(|body| format!("{body}\n")).unwrap_or_default()
    ))
}

fn emit_native_condition_block(
    context: &mut NativeControlContext<'_>,
    prefix: &str,
    condition: &Condition,
    inline: Option<&InlineStmt>,
    body_depth: usize,
    is_loop: bool,
    definitely_runs: bool,
) -> Result<(), Diagnostic> {
    let inline_context = NativeInlineContext {
        source: context.source,
        span: context.span,
        declared: context.declared,
        functions: context.functions,
        allow_break: is_loop || context.native_blocks.iter().any(|frame| frame.is_loop),
    };
    let text = lower_native_control(prefix, Some(condition), inline, &inline_context)?;
    context.out.push_str(&text);
    *context.open_braces += 1;
    context.native_blocks.push(NativeBlockFrame {
        body_depth,
        bindings_before: context.declared.clone(),
        is_loop,
        definitely_runs,
        branch_bindings: HashMap::new(),
        bindings_in_all_branches: None,
        has_else: false,
        bindings_after_reachable_branch: None,
        reachable_branch_flow: None,
        branch_flow: if !is_loop
            && matches!(
                inline,
                Some(inline) if inline_body_is_break(Some(inline), context.source)
            ) {
            NativeBranchFlow::Terminated
        } else {
            NativeBranchFlow::FallThrough
        },
    });
    Ok(())
}

fn close_braces(out: &mut String, open: &mut usize, target: usize) {
    while *open > target {
        out.push_str("}\n");
        *open -= 1;
    }
}

fn emit_native_comment(out: &mut String, source_comment: &str) {
    out.push_str("/*");
    out.push_str(&source_comment.replace("*/", "* /"));
    out.push_str(" */\n");
}

/// Moves top-level generated C function definitions before `main`.
///
/// GCC accepts nested functions as a non-standard extension, while Clang
/// correctly rejects them. NME functions are Python top-level definitions,
/// so the portable C representation is a file-scope C function followed by
/// the generated `main` body.
fn hoist_native_functions(source: &str) -> String {
    const MAIN_HEADER: &str = "int main(void) {\n";
    let Some(main_at) = source.find(MAIN_HEADER) else {
        return source.to_string();
    };
    let prefix = &source[..main_at];
    let body = &source[main_at + MAIN_HEADER.len()..];
    let lines = body.split_inclusive('\n').collect::<Vec<_>>();
    let mut functions = String::new();
    let mut main_body = String::new();
    let mut index = 0usize;
    let mut depth = 0isize;

    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim_end_matches(['\r', '\n']);
        let is_function = depth == 0
            && trimmed.starts_with("int ")
            && trimmed.ends_with(") {")
            && trimmed.contains('(');
        if is_function {
            let mut balance = c_brace_delta(line);
            functions.push_str(line);
            index += 1;
            while balance > 0 && index < lines.len() {
                let nested = lines[index];
                balance += c_brace_delta(nested);
                functions.push_str(nested);
                index += 1;
            }
            continue;
        }

        depth = (depth + c_brace_delta(line)).max(0);
        main_body.push_str(line);
        index += 1;
    }

    format!("{prefix}{functions}{MAIN_HEADER}{main_body}")
}

fn c_brace_delta(line: &str) -> isize {
    let mut delta = 0isize;
    let mut in_string = false;
    let mut in_comment = false;
    let mut escaped = false;
    let mut characters = line.chars().peekable();
    while let Some(character) = characters.next() {
        if in_comment {
            if character == '*' && characters.peek().is_some_and(|next| *next == '/') {
                characters.next();
                in_comment = false;
            }
            continue;
        }
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
        } else if character == '"' {
            in_string = true;
        } else if character == '/' && characters.peek().is_some_and(|next| *next == '*') {
            characters.next();
            in_comment = true;
        } else if character == '{' {
            delta += 1;
        } else if character == '}' {
            delta -= 1;
        }
    }
    delta
}

/// Validates and lowers a Python expression to C for the native core,
/// returning the C text and its static type. Integer expressions use checked
/// native runtime helpers; finite-float expressions use checked `double`
/// helpers; string expressions lower to a name, a literal, or one `+`
/// concatenation through the small runtime helper.
fn check_expr(
    text: &str,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<(String, ExprType), Diagnostic> {
    let expr = Expr::parse(text, "<native>").map_err(|_| not_supported("this expression", span))?;
    lower_expr(&expr, span, declared, functions)
}

fn native_integer_literal(
    value: &impl std::fmt::Display,
    negative: bool,
    span: Span,
) -> Result<String, Diagnostic> {
    let magnitude_text = value.to_string();
    let magnitude = magnitude_text
        .parse::<u64>()
        .map_err(|_| integer_literal_out_of_range(&magnitude_text, negative, span))?;
    let maximum = if negative {
        2_147_483_648
    } else {
        2_147_483_647
    };
    if magnitude > maximum {
        return Err(integer_literal_out_of_range(
            &magnitude_text,
            negative,
            span,
        ));
    }
    if negative {
        if magnitude == 2_147_483_648 {
            Ok("(-2147483647 - 1)".to_string())
        } else {
            Ok(format!("-{magnitude}"))
        }
    } else {
        Ok(magnitude.to_string())
    }
}

fn native_float_literal(value: f64, span: Span) -> Result<String, Diagnostic> {
    if !value.is_finite() {
        return Err(native_non_finite_float(span));
    }
    let mut text = value.to_string();
    if !text.contains('.') && !text.contains('e') && !text.contains('E') {
        text.push_str(".0");
    }
    Ok(text)
}

fn c_string_literal(value: &str, span: Span) -> Result<String, Diagnostic> {
    let mut escaped = String::new();
    for character in value.chars() {
        match character {
            '\0' => return Err(native_string_nul(span)),
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '?' => escaped.push_str("\\?"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\x08' => escaped.push_str("\\b"),
            '\x0c' => escaped.push_str("\\f"),
            '\x0b' => escaped.push_str("\\v"),
            '\x07' => escaped.push_str("\\a"),
            character if character.is_ascii_control() => {
                escaped.push_str(&format!("\\x{:02x}\"\"", character as u8));
            }
            character => escaped.push(character),
        }
    }
    Ok(format!("\"{escaped}\""))
}

fn lower_expr(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<(String, ExprType), Diagnostic> {
    match expr {
        Expr::Constant(constant) => match &constant.value {
            Constant::Int(value) => {
                Ok((native_integer_literal(value, false, span)?, ExprType::Int))
            }
            Constant::Float(value) => Ok((native_float_literal(*value, span)?, ExprType::Float)),
            Constant::Bool(value) => {
                Ok((if *value { "1" } else { "0" }.to_string(), ExprType::Bool))
            }
            Constant::Str(string) => Ok((c_string_literal(string, span)?, ExprType::Str)),
            _ => Err(not_supported("this constant", span)),
        },
        Expr::Name(name) => {
            let id = name.id.as_str();
            if is_native_reserved_name(id) {
                return Err(reserved_name("a variable named", "변수 이름", id, span));
            }
            if functions.contains_key(id) {
                return Err(native_function_value(id, span));
            }
            match declared.get(id).copied() {
                Some(
                    VarType::MaybeInt
                    | VarType::MaybeFloat
                    | VarType::MaybeStr
                    | VarType::MaybeBool,
                ) => Err(uninitialized_name(id, span)),
                Some(VarType::Str) => Ok((id.to_string(), ExprType::Str)),
                Some(VarType::Float) => Ok((id.to_string(), ExprType::Float)),
                Some(VarType::Int) => Ok((id.to_string(), ExprType::Int)),
                Some(VarType::Bool) => Ok((id.to_string(), ExprType::Bool)),
                None => Err(unknown_native_name(id, span)),
            }
        }
        Expr::BinOp(binop) => match binop.op {
            Operator::Add if binop_operands_are_string(binop, span, declared, functions)? => {
                // One binary `+` over string operands; the operands must be a
                // name or a literal so the shared runtime buffer is safe.
                let left = string_operand(&binop.left, span, declared, functions)?;
                let right = string_operand(&binop.right, span, declared, functions)?;
                Ok((format!("nme_cat({left}, {right})"), ExprType::Str))
            }
            Operator::Add | Operator::Sub | Operator::Mult | Operator::Mod => {
                let (left, left_kind) = numeric_operand(&binop.left, span, declared, functions)?;
                let (right, right_kind) = numeric_operand(&binop.right, span, declared, functions)?;
                if matches!(binop.op, Operator::Mod)
                    && (matches!(left_kind, ExprType::Float)
                        || matches!(right_kind, ExprType::Float))
                {
                    return Err(not_supported("modulo on floats", span));
                }
                let kind = if matches!(left_kind, ExprType::Float)
                    || matches!(right_kind, ExprType::Float)
                {
                    ExprType::Float
                } else {
                    ExprType::Int
                };
                let lowered = if kind == ExprType::Int {
                    format!("{}({left}, {right})", integer_operator_text(&binop.op))
                } else {
                    format!("{}({left}, {right})", float_operator_text(&binop.op))
                };
                Ok((lowered, kind))
            }
            _ => Err(not_supported("this operator", span)),
        },
        Expr::Compare(_) => Ok((
            lower_compare(expr, span, declared, functions)?,
            ExprType::Bool,
        )),
        Expr::UnaryOp(unary) => {
            if matches!(unary.op, UnaryOp::USub | UnaryOp::UAdd) {
                if let Expr::Constant(constant) = unary.operand.as_ref() {
                    if let Constant::Int(value) = &constant.value {
                        let negative = matches!(unary.op, UnaryOp::USub);
                        return Ok((
                            native_integer_literal(value, negative, span)?,
                            ExprType::Int,
                        ));
                    }
                }
                let (operand, kind) = numeric_operand(&unary.operand, span, declared, functions)?;
                Ok((
                    if kind == ExprType::Int && matches!(unary.op, UnaryOp::USub) {
                        format!("nme_neg_int({operand})")
                    } else {
                        format!(
                            "({}{operand})",
                            if matches!(unary.op, UnaryOp::USub) {
                                "-"
                            } else {
                                ""
                            }
                        )
                    },
                    kind,
                ))
            } else {
                Err(not_supported("this operator", span))
            }
        }
        Expr::Call(call) => {
            let Expr::Name(callee) = call.func.as_ref() else {
                return Err(not_supported("this call", span));
            };
            if !call.keywords.is_empty() {
                return Err(native_keyword_arguments(span));
            }
            if callee.id.as_str() == "len" && call.args.len() == 1 {
                let (argument, kind) = lower_expr(&call.args[0], span, declared, functions)?;
                if kind == ExprType::Str {
                    return Ok((format!("nme_len({argument})"), ExprType::Int));
                }
            }
            if is_native_reserved_name(callee.id.as_str()) {
                return Err(reserved_name("a call to", "호출", callee.id.as_str(), span));
            }
            if declared.contains_key(callee.id.as_str()) {
                return Err(native_function_name_collision(callee.id.as_str(), span));
            }
            let Some(expected_arguments) = functions.get(callee.id.as_str()).copied() else {
                return Err(unknown_native_function(callee.id.as_str(), span));
            };
            if call.args.len() != expected_arguments {
                return Err(native_function_arity(
                    callee.id.as_str(),
                    expected_arguments,
                    call.args.len(),
                    span,
                ));
            }
            let mut args = Vec::new();
            for argument in &call.args {
                let (text, kind) = lower_expr(argument, span, declared, functions)?;
                if kind != ExprType::Int {
                    return Err(native_function_requires_integer(span));
                }
                args.push(text);
            }
            Ok((format!("{}({})", callee.id, args.join(", ")), ExprType::Int))
        }
        _ => Err(not_supported("this expression", span)),
    }
}

fn binop_operands_are_string(
    binop: &rustpython_parser::ast::ExprBinOp,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<bool, Diagnostic> {
    let left_kind = operand_kind(&binop.left, span, declared, functions)?;
    let right_kind = operand_kind(&binop.right, span, declared, functions)?;
    match (left_kind, right_kind) {
        (ExprType::Str, ExprType::Str) => Ok(true),
        (ExprType::Int | ExprType::Float, ExprType::Int | ExprType::Float) => Ok(false),
        (ExprType::Bool, _) | (_, ExprType::Bool) => {
            Err(not_supported("a boolean in arithmetic", span))
        }
        _ => Err(not_supported("mixing numbers and text", span)),
    }
}

fn operand_kind(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<ExprType, Diagnostic> {
    let (_, kind) = lower_expr(expr, span, declared, functions)?;
    Ok(kind)
}

fn string_operand(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
    _functions: &HashMap<String, usize>,
) -> Result<String, Diagnostic> {
    match expr {
        Expr::Constant(constant) => match &constant.value {
            Constant::Str(string) => c_string_literal(string, span),
            _ => Err(not_supported("this operand", span)),
        },
        Expr::Name(name) => {
            let id = name.id.as_str();
            if is_native_reserved_name(id) {
                return Err(reserved_name("a variable named", "변수 이름", id, span));
            }
            match declared.get(id).copied() {
                Some(VarType::MaybeStr) => Err(uninitialized_name(id, span)),
                Some(VarType::Str) => Ok(id.to_string()),
                _ => Err(not_supported("this operand", span)),
            }
        }
        _ => Err(not_supported("nested string concatenation", span)),
    }
}

fn numeric_operand(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<(String, ExprType), Diagnostic> {
    let (text, kind) = lower_expr(expr, span, declared, functions)?;
    match kind {
        ExprType::Str => return Err(not_supported("a text value in a numeric expression", span)),
        ExprType::Bool => return Err(not_supported("a boolean in a numeric expression", span)),
        ExprType::Int | ExprType::Float => {}
    }
    Ok((text, kind))
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn float_operator_text(operator: &Operator) -> &'static str {
    match operator {
        Operator::Add => "nme_add_float",
        Operator::Sub => "nme_sub_float",
        Operator::Mult => "nme_mul_float",
        Operator::Mod => unreachable!("float modulo is checked by the caller"),
        _ => unreachable!("checked by the caller"),
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn integer_operator_text(operator: &Operator) -> &'static str {
    match operator {
        Operator::Add => "nme_add_int",
        Operator::Sub => "nme_sub_int",
        Operator::Mult => "nme_mul_int",
        Operator::Mod => "nme_mod_int",
        _ => unreachable!("checked by the caller"),
    }
}

/// Validates a condition: a comparison of two integer expressions, optionally
/// negated.
fn check_condition(
    condition: &Condition,
    source: &str,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<String, Diagnostic> {
    match condition {
        Condition::Compare {
            left,
            operator,
            right,
            negated,
        } => {
            let (left, left_kind) = condition_operand(left, source, span, declared, functions)?;
            let (right, right_kind) = condition_operand(right, source, span, declared, functions)?;
            let comparison = match (left_kind, right_kind, operator) {
                (
                    ExprType::Int | ExprType::Float,
                    ExprType::Int | ExprType::Float,
                    CompareOp::Equal,
                ) => format!("{left} == {right}"),
                (
                    ExprType::Int | ExprType::Float,
                    ExprType::Int | ExprType::Float,
                    CompareOp::Greater,
                ) => format!("{left} > {right}"),
                (
                    ExprType::Int | ExprType::Float,
                    ExprType::Int | ExprType::Float,
                    CompareOp::Less,
                ) => format!("{left} < {right}"),
                (
                    ExprType::Int | ExprType::Float,
                    ExprType::Int | ExprType::Float,
                    CompareOp::LessOrEqual,
                ) => format!("{left} <= {right}"),
                (
                    ExprType::Int | ExprType::Float,
                    ExprType::Int | ExprType::Float,
                    CompareOp::GreaterOrEqual,
                ) => format!("{left} >= {right}"),
                (ExprType::Bool, ExprType::Bool, CompareOp::Equal) => {
                    format!("{left} == {right}")
                }
                (ExprType::Str, ExprType::Str, CompareOp::Equal) => {
                    format!("strcmp({left}, {right}) == 0")
                }
                _ => {
                    return Err(not_supported("this comparison", span));
                }
            };
            Ok(if *negated {
                format!("!({comparison})")
            } else {
                comparison
            })
        }
        Condition::Python(code) => {
            let text = code_text(code, source);
            let expr =
                Expr::parse(text, "<native>").map_err(|_| not_supported("this condition", span))?;
            lower_compare(&expr, span, declared, functions)
        }
        Condition::Truthy { value, negated } => {
            let (text, kind) = match value {
                ConditionValue::Name(name) => {
                    if is_native_reserved_name(name) {
                        return Err(reserved_name("a variable named", "변수 이름", name, span));
                    }
                    if functions.contains_key(name) {
                        return Err(native_function_value(name, span));
                    }
                    (
                        name.clone(),
                        match declared.get(name).copied() {
                            Some(
                                VarType::MaybeInt
                                | VarType::MaybeFloat
                                | VarType::MaybeStr
                                | VarType::MaybeBool,
                            ) => {
                                return Err(uninitialized_name(name, span));
                            }
                            Some(VarType::Str) => ExprType::Str,
                            Some(VarType::Float) => ExprType::Float,
                            Some(VarType::Int) => ExprType::Int,
                            Some(VarType::Bool) => ExprType::Bool,
                            None => return Err(unknown_native_name(name, span)),
                        },
                    )
                }
                ConditionValue::Python(code) => {
                    check_expr(code_text(code, source), span, declared, functions)?
                }
                ConditionValue::Literal(literal) => match literal {
                    nme_core::syntax::Literal::True => ("(1)".to_string(), ExprType::Bool),
                    nme_core::syntax::Literal::False => ("(0)".to_string(), ExprType::Bool),
                    nme_core::syntax::Literal::None => {
                        return Err(not_supported("null truthiness", span));
                    }
                },
                ConditionValue::Text(_) => {
                    return Err(not_supported("text in a truthy condition", span));
                }
            };
            if kind == ExprType::Str {
                return Err(not_supported("a text value in a truthy condition", span));
            }
            Ok(if *negated {
                format!("!({text})")
            } else {
                format!("({text})")
            })
        }
        Condition::Logical {
            left,
            operator,
            right,
        } => {
            let left = check_condition(left, source, span, declared, functions)?;
            let right = check_condition(right, source, span, declared, functions)?;
            let operator = match operator {
                LogicalOp::And => "&&",
                LogicalOp::Or => "||",
            };
            Ok(format!("({left} {operator} {right})"))
        }
    }
}

fn condition_operand(
    value: &ConditionValue,
    source: &str,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<(String, ExprType), Diagnostic> {
    match value {
        ConditionValue::Python(code) => {
            check_expr(code_text(code, source), span, declared, functions)
        }
        ConditionValue::Name(name) => {
            if is_native_reserved_name(name) {
                return Err(reserved_name("a variable named", "변수 이름", name, span));
            }
            if functions.contains_key(name) {
                return Err(native_function_value(name, span));
            }
            let kind = match declared.get(name).copied() {
                Some(
                    VarType::MaybeInt
                    | VarType::MaybeFloat
                    | VarType::MaybeStr
                    | VarType::MaybeBool,
                ) => {
                    return Err(uninitialized_name(name, span));
                }
                Some(VarType::Str) => ExprType::Str,
                Some(VarType::Float) => ExprType::Float,
                Some(VarType::Int) => ExprType::Int,
                Some(VarType::Bool) => ExprType::Bool,
                None => return Err(unknown_native_name(name, span)),
            };
            Ok((name.clone(), kind))
        }
        ConditionValue::Text(text) => Ok((c_string_literal(text, span)?, ExprType::Str)),
        ConditionValue::Literal(literal) => match literal {
            nme_core::syntax::Literal::True => Ok(("1".to_string(), ExprType::Bool)),
            nme_core::syntax::Literal::False => Ok(("0".to_string(), ExprType::Bool)),
            nme_core::syntax::Literal::None => Err(not_supported("null in a condition", span)),
        },
    }
}

/// Lowers one `a <op> b` comparison to C. Integer comparisons pass through;
/// string `==`/`!=` comparisons lower through `strcmp`. Anything else is
/// rejected.
fn lower_compare(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<String, Diagnostic> {
    let Expr::Compare(compare) = expr else {
        return Err(not_supported("this condition", span));
    };
    if compare.ops.len() != 1 || compare.comparators.len() != 1 {
        return Err(not_supported("this condition", span));
    }
    let (left, left_kind) = lower_expr(&compare.left, span, declared, functions)?;
    let (right, right_kind) = lower_expr(&compare.comparators[0], span, declared, functions)?;
    match (left_kind, right_kind) {
        (ExprType::Int | ExprType::Float, ExprType::Int | ExprType::Float) => {
            let op = match compare.ops[0] {
                CmpOp::Lt => "<",
                CmpOp::LtE => "<=",
                CmpOp::Gt => ">",
                CmpOp::GtE => ">=",
                CmpOp::Eq => "==",
                CmpOp::NotEq => "!=",
                _ => return Err(not_supported("this comparison", span)),
            };
            Ok(format!("({left} {op} {right})"))
        }
        (ExprType::Str, ExprType::Str) => match compare.ops[0] {
            CmpOp::Eq => Ok(format!("(strcmp({left}, {right}) == 0)")),
            CmpOp::NotEq => Ok(format!("(strcmp({left}, {right}) != 0)")),
            _ => Err(not_supported("ordering text in a condition", span)),
        },
        (ExprType::Bool, ExprType::Bool) => match compare.ops[0] {
            CmpOp::Eq => Ok(format!("({left} == {right})")),
            CmpOp::NotEq => Ok(format!("({left} != {right})")),
            _ => Err(not_supported(
                "ordering boolean values in a condition",
                span,
            )),
        },
        _ => Err(not_supported("incompatible values in a condition", span)),
    }
}

fn code_text<'a>(code: &nme_core::syntax::Code, source: &'a str) -> &'a str {
    match code {
        nme_core::syntax::Code::Source(span) => &source[span.start..span.end],
    }
}

fn emit_say(
    out: &mut String,
    value: &Value,
    source: &str,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<(), Diagnostic> {
    match value {
        Value::Python(code) => {
            let text = code_text(code, source);
            let span = code_span(code);
            let (lowered, kind) = check_expr(text, span, declared, functions)?;
            match kind {
                ExprType::Int => {
                    out.push_str(&format!("printf(\"%d\\n\", {lowered});\n"));
                    Ok(())
                }
                ExprType::Float => {
                    out.push_str(&format!("printf(\"%g\\n\", {lowered});\n"));
                    Ok(())
                }
                ExprType::Bool => {
                    out.push_str(&format!(
                        "printf(\"%s\\n\", {lowered} ? \"True\" : \"False\");\n"
                    ));
                    Ok(())
                }
                ExprType::Str => {
                    out.push_str(&format!("printf(\"%s\\n\", {lowered});\n"));
                    Ok(())
                }
            }
        }
        Value::Text(template) => {
            let mut literal = String::new();
            for part in &template.parts {
                match part {
                    nme_core::syntax::TextPart::Literal(text) => literal.push_str(text),
                    nme_core::syntax::TextPart::Variable(_) => {
                        return Err(not_supported(
                            "a variable inside a sentence",
                            span_of_value(value),
                        ));
                    }
                }
            }
            let escaped = c_string_literal(&literal, span_of_value(value))?;
            out.push_str(&format!("printf(\"%s\\n\", {escaped});\n"));
            Ok(())
        }
        Value::Literal(literal) => match literal {
            nme_core::syntax::Literal::True => {
                out.push_str("puts(\"True\");\n");
                Ok(())
            }
            nme_core::syntax::Literal::False => {
                out.push_str("puts(\"False\");\n");
                Ok(())
            }
            nme_core::syntax::Literal::None => {
                Err(not_supported("null output", span_of_value(value)))
            }
        },
        Value::RandomInteger { .. } | Value::RandomChoice { .. } => {
            Err(not_supported("random values", span_of_value(value)))
        }
        Value::List(_) => Err(not_supported("list values", span_of_value(value))),
        Value::ZeroKnowledge(_) => {
            Err(not_supported("zero-knowledge values", span_of_value(value)))
        }
    }
}

fn assignment_type(
    declared: &HashMap<String, VarType>,
    name: &str,
    expression_type: ExprType,
    span: Span,
) -> Result<bool, Diagnostic> {
    let incoming = match expression_type {
        ExprType::Int => VarType::Int,
        ExprType::Float => VarType::Float,
        ExprType::Str => VarType::Str,
        ExprType::Bool => VarType::Bool,
    };
    if let Some(existing) = declared.get(name).copied() {
        let existing_concrete = concrete_type(existing);
        if existing_concrete != incoming {
            return Err(type_change(name, existing_concrete, incoming, span));
        }
        Ok(false)
    } else {
        Ok(true)
    }
}

fn emit_set(
    out: &mut String,
    declaration_slots: &mut DeclarationSlots,
    declared: &mut HashMap<String, VarType>,
    target: &str,
    value: &Value,
    source: &str,
    functions: &HashMap<String, usize>,
) -> Result<(), Diagnostic> {
    if is_native_reserved_name(target) {
        return Err(reserved_name(
            "a variable named",
            "변수 이름",
            target,
            span_of_value(value),
        ));
    }
    if functions.contains_key(target) {
        return Err(native_function_name_collision(target, span_of_value(value)));
    }
    match value {
        Value::Python(code) => {
            let text = code_text(code, source);
            let span = code_span(code);
            let (lowered, kind) = check_expr(text, span, declared, functions)?;
            let is_new = assignment_type(declared, target, kind, span)?;
            match kind {
                ExprType::Int => {
                    if is_new {
                        declaration_slots.declare(out, &format!("int {target};\n"));
                    }
                    declared.insert(target.to_string(), VarType::Int);
                    out.push_str(&format!("{target} = {lowered};\n"));
                    Ok(())
                }
                ExprType::Float => {
                    if is_new {
                        declaration_slots.declare(out, &format!("double {target};\n"));
                    }
                    declared.insert(target.to_string(), VarType::Float);
                    out.push_str(&format!("{target} = {lowered};\n"));
                    Ok(())
                }
                ExprType::Bool => {
                    if is_new {
                        declaration_slots.declare(out, &format!("int {target};\n"));
                    }
                    declared.insert(target.to_string(), VarType::Bool);
                    out.push_str(&format!("{target} = {lowered};\n"));
                    Ok(())
                }
                ExprType::Str => {
                    if is_new {
                        declaration_slots
                            .declare(out, &format!("char {target}[NME_STRING_CAPACITY];\n"));
                    }
                    declared.insert(target.to_string(), VarType::Str);
                    out.push_str(&format!(
                        "nme_copy({target}, sizeof {target}, {lowered});\n"
                    ));
                    Ok(())
                }
            }
        }
        Value::Literal(literal) => {
            let lowered = match literal {
                nme_core::syntax::Literal::True => "1",
                nme_core::syntax::Literal::False => "0",
                nme_core::syntax::Literal::None => {
                    return Err(not_supported("null value", span_of_value(value)));
                }
            };
            let span = span_of_value(value);
            let is_new = assignment_type(declared, target, ExprType::Bool, span)?;
            if is_new {
                declaration_slots.declare(out, &format!("int {target};\n"));
            }
            declared.insert(target.to_string(), VarType::Bool);
            out.push_str(&format!("{target} = {lowered};\n"));
            Ok(())
        }
        Value::Text(_)
        | Value::RandomInteger { .. }
        | Value::RandomChoice { .. }
        | Value::List(_)
        | Value::ZeroKnowledge(_) => Err(not_supported("this value", span_of_value(value))),
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_update(
    out: &mut String,
    declared: &mut HashMap<String, VarType>,
    target: &str,
    amount: &nme_core::syntax::Code,
    operation: nme_core::syntax::UpdateOp,
    source: &str,
    span: Span,
    functions: &HashMap<String, usize>,
) -> Result<(), Diagnostic> {
    if is_native_reserved_name(target) {
        return Err(reserved_name("a variable named", "변수 이름", target, span));
    }
    if !declared.contains_key(target) {
        return Err(value_change_without_binding(target, span));
    }
    match declared.get(target).copied() {
        Some(kind) if is_maybe_type(kind) => return Err(uninitialized_name(target, span)),
        Some(VarType::Str) => return Err(string_value_change(span)),
        Some(VarType::Bool) => return Err(not_supported("changing a boolean value", span)),
        _ => {}
    }
    if matches!(
        operation,
        nme_core::syntax::UpdateOp::Multiply | nme_core::syntax::UpdateOp::Divide
    ) {
        return Err(not_supported("multiplying or dividing a value", span));
    }
    let amount_text = code_text(amount, source);
    let (lowered, kind) = check_expr(amount_text, span, declared, functions)?;
    if kind != ExprType::Int {
        return Err(not_supported("a non-integer value change amount", span));
    }
    let target_kind = concrete_type(
        declared
            .get(target)
            .copied()
            .expect("value change target was checked above"),
    );
    if target_kind == VarType::Int {
        let helper = match operation {
            nme_core::syntax::UpdateOp::Add => "nme_add_int",
            nme_core::syntax::UpdateOp::Subtract => "nme_sub_int",
            // Rejected above: the native core has no overflow-checked
            // multiply or divide helper yet.
            _ => unreachable!("multiply and divide are rejected above"),
        };
        out.push_str(&format!("{target} = {helper}({target}, {lowered});\n"));
    } else {
        let helper = match operation {
            nme_core::syntax::UpdateOp::Add => "nme_add_float",
            nme_core::syntax::UpdateOp::Subtract => "nme_sub_float",
            _ => unreachable!("multiply and divide are rejected above"),
        };
        out.push_str(&format!("{target} = {helper}({target}, {lowered});\n"));
    }
    Ok(())
}

/// A Python line is accepted when it is a simple integer or string-literal
/// assignment, an integer `return`, or a `def` header over integer
/// parameters. `def` opens a C function body.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn emit_python_line(
    out: &mut String,
    open_braces: &mut usize,
    declaration_slots: &mut DeclarationSlots,
    declared: &mut HashMap<String, VarType>,
    tokens: &[lexer::Token],
    text: &str,
    _source: &str,
    line_span: Span,
    functions: &HashMap<String, usize>,
) -> Option<Diagnostic> {
    let span = Span::new(line_span.start, line_span.start + text.len());
    match tokens.first().map(|token| &token.tok) {
        Some(rustpython_parser::Tok::Def) => {
            let name = match tokens.get(1).map(|token| &token.tok) {
                Some(rustpython_parser::Tok::Name { name }) => name.clone(),
                _ => return Some(native_function_header_not_supported(span)),
            };
            if is_native_reserved_name(&name) || is_c_file_scope_reserved_name(&name) {
                return Some(reserved_name("a function named", "함수 이름", &name, span));
            }
            let Some((_, parameters)) = native_function_header(tokens) else {
                return Some(native_function_header_not_supported(span));
            };
            if let Some(parameter) = parameters
                .iter()
                .enumerate()
                .find_map(|(index, parameter)| {
                    parameters[..index].contains(parameter).then_some(parameter)
                })
            {
                return Some(duplicate_native_parameter(parameter, span));
            }
            if let Some(parameter) = parameters
                .iter()
                .find(|parameter| is_native_reserved_name(parameter))
            {
                return Some(reserved_name(
                    "a function parameter named",
                    "함수 매개변수",
                    parameter,
                    span,
                ));
            }
            if let Some(parameter) = parameters
                .iter()
                .find(|parameter| functions.contains_key(parameter.as_str()))
            {
                return Some(native_function_name_collision(parameter, span));
            }
            if parameters.is_empty() {
                out.push_str(&format!("int {name}(void) {{\n"));
            } else {
                let params = parameters
                    .iter()
                    .map(|parameter| format!("int {parameter}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("int {name}({params}) {{\n"));
            }
            for parameter in parameters {
                declared.insert(parameter, VarType::Int);
            }
            *open_braces += 1;
            None
        }
        Some(rustpython_parser::Tok::Return) => {
            let expression = text
                .strip_prefix("return")
                .map(str::trim)
                .unwrap_or_default();
            match check_expr(expression, span, declared, functions) {
                Ok((lowered, ExprType::Int)) => {
                    out.push_str(&format!("return {lowered};\n"));
                    None
                }
                Ok((_, ExprType::Float | ExprType::Str | ExprType::Bool)) => {
                    Some(native_function_requires_integer(span))
                }
                Err(diag) => Some(diag),
            }
        }
        Some(rustpython_parser::Tok::Name { .. })
            if matches!(
                tokens.get(1).map(|token| &token.tok),
                Some(rustpython_parser::Tok::Equal)
            ) =>
        {
            let name = match &tokens[0].tok {
                rustpython_parser::Tok::Name { name } => name.clone(),
                _ => return Some(not_supported("this Python line", span)),
            };
            if is_native_reserved_name(&name) {
                return Some(reserved_name("a variable named", "변수 이름", &name, span));
            }
            if functions.contains_key(&name) {
                return Some(native_function_name_collision(&name, span));
            }
            let expression = text.split_once('=').map_or(text, |(_, right)| right.trim());
            match check_expr(expression, span, declared, functions) {
                Ok((lowered, ExprType::Int)) => {
                    let is_new = match assignment_type(declared, &name, ExprType::Int, span) {
                        Ok(is_new) => is_new,
                        Err(diag) => return Some(diag),
                    };
                    if is_new {
                        declaration_slots.declare(out, &format!("int {name};\n"));
                    }
                    declared.insert(name.clone(), VarType::Int);
                    out.push_str(&format!("{name} = {lowered};\n"));
                    None
                }
                Ok((lowered, ExprType::Float)) => {
                    let is_new = match assignment_type(declared, &name, ExprType::Float, span) {
                        Ok(is_new) => is_new,
                        Err(diag) => return Some(diag),
                    };
                    if is_new {
                        declaration_slots.declare(out, &format!("double {name};\n"));
                    }
                    declared.insert(name.clone(), VarType::Float);
                    out.push_str(&format!("{name} = {lowered};\n"));
                    None
                }
                Ok((lowered, ExprType::Bool)) => {
                    let is_new = match assignment_type(declared, &name, ExprType::Bool, span) {
                        Ok(is_new) => is_new,
                        Err(diag) => return Some(diag),
                    };
                    if is_new {
                        declaration_slots.declare(out, &format!("int {name};\n"));
                    }
                    declared.insert(name.clone(), VarType::Bool);
                    out.push_str(&format!("{name} = {lowered};\n"));
                    None
                }
                Ok((lowered, ExprType::Str)) => {
                    let is_new = match assignment_type(declared, &name, ExprType::Str, span) {
                        Ok(is_new) => is_new,
                        Err(diag) => return Some(diag),
                    };
                    if is_new {
                        declaration_slots
                            .declare(out, &format!("char {name}[NME_STRING_CAPACITY];\n"));
                    }
                    declared.insert(name.clone(), VarType::Str);
                    out.push_str(&format!("nme_copy({name}, sizeof {name}, {lowered});\n"));
                    None
                }
                Err(diag) => Some(diag),
            }
        }
        _ => Some(not_supported("this Python line", span)),
    }
}

/// C keywords that a Python identifier must not collide with in generated C.
fn is_c_keyword(name: &str) -> bool {
    matches!(
        name,
        "auto"
            | "break"
            | "case"
            | "char"
            | "const"
            | "continue"
            | "default"
            | "do"
            | "double"
            | "else"
            | "enum"
            | "extern"
            | "float"
            | "for"
            | "goto"
            | "if"
            | "inline"
            | "int"
            | "long"
            | "register"
            | "restrict"
            | "return"
            | "short"
            | "signed"
            | "sizeof"
            | "static"
            | "struct"
            | "switch"
            | "typedef"
            | "union"
            | "unsigned"
            | "void"
            | "volatile"
            | "while"
    )
}

/// C reserves double-underscore names and names beginning with an underscore
/// followed by an uppercase letter for the implementation, in every scope.
fn is_c_implementation_reserved_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    name.starts_with("__")
        || (bytes.first() == Some(&b'_') && bytes.get(1).is_some_and(u8::is_ascii_uppercase))
}

/// C reserves every leading-underscore name at file scope. Generated native
/// functions have file scope, while ordinary bindings are function-local.
fn is_c_file_scope_reserved_name(name: &str) -> bool {
    name.starts_with('_')
}

/// Names exposed by the C headers used by generated native code.
const NATIVE_C_HEADER_NAMES: &[&str] = &[
    "CHAR_BIT",
    "SCHAR_MIN",
    "SCHAR_MAX",
    "UCHAR_MAX",
    "CHAR_MIN",
    "CHAR_MAX",
    "MB_LEN_MAX",
    "SHRT_MIN",
    "SHRT_MAX",
    "USHRT_MAX",
    "UINT_MAX",
    "LONG_MIN",
    "LONG_MAX",
    "ULONG_MAX",
    "LLONG_MIN",
    "LLONG_MAX",
    "ULLONG_MAX",
    "DBL_MAX",
    "EOF",
    "NULL",
    "FILE",
    "fpos_t",
    "ptrdiff_t",
    "BUFSIZ",
    "FILENAME_MAX",
    "FOPEN_MAX",
    "L_tmpnam",
    "TMP_MAX",
    "SEEK_SET",
    "SEEK_CUR",
    "SEEK_END",
    "stdin",
    "stdout",
    "EXIT_FAILURE",
    "EXIT_SUCCESS",
    "RAND_MAX",
    "div_t",
    "ldiv_t",
    "lldiv_t",
    "max_align_t",
    "va_list",
    "wchar_t",
    "abs",
    "abort",
    "aligned_alloc",
    "atof",
    "atoi",
    "atol",
    "atoll",
    "atexit",
    "at_quick_exit",
    "bsearch",
    "calloc",
    "div",
    "fclose",
    "feof",
    "ferror",
    "fflush",
    "fgetc",
    "fgetpos",
    "fgets",
    "fopen",
    "fprintf",
    "fputc",
    "fread",
    "freopen",
    "fscanf",
    "fseek",
    "fsetpos",
    "fwrite",
    "fgetws",
    "fputws",
    "getc",
    "getchar",
    "getenv",
    "free",
    "ldiv",
    "lldiv",
    "malloc",
    "memchr",
    "memcmp",
    "memmove",
    "memset",
    "mblen",
    "mbstowcs",
    "mbtowc",
    "perror",
    "putc",
    "putchar",
    "puts",
    "qsort",
    "quick_exit",
    "rand",
    "realloc",
    "remove",
    "rename",
    "rewind",
    "scanf",
    "setbuf",
    "setvbuf",
    "snprintf",
    "sprintf",
    "strcat",
    "strchr",
    "strcoll",
    "strcpy",
    "strcspn",
    "strerror",
    "strncpy",
    "strncat",
    "strncmp",
    "strpbrk",
    "strrchr",
    "strspn",
    "strstr",
    "strtod",
    "strtof",
    "strtok",
    "strtol",
    "strtold",
    "strtoll",
    "strtoul",
    "strtoull",
    "strxfrm",
    "system",
    "srand",
    "tmpfile",
    "tmpnam",
    "ungetc",
    "vfprintf",
    "vfscanf",
    "vprintf",
    "vscanf",
    "vsnprintf",
    "vsprintf",
    "vsscanf",
    "wctomb",
    "wcstombs",
];

/// Names emitted by the native runtime that user identifiers must not shadow.
fn is_native_runtime_name(name: &str) -> bool {
    NATIVE_C_HEADER_NAMES.contains(&name)
        || matches!(
            name,
            "NME_UNUSED"
                | "NME_STRING_CAPACITY"
                | "_nme_i"
                | "INT_MAX"
                | "INT_MIN"
                | "main"
                | "nme_add_int"
                | "nme_add_float"
                | "memcpy"
                | "nme_cat"
                | "nme_cat_buf"
                | "nme_cat_index"
                | "nme_copy"
                | "nme_float_result"
                | "nme_integer_division_by_zero"
                | "nme_integer_overflow"
                | "nme_len"
                | "nme_mod_int"
                | "nme_mul_int"
                | "nme_mul_float"
                | "nme_neg_int"
                | "nme_non_finite_float"
                | "nme_sub_float"
                | "nme_sub_int"
                | "nme_string_overflow"
                | "printf"
                | "strcmp"
                | "stderr"
                | "strlen"
                | "fputs"
                | "exit"
                | "len"
                | "size_t"
        )
}

fn is_native_reserved_name(name: &str) -> bool {
    is_c_keyword(name) || is_c_implementation_reserved_name(name) || is_native_runtime_name(name)
}

fn native_name_reason(name: &str) -> &'static str {
    if is_c_keyword(name) {
        "C keyword"
    } else if is_native_runtime_name(name) {
        "reserved native runtime name"
    } else if is_c_implementation_reserved_name(name) || is_c_file_scope_reserved_name(name) {
        "C implementation-reserved identifier"
    } else {
        "reserved native runtime name"
    }
}

fn native_name_reason_ko(name: &str) -> &'static str {
    if is_c_keyword(name) {
        "C 키워드"
    } else if is_native_runtime_name(name) {
        "네이티브 런타임 예약 이름"
    } else if is_c_implementation_reserved_name(name) || is_c_file_scope_reserved_name(name) {
        "C 구현 예약 식별자"
    } else {
        "네이티브 런타임 예약 이름"
    }
}

fn native_break_outside_loop(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::BreakOutsideLoop,
        "`break` can only be used inside a loop",
        "`멈춰`는 반복문 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside a native `while ... end` or `times:` loop",
        "네이티브 `동안 ... 끝` 또는 `3번:` 반복 안에 넣어 주세요",
    )
}

fn native_return_outside_function(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ReturnOutsideFunction,
        "`return` outside a native function",
        "네이티브 함수 밖의 `return`",
        span,
    )
    .with_bilingual_hint(
        "move it inside a native `def` function, or remove it",
        "네이티브 `def` 함수 안으로 옮기거나 지워 주세요",
    )
}

fn reserved_name(english_kind: &str, korean_kind: &str, name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!(
            "the native backend does not support {english_kind} `{name}` ({}) yet",
            native_name_reason(name)
        ),
        format!(
            "네이티브 백엔드는 아직 {korean_kind} `{name}` ({})을(를) 지원하지 않습니다",
            native_name_reason_ko(name)
        ),
        span,
    )
    .with_bilingual_hint(
        "use only the documented native core: boolean, integer, finite-float, and string values, while/if over comparisons, functions, and say",
        "문서에 있는 네이티브 코어만 쓰세요: 불리언·정수·유한 실수·문자열 값, 비교 조건의 while/if, 함수, say",
    )
}

fn type_change(name: &str, from: VarType, to: VarType, span: Span) -> Diagnostic {
    let (from_english, from_korean) = native_type_names(from);
    let (to_english, to_korean) = native_type_names(to);
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!(
            "the native backend does not support changing the type of `{name}` from {from_english} to {to_english}"
        ),
        format!(
            "네이티브 백엔드는 `{name}`의 타입 변경({from_korean} → {to_korean})을 지원하지 않습니다"
        ),
        span,
    )
    .with_bilingual_hint(
        "assign one native type to each name: boolean, integer, float, or string",
        "이름마다 네이티브 타입 하나만 대입하세요: 불리언, 정수, 실수, 문자열",
    )
}

fn native_type_names(kind: VarType) -> (&'static str, &'static str) {
    match concrete_type(kind) {
        VarType::Int => ("integer", "정수"),
        VarType::Float => ("float", "실수"),
        VarType::Str => ("string", "문자열"),
        VarType::Bool => ("boolean", "불리언"),
        VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr | VarType::MaybeBool => {
            unreachable!()
        }
    }
}

fn value_change_without_binding(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("the native backend requires `{name}` to be assigned before a value change"),
        format!("네이티브 백엔드는 값을 바꾸기 전에 `{name}`을(를) 먼저 대입해야 합니다"),
        span,
    )
    .with_bilingual_hint(
        "assign the name first, then use add/subtract to change its value",
        "먼저 이름에 값을 대입한 다음 더하기/빼기로 값을 바꾸세요",
    )
}

fn string_value_change(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "the native backend does not support changing a string value",
        "네이티브 백엔드는 문자열 값 변경을 지원하지 않습니다",
        span,
    )
    .with_bilingual_hint(
        "use add/subtract with an existing integer or float binding",
        "이미 대입된 정수·실수 바인딩에 더하기/빼기를 사용하세요",
    )
}

fn uninitialized_name(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("the native backend cannot use `{name}` before a conditional assignment has run"),
        format!("네이티브 백엔드는 조건부 대입이 실행되기 전에 `{name}`을(를) 사용할 수 없습니다"),
        span,
    )
    .with_bilingual_hint(
        "assign the name before the control block, or use it inside the block after assignment",
        "제어 블록 전에 이름에 대입하거나 대입한 뒤 블록 안에서 사용하세요",
    )
}

fn integer_literal_out_of_range(magnitude: &str, negative: bool, span: Span) -> Diagnostic {
    let literal = if negative {
        format!("-{magnitude}")
    } else {
        magnitude.to_string()
    };
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!(
            "the native backend only supports integer literals from -2147483648 to 2147483647; `{literal}` is outside that range"
        ),
        format!(
            "네이티브 백엔드는 -2147483648부터 2147483647까지의 정수 리터럴만 지원합니다. `{literal}`은(는) 그 범위를 벗어납니다"
        ),
        span,
    )
    .with_bilingual_hint(
        "use a value within the native integer range or run this program with CPython",
        "네이티브 정수 범위 안의 값을 사용하거나 이 프로그램을 CPython으로 실행하세요",
    )
}

fn native_function_requires_integer(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "native backend functions currently accept and return integer values",
        "네이티브 백엔드 함수는 현재 정수 값만 매개변수와 반환값으로 받습니다",
        span,
    )
    .with_bilingual_hint(
        "pass and return integers in native functions, or run the program with CPython",
        "네이티브 함수에는 정수를 전달하고 반환하거나 프로그램을 CPython으로 실행하세요",
    )
}

fn native_function_requires_return(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "the native backend requires every function to return an integer on every path",
        "네이티브 백엔드는 모든 경로에서 함수가 정수를 반환해야 합니다",
        span,
    )
    .with_bilingual_hint(
        "add a top-level integer return after conditional blocks, or run the program with CPython",
        "조건부 블록 뒤에 최상위 정수 반환을 추가하거나 프로그램을 CPython으로 실행하세요",
    )
}

fn unknown_native_name(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("the native backend cannot use `{name}` without a prior native binding"),
        format!(
            "네이티브 백엔드는 먼저 네이티브 바인딩을 하지 않고 `{name}`을(를) 사용할 수 없습니다"
        ),
        span,
    )
    .with_bilingual_hint(
        "assign the name before using it, or run the program with CPython",
        "사용하기 전에 이름에 대입하거나 프로그램을 CPython으로 실행하세요",
    )
}

fn native_function_value(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("the native backend does not support using native function `{name}` as a value"),
        format!(
            "네이티브 백엔드는 네이티브 함수 `{name}`을(를) 값으로 사용하는 것을 지원하지 않습니다"
        ),
        span,
    )
    .with_bilingual_hint(
        "call the function with its declared positional integer arguments",
        "선언된 위치 기반 정수 인자로 함수를 호출하세요",
    )
}

fn native_function_name_collision(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!(
            "the native backend does not support a binding named `{name}` because it shadows a native function name"
        ),
        format!(
            "네이티브 백엔드는 네이티브 함수 이름을 가리는 `{name}` 바인딩을 지원하지 않습니다"
        ),
        span,
    )
    .with_bilingual_hint(
        "use a different variable or parameter name, or run the program with CPython",
        "다른 변수나 매개변수 이름을 사용하거나 프로그램을 CPython으로 실행하세요",
    )
}

fn duplicate_native_parameter(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("native function parameter `{name}` is listed more than once"),
        format!("네이티브 함수 매개변수 `{name}`이(가) 두 번 이상 나열되었습니다"),
        span,
    )
    .with_bilingual_hint(
        "list each function parameter name only once",
        "함수 매개변수 이름을 각각 한 번만 적으세요",
    )
}

fn unknown_native_function(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("unknown native function `{name}`"),
        format!("알 수 없는 네이티브 함수 `{name}`입니다"),
        span,
    )
    .with_bilingual_hint(
        "define the integer function in this file, or run the program with CPython",
        "이 파일에 정수 함수를 정의하거나 프로그램을 CPython으로 실행하세요",
    )
}

fn native_function_arity(name: &str, expected: usize, actual: usize, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!(
            "native function `{name}` expects {expected} integer argument(s), but got {actual}"
        ),
        format!(
            "네이티브 함수 `{name}`은(는) 정수 인자 {expected}개를 필요로 하지만 {actual}개를 받았습니다"
        ),
        span,
    )
    .with_bilingual_hint(
        "call the function with exactly the integer parameters in its definition",
        "함수 정의에 있는 정수 매개변수 개수와 똑같이 호출하세요",
    )
}

fn native_keyword_arguments(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "the native backend does not support keyword arguments in function calls",
        "네이티브 백엔드는 함수 호출의 키워드 인자를 지원하지 않습니다",
        span,
    )
    .with_bilingual_hint(
        "pass the declared integer parameters positionally, or run the program with CPython",
        "선언된 정수 매개변수를 위치 인자로 전달하거나 프로그램을 CPython으로 실행하세요",
    )
}

fn duplicate_native_function(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("native function `{name}` is defined more than once"),
        format!("네이티브 함수 `{name}`이(가) 두 번 이상 정의되었습니다"),
        span,
    )
    .with_bilingual_hint(
        "keep one definition for each native function name",
        "네이티브 함수 이름마다 정의 하나만 남기세요",
    )
}

fn native_function_header_not_supported(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "the native backend does not support this function header",
        "네이티브 백엔드는 이 함수 헤더를 지원하지 않습니다",
        span,
    )
    .with_bilingual_hint(
        "use a simple def name(integer, ...) header without defaults, annotations, or varargs",
        "기본값·주석·가변 인자 없이 단순한 def 이름(정수, ...) 함수 헤더를 사용하세요",
    )
}

fn native_nested_function(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "the native backend does not support nested function definitions",
        "네이티브 백엔드는 중첩 함수 정의를 지원하지 않습니다",
        span,
    )
    .with_bilingual_hint(
        "move the function definition to the file level, or run the program with CPython",
        "함수 정의를 파일 최상위로 옮기거나 프로그램을 CPython으로 실행하세요",
    )
}

fn native_string_nul(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "the native backend does not support strings with embedded NUL characters",
        "네이티브 백엔드는 내부 NUL 문자가 들어 있는 문자열을 지원하지 않습니다",
        span,
    )
    .with_bilingual_hint(
        "use text without an embedded NUL character, or run the program with CPython",
        "내부 NUL 문자가 없는 텍스트를 사용하거나 프로그램을 CPython으로 실행하세요",
    )
}

fn native_non_finite_float(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "the native backend only supports finite float literals",
        "네이티브 백엔드는 유한한 실수 리터럴만 지원합니다",
        span,
    )
    .with_bilingual_hint(
        "use a finite float literal, or run the program with CPython",
        "유한한 실수 리터럴을 사용하거나 프로그램을 CPython으로 실행하세요",
    )
}

fn not_supported(what: &str, span: Span) -> Diagnostic {
    let what_ko = match what {
        "a boolean in arithmetic" => "산술에서 불리언 값",
        "a boolean in a numeric expression" => "숫자 표현식의 불리언 값",
        "ordering boolean values in a condition" => "조건에서 불리언 값의 순서 비교",
        "incompatible values in a condition" => "조건에서 호환되지 않는 값",
        "changing a boolean value" => "불리언 값 변경",
        "multiplying or dividing a value" => "값의 곱하기·나누기",
        "list values" => "목록 값",
        "repeating over a list" => "목록 반복",
        "waiting" => "기다리기",
        "adding to a list" => "목록에 넣기",
        "skipping to the next round" => "다음 반복으로 건너뛰기",
        _ => what,
    };
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("the native backend does not support {what} yet"),
        format!("네이티브 백엔드는 아직 {what_ko}을(를) 지원하지 않습니다"),
        span,
    )
    .with_bilingual_hint(
        "use only the documented native core: boolean, integer, finite-float, and string values, while/if over comparisons, functions, and say",
        "문서에 있는 네이티브 코어만 쓰세요: 불리언·정수·유한 실수·문자열 값, 비교 조건의 while/if, 함수, say",
    )
}

fn unsupported_statement(stmt: &NmeStmt, span: Span) -> Diagnostic {
    let what = match stmt {
        NmeStmt::Ask { .. } => "input (ask)",
        NmeStmt::Times { .. } => "repeat blocks",
        NmeStmt::ForEach { .. } => "repeating over a list",
        NmeStmt::Wait { .. } => "waiting",
        NmeStmt::Append { .. } => "adding to a list",
        NmeStmt::Continue => "skipping to the next round",
        NmeStmt::UseModule { .. } => "bundled modules",
        NmeStmt::FileRead { .. } | NmeStmt::FileWrite { .. } => "file operations",
        NmeStmt::ModuleImport { .. } => "module imports",
        NmeStmt::ElseIf { .. } | NmeStmt::Else { .. } => "this branch",
        _ => "this statement",
    };
    not_supported(what, span)
}

fn code_span(code: &nme_core::syntax::Code) -> Span {
    match code {
        nme_core::syntax::Code::Source(span) => *span,
    }
}

fn span_of_value(value: &Value) -> Span {
    match value {
        Value::Python(code) => code_span(code),
        _ => Span::new(0, 0),
    }
}
