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

async fn initiate_client(url:String) -> reqwest::RequestBuilder {
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

pub async fn user_getter(account: &str) -> Result<User, Error> {
    let request_url = format!("https://api.twitter.com/1.1/users/show.json?screen_name={}", account);
    let client = initiate_client(request_url).await;
    let res: User = client.send().await?.json().await?;
    Ok(res)
}

pub async fn follower_getter(account:&str, count:usize) -> Result<Followers, Error> {
    let request_url = format!("https://api.twitter.com/1.1/followers/ids.json?screen_name={}&count={}", account, count);
    let client = initiate_client(request_url).await;
    let res:Followers = client.send().await?.json().await?;
    Ok(res)
}

pub async fn tweet_getter(account:&str, count:usize) -> Result<Value, Error> {
    let account:String = if account.chars().map(|c| c.is_numeric()).any(|x| x == false) {
        let user = user_getter(account).await?;
        user.id_str
    } else {
        account.to_string()
    };
    let url = format!("https://api.twitter.com/2/users/{}/tweets?expansions=author_id&tweet.fields=public_metrics,entities&max_results={}", account, count);
    let client = initiate_client(url).await;
    let res:Value = client.send().await?.json().await?;
    Ok(res)
}
