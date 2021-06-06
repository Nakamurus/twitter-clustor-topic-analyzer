extern crate dotenv;

use crate::getters::{user_getter, follower_getter, tweet_getter, extract_tweet_info};
mod getters;

use crate::tweet_tokenizer::token_counter;
mod tweet_tokenizer;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = reqwest::Client::new();
    // println!("{:#?}", token_counter(tweet_getter(&client, "SHISEIDO_brand", 10).await.unwrap().data.iter().map(|x| &x.text as &str).collect()));
    println!("{:#?}", extract_tweet_info(tweet_getter(&client, "SHISEIDO_brand", 10).await.unwrap().data));
}