use lindera::tokenizer::Tokenizer;
use std::collections::{BTreeMap, HashSet};

pub fn token_counter(
    tokenizer: &mut Tokenizer,
    ignore_tokens: &HashSet<String>,
    token_map: &mut BTreeMap<String, usize>,
    sentence: &str,
) {
    for token in tokenizer
        .tokenize(sentence)
        .iter()
        .filter(|token| !ignore_tokens.contains(&token.detail[0]))
    {
        *token_map.entry(token.detail.clone().remove(6)).or_insert(0) += 1;
    }
}
