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
//! - `say`/`show`/`말해` of an integer expression or a plain string literal;
//! - `set x to <int>` / `x은 <int>` / `x = <int>` integer assignments
//!   (including the Python-looking `x = <int>` form);
//! - value changes `x add N` / `x = x + N` / `점수에 N 더해`;
//! - `while`/`if` blocks over integer comparisons, closed by `end`/`끝`;
//! - `break` inside a loop.
//!
//! Anything outside this core is rejected with a clear bilingual diagnostic;
//! it is never silently miscompiled. The rest of NME keeps running on CPython.

use std::collections::{HashMap, HashSet};

use nme_core::diagnostics::{Diagnostic, DiagnosticCode, Span};
use nme_core::syntax::{CompareOp, Condition, ConditionValue, NmeStmt, Value};
use nme_core::{lexer, parser};

use rustpython_parser::ast::{CmpOp, Constant, Expr, Operator, UnaryOp};
use rustpython_parser::Parse as _;

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

    let mut out = String::from("#include <stdio.h>\nint main(void) {\n");
    let mut open_braces = 1usize; // the `main` body
    let mut declared = HashSet::new();
    let mut problems = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if let Some(nme_line) = by_index.get(&index) {
            let total_depth = line.indent + nme_line.virtual_indent;
            close_braces(&mut out, &mut open_braces, total_depth + 1);
            match &nme_line.stmt {
                NmeStmt::Say { value } => {
                    if let Err(diag) = emit_say(&mut out, value, source) {
                        problems.push(diag);
                    }
                }
                NmeStmt::Set { target, value } => {
                    if let Err(diag) = emit_set(&mut out, &mut declared, target, value, source) {
                        problems.push(diag);
                    }
                }
                NmeStmt::Update {
                    target,
                    amount,
                    operation,
                } => {
                    if let Err(diag) = emit_update(&mut out, &mut declared, target, amount, *operation, source) {
                        problems.push(diag);
                    }
                }
                NmeStmt::While {
                    condition,
                    inline: None,
                } => match check_condition(condition, source, nme_line.span) {
                    Ok(condition_text) => {
                        out.push_str(&format!("while ({condition_text}) {{\n"));
                        open_braces += 1;
                    }
                    Err(diag) => problems.push(diag),
                },
                NmeStmt::When {
                    condition,
                    inline: None,
                } => match check_condition(condition, source, nme_line.span) {
                    Ok(condition_text) => {
                        out.push_str(&format!("if ({condition_text}) {{\n"));
                        open_braces += 1;
                    }
                    Err(diag) => problems.push(diag),
                },
                NmeStmt::Break => out.push_str("break;\n"),
                NmeStmt::End => {}
                other => problems.push(unsupported_statement(other, nme_line.span)),
            }
        } else {
            // A non-NME line: blank, comment, or a Python integer assignment.
            let total_depth = line.indent + program.virtual_indents[index];
            close_braces(&mut out, &mut open_braces, total_depth + 1);
            let text = &source[line.span.start..line.span.end];
            let trimmed = text.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                out.push_str(text);
                out.push('\n');
                continue;
            }
            if let Some(diag) = emit_python_assignment(&mut out, &mut declared, &line.tokens, text, source) {
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

fn close_braces(out: &mut String, open: &mut usize, target: usize) {
    while *open > target {
        out.push_str("}\n");
        *open -= 1;
    }
}

/// Validates that a Python expression belongs to the native core: integer
/// literals, names, `+ - *`, unary minus, and parentheses only.
fn check_int_expression(text: &str, _source: &str, span: Span) -> Result<String, Diagnostic> {
    let expr = Expr::parse(text, "<native>")
        .map_err(|_| not_supported("this expression", span))?;
    validate_int_expr(&expr, span).map(|_| text.to_string())
}

fn validate_int_expr(expr: &Expr, span: Span) -> Result<(), Diagnostic> {
    match expr {
        Expr::Constant(constant) => {
            if matches!(constant.value, Constant::Int(_)) {
                Ok(())
            } else {
                Err(not_supported("string or other constant", span))
            }
        }
        Expr::Name(_) => Ok(()),
        Expr::BinOp(binop) => {
            if matches!(
                binop.op,
                Operator::Add | Operator::Sub | Operator::Mult
            ) {
                validate_int_expr(&binop.left, span)?;
                validate_int_expr(&binop.right, span)
            } else {
                Err(not_supported("this operator", span))
            }
        }
        Expr::UnaryOp(unary) => {
            if matches!(unary.op, UnaryOp::USub | UnaryOp::UAdd) {
                validate_int_expr(&unary.operand, span)
            } else {
                Err(not_supported("this operator", span))
            }
        }
        _ => Err(not_supported("this expression", span)),
    }
}

/// Validates a condition: a comparison of two integer expressions, optionally
/// negated.
fn check_condition(condition: &Condition, source: &str, span: Span) -> Result<String, Diagnostic> {
    match condition {
        Condition::Compare {
            left,
            operator,
            right,
            negated,
        } => {
            let left = condition_value(left, source, span)?;
            let right = condition_value(right, source, span)?;
            let op = match operator {
                CompareOp::Equal => "==",
                CompareOp::Greater => ">",
                CompareOp::Less => "<",
            };
            let comparison = format!("{left} {op} {right}");
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
            validate_compare(&expr, span)?;
            Ok(text.to_string())
        }
        _ => Err(not_supported("this condition", span)),
    }
}

fn condition_value(value: &ConditionValue, source: &str, span: Span) -> Result<String, Diagnostic> {
    match value {
        ConditionValue::Python(code) => check_int_expression(code_text(code, source), source, span),
        ConditionValue::Name(name) => Ok(name.clone()),
        ConditionValue::Literal(_) | ConditionValue::Text(_) => {
            Err(not_supported("non-integer condition value", span))
        }
    }
}

fn validate_compare(expr: &Expr, span: Span) -> Result<(), Diagnostic> {
    match expr {
        Expr::Compare(compare) if compare.ops.len() == 1 => {
            if matches!(
                compare.ops[0],
                CmpOp::Lt | CmpOp::LtE | CmpOp::Gt | CmpOp::GtE | CmpOp::Eq | CmpOp::NotEq
            ) {
                validate_int_expr(&compare.left, span)?;
                validate_int_expr(&compare.comparators[0], span)
            } else {
                Err(not_supported("this comparison", span))
            }
        }
        _ => Err(not_supported("this condition", span)),
    }
}

fn code_text<'a>(code: &nme_core::syntax::Code, source: &'a str) -> &'a str {
    match code {
        nme_core::syntax::Code::Source(span) => &source[span.start..span.end],
    }
}

