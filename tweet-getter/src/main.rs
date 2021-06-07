extern crate dotenv;
mod tweet_tokenizer;
mod getters;
mod tweet;
mod user;
mod converters;
mod io_helpers;
mod aggregate_func;

use dotenv::dotenv;

use crate::aggregate_func::aggregate;
use crate::io_helpers::read;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Input taerget account name and numbers of tweets and accounts to fetch");
    let target_account:String = read();
    let tweet_count:usize = read();
    let account_count:usize = read();
    aggregate(&target_account, account_count, tweet_count).await;
}