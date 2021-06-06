#[warn(unused_imports)]

extern crate dotenv;

use crate::getters::{user_getter, follower_getter, tweet_getter};
mod getters;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("{:#?}", tweet_getter("SHISEIDO_brand", 10).await);
}