use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;
use std::collections::{BTreeMap, HashSet};
use std::iter::FromIterator;

fn tokenize(sentences: Vec<&str>) -> Vec<String> {
    let mut tokenizer = Tokenizer::new(Mode::Normal, "");
    let ignore_tokens: HashSet<&str> = HashSet::from_iter(vec!["助動詞", "助詞", "記号", "UNK"]);
    let tokens = sentences
        .iter()
        .map(|sentence| tokenizer.tokenize(sentence))
        .flatten()
        .filter(|token| !ignore_tokens.contains(&token.detail[0] as &str))
        .map(|token| {
            let mut detail = token.detail;
            detail.remove(6)
        });
    tokens.collect::<Vec<String>>()
}

pub fn token_counter(sentences: Vec<&str>) -> Vec<(String, usize)> {
    let tokens = tokenize(sentences);
    let mut token_map = BTreeMap::new();
    for token in tokens {
        *token_map.entry(token).or_insert(0) += 1;
    }
    let mut token_count: Vec<(String, usize)> = Vec::from_iter(token_map);
    token_count.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    token_count
}
