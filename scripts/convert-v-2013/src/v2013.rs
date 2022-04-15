use crate::models::VideoInfo;

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub(crate) fn parse_video<P: AsRef<Path>>(path: P) -> Vec<VideoInfo> {
    let s = read_gz(path);
    let s: Vec<&str> = s.split("\n").collect();
    let mut video_infos = Vec::with_capacity(s.len());
    println!("{:?}", s);
    for s_ in s {
        let video_info = serde_json::from_str::<VideoInfo>(&s_);
        match video_info {
            Ok(n) => video_infos.push(n),
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
