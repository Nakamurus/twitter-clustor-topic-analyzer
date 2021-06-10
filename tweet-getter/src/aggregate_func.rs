use crate::getters::{follower_getter, tweet_getter};
use crate::tweet_tokenizer::token_counter;
use crate::tweet::{Tweet, TweetForCSV};
use crate::converters::{sum_public_metrics, format_tweet_for_csv};
use crate::io_helpers::{tweet_writer, token_count_writer};
use crate::util_structs::{MyTokenizer, ReqwestClientHeader};
use std::collections::BTreeMap;
use std::iter::FromIterator;


pub async fn aggregate(target_account: &str,account_count:usize, tweet_count:usize) {
    let mut client = ReqwestClientHeader::new();
    let mut tokenizer = MyTokenizer::new();
    let ignore_tokens = &tokenizer.ignore_tokens;

    let mut token_map = BTreeMap::new();
    let mut tweet_map:BTreeMap<String, (Tweet, usize)> = BTreeMap::new();

    let mut followers = follower_getter(&mut client, target_account, account_count).await.unwrap().ids;
    println!("Follower crawled!");
    while let Some(follower) = followers.pop() {
        let tweets = tweet_getter(&mut client, follower, tweet_count).await;
        if let Ok(ts) = tweets {
            let ts:Vec<Tweet> = ts.data;
            for tweet in ts.into_iter() {
                let text = tweet.clone().text;
                token_counter(&mut tokenizer.tokenizer, ignore_tokens, &mut token_map, &text);
                (*tweet_map.entry(text).or_insert((tweet, 0))).1 += 1;
            }
        }
    }
    println!("Tweets crawled!");
    let mut tweets_for_csv:Vec<TweetForCSV> = Vec::new();
    let mut seen_id = BTreeMap::new();
    for tweet in tweet_map.values() {
        tweets_for_csv.push(format_tweet_for_csv(&mut client, &mut seen_id, &tweet).await);
    }
    tweets_for_csv.sort_by(|a, b| b.frequency.cmp(&a.frequency).then(b.public_metrics_sum.cmp(&a.public_metrics_sum)));
    println!("Tweets converted for CSV!");
    let mut token_count: Vec<(String, usize)> = Vec::from_iter(token_map);
    token_count.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    token_count_writer(target_account, token_count).unwrap();
    tweet_writer(target_account, tweets_for_csv).unwrap();
}
