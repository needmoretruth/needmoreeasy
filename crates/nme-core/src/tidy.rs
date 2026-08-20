//! Rewrites a working NME program into one spelling of one language.
//!
//! A file that runs may still be a mess: three levels on ten lines, English
//! and Korean in the same sentence, the same statement written four ways.
//! [`tidy`] reads such a file and writes it back in the canonical spelling of
//! one (level, language) pair, taking every word from `docs/syntax.md` and
//! `docs/syntax.ko.md`.
//!
//! The one promise it makes is that **the program still means what it meant**:
//! the Python out of the tidied file is compared, byte for byte, with the
//! Python out of the original, and any rewrite that would change it is thrown
//! away rather than returned. That check runs on every call, not only in the
//! tests, because a tidier that quietly changes a program is worse than no
//! tidier at all.
//!
//! Lines the parser did not recognize as NME go through the Python-to-NME
//! converter in [`crate::convert`], so a mixed file comes out tidy on both
//! halves. The line count and the indentation never change: only the
//! statement text inside each logical line's span is replaced.

use crate::convert::{
    convert_line, spans_multiple_physical_lines, Conversion, Language, SyntaxLevel,
};
use crate::diagnostics::Diagnostic;
use crate::lexer::{self, LogicalLine};
use crate::lower::{apply_edits, Edit};
use crate::render::{asks_for_a_length, Rewrite};
use crate::{parser, transpile};

/// Rewrites `source` into the canonical spelling of `level` and `language`.
///
/// The input must already compile: on failure the diagnostics from
/// [`transpile`] come back unchanged, because tidying a program nobody can run
/// would mean guessing what it was meant to say.
///
/// [`SyntaxLevel::Advanced`] returns the transpiled Python, since ordinary
/// Python is NME's advanced level; `language` has no effect there.
pub fn tidy(
    source: &str,
    level: SyntaxLevel,
    language: Language,
) -> Result<Conversion, Vec<Diagnostic>> {
    let python = transpile::transpile(source)?;
    if level == SyntaxLevel::Advanced {
        return Ok(Conversion {
            changed_lines: differing_lines(source, &python),
            source: python,
        });
    }
    // Tidying one line can take two steps: a line of Python becomes NME, and
    // that NME is then written the way this level and language write it. The
    // pass therefore repeats until nothing moves, which is what makes tidying
    // a tidy program change nothing at all. Every pass is checked against the
    // *original* Python, so no number of passes can drift.
    //
    // Every pass after the first looks only at the lines the pass before it
    // moved, which is what keeps a long program from being rewritten three
    // times over: a line nobody touched had nothing to say about this level
    // and this language, and a second reading would say it again.
    let mut tidied = source.to_string();
    let mut movable: Option<Vec<usize>> = None;
    for _ in 0..PASSES {
        let (next, again) = tidy_once(&tidied, &python, level, language, movable.as_deref());
        let moved = next != tidied;
        tidied = next;
        // Nothing moved, or nothing that moved can move again.
        if !moved || again.is_empty() {
            break;
        }
        movable = Some(again);
    }
    Ok(Conversion {
        changed_lines: differing_lines(source, &tidied),
        source: tidied,
    })
}

/// How many times the rewriting pass may run before the answer is taken as it
/// stands. Two is enough for Python → NME → this spelling; the third is there
/// so a program is never handed back mid-move.
const PASSES: usize = 3;

/// One line's rewrite, and the line it sits on.
#[derive(Clone)]
struct Proposal {
    /// Index in the lexer's logical-line list.
    line: usize,
    /// The spellings this line could take, best first. A line has more than
    /// one when a message could be written as the words it says: that is the
    /// nicer sentence, and the plainer one is there for the file where a name
    /// made later would turn those words into a value.
    spellings: Vec<Edit>,
}

impl Proposal {
    /// Where in the file this line stands. Every spelling replaces the same
    /// span, so the first one speaks for all of them.
    fn span(&self) -> crate::diagnostics::Span {
        self.spellings
            .first()
            .expect("a proposal always holds at least one spelling")
            .span
    }
}

