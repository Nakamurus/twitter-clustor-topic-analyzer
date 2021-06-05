#[warn(unused_imports)]

extern crate dotenv;

use crate::user::{User, Entities, Url};
use crate::tweet::Tweet;
use crate::followers::Followers;
mod user;
mod tweet;
mod followers;

use dotenv::dotenv;
use std::env;
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderName, AUTHORIZATION};
use serde::{Deserialize, Serialize};

async fn initiate_client(url:&str) -> reqwest::RequestBuilder {
    let bearer = env::var("BEARER_TOKEN").expect("bearer token is not found");
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", bearer).parse().unwrap()
    );
    reqwest::Client::new()
        .get(url)
        .headers(headers)
}

async fn user_getter() -> Result<User, Error> {
    let request_url = "https://api.twitter.com/1.1/users/show.json?screen_name=ToshiyukiHorie";
    let client = initiate_client(request_url).await;
    let res: User = client.send().await?.json().await?;
    Ok(res)
}

async fn follower_getter() -> Result<Followers, Error> {
    let request_url = "https://api.twitter.com/1.1/followers/ids.json?screen_name=ToshiyukiHorie&count=10";
    let client = initiate_client(request_url).await;
    let res:Followers = client.send().await?.json().await?;
    Ok(res)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("{:?}", follower_getter().await);
}