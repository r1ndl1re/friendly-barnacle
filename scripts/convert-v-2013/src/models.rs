use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub(crate) struct Comment2013 {
    pub date: i32,
    pub no: i32,
    pub vpos: i32,
    pub comment: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VideoInfo {
    pub video_id: String,
    pub title: String,
    pub description: String,
    pub watch_num: i32,
    pub comment_num: i32,
    pub mylist_num: i32,
    pub category: Option<String>,
    pub tags: String,
    pub upload_time: chrono::DateTime<chrono::Local>,
    pub length: i32,
    pub file_type: String,
    pub size_high: i32,
    pub size_low: i32,
    pub thumbnail_url: Option<String>,
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
    pub thumbnail_url: Option<String>,
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
