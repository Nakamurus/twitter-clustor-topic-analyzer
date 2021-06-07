use crate::tweet::TweetForCSV;

use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("filed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

pub fn token_count_writer(v: Vec<(String, usize)>) -> Result<()> {
    println!("Input Token Counter Name");
    let filename = read::<String>();
    let filepath = std::path::Path::new(&filename);
    let mut wtr = csv::Writer::from_path(filepath)?;
    for (word, count) in v {
        wtr.write_record(&[word, count.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn tweet_writer(v: Vec<TweetForCSV>) -> Result<()> {
    println!("Input Tweets Aggregator Name");
    let filename = read::<String>();
    let filepath = std::path::Path::new(&filename);
    let mut wtr = csv::Writer::from_path(filepath)?;
    for t in v {
        wtr.serialize(TweetForCSV {
            retweet_count: t.retweet_count,
            reply_count: t.reply_count,
            like_count: t.like_count,
            quote_count: t.quote_count,
            user: t.user,
            text: t.text,
            url: t.url,
            image: t.image,
            url_title: t.url_title,
            retweet: t.retweet,
            source: t.source,
        })?;
    }
    wtr.flush()?;
    Ok(())
}