fn emit_say(out: &mut String, value: &Value, source: &str) -> Result<(), Diagnostic> {
    match value {
        Value::Python(code) => {
            let text = code_text(code, source);
            let span = code_span(code);
            let expr = Expr::parse(text, "<native>")
                .map_err(|_| not_supported("this expression", span))?;
            match &expr {
                Expr::Constant(constant) => match &constant.value {
                    Constant::Str(string) => {
                        let escaped = string.replace('\\', "\\\\").replace('"', "\\\"");
                        out.push_str(&format!("printf(\"%s\\n\", \"{escaped}\");\n"));
                        Ok(())
                    }
                    Constant::Int(_) => {
                        out.push_str(&format!("printf(\"%d\\n\", {text});\n"));
                        Ok(())
                    }
                    _ => Err(not_supported("this constant", span)),
                },
                _ => {
                    check_int_expression(text, source, span)?;
                    out.push_str(&format!("printf(\"%d\\n\", {text});\n"));
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
                        return Err(not_supported("a variable inside a sentence", template_span(template)));
                    }
                }
            }
            let escaped = literal.replace('\\', "\\\\").replace('"', "\\\"");
            out.push_str(&format!("printf(\"%s\\n\", \"{escaped}\");\n"));
            Ok(())
        }
        Value::Literal(_) => Err(not_supported("boolean/null output", code_span_for_value(value))),
        Value::RandomInteger { .. } | Value::RandomChoice { .. } => {
            Err(not_supported("random values", code_span_for_value(value)))
        }
    }
}

