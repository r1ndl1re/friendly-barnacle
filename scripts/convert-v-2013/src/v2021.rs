use crate::models;

use csv::QuoteStyle;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::Path;

fn parse_video<P: AsRef<Path>>(path: P) -> Vec<models::VideoInfo> {
    let s = read_jsonl(path);
    let s: Vec<&str> = s.split("\n").collect();
    let mut video_infos = Vec::with_capacity(s.len());

    for s_ in s {
        let video_info = serde_json::from_str::<models::VideoInfo>(s_);
        match video_info {
            Ok(n) => video_infos.push(n),
            Err(_) => continue,
        }
    }
    video_infos
}

fn read_jsonl<P: AsRef<Path>>(path: P) -> String {
    let file = File::open(path).unwrap();
    let mut f = BufReader::new(file);
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
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
