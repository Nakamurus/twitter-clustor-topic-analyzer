use user::User;
use tweet::{Tweet, TweetResponse};
use followers::Followers;

#[path="../src/user.rs"]
mod user;
#[path="../src/tweet.rs"]
mod tweet;
#[path="../src/followers.rs"]
mod followers;

extern crate dotenv;
use std::env;
use reqwest::Error;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde_json::Value;

async fn initiate_client(client: &reqwest::Client, url:String) -> reqwest::RequestBuilder {
    let bearer = env::var("BEARER_TOKEN").expect("bearer token is not found");
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", bearer).parse().unwrap()
    );
    client
        .get(url)
        .headers(headers)
}

pub async fn user_getter(client: &reqwest::Client, account: &str) -> Result<User, Error> {
    let request_url = format!("https://api.twitter.com/1.1/users/show.json?screen_name={}", account);
    let client = initiate_client(client, request_url).await;
    let res: User = client.send().await?.json().await?;
    Ok(res)
}

pub async fn follower_getter(client: &reqwest::Client, account:&str, count:usize) -> Result<Followers, Error> {
    let request_url = format!("https://api.twitter.com/1.1/followers/ids.json?screen_name={}&count={}", account, count);
    let client = initiate_client(client, request_url).await;
    let res:Followers = client.send().await?.json().await?;
    Ok(res)
}

pub async fn tweet_getter(client: &reqwest::Client, account:&str, count:usize) -> Result<Value, Error> {
    let account = user_getter(client, account).await?.id_str;
    let url = format!("https://api.twitter.com/2/users/{}/tweets
    ?expansions=author_id&tweet.fields=public_metrics,entities
    &max_results={}", account, count);
    let client = initiate_client(client, url).await;
    let res:Value = client.send().await?.json().await?;
    Ok(res)
}
