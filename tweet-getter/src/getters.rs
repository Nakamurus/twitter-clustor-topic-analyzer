use crate::user::{User, DefaultUserResponse};
use crate::tweet::TweetResponse;
use followers::Followers;

#[path="../src/user.rs"]
mod user;
#[path="../src/tweet.rs"]
mod tweet;
#[path="../src/followers.rs"]
mod followers;

use reqwest::Error;
use crate::util_structs::ReqwestClientHeader;

async fn get_api(client: &mut ReqwestClientHeader, url:String) -> reqwest::RequestBuilder {
    client.get_client()
        .get(url)
        .headers(client.get_header())
}

pub async fn user_getter(client: &mut ReqwestClientHeader, account: String) -> Result<User, Error> {
    let request_url = if account.chars().map(|c| c.is_numeric()).any(|x| x == false) {
        format!("https://api.twitter.com/1.1/users/show.json?screen_name={}", account)
    } else {
        format!("https://api.twitter.com/1.1/users/show.json?user_id={}", account)
    };
    let client = get_api(client, request_url).await;
    let res: User = client.send().await?.json().await?;
    Ok(res)
}

pub async fn username_getter(client: &mut ReqwestClientHeader, account: &str) -> Result<String, Error> {
    let request_url:String = format!("https://api.twitter.com/2/users/{}", account);
    let client = get_api(client, request_url).await;
    let res: DefaultUserResponse = client.send().await?.json().await?;
    Ok(res.data.name)
}


pub async fn follower_getter(client: &mut ReqwestClientHeader, account:&str, count:usize) -> Result<Followers, Error> {
    let request_url = if account.chars().map(|c| c.is_numeric()).any(|x| x == false) {
        format!("https://api.twitter.com/1.1/followers/ids.json?screen_name={}&count={}", account, count)
    } else {
        format!("https://api.twitter.com/1.1/followers/ids.json?user_id={}&count={}", account, count)
    };
    let client = get_api(client, request_url).await;
    let res:Followers = client.send().await?.json().await?;
    Ok(res)
}

pub async fn tweet_getter(client: &mut ReqwestClientHeader, account: i64, count:usize) -> Result<TweetResponse, Error> {
    let url = format!("https://api.twitter.com/2/users/{}/tweets?expansions=attachments.poll_ids,attachments.media_keys,author_id,entities.mentions.username,in_reply_to_user_id,referenced_tweets.id,referenced_tweets.id.author_id&tweet.fields=attachments,author_id,context_annotations,conversation_id,created_at,entities,geo,id,in_reply_to_user_id,lang,possibly_sensitive,public_metrics,referenced_tweets,reply_settings,source,text,withheld&user.fields=created_at,description,entities,id,name,pinned_tweet_id,profile_image_url,protected,public_metrics,url,username,verified,withheld&media.fields=duration_ms,height,media_key,preview_image_url,type,url,width,public_metrics&max_results={}", account, count);

    let client = get_api(client, url).await;
    let res:TweetResponse = client.send().await?.json().await?;
    Ok(res)
}
