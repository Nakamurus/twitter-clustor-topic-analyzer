use crate::tweet::{Tweet,PublicMetrics, TweetForCSV};
use crate::getters::user_getter;
use crate::util_structs::ReqwestClientHeader;
use std::collections::BTreeMap;

pub fn sum_public_metrics(x: &PublicMetrics) -> i64 {
    x.like_count + x.quote_count + x.reply_count + x.retweet_count
}

pub async fn format_tweet_for_csv(client: &mut ReqwestClientHeader, seen_id: &mut BTreeMap<String, String>, t: &(Tweet, usize)) -> TweetForCSV {
    let x = t.clone().0;
    let frequency = t.1;
    let mut username = String::new();
    if let Some(name) = seen_id.get(&x.author_id) {
        username = name.to_string();
    } else {
        let id = x.clone().author_id;
        let name = user_getter(client, &x.author_id).await.unwrap().name;
        seen_id.insert(id, name.clone());
        username = name;
    };

    let mut retweet = "Original".to_string();
    if let Some(retweets) = x.clone().referenced_tweets {
        retweet = retweets[0].clone().type_field;
    }
    let mut url = String::new();
    let mut url_title = String::new();
    let mut image = String::new();
    if let Some(entities) = x.clone().entities {
        if let Some(urls) = entities.urls {
            url = urls.iter().map(|x| &x.expanded_url as &str).collect::<Vec<&str>>().join("\n");
            url_title = urls.iter().map(|x| x.title.as_deref().unwrap_or("")).collect::<Vec<&str>>().join("\n");
            image = urls.clone().into_iter().map(|x| x.images.unwrap_or(Vec::new()).into_iter().map(|y| y.url).collect::<Vec<String>>()).flatten().collect::<Vec<String>>().join("\n");
        }
    };
    let x = x.clone();
    let metrics = x.public_metrics;
    TweetForCSV::new(
        metrics.retweet_count,
        metrics.reply_count,
        metrics.like_count,
        metrics.quote_count,
        sum_public_metrics(&metrics),
        frequency,
        username,
        x.text,
        url,
        image,
        url_title,
        retweet,
        x.source,
    )
}
