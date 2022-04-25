use crate::models;

use csv::QuoteStyle;
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct VideoTagInfo2013 {
    tag: String,
}

#[derive(Debug, Deserialize)]
struct VideoInfo2013 {
    video_id: String,
    title: String,
    description: String,
    upload_time: chrono::DateTime<chrono::Local>,
    length: u32,
    movie_type: String,
    size_high: u32,
    size_low: u32,
    #[serde(alias = "view_counter")]
    watch_num: u32,
    #[serde(alias = "comment_counter")]
    comment_num: u32,
    #[serde(alias = "mylist_counter")]
    mylist_num: u32,
    tags: Vec<VideoTagInfo2013>,
}

impl VideoInfo2013 {
    fn convert(self) -> models::VideoInfo {
        let tags: Vec<String> = self.tags.iter().map(|x| x.tag.clone()).collect();
        let tags = tags.join(" ");
        models::VideoInfo {
            video_id: self.video_id,
            title: self.title,
            description: self.description,
            watch_num: self.watch_num,
            comment_num: self.comment_num,
            mylist_num: self.mylist_num,
            category: None,
            tags,
            upload_time: self.upload_time,
            length: self.length,
            file_type: self.movie_type,
            size_high: self.size_high,
            size_low: self.size_low,
        }
    }
}

pub(crate) fn parse_video<P: AsRef<Path>>(path: P) -> Vec<models::VideoInfo> {
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

pub(crate) fn create<P: AsRef<Path>>(path: P, extension: &str) -> Result<(), csv::Error> {
    let output_file_path = path.as_ref().parent().unwrap().join("video.csv");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_file_path)
        .unwrap();
    let mut writer = csv::WriterBuilder::new()
        .quote_style(QuoteStyle::NonNumeric)
        .from_writer(file);

    let pattern = format!("{}/*.{}", path.as_ref().to_str().unwrap(), extension);
    for entry in glob::glob(&pattern).unwrap() {
        let p = entry.unwrap();
        println!("video: {}", p.display());
        let video_infos = parse_video(p);
        for video_info in video_infos {
            writer.serialize(video_info.remove_tag())?;
        }
    }
    writer.flush().unwrap();
    Ok(())
}

pub(crate) fn create_tag_csv<P: AsRef<Path>>(path: P, extension: &str) -> Result<(), csv::Error> {
    let output_file_path = path.as_ref().parent().unwrap().join("tag.csv");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_file_path)
        .unwrap();
    let mut writer = csv::WriterBuilder::new()
        .quote_style(QuoteStyle::NonNumeric)
        .from_writer(file);

    let pattern = format!("{}/*.{}", path.as_ref().to_str().unwrap(), extension);
    for entry in glob::glob(&pattern).unwrap() {
        let p = entry.unwrap();
        println!("tag: {}", p.display());
        let video_infos = parse_video(p);
        for video_info in video_infos {
            for tag in video_info.tags.split(" ") {
                writer.serialize(models::TagInfo {
                    video_id: video_info.video_id.to_string(),
                    tag_name: tag.to_string(),
                })?;
            }
        }
    }
    writer.flush().unwrap();
    Ok(())
}
