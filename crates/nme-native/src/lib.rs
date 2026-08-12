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
//! - `say`/`show`/`말해` of an integer expression, a string variable, or a
//!   string literal (one binary `+` concatenation);
//! - signed 32-bit integer literals and arithmetic with explicit overflow and
//!   zero-divisor checks;
//! - `set x to ...` / `x은 ...` / `x = ...` assignments of integers and
//!   string literals (including the Python-looking form);
//!   native string variables use checked 8192-byte buffers;
//! - value changes on an existing integer or float binding:
//!   `x add N` / `x = x + N` / `점수에 N 더해`;
//! - bindings first assigned in a possibly skipped control block must be
//!   initialized before the block or used after assignment within it;
//! - `while`/`if`/`else`/`else if` blocks over integer comparisons, closed
//!   by `end`/`끝`;
//! - `break` inside a loop;
//! - functions over integer parameters with an unconditional integer `return`
//!   (recursion works); calls must name a function in the file and use its
//!   declared arity.
//!
//! Anything outside this core is rejected with a clear bilingual diagnostic;
//! it is never silently miscompiled. The rest of NME keeps running on CPython.

use std::collections::HashMap;

use nme_core::diagnostics::{Diagnostic, DiagnosticCode, Span};
use nme_core::syntax::{CompareOp, Condition, ConditionValue, InlineStmt, NmeStmt, Value};
use nme_core::{lexer, parser};

use rustpython_parser::ast::{CmpOp, Constant, Expr, Operator, UnaryOp};
use rustpython_parser::Parse as _;

/// The static type the native backend tracks per variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VarType {
    Int,
    Float,
    Str,
    MaybeInt,
    MaybeFloat,
    MaybeStr,
}

/// The static type of an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExprType {
    Int,
    Float,
    Str,
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
/// unless the block is statically known to run.
#[derive(Debug)]
struct NativeBlockFrame {
    body_depth: usize,
    bindings_before: HashMap<String, VarType>,
    definitely_runs: bool,
}

fn concrete_type(kind: VarType) -> VarType {
    match kind {
        VarType::Int | VarType::MaybeInt => VarType::Int,
        VarType::Float | VarType::MaybeFloat => VarType::Float,
        VarType::Str | VarType::MaybeStr => VarType::Str,
    }
}

fn maybe_type(kind: VarType) -> VarType {
    match concrete_type(kind) {
        VarType::Int => VarType::MaybeInt,
        VarType::Float => VarType::MaybeFloat,
        VarType::Str => VarType::MaybeStr,
        VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr => unreachable!(),
    }
}

