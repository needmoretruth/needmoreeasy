#!/usr/bin/env python3
"""Rebuilds `COMMON_ENGLISH_WORDS` in `crates/nme-core/src/parser.rs`.

NME repairs one typo. One typo is also all that separates most short English
words from each other, so without a list of the words English already has,
`shop milk` printed `milk` and `well done` printed `done`: `shop` and `well`
were read as misspelled `show` and `tell`.

The list this writes is exactly the set of real English words that sit one
typo away from one of NME's own English words — the words repair would
otherwise claim, and no others. Everything else keeps its repair, so `shwo`,
`tel`, `sya` and `pirnt` still reach `show`, `tell`, `say` and `print`.

Run it after adding an English word to the compiler:

    python3 scripts/build-common-english-words.py

With `--check` it writes nothing and reports whether the list is still the one
the current word lists call for. Both forms need the system spelling
dictionary (`hunspell-en-us` on Debian and Ubuntu); `--check` skips when it is
missing, because a machine without a dictionary cannot answer the question,
and a plain run stops rather than write a list built from nothing.
"""

from __future__ import annotations

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
PARSER = ROOT / "crates" / "nme-core" / "src" / "parser.rs"
DICTIONARIES = [
    pathlib.Path("/usr/share/hunspell/en_US.dic"),
    pathlib.Path("/usr/share/dict/words"),
]
CONST = "COMMON_ENGLISH_WORDS"

