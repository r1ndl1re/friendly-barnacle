use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct VideoInfo {
    pub video_id: String,
    pub title: String,
    pub description: String,
    pub watch_num: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub comment_num: u32,
    pub mylist_num: u32,
    pub category: Option<String>,
    pub tags: String,
    pub upload_time: chrono::DateTime<chrono::Local>,
    pub length: u32,
    pub file_type: String,
    pub size_high: u32,
    pub size_low: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TagInfo {
    pub video_id: String,
    pub tag_name: String,
}