fn emit_set(
    out: &mut String,
    declared: &mut HashSet<String>,
    target: &str,
    value: &Value,
    source: &str,
) -> Result<(), Diagnostic> {
    match value {
        Value::Python(code) => {
            let text = code_text(code, source);
            check_int_expression(text, source, code_span(code))?;
            let prefix = if declared.insert(target.to_string()) {
                "int "
            } else {
                ""
            };
            out.push_str(&format!("{prefix}{target} = {text};\n"));
            Ok(())
        }
        Value::Text(_) | Value::Literal(_) | Value::RandomInteger { .. } | Value::RandomChoice { .. } => {
            Err(not_supported("non-integer value", code_span_for_value(value)))
        }
    }
}

fn emit_update(
    out: &mut String,
    declared: &mut HashSet<String>,
    target: &str,
    amount: &nme_core::syntax::Code,
    operation: nme_core::syntax::UpdateOp,
    source: &str,
) -> Result<(), Diagnostic> {
    let amount_text = code_text(amount, source);
    check_int_expression(amount_text, source, code_span(amount))?;
    let prefix = if declared.insert(target.to_string()) {
        "int "
    } else {
        ""
    };
    let op = match operation {
        nme_core::syntax::UpdateOp::Add => "+=",
        nme_core::syntax::UpdateOp::Subtract => "-=",
    };
    out.push_str(&format!("{prefix}{target} {op} {amount_text};\n"));
    Ok(())
}

/// A Python line is accepted when it is a simple integer assignment such as
/// `score = score + 1` or `score = 0`.
fn emit_python_assignment(
    out: &mut String,
    declared: &mut HashSet<String>,
    tokens: &[lexer::Token],
    text: &str,
    source: &str,
) -> Option<Diagnostic> {
    let name = match tokens.first().map(|token| &token.tok) {
        Some(rustpython_parser::Tok::Name { name }) => name.clone(),
        _ => return Some(not_supported("this Python line", Span::new(0, text.len()))),
    };
    if !matches!(tokens.get(1).map(|token| &token.tok), Some(rustpython_parser::Tok::Equal)) {
        return Some(not_supported("this Python line", Span::new(0, text.len())));
    }
    let expression = text.split_once('=').map(|(_, right)| right.trim()).unwrap_or(text);
    let span = Span::new(0, text.len());
    match check_int_expression(expression, source, span) {
        Ok(expression) => {
            let prefix = if declared.insert(name.clone()) {
                "int "
            } else {
                ""
            };
            out.push_str(&format!("{prefix}{name} = {expression};\n"));
            None
        }
        Err(diag) => Some(diag),
    }
}

fn not_supported(what: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        &format!("the native backend does not support {what} yet"),
        &format!("네이티브 백엔드는 아직 {what}을(를) 지원하지 않습니다"),
        span,
    )
    .with_bilingual_hint(
        "use only the documented native core: integer values, while/if over comparisons, and say",
        "문서에 있는 네이티브 코어만 쓰세요: 정수 값, 비교 조건의 while/if, say",
    )
}

fn unsupported_statement(stmt: &NmeStmt, span: Span) -> Diagnostic {
    let what = match stmt {
        NmeStmt::Ask { .. } => "input (ask)",
        NmeStmt::Times { .. } => "repeat blocks",
        NmeStmt::ElseIf { .. } | NmeStmt::Else { .. } => "else branches",
        NmeStmt::UseModule { .. } => "bundled modules",
        NmeStmt::FileRead { .. } | NmeStmt::FileWrite { .. } => "file operations",
        NmeStmt::ModuleImport { .. } => "module imports",
        _ => "this statement",
    };
    not_supported(what, span)
}

fn code_span(code: &nme_core::syntax::Code) -> Span {
    match code {
        nme_core::syntax::Code::Source(span) => *span,
    }
}

fn code_span_for_value(value: &Value) -> Span {
    match value {
        Value::Python(code) => code_span(code),
        _ => Span::new(0, 0),
    }
}

fn template_span(_template: &nme_core::syntax::TextTemplate) -> Span {
    Span::new(0, 0)
}
