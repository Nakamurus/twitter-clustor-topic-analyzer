#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultUserResponse {
    pub data: DefaultUser,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultUser {
    pub id: String,
    pub name: String,
    pub username: String,
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
    #[serde(rename = "profile_location")]
    pub profile_location: ::serde_json::Value,
    pub description: String,
    pub url: String,
    pub entities: Entities,
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
    pub geo_enabled: ::serde_json::Value,
    pub verified: bool,
    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,
    pub lang: ::serde_json::Value,
    #[serde(rename = "contributors_enabled")]
    pub contributors_enabled: ::serde_json::Value,
    #[serde(rename = "is_translator")]
    pub is_translator: ::serde_json::Value,
    #[serde(rename = "is_translation_enabled")]
    pub is_translation_enabled: ::serde_json::Value,
    #[serde(rename = "profile_background_color")]
    pub profile_background_color: ::serde_json::Value,
    #[serde(rename = "profile_background_image_url")]
    pub profile_background_image_url: ::serde_json::Value,
    #[serde(rename = "profile_background_image_url_https")]
    pub profile_background_image_url_https: ::serde_json::Value,
    #[serde(rename = "profile_background_tile")]
    pub profile_background_tile: ::serde_json::Value,
    #[serde(rename = "profile_image_url")]
    pub profile_image_url: ::serde_json::Value,
    #[serde(rename = "profile_image_url_https")]
    pub profile_image_url_https: String,
    #[serde(rename = "profile_banner_url")]
    pub profile_banner_url: ::serde_json::Value,
    #[serde(rename = "profile_link_color")]
    pub profile_link_color: ::serde_json::Value,
    #[serde(rename = "profile_sidebar_border_color")]
    pub profile_sidebar_border_color: ::serde_json::Value,
    #[serde(rename = "profile_sidebar_fill_color")]
    pub profile_sidebar_fill_color: ::serde_json::Value,
    #[serde(rename = "profile_text_color")]
    pub profile_text_color: ::serde_json::Value,
    #[serde(rename = "profile_use_background_image")]
    pub profile_use_background_image: ::serde_json::Value,
    #[serde(rename = "has_extended_profile")]
    pub has_extended_profile: ::serde_json::Value,
    #[serde(rename = "default_profile")]
    pub default_profile: bool,
    #[serde(rename = "default_profile_image")]
    pub default_profile_image: bool,
    pub following: ::serde_json::Value,
    #[serde(rename = "follow_request_sent")]
    pub follow_request_sent: ::serde_json::Value,
    pub notifications: ::serde_json::Value,
    #[serde(rename = "translator_type")]
    pub translator_type: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities {
    pub url: Url,
    pub description: Description,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub urls: Vec<Url2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url2 {
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
