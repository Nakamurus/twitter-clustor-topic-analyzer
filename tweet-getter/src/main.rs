#[warn(unused_imports)]

extern crate dotenv;

use crate::user::{User, Entities, Url};
use crate::tweet::Tweet;
mod user;
mod tweet;

use dotenv::dotenv;
use std::env;
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderName, AUTHORIZATION};
use serde::{Deserialize, Serialize};



async fn user_getter() -> Result<User, Error> {
    dotenv().ok();
    let bearer_token = env::var("BEARER_TOKEN").expect("bearer token is not found");
    println!("Hello, world!");
    let request_url = "https://api.twitter.com/1.1/users/show.json?screen_name=ToshiyukiHorie";
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", bearer_token).parse().unwrap()
    );
    let client = reqwest::Client::new()
        .get(request_url)
        .headers(headers);
    let res: User = client.send().await?.json().await?;
    Ok(res)
}

#[tokio::main]
async fn main() {
    println!("{:?}", user_getter().await);
}