#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetResponse {
    pub data: Vec<Tweet>,
    pub includes: Option<Includes>,
    pub errors: Option<Vec<Error>>,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweet {
    #[serde(rename = "public_metrics")]
    pub public_metrics: PublicMetrics,
    pub source: String,
    pub entities: Entities,
    pub id: String,
    #[serde(rename = "referenced_tweets")]
    #[serde(default)]
    pub referenced_tweets: Vec<ReferencedTweet>,
    #[serde(rename = "conversation_id")]
    pub conversation_id: String,
    pub text: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "reply_settings")]
    pub reply_settings: String,
    pub lang: String,
    #[serde(rename = "context_annotations")]
    pub context_annotations: Vec<ContextAnnotation>,
    #[serde(rename = "possibly_sensitive")]
    pub possibly_sensitive: bool,
    #[serde(rename = "in_reply_to_user_id")]
    pub in_reply_to_user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMetrics {
    #[serde(rename = "retweet_count")]
    pub retweet_count: i64,
    #[serde(rename = "reply_count")]
    pub reply_count: i64,
    #[serde(rename = "like_count")]
    pub like_count: i64,
    #[serde(rename = "quote_count")]
    pub quote_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities {
    pub urls: Vec<Url>,
    #[serde(default)]
    pub annotations: Vec<Annotation>,
    #[serde(default)]
    pub mentions: Vec<Mention>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub start: i64,
    pub end: i64,
    pub url: String,
    #[serde(rename = "expanded_url")]
    pub expanded_url: String,
    #[serde(rename = "display_url")]
    pub display_url: String,
    pub images: Option<Vec<Image>>,
    pub status: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "unwound_url")]
    pub unwound_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub start: i64,
    pub end: i64,
    pub probability: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "normalized_text")]
    pub normalized_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub start: i64,
    pub end: i64,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextAnnotation {
    pub domain: Domain,
    pub entity: Entity,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Includes {
    pub users: Vec<User>,
    pub tweets: Vec<Tweet>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "profile_image_url")]
    pub profile_image_url: String,
    pub id: String,
    pub url: String,
    #[serde(rename = "public_metrics")]
    pub public_metrics: PublicMetrics2,
    pub name: String,
    pub entities: Entities2,
    pub username: String,
    pub location: String,
    pub verified: bool,
    pub protected: bool,
    pub description: String,
    #[serde(rename = "pinned_tweet_id")]
    pub pinned_tweet_id: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMetrics2 {
    #[serde(rename = "followers_count")]
    pub followers_count: i64,
    #[serde(rename = "following_count")]
    pub following_count: i64,
    #[serde(rename = "tweet_count")]
    pub tweet_count: i64,
    #[serde(rename = "listed_count")]
    pub listed_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities2 {
    pub url: Url2,
    pub description: Description,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url2 {
    pub urls: Vec<Url3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url3 {
    pub start: i64,
    pub end: i64,
    pub url: String,
    #[serde(rename = "expanded_url")]
    pub expanded_url: String,
    #[serde(rename = "display_url")]
    pub display_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub hashtags: Option<Vec<Hashtag>>,
    #[serde(default)]
    pub mentions: Vec<Mention2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashtag {
    pub start: i64,
    pub end: i64,
    pub tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention2 {
    pub start: i64,
    pub end: i64,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMetrics3 {
    #[serde(rename = "retweet_count")]
    pub retweet_count: i64,
    #[serde(rename = "reply_count")]
    pub reply_count: i64,
    #[serde(rename = "like_count")]
    pub like_count: i64,
    #[serde(rename = "quote_count")]
    pub quote_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities3 {
    #[serde(default)]
    pub annotations: Vec<Annotation2>,
    pub urls: Vec<Url4>,
    #[serde(default)]
    pub mentions: Vec<Mention3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation2 {
    pub start: i64,
    pub end: i64,
    pub probability: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "normalized_text")]
    pub normalized_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url4 {
    pub start: i64,
    pub end: i64,
    pub url: String,
    #[serde(rename = "expanded_url")]
    pub expanded_url: String,
    #[serde(rename = "display_url")]
    pub display_url: String,
    pub images: Option<Vec<Image2>>,
    pub status: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "unwound_url")]
    pub unwound_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image2 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention3 {
    pub start: i64,
    pub end: i64,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferencedTweet2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextAnnotation2 {
    pub domain: Domain2,
    pub entity: Entity2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain2 {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity2 {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    pub parameter: String,
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    pub section: Option<String>,
    pub title: String,
    pub value: String,
    pub detail: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "oldest_id")]
    pub oldest_id: String,
    #[serde(rename = "newest_id")]
    pub newest_id: String,
    #[serde(rename = "result_count")]
    pub result_count: i64,
    #[serde(rename = "next_token")]
    pub next_token: String,
}
