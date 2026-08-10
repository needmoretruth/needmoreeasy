//! Safe Python-to-NME conversion.
//!
//! Conversion is deliberately semantics-preserving: lines with a clear NME
//! equivalent are rewritten and every other valid Python line remains
//! advanced syntax. Because all three levels may coexist, the result is still
//! a complete NME program rather than a partial or lossy translation.

use std::collections::HashSet;

use rustpython_parser::{parse as parse_python, Mode, Tok};

use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::{self, LogicalLine, Token};
use crate::lower::{apply_edits, Edit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxLevel {
    Advanced,
    Beginner,
    Sentence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Korean,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conversion {
    pub source: String,
    pub changed_lines: usize,
}

/// Convert valid Python into the requested NME surface level.
///
/// Unsupported constructs remain byte-identical advanced Python. This is the
/// only lossless behavior possible because advanced Python is itself one of
/// NME's three syntax levels.
pub fn convert_python(
    source: &str,
    level: SyntaxLevel,
    language: Language,
) -> Result<Conversion, Vec<Diagnostic>> {
    if let Err(error) = parse_python(source, Mode::Module, "<python>") {
        let start = usize::from(error.offset).min(source.len().saturating_sub(1));
        return Err(vec![Diagnostic::new(
            format!("the Python source is not valid: {}", error.error),
            Span::new(start, (start + 1).min(source.len())),
        )
        .with_hint("fix this Python error before converting it to NME")]);
    }

    if level == SyntaxLevel::Advanced {
        return Ok(Conversion {
            source: source.to_string(),
            changed_lines: 0,
        });
    }

    let lines = lexer::logical_lines(source).map_err(|problem| vec![problem])?;
    let known_names = crate::parser::discover_python_bindings(&lines);
    let edits: Vec<Edit> = lines
        .iter()
        .filter_map(|line| convert_line(source, line, level, language, &known_names))
        .collect();
    let changed_lines = edits.len();
    Ok(Conversion {
        source: apply_edits(source, &edits),
        changed_lines,
    })
}

fn convert_line(
    source: &str,
    line: &LogicalLine,
    level: SyntaxLevel,
    language: Language,
    known_names: &HashSet<String>,
) -> Option<Edit> {
    let replacement = convert_print(source, &line.tokens, level, language, known_names)
        .or_else(|| convert_input(source, &line.tokens, level, language, known_names))
        .or_else(|| convert_range_loop(source, &line.tokens, level, language))
        .or_else(|| convert_condition(source, &line.tokens, level, language))
        .or_else(|| convert_random_import(&line.tokens, level, language))
        .or_else(|| convert_assignment(source, &line.tokens, level, language, known_names))?;
    Some(Edit {
        span: line.span,
        replacement,
    })
}

fn convert_print(
    source: &str,
    tokens: &[Token],
    level: SyntaxLevel,
    language: Language,
    known_names: &HashSet<String>,
) -> Option<String> {
    if tokens.len() < 4
        || !name_is(&tokens[0], "print")
        || !matches!(tokens[1].tok, Tok::Lpar)
        || !matches!(tokens[tokens.len() - 1].tok, Tok::Rpar)
    {
        return None;
    }
    let argument = &tokens[2..tokens.len() - 1];
    if argument.is_empty() || has_top_level_comma(argument) {
        return None;
    }
    if level == SyntaxLevel::Sentence
        && matches!(argument, [Token { tok: Tok::Name { name }, .. }] if !known_names.contains(name.as_str()))
    {
        return None;
    }
    let value = sentence_value(source, argument, level);
    Some(match (level, language) {
        (SyntaxLevel::Beginner, Language::English) => format!("say {value}"),
        (SyntaxLevel::Beginner, Language::Korean) => format!("말해 {value}"),
        (SyntaxLevel::Sentence, Language::English) => format!("show {value}"),
        (SyntaxLevel::Sentence, Language::Korean) => format!("보여줘 {value}"),
        (SyntaxLevel::Advanced, _) => return None,
    })
}

fn convert_input(
    source: &str,
    tokens: &[Token],
    level: SyntaxLevel,
    language: Language,
    known_names: &HashSet<String>,
) -> Option<String> {
    let (target, kind, call) = input_assignment(tokens)?;
    let prompt_tokens = input_arguments(call)?;
    let prompt = if prompt_tokens.is_empty() {
        None
    } else {
        if level == SyntaxLevel::Sentence
            && matches!(prompt_tokens, [Token { tok: Tok::Name { name }, .. }] if !known_names.contains(name.as_str()))
        {
            return None;
        }
        Some(sentence_value(source, prompt_tokens, level))
    };
    let prompt_with_beginner_comma = prompt.as_ref().map(|value| format!(", {value}"));
    let prompt_with_space = prompt.as_ref().map(|value| format!(" {value}"));

    Some(match (level, language, kind) {
        (SyntaxLevel::Beginner, Language::English, InputFlavor::Text) => {
            format!(
                "ask {target}{}",
                prompt_with_beginner_comma.unwrap_or_default()
            )
        }
        (SyntaxLevel::Beginner, Language::Korean, InputFlavor::Text) => {
            format!(
                "물어봐 {target}{}",
                prompt_with_beginner_comma.unwrap_or_default()
            )
        }
        (SyntaxLevel::Sentence, Language::English, InputFlavor::Text) => {
            format!("ask {target}{}", prompt_with_space.unwrap_or_default())
        }
        (SyntaxLevel::Sentence, Language::English, InputFlavor::Number) => {
            format!(
                "ask number {target}{}",
                prompt_with_space.unwrap_or_default()
            )
        }
        (SyntaxLevel::Sentence, Language::Korean, InputFlavor::Text) => {
            format!("{target}을 물어봐{}", prompt_with_space.unwrap_or_default())
        }
        (SyntaxLevel::Sentence, Language::Korean, InputFlavor::Number) => {
            format!(
                "{target}을 숫자로 물어봐{}",
                prompt_with_space.unwrap_or_default()
            )
        }
        // Numeric input is a sentence feature; beginner conversion keeps this
        // advanced Python line rather than changing its semantics.
        (SyntaxLevel::Beginner, _, InputFlavor::Number) | (SyntaxLevel::Advanced, _, _) => {
            return None;
        }
    })
}

#[derive(Clone, Copy)]
enum InputFlavor {
    Text,
    Number,
}

fn input_assignment(tokens: &[Token]) -> Option<(&str, InputFlavor, &[Token])> {
    let [Token {
        tok: Tok::Name { name: target },
        ..
    }, Token {
        tok: Tok::Equal, ..
    }, rest @ ..] = tokens
    else {
        return None;
    };
    if is_call(rest, "input") {
        return Some((target, InputFlavor::Text, rest));
    }
    if rest.len() >= 4
        && name_is(&rest[0], "int")
        && matches!(rest[1].tok, Tok::Lpar)
        && matches!(rest[rest.len() - 1].tok, Tok::Rpar)
        && is_call(&rest[2..rest.len() - 1], "input")
    {
        return Some((target, InputFlavor::Number, &rest[2..rest.len() - 1]));
    }
    None
}

fn is_call(tokens: &[Token], name: &str) -> bool {
    tokens.len() >= 3
        && name_is(&tokens[0], name)
        && matches!(tokens[1].tok, Tok::Lpar)
        && matches!(tokens[tokens.len() - 1].tok, Tok::Rpar)
}

fn input_arguments(tokens: &[Token]) -> Option<&[Token]> {
    is_call(tokens, "input").then(|| &tokens[2..tokens.len() - 1])
}

fn convert_range_loop(
    source: &str,
    tokens: &[Token],
    level: SyntaxLevel,
    language: Language,
) -> Option<String> {
    if tokens.len() < 8
        || !matches!(tokens[0].tok, Tok::For)
        || !name_is(&tokens[1], "_")
        || !matches!(tokens[2].tok, Tok::In)
        || !name_is(&tokens[3], "range")
        || !matches!(tokens[4].tok, Tok::Lpar)
        || !matches!(tokens[tokens.len() - 2].tok, Tok::Rpar)
        || !matches!(tokens[tokens.len() - 1].tok, Tok::Colon)
    {
        return None;
    }
    let count_tokens = &tokens[5..tokens.len() - 2];
    if count_tokens.is_empty() || has_top_level_comma(count_tokens) {
        return None;
    }
    let count = text_of(source, count_tokens);
    Some(match (level, language) {
        (SyntaxLevel::Beginner, Language::English) => format!("{count} times:"),
        (SyntaxLevel::Beginner, Language::Korean) => format!("{count}번:"),
        (SyntaxLevel::Sentence, Language::English) => format!("repeat {count} times"),
        (SyntaxLevel::Sentence, Language::Korean) => format!("{count}번 반복해"),
        (SyntaxLevel::Advanced, _) => return None,
    })
}

fn convert_condition(
    source: &str,
    tokens: &[Token],
    level: SyntaxLevel,
    language: Language,
) -> Option<String> {
    if tokens.len() < 3
        || !matches!(tokens[0].tok, Tok::If)
        || !matches!(tokens[tokens.len() - 1].tok, Tok::Colon)
    {
        return None;
    }
    let condition = text_of(source, &tokens[1..tokens.len() - 1]);
    Some(match (level, language) {
        (SyntaxLevel::Beginner, Language::English) => format!("when {condition}:"),
        (SyntaxLevel::Beginner, Language::Korean) => format!("만약 {condition}:"),
        (SyntaxLevel::Sentence, Language::English) => format!("if {condition}"),
        (SyntaxLevel::Sentence, Language::Korean) => format!("만약에 {condition}"),
        (SyntaxLevel::Advanced, _) => return None,
    })
}

fn convert_random_import(
    tokens: &[Token],
    level: SyntaxLevel,
    language: Language,
) -> Option<String> {
    if !matches!(tokens, [Token { tok: Tok::Import, .. }, token] if name_is(token, "random")) {
        return None;
    }
    Some(match (level, language) {
        (SyntaxLevel::Beginner, Language::English) => "use random".to_string(),
        (SyntaxLevel::Beginner, Language::Korean) => "랜덤 사용".to_string(),
        (SyntaxLevel::Sentence, Language::English) => "use random latest".to_string(),
        (SyntaxLevel::Sentence, Language::Korean) => "랜덤 사용 최신".to_string(),
        (SyntaxLevel::Advanced, _) => return None,
    })
}

fn convert_assignment(
    source: &str,
    tokens: &[Token],
    level: SyntaxLevel,
    language: Language,
    known_names: &HashSet<String>,
) -> Option<String> {
    if level != SyntaxLevel::Sentence {
        return None;
    }
    let [Token {
        tok: Tok::Name { name: target },
        ..
    }, Token {
        tok: Tok::Equal, ..
    }, value @ ..] = tokens
    else {
        return None;
    };
    if value.is_empty() {
        return None;
    }
    if matches!(value, [Token { tok: Tok::Name { name }, .. }] if !known_names.contains(name.as_str()))
    {
        return None;
    }
    let value = sentence_value(source, value, level);
    Some(match language {
        Language::English => format!("set {target} to {value}"),
        Language::Korean => format!("{target}은 {value}"),
    })
}

fn sentence_value(source: &str, tokens: &[Token], _level: SyntaxLevel) -> String {
    text_of(source, tokens).to_string()
}

fn has_top_level_comma(tokens: &[Token]) -> bool {
    let mut depth = 0usize;
    for token in tokens {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Comma if depth == 0 => return true,
            _ => {}
        }
    }
    false
}

fn name_is(token: &Token, expected: &str) -> bool {
    matches!(&token.tok, Tok::Name { name } if name == expected)
}

fn text_of<'a>(source: &'a str, tokens: &[Token]) -> &'a str {
    &source[tokens[0].span.start..tokens[tokens.len() - 1].span.end]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transpile;

    fn converted(source: &str, level: SyntaxLevel, language: Language) -> Conversion {
        convert_python(source, level, language).unwrap()
    }

    #[test]
    fn advanced_conversion_is_byte_identical() {
        let source = "name = input(\"Name? \" )\nprint(name)\n";
        assert_eq!(
            converted(source, SyntaxLevel::Advanced, Language::Korean),
            Conversion {
                source: source.to_string(),
                changed_lines: 0,
            }
        );
    }

    #[test]
    fn converts_the_safe_beginner_subset_in_both_languages() {
        let source = concat!(
            "name = input(\"Name? \")\n",
            "if name:\n",
            "    print(name)\n",
            "for _ in range(2):\n",
            "    print(\"Hi\")\n",
        );
        assert_eq!(
            converted(source, SyntaxLevel::Beginner, Language::English).source,
            concat!(
                "ask name, \"Name? \"\n",
                "when name:\n",
                "    say name\n",
                "2 times:\n",
                "    say \"Hi\"\n",
            )
        );
        assert_eq!(
            converted(source, SyntaxLevel::Beginner, Language::Korean).source,
            concat!(
                "물어봐 name, \"Name? \"\n",
                "만약 name:\n",
                "    말해 name\n",
                "2번:\n",
                "    말해 \"Hi\"\n",
            )
        );
    }

    #[test]
    fn converts_to_colon_free_sentence_structure() {
        let source = concat!(
            "name = input(\"What is your name?\")\n",
            "if name:\n",
            "    print(\"Hello, world!\")\n",
            "for _ in range(2):\n",
            "    print(name)\n",
        );
        let result = converted(source, SyntaxLevel::Sentence, Language::Korean);
        assert_eq!(
            result.source,
            concat!(
                "name을 물어봐 \"What is your name?\"\n",
                "만약에 name\n",
                "    보여줘 \"Hello, world!\"\n",
                "2번 반복해\n",
                "    보여줘 name\n",
            )
        );
        assert_eq!(result.changed_lines, 5);
        assert!(transpile(&result.source).is_ok());
    }

    #[test]
    fn sentence_conversion_handles_numeric_input_and_assignments() {
        let source = "answer = 7\nguess = int(input(\"Guess\"))\nprint(guess)\n";
        assert_eq!(
            converted(source, SyntaxLevel::Sentence, Language::English).source,
            "set answer to 7\nask number guess \"Guess\"\nshow guess\n"
        );
    }

    #[test]
    fn sentence_conversion_keeps_string_and_prompt_semantics() {
        let source = "name = \"Ada\"\nquestion = input(\"Name?\")\nprint(\"Hello name\")\n";
        let result = converted(source, SyntaxLevel::Sentence, Language::English);
        assert_eq!(
            result.source,
            "set name to \"Ada\"\nask question \"Name?\"\nshow \"Hello name\"\n"
        );
        assert_eq!(
            transpile(&result.source).unwrap(),
            "name = \"Ada\"\nquestion = input(\"Name?\")\nprint(\"Hello name\")\n"
        );
    }

    #[test]
    fn unsupported_python_stays_as_advanced_syntax() {
        let source = "while ready:\n    value += 1\nprint(value, end='')\n";
        let result = converted(source, SyntaxLevel::Sentence, Language::English);
        assert_eq!(result.source, source);
        assert_eq!(result.changed_lines, 0);
    }

    #[test]
    fn sentence_conversion_does_not_treat_unresolved_bare_names_as_text() {
        let source = "print(external)\nvalue = input(prompt)\ncopy = external\n";
        let result = converted(source, SyntaxLevel::Sentence, Language::English);
        assert_eq!(
            result.source,
            "print(external)\nset value to input(prompt)\ncopy = external\n"
        );
        assert_eq!(transpile(&result.source).unwrap(), source);
        assert_eq!(result.changed_lines, 1);
    }

    #[test]
    fn rejects_invalid_python_before_conversion() {
        let errors =
            convert_python("if broken\n", SyntaxLevel::Sentence, Language::English).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("not valid"));
        assert!(errors[0].hint.is_some());
    }
}