# Common words that stand whatever the dictionary says. Short entries are
# where a spelling dictionary keeps what nobody writes (`tel`, `sae`), so the
# generated part below takes only words of four letters or more; these carry
# the short ones that ordinary English really does use.
ALWAYS = (
    "a", "about", "above", "across", "after", "again", "against", "all", "almost", "alone",
    "along", "already", "also", "although", "always", "am", "among", "an", "and",
    "another", "answer", "any", "anybody", "anyone", "anything", "anyway", "are", "area",
    "around", "as", "ask", "at", "away", "back", "bad", "bag", "ball", "bank", "be",
    "beautiful", "became", "because", "become", "bed", "been", "before", "began", "begin",
    "behind", "being", "believe", "bell", "below", "beside", "best", "better", "between",
    "big", "bird", "bit", "black", "block", "blue", "board", "boat", "body", "book",
    "born", "both", "bottom", "bowl", "box", "boy", "bread", "break", "bright", "bring",
    "brother", "brown", "build", "built", "business", "busy", "but", "buy", "by", "call",
    "came", "can", "cannot", "car", "care", "carry", "case", "cat", "catch", "cause",
    "cell", "centre", "chair", "chance", "change", "check", "child", "children", "choose",
    "church", "city", "class", "clean", "clear", "close", "cold", "colour", "come",
    "common", "company", "complete", "cool", "copy", "corner", "cost", "could", "count",
    "country", "couple", "course", "cover", "cross", "crowd", "cry", "cup", "cut", "dad",
    "dark", "date", "daughter", "day", "dead", "deal", "dear", "death", "decide", "deep",
    "did", "die", "difference", "different", "difficult", "dinner", "direction", "do",
    "does", "dog", "done", "door", "double", "doubt", "down", "draw", "dream", "dress",
    "drink", "drive", "drop", "dry", "during", "each", "ear", "early", "earth", "east",
    "easy", "eat", "edge", "egg", "eight", "either", "else", "empty", "end", "enjoy",
    "enough", "enter", "entire", "even", "evening", "ever", "every", "everybody",
    "everyone", "everything", "exactly", "example", "except", "expect", "eye", "face",
    "fact", "fail", "fall", "family", "famous", "far", "farm", "fast", "father", "fear",
    "feel", "feet", "fell", "felt", "few", "field", "fight", "fill", "film", "final",
    "find", "fine", "finger", "finish", "fire", "first", "fish", "five", "fix", "floor",
    "flower", "fly", "follow", "food", "foot", "for", "force", "forest", "forget", "form",
    "four", "free", "fresh", "friend", "from", "front", "full", "fun", "funny", "game",
    "garden", "gave", "get", "girl", "give", "glad", "glass", "go", "goes", "gold", "gone",
    "good", "got", "great", "green", "grew", "ground", "group", "grow", "guess", "guy",
    "had", "hair", "half", "hand", "happen", "happy", "hard", "has", "hat", "hate", "have",
    "he", "head", "hear", "heard", "heart", "heat", "heavy", "held", "hello", "help",
    "her", "here", "herself", "hey", "high", "hill", "him", "himself", "his", "history",
    "hit", "hold", "hole", "holiday", "home", "hope", "horse", "hospital", "hot", "hotel",
    "hour", "house", "how", "however", "huge", "human", "hundred", "hungry", "hurry",
    "hurt", "husband", "i", "ice", "idea", "if", "ill", "important", "in", "inside",
    "instead", "interest", "into", "is", "it", "its", "itself", "job", "join", "joy",
    "jump", "just", "keep", "kept", "key", "kid", "kill", "kind", "king", "kitchen",
    "knew", "knife", "know", "known", "lady", "lake", "land", "language", "large", "last",
    "late", "later", "laugh", "law", "lay", "lead", "learn", "least", "leave", "led",
    "left", "leg", "less", "lesson", "let", "letter", "level", "lie", "life", "light",
    "like", "line", "lion", "lip", "list", "listen", "little", "live", "long", "look",
    "lose", "lost", "lot", "loud", "love", "low", "luck", "lunch", "machine", "mad",
    "made", "main", "make", "man", "many", "map", "mark", "market", "marry", "match",
    "matter", "may", "maybe", "me", "mean", "meat", "meet", "member", "men", "message",
    "met", "middle", "might", "mile", "milk", "mind", "mine", "minute", "miss", "mistake",
    "moment", "money", "month", "moon", "more", "morning", "most", "mother", "mountain",
    "mouth", "move", "movie", "much", "music", "must", "my", "myself", "name", "near",
    "nearly", "neck", "need", "needed", "neighbour", "neither", "never", "new", "news",
    "next", "nice", "night", "nine", "no", "nobody", "noise", "none", "nor", "north",
    "nose", "not", "note", "nothing", "notice", "now", "number", "of", "off", "offer",
    "office", "often", "oh", "oil", "okay", "old", "on", "once", "one", "only", "open",
    "or", "order", "other", "our", "out", "outside", "over", "own", "page", "pain",
    "paint", "pair", "paper", "parent", "park", "part", "party", "pass", "past", "pay",
    "peace", "pen", "people", "perhaps", "person", "phone", "photo", "pick", "picture",
    "piece", "place", "plan", "plant", "play", "please", "pocket", "point", "police",
    "poor", "possible", "power", "practice", "prepare", "present", "press", "pretty",
    "price", "problem", "promise", "pull", "push", "put", "question", "quick", "quiet",
    "quite", "radio", "rain", "raise", "ran", "rang", "rather", "reach", "ready", "real",
    "really", "reason", "receive", "red", "remember", "rest", "result", "return", "rich",
    "ride", "right", "ring", "rise", "river", "road", "rock", "rode", "roll", "room",
    "rose", "round", "row", "rule", "run", "sad", "safe", "said", "sail", "salt", "same",
    "sang", "sat", "save", "saw", "say", "school", "sea", "season", "seat", "second",
    "secret", "see", "seem", "seen", "sell", "send", "sense", "sent", "serve", "service",
    "set", "seven", "several", "shall", "shape", "share", "sharp", "she", "sheep", "shell",
    "ship", "shirt", "shoe", "shoot", "shop", "short", "shot", "should", "shoulder",
    "shout", "shut", "sick", "side", "sign", "silver", "simple", "since", "sing", "single",
    "sister", "sit", "six", "size", "skin", "sky", "sleep", "slow", "small", "smell",
    "smile", "smoke", "snow", "so", "soft", "soil", "sold", "soldier", "some", "somebody",
    "someone", "something", "sometimes", "son", "song", "soon", "sorry", "sort", "sound",
    "soup", "south", "space", "speak", "special", "speed", "spend", "spent", "spoke",
    "sport", "spot", "spread", "spring", "square", "stand", "star", "start", "state",
    "station", "stay", "step", "stick", "still", "stone", "stood", "stop", "store",
    "storm", "story", "straight", "strange", "street", "strong", "student", "study",
    "stuff", "subject", "such", "sudden", "sugar", "summer", "sun", "supper", "suppose",
    "sure", "surprise", "sweet", "swim", "table", "tail", "take", "talk", "tall", "taste",
    "teach", "teacher", "team", "tear", "tell", "ten", "term", "test", "than", "thank",
    "that", "the", "their", "them", "themselves", "then", "there", "these", "they",
    "thick", "thin", "thing", "think", "third", "this", "those", "though", "thought",
    "three", "through", "throw", "thus", "tie", "tight", "till", "time", "tiny", "tired",
    "to", "today", "together", "told", "tomorrow", "tone", "tonight", "too", "took",
    "tool", "tooth", "top", "total", "touch", "toward", "town", "train", "travel", "tree",
    "trip", "trouble", "true", "trust", "try", "turn", "twelve", "twenty", "twice", "two",
    "type", "uncle", "under", "understand", "until", "up", "upon", "us", "use", "usual",
    "very", "village", "visit", "voice", "wait", "walk", "wall", "want", "war", "warm",
    "was", "wash", "watch", "water", "wave", "way", "we", "wear", "weather", "week",
    "weight", "welcome", "well", "went", "were", "west", "wet", "what", "wheel", "when",
    "where", "whether", "which", "while", "white", "who", "whole", "whom", "whose", "why",
    "wide", "wife", "wild", "will", "win", "wind", "window", "wine", "wing", "winter",
    "wire", "wise", "wish", "with", "within", "without", "woman", "women", "wonder",
    "wood", "word", "wore", "work", "world", "worry", "worse", "worth", "would", "write",
    "wrong", "wrote", "yard", "year", "yellow", "yes", "yesterday", "yet", "you", "young",
    "your", "yourself",
)

