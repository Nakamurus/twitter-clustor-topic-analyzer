use crate::tweet::{Tweet,PublicMetrics, TweetForCSV};
use crate::getters::username_getter;

pub fn sum_public_metrics(x: &PublicMetrics) -> i64 {
    x.like_count + x.quote_count + x.reply_count + x.retweet_count
}

pub async fn format_tweet_for_csv(client: &reqwest::Client, x: &Tweet) -> TweetForCSV {
    let x = x.clone();
    let username = username_getter(client, &x.author_id).await;

    let mut retweet = "Original".to_string();
    if let Some(retweets) = x.referenced_tweets {
        retweet = retweets[0].clone().type_field;
    }
    let mut url = String::new();
    let mut url_title = String::new();
    let mut image = String::new();
    if let Some(entities) = x.entities {
        if let Some(urls) = entities.urls {
            url = urls.iter().map(|x| &x.expanded_url as &str).collect::<Vec<&str>>().join("\n");
            url_title = urls.iter().map(|x| x.title.as_deref().unwrap_or("")).collect::<Vec<&str>>().join("\n");
            image = urls.clone().into_iter().map(|x| x.images.unwrap_or(Vec::new()).into_iter().map(|y| y.url).collect::<Vec<String>>()).flatten().collect::<Vec<String>>().join("\n");
        }
    };
    let metrics = x.public_metrics;
    TweetForCSV::new(
        metrics.retweet_count,
        metrics.reply_count,
        metrics.like_count,
        metrics.quote_count,
        username.as_deref().unwrap_or(&x.author_id) as &str,
        x.text,
        url,
        image,
        url_title,
        retweet,
        x.source,
    )
}