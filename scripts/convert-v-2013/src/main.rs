use std::path::Path;

mod models;
mod v2013;
mod v2016;
mod v2021;

fn main() {
    let path_2021 = Path::new("./nicocomm/data.20211222/video");
    let path_2018 = Path::new("./nicocomm/data.20181214/video");
    let path_2016 = Path::new("./nicocomm/data.20161216/video");
    let path_2013 = Path::new("./nicocomm/data.20130427/video");

    v2021::create(path_2021, "jsonl").unwrap();
    v2016::create(path_2018, "zip").unwrap();
    v2016::create(path_2016, "zip").unwrap();
    v2013::create(path_2013, "gz").unwrap();

    v2021::create_tag_csv(path_2021, "jsonl").unwrap();
    v2016::create_tag_csv(path_2018, "zip").unwrap();
    v2016::create_tag_csv(path_2016, "zip").unwrap();
    v2013::create_tag_csv(path_2013, "gz").unwrap();
}