# Below this length a dictionary entry is more likely to be an abbreviation
# than a word a beginner typed on purpose, so repair keeps it.
SHORTEST_GENERATED = 4

def inflections(word: str) -> tuple[str, ...]:
    """The everyday endings the dictionary leaves to its affix flags.

    `falls` is as much a word as `fall`, and repair must not claim it either.
    The endings are spelled out rather than glued on, because gluing produced
    words English does not have: `append` + `d` is `appendd`, which is exactly
    the doubled-letter typo repair exists to fix.
    """
    plural = "es" if word.endswith(("s", "x", "z", "ch", "sh")) else "s"
    if word.endswith("e"):
        return (word + plural, word + "d", word[:-1] + "ing", word + "ly")
    if word.endswith("y") and len(word) > 2 and word[-2] not in "aeiou":
        return (word[:-1] + "ies", word[:-1] + "ied", word + "ing", word[:-1] + "ily")
    return (word + plural, word + "ed", word + "ing", word + "ly")


def english_words(checking: bool = False) -> set[str]:
    for path in DICTIONARIES:
        if not path.exists():
            continue
        words = set()
        for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
            word = line.split("/", 1)[0].split("\t", 1)[0].strip()
            # Lower-case only: the capitalised entries are names, and a name
            # is exactly what a beginner may be trying to print.
            if word.isalpha() and word.islower() and word.isascii():
                words.add(word)
        if words:
            return words
    if checking:
        print("common-english-words: skipped, no English dictionary on this machine")
        raise SystemExit(0)
    sys.exit(
        "no English dictionary found — install hunspell-en-us "
        f"(looked in {', '.join(str(path) for path in DICTIONARIES)})"
    )


def nme_english_words() -> set[str]:
    """Every English word the compiler's own word lists accept."""
    source = PARSER.read_text(encoding="utf-8")
    words: set[str] = set()
    for match in re.finditer(
        r"const ([A-Z0-9_]*_EN|[A-Z0-9_]*WORDS|SENTENCE_FILLERS|EXACT_ONLY_ACTION_WORDS)"
        r"(?::\s*&\[&str\])?\s*=\s*&\[(.*?)\];",
        source,
        re.S,
    ):
        name, body = match.group(1), match.group(2)
        if name == CONST:
            continue
        for word in re.findall(r'"([^"]*)"', body):
            if word.isalpha() and word.isascii():
                words.add(word.lower())
    return words


def one_edit_away(word: str, alphabet: str) -> set[str]:
    edits = set()
    for index in range(len(word) + 1):
        head, tail = word[:index], word[index:]
        if tail:
            edits.add(head + tail[1:])
            for letter in alphabet:
                edits.add(head + letter + tail[1:])
        if len(tail) > 1:
            edits.add(head + tail[1] + tail[0] + tail[2:])
        for letter in alphabet:
            edits.add(head + letter + tail)
    edits.discard(word)
    return edits


def main() -> int:
    checking = "--check" in sys.argv[1:]
    dictionary = english_words(checking)
    alphabet = "abcdefghijklmnopqrstuvwxyz"
    known = set(dictionary)
    for word in dictionary:
        known.update(inflections(word))
    ours = nme_english_words()
    claimed = set()
    for word in ours:
        if len(word) < 2:
            continue
        claimed |= {
            neighbour
            for neighbour in one_edit_away(word, alphabet)
            if len(neighbour) >= SHORTEST_GENERATED and neighbour in known
        }
    claimed |= set(ALWAYS)
    # NME's own words stay in. Every reading that matters is settled by exact
    # match first, and `than` has to be here: it is one letter from `then`,
    # and `score is less than 5` must never be read as a condition whose body
    # begins at a mistyped `then`.
    listed = sorted(claimed)

    lines, current = [], "    "
    for word in listed:
        piece = f'"{word}", '
        if len(current) + len(piece) > 97:
            lines.append(current.rstrip())
            current = "    "
        current += piece
    lines.append(current.rstrip().rstrip(","))

    source = PARSER.read_text(encoding="utf-8")
    start = source.index(f"const {CONST}: &[&str] = &[")
    end = source.index("];", start) + 2
    replacement = f"const {CONST}: &[&str] = &[\n" + "\n".join(lines) + ",\n];"
    if source[start:end] == replacement:
        print(f"common-english-words: ok, {len(listed)} words")
        return 0
    if checking:
        print(
            f"common-english-words: the list is out of date ({len(listed)} words expected);"
            " run python3 scripts/build-common-english-words.py"
        )
        return 1
    PARSER.write_text(source[:start] + replacement + source[end:], encoding="utf-8")
    print(f"common-english-words: wrote {len(listed)} words")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
