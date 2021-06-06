#[warn(unused_imports)]

extern crate dotenv;

use crate::getters::{user_getter, follower_getter, tweet_getter};
mod getters;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = reqwest::Client::new();
    println!("{:#?}", tweet_getter(&client, "SHISEIDO_brand", 10).await);
}