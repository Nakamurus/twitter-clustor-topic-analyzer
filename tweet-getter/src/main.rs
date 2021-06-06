extern crate dotenv;

use crate::getters::{user_getter, follower_getter, tweet_getter};
mod getters;

use crate::tweet_tokenizer::token_counter;
mod tweet_tokenizer;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = reqwest::Client::new();
    // println!("{:#?}", token_counter(tweet_getter(&client, "SHISEIDO_brand", 10).await.unwrap()));
    println!("{:#?}", tweet_getter(&client, "SHISEIDO_brand", 10).await);
}