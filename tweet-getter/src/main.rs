extern crate dotenv;

use crate::getters::{user_getter, follower_getter, tweet_getter};
mod getters;

use crate::tweet_tokenizer::token_counter;
mod tweet_tokenizer;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = reqwest::Client::new();
    println!("{:#?}", token_counter(vec!["ブランド「SHISEIDO」の公式Twitterアカウントです。新商品や旬の美容情報、キャンペーン・イベント情報などをお届けします。", "お客さまのキレイにお役立ちする #資生堂 のWebサービスです。 ■商品・ワタシプラスご利用のお問い合わせ：http://shiseido.jp/2ljPzrU ■SNS規約：http://shiseido.jp/1rck6si"]));
    // println!("{:#?}", tweet_getter(&client, "SHISEIDO_brand", 10).await);
}