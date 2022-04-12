mod models;

use flate2::read::GzDecoder;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

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

    let video_id = insert_video(&pool, &video_info).await.unwrap();
    println!("{:?}", video_id);

    let tag_ids = insert_tags(&pool, &video_info.tags).await.unwrap();
    println!("{:?}", tag_ids);
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

fn parse_video_dat<P: AsRef<Path>>(path: P) -> models::VideoInfo {
    let s = read_gz(path);
    let s: Vec<&str> = s.split("\n").collect();
    let video_info: models::VideoInfo = serde_json::from_str(&s[0]).unwrap();
    video_info
}

async fn create_video_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let sql = "
        CREATE TABLE IF NOT EXISTS video (
            id SERIAL NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
            code VARCHAR(255) NOT NULL,
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
            CONSTRAINT un1_video UNIQUE (code)
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

async fn insert_video(
    pool: &Pool<Postgres>,
    video_info: &models::VideoInfo,
) -> Result<i32, sqlx::Error> {
    let sql = "
        INSERT INTO video (
            code
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
        RETURNING *
    ";
    let r = sqlx::query_as::<_, models::Video>(sql)
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
    println!("{:?}", r);
    Ok(r.id)
}

async fn insert_tags(
    pool: &Pool<Postgres>,
    tags: &Vec<models::VideoTagInfo>,
) -> Result<Vec<i32>, sqlx::Error> {
    let mut tag_ids: Vec<i32> = Vec::with_capacity(tags.len());
    let sql = "
        INSERT INTO tag (
            name
        ) VALUES (
            $1
        )
        RETURNING *
    ";

    for tag_ in tags {
        let r = sqlx::query_as::<_, models::Tag>(sql)
            .bind(&tag_.tag)
            .fetch_one(pool)
            .await;
        match r {
            Ok(n) => tag_ids.push(n.id),
            Err(_) => continue,
        }
    }
    Ok(tag_ids)
}
