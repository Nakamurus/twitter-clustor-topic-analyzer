extern crate dotenv;
mod tweet_tokenizer;
mod getters;


use dotenv::dotenv;
use std::io::*;
use std::str::FromStr;

use crate::aggregate_func::aggregate;
mod aggregate_func;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("filed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let target_account:String = read();
    let tweet_count:usize = read();
    let account_count:usize = read();

    aggregate(&target_account, account_count, tweet_count).await;
}