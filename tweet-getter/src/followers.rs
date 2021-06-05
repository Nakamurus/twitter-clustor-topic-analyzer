#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Followers {
    pub ids: Vec<i64>,
    #[serde(rename = "next_cursor")]
    pub next_cursor: i64,
    #[serde(rename = "next_cursor_str")]
    pub next_cursor_str: String,
    #[serde(rename = "previous_cursor")]
    pub previous_cursor: i64,
    #[serde(rename = "previous_cursor_str")]
    pub previous_cursor_str: String,
}
