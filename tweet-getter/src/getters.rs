use user::User;
use tweet::{Tweet, TweetResponse, ExtractedTweetInfo, ReferencedTweet};
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

async fn call_api(client: &reqwest::Client, url:String) -> reqwest::RequestBuilder {
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

pub async fn user_getter(client: &reqwest::Client, account: String) -> Result<User, Error> {
    let request_url = if account.chars().map(|c| c.is_numeric()).any(|x| x == false) {
        format!("https://api.twitter.com/1.1/users/show.json?screen_name={}", account)
    } else {
        format!("https://api.twitter.com/1.1/users/show.json?user_id={}", account)
    };
    let client = call_api(client, request_url).await;
    let res: User = client.send().await?.json().await?;
    Ok(res)
}

pub async fn follower_getter(client: &reqwest::Client, account:String, count:usize) -> Result<Followers, Error> {
    let request_url = if account.chars().map(|c| c.is_numeric()).any(|x| x == false) {
        format!("https://api.twitter.com/1.1/followers/ids.json?screen_name={}&count={}", account, count)
    } else {
        format!("https://api.twitter.com/1.1/followers/ids.json?user_id={}&count={}", account, count)
    };
    let client = call_api(client, request_url).await;
    let res:Followers = client.send().await?.json().await?;
    Ok(res)
}

pub async fn tweet_getter(client: &reqwest::Client, account:String, count:usize) -> Result<TweetResponse, Error> {
    let url = format!("https://api.twitter.com/2/users/{}/tweets?expansions=attachments.poll_ids,attachments.media_keys,author_id,entities.mentions.username,in_reply_to_user_id,referenced_tweets.id,referenced_tweets.id.author_id&tweet.fields=attachments,author_id,context_annotations,conversation_id,created_at,entities,geo,id,in_reply_to_user_id,lang,possibly_sensitive,public_metrics,referenced_tweets,reply_settings,source,text,withheld&user.fields=created_at,description,entities,id,name,pinned_tweet_id,profile_image_url,protected,public_metrics,url,username,verified,withheld&media.fields=duration_ms,height,media_key,preview_image_url,type,url,width,public_metrics&max_results={}", account, count);

    let client = call_api(client, url).await;
    let res:TweetResponse = client.send().await?.json().await?;
    Ok(res)
}

pub fn extract_tweet_info(tweets: &Vec<Tweet>) -> Vec<ExtractedTweetInfo> {
    let mut v = Vec::new();
    for tweet in tweets {
        let tweet = tweet.clone();
        let mut urls = None;
        let mut hashtags = None;
        if let Some(e) = tweet.entities {
            urls = e.urls;
            if let Some(d) = e.description {
                hashtags = d.hashtags;
            }
        }
        let text = tweet.text;
        let metrics = tweet.public_metrics;
        let action_count = metrics.like_count + metrics.retweet_count + metrics.reply_count + metrics.quote_count;
        v.push(ExtractedTweetInfo::new(urls, hashtags, text, action_count));
    }
    v.sort_by(|a, b| b.action_count.cmp(&a.action_count));
    v.dedup_by(|a,b| a.text.eq_ignore_ascii_case(&b.text));
    v
}