fn is_maybe_type(kind: VarType) -> bool {
    matches!(kind, VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr)
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

fn finish_native_block(frame: NativeBlockFrame, declared: &mut HashMap<String, VarType>) {
    if frame.definitely_runs {
        return;
    }
    let names = declared.keys().cloned().collect::<Vec<_>>();
    for name in names {
        if let Some(previous) = frame.bindings_before.get(&name).copied() {
            if is_maybe_type(previous) {
                declared.insert(name, previous);
            }
        } else if let Some(current) = declared.get(&name).copied() {
            declared.insert(name, maybe_type(current));
        }
    }
}

const PREAMBLE: &str = concat!(
    "#include <limits.h>\n",
    "#include <stdio.h>\n",
    "#include <stdlib.h>\n",
    "#include <string.h>\n",
    "#if INT_MAX != 2147483647 || INT_MIN != (-2147483647 - 1)\n",
    "#error \"NME native backend requires a 32-bit C int\"\n",
    "#endif\n",
    "#define NME_STRING_CAPACITY 8192\n",
    "static char nme_cat_buf[NME_STRING_CAPACITY];\n",
    "static void nme_integer_overflow(void) {\n",
    "    fputs(\"nme native: integer overflow / 정수 오버플로가 발생했습니다\\n\", stderr);\n",
    "    exit(1);\n",
    "}\n",
    "static void nme_integer_division_by_zero(void) {\n",
    "    fputs(\"nme native: integer modulo by zero / 정수 나머지의 제수가 0입니다\\n\", stderr);\n",
    "    exit(1);\n",
    "}\n",
    "static int nme_add_int(int left, int right) {\n",
    "    if ((right > 0 && left > INT_MAX - right)\n",
    "        || (right < 0 && left < INT_MIN - right)) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return left + right;\n",
    "}\n",
    "static int nme_sub_int(int left, int right) {\n",
    "    if ((right < 0 && left > INT_MAX + right)\n",
    "        || (right > 0 && left < INT_MIN + right)) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return left - right;\n",
    "}\n",
    "static int nme_mul_int(int left, int right) {\n",
    "    long long result = (long long)left * (long long)right;\n",
    "    if (result > INT_MAX || result < INT_MIN) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return (int)result;\n",
    "}\n",
    "static int nme_mod_int(int left, int right) {\n",
    "    if (right == 0) {\n",
    "        nme_integer_division_by_zero();\n",
    "    }\n",
    "    if (left == INT_MIN && right == -1) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return left % right;\n",
    "}\n",
    "static int nme_neg_int(int value) {\n",
    "    if (value == INT_MIN) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return -value;\n",
    "}\n",
    "static int nme_len(const char *value) {\n",
    "    size_t length = strlen(value);\n",
    "    if (length > (size_t)INT_MAX) {\n",
    "        nme_integer_overflow();\n",
    "    }\n",
    "    return (int)length;\n",
    "}\n",
    "static void nme_string_overflow(void) {\n",
    "    fputs(\"nme native: string value exceeds 8191 bytes / 문자열 값이 8191바이트를 초과했습니다\\n\", stderr);\n",
    "    exit(1);\n",
    "}\n",
    "static void nme_copy(char *destination, size_t capacity, const char *source) {\n",
    "    size_t length = strlen(source);\n",
    "    if (length >= capacity) {\n",
    "        nme_string_overflow();\n",
    "    }\n",
    "    memcpy(destination, source, length + 1);\n",
    "}\n",
    "static char *nme_cat(const char *a, const char *b) {\n",
    "    size_t a_length = strlen(a);\n",
    "    size_t b_length = strlen(b);\n",
    "    if (a_length >= NME_STRING_CAPACITY\n",
    "        || b_length >= NME_STRING_CAPACITY - a_length) {\n",
    "        nme_string_overflow();\n",
    "    }\n",
    "    memcpy(nme_cat_buf, a, a_length);\n",
    "    memcpy(nme_cat_buf + a_length, b, b_length + 1);\n",
    "    return nme_cat_buf;\n",
    "}\n",
    "int main(void) {\n",
);

fn native_function_signatures(lines: &[lexer::LogicalLine]) -> HashMap<String, usize> {
    let mut functions = HashMap::new();
    for line in lines {
        if !matches!(
            line.tokens.first().map(|token| &token.tok),
            Some(rustpython_parser::Tok::Def)
        ) {
            continue;
        }
        let Some(rustpython_parser::Tok::Name { name }) =
            line.tokens.get(1).map(|token| &token.tok)
        else {
            continue;
        };
        let parameters = line
            .tokens
            .iter()
            .filter(|token| matches!(&token.tok, rustpython_parser::Tok::Name { .. }))
            .count()
            .saturating_sub(1);
        functions.insert(name.clone(), parameters);
    }
    functions
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
    let functions = native_function_signatures(&lines);

    let mut out = String::from(PREAMBLE);
    let mut open_braces = 1usize; // the `main` body
    let mut declaration_slots = DeclarationSlots::new(&out);
    let mut declared = HashMap::new();
    let mut native_blocks = Vec::<NativeBlockFrame>::new();
    let mut saved_main_scope = None;
    let mut in_function = false;
    let mut function_span = None;
    let mut function_returned = false;
    let mut problems = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let line_text = &source[line.span.start..line.span.end];
        let is_significant_line = !line_text.trim().is_empty()
            && !line_text.trim_start().starts_with('#');
        let nme_line = by_index.get(&index);
        let current_depth = nme_line.map_or(
            line.indent + program.virtual_indents[index],
            |nme_line| line.indent + nme_line.virtual_indent,
        );
        let is_branch = nme_line.is_some_and(|nme_line| {
            matches!(nme_line.stmt, NmeStmt::ElseIf { .. } | NmeStmt::Else { .. })
        });
        if !is_branch {
            while native_blocks
                .last()
                .is_some_and(|frame| current_depth < frame.body_depth)
            {
                let frame = native_blocks.pop().expect("native block frame exists");
                finish_native_block(frame, &mut declared);
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
                NmeStmt::While {
                    condition,
                    inline: None,
                } => match check_condition(
                    condition,
                    source,
                    nme_line.span,
                    &declared,
                    &functions,
                ) {
                    Ok(condition_text) => {
                        out.push_str(&format!("while ({condition_text}) {{\n"));
                        open_braces += 1;
                        native_blocks.push(NativeBlockFrame {
                            body_depth: current_depth + 1,
                            bindings_before: declared.clone(),
                            definitely_runs: false,
                        });
                    }
                    Err(diag) => problems.push(diag),
                },
                NmeStmt::When {
                    condition,
                    inline: None,
                } => match check_condition(
                    condition,
                    source,
                    nme_line.span,
                    &declared,
                    &functions,
                ) {
                    Ok(condition_text) => {
                        out.push_str(&format!("if ({condition_text}) {{\n"));
                        open_braces += 1;
                        native_blocks.push(NativeBlockFrame {
                            body_depth: current_depth + 1,
                            bindings_before: declared.clone(),
                            definitely_runs: condition_definitely_true(condition),
                        });
                    }
                    Err(diag) => problems.push(diag),
                },
                NmeStmt::Break => out.push_str("break;\n"),
                NmeStmt::Times { count, inline } => {
                    let count_text = code_text(count, source);
                    match check_expr(count_text, nme_line.span, &declared, &functions) {
                        Ok((lowered, ExprType::Int)) => {
                            let header =
                                format!("for (int _nme_i = 0; _nme_i < {lowered}; _nme_i++)");
                            match inline {
                                Some(InlineStmt::Nme(inner)) => {
                                    match lower_inline(
                                        inner,
                                        source,
                                        &declared,
                                        &functions,
                                        nme_line.span,
                                    ) {
                                        Ok(text) => out.push_str(&format!("{header} {text}\n")),
                                        Err(diag) => problems.push(diag),
                                    }
                                }
                                Some(InlineStmt::Python(_)) => {
                                    problems.push(not_supported("this inline body", nme_line.span));
                                }
                                None => {
                                    out.push_str(&format!("{header} {{\n"));
                                    open_braces += 1;
                                    native_blocks.push(NativeBlockFrame {
                                        body_depth: current_depth + 1,
                                        bindings_before: declared.clone(),
                                        definitely_runs: false,
                                    });
                                }
                            }
                        }
                        Err(diag) => problems.push(diag),
                        Ok(_) => problems.push(not_supported("this repeat count", nme_line.span)),
                    }
                }
                NmeStmt::ElseIf {
                    condition,
                    inline: None,
                } => match check_condition(
                    condition,
                    source,
                    nme_line.span,
                    &declared,
                    &functions,
                ) {
                    Ok(condition_text) => {
                        out.push_str(&format!("}} else if ({condition_text}) {{\n"));
                    }
                    Err(diag) => problems.push(diag),
                },
                NmeStmt::Else { inline: None } => out.push_str("} else {\n"),
                NmeStmt::End => {}
                other => problems.push(unsupported_statement(other, nme_line.span)),
            }
        } else {
            // A non-NME line: blank, comment, or a Python assignment.
            let total_depth = line.indent + program.virtual_indents[index];
            close_braces(&mut out, &mut open_braces, total_depth + 1);
            let text = line_text;
            let trimmed = text.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                out.push_str(text);
                out.push('\n');
                continue;
            }
            let is_function_header = !in_function
                && line.indent == 0
                && matches!(
                    line.tokens.first().map(|token| &token.tok),
                    Some(rustpython_parser::Tok::Def)
                );
            if is_function_header {
                saved_main_scope = Some(std::mem::take(&mut declared));
                in_function = true;
            }
            let output_before_line = out.len();
            let diagnostic = emit_python_line(
                &mut out,
                &mut open_braces,
                &mut declaration_slots,
                &mut declared,
                &line.tokens,
                text,
                source,
                line.span,
                &functions,
            );
            if let Some(diag) = diagnostic {
                problems.push(diag);
            } else if is_function_header && out.len() > output_before_line {
                declaration_slots.start_function(&out);
                function_span = Some(line.span);
                function_returned = false;
            } else if in_function
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
) -> Result<String, Diagnostic> {
    match stmt {
        NmeStmt::Say { value } => {
            let mut out = String::new();
            emit_say(&mut out, value, source, declared, functions)?;
            Ok(out.trim_end().to_string())
        }
        _ => Err(not_supported("this inline statement", span)),
    }
}

fn close_braces(out: &mut String, open: &mut usize, target: usize) {
    while *open > target {
        out.push_str("}\n");
        *open -= 1;
    }
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
    let mut escaped = false;
    for character in line.chars() {
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
        } else if character == '{' {
            delta += 1;
        } else if character == '}' {
            delta -= 1;
        }
    }
    delta
}

