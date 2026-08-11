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
//! - `set x to ...` / `x은 ...` / `x = ...` assignments of integers and
//!   string literals (including the Python-looking form);
//! - value changes `x add N` / `x = x + N` / `점수에 N 더해`;
//! - `while`/`if`/`else`/`else if` blocks over integer comparisons, closed
//!   by `end`/`끝`;
//! - `break` inside a loop;
//! - functions over integer parameters with an integer `return` (recursion
//!   works).
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
    Str,
}

/// The static type of an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExprType {
    Int,
    Str,
}

const PREAMBLE: &str = concat!(
    "#include <stdio.h>\n",
    "#include <string.h>\n",
    "static char nme_cat_buf[8192];\n",
    "static char *nme_cat(const char *a, const char *b) {\n",
    "    strcpy(nme_cat_buf, a);\n",
    "    strcat(nme_cat_buf, b);\n",
    "    return nme_cat_buf;\n",
    "}\n",
    "int main(void) {\n",
);

/// Compiles the native core subset of `source` to C source text.
///
/// On failure returns every problem found, ready to render with
/// [`nme_core::diagnostics::render_all`].
pub fn native_compile(source: &str) -> Result<String, Vec<Diagnostic>> {
    let lines = lexer::logical_lines(source).map_err(|problem| vec![problem])?;
    let program = parser::parse_program(source, &lines)?;
    let mut by_index = HashMap::new();
    for nme_line in &program.nme_lines {
        by_index.insert(nme_line.line_index, nme_line);
    }

    let mut out = String::from(PREAMBLE);
    let mut open_braces = 1usize; // the `main` body
    let mut declared = HashMap::new();
    let mut problems = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if let Some(nme_line) = by_index.get(&index) {
            // `else`/`else if` lines emit their own closing `}` before the
            // next branch, so the generic brace closing must not run first.
            let is_branch = matches!(
                nme_line.stmt,
                NmeStmt::ElseIf { .. } | NmeStmt::Else { .. }
            );
            if !is_branch {
                let total_depth = line.indent + nme_line.virtual_indent;
                close_braces(&mut out, &mut open_braces, total_depth + 1);
            }
            match &nme_line.stmt {
                NmeStmt::Say { value } => {
                    if let Err(diag) = emit_say(&mut out, value, source, &declared) {
                        problems.push(diag);
                    }
                }
                NmeStmt::Set { target, value } => {
                    if let Err(diag) =
                        emit_set(&mut out, &mut declared, target, value, source)
                    {
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
                    ) {
                        problems.push(diag);
                    }
                }
                NmeStmt::While {
                    condition,
                    inline: None,
                } => match check_condition(condition, source, nme_line.span, &declared) {
                    Ok(condition_text) => {
                        out.push_str(&format!("while ({condition_text}) {{\n"));
                        open_braces += 1;
                    }
                    Err(diag) => problems.push(diag),
                },
                NmeStmt::When {
                    condition,
                    inline: None,
                } => match check_condition(condition, source, nme_line.span, &declared) {
                    Ok(condition_text) => {
                        out.push_str(&format!("if ({condition_text}) {{\n"));
                        open_braces += 1;
                    }
                    Err(diag) => problems.push(diag),
                },
                NmeStmt::Break => out.push_str("break;\n"),
                NmeStmt::Times { count, inline } => {
                    let count_text = code_text(count, source);
                    match check_expr(count_text, nme_line.span, &declared) {
                        Ok((lowered, ExprType::Int)) => {
                            let header = format!(
                                "for (int _nme_i = 0; _nme_i < {lowered}; _nme_i++)"
                            );
                            match inline {
                                Some(InlineStmt::Nme(inner)) => {
                                    match lower_inline(inner, source, &declared) {
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
                                }
                            }
                        }
                        Err(diag) => problems.push(diag),
                        Ok(_) => problems.push(not_supported(
                            "this repeat count",
                            nme_line.span,
                        )),
                    }
                }
                NmeStmt::ElseIf {
                    condition,
                    inline: None,
                } => match check_condition(condition, source, nme_line.span, &declared) {
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
            let text = &source[line.span.start..line.span.end];
            let trimmed = text.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                out.push_str(text);
                out.push('\n');
                continue;
            }
            if let Some(diag) = emit_python_line(
                &mut out,
                &mut open_braces,
                &mut declared,
                &line.tokens,
                text,
                source,
            ) {
                problems.push(diag);
            }
        }
    }
    close_braces(&mut out, &mut open_braces, 1);
    out.push_str("return 0;\n");
    close_braces(&mut out, &mut open_braces, 0);

    if problems.is_empty() {
        Ok(out)
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
) -> Result<String, Diagnostic> {
    match stmt {
        NmeStmt::Say { value } => {
            let mut out = String::new();
            emit_say(&mut out, value, source, declared)?;
            Ok(out.trim_end().to_string())
        }
        _ => Err(not_supported("this inline statement", Span::new(0, 0))),
    }
}

fn close_braces(out: &mut String, open: &mut usize, target: usize) {
    while *open > target {
        out.push_str("}\n");
        *open -= 1;
    }
}

/// Validates and lowers a Python expression to C for the native core,
/// returning the C text and its static type. Integer expressions pass through
/// unchanged; string expressions lower to a name, a literal, or one `+`
/// concatenation through the small runtime helper.
fn check_expr(
    text: &str,
    span: Span,
    declared: &HashMap<String, VarType>,
) -> Result<(String, ExprType), Diagnostic> {
    let expr = Expr::parse(text, "<native>")
        .map_err(|_| not_supported("this expression", span))?;
    lower_expr(&expr, span, declared)
}

fn lower_expr(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
) -> Result<(String, ExprType), Diagnostic> {
    match expr {
        Expr::Constant(constant) => match &constant.value {
            Constant::Int(value) => Ok((format!("{value}"), ExprType::Int)),
            Constant::Str(string) => {
                let escaped = string.replace('\\', "\\\\").replace('"', "\\\"");
                Ok((format!("\"{escaped}\""), ExprType::Str))
            }
            _ => Err(not_supported("this constant", span)),
        },
        Expr::Name(name) => {
            let id = name.id.as_str();
            if is_c_keyword(id) {
                return Err(not_supported(
                    &format!("a variable named `{id}` (C keyword)"),
                    span,
                ));
            }
            match declared.get(id) {
                Some(VarType::Str) => Ok((id.to_string(), ExprType::Str)),
                Some(VarType::Int) | None => Ok((id.to_string(), ExprType::Int)),
            }
        }
        Expr::BinOp(binop) => match binop.op {
            Operator::Add if binop_operands_are_string(binop, span, declared)? => {
                // One binary `+` over string operands; the operands must be a
                // name or a literal so the shared runtime buffer is safe.
                let left = string_operand(&binop.left, span, declared)?;
                let right = string_operand(&binop.right, span, declared)?;
                Ok((format!("nme_cat({left}, {right})"), ExprType::Str))
            }
            Operator::Add | Operator::Sub | Operator::Mult => {
                let left = int_operand(&binop.left, span, declared)?;
                let right = int_operand(&binop.right, span, declared)?;
                Ok((
                    format!("({left} {} {right})", operator_text(&binop.op)),
                    ExprType::Int,
                ))
            }
            _ => Err(not_supported("this operator", span)),
        },
        Expr::UnaryOp(unary) => {
            if matches!(unary.op, UnaryOp::USub | UnaryOp::UAdd) {
                let operand = int_operand(&unary.operand, span, declared)?;
                Ok((
                    format!(
                        "({}{operand})",
                        if matches!(unary.op, UnaryOp::USub) {
                            "-"
                        } else {
                            ""
                        }
                    ),
                    ExprType::Int,
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
                let (argument, kind) = lower_expr(&call.args[0], span, declared)?;
                if kind == ExprType::Str {
                    return Ok((format!("strlen({argument})"), ExprType::Int));
                }
            }
            if is_c_keyword(callee.id.as_str()) {
                return Err(not_supported(
                    &format!("a call to `{}` (C keyword)", callee.id),
                    span,
                ));
            }
            let mut args = Vec::new();
            for argument in &call.args {
                let (text, kind) = lower_expr(argument, span, declared)?;
                if kind != ExprType::Int {
                    return Err(not_supported("a string argument to a function", span));
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
) -> Result<bool, Diagnostic> {
    let left_kind = operand_kind(&binop.left, span, declared)?;
    let right_kind = operand_kind(&binop.right, span, declared)?;
    match (left_kind, right_kind) {
        (ExprType::Str, ExprType::Str) => Ok(true),
        (ExprType::Int, ExprType::Int) => Ok(false),
        _ => Err(not_supported("mixing numbers and text", span)),
    }
}

fn operand_kind(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
) -> Result<ExprType, Diagnostic> {
    let (_, kind) = lower_expr(expr, span, declared)?;
    Ok(kind)
}

fn string_operand(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
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
            if is_c_keyword(id) {
                return Err(not_supported(
                    &format!("a variable named `{id}` (C keyword)"),
                    span,
                ));
            }
            if matches!(declared.get(id), Some(VarType::Str)) {
                Ok(id.to_string())
            } else {
                Err(not_supported("this operand", span))
            }
        }
        _ => Err(not_supported("nested string concatenation", span)),
    }
}

fn int_operand(
    expr: &Expr,
    span: Span,
    declared: &HashMap<String, VarType>,
) -> Result<String, Diagnostic> {
    let (text, kind) = lower_expr(expr, span, declared)?;
    if kind != ExprType::Int {
        return Err(not_supported("a text value in an integer expression", span));
    }
    Ok(text)
}

fn operator_text(operator: &Operator) -> &'static str {
    match operator {
        Operator::Add => "+",
        Operator::Sub => "-",
        Operator::Mult => "*",
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
) -> Result<String, Diagnostic> {
    match condition {
        Condition::Compare {
            left,
            operator,
            right,
            negated,
        } => {
            let (left, left_kind) = condition_operand(left, source, span, declared)?;
            let (right, right_kind) = condition_operand(right, source, span, declared)?;
            let comparison = match (left_kind, right_kind, operator) {
                (ExprType::Int, ExprType::Int, CompareOp::Equal) => format!("{left} == {right}"),
                (ExprType::Int, ExprType::Int, CompareOp::Greater) => format!("{left} > {right}"),
                (ExprType::Int, ExprType::Int, CompareOp::Less) => format!("{left} < {right}"),
                (ExprType::Int, ExprType::Int, CompareOp::LessOrEqual) => format!("{left} <= {right}"),
                (ExprType::Int, ExprType::Int, CompareOp::GreaterOrEqual) => {
                    format!("{left} >= {right}")
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
            let expr = Expr::parse(text, "<native>")
                .map_err(|_| not_supported("this condition", span))?;
            lower_compare(&expr, span, declared)
        }
        Condition::Truthy { value, negated } => {
            let (text, kind) = match value {
                ConditionValue::Name(name) => {
                    if is_c_keyword(name) {
                        return Err(not_supported(
                            &format!("a variable named `{name}` (C keyword)"),
                            span,
                        ));
                    }
                    (name.clone(), ExprType::Int)
                }
                ConditionValue::Python(code) => {
                    check_expr(code_text(code, source), span, declared)?
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
            if kind != ExprType::Int {
                return Err(not_supported("a text value in a truthy condition", span));
            }
            Ok(if *negated {
                format!("!({text})")
            } else {
                format!("({text})")
            })
        }
        _ => Err(not_supported("this condition", span)),
    }
}

fn condition_operand(
    value: &ConditionValue,
    source: &str,
    span: Span,
    declared: &HashMap<String, VarType>,
) -> Result<(String, ExprType), Diagnostic> {
    match value {
        ConditionValue::Python(code) => check_expr(code_text(code, source), span, declared),
        ConditionValue::Name(name) => {
            if is_c_keyword(name) {
                return Err(not_supported(
                    &format!("a variable named `{name}` (C keyword)"),
                    span,
                ));
            }
            let kind = match declared.get(name) {
                Some(VarType::Str) => ExprType::Str,
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
) -> Result<String, Diagnostic> {
    let Expr::Compare(compare) = expr else {
        return Err(not_supported("this condition", span));
    };
    if compare.ops.len() != 1 || compare.comparators.len() != 1 {
        return Err(not_supported("this condition", span));
    }
    let (left, left_kind) = lower_expr(&compare.left, span, declared)?;
    let (right, right_kind) = lower_expr(&compare.comparators[0], span, declared)?;
    match (left_kind, right_kind) {
        (ExprType::Int, ExprType::Int) => {
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
        _ => Err(not_supported("mixing numbers and text in a condition", span)),
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
) -> Result<(), Diagnostic> {
    match value {
        Value::Python(code) => {
            let text = code_text(code, source);
            let span = code_span(code);
            let (lowered, kind) = check_expr(text, span, declared)?;
            match kind {
                ExprType::Int => {
                    out.push_str(&format!("printf(\"%d\\n\", {lowered});\n"));
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
                        return Err(not_supported("a variable inside a sentence", span_of_value(value)));
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
    }
}

fn emit_set(
    out: &mut String,
    declared: &mut HashMap<String, VarType>,
    target: &str,
    value: &Value,
    source: &str,
) -> Result<(), Diagnostic> {
    if is_c_keyword(target) {
        return Err(not_supported(
            &format!("a variable named `{target}` (C keyword)"),
            span_of_value(value),
        ));
    }
    match value {
        Value::Python(code) => {
            let text = code_text(code, source);
            let span = code_span(code);
            let (lowered, kind) = check_expr(text, span, declared)?;
            let is_new = !declared.contains_key(target);
            match kind {
                ExprType::Int => {
                    let prefix = if is_new { "int " } else { "" };
                    declared.insert(target.to_string(), VarType::Int);
                    out.push_str(&format!("{prefix}{target} = {lowered};\n"));
                    Ok(())
                }
                ExprType::Str => {
                    declared.insert(target.to_string(), VarType::Str);
                    if is_new {
                        out.push_str(&format!("char {target}[8192] = {lowered};\n"));
                    } else {
                        out.push_str(&format!("strcpy({target}, {lowered});\n"));
                    }
                    Ok(())
                }
            }
        }
        Value::Text(_) | Value::Literal(_) | Value::RandomInteger { .. } | Value::RandomChoice { .. } => {
            Err(not_supported("this value", span_of_value(value)))
        }
    }
}

fn emit_update(
    out: &mut String,
    declared: &mut HashMap<String, VarType>,
    target: &str,
    amount: &nme_core::syntax::Code,
    operation: nme_core::syntax::UpdateOp,
    source: &str,
) -> Result<(), Diagnostic> {
    if is_c_keyword(target) {
        return Err(not_supported(
            &format!("a variable named `{target}` (C keyword)"),
            Span::new(0, 0),
        ));
    }
    let amount_text = code_text(amount, source);
    let (lowered, kind) = check_expr(amount_text, Span::new(0, 0), declared)?;
    if kind != ExprType::Int {
        return Err(not_supported("a text value in a value change", Span::new(0, 0)));
    }
    let is_new = !declared.contains_key(target);
    if is_new {
        declared.insert(target.to_string(), VarType::Int);
    }
    let prefix = if is_new { "int " } else { "" };
    let op = match operation {
        nme_core::syntax::UpdateOp::Add => "+=",
        nme_core::syntax::UpdateOp::Subtract => "-=",
    };
    out.push_str(&format!("{prefix}{target} {op} {lowered};\n"));
    Ok(())
}

/// A Python line is accepted when it is a simple integer or string-literal
/// assignment, an integer `return`, or a `def` header over integer
/// parameters. `def` opens a C function body.
fn emit_python_line(
    out: &mut String,
    open_braces: &mut usize,
    declared: &mut HashMap<String, VarType>,
    tokens: &[lexer::Token],
    text: &str,
    _source: &str,
) -> Option<Diagnostic> {
    let span = Span::new(0, text.len());
    match tokens.first().map(|token| &token.tok) {
        Some(rustpython_parser::Tok::Def) => {
            let name = match tokens.get(1).map(|token| &token.tok) {
                Some(rustpython_parser::Tok::Name { name }) => name.clone(),
                _ => return Some(not_supported("this function header", span)),
            };
            if is_c_keyword(&name) {
                return Some(not_supported(
                    &format!("a function named `{name}` (C keyword)"),
                    span,
                ));
            }
            let parameters = tokens
                .iter()
                .filter_map(|token| match &token.tok {
                    rustpython_parser::Tok::Name { name } => Some(name.clone()),
                    _ => None,
                })
                .skip(1) // the function name itself
                .collect::<Vec<_>>();
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
            match check_expr(expression, span, declared) {
                Ok((lowered, ExprType::Int)) => {
                    out.push_str(&format!("return {lowered};\n"));
                    None
                }
                Ok((_, ExprType::Str)) => {
                    Some(not_supported("returning text from a function", span))
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
            if is_c_keyword(&name) {
                return Some(not_supported(
                    &format!("a variable named `{name}` (C keyword)"),
                    span,
                ));
            }
            let expression = text
                .split_once('=')
                .map(|(_, right)| right.trim())
                .unwrap_or(text);
            match check_expr(expression, span, declared) {
                Ok((lowered, ExprType::Int)) => {
                    let is_new = !declared.contains_key(&name);
                    if is_new {
                        declared.insert(name.clone(), VarType::Int);
                    }
                    let prefix = if is_new { "int " } else { "" };
                    out.push_str(&format!("{prefix}{name} = {lowered};\n"));
                    None
                }
                Ok((lowered, ExprType::Str)) => {
                    let is_new = !declared.contains_key(&name);
                    declared.insert(name.clone(), VarType::Str);
                    if is_new {
                        out.push_str(&format!("char {name}[8192] = {lowered};\n"));
                    } else {
                        out.push_str(&format!("strcpy({name}, {lowered});\n"));
                    }
                    None
                }
                Err(diag) => Some(diag),
            }
        }
        _ => Some(not_supported("this Python line", span)),
    }
}

/// C reserved words that a Python identifier must not collide with in the
/// generated C. The native backend rejects them instead of silently
/// renaming, so the C artifact always matches the NME source.
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

fn not_supported(what: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        &format!("the native backend does not support {what} yet"),
        &format!("네이티브 백엔드는 아직 {what}을(를) 지원하지 않습니다"),
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