/// One rewriting pass, with the whole-program check that makes it safe.
///
/// Answers the tidied source and the lines a further pass could still move.
fn tidy_once(
    source: &str,
    python: &str,
    level: SyntaxLevel,
    language: Language,
    movable: Option<&[usize]>,
) -> (String, Vec<usize>) {
    let Ok(lines) = lexer::logical_lines(source) else {
        return (source.to_string(), Vec::new());
    };
    let Ok(program) = parser::parse_program(source, &lines) else {
        return (source.to_string(), Vec::new());
    };
    let proposals = proposed_edits(source, &lines, &program, level, language, movable);
    let candidate = apply_edits(source, &only_edits(&proposals));
    if transpile_matches(&candidate, python) {
        return (candidate, worth_another_look(&proposals));
    }
    keeping_only_safe_edits(source, python, &proposals)
}

/// The lines this pass rewrote, which are the only ones the next pass reads.
///
/// A line can move twice: a line of Python becomes NME, and that NME is then
/// written the way this level and language write it. A condition does the
/// same — `if score` is Python on the way in and `if score exists` once NME
/// can read it — so a line that moved is asked again, whichever half of the
/// tidier moved it.
fn worth_another_look(proposals: &[Proposal]) -> Vec<usize> {
    proposals.iter().map(|proposal| proposal.line).collect()
}

/// Every rewrite the renderer offers for this file, before any of them has
/// been checked.
///
/// A line the parser read as NME is written again from the statement it
/// became; every other line goes to the Python converter, which already knows
/// how to turn safe Python into NME at a chosen level and language.
fn proposed_edits(
    source: &str,
    lines: &[LogicalLine],
    program: &parser::ParsedProgram,
    level: SyntaxLevel,
    language: Language,
    movable: Option<&[usize]>,
) -> Vec<Proposal> {
    let mut proposals = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if movable.is_some_and(|movable| !movable.contains(&index)) {
            continue;
        }
        // Replacing a statement that covers more than one physical line would
        // collapse source lines, so those are left exactly as written — the
        // same rule the Python converter follows.
        if spans_multiple_physical_lines(source, line) {
            continue;
        }
        let written = line.text(source);
        let nme = program.nme_lines.iter().find(|nme| nme.line_index == index);
        let mut spellings = Vec::new();
        // Four spellings, best first: this level, then the other one, each
        // with and without reading a message as the words it says. The level
        // asked for is what a file should come out in, but a line it cannot
        // take at that level is better written at the other one — still in
        // the language asked for — than left in the language it came in.
        let other = match level {
            SyntaxLevel::Beginner => SyntaxLevel::Sentence,
            _ => SyntaxLevel::Beginner,
        };
        for (level, read_messages) in [
            (level, true),
            (level, false),
            (other, true),
            (other, false),
        ] {
            let rewrite = Rewrite {
                source,
                level,
                language,
                length_not_count: asks_for_a_length(written),
                colon: written.trim_end().ends_with(':'),
                containers: &program.container_names,
                read_messages,
            };
            match nme {
                Some(nme) => spellings.extend(rewrite.statement(&nme.stmt, written)),
                None if read_messages => spellings.extend(
                    convert_line(source, line, level, language, &program.container_names)
                        .into_iter()
                        .map(|edit| edit.replacement),
                ),
                None => {}
            }
        }
        spellings.dedup();
        spellings.dedup();
        // A spelling that is the line already on the page means this line is
        // where it should be, and every spelling after it is a worse one. So
        // the list stops there rather than falling through to the next.
        if let Some(at) = spellings.iter().position(|spelling| spelling == written) {
            spellings.truncate(at);
        }
        let spellings = spellings
            .into_iter()
            .map(|replacement| Edit {
                span: line.span,
                replacement,
            })
            .collect::<Vec<_>>();
        if !spellings.is_empty() {
            proposals.push(Proposal {
                line: index,
                spellings,
            });
        }
    }
    proposals
}

