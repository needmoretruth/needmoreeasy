#!/usr/bin/env python3
from pathlib import Path

path = Path("crates/nme-core/src/parser.rs")
text = path.read_text(encoding="utf-8")

module_old = '''fn module_word_matches(token: &Token, module: BundledModuleId, mode: MatchMode) -> bool {
    name_word(token).is_some_and(|word| {
        word_matches(word, module.name_en(), mode)
            || word == module.name_ko()
            || strip_target_particle(word) == module.name_ko()
    })
}
'''
module_new = '''fn module_word_matches(token: &Token, module: BundledModuleId, mode: MatchMode) -> bool {
    name_word(token).is_some_and(|word| {
        word_matches(word, module.name_en(), mode)
            || (module == BundledModuleId::ZeroKnowledge
                && word_matches(word, "zeroknowledge", mode))
            || word == module.name_ko()
            || strip_target_particle(word) == module.name_ko()
    })
}
'''
if module_old not in text:
    raise SystemExit("module matcher anchor changed")
text = text.replace(module_old, module_new, 1)

value_anchor = '''fn parse_zero_knowledge_value(tokens: &[Token]) -> Option<Value> {
    use crate::syntax::ZeroKnowledgeValue as Zk;
'''
value_insert = '''fn parse_zero_knowledge_value(tokens: &[Token]) -> Option<Value> {
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
'''
if value_anchor not in text:
    raise SystemExit("zero-knowledge value anchor changed")
text = text.replace(value_anchor, value_insert, 1)

helper_anchor = '''fn zero_knowledge_code_with_particle(token: &Token, particles: &[&str]) -> Option<Code> {
'''
helper_insert = '''fn zero_knowledge_code_plain(token: &Token) -> Option<Code> {
    name_word(token)?;
    Some(Code::Source(token.span))
}

fn zero_knowledge_code_with_particle(token: &Token, particles: &[&str]) -> Option<Code> {
'''
if helper_anchor not in text:
    raise SystemExit("zero-knowledge helper anchor changed")
text = text.replace(helper_anchor, helper_insert, 1)

path.write_text(text, encoding="utf-8")
print("materialized English sentence zero-knowledge syntax")
