use crate::getters::{follower_getter, tweet_getter, username_getter};
use crate::tweet_tokenizer::token_counter;
use crate::tweet::{Tweet, TweetForCSV};
use crate::converters::{sum_public_metrics, format_tweet_for_csv};
use crate::io_helpers::{tweet_writer, token_count_writer};
use std::collections::BTreeMap;
use std::iter::FromIterator;


pub async fn aggregate(target_account: &str,account_count:usize, tweet_count:usize) {
    let client = reqwest::Client::new();
    let mut token_map = BTreeMap::new();
    let mut tweet_map:BTreeMap<String, Vec<Tweet>> = BTreeMap::new();

    let mut followers = follower_getter(&client, target_account, account_count).await.unwrap().ids;
    while let Some(follower) = followers.pop() {
        let mut follower = follower.to_string();
        let tweets = tweet_getter(&client, &follower as &str, tweet_count).await;
        if let Ok(tweet) = tweets {
            if let Ok(user) = username_getter(&client, &follower).await {
                follower = user;
            }
            let tweet = tweet.data;
            token_counter(&mut token_map, &tweet.iter().map(|x| &x.text as &str).collect());
            tweet_map.insert(follower, tweet);
        } else {
            continue;
        }
    }
    let mut tweets:Vec<Tweet> = tweet_map.values().cloned().flatten().collect();
    tweets.sort_by(|a, b| sum_public_metrics(&b.public_metrics).cmp(&sum_public_metrics(&a.public_metrics)));
    let mut tweets_for_csv:Vec<TweetForCSV> = Vec::new();
    for tweet in tweets.iter() {
        tweets_for_csv.push(format_tweet_for_csv(&client, tweet).await);
    }
    let mut token_count: Vec<(String, usize)> = Vec::from_iter(token_map);
    token_count.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    token_count_writer(token_count);
    tweet_writer(tweets_for_csv);
}