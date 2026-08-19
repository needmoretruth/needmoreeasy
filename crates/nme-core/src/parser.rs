//! Recognizes the advanced, beginner, and sentence levels of NME.
//!
//! A real Python parse always runs first. Valid Python therefore remains
//! byte-identical even when a Python name resembles an easier NME phrase.
//! Easier forms are matched only from lexer tokens; strings and comments are
//! never searched or rewritten as text.

use std::collections::{HashMap, HashSet};

use rustpython_parser::{parse as parse_python, Mode, Tok};

use crate::diagnostics::{Diagnostic, DiagnosticCode, Span};
use crate::lexer::{LogicalLine, Token};
use crate::syntax::{
    BundledModuleId, Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind,
    ItemPosition, ListOrder, Literal, LogicalOp, ModuleVersion, NmeLine, NmeStmt, Reading,
    Spelling, SplitBy, TextPart, TextTemplate, UpdateOp, Value, CHANCE_MAX_PERMILLE,
    COOLDOWN_PREFIX, ELAPSED_PYTHON, FILE_MODULE, FILE_MODULE_KO, FILE_READ_WORDS_EN,
    FILE_READ_WORDS_KO, FILE_WRITE_WORDS_EN, FILE_WRITE_WORDS_KO, MATH_MODULE, MATH_MODULE_KO,
    RANDOM_MODULE, RANDOM_MODULE_KO, SAY_KEYWORD, SAY_KEYWORD_KO, SAY_WORDS_EN, TIMER_NAME,
    TIMES_KEYWORD, TIMES_KEYWORD_KO,
};

const SAY_WORDS_KO: &[&str] = &[
    "말해",
    "말해줘",
    "말해주세요",
    "보여줘",
    "보여주세요",
    "출력해",
    "출력해줘",
    "출력해주세요",
    "해줘",
    "해주세요",
    "읽어줘",
];
/// Output words Korean writes **only at the end of a line**.
///
/// Korean states its verb last, so these are safe there and wrong at the
/// front: `말하기 연습` is speaking practice, `출력하기 어렵습니다` is a
/// complaint about a printer, and `보여주기 싫어요` is somebody refusing.
/// All three open with a word in this list and all three are sentences. At
/// the end of a line the same word can only be the verb, because nothing
/// else in Korean stands there.
const SAY_TRAILING_WORDS_KO: &[&str] = &[
    "말하기",
    "말해라",
    "알려줘",
    "알려주세요",
    "알려줘요",
    "얘기해",
    "얘기해줘",
    "얘기해주세요",
    "표시해",
    "표시해줘",
    "출력하기",
    "보여주기",
    "프린트해",
];
/// The one-syllable short form of the output word.
///
/// `안녕하세요 말` is `안녕하세요 말해줘` with three syllables saved, which is
/// what the owner asked for on 2026-08-19: *계속 넣어야 하는 명령어가 너무
/// 길어서 줄임말도 필요해*.
///
/// `말` is also the ordinary noun *word*, so it is read as the verb only at
/// the end of a line and only when the word in front of it cannot be making
/// it a noun — see [`korean_makes_the_next_word_a_noun`]. `그건 좋은 말` keeps
/// its whole sentence.
const SAY_SHORT_WORDS_KO: &[&str] = &["말"];
/// Words after which a Korean noun is part of a noun phrase rather than the
/// verb of the sentence.
///
/// Korean puts everything that describes a noun in front of it, and only two
/// shapes stand there: an adnominal verb ending in `ㄴ` or `ㄹ` (`좋은 말`,
/// `사랑한다는 말`, `할 말`), which [`is_korean_adnominal`] reads off the
/// syllable itself, and one of these determiners, which carry no ending at
/// all. Together they are what separates `그 말` from `안녕하세요 말`.
const KOREAN_DETERMINERS: &[&str] = &[
    "그", "이", "저", "내", "네", "제", "우리", "저희", "당신", "어느", "옛", "첫", "온갖",
    "별", "헛", "진짜", "참", "한마디", "요즘", "무슨", "웬", "뭔", "아무", "온",
];
const ASK_WORDS_EN: &[&str] = &["ask", "prompt", "question"];
const ASK_WORDS_KO: &[&str] = &[
    "물어봐",
    "물어봐줘",
    "물어보세요",
    "질문해",
    "질문해줘",
    "입력받아",
    "입력받아줘",
    "입력받아주세요",
    "물어봐요",
    "물어봐주세요",
    "질문해주세요",
];
/// The short form of the asking word: `이름 물어 이름이 뭐예요?`.
///
/// Two syllables is short enough that half the language is one edit from it —
/// `물을`, `됐어`, `믿어` — so it is read **exactly** and never repaired. With
/// the one-edit repair the other action words get, `물을 가져와 마셨습니다`
/// asked the reader a question and `됐어 그만해` waited for typing.
const ASK_SHORT_WORDS_KO: &[&str] = &["물어"];
/// Where a message goes, written after the message.
///
/// `put hello on the screen` is a whole sentence a beginner writes on their
/// first day, and it used to print itself. The place is what makes it a
/// command: `put the kettle on` names no screen and stays a sentence.
///
/// Korean marks the same place with the particle already attached, so the
/// word is `화면에` rather than `화면`; `화면` alone is the clear-screen word.
const SCREEN_TAIL_WORDS_EN: &[&str] = &["screen"];
const SCREEN_TAIL_WORDS_KO: &[&str] = &["화면에", "화면에다", "화면에다가", "스크린에"];
/// The verb a screen line may carry. English writes it in front of the
/// message and Korean after the screen word, which is where each language
/// puts a verb. None of them makes the line a command on its own — the screen
/// does that — so they are only here to stop the verb being printed as part
/// of the message.
const SCREEN_VERB_WORDS_EN: &[&str] = &["put", "write", "print", "show", "display", "say", "tell", "output", "draw"];
const SCREEN_VERB_WORDS_KO: &[&str] = &["띄워", "띄워줘", "보여줘", "출력해", "말해", "말해줘", "표시해"];
/// `이름을 5로 해` · `이름을 5라고 하자` · `name becomes 5` · `call it name 5`
/// — saving a value with the everyday verb instead of a saving word.
///
/// Korean closes the sentence with a light verb and English puts its word in
/// the middle, so the two lists do not line up word for word. Both are
/// claimed only in a shape that cannot be anything else: Korean needs the
/// value to carry `로`/`으로` or `라고`/`이라고`, and English needs the name to
/// be a word a sentence may turn into a name.
const SET_MAKE_WORDS_EN: &[&str] = &["becomes", "become", "call"];
/// `해줘` and `해주세요` are absent on purpose: both are already output words
/// (`안녕하세요 해줘` prints the greeting), so a saving sentence can never
/// reach the set matcher through them.
const SET_MAKE_WORDS_KO: &[&str] = &["해", "하자", "합시다", "하죠", "부르자"];
/// Value endings that mark what a Korean saving sentence is turning the name
/// into: `5로 해`, `5라고 하자`.
const SET_MAKE_ENDINGS_KO: &[&str] = &["으로", "로", "이라고", "라고"];
/// Asking words that are only ever a question when the line really asks one.
///
/// `read the label on the bottle` and `get well soon` are sentences; `read
/// name what is your name?` is guide 03 written with a different verb. The
/// question mark is what tells them apart, so these are read only when the
/// line ends in one.
const ASK_QUESTION_WORDS_EN: &[&str] = &["read", "get"];
const ASK_QUESTION_WORDS_KO: &[&str] = &["받아", "받아줘", "여쭤봐", "여쭤봐줘", "여쭈어봐"];
const REPEAT_WORDS_EN: &[&str] = &["repeat", "again", "do"];
const REPEAT_WORDS_KO: &[&str] = &[
    "반복",
    "반복해",
    "반복해줘",
    "반복해주세요",
    "반복하세요",
    "반복해서",
    "반복하고",
    "반복한다음",
    "다시해",
    "다시해주세요",
];
/// Repeat words that are ordinary verbs as well, read only with a count
/// beside them.
///
/// English writes the count after the word (`loop 3 times`) and Korean before
/// it (`3번 돌려`, `3번 되풀이해`), which is where each language puts one.
/// Without the count `loop the ribbon around twice` and `돈을 돌려 주세요` keep
/// every word they have.
///
/// They are also read **exactly**, never repaired. `되풀이해` is one character
/// from `되풀이할`, and `같은 하루를 최대 10회 되풀이할 수 있어요 말해줘` is a line
/// of an example program that has to keep printing.
const REPEAT_COUNT_WORDS_EN: &[&str] = &["loop", "iterate", "cycle"];
const REPEAT_COUNT_WORDS_KO: &[&str] = &[
    "돌려",
    "돌려줘",
    "돌려주세요",
    "되풀이",
    "되풀이해",
    "되풀이해줘",
    "되풀이해서",
    "되풀이하기",
];
const WHEN_WORDS_EN: &[&str] = &["when", "if"];
const WHEN_WORDS_KO: &[&str] = &["만약", "만약에", "만일", "혹시"];
const WHILE_WORDS_EN: &[&str] = &["while"];
const WHILE_WORDS_KO: &[&str] = &["동안", "하는동안", "할동안"];
const BREAK_WORDS_EN: &[&str] = &["break", "breakhere"];
const BREAK_WORDS_KO: &[&str] = &[
    "멈춰",
    "멈춰줘",
    "멈춰라",
    "멈추기",
    "그만해",
    "정지해",
    "종료해",
    "중단",
    "반복멈춰",
    "여기서멈춰",
];
/// Loop control spelled with words that are ordinary Python names as well
/// (`stop`, `quit`). They mean `break`/`continue` only inside an NME block,
/// where a line holding nothing but such a word can only be a mistake —
/// outside one, Python keeps them byte for byte.
const BREAK_ALIAS_WORDS_EN: &[&str] = &["stop", "stophere", "exitloop", "quit"];
const CONTINUE_ALIAS_WORDS_EN: &[&str] = &["keepgoing", "carryon"];
const ELSE_WORDS_EN: &[&str] = &["else", "otherwise"];
const ELSE_WORDS_KO: &[&str] = &[
    "아니면",
    "그렇지않으면",
    "아니면만약",
    "아니면만약에",
    "그렇지않으면만약",
    "그렇지않으면만약에",
];
const END_WORDS_EN: &[&str] = &["end", "finish", "done"];
const END_WORDS_KO: &[&str] = &["끝", "종료", "마침"];
const USE_WORDS_EN: &[&str] = &["use", "load", "get", "import"];
const USE_WORDS_KO: &[&str] = &[
    "사용",
    "사용해",
    "사용해줘",
    "사용해주세요",
    "불러와",
    "불러와줘",
    "가져와",
    "가져와줘",
    "받아",
    "받아줘",
];
const LATEST_WORDS: &[&str] = &["latest", "newest", "최신", "최신판", "최신버전"];
const NUMBER_WORDS: &[&str] = &["number", "numeric", "숫자", "숫자로", "수로"];
const KOREAN_PARTICLES: &[&str] = &[
    "에게서는",
    "한테서는",
    "에게서",
    "한테서",
    "으로는",
    "로는",
    "에게",
    "한테",
    "에서",
    "으로",
    "까지",
    "부터",
    "처럼",
    "보다",
    "이라도",
    "라도",
    // `준피해만큼 베었습니다` — as ordinary as `보다` and `까지`, and missing
    // until 2026-08-19, so the whole word stayed literal.
    "만큼",
    "밖에",
    "에는",
    "에서",
    "은",
    "는",
    "이",
    "가",
    "을",
    "를",
    "와",
    "과",
    "도",
    "의",
    "에",
    "로",
    "아",
    "야",
    "랑",
    "이랑",
    "예요",
    "이에요",
];

/// Korean makes a compound verb by putting a helper verb straight after an
/// `-아/어` form: `말해 봐야` (try saying), `물어봐 주셔서` (for asking me),
/// `저장해 두었습니다` (kept it saved). The helper carries the sentence and the
/// word in front of it is not a command, even though NME spells several of
/// those `-아/어` forms as action words.
///
/// Only these written-out forms count, and only straight after an action
/// word: `말해줘 비가 쏟아졌습니다` is guide 13 and still prints the rain, and
/// `물어봐 이름이 뭐예요?` is guide 03.
const AUXILIARY_VERBS_KO: &[&str] = &[
    // 주다 — doing it for someone
    "줘",
    "줘서",
    "주고",
    "주니",
    "주면",
    "주며",
    "주는",
    "준",
    "주지",
    "주기",
    "주셔서",
    "주시고",
    "주시면",
    "주신",
    "주시는",
    "주셨다",
    "주셨습니다",
    "주었다",
    "주었습니다",
    "주었어요",
    "줬다",
    "줬습니다",
    "줬어요",
    "주라",
    "주렴",
    // 보다 — trying it
    "봐",
    "봐도",
    "봐야",
    "봤자",
    "본들",
    "봐서",
    "보고",
    "보니",
    "보면",
    "보는",
    "본",
    "보지",
    "봤다",
    "봤습니다",
    "봤어요",
    "봤어",
    "보았다",
    "보았습니다",
    "보았어요",
    "보았어",
    "보세요",
    "보시고",
    "보려고",
    // 두다 · 놓다 — leaving it done
    "둬",
    "둬서",
    "두고",
    "두니",
    "두면",
    "두는",
    "둔",
    "두지",
    "두었다",
    "두었습니다",
    "뒀다",
    "뒀습니다",
    "두세요",
    "놓고",
    "놓으니",
    "놓으면",
    "놓는",
    "놓은",
    "놓지",
    "놓았다",
    "놓았습니다",
    "놨다",
    "놨습니다",
    // 버리다 — getting it over with
    "버려",
    "버려서",
    "버리고",
    "버리면",
    "버렸다",
    "버렸습니다",
    "버렸어요",
];

/// True when a Korean word NME knows is really the first half of a compound
/// verb — see [`AUXILIARY_VERBS_KO`]. Every Korean vocabulary word counts,
/// not only the five older actions: `섞어 주시면 됩니다` and
/// `멈춰 주시면 좋겠습니다` are sentences in exactly the same way that
/// `말해 봐야 소용없습니다` is.
fn korean_action_word_carries_an_auxiliary(tokens: &[Token]) -> bool {
    korean_auxiliary_pairs(tokens).next().is_some()
}

/// Where each compound verb on the line starts, in source order.
fn korean_auxiliary_pairs(tokens: &[Token]) -> impl Iterator<Item = usize> + '_ {
    tokens
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| {
            name_word(&pair[0]).is_some_and(is_hangul)
                && (is_nme_vocabulary_word(&pair[0])
                    || token_matches_exact(&pair[0], SORT_WORDS_KO)
                    || token_matches_exact(&pair[0], REVERSE_WORDS_KO)
                    || token_matches_exact(&pair[0], SHUFFLE_WORDS_KO))
                && token_matches_exact(&pair[1], AUXILIARY_VERBS_KO)
                && !pair_spells_one_action_word(pair)
        })
        .map(|(at, _)| at)
}

/// True when the two words together spell one action word NME knows.
///
/// `이름 을 물어 봐 이름이 뭐예요?` writes `물어봐` with a space in the middle
/// of it, and `봐` is a helper verb in its own right. Joined, the two are the
/// asking word, so the second is half of the verb rather than a helper hanging
/// off the first, and the line is a question after all.
fn pair_spells_one_action_word(pair: &[Token]) -> bool {
    let (Some(first), Some(second)) = (name_word(&pair[0]), name_word(&pair[1])) else {
        return false;
    };
    let joined = format!("{first}{second}");
    ALL_ACTION_WORDS
        .iter()
        .any(|list| list.contains(&joined.as_str()))
}

/// True when every compound verb on this line stands inside the question of
/// an explicit `<이름>을 물어봐 …` line.
///
/// Such a line is a question: the name comes first, then the asking word, and
/// everything after the asking word is the text shown while it waits. That
/// text is ordinary Korean, so it may perfectly well contain a compound verb.
/// `주문을 물어봐 마법의 주문을 말해 보세요` does, and reading `말해 보세요` as a
/// sentence threw the question away — the line printed itself instead of
/// asking, and a loop written around it never got an answer and never ended.
///
/// `물어봐 주셔서 감사합니다` is the opposite shape and must stay prose: no
/// name stands in front of the asking word, and the helper verb hangs off the
/// asking word itself rather than off a word inside a question. So the test
/// is not "does the line contain an asking word" but "does every compound
/// verb sit after the question has begun".
fn korean_question_owns_every_auxiliary(tokens: &[Token]) -> bool {
    let Some(shape) = find_ask_shape(tokens, MatchMode::Exact) else {
        return false;
    };
    if !matches!(shape.spelling, Spelling::Korean) || shape.target_at >= shape.action_start {
        return false;
    }
    if shape.prompt_start >= tokens.len() {
        return false;
    }
    korean_auxiliary_pairs(tokens).all(|at| at >= shape.prompt_start)
}

const SET_WORDS_EN: &[&str] = &["set", "save", "remember"];
const SET_WORDS_KO: &[&str] = &[
    "저장",
    "저장해",
    "저장해줘",
    "기억해",
    "기억해줘",
    "설정",
    "설정해",
    "설정해줘",
    "지정",
    "지정해",
    "정해",
    "만들어",
];
/// Particles that mark the name a value is being saved into when Korean puts
/// that name first: `점수를 0으로`, `점수가 0`, `점수에 0 저장해`.
const SET_TARGET_PARTICLES_KO: &[&str] = &["을", "를", "이", "가", "에"];
/// Spoken endings that belong to the sentence rather than to the value.
/// `점수는 0이다` saves the number zero, not the text `0이다`.
const VALUE_ENDINGS_KO: &[&str] = &[
    "입니다", "이에요", "예요", "이다", "으로", "로", "라고", "이라고",
];
const UPDATE_ADD_WORDS_EN: &[&str] = &["add", "increase", "increment", "plus"];
const UPDATE_ADD_WORDS_KO: &[&str] = &["더해", "더해줘", "올려", "올려줘", "늘려", "늘려줘"];
const UPDATE_SUBTRACT_WORDS_EN: &[&str] = &["subtract", "decrease", "decrement", "minus", "remove"];
const UPDATE_SUBTRACT_WORDS_KO: &[&str] = &["빼", "빼줘", "내려", "내려줘", "줄여", "줄여줘"];
// `times` is deliberately absent from the English multiply words: it is the
// repeat marker, and `score times 2` must keep meaning "repeat".
const UPDATE_MULTIPLY_WORDS_EN: &[&str] = &["multiply", "multiplied"];
const UPDATE_MULTIPLY_WORDS_KO: &[&str] = &["곱해", "곱해줘", "곱하기해"];
const UPDATE_DIVIDE_WORDS_EN: &[&str] = &["divide", "divided"];
const UPDATE_DIVIDE_WORDS_KO: &[&str] = &["나눠", "나눠줘", "나누어줘"];
/// Particles that may be attached to the number in a value change.
const UPDATE_AMOUNT_PARTICLES_KO: &[&str] = &["으로", "로", "만큼", "씩", "을", "를"];
/// `주세요` and its relatives are not verbs. They attach to *any* Korean verb
/// to make the request polite — `넣어 주세요`, `적어 주세요`, `전해 주세요` —
/// and the verb in front of them is usually not one NME knows.
///
/// Left to the ordinary matching rules they were read as an action word twice
/// over. Written on their own they are one character from `해주세요`, so
/// `설탕을 조금만 넣어 주세요` printed `설탕을 조금만 넣어`. Glued to the word
/// before them they spell an action word exactly, so `조용히 해 주세요` printed
/// `조용히`. Every one of the twenty-three polite requests measured on
/// 2026-08-19 compiled into something the writer had not written.
///
/// So: never repaired into an action word, and glued to the word before only
/// when that word is *itself* an action word NME already knows — `말해 주세요`
/// and `기다려 주세요` still mean what they say, `해 주세요` and `전해 주세요`
/// are sentences.
const POLITE_AUXILIARY_KO: &[&str] = &[
    "주세요",
    "주십시오",
    "주시겠어요",
    "주시겠습니까",
    "주시길",
    "주시기",
    "주소서",
];

const WAIT_WORDS_EN: &[&str] = &["wait", "pause", "sleep"];
const WAIT_WORDS_KO: &[&str] = &[
    "기다려",
    "기다려줘",
    "기다리세요",
    "기다려주세요",
    "쉬어",
    "쉬어줘",
    "쉬세요",
];
/// Time units dropped before the wait amount is read as an expression. The
/// Korean ones are also stripped when written attached, as in `3초`.
const SECOND_WORDS_EN: &[&str] = &["second", "seconds", "sec", "secs"];
const SECOND_WORDS_KO: &[&str] = &["초동안", "초간", "초만", "초"];
const WAIT_FILLER_WORDS: &[&str] = &["for", "about", "동안", "간"];
const CONTINUE_WORDS_EN: &[&str] = &["skip", "skipthis", "skipit", "nextone"];
const CONTINUE_WORDS_KO: &[&str] = &[
    "건너뛰어",
    "건너뛰어줘",
    "건너뛰기",
    "건너뛰자",
    "넘어가",
    "넘어가줘",
    "계속해",
    "넘겨",
    "다음",
];
const APPEND_WORDS_EN: &[&str] = &["append", "push"];
const APPEND_WORDS_KO: &[&str] = &["넣어", "넣어줘", "추가해", "추가해줘", "붙여", "붙여줘"];
const APPEND_CONNECTORS_EN: &[&str] = &["to", "into", "onto"];
/// Particles marking the list a value is being put into (`친구들에 민수 넣어`).
const APPEND_TARGET_PARTICLES_KO: &[&str] = &["에다가", "에다", "에", "한테", "에게"];
const LIST_WORDS_EN: &[&str] = &["list"];
const LIST_WORDS_KO: &[&str] = &["목록", "리스트"];
/// `set friends to an empty list` / `친구들은 빈 목록`. The word only means a
/// list when a list word follows it, so `빈 방이었습니다` stays a sentence.
const EMPTY_WORDS_EN: &[&str] = &["empty", "blank"];
const EMPTY_WORDS_KO: &[&str] = &["빈", "비어있는", "새"];
/// `record` / `표` — one name holding many named values, each one under a name
/// of its own. Python calls it a dictionary.
///
/// Read as the kind of thing being made **only where a value is being saved**,
/// exactly like the list word beside it. Everywhere else `record`, `table` and
/// `표` are words somebody wrote: `표는 두 장 남았습니다` is about tickets.
const RECORD_WORDS_EN: &[&str] = &["record", "table"];
const RECORD_WORDS_KO: &[&str] = &["표"];
/// `put Mina at 90 in ages` / `나이표에 민수를 90으로 넣어`.
///
/// The Korean verbs are the list-adding verbs, on purpose: it is the same act
/// in the same words, and the name decides which kind of container is meant.
const RECORD_PUT_WORDS_EN: &[&str] = &["put"];
const RECORD_PUT_WORDS_KO: &[&str] = &["넣어", "넣어줘", "넣어주세요", "두어", "두어줘"];
/// The word in front of the value in `put Mina at 90 in ages`.
const RECORD_AT_WORDS_EN: &[&str] = &["at", "as"];
/// The word in front of the record in `put Mina at 90 in ages`.
const RECORD_IN_WORDS_EN: &[&str] = &["in", "into"];
/// `민수를` — the particle that marks which name is being written under.
const RECORD_KEY_PARTICLES_KO: &[&str] = &["을", "를"];
/// `90으로` — the particle that marks the value being written.
const RECORD_VALUE_PARTICLES_KO: &[&str] = &["으로", "로"];
/// `나이표의 민수` · `나이표에서 민수` — the particles that read one value out.
///
/// `의` is the commonest particle in the language, so the gate is never the
/// particle: the name in front of it has to be one the program made a record.
const RECORD_OF_PARTICLES_KO: &[&str] = &["에서", "의"];
/// `인사하기라는 일:` — the noun that closes a Korean job header.
const JOB_WORDS_KO: &[&str] = &["일", "작업"];
/// `인사하기라는` · `계산이라는` — what marks the name in a Korean job header.
const JOB_NAME_SUFFIXES_KO: &[&str] = &["이라는", "라는"];
/// `to greet:` — the word that opens an English job header.
const JOB_LEAD_WORDS_EN: &[&str] = &["to"];
/// `do greet` / `인사하기 해줘` — run a job that was defined earlier.
const RUN_JOB_WORDS_EN: &[&str] = &["do", "run"];
const RUN_JOB_WORDS_KO: &[&str] = &["해", "해줘", "해주세요", "실행해", "실행해줘"];
/// `이름에게 인사하기라는 일:` and `민수에게 인사하기 해줘` — the particle that
/// marks the one thing a job is given.
const JOB_PARAMETER_PARTICLES_KO: &[&str] = &["에게", "한테", "을", "를"];
/// `do greet with Mina` — the word that marks the same thing in English.
const JOB_WITH_WORDS_EN: &[&str] = &["with"];
/// `how many friends` / `친구들 개수` — how many items a list holds.
const COUNT_WORDS_EN: &[&str] = &["count", "number", "many"];
const COUNT_WORDS_KO: &[&str] = &["개수", "갯수"];
/// `the length of name` / `이름 길이` — how many characters a text has.
const LENGTH_WORDS_EN: &[&str] = &["length", "size"];
const LENGTH_WORDS_KO: &[&str] = &["길이", "글자수"];
/// `the total of scores` / `점수들 합`.
const TOTAL_WORDS_EN: &[&str] = &["total", "sum"];
const TOTAL_WORDS_KO: &[&str] = &["합", "합계", "총합"];
/// `the biggest of scores` / `점수들 최댓값`.
const LARGEST_WORDS_EN: &[&str] = &["biggest", "largest", "highest", "maximum"];
const LARGEST_WORDS_KO: &[&str] = &["최댓값", "최대값", "큰"];
const SMALLEST_WORDS_EN: &[&str] = &["smallest", "lowest", "minimum"];
const SMALLEST_WORDS_KO: &[&str] = &["최솟값", "최소값", "작은"];
/// `점수들 중 가장 큰 것` — the scaffolding of the phrase Korean actually
/// says. `가장`/`제일` is the anchor: without one of them a list name
/// followed by `큰` is not a reading at all. English needs none of this — it
/// says `the biggest of scores` — so these three lists have no twin.
const EXTREME_SCOPE_WORDS_KO: &[&str] = &["중", "중에서", "가운데"];
const EXTREME_MOST_WORDS_KO: &[&str] = &["가장", "제일"];
const EXTREME_THING_WORDS_KO: &[&str] = &["것", "값"];
/// `the first of friends` / `친구들 첫 번째`.
const FIRST_WORDS_EN: &[&str] = &["first"];
const FIRST_WORDS_KO: &[&str] = &["첫번째", "첫째", "처음", "첫"];
const LAST_WORDS_EN: &[&str] = &["last"];
const LAST_WORDS_KO: &[&str] = &["마지막", "맨뒤"];
/// `item 3 of friends` / `친구들 3번째`.
const ITEM_WORDS_EN: &[&str] = &["item", "element"];
const ITEM_WORDS_KO: &[&str] = &["번째", "째"];
/// `name in capitals` / `이름 대문자로`.
const CAPITALS_WORDS_EN: &[&str] = &["capitals", "capital", "uppercase"];
const CAPITALS_WORDS_KO: &[&str] = &["대문자로", "대문자"];
const SMALL_LETTERS_WORDS_EN: &[&str] = &["lowercase", "small"];
const SMALL_LETTERS_WORDS_KO: &[&str] = &["소문자로", "소문자"];
/// `show friends joined by comma` / `친구들을 쉼표로 이어 말해줘`.
const JOIN_WORDS_EN: &[&str] = &["joined", "join"];
const JOIN_WORDS_KO: &[&str] = &["이어", "이어서", "이어붙여"];
/// The separators that have a name of their own. A written one works too.
const SEPARATOR_WORDS_EN: &[&str] = &["comma", "space", "newline"];
const SEPARATOR_WORDS_KO: &[&str] = &["쉼표", "빈칸", "공백", "줄바꿈"];
/// `친구들을 붙여` / `friends joined together` — one join word that carries
/// its own separator, and that separator is nothing at all. It is a word of
/// its own rather than an empty [`SEPARATOR_WORDS_KO`] entry because Korean
/// says it as the verb and English says it as an adverb after `joined`.
const JOIN_TOGETHER_WORDS_EN: &[&str] = &["together"];
const JOIN_TOGETHER_WORDS_KO: &[&str] = &["붙여", "붙여서", "붙여줘", "이어붙여", "이어붙여줘"];
/// `friends joined by nothing` / `친구들을 그대로 이어` — the same meaning
/// written where a separator goes.
const EMPTY_SEPARATOR_WORDS_EN: &[&str] = &["nothing"];
const EMPTY_SEPARATOR_WORDS_KO: &[&str] = &["그대로"];
/// `메모를 쉼표로 나눈 것` / `memo split by comma` — the opposite of joining.
///
/// `split` is deliberately the only English word here. `divided` and `cut`
/// would both be one edit from something else the grammar already reads, and
/// this statement is gated on a name rather than on an anchor word.
const SPLIT_WORDS_EN: &[&str] = &["split"];
const SPLIT_WORDS_KO: &[&str] = &["나눈", "쪼갠", "자른"];
/// `줄마다 나눈 것` / `split by line` — cut wherever a line ends.
const SPLIT_LINE_WORDS_EN: &[&str] = &["line", "lines"];
const SPLIT_LINE_WORDS_KO: &[&str] = &["줄마다", "줄별로", "한줄씩"];
/// The noun that closes the Korean phrase: `… 나눈 것`. Without it `나눈` is a
/// modifier waiting for a noun, and `이야기를 둘로 나눈 뒤에` is still a
/// sentence. English needs no such word — it says `memo split by comma`.
const SPLIT_THING_WORDS_KO: &[&str] = &["것", "거", "것들"];
/// `별표를 5개 붙인 것` / `star repeated 5 times` — one piece of text, that many
/// times over.
///
/// This is the shape an earlier round left out on purpose, because
/// `별표를 5번 이어 말해줘` cannot be told apart from the counted loop, where
/// `5번` already means *five times*. What makes it safe now is that it is a
/// **noun phrase and not a command**: it is gated on a name the program has
/// already saved, the count is followed by a counting word, and the whole
/// thing closes with `붙인 것`, which no loop ever says. So `5번` may be
/// written here after all.
const REPEAT_TEXT_WORDS_EN: &[&str] = &["repeated"];
const REPEAT_TEXT_WORDS_KO: &[&str] = &["붙인", "이어붙인"];
/// The counting word after the number: `5 times` / `5개`, `5번`.
const COPIES_WORDS_EN: &[&str] = &["times"];
const COPIES_WORDS_KO: &[&str] = &["개", "번"];
/// `sort friends` / `친구들 정렬해`.
const SORT_WORDS_EN: &[&str] = &["sort"];
const SORT_WORDS_KO: &[&str] = &["정렬해", "정렬해줘", "정렬"];
const REVERSE_WORDS_EN: &[&str] = &["reverse"];
const REVERSE_WORDS_KO: &[&str] = &["거꾸로", "거꾸로해", "거꾸로해줘", "뒤집어", "뒤집어줘"];
const SHUFFLE_WORDS_EN: &[&str] = &["shuffle"];
const SHUFFLE_WORDS_KO: &[&str] = &["섞어", "섞어줘", "섞어주세요"];
/// `if friends contains Mina` — English says it with a verb.
const CONTAINS_WORDS_EN: &[&str] = &["contains", "contain", "includes", "include", "holds"];
/// `만약에 친구들에 민수가 있으면` — Korean marks the list with a particle and
/// closes the clause with `있으면`/`없으면`, which the condition grammar
/// already reads. These are the particles that mark the list.
const CONTAINS_WORDS_KO: &[&str] = &["안에는", "속에는", "안에", "속에", "에는", "에"];
/// `repeat forever` / `계속 반복해`.
const FOREVER_WORDS_EN: &[&str] = &["forever", "always"];
const FOREVER_WORDS_KO: &[&str] = &["계속", "무한", "끝없이"];
/// The word that opens an English reading of a list: `how many friends`.
///
/// It is also `show` without its `s`, which is why it is protected below:
/// `how are you` used to print `are you`.
const READING_LEAD_WORDS_EN: &[&str] = &["how"];
/// `the remainder of pile divided by 4` / `쌓인돌을 4로 나눈 나머지`.
const REMAINDER_WORDS_EN: &[&str] = &["remainder", "rest", "leftover"];
const REMAINDER_WORDS_KO: &[&str] = &["나머지"];
/// The dividing word each language puts before the number.
const DIVIDED_WORDS_EN: &[&str] = &["divided", "shared", "split"];
const DIVIDED_WORDS_KO: &[&str] = &["나눈", "나눈뒤", "나누고"];
/// `for each friend in friends with place` /
/// `친구들의 친구마다 순서와 함께 반복해` — a loop that also holds which turn it
/// is on. The word after `with`, and the word before `와 함께`, is the name
/// that holds it, so the writer chooses it rather than the compiler.
const POSITION_WORDS_EN: &[&str] = &["with"];
const POSITION_WORDS_KO: &[&str] = &["함께", "같이"];
/// `use greet from "helper.nme"` / `"helper.nme"에서 greet 가져와`.
const NME_IMPORT_WORDS_EN: &[&str] = &["use", "take", "borrow"];
const NME_IMPORT_WORDS_KO: &[&str] = &["가져와", "가져와줘", "가져오기", "불러와", "불러오기"];
/// Particles a reading word may carry when it stands in a condition
/// (`친구들 개수가 3보다 크면`).
const READING_PARTICLES_KO: &[&str] = &["이", "가", "은", "는", "을", "를"];
/// Words that make a random pick. They must be written exactly.
///
/// A one-edit repair here reads `따라` — "along", one of the commonest words
/// in ordinary Korean — as `골라`, which turned the sentence
/// `강을 따라 집으로 갑니다` into a random pick from four fragments with no
/// error at all. A pick now needs a word that can only mean picking.
const RANDOM_CHOICE_WORDS: &[&str] = &[
    "랜덤선택",
    "하나골라",
    "골라",
    "하나뽑아",
    "뽑아",
    "randomchoice",
    "pick",
    "choose",
];
/// `say slowly Hello` / `천천히 말해줘 안녕` — text told one character at a time.
const SLOW_WORDS_EN: &[&str] = &["slowly"];
const SLOW_WORDS_KO: &[&str] = &["천천히"];
/// The intensity word in `say very slowly` / `아주 천천히 말해줘`.
const VERY_WORDS_EN: &[&str] = &["very"];
const VERY_WORDS_KO: &[&str] = &["아주"];
/// The marker before an explicit pause: `slowly every 3 seconds` / `3초씩`.
const SLOW_EVERY_WORDS_EN: &[&str] = &["every"];
const SLOW_EVERY_WORDS_KO: &[&str] = &["초씩"];
/// Seconds between characters for the plain and the very slow spelling.
const SLOW_SECONDS: &str = "0.04";
const VERY_SLOW_SECONDS: &str = "0.12";
/// `story:` / `이야기:` — the block in which every line is text.
///
/// The trailing colon is required, and it is the whole safety argument for
/// this form: `story:` and `이야기:` are not valid Python, while the ordinary
/// sentences that mention the same word — `옛날 이야기`, `story time`,
/// `tell me a story`, `이야기를 들려줘` — carry no colon and stay sentences.
const STORY_WORDS_EN: &[&str] = &["story", "tale"];
const STORY_WORDS_KO: &[&str] = &["이야기", "얘기"];
/// `slow story:` / `천천히 이야기:` — a story told one character at a time.
const STORY_SLOW_WORDS_EN: &[&str] = &["slow", "slowly"];
const STORY_SLOW_WORDS_KO: &[&str] = &["천천히"];
/// The full-width colon a Korean IME writes. Python has no meaning for it at
/// all, so the lexer hands it over as ordinary sentence text and only the
/// block headers that ask for a colon accept it.
const FULL_WIDTH_COLON: &str = "\u{ff1a}";
/// `30% 확률로` / `30% chance` — the words that turn a percentage into a
/// chance. A percentage on its own is never one, which is what keeps
/// `전체의 30%가 왔습니다` and `I am 100% sure` ordinary sentences.
const CHANCE_WORDS_EN: &[&str] = &["chance", "chances", "probability"];
const CHANCE_WORDS_KO: &[&str] = &["확률로", "확률"];
/// The written spelling of `%`: `30 percent chance` / `30 퍼센트 확률로`.
const CHANCE_PERCENT_WORDS_EN: &[&str] = &["percent", "percentage"];
const CHANCE_PERCENT_WORDS_KO: &[&str] = &["퍼센트", "프로"];
/// `30% of the time` — the other English way to say the same thing.
const CHANCE_TIME_WORDS_EN: &[&str] = &["time"];
/// `with a 30% chance` — a word that may lead the phrase.
const CHANCE_LEAD_WORDS_EN: &[&str] = &["with"];
/// Particles that may sit inside a Korean chance: `30%의 확률로`, `확률 30%로`.
const CHANCE_PARTICLES_KO: &[&str] = &["의", "로", "으로"];
/// `luck is a 30% chance` — the English shape that saves a chance in a name.
const CHANCE_IS_WORDS_EN: &[&str] = &["is", "equals"];
/// `clear the screen` / `화면 지워`.
const CLEAR_SCREEN_WORDS_EN: &[&str] = &["clear"];
const CLEAR_SCREEN_WORDS_KO: &[&str] = &["화면"];
const CLEAR_SCREEN_ACTIONS_EN: &[&str] = &["screen"];
const CLEAR_SCREEN_ACTIONS_KO: &[&str] = &["지워", "지워줘", "비워", "비워줘"];
/// `draw a line` / `줄 그어`.
const DRAW_LINE_WORDS_EN: &[&str] = &["draw"];
const DRAW_LINE_WORDS_KO: &[&str] = &["줄", "가로줄"];
const DRAW_LINE_ACTIONS_EN: &[&str] = &["line"];
const DRAW_LINE_ACTIONS_KO: &[&str] = &["그어", "그어줘"];
/// `say in a box Hello` / `상자로 말해줘 안녕`.
const BOX_WORDS_EN: &[&str] = &["box"];
const BOX_WORDS_KO: &[&str] = &["상자로"];
/// `say in the middle Hello` / `가운데 말해줘 안녕`.
const MIDDLE_WORDS_EN: &[&str] = &["middle"];
const MIDDLE_WORDS_KO: &[&str] = &["가운데"];
/// `start the timer` / `시간 재기 시작해`. The Korean spellings are written
/// joined, exactly like the other multi-word Korean actions, because the
/// matcher glues neighbouring words back together before comparing.
const START_TIMER_WORDS_EN: &[&str] = &["start"];
const START_TIMER_WORDS_KO: &[&str] = &["시간재기시작해", "시간재기시작"];
const TIMER_WORDS_EN: &[&str] = &["timer"];
/// `put door on cooldown for 3 seconds` / `문 쿨타임 3초 걸어`.
const COOLDOWN_WORDS_EN: &[&str] = &["cooldown"];
const COOLDOWN_WORDS_KO: &[&str] = &["쿨타임", "쿨타임을", "쿨타임은", "쿨타임이"];
const COOLDOWN_SET_WORDS_EN: &[&str] = &["put"];
const COOLDOWN_SET_WORDS_KO: &[&str] = &["걸어", "걸어줘"];
/// `when door is ready` / `문 쿨타임이 끝났으면`.
const COOLDOWN_READY_WORDS_EN: &[&str] = &["ready"];
const COOLDOWN_READY_WORDS_KO: &[&str] = &["끝났으면"];
/// `when door is on cooldown` / `문 쿨타임이 남았으면`.
const COOLDOWN_BUSY_WORDS_KO: &[&str] = &["남았으면"];
/// `문 쿨타임 끝날때까지 기다려` — the Korean wait spelling, written joined.
const COOLDOWN_UNTIL_WORDS_KO: &[&str] = &["끝날때까지"];
/// `elapsed` / `잰시간` — the stopwatch reading, usable wherever a value is.
const ELAPSED_WORDS_EN: &[&str] = &["elapsed"];
const ELAPSED_WORDS_KO: &[&str] = &["잰시간", "걸린시간"];
const EACH_WORDS_EN: &[&str] = &["each", "every"];
/// Korean loop-variable ending in `이름들의 이름마다 반복해`.
const EACH_SUFFIX_KO: &str = "마다";
/// Particles that may sit between the collection and the loop variable.
const EACH_CONTAINER_PARTICLES_KO: &[&str] = &["가운데", "안의", "속의", "에서", "중", "의"];
/// English words that never name a value.
///
/// A command word at the start of a line does not make the rest of the line
/// its argument. `set the table for four people` is a sentence, and reading
/// it as a name called `the` holding `table for four people` produced a
/// program that ran, printed nothing and reported nothing. So did `remember
/// to water the plants`, `Set your alarm for the early train.` and `ask me
/// anything you like`. When the name a sentence form would create is one of
/// these, the line is prose and prints itself.
const NOT_A_NAME_EN: &[&str] = &[
    "a",
    "about",
    "above",
    "across",
    "after",
    "again",
    "against",
    "all",
    "an",
    "along",
    "already",
    "also",
    "although",
    "always",
    "am",
    "among",
    "another",
    "any",
    "anyone",
    "anything",
    "anyway",
    "are",
    "around",
    "aside",
    "at",
    "away",
    "back",
    "be",
    "because",
    "been",
    "before",
    "behind",
    "being",
    "below",
    "beside",
    "between",
    "beyond",
    "both",
    "but",
    "by",
    "can",
    "could",
    "did",
    "does",
    "down",
    "during",
    "each",
    "either",
    "enough",
    "even",
    "ever",
    "every",
    "everyone",
    "everything",
    "except",
    "for",
    "from",
    "had",
    "has",
    "have",
    "he",
    "her",
    "here",
    "hers",
    "herself",
    "him",
    "himself",
    "his",
    "how",
    "however",
    "i",
    "if",
    "inside",
    "instead",
    "into",
    "it",
    "its",
    "itself",
    "just",
    "may",
    "me",
    "might",
    "must",
    "my",
    "myself",
    "near",
    "nearly",
    "neither",
    "never",
    "no",
    "nobody",
    "none",
    "nor",
    "nothing",
    "now",
    "of",
    "off",
    "on",
    "onto",
    "or",
    "other",
    "our",
    "ours",
    "ourselves",
    "out",
    "outside",
    "over",
    "past",
    "perhaps",
    "please",
    "quite",
    "rather",
    "really",
    "she",
    "should",
    "since",
    "so",
    "some",
    "someone",
    "something",
    "still",
    "such",
    "than",
    "that",
    "the",
    "their",
    "theirs",
    "them",
    "themselves",
    "then",
    "there",
    "these",
    "they",
    "this",
    "those",
    "though",
    "through",
    "throughout",
    "to",
    "together",
    "too",
    "toward",
    "towards",
    "under",
    "unless",
    "until",
    "up",
    "upon",
    "us",
    "very",
    "was",
    "we",
    "were",
    "what",
    "whatever",
    "where",
    "whether",
    "which",
    "while",
    "who",
    "whoever",
    "whom",
    "whose",
    "why",
    "will",
    "with",
    "within",
    "without",
    "would",
    "yet",
    "you",
    "your",
    "yours",
    "yourself",
    // Adverbs. `Ask nicely and she might say yes.` is a sentence; a value
    // called `nicely` is not something anybody sets out to make.
    "absolutely",
    "badly",
    "barely",
    "carefully",
    "certainly",
    "clearly",
    "completely",
    "definitely",
    "easily",
    "entirely",
    "exactly",
    "finally",
    "gently",
    "happily",
    "hardly",
    "kindly",
    "loudly",
    "mostly",
    "nicely",
    "politely",
    "probably",
    "quickly",
    "quietly",
    "sadly",
    "safely",
    "simply",
    "slowly",
    "soon",
    "later",
    "softly",
    "suddenly",
    "totally",
    "usually",
];

const SENTENCE_FILLERS: &[&str] = &["please", "좀", "혹시", "제발"];
/// How a Korean question ends when the writer left the `?` off.
/// `이름이 뭐예요` is asking, mark or no mark.
const KOREAN_QUESTION_PREDICATES: &[&str] = &[
    "뭐예요",
    "뭐에요",
    "뭐야",
    "뭐죠",
    "무엇인가요",
    "무엇이에요",
    "무엇입니까",
    "뭔가요",
];
const COMMAND_ENDINGS: &[&str] = &["?", "!"];
/// Counting words accepted wherever a repeat count or a number of seconds is
/// expected. They are read only in those two places, never as a name, so an
/// ordinary variable called `one` still means itself everywhere else.
const NUMBER_WORDS_EN: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "once",
    "twice",
];
/// Written value of each word in [`NUMBER_WORDS_EN`], in the same order.
const NUMBER_VALUES_EN: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "1", "2",
];
/// Both Korean counting systems: the native `하나`…`열` and the Sino-Korean
/// `일`…`십`, each in the plain and the counter-attached spelling.
const NUMBER_WORDS_KO: &[&str] = &[
    "하나", "한", "둘", "두", "셋", "세", "넷", "네", "다섯", "여섯", "일곱", "여덟", "아홉", "열",
    "일", "이", "삼", "사", "오", "육", "칠", "팔", "구", "십",
];
/// Written value of each word in [`NUMBER_WORDS_KO`], in the same order.
const NUMBER_VALUES_KO: &[&str] = &[
    "1", "1", "2", "2", "3", "3", "4", "4", "5", "6", "7", "8", "9", "10", "1", "2", "3", "4", "5",
    "6", "7", "8", "9", "10",
];
/// Counter words that mark a repeat count. `times` and `번` may stand alone;
/// the rest are ordinary nouns too, so they only count when a number is
/// written right in front of them.
const TIMES_WORDS_EN: &[&str] = &["times", "time", "loops", "loop", "rounds", "round"];
const TIMES_WORDS_KO: &[&str] = &["번", "회", "차례", "판"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum MatchMode {
    Exact,
    Recover,
}

/// The parser result also records virtual indentation for explicit sentence
/// blocks.  The transpiler uses that information to indent ordinary Python
/// lines mixed into a `while ... 끝` block without changing the source file.
#[derive(Debug, Clone)]
pub struct ParsedProgram {
    pub nme_lines: Vec<NmeLine>,
    pub virtual_indents: Vec<usize>,
    /// Blank lines inside a story block, and the `print()` that each one
    /// becomes. They hold no tokens, so they are not logical lines and the
    /// replacement has to name its own place in the source.
    pub story_blank_lines: Vec<(Span, String)>,
}

/// Parse all logical lines, collecting independent beginner-facing errors.
pub fn parse(source: &str, lines: &[LogicalLine]) -> Result<Vec<NmeLine>, Vec<Diagnostic>> {
    parse_program(source, lines).map(|program| program.nme_lines)
}

/// Parse a complete program, including indentation-free blocks closed by
/// `end`/`끝`.  Existing indentation-based blocks remain supported, so users
/// can move one line at a time from sentence syntax to Python.
#[allow(clippy::too_many_lines)]
pub fn parse_program(
    source: &str,
    lines: &[LogicalLine],
) -> Result<ParsedProgram, Vec<Diagnostic>> {
    let mut found = Vec::new();
    let mut problems = Vec::new();
    let mut bindings = BindingEnv::new();
    let mut virtual_indents = vec![0; lines.len()];
    let mut blocks = Vec::<ExplicitBlock>::new();
    // `만약에 체력이 0보다 크면 살아있음 말해줘` opens no block — the whole
    // condition is on one line — so an `아니면` under it used to be refused,
    // even though the two lines lower to Python that runs. This remembers the
    // last such line so the next one can attach to it.
    let mut inline_branch: Option<InlineBranch> = None;
    // Where every physical line starts, so a blank line inside a story block
    // can be given a `print()` of its own. Blank lines carry no tokens and so
    // never become logical lines.
    let line_starts = crate::lexer::line_start_offsets(source);
    let mut story_blank_lines = Vec::<(Span, String)>::new();
    // Tracks whether any NME statement has been seen. A lone `end`/`끝`
    // after that is almost always a leftover block terminator, so it gets a
    // friendly diagnostic instead of silently staying Python (where it would
    // fail at runtime). Pure-Python files still keep `end` byte-identical.
    let mut saw_nme = false;
    // Physical indentation of the previous logical line, and whether that
    // line was allowed to open a deeper one.
    let mut previous_indent = 0usize;
    let mut previous_opens_suite = false;
    // The most recent line that looks like a block header but opened no
    // block. An `end` with nothing to close names that line instead of
    // itself, because that is where the writer's mistake is.
    let mut block_header_lines = HashSet::<usize>::new();
    // Python compound headers normally get their body indentation from the
    // source.  Inside an indentation-free NME block, however, a learner may
    // write a normal Python header (`if x:`) and then continue with the body
    // at the same logical level.  Keep those headers separately so ordinary
    // Python receives the same virtual indentation that NME statements do.
    // The source indent is retained as the unambiguous signal for a real
    // Python dedent; explicit NME `break`/`end`/branch lines also close a
    // flat Python suite when they are at the header's level.
    let mut python_header_indents = Vec::<(usize, bool)>::new();
    // Top-level Python headers are intentionally not virtualized, but their
    // loop kind still matters when an NME inline body appears in their
    // physically indented suite. This keeps `for ...:`/`while ...:` bodies
    // valid while allowing inline `break` diagnostics in ordinary Python
    // conditional suites.
    let mut top_level_python_loop_indents = Vec::<usize>::new();
    // An NME loop header that uses ordinary indentation instead of `end`
    // (`3 times:` with an indented body) opens no explicit block, so nothing
    // else here knew its body was inside a loop: `skip` and `건너뛰어` stayed
    // bare Python names that raise `NameError`, and Python's own `continue`
    // was refused. Their header indentation is remembered here instead.
    let mut nme_loop_indents = Vec::<usize>::new();
    // The same for an NME conditional written with a colon and an indented
    // body. Without it `아니면:` had nothing to attach to, while English got
    // `else:` for free because Python spells it the same way.
    let mut nme_branch_indents = Vec::<usize>::new();
    // `except*` suites have one Python-specific control-flow restriction:
    // `break`, `continue`, and `return` are not allowed in their bodies.
    // Track their physical header indentation separately from ordinary
    // Python headers so nested functions and classes still use BindingEnv.
    let mut python_except_star_indents = Vec::<(usize, usize)>::new();
    let mut python_try_indents = Vec::<usize>::new();
    let mut async_function_contexts = Vec::<AsyncFunctionContext>::new();
    let mut completed_async_functions = Vec::<AsyncFunctionContext>::new();
    let mut python_declaration_contexts = vec![PythonDeclarationContext {
        body_scope_depth: 0,
        seen_names: HashSet::new(),
        annotation_targets: HashSet::new(),
        declarations: HashMap::new(),
    }];

    for (index, line) in lines.iter().enumerate() {
        let is_end = exact_end(line.tokens.as_slice());
        let is_break = exact_break(line.tokens.as_slice());
        let is_continue = exact_continue(line.tokens.as_slice());
        let branch_shape = branch_shape(line.tokens.as_slice());

        let opens_suite = opens_a_suite(source, line);
        if line.indent > previous_indent && !previous_opens_suite {
            problems.push(unexpected_indent_diagnostic(source, line));
        }
        previous_indent = line.indent;
        previous_opens_suite = opens_suite;

        // ------------------------------------------------------- a story
        //
        // Inside `이야기:` / `story:` every line is text. Not `3초 기다려`,
        // not `만약에 …`, not even a line of ordinary Python: a story is
        // prose, and a line of prose that silently turns into a statement is
        // the worst thing this compiler can do to a program. The one line
        // that still means what it says is the closing `end`/`끝`, because
        // without it a flat story could never be closed at all.
        if let Some(story) = blocks.last().and_then(ExplicitBlock::as_story).cloned() {
            let ends_here = match story.close_on_dedent {
                // An `end` written further out closes a block further out, so
                // the story finishes first and leaves that `end` alone.
                Some(header_indent) if is_end.is_some() => line.indent < header_indent,
                Some(header_indent) => line.indent <= header_indent,
                None => false,
            };
            if !ends_here && is_end.is_none() {
                let depth = blocks.len();
                bindings.enter_line(line.indent + depth);
                let virtual_indent = depth.saturating_sub(line.indent);
                let prefix = story_prefix(source, &line_starts, line, virtual_indent);
                push_story_blanks(
                    &mut story_blank_lines,
                    source,
                    &line_starts,
                    &prefix,
                    story.last_line,
                    line.number,
                );
                let value = Value::Text(make_text_template(
                    source,
                    &line.tokens,
                    &bindings.visible_names(),
                ));
                let stmt = match &story.seconds {
                    Some(seconds) => NmeStmt::SaySlowly {
                        value,
                        seconds: seconds.clone(),
                    },
                    None => NmeStmt::Say { value },
                };
                if let Some(ExplicitBlock::Story(open)) = blocks.last_mut() {
                    open.prefix = prefix;
                    open.last_line = last_physical_line(source, line);
                    open.has_body = true;
                }
                virtual_indents[index] = virtual_indent;
                found.push(NmeLine {
                    line_index: index,
                    span: line.span,
                    stmt,
                    virtual_indent,
                });
                saw_nme = true;
                continue;
            }
            // The story is over. The blank lines it still owed belong to it.
            push_story_blanks(
                &mut story_blank_lines,
                source,
                &line_starts,
                &story.prefix,
                story.last_line,
                line.number,
            );
            if ends_here {
                if !story.has_body {
                    problems.push(empty_story_diagnostic(story.header_span));
                }
                blocks.pop();
            }
        }

        // An indented-suite sentence block (whose first body line was
        // physically indented) may end at the physical dedent, like ordinary
        // Python, but only when the remaining `end`/`끝` lines cannot close
        // the nested reading anyway. That keeps every previously valid
        // program unchanged: a nested header with enough closing `end`s stays
        // nested, while an ambiguous indented block followed by a flat block
        // with too few `end`s becomes a sibling instead of a missing-end
        // error. A flat statement at the block's own level keeps the suite
        // flat from there on (only `end` closes it), so mixed indented+flat
        // bodies keep working. Explicit closers (`end`, `break`, branches)
        // are handled by their own paths below.
        if !(is_end.is_some() || is_break || is_continue || branch_shape.is_some()) {
            let line_is_header = is_header_shape(&line.tokens);
            let remaining_ends = count_remaining_ends(lines, index);
            loop {
                let open = blocks.len();
                let Some(close_on_dedent) = blocks.last().and_then(ExplicitBlock::close_on_dedent)
                else {
                    break;
                };
                if line.indent == close_on_dedent && line_is_header && open >= remaining_ends {
                    blocks.pop();
                } else if line.indent == close_on_dedent {
                    if let Some(top) = blocks.last_mut() {
                        top.clear_close_on_dedent();
                    }
                    break;
                } else {
                    break;
                }
            }
        }

        let depth = blocks.len();
        if depth == 0 {
            python_header_indents.clear();
        }
        while top_level_python_loop_indents
            .last()
            .is_some_and(|header_indent| line.indent <= *header_indent)
        {
            top_level_python_loop_indents.pop();
        }
        while nme_loop_indents
            .last()
            .is_some_and(|header_indent| line.indent <= *header_indent)
        {
            nme_loop_indents.pop();
        }
        let inside_indented_nme_loop = !nme_loop_indents.is_empty();
        while nme_branch_indents
            .last()
            .is_some_and(|header_indent| line.indent < *header_indent)
        {
            nme_branch_indents.pop();
        }
        // A line at the header's own level that is not another branch ends
        // the chain.
        if branch_shape.is_none()
            && nme_branch_indents
                .last()
                .is_some_and(|header_indent| line.indent == *header_indent)
        {
            nme_branch_indents.pop();
        }
        let indented_nme_branch = nme_branch_indents
            .last()
            .is_some_and(|header_indent| line.indent == *header_indent);
        // A one-line condition on the line straight above, at this very
        // indent. It opened no block, so `depth` is still zero and the
        // branch below it would otherwise be turned away.
        let follows_inline_branch = inline_branch
            .as_ref()
            .is_some_and(|open| open.indent == line.indent);
        let closes_flat_python_suite =
            is_end.is_some() || is_break || is_continue || branch_shape.is_some();
        while python_header_indents.last().is_some_and(|header_indent| {
            line.indent < header_indent.0
                || (closes_flat_python_suite && line.indent <= header_indent.0)
        }) {
            python_header_indents.pop();
        }
        while python_try_indents
            .last()
            .is_some_and(|header_indent| line.indent < *header_indent)
        {
            python_try_indents.pop();
        }
        if python_try_indents.last().is_some_and(|header_indent| {
            line.indent == *header_indent
                && !is_python_try_header(&line.tokens)
                && !is_python_try_clause_header(&line.tokens)
        }) {
            python_try_indents.pop();
        }
        while python_except_star_indents
            .last()
            .is_some_and(|(header_indent, _)| line.indent <= *header_indent)
        {
            python_except_star_indents.pop();
        }
        // Keep Python's compatibility rule strict at the top level: an
        // indented Python body is still the user's responsibility unless an
        // explicit NME block is already open. This prevents a malformed
        // ordinary `if` from being repaired silently by NME.
        let python_depth = if depth > 0 {
            python_header_indents.len()
        } else {
            0
        };

        // A logical line inside an explicit block receives a virtual level.
        // Physical indentation is still retained, so nested Python remains
        // possible and ordinary Python lines can be mixed freely.
        let branch_depth = branch_shape.is_some().then(|| depth.saturating_sub(1));
        let base_line_depth = if is_end.is_some() || branch_depth.is_some() {
            branch_depth.unwrap_or_else(|| depth.saturating_sub(1))
        } else {
            depth
        };
        let line_depth = base_line_depth + python_depth;
        virtual_indents[index] = line_depth.saturating_sub(line.indent);
        let mut parse_line = line.clone();
        parse_line.indent = line.indent + line_depth;

        // A sentence header without physical indentation is allowed to open
        // an explicit block when a matching end appears later.  Giving the
        // existing suite parser a synthetic next indent keeps all old
        // indentation diagnostics and inline handling intact.
        let next_indent = lines.get(index + 1).map(|next| next.indent + depth);
        let has_next_line = lines.get(index + 1).is_some();
        let unindented_next_line = lines
            .get(index + 1)
            .is_some_and(|next| next.indent <= line.indent);
        let has_colon = line
            .tokens
            .iter()
            .any(|token| matches!(token.tok, Tok::Colon));
        // A colon normally means advanced Python, so keep its indentation
        // rules.  The compact NME repeat header (`3 times:` / `3번:`) is not
        // valid Python, though, and may use the same explicit `end`/`끝`
        // terminator as sentence blocks without forcing the learner to indent.
        let nme_colon_header =
            has_colon && !is_valid_python_header(token_text(source, &line.tokens));
        let flat_body_follows = has_next_line && unindented_next_line;
        let force_suite = is_header_shape(&line.tokens)
            && ((!has_colon && (has_future_end(lines, index) || flat_body_follows))
                // A colon-bearing beginner header only needs virtual
                // indentation when its body is actually flat and an
                // explicit terminator exists. If the next line is
                // physically indented, keep the ordinary suite semantics
                // and do not claim a later `end` for this header.
                || (nme_colon_header && has_future_end(lines, index) && flat_body_follows));
        let next_indent = force_suite.then_some(parse_line.indent + 1).or(next_indent);

        bindings.enter_line(parse_line.indent);
        let python_scope_depth = bindings.python_scope_depth();
        while async_function_contexts
            .last()
            .is_some_and(|context| context.body_scope_depth > python_scope_depth)
        {
            if let Some(context) = async_function_contexts.pop() {
                completed_async_functions.push(context);
            }
        }
        while python_declaration_contexts
            .last()
            .is_some_and(|context| context.body_scope_depth > python_scope_depth)
        {
            python_declaration_contexts.pop();
        }
        let inside_python_except_star = python_except_star_indents
            .last()
            .is_some_and(|(_, scope_depth)| *scope_depth == bindings.python_scope_depth());
        let known_names = bindings.visible_names();
        let block = BlockCtx::TopLevel {
            line: &parse_line,
            next_indent,
        };

        // `end` and a bare `break` are valid Python-shaped words in a few
        // contexts, so an already-open explicit block claims them before
        // Python-wins. Outside a block, a stray `end` after any NME
        // statement is reported (it would only fail at runtime as Python);
        // in a pure-Python file `end`/`끝` remain untouched identifiers.
        let direct_stmt = if is_end.is_some() && depth > 0 {
            Some(Ok(Some(NmeStmt::End)))
        } else if is_end.is_some() && saw_nme {
            Some(Err(
                match unreadable_block_header_before(source, lines, index, &block_header_lines) {
                    Some(header) => unreadable_block_header_diagnostic(header),
                    None => unmatched_end_diagnostic(line.span),
                },
            ))
        } else if is_break
            && (depth > 0
                || inside_indented_nme_loop
                || (line.indent == 0
                    && action_phrase_at(&line.tokens, 0, BREAK_WORDS_EN, MatchMode::Exact)
                        .is_some())
                || (is_korean_break_alias(&line.tokens)
                    && !is_valid_python_statement(token_text(source, &line.tokens))))
        {
            Some(Ok(Some(NmeStmt::Break)))
        } else if is_continue && (depth > 0 || inside_indented_nme_loop) {
            // `skip` and `건너뛰어` are ordinary Python names on their own, so
            // like `break` they are only read as NME inside an NME block.
            Some(Ok(Some(NmeStmt::Continue)))
        } else if (depth > 0 || inside_indented_nme_loop)
            && block_only_loop_control(
                &line.tokens,
                &known_names,
                BREAK_WORDS_EN,
                BREAK_WORDS_KO,
                BREAK_ALIAS_WORDS_EN,
            )
        {
            Some(Ok(Some(NmeStmt::Break)))
        } else if (depth > 0 || inside_indented_nme_loop)
            && block_only_loop_control(
                &line.tokens,
                &known_names,
                CONTINUE_WORDS_EN,
                CONTINUE_WORDS_KO,
                CONTINUE_ALIAS_WORDS_EN,
            )
        {
            Some(Ok(Some(NmeStmt::Continue)))
        } else if branch_shape.is_some()
            && depth == 0
            && indented_nme_branch
            && !is_valid_python_statement(token_text(source, &line.tokens))
            && !is_valid_python_header(token_text(source, &line.tokens))
        {
            Some(match_colon_branch(
                source,
                &line.tokens,
                &block,
                &known_names,
            ))
        } else if branch_shape.is_some()
            && depth == 0
            && !follows_inline_branch
            && !line
                .tokens
                .iter()
                .any(|token| matches!(token.tok, Tok::Equal | Tok::Colon))
            && !is_valid_python_statement(token_text(source, &line.tokens))
        {
            Some(Err(branch_without_condition_diagnostic(line.span)))
        } else if branch_shape.is_some()
            && (depth > 0 || follows_inline_branch || is_korean_branch_alias(&line.tokens))
            && !line
                .tokens
                .iter()
                .any(|token| matches!(token.tok, Tok::Equal | Tok::Colon))
            && (!is_valid_python_statement(token_text(source, &line.tokens))
                || (depth > 0 && line.tokens.len() == 1))
        {
            Some(match_branch(
                source,
                &line.tokens,
                &block,
                &known_names,
                MatchMode::Exact,
            ))
        } else {
            None
        };
        let classified =
            direct_stmt.unwrap_or_else(|| classify(source, &line.tokens, &block, &known_names));
        match classified {
            Ok(Some(stmt)) => {
                saw_nme = true;
                let inside_loop = blocks
                    .iter()
                    .any(|block| matches!(block, ExplicitBlock::Loop { .. }))
                    || python_header_indents.iter().any(|(_, is_loop)| *is_loop)
                    || !top_level_python_loop_indents.is_empty()
                    || inside_indented_nme_loop;
                if inline_break_is_outside_loop(&stmt, source, inside_loop) {
                    problems.push(break_outside_loop_diagnostic(line.span));
                    continue;
                }
                if inline_continue_is_outside_loop(&stmt, &line.tokens, inside_loop) {
                    problems.push(continue_outside_loop_diagnostic(line.span));
                    continue;
                }
                if inline_return_is_outside_function(
                    &stmt,
                    &line.tokens,
                    bindings.inside_function(),
                ) {
                    problems.push(return_outside_function_diagnostic(line.span));
                    continue;
                }
                if inline_yield_inside_comprehension(&stmt, &line.tokens) {
                    problems.push(yield_inside_comprehension_diagnostic(line.span));
                    continue;
                }
                if inline_async_comprehension_outside_async_function(
                    &stmt,
                    &line.tokens,
                    bindings.inside_async_function(),
                ) {
                    problems.push(async_comprehension_outside_async_function_diagnostic(
                        line.span,
                    ));
                    continue;
                }
                remember_async_generator_context(
                    &mut async_function_contexts,
                    &line.tokens,
                    python_scope_depth,
                    line.span,
                );
                if inline_yield_is_outside_function(&stmt, &line.tokens, bindings.inside_function())
                {
                    problems.push(yield_outside_function_diagnostic(line.span));
                    continue;
                }
                if inline_await_is_outside_async_function(
                    &stmt,
                    &line.tokens,
                    bindings.inside_async_function(),
                ) {
                    problems.push(await_outside_async_function_diagnostic(line.span));
                    continue;
                }
                if inline_yield_from_is_in_async_function(
                    &stmt,
                    &line.tokens,
                    bindings.inside_async_function(),
                ) {
                    problems.push(yield_from_async_function_diagnostic(line.span));
                    continue;
                }
                if inside_python_except_star
                    && (matches!(stmt, NmeStmt::Break)
                        || inline_except_star_control_flow(&stmt, &line.tokens))
                {
                    problems.push(except_star_control_flow_diagnostic(line.span));
                    continue;
                }
                if matches!(stmt, NmeStmt::End) {
                    if let Some(ExplicitBlock::Story(story)) = blocks.last() {
                        if !story.has_body {
                            problems.push(empty_story_diagnostic(story.header_span));
                        }
                    }
                    if blocks.is_empty() {
                        problems.push(
                            match unreadable_block_header_before(
                                source,
                                lines,
                                index,
                                &block_header_lines,
                            ) {
                                Some(header) => unreadable_block_header_diagnostic(header),
                                None => unmatched_end_diagnostic(line.span),
                            },
                        );
                        continue;
                    }
                    blocks.pop();
                }
                if matches!(stmt, NmeStmt::Break)
                    && line.indent == 0
                    && !blocks
                        .iter()
                        .any(|block| matches!(block, ExplicitBlock::Loop { .. }))
                {
                    problems.push(break_outside_loop_diagnostic(line.span));
                    continue;
                }
                let attaches_to_inline_branch =
                    !indented_nme_branch && branch_shape.is_some() && follows_inline_branch;
                if let Some(branch) = &branch_shape {
                    // An indented NME conditional has no explicit block to
                    // check against; CPython validates the finished suite.
                    if attaches_to_inline_branch {
                        let open = inline_branch.as_mut().expect("checked above");
                        if open.else_seen {
                            problems.push(duplicate_else_diagnostic(line.span));
                            continue;
                        }
                        if matches!(branch, BranchShape::Else) {
                            open.else_seen = true;
                        }
                    } else if !indented_nme_branch
                        && !validate_branch(branch, &mut blocks, line.span, &mut problems)
                    {
                        continue;
                    }
                }
                let base_target_indent = if matches!(stmt, NmeStmt::End) || branch_shape.is_some() {
                    base_line_depth
                } else {
                    depth
                };
                let virtual_indent =
                    (base_target_indent + python_depth).saturating_sub(line.indent);
                if reads_elapsed(&stmt) && !known_names.contains(TIMER_NAME) {
                    problems.push(timer_not_started_diagnostic(line.span));
                    continue;
                }
                bindings.remember_nme(&stmt);
                // Only a one-line condition leaves an `else` open, and only
                // for the line straight after it.
                inline_branch = match &stmt {
                    NmeStmt::When { inline: Some(_), .. } => Some(InlineBranch {
                        indent: line.indent,
                        else_seen: false,
                    }),
                    NmeStmt::ElseIf { inline: Some(_), .. } | NmeStmt::Else { inline: Some(_) }
                        if attaches_to_inline_branch =>
                    {
                        inline_branch
                    }
                    _ => None,
                };
                found.push(NmeLine {
                    line_index: index,
                    span: line.span,
                    stmt,
                    virtual_indent,
                });
                // A story always opens its block: prose has no other way
                // of saying where it ends, and the lines below it must not
                // be read as commands.
                if let Some(NmeStmt::Story { seconds }) = found.last().map(|line| &line.stmt) {
                    let seconds = seconds.clone();
                    bindings.push_explicit_scope(parse_line.indent + 1);
                    block_header_lines.insert(index);
                    blocks.push(ExplicitBlock::Story(StoryBlock {
                        close_on_dedent: (!flat_body_follows).then_some(line.indent),
                        header_span: line.span,
                        has_body: false,
                        seconds,
                        prefix: story_prefix(source, &line_starts, line, virtual_indent + 1),
                        last_line: last_physical_line(source, line),
                    }));
                }
                // A job's body is a real Python function scope, so names set
                // inside it stay inside it. Like a story and like a repeat, a
                // job always opens an explicit block: an indented body closes
                // on its own dedent, and a flat one waits for `end`/`끝`.
                //
                // Opening the block only for a flat body is what this used to
                // do, and it meant the shape every guide in this repository
                // teaches — indented body, `끝` underneath — was refused with
                // `E0101` pointing at a header that was perfectly correct.
                if let Some(NmeStmt::Job { parameters, .. }) = found.last().map(|line| &line.stmt) {
                    // The name the job is given is bound by its header, so the
                    // body may use it straight away.
                    let given: HashSet<String> = parameters.iter().cloned().collect();
                    if flat_body_follows || has_future_end(lines, index) {
                        bindings.push_function_scope(parse_line.indent + 1, given);
                        block_header_lines.insert(index);
                        blocks.push(ExplicitBlock::Job {
                            close_on_dedent: (!flat_body_follows).then_some(line.indent),
                        });
                    } else {
                        // Indented body and no `end` anywhere below: this is an
                        // ordinary Python suite that its own dedent closes, the
                        // same route a `3번 반복해` with an indented body takes.
                        bindings.push_pending_function_scope(parse_line.indent, given);
                    }
                }
                if matches!(
                    found.last().map(|line| &line.stmt),
                    Some(
                        NmeStmt::Times { inline: None, .. }
                            | NmeStmt::ForEach { inline: None, .. }
                            | NmeStmt::While { inline: None, .. }
                            | NmeStmt::Forever { inline: None }
                    )
                ) && !force_suite
                {
                    nme_loop_indents.push(line.indent);
                }
                if matches!(
                    found.last().map(|line| &line.stmt),
                    Some(NmeStmt::When { inline: None, .. } | NmeStmt::ElseIf { inline: None, .. })
                ) && !force_suite
                    && nme_branch_indents.last() != Some(&line.indent)
                {
                    nme_branch_indents.push(line.indent);
                }
                if let Some(
                    NmeStmt::Times { inline: None, .. }
                    | NmeStmt::ForEach { inline: None, .. }
                    | NmeStmt::When { inline: None, .. }
                    | NmeStmt::While { inline: None, .. }
                    | NmeStmt::Forever { inline: None }
                    | NmeStmt::Chance { inline: None, .. },
                ) = found.last().map(|line| &line.stmt)
                {
                    if force_suite {
                        let is_loop = matches!(
                            found.last().map(|line| &line.stmt),
                            Some(
                                NmeStmt::While { .. }
                                    | NmeStmt::Times { .. }
                                    | NmeStmt::Forever { .. }
                                    | NmeStmt::ForEach { .. }
                            )
                        );
                        bindings.push_explicit_scope(parse_line.indent + 1);
                        block_header_lines.insert(index);
                        let close_on_dedent = (!flat_body_follows).then_some(line.indent);
                        blocks.push(if is_loop {
                            ExplicitBlock::Loop { close_on_dedent }
                        } else {
                            ExplicitBlock::Conditional {
                                else_seen: false,
                                close_on_dedent,
                            }
                        });
                    }
                }
            }
            Ok(None) => {
                let valid_python_header = is_valid_python_header(token_text(source, &line.tokens));
                let python_loop_header = is_python_loop_header(&line.tokens);
                let inline_python_scope_body = python_inline_suite_body(&line.tokens);
                let inline_python_function_body = python_inline_function_body(&line.tokens);
                let inline_python_class_body =
                    inline_python_scope_body.filter(|_| is_python_class_header(&line.tokens));
                let (context_tokens, inside_function, inside_async_function) =
                    if let Some(body) = inline_python_function_body {
                        (body, true, is_python_async_function_header(&line.tokens))
                    } else if let Some(body) = inline_python_scope_body {
                        (body, false, false)
                    } else {
                        (
                            line.tokens.as_slice(),
                            bindings.inside_function(),
                            bindings.inside_async_function(),
                        )
                    };
                let inline_python_scope = inline_python_scope_body.is_some();
                let inside_inline_python_class = inline_python_class_body.is_some();
                let contextual_function = inside_function && !inside_inline_python_class;
                let has_enclosing_function = if inline_python_scope {
                    bindings.has_function_scope()
                } else {
                    bindings.has_enclosing_function()
                };
                if let Some(kind) = remember_python_declaration_context(
                    &mut python_declaration_contexts,
                    &line.tokens,
                    python_scope_depth,
                ) {
                    problems.push(python_declaration_conflict_diagnostic(kind, line.span));
                    continue;
                }
                bindings.remember_python(&line.tokens, parse_line.indent);
                if depth > 0 && valid_python_header {
                    python_header_indents.push((line.indent, python_loop_header));
                } else if depth == 0 && valid_python_header && python_loop_header {
                    top_level_python_loop_indents.push(line.indent);
                }
                if is_python_async_for_header(&line.tokens) && !bindings.inside_async_function() {
                    problems.push(async_for_outside_async_function_diagnostic(line.span));
                }
                if is_python_async_with_header(&line.tokens) && !bindings.inside_async_function() {
                    problems.push(async_with_outside_async_function_diagnostic(line.span));
                }
                if contains_python_nonlocal(context_tokens) && !has_enclosing_function {
                    problems.push(nonlocal_outside_function_diagnostic(line.span));
                }
                if is_python_import_star_line(context_tokens)
                    && is_valid_python_statement(token_text(source, context_tokens))
                    && (inline_python_scope || bindings.inside_non_module_scope())
                {
                    problems.push(import_star_outside_module_diagnostic(line.span));
                }
                if is_python_return_line(context_tokens) && !contextual_function {
                    problems.push(return_outside_function_diagnostic(line.span));
                    continue;
                }
                let inside_loop = blocks
                    .iter()
                    .any(|block| matches!(block, ExplicitBlock::Loop { .. }))
                    || python_header_indents.iter().any(|(_, is_loop)| *is_loop)
                    || !top_level_python_loop_indents.is_empty()
                    || inside_indented_nme_loop;
                if is_python_continue_line(context_tokens) && (!inside_loop || inline_python_scope)
                {
                    problems.push(continue_outside_loop_diagnostic(line.span));
                    continue;
                }
                if inline_python_scope && is_python_break_line(context_tokens) {
                    problems.push(break_outside_loop_diagnostic(line.span));
                    continue;
                }
                if inside_python_except_star && is_python_except_star_control_line(&line.tokens) {
                    problems.push(except_star_control_flow_diagnostic(line.span));
                    continue;
                }
                if contains_yield_inside_comprehension(context_tokens) {
                    problems.push(yield_inside_comprehension_diagnostic(line.span));
                    continue;
                }
                if contains_async_comprehension_outside_async_function(
                    context_tokens,
                    inside_async_function,
                ) {
                    problems.push(async_comprehension_outside_async_function_diagnostic(
                        line.span,
                    ));
                    continue;
                }
                if let Some(body) = inline_python_function_body {
                    let has_direct_yield = contains_yield_outside_lambda(body)
                        && !contains_yield_inside_comprehension(body);
                    if is_python_async_function_header(&line.tokens)
                        && has_direct_yield
                        && contains_return_with_value(body)
                    {
                        problems.push(return_value_in_async_generator_diagnostic(line.span));
                        continue;
                    }
                } else if !inline_python_scope {
                    remember_async_generator_context(
                        &mut async_function_contexts,
                        &line.tokens,
                        python_scope_depth,
                        line.span,
                    );
                }
                if contains_yield_outside_lambda(context_tokens) && !inside_function {
                    problems.push(yield_outside_function_diagnostic(line.span));
                    continue;
                }
                if contains_invalid_await(context_tokens, inside_async_function) {
                    problems.push(await_outside_async_function_diagnostic(line.span));
                    continue;
                }
                if contains_yield_from_outside_lambda(context_tokens) && inside_async_function {
                    problems.push(yield_from_async_function_diagnostic(line.span));
                }
                if is_python_try_header(&line.tokens) {
                    python_try_indents.push(line.indent);
                }
                if is_python_except_star_header(&line.tokens)
                    && python_try_indents
                        .last()
                        .is_some_and(|header_indent| *header_indent == line.indent)
                {
                    python_except_star_indents.push((line.indent, bindings.python_scope_depth()));
                }
            }
            Err(problem) => problems.push(problem),
        }
    }

    completed_async_functions.extend(async_function_contexts);
    for context in completed_async_functions {
        if context.has_yield {
            for span in context.return_value_spans {
                problems.push(return_value_in_async_generator_diagnostic(span));
            }
        }
    }

    for block in &blocks {
        if let ExplicitBlock::Story(story) = block {
            if !story.has_body {
                problems.push(empty_story_diagnostic(story.header_span));
            }
        }
    }
    if !blocks.is_empty() {
        problems.extend(
            blocks
                .iter()
                .filter(|block| {
                    !block.ends_at_the_end_of_the_file()
                        && !matches!(block, ExplicitBlock::Story(story) if !story.has_body)
                })
                .map(|block| {
                    missing_end_diagnostic(block, lines.last().map_or(0, |line| line.span.end))
                }),
        );
    }

    if problems.is_empty() {
        Ok(ParsedProgram {
            nme_lines: found,
            virtual_indents,
            story_blank_lines,
        })
    } else {
        Err(problems)
    }
}

#[derive(Debug, Clone)]
/// The last one-line conditional, and how many `else` lines it has taken.
///
/// An `else` may only follow its `if` directly, so any other statement clears
/// this. The indent has to match as well: an `else` written further in belongs
/// to something else.
struct InlineBranch {
    indent: usize,
    else_seen: bool,
}

enum ExplicitBlock {
    Loop {
        /// When the block's body started physically indented, the suite
        /// follows ordinary Python dedent rules: a later line at or above
        /// the header level closes it. Flat bodies only close on `end`.
        close_on_dedent: Option<usize>,
    },
    Conditional {
        else_seen: bool,
        close_on_dedent: Option<usize>,
    },
    /// `이야기:` / `story:`. Kept apart from the other two because nothing
    /// inside it is ever read as a command.
    Story(StoryBlock),
    /// `to greet:` / `인사하기라는 일:`. Kept apart because it is neither a
    /// loop nor a condition: `멈춰` may not leave it and `아니면` may not
    /// follow it, and both of those fall out of it having its own variant.
    Job { close_on_dedent: Option<usize> },
}

/// One open story block: how it closes, how its lines are told, and what the
/// compiler needs in order to write a line of its own inside it.
#[derive(Debug, Clone)]
struct StoryBlock {
    /// Indentation of the `이야기:` line, when the body below it was written
    /// indented. `None` when the body is flat, and then only `end`/`끝`
    /// closes the story.
    close_on_dedent: Option<usize>,
    /// Where the `이야기:` line is, so an empty story can point at it.
    header_span: Span,
    /// Whether any line has been told inside this story yet. A story with
    /// none becomes `if True:` with no body, which is not a program.
    has_body: bool,
    /// The pause between two characters, when the story is told slowly.
    seconds: Option<Code>,
    /// Indentation for a line the compiler writes itself — the `print()`
    /// that stands in for a blank line.
    prefix: String,
    /// Physical line the story has been read up to.
    last_line: usize,
}

impl ExplicitBlock {
    fn close_on_dedent(&self) -> Option<usize> {
        match self {
            ExplicitBlock::Loop { close_on_dedent }
            | ExplicitBlock::Job { close_on_dedent }
            | ExplicitBlock::Conditional {
                close_on_dedent, ..
            } => *close_on_dedent,
            ExplicitBlock::Story(story) => story.close_on_dedent,
        }
    }

    /// A flat statement at the block's own level means the suite is flat from
    /// there on, so only an explicit `end` can close it again.
    fn clear_close_on_dedent(&mut self) {
        match self {
            ExplicitBlock::Loop { close_on_dedent }
            | ExplicitBlock::Job { close_on_dedent }
            | ExplicitBlock::Conditional {
                close_on_dedent, ..
            } => {
                *close_on_dedent = None;
            }
            ExplicitBlock::Story(story) => story.close_on_dedent = None,
        }
    }

    fn as_story(&self) -> Option<&StoryBlock> {
        match self {
            ExplicitBlock::Story(story) => Some(story),
            _ => None,
        }
    }

    /// A story whose body was written indented ends where that indentation
    /// ends, and the end of the file is the plainest dedent there is. Every
    /// other block, and a flat story, still needs its `end`/`끝`.
    fn ends_at_the_end_of_the_file(&self) -> bool {
        matches!(self, ExplicitBlock::Story(story) if story.close_on_dedent.is_some())
    }
}

enum BlockCtx<'a> {
    TopLevel {
        line: &'a LogicalLine,
        next_indent: Option<usize>,
    },
    Inline,
}

#[derive(Debug, Clone, Copy)]
enum BranchShape {
    Else,
    ElseIf,
}

fn exact_end(tokens: &[Token]) -> Option<Spelling> {
    if tokens.len() == 1 && token_matches_exact(&tokens[0], END_WORDS_EN) {
        Some(Spelling::English)
    } else if tokens.len() == 1 && token_matches_exact(&tokens[0], END_WORDS_KO) {
        Some(Spelling::Korean)
    } else {
        None
    }
}

fn exact_break(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    let consumed = action_phrase_at(tokens, 0, BREAK_WORDS_EN, MatchMode::Exact)
        .or_else(|| action_phrase_at(tokens, 0, BREAK_WORDS_KO, MatchMode::Exact));
    consumed.is_some_and(|consumed| tokens[consumed..].iter().all(is_command_ending))
}

fn exact_continue(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    let consumed = action_phrase_at(tokens, 0, CONTINUE_WORDS_EN, MatchMode::Exact)
        .or_else(|| action_phrase_at(tokens, 0, CONTINUE_WORDS_KO, MatchMode::Exact));
    consumed.is_some_and(|consumed| tokens[consumed..].iter().all(is_command_ending))
}

/// Loop control written a way that is only safe to read inside an NME block:
/// either with a word Python would keep as a name (`stop`, `quit`), or with
/// one character mistyped (`brek`, `멈처`, `skipp`, `건너뛰여`).
///
/// A name the program has already set is never claimed, so a variable that
/// happens to sit one edit from `break` keeps its own meaning.
fn block_only_loop_control(
    tokens: &[Token],
    known_names: &HashSet<String>,
    words_en: &[&str],
    words_ko: &[&str],
    alias_en: &[&str],
) -> bool {
    if tokens.is_empty() {
        return false;
    }
    if name_word(&tokens[0]).is_some_and(|word| known_names.contains(word)) {
        return false;
    }
    let consumed = action_phrase_at(tokens, 0, alias_en, MatchMode::Exact)
        .or_else(|| action_phrase_at(tokens, 0, words_en, MatchMode::Recover))
        .or_else(|| action_phrase_at(tokens, 0, words_ko, MatchMode::Recover));
    consumed.is_some_and(|consumed| tokens[consumed..].iter().all(is_command_ending))
}

fn branch_shape(tokens: &[Token]) -> Option<BranchShape> {
    if tokens.is_empty() {
        return None;
    }
    if matches!(tokens[0].tok, Tok::Elif)
        || token_matches_exact(&tokens[0], &["elif"])
        || token_matches_exact(
            &tokens[0],
            &[
                "아니면만약",
                "아니면만약에",
                "그렇지않으면만약",
                "그렇지않으면만약에",
            ],
        )
        || (token_matches_exact(&tokens[0], &["아니면", "그렇지않으면"])
            && when_action_at(tokens, 1, MatchMode::Exact).is_some())
        || (action_phrase_at(tokens, 0, ELSE_WORDS_EN, MatchMode::Exact)
            .is_some_and(|consumed| when_action_at(tokens, consumed, MatchMode::Exact).is_some()))
        || (action_phrase_at(tokens, 0, ELSE_WORDS_KO, MatchMode::Exact)
            .is_some_and(|consumed| when_action_at(tokens, consumed, MatchMode::Exact).is_some()))
    {
        return Some(BranchShape::ElseIf);
    }
    (action_phrase_at(tokens, 0, ELSE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, ELSE_WORDS_KO, MatchMode::Exact).is_some())
    .then_some(BranchShape::Else)
}

fn is_korean_branch_alias(tokens: &[Token]) -> bool {
    !tokens.is_empty() && action_phrase_at(tokens, 0, ELSE_WORDS_KO, MatchMode::Exact).is_some()
}

/// `skip` / `건너뛰어` on its own, as the one-line body of an NME block.
fn is_skip_alias(tokens: &[Token]) -> bool {
    action_phrase_at(tokens, 0, CONTINUE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, CONTINUE_WORDS_KO, MatchMode::Exact).is_some()
}

fn is_korean_break_alias(tokens: &[Token]) -> bool {
    action_phrase_at(tokens, 0, BREAK_WORDS_KO, MatchMode::Exact).is_some()
}

fn is_header_shape(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    when_action_at(tokens, 0, MatchMode::Exact).is_some()
        || english_for_each_start(tokens, MatchMode::Exact).is_some()
        || english_for_each_start(tokens, MatchMode::Recover).is_some()
        || korean_for_each_shape(tokens)
        || repeat_action_at(tokens, 0, MatchMode::Exact).is_some()
        || matches!(tokens[0].tok, Tok::While)
        || action_phrase_at(tokens, 0, WHILE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, WHILE_WORDS_KO, MatchMode::Exact).is_some()
        || find_count_marker(tokens, MatchMode::Exact).is_some()
        || tokens.iter().any(|token| {
            name_word(token).is_some_and(|word| {
                word.len() > TIMES_KEYWORD_KO.len() && word.ends_with(TIMES_KEYWORD_KO)
            })
        })
        || tokens.last().is_some_and(|token| {
            token_matches_exact(token, WHILE_WORDS_KO)
                && tokens.len() > 1
                && !output_word_before(tokens, tokens.len() - 1)
        })
        || forever_body_start(tokens, MatchMode::Exact).is_some()
        || subject_condition_shape(tokens)
        || story_colon_shape(tokens)
        || job_header(tokens).is_some()
        || chance_prefix(tokens).is_some_and(|prefix| prefix.consumed == tokens.len())
}

fn has_future_end(lines: &[LogicalLine], index: usize) -> bool {
    lines[index + 1..]
        .iter()
        .any(|line| exact_end(&line.tokens).is_some())
}

/// Number of whole-statement `end`/`끝` lines after `index`. Used to decide
/// whether a dedented header can only be a sibling block (when the remaining
/// `end`s are not enough to close the nested reading anyway).
fn count_remaining_ends(lines: &[LogicalLine], index: usize) -> usize {
    lines[index + 1..]
        .iter()
        .filter(|line| exact_end(&line.tokens).is_some())
        .count()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn validate_branch(
    branch: &BranchShape,
    blocks: &mut [ExplicitBlock],
    span: Span,
    problems: &mut Vec<Diagnostic>,
) -> bool {
    let Some(top) = blocks.last_mut() else {
        problems.push(branch_without_condition_diagnostic(span));
        return false;
    };
    let ExplicitBlock::Conditional { else_seen, .. } = top else {
        problems.push(branch_without_condition_diagnostic(span));
        return false;
    };
    match branch {
        BranchShape::ElseIf if *else_seen => {
            problems.push(duplicate_else_diagnostic(span));
            false
        }
        BranchShape::Else => {
            if *else_seen {
                problems.push(duplicate_else_diagnostic(span));
                false
            } else {
                *else_seen = true;
                true
            }
        }
        BranchShape::ElseIf => true,
    }
}

fn unmatched_end_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::StrayEnd,
        "there is no open NME block for this `end`",
        "이 `끝`을 닫을 열린 NME 블록이 없어요",
        span,
    )
    .with_bilingual_hint(
        "open a `while`, `if`, or `repeat` block first",
        "먼저 `동안`, `만약`, 또는 `반복` 블록을 열어 주세요",
    )
}

fn break_outside_loop_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::BreakOutsideLoop,
        "`break` can only be used inside a loop",
        "`멈춰`는 반복문 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside `while ... end`, `repeat ... end`, or a Python `for`/`while` loop",
        "`동안 ... 끝`, `반복 ... 끝`, 또는 Python `for`/`while` 반복문 안에 넣어 주세요",
    )
}

fn continue_outside_loop_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ContinueOutsideLoop,
        "`continue` can only be used inside a loop",
        "`continue`는 반복문 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside `while`, `repeat`, or a Python `for`/`while` loop, or remove it",
        "`while`, `repeat`, 또는 Python `for`/`while` 반복문 안에 넣거나 지워 주세요",
    )
}

fn return_outside_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ReturnOutsideFunction,
        "`return` can only be used inside a function",
        "`return`은 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside a `def` function, or remove it",
        "`def` 함수 안에 넣거나 지워 주세요",
    )
}

fn yield_outside_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::YieldOutsideFunction,
        "`yield` can only be used inside a function",
        "`yield`는 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside a `def` or `async def` function, or remove it",
        "`def` 또는 `async def` 함수 안에 넣거나 지워 주세요",
    )
}

fn await_outside_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AwaitOutsideAsyncFunction,
        "`await` can only be used inside an async function",
        "`await`는 비동기 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside an `async def` function, or remove it",
        "`async def` 함수 안에 넣거나 지워 주세요",
    )
}

fn yield_from_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::YieldFromAsyncFunction,
        "`yield from` cannot be used inside an async function",
        "비동기 함수 안에서는 `yield from`을 쓸 수 없어요",
        span,
    )
    .with_bilingual_hint(
        "use `async for` to yield values from an async source, or use a normal `def` generator",
        "비동기 원천의 값을 내보내려면 `async for`를 쓰거나 일반 `def` 제너레이터를 사용해 주세요",
    )
}

fn async_for_outside_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AsyncForOutsideAsyncFunction,
        "`async for` can only be used inside an async function",
        "`async for`는 비동기 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside an `async def` function, or use an ordinary `for` loop",
        "`async def` 함수 안에 넣거나 일반 `for` 반복문을 사용해 주세요",
    )
}

fn async_with_outside_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AsyncWithOutsideAsyncFunction,
        "`async with` can only be used inside an async function",
        "`async with`는 비동기 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside an `async def` function, or use an ordinary `with` block",
        "`async def` 함수 안에 넣거나 일반 `with` 블록을 사용해 주세요",
    )
}

fn nonlocal_outside_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::NonlocalOutsideFunction,
        "`nonlocal` can only be used inside a nested function",
        "`nonlocal`은 중첩 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it in a nested function or class under another function, or remove it",
        "다른 함수 아래의 중첩 함수나 클래스에 넣거나 지워 주세요",
    )
}

fn import_star_outside_module_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ImportStarOutsideModule,
        "`from ... import *` can only be used at module scope",
        "`from ... import *`은 모듈 범위에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "import the names explicitly here, or move the star import to the module level",
        "여기서는 이름을 명시적으로 import하거나 별표 import를 모듈 수준으로 옮겨 주세요",
    )
}

fn except_star_control_flow_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ControlFlowInExceptStar,
        "`break`, `continue`, and `return` cannot be used inside an `except*` block",
        "`except*` 블록 안에서는 `break`, `continue`, `return`을 쓸 수 없어요",
        span,
    )
    .with_bilingual_hint(
        "move the control-flow statement outside the `except*` block, or use a normal `except` block",
        "제어 흐름 문장을 `except*` 블록 밖으로 옮기거나 일반 `except` 블록을 사용해 주세요",
    )
}

fn yield_inside_comprehension_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::YieldInsideComprehension,
        "`yield` cannot be used inside a comprehension",
        "컴프리헨션 안에서는 `yield`를 쓸 수 없어요",
        span,
    )
    .with_bilingual_hint(
        "replace the comprehension with an explicit loop, or move `yield` outside it",
        "컴프리헨션을 명시적인 반복문으로 바꾸거나 `yield`를 밖으로 옮겨 주세요",
    )
}

fn async_comprehension_outside_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AsyncComprehensionOutsideAsyncFunction,
        "an async comprehension must be inside an async function",
        "비동기 컴프리헨션은 비동기 함수 안에 있어야 해요",
        span,
    )
    .with_bilingual_hint(
        "move the comprehension into an `async def` function, or use an ordinary `for` comprehension",
        "컴프리헨션을 `async def` 함수 안으로 옮기거나 일반 `for` 컴프리헨션을 사용해 주세요",
    )
}

fn return_value_in_async_generator_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ReturnValueInAsyncGenerator,
        "an async generator cannot return a value",
        "비동기 제너레이터에서는 값을 반환할 수 없어요",
        span,
    )
    .with_bilingual_hint(
        "use a bare `return`, or move the value-returning statement into a separate async function",
        "값이 없는 `return`을 사용하거나 값을 반환하는 문장을 별도의 비동기 함수로 옮겨 주세요",
    )
}

fn python_declaration_conflict_diagnostic(kind: PythonDeclarationKind, span: Span) -> Diagnostic {
    match kind {
        PythonDeclarationKind::Global => Diagnostic::bilingual(
            DiagnosticCode::GlobalDeclarationConflict,
            "`global` conflicts with an earlier name use or assignment",
            "`global` 선언이 앞선 이름 사용이나 대입과 충돌해요",
            span,
        )
        .with_bilingual_hint(
            "move `global` before the first use or assignment, and do not declare a parameter global",
            "첫 사용이나 대입보다 `global`을 먼저 적고, 매개변수를 global로 선언하지 말아 주세요",
        ),
        PythonDeclarationKind::Nonlocal => Diagnostic::bilingual(
            DiagnosticCode::NonlocalDeclarationConflict,
            "`nonlocal` conflicts with an earlier name use or assignment",
            "`nonlocal` 선언이 앞선 이름 사용이나 대입과 충돌해요",
            span,
        )
        .with_bilingual_hint(
            "move `nonlocal` before the first use or assignment, and do not declare a parameter nonlocal",
            "첫 사용이나 대입보다 `nonlocal`을 먼저 적고, 매개변수를 nonlocal로 선언하지 말아 주세요",
        ),
    }
}

fn branch_without_condition_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::BranchWithoutCondition,
        "`else` or `elif` needs an open condition block",
        "`아니면`이나 `elif` 앞에 열린 조건 블록이 필요해요",
        span,
    )
    .with_bilingual_hint(
        "start with `if condition` and close the whole block with `end`",
        "`만약 조건`으로 시작하고 전체 블록을 `끝`으로 닫아 주세요",
    )
}

fn duplicate_else_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::DuplicateElse,
        "this condition already has an `else` branch",
        "이 조건에는 이미 `아니면` 가지가 있어요",
        span,
    )
    .with_bilingual_hint(
        "put another condition before `else`, or close the block",
        "`아니면` 전에 조건을 더 쓰거나 블록을 닫아 주세요",
    )
}

fn inline_break_is_outside_loop(stmt: &NmeStmt, source: &str, inside_loop: bool) -> bool {
    match stmt {
        NmeStmt::Break => !inside_loop,
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::Forever { inline }
        | NmeStmt::While { inline, .. } => inline
            .as_ref()
            .is_some_and(|body| inline_break_is_outside_loop_in_body(body, source, true)),
        NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline
            .as_ref()
            .is_some_and(|body| inline_break_is_outside_loop_in_body(body, source, inside_loop)),
        _ => false,
    }
}

fn inline_break_is_outside_loop_in_body(
    body: &InlineStmt,
    source: &str,
    inside_loop: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_break_is_outside_loop(inner, source, inside_loop),
        InlineStmt::Python(span) => source[span.start..span.end].trim() == "break" && !inside_loop,
    }
}

fn inline_continue_is_outside_loop(stmt: &NmeStmt, tokens: &[Token], inside_loop: bool) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::Forever { inline }
        | NmeStmt::While { inline, .. } => inline
            .as_ref()
            .is_some_and(|body| inline_continue_is_outside_loop_in_body(body, tokens, true)),
        NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline
            .as_ref()
            .is_some_and(|body| inline_continue_is_outside_loop_in_body(body, tokens, inside_loop)),
        _ => false,
    }
}

fn inline_continue_is_outside_loop_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_loop: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_continue_is_outside_loop(inner, tokens, inside_loop),
        InlineStmt::Python(span) => {
            first_token_in_span(tokens, *span)
                .is_some_and(|token| matches!(token.tok, Tok::Continue))
                && !inside_loop
        }
    }
}

fn inline_except_star_control_flow(stmt: &NmeStmt, tokens: &[Token]) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::Forever { inline }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline
            .as_ref()
            .is_some_and(|body| inline_except_star_control_flow_in_body(body, tokens)),
        _ => false,
    }
}

fn inline_except_star_control_flow_in_body(body: &InlineStmt, tokens: &[Token]) -> bool {
    match body {
        InlineStmt::Nme(inner) => {
            matches!(inner.as_ref(), NmeStmt::Break)
                || inline_except_star_control_flow(inner, tokens)
        }
        InlineStmt::Python(span) => first_token_in_span(tokens, *span)
            .is_some_and(|token| matches!(token.tok, Tok::Break | Tok::Continue | Tok::Return)),
    }
}

fn inline_return_is_outside_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_return_is_outside_function_in_body(body, tokens, inside_function)
        }),
        _ => false,
    }
}

fn inline_return_is_outside_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_return_is_outside_function(inner, tokens, inside_function),
        InlineStmt::Python(span) => {
            first_token_in_span(tokens, *span).is_some_and(|token| matches!(token.tok, Tok::Return))
                && !inside_function
        }
    }
}

fn inline_yield_inside_comprehension(stmt: &NmeStmt, tokens: &[Token]) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline
            .as_ref()
            .is_some_and(|body| inline_yield_inside_comprehension_in_body(body, tokens)),
        _ => false,
    }
}

fn inline_yield_inside_comprehension_in_body(body: &InlineStmt, tokens: &[Token]) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_yield_inside_comprehension(inner, tokens),
        InlineStmt::Python(span) => contains_yield_inside_comprehension_in_span(tokens, *span),
    }
}

fn inline_async_comprehension_outside_async_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_async_comprehension_outside_async_function_in_body(
                body,
                tokens,
                inside_async_function,
            )
        }),
        _ => false,
    }
}

fn inline_async_comprehension_outside_async_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => {
            inline_async_comprehension_outside_async_function(inner, tokens, inside_async_function)
        }
        InlineStmt::Python(span) => contains_async_comprehension_outside_async_function_in_span(
            tokens,
            *span,
            inside_async_function,
        ),
    }
}

fn inline_yield_is_outside_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_yield_is_outside_function_in_body(body, tokens, inside_function)
        }),
        _ => false,
    }
}

fn inline_yield_is_outside_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_yield_is_outside_function(inner, tokens, inside_function),
        InlineStmt::Python(span) => {
            contains_yield_outside_lambda_in_span(tokens, *span) && !inside_function
        }
    }
}

fn inline_await_is_outside_async_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_await_is_outside_async_function_in_body(body, tokens, inside_async_function)
        }),
        _ => false,
    }
}

fn inline_await_is_outside_async_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => {
            inline_await_is_outside_async_function(inner, tokens, inside_async_function)
        }
        InlineStmt::Python(span) => {
            contains_invalid_await_in_span(tokens, *span, inside_async_function)
        }
    }
}

fn inline_yield_from_is_in_async_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_yield_from_is_in_async_function_in_body(body, tokens, inside_async_function)
        }),
        _ => false,
    }
}

fn inline_yield_from_is_in_async_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => {
            inline_yield_from_is_in_async_function(inner, tokens, inside_async_function)
        }
        InlineStmt::Python(span) => {
            contains_yield_from_outside_lambda_in_span(tokens, *span) && inside_async_function
        }
    }
}

fn first_token_in_span(tokens: &[Token], span: Span) -> Option<&Token> {
    tokens
        .iter()
        .find(|token| token.span.start >= span.start && token.span.end <= span.end)
}

fn contains_yield_inside_comprehension(tokens: &[Token]) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        matches!(token.tok, Tok::Yield) && yield_is_inside_comprehension(tokens, index)
    })
}

fn contains_yield_inside_comprehension_in_span(tokens: &[Token], span: Span) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        token.span.start >= span.start
            && token.span.end <= span.end
            && matches!(token.tok, Tok::Yield)
            && yield_is_inside_comprehension(tokens, index)
    })
}

fn contains_async_comprehension_outside_async_function(
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    tokens.windows(2).enumerate().any(|(index, pair)| {
        matches!(pair[0].tok, Tok::Async)
            && matches!(pair[1].tok, Tok::For)
            && async_for_is_inside_comprehension(tokens, index)
            && (!inside_async_function || enclosing_lambda_body_start(tokens, index).is_some())
    })
}

fn contains_async_comprehension_outside_async_function_in_span(
    tokens: &[Token],
    span: Span,
    inside_async_function: bool,
) -> bool {
    tokens.windows(2).enumerate().any(|(index, pair)| {
        pair[0].span.start >= span.start
            && pair[1].span.end <= span.end
            && matches!(pair[0].tok, Tok::Async)
            && matches!(pair[1].tok, Tok::For)
            && async_for_is_inside_comprehension(tokens, index)
            && (!inside_async_function || enclosing_lambda_body_start(tokens, index).is_some())
    })
}

fn async_for_is_inside_comprehension(tokens: &[Token], async_index: usize) -> bool {
    let depths = token_depths(tokens);
    let closes = matching_bracket_closes(tokens);
    (0..async_index).any(|open_index| {
        let is_open = matches!(tokens[open_index].tok, Tok::Lpar | Tok::Lsqb | Tok::Lbrace);
        let Some(close_index) = closes[open_index] else {
            return false;
        };
        is_open && async_index < close_index && depths[async_index] == depths[open_index] + 1
    })
}

fn remember_async_generator_context(
    contexts: &mut Vec<AsyncFunctionContext>,
    tokens: &[Token],
    python_scope_depth: usize,
    span: Span,
) {
    let has_direct_yield =
        contains_yield_outside_lambda(tokens) && !contains_yield_inside_comprehension(tokens);
    let has_return_value = contains_return_with_value(tokens);
    if let Some(context) = contexts
        .last_mut()
        .filter(|context| context.body_scope_depth == python_scope_depth)
    {
        context.has_yield |= has_direct_yield;
        if has_return_value {
            context.return_value_spans.push(span);
        }
    }
    if is_python_async_function_header(tokens) {
        contexts.push(AsyncFunctionContext {
            body_scope_depth: python_scope_depth + 1,
            has_yield: false,
            return_value_spans: Vec::new(),
        });
    }
}

fn contains_return_with_value(tokens: &[Token]) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        matches!(token.tok, Tok::Return)
            && tokens
                .get(index + 1)
                .is_some_and(|next| !matches!(next.tok, Tok::Semi))
    })
}

fn remember_python_declaration_context(
    contexts: &mut Vec<PythonDeclarationContext>,
    tokens: &[Token],
    python_scope_depth: usize,
) -> Option<PythonDeclarationKind> {
    if let Some(body) = python_inline_suite_body(tokens) {
        if let Some(context) = contexts
            .last_mut()
            .filter(|context| context.body_scope_depth == python_scope_depth)
        {
            for name in python_names_seen_in_scope(tokens, &[]) {
                context.seen_names.insert(name);
            }
        }
        let (_, parameters) = python_scope_header(tokens).expect("inline Python scope header");
        let mut inline_context = PythonDeclarationContext {
            body_scope_depth: python_scope_depth + 1,
            seen_names: parameters,
            annotation_targets: HashSet::new(),
            declarations: HashMap::new(),
        };
        let declarations = python_declarations(body);
        return remember_python_declarations_in_scope(&mut inline_context, body, &declarations);
    }

    let declarations = if python_scope_header(tokens).is_some() {
        Vec::new()
    } else {
        python_declarations(tokens)
    };
    let conflict = contexts
        .last_mut()
        .filter(|context| context.body_scope_depth == python_scope_depth)
        .and_then(|context| remember_python_declarations_in_scope(context, tokens, &declarations));

    if let Some((_, parameters)) = python_scope_header(tokens) {
        contexts.push(PythonDeclarationContext {
            body_scope_depth: python_scope_depth + 1,
            seen_names: parameters,
            annotation_targets: HashSet::new(),
            declarations: HashMap::new(),
        });
    }
    conflict
}

fn remember_python_declarations_in_scope(
    context: &mut PythonDeclarationContext,
    tokens: &[Token],
    declarations: &[PythonDeclaration],
) -> Option<PythonDeclarationKind> {
    let mut conflict = None;
    for (declaration_index, declaration) in declarations.iter().enumerate() {
        let declaration_start = declaration
            .names
            .first()
            .map_or(0, |(_, name_index)| name_index.saturating_sub(1));
        for name in python_names_seen_in_scope(
            &tokens[..declaration_start],
            &declarations[..declaration_index],
        ) {
            context.seen_names.insert(name);
        }
        for name in python_annotation_target_names(&tokens[..declaration_start]) {
            context.annotation_targets.insert(name);
        }
        for (name, _) in &declaration.names {
            let has_other_declaration = context
                .declarations
                .get(name)
                .is_some_and(|previous| *previous != declaration.kind);
            let has_annotation_target = context.annotation_targets.contains(name);
            if conflict.is_none()
                && (has_other_declaration
                    || context.seen_names.contains(name)
                    || has_annotation_target)
            {
                conflict = Some(declaration.kind);
            }
            context
                .declarations
                .entry(name.clone())
                .or_insert(declaration.kind);
        }
    }
    for name in python_annotation_target_names(tokens) {
        if context.body_scope_depth != 0 && conflict.is_none() {
            if let Some(kind) = context.declarations.get(&name) {
                conflict = Some(*kind);
            }
        }
        context.annotation_targets.insert(name);
    }
    for name in python_names_seen_in_scope(tokens, declarations) {
        context.seen_names.insert(name);
    }
    conflict
}

fn python_declarations(tokens: &[Token]) -> Vec<PythonDeclaration> {
    let mut declarations = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let kind = match tokens[index].tok {
            Tok::Global => PythonDeclarationKind::Global,
            Tok::Nonlocal => PythonDeclarationKind::Nonlocal,
            _ => {
                index += 1;
                continue;
            }
        };
        if index > 0 && !matches!(tokens[index - 1].tok, Tok::Semi | Tok::Colon | Tok::Newline) {
            index += 1;
            continue;
        }
        let mut names = Vec::new();
        let mut cursor = index + 1;
        while cursor < tokens.len() && !matches!(tokens[cursor].tok, Tok::Semi) {
            if let Tok::Name { name } = &tokens[cursor].tok {
                let previous_is_separator =
                    cursor == index + 1 || matches!(tokens[cursor - 1].tok, Tok::Comma);
                if previous_is_separator {
                    names.push((name.clone(), cursor));
                }
            }
            cursor += 1;
        }
        if !names.is_empty() {
            declarations.push(PythonDeclaration { kind, names });
        }
        index = cursor;
    }
    declarations
}

fn python_names_seen_in_scope(tokens: &[Token], declarations: &[PythonDeclaration]) -> Vec<String> {
    let declared_indices: HashSet<usize> = declarations
        .iter()
        .flat_map(|declaration| declaration.names.iter().map(|(_, index)| *index))
        .collect();
    if let Some((name, _)) = python_scope_header(tokens) {
        return vec![name];
    }
    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            let Tok::Name { name } = &token.tok else {
                return None;
            };
            if declared_indices.contains(&index)
                || index > 0 && matches!(tokens[index - 1].tok, Tok::Dot)
                || is_python_keyword_argument_name(tokens, index)
                || token_is_inside_lambda(tokens, index)
                || is_lambda_parameter_name(tokens, index)
                || is_python_annotation_target_name(tokens, index)
                || is_comprehension_local_name(tokens, index)
            {
                None
            } else {
                Some(name.clone())
            }
        })
        .collect()
}

fn python_annotation_target_names(tokens: &[Token]) -> Vec<String> {
    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if !is_python_annotation_target_name(tokens, index) {
                return None;
            }
            let Tok::Name { name } = &token.tok else {
                return None;
            };
            Some(name.clone())
        })
        .collect()
}

fn is_python_keyword_argument_name(tokens: &[Token], index: usize) -> bool {
    tokens
        .get(index + 1)
        .is_some_and(|token| matches!(token.tok, Tok::Equal))
        && tokens
            .get(index.wrapping_sub(1))
            .is_some_and(|token| matches!(token.tok, Tok::Lpar | Tok::Comma))
}

fn is_lambda_parameter_name(tokens: &[Token], index: usize) -> bool {
    let depths = token_depths(tokens);
    let Some(lambda_index) = (0..index).rev().find(|&candidate| {
        if !matches!(tokens[candidate].tok, Tok::Lambda) {
            return false;
        }
        (candidate + 1..tokens.len()).any(|colon| {
            depths[colon] == depths[candidate]
                && matches!(tokens[colon].tok, Tok::Colon)
                && index < colon
        })
    }) else {
        return false;
    };
    let Some(colon_index) = (lambda_index + 1..tokens.len()).find(|&candidate| {
        depths[candidate] == depths[lambda_index] && matches!(tokens[candidate].tok, Tok::Colon)
    }) else {
        return false;
    };
    if index >= colon_index || !matches!(tokens[index].tok, Tok::Name { .. }) {
        return false;
    }
    let mut in_default = false;
    for candidate in lambda_index + 1..index {
        if depths[candidate] != depths[lambda_index] {
            continue;
        }
        match tokens[candidate].tok {
            Tok::Equal => in_default = true,
            Tok::Comma => in_default = false,
            _ => {}
        }
    }
    !in_default
}

fn is_python_annotation_target_name(tokens: &[Token], index: usize) -> bool {
    if !matches!(
        tokens.get(index).map(|token| &token.tok),
        Some(Tok::Name { .. })
    ) {
        return false;
    }
    let depths = token_depths(tokens);
    let Some(next) = tokens.get(index + 1) else {
        return false;
    };
    if !matches!(next.tok, Tok::Colon) || depths[index] != depths[index + 1] {
        return false;
    }
    index == 0
        || tokens
            .get(index.wrapping_sub(1))
            .is_some_and(|previous| matches!(previous.tok, Tok::Comma | Tok::Semi))
}

fn is_comprehension_local_name(tokens: &[Token], index: usize) -> bool {
    let depths = token_depths(tokens);
    let closes = matching_bracket_closes(tokens);
    (0..index).any(|open_index| {
        if !matches!(tokens[open_index].tok, Tok::Lpar | Tok::Lsqb | Tok::Lbrace) {
            return false;
        }
        let Some(close_index) = closes[open_index] else {
            return false;
        };
        if index >= close_index {
            return false;
        }
        let body_depth = depths[open_index] + 1;
        let for_indices: Vec<usize> = (open_index + 1..close_index)
            .filter(|&candidate| {
                depths[candidate] == body_depth && matches!(tokens[candidate].tok, Tok::For)
            })
            .collect();
        let Some(first_for) = for_indices.first().copied() else {
            return false;
        };
        if index > open_index && index < first_for && depths[index] >= body_depth {
            return true;
        }
        for for_index in for_indices {
            let Some(in_index) = (for_index + 1..close_index).find(|&candidate| {
                depths[candidate] == body_depth && matches!(tokens[candidate].tok, Tok::In)
            }) else {
                continue;
            };
            if (for_index + 1..in_index).contains(&index) && depths[index] >= body_depth {
                return true;
            }
            let next_for = (in_index + 1..close_index).find(|&candidate| {
                depths[candidate] == body_depth && matches!(tokens[candidate].tok, Tok::For)
            });
            let segment_end = next_for.unwrap_or(close_index);
            if index > in_index
                && index < segment_end
                && (in_index + 1..index).any(|candidate| {
                    depths[candidate] == body_depth && matches!(tokens[candidate].tok, Tok::If)
                })
            {
                return true;
            }
        }
        false
    })
}

fn yield_is_inside_comprehension(tokens: &[Token], target_index: usize) -> bool {
    let depths = token_depths(tokens);
    let closes = matching_bracket_closes(tokens);
    let lambda_body_start = enclosing_lambda_body_start(tokens, target_index);
    (0..target_index).any(|open_index| {
        let is_open = matches!(tokens[open_index].tok, Tok::Lpar | Tok::Lsqb | Tok::Lbrace);
        let Some(close_index) = closes[open_index] else {
            return false;
        };
        if !is_open || target_index >= close_index {
            return false;
        }
        let body_depth = depths[open_index] + 1;
        let has_comprehension_for = (open_index + 1..close_index)
            .any(|index| depths[index] == body_depth && matches!(tokens[index].tok, Tok::For));
        has_comprehension_for
            && lambda_body_start.is_none_or(|lambda_start| open_index >= lambda_start)
    })
}

fn matching_bracket_closes(tokens: &[Token]) -> Vec<Option<usize>> {
    let mut stack = Vec::new();
    let mut closes = vec![None; tokens.len()];
    for (index, token) in tokens.iter().enumerate() {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => stack.push(index),
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => {
                if let Some(open_index) = stack.pop() {
                    closes[open_index] = Some(index);
                }
            }
            _ => {}
        }
    }
    closes
}

fn contains_yield_outside_lambda(tokens: &[Token]) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        matches!(token.tok, Tok::Yield) && !token_is_inside_lambda(tokens, index)
    })
}

fn contains_yield_outside_lambda_in_span(tokens: &[Token], span: Span) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        token.span.start >= span.start
            && token.span.end <= span.end
            && matches!(token.tok, Tok::Yield)
            && !token_is_inside_lambda(tokens, index)
    })
}

fn contains_invalid_await(tokens: &[Token], inside_async_function: bool) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        matches!(token.tok, Tok::Await)
            && (token_is_inside_lambda(tokens, index) || !inside_async_function)
    })
}

fn contains_invalid_await_in_span(
    tokens: &[Token],
    span: Span,
    inside_async_function: bool,
) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        token.span.start >= span.start
            && token.span.end <= span.end
            && matches!(token.tok, Tok::Await)
            && (token_is_inside_lambda(tokens, index) || !inside_async_function)
    })
}

fn contains_yield_from_outside_lambda(tokens: &[Token]) -> bool {
    tokens.windows(2).enumerate().any(|(index, pair)| {
        matches!(pair[0].tok, Tok::Yield)
            && matches!(pair[1].tok, Tok::From)
            && !token_is_inside_lambda(tokens, index)
    })
}

fn contains_yield_from_outside_lambda_in_span(tokens: &[Token], span: Span) -> bool {
    tokens.windows(2).enumerate().any(|(index, pair)| {
        pair[0].span.start >= span.start
            && pair[1].span.end <= span.end
            && matches!(pair[0].tok, Tok::Yield)
            && matches!(pair[1].tok, Tok::From)
            && !token_is_inside_lambda(tokens, index)
    })
}

fn token_is_inside_lambda(tokens: &[Token], target_index: usize) -> bool {
    enclosing_lambda_body_start(tokens, target_index).is_some()
}

fn enclosing_lambda_body_start(tokens: &[Token], target_index: usize) -> Option<usize> {
    let depths = token_depths(tokens);
    (0..target_index).rev().find_map(|lambda_index| {
        if !matches!(tokens[lambda_index].tok, Tok::Lambda) {
            return None;
        }
        let lambda_depth = depths[lambda_index];
        let colon_index = (lambda_index + 1..target_index).find(|&index| {
            depths[index] == lambda_depth && matches!(tokens[index].tok, Tok::Colon)
        })?;
        let body_ends_before_target = (colon_index + 1..target_index).any(|index| {
            depths[index] == lambda_depth
                && matches!(
                    tokens[index].tok,
                    Tok::Comma | Tok::Semi | Tok::Rpar | Tok::Rsqb | Tok::Rbrace
                )
        });
        (!body_ends_before_target).then_some(colon_index + 1)
    })
}

fn token_depths(tokens: &[Token]) -> Vec<usize> {
    let mut depth = 0usize;
    tokens
        .iter()
        .map(|token| {
            let before = depth;
            match token.tok {
                Tok::Rpar | Tok::Rsqb | Tok::Rbrace => {
                    depth = depth.saturating_sub(1);
                }
                Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
                _ => {}
            }
            before
        })
        .collect()
}

fn is_python_return_line(tokens: &[Token]) -> bool {
    has_direct_python_statement(tokens, |tok| matches!(tok, Tok::Return))
}

fn is_python_continue_line(tokens: &[Token]) -> bool {
    has_direct_python_statement(tokens, |tok| matches!(tok, Tok::Continue))
}

fn is_python_break_line(tokens: &[Token]) -> bool {
    has_direct_python_statement(tokens, |tok| matches!(tok, Tok::Break))
}

fn has_direct_python_statement<F>(tokens: &[Token], predicate: F) -> bool
where
    F: Fn(&Tok) -> bool,
{
    let depths = token_depths(tokens);
    tokens.iter().enumerate().any(|(index, token)| {
        depths[index] == 0
            && predicate(&token.tok)
            && (index == 0
                || (depths[index - 1] == 0 && matches!(tokens[index - 1].tok, Tok::Semi)))
    })
}

/// True when this line is allowed to be followed by a more deeply indented
/// one: it ends with `:`, it is an NME block header, or it starts with a
/// Python keyword that opens a suite. A broken Python header counts, so its
/// own diagnostic speaks instead of a second one about the indentation.
fn opens_a_suite(source: &str, line: &LogicalLine) -> bool {
    if line.tokens.is_empty() {
        return false;
    }
    token_text(source, &line.tokens).ends_with(':')
        || is_header_shape(&line.tokens)
        || matches!(
            line.tokens[0].tok,
            Tok::If
                | Tok::For
                | Tok::While
                | Tok::Def
                | Tok::Class
                | Tok::Try
                | Tok::Except
                | Tok::Else
                | Tok::Elif
                | Tok::Finally
                | Tok::With
                | Tok::Async
                | Tok::Match
                | Tok::Case
        )
}

/// `  say hello` at the top of a file is an ordinary slip, and CPython
/// answers it with `IndentationError: unexpected indent`. NME says which
/// line starts with a space and what opens a block instead.
fn unexpected_indent_diagnostic(source: &str, line: &LogicalLine) -> Diagnostic {
    let line_start = source[..line.span.start]
        .rfind('\n')
        .map_or(0, |offset| offset + 1);
    Diagnostic::bilingual(
        DiagnosticCode::UnexpectedIndent,
        "this line starts with a space, and nothing above it opens a block",
        "이 줄이 공백으로 시작하는데, 위에 블록을 여는 줄이 없어요",
        Span::new(line_start, line.span.start),
    )
    .with_bilingual_hint(
        "delete the spaces at the start of the line, or open a block above it with `repeat 3 times`, `if ...`, or `for ...:`",
        "줄 앞의 공백을 지우거나, 위에 `3번 반복해`나 `만약에 ...` 같은 블록을 여는 줄을 적어 주세요",
    )
}

/// The `end` closes nothing because the line meant to open the block was
/// never understood. Pointing at the `end` hides the real mistake, so the
/// header is named instead.
/// The last line before `index` that reads like a block header and yet
/// opened no block. Every `end` with nothing to close is a consequence; the
/// cause is up there, and naming the `end` hides it.
fn unreadable_block_header_before(
    source: &str,
    lines: &[LogicalLine],
    index: usize,
    block_header_lines: &HashSet<usize>,
) -> Option<Span> {
    lines[..index]
        .iter()
        .enumerate()
        .rev()
        .find_map(|(at, line)| {
            if block_header_lines.contains(&at) || line.tokens.is_empty() {
                return None;
            }
            let text = token_text(source, &line.tokens);
            if is_valid_python_header(text) || is_valid_python_statement(text) {
                return None;
            }
            let python_suite_word = matches!(
                line.tokens[0].tok,
                Tok::For | Tok::While | Tok::If | Tok::Def | Tok::Class | Tok::With
            );
            (is_header_shape(&line.tokens) || python_suite_word).then_some(line.span)
        })
}

fn unreadable_block_header_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::StrayEnd,
        "this line looks like the start of a block, but I could not read it, so the `end` below closes nothing",
        "이 줄은 블록을 여는 줄로 보이는데 읽지 못했어요. 그래서 아래의 `끝`이 닫을 블록이 없어요",
        span,
    )
    .with_bilingual_hint(
        "fix this line first, for example `repeat 3 times`, `for each name in names`, or `if ready`",
        "이 줄을 먼저 고쳐 주세요. 예를 들어 `3번 반복해`, `이름들 마다`, `만약에 준비가 있으면`처럼 씁니다",
    )
}

fn empty_story_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::StoryEmpty,
        "this story has nothing in it",
        "이 이야기 안에 글이 한 줄도 없어요",
        span,
    )
    .with_bilingual_hint(
        "write the story on the lines below, and close it with `end`",
        "아래 줄에 이야기를 적고 `끝`으로 닫아 주세요",
    )
}

fn missing_end_diagnostic(block: &ExplicitBlock, offset: usize) -> Diagnostic {
    let (english, korean) = match block {
        ExplicitBlock::Loop { .. } => (
            "this loop is missing its closing `end`",
            "이 반복문에는 닫는 `끝`이 필요해요",
        ),
        ExplicitBlock::Conditional { .. } => (
            "this condition is missing its closing `end`",
            "이 조건문에는 닫는 `끝`이 필요해요",
        ),
        ExplicitBlock::Story(_) => (
            "this story is missing its closing `end`",
            "이 이야기에는 닫는 `끝`이 필요해요",
        ),
        ExplicitBlock::Job { .. } => (
            "this job is missing its closing `end`",
            "이 일에는 닫는 `끝`이 필요해요",
        ),
    };
    Diagnostic::bilingual(
        DiagnosticCode::MissingEnd,
        english,
        korean,
        Span::new(offset, offset),
    )
    .with_bilingual_hint(
        "add `end`/`끝` on a line by itself",
        "줄 하나에 `end` 또는 `끝`만 적어 주세요",
    )
}

#[allow(clippy::too_many_lines)]
fn classify(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    debug_assert!(!tokens.is_empty());

    let text = token_text(source, tokens);
    if is_valid_python_statement(text) || is_valid_python_header(text) {
        // Python accepts a few whole lines that cannot do anything at all.
        // Letting Python "win" one of those means shipping a program that
        // silently does nothing, so they are named here instead, and the ones
        // that are plainly written sentences fall through and print.
        if let Some(problem) = statement_does_nothing(tokens) {
            return Err(problem);
        }
        if !valid_python_is_a_sentence(tokens, known_names) {
            return Ok(None);
        }
    }

    // Future Python grammar may be newer than rustpython-parser. A
    // call/attribute/subscript shape is never NME's whitespace-led beginner
    // form, so preserve it for the selected CPython instead of hijacking it.
    // `Hello. Goodbye` wears that shape too, and it is not Python anybody
    // wrote: no name on it was ever given a value, so the "call" is a
    // `NameError` waiting to happen on a line the writer read as a sentence.
    if looks_like_python_invocation(tokens)
        && !is_header_shape(tokens)
        && !is_dotted_words_only(tokens, known_names)
    {
        return Ok(None);
    }
    // rustpython-parser can lag behind the CPython selected by the CLI (for
    // example, Python 3.14 t-strings). An adjacent name+string prefix is a
    // strong signal for a newer string-prefix grammar rather than
    // conversational NME. Preserve it byte-for-byte; `nme check` and
    // `nme build` will ask the real CPython whether they are valid.
    if looks_like_future_python(tokens) {
        return Ok(None);
    }

    // `3번반복해서 안녕 말해줘` — the repeat word is stuck to the counter, so
    // every matcher below would read it as part of the message. It has to be
    // caught before the output word at the end of the line claims it.
    if let Some((index, split)) = glued_count_and_repeat(tokens) {
        let word = name_word(&tokens[index]).unwrap_or("");
        return Err(glued_word_diagnostic(&tokens[index], word, &split));
    }

    // `item 0 of friends` / `친구들 0번째`. Python would hand back the last
    // item; the writer asked for one before the first. Say so.
    if let Some(problem) = zero_item_position(tokens, known_names) {
        return Err(problem);
    }

    // `할 일은 목록` — a list line whose name was written as two words.
    if let Some(problem) = name_written_with_a_space(source, tokens, known_names) {
        return Err(problem);
    }

    // `별들을 이어 말해줘` · `show stars joined`. A join with no separator used
    // to print itself, which reads like success. Named here, before any
    // matcher can turn it into text.
    if let Some(problem) = join_without_a_separator(tokens, known_names) {
        return Err(problem);
    }

    // A story block and a chance are read before every other sentence
    // matcher. Both hang on punctuation the rest of the grammar never uses —
    // a closing colon, and a `%` — so no ordinary sentence can reach them.
    if let Some(stmt) = match_story(source, tokens) {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_chance_set(source, tokens, known_names)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_chance(source, tokens, block, known_names)? {
        return Ok(Some(stmt));
    }

    // A named job hangs on the same kind of structure a story does: a closing
    // colon and a block underneath. A line that runs one hangs on something
    // stronger still — a name the program has already made a job — so both
    // are read before any word-led matcher can claim their ordinary words.
    if let Some(stmt) = match_job(tokens, block) {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_run_job(tokens, known_names)? {
        return Ok(Some(stmt));
    }

    // `말해 봐야 소용없는 일이었습니다` · `물어봐 주셔서 감사합니다`. An action
    // word with a helper verb straight after it is half of a compound verb,
    // and the line ends the way a written Korean sentence ends. Both halves
    // are needed: without the helper `말해줘 비가 쏟아졌습니다` would stop
    // printing the rain, and without the ending `말해 봐` is still output.
    //
    // Unless the compound verb is inside a question. `주문을 물어봐 마법의
    // 주문을 말해 보세요` names what it is asking for before it asks, and the
    // rest of the line is the text shown while it waits; a helper verb in
    // that text is not the line's own verb.
    if korean_action_word_carries_an_auxiliary(tokens)
        && is_written_korean_sentence(tokens, known_names)
        && !korean_question_owns_every_auxiliary(tokens)
    {
        return Ok(Some(NmeStmt::Say {
            value: Value::Text(make_text_template(source, tokens, known_names)),
        }));
    }

    // A natural condition may start with its subject (`색이 빨강과 같으면
    // ...`) instead of an explicit `if`/`만약`. Check this before value-change
    // recovery so a misspelled action such as `말해` is not mistaken for
    // `더해`.
    if let Some(stmt) = match_subject_when(source, tokens, block, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }
    // Let a structured subject-first condition with a one-edit connector
    // typo win over an exact output word at the end (`이름이 있으먄
    // 안녕 말해줘`). Ordinary prose is guarded by the connector-shape checks
    // inside `match_subject_when`.
    if let Some(stmt) = match_subject_when(source, tokens, block, known_names, MatchMode::Recover)?
    {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_update(source, tokens, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_break(source, tokens, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_while(source, tokens, block, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_branch(source, tokens, block, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }

    // `from "module.nme" import names` is not valid Python, so the keyword
    // gate below must not swallow it.
    if matches!(tokens.first().map(|token| &token.tok), Some(Tok::From))
        && tokens
            .get(1)
            .is_some_and(|token| matches!(token.tok, Tok::String { .. }))
    {
        if let Some(stmt) = match_module_import(source, tokens, known_names, MatchMode::Exact)? {
            return Ok(Some(stmt));
        }
    }

    // `use greet from "helper.nme"` / `"helper.nme"에서 greet 가져와`. Read
    // before the bundled-module statement, which would otherwise see `use`
    // and complain that there is no module called `greet`.
    if tokens.iter().any(|token| is_nme_path(source, token)) {
        if let Some(stmt) =
            match_sentence_module_import(source, tokens, known_names, MatchMode::Exact)?
        {
            return Ok(Some(stmt));
        }
    }

    // `for each name in names` is not valid Python either, and `for` is a
    // Python keyword, so it has to be recognized before the gate below. The
    // same is true of `for eahc name in names`: without the recovery here,
    // the keyword gate would hand the whole line back to Python.
    if english_for_each_start(tokens, MatchMode::Exact).is_some() {
        return match_for_each(source, tokens, block, known_names, MatchMode::Exact);
    }
    if english_for_each_start(tokens, MatchMode::Recover).is_some() {
        return match_for_each(source, tokens, block, known_names, MatchMode::Recover);
    }

    if is_python_keyword(&tokens[0].tok) && !matches!(tokens[0].tok, Tok::If) {
        return Ok(None);
    }

    macro_rules! exact_match {
        ($matcher:expr) => {
            if let Some(stmt) = $matcher? {
                return Ok(Some(stmt));
            }
        };
    }
    // These four run before the older actions because each ends with a word
    // the output vocabulary would otherwise claim: `기다려`, `건너뛰어`, `넣어`,
    // and the `마다` loop shape.
    // The screen and timing sentences come first for the same reason: each
    // of them ends in, or contains, a word one of the older actions would
    // otherwise claim (`기다려`, `걸어`, a number of seconds, an output word).
    exact_match!(match_say_slowly(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    exact_match!(match_say_in_box(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    exact_match!(match_say_in_middle(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    if let Some(stmt) = match_clear_screen(tokens, MatchMode::Exact) {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_draw_line(tokens, MatchMode::Exact) {
        return Ok(Some(stmt));
    }
    // Before every output word: the screen phrase says where the message
    // goes, and the words in front of it are the message, not part of it.
    if let Some(stmt) = match_say_on_screen(source, tokens, known_names) {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_start_timer(tokens, MatchMode::Exact) {
        return Ok(Some(stmt));
    }
    exact_match!(match_cooldown(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    if let Some(stmt) = match_cooldown_wait(tokens, known_names, MatchMode::Exact) {
        return Ok(Some(stmt));
    }
    exact_match!(match_wait(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_continue(tokens, MatchMode::Exact));
    // A loop with no counter has to be read before the repeat word sends the
    // line to the counted loop, which would then ask where its number is.
    exact_match!(match_forever(
        source,
        tokens,
        block,
        known_names,
        MatchMode::Exact
    ));
    exact_match!(match_arrange(tokens, known_names, MatchMode::Exact));
    // Before the list statement: a record line uses the same verb and the
    // same container particle, and only the extra marks tell them apart.
    exact_match!(match_record_put(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    exact_match!(match_append(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_for_each(
        source,
        tokens,
        block,
        known_names,
        MatchMode::Exact
    ));

    if when_action_at(tokens, 0, MatchMode::Exact).is_some() {
        // `혹시` may decline the line (it is also a politeness filler), so the
        // remaining matchers still get their turn.
        exact_match!(match_when(
            source,
            tokens,
            block,
            known_names,
            MatchMode::Exact
        ));
    }
    if repeat_action_at(tokens, 0, MatchMode::Exact).is_some() {
        return match_times(source, tokens, block, known_names, MatchMode::Exact);
    }
    if ask_action_at(tokens, 0, MatchMode::Exact).is_some() {
        // An `ask` at the start of a line does not make the rest of the line
        // a question: `ask Mum about the recipe` asks nobody anything. When
        // `match_ask` declines, the line goes on to the sentence path rather
        // than being handed to Python, which would answer a written sentence
        // with CPython's own `SyntaxError`.
        exact_match!(match_ask(source, tokens, known_names, MatchMode::Exact));
    }
    if output_action_at(tokens, 0, MatchMode::Exact).is_some() {
        return match_say(source, tokens, known_names, MatchMode::Exact);
    }
    if set_action_at(tokens, 0, MatchMode::Exact).is_some() {
        // Likewise `set the table for four people`: a `set` at the start of
        // a line of ordinary words is part of the sentence.
        exact_match!(match_set(source, tokens, known_names, MatchMode::Exact));
    }
    if action_phrase_at(tokens, 0, USE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, USE_WORDS_KO, MatchMode::Exact).is_some()
    {
        // When the module matcher declines, the line goes on to the sentence
        // path rather than back to Python — the same way `match_ask` and
        // `match_set` do. `사용 설명서를 잃어버렸습니다` opens with `사용` and is
        // a lost manual; handing it to Python answered it with CPython's own
        // `SyntaxError`.
        if recoverable_module_shape(tokens) {
            exact_match!(match_use_module(
                source,
                tokens,
                known_names,
                MatchMode::Recover
            ));
        } else {
            exact_match!(match_use_module(
                source,
                tokens,
                known_names,
                MatchMode::Exact
            ));
        }
    }
    if action_phrase_at(tokens, 0, FILE_READ_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, FILE_WRITE_WORDS_EN, MatchMode::Exact).is_some()
        || tokens.iter().any(|token| {
            action_phrase_at(
                std::slice::from_ref(token),
                0,
                FILE_READ_WORDS_KO,
                MatchMode::Exact,
            )
            .is_some()
        })
        || tokens.iter().any(|token| {
            action_phrase_at(
                std::slice::from_ref(token),
                0,
                FILE_WRITE_WORDS_KO,
                MatchMode::Exact,
            )
            .is_some()
        })
    {
        exact_match!(match_file_io(source, tokens, known_names, MatchMode::Exact));
    }
    exact_match!(match_when(
        source,
        tokens,
        block,
        known_names,
        MatchMode::Exact
    ));
    exact_match!(match_times(
        source,
        tokens,
        block,
        known_names,
        MatchMode::Exact
    ));
    // A count marker followed by a one-edit repeat action is stronger
    // sentence structure than an exact output action at the end of the line.
    // For example, `2번 반목해서 다시 말해줘` should recover `반복해서`
    // instead of printing the entire prefix as plain text.
    if has_recoverable_repeat_shape(tokens) {
        exact_match!(match_times(
            source,
            tokens,
            block,
            known_names,
            MatchMode::Recover
        ));
    }
    // A misspelled condition starter followed by a real connector is a
    // stronger sentence shape than an exact output word at the end. Without
    // this early recovery, `만악에 이름이 있으면 안녕 말해줘` would be read as
    // plain output because `말해줘` is exact.
    let recoverable_condition_starter = when_action_at(tokens, 0, MatchMode::Exact).is_none()
        && when_action_at(tokens, 0, MatchMode::Recover)
            .is_some_and(|(_, consumed)| find_condition_connector(&tokens[consumed..]).is_some());
    if recoverable_condition_starter {
        exact_match!(match_when(
            source,
            tokens,
            block,
            known_names,
            MatchMode::Recover
        ));
    }
    exact_match!(match_ask(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_say(source, tokens, known_names, MatchMode::Exact));

    // Check this before Korean assignment particles such as `은`/`는` can
    // make `이름은 뭐예요?` look like a sentence assignment. Explicit output
    // and ask actions above still win when the learner writes them.
    if let Some(stmt) = match_natural_question(source, tokens, known_names) {
        return Ok(Some(stmt));
    }

    exact_match!(match_set(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_use_module(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));

    // A bare contraction such as `Don't stop!` can put `Don` one edit away
    // from the repeat alias `do`, and `It's easy` can put `It` near `if`.
    // When no complete NME shape is present, ordinary word-like input should
    // win over those weak typo candidates.
    if looks_like_plain_prose(tokens) && !has_recoverable_sentence_shape(tokens, known_names) {
        // Printing the line is right for prose and wrong for a command whose
        // action word NME does not accept, so the near miss is named first.
        if near_miss_action_word(tokens, known_names).is_some() {
            return Err(unknown_action_word_diagnostic(tokens, known_names));
        }
        // `겨울을 나려고 곡식을 저장했습니다` — `저장했습니다` is one word, and
        // splitting it into `저장 했습니다` names a saving word nobody wrote.
        if !is_written_korean_sentence(tokens, known_names) {
            if let Some((index, split)) = glued_action_word(tokens) {
                let word = name_word(&tokens[index]).unwrap_or("");
                return Err(glued_word_diagnostic(&tokens[index], word, &split));
            }
        }
        let value = parse_value(source, tokens, known_names, true)
            .map_err(|()| missing_action_diagnostic(tokens))?;
        return Ok(Some(NmeStmt::Say { value }));
    }

    let recovered = [
        match_subject_when(source, tokens, block, known_names, MatchMode::Recover),
        match_update(source, tokens, known_names, MatchMode::Recover),
        match_when(source, tokens, block, known_names, MatchMode::Recover),
        match_times(source, tokens, block, known_names, MatchMode::Recover),
        match_ask(source, tokens, known_names, MatchMode::Recover),
        match_say(source, tokens, known_names, MatchMode::Recover),
        match_set(source, tokens, known_names, MatchMode::Recover),
        match_use_module(source, tokens, known_names, MatchMode::Recover),
        match_file_io(source, tokens, known_names, MatchMode::Recover),
        // Waiting, adding to a list, and looping over a list are ordinary
        // first-week statements, so their action words earn the same one-edit
        // repair the older ones have always had.
        match_wait(source, tokens, known_names, MatchMode::Recover),
        match_break(source, tokens, known_names, MatchMode::Recover),
        match_continue(tokens, MatchMode::Recover),
        match_append(source, tokens, known_names, MatchMode::Recover),
        match_for_each(source, tokens, block, known_names, MatchMode::Recover),
    ];
    let mut candidates = Vec::new();
    let mut recovery_problems = Vec::new();
    for result in recovered {
        match result {
            Ok(Some(stmt)) => candidates.push(stmt),
            Ok(None) => {}
            Err(problem) => recovery_problems.push(problem),
        }
    }
    // A line that ends the way a written Korean sentence ends beats a single
    // repaired reading of it. `올해 여덟 살이 되었습니다` puts `올해` one
    // character from `말해`, and reading it as output threw the first word
    // away. The same question was already asked when recovery was ambiguous;
    // one candidate used to go through unquestioned.
    let korean_sentence_beats_recovery = is_written_korean_sentence(tokens, known_names)
        && prose_beats_recovery(source, tokens, known_names);
    if candidates.len() == 1 && recovery_problems.is_empty() && !korean_sentence_beats_recovery {
        // An output word the prose rule turned down is still a reading. When
        // the same misspelling is one edit from a second action as well, the
        // line has two meanings and only the writer knows which: `asy name
        // Hello` is `say` or `ask`, and choosing one silently is the guess
        // this compiler exists to avoid.
        let output_declined_for_prose = !recoverable_output_shape(tokens, known_names)
            && (output_action_at(tokens, 0, MatchMode::Recover).is_some()
                || output_action_ending(tokens, MatchMode::Recover, known_names).is_some());
        if output_declined_for_prose && single_word_ties_two_actions(tokens) {
            return Err(ambiguous_action_diagnostic(tokens));
        }
        return Ok(candidates.pop());
    }
    let report_recovery = !prose_beats_recovery(source, tokens, known_names);
    if report_recovery
        && (candidates.len() > 1 || (!candidates.is_empty() && !recovery_problems.is_empty()))
    {
        return Err(ambiguous_action_diagnostic(tokens));
    }
    if report_recovery && recovery_problems.len() == 1 {
        return Err(recovery_problems.pop().expect("one recovery problem"));
    }
    if report_recovery && recovery_problems.len() > 1 {
        return Err(ambiguous_action_diagnostic(tokens));
    }

    // `재미있는 이야기: 시작` — a label and its text. Every block header that
    // uses a colon has had its turn above, so what is left is writing.
    if is_written_label(source, tokens) {
        return Ok(Some(NmeStmt::Say {
            value: Value::Text(make_text_template(source, tokens, known_names)),
        }));
    }

    // A written Korean sentence is a sentence even with a number or a `%` in
    // it. Every matcher above has had its turn, so nothing that is a command
    // reaches here.
    //
    // Unless a command word opens or closes the line. `2초 기다립니다` ends the
    // way a sentence ends and is still somebody asking for a two-second wait
    // with a word NME does not accept; naming the word is worth a bad minute,
    // printing the line back is a bad afternoon. The prose branch below asks
    // the same question, and the two must agree.
    if is_written_korean_sentence(tokens, known_names)
        && !opens_or_closes_with_a_command_word(tokens)
    {
        return Ok(Some(NmeStmt::Say {
            value: Value::Text(make_text_template(source, tokens, known_names)),
        }));
    }

    // Invalid Python led by another Python keyword belongs to Python. This
    // preserves its own context-sensitive diagnostics (`elif`, `except`, ...)
    // while still allowing the deliberately supported mixed `if 조건` form.
    if is_python_keyword(&tokens[0].tok) {
        return Ok(None);
    }
    if is_written_prose_line(tokens) {
        if near_miss_action_word(tokens, known_names).is_some() {
            return Err(unknown_action_word_diagnostic(tokens, known_names));
        }
        // `겨울을 나려고 곡식을 저장했습니다` — `저장했습니다` is one word, and
        // splitting it into `저장 했습니다` names a saving word nobody wrote.
        if !is_written_korean_sentence(tokens, known_names) {
            if let Some((index, split)) = glued_action_word(tokens) {
                let word = name_word(&tokens[index]).unwrap_or("");
                return Err(glued_word_diagnostic(&tokens[index], word, &split));
            }
        }
        let value = parse_value(source, tokens, known_names, true)
            .map_err(|()| missing_action_diagnostic(tokens))?;
        return Ok(Some(NmeStmt::Say { value }));
    }
    // Handing a written sentence back to Python means the beginner reads
    // CPython's `SyntaxError`, in English, with the caret inside a Hangul
    // syllable. NME wrote nothing on this line, so NME explains it.
    if tokens.len() > 1 && looks_like_written_sentence(tokens) {
        return Err(unknown_action_word_diagnostic(tokens, known_names));
    }
    // The same sentence with a hyphen, a slash, a wave dash or a bracket in
    // it. Python reads every one of those as an operator, so the line was
    // handed back and CPython answered a Korean sentence with an English
    // `SyntaxError`. Asked last, and only of a line Python has already
    // refused, so valid Python still wins.
    if is_written_korean_sentence_with_punctuation(source, tokens, known_names) {
        return Ok(Some(NmeStmt::Say {
            value: Value::Text(make_text_template(source, tokens, known_names)),
        }));
    }
    Ok(None)
}

// ---------------------------------------------------------------- output

fn match_say(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let action_start = leading_sentence_fillers(tokens);
    if let Some((spelling, consumed)) = output_action_at(tokens, action_start, mode) {
        let mut body_start = action_start + consumed;
        if tokens.get(body_start).is_some_and(is_command_ending) && body_start + 1 < tokens.len() {
            body_start += 1;
        }
        if body_start + 1 < tokens.len()
            && tokens.get(body_start).is_some_and(is_show_request_pronoun)
        {
            body_start += 1;
        }
        if body_start >= tokens.len() {
            // The action word was only a guess, so an empty message means the
            // guess was wrong: `입력해 주세요` is a sentence, not `출력해주세요`
            // with nothing to show.
            if mode == MatchMode::Recover {
                return Ok(None);
            }
            return Err(say_missing(spelling, tokens[action_start].span));
        }
        let body = trim_trailing_fillers(&tokens[body_start..]);
        if (spelling == Spelling::English || action_start == 0)
            && output_action_at(tokens, action_start, MatchMode::Exact).is_none()
            && !output_repair_claims_one_word(body)
        {
            // A repaired output word only claims one word of message — in
            // English wherever it stands, in Korean when it opens the line.
            // See `output_repair_claims_one_word`.
            return Ok(None);
        }
        let prefer_text = action_start != 0
            || consumed != 1
            || mode == MatchMode::Recover
            || (!token_is_exact_name(&tokens[action_start], SAY_KEYWORD)
                && !token_is_exact_name(&tokens[action_start], SAY_KEYWORD_KO));
        if !prefer_text {
            let span = span_of(body);
            let text = &source[span.start..span.end];
            if looks_like_broken_expression(body) && !is_valid_python_expression(text) {
                return Err(Diagnostic::bilingual(
                    DiagnosticCode::SayValueBroken,
                    "I couldn't understand what you want to `say`",
                    "`말해` 뒤의 값을 이해하지 못했어요",
                    span,
                )
                .with_bilingual_hint(
                    "finish the value, or use plain words such as `show Hello world`",
                    "값을 완성하거나 `안녕하세요 말해줘`처럼 평범한 문장으로 쓰세요",
                ));
            }
        }
        let value = parse_value(source, body, known_names, prefer_text).map_err(|()| {
            Diagnostic::bilingual(
                DiagnosticCode::SayValueUnparseable,
                "I couldn't understand what to show",
                "무엇을 말할지 이해하지 못했어요",
                span_of(body),
            )
            .with_bilingual_hint(
                "write a value, or a sentence such as `show Hello world`",
                "`안녕하세요 말해줘`처럼 평범한 문장으로 적어도 돼요",
            )
        })?;
        return Ok(Some(NmeStmt::Say { value }));
    }

    let Some((action_start, spelling, action_end)) =
        output_action_ending(tokens, mode, known_names)
    else {
        return Ok(None);
    };
    if action_start == 0 {
        if mode == MatchMode::Recover {
            return Ok(None);
        }
        return Err(say_missing(spelling, tokens[action_start].span));
    }
    debug_assert!(action_end <= tokens.len());
    let mut value_start = leading_sentence_fillers(&tokens[..action_start]);
    if value_start + 1 < action_start
        && tokens.get(value_start).is_some_and(is_show_request_pronoun)
    {
        value_start += 1;
    }
    let value_tokens =
        trim_suffix_say_value(trim_trailing_fillers(&tokens[value_start..action_start]));
    if spelling == Spelling::English
        && output_action_ending(tokens, MatchMode::Exact, known_names).is_none()
        && !output_repair_claims_one_word(&value_tokens)
    {
        // `Hello world show` is the message-first order English tolerates and
        // it takes any message, because the output word is written exactly.
        // A *repaired* one takes a single word: `Clear a path through the
        // snow.` is a sentence, not `show` with `snow` misspelled.
        return Ok(None);
    }
    if value_tokens.is_empty() {
        if mode == MatchMode::Recover {
            return Ok(None);
        }
        return Err(say_missing(spelling, tokens[action_start].span));
    }
    let value = parse_value(source, &value_tokens, known_names, true).map_err(|()| {
        Diagnostic::bilingual(
            DiagnosticCode::SaySentenceUnparseable,
            "I couldn't understand the sentence to show",
            "말할 문장을 이해하지 못했어요",
            span_of(&value_tokens),
        )
        .with_bilingual_hint(
            "write it like `Hello world show`",
            "`안녕하세요 말해줘`처럼 쓰세요",
        )
    })?;
    Ok(Some(NmeStmt::Say { value }))
}

/// True when a sentence form may make this word into a name.
///
/// Only English is asked: Korean marks its target with a particle, which is
/// already the proof that a name was meant.
fn is_bindable_english_name(word: &str) -> bool {
    !NOT_A_NAME_EN
        .iter()
        .any(|known| word.eq_ignore_ascii_case(known))
}

/// True when a *repaired* output word may claim the rest of the line.
///
/// A misspelling is a guess, and a guess may not eat a sentence. In English
/// two guesses used to be made silently: a misspelling anywhere on a line of
/// ordinary words claimed the line — `Today is a good day` printed `Today is
/// a good`, because `day` is one letter from `say`, and `Clear a path through
/// the snow.` lost `snow` to `show` — and an output word written *after* the
/// message claimed `There is nothing left to say.`
///
/// Korean has the same hole at the start of a line, and only there: Korean
/// puts its action word last, so `말해` opening a line is already the unusual
/// order, and one character away from it is `올해` (*this year*) and `말을`
/// (*the words*). `올해 여덟 살이 되었습니다` printed `여덟 살이 되었습니다`.
///
/// Both readings are kept for what a beginner really writes: one word of
/// message, which is `shwo hello`, `hello sya` and `말헤줘 안녕`. With more
/// than one word there, the line is prose and prints itself whole. An output
/// word spelled *exactly* is not a guess, so `Hello world show` and
/// `안녕하세요 말해줘` keep their whole message.
fn output_repair_claims_one_word(message: &[Token]) -> bool {
    // ...and that one word has to be a word. `말씀해 주십시오` puts `말씀해` one
    // character from `말해`, leaving `주십시오` as the whole message; nobody
    // asks a program to print the word *please*.
    message.len() == 1
        && name_word(&message[0]).is_some()
        && !token_matches_exact(&message[0], POLITE_AUXILIARY_KO)
        && !token_matches_exact(&message[0], AUXILIARY_VERBS_KO)
}

fn say_missing(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::SayMissing,
        "there is nothing to show",
        "말할 내용이 비어 있어요",
        span,
    )
    .with_bilingual_hint(
        "write `show Hello world`",
        "`안녕하세요 말해줘`처럼 내용을 함께 적어 주세요",
    )
}

fn is_show_request_pronoun(token: &Token) -> bool {
    token_matches_exact(token, &["me", "나", "나를", "나에게"])
}

fn output_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, SAY_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, SAY_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

fn leading_sentence_fillers(tokens: &[Token]) -> usize {
    let mut index = 0;
    while tokens
        .get(index)
        .is_some_and(|token| token_matches_exact(token, SENTENCE_FILLERS))
    {
        index += 1;
    }
    index
}

/// Drops politeness fillers from the end of a message. Korean puts `좀` right
/// before the verb and English puts `please` right after the message, so both
/// land at the end of what would otherwise be printed. A message that is
/// nothing but a filler is left alone: then the filler is the message.
fn trim_trailing_fillers(tokens: &[Token]) -> &[Token] {
    let mut end = tokens.len();
    while end > 1 && token_matches_exact(&tokens[end - 1], SENTENCE_FILLERS) {
        end -= 1;
    }
    &tokens[..end]
}

// ---------------------------------------------------------------- input

/// True when the words of a question ask for a number rather than for text.
///
/// The line a learner writes next is almost always a comparison, and comparing
/// what `input()` gives back — always text — with a number is silently false
/// for ever. The bare question form has always read these as numbers; this is
/// the same rule, so that writing the clearer `ask age How old are you?` does
/// not quietly mean something different from writing the question alone.
fn question_asks_for_a_number(source: &str, tokens: &[Token]) -> bool {
    // `이름을 물어봐` has no question at all, and asking for the text of an
    // empty slice would take the compiler down.
    if tokens.is_empty() {
        return false;
    }
    if tokens.iter().any(|token| token_word(token) == Some("몇")) {
        return true;
    }
    let text = token_text(source, tokens).to_lowercase();
    ["how many", "how old", "how much", "how long", "how tall"]
        .iter()
        .any(|opening| text.contains(opening))
}

fn match_ask(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(shape) = find_ask_shape(tokens, mode) else {
        return Ok(None);
    };
    // `ask what is your name` · `ask how old are you` · `물어봐 이름이 뭐예요?`
    // — the action word and then the question, with no name in between. The
    // question forms already know which name each of them answers, so the
    // line means exactly what the same question means written on its own.
    // `ask who is there` is not one of them: there is no name it could save
    // into, so it is still refused.
    if let Some(question) = tokens.get(shape.target_at..) {
        if let Some(stmt) = match_natural_question(source, question, known_names) {
            return Ok(Some(stmt));
        }
    }
    let Some(target_token) = tokens.get(shape.target_at) else {
        return Err(ask_target_diagnostic(
            shape.spelling,
            tokens[shape.action_start].span,
        ));
    };
    let Some(target_word) = name_word(target_token) else {
        return Err(ask_target_diagnostic(shape.spelling, target_token.span));
    };
    // `ask What is your name?` would otherwise save the answer into a
    // variable called `What`, and `ask the ...` into one called `the`.
    // Neither is a name the writer set, so neither is ever emitted.
    if opens_a_question(tokens, shape.target_at) || is_english_article(Some(target_token)) {
        return Err(ask_target_diagnostic(shape.spelling, target_token.span));
    }
    let prompt_is_written_out = tokens[shape.prompt_start.min(tokens.len())..]
        .iter()
        .all(|token| !matches!(token.tok, Tok::Comma | Tok::String { .. }));
    if shape.spelling == Spelling::English
        && prompt_is_written_out
        && !is_bindable_english_name(target_word)
    {
        // `ask your teacher about the field trip` and `ask me anything you
        // like` are sentences: neither `your` nor `me` is a name anybody meant
        // to create, and stopping the program at an `input()` nobody wrote is
        // the worst way to find that out. Quotes or a comma say a name was
        // meant, so `ask your "hi"` still asks. The words of the prompt are
        // never looked at: everything after the name is text.
        return Ok(None);
    }
    let target = strip_target_particle(target_word).to_string();
    if target.is_empty() {
        return Err(ask_target_diagnostic(shape.spelling, target_token.span));
    }

    let mut prompt_end = shape.prompt_end.unwrap_or(tokens.len());
    if shape.prompt_start + 1 == prompt_end && tokens.last().is_some_and(is_command_ending) {
        prompt_end -= 1;
    }
    let prompt = if shape.prompt_start >= prompt_end {
        None
    } else if matches!(tokens[shape.prompt_start].tok, Tok::Comma) {
        let expression_tokens = &tokens[shape.prompt_start + 1..prompt_end];
        if expression_tokens.is_empty() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::AskQuestionMissing,
                "the question after the comma is missing",
                "쉼표 뒤의 질문이 비어 있어요",
                tokens[shape.prompt_start].span,
            )
            .with_bilingual_hint(
                "add a question after the comma",
                "쉼표 뒤에 질문을 적어 주세요",
            ));
        }
        let span = span_of(expression_tokens);
        if is_valid_python_expression(&source[span.start..span.end]) {
            Some(Value::Python(Code::Source(span)))
        } else if expression_tokens
            .iter()
            .all(|token| token_word(token).is_some() || is_command_ending(token))
        {
            Some(Value::Text(make_text_template(
                source,
                expression_tokens,
                known_names,
            )))
        } else {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::AskQuestionUnparseable,
                "I couldn't understand the question",
                "질문 내용을 이해하지 못했어요",
                span,
            )
            .with_bilingual_hint(
                "remove the comma to write a plain sentence without quotes",
                "쉼표를 빼면 따옴표 없는 평범한 문장으로 쓸 수 있어요",
            ));
        }
    } else {
        // A comma means precise beginner syntax. Without one, the remainder is
        // deliberately sentence text and therefore needs no quotes.
        let prompt_tokens = &tokens[shape.prompt_start..prompt_end];
        let prompt_span = span_of(prompt_tokens);
        if is_valid_python_expression(&source[prompt_span.start..prompt_span.end])
            && !matches!(prompt_tokens[0].tok, Tok::Name { .. })
        {
            Some(Value::Python(Code::Source(prompt_span)))
        } else {
            // A question is addressed to a person, so it is text and nothing
            // else. Substituting names into it wrote the program's own values
            // into the words on the screen: `비밀번호는 용` followed by
            // `입력을 물어봐 비밀번호가 무엇입니까?` asked *용가 무엇입니까?* —
            // the password program printed the password. The bare-question
            // form has always read the words literally; this makes the two
            // spellings of the same statement agree.
            Some(Value::Text(make_text_template(
                source,
                prompt_tokens,
                &HashSet::new(),
            )))
        }
    };
    // `ask age How old are you?` used to give text while the same question
    // written on its own gave a number. Reading the words is what decides it,
    // whichever spelling the writer chose. An explicit `ask number` / `숫자로`
    // already says so and is left alone.
    let kind = if shape.kind == InputKind::Text
        && question_asks_for_a_number(source, &tokens[shape.prompt_start..prompt_end])
    {
        InputKind::Number
    } else {
        shape.kind
    };
    Ok(Some(NmeStmt::Ask {
        target,
        prompt,
        kind,
    }))
}

/// Match the smallest conversational input forms:
/// `이름이 뭐예요` and `What is your name`.
///
/// The Korean/English question predicate is deliberate proof of intent. A
/// normal sentence such as `안녕하세요!` therefore remains output, while a
/// beginner can start asking without learning `ask`, a comma, quotes, or even
/// a question mark. More complex questions still use the explicit
/// `물어봐`/`ask` form.
fn match_natural_question(
    source: &str,
    tokens: &[Token],
    _known_names: &HashSet<String>,
) -> Option<NmeStmt> {
    let has_question_mark = tokens
        .last()
        .is_some_and(|token| token_matches_exact(token, &["?"]));
    let question_end = if has_question_mark {
        tokens.len().checked_sub(1)?
    } else {
        tokens.len()
    };
    if question_end < 2 {
        return None;
    }

    // A question about how many, or about an age, is answered with a number,
    // and the next line a learner writes is almost always a comparison — so
    // read it as one instead of leaving a string behind.
    let asks_for_a_number = natural_age_question_target(tokens, question_end).is_some()
        || question_asks_for_a_number(source, &tokens[..question_end]);
    let target = if let Some(target) = natural_age_question_target(tokens, question_end) {
        Some(target)
    } else if let Some(first) = tokens.first().and_then(name_word) {
        // `내 이름은 뭐예요?` is the same beginner question as
        // `이름이 뭐예요?`; the possessive is natural speech, not part of
        // the variable name.
        let target_at = usize::from(matches!(first, "내" | "제" | "우리"));
        let target_word = tokens.get(target_at).and_then(name_word)?;
        let particle_at = target_at + 1;
        let predicate_at = if tokens
            .get(particle_at)
            .is_some_and(|token| token_matches_exact(token, &["은", "는", "이", "가", "을", "를"]))
        {
            particle_at + 1
        } else {
            particle_at
        };
        // Both attached (`이름이`) and spoken, separated particles (`이름 이`)
        // are common. A bare target is also safe once the distinctive
        // question predicate has been proven below.
        let korean_target = strip_natural_question_particle(target_word).or(Some(target_word));
        let predicate = tokens.get(predicate_at).and_then(token_word)?;
        let is_korean_question = KOREAN_QUESTION_PREDICATES.contains(&predicate)
            || (predicate == "몇" && tokens.get(predicate_at + 1).and_then(token_word).is_some());
        korean_target.filter(|_| is_korean_question)
    } else {
        None
    }
    .or_else(|| {
        let first = tokens.first().and_then(token_word)?;
        let (subject_at, matches_shape) = if first.eq_ignore_ascii_case("what") {
            if tokens
                .get(1)
                .and_then(token_word)
                .is_some_and(|word| word.eq_ignore_ascii_case("is"))
            {
                (2, question_end >= 4)
            } else if tokens
                .get(1)
                .and_then(token_word)
                .is_some_and(|word| word.eq_ignore_ascii_case("s"))
            {
                // The sentence lexer separates the apostrophe in `What's`
                // so it can safely preserve ordinary contractions.
                (2, question_end >= 4)
            } else {
                (0, false)
            }
        } else if first.eq_ignore_ascii_case("what's") || first.eq_ignore_ascii_case("whats") {
            (1, question_end >= 3)
        } else {
            (0, false)
        };
        if !matches_shape
            || !tokens
                .get(subject_at)
                .and_then(token_word)
                .is_some_and(|word| {
                    word.eq_ignore_ascii_case("your")
                        || word.eq_ignore_ascii_case("the")
                        || word.eq_ignore_ascii_case("my")
                        || word.eq_ignore_ascii_case("our")
                })
        {
            return None;
        }
        name_word(tokens.get(question_end - 1)?)
    })?;

    let prompt_tokens = if has_question_mark {
        &tokens[..=question_end]
    } else {
        &tokens[..question_end]
    };
    let prompt = Value::Text(make_text_template(source, prompt_tokens, &HashSet::new()));
    Some(NmeStmt::Ask {
        target: target.to_string(),
        prompt: Some(prompt),
        kind: if asks_for_a_number || target == "age" {
            InputKind::Number
        } else {
            InputKind::Text
        },
    })
}

fn natural_age_question_target(tokens: &[Token], question_end: usize) -> Option<&'static str> {
    let word = |index: usize| tokens.get(index).and_then(token_word);
    let korean_age = [
        "살이에요",
        "살이예요",
        "살이야",
        "살인가요",
        "살입니까",
        "살이죠",
    ];
    if word(0) == Some("몇") && word(1).is_some_and(|value| korean_age.contains(&value)) {
        return Some("나이");
    }
    if word(0) == Some("나")
        && word(1) == Some("몇")
        && word(2).is_some_and(|value| korean_age.contains(&value))
    {
        return Some("나이");
    }
    if word(0).is_some_and(|value| value.eq_ignore_ascii_case("how"))
        && word(1).is_some_and(|value| value.eq_ignore_ascii_case("old"))
        && ((word(2).is_some_and(|value| value.eq_ignore_ascii_case("are"))
            && word(3).is_some_and(|value| value.eq_ignore_ascii_case("you")))
            || (word(2).is_some_and(|value| value.eq_ignore_ascii_case("am"))
                && word(3).is_some_and(|value| value.eq_ignore_ascii_case("i"))))
        && question_end >= 4
    {
        return Some("age");
    }
    None
}

fn strip_natural_question_particle(word: &str) -> Option<&str> {
    // A final `이` can be either the subject particle (`이름이`) or part of a
    // normal Korean noun (`나이`, `아이`, `종이`). Keep the common noun forms
    // intact; attached `은`/`는` and less ambiguous particles still strip as
    // expected.
    if [
        "나이",
        "아이",
        "고양이",
        "강아지",
        "종이",
        "사이",
        "회의",
        "이야기",
        "의미",
    ]
    .contains(&word)
    {
        return None;
    }
    strip_any_suffix(word, &["은", "는", "이", "가", "을", "를"])
}

struct AskShape {
    action_start: usize,
    target_at: usize,
    prompt_start: usize,
    /// Where the question stops, when the name comes after it rather than
    /// before. `None` means the question runs to the end of the line.
    prompt_end: Option<usize>,
    spelling: Spelling,
    kind: InputKind,
}

/// Words an English question opens with.
fn is_question_word(token: &Token) -> bool {
    token_matches_exact(
        token,
        &[
            "what", "who", "whom", "whose", "which", "where", "when", "why", "how",
        ],
    )
}

/// True when the word picked as the name to hold the answer is really the
/// first word of the question: `ask What is your name?` would otherwise save
/// the answer into a variable called `What`.
///
/// A question word standing alone, or followed by the beginner comma, is an
/// ordinary name — `ask when, "which day: "` names a variable `when`.
fn opens_a_question(tokens: &[Token], target_at: usize) -> bool {
    tokens.get(target_at).is_some_and(is_question_word)
        && tokens
            .get(target_at + 1)
            .is_some_and(|token| !matches!(token.tok, Tok::Comma))
}

fn find_ask_shape(tokens: &[Token], mode: MatchMode) -> Option<AskShape> {
    let action_start = leading_sentence_fillers(tokens);
    if let Some((spelling, consumed)) = ask_action_at(tokens, action_start, mode) {
        let mut target_at = action_start + consumed;
        // `ask the name What is your name?` — the article belongs to the
        // sentence, not to the name the answer goes into.
        while is_english_article(tokens.get(target_at)) && target_at + 1 < tokens.len() {
            target_at += 1;
        }
        let mut kind = if tokens
            .get(target_at)
            .is_some_and(|token| token_matches_exact(token, NUMBER_WORDS))
        {
            target_at += 1;
            InputKind::Number
        } else {
            InputKind::Text
        };
        // `ask age as a number How old are you?` — the same request, written
        // after the name. Dropping the phrase silently would throw the
        // `int()` away and leave text where a number was asked for.
        let mut prompt_start = target_at + 1;
        if matches!(kind, InputKind::Text) {
            let mut probe = prompt_start;
            if token_matches_exact_at(tokens, probe, &["as"]) {
                probe += 1;
            }
            while is_english_article(tokens.get(probe)) {
                probe += 1;
            }
            if probe > prompt_start
                && token_matches_exact_at(tokens, probe, NUMBER_WORDS)
                && probe + 1 < tokens.len()
            {
                kind = InputKind::Number;
                prompt_start = probe + 1;
            }
        }
        // `ask What is your name? name` — the question came first and the
        // name closes the line, after the question mark.
        if opens_a_question(tokens, target_at) {
            let last = tokens.len().checked_sub(1)?;
            if last > target_at + 1
                && name_word(&tokens[last]).is_some()
                && tokens.get(last - 1).is_some_and(is_command_ending)
            {
                return Some(AskShape {
                    action_start,
                    target_at: last,
                    prompt_start: target_at,
                    prompt_end: Some(last),
                    spelling,
                    kind,
                });
            }
        }
        return Some(AskShape {
            action_start,
            target_at,
            prompt_start,
            prompt_end: None,
            spelling,
            kind,
        });
    }

    let mut target_at = 0;
    while tokens
        .get(target_at)
        .is_some_and(|token| token_matches_exact(token, SENTENCE_FILLERS))
    {
        target_at += 1;
    }
    name_word(tokens.get(target_at)?).filter(|name| !name.is_empty())?;
    for action_start in target_at + 1..tokens.len() {
        let Some((spelling, consumed)) = ask_action_at(tokens, action_start, mode) else {
            continue;
        };
        let modifiers = &tokens[target_at + 1..action_start];
        if !modifiers.iter().all(is_ask_modifier) {
            continue;
        }
        let kind = if modifiers.iter().any(|token| {
            token_matches_exact(token, NUMBER_WORDS) || name_word(token) == Some("숫자")
        }) {
            InputKind::Number
        } else {
            InputKind::Text
        };
        return Some(AskShape {
            action_start,
            target_at,
            prompt_start: action_start + consumed,
            prompt_end: None,
            spelling,
            kind,
        });
    }
    None
}

fn ask_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, ASK_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, ASK_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
        .or_else(|| {
            action_phrase_at(tokens, start, ASK_SHORT_WORDS_KO, MatchMode::Exact)
                .map(|consumed| (Spelling::Korean, consumed))
        })
        .or_else(|| question_ask_action_at(tokens, start))
}

/// An asking word that is also an ordinary verb, read only on a line that
/// really asks something. See [`ASK_QUESTION_WORDS_EN`].
///
/// Never repaired from a misspelling: the question mark is doing the work of
/// the word here, and a guess on top of it would claim `Did you get it?`.
/// See [`line_asks_a_question`] for what counts as asking.
fn question_ask_action_at(tokens: &[Token], start: usize) -> Option<(Spelling, usize)> {
    if !line_asks_a_question(tokens) {
        return None;
    }
    action_phrase_at(tokens, start, ASK_QUESTION_WORDS_EN, MatchMode::Exact)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, ASK_QUESTION_WORDS_KO, MatchMode::Exact)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}


fn is_ask_modifier(token: &Token) -> bool {
    token_matches_exact(
        token,
        &[
            "을",
            "를",
            "에게",
            "한테",
            "number",
            "numeric",
            "숫자",
            "숫자로",
            "수로",
            "로",
            "으로",
            "좀",
        ],
    )
}

fn ask_target_diagnostic(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AskTargetInvalid,
        "write the name that should hold the answer",
        "대답을 담을 이름이 필요해요",
        span,
    )
    .with_bilingual_hint(
        "for example: `ask name What is your name`",
        "`이름을 물어봐 이름이 뭐예요`처럼 쓰세요",
    )
}

// ----------------------------------------------------------- control flow

/// Words that connect a value change to its parts. None of them is ever a
/// name a beginner sets, so a line that opens with one has been written back
/// to front rather than naming a variable.
const UPDATE_CONNECTOR_WORDS_EN: &[&str] = &["to", "by", "from", "of", "into", "onto"];

fn match_update(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // A line that opens with an output or question word is that statement,
    // whatever its free text happens to say. Without this, the arithmetic
    // words inside a message quietly rewrite the whole line: `show I will
    // multiply by 2` used to become `show = show * 2`.
    if tokens.first().is_some_and(starts_a_different_statement) {
        return Ok(None);
    }
    // `카드를 잘 섞어 나눠 주세요` is a request to deal the cards, and it
    // compiled to `카드 = 카드 / 주세요` — a division by a name nothing ever
    // set. A line that ends the way a written Korean sentence ends and holds
    // no action word NME knows is a sentence. `점수에 1 더해` holds `더해`, so
    // every real value change is untouched.
    if korean_line_is_a_sentence(tokens, known_names)
        && prose_beats_recovery(source, tokens, known_names)
    {
        return Ok(None);
    }
    // `to score add 1` / `by 1 increase score` — the connector moved to the
    // front. Reading the line without it gives the ordinary order back;
    // reading it *with* it would set a variable called `to`, which nobody
    // wrote, so that result is never emitted.
    if tokens.len() > 2
        && token_matches_exact(&tokens[0], UPDATE_CONNECTOR_WORDS_EN)
        && !name_word(&tokens[0]).is_some_and(|word| known_names.contains(word))
    {
        if let Some(stmt) = match_update(source, &tokens[1..], known_names, mode)? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = amount_first_update(source, &tokens[1..], mode, known_names) {
            return Ok(Some(stmt));
        }
        return Ok(None);
    }
    if let Some((action_start, operation, _)) = update_action_ending(tokens, mode) {
        let target_token = tokens
            .first()
            .ok_or_else(|| update_diagnostic(span_of(tokens)))?;
        // Korean may put the amount first and mark the name with `에`:
        // `1을 점수에 더해`. The name is the one carrying the particle, and it
        // has to exist already, so ordinary speech is never made arithmetic.
        if name_word(target_token).is_none() {
            if let Some(stmt) =
                korean_amount_first_update(source, tokens, known_names, action_start, operation)
            {
                return Ok(Some(stmt));
            }
        }
        // Only a name can have its value changed. A line that starts with a
        // number or a piece of text is some other sentence that merely
        // happens to contain `더해`, so let the other matchers read it.
        if name_word(target_token).is_none() {
            return Ok(None);
        }
        let target = name_word(target_token)
            .and_then(update_target_name)
            .ok_or_else(|| update_diagnostic(target_token.span))?;
        let mut amount_tokens = tokens[1..action_start].to_vec();
        while amount_tokens
            .first()
            .is_some_and(|token| is_update_connector(token, &["에", "에서", "에게", "한테"]))
        {
            amount_tokens.remove(0);
        }
        while amount_tokens
            .last()
            .is_some_and(|token| is_update_connector(token, &["을", "를", "만큼"]))
        {
            amount_tokens.pop();
        }
        // `이 사진에서 저를 빼 주세요` and `값을 올려 받았습니다` end the way a
        // written Korean sentence ends, and the "amount" is a word nothing
        // ever set. There was no arithmetic on this line to explain.
        if is_unset_word(&amount_tokens, known_names)
            && (is_written_korean_sentence(tokens, known_names)
                || is_written_korean_sentence_with_punctuation(source, tokens, known_names))
        {
            return Ok(None);
        }
        return finish_update(
            source,
            tokens,
            known_names,
            target,
            &amount_tokens,
            operation,
        )
        .map(Some);
    }

    for action_start in 1..tokens.len() {
        let Some((operation, consumed)) = update_action_at(tokens, action_start, mode) else {
            continue;
        };
        if name_word(&tokens[0]).is_none() {
            return Ok(None);
        }
        let target = name_word(&tokens[0])
            .and_then(update_target_name)
            .ok_or_else(|| update_diagnostic(tokens[0].span))?;
        let mut amount_end = tokens.len();
        if tokens
            .get(amount_end.saturating_sub(1))
            .is_some_and(is_command_ending)
        {
            amount_end -= 1;
        }
        let mut amount_tokens = tokens[action_start + consumed..amount_end].to_vec();
        while amount_tokens
            .first()
            .is_some_and(|token| is_update_connector(token, &["by", "to", "of"]))
        {
            amount_tokens.remove(0);
        }
        // `이 사진에서 저를 빼 주세요` and `값을 올려 받았습니다` end the way a
        // written Korean sentence ends, and the "amount" is a word nothing
        // ever set. There was no arithmetic on this line to explain.
        if is_unset_word(&amount_tokens, known_names)
            && (is_written_korean_sentence(tokens, known_names)
                || is_written_korean_sentence_with_punctuation(source, tokens, known_names))
        {
            return Ok(None);
        }
        return finish_update(
            source,
            tokens,
            known_names,
            target,
            &amount_tokens,
            operation,
        )
        .map(Some);
    }

    // English also reads naturally as `add 1 to score` or
    // `increase score by 1`. Keep this form deliberately exact so a normal
    // Python expression cannot be claimed by the sentence matcher.
    if let Some((operation, consumed)) = update_action_at(tokens, 0, mode) {
        let mut remainder_end = tokens.len();
        if tokens
            .get(remainder_end.saturating_sub(1))
            .is_some_and(is_command_ending)
        {
            remainder_end -= 1;
        }
        let remainder = &tokens[consumed..remainder_end];
        let separator = remainder
            .iter()
            .position(|token| is_update_connector(token, &["to", "by", "from"]));
        let Some(separator) = separator else {
            return Err(update_diagnostic(span_of(tokens)));
        };
        let (left, right) = remainder.split_at(separator);
        let right = &right[1..];
        let (target_tokens, amount_tokens) = if (operation == UpdateOp::Add
            && token_matches_exact(&remainder[separator], &["to"]))
            || (operation == UpdateOp::Subtract
                && token_matches_exact(&remainder[separator], &["from"]))
        {
            (right, left)
        } else if !left.is_empty() && !right.is_empty() {
            (left, right)
        } else {
            return Err(update_diagnostic(span_of(tokens)));
        };
        if target_tokens.len() != 1 {
            return Err(update_diagnostic(span_of(tokens)));
        }
        let target = name_word(&target_tokens[0])
            .and_then(update_target_name)
            .ok_or_else(|| update_diagnostic(span_of(tokens)))?;
        return finish_update(
            source,
            tokens,
            known_names,
            target,
            amount_tokens,
            operation,
        )
        .map(Some);
    }

    Ok(None)
}

/// True when the amount is a single word the program never set. Such a word
/// is not arithmetic: `add Mina to friends` means putting `Mina` in the list.
fn is_unset_word(tokens: &[Token], known_names: &HashSet<String>) -> bool {
    let [token] = tokens else {
        return false;
    };
    literal_token(token).is_none()
        && name_word(token).is_some_and(|word| {
            !known_names.contains(word) && !word.starts_with(|c: char| c.is_ascii_digit())
        })
}

/// Builds the value change, or the list append the writer actually meant.
///
/// `add Mina to friends` used to compile to `friends = friends + Mina` and
/// die with `NameError`. When the target really was made a list it is an
/// append; when it was not, the line is reported here rather than at run
/// time. `docs/syntax.md` §12 warns about exactly this confusion.
fn finish_update(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    target: String,
    amount_tokens: &[Token],
    operation: UpdateOp,
) -> Result<NmeStmt, Diagnostic> {
    if operation == UpdateOp::Add && is_unset_word(amount_tokens, known_names) {
        let word = name_word(&amount_tokens[0]).expect("checked by is_unset_word");
        if is_list_name(known_names, &target) {
            if let Some(problem) = not_a_list(&target, known_names, span_of(tokens)) {
                return Err(problem);
            }
            let value = parse_value(source, amount_tokens, known_names, true)
                .map_err(|()| append_diagnostic(span_of(tokens)))?;
            return Ok(NmeStmt::Append { target, value });
        }
        return Err(add_unset_word_diagnostic(word, span_of(tokens)));
    }
    // `remove Mina from friends` used to compile to `friends = friends -
    // Mina`, which is a `TypeError` even when both names exist: nothing can
    // be subtracted from a list. So on a list the word can only mean taking
    // one item back out.
    if operation == UpdateOp::Subtract && !amount_tokens.is_empty() {
        // `remove Mina from ages` / `나이표에서 민수 빼`. A record has no
        // `.remove`, so this cannot go through the list path: `del` is the
        // Python for taking one named value back out.
        if is_record_name(known_names, &target) {
            let [key_token] = amount_tokens else {
                return Err(record_remove_diagnostic(span_of(tokens)));
            };
            let key = record_key_value(key_token, known_names, READING_PARTICLES_KO)
                .ok_or_else(|| record_remove_diagnostic(span_of(tokens)))?;
            return Ok(NmeStmt::RecordRemove { target, key });
        }
        if is_list_name(known_names, &target) {
            let value = parse_value(source, amount_tokens, known_names, true)
                .map_err(|()| remove_diagnostic(span_of(tokens)))?;
            return Ok(NmeStmt::Remove { target, value });
        }
        if is_unset_word(amount_tokens, known_names) {
            let word = name_word(&amount_tokens[0]).expect("checked by is_unset_word");
            return Err(subtract_unset_word_diagnostic(word, span_of(tokens)));
        }
    }
    let amount = parse_update_amount(source, amount_tokens, known_names)
        .ok_or_else(|| update_diagnostic(span_of(tokens)))?;
    Ok(NmeStmt::Update {
        target,
        amount,
        operation,
    })
}

fn remove_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AppendUnparseable,
        "I couldn't understand what to take out of the list",
        "목록에서 무엇을 뺄지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `remove Mina from friends` or `친구들에서 민수 빼`",
        "`친구들에서 민수 빼` 또는 `remove Mina from friends`처럼 적어 주세요",
    )
}

/// The same shape as [`remove_diagnostic`], worded for a record: a record is
/// emptied one **name** at a time, not one item at a time.
fn record_remove_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::RecordNameUnknown,
        "I couldn't understand which name to take out of the record",
        "표에서 어떤 이름을 뺄지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `remove Mina from ages` or `나이표에서 민수 빼`",
        "`나이표에서 민수 빼` 또는 `remove Mina from ages`처럼 적어 주세요",
    )
}

fn subtract_unset_word_diagnostic(word: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UpdateUnparseable,
        "I couldn't understand this value change",
        "값을 어떻게 바꿀지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        format!(
            "nothing was saved in `{word}`, so it is not a number to take away; to take it out \
             of a list, make the list first with `set friends to an empty list`"
        ),
        format!(
            "`{word}`에 저장된 값이 없어서 뺄 수가 없어요. 목록에서 빼려면 먼저 \
             `친구들은 빈 목록`처럼 목록을 만들어 주세요"
        ),
    )
}

fn add_unset_word_diagnostic(word: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UpdateUnparseable,
        "I couldn't understand this value change",
        "값을 어떻게 바꿀지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        format!(
            "nothing was saved in `{word}`, so it is not a number to add; to put it in a list \
             write `append {word} to friends`"
        ),
        format!(
            "`{word}`에는 저장된 값이 없어서 더할 수 없어요. 목록에 넣으려면 `친구들에 {word} 넣어`처럼 적어 주세요"
        ),
    )
}

/// `by 1 increase score` with its leading connector already removed: the
/// amount comes first and the name closes the line.
fn amount_first_update(
    source: &str,
    tokens: &[Token],
    mode: MatchMode,
    known_names: &HashSet<String>,
) -> Option<NmeStmt> {
    let end = trim_command_endings(tokens).len();
    for action_start in 1..end {
        let Some((operation, consumed)) = update_action_at(tokens, action_start, mode) else {
            continue;
        };
        let [target_token] = &tokens[action_start + consumed..end] else {
            continue;
        };
        let Some(target) = name_word(target_token).and_then(update_target_name) else {
            continue;
        };
        let Some(amount) = parse_update_amount(source, &tokens[..action_start], known_names) else {
            continue;
        };
        return Some(NmeStmt::Update {
            target,
            amount,
            operation,
        });
    }
    None
}

/// `1을 점수에 더해` — Korean may put the amount first. The name is the word
/// carrying `에`, and it must already exist, so an ordinary sentence that
/// happens to end in `더해` is never turned into arithmetic.
fn korean_amount_first_update(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    action_start: usize,
    operation: UpdateOp,
) -> Option<NmeStmt> {
    let head = &tokens[..action_start];
    if head.len() < 2 {
        return None;
    }
    let target_at = head.len() - 1;
    let target = name_word(&head[target_at])
        .and_then(|word| strip_any_suffix(word, &["에게", "한테", "에"]))
        .map(str::to_string)
        .filter(|name| known_names.contains(name))?;
    let mut amount_tokens = head[..target_at].to_vec();
    while amount_tokens
        .last()
        .is_some_and(|token| is_update_connector(token, &["을", "를", "만큼"]))
    {
        amount_tokens.pop();
    }
    let amount = parse_update_amount(source, &amount_tokens, known_names)?;
    Some(NmeStmt::Update {
        target,
        amount,
        operation,
    })
}

fn update_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(UpdateOp, usize)> {
    // A typo such as `말헤` is equally close to the output action `말해` and
    // the update action `더해`. Prefer the explicit output vocabulary rather
    // than silently turning a spoken sentence into arithmetic. Exact update
    // words (`더해`, `add`, ...) remain unaffected.
    if mode == MatchMode::Recover
        && (action_phrase_at(tokens, start, SAY_WORDS_EN, MatchMode::Recover).is_some()
            || action_phrase_at(tokens, start, SAY_WORDS_KO, MatchMode::Recover).is_some())
    {
        return None;
    }
    action_phrase_at(tokens, start, UPDATE_ADD_WORDS_EN, mode)
        .or_else(|| action_phrase_at(tokens, start, UPDATE_ADD_WORDS_KO, mode))
        .map(|consumed| (UpdateOp::Add, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, UPDATE_SUBTRACT_WORDS_EN, mode)
                .or_else(|| action_phrase_at(tokens, start, UPDATE_SUBTRACT_WORDS_KO, mode))
                .map(|consumed| (UpdateOp::Subtract, consumed))
        })
        .or_else(|| {
            action_phrase_at(tokens, start, UPDATE_MULTIPLY_WORDS_EN, mode)
                .or_else(|| action_phrase_at(tokens, start, UPDATE_MULTIPLY_WORDS_KO, mode))
                .map(|consumed| (UpdateOp::Multiply, consumed))
        })
        .or_else(|| {
            action_phrase_at(tokens, start, UPDATE_DIVIDE_WORDS_EN, mode)
                .or_else(|| action_phrase_at(tokens, start, UPDATE_DIVIDE_WORDS_KO, mode))
                .map(|consumed| (UpdateOp::Divide, consumed))
        })
}

fn update_action_ending(tokens: &[Token], mode: MatchMode) -> Option<(usize, UpdateOp, usize)> {
    let mut end = tokens.len();
    if tokens.last().is_some_and(is_command_ending) {
        end -= 1;
    }
    let start_at = end.saturating_sub(3);
    for start in start_at..end {
        if let Some((operation, consumed)) = update_action_at(tokens, start, mode) {
            if start + consumed == end {
                return Some((start, operation, end));
            }
        }
    }
    None
}

/// Words that own the line they open, so a value change may not start there.
fn starts_a_different_statement(token: &Token) -> bool {
    [
        SAY_WORDS_EN,
        ASK_WORDS_EN,
        SAY_WORDS_KO,
        ASK_WORDS_KO,
        // `set left to score divided by 4` used to become `set = set / 4`,
        // binding a name called `set` that nobody wrote.
        SET_WORDS_EN,
        SET_WORDS_KO,
        WHEN_WORDS_EN,
        WHEN_WORDS_KO,
        ELSE_WORDS_EN,
        ELSE_WORDS_KO,
        WHILE_WORDS_EN,
        REPEAT_WORDS_EN,
        SLOW_WORDS_KO,
        VERY_WORDS_KO,
        BOX_WORDS_KO,
        MIDDLE_WORDS_KO,
        CLEAR_SCREEN_WORDS_EN,
        CLEAR_SCREEN_WORDS_KO,
        DRAW_LINE_WORDS_EN,
        DRAW_LINE_WORDS_KO,
    ]
    .iter()
    .any(|words| token_matches_exact(token, words))
}

fn update_target_name(word: &str) -> Option<String> {
    strip_any_suffix(
        word,
        &[
            "에다가",
            "에다",
            "에서",
            "에게",
            "한테",
            "에",
            "으로",
            "로",
            "을",
            "를",
            "은",
            "는",
        ],
    )
    .map(str::to_string)
    .or_else(|| (!word.is_empty()).then(|| word.to_string()))
}

fn parse_update_amount(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<Code> {
    if tokens.is_empty() {
        return None;
    }
    let mut tokens = tokens;
    // The lexer separates `2로` into a number and a particle, so drop a
    // trailing particle token before reading the expression.
    while tokens.len() > 1
        && tokens
            .last()
            .is_some_and(|token| token_matches_exact(token, UPDATE_AMOUNT_PARTICLES_KO))
    {
        tokens = &tokens[..tokens.len() - 1];
    }
    // Spoken Korean attaches the particle to the word: `결과를 둘째수로 나눠`.
    // A name with the particle still on it is a perfectly valid Python
    // expression — it is just a name nothing ever set — so checking validity
    // first left `결과 = 결과 / 둘째수로` behind, a `NameError` on a line that
    // reads correctly. What tells the two apart is which of them the program
    // actually made, so ask that before asking Python.
    let trimmed = strip_attached_particle_span(source, tokens, UPDATE_AMOUNT_PARTICLES_KO);
    if let Some(trimmed) = trimmed {
        let base = &source[trimmed.start..trimmed.end];
        if known_names.contains(base)
            && !known_names.contains(&source[span_of(tokens).start..span_of(tokens).end])
        {
            return Some(Code::Source(trimmed));
        }
    }
    let span = span_of(tokens);
    if is_valid_python_expression(&source[span.start..span.end]) {
        return Some(Code::Source(span));
    }
    let trimmed = trimmed?;
    is_valid_python_expression(&source[trimmed.start..trimmed.end]).then_some(Code::Source(trimmed))
}

/// Shortens `tokens`' span by one attached Korean particle on the last token.
/// Returns `None` when the last token carries none of them.
fn strip_attached_particle_span(
    source: &str,
    tokens: &[Token],
    particles: &[&str],
) -> Option<Span> {
    let last = tokens.last()?;
    let Tok::Name { name } = &last.tok else {
        return None;
    };
    let mut ordered = particles.to_vec();
    ordered.sort_by_key(|particle| std::cmp::Reverse(particle.len()));
    let particle = ordered.into_iter().find(|particle| {
        name.strip_suffix(particle)
            .is_some_and(|base| !base.is_empty())
    })?;
    let start = span_of(tokens).start;
    let end = last.span.end - particle.len();
    (end > start && source.is_char_boundary(end)).then(|| Span::new(start, end))
}

fn is_update_connector(token: &Token, words: &[&str]) -> bool {
    token_matches_exact(token, words)
}

fn update_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UpdateUnparseable,
        "I couldn't understand this value change",
        "값을 어떻게 바꿀지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `score add 1` or `점수에 1 더해`",
        "`점수에 1 더해` 또는 `score add 1`처럼 적어 주세요",
    )
}

fn match_break(
    _source: &str,
    tokens: &[Token],
    _known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(consumed) = action_phrase_at(tokens, 0, BREAK_WORDS_EN, mode)
        .or_else(|| action_phrase_at(tokens, 0, BREAK_WORDS_KO, mode))
    else {
        return Ok(None);
    };
    if tokens[consumed..]
        .iter()
        .any(|token| !is_command_ending(token))
    {
        return Err(Diagnostic::bilingual(
            DiagnosticCode::BreakCommandUnparseable,
            "I couldn't understand this break command",
            "이 반복 중단 명령을 이해하지 못했어요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            "write only `break` or `여기서 멈춰`",
            "`break` 또는 `여기서 멈춰`만 적어 주세요",
        ));
    }
    Ok(Some(NmeStmt::Break))
}

// ------------------------------------------------------------------ waiting

/// `wait 3 seconds` / `3초 기다려`. The unit word is optional in English and
/// may be written attached to the number in Korean, which is how people
/// actually type it.
fn match_wait(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // `… then show Time to sleep` ends in a waiting word but is a condition
    // with a message, not a wait. The opening word decides the line.
    let action_at = leading_sentence_fillers(tokens);
    if tokens
        .get(action_at)
        .is_some_and(starts_a_different_statement)
    {
        return Ok(None);
    }
    if let Some(consumed) = action_phrase_at(tokens, action_at, WAIT_WORDS_EN, mode)
        .or_else(|| action_phrase_at(tokens, action_at, WAIT_WORDS_KO, mode))
    {
        return wait_from(source, tokens, &tokens[action_at + consumed..], known_names);
    }
    let Some(action_start) = wait_action_ending(tokens, mode) else {
        return Ok(None);
    };
    wait_from(source, tokens, &tokens[..action_start], known_names)
}

/// Builds the wait from its amount region. A region with no number at all is
/// ordinary speech (`잠깐 기다려`), so it falls through to the sentence output
/// rules instead of becoming an error.
fn wait_from(
    source: &str,
    tokens: &[Token],
    amount: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // `wait two seconds` / `일초 기다려` — the length may be written as a
    // counting word. It is looked up before the guard below because such a
    // word is a number, not the ordinary speech the guard protects.
    if let Some(seconds) = number_word_wait_amount(amount) {
        return Ok(Some(NmeStmt::Wait { seconds }));
    }
    // `잠깐 기다려` is ordinary speech, and `잠깐` happens to be a valid Python
    // name, so a wait needs a number or a name the program already knows.
    let mentions_a_number = amount.iter().any(|token| {
        matches!(token.tok, Tok::Int { .. } | Tok::Float { .. })
            || name_word(token).is_some_and(|word| word.starts_with(|c: char| c.is_ascii_digit()))
    });
    let names_a_known_value =
        amount.len() == 1 && name_word(&amount[0]).is_some_and(|word| known_names.contains(word));
    if !mentions_a_number && !names_a_known_value {
        return Ok(None);
    }
    if let Some(seconds) = parse_wait_amount(source, amount) {
        return Ok(Some(NmeStmt::Wait { seconds }));
    }
    Err(wait_amount_diagnostic(span_of(tokens)))
}

fn wait_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<usize> {
    action_phrase_at(tokens, start, WAIT_WORDS_EN, mode)
        .or_else(|| action_phrase_at(tokens, start, WAIT_WORDS_KO, mode))
}

/// Index of a wait action that finishes the line, as Korean word order puts it.
fn wait_action_ending(tokens: &[Token], mode: MatchMode) -> Option<usize> {
    let mut end = tokens.len();
    while end > 0 && is_command_ending(&tokens[end - 1]) {
        end -= 1;
    }
    let start_at = end.saturating_sub(2);
    (start_at..end)
        .find(|&start| wait_action_at(tokens, start, mode).is_some_and(|used| start + used == end))
}

/// Strips the words that surround a wait length without being part of it:
/// `for`, `about`, `동안`, the unit words, sentence punctuation, and politeness
/// fillers, wherever the writer put them.
fn wait_amount_core(tokens: &[Token]) -> &[Token] {
    let mut tokens = tokens;
    // `for` lexes as a Python keyword rather than a word, so it is matched by
    // its token as well as by its spelling.
    while tokens.first().is_some_and(|token| {
        matches!(token.tok, Tok::For)
            || token_matches_exact(token, WAIT_FILLER_WORDS)
            || token_matches_exact(token, SENTENCE_FILLERS)
    }) {
        tokens = &tokens[1..];
    }
    while tokens.last().is_some_and(|token| {
        is_command_ending(token)
            || token_matches_exact(token, SECOND_WORDS_EN)
            || token_matches_exact(token, SECOND_WORDS_KO)
            || token_matches_exact(token, WAIT_FILLER_WORDS)
            || token_matches_exact(token, SENTENCE_FILLERS)
    }) {
        tokens = &tokens[..tokens.len() - 1];
    }
    tokens
}

/// A wait length written as one counting word, with or without the Korean
/// unit attached: `two seconds`, `일초`, `세 초`.
fn number_word_wait_amount(tokens: &[Token]) -> Option<Code> {
    let [token] = wait_amount_core(tokens) else {
        return None;
    };
    name_word(token).and_then(|word| number_word_code(word, SECOND_WORDS_KO))
}

fn parse_wait_amount(source: &str, tokens: &[Token]) -> Option<Code> {
    let tokens = wait_amount_core(tokens);
    if tokens.is_empty() {
        return None;
    }
    let span = span_of(tokens);
    if is_valid_python_expression(&source[span.start..span.end]) {
        return Some(Code::Source(span));
    }
    let trimmed = strip_attached_particle_span(source, tokens, SECOND_WORDS_KO)?;
    is_valid_python_expression(&source[trimmed.start..trimmed.end]).then_some(Code::Source(trimmed))
}

fn wait_amount_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::WaitAmountUnparseable,
        "I couldn't understand how long to wait",
        "얼마나 기다릴지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `wait 3 seconds` or `3초 기다려`",
        "`3초 기다려` 또는 `wait 3 seconds`처럼 적어 주세요",
    )
}

// ------------------------------------------------------------- skip a round

/// `skip` / `건너뛰어` — the sentence spelling of Python's `continue`.
fn match_continue(tokens: &[Token], mode: MatchMode) -> Result<Option<NmeStmt>, Diagnostic> {
    let english = action_phrase_at(tokens, 0, CONTINUE_WORDS_EN, mode);
    let Some(consumed) = english.or_else(|| action_phrase_at(tokens, 0, CONTINUE_WORDS_KO, mode))
    else {
        return Ok(None);
    };
    if tokens[consumed..]
        .iter()
        .any(|token| !is_command_ending(token))
    {
        // Korean skip words are ordinary verbs too, so a longer Korean line is
        // left to the sentence rules rather than claimed as a broken command.
        if english.is_none() {
            return Ok(None);
        }
        return Err(Diagnostic::bilingual(
            DiagnosticCode::ContinueCommandUnparseable,
            "I couldn't understand this skip command",
            "이 건너뛰기 명령을 이해하지 못했어요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            "write only `skip` or `건너뛰어`",
            "`건너뛰어` 또는 `skip`만 적어 주세요",
        ));
    }
    Ok(Some(NmeStmt::Continue))
}

// -------------------------------------------------------- adding to a list

/// `append Mina to friends` / `친구들에 민수 넣어`.
///
/// `add` is deliberately not an append word: `add 1 to score` already means a
/// value change, and one spelling may not mean two things.
/// A list statement written on a name that holds a record.
///
/// The opposite direction has been refused since records landed
/// (`not_a_record`), but this one was not, so `표에 사과 넣어` and
/// `append apple to ages` compiled to `표.append("사과")` — a program that
/// dies with `AttributeError: 'dict' object has no attribute 'append'` on a
/// line that reads perfectly. A record is written to by name, and saying so
/// is the whole of the fix.
fn not_a_list(target: &str, known_names: &HashSet<String>, span: Span) -> Option<Diagnostic> {
    if !is_record_name(known_names, target) {
        return None;
    }
    Some(
        Diagnostic::bilingual(
            DiagnosticCode::RecordNameUnknown,
            format!("`{target}` holds a record, not a list"),
            format!("`{target}`에는 목록이 아니라 표가 들어 있어요"),
            span,
        )
        .with_bilingual_hint(
            format!(
                "a record keeps every value under a name, so say which name: \
                 `put Mina at 90 in {target}`"
            ),
            format!(
                "표는 값마다 이름을 붙여 담으니 어느 이름인지 적어 주세요: \
                 `{target}에 민수를 90으로 넣어`"
            ),
        ),
    )
}

fn match_append(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // `… then show Time to sleep` ends in a waiting word but is a condition
    // with a message, not a wait. The opening word decides the line.
    if tokens.first().is_some_and(starts_a_different_statement) {
        return Ok(None);
    }
    // `to friends append Mina` — the connector moved to the front. `to` is
    // never a name a beginner sets, so this ordering can only be the list
    // line the writer meant.
    if tokens.len() > 3
        && token_matches_exact(&tokens[0], APPEND_CONNECTORS_EN)
        && name_word(&tokens[1]).is_some()
    {
        if let Some(consumed) = action_phrase_at(tokens, 2, APPEND_WORDS_EN, mode) {
            let target = name_word(&tokens[1])
                .map(str::to_string)
                .ok_or_else(|| append_diagnostic(tokens[1].span))?;
            let value_tokens = trim_command_endings(&tokens[2 + consumed..]);
            if value_tokens.is_empty() {
                return Err(append_diagnostic(span_of(tokens)));
            }
            if let Some(problem) = not_a_list(&target, known_names, span_of(tokens)) {
                return Err(problem);
            }
            let value = parse_value(source, value_tokens, known_names, true)
                .map_err(|()| append_diagnostic(span_of(tokens)))?;
            return Ok(Some(NmeStmt::Append { target, value }));
        }
    }
    // `friends append Mina` — the list first, the order Korean writes and the
    // one a beginner reaches for after reading `friends.append("Mina")`. The
    // name has to be one the program already made a list, so no sentence can
    // land here.
    if tokens.len() >= 3 {
        if let Some(target) = list_name_at(&tokens[0], known_names) {
            if let Some(consumed) = action_phrase_at(tokens, 1, APPEND_WORDS_EN, mode) {
                let value_tokens = trim_command_endings(&tokens[1 + consumed..]);
                if !value_tokens.is_empty() {
                    let value = parse_value(source, value_tokens, known_names, true)
                        .map_err(|()| append_diagnostic(span_of(tokens)))?;
                    return Ok(Some(NmeStmt::Append { target, value }));
                }
            }
        }
    }
    if let Some(consumed) = action_phrase_at(tokens, 0, APPEND_WORDS_EN, mode) {
        let mut end = tokens.len();
        while end > consumed && is_command_ending(&tokens[end - 1]) {
            end -= 1;
        }
        let rest = &tokens[consumed..end];
        let Some(separator) = rest
            .iter()
            .position(|token| token_matches_exact(token, APPEND_CONNECTORS_EN))
        else {
            return Err(append_diagnostic(span_of(tokens)));
        };
        let (value_tokens, target_tokens) = (&rest[..separator], &rest[separator + 1..]);
        if value_tokens.is_empty() || target_tokens.len() != 1 {
            return Err(append_diagnostic(span_of(tokens)));
        }
        let target = name_word(&target_tokens[0])
            .map(str::to_string)
            .ok_or_else(|| append_diagnostic(target_tokens[0].span))?;
        if let Some(problem) = not_a_list(&target, known_names, span_of(tokens)) {
            return Err(problem);
        }
        let value = parse_value(source, value_tokens, known_names, true)
            .map_err(|()| append_diagnostic(span_of(tokens)))?;
        return Ok(Some(NmeStmt::Append { target, value }));
    }

    // Korean puts the action last: `<목록>에 <값> 넣어`.
    let Some(action_start) = korean_append_action_start(tokens, mode) else {
        return korean_leading_append(source, tokens, known_names, mode);
    };
    if action_start < 2 {
        return Ok(None);
    }
    let head = &tokens[..action_start];
    // The target particle is what separates `친구들에 민수 넣어` from ordinary
    // speech such as `설탕을 넣어`; without it this is not a list line.
    let marked: Vec<usize> = head
        .iter()
        .enumerate()
        .filter(|(_, token)| korean_append_target(token).is_some())
        .map(|(index, _)| index)
        .collect();
    // `친구들 민수 넣어` — the particle left off. Korean drops `에` in speech
    // all the time, and a beginner copying `친구들에 민수 넣어` drops it too.
    // The name has to be one the program already made a list, so ordinary
    // speech such as `설탕 한 스푼 넣어` still stays speech.
    if marked.is_empty() {
        let Some(target) = list_name_at(&head[0], known_names) else {
            return Ok(None);
        };
        let value_tokens = trim_suffix_say_value(&head[1..]);
        if value_tokens.is_empty() {
            return Ok(None);
        }
        let value = parse_value(source, &value_tokens, known_names, true)
            .map_err(|()| append_diagnostic(span_of(tokens)))?;
        return Ok(Some(NmeStmt::Append { target, value }));
    }
    // Two words carrying the particle could each be the list, and guessing
    // between them is exactly what must not happen.
    let [target_at] = marked[..] else {
        return Ok(None);
    };
    // Only the two ends are read: a list marked in the middle would leave the
    // value in two pieces with the list name sitting between them.
    if target_at != 0 && target_at + 1 != head.len() {
        return Ok(None);
    }
    let target = korean_append_target(&head[target_at]).expect("checked above");
    // `민수를 친구들에 넣어` puts the value first. That order is only claimed
    // when the list is a name the program already made, so ordinary speech
    // such as `설탕을 그릇에 넣어` stays speech.
    if target_at != 0 && !known_names.contains(&target) {
        return Ok(None);
    }
    // A **repaired** list word is a guess, and a guess may not also invent the
    // container it puts something in. `너에게 하고 싶은 말이 있어` put `있어`
    // one character from `넣어` and became `너.append("하고 싶은 말이")`, a
    // program that dies with `NameError` on a line that reads as a sentence.
    // The exact spellings are untouched.
    if target_at == 0 && mode == MatchMode::Recover && !known_names.contains(&target) {
        return Ok(None);
    }
    // `그릇에 설탕을 한 스푼으로 넣어` marks a name **and** a value, which is
    // the record shape, not the list one — and `그릇` is not something this
    // program ever made. Appending the whole of `설탕을 한 스푼으로` as one
    // piece of text writes a program nobody asked for, so somebody cooking
    // keeps their sentence. When the name *is* a list the record matcher has
    // already refused the line and said why.
    if target_at == 0 && !known_names.contains(&target) && korean_record_shape(head, known_names) {
        return Ok(None);
    }
    let value_tokens = if target_at == 0 {
        trim_suffix_say_value(&head[1..])
    } else {
        trim_suffix_say_value(&head[..target_at])
    };
    if value_tokens.is_empty() {
        return Ok(None);
    }
    if let Some(problem) = not_a_list(&target, known_names, span_of(tokens)) {
        return Err(problem);
    }
    let value = parse_value(source, &value_tokens, known_names, true)
        .map_err(|()| append_diagnostic(span_of(tokens)))?;
    Ok(Some(NmeStmt::Append { target, value }))
}

/// The list name a word points at, once its `에`/`한테` particle is removed.
fn korean_append_target(token: &Token) -> Option<String> {
    name_word(token)
        .and_then(|word| strip_any_suffix(word, APPEND_TARGET_PARTICLES_KO))
        .map(str::to_string)
}

/// Index of a Korean list-adding word that closes the line.
fn korean_append_action_start(tokens: &[Token], mode: MatchMode) -> Option<usize> {
    let end = trim_command_endings(tokens).len();
    let start_at = end.saturating_sub(2);
    (start_at..end).find(|&start| {
        action_phrase_at(tokens, start, APPEND_WORDS_KO, mode)
            .is_some_and(|used| start + used == end)
    })
}

/// `넣어 친구들에 민수` — the action word may also open the line, the way it
/// does when the action is dictated first. The list must already exist, which
/// is what keeps ordinary speech out.
fn korean_leading_append(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(consumed) = action_phrase_at(tokens, 0, APPEND_WORDS_KO, mode) else {
        return Ok(None);
    };
    let rest = trim_command_endings(&tokens[consumed..]);
    if rest.len() < 2 {
        return Ok(None);
    }
    let Some(target) = korean_append_target(&rest[0]).filter(|name| known_names.contains(name))
    else {
        return Ok(None);
    };
    let value_tokens = trim_suffix_say_value(&rest[1..]);
    if value_tokens.is_empty() {
        return Ok(None);
    }
    if let Some(problem) = not_a_list(&target, known_names, span_of(tokens)) {
        return Err(problem);
    }
    let value = parse_value(source, &value_tokens, known_names, true)
        .map_err(|()| append_diagnostic(span_of(tokens)))?;
    Ok(Some(NmeStmt::Append { target, value }))
}

fn append_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AppendUnparseable,
        "I couldn't understand what to add to the list",
        "목록에 무엇을 넣을지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `append Mina to friends` or `친구들에 민수 넣어`",
        "`친구들에 민수 넣어` 또는 `append Mina to friends`처럼 적어 주세요",
    )
}

// ------------------------------------------------ readings taken from a name

/// The list this word names, once any Korean particle is off it.
///
/// `None` unless the program made that name a list earlier. This is the gate
/// under every list reading: `the first of many` and `sort out your things`
/// name nothing the program ever made, so they stay ordinary sentences.
fn list_name_at(token: &Token, known_names: &HashSet<String>) -> Option<String> {
    let word = name_word(token)?;
    let name = resolve_known_particle(word, known_names)?;
    is_list_name(known_names, name).then(|| name.to_string())
}

/// The saved name this word points at, whether or not it holds a list.
fn saved_name_at(token: &Token, known_names: &HashSet<String>) -> Option<String> {
    let word = name_word(token)?;
    resolve_known_particle(word, known_names).map(str::to_string)
}

/// The record this word names, once any Korean particle is off it.
///
/// `None` unless the program made that name a record. Every record statement
/// hangs off this: `in`, `의`, `넣어` and `빼` are ordinary words, and the
/// name is the only thing that can say which kind of container is meant.
fn record_name_at(token: &Token, known_names: &HashSet<String>) -> Option<String> {
    let word = name_word(token)?;
    let name = resolve_known_particle(word, known_names)?;
    is_record_name(known_names, name).then(|| name.to_string())
}

/// A name that can be counted: a list, or a record. `개수` and `how many`
/// mean the same thing for both, and `len(...)` is the Python for both.
fn countable_name_at(token: &Token, known_names: &HashSet<String>) -> Option<String> {
    let word = name_word(token)?;
    let name = resolve_known_particle(word, known_names)?;
    (is_list_name(known_names, name) || is_record_name(known_names, name)).then(|| name.to_string())
}

/// The name a record keeps one value under: a name the program already saved,
/// a number, a quoted string, or the word itself as text.
///
/// A saved name wins, because that is what a loop over a record hands the
/// writer: `for each name in ages` then `show name in ages`.
fn record_key_value(
    token: &Token,
    known_names: &HashSet<String>,
    particles: &[&str],
) -> Option<Value> {
    if matches!(
        token.tok,
        Tok::Int { .. } | Tok::Float { .. } | Tok::String { .. }
    ) {
        return Some(Value::Python(Code::Source(token.span)));
    }
    let word = name_word(token)?;
    let word = resolve_known_particle(word, known_names)
        .unwrap_or_else(|| strip_any_suffix(word, particles).unwrap_or(word));
    Some(record_key_from_word(word, known_names))
}

fn record_key_from_word(word: &str, known_names: &HashSet<String>) -> Value {
    if known_names.contains(word) {
        Value::Python(Code::Generated(word.to_string()))
    } else {
        Value::Text(TextTemplate {
            parts: vec![TextPart::Literal(word.to_string())],
        })
    }
}

/// A reading word may carry a subject particle when it stands in a condition.
fn strip_reading_particle(word: &str) -> &str {
    strip_any_suffix(word, READING_PARTICLES_KO).unwrap_or(word)
}

/// True when `word` is one of `words`, with or without a subject particle.
///
/// The word as written is tried first, because several reading words end in
/// what is also a particle: stripping `길이` would leave `길`, and `작은`
/// would leave `작`.
fn reading_word_matches(word: &str, words: &[&str]) -> bool {
    words.contains(&word) || words.contains(&strip_reading_particle(word))
}

/// Which reading a word asks for, and what kind of name it may be taken from.
struct ReadingWord {
    kind: ReadingKind,
    used: usize,
    needs: NameNeeded,
}

/// What a reading needs of the name it is taken from.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NameNeeded {
    /// `the total of scores` — only a list. A record has no order and no
    /// numbers to add up, and refusing says so.
    List,
    /// `how many` — a list or a record. `len(...)` is right for both.
    ListOrRecord,
    /// `the length of name` — any name the program saved.
    Saved,
}

enum ReadingKind {
    Value(Reading),
    Position(ItemPosition),
}

/// One sentence reading — `친구들 개수`, `how many friends`, `the first of
/// friends` — and how many tokens it used.
///
/// The count is what lets a condition read one as its left-hand side while a
/// value still requires the reading to be the whole of what it was given.
fn reading_prefix(tokens: &[Token], known_names: &HashSet<String>) -> Option<(Value, usize)> {
    english_reading_prefix(tokens, known_names)
        .or_else(|| korean_reading_prefix(tokens, known_names))
}

fn english_reading_prefix(
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<(Value, usize)> {
    // `the remainder of pile divided by 4`.
    if let Some(found) = english_remainder(tokens, known_names) {
        return Some(found);
    }
    // `name in capitals` — the one English reading that says the name first,
    // because that is the order the words come in.
    if tokens.len() > 2 && matches!(tokens[1].tok, Tok::In) {
        if let Some(of) = saved_name_at(&tokens[0], known_names) {
            if let Some((reading, used)) = english_case_words(tokens, 2) {
                return Some((Value::Reading { of, reading }, 2 + used));
            }
        }
    }
    // `Mina in ages` — one value out of a record. `in` is one of the
    // commonest words in English, so the record name carries the whole gate:
    // `the best in class` names nothing the program made.
    if tokens.len() > 2
        && (matches!(tokens[1].tok, Tok::In) || token_matches_exact(&tokens[1], &["in"]))
    {
        if let Some(of) = record_name_at(&tokens[2], known_names) {
            if let Some(key) = record_key_value(&tokens[0], known_names, READING_PARTICLES_KO) {
                return Some((
                    Value::Entry {
                        of,
                        key: Box::new(key),
                    },
                    3,
                ));
            }
        }
    }
    // `friends joined by comma` · `friends joined together` — likewise, and
    // gated on a real list.
    if tokens.len() > 2 {
        if let Some(of) = list_name_at(&tokens[0], known_names) {
            if token_matches_exact(&tokens[1], JOIN_WORDS_EN) {
                let mut cursor = 2;
                if tokens.get(cursor).is_some_and(is_separator_connector) {
                    cursor += 1;
                }
                let (separator, used) = english_separator(tokens, cursor)?;
                return Some((Value::Joined { of, separator }, cursor + used));
            }
        }
    }
    // `star repeated 5 times` — the same text over and over, and gated on a
    // saved name for the same reason.
    if tokens.len() > 3 {
        if let Some(of) = saved_name_at(&tokens[0], known_names) {
            if token_matches_exact(&tokens[1], REPEAT_TEXT_WORDS_EN)
                && token_matches_exact(&tokens[3], COPIES_WORDS_EN)
            {
                if let Some(times) = remainder_divisor(&tokens[2], known_names) {
                    return Some((Value::Repeated { of, times }, 4));
                }
            }
        }
    }
    // `memo split by line` · `line split by comma` — the opposite, and gated
    // on any saved name, because what is cut up is text rather than a list.
    if tokens.len() > 3 {
        if let Some(of) = saved_name_at(&tokens[0], known_names) {
            if token_matches_exact(&tokens[1], SPLIT_WORDS_EN)
                && token_matches_exact(&tokens[2], &["by", "on", "at", "into"])
            {
                if let Some(by) = english_split_target(&tokens[3]) {
                    return Some((Value::Split { of, by }, 4));
                }
            }
        }
    }

    let mut at = 0;
    if token_matches_exact(tokens.first()?, &["the", "a", "an"]) {
        at += 1;
    }
    // `how many friends`.
    if tokens
        .get(at)
        .is_some_and(|token| token_matches_exact(token, READING_LEAD_WORDS_EN))
        && tokens
            .get(at + 1)
            .is_some_and(|token| token_matches_exact(token, COUNT_WORDS_EN))
    {
        let of = countable_name_at(tokens.get(at + 2)?, known_names)?;
        return Some((
            Value::Reading {
                of,
                reading: Reading::Count,
            },
            at + 3,
        ));
    }
    // `item 3 of friends`.
    if tokens
        .get(at)
        .is_some_and(|token| token_matches_exact(token, ITEM_WORDS_EN))
    {
        let position = english_item_index(tokens.get(at + 1)?, known_names)?;
        if !tokens
            .get(at + 2)
            .is_some_and(|token| token_matches_exact(token, &["of", "in"]))
        {
            return None;
        }
        let of = list_name_at(tokens.get(at + 3)?, known_names)?;
        return Some((Value::Item { of, position }, at + 4));
    }
    // `<reading> of <name>` — the shape the rest of them share. The `of` is
    // what keeps `count me in` and `the first of many` out.
    let word = english_reading_word(tokens, at)?;
    let mut cursor = at + word.used;
    if !tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["of", "in"]))
    {
        return None;
    }
    cursor += 1;
    let name_token = tokens.get(cursor)?;
    let of = match word.needs {
        NameNeeded::List => list_name_at(name_token, known_names)?,
        NameNeeded::ListOrRecord => countable_name_at(name_token, known_names)?,
        NameNeeded::Saved => saved_name_at(name_token, known_names)?,
    };
    let value = match word.kind {
        ReadingKind::Value(reading) => Value::Reading { of, reading },
        ReadingKind::Position(position) => Value::Item { of, position },
    };
    Some((value, cursor + 1))
}

/// `capitals` · `capital letters` · `uppercase` · `small letters` · `lowercase`.
fn english_case_words(tokens: &[Token], at: usize) -> Option<(Reading, usize)> {
    let (reading, mut used) = if token_matches_exact(tokens.get(at)?, CAPITALS_WORDS_EN) {
        (Reading::Capitals, 1)
    } else if token_matches_exact(tokens.get(at)?, SMALL_LETTERS_WORDS_EN) {
        (Reading::SmallLetters, 1)
    } else {
        return None;
    };
    if tokens
        .get(at + used)
        .is_some_and(|token| token_matches_exact(token, &["letters", "case"]))
    {
        used += 1;
    }
    Some((reading, used))
}

fn english_reading_word(tokens: &[Token], at: usize) -> Option<ReadingWord> {
    let token = tokens.get(at)?;
    let listed = |kind, used| {
        Some(ReadingWord {
            kind,
            used,
            needs: NameNeeded::List,
        })
    };
    if token_matches_exact(token, COUNT_WORDS_EN) {
        return Some(ReadingWord {
            kind: ReadingKind::Value(Reading::Count),
            used: 1,
            needs: NameNeeded::ListOrRecord,
        });
    }
    if token_matches_exact(token, TOTAL_WORDS_EN) {
        return listed(ReadingKind::Value(Reading::Total), 1);
    }
    if token_matches_exact(token, LARGEST_WORDS_EN) {
        return listed(ReadingKind::Value(Reading::Largest), 1);
    }
    if token_matches_exact(token, SMALLEST_WORDS_EN) {
        return listed(ReadingKind::Value(Reading::Smallest), 1);
    }
    if token_matches_exact(token, FIRST_WORDS_EN) {
        return listed(ReadingKind::Position(ItemPosition::First), 1);
    }
    if token_matches_exact(token, LAST_WORDS_EN) {
        return listed(ReadingKind::Position(ItemPosition::Last), 1);
    }
    // The two text readings work on any saved name, because a name holding a
    // number or a word is exactly what they are for.
    if token_matches_exact(token, LENGTH_WORDS_EN) {
        return Some(ReadingWord {
            kind: ReadingKind::Value(Reading::Count),
            used: 1,
            needs: NameNeeded::Saved,
        });
    }
    let (reading, used) = english_case_words(tokens, at)?;
    Some(ReadingWord {
        kind: ReadingKind::Value(reading),
        used,
        needs: NameNeeded::Saved,
    })
}

/// The one-based position in `item 3 of friends`. Zero is refused elsewhere,
/// where there is a span to point a caret at.
fn english_item_index(token: &Token, known_names: &HashSet<String>) -> Option<ItemPosition> {
    match &token.tok {
        Tok::Int { .. } => {
            (!is_zero_position(token)).then_some(ItemPosition::Numbered(Code::Source(token.span)))
        }
        Tok::Name { name } if known_names.contains(name) => {
            Some(ItemPosition::Numbered(Code::Source(token.span)))
        }
        _ => None,
    }
}

fn is_zero_position(token: &Token) -> bool {
    matches!(&token.tok, Tok::Int { value } if value.to_string() == "0")
}

/// A named separator, a written one, or the comma mark itself.
fn english_separator(tokens: &[Token], at: usize) -> Option<(String, usize)> {
    let token = tokens.get(at)?;
    if let Some(text) = named_separator(token) {
        return Some((text, 1));
    }
    if matches!(token.tok, Tok::Comma) {
        return Some((", ".to_string(), 1));
    }
    None
}

/// `comma` · `space` · `newline` and their Korean twins, with the Korean
/// `로`/`으로` taken off. The comma is a comma and a space, because that is
/// how a list is read out loud.
///
/// `nothing` and `그대로` are separators too, and what they name is the empty
/// one: `친구들을 그대로 이어` runs the items together. They are matched before
/// the particle is stripped, because `그대로` ends in what is also a particle.
fn named_separator(token: &Token) -> Option<String> {
    let word = name_word(token)?;
    if EMPTY_SEPARATOR_WORDS_EN.contains(&word)
        || EMPTY_SEPARATOR_WORDS_KO.contains(&word)
        || JOIN_TOGETHER_WORDS_EN.contains(&word)
    {
        return Some(String::new());
    }
    let word = strip_any_suffix(word, &["으로", "로"]).unwrap_or(word);
    if !SEPARATOR_WORDS_EN.contains(&word) && !SEPARATOR_WORDS_KO.contains(&word) {
        return None;
    }
    let text = match word {
        "comma" | "쉼표" => ", ",
        "space" | "빈칸" | "공백" => " ",
        "newline" | "줄바꿈" => "\n",
        _ => return None,
    };
    Some(text.to_string())
}

/// `line` · `comma` · `space` · `newline`, or the comma mark itself.
fn english_split_target(token: &Token) -> Option<SplitBy> {
    if token_matches_exact(token, SPLIT_LINE_WORDS_EN) {
        return Some(SplitBy::Lines);
    }
    if matches!(token.tok, Tok::Comma) {
        return Some(SplitBy::Text(",".to_string()));
    }
    split_separator(token).map(SplitBy::Text)
}

/// `줄마다` · `쉼표로` · `빈칸으로` · `줄바꿈으로`.
fn korean_split_target(token: &Token) -> Option<SplitBy> {
    if token_matches_exact(token, SPLIT_LINE_WORDS_KO) {
        return Some(SplitBy::Lines);
    }
    split_separator(token).map(SplitBy::Text)
}

/// The separator a **split** cuts on, which is not always the one a join puts
/// in. A joined list reads `Mina, Ada` out loud, so its comma carries a space;
/// a line read back out of a file says `Mina,Ada`, and looking for `", "`
/// there would find nothing. So the comma is `","` here and `", "` there.
fn split_separator(token: &Token) -> Option<String> {
    let word = name_word(token)?;
    let word = strip_any_suffix(word, &["으로", "로"]).unwrap_or(word);
    let text = match word {
        "comma" | "쉼표" => ",",
        "space" | "빈칸" | "공백" => " ",
        "newline" | "줄바꿈" => "\n",
        _ => return None,
    };
    Some(text.to_string())
}

/// `the remainder of pile divided by 4` — the one English reading with two
/// operands, so it is read before the single-operand shapes.
fn english_remainder(tokens: &[Token], known_names: &HashSet<String>) -> Option<(Value, usize)> {
    let at = usize::from(token_matches_exact(tokens.first()?, &["the", "a", "an"]));
    if !token_matches_exact(tokens.get(at)?, REMAINDER_WORDS_EN) {
        return None;
    }
    if !token_matches_exact(tokens.get(at + 1)?, &["of", "when"]) {
        return None;
    }
    let of = saved_name_at(tokens.get(at + 2)?, known_names)?;
    if !token_matches_exact(tokens.get(at + 3)?, DIVIDED_WORDS_EN) {
        return None;
    }
    if !token_matches_exact(tokens.get(at + 4)?, &["by", "into"]) {
        return None;
    }
    let by = remainder_divisor(tokens.get(at + 5)?, known_names)?;
    Some((Value::Remainder { of, by }, at + 6))
}

/// The number a remainder divides by: a written number, or a saved name.
fn remainder_divisor(token: &Token, known_names: &HashSet<String>) -> Option<Code> {
    match &token.tok {
        Tok::Int { .. } | Tok::Float { .. } => Some(Code::Source(token.span)),
        Tok::Name { name } if known_names.contains(name) => Some(Code::Source(token.span)),
        _ => None,
    }
}

fn korean_reading_prefix(
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<(Value, usize)> {
    // `나이표의 민수` · `나이표에서 민수` — one value out of a record. Read
    // first because `의` is a particle every other reading also allows, and
    // only a record name can reach this at all.
    if let Some(found) = korean_record_entry(tokens, known_names) {
        return Some(found);
    }
    let name_token = tokens.first()?;
    let name = saved_name_at(name_token, known_names)?;
    let rest = &tokens[1..];
    let listed = is_list_name(known_names, &name);

    // `쌓인돌을 4로 나눈 나머지`. The lexer cuts `4로` into the number and the
    // particle, so the phrase is four tokens after the name.
    if rest.len() > 3
        && token_matches_exact(&rest[1], &["으로", "로"])
        && token_matches_exact(&rest[2], DIVIDED_WORDS_KO)
        && reading_word_matches(name_word(&rest[3])?, REMAINDER_WORDS_KO)
    {
        if let Some(by) = remainder_divisor(&rest[0], known_names) {
            return Some((Value::Remainder { of: name, by }, 5));
        }
    }
    // `별표를 5개 붙인 것`, and `별표를 5개 이어 붙인 것` written with the two
    // halves of the verb apart, which is how people type it.
    if rest.len() > 3 && token_matches_exact(&rest[1], COPIES_WORDS_KO) {
        let verb_at = 2 + usize::from(token_matches_exact(&rest[2], JOIN_WORDS_KO));
        if rest.len() > verb_at + 1
            && token_matches_exact(&rest[verb_at], REPEAT_TEXT_WORDS_KO)
            && token_matches_exact(&rest[verb_at + 1], SPLIT_THING_WORDS_KO)
        {
            if let Some(times) = remainder_divisor(&rest[0], known_names) {
                return Some((
                    Value::Repeated {
                        of: name.clone(),
                        times,
                    },
                    verb_at + 3,
                ));
            }
        }
    }
    // `메모를 줄마다 나눈 것` — one piece of text cut into a list. Read before
    // the list readings because what is cut up is text, and a name holding
    // text is not a list name.
    if rest.len() > 2
        && token_matches_exact(&rest[1], SPLIT_WORDS_KO)
        && token_matches_exact(&rest[2], SPLIT_THING_WORDS_KO)
    {
        if let Some(by) = korean_split_target(&rest[0]) {
            return Some((Value::Split { of: name, by }, 4));
        }
    }
    if listed {
        // `친구들을 붙여` — every item run together with nothing between them.
        // The verb carries its own separator, so no separator word stands in
        // front of it.
        if rest
            .first()
            .is_some_and(|token| token_matches_exact(token, JOIN_TOGETHER_WORDS_KO))
        {
            return Some((
                Value::Joined {
                    of: name,
                    separator: String::new(),
                },
                2,
            ));
        }
        // `친구들을 쉼표로 이어`, `친구들을 그대로 이어` — every item in one
        // piece of text.
        if rest.len() > 1 {
            if let Some(separator) = named_separator(&rest[0]) {
                if token_matches_exact(&rest[1], JOIN_WORDS_KO) {
                    return Some((
                        Value::Joined {
                            of: name,
                            separator,
                        },
                        3,
                    ));
                }
            }
        }
        // `점수들 중 가장 큰 것`.
        if let Some((reading, used)) = korean_extreme_phrase(rest) {
            return Some((Value::Reading { of: name, reading }, 1 + used));
        }
        // `친구들 첫 번째` · `친구들 마지막` · `친구들 3번째`.
        if let Some((position, used)) = korean_item_position(rest, known_names) {
            return Some((Value::Item { of: name, position }, 1 + used));
        }
    }
    let word = name_word(rest.first()?)?;
    let recorded = is_record_name(known_names, &name);
    for (words, reading) in [
        (COUNT_WORDS_KO, Reading::Count),
        (TOTAL_WORDS_KO, Reading::Total),
        (LARGEST_WORDS_KO, Reading::Largest),
        (SMALLEST_WORDS_KO, Reading::Smallest),
    ] {
        // `나이표 개수` counts a record the same way `친구들 개수` counts a
        // list. The other three have no meaning for a record: there is no
        // order to it and nothing to add up.
        let allowed = listed || (recorded && reading == Reading::Count);
        if allowed && reading_word_matches(word, words) {
            return Some((Value::Reading { of: name, reading }, 2));
        }
    }
    for (words, reading) in [
        (LENGTH_WORDS_KO, Reading::Count),
        (CAPITALS_WORDS_KO, Reading::Capitals),
        (SMALL_LETTERS_WORDS_KO, Reading::SmallLetters),
    ] {
        if reading_word_matches(word, words) {
            return Some((Value::Reading { of: name, reading }, 2));
        }
    }
    None
}

/// `나이표의 민수` · `나이표에서 민수` — one value a record keeps.
///
/// Two words exactly. The first carries `의` or `에서` and names a record the
/// program made; the second is the name that value is kept under. `의` is the
/// commonest particle in Korean, so without the record set behind it this
/// would claim half the language.
fn korean_record_entry(tokens: &[Token], known_names: &HashSet<String>) -> Option<(Value, usize)> {
    let word = name_word(tokens.first()?)?;
    let of = strip_any_suffix(word, RECORD_OF_PARTICLES_KO)
        .filter(|base| is_record_name(known_names, base))?;
    let key = record_key_value(tokens.get(1)?, known_names, READING_PARTICLES_KO)?;
    Some((
        Value::Entry {
            of: of.to_string(),
            key: Box::new(key),
        },
        2,
    ))
}

/// `중 가장 큰 것` · `가장 작은 값` · `중에서 제일 큰`.
///
/// `가장`/`제일` is required, so a list name followed by the ordinary word
/// `큰` is not a reading.
fn korean_extreme_phrase(tokens: &[Token]) -> Option<(Reading, usize)> {
    let mut at = 0;
    if tokens
        .first()
        .is_some_and(|token| token_matches_exact(token, EXTREME_SCOPE_WORDS_KO))
    {
        at += 1;
    }
    if !tokens
        .get(at)
        .is_some_and(|token| token_matches_exact(token, EXTREME_MOST_WORDS_KO))
    {
        return None;
    }
    at += 1;
    let word = name_word(tokens.get(at)?)?;
    let reading = if reading_word_matches(word, LARGEST_WORDS_KO) {
        Reading::Largest
    } else if reading_word_matches(word, SMALLEST_WORDS_KO) {
        Reading::Smallest
    } else {
        return None;
    };
    at += 1;
    if tokens
        .get(at)
        .is_some_and(|token| token_matches_exact(token, EXTREME_THING_WORDS_KO))
    {
        at += 1;
    }
    Some((reading, at))
}

fn korean_item_position(
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<(ItemPosition, usize)> {
    let first = tokens.first()?;
    if let Some(word) = name_word(first) {
        if reading_word_matches(word, FIRST_WORDS_KO) {
            // `첫` on its own is only half the word; it needs its counter.
            if word == "첫" {
                return tokens
                    .get(1)
                    .and_then(|token| name_word(token))
                    .filter(|word| reading_word_matches(word, ITEM_WORDS_KO))
                    .map(|_| (ItemPosition::First, 2));
            }
            return Some((ItemPosition::First, 1));
        }
        if reading_word_matches(word, LAST_WORDS_KO) {
            return Some((ItemPosition::Last, 1));
        }
    }
    // `친구들 3번째` — the lexer splits the number from the counter.
    let counter = tokens.get(1)?;
    if !reading_word_matches(name_word(counter)?, ITEM_WORDS_KO) {
        return None;
    }
    let position = match &first.tok {
        Tok::Int { .. } => (!is_zero_position(first))
            .then_some(ItemPosition::Numbered(Code::Source(first.span)))?,
        Tok::Name { name } if known_names.contains(name) => {
            ItemPosition::Numbered(Code::Source(first.span))
        }
        _ => return None,
    };
    Some((position, 2))
}

/// `item 0 of friends` / `친구들 0번째`.
///
/// Reading it as `friends[-1]` would quietly hand back the *last* item, which
/// is the opposite of what the writer asked for, so the line is refused with
/// the rule spelled out instead.
fn zero_item_position(tokens: &[Token], known_names: &HashSet<String>) -> Option<Diagnostic> {
    let zero_at = tokens.iter().position(is_zero_position)?;
    let english = zero_at >= 1
        && token_matches_exact(&tokens[zero_at - 1], ITEM_WORDS_EN)
        && tokens
            .get(zero_at + 1)
            .is_some_and(|token| token_matches_exact(token, &["of", "in"]))
        && tokens
            .get(zero_at + 2)
            .is_some_and(|token| list_name_at(token, known_names).is_some());
    let korean = zero_at >= 1
        && list_name_at(&tokens[zero_at - 1], known_names).is_some()
        && tokens.get(zero_at + 1).is_some_and(|token| {
            name_word(token).is_some_and(|word| reading_word_matches(word, ITEM_WORDS_KO))
        });
    (english || korean).then(|| {
        Diagnostic::bilingual(
            DiagnosticCode::ItemCountsFromOne,
            "items are counted from 1",
            "몇 번째인지는 1부터 셉니다",
            tokens[zero_at].span,
        )
        .with_bilingual_hint(
            "write `item 1 of friends` for the first one, or `the last of friends`",
            "맨 앞은 `친구들 1번째`, 맨 뒤는 `친구들 마지막`이라고 적어 주세요",
        )
    })
}

/// `할 일은 목록` — a Korean name written with a space in the middle of it.
///
/// A name is one word, so `할 일` is two words and the whole line falls
/// through to prose: guide 05 taught exactly this, and the learner saw their
/// own sentence printed back instead of getting a list. Joining the first two
/// words is tried here, and when that turns the line into a **list**
/// statement the line is refused with the spelling that works.
///
/// Only a list statement is claimed. An ordinary sentence does not contain
/// `목록` or `넣어` in this shape by accident, while `오늘 날씨는 맑음` joins
/// into a plain assignment and so keeps printing.
fn name_written_with_a_space(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<Diagnostic> {
    if tokens.len() < 3 {
        return None;
    }
    let first = name_word(&tokens[0])?;
    let second = name_word(&tokens[1])?;
    if !is_hangul(first) || !is_hangul(second) {
        return None;
    }
    let joined = format!("{first}{second}");
    let mut merged = Vec::with_capacity(tokens.len() - 1);
    merged.push(Token {
        tok: Tok::Name {
            name: joined.clone(),
        },
        span: Span::new(tokens[0].span.start, tokens[1].span.end),
    });
    merged.extend(tokens[2..].iter().cloned());
    let makes_a_list = matches!(
        match_set(source, &merged, known_names, MatchMode::Exact),
        Ok(Some(NmeStmt::Set {
            value: Value::List(_),
            ..
        }))
    ) || matches!(
        match_append(source, &merged, known_names, MatchMode::Exact),
        Ok(Some(NmeStmt::Append { .. }))
    );
    if !makes_a_list {
        return None;
    }
    Some(
        Diagnostic::bilingual(
            DiagnosticCode::NameHasSpace,
            "a name cannot have a space in it",
            "이름에는 띄어쓰기를 쓸 수 없어요",
            Span::new(tokens[0].span.start, tokens[1].span.end),
        )
        .with_bilingual_hint(
            format!("write `{joined}` as one word instead of `{first} {second}`"),
            format!("`{first} {second}` 대신 `{joined}`처럼 붙여 써 주세요"),
        ),
    )
}

// ------------------------------------------------- putting a list in order

// ------------------------------------------------ putting a value in a record

/// `put Mina at 90 in ages` / `나이표에 민수를 90으로 넣어`.
///
/// The Korean verb is the list-adding verb, and the Korean particle on the
/// container is the list-adding particle. What separates the two lines is the
/// **shape**: a record line marks a name *and* a value, a list line marks only
/// the thing being added. Read before the list statement, so the extra marks
/// are never swallowed into a piece of text.
fn match_record_put(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if tokens.first().is_some_and(starts_a_different_statement) {
        return Ok(None);
    }
    let body = trim_command_endings(tokens);
    if let Some(stmt) = english_record_put(source, tokens, body, known_names, mode)? {
        return Ok(Some(stmt));
    }
    korean_record_put(source, tokens, body, known_names, mode)
}

/// `put Mina at 90 in ages`.
fn english_record_put(
    source: &str,
    tokens: &[Token],
    body: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(consumed) = action_phrase_at(body, 0, RECORD_PUT_WORDS_EN, mode) else {
        return Ok(None);
    };
    let Some(at_index) =
        (consumed..body.len()).find(|&index| token_matches_exact(&body[index], RECORD_AT_WORDS_EN))
    else {
        return Ok(None);
    };
    let Some(in_index) = (at_index + 1..body.len()).rev().find(|&index| {
        matches!(body[index].tok, Tok::In) || token_matches_exact(&body[index], RECORD_IN_WORDS_EN)
    }) else {
        return Ok(None);
    };
    // The record has to be the last word: `put the kettle on at eight in the
    // morning` leaves words after it and is a sentence.
    if in_index + 2 != body.len() {
        return Ok(None);
    }
    let key_tokens = &body[consumed..at_index];
    let value_tokens = &body[at_index + 1..in_index];
    let [key_token] = key_tokens else {
        return Ok(None);
    };
    if value_tokens.is_empty() {
        return Ok(None);
    }
    let name_token = &body[in_index + 1];
    let Some(target) = record_name_at(name_token, known_names) else {
        return not_a_record(name_token, tokens, known_names);
    };
    let Some(key) = record_key_value(key_token, known_names, READING_PARTICLES_KO) else {
        return Ok(None);
    };
    let value = parse_value(source, value_tokens, known_names, true)
        .map_err(|()| record_put_diagnostic(span_of(tokens)))?;
    Ok(Some(NmeStmt::RecordPut { target, key, value }))
}

/// `나이표에 민수를 90으로 넣어`.
fn korean_record_put(
    source: &str,
    tokens: &[Token],
    body: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(action_start) = korean_record_put_action_start(body, mode) else {
        return Ok(None);
    };
    let head = &body[..action_start];
    if head.len() < 3 {
        return Ok(None);
    }
    let Some(container) =
        name_word(&head[0]).and_then(|word| strip_any_suffix(word, APPEND_TARGET_PARTICLES_KO))
    else {
        return Ok(None);
    };
    let Some((key, value_at)) = korean_record_key(head, known_names) else {
        return Ok(None);
    };
    let Some(value_tokens) = korean_record_value(&head[value_at..]) else {
        return Ok(None);
    };
    if value_tokens.is_empty() {
        return Ok(None);
    }
    if !is_record_name(known_names, container) {
        return not_a_record(&head[0], tokens, known_names);
    }
    let value = parse_value(source, &value_tokens, known_names, true)
        .map_err(|()| record_put_diagnostic(span_of(tokens)))?;
    Ok(Some(NmeStmt::RecordPut {
        target: container.to_string(),
        key,
        value,
    }))
}

/// True when a Korean container line is written in the **record** shape: a
/// name marked with `을`/`를` and a value marked with `으로`/`로`, which is one
/// mark more than adding to a list ever needs.
fn korean_record_shape(head: &[Token], known_names: &HashSet<String>) -> bool {
    head.len() >= 3
        && korean_record_key(head, known_names)
            .is_some_and(|(_, value_at)| korean_record_value(&head[value_at..]).is_some())
}

/// Index of a Korean record-writing word that closes the line.
fn korean_record_put_action_start(tokens: &[Token], mode: MatchMode) -> Option<usize> {
    let end = tokens.len();
    let start_at = end.saturating_sub(2);
    (start_at..end).find(|&start| {
        action_phrase_at(tokens, start, RECORD_PUT_WORDS_KO, mode)
            .is_some_and(|used| start + used == end)
    })
}

/// The name a Korean record line writes under, and where the value begins.
///
/// The particle may be glued to the word (`민수를`) or standing on its own,
/// which is what the lexer leaves behind for a number (`3 을`).
fn korean_record_key(head: &[Token], known_names: &HashSet<String>) -> Option<(Value, usize)> {
    if let Some(at) = head
        .iter()
        .position(|token| token_matches_exact(token, RECORD_KEY_PARTICLES_KO))
    {
        if at == 2 {
            let key = record_key_value(&head[1], known_names, &[])?;
            return Some((key, at + 1));
        }
        return None;
    }
    let word = name_word(head.get(1)?)?;
    let base = strip_any_suffix(word, RECORD_KEY_PARTICLES_KO)?;
    let key = resolve_known_particle(word, known_names).map_or_else(
        || record_key_from_word(base, known_names),
        |name| record_key_from_word(name, known_names),
    );
    Some((key, 2))
}

/// The value a Korean record line writes, with its `으로`/`로` taken off.
///
/// The marker is required: without it `나이표에 민수를 넣어` is the list
/// statement it looks like, and this one declines.
fn korean_record_value(tokens: &[Token]) -> Option<Vec<Token>> {
    let (last, head) = tokens.split_last()?;
    if token_matches_exact(last, RECORD_VALUE_PARTICLES_KO) {
        return Some(head.to_vec());
    }
    let word = name_word(last)?;
    let base = strip_any_suffix(word, RECORD_VALUE_PARTICLES_KO)?;
    let mut value = head.to_vec();
    value.push(Token {
        tok: Tok::Name {
            name: base.to_string(),
        },
        span: Span::new(last.span.start, last.span.start + base.len()),
    });
    Some(value)
}

/// A record line naming something that is not a record.
///
/// A name the program never made anything is left alone — `그릇에 설탕을
/// 스푼으로 넣어` is somebody cooking. A name it made a **list** is refused,
/// because the list statement would otherwise take the whole of
/// `민수를 90` and append it as one piece of text.
fn not_a_record(
    token: &Token,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(name) = name_word(token).and_then(|word| resolve_known_particle(word, known_names))
    else {
        return Ok(None);
    };
    if !is_list_name(known_names, name) {
        return Ok(None);
    }
    Err(Diagnostic::bilingual(
        DiagnosticCode::RecordNameUnknown,
        format!("`{name}` holds a list, not a record"),
        format!("`{name}`에는 표가 아니라 목록이 들어 있어요"),
        span_of(tokens),
    )
    .with_bilingual_hint(
        format!(
            "a list keeps items in order and has no names to write them under; write \
                 `set ages to an empty record` first, or `append Mina to {name}`"
        ),
        format!(
            "목록은 순서대로만 담아서 이름을 붙일 자리가 없어요. 먼저 `나이표는 빈 표`라고 \
                 적거나, `{name}에 민수 넣어`처럼 적어 주세요"
        ),
    ))
}

fn record_put_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::RecordNameUnknown,
        "I couldn't understand what to put in the record",
        "표에 무엇을 넣을지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `put Mina at 90 in ages` or `나이표에 민수를 90으로 넣어`",
        "`나이표에 민수를 90으로 넣어` 또는 `put Mina at 90 in ages`처럼 적어 주세요",
    )
}

// ------------------------------------------------------------- a named job

/// `to greet:` / `인사하기라는 일:` — the name of a job, when the line is one.
///
/// Structure only. `일`, `하기`, `to` and `do` are ordinary words in both
/// languages, so what is asked for is a shape no ordinary sentence has: a
/// closing colon with the phrase and nothing else in front of it.
fn job_header(tokens: &[Token]) -> Option<(String, Vec<String>)> {
    let body = strip_closing_colon(tokens)?;
    korean_job_header(body).or_else(|| english_job_header(body))
}

/// `인사하기라는 일:` · `계산이라는 작업:` · `이름에게 인사하기라는 일:`
fn korean_job_header(body: &[Token]) -> Option<(String, Vec<String>)> {
    let (given, name_token, noun) = match body {
        [name_token, noun] => (None, name_token, noun),
        [given, name_token, noun] => (Some(given), name_token, noun),
        _ => return None,
    };
    if !token_matches_exact(noun, JOB_WORDS_KO) {
        return None;
    }
    let word = name_word(name_token)?;
    let name = strip_any_suffix(word, JOB_NAME_SUFFIXES_KO)?;
    if !is_plain_python_name(name) {
        return None;
    }
    let parameters = match given {
        None => Vec::new(),
        Some(token) => {
            let word = name_word(token)?;
            let parameter = strip_any_suffix(word, JOB_PARAMETER_PARTICLES_KO)?;
            if !is_plain_python_name(parameter) {
                return None;
            }
            vec![parameter.to_string()]
        }
    };
    Some((name.to_string(), parameters))
}

/// `to greet:` · `to greet someone:`
///
/// Every word has to be one somebody would give a job or the thing it is
/// given. `To do:`, `To be:` and `To my knowledge:` are headings, and `do`,
/// `be` and `my` are in `NOT_A_NAME_EN` for exactly this reason.
fn english_job_header(body: &[Token]) -> Option<(String, Vec<String>)> {
    let (lead, name_token, given) = match body {
        [lead, name_token] => (lead, name_token, None),
        [lead, name_token, given] => (lead, name_token, Some(given)),
        _ => return None,
    };
    if !token_matches_exact(lead, JOB_LEAD_WORDS_EN) {
        return None;
    }
    let name = english_job_word(name_token)?;
    if !is_bindable_english_name(&name) {
        return None;
    }
    // The thing the job is given is only checked for being a plain name.
    // `NOT_A_NAME_EN` is the list of words a *sentence* may never quietly turn
    // into a name, and it holds `someone`, `something` and `each` — which are
    // exactly the words a beginner names the thing a job is given. The job
    // name in front of it already carries that check, and the header carries
    // three more: the opening word, the colon, and a block underneath.
    let parameters = match given {
        None => Vec::new(),
        Some(token) => vec![english_job_word(token)?],
    };
    Some((name, parameters))
}

/// One word of an English job header: a plain name that is not a Python
/// keyword.
fn english_job_word(token: &Token) -> Option<String> {
    if is_python_keyword(&token.tok) {
        return None;
    }
    let word = name_word(token)?;
    is_plain_python_name(word).then(|| word.to_string())
}

/// `to greet:` / `인사하기라는 일:` — a piece of program with a name.
///
/// **A colon and a block.** A header with nothing under it is not a job at
/// all: it declines here and the line prints, which is what keeps a lone
/// `To do:` the heading it is. There is no one-line form either — the colon
/// has to close the line — so `to summarise: it was fine` can never become a
/// function.
fn match_job(tokens: &[Token], block: &BlockCtx<'_>) -> Option<NmeStmt> {
    let (name, parameters) = job_header(tokens)?;
    let BlockCtx::TopLevel { line, next_indent } = block else {
        return None;
    };
    if !next_indent.is_some_and(|next| next > line.indent) {
        return None;
    }
    Some(NmeStmt::Job { name, parameters })
}

/// `do greet` · `run greet` / `인사하기 해줘` · `인사하기 실행해`.
///
/// Two words, and the name must be a job the program already made. That is
/// the whole gate: `do` and `해줘` are far too ordinary to carry one.
fn match_run_job(
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let body = trim_command_endings(tokens);
    // `do greet` / `인사하기 해줘` — a job that is given nothing.
    if let [first, second] = body {
        if token_matches_exact(first, RUN_JOB_WORDS_EN) {
            if let Some(stmt) = run_job_taking_nothing(second, known_names)? {
                return Ok(Some(stmt));
            }
        }
        if token_matches_exact(second, RUN_JOB_WORDS_KO) {
            if let Some(stmt) = run_job_taking_nothing(first, known_names)? {
                return Ok(Some(stmt));
            }
        }
        return Ok(None);
    }
    // `do greet with Mina` — the thing it is given closes the line.
    if let [first, name_token, marker, given] = body {
        if token_matches_exact(first, RUN_JOB_WORDS_EN)
            && (matches!(marker.tok, Tok::With) || token_matches_exact(marker, JOB_WITH_WORDS_EN))
        {
            if let Some(value) = record_key_value(given, known_names, &[]) {
                if let Some(stmt) = run_job_taking_one(name_token, value, known_names)? {
                    return Ok(Some(stmt));
                }
            }
        }
    }
    // `민수에게 인사하기 해줘` — Korean marks it with a particle and puts the
    // verb last. It is read after the English shape rather than instead of it,
    // because the lexer splits a particle off a number and `3 에게 두배 해줘`
    // is then four tokens, exactly like `do greet with Mina`.
    let Some((value, used)) = korean_job_argument(body, known_names) else {
        return Ok(None);
    };
    let Some([name_token, verb]) = body.get(used..) else {
        return Ok(None);
    };
    if !token_matches_exact(verb, RUN_JOB_WORDS_KO) {
        return Ok(None);
    }
    run_job_taking_one(name_token, value, known_names)
}

/// `do greet` — a job that is given nothing, unless the job takes one thing,
/// in which case the line is refused rather than run wrong.
fn run_job_taking_nothing(
    token: &Token,
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some(name) = job_call_name(token, known_names, 0) {
        return Ok(Some(NmeStmt::RunJob {
            name,
            arguments: Vec::new(),
        }));
    }
    if job_call_name(token, known_names, 1).is_some() {
        return Err(job_argument_count_diagnostic(token, 1));
    }
    Ok(None)
}

/// `do greet with Mina`, and the same refusal the other way round.
fn run_job_taking_one(
    token: &Token,
    value: Value,
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some(name) = job_call_name(token, known_names, 1) {
        return Ok(Some(NmeStmt::RunJob {
            name,
            arguments: vec![value],
        }));
    }
    if job_call_name(token, known_names, 0).is_some() {
        return Err(job_argument_count_diagnostic(token, 0));
    }
    Ok(None)
}

/// Running a job with the wrong number of things is a Python `TypeError` at
/// run time on a line that looks right, so it is named here instead.
fn job_argument_count_diagnostic(token: &Token, takes: usize) -> Diagnostic {
    let name = name_word(token).unwrap_or("");
    let (english, korean) = if takes == 0 {
        (
            format!("`{name}` is given nothing, so write `do {name}`"),
            format!("`{name}`은(는) 받는 것이 없어요. `{name} 해줘`라고 적어 주세요"),
        )
    } else {
        (
            format!("`{name}` is given one thing, so write `do {name} with Mina`"),
            format!("`{name}`은(는) 하나를 받아요. `민수에게 {name} 해줘`처럼 적어 주세요"),
        )
    };
    Diagnostic::bilingual(
        DiagnosticCode::JobArgumentCount,
        "this job is given a different number of things than it takes",
        "이 일이 받는 것의 개수가 맞지 않아요",
        token.span,
    )
    .with_bilingual_hint(english, korean)
}

/// The job this word names, when the program made one taking `takes` things.
fn job_call_name(token: &Token, known_names: &HashSet<String>, takes: usize) -> Option<String> {
    let word = name_word(token)?;
    let name = resolve_known_particle(word, known_names)?;
    is_job_name(known_names, name, takes).then(|| name.to_string())
}

/// `민수에게` · `3 에게` — the one thing a Korean line hands to a job, and how
/// many tokens it took. The lexer splits a particle off a number and leaves it
/// attached to a word, so both have to be read.
fn korean_job_argument(body: &[Token], known_names: &HashSet<String>) -> Option<(Value, usize)> {
    if body.len() >= 2 && token_matches_exact(&body[1], JOB_PARAMETER_PARTICLES_KO) {
        return record_key_value(body.first()?, known_names, &[]).map(|value| (value, 2));
    }
    let word = name_word(body.first()?)?;
    strip_any_suffix(word, JOB_PARAMETER_PARTICLES_KO)?;
    record_key_value(body.first()?, known_names, JOB_PARAMETER_PARTICLES_KO).map(|value| (value, 1))
}

/// `sort friends` / `친구들 정렬해`, and the two orders beside it.
///
/// Both spellings name exactly one thing, and that thing has to be a list the
/// program made. `sort out your things` names four words and no list, so it
/// is left alone and prints.
fn match_arrange(
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let body = trim_command_endings(tokens);
    if body.len() != 2 && body.len() != 3 {
        return Ok(None);
    }
    for (words, order) in [
        (SORT_WORDS_EN, ListOrder::Sorted),
        (REVERSE_WORDS_EN, ListOrder::Reversed),
        (SHUFFLE_WORDS_EN, ListOrder::Shuffled),
    ] {
        let Some(consumed) = action_phrase_at(body, 0, words, mode) else {
            continue;
        };
        if consumed + 1 != body.len() {
            return Ok(None);
        }
        let Some(target) = list_name_at(&body[consumed], known_names) else {
            return Err(not_a_list_diagnostic(&body[consumed], span_of(tokens)));
        };
        return Ok(Some(NmeStmt::Arrange { target, order }));
    }
    for (words, order) in [
        (SORT_WORDS_KO, ListOrder::Sorted),
        (REVERSE_WORDS_KO, ListOrder::Reversed),
        (SHUFFLE_WORDS_KO, ListOrder::Shuffled),
    ] {
        let Some(consumed) = action_phrase_at(body, 1, words, mode) else {
            continue;
        };
        if 1 + consumed != body.len() {
            continue;
        }
        let Some(target) = list_name_at(&body[0], known_names) else {
            return Err(not_a_list_diagnostic(&body[0], span_of(tokens)));
        };
        return Ok(Some(NmeStmt::Arrange { target, order }));
    }
    Ok(None)
}

/// `별들을 이어 말해줘` · `show stars joined` — a list and a joining word, and
/// nothing saying what goes between the items.
///
/// Left alone these printed themselves — `print(str(별들) + "을 이어")` — which
/// looks like success and is not. Naming them is safe because the list name is
/// the anchor: `이어` and `join` are ordinary words, and only a name the
/// program has already made a list can reach this at all.
fn join_without_a_separator(tokens: &[Token], known_names: &HashSet<String>) -> Option<Diagnostic> {
    for at in 0..tokens.len().saturating_sub(1) {
        let Some(name) = list_name_at(&tokens[at], known_names) else {
            continue;
        };
        let english = token_matches_exact(&tokens[at + 1], JOIN_WORDS_EN);
        let korean = token_matches_exact(&tokens[at + 1], JOIN_WORDS_KO);
        if !english && !korean {
            continue;
        }
        // `이어붙여` is in both lists: it is a joining word, and it carries its
        // own separator, which is nothing. So it is never missing one.
        if token_matches_exact(&tokens[at + 1], JOIN_TOGETHER_WORDS_KO) {
            continue;
        }
        // English writes the separator after the joining word. Korean writes
        // it before, so a joining word standing right after the name has
        // none — which is the whole of this shape.
        if english && english_separator_at(tokens, at + 2).is_some() {
            continue;
        }
        // Anything else left on the line means it is a sentence that happens
        // to hold both words: `친구들을 이어 갔습니다`, `friends join us`.
        // A dangling `by`/`with` is part of the half-written join, not part of
        // a sentence, so it is stepped over first.
        let mut after = at + 2;
        if english && tokens.get(after).is_some_and(is_separator_connector) {
            after += 1;
        }
        if !only_command_furniture(&tokens[after..]) {
            continue;
        }
        return Some(join_separator_missing_diagnostic(&name, span_of(tokens)));
    }
    None
}

/// The separator a join names, wherever the optional `by`/`with` leaves it.
fn english_separator_at(tokens: &[Token], at: usize) -> Option<(String, usize)> {
    let mut cursor = at;
    if tokens.get(cursor).is_some_and(is_separator_connector) {
        cursor += 1;
    }
    english_separator(tokens, cursor)
}

/// `by` and `with`, either of which may stand between `joined` and the
/// separator it names. `with` is a Python keyword, so it never arrives as an
/// ordinary word and has to be named by its token — until this was written,
/// `friends joined with comma` was documented and printed itself.
fn is_separator_connector(token: &Token) -> bool {
    matches!(token.tok, Tok::With) || token_matches_exact(token, &["by", "with"])
}

/// True when every token left is an ending, a politeness word, or an output
/// word — the furniture a command may carry, and nothing that could be the
/// rest of a sentence.
fn only_command_furniture(tokens: &[Token]) -> bool {
    tokens.iter().all(|token| {
        is_command_ending(token)
            || token_matches_exact(token, SENTENCE_FILLERS)
            || token_matches_exact(token, SAY_WORDS_EN)
            || token_matches_exact(token, SAY_WORDS_KO)
    })
}

fn join_separator_missing_diagnostic(name: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::JoinSeparatorMissing,
        format!("this says to join `{name}` but not what to put between the items"),
        format!("`{name}`을(를) 이어 붙이라고 했지만 사이에 무엇을 넣을지가 없어요"),
        span,
    )
    .with_bilingual_hint(
        format!(
            "write `{name} joined by comma`, `{name} joined by space`, or `{name} joined together` for nothing between"
        ),
        format!(
            "`{name}을 쉼표로 이어`, `{name}을 빈칸으로 이어`처럼 쓰세요. 사이에 아무것도 넣지 않으려면 `{name}을 붙여`라고 씁니다"
        ),
    )
}

fn not_a_list_diagnostic(token: &Token, span: Span) -> Diagnostic {
    let name = name_word(token).unwrap_or("");
    Diagnostic::bilingual(
        DiagnosticCode::ListNameUnknown,
        format!("`{name}` was never made into a list"),
        format!("`{name}`을(를) 목록으로 만든 적이 없어요"),
        span,
    )
    .with_bilingual_hint(
        format!("write `set {name} to an empty list` first, then put things in it"),
        format!("먼저 `{name}은 빈 목록`이라고 적고 나서 그 안에 넣어 주세요"),
    )
}

// -------------------------------------------------------- a loop with no end

/// `repeat forever` / `계속 반복해`. `break` / `멈춰` is the way out.
fn match_forever(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(start) = forever_body_start(tokens, mode) else {
        return Ok(None);
    };
    let mut start = start;
    if tokens.get(start).is_some_and(is_connector_word) {
        start += 1;
    }
    let inline = parse_sentence_repeat_body(
        source,
        &tokens[start..],
        block,
        span_of(&tokens[..start]),
        known_names,
    )?;
    Ok(Some(NmeStmt::Forever { inline }))
}

/// Where the body of `repeat forever` / `계속 반복해` begins, or `None` when
/// the line is not one. Both words are required, in either order, which is
/// what keeps the ordinary word `forever` out of an ordinary sentence.
fn forever_body_start(tokens: &[Token], mode: MatchMode) -> Option<usize> {
    if let Some((_, consumed)) = repeat_action_at(tokens, 0, mode) {
        if let Some(used) = action_phrase_at(tokens, consumed, FOREVER_WORDS_EN, mode) {
            return Some(consumed + used);
        }
    }
    let used = action_phrase_at(tokens, 0, FOREVER_WORDS_KO, mode)?;
    let (_, consumed) = repeat_action_at(tokens, used, mode)?;
    Some(used + consumed)
}

// ------------------------------------------------- a chance, and a story

/// The `30%` that opens a chance, and how much of the line it takes.
///
/// The shape only: the number is checked where a diagnostic can be reported,
/// because a header check has no source text to quote and nowhere to put a
/// problem.
#[derive(Debug, Clone, Copy)]
struct ChancePrefix {
    /// Tokens consumed, so the caller knows where the body starts.
    consumed: usize,
    /// Index of the number token.
    number_at: usize,
    /// Whether a `-` stands in front of the number.
    negative: bool,
}

/// `30% 확률로 …` / `30% chance …` — the head of every chance form.
///
/// `None` unless the whole phrase is there: a number, a percent mark, and a
/// word that can only mean a chance. A percentage on its own is never a
/// chance, which is what keeps `전체의 30%가 왔습니다`, `I am 100% sure`, and
/// `100% 확신합니다` the ordinary lines they are.
fn chance_prefix(tokens: &[Token]) -> Option<ChancePrefix> {
    korean_chance_prefix(tokens).or_else(|| english_chance_prefix(tokens))
}

/// `[with] [a] 30[%|percent] chance` and `30% of the time`.
fn english_chance_prefix(tokens: &[Token]) -> Option<ChancePrefix> {
    // `with` is a Python keyword, so it never arrives as a plain word.
    let leads = matches!(tokens.first().map(|token| &token.tok), Some(Tok::With))
        || token_matches_exact_at(tokens, 0, CHANCE_LEAD_WORDS_EN);
    let mut cursor = usize::from(leads);
    if is_english_article(tokens.get(cursor)) {
        cursor += 1;
    }
    let number_at = cursor
        + usize::from(matches!(
            tokens.get(cursor).map(|t| &t.tok),
            Some(Tok::Minus)
        ));
    if !is_chance_number(tokens.get(number_at)) {
        return None;
    }
    cursor = number_at + 1;
    cursor += chance_percent_mark(tokens, cursor, CHANCE_PERCENT_WORDS_EN)?;
    if token_matches_exact_at(tokens, cursor, CHANCE_WORDS_EN) {
        cursor += 1;
    } else if token_matches_exact_at(tokens, cursor, &["of"]) {
        // `30% of the time` says the same thing without the noun.
        cursor += 1;
        if is_english_article(tokens.get(cursor)) {
            cursor += 1;
        }
        if !token_matches_exact_at(tokens, cursor, CHANCE_TIME_WORDS_EN) {
            return None;
        }
        cursor += 1;
    } else {
        return None;
    }
    Some(ChancePrefix {
        consumed: cursor,
        number_at,
        negative: number_at > 0 && matches!(tokens[number_at - 1].tok, Tok::Minus),
    })
}

/// `30[%|퍼센트][의] 확률로` and the word-first `확률 30%로`.
fn korean_chance_prefix(tokens: &[Token]) -> Option<ChancePrefix> {
    let leads = token_matches_exact_at(tokens, 0, CHANCE_WORDS_KO);
    let start = usize::from(leads);
    let number_at = start
        + usize::from(matches!(
            tokens.get(start).map(|t| &t.tok),
            Some(Tok::Minus)
        ));
    if !is_chance_number(tokens.get(number_at)) {
        return None;
    }
    let mut cursor = number_at + 1;
    cursor += chance_percent_mark(tokens, cursor, CHANCE_PERCENT_WORDS_KO)?;
    if leads {
        // `확률 30%로` — the particle is what makes this a command. Without
        // it, `확률 30%는 낮습니다` and `확률 30% 정도입니다` are remarks about
        // a percentage, and a remark must never become a program.
        if !token_matches_exact_at(tokens, cursor, CHANCE_PARTICLES_KO) {
            return None;
        }
        cursor += 1;
    } else {
        // `30%의 확률로` — the particle sits between the two halves.
        if token_matches_exact_at(tokens, cursor, &["의"]) {
            cursor += 1;
        }
        if !token_matches_exact_at(tokens, cursor, CHANCE_WORDS_KO) {
            return None;
        }
        cursor += 1;
    }
    Some(ChancePrefix {
        consumed: cursor,
        number_at,
        negative: number_at > start,
    })
}

fn is_chance_number(token: Option<&Token>) -> bool {
    token.is_some_and(|token| matches!(token.tok, Tok::Int { .. } | Tok::Float { .. }))
}

/// The `%` between the number and the chance word, written as the mark or as
/// a word. Returns how many tokens it took.
fn chance_percent_mark(tokens: &[Token], at: usize, words: &[&str]) -> Option<usize> {
    if matches!(tokens.get(at).map(|token| &token.tok), Some(Tok::Percent)) {
        return Some(1);
    }
    token_matches_exact_at(tokens, at, words).then_some(1)
}

/// How often the chance happens, in thousandths.
///
/// A percentage may name one decimal place and nothing finer, and it has to
/// sit between 0 and 100. Both are reported rather than repaired: rounding
/// `30.25%` to `30.3%` would make the program mean something its writer did
/// not write, and that is the one thing this compiler never does.
fn chance_permille(
    source: &str,
    tokens: &[Token],
    prefix: &ChancePrefix,
) -> Result<u32, Diagnostic> {
    let token = &tokens[prefix.number_at];
    let written = &source[token.span.start..token.span.end];
    let span = Span::new(
        tokens[prefix.number_at - usize::from(prefix.negative)]
            .span
            .start,
        token.span.end,
    );
    let (whole, fraction) = written.split_once('.').unwrap_or((written, ""));
    if !whole.chars().all(|digit| digit.is_ascii_digit())
        || !fraction.chars().all(|digit| digit.is_ascii_digit())
    {
        // `1_000`, `0x40`, `1e3`: not a percentage anybody wrote by hand.
        return Err(chance_out_of_range_diagnostic(span));
    }
    if fraction.chars().count() > 1 {
        return Err(chance_too_precise_diagnostic(whole, fraction, span));
    }
    let tenths = fraction
        .chars()
        .next()
        .and_then(|digit| digit.to_digit(10))
        .unwrap_or(0);
    let whole_value = if whole.is_empty() {
        Some(0)
    } else {
        whole.parse::<u32>().ok()
    };
    let permille = whole_value
        .filter(|value| *value <= 100)
        .map(|value| value * 10 + tenths)
        .filter(|permille| *permille <= CHANCE_MAX_PERMILLE)
        .filter(|permille| !prefix.negative || *permille == 0);
    permille.ok_or_else(|| chance_out_of_range_diagnostic(span))
}

fn chance_too_precise_diagnostic(whole: &str, fraction: &str, span: Span) -> Diagnostic {
    let written = format!("{whole}.{fraction}");
    let rounded = nearest_tenth(whole, fraction);
    Diagnostic::bilingual(
        DiagnosticCode::ChanceTooPrecise,
        "a chance can only go to one decimal place",
        "확률은 소수점 첫째 자리까지만 정할 수 있어요",
        span,
    )
    .with_bilingual_hint(
        format!("write {rounded}% instead of {written}%"),
        format!("{written}% 대신 {rounded}%처럼 적어 주세요"),
    )
}

/// The nearest tenth of the number as written, so the hint can name a
/// percentage the writer can actually use. Digits, not floating point: the
/// answer has to be the one a reader gets on paper.
fn nearest_tenth(whole: &str, fraction: &str) -> String {
    let mut digits = fraction.chars().filter_map(|digit| digit.to_digit(10));
    let tenths = u64::from(digits.next().unwrap_or(0));
    let rounds_up = digits.next().is_some_and(|digit| digit >= 5);
    let scaled = whole
        .parse::<u64>()
        .unwrap_or(0)
        .saturating_mul(10)
        .saturating_add(tenths)
        .saturating_add(u64::from(rounds_up));
    format!("{}.{}", scaled / 10, scaled % 10)
}

fn chance_out_of_range_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ChanceOutOfRange,
        "a chance must be between 0% and 100%",
        "확률은 0%부터 100% 사이여야 해요",
        span,
    )
    .with_bilingual_hint(
        "`0%` never happens and `100%` always happens",
        "`0%`는 절대 일어나지 않고 `100%`는 항상 일어나요",
    )
}

/// `30% 확률로 말해줘 당첨` / `30% chance show You win`, and the block that
/// opens with the same words and nothing after them.
fn match_chance(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(prefix) = chance_prefix(tokens) else {
        return Ok(None);
    };
    let body = &tokens[prefix.consumed..];
    let inline = match parse_suite_body(
        source,
        body,
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    ) {
        Ok(inline) => inline,
        // A header with nothing under it is a real mistake and is reported.
        // A body of ordinary words is not a mistake at all: `a 30% chance is
        // small` is a sentence about a chance, so the line goes back to the
        // matchers that print it.
        Err(problem) if body.is_empty() || has_recoverable_sentence_shape(body, known_names) => {
            return Err(problem)
        }
        Err(_) => return Ok(None),
    };
    if !chance_body_is_a_command(source, body, inline.as_ref()) {
        return Ok(None);
    }
    // The number is only worth complaining about once the line is certainly
    // a chance; `a 33.33% chance of winning` is a sentence, not a mistake.
    let permille = chance_permille(source, tokens, &prefix)?;
    Ok(Some(NmeStmt::Chance { permille, inline }))
}

/// True when what follows a chance is a command rather than more words.
///
/// `a 30% chance of rain` is a weather report and `확률 30%로 계산했습니다` is
/// a remark; both would otherwise print their own tail three times in ten.
/// The test is exact: the parser prints any line of ordinary words, so a
/// printed body that still reads as the whole body was never a command.
fn chance_body_is_a_command(source: &str, body: &[Token], inline: Option<&InlineStmt>) -> bool {
    match inline {
        None => true,
        Some(InlineStmt::Nme(stmt)) => !is_bare_prose_say(stmt, source, body),
        // Python that only names something does nothing: `a 20% chance
        // remains` would compile to `if …: remains`, which is a NameError
        // waiting on a dice roll.
        Some(InlineStmt::Python(_)) => python_body_acts(body),
    }
}

/// True when this statement is the body printed back word for word, which is
/// what the parser does with any line it finds no action in.
fn is_bare_prose_say(stmt: &NmeStmt, source: &str, body: &[Token]) -> bool {
    let NmeStmt::Say {
        value: Value::Text(template),
    } = stmt
    else {
        return false;
    };
    template_source_text(template) == token_text(source, body)
}

/// The words a text template was built from, joined back together. A
/// template is made of the source between its tokens, so this is the source
/// again whenever nothing was dropped.
fn template_source_text(template: &TextTemplate) -> String {
    template
        .parts
        .iter()
        .map(|part| match part {
            TextPart::Literal(text) => text.as_str(),
            TextPart::Variable(name) => name.as_str(),
        })
        .collect()
}

/// True when a line of Python does something: it calls, it assigns, or it is
/// one of the statement keywords. A bare expression (`remains`, `he is late`)
/// is a sentence somebody wrote, not a command.
fn python_body_acts(tokens: &[Token]) -> bool {
    tokens.iter().any(|token| {
        matches!(
            token.tok,
            Tok::Lpar
                | Tok::Equal
                | Tok::PlusEqual
                | Tok::MinusEqual
                | Tok::StarEqual
                | Tok::SlashEqual
                | Tok::DoubleSlashEqual
                | Tok::DoubleStarEqual
                | Tok::PercentEqual
                | Tok::Break
                | Tok::Continue
                | Tok::Pass
                | Tok::Return
                | Tok::Raise
                | Tok::Del
                | Tok::Assert
                | Tok::Import
                | Tok::From
        )
    })
}

/// `운은 30% 확률` / `luck is a 30% chance` — a chance saved in a name, so
/// the rest of the program can ask about it with the ordinary condition
/// words (`만약에 운이 있으면` / `if luck`).
fn match_chance_set(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some((target, value_at)) = chance_set_target(tokens, known_names) else {
        return Ok(None);
    };
    let value = &tokens[value_at..];
    let Some(prefix) = chance_prefix(value) else {
        return Ok(None);
    };
    if prefix.consumed != value.len() {
        return Ok(None);
    }
    let permille = chance_permille(source, value, &prefix)?;
    Ok(Some(NmeStmt::Set {
        target,
        value: Value::Chance { permille },
    }))
}

/// The name a chance is being saved into, and where the chance itself starts.
fn chance_set_target(tokens: &[Token], known_names: &HashSet<String>) -> Option<(String, usize)> {
    // `set luck to a 30% chance` / `luck save a 30% chance`.
    if let Some((_, consumed)) = set_action_at(tokens, 0, MatchMode::Exact) {
        let name_at = consumed;
        let target = name_word(tokens.get(name_at)?)?;
        let mut value_at = name_at + 1;
        if is_update_connector(tokens.get(value_at)?, UPDATE_CONNECTOR_WORDS_EN) {
            value_at += 1;
        }
        return update_target_name(target).map(|target| (target, value_at));
    }
    // `luck is a 30% chance`.
    if token_matches_exact_at(tokens, 1, CHANCE_IS_WORDS_EN) {
        let target = name_word(tokens.first()?)?;
        return update_target_name(target).map(|target| (target, 2));
    }
    // `운은 30% 확률` — the Korean particle is the only marker there is.
    let word = name_word(tokens.first()?)?;
    let target =
        resolve_known_particle(word, known_names).or_else(|| strip_assignment_particle(word))?;
    Some((target.to_string(), 1))
}

/// `이야기:` / `story:`, with the speed each line inside is told at.
///
/// The line must be the phrase and its colon and nothing else. Every other
/// line that mentions a story is an ordinary sentence and stays one.
fn match_story(source: &str, tokens: &[Token]) -> Option<NmeStmt> {
    let body = strip_closing_colon(tokens)?;
    if body.is_empty() {
        return None;
    }
    korean_story_speed(source, body)
        .or_else(|| english_story_speed(source, body))
        .map(|StorySpeed(seconds)| NmeStmt::Story { seconds })
}

/// Drops the closing `:`, in either width. A Korean keyboard writes the
/// full-width `：`, which Python cannot read at all and the lexer therefore
/// hands over as ordinary sentence text.
fn strip_closing_colon(tokens: &[Token]) -> Option<&[Token]> {
    let last = tokens.last()?;
    let closed = matches!(last.tok, Tok::Colon) || token_matches_exact(last, &[FULL_WIDTH_COLON]);
    closed.then(|| &tokens[..tokens.len() - 1])
}

/// True for a line shaped like a story header, which is all the block and
/// indentation checks need to know before the line is read properly.
fn story_colon_shape(tokens: &[Token]) -> bool {
    let Some(body) = strip_closing_colon(tokens) else {
        return false;
    };
    let Some(last) = body.last() else {
        return false;
    };
    token_matches_exact(last, STORY_WORDS_KO)
        || token_matches_exact(last, STORY_WORDS_EN)
        || (token_matches_exact(last, SECOND_WORDS_EN)
            && body
                .iter()
                .any(|token| token_matches_exact(token, STORY_WORDS_EN)))
}

/// `story:` · `slow story:` · `very slow story:` · `slow story every 3 seconds:`
fn english_story_speed(source: &str, tokens: &[Token]) -> Option<StorySpeed> {
    let mut cursor = usize::from(is_english_article(tokens.first()));
    let very = token_matches_exact_at(tokens, cursor, VERY_WORDS_EN);
    cursor += usize::from(very);
    let slow = token_matches_exact_at(tokens, cursor, STORY_SLOW_WORDS_EN);
    cursor += usize::from(slow);
    if very && !slow {
        return None;
    }
    if is_english_article(tokens.get(cursor)) {
        cursor += 1;
    }
    if !token_matches_exact_at(tokens, cursor, STORY_WORDS_EN) {
        return None;
    }
    cursor += 1;
    if cursor == tokens.len() {
        return Some(story_fixed_speed(slow, very));
    }
    // `slow story every 3 seconds:` names the pause itself.
    if !token_matches_exact_at(tokens, cursor, SLOW_EVERY_WORDS_EN) {
        return None;
    }
    let amount_start = cursor + 1;
    let unit = (amount_start..tokens.len())
        .find(|&index| token_matches_exact(&tokens[index], SECOND_WORDS_EN))?;
    if unit + 1 != tokens.len() {
        return None;
    }
    Some(StorySpeed(Some(parse_wait_amount(
        source,
        &tokens[amount_start..=unit],
    )?)))
}

/// `이야기:` · `천천히 이야기:` · `아주 천천히 이야기:` · `0.2초씩 천천히 이야기:`
fn korean_story_speed(source: &str, tokens: &[Token]) -> Option<StorySpeed> {
    let story_at = tokens.len() - 1;
    if !token_matches_exact(&tokens[story_at], STORY_WORDS_KO) {
        return None;
    }
    let speed = &tokens[..story_at];
    let Some((slow, head)) = speed.split_last() else {
        return Some(StorySpeed(None));
    };
    if !token_matches_exact(slow, STORY_SLOW_WORDS_KO) {
        return None;
    }
    if head.is_empty() {
        return Some(story_fixed_speed(true, false));
    }
    if head.len() == 1 && token_matches_exact(&head[0], VERY_WORDS_KO) {
        return Some(story_fixed_speed(true, true));
    }
    // `0.2초씩 천천히` — the amount is everything before the marker.
    let (marker, amount) = head.split_last()?;
    if !token_matches_exact(marker, SLOW_EVERY_WORDS_KO) || amount.is_empty() {
        return None;
    }
    Some(StorySpeed(Some(expression_code(source, amount)?)))
}

/// Indentation for a line the compiler writes itself inside a story: what
/// the source already has in front of `line`, plus the levels an explicit
/// block adds on top.
fn story_prefix(
    source: &str,
    line_starts: &[usize],
    line: &LogicalLine,
    virtual_indent: usize,
) -> String {
    let start = line_starts.get(line.number - 1).copied().unwrap_or(0);
    format!(
        "{}{}",
        &source[start..line.span.start],
        "    ".repeat(virtual_indent)
    )
}

/// The physical line a logical line ends on. Everything except a bracketed
/// Python expression ends on the line it started.
fn last_physical_line(source: &str, line: &LogicalLine) -> usize {
    line.number + source[line.span.start..line.span.end].matches('\n').count()
}

/// A blank line inside a story is an empty line of the story, so it prints
/// one. Blank lines hold no tokens and therefore never become logical lines,
/// so the replacement has to name its own place in the source. A line with a
/// comment on it is a comment, not an empty line, and is left alone.
fn push_story_blanks(
    out: &mut Vec<(Span, String)>,
    source: &str,
    line_starts: &[usize],
    prefix: &str,
    after: usize,
    before: usize,
) {
    for number in (after + 1)..before {
        let Some(start) = line_starts.get(number - 1).copied() else {
            return;
        };
        let end = line_starts
            .get(number)
            .map_or(source.len(), |next| next.saturating_sub(1));
        if end < start || !source[start..end].trim().is_empty() {
            continue;
        }
        out.push((Span::new(start, end), format!("{prefix}print()")));
    }
}

/// How fast one story is told: all at once, or one character at a time with
/// a named pause between them.
#[derive(Debug, Clone)]
struct StorySpeed(Option<Code>);

/// The pause between two characters when the story does not name one.
fn story_fixed_speed(slow: bool, very: bool) -> StorySpeed {
    StorySpeed(slow.then(|| {
        Code::Generated(
            if very {
                VERY_SLOW_SECONDS
            } else {
                SLOW_SECONDS
            }
            .to_string(),
        )
    }))
}

// ------------------------------------------- slow text, screen, and timing

/// `say slowly Hello` / `천천히 말해줘 안녕`.
///
/// The message is read exactly the way the ordinary output statement reads
/// it, so a name written inside the sentence is still substituted.
fn match_say_slowly(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let start = leading_sentence_fillers(tokens);
    let Some((seconds, value_start)) = slow_speed_at(source, tokens, start, mode) else {
        return Ok(None);
    };
    let body = &tokens[value_start..];
    if body.is_empty() {
        return Err(say_missing(Spelling::English, span_of(tokens)));
    }
    let value = parse_value(source, body, known_names, true)
        .map_err(|()| say_value_unparseable(span_of(body)))?;
    Ok(Some(NmeStmt::SaySlowly { value, seconds }))
}

/// How long to pause between two characters, and where the message starts.
///
/// English puts the speed after the output word (`say very slowly …`);
/// Korean puts it before it (`아주 천천히 말해줘 …`, `3초씩 천천히 말해줘 …`).
fn slow_speed_at(
    source: &str,
    tokens: &[Token],
    start: usize,
    mode: MatchMode,
) -> Option<(Code, usize)> {
    if let Some((_, consumed)) = output_action_at(tokens, start, mode) {
        let mut cursor = start + consumed;
        let very = tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, VERY_WORDS_EN));
        if very {
            cursor += 1;
        }
        if !tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, SLOW_WORDS_EN))
        {
            return None;
        }
        cursor += 1;
        if tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, SLOW_EVERY_WORDS_EN))
        {
            let amount_start = cursor + 1;
            let unit = (amount_start..tokens.len())
                .find(|&index| token_matches_exact(&tokens[index], SECOND_WORDS_EN))?;
            let seconds = parse_wait_amount(source, &tokens[amount_start..=unit])?;
            return Some((seconds, unit + 1));
        }
        let fixed = if very {
            VERY_SLOW_SECONDS
        } else {
            SLOW_SECONDS
        };
        return Some((Code::Generated(fixed.to_string()), cursor));
    }

    let mut cursor = start;
    let mut seconds = None;
    // `3초씩 천천히` — the amount is everything before the `초씩` marker.
    let interval_unit = (start + 1..tokens.len()).find(|&index| {
        token_matches_exact(&tokens[index], SLOW_EVERY_WORDS_KO)
            && tokens
                .get(index + 1)
                .is_some_and(|next| token_matches_exact(next, SLOW_WORDS_KO))
    });
    if let Some(unit) = interval_unit {
        seconds = Some(expression_code(source, &tokens[start..unit])?);
        cursor = unit + 1;
    } else if tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, VERY_WORDS_KO))
    {
        seconds = Some(Code::Generated(VERY_SLOW_SECONDS.to_string()));
        cursor += 1;
    }
    if !tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, SLOW_WORDS_KO))
    {
        return None;
    }
    cursor += 1;
    let consumed = action_phrase_at(tokens, cursor, SAY_WORDS_KO, mode)?;
    Some((
        seconds.unwrap_or_else(|| Code::Generated(SLOW_SECONDS.to_string())),
        cursor + consumed,
    ))
}

/// `clear the screen` / `화면 지워`.
/// `put hello on the screen` / `안녕하세요 화면에` — the message and then the
/// place it goes.
///
/// The screen is what makes the line a command. `put the kettle on` names no
/// screen and stays the sentence it is, and so does every other line in the
/// prose corpora, because none of them ends in `on the screen` or `화면에`.
///
/// Read before the output words so that `show hello on the screen` prints the
/// greeting rather than the whole phrase.
fn match_say_on_screen(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<NmeStmt> {
    let body = trim_command_endings(tokens);
    let mut end = body.len();
    // The verb is what makes this a command. Without it `words appeared on
    // the screen` and `휴대폰 화면에` are things somebody wrote.
    let mut has_a_verb = false;
    if end > 1 && token_matches_exact(&body[end - 1], SCREEN_VERB_WORDS_KO) {
        end -= 1;
        has_a_verb = true;
    }
    let screen_at = end.checked_sub(1)?;
    if token_matches_exact(&body[screen_at], SCREEN_TAIL_WORDS_KO) {
        end = screen_at;
    } else if token_matches_exact(&body[screen_at], SCREEN_TAIL_WORDS_EN) {
        end = screen_at;
        if end > 0 && is_english_article(body.get(end - 1)) {
            end -= 1;
        }
        // Without the connector this is a name, not a place: `the screen` on
        // its own is what a shop sells.
        if end > 0 && token_matches_exact(&body[end - 1], &["on", "onto", "to", "in", "up"]) {
            end -= 1;
        } else {
            return None;
        }
    } else {
        return None;
    }
    let mut start = leading_sentence_fillers(body);
    if start + 1 < end && token_matches_exact(&body[start], SCREEN_VERB_WORDS_EN) {
        start += 1;
        has_a_verb = true;
    }
    if start + 1 < end && body.get(start).is_some_and(is_show_request_pronoun) {
        start += 1;
    }
    if start >= end || !has_a_verb {
        return None;
    }
    let value = parse_value(source, &body[start..end], known_names, true).ok()?;
    Some(NmeStmt::Say { value })
}

fn match_clear_screen(tokens: &[Token], mode: MatchMode) -> Option<NmeStmt> {
    fixed_screen_sentence(
        tokens,
        mode,
        CLEAR_SCREEN_WORDS_EN,
        CLEAR_SCREEN_ACTIONS_EN,
        CLEAR_SCREEN_WORDS_KO,
        CLEAR_SCREEN_ACTIONS_KO,
    )
    .then_some(NmeStmt::ClearScreen)
}

/// `draw a line` / `줄 그어`.
fn match_draw_line(tokens: &[Token], mode: MatchMode) -> Option<NmeStmt> {
    fixed_screen_sentence(
        tokens,
        mode,
        DRAW_LINE_WORDS_EN,
        DRAW_LINE_ACTIONS_EN,
        DRAW_LINE_WORDS_KO,
        DRAW_LINE_ACTIONS_KO,
    )
    .then_some(NmeStmt::DrawLine)
}

/// A whole-line sentence with no value in it: an English verb and its object
/// (`clear the screen`), or a Korean subject and its verb (`화면 지워`).
///
/// Nothing else may be on the line, so a message that merely mentions the
/// same words (`화면 지워도 되는지 말해줘`) stays a message.
fn fixed_screen_sentence(
    tokens: &[Token],
    mode: MatchMode,
    english_verb: &[&str],
    english_object: &[&str],
    korean_subject: &[&str],
    korean_verb: &[&str],
) -> bool {
    let words = trim_command_endings(tokens);
    if let Some(consumed) = action_phrase_at(words, 0, english_verb, mode) {
        let cursor = consumed + usize::from(is_english_article(words.get(consumed)));
        return words.len() == cursor + 1 && token_matches_exact(&words[cursor], english_object);
    }
    words.len() == 2
        && token_matches_exact(&words[0], korean_subject)
        && token_matches_exact(&words[1], korean_verb)
}

/// `say in a box Hello` / `상자로 말해줘 안녕`, and the centred twin. Returns
/// the message; the caller decides which frame to draw around it.
fn framed_say_value(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
    english_frame: &[&str],
    korean_frame: &[&str],
) -> Result<Option<Value>, Diagnostic> {
    let start = leading_sentence_fillers(tokens);
    let value_start = if let Some((_, consumed)) = output_action_at(tokens, start, mode) {
        let mut cursor = start + consumed;
        if !tokens
            .get(cursor)
            .is_some_and(|token| matches!(token.tok, Tok::In))
        {
            return Ok(None);
        }
        cursor += 1;
        cursor += usize::from(is_english_article(tokens.get(cursor)));
        if !tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, english_frame))
        {
            return Ok(None);
        }
        cursor + 1
    } else if tokens
        .get(start)
        .is_some_and(|token| token_matches_exact(token, korean_frame))
    {
        let Some(consumed) = action_phrase_at(tokens, start + 1, SAY_WORDS_KO, mode) else {
            return Ok(None);
        };
        start + 1 + consumed
    } else {
        return Ok(None);
    };
    let body = &tokens[value_start..];
    if body.is_empty() {
        return Err(say_missing(Spelling::English, span_of(tokens)));
    }
    parse_value(source, body, known_names, true)
        .map(Some)
        .map_err(|()| say_value_unparseable(span_of(body)))
}

/// `say in a box Hello` / `상자로 말해줘 안녕`.
fn match_say_in_box(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    Ok(framed_say_value(
        source,
        tokens,
        known_names,
        mode,
        BOX_WORDS_EN,
        BOX_WORDS_KO,
    )?
    .map(|value| NmeStmt::SayInBox { value }))
}

/// `say in the middle Hello` / `가운데 말해줘 안녕`.
fn match_say_in_middle(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    Ok(framed_say_value(
        source,
        tokens,
        known_names,
        mode,
        MIDDLE_WORDS_EN,
        MIDDLE_WORDS_KO,
    )?
    .map(|value| NmeStmt::SayInMiddle { value }))
}

/// `start the timer` / `시간 재기 시작해`.
fn match_start_timer(tokens: &[Token], mode: MatchMode) -> Option<NmeStmt> {
    let words = trim_command_endings(tokens);
    if let Some(consumed) = action_phrase_at(words, 0, START_TIMER_WORDS_KO, mode) {
        return (consumed == words.len()).then_some(NmeStmt::StartTimer);
    }
    let consumed = action_phrase_at(words, 0, START_TIMER_WORDS_EN, mode)?;
    let cursor = consumed + usize::from(is_english_article(words.get(consumed)));
    (words.len() == cursor + 1 && token_matches_exact(&words[cursor], TIMER_WORDS_EN))
        .then_some(NmeStmt::StartTimer)
}

/// `put door on cooldown for 3 seconds` / `문 쿨타임 3초 걸어`.
fn match_cooldown(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let words = trim_command_endings(tokens);
    if let Some(consumed) = action_phrase_at(words, 0, COOLDOWN_SET_WORDS_EN, mode) {
        let Some(target) = words.get(consumed).and_then(name_word) else {
            return Ok(None);
        };
        let mut cursor = consumed + 1;
        if !words
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, &["on"]))
        {
            return Ok(None);
        }
        cursor += 1;
        if !words
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, COOLDOWN_WORDS_EN))
        {
            return Ok(None);
        }
        cursor += 1;
        let Some(seconds) = parse_wait_amount(source, &words[cursor..]) else {
            return Err(wait_amount_diagnostic(span_of(tokens)));
        };
        return Ok(Some(NmeStmt::Cooldown {
            target: cooldown_target_name(target, known_names),
            seconds,
        }));
    }

    // Korean puts the action last: `<이름> 쿨타임 <n>초 걸어`.
    let start_at = words.len().saturating_sub(2);
    let Some(action_start) = (start_at..words.len()).find(|&start| {
        action_phrase_at(words, start, COOLDOWN_SET_WORDS_KO, mode)
            .is_some_and(|used| start + used == words.len())
    }) else {
        return Ok(None);
    };
    // `<이름>`, `쿨타임`, and at least one word of amount have to come first;
    // without them this is an ordinary sentence that happens to end in `걸어`.
    if action_start < 3 || !token_matches_exact(&words[1], COOLDOWN_WORDS_KO) {
        return Ok(None);
    }
    let Some(target) = name_word(&words[0]) else {
        return Ok(None);
    };
    let Some(seconds) = parse_wait_amount(source, &words[2..action_start]) else {
        return Err(wait_amount_diagnostic(span_of(tokens)));
    };
    Ok(Some(NmeStmt::Cooldown {
        target: cooldown_target_name(target, known_names),
        seconds,
    }))
}

/// `wait for door` / `문 쿨타임 끝날때까지 기다려`.
fn match_cooldown_wait(
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Option<NmeStmt> {
    let words = trim_command_endings(tokens);
    if let Some(consumed) = action_phrase_at(words, 0, WAIT_WORDS_EN, mode) {
        if words.len() != consumed + 2 || !matches!(words[consumed].tok, Tok::For) {
            return None;
        }
        let target = cooldown_target_name(name_word(&words[consumed + 1])?, known_names);
        // `wait for pause_length` reads as a length of time when the program
        // already has a `pause_length`, so a name that is known but is not a
        // cooldown is left to the ordinary wait rules.
        let is_cooldown = known_names.contains(&format!("{COOLDOWN_PREFIX}{target}"));
        return (is_cooldown || !known_names.contains(&target))
            .then_some(NmeStmt::WaitForCooldown { target });
    }
    if words.len() < 4 {
        return None;
    }
    let action_start = words.len() - 1;
    if !token_matches_exact(&words[action_start], WAIT_WORDS_KO)
        || !token_matches_exact(&words[1], COOLDOWN_WORDS_KO)
    {
        return None;
    }
    let target = name_word(&words[0])?;
    let until = action_phrase_at(words, 2, COOLDOWN_UNTIL_WORDS_KO, mode)?;
    (2 + until == action_start).then(|| NmeStmt::WaitForCooldown {
        target: cooldown_target_name(target, known_names),
    })
}

/// `<name> is ready` / `<이름> 쿨타임이 끝났으면`, plus the index the inline
/// body starts at. Both spellings are ordinary conditions, so they work in
/// `when`, `while`, `else if`, and the one-line forms of all three.
fn cooldown_condition_at(tokens: &[Token], start: usize) -> Option<(Condition, usize)> {
    english_cooldown_condition_at(tokens, start)
        .or_else(|| korean_cooldown_condition_at(tokens, start))
}

fn english_cooldown_condition_at(tokens: &[Token], start: usize) -> Option<(Condition, usize)> {
    let target = name_word(tokens.get(start)?)?;
    if token_word(tokens.get(start + 1)?) != Some("is") {
        return None;
    }
    let (ready, mut body_start) = if tokens
        .get(start + 2)
        .is_some_and(|token| token_matches_exact(token, COOLDOWN_READY_WORDS_EN))
    {
        (true, start + 3)
    } else if tokens
        .get(start + 2)
        .is_some_and(|token| token_matches_exact(token, &["on"]))
        && tokens
            .get(start + 3)
            .is_some_and(|token| token_matches_exact(token, COOLDOWN_WORDS_EN))
    {
        (false, start + 4)
    } else {
        return None;
    };
    // `then` separates the condition from a one-line body, exactly as it
    // does after every other English condition.
    if tokens
        .get(body_start)
        .is_some_and(|token| token_word(token) == Some("then"))
    {
        body_start += 1;
    }
    Some((cooldown_condition(target, ready), body_start))
}

fn korean_cooldown_condition_at(tokens: &[Token], start: usize) -> Option<(Condition, usize)> {
    let target = name_word(tokens.get(start)?)?;
    if !token_matches_exact(tokens.get(start + 1)?, COOLDOWN_WORDS_KO) {
        return None;
    }
    let marker = tokens.get(start + 2)?;
    let ready = if token_matches_exact(marker, COOLDOWN_READY_WORDS_KO) {
        true
    } else if token_matches_exact(marker, COOLDOWN_BUSY_WORDS_KO) {
        false
    } else {
        return None;
    };
    Some((cooldown_condition(target, ready), start + 3))
}

/// The Python behind `is ready` and `is on cooldown`. It is written here
/// rather than taken from the source, because the source never spells it.
fn cooldown_condition(target: &str, ready: bool) -> Condition {
    let operator = if ready { ">=" } else { "<" };
    Condition::Truthy {
        value: ConditionValue::Python(Code::Generated(format!(
            "__import__(\"time\").time() {operator} {COOLDOWN_PREFIX}{target}"
        ))),
        negated: false,
    }
}

/// The NME name a cooldown belongs to. A Korean particle is only removed
/// when the program already knows the shorter name, exactly as everywhere
/// else, so a name that merely ends in a particle survives whole.
fn cooldown_target_name(word: &str, known_names: &HashSet<String>) -> String {
    resolve_known_particle(word, known_names)
        .unwrap_or(word)
        .to_string()
}

/// `elapsed` / `잰시간` standing alone as a value.
///
/// A name the program made itself always wins, so a program with its own
/// `elapsed` keeps it.
fn parse_elapsed_value(tokens: &[Token], known_names: &HashSet<String>) -> Option<Value> {
    (tokens.len() == 1 && is_elapsed_word(&tokens[0], known_names)).then_some(Value::Elapsed)
}

fn is_elapsed_word(token: &Token, known_names: &HashSet<String>) -> bool {
    name_word(token).is_some_and(|word| {
        !known_names.contains(word)
            && (ELAPSED_WORDS_EN.contains(&word) || ELAPSED_WORDS_KO.contains(&word))
    })
}

/// True when this statement reads the stopwatch, so the parser can say that
/// the timer was never started instead of leaving a `NameError` for later.
fn reads_elapsed(stmt: &NmeStmt) -> bool {
    match stmt {
        NmeStmt::Say { value }
        | NmeStmt::Set { value, .. }
        | NmeStmt::Append { value, .. }
        | NmeStmt::FileWrite { value, .. }
        | NmeStmt::SayInBox { value }
        | NmeStmt::SayInMiddle { value }
        | NmeStmt::SaySlowly { value, .. } => value_reads_elapsed(value),
        NmeStmt::Ask { prompt, .. } => prompt.as_ref().is_some_and(value_reads_elapsed),
        NmeStmt::When { condition, inline }
        | NmeStmt::While { condition, inline }
        | NmeStmt::ElseIf { condition, inline } => {
            condition_reads_elapsed(condition) || inline_reads_elapsed(inline.as_ref())
        }
        NmeStmt::Else { inline } => inline_reads_elapsed(inline.as_ref()),
        NmeStmt::Times { inline, .. } | NmeStmt::ForEach { inline, .. } => {
            inline_reads_elapsed(inline.as_ref())
        }
        _ => false,
    }
}

fn value_reads_elapsed(value: &Value) -> bool {
    match value {
        Value::Elapsed => true,
        Value::List(items) => items.iter().any(value_reads_elapsed),
        _ => false,
    }
}

fn condition_reads_elapsed(condition: &Condition) -> bool {
    match condition {
        Condition::Truthy { value, .. } => condition_value_reads_elapsed(value),
        Condition::Compare { left, right, .. } => {
            condition_value_reads_elapsed(left) || condition_value_reads_elapsed(right)
        }
        Condition::Logical { left, right, .. } => {
            condition_reads_elapsed(left) || condition_reads_elapsed(right)
        }
        Condition::Python(_) => false,
    }
}

fn condition_value_reads_elapsed(value: &ConditionValue) -> bool {
    matches!(value, ConditionValue::Python(Code::Generated(text)) if text == ELAPSED_PYTHON)
}

fn inline_reads_elapsed(inline: Option<&InlineStmt>) -> bool {
    matches!(inline, Some(InlineStmt::Nme(inner)) if reads_elapsed(inner))
}

fn timer_not_started_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::TimerNotStarted,
        "the timer has not been started yet",
        "시간 재기를 아직 시작하지 않았어요",
        span,
    )
    .with_bilingual_hint(
        "write `start the timer` on an earlier line",
        "앞 줄에 `시간 재기 시작해`라고 적어 주세요",
    )
}

/// The words of a line, with a trailing `?`, `!`, or `.` dropped.
fn trim_command_endings(tokens: &[Token]) -> &[Token] {
    let mut end = tokens.len();
    while end > 0 && is_command_ending(&tokens[end - 1]) {
        end -= 1;
    }
    &tokens[..end]
}

fn is_english_article(token: Option<&Token>) -> bool {
    token.is_some_and(|token| token_matches_exact(token, &["a", "an", "the"]))
}

fn say_value_unparseable(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::SayValueUnparseable,
        "I couldn't understand what to show",
        "무엇을 말할지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write a value, or a sentence such as `show Hello world`",
        "`안녕하세요 말해줘`처럼 평범한 문장으로 적어도 돼요",
    )
}

// -------------------------------------------------------- repeat over a list

/// `for each name in names` / `이름들의 이름마다 반복해`.
fn match_for_each(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some(stmt) = match_english_for_each(source, tokens, block, known_names, mode)? {
        return Ok(Some(stmt));
    }
    match_korean_for_each(source, tokens, block, known_names, mode)
}

/// Index of the loop variable in `for each <name> in <items>`, allowing an
/// optional leading repeat word.
///
/// `for` is a Python keyword only in lower case, so `For each` and `FOR EACH`
/// arrive as ordinary words; both spellings mean the same loop, exactly as
/// every other English action word folds case. `foreach` written as one word
/// is the same header again.
fn english_for_each_start(tokens: &[Token], mode: MatchMode) -> Option<usize> {
    let start = repeat_action_at(tokens, 0, mode).map_or(0, |(_, consumed)| consumed);
    let opener = tokens.get(start)?;
    if token_word_matches(opener, "foreach", mode) {
        return Some(start + 1);
    }
    if !matches!(opener.tok, Tok::For) && !token_matches_exact(opener, &["for"]) {
        return None;
    }
    word_matches_any(tokens.get(start + 1)?, EACH_WORDS_EN, mode).then_some(start + 2)
}

fn match_english_for_each(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(name_at) = english_for_each_start(tokens, mode) else {
        return Ok(None);
    };
    let Some(name) = tokens.get(name_at).and_then(name_word).map(str::to_string) else {
        return Err(for_each_diagnostic(span_of(tokens)));
    };
    if !tokens
        .get(name_at + 1)
        .is_some_and(|token| matches!(token.tok, Tok::In) || token_matches_exact(token, &["in"]))
    {
        return Err(for_each_diagnostic(span_of(tokens)));
    }
    let tail = &tokens[name_at + 2..];
    let colon_at = tail.iter().position(|token| {
        matches!(token.tok, Tok::Colon | Tok::And) || token_matches_exact(token, &["then"])
    });
    let items_tokens = colon_at.map_or(tail, |at| &tail[..at]);
    // `for each friend in friends with place` — the tail names a second name
    // to hold which turn the loop is on. It is looked for only inside a
    // header that is already a complete loop, so the ordinary word `with`
    // cannot reach this from an ordinary sentence.
    let (position, items_tokens) = match english_position_phrase(items_tokens) {
        Some((position, from)) => (Some(position), &items_tokens[..from]),
        None => (None, items_tokens),
    };
    let Some(items) = expression_code(source, items_tokens) else {
        return Err(for_each_diagnostic(span_of(tokens)));
    };
    let header_end = colon_at.map_or(span_of(tokens).end, |at| tail[at].span.end);
    let body = colon_at.map_or(&tail[tail.len()..], |at| &tail[at + 1..]);
    // The loop name is bound by the header, so the body may already use it.
    let mut body_names = known_names.clone();
    body_names.insert(name.clone());
    if let Some(position) = &position {
        body_names.insert(position.clone());
    }
    let inline = parse_suite_body(
        source,
        body,
        block,
        SuiteKind::Repeat,
        Span::new(tokens[0].span.start, header_end),
        &body_names,
    )?;
    Ok(Some(NmeStmt::ForEach {
        name,
        items,
        position,
        inline,
    }))
}

/// `… in friends with place` — the name that holds which turn the loop is on,
/// and where the collection stops.
///
/// The `with` has to be the last one on the line and has to be followed by a
/// plain name and nothing else, so `for each row in read_csv(path, with_head)`
/// keeps its whole expression.
fn english_position_phrase(items_tokens: &[Token]) -> Option<(String, usize)> {
    let at = items_tokens.iter().rposition(is_english_position_word)?;
    if at == 0 {
        return None;
    }
    let mut name_at = at + 1;
    while items_tokens
        .get(name_at)
        .is_some_and(|token| token_matches_exact(token, &["its", "the", "a"]))
    {
        name_at += 1;
    }
    if name_at + 1 != items_tokens.len() {
        return None;
    }
    let name = name_word(items_tokens.get(name_at)?)?;
    (is_plain_python_name(name) && is_bindable_english_name(name)).then(|| (name.to_string(), at))
}

/// Position of the loop variable and of the first token after it, for
/// `<목록>의 <이름>마다 ...`. The ending may be attached to the name or, as
/// people often type it, written as a word of its own: `친구 마다`.
fn korean_for_each_variable(tokens: &[Token]) -> Option<(usize, usize)> {
    if let Some(name_at) = tokens.iter().position(|token| {
        name_word(token).is_some_and(|word| {
            word.strip_suffix(EACH_SUFFIX_KO)
                .is_some_and(|base| !base.is_empty())
        })
    }) {
        return Some((name_at, name_at + 1));
    }
    let suffix_at = tokens
        .iter()
        .position(|token| token_is_exact_name(token, EACH_SUFFIX_KO))?;
    (suffix_at > 0 && name_word(&tokens[suffix_at - 1]).is_some())
        .then_some((suffix_at - 1, suffix_at + 1))
}

/// True when the line looks like `<목록>의 <이름>마다 ...`.
fn korean_for_each_shape(tokens: &[Token]) -> bool {
    let Some((name_at, rest_at)) = korean_for_each_variable(tokens) else {
        return false;
    };
    let rest = &tokens[rest_at..];
    // `순서와 함께` stands between the loop name and the repeat word, so it has
    // to be stepped over here as well; otherwise the line is not seen as a
    // block header and its body is asked to indent.
    let rest = match korean_position_phrase(rest) {
        Some((_, used)) => &rest[used..],
        None => rest,
    };
    name_at > 0
        && (repeat_action_at(rest, 0, MatchMode::Recover).is_some()
            || rest.iter().any(|token| matches!(token.tok, Tok::Colon)))
}

fn match_korean_for_each(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // The loop variable is the word ending in `마다`; everything before it is
    // the collection.
    let Some((name_at, rest_at)) = korean_for_each_variable(tokens) else {
        return Ok(None);
    };
    if name_at == 0 {
        return Ok(None);
    }
    let rest = &tokens[rest_at..];
    // `친구들의 친구마다 순서와 함께 반복해` — the name in front of `와 함께`
    // holds which turn the loop is on, and the loop goes on from there.
    let (position, rest) = match korean_position_phrase(rest) {
        Some((position, used)) => (Some(position), &rest[used..]),
        None => (None, rest),
    };
    let colon_at = rest
        .iter()
        .position(|token| matches!(token.tok, Tok::Colon));
    let repeat_consumed = repeat_action_at(rest, 0, mode).map(|(_, consumed)| consumed);
    // Without a closing repeat word or a colon this is ordinary speech.
    let Some(body_at) = repeat_consumed.or_else(|| colon_at.map(|at| at + 1)) else {
        return Ok(None);
    };
    let name = name_word(&tokens[name_at])
        .map(|word| word.strip_suffix(EACH_SUFFIX_KO).unwrap_or(word))
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .ok_or_else(|| for_each_diagnostic(span_of(tokens)))?;
    let items_tokens = &tokens[..name_at];
    // `친구들의` is a valid Python name on its own, so the particle has to be
    // taken off first or the loop would read over a name nobody defined.
    let items = strip_attached_particle_span(source, items_tokens, EACH_CONTAINER_PARTICLES_KO)
        .filter(|span| is_valid_python_expression(&source[span.start..span.end]))
        .map(Code::Source)
        .or_else(|| expression_code(source, items_tokens))
        .ok_or_else(|| for_each_diagnostic(span_of(tokens)))?;
    let header_end = colon_at.map_or(tokens[name_at].span.end, |at| rest[at].span.end);
    let mut body_names = known_names.clone();
    body_names.insert(name.clone());
    if let Some(position) = &position {
        body_names.insert(position.clone());
    }
    let inline = parse_suite_body(
        source,
        &rest[body_at.min(rest.len())..],
        block,
        SuiteKind::Repeat,
        Span::new(tokens[0].span.start, header_end),
        &body_names,
    )?;
    Ok(Some(NmeStmt::ForEach {
        name,
        items,
        position,
        inline,
    }))
}

/// `순서와 함께` — the name that holds which turn the loop is on, and how many
/// tokens it took. `함께`/`같이` is the anchor: without it a name carrying
/// `와`/`과` is just part of the sentence.
fn korean_position_phrase(rest: &[Token]) -> Option<(String, usize)> {
    if !token_matches_exact(rest.get(1)?, POSITION_WORDS_KO) {
        return None;
    }
    let word = name_word(rest.first()?)?;
    let name = strip_any_suffix(word, &["와", "과"])?;
    (!name.is_empty() && is_plain_python_name(name)).then(|| (name.to_string(), 2))
}

/// `with`, which Python spells as a keyword of its own, so it never arrives as
/// an ordinary word the way `each` and `in` do.
fn is_english_position_word(token: &Token) -> bool {
    matches!(token.tok, Tok::With) || token_matches_exact(token, POSITION_WORDS_EN)
}

/// True when a word can stand as a Python name on its own: it starts with a
/// letter or an underscore and holds nothing else. Hangul counts as letters,
/// which is what lets `순서` be the name that holds the position.
fn is_plain_python_name(word: &str) -> bool {
    let mut characters = word.chars();
    characters
        .next()
        .is_some_and(|first| first == '_' || first.is_alphabetic())
        && characters.all(|character| character == '_' || character.is_alphanumeric())
}

fn expression_code(source: &str, tokens: &[Token]) -> Option<Code> {
    if tokens.is_empty() {
        return None;
    }
    let span = span_of(tokens);
    is_valid_python_expression(&source[span.start..span.end]).then_some(Code::Source(span))
}

fn for_each_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ForEachUnparseable,
        "I couldn't understand this repeat-over-a-list line",
        "이 목록 반복 줄을 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `for each name in names` or `이름들의 이름마다 반복해`",
        "`이름들의 이름마다 반복해` 또는 `for each name in names`처럼 적어 주세요",
    )
}

#[allow(clippy::too_many_lines)]
fn match_while(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // Spoken Korean often puts the loop ending on the subject: `준비하는
    // 동안` may be tokenized as the single name `준비하는동안`. Split only
    // these documented endings; Python-valid names still win before this
    // matcher is reached.
    if let Some(subject) = tokens.first().and_then(split_attached_while_token) {
        let condition = parse_natural_condition(
            source,
            std::slice::from_ref(&subject),
            None,
            known_names,
            Spelling::Korean,
        )?;
        let inline = parse_suite_body(
            source,
            &tokens[1..],
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::While { condition, inline }));
    }
    if let Some((condition_tokens, body_start)) = korean_while_connector(tokens) {
        if let Ok(condition) = parse_natural_condition(
            source,
            &condition_tokens,
            None,
            known_names,
            Spelling::Korean,
        ) {
            let inline = parse_suite_body(
                source,
                &tokens[body_start..],
                block,
                SuiteKind::Condition,
                span_of(tokens),
                known_names,
            )?;
            return Ok(Some(NmeStmt::While { condition, inline }));
        }
    }
    let (spelling, condition_start, condition_end, trailing_while) =
        if matches!(tokens.first().map(|token| &token.tok), Some(Tok::While))
            || action_phrase_at(tokens, 0, WHILE_WORDS_EN, mode).is_some()
        {
            let consumed = if matches!(tokens.first().map(|token| &token.tok), Some(Tok::While)) {
                1
            } else {
                action_phrase_at(tokens, 0, WHILE_WORDS_EN, mode).expect("checked above")
            };
            // `while 준비 동안 성공 말해줘` and `while 점수가 3보다 작을 동안`
            // mix the English keyword with a Korean while ending. Split the
            // ending exactly like the Korean spellings so it cannot be
            // lowered as the loop's inline body.
            if let Some((condition_tokens, body_rel)) = korean_while_connector(&tokens[consumed..])
            {
                if let Ok(condition) = parse_natural_condition(
                    source,
                    &condition_tokens,
                    None,
                    known_names,
                    Spelling::Korean,
                ) {
                    let inline = parse_suite_body(
                        source,
                        &tokens[consumed + body_rel..],
                        block,
                        SuiteKind::Condition,
                        span_of(tokens),
                        known_names,
                    )?;
                    return Ok(Some(NmeStmt::While { condition, inline }));
                }
            }
            // A Korean while ending may also close a comparison condition
            // after the English keyword: `while 점수가 3보다 작을 동안`.
            let trailing = tokens
                .last()
                .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO))
                && !output_word_before(&tokens[consumed..], tokens.len() - 1 - consumed);
            if trailing && tokens.len() > consumed + 1 {
                (Spelling::English, consumed, tokens.len() - 1, true)
            } else {
                (Spelling::English, consumed, tokens.len(), false)
            }
        } else if action_phrase_at(tokens, 0, WHILE_WORDS_KO, mode).is_some() {
            let consumed =
                action_phrase_at(tokens, 0, WHILE_WORDS_KO, mode).expect("checked above");
            (Spelling::Korean, consumed, tokens.len(), false)
        } else if tokens.len() > 1
            && tokens
                .last()
                .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO))
            && !output_word_before(tokens, tokens.len() - 1)
        {
            (Spelling::Korean, 0, tokens.len() - 1, true)
        } else {
            return Ok(None);
        };

    let condition_slice = &tokens[condition_start..condition_end];
    if condition_slice.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }

    if !trailing_while {
        if let Some(colon_at) = find_condition_colon(source, tokens, condition_start) {
            if colon_at == condition_start {
                return Err(condition_missing(spelling, tokens[colon_at].span));
            }
            let condition_span = Span::new(
                tokens[condition_start].span.start,
                tokens[colon_at - 1].span.end,
            );
            if !is_valid_python_expression(&source[condition_span.start..condition_span.end]) {
                return Err(condition_invalid(spelling, condition_span));
            }
            let inline = parse_suite_body(
                source,
                &tokens[colon_at + 1..],
                block,
                SuiteKind::Condition,
                Span::new(tokens[0].span.start, tokens[colon_at].span.end),
                known_names,
            )?;
            return Ok(Some(NmeStmt::While {
                condition: Condition::Python(Code::Source(condition_span)),
                inline,
            }));
        }
    }

    if !trailing_while {
        if let Some((condition, body_start)) = cooldown_condition_at(tokens, condition_start) {
            let inline = parse_suite_body(
                source,
                &tokens[body_start..],
                block,
                SuiteKind::Condition,
                span_of(tokens),
                known_names,
            )?;
            return Ok(Some(NmeStmt::While { condition, inline }));
        }
    }
    let (condition_tokens, body_start, connector) = if trailing_while {
        if let Some((relative_at, connector)) = find_condition_connector(condition_slice) {
            let (condition, _, connector) =
                condition_tokens_before(tokens, condition_start, relative_at, connector);
            (condition, tokens.len(), Some(connector))
        } else {
            (condition_slice.to_vec(), tokens.len(), None)
        }
    } else if let Some((relative_at, connector)) = find_condition_connector(condition_slice) {
        let (condition, body_start, connector) =
            condition_tokens_before(tokens, condition_start, relative_at, connector);
        (condition, body_start, Some(connector))
    } else {
        (tokens[condition_start..].to_vec(), tokens.len(), None)
    };
    if condition_tokens.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }
    let condition =
        parse_natural_condition(source, &condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        &tokens[body_start..],
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::While { condition, inline }))
}

/// `아니면:` / `else if score == 0:` — a branch of an NME conditional whose
/// body is written with ordinary indentation instead of `끝`/`end`.
///
/// The colon form is the beginner spelling, and English gets its half free
/// because Python already spells `else:` and `elif …:` that way. Korean does
/// not, so `아니면:` had nothing to attach to and the whole program was
/// refused. Only a line at exactly the header's indentation reaches here.
fn match_colon_branch(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(colon_at) = tokens
        .iter()
        .position(|token| matches!(token.tok, Tok::Colon))
    else {
        return Ok(None);
    };
    if colon_at + 1 != tokens.len() {
        return Ok(None);
    }
    let head = &tokens[..colon_at];
    match branch_shape(head) {
        Some(BranchShape::Else) => {
            let closes = action_phrase_at(head, 0, ELSE_WORDS_EN, MatchMode::Exact)
                .or_else(|| action_phrase_at(head, 0, ELSE_WORDS_KO, MatchMode::Exact));
            if closes != Some(head.len()) {
                return Ok(None);
            }
            Ok(Some(NmeStmt::Else { inline: None }))
        }
        Some(BranchShape::ElseIf) => {
            match_branch(source, head, block, known_names, MatchMode::Exact)
        }
        None => Ok(None),
    }
}

fn match_branch(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // A colon-bearing `else:`/`elif ...:` is ordinary Python.  The easy
    // branch spelling deliberately omits the colon and closes with `end`.
    if tokens.iter().any(|token| matches!(token.tok, Tok::Colon)) {
        return Ok(None);
    }
    let Some(shape) = branch_shape(tokens) else {
        return Ok(None);
    };
    let (consumed, spelling) = if matches!(shape, BranchShape::ElseIf) {
        if matches!(tokens.first().map(|token| &token.tok), Some(Tok::Elif))
            || token_matches_exact(&tokens[0], &["elif"])
        {
            (1, Spelling::English)
        } else if let Some(consumed) = action_phrase_at(tokens, 0, ELSE_WORDS_EN, mode) {
            (
                consumed + when_action_at(tokens, consumed, mode).map_or(0, |(_, used)| used),
                Spelling::English,
            )
        } else if let Some(consumed) = action_phrase_at(tokens, 0, ELSE_WORDS_KO, mode) {
            (
                consumed + when_action_at(tokens, consumed, mode).map_or(0, |(_, used)| used),
                Spelling::Korean,
            )
        } else {
            return Ok(None);
        }
    } else if let Some(consumed) = action_phrase_at(tokens, 0, ELSE_WORDS_EN, mode) {
        (consumed, Spelling::English)
    } else if let Some(consumed) = action_phrase_at(tokens, 0, ELSE_WORDS_KO, mode) {
        (consumed, Spelling::Korean)
    } else {
        return Ok(None);
    };

    if matches!(shape, BranchShape::Else) {
        let body = &tokens[consumed..];
        let inline = parse_suite_body(
            source,
            body,
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Else { inline }));
    }

    if consumed >= tokens.len() {
        return Err(condition_missing(spelling, tokens[0].span));
    }
    let condition_start = consumed;
    if let Some(colon_at) = find_condition_colon(source, tokens, condition_start) {
        if colon_at == condition_start {
            return Err(condition_missing(spelling, tokens[colon_at].span));
        }
        let condition_span = Span::new(
            tokens[condition_start].span.start,
            tokens[colon_at - 1].span.end,
        );
        if !is_valid_python_expression(&source[condition_span.start..condition_span.end]) {
            return Err(condition_invalid(spelling, condition_span));
        }
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Condition,
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::ElseIf {
            condition: Condition::Python(Code::Source(condition_span)),
            inline,
        }));
    }
    if let Some((condition, body_start)) = cooldown_condition_at(tokens, condition_start) {
        let inline = parse_suite_body(
            source,
            &tokens[body_start..],
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::ElseIf { condition, inline }));
    }
    let remainder = &tokens[condition_start..];
    let (condition_tokens, body_start, connector) = match find_condition_connector(remainder) {
        Some((relative_at, connector)) => {
            let (condition, body_start, connector) =
                condition_tokens_before(tokens, condition_start, relative_at, connector);
            (condition, body_start, Some(connector))
        }
        None => (remainder.to_vec(), tokens.len(), None),
    };
    let condition =
        parse_natural_condition(source, &condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        &tokens[body_start..],
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::ElseIf { condition, inline }))
}

// -------------------------------------------------------------- condition

/// Match a conversational condition whose subject comes first, for example
/// `name exists then show hello` or `색이 빨강과 같으면 말해 yes`.
///
/// The explicit `if`/`만약` forms remain the clearest spelling, but accepting
/// the subject-first form is important for learners who are writing a spoken
/// sentence rather than translating Python word-for-word.  A bare `then`
/// sentence is only claimed when its body has an unmistakable action; this
/// keeps ordinary prose such as `Hello then world` as prose.
fn match_subject_when(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // No `만약` opens this line, so the only evidence of a condition is a
    // connector in the middle of it — and `-면` is also how an ordinary
    // Korean sentence says *when* or *in order to*. `3층에서 내리면 됩니다`
    // became `if (3 == "층에서 내리")`, cut inside a word.
    //
    // A line that ends the way a written Korean sentence ends and carries no
    // action word at all keeps its own meaning. One action word is enough to
    // make it a command again: `색이 빨강과 같으면 말해 맞아요` still compares,
    // and the explicit `만약에 …` spelling was never in doubt.
    if korean_line_is_a_sentence(tokens, known_names)
        && !tokens
            .iter()
            .filter_map(name_word)
            .any(|word| is_action_word(word) || is_nme_condition_word(word))
    {
        return Ok(None);
    }
    // `상점선택을 물어봐 살까요? 사려면 …` — the line names what it is asking
    // for and then asks. `-면` inside the question is part of the question,
    // not a connector, and reading it as one produced a comparison against a
    // name that was never made.
    if find_ask_shape(tokens, MatchMode::Exact).is_some_and(|shape| {
        shape.spelling == Spelling::Korean && shape.target_at == 0 && shape.action_start == 1
    }) {
        return Ok(None);
    }
    // An object particle marks what a verb acts on, never the subject of a
    // comparison. A word wearing one that the program never made is not what
    // this line is about, and emitting it with the particle still attached is
    // a `NameError` on a line that looked like it worked.
    if tokens.first().and_then(name_word).is_some_and(|word| {
        is_hangul(word)
            && resolve_known_particle(word, known_names).is_none()
            && ["을", "를"]
                .iter()
                .any(|particle| word.strip_suffix(particle).is_some_and(|base| !base.is_empty()))
    }) {
        return Ok(None);
    }
    // Explicit starters and other high-confidence sentence actions own the
    // line. Without this guard, a normal `if ... then ...` or `3 times ...`
    // line could be re-read as a subject-first condition because it contains
    // a comparison word somewhere in its body.
    if when_action_at(tokens, 0, MatchMode::Recover).is_some()
        || repeat_action_at(tokens, 0, MatchMode::Recover).is_some()
        || attached_korean_times_sentence(source, tokens, known_names).is_some()
        || find_count_marker(tokens, MatchMode::Exact).is_some()
        || ask_action_at(tokens, 0, MatchMode::Recover).is_some()
        || output_action_at(tokens, 0, MatchMode::Recover).is_some()
        || set_action_at(tokens, 0, MatchMode::Recover).is_some()
        || action_phrase_at(tokens, 0, USE_WORDS_EN, MatchMode::Recover).is_some()
        || action_phrase_at(tokens, 0, USE_WORDS_KO, MatchMode::Recover).is_some()
        || action_phrase_at(tokens, 0, WHILE_WORDS_EN, MatchMode::Recover).is_some()
        || action_phrase_at(tokens, 0, WHILE_WORDS_KO, MatchMode::Recover).is_some()
        || matches!(tokens.first().map(|token| &token.tok), Some(Tok::While))
        || tokens
            .last()
            .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO))
    {
        return Ok(None);
    }
    // `문 쿨타임이 끝났으면 발사 말해줘` — the Korean cooldown condition also
    // works without an explicit `만약`. Only the Korean spelling is claimed
    // here: bare `door is ready` is a valid Python line and stays Python.
    if let Some((condition, body_start)) = korean_cooldown_condition_at(tokens, 0) {
        let inline = parse_suite_body(
            source,
            &tokens[body_start..],
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::When { condition, inline }));
    }
    let Some((relative_at, connector)) = find_condition_connector(tokens) else {
        return Ok(None);
    };
    // `안녕 말해줘 아니면` — the connector closes the line and stands right
    // after the output word, so there is no condition here, only a message.
    if relative_at + 1 == tokens.len() && output_word_ends_just_before(tokens, relative_at) {
        return Ok(None);
    }
    if mode == MatchMode::Exact && find_exact_condition_connector(tokens).is_none() {
        return Ok(None);
    }
    let attached_subject = relative_at == 0
        && split_attached_condition_token(&tokens[0])
            .is_some_and(|(_, attached)| attached == connector);
    if relative_at == 0 && !attached_subject {
        return Ok(None);
    }
    if mode == MatchMode::Recover
        && find_exact_condition_connector(tokens).is_none()
        && (output_action_at(tokens, relative_at, MatchMode::Recover).is_some()
            || !recovered_condition_connector_is_plausible(&tokens[relative_at]))
    {
        return Ok(None);
    }
    let (condition_tokens, body_start, connector) =
        condition_tokens_before(tokens, 0, relative_at, connector);
    if condition_tokens.is_empty() {
        return Ok(None);
    }
    if matches!(connector, ConditionConnector::Then)
        && !subject_condition_body_is_action(&tokens[body_start..], mode, known_names)
    {
        return Ok(None);
    }
    let condition = parse_natural_condition(
        source,
        &condition_tokens,
        Some(connector),
        known_names,
        Spelling::Korean,
    )?;
    let inline = parse_suite_body(
        source,
        &tokens[body_start..],
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::When { condition, inline }))
}

fn recovered_condition_connector_is_plausible(token: &Token) -> bool {
    let Some(word) = token_word(token) else {
        return false;
    };
    word.is_ascii()
        || word
            .chars()
            .last()
            .is_some_and(|character| matches!(character, '면' | '먄'))
}

/// Whether a line *could* be a subject-first condition, asked while the
/// block structure is being worked out and before any name is known. The
/// names are therefore empty here: the only reading that needs them is the
/// one-syllable `말`, and a line ending in it is an inline body rather than a
/// block header.
fn subject_condition_shape(tokens: &[Token]) -> bool {
    let Some((relative_at, connector)) = find_condition_connector(tokens) else {
        return false;
    };
    let attached_subject = relative_at == 0
        && split_attached_condition_token(&tokens[0])
            .is_some_and(|(_, attached)| attached == connector);
    if relative_at == 0 && !attached_subject {
        return false;
    }
    let (_, body_start, _) = condition_tokens_before(tokens, 0, relative_at, connector);
    !matches!(connector, ConditionConnector::Then)
        || subject_condition_body_is_action(&tokens[body_start..], MatchMode::Exact, &HashSet::new())
}

fn subject_condition_body_is_action(
    tokens: &[Token],
    mode: MatchMode,
    known_names: &HashSet<String>,
) -> bool {
    if tokens.is_empty() {
        return false;
    }
    output_action_at(tokens, 0, mode).is_some()
        || output_action_ending(tokens, mode, known_names).is_some()
        || ask_action_at(tokens, 0, mode).is_some()
        || set_action_at(tokens, 0, mode).is_some()
        || update_action_at(tokens, 0, mode).is_some()
        || action_phrase_at(tokens, 0, BREAK_WORDS_EN, mode).is_some()
        || action_phrase_at(tokens, 0, BREAK_WORDS_KO, mode).is_some()
}

fn match_when(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some((spelling, consumed)) = when_action_at(tokens, 0, mode) else {
        return Ok(None);
    };
    // `혹시` is a politeness filler as well as a condition word. Only a real
    // comparison after it makes it a condition; on its own it is the `혹시`
    // of `혹시 안녕 말해줘`, which is a plain sentence.
    if token_word(&tokens[0]) == Some("혹시")
        && (tokens
            .iter()
            .enumerate()
            .any(|(index, _)| ask_action_at(tokens, index, MatchMode::Exact).is_some())
            || (find_condition_connector(&tokens[consumed..]).is_none()
                && find_condition_colon(source, tokens, consumed).is_none()))
    {
        return Ok(None);
    }
    let starter_exact = mode == MatchMode::Exact;
    if tokens.len() == consumed {
        return Err(condition_missing(spelling, tokens[0].span));
    }

    if let Some(colon_at) = find_condition_colon(source, tokens, consumed) {
        if colon_at == consumed {
            return Err(condition_missing(spelling, tokens[colon_at].span));
        }
        let condition_span = Span::new(tokens[consumed].span.start, tokens[colon_at - 1].span.end);
        if !is_valid_python_expression(&source[condition_span.start..condition_span.end]) {
            return Err(condition_invalid(spelling, condition_span));
        }
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Condition,
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::When {
            condition: Condition::Python(Code::Source(condition_span)),
            inline,
        }));
    }

    // A Korean cooldown condition ends in its own connector (`끝났으면`), so
    // it has to be read before the generic connector search cuts that word
    // off and leaves a meaningless comparison behind.
    if let Some((condition, body_start)) = cooldown_condition_at(tokens, consumed) {
        let inline = parse_suite_body(
            source,
            &tokens[body_start..],
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::When { condition, inline }));
    }

    let natural = find_condition_connector(&tokens[consumed..]);
    if !starter_exact && natural.is_none() && matches!(block, BlockCtx::Inline) {
        // A short sentence word may be one edit away from a condition alias.
        // Without a connector, colon, or following block there is not enough
        // evidence to recover it as a typo, so let another construct decide.
        return Ok(None);
    }
    let (condition_tokens, body_start, connector) = match natural {
        Some((relative_at, connector)) => {
            let (condition, body_start, connector) =
                condition_tokens_before(tokens, consumed, relative_at, connector);
            (condition, body_start, Some(connector))
        }
        None => (tokens[consumed..].to_vec(), tokens.len(), None),
    };
    if condition_tokens.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }
    let condition =
        parse_natural_condition(source, &condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        &tokens[body_start..],
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::When { condition, inline }))
}

fn when_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    if tokens
        .get(start)
        .is_some_and(|token| matches!(token.tok, Tok::If))
    {
        return Some((Spelling::English, 1));
    }
    action_phrase_at(tokens, start, WHEN_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, WHEN_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ConditionConnector {
    Then,
    Exists,
    Missing,
    Equals,
    NotEquals,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
}

/// The second half of a split `<=`/`>=`, in every ending Korean gives it.
///
/// The bare adnominals — `큰`, `작은`, `많은`, `다른`, `같은` — are deliberately
/// **not** comparison endings on their own. They are among the most ordinary
/// words Korean has, and listing them turned `더 큰 수예요 말해줘`,
/// `상자로 말해줘 작은 차림표` and `도전값과 다른 영지식 도전 만들기` into
/// comparisons. Here they are safe because the word in front of them
/// (`크거나`/`작거나`) can mean nothing else.
const OR_EQUAL_SECOND_WORDS_KO: &[&str] =
    &["같으면", "같다면", "같을", "같은", "같다", "같으니", "같으니까"];

fn find_exact_condition_connector(tokens: &[Token]) -> Option<(usize, ConditionConnector)> {
    if let Some(inner) = strip_outer_condition_parentheses(tokens) {
        return find_exact_condition_connector(inner)
            .map(|(index, connector)| (index + 1, connector));
    }
    // Spoken Korean splits `<=`/`>=` into two tokens (`10보다 작거나
    // 같으면`); the lone `같으면` would otherwise match equality.
    //
    // The second word changes shape with what comes after it: `같으면` before
    // a body, `같을`/`같은` before `동안`. Only the first was listed here, so
    // `점수가 10보다 크거나 같을 동안` reached the one-letter repair, which
    // read `같을` as `작을` and built the loop with the comparison **reversed**
    // — `while (점수 < 10)` for a line that says "while it is 10 or more".
    for (index, pair) in tokens.windows(2).enumerate() {
        let Some(second) = token_word(&pair[1]) else {
            continue;
        };
        if !OR_EQUAL_SECOND_WORDS_KO.contains(&second) {
            continue;
        }
        match token_word(&pair[0]) {
            Some("작거나") => return Some((index, ConditionConnector::LessOrEqual)),
            Some("크거나") => return Some((index, ConditionConnector::GreaterOrEqual)),
            _ => {}
        }
    }
    let last_operand = last_logical_operand_start(tokens);
    let exact = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if is_or_equal_phrase_at(tokens, index)
                || tokens.get(index.saturating_sub(1)).is_some_and(|previous| {
                    token_word(previous) == Some("or") && is_or_equal_phrase_at(tokens, index - 1)
                })
            {
                return None;
            }
            let connector = condition_connector_exact(token, index + 1 == tokens.len())?;
            // A `then` body marker may sit before the final `and`/`or`
            // operand because it separates the inline body. Korean
            // comparison endings may only close the final operand, so an
            // earlier comparison (`점수가 0보다 크면 그리고 ...`) stays
            // intact instead of being cut at its ending.
            (index >= last_operand || token_word(token) == Some("then"))
                .then_some((index, connector))
        })
        .collect::<Vec<_>>();
    if exact.is_empty() {
        // Spoken Korean splits the negation into two tokens: `같지 않으면`,
        // `같지 않다면`, or `같지 않을` at the end of a condition.
        for (index, pair) in tokens.windows(2).enumerate() {
            if index >= last_operand
                && token_word(&pair[0]) == Some("같지")
                && matches!(token_word(&pair[1]), Some("않으면" | "않다면" | "않을"))
            {
                return Some((index, ConditionConnector::NotEquals));
            }
        }
        // `점수가 0보다 큰 동안` — the plain adjective closes a comparison, but
        // only once `보다` has said one is being made. Without that marker
        // `큰` is the ordinary Korean word for "big", as in
        // `아주 큰 소리로 말해줘`, and stays part of the sentence.
        if let Some(marker_at) = tokens
            .iter()
            .position(|token| name_word(token).is_some_and(|word| word.ends_with("보다")))
        {
            if let Some(found) =
                tokens
                    .iter()
                    .enumerate()
                    .skip(marker_at + 1)
                    .find_map(|(index, token)| match token_word(token) {
                        Some("큰") => Some((index, ConditionConnector::Greater)),
                        Some("작은") => Some((index, ConditionConnector::Less)),
                        _ => None,
                    })
            {
                return Some(found);
            }
        }
    }
    exact
        .iter()
        .copied()
        .find(|(_, connector)| *connector == ConditionConnector::Then)
        .or_else(|| exact.first().copied())
}

/// First token index of the last `and`/`or` operand at bracket depth zero.
/// English `then` ends the condition scan: logical words after it belong to
/// the inline body (`if a then show x or y`).
fn last_logical_operand_start(tokens: &[Token]) -> usize {
    let mut last = 0usize;
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            _ => {}
        }
        if depth == 0 {
            if token_word(token) == Some("then") {
                break;
            }
            // `or equal` belongs to a `less/greater than or equal to`
            // comparison, not to logical `or`.
            if token_word(token) == Some("or") && is_or_equal_phrase_at(tokens, index) {
                continue;
            }
            if token_matches_exact(token, &["and", "or", "그리고", "또는"]) {
                last = index + 1;
            }
        }
    }
    last
}

/// True when `tokens[index]` is `or` followed by `equal`/`equals`, the
/// natural-language `<=`/`>=` phrase.
fn is_or_equal_phrase_at(tokens: &[Token], index: usize) -> bool {
    token_word(&tokens[index]) == Some("or")
        && tokens
            .get(index + 1)
            .is_some_and(|token| matches!(token_word(token), Some("equal" | "equals")))
}

fn find_condition_connector(tokens: &[Token]) -> Option<(usize, ConditionConnector)> {
    if let Some(inner) = strip_outer_condition_parentheses(tokens) {
        return find_condition_connector(inner).map(|(index, connector)| (index + 1, connector));
    }
    if let Some(connector) = find_exact_condition_connector(tokens) {
        return Some(connector);
    }

    // Only recover a connector typo when the whole condition has no exact
    // connector. Otherwise `than ... then` could split at `than`, because it
    // is one edit away from `then`.
    let last_operand = last_logical_operand_start(tokens);
    let recovered = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if is_or_equal_phrase_at(tokens, index)
                || tokens.get(index.saturating_sub(1)).is_some_and(|previous| {
                    token_word(previous) == Some("or") && is_or_equal_phrase_at(tokens, index - 1)
                })
            {
                return None;
            }
            let connector = condition_connector_recovered(token, index + 1 == tokens.len())?;
            (index >= last_operand || token_word(token) == Some("then"))
                .then_some((index, connector))
        })
        .collect::<Vec<_>>();
    (recovered.len() == 1).then(|| recovered[0])
}

/// Returns a condition token with a Korean connector suffix removed. Korean
/// writers commonly attach endings (`name이면`, `준비있으면`) to the preceding
/// name, while the Python tokenizer quite correctly keeps the whole word as
/// one identifier. The parser can split that one token without touching the
/// source bytes used for diagnostics or lowering.
fn split_attached_condition_token(token: &Token) -> Option<(Token, ConditionConnector)> {
    let word = name_word(token)?;
    // Leave a one-edit misspelling of a complete connector for the bounded
    // recovery path below. Otherwise the generic `면` suffix would turn
    // `잇으면` into the literal value `잇으` before recovery gets a chance.
    let full_connectors = [
        "그러면",
        "그럼",
        "하면",
        "이면",
        "이라면",
        "있으면",
        "있다면",
        "없으면",
        "없다면",
        "같으면",
        "같다면",
        "같지않으면",
        "같지않다면",
        "같지않을",
        "크면",
        "크다면",
        "작으면",
        "작다면",
        "비었으면",
        "비었다면",
        "비어있으면",
        "비어있다면",
    ];
    if full_connectors.iter().any(|candidate| {
        word != *candidate && word.chars().count() >= 2 && one_typo_away(word, candidate)
    }) {
        return None;
    }
    // Do not reinterpret a connector token itself as a value plus the short
    // `면` ending. For example, `같으면` must remain the equality connector;
    // otherwise its generic suffix would produce the bogus right-hand value
    // `같으`.
    if [
        "그러면",
        "그렇다면",
        "있으면",
        "없으면",
        "같으면",
        "같지않으면",
        "같지않다면",
        "같지않을",
        "크면",
        "작으면",
        "하면",
        "이면",
        "이라면",
        "라면",
        "면",
        "같먄",
        "있먄",
        "없먄",
        "크먄",
        "작먄",
        "라먄",
        "있으먄",
        "없으먄",
        "같으먄",
        "크으먄",
        "작으먄",
        "먄",
        // `초과면`/`미만이면` are whole comparison endings as well; split at
        // the generic `면` they would leave the meaningless value `초과`.
        "초과면",
        "초과이면",
        "미만이면",
        "미만면",
        // `비어있으면` ends in `있으면`; split there it would leave the
        // meaningless value `비어`.
        "비었으면",
        "비었다면",
        "비어있으면",
        "비어있다면",
    ]
    .contains(&word)
    {
        return None;
    }
    let (suffix, connector) = [
        ("그러면", ConditionConnector::Then),
        ("그렇다면", ConditionConnector::Then),
        ("있으면", ConditionConnector::Exists),
        ("없으면", ConditionConnector::Missing),
        ("같으면", ConditionConnector::Equals),
        ("크면", ConditionConnector::Greater),
        ("작으면", ConditionConnector::Less),
        ("있으먄", ConditionConnector::Exists),
        ("없으먄", ConditionConnector::Missing),
        ("같으먄", ConditionConnector::Equals),
        ("크으먄", ConditionConnector::Greater),
        ("작으먄", ConditionConnector::Less),
        // Korean speakers often attach the short comparison ending to the
        // right-hand value: `이름이 철수면` / `준비가 거짓이면` /
        // `이름이 철수라면`. Treat those forms as equality, while keeping
        // the bare words `면` and `라면` ordinary text.
        ("이라면", ConditionConnector::Then),
        ("라면", ConditionConnector::Equals),
        ("이면", ConditionConnector::Then),
        ("하면", ConditionConnector::Then),
        ("먄", ConditionConnector::Then),
        ("면", ConditionConnector::Equals),
    ]
    .into_iter()
    .find(|(suffix, _)| word.ends_with(suffix) && word.len() > suffix.len())?;
    let base_end = token.span.end.saturating_sub(suffix.len());
    let base = word.strip_suffix(suffix)?;
    Some((
        Token {
            tok: Tok::Name {
                name: base.to_string(),
            },
            span: Span::new(token.span.start, base_end),
        },
        connector,
    ))
}

fn split_attached_while_token(token: &Token) -> Option<Token> {
    let word = name_word(token)?;
    let suffix = ["하는동안", "할동안", "동안"]
        .into_iter()
        .find(|suffix| word.ends_with(suffix) && word.len() > suffix.len())?;
    let base = word.strip_suffix(suffix)?;
    let base_end = token.span.end.saturating_sub(suffix.len());
    Some(Token {
        tok: Tok::Name {
            name: base.to_string(),
        },
        span: Span::new(token.span.start, base_end),
    })
}

fn korean_while_connector(tokens: &[Token]) -> Option<(Vec<Token>, usize)> {
    // Prefer the last `동안` so a logical condition may carry an ending on
    // every operand: `점수가 5와 같지 않을 동안 그리고 점수가 0보다 클 동안`.
    // Earlier `동안` markers are loop endings too and only describe how the
    // operands are spoken, so they are dropped from the condition tokens. A
    // leading Korean while word is also dropped here; keeping it would make
    // an outer parenthesized condition start with the loop keyword instead
    // of its actual subject.
    let condition_start = usize::from(
        tokens
            .first()
            .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO)),
    );
    for (index, token) in tokens.iter().enumerate().skip(condition_start + 1).rev() {
        if !token_matches_exact(token, &["동안"]) {
            continue;
        }
        // Everything after an output word is the message, so a `동안` that
        // follows one is ordinary Korean, not the end of a loop condition.
        if output_word_before(tokens, index) {
            continue;
        }
        let mut condition = tokens[condition_start..index]
            .iter()
            .filter(|token| !token_matches_exact(token, &["동안"]))
            .cloned()
            .collect::<Vec<_>>();
        if condition
            .last()
            .is_some_and(|last| token_matches_exact(last, &["하는", "할"]))
        {
            condition.pop();
        } else if let Some(last) = condition.last_mut() {
            if let Some(base) = split_while_participle(last) {
                *last = base;
            }
        }
        if !condition.is_empty() {
            let mut body_start = index + 1;
            while tokens
                .get(body_start)
                .is_some_and(|token| matches!(token.tok, Tok::Rpar | Tok::Rsqb | Tok::Rbrace))
            {
                condition.push(tokens[body_start].clone());
                body_start += 1;
            }
            // A Korean comparison ending may appear before a logical
            // connector inside a whole wrapper. It is part of the condition,
            // not the loop boundary, so keep the remaining wrapped tokens
            // while dropping only the spoken `동안` markers.
            if tokens
                .get(body_start)
                .is_some_and(|token| token_matches_exact(token, &["and", "or", "그리고", "또는"]))
            {
                if let Some(wrapper_end) = condition_wrapper_end(tokens, condition_start) {
                    condition.extend(
                        tokens[body_start..=wrapper_end]
                            .iter()
                            .filter(|token| !token_matches_exact(token, &["동안"]))
                            .cloned(),
                    );
                    body_start = wrapper_end + 1;
                }
            }
            return Some((condition, body_start));
        }
    }
    None
}

fn split_while_participle(token: &Token) -> Option<Token> {
    let word = name_word(token)?;
    let suffix = ["하는", "할"]
        .into_iter()
        .find(|suffix| word.ends_with(suffix) && word.len() > suffix.len())?;
    let base = word.strip_suffix(suffix)?;
    let base_end = token.span.end.saturating_sub(suffix.len());
    Some(Token {
        tok: Tok::Name {
            name: base.to_string(),
        },
        span: Span::new(token.span.start, base_end),
    })
}

fn condition_tokens_before(
    tokens: &[Token],
    start: usize,
    relative_connector_at: usize,
    connector: ConditionConnector,
) -> (Vec<Token>, usize, ConditionConnector) {
    let at = start + relative_connector_at;
    let mut condition = tokens[start..at].to_vec();
    let mut body_start = at + 1;
    let mut connector = connector;
    // A split Korean negation spans two tokens (`같지 않으면`), so the body
    // starts after the second one rather than after the connector word.
    if connector == ConditionConnector::NotEquals
        && tokens
            .get(at)
            .is_some_and(|token| token_word(token) == Some("같지"))
        && tokens
            .get(at + 1)
            .is_some_and(|next| matches!(token_word(next), Some("않으면" | "않다면" | "않을")))
    {
        body_start = at + 2;
    }
    // `작거나 같으면` / `크거나 같으면` also spans two tokens.
    if matches!(
        connector,
        ConditionConnector::LessOrEqual | ConditionConnector::GreaterOrEqual
    ) && tokens
        .get(at + 1)
        .is_some_and(|next| token_word(next) == Some("같으면"))
    {
        body_start = at + 2;
    }
    if let Some(token) = tokens.get(at) {
        if let Some((base, attached_connector)) = split_attached_condition_token(token) {
            // `name이면` is a truthy condition when it is the whole subject,
            // but `ready가 거짓이면` is naturally an equality comparison.
            // The preceding condition tokens provide the disambiguating
            // context without making the lexer guess from raw source text.
            let context_equality = attached_connector == ConditionConnector::Then
                && !condition.is_empty()
                && name_word(token).is_some_and(|word| {
                    word.ends_with("이면") || word.ends_with("이라면") || word.ends_with("먄")
                });
            // A short ending attached directly to the only subject —
            // `준비면` / `준비라면` — is a truthy condition, not an equality
            // with a missing right-hand value. Equality needs a preceding
            // subject and a separate right-hand word, as in `이름이 철수면`.
            let subject_only_then =
                attached_connector == ConditionConnector::Equals && condition.is_empty();
            if context_equality {
                connector = ConditionConnector::Equals;
            }
            if subject_only_then {
                connector = ConditionConnector::Then;
            }
            if attached_connector == connector || context_equality || subject_only_then {
                condition.push(base);
                body_start = at + 1;
            }
        }
        // Spoken Korean may separate both the subject particle and the short
        // ending: `준비 가 거짓 이면` or `이름 이 철수 면`. A multi-token
        // condition is an equality comparison; a single subject keeps the
        // truthy/then meaning.
        //
        // A condition that already carries a comparison symbol
        // (`만약 점수 > 10 이면`) has said what it compares, so the ending is
        // only the sentence's own `이면` and the whole comparison stands.
        if token_matches_exact(token, &["이면", "이라면", "면", "먄"]) {
            connector = if condition.len() > 1 && !contains_comparison_symbol(&condition) {
                ConditionConnector::Equals
            } else {
                ConditionConnector::Then
            };
        }
    }
    while tokens
        .get(body_start)
        .is_some_and(|token| matches!(token.tok, Tok::Rpar | Tok::Rsqb | Tok::Rbrace))
    {
        condition.push(tokens[body_start].clone());
        body_start += 1;
    }
    (condition, body_start, connector)
}

/// True when a condition already compares with a written symbol, so a Korean
/// sentence ending after it adds nothing to the comparison.
fn contains_comparison_symbol(tokens: &[Token]) -> bool {
    tokens.iter().any(|token| {
        matches!(
            token.tok,
            Tok::Less
                | Tok::Greater
                | Tok::LessEqual
                | Tok::GreaterEqual
                | Tok::EqEqual
                | Tok::NotEqual
        )
    })
}

fn parse_natural_condition(
    source: &str,
    tokens: &[Token],
    connector: Option<ConditionConnector>,
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<Condition, Diagnostic> {
    if tokens.is_empty() {
        return Err(condition_missing(spelling, Span::new(0, 0)));
    }
    // `<name> is ready` / `<이름> 쿨타임이 남았으면` is a whole condition on
    // its own; nothing inside it is a comparison to be taken apart.
    if let Some((condition, end)) = cooldown_condition_at(tokens, 0) {
        if end == tokens.len() {
            return Ok(condition);
        }
    }
    // Parentheses around a whole NME condition should not turn its logical
    // connectors into an opaque Python expression. Keep the token-based
    // logical grammar active while still allowing parentheses inside an
    // operand, such as `if (ready) and score > 2`.
    if let Some(inner) = strip_outer_condition_parentheses(tokens) {
        return parse_natural_condition(source, inner, connector, known_names, spelling);
    }
    // `or` has lower precedence than `and`, just like Python.  Splitting on
    // tokens (rather than source text) keeps strings and nested expressions
    // out of the easy-language grammar. A split that would produce an empty
    // operand (leading or trailing logical word) falls through to the atom
    // parser, which reports an exact diagnostic instead of panicking.
    if let Some(index) = logical_operator_at(tokens, LogicalOp::Or) {
        if index > 0 && index + 1 < tokens.len() {
            let left =
                parse_natural_condition(source, &tokens[..index], None, known_names, spelling)?;
            let right = parse_natural_condition(
                source,
                &tokens[index + 1..],
                connector,
                known_names,
                spelling,
            )?;
            return Ok(Condition::Logical {
                left: Box::new(left),
                operator: LogicalOp::Or,
                right: Box::new(right),
            });
        }
    }
    if let Some(index) = logical_operator_at(tokens, LogicalOp::And) {
        if index > 0 && index + 1 < tokens.len() {
            let left =
                parse_natural_condition(source, &tokens[..index], None, known_names, spelling)?;
            let right = parse_natural_condition(
                source,
                &tokens[index + 1..],
                connector,
                known_names,
                spelling,
            )?;
            return Ok(Condition::Logical {
                left: Box::new(left),
                operator: LogicalOp::And,
                right: Box::new(right),
            });
        }
    }
    parse_natural_condition_atom(source, tokens, connector, known_names, spelling)
}

fn condition_wrapper_end(tokens: &[Token], start: usize) -> Option<usize> {
    if !tokens
        .get(start)
        .is_some_and(|token| matches!(&token.tok, Tok::Lpar))
    {
        return None;
    }
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate().skip(start) {
        match token.tok {
            Tok::Lpar => depth += 1,
            Tok::Rpar => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn strip_outer_condition_parentheses(tokens: &[Token]) -> Option<&[Token]> {
    if tokens.len() < 2
        || !tokens
            .first()
            .is_some_and(|token| matches!(&token.tok, Tok::Lpar))
    {
        return None;
    }
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match token.tok {
            Tok::Lpar => depth += 1,
            Tok::Rpar => {
                depth = depth.checked_sub(1)?;
                if depth == 0 && index + 1 != tokens.len() {
                    return None;
                }
            }
            _ => {}
        }
    }
    (depth == 0).then(|| &tokens[1..tokens.len() - 1])
}

fn logical_operator_at(tokens: &[Token], operator: LogicalOp) -> Option<usize> {
    let expected = match operator {
        LogicalOp::And => &["and", "그리고"][..],
        LogicalOp::Or => &["or", "또는"][..],
    };
    let mut depth = 0usize;
    let exact = tokens.iter().enumerate().find_map(|(index, token)| {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            _ => {}
        }
        // `or equal` is part of a `less/greater than or equal to`
        // comparison, not a logical `or`.
        (depth == 0
            && token_matches_exact(token, expected)
            && !(expected.contains(&"or") && is_or_equal_phrase_at(tokens, index)))
        .then_some(index)
    });
    if exact.is_some() {
        return exact;
    }

    // A single misspelled logical connector is easy to recover without
    // guessing across arbitrary expressions. Keep the same precedence and
    // bracket-depth rules as the exact path.
    //
    // `of` and `on` are one edit away from `or`, and `an` from `and`, but all
    // three are ordinary parts of the sentence grammar (`the length of name`,
    // `is on cooldown`, `an empty list`). Repairing one of them would split a
    // condition down the middle and mean something nobody wrote.
    depth = 0;
    tokens.iter().enumerate().find_map(|(index, token)| {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            _ => {}
        }
        (depth == 0
            && !(expected.contains(&"or") && is_or_equal_phrase_at(tokens, index))
            && !token_matches_exact(token, &["of", "on", "an", "in"])
            && word_matches_any(token, expected, MatchMode::Recover))
        .then_some(index)
    })
}

fn parse_natural_condition_atom(
    source: &str,
    tokens: &[Token],
    connector: Option<ConditionConnector>,
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<Condition, Diagnostic> {
    let mut cleaned: Vec<&Token> = tokens.iter().collect();
    while cleaned.first().is_some_and(|token| {
        token_matches_exact(token, &["정말", "혹시", "please", "really", "the"])
    }) {
        cleaned.remove(0);
    }
    if cleaned.is_empty() {
        return Err(condition_missing(spelling, span_of(tokens)));
    }

    if let Some(condition) = parse_english_condition(source, &cleaned, known_names) {
        return Ok(condition);
    }

    if looks_like_incomplete_english_condition(&cleaned) {
        let condition_span = Span::new(cleaned[0].span.start, cleaned[cleaned.len() - 1].span.end);
        return Err(condition_invalid(spelling, condition_span));
    }

    if connector.is_none()
        && cleaned.len() > 1
        && !cleaned
            .iter()
            .any(|token| token_matches_exact(token, &["and", "or", "그리고", "또는"]))
    {
        // A Korean comparison ending may live in a logical operand that has
        // no connector of its own: `점수가 0보다 크면 그리고 점수가 3보다
        // 작으면`. Discover the ending inside this operand and reparse it.
        let owned: Vec<Token> = cleaned.iter().map(|token| (*token).clone()).collect();
        if let Some((relative_at, found)) = find_condition_connector(&owned) {
            let (condition, body_start, found) =
                condition_tokens_before(&owned, 0, relative_at, found);
            if body_start == owned.len() && !condition.is_empty() {
                return parse_natural_condition(
                    source,
                    &condition,
                    Some(found),
                    known_names,
                    spelling,
                );
            }
        }
    }

    match connector {
        Some(ConditionConnector::Missing) => {
            if let Some(condition) = korean_contains_condition(&cleaned, known_names, true) {
                return Ok(condition);
            }
            let (value, explicit_not) = parse_truth_subject(&cleaned, known_names, spelling)?;
            return Ok(Condition::Truthy {
                value,
                negated: !explicit_not,
            });
        }
        Some(ConditionConnector::Exists) => {
            if let Some(condition) = korean_contains_condition(&cleaned, known_names, false) {
                return Ok(condition);
            }
            let (value, explicit_not) = parse_truth_subject(&cleaned, known_names, spelling)?;
            return Ok(Condition::Truthy {
                value,
                negated: explicit_not,
            });
        }
        Some(
            ConditionConnector::Greater
            | ConditionConnector::Less
            | ConditionConnector::GreaterOrEqual
            | ConditionConnector::LessOrEqual,
        ) => {
            let operator = match connector {
                Some(ConditionConnector::Greater) => CompareOp::Greater,
                Some(ConditionConnector::Less) => CompareOp::Less,
                Some(ConditionConnector::GreaterOrEqual) => CompareOp::GreaterOrEqual,
                _ => CompareOp::LessOrEqual,
            };
            return parse_korean_comparison(
                source,
                &cleaned,
                known_names,
                operator,
                &["보다", "더", "작을", "클", "작거나", "크거나"],
                spelling,
                false,
            );
        }
        Some(ConditionConnector::Equals | ConditionConnector::NotEquals) => {
            return parse_korean_comparison(
                source,
                &cleaned,
                known_names,
                CompareOp::Equal,
                &["과", "와", "랑", "이랑", "하고", "to"],
                spelling,
                matches!(connector, Some(ConditionConnector::NotEquals)),
            );
        }
        _ => {}
    }

    if cleaned.len() == 1 {
        return Ok(Condition::Truthy {
            value: condition_left(cleaned[0], known_names),
            negated: false,
        });
    }

    let condition_span = Span::new(cleaned[0].span.start, cleaned[cleaned.len() - 1].span.end);
    let condition_text = &source[condition_span.start..condition_span.end];
    if is_valid_python_expression(condition_text) {
        return Ok(Condition::Python(Code::Source(condition_span)));
    }

    Err(condition_invalid(spelling, condition_span))
}

/// `만약에 친구들에 민수가 있으면` — a list holding a value.
///
/// Two words exactly: the list, carrying the particle that marks a container,
/// and the thing being looked for. The particle plus a name the program
/// already made is the whole gate, so `만약에 시간이 있으면` (one word) and
/// `만약에 준비가 없으면` keep their old meaning.
fn korean_contains_condition(
    tokens: &[&Token],
    known_names: &HashSet<String>,
    negated: bool,
) -> Option<Condition> {
    let [container, member] = tokens else {
        return None;
    };
    let word = name_word(container)?;
    let base = CONTAINS_WORDS_KO
        .iter()
        .find_map(|particle| word.strip_suffix(particle).filter(|base| !base.is_empty()))?;
    if !known_names.contains(base) {
        return None;
    }
    let member_word = name_word(member)?;
    let right = match resolve_known_particle(member_word, known_names) {
        Some(name) => ConditionValue::Name(name.to_string()),
        None => ConditionValue::Text(strip_reading_particle(member_word).to_string()),
    };
    Some(Condition::Compare {
        left: ConditionValue::Name(base.to_string()),
        operator: CompareOp::Contains,
        right,
        negated,
    })
}

/// A reading standing where one side of a condition does: `친구들 개수가 3보다
/// 크면`, `if how many friends is greater than 3`.
fn condition_reading_at(
    tokens: &[&Token],
    known_names: &HashSet<String>,
) -> Option<(ConditionValue, usize)> {
    let owned: Vec<Token> = tokens.iter().map(|token| (*token).clone()).collect();
    match reading_prefix(&owned, known_names) {
        Some((Value::Reading { of, reading }, used)) => {
            Some((ConditionValue::Reading { of, reading }, used))
        }
        Some((Value::Remainder { of, by }, used)) => {
            Some((ConditionValue::Remainder { of, by }, used))
        }
        Some((Value::Entry { of, key }, used)) => Some((ConditionValue::Entry { of, key }, used)),
        _ => None,
    }
}

fn parse_truth_subject(
    tokens: &[&Token],
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<(ConditionValue, bool), Diagnostic> {
    let tokens = if tokens.len() == 2 && token_matches_exact(tokens[1], &["은", "는", "이", "가"])
    {
        &tokens[..1]
    } else {
        tokens
    };
    let mut cursor = 1;
    if tokens
        .get(cursor)
        .is_some_and(|token| token_word(token) == Some("is"))
    {
        cursor += 1;
    }
    let explicit_not = tokens
        .get(cursor)
        .is_some_and(|token| matches!(token.tok, Tok::Not) || token_word(token) == Some("not"));
    if explicit_not {
        cursor += 1;
    }
    if cursor != tokens.len() {
        return Err(condition_invalid(
            spelling,
            Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end),
        ));
    }
    Ok((condition_left(tokens[0], known_names), explicit_not))
}

fn parse_korean_comparison(
    source: &str,
    tokens: &[&Token],
    known_names: &HashSet<String>,
    operator: CompareOp,
    trailing_markers: &[&str],
    spelling: Spelling,
    negated: bool,
) -> Result<Condition, Diagnostic> {
    if tokens.len() < 2 {
        return Err(condition_invalid(spelling, span_of_refs(tokens)));
    }
    let (left, consumed) = condition_reading_at(tokens, known_names)
        .filter(|&(_, used)| used < tokens.len())
        .unwrap_or_else(|| (condition_left(tokens[0], known_names), 1));
    let mut right = tokens[consumed..]
        .iter()
        .map(|token| (*token).clone())
        .collect::<Vec<_>>();
    while right
        .first()
        .is_some_and(|token| token_matches_exact(token, &["은", "는", "이", "가"]))
    {
        right.remove(0);
    }
    trim_condition_markers(&mut right, trailing_markers);
    if right.is_empty() {
        return Err(condition_invalid(spelling, span_of_refs(tokens)));
    }
    let right = condition_rhs(source, &right, known_names)
        .ok_or_else(|| condition_invalid(spelling, span_of_refs(tokens)))?;
    Ok(Condition::Compare {
        left,
        operator,
        right,
        negated,
    })
}

fn parse_english_condition(
    source: &str,
    tokens: &[&Token],
    known_names: &HashSet<String>,
) -> Option<Condition> {
    if tokens.len() < 2 {
        return None;
    }
    let (left, mut cursor) = condition_reading_at(tokens, known_names)
        .filter(|&(_, used)| used < tokens.len())
        .unwrap_or_else(|| (condition_left(tokens[0], known_names), 1));
    if token_word(tokens[cursor]) == Some("is") {
        cursor += 1;
    }
    if tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["really"]))
    {
        cursor += 1;
    }
    // `friends does not contain Mina` — the helper verb is only skipped when
    // a `not` follows it, so an ordinary word `does` is never eaten.
    if tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["does", "do", "did"]))
        && tokens
            .get(cursor + 1)
            .is_some_and(|token| matches!(token.tok, Tok::Not) || token_word(token) == Some("not"))
    {
        cursor += 1;
    }
    let negated = tokens
        .get(cursor)
        .is_some_and(|token| matches!(token.tok, Tok::Not) || token_word(token) == Some("not"));
    if negated {
        cursor += 1;
    }
    let predicate = tokens.get(cursor).and_then(|token| token_word(token))?;
    if condition_word_matches(
        predicate,
        &["exists", "present", "missing", "absent", "empty"],
    ) {
        if cursor + 1 != tokens.len() {
            return None;
        }
        let missing = condition_word_matches(predicate, &["missing", "absent", "empty"]);
        return Some(Condition::Truthy {
            value: left,
            negated: missing ^ negated,
        });
    }
    // `if friends contains Mina`. Matched exactly, never repaired: these are
    // ordinary English words and a one-letter repair would start claiming
    // sentences that merely contain them.
    if tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, CONTAINS_WORDS_EN))
    {
        let ConditionValue::Name(name) = &left else {
            return None;
        };
        if !known_names.contains(name) {
            return None;
        }
        cursor += 1;
        let right_tokens = tokens.get(cursor..)?;
        if right_tokens.is_empty() {
            return None;
        }
        let owned = right_tokens
            .iter()
            .map(|token| (*token).clone())
            .collect::<Vec<_>>();
        let right = condition_rhs(source, &owned, known_names)?;
        return Some(Condition::Compare {
            left,
            operator: CompareOp::Contains,
            right,
            negated,
        });
    }
    let operator = if condition_word_matches(
        predicate,
        &["greater", "above", "great", "larger", "bigger", "higher", "more"],
    ) {
        CompareOp::Greater
    } else if condition_word_matches(
        predicate,
        &["less", "below", "small", "smaller", "lower", "fewer"],
    ) {
        CompareOp::Less
    } else if condition_word_matches(predicate, &["equals", "equal", "same"]) {
        CompareOp::Equal
    } else {
        return None;
    };
    cursor += 1;
    while tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["to", "than", "as"]))
    {
        cursor += 1;
    }
    // `less than or equal to` / `greater than or equal to` narrow the
    // comparison to `<=` / `>=`.
    if tokens
        .get(cursor)
        .is_some_and(|token| token_word(token) == Some("or"))
        && tokens.get(cursor + 1).is_some_and(|token| {
            condition_word_matches(token_word(token).unwrap_or(""), &["equal", "equals"])
        })
    {
        let operator = match operator {
            CompareOp::Greater => CompareOp::GreaterOrEqual,
            CompareOp::Less => CompareOp::LessOrEqual,
            other => other,
        };
        cursor += 2;
        while tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, &["to", "than", "as"]))
        {
            cursor += 1;
        }
        let right_tokens = tokens.get(cursor..)?;
        if right_tokens.is_empty() {
            return None;
        }
        let owned = right_tokens
            .iter()
            .map(|token| (*token).clone())
            .collect::<Vec<_>>();
        let right = condition_rhs(source, &owned, known_names)?;
        return Some(Condition::Compare {
            left,
            operator,
            right,
            negated,
        });
    }
    let right_tokens = tokens.get(cursor..)?;
    if right_tokens.is_empty() {
        return None;
    }
    let owned = right_tokens
        .iter()
        .map(|token| (*token).clone())
        .collect::<Vec<_>>();
    let right = condition_rhs(source, &owned, known_names)?;
    Some(Condition::Compare {
        left,
        operator,
        right,
        negated,
    })
}

/// A colon-free condition can make an incomplete comparison look like valid
/// Python (`score is greater` is a valid identity expression). Once the user
/// has clearly started one of NME's comparison words, keep the missing value
/// as a friendly NME diagnostic instead of silently emitting that expression.
fn looks_like_incomplete_english_condition(tokens: &[&Token]) -> bool {
    if tokens.len() < 2 {
        return false;
    }
    let mut cursor = 1;
    if tokens.get(cursor).and_then(|token| token_word(token)) == Some("is") {
        cursor += 1;
    }
    if tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["really"]))
    {
        cursor += 1;
    }
    if tokens
        .get(cursor)
        .is_some_and(|token| matches!(token.tok, Tok::Not) || token_word(token) == Some("not"))
    {
        cursor += 1;
    }
    let Some(predicate) = tokens.get(cursor).and_then(|token| token_word(token)) else {
        return false;
    };
    if !condition_word_matches(
        predicate,
        &[
            "greater", "above", "great", "larger", "bigger", "higher", "more", "less", "below",
            "small", "smaller", "lower", "fewer", "equals", "equal", "same",
        ],
    ) {
        return false;
    }
    cursor += 1;
    while tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["to", "than", "as"]))
    {
        cursor += 1;
    }
    cursor >= tokens.len()
}

fn condition_left(token: &Token, known_names: &HashSet<String>) -> ConditionValue {
    if let Some(literal) = literal_token(token) {
        return ConditionValue::Literal(literal);
    }
    let Some(word) = name_word(token) else {
        return ConditionValue::Python(Code::Source(token.span));
    };
    let name = resolve_known_particle(word, known_names)
        .or_else(|| strip_any_suffix(word, &["은", "는", "이", "가", "을", "를"]))
        .unwrap_or(word);
    if is_elapsed_name(name, known_names) {
        return elapsed_condition_value();
    }
    ConditionValue::Name(name.to_string())
}

/// `잰시간` and `elapsed` read the stopwatch wherever a condition may name a
/// value, so `만약 잰시간이 3보다 크면` compares seconds and not a name.
fn is_elapsed_name(name: &str, known_names: &HashSet<String>) -> bool {
    !known_names.contains(name)
        && (ELAPSED_WORDS_EN.contains(&name) || ELAPSED_WORDS_KO.contains(&name))
}

fn elapsed_condition_value() -> ConditionValue {
    ConditionValue::Python(Code::Generated(ELAPSED_PYTHON.to_string()))
}

fn condition_rhs(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<ConditionValue> {
    if tokens.len() == 1 {
        if let Some(literal) = literal_token(&tokens[0]) {
            return Some(ConditionValue::Literal(literal));
        }
        if let Some(word) = name_word(&tokens[0]) {
            if is_elapsed_name(word, known_names) {
                return Some(elapsed_condition_value());
            }
            if let Some(name) = resolve_known_particle(word, known_names) {
                return Some(ConditionValue::Name(name.to_string()));
            }
            return Some(ConditionValue::Text(word.to_string()));
        }
    }
    let span = span_of(tokens);
    let text = &source[span.start..span.end];
    if is_valid_python_expression(text) && tokens.iter().any(is_code_token) {
        return Some(ConditionValue::Python(Code::Source(span)));
    }
    tokens
        .iter()
        .all(is_text_token)
        .then(|| ConditionValue::Text(text.to_string()))
}

fn trim_condition_markers(tokens: &mut Vec<Token>, markers: &[&str]) {
    while tokens
        .last()
        .is_some_and(|token| token_matches_exact(token, markers))
    {
        tokens.pop();
    }
    if let Some(last) = tokens.last_mut() {
        trim_name_token_suffix(last, markers);
    }
}

fn condition_connector_exact(token: &Token, is_last: bool) -> Option<ConditionConnector> {
    let word = token_word(token)?;
    if let Some((_, connector)) = split_attached_condition_token(token) {
        return Some(connector);
    }
    let candidates = [
        (
            ConditionConnector::Then,
            &[
                "then",
                "그러면",
                "그럼",
                "하면",
                "이면",
                "이라면",
                "경우",
                "때",
                "일때",
            ][..],
        ),
        (ConditionConnector::Exists, &["있으면", "있다면"][..]),
        (
            ConditionConnector::Missing,
            &[
                "없으면",
                "없다면",
                "비었으면",
                "비었다면",
                "비어있으면",
                "비어있다면",
            ][..],
        ),
        (
            ConditionConnector::Equals,
            &["같으면", "같다면", "라면", "면", "같을"][..],
        ),
        (
            ConditionConnector::NotEquals,
            &["같지않으면", "같지않다면", "같지않을"][..],
        ),
        (
            ConditionConnector::Greater,
            &[
                "크면", "크다면", "클", "초과면", "초과이면", "초과인", "초과일", "많으면",
                "많다면", "많을", "넘으면", "넘는다면", "넘을",
            ][..],
        ),
        (
            ConditionConnector::Less,
            &[
                "작으면", "작다면", "작을", "미만이면", "미만면", "미만인", "미만일",
                "적으면", "적다면", "적을", "모자라면", "안되면",
            ][..],
        ),
        (
            ConditionConnector::GreaterOrEqual,
            &["크거나같으면", "크거나같다면", "크거나같을", "크거나같은", "이상이면", "이상인", "이상일"][..],
        ),
        (
            ConditionConnector::LessOrEqual,
            &["작거나같으면", "작거나같다면", "작거나같을", "작거나같은", "이하이면", "이하인", "이하일"][..],
        ),
    ];
    for (kind, words) in candidates {
        if words.contains(&word) {
            return Some(kind);
        }
    }
    if is_last {
        if matches!(word, "exists" | "present") {
            return Some(ConditionConnector::Exists);
        }
        if matches!(word, "missing" | "absent") {
            return Some(ConditionConnector::Missing);
        }
    }
    None
}

fn condition_connector_recovered(token: &Token, is_last: bool) -> Option<ConditionConnector> {
    let word = token_word(token)?;
    // Some Korean consonant substitutions are equally close to two
    // connectors under plain edit distance (`잇으면` is close to both
    // `있으면` and `없으면`). These common spoken spellings have a clear
    // intended meaning, so resolve them before collecting ambiguous fuzzy
    // candidates.
    match word {
        "잇으면" | "잇다면" => return Some(ConditionConnector::Exists),
        "업으면" | "업다면" => return Some(ConditionConnector::Missing),
        _ => {}
    }
    let candidates = [
        (
            ConditionConnector::Then,
            &[
                "then",
                "그러면",
                "그럼",
                "하면",
                "이면",
                "이라면",
                "경우",
                "때",
                "일때",
            ][..],
        ),
        (ConditionConnector::Exists, &["있으면", "있다면"][..]),
        (ConditionConnector::Missing, &["없으면", "없다면"][..]),
        (
            ConditionConnector::Equals,
            &["같으면", "같다면", "라면", "면"][..],
        ),
        (ConditionConnector::Greater, &["크면", "크다면", "클"][..]),
        (ConditionConnector::Less, &["작으면", "작다면", "작을"][..]),
    ];
    let mut recovered = candidates
        .iter()
        .filter_map(|(kind, words)| {
            words
                .iter()
                .any(|candidate| {
                    !word.eq_ignore_ascii_case(candidate)
                        && word != "the"
                        // `than` is a normal part of `greater than`/`less
                        // than`, not a misspelled `then` connector.  Keeping
                        // it out avoids making an otherwise clear condition
                        // ambiguous when the real connector is also mistyped.
                        && word != "than"
                        && word.chars().count() >= 2
                        && one_typo_away(word, candidate)
                })
                .then_some(*kind)
        })
        .collect::<Vec<_>>();
    if is_last {
        if ["exists", "present"].iter().any(|candidate| {
            !word.eq_ignore_ascii_case(candidate) && one_typo_away(word, candidate)
        }) {
            recovered.push(ConditionConnector::Exists);
        }
        if ["missing", "absent"].iter().any(|candidate| {
            !word.eq_ignore_ascii_case(candidate) && one_typo_away(word, candidate)
        }) {
            recovered.push(ConditionConnector::Missing);
        }
    }
    // Korean learners often shorten `같으면`/`작으면`/`크면` to the spoken
    // `...먄` ending. It is a bounded connector-only repair, not a general
    // fuzzy match, and still goes through the unique-candidate check below.
    match word {
        "있으먄" | "있먄" => recovered.push(ConditionConnector::Exists),
        "없으먄" | "없먄" => recovered.push(ConditionConnector::Missing),
        "같먄" | "같으먄" | "라먄" | "먄" => recovered.push(ConditionConnector::Equals),
        "크먄" | "크으먄" => recovered.push(ConditionConnector::Greater),
        "작먄" | "작으먄" => recovered.push(ConditionConnector::Less),
        _ => {}
    }
    recovered.sort_by_key(|kind| *kind as u8);
    recovered.dedup();
    if recovered.len() == 1 {
        recovered.first().copied()
    } else {
        None
    }
}

fn condition_missing(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ConditionMissing,
        "the condition is missing",
        "조건이 비어 있어요",
        span,
    )
    .with_bilingual_hint(
        "write `if ready` or `if score > 10` and indent the next line",
        "`만약에 준비됐으면`처럼 적고 다음 줄을 들여쓰세요",
    )
}

fn condition_invalid(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ConditionInvalid,
        "I couldn't understand this condition",
        "이 조건을 확실하게 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "try `if ready`, `if score > 10`, or `if name exists`",
        "`만약에 이름이 있으면` 또는 `만약 점수 > 10`처럼 적어 보세요",
    )
}

// --------------------------------------------------------------- repeat

#[allow(clippy::too_many_lines)]
fn match_times(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // `다시 한 번 설명해 주세요` puts `다시` one character from `다시해`, and was
    // answered with "the repeat count is missing". A repaired repeat word is
    // a guess, and a line that ends the way a written Korean sentence ends
    // beats a guess. `2번 반목해서 다시 말해줘` ends in `말해줘` and still loops.
    if mode == MatchMode::Recover && korean_line_is_a_sentence(tokens, known_names) {
        return Ok(None);
    }
    if let Some((count, body_start)) = attached_korean_times_sentence(source, tokens, known_names) {
        let mut body_start = body_start;
        if let Some((_, consumed)) = repeat_action_at(tokens, body_start, mode) {
            body_start += consumed;
            if tokens.get(body_start).is_some_and(is_connector_word) {
                body_start += 1;
            }
        }
        let inline = parse_sentence_repeat_body(
            source,
            &tokens[body_start..],
            block,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }
    if let Some((count, colon_at)) = attached_korean_times_header(source, tokens, known_names) {
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Repeat,
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }
    if let Some((times_at, spelling)) = find_times_colon(tokens, mode) {
        let count_start = repeat_action_at(tokens, 0, mode).map_or(0, |(_, consumed)| consumed);
        let count = parse_count(
            source,
            &tokens[count_start..times_at],
            known_names,
            spelling,
        )?;
        let colon_at = times_at + 1;
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Repeat,
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }

    // A bare count header (`3 times` / `3번`) opens a block in the same way as
    // the colon form, with `end`/`끝` providing the closing line.
    //
    // `3번 환영합니다` repeats one word three times. `1번 출구에서 만납시다` is
    // where two people are meeting, and `이번 주에는 비가 세 번 왔습니다` is the
    // weather. Both ended the way a written Korean sentence ends, and neither
    // is one number followed by one word, which is the whole of the counted
    // sentence shape.
    let counted_sentence_shape = |(marker_at, spelling): &(usize, Spelling)| {
        // `지우기 전에 한 번 더 확인` has `번` in it and no count in front of
        // it, and was answered with "I couldn't understand how many times to
        // repeat". There is no count here because there is no loop here.
        if *marker_at > 1
            && parse_count(source, &tokens[..*marker_at], known_names, *spelling).is_err()
        {
            return false;
        }
        if repeat_action_at(tokens, 0, mode).is_some()
            || repeat_action_at(tokens, marker_at + 1, MatchMode::Exact).is_some()
        {
            return true;
        }
        // With no repeat word on the line, a Korean count written out in
        // words is how people say how often something happened, not how they
        // open a loop: `두 번 갔습니다`, `세 번 왔습니다`, `한 번 더 확인`. Every
        // documented loop of this shape counts in digits — `3번 환영합니다` —
        // and spelling the count out still works with `반복해서` beside it.
        let counts_in_digits = tokens[..*marker_at].iter().any(|token| match &token.tok {
            Tok::Int { .. } | Tok::Float { .. } => true,
            Tok::Name { name } => name.starts_with(|character: char| character.is_ascii_digit()),
            _ => false,
        });
        if *spelling == Spelling::Korean && *marker_at + 1 < tokens.len() && !counts_in_digits {
            return false;
        }
        (*marker_at == 1 && tokens.len() == marker_at + 2)
            || !is_written_korean_sentence(tokens, known_names)
    };
    if let Some((marker_at, spelling)) =
        find_count_marker(tokens, mode).filter(counted_sentence_shape)
    {
        if marker_at + 1 == tokens.len()
            && marker_at > 0
            && repeat_action_at(tokens, 0, mode).is_none()
        {
            let count = parse_count(source, &tokens[..marker_at], known_names, spelling)?;
            let inline = parse_suite_body(
                source,
                &tokens[tokens.len()..],
                block,
                SuiteKind::Repeat,
                span_of(tokens),
                known_names,
            )?;
            return Ok(Some(NmeStmt::Times { count, inline }));
        }
        if marker_at > 0
            && marker_at + 1 < tokens.len()
            && repeat_action_at(tokens, 0, mode).is_none()
            && repeat_action_at(tokens, 0, MatchMode::Recover).is_none()
            && repeat_action_at(tokens, marker_at + 1, mode).is_none()
            && repeat_action_at(tokens, marker_at + 1, MatchMode::Recover).is_none()
        {
            let count = parse_count(source, &tokens[..marker_at], known_names, spelling)?;
            let mut body_start = marker_at + 1;
            if tokens.get(body_start).is_some_and(is_connector_word) {
                body_start += 1;
            }
            let inline = parse_sentence_repeat_body(
                source,
                &tokens[body_start..],
                block,
                span_of(&tokens[..body_start]),
                known_names,
            )?;
            return Ok(Some(NmeStmt::Times { count, inline }));
        }
    }

    // Sentence order: `3번 반복해 ...` / `3 times repeat ...`.
    if let Some((marker_at, spelling)) = find_count_marker(tokens, mode) {
        if let Some((_, consumed)) = repeat_action_at(tokens, marker_at + 1, mode) {
            if marker_at == 0 {
                return Err(repeat_count_missing(spelling, tokens[marker_at + 1].span));
            }
            let count = parse_count(source, &tokens[..marker_at], known_names, spelling)?;
            let mut body_start = marker_at + 1 + consumed;
            if tokens.get(body_start).is_some_and(is_connector_word) {
                body_start += 1;
            }
            let inline = parse_sentence_repeat_body(
                source,
                &tokens[body_start..],
                block,
                span_of(&tokens[..body_start]),
                known_names,
            )?;
            return Ok(Some(NmeStmt::Times { count, inline }));
        }
    }

    // English-first and freely mixed order: `repeat 3 times` / `반복해 3 times`.
    if let Some((spelling, consumed)) = repeat_action_at(tokens, 0, mode) {
        // The whole line is searched, not only the tail, so a counter word
        // can still see the repeat word that opened the line. An exact repeat
        // word is proof enough of intent to also repair a misspelled counter
        // (`repeat 3 tiems and show hello`).
        let Some((marker_at, marker_spelling)) = find_count_marker(tokens, mode)
            .or_else(|| find_count_marker(tokens, MatchMode::Recover))
            .filter(|&(at, _)| at >= consumed)
        else {
            // `repeat once`, `repeat twice`, `repeat 3` — the counting word
            // already says how many, so no counter word is needed.
            if let Some(token) = tokens
                .get(consumed)
                .filter(|token| is_written_number(token))
            {
                let count =
                    parse_count(source, std::slice::from_ref(token), known_names, spelling)?;
                let mut body_start = consumed + 1;
                if tokens.get(body_start).is_some_and(is_connector_word) {
                    body_start += 1;
                }
                let inline = parse_sentence_repeat_body(
                    source,
                    &tokens[body_start..],
                    block,
                    span_of(&tokens[..body_start]),
                    known_names,
                )?;
                return Ok(Some(NmeStmt::Times { count, inline }));
            }
            return Err(repeat_count_missing(spelling, tokens[0].span));
        };
        if marker_at == consumed {
            return Err(repeat_count_missing(spelling, tokens[0].span));
        }
        let count = parse_count(
            source,
            &tokens[consumed..marker_at],
            known_names,
            marker_spelling,
        )?;
        let mut body_start = marker_at + 1;
        if tokens.get(body_start).is_some_and(is_connector_word) {
            body_start += 1;
        }
        let inline = parse_sentence_repeat_body(
            source,
            &tokens[body_start..],
            block,
            span_of(&tokens[..body_start]),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }

    Ok(None)
}

/// Parse a body that was introduced by the sentence repeat spelling. A plain
/// run of words is naturally a thing to say (`3번 안녕하세요`), while a
/// beginner/Python-shaped body still goes through the normal classifier.
fn parse_sentence_repeat_body(
    source: &str,
    body: &[Token],
    block: &BlockCtx<'_>,
    header_span: Span,
    known_names: &HashSet<String>,
) -> Result<Option<InlineStmt>, Diagnostic> {
    // `repeat 3 times.` / `3번 반복해.` — a body made of nothing but sentence
    // punctuation is punctuation. The header opens a block, exactly as it
    // does without the full stop, instead of printing `.` three times.
    // `repeat 3 times, show hello` — the beginner comma belongs to the
    // header, not to what should be printed three times.
    let mut body = body;
    while body
        .first()
        .is_some_and(|token| matches!(token.tok, Tok::Comma))
    {
        body = &body[1..];
    }
    let body = if body.iter().all(is_command_ending) {
        &body[..0]
    } else {
        body
    };
    // `3번 되풀이해서 안녕 말해줘` — the body opens with another word for
    // "repeat". Printing it three times is never what that means.
    if body.first().and_then(name_word).is_some_and(|word| {
        NEAR_MISS_ACTIONS
            .iter()
            .any(|(written, _)| word.eq_ignore_ascii_case(written))
    }) {
        return Err(unknown_action_word_diagnostic(body, known_names));
    }
    if branch_shape(body).is_some() {
        return Err(branch_without_condition_diagnostic(span_of(body)));
    }
    if let Some(inner) = match_break(source, body, known_names, MatchMode::Exact)? {
        return Ok(Some(InlineStmt::Nme(Box::new(inner))));
    }
    let plain_words = !body.is_empty()
        && body.iter().all(is_text_token)
        && !body.iter().any(|token| literal_token(token).is_some());
    let has_action = output_action_at(body, 0, MatchMode::Exact).is_some()
        || output_action_at(body, 0, MatchMode::Recover).is_some()
        || output_action_ending(body, MatchMode::Exact, known_names).is_some()
        || output_action_ending(body, MatchMode::Recover, known_names).is_some()
        || ask_action_at(body, 0, MatchMode::Exact).is_some()
        || ask_action_at(body, 0, MatchMode::Recover).is_some()
        || find_ask_shape(body, MatchMode::Exact).is_some()
        || find_ask_shape(body, MatchMode::Recover).is_some();
    if !body.is_empty() && (!plain_words || has_action) {
        if let Some(inner) = classify(source, body, &BlockCtx::Inline, known_names)? {
            if matches!(&inner, NmeStmt::ElseIf { .. } | NmeStmt::Else { .. }) {
                return Err(branch_without_condition_diagnostic(span_of(body)));
            }
            return Ok(Some(InlineStmt::Nme(Box::new(inner))));
        }
    }
    if plain_words {
        let value = parse_value(source, body, known_names, true).map_err(|()| {
            Diagnostic::bilingual(
                DiagnosticCode::RepeatBodyUnparseable,
                "I couldn't understand what to repeat",
                "무엇을 반복할지 이해하지 못했어요",
                span_of(body),
            )
            .with_bilingual_hint(
                "write a sentence such as `3번 안녕하세요` or add `말해줘`",
                "`3번 안녕하세요`처럼 쓰거나 끝에 `말해줘`를 붙여 주세요",
            )
        })?;
        return Ok(Some(InlineStmt::Nme(Box::new(NmeStmt::Say { value }))));
    }
    parse_suite_body(
        source,
        body,
        block,
        SuiteKind::Repeat,
        header_span,
        known_names,
    )
}

fn has_recoverable_repeat_shape(tokens: &[Token]) -> bool {
    if let Some((marker_at, _)) = find_count_marker(tokens, MatchMode::Exact) {
        if repeat_action_at(tokens, marker_at + 1, MatchMode::Exact).is_none()
            && repeat_action_at(tokens, marker_at + 1, MatchMode::Recover).is_some()
        {
            return true;
        }
    }

    repeat_action_at(tokens, 0, MatchMode::Exact).is_none()
        && repeat_action_at(tokens, 0, MatchMode::Recover).is_some()
        && find_count_marker(tokens, MatchMode::Exact).is_some()
}

fn repeat_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, REPEAT_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, REPEAT_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
        .or_else(|| counted_repeat_action_at(tokens, start))
}

/// A repeat word from [`REPEAT_COUNT_WORDS_EN`] or [`REPEAT_COUNT_WORDS_KO`],
/// read only when a count stands beside it. Never repaired: the count is
/// carrying the meaning here, and a guess on top of it would claim any
/// sentence with a number in it.
fn counted_repeat_action_at(tokens: &[Token], start: usize) -> Option<(Spelling, usize)> {
    let counts_at = |index: usize| {
        tokens.get(index).is_some_and(|token| {
            matches!(token.tok, Tok::Int { .. } | Tok::Float { .. })
                || token_matches_exact(token, NUMBER_WORDS_EN)
                || token_matches_exact(token, NUMBER_WORDS_KO)
                || token_matches_exact(token, TIMES_WORDS_KO)
                || name_word(token)
                    .and_then(|word| word.chars().next())
                    .is_some_and(|first| first.is_ascii_digit())
        })
    };
    if let Some(consumed) = action_phrase_at(tokens, start, REPEAT_COUNT_WORDS_EN, MatchMode::Exact)
    {
        if counts_at(start + consumed) {
            return Some((Spelling::English, consumed));
        }
    }
    if let Some(consumed) = action_phrase_at(tokens, start, REPEAT_COUNT_WORDS_KO, MatchMode::Exact)
    {
        if start > 0 && counts_at(start - 1) {
            return Some((Spelling::Korean, consumed));
        }
    }
    None
}

fn parse_count(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<Code, Diagnostic> {
    let tokens = trim_command_endings(tokens);
    if tokens.is_empty() {
        return Err(repeat_count_missing(spelling, Span::new(0, 0)));
    }
    let span = span_of(tokens);
    if let [token] = tokens {
        if let Some(code) = name_word(token).and_then(|word| number_word_code(word, TIMES_WORDS_KO))
        {
            return Ok(code);
        }
    }
    // A bare word nobody set would compile to `range(word)` and die with
    // `NameError`, so it is reported here by name instead.
    if let Some(word) = unreadable_number_word(tokens, known_names) {
        return Err(repeat_count_word_diagnostic(word, span));
    }
    if !is_valid_python_expression(&source[span.start..span.end]) {
        return Err(Diagnostic::bilingual(
            DiagnosticCode::RepeatCountUnparseable,
            "I couldn't understand how many times to repeat",
            "몇 번 반복할지 이해하지 못했어요",
            span,
        )
        .with_bilingual_hint(
            "write a number, like `repeat 3 times`",
            "`3번 반복해`처럼 횟수를 적어 주세요",
        ));
    }
    Ok(Code::Source(span))
}

/// The digits a counting word stands for: `three` and `세` are both `3`.
fn number_word_digits(word: &str) -> Option<&'static str> {
    debug_assert_eq!(NUMBER_WORDS_EN.len(), NUMBER_VALUES_EN.len());
    debug_assert_eq!(NUMBER_WORDS_KO.len(), NUMBER_VALUES_KO.len());
    if let Some(index) = NUMBER_WORDS_EN
        .iter()
        .position(|candidate| word.eq_ignore_ascii_case(candidate))
    {
        return Some(NUMBER_VALUES_EN[index]);
    }
    NUMBER_WORDS_KO
        .iter()
        .position(|candidate| word == *candidate)
        .map(|index| NUMBER_VALUES_KO[index])
}

/// True when a token can be the number a counter word counts, so `3 times`
/// and `세 번` mark a count while `story time` and `이 판` do not.
fn is_written_number(token: &Token) -> bool {
    matches!(token.tok, Tok::Int { .. } | Tok::Float { .. })
        || name_word(token).is_some_and(|word| number_word_digits(word).is_some())
}

/// A count or a number of seconds written as one word: `세`, `three`, or the
/// Korean spelling with its unit attached, as in `일초`.
fn number_word_code(word: &str, units: &[&str]) -> Option<Code> {
    if let Some(digits) = number_word_digits(word) {
        return Some(Code::Generated(digits.to_string()));
    }
    strip_any_suffix(word, units)
        .and_then(number_word_digits)
        .map(|digits| Code::Generated(digits.to_string()))
}

/// A single word the compiler cannot turn into a number. Left alone it would
/// reach `range()` or `sleep()` as a name nobody set, so it is named in the
/// diagnostic instead of failing at run time with `NameError`.
fn unreadable_number_word<'a>(
    tokens: &'a [Token],
    known_names: &HashSet<String>,
) -> Option<&'a str> {
    let [token] = tokens else {
        return None;
    };
    let word = name_word(token)?;
    (!known_names.contains(word) && number_word_digits(word).is_none()).then_some(word)
}

fn repeat_count_word_diagnostic(word: &str, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::RepeatCountUnparseable,
        "I couldn't understand how many times to repeat",
        "몇 번 반복할지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        format!("`{word}` is not a number; write `repeat 3 times` or `repeat three times`"),
        format!("`{word}`은 숫자가 아니에요. `3번 반복해` 또는 `세 번 반복해`처럼 적어 주세요"),
    )
}

fn repeat_count_missing(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::RepeatCountMissing,
        "the repeat count is missing",
        "반복 횟수가 비어 있어요",
        span,
    )
    .with_bilingual_hint(
        "write `repeat 3 times`",
        "`3번 반복해`처럼 숫자를 함께 적어 주세요",
    )
}

fn find_count_marker(tokens: &[Token], mode: MatchMode) -> Option<(usize, Spelling)> {
    tokens.iter().enumerate().find_map(|(index, token)| {
        if token_word_matches(token, TIMES_KEYWORD, mode) {
            return Some((index, Spelling::English));
        }
        if token_is_exact_name(token, TIMES_KEYWORD_KO) {
            return Some((index, Spelling::Korean));
        }
        // `time`, `round`, `회`, `판` are ordinary nouns as well, so they only
        // mark a count when the number sits right in front of them *and* the
        // repeat word is next to them. That keeps `show Time to sleep` and
        // `최대 10회 되풀이할 수 있어요` ordinary sentences.
        if index == 0 || !is_written_number(&tokens[index - 1]) {
            return None;
        }
        if repeat_action_at(tokens, index + 1, MatchMode::Recover).is_none()
            && repeat_action_at(tokens, 0, MatchMode::Recover).is_none()
        {
            return None;
        }
        if token_matches_exact(token, TIMES_WORDS_EN) {
            Some((index, Spelling::English))
        } else if token_matches_exact(token, TIMES_WORDS_KO) {
            Some((index, Spelling::Korean))
        } else {
            None
        }
    })
}

// --------------------------------------------------------------- modules

/// Sentence-level file and JSON forms, so a beginner can read a file into a
/// name and write text to a file without the `use file` module or Python
/// punctuation. Both languages share the same meaning:
///
/// - `read "notes.txt" into memo` / `memo read "notes.txt"`
/// - `memo에 "notes.txt" 읽어서` / `memo에 "notes.txt" 읽어서 저장해`
/// - `write "hello" to "out.txt"` / `"out.txt" 파일에 "hello"를 저장해`
///
/// The path is always a quoted string; the write value is a beginner value.
/// Weak matches (`read the book`, `write hello`) fall through to plain
/// sentence output instead of being claimed as file operations.
#[allow(clippy::too_many_lines)]
fn match_file_io(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let is_string = |token: &Token| matches!(token.tok, Tok::String { .. });
    let path_of = |tokens: &[Token]| -> Option<Code> {
        let token = tokens.first()?;
        if !is_string(token) {
            return None;
        }
        let span = token.span;
        is_valid_python_expression(&source[span.start..span.end]).then_some(Code::Source(span))
    };

    // English action-first read: `read "notes.txt" into memo`.
    if let Some(consumed) = action_phrase_at(tokens, 0, FILE_READ_WORDS_EN, mode) {
        let Some(path) = path_of(&tokens[consumed..]) else {
            return Ok(None);
        };
        let mut rest = &tokens[consumed + 1..];
        if rest
            .first()
            .is_some_and(|token| token_matches_exact(token, &["into", "as", "in"]))
        {
            rest = &rest[1..];
        }
        if let Some(target) = rest
            .first()
            .and_then(|t| name_word(t))
            .map(strip_saved_target)
        {
            if rest.len() == 1 || (rest.len() == 2 && is_command_ending(&rest[1])) {
                return Ok(Some(NmeStmt::FileRead {
                    target: target.to_string(),
                    path,
                }));
            }
        }
        return Err(file_read_target_diagnostic(span_of(tokens)));
    }

    // Korean read and English/Korean target-first read:
    // `memo에 "notes.txt" 읽어서` / `memo read "notes.txt"`. The path sits
    // before a Korean read word but after the English `read`.
    let ko_read_at = tokens.iter().position(|token| {
        action_phrase_at(std::slice::from_ref(token), 0, FILE_READ_WORDS_KO, mode).is_some()
    });
    let en_read_at = tokens.iter().position(|token| {
        action_phrase_at(std::slice::from_ref(token), 0, FILE_READ_WORDS_EN, mode).is_some()
    });
    if let Some(action_at) = ko_read_at.or(en_read_at) {
        // The name being read into is the first word, so the reading word
        // cannot be the first word as well. Without this the slice below runs
        // backwards and the compiler dies: one Korean noun one edit away from
        // `읽고` — `경고`, `참고`, `보고` — was enough to crash it.
        if action_at == 0 {
            return Ok(None);
        }
        let Some(target) = name_word(&tokens[0]).and_then(update_target_name) else {
            return Ok(None);
        };
        let path_tokens = if ko_read_at.is_some() {
            let mut middle = &tokens[1..action_at];
            if middle.first().is_some_and(|token| {
                is_update_connector(
                    token,
                    &["에", "에서", "에게", "한테", "는", "은", "으로", "로"],
                )
            }) {
                middle = &middle[1..];
            }
            middle
        } else {
            let mut after = &tokens[action_at + 1..];
            if after
                .first()
                .is_some_and(|token| token_matches_exact(token, &["into", "as", "in"]))
            {
                after = &after[1..];
            }
            after
        };
        let Some(path) = path_of(path_tokens) else {
            return Ok(None);
        };
        let tail_ok = if ko_read_at.is_some() {
            let after = &tokens[action_at + 1..];
            after.len() <= 2
                && after.iter().all(|token| {
                    token_matches_exact(token, FILE_WRITE_WORDS_KO) || is_command_ending(token)
                })
        } else {
            let after = &tokens[action_at + 1..];
            path_of(after).is_some()
                && (after.len() == 1 || (after.len() == 2 && is_command_ending(&after[1])))
        };
        if tail_ok {
            return Ok(Some(NmeStmt::FileRead { target, path }));
        }
        return Err(file_read_target_diagnostic(span_of(tokens)));
    }

    // English action-first write: `write "hello" to "out.txt"`.
    if let Some(consumed) = action_phrase_at(tokens, 0, FILE_WRITE_WORDS_EN, mode) {
        let mut end = tokens.len();
        if tokens
            .get(end.saturating_sub(1))
            .is_some_and(is_command_ending)
        {
            end -= 1;
        }
        let Some(to_at) = tokens[consumed..end]
            .iter()
            .position(|token| token_matches_exact(token, &["to", "into"]))
            .map(|at| at + consumed)
        else {
            return Ok(None);
        };
        let value = parse_value(source, &tokens[consumed..to_at], known_names, true)
            .map_err(|()| file_write_diagnostic(span_of(tokens)))?;
        let Some(path) = path_of(&tokens[to_at + 1..end]) else {
            return Err(file_path_diagnostic(span_of(tokens)));
        };
        return Ok(Some(NmeStmt::FileWrite { path, value }));
    }

    // Korean write: `"out.txt" 파일에 "hello"를 저장해`.
    let write_at = tokens.iter().rposition(|token| {
        action_phrase_at(std::slice::from_ref(token), 0, FILE_WRITE_WORDS_KO, mode).is_some()
    });
    if let Some(write_at) = write_at {
        let Some(path) = path_of(&tokens[..1]) else {
            return Ok(None);
        };
        if tokens.get(1).is_some_and(|token| {
            is_update_connector(token, &["파일에", "파일을", "에", "로", "으로", "에다"])
        }) {
            let mut value_tokens = &tokens[2..write_at];
            while value_tokens
                .last()
                .is_some_and(|token| is_update_connector(token, &["을", "를", "만큼"]))
            {
                value_tokens = &value_tokens[..value_tokens.len() - 1];
            }
            if value_tokens.is_empty() {
                return Err(file_write_diagnostic(span_of(tokens)));
            }
            // A Korean particle may be glued to the final word (`점수를`);
            // strip it from the source span so the value stays a Python name.
            let value = if let Some(last) = value_tokens.last() {
                if let Some(word) = name_word(last) {
                    if let Some(stripped) = strip_any_suffix(word, &["을", "를", "만큼"]) {
                        let base = last.span;
                        let end = base.end - (word.len() - stripped.len());
                        Value::Python(Code::Source(Span::new(base.start, end)))
                    } else {
                        parse_value(source, value_tokens, known_names, true)
                            .map_err(|()| file_write_diagnostic(span_of(tokens)))?
                    }
                } else {
                    parse_value(source, value_tokens, known_names, true)
                        .map_err(|()| file_write_diagnostic(span_of(tokens)))?
                }
            } else {
                return Err(file_write_diagnostic(span_of(tokens)));
            };
            return Ok(Some(NmeStmt::FileWrite { path, value }));
        }
        return Err(file_write_diagnostic(span_of(tokens)));
    }

    Ok(None)
}

fn file_read_target_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::MissingAction,
        "the file read needs a target name",
        "파일을 읽어 넣을 이름이 필요해요",
        span,
    )
    .with_bilingual_hint(
        "write `read \"notes.txt\" into memo` or `memo에 \"notes.txt\" 읽어서`",
        "`read \"notes.txt\" into memo` 또는 `memo에 \"notes.txt\" 읽어서`처럼 쓰세요",
    )
}

fn file_write_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::SaveValueUnparseable,
        "I couldn't understand this file write",
        "파일에 저장할 내용을 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `write \"hello\" to \"out.txt\"` or `\"out.txt\" 파일에 \"hello\"를 저장해`",
        "`write \"hello\" to \"out.txt\"` 또는 `\"out.txt\" 파일에 \"hello\"를 저장해`처럼 쓰세요",
    )
}

fn file_path_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::MissingAction,
        "the file name must be a quoted path",
        "파일 이름은 따옴표로 감싼 경로여야 해요",
        span,
    )
    .with_bilingual_hint(
        "write the path in quotes, for example `\"notes.txt\"`",
        "`\"notes.txt\"`처럼 경로를 따옴표 안에 적어 주세요",
    )
}

/// `from "helper.nme" import greet, score` — a beginner module import. The
/// quoted path is not valid Python (`from <string>` is a syntax error), so
/// NME can claim it. The explicit name list is the module interface: only
/// those names cross the file boundary.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn match_module_import(
    source: &str,
    tokens: &[Token],
    _known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if !matches!(tokens.first().map(|token| &token.tok), Some(Tok::From))
        || !matches!(
            tokens.get(1).map(|token| &token.tok),
            Some(Tok::String { .. })
        )
        || !matches!(tokens.get(2).map(|token| &token.tok), Some(Tok::Import))
        || mode != MatchMode::Exact
    {
        return Ok(None);
    }
    let path_span = tokens[1].span;
    let path_text = &source[path_span.start..path_span.end];
    let path_stripped = path_text.trim_matches(['\'', '"']);
    if !path_stripped.ends_with(".nme") {
        return Err(Diagnostic::bilingual(
            DiagnosticCode::MissingAction,
            "a module import path must end in .nme",
            "모듈 경로는 .nme로 끝나야 해요",
            path_span,
        )
        .with_bilingual_hint(
            "write the other program's name in quotes, e.g. `from \"helper.nme\" import greet`",
            "다른 프로그램의 이름을 따옴표로 적으세요. 예: `from \"helper.nme\" import greet`",
        ));
    }
    let stem = path_stripped
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(path_stripped)
        .strip_suffix(".nme")
        .unwrap_or(path_stripped);
    let valid_identifier = !stem.is_empty()
        && stem.chars().enumerate().all(|(index, character)| {
            character == '_'
                || character.is_alphanumeric()
                    && (index > 0 || character.is_alphabetic() || character == '_')
        });
    if !valid_identifier {
        return Err(Diagnostic::bilingual(
            DiagnosticCode::MissingAction,
            "the module file name must be a Python identifier",
            "모듈 파일 이름은 Python 식별자여야 해요",
            path_span,
        )
        .with_bilingual_hint(
            "rename the module with letters, numbers, and underscores only, e.g. `shape_math.nme`",
            "모듈 이름은 문자·숫자·밑줄만 사용하세요. 예: `shape_math.nme`",
        ));
    }
    let mut names = Vec::new();
    let mut index = 3;
    let mut expected = true;
    while index < tokens.len() {
        match &tokens[index].tok {
            Tok::Comma => {
                if expected {
                    return Err(module_import_shape_diagnostic(span_of(tokens)));
                }
                expected = true;
            }
            Tok::Name { name } if expected => {
                names.push(name.clone());
                expected = false;
            }
            _ => return Err(module_import_shape_diagnostic(span_of(tokens))),
        }
        index += 1;
    }
    if expected || names.is_empty() {
        return Err(module_import_shape_diagnostic(span_of(tokens)));
    }
    Ok(Some(NmeStmt::ModuleImport {
        path: Code::Source(path_span),
        names,
    }))
}

/// True for a quoted path that names another NME program.
///
/// This is the whole gate under the sentence spellings below: no ordinary
/// sentence carries a quoted `.nme` path, so the forms can be read before the
/// bundled-module statement without taking `use random` away from it.
fn is_nme_path(source: &str, token: &Token) -> bool {
    matches!(token.tok, Tok::String { .. })
        && source[token.span.start..token.span.end]
            .trim_matches(['\'', '"'])
            .ends_with(".nme")
}

/// `use greet from "helper.nme"` · `"helper.nme"에서 greet 가져와`.
///
/// Both spellings are turned back into the Python-shaped form and handed to
/// the same matcher, so the path rules, the name rules and every diagnostic
/// stay in one place.
fn match_sentence_module_import(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some((path_at, names)) = sentence_import_shape(source, tokens, mode) else {
        return Ok(None);
    };
    let mut rebuilt = Vec::with_capacity(names.len() * 2 + 3);
    rebuilt.push(Token {
        tok: Tok::From,
        span: tokens[0].span,
    });
    rebuilt.push(tokens[path_at].clone());
    rebuilt.push(Token {
        tok: Tok::Import,
        span: tokens[path_at].span,
    });
    for (index, name) in names.iter().enumerate() {
        if index > 0 {
            rebuilt.push(Token {
                tok: Tok::Comma,
                span: name.span,
            });
        }
        rebuilt.push(name.clone());
    }
    match_module_import(source, &rebuilt, known_names, MatchMode::Exact)
}

/// Where the path is, and which names were asked for.
fn sentence_import_shape(
    source: &str,
    tokens: &[Token],
    mode: MatchMode,
) -> Option<(usize, Vec<Token>)> {
    // English says the action first: `use greet from "helper.nme"`.
    if let Some(consumed) = action_phrase_at(tokens, 0, NME_IMPORT_WORDS_EN, mode) {
        let from_at = tokens
            .iter()
            .position(|token| matches!(token.tok, Tok::From))?;
        if from_at <= consumed || from_at + 2 != tokens.len() {
            return None;
        }
        if !is_nme_path(source, tokens.get(from_at + 1)?) {
            return None;
        }
        return Some((from_at + 1, import_names(&tokens[consumed..from_at])?));
    }
    // Korean says the file first and the action last:
    // `"helper.nme"에서 greet 가져와`.
    if !is_nme_path(source, tokens.first()?) {
        return None;
    }
    let mut start = 1;
    if tokens
        .get(start)
        .is_some_and(|token| token_matches_exact(token, &["에서", "에서는", "의", "에"]))
    {
        start += 1;
    }
    let end = trim_command_endings(tokens).len();
    let action_start = (start..end)
        .find(|&at| {
            action_phrase_at(tokens, at, NME_IMPORT_WORDS_KO, mode)
                .is_some_and(|used| at + used == end)
        })
        .filter(|&at| at > start)?;
    Some((0, import_names(&tokens[start..action_start])?))
}

/// The comma-separated names between the action word and the path.
fn import_names(tokens: &[Token]) -> Option<Vec<Token>> {
    let mut names = Vec::new();
    let mut expected = true;
    for token in tokens {
        match &token.tok {
            Tok::Comma if !expected => expected = true,
            Tok::Name { .. } if expected => {
                names.push(token.clone());
                expected = false;
            }
            _ => return None,
        }
    }
    (!expected && !names.is_empty()).then_some(names)
}

fn module_import_shape_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::MissingAction,
        "I couldn't understand this module import",
        "모듈 가져오기 문장을 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `from \"helper.nme\" import greet` with simple names after `import`",
        "`from \"helper.nme\" import greet`처럼 import 뒤에 간단한 이름을 적으세요",
    )
}

#[allow(clippy::too_many_lines)]
fn match_use_module(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some((action_start, action_end, spelling)) = find_use_action(tokens, mode) else {
        return Ok(None);
    };

    let mut module = None;
    for candidate in BundledModuleId::ALL {
        let positions = tokens
            .iter()
            .enumerate()
            .filter_map(|(index, token)| {
                module_word_matches(token, candidate, mode).then_some(index)
            })
            .collect::<Vec<_>>();
        match positions.as_slice() {
            [] => {}
            [single] => {
                if module.is_some() {
                    return Err(unsupported_module_diagnostic(span_of(tokens)));
                }
                module = Some((candidate, *single));
            }
            _ => return Err(unsupported_module_diagnostic(span_of(tokens))),
        }
    }
    let Some((module, module_at)) = module else {
        if find_use_action(tokens, MatchMode::Exact).is_none() {
            // The action word was only a guess — `set` is one letter from
            // `get` — and nothing on the line names a module. Then this is
            // not a module line, and `set the table for four people` must not
            // be answered with the list of modules NME bundles.
            return Ok(None);
        }
        // `사용`, `가져와` and `받아` are ordinary Korean words as well.
        // `사용 설명서를 잃어버렸습니다` is a lost manual and
        // `물을 가져와 마셨습니다` is a drink of water; neither names a module,
        // and both were answered with the list of modules NME bundles. A line
        // that asks something is left alone: `이름을 받아 이름이 뭐예요?` is a
        // question with the wrong word for asking, and saying so is right.
        if is_written_korean_sentence(tokens, known_names) && !line_asks_a_question(tokens) {
            return Ok(None);
        }
        return Err(unsupported_module_diagnostic(span_of(tokens)));
    };

    // `list`, `text`, `math` and `date` are words people write in ordinary
    // sentences, so they name a module only when they stand beside the
    // `use`/`사용` word. Without this, `get the list of names` was answered
    // with the list of modules NME bundles instead of being printed, and
    // `이 날짜 사용법을 알려 주세요` would be a module line rather than a
    // question.
    if module.name_is_an_ordinary_word()
        && !module_touches_the_action(tokens, action_start, action_end, module_at)
    {
        return Ok(None);
    }

    let latest_positions = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| word_matches_any(token, LATEST_WORDS, mode).then_some(index))
        .collect::<Vec<_>>();
    let version_positions = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            word_matches_any(token, &["version", "버전"], mode).then_some(index)
        })
        .collect::<Vec<_>>();
    if !latest_positions.is_empty() && !version_positions.is_empty() {
        return Err(Diagnostic::bilingual(
            DiagnosticCode::LatestAndVersion,
            "choose either latest or one exact module version",
            "최신 버전과 특정 버전 중 하나만 골라 주세요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            format!(
                "write `use {} latest` or `use {} version {}`",
                module.name_en(),
                module.name_en(),
                module.version()
            ),
            format!(
                "`{} 사용 최신` 또는 `{} 사용 버전 {}`처럼 쓰세요",
                module.name_ko(),
                module.name_ko(),
                module.version()
            ),
        ));
    }
    if latest_positions.len() > 1 || version_positions.len() > 1 {
        return Err(module_shape_diagnostic(spelling, span_of(tokens)));
    }

    let mut used = vec![false; tokens.len()];
    for slot in &mut used[action_start..action_end] {
        *slot = true;
    }
    used[module_at] = true;
    for &index in &latest_positions {
        used[index] = true;
    }

    let requested = if !latest_positions.is_empty() {
        ModuleVersion::Latest
    } else if let Some(&version_at) = version_positions.first() {
        if version_at < action_end.max(module_at + 1) {
            return Err(module_shape_diagnostic(spelling, tokens[version_at].span));
        }
        used[version_at] = true;
        let mut value_end = tokens.len();
        if tokens.last().is_some_and(is_command_ending) {
            value_end -= 1;
            used[value_end] = true;
        }
        let value_tokens = tokens.get(version_at + 1..value_end).ok_or_else(|| {
            Diagnostic::bilingual(
                DiagnosticCode::ModuleVersionMissing,
                "the module version is missing",
                "모듈 버전이 비어 있어요",
                tokens[version_at].span,
            )
            .with_bilingual_hint(
                format!("use `latest`, or version {}", module.version()),
                format!("`최신` 또는 버전 {}을 사용하세요", module.version()),
            )
        })?;
        if value_tokens.is_empty() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::ModuleVersionMissing,
                "the module version is missing",
                "모듈 버전이 비어 있어요",
                tokens[version_at].span,
            )
            .with_bilingual_hint(
                format!("use `latest`, or version {}", module.version()),
                format!("`최신` 또는 버전 {}을 사용하세요", module.version()),
            ));
        }
        for slot in &mut used[version_at + 1..value_end] {
            *slot = true;
        }
        let value_span = span_of(value_tokens);
        let raw = &source[value_span.start..value_span.end];
        let version = raw.trim_matches(['\'', '"']).to_string();
        if version != module.version() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::UnbundledVersion,
                format!("{} version {version} is not bundled", module.name_en()),
                format!("{} 버전 {version}은 내장되어 있지 않아요", module.name_ko()),
                value_span,
            )
            .with_bilingual_hint(
                format!("use `latest`; this compiler bundles {}", module.version()),
                format!(
                    "`최신`을 사용하세요. 이 컴파일러에는 {}이 들어 있어요",
                    module.version()
                ),
            ));
        }
        ModuleVersion::Exact(version)
    } else {
        ModuleVersion::Bundled
    };

    for (index, token) in tokens.iter().enumerate() {
        if used[index]
            || token_matches_exact(token, &["please", "the", "module", "모듈", "모듈을", "좀"])
            || is_command_ending(token)
        {
            continue;
        }
        // A word left over on a `random`, `file` or `zero_knowledge` line is a
        // module line written wrongly, and saying so is worth a bad minute.
        // A word left over beside `list`, `text`, `math` or `date` is far more
        // likely to be the sentence those words belong to — `I use text
        // messages every day` — so the line is handed back unclaimed.
        if module.name_is_an_ordinary_word() {
            return Ok(None);
        }
        return Err(module_shape_diagnostic(spelling, token.span));
    }

    let collisions = module_binding_names(module)
        .iter()
        .filter(|name| known_names.contains(**name))
        .copied()
        .collect::<Vec<_>>();
    if !collisions.is_empty() {
        return Err(module_name_collision_diagnostic(
            module,
            span_of(tokens),
            &collisions,
        ));
    }

    Ok(Some(NmeStmt::UseModule { module, requested }))
}

/// Names a bundled module would bind, so a later `use` can refuse to
/// overwrite an existing value.
fn module_binding_names(module: BundledModuleId) -> &'static [&'static str] {
    match module {
        BundledModuleId::Random => &[
            RANDOM_MODULE,
            RANDOM_MODULE_KO,
            "random_number",
            "random_pick",
            "shuffle",
            "랜덤정수",
            "랜덤선택",
            "섞기",
            "random_version",
            "랜덤버전",
        ],
        BundledModuleId::File => &[
            FILE_MODULE,
            FILE_MODULE_KO,
            "file_read",
            "file_write",
            "json_load",
            "json_save",
            "파일읽기",
            "파일쓰기",
            "json읽기",
            "json저장",
            "file_version",
            "파일버전",
        ],
        BundledModuleId::ZeroKnowledge => &[
            "영지식비밀난수",
            "zk_prime",
            "영지식큰소수",
            "zk_order",
            "영지식부분군크기",
            "zk_generator",
            "영지식생성원",
            "zk_challenge_bits",
            "영지식도전비트",
            "zk_challenge_limit",
            "영지식도전범위",
            "zk_secret",
            "영지식비밀만들기",
            "zk_public",
            "영지식공개값",
            "zk_nonce",
            "영지식일회값만들기",
            "zk_commitment",
            "영지식약속",
            "zk_challenge",
            "영지식도전만들기",
            "zk_challenge_except",
            "영지식다른도전",
            "zk_response",
            "영지식응답",
            "zk_verify",
            "영지식검증",
            "zk_simulated_response",
            "영지식모의응답만들기",
            "zk_simulated_commitment",
            "영지식모의약속",
            "zk_group_bytes",
            "영지식그룹바이트",
            "_nme_zk_context_bytes",
            "_nme_zk_int_bytes",
            "_nme_zk_context_frame",
            "zk_nizk_challenge",
            "영지식비대화도전",
            "zk_nizk_prove",
            "영지식비대화증명",
            "zk_nizk_verify",
            "영지식비대화검증",
            "zero_knowledge_version",
            "영지식버전",
        ],
        BundledModuleId::List => &[
            "count",
            "개수",
            "sort",
            "정렬",
            "reverse",
            "뒤집기",
            "remove",
            "빼기",
            "first",
            "첫번째",
            "last",
            "마지막",
            "sum",
            "합계",
            "largest",
            "최대",
            "smallest",
            "최소",
            "list_version",
            "목록버전",
        ],
        BundledModuleId::Text => &[
            "upper",
            "대문자",
            "lower",
            "소문자",
            "trim",
            "공백없애기",
            "split",
            "나누기",
            "join",
            "합치기",
            "replace",
            "바꾸기",
            "starts_with",
            "로시작",
            "length",
            "길이",
            "text_version",
            "글자버전",
        ],
        BundledModuleId::Math => &[
            MATH_MODULE,
            MATH_MODULE_KO,
            "root",
            "제곱근",
            "round_to",
            "반올림",
            "pi",
            "원주율",
            "power",
            "거듭제곱",
            "absolute",
            "절댓값",
            "floor",
            "내림",
            "ceil",
            "올림",
            "math_version",
            "수학버전",
        ],
        BundledModuleId::Date => &[
            // `date` and `날짜` themselves are not here, because the adapter
            // does not bind them: a program that already keeps its own `date`
            // must still be able to write `use date`.
            "날짜모듈",
            "today",
            "오늘",
            "now",
            "지금",
            "year",
            "올해",
            "month",
            "이번달",
            "day_of_month",
            "오늘일자",
            "weekday",
            "요일",
            "days_after",
            "며칠뒤",
            "date_version",
            "날짜버전",
        ],
    }
}

fn module_name_collision_diagnostic(
    module: BundledModuleId,
    span: Span,
    collisions: &[&str],
) -> Diagnostic {
    let names = collisions.join(", ");
    Diagnostic::bilingual(
        DiagnosticCode::ModuleNameCollision,
        format!(
            "the {} module would overwrite existing name(s): {names}",
            module.name_en()
        ),
        format!(
            "{} 모듈이 이미 있는 이름을 덮어쓸 수 있어요: {names}",
            module.name_ko()
        ),
        span,
    )
    .with_bilingual_hint(
        "rename the existing value, or load the module before assigning that name",
        "기존 값을 다른 이름으로 바꾸거나, 그 이름을 쓰기 전에 모듈을 불러오세요",
    )
}

fn module_word_matches(token: &Token, module: BundledModuleId, mode: MatchMode) -> bool {
    // `list` is one edit from `last`, `text` from `next`, `math` from `path`,
    // and `date` from `data`, `late` and `gate`. Repairing a typo into one of
    // those names would let an ordinary sentence name a module, so the four
    // ordinary names are matched exactly and only the rarer ones earn a
    // one-edit repair.
    let mode = if module.name_is_an_ordinary_word() {
        MatchMode::Exact
    } else {
        mode
    };
    name_word(token).is_some_and(|word| {
        word_matches(word, module.name_en(), mode)
            || (module == BundledModuleId::ZeroKnowledge
                && word_matches(word, "zeroknowledge", mode))
            || word == module.name_ko()
            || strip_target_particle(word) == module.name_ko()
    })
}

/// True when the module name stands directly beside the `use`/`사용` word.
///
/// English writes `use list`, Korean writes `목록 사용`, and either language
/// may put `latest` between the two (`use latest list`, `최신 목록 사용`).
/// Anything further apart — `get the list of names`, `use the text I sent` —
/// has a word in between that a module line never has, and is a sentence.
fn module_touches_the_action(
    tokens: &[Token],
    action_start: usize,
    action_end: usize,
    module_at: usize,
) -> bool {
    let after_action = |at: usize| {
        at == action_end
            || (at == action_end + 1
                && tokens
                    .get(action_end)
                    .is_some_and(|token| word_matches_any(token, LATEST_WORDS, MatchMode::Exact)))
    };
    let before_action = |at: usize| {
        at + 1 == action_start
            || (at + 2 == action_start
                && tokens
                    .get(at + 1)
                    .is_some_and(|token| word_matches_any(token, LATEST_WORDS, MatchMode::Exact)))
    };
    after_action(module_at) || before_action(module_at)
}

fn unsupported_module_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "NME bundles `use random`, `use file`, `use list`, `use text`, `use math`, `use date`, and `use zero_knowledge`",
        "NME에는 쉬운 `랜덤`, `파일`, `목록`, `글자`, `수학`, `날짜`, `영지식` 모듈이 들어 있어요",
        span,
    )
    .with_bilingual_hint(
        "write one module line such as `use random latest`, `use list latest`, or `use math latest`",
        "`랜덤 사용 최신`, `목록 사용 최신`, `수학 사용 최신`처럼 한 줄로 적어 주세요",
    )
}

/// Where a module line states its action.
///
/// English states it first and nowhere else. Reading it anywhere on the line
/// meant that `At the end of the use we turned back.` and `We use 2 spoons of
/// salt.` were answered with a list of the modules NME bundles — `use` was in
/// the sentence, so the sentence became a module line. Korean writes the
/// action after the module (`랜덤 사용 최신`), so Korean is still searched
/// across the line.
fn find_use_action(tokens: &[Token], mode: MatchMode) -> Option<(usize, usize, Spelling)> {
    let english_start = leading_sentence_fillers(tokens);
    if let Some(consumed) = action_phrase_at(tokens, english_start, USE_WORDS_EN, mode) {
        return Some((english_start, english_start + consumed, Spelling::English));
    }
    // Elsewhere on the line, an English `use` word only counts when a module
    // is named beside it: `never use random` is a module line written back to
    // front, and `We use 2 spoons of salt.` is a sentence.
    let names_a_module = tokens.iter().any(|token| {
        BundledModuleId::ALL
            .iter()
            .any(|module| module_word_matches(token, *module, MatchMode::Exact))
    });
    for start in 0..tokens.len() {
        if names_a_module {
            if let Some(consumed) = action_phrase_at(tokens, start, USE_WORDS_EN, mode) {
                return Some((start, start + consumed, Spelling::English));
            }
        }
        if let Some(consumed) = action_phrase_at(tokens, start, USE_WORDS_KO, mode) {
            return Some((start, start + consumed, Spelling::Korean));
        }
    }
    None
}

fn recoverable_module_shape(tokens: &[Token]) -> bool {
    let recovered_action = find_use_action(tokens, MatchMode::Recover);
    let action_recovered =
        find_use_action(tokens, MatchMode::Exact).is_none() && recovered_action.is_some();
    // `list`, `text`, `math` and `date` are ordinary words. On a real module
    // line the name stands beside the `use`/`사용` word, which is the rule
    // `match_use_module` already applies; anywhere else the word belongs to
    // the sentence it came from. Counting it wherever it appeared made
    // `No date has been set for the repairs.` a mistyped module line — `set`
    // is one edit from `get` — which took the line off the prose path and
    // answered it with `the repeat count is missing`.
    let names_this_module = |token: &Token, at: usize, module: BundledModuleId, mode: MatchMode| {
        module_word_matches(token, module, mode)
            && (!module.name_is_an_ordinary_word()
                || recovered_action.is_some_and(|(start, end, _)| {
                    module_touches_the_action(tokens, start, end, at)
                }))
    };
    let module_names = |mode: MatchMode| {
        tokens
            .iter()
            .enumerate()
            .filter(|(at, token)| {
                BundledModuleId::ALL
                    .iter()
                    .any(|module| names_this_module(token, *at, *module, mode))
            })
            .count()
    };
    let module_exact = module_names(MatchMode::Exact);
    let module_recovered = module_names(MatchMode::Recover);
    let exact_latest = tokens
        .iter()
        .filter(|token| word_matches_any(token, LATEST_WORDS, MatchMode::Exact))
        .count();
    let recovered_latest = tokens
        .iter()
        .filter(|token| word_matches_any(token, LATEST_WORDS, MatchMode::Recover))
        .count();
    let exact_version = tokens
        .iter()
        .filter(|token| word_matches_any(token, &["version", "버전"], MatchMode::Exact))
        .count();
    let recovered_version = tokens
        .iter()
        .filter(|token| word_matches_any(token, &["version", "버전"], MatchMode::Recover))
        .count();

    // A module line always names its module. Without that, one ordinary word
    // one edit from `use`/`load`/`get` claimed the whole line: `end of the
    // road` was told to choose a module version because `road` is one edit
    // from `load`, and `Are you coming with us?` because `us` is one edit
    // from `use`. A misspelling only starts a module line when a module is
    // actually on it.
    let names_a_module = module_exact + module_recovered > 0;
    names_a_module
        && (action_recovered
            || (module_exact == 0 && module_recovered == 1)
            || (exact_latest == 0 && recovered_latest == 1)
            || (exact_version == 0 && recovered_version == 1))
}

fn module_shape_diagnostic(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ModuleShapeInvalid,
        "I couldn't understand this module line",
        "이 모듈 문장을 확실하게 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `use random latest`, `use file latest`, or `use zero_knowledge latest`, with an optional version",
        "`랜덤 사용 최신`, `파일 사용 최신`, `영지식 사용 최신`처럼 쓰고, 원하면 버전을 붙이세요",
    )
}

// ------------------------------------------------------------ assignment

/// `점수는 0입니다` — a number spoken as a whole sentence. The ending stands
/// as its own word there, because Python's lexer cuts a number away from the
/// Hangul after it, and what is left in front of the ending is the value.
///
/// `할인율은 30%입니다` is not one of these: a `%` follows the number, so the
/// number is not the whole value and the line is a sentence about a discount.
fn korean_sentence_saves_one_number(tokens: &[Token]) -> bool {
    let line = without_trailing_marks(tokens);
    let [.., before, last] = line else {
        return false;
    };
    name_word(last).is_some_and(|word| SENTENCE_ENDINGS_KO.contains(&word))
        && matches!(before.tok, Tok::Int { .. } | Tok::Float { .. })
}

/// True when a line ends the way a written Korean sentence ends and is
/// therefore not a name being given a value, whichever shape of assignment it
/// also resembles.
///
/// The guard itself is old; what was new on 2026-08-19 is asking it on every
/// path instead of only after a `은`/`는`. `저장 지점에 도착했습니다` became
/// `지점 = "도착했습니다"`, `극장 화면이 아주 컸습니다` became
/// `화면이 = "아주 컸습니다"` — a program that runs, prints nothing and says
/// nothing — because a repaired saving word claimed the line before the
/// sentence rule was ever consulted.
fn korean_line_is_a_sentence(tokens: &[Token], known_names: &HashSet<String>) -> bool {
    is_written_korean_sentence(tokens, known_names) && !korean_sentence_saves_one_number(tokens)
}

#[allow(clippy::too_many_lines)]
fn match_set(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // A repaired saving word is a guess, and a line that ends the way a
    // written Korean sentence ends beats a guess: `극장 화면이 아주 컸습니다`
    // put `극장` one character from `저장`, and `설정을 초기화하시겠습니까?`
    // put `설정을` one character from `설정`. Both became a name holding text,
    // and both are sentences.
    if mode == MatchMode::Recover && korean_line_is_a_sentence(tokens, known_names) {
        return Ok(None);
    }
    // A spoken target-first form is often the first bridge from plain
    // sentences to assignments: `이름 저장 민수` / `name save Mina`.  Keep it
    // deliberately strict (the save word must be the second token) so normal
    // prose is not silently turned into a variable assignment.
    // English spells this one exactly. `have` is one letter from `save`, and
    // with a repair allowed here `I have 3 apples` quietly became a value
    // called `I` holding the text `3 apples`. Korean marks its target with a
    // particle, so a repair there still has something to lean on.
    let target_first_spelling = set_action_at(tokens, 1, mode).map(|(spelling, _)| spelling);
    let target_first_is_written_out = target_first_spelling != Some(Spelling::English)
        || set_action_at(tokens, 1, MatchMode::Exact).is_some();
    if tokens.len() >= 2
        && name_word(&tokens[0]).is_some()
        && set_action_at(tokens, 1, mode).is_some()
        && target_first_is_written_out
    {
        let target_token = &tokens[0];
        let target = strip_saved_target(name_word(target_token).expect("checked name token"));
        let Some((_, consumed)) = set_action_at(tokens, 1, mode) else {
            unreachable!("set action was checked above");
        };
        let mut value_start = 1 + consumed;
        if tokens.get(value_start).is_some_and(|token| {
            token_matches_exact(
                token,
                &["to", "as", "is", "로", "으로", "을", "를", "은", "는", "에"],
            )
        }) {
            value_start += 1;
        }
        if value_start >= tokens.len() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::SaveValueMissing,
                "the value to save is missing",
                "저장할 값이 비어 있어요",
                target_token.span,
            )
            .with_bilingual_hint(
                "write `name save Mina` or `이름 저장 민수`",
                "`이름 저장 민수` 또는 `name save Mina`처럼 값을 뒤에 적어 주세요",
            ));
        }
        // `PDF로 저장해 두었습니다` is not a name holding `두었습니다`. Every
        // shape of assignment asks this, not only the one after a `은`/`는`.
        if korean_value_is_a_sentence(&tokens[value_start..]) {
            return Ok(None);
        }
        let value = set_value(source, &tokens[value_start..], known_names).map_err(|()| {
            Diagnostic::bilingual(
                DiagnosticCode::SaveValueUnparseable,
                "I couldn't understand the value to save",
                "저장할 값을 이해하지 못했어요",
                span_of(&tokens[value_start..]),
            )
            .with_bilingual_hint(
                "write a number, name, or plain sentence",
                "숫자, 이름, 또는 평범한 문장을 적어 주세요",
            )
        })?;
        return Ok(Some(NmeStmt::Set {
            target: target.to_string(),
            value,
        }));
    }

    if let Some(stmt) = english_make_set(source, tokens, known_names)? {
        return Ok(Some(stmt));
    }

    if let Some(first) = name_word(&tokens[0]) {
        if let Some(target) = strip_assignment_particle(first) {
            if korean_value_is_a_sentence(&tokens[1..]) || is_labelled_phrase(source, &tokens[1..])
            {
                return Ok(None);
            }
            if korean_quotative_noun_phrase(first, &tokens[1..], known_names) {
                return Ok(None);
            }
            if tokens.len() == 1 {
                return Err(Diagnostic::bilingual(
                    DiagnosticCode::SaveValueMissing,
                    "the value to save is missing",
                    "저장할 값이 비어 있어요",
                    tokens[0].span,
                )
                .with_bilingual_hint(
                    "write a value after the name",
                    "`인사는 안녕하세요`처럼 값을 뒤에 적어 주세요",
                ));
            }
            let value = set_value(source, &tokens[1..], known_names).map_err(|()| {
                Diagnostic::bilingual(
                    DiagnosticCode::SaveValueUnparseable,
                    "I couldn't understand the value to save",
                    "저장할 값을 이해하지 못했어요",
                    span_of(&tokens[1..]),
                )
                .with_bilingual_hint(
                    "write a number, name, or plain sentence",
                    "숫자, 이름, 또는 평범한 문장을 적어 주세요",
                )
            })?;
            return Ok(Some(NmeStmt::Set {
                target: target.to_string(),
                value,
            }));
        }
    }

    if tokens.len() >= 3
        && name_word(&tokens[0]).is_some()
        && token_matches_exact(&tokens[1], &["은", "는"])
        && !korean_value_is_a_sentence(&tokens[2..])
        && !is_labelled_phrase(source, &tokens[2..])
    {
        let target = name_word(&tokens[0]).expect("checked name token");
        let value = set_value(source, &tokens[2..], known_names).map_err(|()| {
            Diagnostic::bilingual(
                DiagnosticCode::SaveValueUnparseable,
                "I couldn't understand the value to save",
                "저장할 값을 이해하지 못했어요",
                span_of(&tokens[2..]),
            )
            .with_bilingual_hint(
                "write a value after the name",
                "`인사 는 안녕하세요`처럼 값을 뒤에 적어 주세요",
            )
        })?;
        return Ok(Some(NmeStmt::Set {
            target: target.to_string(),
            value,
        }));
    }

    if tokens.len() >= 2 {
        if let Some(stmt) = korean_target_first_set(source, tokens, known_names, mode)? {
            return Ok(Some(stmt));
        }
    }

    if let Some((spelling, consumed)) = set_action_at(tokens, 0, mode) {
        let Some(target_token) = tokens.get(consumed) else {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::SaveNameMissing,
                "the name to save is missing",
                "값을 저장할 이름이 비어 있어요",
                tokens[0].span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello` or `저장 인사 Hello`",
                "`저장 인사 안녕하세요` 또는 `set greeting to Hello`처럼 쓰세요",
            ));
        };
        let Some(target_word) = name_word(target_token) else {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::SaveNameNotSimple,
                "use a simple name here",
                "여기에는 간단한 이름을 써 주세요",
                target_token.span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello` or `저장 인사 Hello`",
                "`저장 인사 안녕하세요` 또는 `set greeting to Hello`처럼 쓰세요",
            ));
        };
        let marked_by_a_connector = tokens
            .get(consumed + 1)
            .is_some_and(|token| token_matches_exact(token, &["to", "as", "is", "into"]));
        if spelling == Spelling::English
            && !marked_by_a_connector
            && !is_bindable_english_name(target_word)
        {
            // `set the table for four people` — see `NOT_A_NAME_EN`. A `to`
            // after the name says a name was meant, whatever the word is:
            // `set then to 1` really does make a value called `then`, and
            // converting Python back into sentences writes exactly that.
            return Ok(None);
        }
        let target = if spelling == Spelling::Korean {
            strip_saved_target(target_word)
        } else {
            target_word
        };
        let mut value_start = consumed + 1;
        if tokens.get(value_start).is_some_and(|token| {
            token_matches_exact(
                token,
                &["to", "as", "is", "로", "으로", "을", "를", "은", "는", "에"],
            )
        }) {
            value_start += 1;
        }
        if let Some(problem) = broken_set_connector(source, target, &tokens[value_start..]) {
            return Err(problem);
        }
        if value_start >= tokens.len() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::SaveValueMissing,
                "the value to save is missing",
                "저장할 값이 비어 있어요",
                target_token.span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello` or `저장 인사 Hello`",
                "`저장 인사 안녕하세요` 또는 `set greeting to Hello`처럼 쓰세요",
            ));
        }
        // `저장 지점에 도착했습니다` is a place somebody arrived at, not a name
        // called `지점` holding `도착했습니다`. `저장 인사 안녕하세요` still
        // saves the greeting: one word is a value, not a sentence.
        if korean_value_is_a_sentence(&tokens[value_start..]) {
            return Ok(None);
        }
        let value = set_value(source, &tokens[value_start..], known_names).map_err(|()| {
            Diagnostic::bilingual(
                DiagnosticCode::SaveValueUnparseable,
                "I couldn't understand the value to save",
                "저장할 값을 이해하지 못했어요",
                span_of(&tokens[value_start..]),
            )
            .with_bilingual_hint(
                "write a number, name, or plain sentence",
                "숫자, 이름, 또는 평범한 문장을 적어 주세요",
            )
        })?;
        return Ok(Some(NmeStmt::Set {
            target: target.to_string(),
            value,
        }));
    }
    Ok(None)
}

/// `name becomes 5` and `call it name 5` — saving a value with the everyday
/// verb instead of a saving word, which is what a beginner reaches for when
/// `set` has not been read yet.
///
/// Both are written exactly; neither is ever repaired from a misspelling.
/// The name has to be a word a sentence may turn into a name, which is what
/// keeps `Call it a day.` and `Call it what you like.` sentences: `a` and
/// `what` are in [`NOT_A_NAME_EN`].
fn english_make_set(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let body = trim_command_endings(tokens);
    let (target_at, value_at) = if body.len() >= 4
        && token_matches_exact(&body[0], SET_MAKE_WORDS_EN)
        && token_matches_exact(&body[1], &["it"])
    {
        // `call it name 5` — the name after the `it` the sentence starts with.
        (2, 3)
    } else if body.len() >= 3 && token_matches_exact(&body[1], SET_MAKE_WORDS_EN) {
        // `name becomes 5` — the name first, the way English says it.
        (0, 2)
    } else {
        return Ok(None);
    };
    let Some(target) = name_word(&body[target_at]).filter(|word| is_bindable_english_name(word))
    else {
        return Ok(None);
    };
    let value_tokens = &body[value_at..];
    // `Water becomes ice at zero degrees.` and `Winter becomes spring.` are
    // sentences. `becomes` therefore takes a value that could not be part of
    // one: a number, `true`/`false`/`none`, or a name the program already
    // made. Text is saved with `set greeting to Hello`, which says so.
    if target_at == 0
        && !(number_value_code(source, value_tokens).is_some()
            || (value_tokens.len() == 1
                && (literal_token(&value_tokens[0]).is_some()
                    || name_word(&value_tokens[0])
                        .is_some_and(|word| known_names.contains(word)))))
    {
        return Ok(None);
    }
    let value = set_value(source, value_tokens, known_names).map_err(|()| {
        Diagnostic::bilingual(
            DiagnosticCode::SaveValueUnparseable,
            "I couldn't understand the value to save",
            "저장할 값을 이해하지 못했어요",
            span_of(value_tokens),
        )
        .with_bilingual_hint(
            "write a number, name, or plain sentence",
            "숫자, 이름, 또는 평범한 문장을 적어 주세요",
        )
    })?;
    Ok(Some(NmeStmt::Set {
        target: target.to_string(),
        value,
    }))
}

/// The number a value region holds once the sentence's own tail is taken off:
/// a full stop, semicolon, comma, or exclamation mark the writer ended with,
/// and the Korean endings `이다`/`입니다`/`이에요`/`예요`/`으로`/`로`.
///
/// Only a number is recovered this way. `인사는 안녕하세요` ends in `예요` too,
/// and it is text that must survive exactly as written.
/// `set score = 0` and `set score t 0` would otherwise save the text `"= 0"`
/// or `"t 0"`, and the program would fail much later with a `TypeError` a
/// long way from the line that caused it. Both shapes are refused instead,
/// with the line spelled out the way NME reads it.
///
/// Only two shapes are claimed, so `set greeting Hello world` still saves the
/// sentence: an `=` (or any other operator) standing where the connector
/// belongs, and a one-edit misspelling of a connector directly in front of a
/// number.
fn broken_set_connector(source: &str, target: &str, value: &[Token]) -> Option<Diagnostic> {
    const CONNECTORS: &[&str] = &["to", "as", "is", "into"];
    let first = value.first()?;
    let operator = matches!(first.tok, Tok::Equal | Tok::EqEqual | Tok::ColonEqual);
    let misspelled_connector = value.len() == 2
        && matches!(value[1].tok, Tok::Int { .. } | Tok::Float { .. })
        && token_word(first).is_some_and(|word| {
            !CONNECTORS
                .iter()
                .any(|known| word.eq_ignore_ascii_case(known))
                && CONNECTORS.iter().any(|known| one_typo_away(word, known))
        });
    if !operator && !misspelled_connector {
        return None;
    }
    let rest = &value[1..];
    let written = if rest.is_empty() {
        String::new()
    } else {
        source[span_of(rest).start..span_of(rest).end].to_string()
    };
    Some(
        Diagnostic::bilingual(
            DiagnosticCode::SaveValueUnparseable,
            "I couldn't understand the value to save",
            "저장할 값을 이해하지 못했어요",
            span_of(value),
        )
        .with_bilingual_hint(
            format!("write `set {target} to {written}`"),
            format!("`{target}는 {written}`처럼 적어 주세요"),
        ),
    )
}

/// Endings that make a Korean phrase a whole sentence rather than a value.
/// `점수는 0입니다` still saves zero, because what is left after the ending is
/// a number; `좋은 아침입니다` leaves `아침`, which is a word, so the line is
/// prose and prints itself.
const SENTENCE_ENDINGS_KO: &[&str] = &[
    "이었습니다",
    "였습니다",
    "습니다",
    "읍니다",
    "입니다",
    // `확신합니다`, `동의합니다`, `할인합니다` — the ending most Korean
    // sentences about doing something end in.
    "합니다",
    "됩니다",
    "이에요",
    "이었다",
    "였다",
    "예요",
    "해요",
    "었다",
    "았다",
    "이다",
    // The `-ㅂ니다` family, which is most of written Korean and was missing
    // whole: `모릅니다`, `마십니다`, `나옵니다`, `납니다`, `살이십니다`. It goes
    // last so that the longer endings above are still the ones found first,
    // which is what keeps `정답은 7입니다` saving the number seven.
    "니다",
    // `만납시다`, `갑시다` — the "let us" ending.
    "시다",
    // A question: `초기화하시겠습니까`, `어디예요`.
    "니까",
    // The polite `-요` endings — see `POLITE_ENDINGS_KO` for the one extra
    // condition they carry.
    "세요",
    "네요",
    "군요",
    "데요",
    "거든요",
    "어요",
    "아요",
    "죠",
    // `됐었는데`, `좋았는데` — a sentence left hanging is still a sentence.
    "는데",
    "구나",
];

/// The ending above that is also how a single polite *word* is made.
/// `안녕하세요` is a greeting somebody saves under a name; `주세요` is a whole
/// request. So it counts as the end of a sentence only when a word stands in
/// front of it — `인사는 안녕하세요` saves a greeting, and
/// `게임을 시작하려면 아무 키나 누르세요` is an instruction printed as written.
/// The other `-요` endings are never a name for anything, so `지금은 괜찮아요`
/// is a sentence however short it is.
const POLITE_ENDINGS_KO: &[&str] = &["세요"];

/// The sentence ending `word` carries, if it carries one. `words_in_front` is
/// how many words stand before it in whatever is being judged — a whole line,
/// or the value after a name — and only the polite endings look at it.
fn korean_sentence_ending_of(word: &str, words_in_front: usize) -> Option<&'static str> {
    let ending = SENTENCE_ENDINGS_KO
        .iter()
        .find(|ending| word.ends_with(*ending))?;
    if words_in_front == 0 && POLITE_ENDINGS_KO.contains(ending) {
        return None;
    }
    Some(ending)
}

/// True when what follows a Korean `은`/`는` is a sentence, not a value.
/// True for a line that ends the way a written Korean sentence ends, even
/// when it carries digits or a `%`.
///
/// `100% 확신합니다` is a valid Python expression — a number modulo a name —
/// so without this the writer is handed a `NameError` at run time instead of
/// their own sentence. `전체의 30%가 왔습니다` is not Python at all and would
/// reach CPython as a syntax error. Both are sentences, and both print.
/// A line of words with one `:` in it and nothing that could be code.
///
/// `제목: 오늘 할 일` and `재미있는 이야기: 시작` are headings somebody wrote.
/// Python reads a colon as an annotation and needs a type after it, so these
/// lines were reaching CPython as a syntax error.
fn is_written_label(source: &str, tokens: &[Token]) -> bool {
    if tokens.len() < 2 || !has_top_level_colon(tokens) {
        return false;
    }
    if is_valid_python_statement(token_text(source, tokens)) {
        return false;
    }
    let colons = tokens
        .iter()
        .filter(|token| matches!(token.tok, Tok::Colon))
        .count();
    colons == 1
        && tokens.iter().enumerate().all(|(index, token)| {
            matches!(token.tok, Tok::Colon)
                || is_sentence_word_token(index, token)
                || matches!(
                    token.tok,
                    Tok::Int { .. } | Tok::Float { .. } | Tok::String { .. } | Tok::Percent
                )
        })
}

/// The line with any `?`, `!` or full stop at the end of it set aside, so
/// that the word before the mark is the one a sentence ending is looked for
/// in. `설정을 초기화하시겠습니까?` is a question, and reading only the `?` left
/// the question itself standing as a name being given a value.
fn without_trailing_marks(tokens: &[Token]) -> &[Token] {
    let mut end = tokens.len();
    while end > 1 && is_command_ending(&tokens[end - 1]) {
        end -= 1;
    }
    &tokens[..end]
}

/// Punctuation people write inside a sentence and Python reads as code:
/// `K-POP`, `A/S`, `오전 9시~오후 6시`, `(괄호 안은 …)`, `[초안]`.
/// True when the line is asking something: a `?`, or a Korean question
/// predicate with the mark left off (`이름이 뭐예요`). There a misspelled
/// `물어봐` is worth naming rather than printing.
fn line_asks_a_question(tokens: &[Token]) -> bool {
    tokens.last().is_some_and(|token| {
        token_matches_exact(token, &["?"]) || token_matches_exact(token, KOREAN_QUESTION_PREDICATES)
    })
}

fn is_written_punctuation(tok: &Tok) -> bool {
    matches!(
        tok,
        Tok::Minus
            | Tok::Slash
            | Tok::Tilde
            | Tok::Plus
            | Tok::Amper
            | Tok::Star
            | Tok::Vbar
            | Tok::Lpar
            | Tok::Rpar
            | Tok::Lsqb
            | Tok::Rsqb
            | Tok::Lbrace
            | Tok::Rbrace
            | Tok::Comma
            | Tok::Colon
            | Tok::Percent
            | Tok::Dot
    )
}

/// A written Korean sentence carrying punctuation, on a line Python cannot
/// read.
///
/// [`is_written_korean_sentence`] turns such a line down because the
/// punctuation is not a word, and the line then goes back to Python — which
/// answers a Korean sentence with an English `SyntaxError` whose caret lands
/// inside a Hangul syllable. The Python check is asked here rather than
/// assumed, so a line that really is Python keeps its own meaning.
fn is_written_korean_sentence_with_punctuation(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> bool {
    if tokens.len() < 2 {
        return false;
    }
    let mut end = tokens.len();
    while end > 1
        && (is_command_ending(&tokens[end - 1])
            || matches!(
                tokens[end - 1].tok,
                Tok::Rpar | Tok::Rsqb | Tok::Rbrace | Tok::Comma | Tok::Dot
            ))
    {
        end -= 1;
    }
    let Some(word) = tokens[..end].last().and_then(name_word) else {
        return false;
    };
    if known_names.contains(word) || !is_hangul(word) {
        return false;
    }
    if korean_sentence_ending_of(word, end.saturating_sub(2)).is_none() {
        return false;
    }
    let all_written = tokens.iter().enumerate().all(|(index, token)| {
        is_sentence_word_token(index, token)
            || is_written_punctuation(&token.tok)
            || matches!(
                token.tok,
                Tok::Int { .. } | Tok::Float { .. } | Tok::String { .. }
            )
    });
    all_written && !is_valid_python_statement(token_text(source, tokens))
}

fn is_written_korean_sentence(tokens: &[Token], known_names: &HashSet<String>) -> bool {
    if tokens.len() < 2 {
        return false;
    }
    let line = without_trailing_marks(tokens);
    let Some(word) = line.last().and_then(name_word) else {
        return false;
    };
    if known_names.contains(word) || !is_hangul(word) {
        return false;
    }
    // One word in front is the shape of a name being given a value
    // (`인사는 안녕하세요`), so a polite ending needs two.
    let Some(ending) = korean_sentence_ending_of(word, line.len().saturating_sub(2)) else {
        return false;
    };
    // `점수는 0입니다` is the number zero spoken as a sentence, and that is a
    // value someone is saving. `아침입니다` is a morning.
    let stem = &word[..word.len() - ending.len()];
    if !stem.is_empty() && stem.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }
    // Nothing on the line may be code: no assignment, no call, no subscript,
    // no colon. Numbers and the percent sign are ordinary sentence writing.
    tokens.iter().enumerate().all(|(index, token)| {
        is_sentence_word_token(index, token)
            || matches!(
                token.tok,
                Tok::Int { .. } | Tok::Float { .. } | Tok::String { .. } | Tok::Percent
            )
    })
}

/// `이야기: 시작` — a label, a colon, and the words after it.
///
/// Python cannot read it (an annotation needs a type after the colon) and no
/// NME statement is shaped like it, so it is a line somebody wrote. Saving it
/// under the first word of the line, which is what a Korean topic particle
/// would otherwise ask for, is exactly the silent rewrite this compiler
/// exists to avoid. A real Python value keeps its colon — a dict, a slice, a
/// lambda — and those are still values.
fn is_labelled_phrase(source: &str, value: &[Token]) -> bool {
    if value.is_empty() || !has_top_level_colon(value) {
        return false;
    }
    !is_valid_python_expression(token_text(source, value))
}

/// A `:` that is not inside brackets, where Python would read a dict, a
/// slice, or a lambda instead.
fn has_top_level_colon(tokens: &[Token]) -> bool {
    let mut depth = 0usize;
    tokens.iter().any(|token| match token.tok {
        Tok::Lpar | Tok::Lsqb | Tok::Lbrace => {
            depth += 1;
            false
        }
        Tok::Rpar | Tok::Rsqb | Tok::Rbrace => {
            depth = depth.saturating_sub(1);
            false
        }
        Tok::Colon => depth == 0,
        _ => false,
    })
}

fn korean_value_is_a_sentence(value: &[Token]) -> bool {
    let Some(word) = value.last().and_then(name_word) else {
        return false;
    };
    // `할인율은 30%입니다` — Python's lexer cuts `30%입니다` into a number, a
    // percent sign, and the ending standing alone, so the ending is the whole
    // last word. The line is still a sentence, and reading it as a value
    // saved `30 % 입니다`: a modulo against a name nothing ever bound.
    if SENTENCE_ENDINGS_KO.contains(&word) {
        return !value_is_one_number(&value[..value.len() - 1]);
    }
    let Some(ending) =
        korean_sentence_ending_of(word, value.len() - 1).filter(|ending| word.len() > ending.len())
    else {
        return false;
    };
    let stem = &word[..word.len() - ending.len()];
    // `0이다` is the number zero spoken as a sentence; `아침입니다` is a
    // morning. Only the first is a value.
    !stem.chars().all(|character| character.is_ascii_digit())
}

/// True when a value is one written number and nothing else.
///
/// `정답은 7입니다` saves the number seven: the number is the whole value and
/// the ending is only how it was spoken. `할인율은 30%입니다` has a percent
/// sign after the number, so the number is not the whole value and the line
/// is a sentence.
fn value_is_one_number(value: &[Token]) -> bool {
    matches!(value, [token] if matches!(token.tok, Tok::Int { .. } | Tok::Float { .. }))
}

fn number_value_code(source: &str, tokens: &[Token]) -> Option<Code> {
    let mut end = tokens.len();
    while end > 1
        && tokens.get(end - 1).is_some_and(|token| {
            is_command_ending(token)
                || matches!(token.tok, Tok::Semi | Tok::Comma)
                || token_matches_exact(token, VALUE_ENDINGS_KO)
        })
    {
        end -= 1;
    }
    let [token] = &tokens[..end] else {
        return None;
    };
    let span = token.span;
    match token.tok {
        Tok::Int { .. } => Some(Code::Source(span)),
        // `0.` is a Python float, but at the end of a written sentence the
        // dot is a full stop: there are no digits after it.
        Tok::Float { .. } => {
            let text = &source[span.start..span.end];
            match text.strip_suffix('.') {
                Some(digits)
                    if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) =>
                {
                    Some(Code::Source(Span::new(span.start, span.end - 1)))
                }
                Some(_) => None,
                None => Some(Code::Source(span)),
            }
        }
        _ => None,
    }
}

/// Reads the value of an assignment, taking sentence punctuation and spoken
/// endings off a number first.
fn set_value(source: &str, tokens: &[Token], known_names: &HashSet<String>) -> Result<Value, ()> {
    if let Some(code) = number_value_code(source, tokens) {
        return Ok(Value::Python(code));
    }
    // `친구들은 목록` / `set friends to an empty list`. A name is being given
    // a value here, so the list word is the value; everywhere else it is a
    // word somebody wrote.
    if empty_list_phrase(tokens) {
        return Ok(Value::List(Vec::new()));
    }
    // `나이표는 빈 표` / `set ages to an empty record`. Same rule as the list
    // word: only here is `표` the kind of thing being made.
    if empty_record_phrase(tokens) {
        return Ok(Value::EmptyRecord);
    }
    parse_value(source, tokens, known_names, true)
}

/// `점수를 0으로` · `점수가 0` · `점수에 0 저장해` · `점수를 0으로 만들어`.
///
/// Korean marks the name being set with a particle and puts any saving word
/// last. Without a saving word the value must be a number, so ordinary speech
/// such as `강을 따라 집으로 갑니다` is never turned into an assignment.
fn korean_target_first_set(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(target) = name_word(&tokens[0])
        .and_then(|word| strip_any_suffix(word, SET_TARGET_PARTICLES_KO))
        .map(str::to_string)
    else {
        return Ok(None);
    };
    let mut end = trim_command_endings(tokens).len();
    let mut saving_word = false;
    if let Some(start) = (end.saturating_sub(2)..end).find(|&start| {
        start >= 1
            && action_phrase_at(tokens, start, SET_WORDS_KO, mode)
                .is_some_and(|consumed| start + consumed == end)
    }) {
        end = start;
        saving_word = true;
    }
    // `이름을 5로 해` · `이름을 5라고 하자` — the everyday light verb closing the
    // sentence instead of a saving word. It only counts when the value says
    // what the name is being turned into, because `해` attaches to any noun
    // in the language: without that mark `밥을 맛있게 해` would become a name
    // called `밥` holding the word `맛있게`.
    if !saving_word
        && end >= 3
        && token_matches_exact(&tokens[end - 1], SET_MAKE_WORDS_KO)
        && korean_value_marks_what_it_becomes(&tokens[end - 2])
    {
        end -= 1;
        saving_word = true;
    }
    let value_tokens = &tokens[1..end];
    if value_tokens.is_empty() || korean_value_is_a_sentence(value_tokens) {
        return Ok(None);
    }
    let number = number_value_code(source, value_tokens);
    if !saving_word && number.is_none() {
        return Ok(None);
    }
    let value = if let Some(code) = number {
        Value::Python(code)
    } else {
        let trimmed = trim_value_endings(value_tokens);
        set_value(source, &trimmed, known_names).map_err(|()| {
            Diagnostic::bilingual(
                DiagnosticCode::SaveValueUnparseable,
                "I couldn't understand the value to save",
                "저장할 값을 이해하지 못했어요",
                span_of(value_tokens),
            )
            .with_bilingual_hint(
                "write a number, name, or plain sentence",
                "숫자, 이름, 또는 평범한 문장을 적어 주세요",
            )
        })?
    };
    Ok(Some(NmeStmt::Set { target, value }))
}

/// Drops a trailing `으로`/`로`/`라고` written as its own word, which marks a
/// value in Korean without being part of it.
fn trim_value_endings(tokens: &[Token]) -> Vec<Token> {
    let mut value = tokens.to_vec();
    while value.len() > 1
        && value
            .last()
            .is_some_and(|token| token_matches_exact(token, SET_MAKE_ENDINGS_KO))
    {
        value.pop();
    }
    // `인사를 안녕하세요라고 하자` writes the mark joined to the value, and the
    // quotative `라고` is never part of what the value says.
    if let Some(last) = value.last_mut() {
        trim_name_token_suffix(last, &["이라고", "라고"]);
    }
    value
}

/// True when the last word of a Korean value says what the name is being
/// turned into: `5로`, `5라고`, or the mark written as its own word.
fn korean_value_marks_what_it_becomes(token: &Token) -> bool {
    token_word(token).is_some_and(|word| {
        SET_MAKE_ENDINGS_KO
            .iter()
            .any(|ending| word.strip_suffix(ending).is_some_and(|base| !base.is_empty()))
            || SET_MAKE_ENDINGS_KO
                .iter()
                .any(|ending| word.eq_ignore_ascii_case(ending))
    })
}

fn set_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, SET_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, SET_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

fn strip_saved_target(word: &str) -> &str {
    strip_assignment_particle(word).unwrap_or_else(|| {
        [
            "에게", "한테", "에서", "으로", "로", "을", "를", "에", "은", "는",
        ]
        .iter()
        .find_map(|particle| word.strip_suffix(particle).filter(|base| !base.is_empty()))
        .unwrap_or(word)
    })
}

// ---------------------------------------------------------- value parsing

fn parse_value(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    prefer_text: bool,
) -> Result<Value, ()> {
    if tokens.is_empty() {
        return Err(());
    }
    if tokens.len() == 1 {
        if let Some(literal) = literal_token(&tokens[0]) {
            return Ok(Value::Literal(literal));
        }
    }
    if let Some(value) = parse_elapsed_value(tokens, known_names) {
        return Ok(value);
    }
    if let Some(value) = parse_zero_knowledge_value(tokens) {
        return Ok(value);
    }
    if let Some(value) = parse_random_integer(source, tokens) {
        return Ok(value);
    }
    if let Some(value) = parse_random_choice(source, tokens) {
        return Ok(value);
    }
    if let Some(value) = parse_list_value(source, tokens, known_names) {
        return Ok(value);
    }
    // A reading only counts as one when it is the whole of what was given.
    // `친구들 개수가 궁금합니다` has three tokens and a reading uses two, so
    // it stays the sentence it is.
    if let Some((value, used)) = reading_prefix(tokens, known_names) {
        if used == tokens.len() {
            return Ok(value);
        }
    }

    let span = span_of(tokens);
    let text = &source[span.start..span.end];
    let single_known_name =
        tokens.len() == 1 && name_word(&tokens[0]).is_some_and(|name| known_names.contains(name));
    let single_unknown_name =
        tokens.len() == 1 && name_word(&tokens[0]).is_some() && !single_known_name;
    let clearly_code = tokens.len() == 1 && !matches!(tokens[0].tok, Tok::Name { .. })
        || tokens.iter().any(|token| {
            matches!(
                token.tok,
                Tok::Plus
                    | Tok::Minus
                    | Tok::Star
                    | Tok::DoubleStar
                    | Tok::Slash
                    | Tok::DoubleSlash
                    | Tok::Percent
                    | Tok::Lpar
                    | Tok::Lsqb
                    | Tok::Lbrace
                    | Tok::EqEqual
                    | Tok::NotEqual
                    | Tok::Less
                    | Tok::Greater
                    | Tok::LessEqual
                    | Tok::GreaterEqual
            )
        });
    if is_valid_python_expression(text)
        && ((!prefer_text && !single_unknown_name) || single_known_name || clearly_code)
    {
        return Ok(Value::Python(Code::Source(span)));
    }
    Ok(Value::Text(make_text_template(source, tokens, known_names)))
}

/// `list of Mina, Ada` / `목록 민수, 지안`.
///
/// The marker word is required. Without it a comma-separated sentence stays
/// ordinary text, which is what a learner writing `Mina, Ada and Grace` means.
fn parse_list_value(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<Value> {
    let mut start = 0;
    if token_matches_exact(tokens.first()?, LIST_WORDS_EN) {
        start = 1;
        if tokens
            .get(1)
            .is_some_and(|token| token_matches_exact(token, &["of"]))
        {
            start = 2;
        } else if !tokens[1..]
            .iter()
            .any(|token| matches!(token.tok, Tok::Comma))
        {
            // `list of Mina, Ada` names its items with `of`, and `list Mina,
            // Ada` names them with commas. `List the ingredients on the back.`
            // does neither: it is a sentence, and reading it as a list put the
            // whole of it inside one pair of brackets.
            return None;
        }
    } else if token_matches_exact(tokens.first()?, LIST_WORDS_KO) {
        start = 1;
    }
    if start == 0 {
        return None;
    }
    // A list with nothing after the marker word is only a list where a value
    // is being saved; `empty_list_phrase` reads it there. Here the words are
    // what somebody wrote.
    let items = &tokens[start..];
    if items.is_empty() {
        return None;
    }
    let mut values = Vec::new();
    for part in split_list_items(items) {
        if part.is_empty() {
            continue;
        }
        values.push(parse_value(source, &part, known_names, true).ok()?);
    }
    (!values.is_empty()).then_some(Value::List(values))
}

/// `목록` · `list` · `list of` · `빈 목록` · `an empty list`.
///
/// **Only an assignment reads these as an empty list.** As the thing an
/// output word is given, the same words are the ones somebody wrote:
/// `목록을 보여 주세요` asks to be shown a list, and answering it with `[]`
/// puts an empty pair of brackets on the screen and tells the writer nothing.
/// An article is accepted only in front of `empty`/`빈`, so the set of
/// phrases claimed here is exactly the set that made a list before.
fn empty_list_phrase(tokens: &[Token]) -> bool {
    let Some(first) = tokens.first() else {
        return false;
    };
    let article = token_matches_exact(first, &["a", "an", "the"]);
    let mut at = usize::from(article);
    let empty = tokens.get(at).is_some_and(|token| {
        token_matches_exact(token, EMPTY_WORDS_EN) || token_matches_exact(token, EMPTY_WORDS_KO)
    });
    if article && !empty {
        return false;
    }
    at += usize::from(empty);
    if !tokens.get(at).is_some_and(|token| {
        token_matches_exact(token, LIST_WORDS_EN) || token_matches_exact(token, LIST_WORDS_KO)
    }) {
        return false;
    }
    at += 1;
    if tokens
        .get(at)
        .is_some_and(|token| token_matches_exact(token, &["of"]))
    {
        at += 1;
    }
    at == tokens.len()
}

/// `표` · `record` · `an empty record` · `빈 표`.
///
/// **Only an assignment reads these as a record**, for the same reason the
/// list word is read that way only there: `표를 보여 주세요` asks to be shown a
/// table, and answering it with `{}` puts an empty pair of braces on the
/// screen and tells the writer nothing.
fn empty_record_phrase(tokens: &[Token]) -> bool {
    let Some(first) = tokens.first() else {
        return false;
    };
    let article = token_matches_exact(first, &["a", "an", "the"]);
    let mut at = usize::from(article);
    let empty = tokens.get(at).is_some_and(|token| {
        token_matches_exact(token, EMPTY_WORDS_EN) || token_matches_exact(token, EMPTY_WORDS_KO)
    });
    if article && !empty {
        return false;
    }
    at += usize::from(empty);
    if !tokens.get(at).is_some_and(|token| {
        token_matches_exact(token, RECORD_WORDS_EN) || token_matches_exact(token, RECORD_WORDS_KO)
    }) {
        return false;
    }
    at + 1 == tokens.len()
}

/// Words people put between list items, standing alone or attached to the
/// word before them as a Korean particle (`민수와 지안`).
const LIST_JOINERS: &[&str] = &["and", "그리고", "와", "과", "이랑", "랑"];

/// Cuts a list into its items. A Korean joining particle is part of the word
/// it follows, so the particle is trimmed off and the item ends there.
fn split_list_items(tokens: &[Token]) -> Vec<Vec<Token>> {
    let mut items = Vec::new();
    let mut current: Vec<Token> = Vec::new();
    // `사과` ends in `과`, which is also the Korean word for `and`. Once the
    // writer has shown their separator by using a comma, trust it and stop
    // reading a joiner out of the end of a word.
    let comma_separated = tokens.iter().any(|token| matches!(token.tok, Tok::Comma));
    for token in tokens {
        if matches!(token.tok, Tok::Comma | Tok::And) || token_matches_exact(token, LIST_JOINERS) {
            items.push(std::mem::take(&mut current));
            continue;
        }
        if let (false, Tok::Name { name }) = (comma_separated, &token.tok) {
            if let Some(base) = LIST_JOINERS
                .iter()
                .filter(|joiner| joiner.chars().next().is_some_and(|c| !c.is_ascii()))
                .find_map(|joiner| name.strip_suffix(joiner).filter(|base| !base.is_empty()))
            {
                current.push(Token {
                    tok: Tok::Name {
                        name: base.to_string(),
                    },
                    span: Span::new(token.span.start, token.span.start + base.len()),
                });
                items.push(std::mem::take(&mut current));
                continue;
            }
        }
        if is_command_ending(token) {
            continue;
        }
        current.push(token.clone());
    }
    items.push(current);
    items
}

fn parse_zero_knowledge_value(tokens: &[Token]) -> Option<Value> {
    use crate::syntax::ZeroKnowledgeValue as Zk;

    // English sentence spellings mirror the Korean zero-knowledge value
    // grammar without requiring underscores, calls, commas, or parentheses.
    // `zeroknowledge` is reserved only for the module line; value phrases use
    // ordinary words so a complete sentence source can stay letters-only.
    if tokens.len() == 7
        && token_matches_exact(&tokens[3], &["zero"])
        && token_matches_exact(&tokens[4], &["knowledge"])
        && token_matches_exact(&tokens[5], &["challenge"])
        && token_matches_exact(&tokens[6], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkChallenge {
            public_key: zero_knowledge_code_plain(&tokens[0])?,
            commitment: zero_knowledge_code_plain(&tokens[1])?,
            context: zero_knowledge_code_plain(&tokens[2])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[2], &["zero"])
        && token_matches_exact(&tokens[3], &["knowledge"])
        && token_matches_exact(&tokens[4], &["proof"])
        && token_matches_exact(&tokens[5], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkProof {
            secret: zero_knowledge_code_plain(&tokens[0])?,
            context: zero_knowledge_code_plain(&tokens[1])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[3], &["zero"])
        && token_matches_exact(&tokens[4], &["knowledge"])
        && token_matches_exact(&tokens[5], &["verify"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkVerify {
            public_key: zero_knowledge_code_plain(&tokens[0])?,
            proof: zero_knowledge_code_plain(&tokens[1])?,
            context: zero_knowledge_code_plain(&tokens[2])?,
        }));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[0], &["zero"])
        && token_matches_exact(&tokens[1], &["knowledge"])
        && token_matches_exact(&tokens[2], &["secret"])
        && token_matches_exact(&tokens[3], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Secret));
    }

    if tokens.len() == 5
        && token_matches_exact(&tokens[1], &["zero"])
        && token_matches_exact(&tokens[2], &["knowledge"])
        && token_matches_exact(&tokens[3], &["public"])
        && token_matches_exact(&tokens[4], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Public {
            secret: zero_knowledge_code_plain(&tokens[0])?,
        }));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[0], &["zero"])
        && token_matches_exact(&tokens[1], &["knowledge"])
        && token_matches_exact(&tokens[2], &["nonce"])
        && token_matches_exact(&tokens[3], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Nonce));
    }

    if tokens.len() == 5
        && token_matches_exact(&tokens[1], &["zero"])
        && token_matches_exact(&tokens[2], &["knowledge"])
        && token_matches_exact(&tokens[3], &["commitment"])
        && token_matches_exact(&tokens[4], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Commitment {
            nonce: zero_knowledge_code_plain(&tokens[0])?,
        }));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[0], &["zero"])
        && token_matches_exact(&tokens[1], &["knowledge"])
        && token_matches_exact(&tokens[2], &["challenge"])
        && token_matches_exact(&tokens[3], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Challenge));
    }

    // The five English spellings below mirror the Korean ones word for word.
    // Without them `set ok to p c e z zero knowledge verify` was saved as a
    // *sentence*, which is the worst outcome this compiler has: the program
    // ran, checked nothing, and said nothing.
    if tokens.len() == 6
        && token_matches_exact(&tokens[1], &["different", "other"])
        && token_matches_exact(&tokens[2], &["zero"])
        && token_matches_exact(&tokens[3], &["knowledge"])
        && token_matches_exact(&tokens[4], &["challenge"])
        && token_matches_exact(&tokens[5], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::ChallengeExcept {
            excluded: zero_knowledge_code_plain(&tokens[0])?,
        }));
    }

    if tokens.len() == 7
        && token_matches_exact(&tokens[3], &["zero"])
        && token_matches_exact(&tokens[4], &["knowledge"])
        && token_matches_exact(&tokens[5], &["response"])
        && token_matches_exact(&tokens[6], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Response {
            nonce: zero_knowledge_code_plain(&tokens[0])?,
            secret: zero_knowledge_code_plain(&tokens[1])?,
            challenge: zero_knowledge_code_plain(&tokens[2])?,
        }));
    }

    if tokens.len() == 7
        && token_matches_exact(&tokens[4], &["zero"])
        && token_matches_exact(&tokens[5], &["knowledge"])
        && token_matches_exact(&tokens[6], &["verify"])
    {
        return Some(Value::ZeroKnowledge(Zk::Verify {
            public_key: zero_knowledge_code_plain(&tokens[0])?,
            commitment: zero_knowledge_code_plain(&tokens[1])?,
            challenge: zero_knowledge_code_plain(&tokens[2])?,
            response: zero_knowledge_code_plain(&tokens[3])?,
        }));
    }

    if tokens.len() == 5
        && token_matches_exact(&tokens[0], &["zero"])
        && token_matches_exact(&tokens[1], &["knowledge"])
        && token_matches_exact(&tokens[2], &["simulated"])
        && token_matches_exact(&tokens[3], &["response"])
        && token_matches_exact(&tokens[4], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::SimulatedResponse));
    }

    if tokens.len() == 8
        && token_matches_exact(&tokens[3], &["zero"])
        && token_matches_exact(&tokens[4], &["knowledge"])
        && token_matches_exact(&tokens[5], &["simulated"])
        && token_matches_exact(&tokens[6], &["commitment"])
        && token_matches_exact(&tokens[7], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::SimulatedCommitment {
            public_key: zero_knowledge_code_plain(&tokens[0])?,
            challenge: zero_knowledge_code_plain(&tokens[1])?,
            response: zero_knowledge_code_plain(&tokens[2])?,
        }));
    }

    if tokens.len() == 7
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["비대화"])
        && token_matches_exact(&tokens[5], &["도전"])
        && token_matches_exact(&tokens[6], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkChallenge {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            commitment: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            context: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[2], &["영지식"])
        && token_matches_exact(&tokens[3], &["비대화"])
        && token_matches_exact(&tokens[4], &["증명"])
        && token_matches_exact(&tokens[5], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkProof {
            secret: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            context: zero_knowledge_code_with_particle(&tokens[1], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["비대화"])
        && token_matches_exact(&tokens[5], &["검증"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkVerify {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            proof: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            context: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 3
        && token_matches_exact(&tokens[0], &["영지식"])
        && token_matches_exact(&tokens[1], &["비밀"])
        && token_matches_exact(&tokens[2], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Secret));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[1], &["영지식"])
        && token_matches_exact(&tokens[2], &["공개값"])
        && token_matches_exact(&tokens[3], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Public {
            secret: zero_knowledge_code_with_particle(&tokens[0], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 3
        && token_matches_exact(&tokens[0], &["영지식"])
        && token_matches_exact(&tokens[1], &["일회값"])
        && token_matches_exact(&tokens[2], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Nonce));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[1], &["영지식"])
        && token_matches_exact(&tokens[2], &["약속"])
        && token_matches_exact(&tokens[3], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Commitment {
            nonce: zero_knowledge_code_with_particle(&tokens[0], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 3
        && token_matches_exact(&tokens[0], &["영지식"])
        && token_matches_exact(&tokens[1], &["도전"])
        && token_matches_exact(&tokens[2], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Challenge));
    }

    if tokens.len() == 5
        && token_matches_exact(&tokens[1], &["다른"])
        && token_matches_exact(&tokens[2], &["영지식"])
        && token_matches_exact(&tokens[3], &["도전"])
        && token_matches_exact(&tokens[4], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::ChallengeExcept {
            excluded: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["응답"])
        && token_matches_exact(&tokens[5], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Response {
            nonce: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            secret: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            challenge: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[4], &["영지식"])
        && token_matches_exact(&tokens[5], &["검증"])
    {
        return Some(Value::ZeroKnowledge(Zk::Verify {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            commitment: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            challenge: zero_knowledge_code_with_particle(&tokens[2], &["과", "와"])?,
            response: zero_knowledge_code_with_particle(&tokens[3], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[0], &["영지식"])
        && token_matches_exact(&tokens[1], &["모의"])
        && token_matches_exact(&tokens[2], &["응답"])
        && token_matches_exact(&tokens[3], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::SimulatedResponse));
    }

    if tokens.len() == 7
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["모의"])
        && token_matches_exact(&tokens[5], &["약속"])
        && token_matches_exact(&tokens[6], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::SimulatedCommitment {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            challenge: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            response: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"])?,
        }));
    }

    None
}

fn zero_knowledge_code_plain(token: &Token) -> Option<Code> {
    name_word(token)?;
    Some(Code::Source(token.span))
}

fn zero_knowledge_code_with_particle(token: &Token, particles: &[&str]) -> Option<Code> {
    let word = name_word(token)?;
    let stripped = particles
        .iter()
        .find_map(|particle| word.strip_suffix(particle).filter(|base| !base.is_empty()))?;
    let removed = word.len() - stripped.len();
    Some(Code::Source(Span::new(
        token.span.start,
        token.span.end - removed,
    )))
}

fn parse_random_integer(source: &str, tokens: &[Token]) -> Option<Value> {
    let random_at = tokens.iter().position(|token| {
        word_matches_any(
            token,
            &[
                "랜덤",
                "랜덤정수",
                "무작위",
                "무작위숫자",
                "random",
                "randomnumber",
            ],
            MatchMode::Recover,
        )
    })?;

    // Korean/mixed order: `1부터 6까지 랜덤정수`.
    if random_at > 0 {
        let head = &tokens[..random_at];
        if let Some(from) = range_marker_from(head, 0, &["부터", "에서"]) {
            if let Some(to) = range_marker_from(head, from.at + 1, &["까지"]) {
                let low = trimmed_span(source, head[0].span.start, from.bound_end);
                let high = trimmed_span(source, bound_start_after(head, &from)?, to.bound_end);
                if let Some(value) = random_integer_between(source, low, high) {
                    return Some(value);
                }
            }
        }
    }

    // English-first order: `random number from 1 to 6`.
    let from = range_marker_from(tokens, 0, &["from", "부터", "에서"])?;
    let to = range_marker_from(tokens, from.at + 1, &["to", "까지"])?;
    let low = trimmed_span(source, bound_start_after(tokens, &from)?, to.bound_end);
    let high = trimmed_span(
        source,
        bound_start_after(tokens, &to)?,
        tokens.last()?.span.end,
    );
    random_integer_between(source, low, high)
}

/// One end marker of a random range, and where the bound in front of it ends.
struct RangeMarker {
    at: usize,
    /// True when the marker is written joined to the bound (`적공격까지`)
    /// rather than standing on its own (`6 까지`).
    attached: bool,
    /// Byte offset in the source where the bound in front of the marker ends.
    bound_end: usize,
}

/// Where a range marker sits, searching from `start`.
///
/// A Korean marker may be written joined to the bound in front of it, which
/// is what Korean does whenever that bound is a word rather than a digit:
/// `1부터 적공격까지 랜덤정수`. English markers are never read that way —
/// `photo`, `into` and `auto` all end in `to`.
fn range_marker_from(tokens: &[Token], start: usize, markers: &[&str]) -> Option<RangeMarker> {
    (start..tokens.len()).find_map(|at| {
        // `token_word`, not `name_word`: `from` is a Python keyword and never
        // reaches the parser as a plain name.
        let word = token_word(&tokens[at])?;
        if markers.contains(&word) {
            return Some(RangeMarker {
                at,
                attached: false,
                bound_end: tokens[at].span.start,
            });
        }
        markers.iter().find_map(|marker| {
            (is_hangul(marker) && word.strip_suffix(marker).is_some_and(|base| !base.is_empty()))
                .then(|| RangeMarker {
                    at,
                    attached: true,
                    bound_end: tokens[at].span.end - marker.len(),
                })
        })
    })
}

/// Where the bound after a marker begins.
fn bound_start_after(tokens: &[Token], marker: &RangeMarker) -> Option<usize> {
    if marker.attached {
        Some(tokens[marker.at].span.end)
    } else {
        Some(tokens.get(marker.at + 1)?.span.start)
    }
}

/// A span with the spaces around it taken off, so a bound never carries one
/// into the Python.
fn trimmed_span(source: &str, start: usize, end: usize) -> Span {
    let mut start = start;
    let mut end = end;
    let bytes = source.as_bytes();
    while start < end && bytes[start].is_ascii_whitespace() {
        start += 1;
    }
    while end > start && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    Span::new(start, end)
}

fn random_integer_between(source: &str, low: Span, high: Span) -> Option<Value> {
    if low.start >= low.end || high.start >= high.end {
        return None;
    }
    (is_valid_python_expression(&source[low.start..low.end])
        && is_valid_python_expression(&source[high.start..high.end]))
    .then_some(Value::RandomInteger {
        low: Code::Source(low),
        high: Code::Source(high),
    })
}

/// The words that mark one choice off from the next: `red or green`,
/// `빨강 또는 초록`, or a comma between them.
const CHOICE_SEPARATOR_WORDS: &[&str] = &["or", "and", "또는", "이나", "나"];

/// True when the alternatives are marked off from each other.
///
/// Without this the parser split any sentence containing `골라` on its spaces
/// and picked one of the pieces: `마음에 드는 것을 골라 보세요` printed a
/// different word of itself every run, and nothing in the line ever named a
/// choice. A pick only reads as a pick when the writer separated the choices —
/// with `또는`/`or`, or with a comma. `중에서` before the pick word is not
/// enough on its own: `여러 개 중에서 뽑아` is one phrase, not two choices, and
/// it too was picking a word of itself at random.
fn choices_are_marked(choices_tokens: &[Token]) -> bool {
    choices_tokens.iter().any(|token| {
        token_matches_exact(token, CHOICE_SEPARATOR_WORDS) || matches!(token.tok, Tok::Comma)
    })
}

fn parse_random_choice(source: &str, tokens: &[Token]) -> Option<Value> {
    let pick_at = tokens
        .iter()
        .position(|token| token_matches_exact(token, RANDOM_CHOICE_WORDS))?;
    let choices_tokens = if pick_at == 0 {
        let start = tokens
            .iter()
            .position(|token| token_matches_exact(token, &["from", "중에서"]))?
            + 1;
        &tokens[start..]
    } else {
        &tokens[..pick_at]
    };
    if !choices_are_marked(choices_tokens) {
        return None;
    }
    let choices: Vec<String> = choices_tokens
        .iter()
        .filter(|token| {
            !token_matches_exact(token, &["or", "and", "또는", "이나", "중", "중에서"])
                && !matches!(token.tok, Tok::Comma)
        })
        .map(|token| {
            let raw = &source[token.span.start..token.span.end];
            raw.trim_matches(['\'', '"']).to_string()
        })
        .filter(|choice| !choice.is_empty())
        .collect();
    (choices.len() >= 2).then_some(Value::RandomChoice { choices })
}

fn make_text_template(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> TextTemplate {
    let mut parts = Vec::new();
    let mut cursor = tokens[0].span.start;
    let end = tokens[tokens.len() - 1].span.end;

    for token in tokens {
        let Some(word) = name_word(token) else {
            continue;
        };
        let Some((variable, particle)) = split_template_variable(word, known_names) else {
            continue;
        };
        if cursor < token.span.start {
            push_literal(&mut parts, &source[cursor..token.span.start]);
        }
        parts.push(TextPart::Variable(variable.to_string()));
        if !particle.is_empty() {
            push_literal(&mut parts, particle);
        }
        cursor = token.span.end;
    }
    if cursor < end {
        push_literal(&mut parts, &source[cursor..end]);
    }
    if parts.is_empty() {
        parts.push(TextPart::Literal(
            source[tokens[0].span.start..end].to_string(),
        ));
    }
    TextTemplate { parts }
}

fn push_literal(parts: &mut Vec<TextPart>, text: &str) {
    if text.is_empty() {
        return;
    }
    match parts.last_mut() {
        Some(TextPart::Literal(existing)) => existing.push_str(text),
        _ => parts.push(TextPart::Literal(text.to_string())),
    }
}

/// Which name inside a sentence is shown as a value, and what is left over.
///
/// **A name a bundled module bound is never one of them.** The writer did not
/// choose those names and mostly does not know them, so a sentence holding one
/// is a sentence: after `use math`, `the floor is cold` printed
/// `the <built-in function floor> is cold`, and after `글자 사용`,
/// `길이가 조금 짧습니다` showed a function where the length should have been.
/// A name the *program* made is different — the writer wrote it, and showing
/// its value in a sentence is the whole point of the form.
fn split_template_variable<'a>(
    word: &'a str,
    known_names: &'a HashSet<String>,
) -> Option<(&'a str, &'a str)> {
    if known_names.contains(word) {
        return (!is_module_name(known_names, word)).then_some((word, ""));
    }
    let mut candidates: Vec<&String> = known_names
        .iter()
        .filter(|name| {
            word.starts_with(name.as_str()) && !is_module_name(known_names, name.as_str())
        })
        .collect();
    candidates.sort_by_key(|name| std::cmp::Reverse(name.chars().count()));
    for name in candidates {
        let particle = &word[name.len()..];
        if KOREAN_PARTICLES.contains(&particle) {
            return Some((&word[..name.len()], particle));
        }
    }
    None
}

// --------------------------------------------------------------- suites

#[derive(Clone, Copy)]
enum SuiteKind {
    Repeat,
    Condition,
}

fn parse_suite_body(
    source: &str,
    body: &[Token],
    block: &BlockCtx<'_>,
    kind: SuiteKind,
    header_span: Span,
    known_names: &HashSet<String>,
) -> Result<Option<InlineStmt>, Diagnostic> {
    if body.is_empty() {
        return match block {
            BlockCtx::TopLevel { line, next_indent } => {
                if next_indent.is_some_and(|next| next > line.indent) {
                    Ok(None)
                } else {
                    Err(indentation_diagnostic(kind, line.span))
                }
            }
            BlockCtx::Inline => Err(inline_block_diagnostic(kind, header_span)),
        };
    }

    let body_span = span_of(body);
    if has_top_level_semicolon(body) {
        return Err(one_statement_diagnostic(kind, body_span));
    }
    if branch_shape(body).is_some() {
        return Err(branch_without_condition_diagnostic(body_span));
    }
    // Korean `멈춰` is a valid Python identifier, so the Python-wins check in
    // `classify` intentionally leaves a bare top-level name alone. Inside an
    // already recognized NME suite, however, the documented Korean break
    // spelling is unambiguous and must lower to `break` rather than leaking
    // the identifier into generated Python.
    if is_korean_break_alias(body) {
        return Ok(Some(InlineStmt::Nme(Box::new(NmeStmt::Break))));
    }
    if is_skip_alias(body) {
        return Ok(Some(InlineStmt::Nme(Box::new(NmeStmt::Continue))));
    }
    if let Some(inner) = classify(source, body, &BlockCtx::Inline, known_names)? {
        if matches!(&inner, NmeStmt::ElseIf { .. } | NmeStmt::Else { .. }) {
            return Err(branch_without_condition_diagnostic(body_span));
        }
        return Ok(Some(InlineStmt::Nme(Box::new(inner))));
    }
    if !is_valid_python_statement(&source[body_span.start..body_span.end]) {
        return Err(body_diagnostic(kind, body_span));
    }
    Ok(Some(InlineStmt::Python(body_span)))
}

fn indentation_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    match kind {
        SuiteKind::Repeat => Diagnostic::bilingual(
            DiagnosticCode::IndentationRequired,
            "the lines that should repeat must be indented",
            "반복할 다음 줄은 들여써야 해요",
            span,
        )
        .with_bilingual_hint(
            "or keep it on one line: `repeat 3 times and show Hello`",
            "한 줄로 `3번 반복해서 안녕 말해줘`라고 써도 돼요",
        ),
        SuiteKind::Condition => Diagnostic::bilingual(
            DiagnosticCode::ColonRequired,
            "this condition needs `:` or an indented next line",
            "조건 다음에는 실행할 줄이나 `:`이 필요해요",
            span,
        )
        .with_bilingual_hint(
            "or put one statement after `then`",
            "한 문장은 `있으면` 뒤에 바로 적어도 돼요",
        ),
    }
}

fn inline_block_diagnostic(_kind: SuiteKind, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::BlockWithoutStatement,
        "a block can't start here without a statement",
        "이 한 줄 블록에 실행할 문장이 없어요",
        span,
    )
    .with_bilingual_hint(
        "put one statement here, or use an indented block on the next line",
        "실행할 문장을 이어 쓰거나 다음 줄에 들여쓰세요",
    )
}

fn one_statement_diagnostic(_kind: SuiteKind, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::OneStatementPerLine,
        "only one statement fits on this line",
        "한 줄에는 문장 하나만 넣을 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put multiple statements on separate indented lines",
        "여러 문장은 다음 줄부터 하나씩 들여쓰세요",
    )
}

fn body_diagnostic(_kind: SuiteKind, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::BodyUnparseable,
        "I couldn't understand the statement here",
        "여기 있는 문장을 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write one Python, beginner, or sentence-style statement",
        "Python, 초급, 문장형 문법 중 한 문장을 적어 주세요",
    )
}

// --------------------------------------------------------------- helpers

#[derive(Clone, Copy)]
enum BindingScopeKind {
    Root,
    Function,
    AsyncFunction,
    Class,
    Other,
}

struct BindingScope {
    body_indent: usize,
    names: HashSet<String>,
    kind: BindingScopeKind,
}

struct AsyncFunctionContext {
    body_scope_depth: usize,
    has_yield: bool,
    return_value_spans: Vec<Span>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PythonDeclarationKind {
    Global,
    Nonlocal,
}

struct PythonDeclaration {
    kind: PythonDeclarationKind,
    names: Vec<(String, usize)>,
}

struct PythonDeclarationContext {
    body_scope_depth: usize,
    seen_names: HashSet<String>,
    annotation_targets: HashSet<String>,
    declarations: HashMap<String, PythonDeclarationKind>,
}

struct PendingScope {
    header_indent: usize,
    names: HashSet<String>,
    kind: BindingScopeKind,
}

struct BindingEnv {
    scopes: Vec<BindingScope>,
    pending: Option<PendingScope>,
}

impl BindingEnv {
    fn new() -> Self {
        Self {
            scopes: vec![BindingScope {
                body_indent: 0,
                names: HashSet::new(),
                kind: BindingScopeKind::Root,
            }],
            pending: None,
        }
    }

    fn enter_line(&mut self, indent: usize) {
        if let Some(pending) = self.pending.take() {
            if indent > pending.header_indent {
                self.scopes.push(BindingScope {
                    body_indent: indent,
                    names: pending.names,
                    kind: pending.kind,
                });
            }
        }
        while self.scopes.len() > 1 && indent < self.scopes.last().expect("root scope").body_indent
        {
            self.scopes.pop();
        }
    }

    fn visible_names(&self) -> HashSet<String> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.names.iter().cloned())
            .collect()
    }

    fn push_explicit_scope(&mut self, body_indent: usize) {
        self.scopes.push(BindingScope {
            body_indent,
            names: HashSet::new(),
            kind: BindingScopeKind::Other,
        });
    }

    /// The body of a named job whose lines are flat and closed by `end`.
    /// It is a real Python function scope, so names set inside it stay inside
    /// it, and an ordinary Python `return` written in there is accepted
    /// rather than refused.
    fn push_function_scope(&mut self, body_indent: usize, names: HashSet<String>) {
        self.scopes.push(BindingScope {
            body_indent,
            names,
            kind: BindingScopeKind::Function,
        });
    }

    /// The body of a named job written the way Python writes one: indented
    /// under its header. The scope opens when the first indented line arrives
    /// and closes when the indentation does, which is the same path an
    /// ordinary `def` header already takes.
    fn push_pending_function_scope(&mut self, header_indent: usize, names: HashSet<String>) {
        self.pending = Some(PendingScope {
            header_indent,
            names,
            kind: BindingScopeKind::Function,
        });
    }

    fn inside_function(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            match scope.kind {
                BindingScopeKind::Function | BindingScopeKind::AsyncFunction => return true,
                BindingScopeKind::Class => return false,
                BindingScopeKind::Root | BindingScopeKind::Other => {}
            }
        }
        false
    }

    fn inside_async_function(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            match scope.kind {
                BindingScopeKind::AsyncFunction => return true,
                BindingScopeKind::Function | BindingScopeKind::Class => return false,
                BindingScopeKind::Root | BindingScopeKind::Other => {}
            }
        }
        false
    }

    fn inside_non_module_scope(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            match scope.kind {
                BindingScopeKind::Root => return false,
                BindingScopeKind::Function
                | BindingScopeKind::AsyncFunction
                | BindingScopeKind::Class => return true,
                BindingScopeKind::Other => {}
            }
        }
        false
    }

    fn python_scope_depth(&self) -> usize {
        self.scopes
            .iter()
            .filter(|scope| {
                matches!(
                    scope.kind,
                    BindingScopeKind::Function
                        | BindingScopeKind::AsyncFunction
                        | BindingScopeKind::Class
                )
            })
            .count()
    }

    fn has_enclosing_function(&self) -> bool {
        let Some((current_index, current_scope)) = self
            .scopes
            .iter()
            .enumerate()
            .rev()
            .find(|(_, scope)| !matches!(scope.kind, BindingScopeKind::Other))
        else {
            return false;
        };
        if !matches!(
            current_scope.kind,
            BindingScopeKind::Function | BindingScopeKind::AsyncFunction | BindingScopeKind::Class
        ) {
            return false;
        }
        self.scopes[..current_index].iter().any(|scope| {
            matches!(
                scope.kind,
                BindingScopeKind::Function | BindingScopeKind::AsyncFunction
            )
        })
    }

    fn has_function_scope(&self) -> bool {
        self.scopes.iter().any(|scope| {
            matches!(
                scope.kind,
                BindingScopeKind::Function | BindingScopeKind::AsyncFunction
            )
        })
    }

    fn remember_nme(&mut self, stmt: &NmeStmt) {
        remember_bindings(stmt, &mut self.scopes.last_mut().expect("root scope").names);
    }

    fn remember_python(&mut self, tokens: &[Token], indent: usize) {
        remember_python_binding(
            tokens,
            &mut self.scopes.last_mut().expect("root scope").names,
        );
        if let Some((name, parameters)) = python_scope_header(tokens) {
            // `def greet():` written in ordinary Python is a job the sentence
            // tier can run by name. Only a job that takes nothing: `do greet`
            // passes no arguments, and calling a function that wants some
            // would fail at run time on a line that looks right.
            if is_python_function_header(tokens) && parameters.is_empty() {
                remember_job_name(
                    &mut self.scopes.last_mut().expect("root scope").names,
                    &name,
                    0,
                );
            }
            self.scopes
                .last_mut()
                .expect("root scope")
                .names
                .insert(name);
            if python_inline_suite_body(tokens).is_none() {
                self.pending = Some(PendingScope {
                    header_indent: indent,
                    names: parameters,
                    kind: if is_python_async_function_header(tokens) {
                        BindingScopeKind::AsyncFunction
                    } else if is_python_function_header(tokens) {
                        BindingScopeKind::Function
                    } else if is_python_class_header(tokens) {
                        BindingScopeKind::Class
                    } else {
                        BindingScopeKind::Other
                    },
                });
            }
        }
    }
}

fn python_scope_header(tokens: &[Token]) -> Option<(String, HashSet<String>)> {
    let keyword_at = if matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::Def))
    {
        1
    } else if matches!(
        tokens.first().map(|token| &token.tok),
        Some(Tok::Def | Tok::Class)
    ) {
        0
    } else {
        return None;
    };
    let name = name_word(tokens.get(keyword_at + 1)?)?.to_string();
    let mut parameters = HashSet::new();
    if matches!(tokens[keyword_at].tok, Tok::Def) {
        let mut inside_parameters = false;
        for token in &tokens[keyword_at + 2..] {
            match &token.tok {
                Tok::Lpar => inside_parameters = true,
                Tok::Rpar => break,
                Tok::Name { name } if inside_parameters => {
                    parameters.insert(name.clone());
                }
                _ => {}
            }
        }
    }
    Some((name, parameters))
}

fn python_inline_suite_body(tokens: &[Token]) -> Option<&[Token]> {
    python_scope_header(tokens)?;
    let depths = token_depths(tokens);
    let colon_index = tokens.iter().enumerate().find_map(|(index, token)| {
        (depths[index] == 0 && matches!(token.tok, Tok::Colon)).then_some(index)
    })?;
    let body = tokens.get(colon_index + 1..)?;
    (!body.is_empty()).then_some(body)
}

fn python_inline_function_body(tokens: &[Token]) -> Option<&[Token]> {
    is_python_function_header(tokens).then(|| python_inline_suite_body(tokens))?
}

fn remember_python_binding(tokens: &[Token], names: &mut HashSet<String>) {
    if let [Token {
        tok: Tok::Name { name },
        ..
    }, Token {
        tok: Tok::Equal, ..
    }, rest @ ..] = tokens
    {
        names.insert(name.clone());
        // `friends = []` in ordinary Python makes a list just as
        // `친구들은 빈 목록` does, and the sentence statements that need a list
        // should work on it. Only a literal `[` is claimed: a call could
        // return anything.
        if matches!(rest.first().map(|token| &token.tok), Some(Tok::Lsqb)) {
            remember_list_name(names, name);
        }
        // `ages = {}` in ordinary Python makes a record just as `나이표는 빈 표`
        // does. `{` alone is not enough — `{1, 2}` is a set — so either the
        // braces are empty or something inside them is written `key: value`.
        if matches!(rest.first().map(|token| &token.tok), Some(Tok::Lbrace))
            && (matches!(rest.get(1).map(|token| &token.tok), Some(Tok::Rbrace))
                || rest.iter().any(|token| matches!(token.tok, Tok::Colon)))
        {
            remember_record_name(names, name);
        }
    }

    // A simple Python loop target is available to sentence syntax in its
    // indented body. Destructuring names are safe to remember too; attribute
    // and subscript targets contain no standalone binding token we claim.
    if matches!(tokens.first().map(|token| &token.tok), Some(Tok::For)) {
        for token in tokens.iter().skip(1) {
            if matches!(token.tok, Tok::In) {
                break;
            }
            if let Tok::Name { name } = &token.tok {
                names.insert(name.clone());
            }
        }
    }

    remember_import_bindings(tokens, names);
}

fn remember_import_bindings(tokens: &[Token], names: &mut HashSet<String>) {
    let import_at = if matches!(tokens.first().map(|token| &token.tok), Some(Tok::Import)) {
        Some(0)
    } else if matches!(tokens.first().map(|token| &token.tok), Some(Tok::From)) {
        tokens
            .iter()
            .position(|token| matches!(token.tok, Tok::Import))
    } else {
        None
    };
    let Some(import_at) = import_at else {
        return;
    };

    let mut index = import_at + 1;
    while index < tokens.len() {
        if matches!(tokens[index].tok, Tok::Comma | Tok::Lpar | Tok::Rpar) {
            index += 1;
            continue;
        }
        let Some(name) = name_word(&tokens[index]) else {
            index += 1;
            continue;
        };
        let default_name = name.to_string();
        index += 1;
        while index + 1 < tokens.len()
            && matches!(tokens[index].tok, Tok::Dot)
            && name_word(&tokens[index + 1]).is_some()
        {
            index += 2;
        }
        let binding = if tokens
            .get(index)
            .is_some_and(|token| token_matches_exact(token, &["as"]))
        {
            index += 1;
            let alias = tokens.get(index).and_then(name_word).map(str::to_string);
            index += usize::from(alias.is_some());
            alias
        } else {
            Some(default_name)
        };
        if let Some(binding) = binding {
            names.insert(binding);
        }
        while index < tokens.len() && !matches!(tokens[index].tok, Tok::Comma) {
            index += 1;
        }
    }
}

/// Prefix under which a name that was built as a list is remembered inside
/// the ordinary name set. `[` cannot occur in a Python identifier, so this
/// marker can never collide with a name a program is able to bind, and every
/// existing `known_names.contains(...)` test is unaffected.
const LIST_NAME_MARKER: &str = "[]";

fn remember_list_name(names: &mut HashSet<String>, name: &str) {
    names.insert(format!("{LIST_NAME_MARKER}{name}"));
}

/// True when `name` was made a list earlier in the same program.
fn is_list_name(names: &HashSet<String>, name: &str) -> bool {
    names.contains(&format!("{LIST_NAME_MARKER}{name}"))
}

/// Prefix under which a name that was built as a record is remembered, the
/// same way a list name is. The two sets are separate because the statements
/// that read one way for a list and another for a record — `개수`, `빼`,
/// `넣어` — have to know which kind the name holds, and nothing in the wording
/// can tell them.
const RECORD_NAME_MARKER: &str = "[record]";

fn remember_record_name(names: &mut HashSet<String>, name: &str) {
    names.insert(format!("{RECORD_NAME_MARKER}{name}"));
}

/// True when `name` was made a record earlier in the same program.
fn is_record_name(names: &HashSet<String>, name: &str) -> bool {
    names.contains(&format!("{RECORD_NAME_MARKER}{name}"))
}

/// Prefix under which a name that was made a job is remembered.
///
/// `do`, `해` and `해줘` are ordinary words, so a line that runs a job is
/// never recognized by its verb. This set is the whole gate.
const JOB_NAME_MARKER: &str = "[job]";

/// How many things the job is given is remembered with it, so a line that
/// runs one can never hand it the wrong number of them.
fn remember_job_name(names: &mut HashSet<String>, name: &str, takes: usize) {
    names.insert(format!("{JOB_NAME_MARKER}{takes}:{name}"));
}

/// True when `name` was made a job taking `takes` things earlier in the same
/// program.
fn is_job_name(names: &HashSet<String>, name: &str, takes: usize) -> bool {
    names.contains(&format!("{JOB_NAME_MARKER}{takes}:{name}"))
}

/// Prefix under which a name a bundled module bound is remembered beside the
/// ordinary name set, the same way a list name is. `[` cannot occur in a
/// Python identifier, so it can never collide with a name a program binds.
const MODULE_NAME_MARKER: &str = "[module]";

fn remember_module_name(names: &mut HashSet<String>, name: &str) {
    names.insert(format!("{MODULE_NAME_MARKER}{name}"));
}

/// True when `name` came from a `use` line rather than from the program.
fn is_module_name(names: &HashSet<String>, name: &str) -> bool {
    names.contains(&format!("{MODULE_NAME_MARKER}{name}"))
}

fn remember_bindings(stmt: &NmeStmt, names: &mut HashSet<String>) {
    match stmt {
        NmeStmt::Append { target, .. } => {
            remember_list_name(names, target);
        }
        NmeStmt::RecordPut { target, .. } => {
            names.insert(target.clone());
            remember_record_name(names, target);
        }
        NmeStmt::Set {
            target,
            value: Value::EmptyRecord,
        } => {
            names.insert(target.clone());
            remember_record_name(names, target);
        }
        NmeStmt::Job { name, parameters } => {
            names.insert(name.clone());
            remember_job_name(names, name, parameters.len());
        }
        NmeStmt::Set {
            target,
            value: Value::List(_) | Value::Split { .. },
        } => {
            // A split hands back a list, so the name it is saved into is one:
            // `이름들은 메모를 줄마다 나눈 것` then `이름들 개수` counts them.
            names.insert(target.clone());
            remember_list_name(names, target);
        }
        NmeStmt::Ask { target, .. } | NmeStmt::Set { target, .. } => {
            names.insert(target.clone());
        }
        NmeStmt::FileRead { target, .. } => {
            names.insert(target.clone());
        }
        // The stopwatch and each cooldown bind one Python name apiece. They
        // are remembered like any other name so the parser can tell a
        // program that reads them from one that never set them.
        NmeStmt::StartTimer => {
            names.insert(TIMER_NAME.to_string());
        }
        NmeStmt::Cooldown { target, .. } => {
            names.insert(format!("{COOLDOWN_PREFIX}{target}"));
        }
        NmeStmt::ForEach {
            name,
            position,
            inline,
            ..
        } => {
            names.insert(name.clone());
            if let Some(position) = position {
                names.insert(position.clone());
            }
            if let Some(InlineStmt::Nme(inner)) = inline {
                remember_bindings(inner, names);
            }
        }
        NmeStmt::ModuleImport {
            names: imported, ..
        } => {
            for name in imported {
                names.insert(name.clone());
            }
        }
        NmeStmt::UseModule { module, .. } => {
            for name in module_binding_names(*module) {
                names.insert((*name).to_string());
                remember_module_name(names, name);
            }
        }
        NmeStmt::Forever {
            inline: Some(InlineStmt::Nme(inner)),
        }
        | NmeStmt::Times {
            inline: Some(InlineStmt::Nme(inner)),
            ..
        }
        | NmeStmt::When {
            inline: Some(InlineStmt::Nme(inner)),
            ..
        }
        | NmeStmt::While {
            inline: Some(InlineStmt::Nme(inner)),
            ..
        }
        | NmeStmt::ElseIf {
            inline: Some(InlineStmt::Nme(inner)),
            ..
        }
        | NmeStmt::Else {
            inline: Some(InlineStmt::Nme(inner)),
        } => remember_bindings(inner, names),
        _ => {}
    }
}

fn strip_target_particle(word: &str) -> &str {
    for particle in ["에게", "한테", "으로", "로", "을", "를"] {
        if let Some(base) = word.strip_suffix(particle) {
            if !base.is_empty() {
                return base;
            }
        }
    }
    word
}

/// True for `사랑한다는 말` — a whole clause, the ending that hangs it on a
/// noun, and the noun. Ten of fourteen ordinary phrases of that shape were
/// becoming assignments on 2026-08-19: `사랑한다 = "말"` binds a name that is a
/// verb and prints nothing, and the writer never finds out.
///
/// `-다는`/`-라는`/`-냐는`/`-자는` are the endings that do it. A noun really can
/// end in `다` (`바다는`, `소다는`), so the value settles the rest: a number, a
/// name the program made, or anything longer than one plain word keeps the
/// line an assignment. What is refused is exactly the shape of a noun phrase —
/// clause, ending, one bare noun — and `바다는 파랗다` is one of those too.
fn korean_quotative_noun_phrase(
    word: &str,
    value: &[Token],
    known_names: &HashSet<String>,
) -> bool {
    let Some(base) = word.strip_suffix('는') else {
        return false;
    };
    if !base.ends_with(['다', '라', '냐', '자']) {
        return false;
    }
    let [only] = value else {
        return false;
    };
    name_word(only).is_some_and(|noun| !known_names.contains(noun))
}

fn strip_assignment_particle(word: &str) -> Option<&str> {
    for particle in ["은", "는"] {
        if let Some(base) = word.strip_suffix(particle) {
            if !base.is_empty() {
                return Some(base);
            }
        }
    }
    None
}

fn resolve_known_particle<'a>(word: &'a str, known_names: &'a HashSet<String>) -> Option<&'a str> {
    if known_names.contains(word) {
        return Some(word);
    }
    for particle in KOREAN_PARTICLES {
        if let Some(base) = word.strip_suffix(particle) {
            if known_names.contains(base) {
                return Some(&word[..base.len()]);
            }
        }
    }
    None
}

fn is_connector_word(token: &Token) -> bool {
    matches!(token.tok, Tok::And)
        || token_matches_exact(token, &["and", "then", "해서", "그리고", "그러면"])
}

fn is_python_keyword(tok: &Tok) -> bool {
    matches!(
        tok,
        Tok::False
            | Tok::None
            | Tok::True
            | Tok::And
            | Tok::As
            | Tok::Assert
            | Tok::Async
            | Tok::Await
            | Tok::Break
            | Tok::Case
            | Tok::Class
            | Tok::Continue
            | Tok::Def
            | Tok::Del
            | Tok::Elif
            | Tok::Else
            | Tok::Except
            | Tok::Finally
            | Tok::For
            | Tok::From
            | Tok::Global
            | Tok::If
            | Tok::Import
            | Tok::In
            | Tok::Is
            | Tok::Lambda
            | Tok::Match
            | Tok::Nonlocal
            | Tok::Not
            | Tok::Or
            | Tok::Pass
            | Tok::Raise
            | Tok::Return
            | Tok::Try
            | Tok::Type
            | Tok::While
            | Tok::With
            | Tok::Yield
    )
}

fn find_times_colon(tokens: &[Token], mode: MatchMode) -> Option<(usize, Spelling)> {
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Name { name }
                if (word_matches(name, TIMES_KEYWORD, mode) || name == TIMES_KEYWORD_KO)
                    && depth == 0
                    && index > 0
                    && matches!(
                        tokens.get(index + 1).map(|next| &next.tok),
                        Some(Tok::Colon)
                    ) =>
            {
                return Some((
                    index,
                    if word_matches(name, TIMES_KEYWORD, mode) {
                        Spelling::English
                    } else {
                        Spelling::Korean
                    },
                ));
            }
            _ => {}
        }
    }
    None
}

fn find_condition_colon(source: &str, tokens: &[Token], condition_start: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut first = None;
    for (index, token) in tokens.iter().enumerate().skip(condition_start) {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Colon if depth == 0 => {
                first.get_or_insert(index);
                if index > condition_start {
                    let condition = Span::new(
                        tokens[condition_start].span.start,
                        tokens[index - 1].span.end,
                    );
                    if is_valid_python_expression(&source[condition.start..condition.end]) {
                        return Some(index);
                    }
                }
            }
            _ => {}
        }
    }
    first
}

fn action_phrase_at(
    tokens: &[Token],
    start: usize,
    expected: &[&str],
    mode: MatchMode,
) -> Option<usize> {
    let available = tokens.len().saturating_sub(start).min(3);
    for consumed in (1..=available).rev() {
        // Concatenating separate English words is useful for attached Korean
        // endings, but in recovery it turns ordinary prose such as `I am`
        // into the one-edit condition starter `if`. Keep typo recovery local
        // to one ASCII token; exact multi-word legacy aliases still work.
        if mode == MatchMode::Recover
            && consumed > 1
            && tokens[start..start + consumed]
                .iter()
                .all(|token| token_word(token).is_some_and(str::is_ascii))
        {
            continue;
        }
        let mut actual = String::new();
        let mut all_words = true;
        for token in &tokens[start..start + consumed] {
            if let Some(word) = token_word(token) {
                actual.push_str(word);
            } else {
                all_words = false;
                break;
            }
        }
        if !all_words {
            continue;
        }
        // A polite `주세요` glued to the word before it only spells an action
        // when that word already is one. See `POLITE_AUXILIARY_KO`.
        if consumed > 1
            && tokens
                .get(start + consumed - 1)
                .is_some_and(|token| token_matches_exact(token, POLITE_AUXILIARY_KO))
            && !head_is_exact_action(tokens, start, consumed - 1, expected)
        {
            continue;
        }
        // Several spellings of the *same* action tied at the best rank still
        // mean that action (`건너뛰여` is equally close to `건너뛰어` and
        // `건너뛰기`, and both are `continue`), so only an exact word written
        // twice in one table is treated as a table mistake worth skipping.
        if let Some((best, matches)) = best_action_rank(&actual, expected, mode) {
            if best > 0 || matches == 1 {
                return Some(consumed);
            }
        }
    }
    None
}

/// True when the words before a polite `주세요` are already an action word on
/// their own — written out rather than calling [`action_phrase_at`] again, so
/// that the check cannot ask itself the same question forever.
///
/// Either spelling of the same action counts, because Korean drops the `줘`
/// as readily as it keeps it: `보여 주세요` is `보여줘` asked politely. What
/// does not count is a single syllable. `해` is the light verb *do*, it
/// attaches to any noun in the language, and `해주세요` standing in the output
/// table is the tail of `출력해주세요` rather than a word of its own — so
/// `조용히 해 주세요` is somebody asking for quiet.
fn head_is_exact_action(tokens: &[Token], start: usize, len: usize, expected: &[&str]) -> bool {
    if len == 0 {
        return false;
    }
    let mut head = String::new();
    for token in &tokens[start..start + len] {
        match token_word(token) {
            Some(word) => head.push_str(word),
            None => return false,
        }
    }
    if head.chars().count() < 2 {
        return false;
    }
    let plain = format!("{head}줘");
    expected
        .iter()
        .any(|candidate| head.eq_ignore_ascii_case(candidate) || plain == *candidate)
}

/// The best (lowest) repair rank among `expected`, and how many candidates
/// share it. One candidate at the best rank is a confident match; several are
/// a tie, which no amount of guessing can resolve.
fn best_action_rank(actual: &str, expected: &[&str], mode: MatchMode) -> Option<(u8, usize)> {
    let mut best: Option<u8> = None;
    let mut matches = 0usize;
    for candidate in expected {
        let Some(rank) = action_recovery_rank(actual, candidate, mode) else {
            continue;
        };
        match best {
            Some(current) if rank > current => {}
            Some(current) if rank == current => matches += 1,
            _ => {
                best = Some(rank);
                matches = 1;
            }
        }
    }
    best.map(|rank| (rank, matches))
}

/// Worst repair rank an action word may still be recovered from.
const RECOVERY_RANK_WORST: u8 = 3;

/// How far a written word is from one action word, lower being a better
/// repair. Ranking matters because the Korean output words overlap by design:
/// `말해조` is a whole-word repair of `말해줘` (rank 1) but only a
/// dropped-character repair of `말해` (rank 2), so the writer plainly meant
/// `말해줘`. While both merely counted as "one edit" the match looked
/// ambiguous, recovery switched itself off, and the typo was printed instead.
fn action_recovery_rank(actual: &str, expected: &str, mode: MatchMode) -> Option<u8> {
    if actual.eq_ignore_ascii_case(expected) {
        return Some(0);
    }
    if mode == MatchMode::Exact || actual.chars().count() < 2 || is_own_vocabulary(actual) {
        return None;
    }
    let actual = actual
        .chars()
        .flat_map(char::to_lowercase)
        .collect::<String>();
    let expected = expected
        .chars()
        .flat_map(char::to_lowercase)
        .collect::<String>();
    if one_typo_away(&actual, &expected) {
        // A replaced or swapped character keeps the word whole; adding or
        // dropping one turns a longer action word into a shorter one.
        return Some(u8::from(actual.chars().count() != expected.chars().count()) + 1);
    }
    action_typo_away(&actual, &expected).then_some(RECOVERY_RANK_WORST)
}

/// Every statement vocabulary a whole line can open or close with. Used only
/// to name the candidates when a misspelling could be two of them at once.
const AMBIGUITY_TABLES: &[&[&str]] = &[
    SAY_WORDS_EN,
    SAY_WORDS_KO,
    ASK_WORDS_EN,
    ASK_WORDS_KO,
    SET_WORDS_EN,
    SET_WORDS_KO,
    REPEAT_WORDS_EN,
    REPEAT_WORDS_KO,
    WAIT_WORDS_EN,
    WAIT_WORDS_KO,
    APPEND_WORDS_EN,
    APPEND_WORDS_KO,
    UPDATE_ADD_WORDS_EN,
    UPDATE_ADD_WORDS_KO,
    UPDATE_SUBTRACT_WORDS_EN,
    UPDATE_SUBTRACT_WORDS_KO,
];

/// The action words a misspelling is equally close to. Naming them is what
/// makes the ambiguity diagnostic something a writer can act on: `대해` is one
/// edit from both `말해` and `더해`, and only the writer knows which was meant.
fn tied_action_words(tokens: &[Token]) -> Vec<String> {
    let mut positions = vec![0];
    if tokens.len() > 1 {
        positions.push(tokens.len() - 1);
    }
    let mut found: Vec<String> = Vec::new();
    for at in positions {
        let Some(actual) = tokens.get(at).and_then(token_word) else {
            continue;
        };
        for table in AMBIGUITY_TABLES {
            let Some((best, _)) = best_action_rank(actual, table, MatchMode::Recover) else {
                continue;
            };
            if best == 0 {
                continue;
            }
            found.extend(
                table
                    .iter()
                    .filter(|candidate| {
                        action_recovery_rank(actual, candidate, MatchMode::Recover) == Some(best)
                    })
                    .map(|candidate| (*candidate).to_string()),
            );
        }
    }
    found.sort();
    found.dedup();
    if found.len() < 2 {
        found.clear();
    }
    found
}

fn token_matches_exact_at(tokens: &[Token], index: usize, expected: &[&str]) -> bool {
    tokens
        .get(index)
        .is_some_and(|token| token_matches_exact(token, expected))
}

fn token_matches_exact(token: &Token, expected: &[&str]) -> bool {
    token_word(token).is_some_and(|actual| {
        expected
            .iter()
            .any(|candidate| actual.eq_ignore_ascii_case(candidate))
    })
}

fn word_matches_any(token: &Token, expected: &[&str], mode: MatchMode) -> bool {
    token_word(token).is_some_and(|actual| {
        expected
            .iter()
            .any(|candidate| word_matches(actual, candidate, mode))
    })
}

fn token_word_matches(token: &Token, expected: &str, mode: MatchMode) -> bool {
    token_word(token).is_some_and(|actual| word_matches(actual, expected, mode))
}

fn word_matches(actual: &str, expected: &str, mode: MatchMode) -> bool {
    if actual.eq_ignore_ascii_case(expected) {
        return true;
    }
    if mode == MatchMode::Exact || actual.chars().count() < 2 || is_own_vocabulary(actual) {
        return false;
    }
    action_typo_away(
        &actual
            .chars()
            .flat_map(char::to_lowercase)
            .collect::<String>(),
        &expected
            .chars()
            .flat_map(char::to_lowercase)
            .collect::<String>(),
    )
}

/// A word the sentence grammar already uses for one thing is never read as a
/// misspelling of something else.
///
/// `가장` is one substitution away from `저장` (save), and `가장 좋은
/// 하루였습니다` was quietly stored in a name called `좋` because of it. A word
/// NME spells out itself has a meaning of its own, so guessing a different
/// one from it can only be wrong.
fn is_own_vocabulary(word: &str) -> bool {
    // An English word the compiler already knows as one a beginner writes
    // *instead of* an action word is never repaired into that action. The
    // tables exist to reject those words and say what to write; repairing one
    // would be the translation they exist to refuse — `let score be 0` saved
    // the text `"be 0"`, and `Let's not talk about it tonight.` made a value
    // called `s`. Korean is left out on purpose: `말해라` is in the same table
    // and is one edit from `말해줘`, and repairing it is what a Korean writer
    // gets today.
    if word.is_ascii() && is_mistaken_action_word(word) {
        return true;
    }
    // The one Korean exception, and it is not a mistaken action word: a
    // polite `주세요` is one character from `해주세요`, and it is how nearly
    // every Korean request is written. See `POLITE_AUXILIARY_KO`.
    if POLITE_AUXILIARY_KO.contains(&word) {
        return true;
    }
    [
        // `to`, `by`, `of`, `into`, `onto`, `from` join the parts of a
        // statement together, and `and`/`or` join two of anything. `to` is
        // one letter from the repeat alias `do`, and `to 0 set score` was
        // read as "repeat 0 times" because of it; `and` is one letter from
        // `add`, and `She was born in 1952 and died in 2019.` became
        // `She = She + (died in 2019.)`. A word NME already spells out has a
        // job of its own.
        UPDATE_CONNECTOR_WORDS_EN,
        APPEND_CONNECTORS_EN,
        CHOICE_SEPARATOR_WORDS,
        READING_LEAD_WORDS_EN,
        DIVIDED_WORDS_KO,
        REMAINDER_WORDS_KO,
        EXTREME_SCOPE_WORDS_KO,
        EXTREME_MOST_WORDS_KO,
        EXTREME_THING_WORDS_KO,
        SEPARATOR_WORDS_KO,
        EMPTY_WORDS_KO,
    ]
    .iter()
    .any(|words| words.contains(&word))
}

/// Action words tolerate one edit, plus the common two-keystroke typo where a
/// single extra/missing character is combined with a swap or replacement.
/// The match remains candidate-unique in `action_phrase_at`, so broad prose
/// is never silently assigned an arbitrary action.
fn action_typo_away(actual: &str, expected: &str) -> bool {
    if one_typo_away(actual, expected) {
        return true;
    }
    let actual_chars = actual.chars().collect::<Vec<_>>();
    let expected_chars = expected.chars().collect::<Vec<_>>();
    if actual_chars.len().abs_diff(expected_chars.len()) > 2 {
        return false;
    }
    for index in 1..actual_chars.len() {
        let mut shortened = actual_chars.clone();
        shortened.remove(index);
        if adjacent_transposition_away(&shortened, &expected_chars) {
            return true;
        }
    }
    for index in 0..expected_chars.len() {
        let mut shortened = expected_chars.clone();
        shortened.remove(index);
        if adjacent_transposition_away(&actual_chars, &shortened) {
            return true;
        }
    }
    false
}

fn adjacent_transposition_away(left: &[char], right: &[char]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let differences = left
        .iter()
        .zip(right)
        .enumerate()
        .filter_map(|(index, (a, b))| (a != b).then_some(index))
        .collect::<Vec<_>>();
    differences.len() == 2
        && differences[1] == differences[0] + 1
        && left[differences[0]] == right[differences[1]]
        && left[differences[1]] == right[differences[0]]
}

fn condition_word_matches(actual: &str, expected: &[&str]) -> bool {
    expected.iter().any(|candidate| {
        actual.eq_ignore_ascii_case(candidate)
            || (actual != "the" && actual.chars().count() >= 2 && one_typo_away(actual, candidate))
    })
}

/// True when an exact output action word stands before `index` on this line.
///
/// Korean writes the action last, so a block word that *follows* an output
/// word is part of the message (`안녕 말해줘 동안`, `천천히 말해줘 커서가
/// 깜빡이는 동안`), never a loop header. The rule is positional, not lexical:
/// `커서가 깜빡이는 동안 말해줘` still opens nothing and prints, because there
/// the output word comes after the block word. English needs no such rule,
/// because its output word already opens the line.
fn output_word_before(tokens: &[Token], index: usize) -> bool {
    (0..index.min(tokens.len()))
        .any(|start| output_action_at(tokens, start, MatchMode::Exact).is_some())
}

/// True when an exact output action word ends exactly at `index`. A block
/// word written straight after the message (`안녕 말해줘 아니면`) closes
/// nothing: everything after the output word is what gets printed.
fn output_word_ends_just_before(tokens: &[Token], index: usize) -> bool {
    (0..index).any(|start| {
        output_action_at(tokens, start, MatchMode::Exact)
            .is_some_and(|(_, consumed)| start + consumed == index)
    })
}

fn output_action_ending(
    tokens: &[Token],
    mode: MatchMode,
    known_names: &HashSet<String>,
) -> Option<(usize, Spelling, usize)> {
    let mut end = tokens.len();
    while end > 1
        && (is_command_ending(&tokens[end - 1]) || matches!(tokens[end - 1].tok, Tok::Comma))
    {
        end -= 1;
    }
    let start_at = end.saturating_sub(3);
    for start in start_at..end {
        if let Some((spelling, consumed)) = output_action_at(tokens, start, mode) {
            if start + consumed == end {
                return Some((start, spelling, end));
            }
        }
        if let Some(consumed) = trailing_output_action_at(tokens, start, known_names) {
            if start + consumed == end {
                return Some((start, Spelling::Korean, end));
            }
        }
    }
    None
}

/// An output word that counts only where Korean puts its verb — at the end of
/// the line, with a message in front of it.
///
/// These are never repaired from a misspelling. A one-edit guess at a word
/// this short would claim half the language: `말` is one character from `물`,
/// `발`, `날` and `살`.
fn trailing_output_action_at(
    tokens: &[Token],
    start: usize,
    known_names: &HashSet<String>,
) -> Option<usize> {
    if start == 0 || korean_makes_the_next_word_a_noun(&tokens[start - 1]) {
        return None;
    }
    let token = tokens.get(start)?;
    if token_matches_exact(token, SAY_TRAILING_WORDS_KO) {
        return Some(1);
    }
    if token_matches_exact(token, SAY_SHORT_WORDS_KO)
        && message_reads_like_speech(&tokens[start - 1], known_names)
    {
        return Some(1);
    }
    None
}

/// True when the word in front of the one-syllable `말` is something a
/// program would say.
///
/// One syllable is not much to go on, so this asks for more than the noun
/// phrase test: the message has to end the way spoken Korean ends
/// (`안녕하세요 말`, `고맙습니다 말`), or be a name the program already made
/// (`점수 말`), or not be Korean at all (`hello 말`). A bare Korean noun is
/// not enough, because `엄마 말`, `친구 말` and `농담 말` are all noun phrases
/// somebody wrote.
fn message_reads_like_speech(token: &Token, known_names: &HashSet<String>) -> bool {
    let Some(word) = name_word(token) else {
        // A number or a quoted piece of text is a value, never a noun phrase.
        return true;
    };
    if !is_hangul(word) || resolve_known_particle(word, known_names).is_some() {
        return true;
    }
    if is_korean_adnominal(word) {
        // `좋은 말`, `할 말`, `사랑한다는 말`.
        return false;
    }
    SENTENCE_ENDINGS_KO
        .iter()
        .any(|ending| word.ends_with(ending))
        || word.ends_with('요')
        || word.ends_with('다')
}

/// True when this Korean word makes the noun after it part of a noun phrase.
///
/// Korean puts everything that describes a noun in front of it, and only two
/// shapes stand there: an adnominal verb ending (`좋은 말`, `사랑한다는 말`,
/// `할 말`) and a determiner (`그 말`, `무슨 말`). Neither can be the subject
/// of a command, so `말` after one of them is the ordinary noun *word*.
fn korean_makes_the_next_word_a_noun(token: &Token) -> bool {
    token_matches_exact(token, KOREAN_DETERMINERS)
        || name_word(token).is_some_and(joins_two_nouns)
}

/// Particles that tie the word they are on to the noun after it: `듣기와
/// 말하기`, `글쓰기보다 말하기`, `친구의 말`. A word wearing one of them is
/// half of a noun phrase, never the thing a command is about.
fn joins_two_nouns(word: &str) -> bool {
    ["이랑", "하고", "보다", "처럼", "같이", "부터", "까지", "와", "과", "랑", "의"]
        .iter()
        .any(|particle| {
            word.strip_suffix(particle)
                .is_some_and(|base| !base.is_empty())
        })
}

/// True when a Korean word ends in the `ㄴ` or `ㄹ` that turns a verb into a
/// description of the noun after it: `좋은`, `하는`, `할`, `보던`.
///
/// The object and topic particles end the same way — `결과를` and `점수는` both
/// close with `ㄹ`/`ㄴ` — so this is asked only of the one-syllable `말`, and
/// only after a name the program made has already been let through. On the
/// longer words it would cost the commonest command shape there is:
/// `결과를 알려줘`.
fn is_korean_adnominal(word: &str) -> bool {
    let Some(last) = word.chars().last() else {
        return false;
    };
    let code = last as u32;
    if !(0xac00..=0xd7a3).contains(&code) {
        return false;
    }
    // The 28 final consonants of a Hangul syllable, in Unicode order.
    // 4 is `ㄴ` and 8 is `ㄹ`.
    matches!((code - 0xac00) % 28, 4 | 8)
}

fn trim_suffix_say_value(tokens: &[Token]) -> Vec<Token> {
    let mut value = tokens.to_vec();
    while value
        .last()
        .is_some_and(|token| token_matches_exact(token, &["라고", "이라고", "하고", "을", "를"]))
    {
        value.pop();
    }
    if let Some(last) = value.last_mut() {
        trim_name_token_suffix(last, &["이라고", "라고", "하고", "을", "를"]);
    }
    value
}

fn trim_name_token_suffix(token: &mut Token, suffixes: &[&str]) -> bool {
    let Some(word) = name_word(token) else {
        return false;
    };
    let Some(base) = strip_any_suffix(word, suffixes) else {
        return false;
    };
    let removed = word.len() - base.len();
    token.tok = Tok::Name {
        name: base.to_string(),
    };
    token.span.end = token.span.end.saturating_sub(removed);
    true
}

fn strip_any_suffix<'a>(word: &'a str, suffixes: &[&str]) -> Option<&'a str> {
    let mut ordered = suffixes.to_vec();
    ordered.sort_by_key(|suffix| std::cmp::Reverse(suffix.len()));
    ordered
        .into_iter()
        .find_map(|suffix| word.strip_suffix(suffix).filter(|base| !base.is_empty()))
}

fn literal_token(token: &Token) -> Option<Literal> {
    match &token.tok {
        Tok::True => Some(Literal::True),
        Tok::False => Some(Literal::False),
        Tok::None => Some(Literal::None),
        Tok::Name { name } if name.eq_ignore_ascii_case("true") || name == "참" => {
            Some(Literal::True)
        }
        Tok::Name { name } if name.eq_ignore_ascii_case("false") || name == "거짓" => {
            Some(Literal::False)
        }
        Tok::Name { name }
            if name.eq_ignore_ascii_case("none")
                || name.eq_ignore_ascii_case("null")
                || name == "없음" =>
        {
            Some(Literal::None)
        }
        _ => None,
    }
}

fn is_code_token(token: &Token) -> bool {
    !matches!(token.tok, Tok::Name { .. })
}

fn is_text_token(token: &Token) -> bool {
    matches!(token.tok, Tok::Name { .. } | Tok::String { .. })
}

fn is_command_ending(token: &Token) -> bool {
    matches!(token.tok, Tok::Dot) || token_matches_exact(token, COMMAND_ENDINGS)
}

fn looks_like_python_invocation(tokens: &[Token]) -> bool {
    tokens.len() > 1
        && name_word(&tokens[0]).is_some()
        && matches!(tokens[1].tok, Tok::Lpar | Tok::Dot | Tok::Lsqb)
}

fn looks_like_future_python(tokens: &[Token]) -> bool {
    tokens.windows(2).any(|pair| {
        matches!(pair[0].tok, Tok::Name { .. })
            && matches!(pair[1].tok, Tok::String { .. })
            && pair[0].span.end == pair[1].span.start
    })
}

fn looks_like_plain_prose(tokens: &[Token]) -> bool {
    tokens
        .iter()
        .enumerate()
        .all(|(index, token)| is_sentence_word_token(index, token))
}

/// A written line that is words and nothing but words, except that it is
/// allowed to carry numbers.
///
/// Prices, ages, times, dates, room and chapter numbers are what people put
/// in sentences, and one digit anywhere used to switch the sentence path off
/// for the whole line: `The soup needs cream.` printed and `The soup needs
/// 250 ml of cream.` did not. Korean already had this through
/// [`is_written_korean_sentence`]; English had nothing.
///
/// This is deliberately *not* [`looks_like_plain_prose`]: the checks that
/// decide whether valid Python is really a sentence keep the stricter
/// reading, so `score is 0` is still named as a line that does nothing.
fn looks_like_written_prose(tokens: &[Token]) -> bool {
    tokens.iter().enumerate().all(|(index, token)| {
        is_sentence_word_token(index, token)
            || matches!(
                token.tok,
                Tok::Int { .. } | Tok::Float { .. } | Tok::Percent
            )
    })
}

/// A token that can be part of a written sentence. A Python keyword that only
/// ever opens a statement counts when it is not the first word, because it
/// cannot be code there: `insert coin to continue` is a sentence, while a
/// line that starts with `continue` is Python's own statement.
fn is_sentence_word_token(index: usize, token: &Token) -> bool {
    token_word(token).is_some()
        || is_command_ending(token)
        || matches!(token.tok, Tok::Comma | Tok::In | Tok::Not)
        || (index > 0
            && matches!(
                token.tok,
                Tok::Break
                    | Tok::Continue
                    | Tok::Pass
                    | Tok::Return
                    | Tok::Raise
                    | Tok::Del
                    | Tok::Assert
                    | Tok::Global
                    | Tok::Nonlocal
                    | Tok::Def
                    | Tok::Class
                    | Tok::Try
                    | Tok::Except
                    | Tok::Finally
                    | Tok::With
                    | Tok::For
                    | Tok::While
            ))
}

fn has_recoverable_sentence_shape(tokens: &[Token], known_names: &HashSet<String>) -> bool {
    has_recoverable_repeat_shape(tokens)
        || recoverable_output_shape(tokens, known_names)
        || find_ask_shape(tokens, MatchMode::Recover).is_some()
        || (set_action_at(tokens, 0, MatchMode::Recover).is_some() && tokens.len() > 1)
        || recoverable_module_shape(tokens)
        || (action_phrase_at(tokens, 0, USE_WORDS_EN, MatchMode::Recover).is_some()
            && tokens.len() > 1)
        || (action_phrase_at(tokens, 0, USE_WORDS_KO, MatchMode::Recover).is_some()
            && tokens.len() > 1)
        || has_recoverable_append_shape(tokens)
        || english_for_each_start(tokens, MatchMode::Recover).is_some()
        || korean_for_each_shape(tokens)
}

/// True when an output word could really claim this line, which is the same
/// question [`match_say`] answers.
///
/// The two must agree. A line the output matcher will decline is not a
/// recoverable sentence shape, and calling it one sent the line to the
/// recovery round instead of the sentence path: `quick and lazy are fun words
/// to say` was answered by a near miss at a *value change*, because `say`
/// stands at the end of it.
fn recoverable_output_shape(tokens: &[Token], known_names: &HashSet<String>) -> bool {
    if let Some((_, consumed)) = output_action_at(tokens, 0, MatchMode::Recover) {
        // At the start of a line the rule is the same in both languages.
        let repaired = output_action_at(tokens, 0, MatchMode::Exact).is_none();
        if !repaired || output_repair_claims_one_word(&tokens[consumed..]) {
            return true;
        }
    }
    if let Some((start, spelling, _)) = output_action_ending(tokens, MatchMode::Recover, known_names)
    {
        let repaired = spelling == Spelling::English
            && output_action_ending(tokens, MatchMode::Exact, known_names).is_none();
        if !repaired || output_repair_claims_one_word(&tokens[..start]) {
            return true;
        }
    }
    false
}

/// True when a misspelled list-adding word closes or opens the line, which is
/// the only thing that separates `appned Mina to friends` from ordinary
/// prose. The exact spellings never reach here; they matched already.
fn has_recoverable_append_shape(tokens: &[Token]) -> bool {
    if tokens.len() < 3 {
        return false;
    }
    if action_phrase_at(tokens, 0, APPEND_WORDS_EN, MatchMode::Exact).is_none()
        && action_phrase_at(tokens, 0, APPEND_WORDS_EN, MatchMode::Recover).is_some()
        && tokens
            .iter()
            .any(|token| token_matches_exact(token, APPEND_CONNECTORS_EN))
    {
        return true;
    }
    korean_append_action_start(tokens, MatchMode::Exact).is_none()
        && korean_append_action_start(tokens, MatchMode::Recover).is_some()
}

// ------------------------------------------------- words that are not actions
//
// A beginner reaches for a word NME does not accept far more often than for
// one it does. Two things must never happen then: CPython's own
// `SyntaxError`, whose caret lands in the middle of a Hangul syllable and
// whose text is English only, and the prose fallback quietly turning the
// command into `print("output hello")`. Both are replaced by a diagnostic
// that names the word and offers the action word NME does know.

/// Words a beginner writes where an action word belongs, with the action
/// NME actually accepts. These are **rejected, never translated**: guessing
/// that `output` meant `show` would be exactly the silent rewrite this
/// compiler exists to avoid.
/// Words a beginner writes where an action word belongs, with the action NME
/// actually accepts. They are **rejected, never translated**: guessing that
/// `output` meant `show` would be exactly the silent rewrite this compiler
/// exists to avoid.
///
/// The table is consulted in two places with different strictness. Inside the
/// body of a repeat sentence (`3번 ... 안녕 말해줘`) the line is already known
/// to be a command, so every word here counts. On an ordinary line, only the
/// three groups below count, and each of them asks for the shape of its own
/// action first — otherwise `store it away`, `echo of the mountain` and
/// `말하기 연습` would stop being the sentences they are.
const NEAR_MISS_ACTIONS: &[(&str, &str)] = &[
    ("output", "show"),
    ("speak", "show"),
    ("puts", "show"),
    ("printf", "show"),
    ("echo", "show"),
    ("log", "show"),
    ("insert", "add"),
    ("input", "ask"),
    ("store", "set"),
    ("let", "set"),
    ("delay", "wait"),
    ("loop", "repeat"),
    ("말하기", "말해줘"),
    ("말해라", "말해줘"),
    ("말합니다", "말해줘"),
    ("출력하기", "말해줘"),
    ("프린트해", "말해줘"),
    ("보여주기", "보여줘"),
    ("입력해", "물어봐"),
    ("무러봐", "물어봐"),
    ("돌려서", "반복해서"),
    ("루프해서", "반복해서"),
    ("반복합니다", "반복해"),
    ("반복하기", "반복해"),
    ("집어넣어", "넣어"),
    ("너허", "넣어"),
    ("정해", "저장해"),
    ("지정해", "저장해"),
];

/// Words that are only ever an attempt at a command when they open a line.
/// None of them starts an ordinary English or Korean sentence.
const COMMAND_WORDS_LEADING: &[(&str, &str)] = &[
    ("output", "show"),
    ("speak", "show"),
    ("puts", "show"),
    ("printf", "show"),
    ("출력하기", "말해줘"),
    ("프린트해", "말해줘"),
    ("보여주기", "보여줘"),
    ("말합니다", "말해줘"),
    ("말해라", "말해줘"),
];

/// Words for asking. Korean puts them in the middle of the line, so they are
/// looked for everywhere — but only when the line really asks something and
/// ends in a question mark. `입력해 주세요` is then still a sentence.
/// Words that are an attempt at a command only when they open a line **and**
/// take a single word after them.
///
/// Every one of them is an ordinary English verb as well, so a whole sentence
/// is never claimed: `write it down before you forget`, `echo of the mountain`
/// and `log the miles you walked this week` all keep their words. `write
/// hello` keeps none of them — nothing in English writes a greeting at
/// nothing — and it used to print itself, which is the failure this table
/// exists to end.
///
/// `write` is not translated into `show` for the same reason `output` is not:
/// `write text to "notes.txt"` is the file statement, so `write` already has
/// a job here, and guessing between the two would be worse than asking.
const COMMAND_WORDS_LEADING_ONE_WORD: &[(&str, &str)] = &[
    ("write", "show"),
    ("echo", "show"),
    ("log", "show"),
    ("puts", "show"),
    ("read", "ask"),
];

const COMMAND_WORDS_ASKING: &[(&str, &str)] = &[
    ("input", "ask"),
    ("입력해", "물어봐"),
    ("무러봐", "물어봐"),
];

/// Words for adding to a list, claimed only when the line names a list the
/// program already has. `insert coin to continue` names none, so it prints.
const COMMAND_WORDS_APPENDING: &[(&str, &str)] = &[
    ("insert", "add"),
    ("put", "add"),
    ("집어넣어", "넣어"),
    ("너허", "넣어"),
];

/// Further words seen in beginner corpora. They only enrich a message on a
/// line NME is refusing anyway, so a word here never turns text into an
/// error by itself.
const MISTAKEN_ACTIONS: &[(&str, &str)] = &[
    ("write", "show"),
    ("read", "ask"),
    ("get", "ask"),
    ("assign", "set"),
    ("make", "set"),
    ("bump", "add"),
    ("raise", "add"),
    ("hold", "wait"),
    ("rest", "wait"),
    ("iterate", "repeat"),
    ("put", "add"),
    ("대기", "기다려"),
    ("대기해", "기다려"),
    ("대기해줘", "기다려"),
    ("기다립니다", "기다려"),
    ("멈춰줘", "기다려"),
    ("잠시멈춰", "기다려"),
    ("슬립", "기다려"),
    ("증가해", "더해"),
    ("증가시켜", "더해"),
    ("감소해", "빼"),
    ("감소시켜", "빼"),
    ("무러봐", "물어봐"),
    ("너허", "넣어"),
];

/// Action words offered when nothing closer is known, one list per script so
/// a Korean line is never answered with an English word.
const BASIC_ACTIONS_EN: &[&str] = &["show", "ask", "set", "wait", "repeat"];
const BASIC_ACTIONS_KO: &[&str] = &["말해줘", "물어봐", "저장해", "기다려", "반복해"];

fn is_hangul(word: &str) -> bool {
    word.chars().any(|character| {
        matches!(character,
            '\u{ac00}'..='\u{d7a3}' | '\u{1100}'..='\u{11ff}' | '\u{3130}'..='\u{318f}')
    })
}

/// The action word closest to `word`, or `None` when nothing is close enough
/// to be worth naming.
fn suggest_action_word(word: &str) -> Option<&'static str> {
    for (written, action) in NEAR_MISS_ACTIONS.iter().chain(MISTAKEN_ACTIONS) {
        if word.eq_ignore_ascii_case(written) {
            return Some(action);
        }
    }
    let basics = if is_hangul(word) {
        BASIC_ACTIONS_KO
    } else {
        BASIC_ACTIONS_EN
    };
    let lowered = word.to_lowercase();
    basics
        .iter()
        .copied()
        .find(|action| one_typo_away(&lowered, action))
        .or_else(|| {
            // Korean verbs are inflected, so a shared stem of two or more
            // syllables is a stronger signal than a letter-by-letter edit
            // count (`기다립니다` against `기다려`).
            is_hangul(word)
                .then(|| {
                    basics.iter().copied().find(|action| {
                        let stem: String = action.chars().take(2).collect();
                        stem.chars().count() == 2 && word.starts_with(&stem)
                    })
                })
                .flatten()
        })
}

/// The word a beginner wrote where an action word belongs, when the rest of
/// the line has the shape of that action too. Prose is never claimed: the
/// leading group must open the line, the asking group needs a question mark,
/// and the adding group needs a list the program already has.
fn near_miss_action_word(
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<(usize, &'static str)> {
    let start = leading_sentence_fillers(tokens);
    if tokens.len() - start < 2 {
        return None;
    }
    let lookup = |word: &str, table: &'static [(&'static str, &'static str)]| {
        table
            .iter()
            .find(|(written, _)| word.eq_ignore_ascii_case(written))
            .map(|(_, action)| *action)
    };
    if let Some(action) =
        name_word(&tokens[start]).and_then(|word| lookup(word, COMMAND_WORDS_LEADING))
    {
        return Some((start, action));
    }
    // One word of message, and beside it one of the verbs that is also an
    // ordinary English verb. English writes the action first and NME also
    // takes it after the message (`hello show`), so both ends are asked.
    // See `COMMAND_WORDS_LEADING_ONE_WORD`.
    let body = trim_command_endings(tokens);
    if body.len() - start == 2 {
        for (index, other) in [(start, start + 1), (start + 1, start)] {
            // The other word has to be one a message could be made of.
            // `log out`, `read on` and `echo back` are English phrasal verbs,
            // and `out`, `on` and `back` are in `NOT_A_NAME_EN`.
            if !name_word(&body[other]).is_some_and(is_bindable_english_name) {
                continue;
            }
            if let Some(action) =
                name_word(&body[index]).and_then(|word| lookup(word, COMMAND_WORDS_LEADING_ONE_WORD))
            {
                return Some((index, action));
            }
        }
    }
    let asks = tokens
        .last()
        .is_some_and(|token| token_matches_exact(token, &["?"]));
    let names_a_list = tokens.iter().any(|token| {
        name_word(token).is_some_and(|word| resolve_known_particle(word, known_names).is_some())
    });
    (start..tokens.len()).find_map(|index| {
        let word = name_word(&tokens[index])?;
        if asks {
            if let Some(action) = lookup(word, COMMAND_WORDS_ASKING) {
                return Some((index, action));
            }
        }
        if names_a_list {
            if let Some(action) = lookup(word, COMMAND_WORDS_APPENDING) {
                return Some((index, action));
            }
        }
        None
    })
}

/// True when a word a beginner writes where an action word belongs stands on
/// the line, spelled exactly as one of the tables spells it.
fn is_mistaken_action_word(word: &str) -> bool {
    NEAR_MISS_ACTIONS
        .iter()
        .chain(MISTAKEN_ACTIONS)
        .any(|(written, _)| word.eq_ignore_ascii_case(written))
}

/// True when the first or the last word of the line is an action word, or a
/// word a beginner writes where an action word belongs.
///
/// English states its action first and Korean states it last, so those two
/// places are where a command word can be doing a command's job. This is what
/// separates `hold 2 seconds` — a wait written with the wrong word — from
/// `Room 214 is at the end of the corridor`, where `end` is only part of the
/// sentence. It is asked only of a line that carries a number, which is the
/// one case where the sentence path had to be widened.
fn opens_or_closes_with_a_command_word(tokens: &[Token]) -> bool {
    let words = tokens.iter().filter_map(name_word).collect::<Vec<_>>();
    if [words.first().copied(), words.last().copied()]
        .into_iter()
        .flatten()
        .any(|word| is_action_word(word) || is_mistaken_action_word(word))
    {
        return true;
    }
    // A number written straight beside a command word is the shape of a
    // command with a count — `wait 3`, `3 times`, `to 0 set score` — and not
    // the shape of a sentence. `Room 214 is at the end of the corridor.` puts
    // its number nowhere near `end`, so it stays a sentence.
    (0..tokens.len()).any(|index| {
        let numeric = |at: usize| {
            tokens
                .get(at)
                .is_some_and(|token| matches!(token.tok, Tok::Int { .. } | Tok::Float { .. }))
        };
        name_word(&tokens[index]).is_some_and(is_action_word)
            && (numeric(index + 1) || (index > 0 && numeric(index - 1)))
    })
}

/// True when the line is written prose: either words alone, or words and
/// numbers with no command word at either end.
fn is_written_prose_line(tokens: &[Token]) -> bool {
    looks_like_plain_prose(tokens)
        || (looks_like_written_prose(tokens) && !opens_or_closes_with_a_command_word(tokens))
}

/// True when every token on the line is a word, a number, a quoted piece of
/// text, or a sentence mark. That is the shape of something a person wrote as
/// a sentence; Python code always brings an operator, a bracket, or a colon.
fn looks_like_written_sentence(tokens: &[Token]) -> bool {
    tokens.iter().enumerate().all(|(index, token)| {
        is_sentence_word_token(index, token)
            || matches!(
                token.tok,
                Tok::None
                    | Tok::True
                    | Tok::False
                    | Tok::Int { .. }
                    | Tok::Float { .. }
                    | Tok::String { .. }
            )
    })
}

/// Which word to name when the line has no action NME knows. English states
/// the action first, Korean states it last, so each script is asked about the
/// end of the line where its verb belongs.
fn unreadable_action_token(tokens: &[Token], known_names: &HashSet<String>) -> usize {
    if let Some((index, _)) = near_miss_action_word(tokens, known_names) {
        return index;
    }
    // `please` and `좀` are politeness, never the action, so they are never
    // the word a beginner is told about.
    let start = leading_sentence_fillers(tokens);
    let hangul_line = tokens.iter().filter_map(name_word).any(is_hangul);
    if hangul_line {
        (start..tokens.len())
            .rev()
            .find(|index| name_word(&tokens[*index]).is_some())
            .unwrap_or(tokens.len() - 1)
    } else {
        (start..tokens.len())
            .find(|index| name_word(&tokens[*index]).is_some())
            .unwrap_or(start.min(tokens.len() - 1))
    }
}

fn unknown_action_word_diagnostic(tokens: &[Token], known_names: &HashSet<String>) -> Diagnostic {
    let index = unreadable_action_token(tokens, known_names);
    let token = &tokens[index];
    let word = name_word(token).unwrap_or("");
    let problem = Diagnostic::bilingual(
        DiagnosticCode::UnknownActionWord,
        format!("I don't know what `{word}` does"),
        format!("`{word}`가 무엇을 하는 말인지 모르겠어요"),
        token.span,
    );
    match suggest_action_word(word) {
        Some(action) => problem.with_bilingual_hint(
            format!("`{word}` is not an action word here; did you mean `{action}`?"),
            format!("여기서 `{word}`는 동작 단어가 아니에요. 혹시 `{action}`인가요?"),
        ),
        None => problem.with_bilingual_hint(
            format!(
                "write an action word such as {}",
                BASIC_ACTIONS_EN
                    .iter()
                    .map(|action| format!("`{action}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "{} 같은 동작 단어를 적어 주세요",
                BASIC_ACTIONS_KO
                    .iter()
                    .map(|action| format!("`{action}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ),
    }
}

/// Every action word NME accepts, in both languages, for the one job of
/// telling a beginner where a glued-together word should come apart.
const ALL_ACTION_WORDS: &[&[&str]] = &[
    SAY_WORDS_EN,
    SAY_WORDS_KO,
    ASK_WORDS_EN,
    ASK_WORDS_KO,
    SET_WORDS_EN,
    SET_WORDS_KO,
    WAIT_WORDS_EN,
    WAIT_WORDS_KO,
    REPEAT_WORDS_EN,
    REPEAT_WORDS_KO,
    UPDATE_ADD_WORDS_EN,
    UPDATE_ADD_WORDS_KO,
    UPDATE_SUBTRACT_WORDS_EN,
    UPDATE_SUBTRACT_WORDS_KO,
    APPEND_WORDS_EN,
    APPEND_WORDS_KO,
    END_WORDS_EN,
    END_WORDS_KO,
];

fn is_action_word(word: &str) -> bool {
    ALL_ACTION_WORDS
        .iter()
        .any(|list| list.iter().any(|known| word.eq_ignore_ascii_case(known)))
}

/// Korean counters stay attached to their number, so `3번` is one word and
/// `3번반복해서` comes apart as `3번` + `반복해서`.
const ATTACHED_COUNTERS_KO: &[char] = &['번', '회', '차', '판', '초'];

/// The spaces a beginner left out. `sayhello` is `say hello`, `점수는0` is
/// `점수는 0`, and `점수에1더해` is `점수에 1 더해`. Returns `None` when the
/// word is just a word.
fn unglue(word: &str) -> Option<String> {
    let mut pieces: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut previous_digit = false;
    for character in word.chars() {
        let digit = character.is_ascii_digit();
        let counter = ATTACHED_COUNTERS_KO.contains(&character);
        if !current.is_empty() && digit != previous_digit && !(previous_digit && counter) {
            pieces.push(std::mem::take(&mut current));
        }
        current.push(character);
        previous_digit = digit || (previous_digit && counter);
    }
    if !current.is_empty() {
        pieces.push(current);
    }
    let split = pieces
        .into_iter()
        .flat_map(|piece| split_off_action_word(&piece))
        .collect::<Vec<_>>();
    // Two pieces alone prove nothing: `data1` is one name. A split is only
    // worth showing when one of its pieces is a word NME knows.
    let found_a_word = split
        .iter()
        .any(|piece| is_action_word(piece) || strip_assignment_particle(piece).is_some());
    (split.len() > 1 && found_a_word).then(|| split.join(" "))
}

/// One piece of a glued word, cut where an action word starts or ends.
fn split_off_action_word(piece: &str) -> Vec<String> {
    let boundaries = piece
        .char_indices()
        .map(|(offset, _)| offset)
        .chain(std::iter::once(piece.len()))
        .collect::<Vec<_>>();
    for &at in &boundaries {
        if at == 0 || at == piece.len() {
            continue;
        }
        let (left, right) = piece.split_at(at);
        // Both halves must be real words. Without this, `Don't stop!` comes
        // apart as `Do` + `n`, because `do` is one of the repeat words.
        //
        // English needs a longer half than Korean does. Two ASCII letters are
        // not a word, and letting them count read `doctor` as `do ctor` and
        // `finished` as `finish ed`, so `story of a small town doctor` was
        // refused with a suggestion nobody could act on. A Korean word really
        // can be two characters (`안녕말해줘` is `안녕` + `말해줘`).
        // English needs longer halves than Korean does. Two or three ASCII
        // letters left over are not a word, and letting them count read
        // `doctor` as `do ctor`, `finished` as `finish ed`, `friend` as `fri
        // end` and `telling` as `tell ing` — so `story of a small town
        // doctor` was refused with a suggestion nobody could act on. The half
        // that is the action word may be short (`say`); the half that is left
        // over may not. A Korean word really can be two characters
        // (`안녕말해줘` is `안녕` + `말해줘`).
        let long_enough = |piece: &str, least: usize| {
            piece.chars().count() >= if piece.is_ascii() { least } else { 2 }
        };
        let split_here = |action: &str, rest: &str| {
            is_action_word(action) && long_enough(action, 3) && long_enough(rest, 5)
        };
        if split_here(left, right) || split_here(right, left) {
            return vec![left.to_string(), right.to_string()];
        }
        if strip_assignment_particle(left).is_some() && right.chars().all(|c| c.is_ascii_digit()) {
            return vec![left.to_string(), right.to_string()];
        }
    }
    vec![piece.to_string()]
}

/// A word on the line that is two words with the space missing, such as
/// `wait3` or `3번반복해서`. Only the first and last word are considered,
/// because that is where an action word belongs.
fn glued_action_word(tokens: &[Token]) -> Option<(usize, String)> {
    if tokens.len() < 2 {
        return None;
    }
    let last = tokens.len() - 1;
    [0, last].into_iter().find_map(|index| {
        let word = name_word(&tokens[index])?;
        if is_action_word(word) {
            return None;
        }
        let split = unglue(word)?;
        split
            .split(' ')
            .any(is_action_word)
            .then_some((index, split))
    })
}

/// `3번반복해서 안녕 말해줘` — the counter and the repeat word were typed with
/// the space missing, so the repeat word vanishes into the printed message.
/// A number in front and an exact repeat word behind make this unmistakable,
/// which is why an ordinary name such as `반복횟수는` is never claimed.
fn glued_count_and_repeat(tokens: &[Token]) -> Option<(usize, String)> {
    (1..tokens.len()).find_map(|index| {
        if !is_written_number(&tokens[index - 1]) {
            return None;
        }
        let word = name_word(&tokens[index])?;
        TIMES_WORDS_KO.iter().find_map(|marker| {
            let rest = word.strip_prefix(marker)?;
            REPEAT_WORDS_KO
                .contains(&rest)
                .then(|| (index, format!("{marker} {rest}")))
        })
    })
}

fn glued_word_diagnostic(token: &Token, word: &str, split: &str) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnknownActionWord,
        format!("I don't know what `{word}` does"),
        format!("`{word}`가 무엇을 하는 말인지 모르겠어요"),
        token.span,
    )
    .with_bilingual_hint(
        format!("`{word}` looks like two words with the space missing; did you mean `{split}`?"),
        format!("`{word}`는 띄어쓰기가 빠진 두 낱말로 보여요. 혹시 `{split}`인가요?"),
    )
}

/// A whole line that Python accepts and that still cannot do anything.
///
/// Three shapes reach here, and none of them is ever what a beginner meant:
/// a name standing alone (`hello`, `sayhello`), a `name: value` note with no
/// `=` whose target is an action word (`say: hello`), and a comparison whose
/// answer is thrown away (`score is 0`). NME refuses all three and says what
/// to write instead. Everything else Python accepts is left alone.
///
/// A name that an earlier line assigned is refused too: reading it in a
/// program file still does nothing, and saying so where the writer can see it
/// beats a `NameError` later or silence forever.
fn statement_does_nothing(tokens: &[Token]) -> Option<Diagnostic> {
    if let Some(word) = (tokens.len() == 1).then(|| name_word(&tokens[0])).flatten() {
        // A one-word line that is one of NME's own words stays an ordinary
        // Python name, exactly as it does today: `end`, `skip`, `멈춰` close
        // blocks and leave loops, and `say`, `times`, `목록` are names a
        // Python program is free to use. Only a word NME has no meaning for
        // is refused, because then the line can only be a mistake.
        if is_nme_vocabulary_word(&tokens[0]) {
            return None;
        }
        let problem = Diagnostic::bilingual(
            DiagnosticCode::StatementDoesNothing,
            format!("this line is only the name `{word}`, so it does nothing"),
            format!("이 줄은 `{word}`라는 이름뿐이라 아무 일도 하지 않아요"),
            tokens[0].span,
        );
        if let Some(split) = unglue(word) {
            return Some(problem.with_bilingual_hint(
                format!(
                    "`{word}` looks like two words with the space missing; did you mean `{split}`?"
                ),
                format!("`{word}`는 띄어쓰기가 빠진 두 낱말로 보여요. 혹시 `{split}`인가요?"),
            ));
        }
        if let Some(closing) = END_WORDS_EN
            .iter()
            .chain(END_WORDS_KO)
            .find(|known| one_typo_away(word, known))
        {
            return Some(problem.with_bilingual_hint(
                format!("did you mean `{closing}`, to close a block?"),
                format!("블록을 닫는 `{closing}`을 쓰려던 건가요?"),
            ));
        }
        // A word NME cannot say anything useful about is left to the sentence
        // path, which prints it (see `valid_python_is_a_sentence`). One name
        // per line is also how a plain list of names is written, and guessing
        // that `Mina` wanted an *action* would be the silent rewrite this
        // whole check exists to stop.
        return None;
    }
    if story_annotation(tokens) {
        let text = token_text_without_colon(tokens);
        let opener = name_word(&tokens[0]).unwrap_or("story");
        return Some(
            Diagnostic::bilingual(
                DiagnosticCode::StatementDoesNothing,
                "this looks like the start of a story, but Python reads it as a note about a \
                 name, so the line does nothing",
                "이 줄은 이야기의 시작처럼 보이지만 Python은 이름에 다는 메모로 읽어서 \
                 아무 일도 하지 않아요",
                span_of(tokens),
            )
            .with_bilingual_hint(
                format!(
                    "write `{opener}:` on a line of its own and put the story under it, or \
                     remove the `:` and write `{text}`"
                ),
                format!(
                    "`{opener}:`만 한 줄에 적고 그 아래에 이야기를 쓰거나, `:`를 지우고 \
                     `{text}`처럼 적어 주세요"
                ),
            ),
        );
    }
    if bare_annotation_action(tokens).is_some() {
        let text = token_text_without_colon(tokens);
        return Some(
            Diagnostic::bilingual(
                DiagnosticCode::StatementDoesNothing,
                "a `:` here only writes a note about a name, so this line shows nothing",
                "여기의 `:`는 이름에 메모를 다는 표기라서 이 줄은 아무것도 보여 주지 않아요",
                span_of(tokens),
            )
            .with_bilingual_hint(
                format!("remove the `:` and write `{text}`"),
                format!("`:`를 지우고 `{text}`처럼 적어 주세요"),
            ),
        );
    }
    if bare_comparison(tokens) {
        return Some(
            Diagnostic::bilingual(
                DiagnosticCode::StatementDoesNothing,
                "this line only compares two things and throws the answer away",
                "이 줄은 두 값을 비교만 하고 그 답을 버려요",
                span_of(tokens),
            )
            .with_bilingual_hint(
                "to save a value write `set score to 0`; to decide something write `if score is 0`",
                "값을 저장하려면 `점수는 0`처럼, 무엇을 결정하려면 `만약에 점수가 0이면`처럼 적어 주세요",
            ),
        );
    }
    None
}

fn is_nme_vocabulary_word(token: &Token) -> bool {
    ALL_ACTION_WORDS
        .iter()
        .chain(&[
            BREAK_WORDS_EN,
            BREAK_WORDS_KO,
            BREAK_ALIAS_WORDS_EN,
            CONTINUE_WORDS_EN,
            CONTINUE_WORDS_KO,
            CONTINUE_ALIAS_WORDS_EN,
            WHEN_WORDS_EN,
            WHEN_WORDS_KO,
            WHILE_WORDS_EN,
            WHILE_WORDS_KO,
            ELSE_WORDS_EN,
            ELSE_WORDS_KO,
            TIMES_WORDS_EN,
            TIMES_WORDS_KO,
            LIST_WORDS_EN,
            LIST_WORDS_KO,
            USE_WORDS_EN,
            USE_WORDS_KO,
            NUMBER_WORDS_EN,
            NUMBER_WORDS_KO,
            EACH_WORDS_EN,
            SENTENCE_FILLERS,
            // The spellings this round added. Without them the compound-verb
            // rule cannot see the word in front of a helper verb, and
            // `길을 물어 보았습니다` became a question nobody asked.
            SAY_TRAILING_WORDS_KO,
            SAY_SHORT_WORDS_KO,
            SCREEN_VERB_WORDS_KO,
            ASK_SHORT_WORDS_KO,
            ASK_QUESTION_WORDS_EN,
            ASK_QUESTION_WORDS_KO,
            SET_MAKE_WORDS_EN,
            SET_MAKE_WORDS_KO,
            REPEAT_COUNT_WORDS_EN,
            REPEAT_COUNT_WORDS_KO,
        ])
        .any(|list| token_matches_exact(token, list))
}

/// `say: hello` — Python reads this as a note about a name called `say`, and
/// prints nothing. Only an action word as the target is claimed, so an
/// ordinary `count: int` keeps its Python meaning.
/// `이야기: 그만하자` — a story word, a colon, and one word after it.
///
/// Python reads that as a note about a name: it compiles, does nothing, and
/// says nothing. It is one line break away from a story block, so the two
/// readings are named instead of one being guessed.
fn story_annotation(tokens: &[Token]) -> bool {
    tokens.len() == 3
        && matches!(tokens[1].tok, Tok::Colon)
        && name_word(&tokens[2]).is_some()
        && (token_matches_exact(&tokens[0], STORY_WORDS_EN)
            || token_matches_exact(&tokens[0], STORY_WORDS_KO))
}

fn bare_annotation_action(tokens: &[Token]) -> Option<()> {
    if tokens.len() != 3 || !matches!(tokens[1].tok, Tok::Colon) {
        return None;
    }
    name_word(&tokens[0])?;
    let follows_a_word = name_word(&tokens[2]).is_some();
    (follows_a_word
        && (output_action_at(tokens, 0, MatchMode::Exact).is_some()
            || ask_action_at(tokens, 0, MatchMode::Exact).is_some()))
    .then_some(())
}

fn token_text_without_colon(tokens: &[Token]) -> String {
    tokens
        .iter()
        .filter(|token| !matches!(token.tok, Tok::Colon))
        .filter_map(name_word)
        .collect::<Vec<_>>()
        .join(" ")
}

/// `score is 0` — a comparison as a whole statement. Brackets, an `=`, a
/// call, or a Python keyword mean the line is doing something else, so those
/// are left alone.
fn bare_comparison(tokens: &[Token]) -> bool {
    if tokens.len() < 3 || !matches!(tokens[0].tok, Tok::Name { .. }) {
        return false;
    }
    // `log in first` is three words that Python happens to read as a
    // membership test. It is a sentence, and the fallback prints it.
    if looks_like_plain_prose(tokens) {
        return false;
    }
    if tokens.iter().any(|token| {
        matches!(
            token.tok,
            Tok::Equal
                | Tok::Lpar
                | Tok::Rpar
                | Tok::Lsqb
                | Tok::Rsqb
                | Tok::Lbrace
                | Tok::Rbrace
                | Tok::Semi
                | Tok::Colon
                | Tok::Dot
        ) || (is_python_keyword(&token.tok) && !matches!(token.tok, Tok::Is | Tok::Not | Tok::In))
    }) {
        return false;
    }
    tokens.iter().any(|token| {
        matches!(
            token.tok,
            Tok::Is
                | Tok::In
                | Tok::EqEqual
                | Tok::NotEqual
                | Tok::Less
                | Tok::Greater
                | Tok::LessEqual
                | Tok::GreaterEqual
        )
    })
}

/// A line Python accepts that is really a written sentence, so the sentence
/// fallback should print it instead of leaving a program that does nothing.
///
/// Two shapes qualify, and both are unmistakable. A comparison spelled out in
/// words (`log in first`) never appears in code that means anything, because
/// its answer is thrown away. And one Korean word that ends a sentence
/// (`끝입니다`) is a sentence, unless the program itself gave that name a
/// value earlier.
fn valid_python_is_a_sentence(tokens: &[Token], known_names: &HashSet<String>) -> bool {
    // `30% chance` and `30% 확률로` are a modulo expression to Python: one
    // number divided by one name, with the answer thrown away. Nothing else
    // is on the line, so the writer meant the chance block. A program that
    // really does keep a name spelled that way still wins, because then the
    // line can be doing arithmetic with it.
    if let Some(prefix) = chance_prefix(tokens) {
        let claims_whole_line = prefix.consumed == tokens.len();
        let uses_a_saved_name = tokens[..prefix.consumed]
            .iter()
            .filter_map(name_word)
            .any(|word| known_names.contains(word));
        if claims_whole_line && !uses_a_saved_name {
            return true;
        }
    }
    // `100% 확신합니다` is `100 % 확신합니다` to Python: one number divided by
    // one name, with the answer thrown away and a `NameError` at the end of
    // it. The writer wrote a sentence.
    if is_written_korean_sentence(tokens, known_names) {
        return true;
    }
    if tokens.len() > 2 && looks_like_plain_prose(tokens) {
        let word_comparison = tokens
            .iter()
            .any(|token| matches!(token.tok, Tok::Is | Tok::In));
        if word_comparison {
            return true;
        }
    }
    // `Hello. Goodbye`, `안녕. 잘가`, `Yes. No`. A full stop between two words
    // is attribute access to Python: valid, useless, and a `NameError` at run
    // time naming a word the writer thought was a sentence. Five of the seven
    // most ordinary lines of that shape were passing straight through on
    // 2026-08-19. Nothing in such a line is a name the program made and
    // nothing is called, so there is no reading in which it is code — and
    // handing it back to the NME path also lets `안녕하세요. 말해줘` be the
    // output statement it plainly is.
    if is_dotted_words_only(tokens, known_names) {
        return true;
    }
    // One word on a line of its own, and the program never gave that name a
    // value. Python reads the name, throws the answer away, and dies with a
    // `NameError` pointing at a line that is not the mistake. Nothing else
    // the word could mean is code, so it is what somebody wrote: `Hello`,
    // `Prologue`, `끝입니다`, and each line of a list of names.
    //
    // A name the program *did* set stays Python. There the line really is
    // Python doing nothing, and it is not NME's to change.
    if tokens.len() == 1 {
        if let Some(word) = name_word(&tokens[0]) {
            return !known_names.contains(word) && !is_nme_vocabulary_word(&tokens[0]);
        }
    }
    false
}

/// True when a line of ordinary words should print rather than be reported as
/// a half-recognized command. Every recovery candidate for such a line rests
/// on a one-letter guess about a word that is simply a word — `let me think`,
/// `내일 다시 해 보세요`, `입력해 주세요`. A line that ends in a question mark
/// is excluded: there the writer is asking something, and a misspelled `ask`
/// is worth reporting rather than printing.
/// True for a line that is nothing but plain words joined by full stops, none
/// of which the program ever gave a value: `Hello. Goodbye`, `안녕. 잘가`.
///
/// The shape has to be exact — word, stop, word, stop, word — so anything with
/// a bracket, a number, an operator or a call in it is left to Python, which is
/// where every real dotted line lives. A name the program *did* make also keeps
/// the line: `친구들.sort` is somebody's Python, however little it does.
///
/// The spacing decides the rest, and it decides it well: writing puts the stop
/// against the word before it and a space after it (`Hello. Goodbye`), while
/// attribute access has no space at all (`아니면.foo`, which is a perfectly
/// good Python name followed by a field). Requiring the writing shape is what
/// keeps every NME word usable as an ordinary Python name.
fn is_dotted_words_only(tokens: &[Token], known_names: &HashSet<String>) -> bool {
    if tokens.len() < 3 || tokens.len().is_multiple_of(2) {
        return false;
    }
    tokens.iter().enumerate().all(|(at, token)| {
        if at % 2 == 0 {
            return name_word(token).is_some_and(|word| !known_names.contains(word));
        }
        matches!(token.tok, Tok::Dot)
            && tokens[at - 1].span.end == token.span.start
            && token.span.end < tokens[at + 1].span.start
    })
}

fn prose_beats_recovery(source: &str, tokens: &[Token], known_names: &HashSet<String>) -> bool {
    // A written Korean sentence counts as prose here even with a number or a
    // `%` in it: `나는 100% 동의합니다` is agreement, and the nearest command
    // to it is a one-letter guess at a value change.
    //
    // A heading counts too. `메모: 내일 우산 챙기기` is a note to self, and the
    // colon in it is the only thing that made a recovered value change look
    // like the better reading.
    if !is_written_prose_line(tokens)
        && !is_written_korean_sentence(tokens, known_names)
        && !is_written_label(source, tokens)
    {
        return false;
    }
    if line_asks_a_question(tokens) {
        return false;
    }
    if single_word_ties_two_actions(tokens) {
        return false;
    }
    // A line that names one of the bundled modules, or asks for `latest` or a
    // version, is a `use` line with a typo in it (`usk random latest`), never
    // a sentence someone wanted printed.
    let names_a_module = tokens.iter().any(|token| {
        BundledModuleId::ALL
            .iter()
            .any(|module| module_word_matches(token, *module, MatchMode::Recover))
            || word_matches_any(token, LATEST_WORDS, MatchMode::Recover)
            || word_matches_any(token, &["version", "버전"], MatchMode::Recover)
    });
    if names_a_module {
        return false;
    }
    !tokens
        .iter()
        .filter_map(name_word)
        .any(|word| is_action_word(word) || is_nme_condition_word(word))
}

/// True when one word on the line is the same small distance from two or more
/// different action words (`asy` from both `say` and `ask`). That is a typo
/// worth reporting. A tie made of two *different* words each guessing at a
/// different action is not: it is what ordinary prose looks like from the
/// inside of a spell checker.
fn single_word_ties_two_actions(tokens: &[Token]) -> bool {
    let mut positions = vec![0];
    if tokens.len() > 1 {
        positions.push(tokens.len() - 1);
    }
    positions.into_iter().any(|at| {
        let Some(actual) = tokens.get(at).and_then(token_word) else {
            return false;
        };
        let mut found: Vec<&str> = Vec::new();
        for table in AMBIGUITY_TABLES {
            let Some((best, _)) = best_action_rank(actual, table, MatchMode::Recover) else {
                continue;
            };
            if best == 0 {
                continue;
            }
            found.extend(table.iter().copied().filter(|candidate| {
                action_recovery_rank(actual, candidate, MatchMode::Recover) == Some(best)
            }));
        }
        found.sort_unstable();
        found.dedup();
        found.len() >= 2
    })
}

fn is_nme_condition_word(word: &str) -> bool {
    [
        WHEN_WORDS_EN,
        WHEN_WORDS_KO,
        WHILE_WORDS_EN,
        WHILE_WORDS_KO,
        ELSE_WORDS_EN,
        ELSE_WORDS_KO,
        USE_WORDS_EN,
        USE_WORDS_KO,
    ]
    .iter()
    .any(|list| list.iter().any(|known| word.eq_ignore_ascii_case(known)))
}

fn ambiguous_action_diagnostic(tokens: &[Token]) -> Diagnostic {
    let problem = Diagnostic::bilingual(
        DiagnosticCode::AmbiguousAction,
        "this sentence could mean more than one action",
        "이 문장은 두 가지 동작으로 읽힐 수 있어요",
        span_of(tokens),
    );
    let candidates = tied_action_words(tokens);
    if candidates.is_empty() {
        return problem.with_bilingual_hint(
            "spell the action word exactly so there is one clear meaning",
            "동작 단어를 정확히 적어 뜻을 하나로 정해 주세요",
        );
    }
    let listed = candidates
        .iter()
        .map(|word| format!("`{word}`"))
        .collect::<Vec<_>>()
        .join(", ");
    problem.with_bilingual_hint(
        format!("write one of {listed} exactly, so there is one clear meaning"),
        format!("{listed} 가운데 하나를 정확히 적어 뜻을 하나로 정해 주세요"),
    )
}

fn missing_action_diagnostic(tokens: &[Token]) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::MissingAction,
        "I couldn't find one clear action on this line",
        "이 줄에서 무엇을 할지 찾지 못했어요",
        span_of(tokens),
    )
    .with_bilingual_hint(
        "add an action such as `show`, `ask`, or `repeat`",
        "끝에 `말해줘`를 붙이거나 `물어봐`, `반복해` 같은 동작을 적어 주세요",
    )
}

fn span_of_refs(tokens: &[&Token]) -> Span {
    debug_assert!(!tokens.is_empty());
    Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end)
}

/// The count in a Korean counter word written attached to it (`3번`, `한번`,
/// `삼회`). A count that is neither a number nor a name the program knows is
/// refused here, so it can never become `range(<undefined word>)`.
fn attached_korean_count(
    source: &str,
    name: &str,
    span: Span,
    known_names: &HashSet<String>,
) -> Option<(Code, &'static str)> {
    let counter = TIMES_WORDS_KO.iter().find(|counter| {
        name.strip_suffix(*counter)
            .is_some_and(|rest| !rest.is_empty())
    })?;
    let count = name.strip_suffix(counter)?;
    if let Some(digits) = number_word_digits(count) {
        return Some((Code::Generated(digits.to_string()), counter));
    }
    if !count.starts_with(|character: char| character.is_ascii_digit())
        && !known_names.contains(count)
    {
        return None;
    }
    let count_span = Span::new(span.start, span.end - counter.len());
    is_valid_python_expression(&source[count_span.start..count_span.end])
        .then_some((Code::Source(count_span), counter))
}

fn attached_korean_times_header(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<(Code, usize)> {
    let [Token {
        tok: Tok::Name { name },
        span,
    }, Token {
        tok: Tok::Colon, ..
    }, ..] = tokens
    else {
        return None;
    };
    attached_korean_count(source, name, *span, known_names).map(|(count, _)| (count, 1))
}

fn attached_korean_times_sentence(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<(Code, usize)> {
    let Token {
        tok: Tok::Name { name },
        span,
    } = tokens.first()?
    else {
        return None;
    };
    if tokens
        .get(1)
        .is_some_and(|token| matches!(token.tok, Tok::Colon))
    {
        return None;
    }
    let (count, counter) = attached_korean_count(source, name, *span, known_names)?;
    // Only `번` is unmistakably a repeat counter standing on its own. `회`,
    // `차례`, and `판` are ordinary nouns too, so they need the repeat word
    // right after them before the line becomes a loop.
    if counter != TIMES_KEYWORD_KO && repeat_action_at(tokens, 1, MatchMode::Recover).is_none() {
        return None;
    }
    // `3번 환영합니다` repeats one word; `1번 출구에서 만납시다` is where two
    // people are meeting, and `이번 주에는 비가 세 번 왔습니다` is the weather —
    // `이` being the number two is what turned that one into a loop. With no
    // repeat word on the line, more than one word of body plus the ending of
    // a written Korean sentence is a sentence.
    if tokens.len() > 2
        && repeat_action_at(tokens, 1, MatchMode::Recover).is_none()
        && is_written_korean_sentence(tokens, known_names)
    {
        return None;
    }
    Some((count, 1))
}

fn one_typo_away(actual: &str, expected: &str) -> bool {
    if actual == expected {
        return true;
    }
    let left: Vec<char> = actual.chars().collect();
    let right: Vec<char> = expected.chars().collect();
    if left.len().abs_diff(right.len()) > 1 {
        return false;
    }
    if left.len() == right.len() {
        let differences: Vec<usize> = left
            .iter()
            .zip(&right)
            .enumerate()
            .filter_map(|(index, (a, b))| (a != b).then_some(index))
            .collect();
        return differences.len() == 1
            || (differences.len() == 2
                && differences[1] == differences[0] + 1
                && left[differences[0]] == right[differences[1]]
                && left[differences[1]] == right[differences[0]]);
    }
    let (shorter, longer) = if left.len() < right.len() {
        (&left, &right)
    } else {
        (&right, &left)
    };
    let (mut short_at, mut long_at, mut skipped) = (0, 0, false);
    while short_at < shorter.len() && long_at < longer.len() {
        if shorter[short_at] == longer[long_at] {
            short_at += 1;
            long_at += 1;
        } else if skipped {
            return false;
        } else {
            skipped = true;
            long_at += 1;
        }
    }
    true
}

fn token_word(token: &Token) -> Option<&str> {
    match &token.tok {
        Tok::Name { name } => Some(name),
        Tok::If => Some("if"),
        Tok::While => Some("while"),
        Tok::Break => Some("break"),
        Tok::Elif => Some("elif"),
        Tok::Else => Some("else"),
        Tok::Is => Some("is"),
        Tok::And => Some("and"),
        Tok::Or => Some("or"),
        Tok::As => Some("as"),
        Tok::From => Some("from"),
        Tok::Import => Some("import"),
        _ => None,
    }
}

fn name_word(token: &Token) -> Option<&str> {
    match &token.tok {
        Tok::Name { name } => Some(name),
        _ => None,
    }
}

fn token_is_exact_name(token: &Token, expected: &str) -> bool {
    matches!(&token.tok, Tok::Name { name } if name == expected)
}

fn has_top_level_semicolon(tokens: &[Token]) -> bool {
    let mut depth = 0usize;
    for token in tokens {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Semi if depth == 0 => return true,
            _ => {}
        }
    }
    false
}

fn looks_like_broken_expression(tokens: &[Token]) -> bool {
    tokens.iter().any(|token| {
        matches!(
            token.tok,
            Tok::Plus
                | Tok::Minus
                | Tok::Star
                | Tok::DoubleStar
                | Tok::Slash
                | Tok::DoubleSlash
                | Tok::Percent
                | Tok::Lpar
                | Tok::Rpar
                | Tok::Lsqb
                | Tok::Rsqb
                | Tok::Lbrace
                | Tok::Rbrace
                | Tok::EqEqual
                | Tok::NotEqual
                | Tok::Less
                | Tok::Greater
                | Tok::LessEqual
                | Tok::GreaterEqual
        )
    })
}

fn span_of(tokens: &[Token]) -> Span {
    debug_assert!(!tokens.is_empty());
    Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end)
}

fn token_text<'a>(source: &'a str, tokens: &[Token]) -> &'a str {
    let span = span_of(tokens);
    &source[span.start..span.end]
}

fn is_valid_python_statement(text: &str) -> bool {
    parse_python(text, Mode::Module, "<nme>").is_ok()
}

fn is_valid_python_header(text: &str) -> bool {
    parse_python(&format!("{text}\n    pass"), Mode::Module, "<nme>").is_ok()
}

fn is_python_loop_header(tokens: &[Token]) -> bool {
    matches!(
        tokens.first().map(|token| &token.tok),
        Some(Tok::For | Tok::While)
    ) || (matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::For)))
}

fn is_python_function_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Def))
        || (matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
            && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::Def)))
}

fn is_python_async_function_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::Def))
}

fn is_python_async_for_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::For))
}

fn is_python_async_with_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::With))
}

fn contains_python_nonlocal(tokens: &[Token]) -> bool {
    python_declarations(tokens)
        .iter()
        .any(|declaration| matches!(declaration.kind, PythonDeclarationKind::Nonlocal))
}

fn is_python_import_star_line(tokens: &[Token]) -> bool {
    let depths = token_depths(tokens);
    tokens.iter().enumerate().any(|(start, token)| {
        if depths[start] != 0
            || !matches!(token.tok, Tok::From)
            || (start > 0
                && !(depths[start - 1] == 0 && matches!(tokens[start - 1].tok, Tok::Semi)))
        {
            return false;
        }
        let end = (start + 1..tokens.len())
            .find(|&index| depths[index] == 0 && matches!(tokens[index].tok, Tok::Semi))
            .unwrap_or(tokens.len());
        let statement = &tokens[start..end];
        let Some(import_index) = statement
            .iter()
            .position(|token| matches!(token.tok, Tok::Import))
        else {
            return false;
        };
        statement[import_index + 1..]
            .iter()
            .any(|token| matches!(token.tok, Tok::Star))
    })
}

fn is_python_except_star_control_line(tokens: &[Token]) -> bool {
    has_direct_python_statement(tokens, |tok| {
        matches!(tok, Tok::Break | Tok::Continue | Tok::Return)
    })
}

fn is_python_except_star_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Except))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::Star))
}

fn is_python_try_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Try))
}

fn is_python_try_clause_header(tokens: &[Token]) -> bool {
    matches!(
        tokens.first().map(|token| &token.tok),
        Some(Tok::Except | Tok::Else | Tok::Finally)
    )
}

fn is_python_class_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Class))
}

fn is_valid_python_expression(text: &str) -> bool {
    parse_python(text, Mode::Expression, "<nme>").is_ok()
}

#[cfg(test)]
mod tests {
    use super::one_typo_away;

    #[test]
    fn accepts_one_edit_or_adjacent_transposition() {
        assert!(one_typo_away("말헤", "말해"));
        assert!(one_typo_away("물어바", "물어봐"));
        assert!(one_typo_away("repaet", "repeat"));
        assert!(!one_typo_away("completely", "repeat"));
    }
}

#[cfg(test)]
mod zero_knowledge_tests {

    use crate::diagnostics::DiagnosticCode;
    use crate::transpile;

    #[test]
    fn zero_knowledge_nizk_sentences_bind_an_explicit_context() {
        let source = "영지식 사용 최신
비밀값은 영지식 비밀 만들기
공개값은 비밀값으로 영지식 공개값 만들기
문맥값은 결제 승인 요청
증명값은 비밀값과 문맥값으로 영지식 비대화 증명 만들기
검증값은 공개값과 증명값과 문맥값으로 영지식 비대화 검증
일회값은 영지식 일회값 만들기
약속값은 일회값으로 영지식 약속 만들기
도전값은 공개값과 약속값과 문맥값으로 영지식 비대화 도전 만들기
";
        let python = transpile(source).expect("context-bound NIZK sentences must transpile");
        assert!(
            python.contains("증명값 = zk_nizk_prove(비밀값, 문맥값)"),
            "{python}"
        );
        assert!(
            python.contains("검증값 = zk_nizk_verify(공개값, 증명값, 문맥값)"),
            "{python}"
        );
        assert!(
            python.contains("도전값 = zk_nizk_challenge(공개값, 약속값, 문맥값)"),
            "{python}"
        );
    }

    #[test]
    fn zero_knowledge_sentence_values_lower_without_python_punctuation() {
        let source = "영지식 사용 최신
비밀값은 영지식 비밀 만들기
공개값은 비밀값으로 영지식 공개값 만들기
일회값은 영지식 일회값 만들기
약속값은 일회값으로 영지식 약속 만들기
도전값은 영지식 도전 만들기
응답값은 일회값과 비밀값과 도전값으로 영지식 응답 만들기
검증값은 공개값과 약속값과 도전값과 응답값으로 영지식 검증
";
        let python = transpile(source).expect("zero-knowledge sentences must transpile");
        assert!(python.contains("import secrets as 영지식비밀난수"));
        assert!(python.contains(r#"비밀값 = __import__("secrets").randbelow"#));
        assert!(python.contains("공개값 = pow(2, 비밀값, 0x"));
        assert!(python.contains("검증값 = (1 < (공개값)"));
    }

    #[test]
    fn zero_knowledge_module_protects_helper_names() {
        let source = "영지식검증 = 1
영지식 사용 최신
";
        let problems = transpile(source).expect_err("helper collision must be rejected");
        assert!(problems
            .iter()
            .any(|problem| problem.code == DiagnosticCode::ModuleNameCollision));
    }
}
