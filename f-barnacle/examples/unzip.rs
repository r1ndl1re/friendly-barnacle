use std::{fs, io::Read};
use zip::ZipArchive;

fn main() {
    let zip_file_name = r"E:\dataset\nicocomm\data.20211222\comment\0000.zip";
    let extract_file_name = "nm2943000";
    unzip_by_file(zip_file_name, extract_file_name);
}

fn unzip_by_file<P: AsRef<std::path::Path>>(zip_file_name: P, extract_file_name: &str) {
    let zip_file = fs::File::open(zip_file_name).expect("Can't open zip file");
    let mut archive = ZipArchive::new(zip_file).unwrap();

    let mut file = archive.by_name(extract_file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