/// Adds the edits back in groups, keeping every group the whole program still
/// transpiles the same way and taking apart the groups it does not.
///
/// This runs only when the fast path disagreed. Adding them back strictly one
/// at a time would be simpler, and on a four-hundred-line program with a
/// handful of lines that cannot be written at this level it meant four hundred
/// transpiles and a person waiting minutes. Halving a group asks exactly the
/// same question of far fewer programs, and the answer it keeps is the same
/// one: no edit survives unless the whole file still transpiles byte for byte
/// to the Python it started with.
fn keeping_only_safe_edits(
    source: &str,
    python: &str,
    proposals: &[Proposal],
) -> (String, Vec<usize>) {
    // First ask the program itself which lines it disagrees on, and drop the
    // edits sitting on them. That is the whole answer nearly every time, and
    // it costs one transpile rather than one per line.
    if let Some(answer) = without_the_lines_to_blame(source, python, proposals) {
        return answer;
    }
    let mut kept: Vec<Proposal> = Vec::new();
    keep_what_holds(source, python, &mut kept, proposals);
    let candidate = apply_edits(source, &only_edits(&kept));
    // Belt and braces: if even this cannot be trusted, hand back what came in.
    if transpile_matches(&candidate, python) {
        (candidate, worth_another_look(&kept))
    } else {
        (source.to_string(), Vec::new())
    }
}

/// Drops the edits that sit on the lines the tidied program went wrong at,
/// and answers the result if that alone made it right again.
///
/// One NME statement is one Python line, so a line of the produced Python that
/// is not the line it was names the source line that spoiled it. When the
/// program will not compile at all, the diagnostic points at the line instead.
fn without_the_lines_to_blame(
    source: &str,
    python: &str,
    proposals: &[Proposal],
) -> Option<(String, Vec<usize>)> {
    let mut kept = proposals.to_vec();
    for _ in 0..BLAME_ROUNDS {
        let candidate = apply_edits(source, &only_edits(&kept));
        let spoiled = lines_to_blame(&candidate, python);
        if spoiled.is_empty() {
            return Some((candidate, worth_another_look(&kept)));
        }
        // A line the program blamed loses its best spelling and keeps the
        // rest: the next one down is the same statement written more plainly,
        // and trying that before giving the line up is what lets a message
        // whose words are also a name still be written in the language asked
        // for. A line with nothing left to try is dropped.
        let mut moved = false;
        kept = kept
            .into_iter()
            .filter_map(|mut proposal| {
                if !spoiled.contains(&line_at(source, proposal.span().start)) {
                    return Some(proposal);
                }
                moved = true;
                proposal.spellings.remove(0);
                (!proposal.spellings.is_empty()).then_some(proposal)
            })
            .collect();
        if moved {
            continue;
        }
        // The blame landed on a line this pass never touched, and a block
        // that will not close is always reported at the line where it should
        // have closed rather than at the header that opened it. The headers
        // that lost their colon are the suspects: a colon block ends where
        // the indentation ends, while the same block written as a sentence
        // may want an `end` of its own. Put their colons back and ask again.
        let fewer: Vec<Proposal> = kept
            .iter()
            .filter(|proposal| !drops_a_colon(source, proposal))
            .cloned()
            .collect();
        if fewer.len() == kept.len() {
            return None;
        }
        kept = fewer;
    }
    None
}

/// True when this rewrite would take the colon off a block header.
fn drops_a_colon(source: &str, proposal: &Proposal) -> bool {
    let span = proposal.span();
    source[span.start..span.end].ends_with(':')
        && proposal
            .spellings
            .first()
            .is_some_and(|edit| !edit.replacement.ends_with(':'))
}

/// How many times tidying may drop the lines it is blamed for before it gives
/// up and searches properly. One round answers nearly every file; the rest are
/// for a program with many lines that cannot be written at this level, where
/// each round costs one transpile and the halving search would cost hundreds.
const BLAME_ROUNDS: usize = 24;

