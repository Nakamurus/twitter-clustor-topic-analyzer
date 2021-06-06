#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetResponse {
    pub data: Vec<Tweet>,
    pub includes: Includes,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweet {
    pub entities: Entities,
    #[serde(rename = "author_id")]
    pub author_id: String,
    pub text: String,
    #[serde(rename = "public_metrics")]
    pub public_metrics: PublicMetrics,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities {
    pub annotations: Vec<Annotation>,
    #[serde(default)]
    pub hashtags: Vec<Hashtag>,
    #[serde(default)]
    pub mentions: Vec<Mention>,
    #[serde(default)]
    pub urls: Vec<Url>,
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
pub struct Hashtag {
    pub start: i64,
    pub end: i64,
    pub tag: String,
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
pub struct Includes {
    pub users: Vec<User>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
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