/// Validates and lowers a Python expression to C for the native core,
/// returning the C text and its static type. Integer expressions pass through
/// Integer expressions use checked native runtime helpers; string expressions
/// lower to a name, a literal, or one `+` concatenation through the small
/// runtime helper.
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
    let maximum = if negative { 2_147_483_648 } else { 2_147_483_647 };
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

fn lower_expr(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
    functions: &HashMap<String, usize>,
) -> Result<(String, ExprType), Diagnostic> {
    match expr {
        Expr::Constant(constant) => match &constant.value {
            Constant::Int(value) => Ok((
                native_integer_literal(value, false, span)?,
                ExprType::Int,
            )),
            Constant::Float(value) => Ok((format!("{value}"), ExprType::Float)),
            Constant::Str(string) => {
                let escaped = string.replace('\\', "\\\\").replace('"', "\\\"");
                Ok((format!("\"{escaped}\""), ExprType::Str))
            }
            _ => Err(not_supported("this constant", span)),
        },
        Expr::Name(name) => {
            let id = name.id.as_str();
            if is_native_reserved_name(id) {
                return Err(reserved_name("a variable named", "변수 이름", id, span));
            }
            match declared.get(id).copied() {
                Some(VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr) => {
                    Err(uninitialized_name(id, span))
                }
                Some(VarType::Str) => Ok((id.to_string(), ExprType::Str)),
                Some(VarType::Float) => Ok((id.to_string(), ExprType::Float)),
                Some(VarType::Int) | None => Ok((id.to_string(), ExprType::Int)),
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
                let (right, right_kind) =
                    numeric_operand(&binop.right, span, declared, functions)?;
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
                    format!("({left} {} {right})", operator_text(&binop.op))
                };
                Ok((
                    lowered,
                    kind,
                ))
            }
            _ => Err(not_supported("this operator", span)),
        },
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
            if callee.id.as_str() == "len" && call.args.len() == 1 {
                let (argument, kind) = lower_expr(&call.args[0], span, declared, functions)?;
                if kind == ExprType::Str {
                    return Ok((format!("nme_len({argument})"), ExprType::Int));
                }
            }
            if is_native_reserved_name(callee.id.as_str()) {
                return Err(reserved_name(
                    "a call to",
                    "호출",
                    callee.id.as_str(),
                    span,
                ));
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
            Constant::Str(string) => {
                let escaped = string.replace('\\', "\\\\").replace('"', "\\\"");
                Ok(format!("\"{escaped}\""))
            }
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
    if matches!(kind, ExprType::Str) {
        return Err(not_supported("a text value in a numeric expression", span));
    }
    Ok((text, kind))
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn operator_text(operator: &Operator) -> &'static str {
    match operator {
        Operator::Add => "+",
        Operator::Sub => "-",
        Operator::Mult => "*",
        Operator::Mod => "%",
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
            let (right, right_kind) =
                condition_operand(right, source, span, declared, functions)?;
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
                    (
                        name.clone(),
                        match declared.get(name).copied() {
                            Some(VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr) => {
                                return Err(uninitialized_name(name, span));
                            }
                            Some(VarType::Float) => ExprType::Float,
                            _ => ExprType::Int,
                        },
                    )
                }
                ConditionValue::Python(code) => {
                    check_expr(code_text(code, source), span, declared, functions)?
                }
                ConditionValue::Literal(literal) => match literal {
                    nme_core::syntax::Literal::True => ("(1)".to_string(), ExprType::Int),
                    nme_core::syntax::Literal::False => ("(0)".to_string(), ExprType::Int),
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
        Condition::Logical { .. } => Err(not_supported("this condition", span)),
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
            let kind = match declared.get(name).copied() {
                Some(VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr) => {
                    return Err(uninitialized_name(name, span));
                }
                Some(VarType::Str) => ExprType::Str,
                Some(VarType::Float) => ExprType::Float,
                _ => ExprType::Int,
            };
            Ok((name.clone(), kind))
        }
        ConditionValue::Text(text) => {
            let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
            Ok((format!("\"{escaped}\""), ExprType::Str))
        }
        ConditionValue::Literal(_) => Err(not_supported("boolean/null in a condition", span)),
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
    let (right, right_kind) =
        lower_expr(&compare.comparators[0], span, declared, functions)?;
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
        _ => Err(not_supported(
            "mixing numbers and text in a condition",
            span,
        )),
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
            let escaped = literal.replace('\\', "\\\\").replace('"', "\\\"");
            out.push_str(&format!("printf(\"%s\\n\", \"{escaped}\");\n"));
            Ok(())
        }
        Value::Literal(_) => Err(not_supported("boolean/null output", span_of_value(value))),
        Value::RandomInteger { .. } | Value::RandomChoice { .. } => {
            Err(not_supported("random values", span_of_value(value)))
        }
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
                ExprType::Str => {
                    if is_new {
                        declaration_slots.declare(
                            out,
                            &format!("char {target}[NME_STRING_CAPACITY];\n"),
                        );
                    }
                    declared.insert(target.to_string(), VarType::Str);
                    out.push_str(&format!(
                        "nme_copy({target}, sizeof {target}, {lowered});\n"
                    ));
                    Ok(())
                }
            }
        }
        Value::Text(_)
        | Value::Literal(_)
        | Value::RandomInteger { .. }
        | Value::RandomChoice { .. }
        | Value::ZeroKnowledge(_) => Err(not_supported("this value", span_of_value(value))),
    }
}

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
        return Err(reserved_name(
            "a variable named",
            "변수 이름",
            target,
            span,
        ));
    }
    if !declared.contains_key(target) {
        return Err(value_change_without_binding(target, span));
    }
    match declared.get(target).copied() {
        Some(kind) if is_maybe_type(kind) => return Err(uninitialized_name(target, span)),
        Some(VarType::Str) => return Err(string_value_change(span)),
        _ => {}
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
        };
        out.push_str(&format!("{target} = {helper}({target}, {lowered});\n"));
    } else {
        let op = match operation {
            nme_core::syntax::UpdateOp::Add => "+=",
            nme_core::syntax::UpdateOp::Subtract => "-=",
        };
        out.push_str(&format!("{target} {op} {lowered};\n"));
    }
    Ok(())
}