fn lines_to_blame(candidate: &str, python: &str) -> Vec<usize> {
    match transpile::transpile(candidate) {
        Err(problems) => problems
            .iter()
            .map(|problem| line_at(candidate, problem.span.start))
            .collect(),
        Ok(produced) => produced
            .lines()
            .zip(python.lines())
            .enumerate()
            .filter_map(|(index, (produced, wanted))| (produced != wanted).then_some(index + 1))
            .collect(),
    }
}

/// The 1-based line an offset falls on.
fn line_at(text: &str, offset: usize) -> usize {
    text[..offset.min(text.len())]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1
}

fn keep_what_holds(source: &str, python: &str, kept: &mut Vec<Proposal>, candidates: &[Proposal]) {
    if candidates.is_empty() {
        return;
    }
    let before = kept.len();
    kept.extend_from_slice(candidates);
    if transpile_matches(&apply_edits(source, &only_edits(kept)), python) {
        return;
    }
    kept.truncate(before);
    if let [proposal] = candidates {
        // One line on its own, and its best spelling did not hold. The rest
        // are plainer rather than different, so each is worth asking about.
        for spelling in proposal.spellings.iter().skip(1) {
            kept.push(Proposal {
                line: proposal.line,
                spellings: vec![spelling.clone()],
            });
            if transpile_matches(&apply_edits(source, &only_edits(kept)), python) {
                return;
            }
            kept.pop();
        }
        return;
    }
    let middle = candidates.len() / 2;
    keep_what_holds(source, python, kept, &candidates[..middle]);
    keep_what_holds(source, python, kept, &candidates[middle..]);
}

fn only_edits(proposals: &[Proposal]) -> Vec<Edit> {
    proposals
        .iter()
        .filter_map(|proposal| proposal.spellings.first().cloned())
        .collect()
}

fn transpile_matches(candidate: &str, python: &str) -> bool {
    transpile::transpile(candidate)
        .is_ok_and(|produced| transpile::is_the_same_program(&produced, python))
}

