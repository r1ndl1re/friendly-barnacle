use crate::models::VideoInfo;

use csv::QuoteStyle;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::Path;

const PATTERN: &str = "./nicocomm/data.20211222/video/*.jsonl";
const CSV_PATH: &str = "./nicocomm/data.20211222/video.csv";
const CSV_TAG_PATH: &str = "./nicocomm/data.20211222/tag.csv";

fn parse_video<P: AsRef<Path>>(path: P) -> Vec<VideoInfo> {
    let s = read_jsonl(path);
    let s: Vec<&str> = s.split("\n").collect();
    let mut video_infos = Vec::with_capacity(s.len());

    for s_ in s {
        let video_info = serde_json::from_str::<VideoInfo>(s_);
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

pub(crate) fn create() -> Result<(), csv::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(CSV_PATH)
        .unwrap();
    let mut writer = csv::WriterBuilder::new()
        .quote_style(QuoteStyle::NonNumeric)
        .from_writer(file);

    for entry in glob::glob(PATTERN).unwrap() {
        let path = entry.unwrap();
        let video_infos = parse_video(path);
        for video_info in video_infos {
            writer.serialize(video_info)?;
        }
    }
    writer.flush();
    Ok(())
}

pub(crate) fn create_tag_csv() -> Result<(), csv::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(CSV_TAG_PATH)
        .unwrap();
    let mut writer = csv::WriterBuilder::new()
        .quote_style(QuoteStyle::NonNumeric)
        .from_writer(file);

    for entry in glob::glob(PATTERN).unwrap() {
        let path = entry.unwrap();
        let video_infos = parse_video(path);
        for video_info in video_infos {
            writer.serialize(video_info)?;
        }
    }
    writer.flush().unwrap();
    Ok(())
}
