use flate2::read::GzDecoder;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Comment2013 {
    date: usize,
    no: usize,
    vpos: usize,
    comment: String,
    command: String,
}

#[derive(Debug, Deserialize)]
struct VideoTag {
    tag: String,
    category: Option<u8>,
    lock: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct VideoInfo {
    video_id: String,
    thread_id: usize,
    title: String,
    description: String,
    thumbnail_url: String,
    upload_time: String,
    length: usize,
    movie_type: String,
    size_high: usize,
    size_low: usize,
    view_counter: usize,
    comment_counter: usize,
    mylist_counter: usize,
    last_res_body: String,
    tags: Vec<VideoTag>,
}

fn main() {
    let path = "0000.dat.gz";
    let s = read_gz(path);
    let s: Vec<&str> = s.split("\n").collect();
    let video_info: VideoInfo = serde_json::from_str(&s[0]).unwrap();
    println!("{:?}", video_info);
}

fn read_gz<P: AsRef<Path>>(path: P) -> String {
    let file = File::open(path).unwrap();
    let f = BufReader::new(file);
    let mut gz = GzDecoder::new(f);
    let mut s = String::new();
    gz.read_to_string(&mut s).unwrap();
    s
}