/// How many lines of `after` are not the line of `before` in the same place.
fn differing_lines(before: &str, after: &str) -> usize {
    let mut before = before.lines();
    let mut after = after.lines();
    let mut changed = 0;
    loop {
        match (before.next(), after.next()) {
            (None, None) => return changed,
            (left, right) => {
                if left != right {
                    changed += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// The four cells that write NME. Advanced writes Python and is checked
    /// on its own.
    const NME_CELLS: [(SyntaxLevel, Language); 4] = [
        (SyntaxLevel::Sentence, Language::English),
        (SyntaxLevel::Sentence, Language::Korean),
        (SyntaxLevel::Beginner, Language::English),
        (SyntaxLevel::Beginner, Language::Korean),
    ];

    const CELLS: [(SyntaxLevel, Language); 6] = [
        (SyntaxLevel::Sentence, Language::English),
        (SyntaxLevel::Sentence, Language::Korean),
        (SyntaxLevel::Beginner, Language::English),
        (SyntaxLevel::Beginner, Language::Korean),
        (SyntaxLevel::Advanced, Language::English),
        (SyntaxLevel::Advanced, Language::Korean),
    ];

    fn tidied(source: &str, level: SyntaxLevel, language: Language) -> Conversion {
        tidy(source, level, language).expect("this program compiles")
    }

    fn sentence(source: &str, language: Language) -> String {
        tidied(source, SyntaxLevel::Sentence, language).source
    }

    fn python(source: &str) -> String {
        transpile::transpile(source).expect("this program compiles")
    }

    /// Every `.nme` program in `examples/`, with its file name.
    fn corpus() -> Vec<(String, String)> {
        let folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
        let mut programs = Vec::new();
        for entry in std::fs::read_dir(folder).expect("the examples folder") {
            let path = entry.expect("an example").path();
            if path.extension().and_then(|name| name.to_str()) != Some("nme") {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .expect("a file name")
                .to_string();
            programs.push((
                name,
                std::fs::read_to_string(&path).expect("a readable file"),
            ));
        }
        programs.sort();
        assert!(programs.len() > 40, "the examples folder is the corpus");
        programs
    }

    #[test]
    fn every_example_keeps_its_python_and_its_line_count() {
        for (name, source) in corpus() {
            let Ok(python) = transpile::transpile(&source) else {
                continue;
            };
            let lines = source.lines().count();
            for (level, language) in CELLS {
                let conversion = tidied(&source, level, language);
                let again = transpile::transpile(&conversion.source)
                    .unwrap_or_else(|problems| panic!("{name}: {problems:?}"));
                assert!(
                    transpile::is_the_same_program(&again, &python),
                    "{name} tidied to {level:?}/{language:?} became a different program\n\
                     {python}\n---\n{again}"
                );
                assert_eq!(
                    conversion.source.lines().count(),
                    lines,
                    "{name} tidied to {level:?}/{language:?} changed its line count"
                );
            }
        }
    }

    #[test]
    fn tidying_a_tidy_program_changes_nothing() {
        // The whole corpus settles in one further pass; these four are the
        // ones that took two, so they are the ones a change would break.
        const RESTLESS: [&str; 4] = [
            "rpg-tower",
            "story-sentence",
            "needmorecoin-beginner",
            "time-loop-beginner",
        ];
        for (name, source) in corpus() {
            if !RESTLESS.iter().any(|restless| name.starts_with(restless)) {
                continue;
            }
            for (level, language) in CELLS {
                let once = tidied(&source, level, language);
                let twice = tidied(&once.source, level, language);
                assert_eq!(
                    twice.changed_lines, 0,
                    "{name} at {level:?}/{language:?} kept moving"
                );
                assert_eq!(twice.source, once.source);
            }
        }
    }

    /// The 234-line Korean sentence example is the best single test there is:
    /// nothing in it is Python, so every line has to survive the round trip.
    #[test]
    fn the_tower_in_korean_is_already_tidy_korean() {
        let source = std::fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/rpg-tower.ko.nme"),
        )
        .expect("the Korean tower");
        let conversion = tidied(&source, SyntaxLevel::Sentence, Language::Korean);
        assert_eq!(conversion.changed_lines, 0);
        assert_eq!(conversion.source, source);
    }

    #[test]
    fn a_korean_program_says_the_same_thing_in_english_and_back() {
        let source = std::fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/rpg-tower.ko.nme"),
        )
        .expect("the Korean tower");
        let wanted = python(&source);
        let english = sentence(&source, Language::English);
        assert_eq!(python(&english), wanted, "the English trip changed it");
        let home_again = sentence(&english, Language::Korean);
        assert_eq!(python(&home_again), wanted, "the way home changed it");
        assert_eq!(home_again.lines().count(), source.lines().count());
    }

    #[test]
    fn a_mess_of_levels_and_languages_becomes_one_korean_program() {
        let source = concat!(
            "# three levels, two languages, one file\n",
            "say \"start\"\n",
            "보여줘 안녕하세요\n",
            "Hello world show\n",
            "score = 0\n",
            "점수는 3\n",
            "to score add 1\n",
            "1을 점수에 더해\n",
            "친구들은 목록 민수, 지안\n",
            "민수를 친구들에 넣어\n",
            "show how many 친구들\n",
            "2 times: say \"hi\"\n",
            "3번 반복해서 좋아요 말해줘\n",
            "if score is above 2 then show high\n",
            "만약에 점수가 1보다 크면 높음 말해줘\n",
        );
        let conversion = tidied(source, SyntaxLevel::Sentence, Language::Korean);
        assert_eq!(
            conversion.source,
            concat!(
                "# three levels, two languages, one file\n",
                "start 말해줘\n",
                "안녕하세요 말해줘\n",
                "Hello world 말해줘\n",
                "score는 0\n",
                "점수는 3\n",
                "score에 1 더해\n",
                "점수에 1 더해\n",
                "친구들은 목록 민수, 지안\n",
                "친구들에 민수 넣어\n",
                "친구들 개수 말해줘\n",
                "2번 반복해서 hi 말해줘\n",
                "3번 반복해서 좋아요 말해줘\n",
                "만약에 score가 2보다 크면 high 말해줘\n",
                "만약에 점수가 1보다 크면 높음 말해줘\n",
            )
        );
        assert_eq!(python(&conversion.source), python(source));
        assert_eq!(conversion.source.lines().count(), source.lines().count());
    }

    #[test]
    fn a_program_that_does_not_compile_reports_the_same_problems() {
        let source = "if\nshow hello\n";
        let problems = transpile::transpile(source).expect_err("this does not compile");
        for (level, language) in CELLS {
            let refused = tidy(source, level, language).expect_err("nothing to tidy");
            assert_eq!(refused, problems);
        }
    }

    #[test]
    fn advanced_is_the_python_whatever_the_language() {
        let source = "set score to 1\nshow score\n";
        for language in [Language::English, Language::Korean] {
            let conversion = tidied(source, SyntaxLevel::Advanced, language);
            assert_eq!(conversion.source, "score = 1\nprint(score)\n");
            assert_eq!(conversion.changed_lines, 2);
        }
    }

    #[test]
    fn a_name_in_the_middle_of_text_survives_the_trip() {
        let source = "ask name What is your name?\nshow Hello name!\n";
        assert_eq!(
            python(source),
            "name = input(\"What is your name?\" + \" \")\nprint(\"Hello \" + str(name) + \"!\")\n"
        );
        assert_eq!(
            sentence(source, Language::Korean),
            "name을 물어봐 What is your name?\nHello name! 말해줘\n"
        );
        assert_eq!(python(&sentence(source, Language::Korean)), python(source));
        assert_eq!(sentence(source, Language::English), source);
    }

    #[test]
    fn the_words_of_a_statement_come_from_the_syntax_reference() {
        let source = concat!(
            "set score to 0\n",
            "add 1 to score\n",
            "subtract 2 from score\n",
            "multiply score by 3\n",
            "divide score by 4\n",
            "show score\n",
            "set friends to list of Mina, Ada\n",
            "show how many friends\n",
            "show the first of friends\n",
            "show friends joined by comma\n",
            "sort friends\n",
            "set ages to an empty record\n",
            "put Mina at 90 in ages\n",
            "show Mina in ages\n",
            "wait 1 second\n",
            "if score is greater than 2 then show high\n",
        );
        // English sentence syntax is already what this file is written in.
        assert_eq!(sentence(source, Language::English), source);
        assert_eq!(
            sentence(source, Language::Korean),
            concat!(
                "score는 0\n",
                "score에 1 더해\n",
                "score에서 2 빼줘\n",
                "score에 3 곱해\n",
                "score를 4로 나눠\n",
                "score 말해줘\n",
                "friends는 목록 Mina, Ada\n",
                "friends 개수 말해줘\n",
                "friends 첫 번째 말해줘\n",
                "friends를 쉼표로 이어 말해줘\n",
                "friends 정렬해\n",
                "ages는 빈 표\n",
                "ages에 Mina를 90으로 넣어\n",
                "ages의 Mina 말해줘\n",
                "1초 기다려\n",
                "만약에 score가 2보다 크면 high 말해줘\n",
            )
        );
    }

    /// A condition made of two names is Python on the way in — the converter
    /// writes `if a and b` — and a natural-language condition once NME can
    /// read it. Tidying has to reach the second spelling in one call, or the
    /// same command run twice would answer two different files.
    #[test]
    fn a_condition_of_two_names_settles_in_one_go() {
        let source = concat!(
            "one = True\n",
            "two = False\n",
            "if one and two\n",
            "  show both\n",
            "end\n",
        );
        let once = tidied(source, SyntaxLevel::Sentence, Language::English);
        assert!(
            once.source.contains("if one exists and two exists\n"),
            "the condition stopped half way: {}",
            once.source
        );
        let twice = tidied(&once.source, SyntaxLevel::Sentence, Language::English);
        assert_eq!(twice.changed_lines, 0);
        assert_eq!(python(&once.source), python(source));
    }

    /// `how many friends` and `the length of name` are one value to the
    /// parser and two sentences to a reader: `name 개수` prints the words
    /// `name 개수` instead of counting the letters. The line as written is
    /// what tells them apart, and it has to survive both ways across.
    #[test]
    fn a_length_stays_a_length_and_a_count_stays_a_count() {
        let source = concat!(
            "ask name What is your name?\n",
            "set friends to list of Mina, Ada\n",
            "show the length of name\n",
            "show how many friends\n",
        );
        let korean = sentence(source, Language::English);
        assert_eq!(korean, source, "English sentence syntax is what this is");
        let korean = sentence(source, Language::Korean);
        assert_eq!(
            korean,
            concat!(
                "name을 물어봐 What is your name?\n",
                "friends는 목록 Mina, Ada\n",
                "name 길이 말해줘\n",
                "friends 개수 말해줘\n",
            )
        );
        assert_eq!(python(&korean), python(source));
        assert_eq!(sentence(&korean, Language::English), source);
    }

    #[test]
    fn beginner_keeps_the_sentence_form_where_it_has_no_row_of_its_own() {
        let source = concat!(
            "set score to 0\n",
            "show score\n",
            "ask name What is your name?\n",
            "set friends to list of Mina, Ada\n",
            "sort friends\n",
            "start the timer\n",
            "clear the screen\n",
        );
        // `sort` and the screen sentences have no beginner row, so they keep
        // the sentence spelling. The list does have one — beginner syntax
        // writes values as Python — so it is written out as a Python list.
        let conversion = tidied(source, SyntaxLevel::Beginner, Language::English);
        assert_eq!(
            conversion.source,
            concat!(
                "save score to 0\n",
                "say score\n",
                "ask name, \"What is your name?\" + \" \"\n",
                "save friends to [\"Mina\", \"Ada\"]\n",
                "sort friends\n",
                "start the timer\n",
                "clear the screen\n",
            )
        );
        assert_eq!(python(&conversion.source), python(source));
    }

    #[test]
    fn a_story_prints_its_lines_exactly_as_they_stand() {
        let source = concat!(
            "story:\n",
            "  The door opened.\n",
            "  show me the way\n",
            "end\n",
        );
        for (level, language) in NME_CELLS {
            let conversion = tidied(source, level, language);
            assert!(
                conversion.source.contains("  The door opened.\n"),
                "a story line was rewritten at {level:?}/{language:?}: {}",
                conversion.source
            );
            assert!(conversion.source.contains("  show me the way\n"));
            assert_eq!(python(&conversion.source), python(source));
        }
    }

    /// Tidying used to leave a Korean particle exactly as the writer typed it,
    /// on the grounds that which half of a pair a word takes is a question
    /// about its sound and the tidier could not hear. It can: the rule is in
    /// `korean_particle`, and it covers Hangul, English words, numbers and —
    /// since 2026-08-20 — a name that is one letter, which is read as the
    /// letter's own name (`p` is 피 and takes 는, not the 은 that `skip`
    /// takes). So a tidied file is uniform here as it is everywhere else.
    #[test]
    fn the_particle_is_the_one_the_word_takes() {
        assert_eq!(sentence("금화는 10\n금화에 5 더해\n", Language::Korean),
                   "금화는 10\n금화에 5 더해\n");
        assert_eq!(sentence("금화은 10\n", Language::Korean), "금화는 10\n");
        assert_eq!(sentence("set p to 1\n", Language::Korean), "p는 1\n");
        assert_eq!(sentence("set skip to 1\n", Language::Korean), "skip은 1\n");
        assert_eq!(sentence("set friend to 1\n", Language::Korean), "friend는 1\n");
    }

    #[test]
    fn a_line_it_cannot_write_is_left_exactly_as_it_was() {
        // A condition written as a method call has no words in either
        // language, so the line keeps every character it had.
        let source = "digest는 \"00ab\"\n어려움은 2\nif digest.startswith(\"0\" * 어려움):\n    print(1)\n";
        for (level, language) in NME_CELLS {
            let conversion = tidied(source, level, language);
            assert!(
                conversion
                    .source
                    .contains("if digest.startswith(\"0\" * 어려움):\n"),
                "at {level:?}/{language:?}: {}",
                conversion.source
            );
        }
    }
}
