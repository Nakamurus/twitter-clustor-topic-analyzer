use crate::getters::{follower_getter, tweet_getter, user_getter};
use crate::tweet_tokenizer::token_counter;
use crate::tweet::{Tweet, TweetForCSV};
use crate::converters::format_tweet_for_csv;
use crate::io_helpers::{follower_token_count_writer, follower_tweet_count_writer, target_token_count_writer, target_tweet_count_writer};
use crate::util_structs::{MyTokenizer, ReqwestClientHeader};
use std::collections::BTreeMap;
use std::iter::FromIterator;


pub async fn aggregate(target_account: &str,account_count:usize, tweet_count:usize) {
    let mut client = ReqwestClientHeader::new();
    let mut followers = follower_getter(&mut client, target_account, account_count).await.unwrap().ids;
    let target_id = user_getter(&mut client, &target_account).await.unwrap().id.parse::<i64>().unwrap();
    let mut target = vec![target_id];
    println!("Follower crawled!");

    tweet_getter_wrapper(&mut client, &mut target, target_account, tweet_count, false).await;
    tweet_getter_wrapper(&mut client, &mut followers, target_account, tweet_count, true).await;
}

async fn tweet_getter_wrapper(client: &mut ReqwestClientHeader, targets: &mut Vec<i64>, account:&str, tweet_count:usize, isfollower:bool) {
    let mut tokenizer = MyTokenizer::new();
    let ignore_tokens = &tokenizer.ignore_tokens;
    let mut token_map = BTreeMap::new();
    let mut tweet_map:BTreeMap<String, (Tweet, usize)> = BTreeMap::new();

    while let Some(target) = targets.pop() {
        let mut next_token = String::new();
        if !isfollower {
            for _ in 0usize..=30 {
                let tweets = tweet_getter(client, target, tweet_count, &next_token as &str).await;
                if let Ok(ts) = tweets {
                    next_token = ts.meta.next_token;
                    let ts:Vec<Tweet> = ts.data;
                    for tweet in ts.into_iter() {
                        let text = tweet.clone().text;
                        token_counter(&mut tokenizer.tokenizer, ignore_tokens, &mut token_map, &text);
                        (*tweet_map.entry(text).or_insert((tweet, 0))).1 += 1;
                    }
                }
            }
        } else {
            let tweets = tweet_getter(client, target, tweet_count, "").await;
            if let Ok(ts) = tweets {
                let ts:Vec<Tweet> = ts.data;
                for tweet in ts.into_iter() {
                    let text = tweet.clone().text;
                    token_counter(&mut tokenizer.tokenizer, ignore_tokens, &mut token_map, &text);
                    (*tweet_map.entry(text).or_insert((tweet, 0))).1 += 1;
                }
            }
        }
    }
    let tweets_for_csv = csv_converter_wrapper(client, tweet_map).await;
    let mut token_count: Vec<(String, usize)> = Vec::from_iter(token_map);
    token_count.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    if isfollower {
        target_token_count_writer(account, token_count);
        target_tweet_count_writer(account, tweets_for_csv);
    } else {
        follower_token_count_writer(account, token_count);
        follower_tweet_count_writer(account, tweets_for_csv);
    }
}

async fn csv_converter_wrapper(client: &mut ReqwestClientHeader, tweet_map:BTreeMap<String, (Tweet, usize)>) -> Vec<TweetForCSV> {
    let mut tweets_for_csv:Vec<TweetForCSV> = Vec::new();
    let mut seen_id = BTreeMap::new();
    for tweet in tweet_map.values() {
        tweets_for_csv.push(format_tweet_for_csv(client, &mut seen_id, &tweet).await);
    }
    tweets_for_csv.sort_by(|a, b| b.frequency.cmp(&a.frequency).then(b.public_metrics_sum.cmp(&a.public_metrics_sum)));
    println!("Tweets converted for CSV!");
    tweets_for_csv
}