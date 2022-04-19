use crate::models;

use csv::QuoteStyle;
use serde::Deserialize;
use serde_with::chrono::datetime_utc_ts_seconds_from_any;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct VideoInfo2016 {
    video_id: String,
    title: String,
    description: String,
    #[serde(with = "datetime_utc_ts_seconds_from_any")]
    upload_time: chrono::DateTime<chrono::Utc>,
    length: u32,
    file_type: String,
    size_high: u32,
    size_low: u32,
    watch_num: u32,
    comment_num: u32,
    mylist_num: u32,
    category: Option<String>,
    tags: Vec<String>,
}

impl VideoInfo2016 {
    fn convert(self) -> models::VideoInfo {
        let tags = self.tags.join(" ");
        models::VideoInfo {
            video_id: self.video_id,
            title: self.title,
            description: self.description,
            watch_num: self.watch_num,
            comment_num: self.comment_num,
            mylist_num: self.mylist_num,
            tags,
            category: self.category,
            upload_time: self.upload_time.with_timezone(&chrono::Local),
            length: self.length,
            file_type: self.file_type,
            size_high: self.size_high,
            size_low: self.size_low,
        }
    }
}

pub(crate) fn parse_video<P: AsRef<Path>>(path: P) -> Vec<models::VideoInfo> {
    let s = unzip(path);
    let s: Vec<&str> = s.split("\n").collect();
    let mut video_infos = Vec::with_capacity(s.len());

    for s_ in s {
        let video_info = serde_json::from_str::<VideoInfo2016>(s_);
        match video_info {
            Ok(n) => video_infos.push(n.convert()),
            Err(_) => continue,
        }
    }
    video_infos
}

fn unzip<P: AsRef<Path>>(path: P) -> String {
    let file = File::open(path).unwrap();
    let mut s = String::new();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut file = archive.by_index(0).unwrap();
    file.read_to_string(&mut s).unwrap();
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
            writer.serialize(video_info)?;
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
