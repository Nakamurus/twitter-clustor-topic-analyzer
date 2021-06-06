use crate::getters::{user_getter, follower_getter, tweet_getter, extract_tweet_info};

use crate::tweet_tokenizer::token_counter;

use std::collections::BTreeMap;
use std::iter::FromIterator;

pub async fn aggregate(target_account: &str,account_count:usize, tweet_count:usize) {
    let client = reqwest::Client::new();
    let mut followers = follower_getter(&client, target_account.to_string(), account_count).await.unwrap().ids;
    let mut token_map = BTreeMap::new();
    let mut tweet_map = BTreeMap::new();
    while let Some(follower) = followers.pop() {
        let mut follower = follower.to_string();
        let tweets = tweet_getter(&client, follower.clone(), tweet_count).await;
        if let Ok(tweet) = tweets {
            if let Ok(user) = user_getter(&client, follower.clone()).await {
                follower = user.screen_name;
            }
            tweet_map.insert(follower, extract_tweet_info(&tweet.data));
            token_counter(&mut token_map, &tweet.data.iter().map(|x| &x.text as &str).collect());
        } else {
            continue;
        }
    }
    let mut token_count: Vec<(String, usize)> = Vec::from_iter(token_map);
    token_count.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    println!("{:#?}", token_count);
    println!("{:#?}", tweet_map);
}