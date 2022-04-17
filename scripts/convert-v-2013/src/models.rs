use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub(crate) struct Comment2013 {
    pub date: i32,
    pub no: i32,
    pub vpos: i32,
    pub comment: String,
    pub command: String,
}

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
    pub upload_time: chrono::DateTime<chrono::Local>,
    pub length: u32,
    pub file_type: String,
    pub size_high: u32,
    pub size_low: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TagInfo {
    video_id: String,
    tag_name: String,
}

#[derive(Debug, FromRow)]
pub(crate) struct Video {
    pub id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub code: String,
    pub title: String,
    pub description: String,
    pub watch_num: i32,
    pub comment_num: i32,
    pub mylist_num: i32,
    pub category: Option<String>,
    pub length: i32,
    pub file_type: String,
    pub upload_time: chrono::DateTime<chrono::Utc>,
    pub size_high: i32,
    pub size_low: i32,
}

#[derive(Debug, FromRow)]
pub(crate) struct Tag {
    pub id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}
