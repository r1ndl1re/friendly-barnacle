use flate2::read::GzDecoder;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Comment2013 {
    date: u32,
    no: u32,
    vpos: u32,
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
    title: String,
    description: String,
    thumbnail_url: String,
    upload_time: chrono::DateTime<chrono::Local>,
    length: u32,
    movie_type: String,
    size_high: u32,
    size_low: u32,
    view_counter: u32,
    comment_counter: u32,
    mylist_counter: u32,
    tags: Vec<VideoTag>,
}

#[derive(Debug, sqlx::FromRow)]
struct Video {
    id: u32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    video_code: String,
    title: String,
    description: String,
    watch_num: u32,
    comment_num: u32,
    mylist_num: u32,
    category: String,
    thumbnail_url: String,
    length: u32,
    file_type: String,
    upload_time: chrono::DateTime<chrono::Utc>,
    size_high: u32,
    size_low: u32,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let video_info = parse_video_dat("0000.dat.gz");
    println!("{:?}", video_info);

    let postgress_info = "postgres://app_user:hogehoge@localhost:5432/defaultdb";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(postgress_info)
        .await?;

    _ = create_video_table(&pool)
        .await
        .expect("failed to create video table");

    _ = create_tag_table(&pool)
        .await
        .expect("failed to create tag table");

    _ = create_relation_table(&pool)
        .await
        .expect("failed to create video_tag_relation table");

    let video_id = insert_video(&pool, &video_info)
        .await
        .expect("unable to insert");
    println!("{}", video_id);

    Ok(())
}

fn read_gz<P: AsRef<Path>>(path: P) -> String {
    let file = File::open(path).unwrap();
    let f = BufReader::new(file);
    let mut gz = GzDecoder::new(f);
    let mut s = String::new();
    gz.read_to_string(&mut s).unwrap();
    s
}

fn parse_video_dat<P: AsRef<Path>>(path: P) -> VideoInfo {
    let s = read_gz(path);
    let s: Vec<&str> = s.split("\n").collect();
    let video_info: VideoInfo = serde_json::from_str(&s[0]).unwrap();
    video_info
}

async fn create_video_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let sql = "
        CREATE TABLE IF NOT EXISTS video (
            id SERIAL NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            video_code VARCHAR(255) NOT NULL,
            title VARCHAR(255) NOT NULL,
            description VARCHAR(4000),
            watch_num INTEGER,
            comment_num INTEGER,
            mylist_num INTEGER,
            category VARCHAR(255),
            thumbnail_url VARCHAR(255),
            length INTEGER,
            file_type VARCHAR(3),
            upload_time TIMESTAMP WITH TIME ZONE,
            size_high INTEGER,
            size_low INTEGER,
            CONSTRAINT pk_video PRIMARY KEY (id),
            CONSTRAINT un1_video UNIQUE (video_code)
        )
    ";

    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

async fn create_tag_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let sql = "
        CREATE TABLE IF NOT EXISTS tag (
            id SERIAL NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            name VARCHAR(255) NOT NULL,
            CONSTRAINT pk_tag PRIMARY KEY (id),
            CONSTRAINT un1_tag UNIQUE (name)
        )
    ";

    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

async fn create_relation_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let sql = "
        CREATE TABLE IF NOT EXISTS video_tag_relation (
            id SERIAL NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            video_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            CONSTRAINT pk_video_tag_relation PRIMARY KEY (id),
            CONSTRAINT un1_video_tag_relation UNIQUE (video_id, tag_id),
            CONSTRAINT fk1_video_tag_relation FOREIGN KEY (video_id) REFERENCES video (id),
            CONSTRAINT fk2_video_tag_relation FOREIGN KEY (tag_id) REFERENCES tag (id)
        )
    ";

    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

async fn insert_video(pool: &Pool<Postgres>, video_info: &VideoInfo) -> Result<u32, sqlx::Error> {
    let sql = "
        INSERT INTO video (
            video_code
        ,   title
        ,   description
        ,   watch_num
        ,   comment_num
        ,   mylist_num
        ,   thumbnail_url
        ,   length
        ,   file_type
        ,   upload_time
        ,   size_high
        ,   size_low
        ) VALUES (
            $1
        ,   $2
        ,   $3
        ,   $4
        ,   $5
        ,   $6
        ,   $7
        ,   $8
        ,   $9
        ,   $10
        ,   $11
        ,   $12
        )
    ";
    let video = sqlx::query_as::<_, Video>(sql)
        .bind(&video_info.video_id)
        .bind(&video_info.title)
        .bind(&video_info.description)
        .bind(&video_info.view_counter)
        .bind(&video_info.comment_counter)
        .bind(&video_info.mylist_counter)
        .bind(&video_info.thumbnail_url)
        .bind(&video_info.length)
        .bind(&video_info.movie_type)
        .bind(&video_info.upload_time)
        .bind(&video_info.size_high)
        .bind(&video_info.size_low)
        .fetch_one(pool)
        .await?;
    Ok(video.id)
}
