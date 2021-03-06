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
    pub entities: Option<Entities>,
    pub id: String,
    #[serde(rename = "referenced_tweets")]
    #[serde(default)]
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    #[serde(rename = "attachments")]
    pub attachments: Option<Attachment>,
    #[serde(rename = "conversation_id")]
    pub conversation_id: Option<String>,
    pub text: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "reply_settings")]
    pub reply_settings: String,
    pub lang: String,
    #[serde(rename = "context_annotations")]
    pub context_annotations: Option<Vec<ContextAnnotation>>,
    #[serde(rename = "possibly_sensitive")]
    pub possibly_sensitive: bool,
    #[serde(rename = "in_reply_to_user_id")]
    pub in_reply_to_user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "media_keys")]
    pub media_keys: Option<Vec<String>>,
    #[serde(rename = "poll_ids")]
    pub poll_ids: Option<Vec<String>>,
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
    pub urls: Option<Vec<Url>>,
    #[serde(default)]
    pub annotations: Option<Vec<Annotation>>,
    #[serde(default)]
    pub mentions: Option<Vec<Mention>>,
    pub description: Option<Description>,
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
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Includes {
    pub users: Option<Vec<User>>,
    pub tweets: Option<Vec<Tweet>>,
    pub media: Option<Vec<Media>>,
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
    pub entities: Option<Entities>,
    pub username: String,
    pub location: Option<String>,
    pub verified: bool,
    pub protected: bool,
    pub description: Option<String>,
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
    pub mentions: Option<Vec<Mention>>,
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: Option<i64>,
    #[serde(rename = "id_str")]
    pub id_str: Option<String>,
    pub indices: Option<Vec<i64>>,
    #[serde(rename = "media_url")]
    pub media_url: Option<String>,
    #[serde(rename = "media_url_https")]
    pub media_url_https: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "display_url")]
    pub display_url: Option<String>,
    #[serde(rename = "expanded_url")]
    pub expanded_url: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub sizes: Option<Sizes>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes {
    pub thumb: Thumb,
    pub large: Large,
    pub medium: Medium,
    pub small: Small,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumb {
    pub w: i64,
    pub h: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Large {
    pub w: i64,
    pub h: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Medium {
    pub w: i64,
    pub h: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Small {
    pub w: i64,
    pub h: i64,
    pub resize: String,
}

#[derive(Debug, serde_derive::Serialize)]
pub struct TweetForCSV {
    pub retweet_count: i64,
    pub reply_count: i64,
    pub like_count: i64,
    pub quote_count: i64,
    pub public_metrics_sum: i64,
    pub user: String,
    pub text: String,
    pub url: String,
    pub image: String,
    pub url_title: String,
    pub retweet: String,
    pub source: String,
    pub frequency: usize,
}

impl TweetForCSV {
    pub fn new(
        retweet_count: i64,
        reply_count: i64,
        like_count: i64,
        quote_count: i64,
        public_metrics_sum: i64,
        frequency: usize,
        user: impl Into<String>,
        text: impl Into<String>,
        url: impl Into<String>,
        image: impl Into<String>,
        url_title: impl Into<String>,
        retweet: impl Into<String>,
        source: impl Into<String>,
    ) -> TweetForCSV {
        TweetForCSV {
            retweet_count: retweet_count,
            reply_count: reply_count,
            like_count: like_count,
            quote_count: quote_count,
            public_metrics_sum: public_metrics_sum,
            frequency: frequency,
            user: user.into(),
            text: text.into(),
            url: url.into(),
            image: image.into(),
            url_title: url_title.into(),
            retweet: retweet.into(),
            source: source.into(),
        }
    }
}
