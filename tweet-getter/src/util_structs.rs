use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use std::collections::HashSet;
use std::env;
use std::iter::FromIterator;

pub struct ReqwestClientHeader {
    client: reqwest::Client,
    header: HeaderMap<HeaderValue>,
}

impl ReqwestClientHeader {
    pub fn new() -> ReqwestClientHeader {
        let bearer = env::var("BEARER_TOKEN").expect("bearer token is not found");
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", bearer).parse().unwrap());
        ReqwestClientHeader {
            client: Client::new(),
            header: headers,
        }
    }

    pub fn get_client(&mut self) -> &mut reqwest::Client {
        &mut self.client
    }

    pub fn get_header(&self) -> HeaderMap<HeaderValue> {
        self.header.clone()
    }
}

pub struct MyTokenizer {
    pub tokenizer: Tokenizer,
    pub ignore_tokens: HashSet<String>,
}

impl MyTokenizer {
    pub fn new() -> MyTokenizer {
        MyTokenizer {
            tokenizer: Tokenizer::new(Mode::Normal, ""),
            ignore_tokens: HashSet::from_iter(vec![
                "助動詞".to_string(),
                "助詞".to_string(),
                "記号".to_string(),
                "UNK".to_string(),
            ]),
        }
    }
}