/// A Python line is accepted when it is a simple integer or string-literal
/// assignment, an integer `return`, or a `def` header over integer
/// parameters. `def` opens a C function body.
#[allow(clippy::too_many_lines)]
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
                _ => return Some(not_supported("this function header", span)),
            };
            if is_native_reserved_name(&name) {
                return Some(reserved_name("a function named", "함수 이름", &name, span));
            }
            let parameters = tokens
                .iter()
                .filter_map(|token| match &token.tok {
                    rustpython_parser::Tok::Name { name } => Some(name.clone()),
                    _ => None,
                })
                .skip(1) // the function name itself
                .collect::<Vec<_>>();
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
            if parameters.is_empty() {
                out.push_str(&format!("int {name}() {{\n"));
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
                Ok((_, ExprType::Float | ExprType::Str)) => {
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
                Ok((lowered, ExprType::Str)) => {
                    let is_new = match assignment_type(declared, &name, ExprType::Str, span) {
                        Ok(is_new) => is_new,
                        Err(diag) => return Some(diag),
                    };
                    if is_new {
                        declaration_slots.declare(
                            out,
                            &format!("char {name}[NME_STRING_CAPACITY];\n"),
                        );
                    }
                    declared.insert(name.clone(), VarType::Str);
                    out.push_str(&format!(
                        "nme_copy({name}, sizeof {name}, {lowered});\n"
                    ));
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

/// Names emitted by the native runtime that user identifiers must not shadow.
fn is_native_reserved_name(name: &str) -> bool {
    is_c_keyword(name)
        || matches!(
            name,
            "NME_STRING_CAPACITY"
                | "_nme_i"
                | "INT_MAX"
                | "INT_MIN"
                | "main"
                | "nme_add_int"
                | "memcpy"
                | "nme_cat"
                | "nme_cat_buf"
                | "nme_copy"
                | "nme_integer_division_by_zero"
                | "nme_integer_overflow"
                | "nme_len"
                | "nme_mod_int"
                | "nme_mul_int"
                | "nme_neg_int"
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

fn native_name_reason(name: &str) -> &'static str {
    if is_c_keyword(name) {
        "C keyword"
    } else {
        "reserved native runtime name"
    }
}

fn native_name_reason_ko(name: &str) -> &'static str {
    if is_c_keyword(name) {
        "C 키워드"
    } else {
        "네이티브 런타임 예약 이름"
    }
}

fn reserved_name(
    english_kind: &str,
    korean_kind: &str,
    name: &str,
    span: Span,
) -> Diagnostic {
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
        "use only the documented native core: integer and string values, while/if over comparisons, functions, and say",
        "문서에 있는 네이티브 코어만 쓰세요: 정수·문자열 값, 비교 조건의 while/if, 함수, say",
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
        "assign one native type to each name: integer, float, or string",
        "이름마다 네이티브 타입 하나만 대입하세요: 정수, 실수, 문자열",
    )
}

fn native_type_names(kind: VarType) -> (&'static str, &'static str) {
    match concrete_type(kind) {
        VarType::Int => ("integer", "정수"),
        VarType::Float => ("float", "실수"),
        VarType::Str => ("string", "문자열"),
        VarType::MaybeInt | VarType::MaybeFloat | VarType::MaybeStr => unreachable!(),
    }
}

fn value_change_without_binding(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!(
            "the native backend requires `{name}` to be assigned before a value change"
        ),
        format!(
            "네이티브 백엔드는 값을 바꾸기 전에 `{name}`을(를) 먼저 대입해야 합니다"
        ),
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
        format!(
            "the native backend cannot use `{name}` before a conditional assignment has run"
        ),
        format!(
            "네이티브 백엔드는 조건부 대입이 실행되기 전에 `{name}`을(를) 사용할 수 없습니다"
        ),
        span,
    )
    .with_bilingual_hint(
        "assign the name before the control block, or use it inside the block after assignment",
        "제어 블록 전에 이름에 대입하거나 대입한 뒤 블록 안에서 사용하세요",
    )
}

fn integer_literal_out_of_range(
    magnitude: &str,
    negative: bool,
    span: Span,
) -> Diagnostic {
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

fn native_function_arity(
    name: &str,
    expected: usize,
    actual: usize,
    span: Span,
) -> Diagnostic {
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

fn not_supported(what: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        format!("the native backend does not support {what} yet"),
        format!("네이티브 백엔드는 아직 {what}을(를) 지원하지 않습니다"),
        span,
    )
    .with_bilingual_hint(
        "use only the documented native core: integer and string values, while/if over comparisons, functions, and say",
        "문서에 있는 네이티브 코어만 쓰세요: 정수·문자열 값, 비교 조건의 while/if, 함수, say",
    )
}

fn unsupported_statement(stmt: &NmeStmt, span: Span) -> Diagnostic {
    let what = match stmt {
        NmeStmt::Ask { .. } => "input (ask)",
        NmeStmt::Times { .. } => "repeat blocks",
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
