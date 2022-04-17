use crate::models::VideoInfo;

use flate2::read::GzDecoder;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct VideoTagInfo2013 {
    tag: String,
    category: Option<u8>,
    lock: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct VideoInfo2013 {
    video_id: String,
    title: String,
    description: String,
    thumbnail_url: Option<String>,
    upload_time: chrono::DateTime<chrono::Local>,
    length: u32,
    movie_type: String,
    size_high: u32,
    size_low: u32,
    watch_num: u32,
    comment_num: u32,
    mylist_num: u32,
    tags: Vec<VideoTagInfo2013>,
}

impl VideoInfo2013 {
    fn convert(self) -> VideoInfo {
        let tags: Vec<String> = self.tags.iter().map(|x| x.tag.clone()).collect();
        let tags = tags.join(" ");
        VideoInfo {
            video_id: self.video_id,
            title: self.title,
            description: self.description,
            watch_num: self.watch_num,
            comment_num: self.comment_num,
            mylist_num: self.mylist_num,
            category: None,
            tags: tags,
            upload_time: self.upload_time,
            length: self.length,
            file_type: self.movie_type,
            size_high: self.size_high,
            size_low: self.size_low,
            thumbnail_url: self.thumbnail_url,
        }
    }
}

pub(crate) fn parse_video<P: AsRef<Path>>(path: P) -> Vec<VideoInfo> {
    let s = read_gz(path);
    let s: Vec<&str> = s.split("\n").collect();
    let mut video_infos = Vec::with_capacity(s.len());
    for s_ in s {
        let video_info = serde_json::from_str::<VideoInfo2013>(&s_);
        match video_info {
            Ok(n) => video_infos.push(n.convert()),
            Err(_) => continue,
        };
    }
    video_infos
}

fn read_gz<P: AsRef<Path>>(path: P) -> String {
    let file = File::open(path).unwrap();
    let f = BufReader::new(file);
    let mut gz = GzDecoder::new(f);
    let mut s = String::new();
    gz.read_to_string(&mut s).unwrap();
    s
}
