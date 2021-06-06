use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;
use std::collections::{BTreeMap, HashSet};
use std::iter::FromIterator;

fn tokenize(sentences: &Vec<&str>) -> Vec<String> {
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

pub fn token_counter(token_map: &mut BTreeMap<String, usize>, sentences: &Vec<&str>) {
    let tokens = tokenize(sentences);
    for token in tokens {
        *token_map.entry(token).or_insert(0) += 1;
    }
}
