use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweet {
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub id: i64,
    #[serde(rename = "id_str")]
    pub id_str: String,
    #[serde(rename = "full_text")]
    pub full_text: String,
    pub truncated: bool,
    #[serde(rename = "display_text_range")]
    pub display_text_range: Vec<i64>,
    pub entities: Entities,
    pub source: String,
    #[serde(rename = "in_reply_to_status_id")]
    pub in_reply_to_status_id: i64,
    #[serde(rename = "in_reply_to_status_id_str")]
    pub in_reply_to_status_id_str: String,
    #[serde(rename = "in_reply_to_user_id")]
    pub in_reply_to_user_id: i64,
    #[serde(rename = "in_reply_to_user_id_str")]
    pub in_reply_to_user_id_str: String,
    #[serde(rename = "in_reply_to_screen_name")]
    pub in_reply_to_screen_name: String,
    pub user: User,
    pub geo: ::serde_json::Value,
    pub coordinates: ::serde_json::Value,
    pub place: ::serde_json::Value,
    pub contributors: ::serde_json::Value,
    #[serde(rename = "is_quote_status")]
    pub is_quote_status: bool,
    #[serde(rename = "retweet_count")]
    pub retweet_count: i64,
    #[serde(rename = "favorite_count")]
    pub favorite_count: i64,
    pub favorited: bool,
    pub retweeted: bool,
    #[serde(rename = "possibly_sensitive")]
    pub possibly_sensitive: bool,
    #[serde(rename = "possibly_sensitive_appealable")]
    pub possibly_sensitive_appealable: bool,
    pub lang: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities {
    pub hashtags: Vec<::serde_json::Value>,
    pub symbols: Vec<::serde_json::Value>,
    #[serde(rename = "user_mentions")]
    pub user_mentions: Vec<::serde_json::Value>,
    pub urls: Vec<Url>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub url: String,
    #[serde(rename = "expanded_url")]
    pub expanded_url: String,
    #[serde(rename = "display_url")]
    pub display_url: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    #[serde(rename = "id_str")]
    pub id_str: String,
    pub name: String,
    #[serde(rename = "screen_name")]
    pub screen_name: String,
    pub location: String,
    pub description: String,
    pub url: String,
    pub entities: Entities2,
    pub protected: bool,
    #[serde(rename = "followers_count")]
    pub followers_count: i64,
    #[serde(rename = "friends_count")]
    pub friends_count: i64,
    #[serde(rename = "listed_count")]
    pub listed_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "favourites_count")]
    pub favourites_count: i64,
    #[serde(rename = "utc_offset")]
    pub utc_offset: ::serde_json::Value,
    #[serde(rename = "time_zone")]
    pub time_zone: ::serde_json::Value,
    #[serde(rename = "geo_enabled")]
    pub geo_enabled: bool,
    pub verified: bool,
    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,
    pub lang: ::serde_json::Value,
    #[serde(rename = "contributors_enabled")]
    pub contributors_enabled: bool,
    #[serde(rename = "is_translator")]
    pub is_translator: bool,
    #[serde(rename = "is_translation_enabled")]
    pub is_translation_enabled: bool,
    #[serde(rename = "profile_background_color")]
    pub profile_background_color: String,
    #[serde(rename = "profile_background_image_url")]
    pub profile_background_image_url: String,
    #[serde(rename = "profile_background_image_url_https")]
    pub profile_background_image_url_https: String,
    #[serde(rename = "profile_background_tile")]
    pub profile_background_tile: bool,
    #[serde(rename = "profile_image_url")]
    pub profile_image_url: String,
    #[serde(rename = "profile_image_url_https")]
    pub profile_image_url_https: String,
    #[serde(rename = "profile_banner_url")]
    pub profile_banner_url: String,
    #[serde(rename = "profile_link_color")]
    pub profile_link_color: String,
    #[serde(rename = "profile_sidebar_border_color")]
    pub profile_sidebar_border_color: String,
    #[serde(rename = "profile_sidebar_fill_color")]
    pub profile_sidebar_fill_color: String,
    #[serde(rename = "profile_text_color")]
    pub profile_text_color: String,
    #[serde(rename = "profile_use_background_image")]
    pub profile_use_background_image: bool,
    #[serde(rename = "has_extended_profile")]
    pub has_extended_profile: bool,
    #[serde(rename = "default_profile")]
    pub default_profile: bool,
    #[serde(rename = "default_profile_image")]
    pub default_profile_image: bool,
    pub following: ::serde_json::Value,
    #[serde(rename = "follow_request_sent")]
    pub follow_request_sent: ::serde_json::Value,
    pub notifications: ::serde_json::Value,
    #[serde(rename = "translator_type")]
    pub translator_type: String,
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
    pub url: String,
    #[serde(rename = "expanded_url")]
    pub expanded_url: String,
    #[serde(rename = "display_url")]
    pub display_url: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub urls: Vec<::serde_json::Value>,
}
