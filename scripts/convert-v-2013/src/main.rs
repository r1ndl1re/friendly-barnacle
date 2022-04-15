mod models;
mod v2013;
mod v2021;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
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

    let video_infos = v2013::parse_video("0000.dat.gz");

    for video_info in video_infos {
        add_video(&pool, &video_info).await?;
    }
    Ok(())
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

pub(crate) async fn add_video(
    pool: &Pool<Postgres>,
    video_info: &models::VideoInfo,
) -> Result<(), sqlx::Error> {
    let video_id = upsert_video(&pool, &video_info).await?;
    let tag_ids = insert_tags(&pool, &video_info.tags).await?;
    insert_video_tag_relation(&pool, video_id, &tag_ids).await?;
    Ok(())
}

async fn upsert_video(
    pool: &Pool<Postgres>,
    video_info: &models::VideoInfo,
) -> Result<i32, sqlx::Error> {
    let r = check_video(pool, &video_info.video_id).await?;
    let video_id = match r {
        Some(n) => n.id,
        None => insert_video(pool, video_info).await?,
    };
    Ok(video_id)
}

async fn check_video(
    pool: &Pool<Postgres>,
    video_code: &str,
) -> Result<Option<models::Video>, sqlx::Error> {
    let sql = "
    SELECT
        *
    FROM
        video
    WHERE
        code = $1
    ";

    let r = sqlx::query_as::<_, models::Video>(sql)
        .bind(video_code)
        .fetch_optional(pool)
        .await?;

    Ok(r)
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

    let sql_exist = "SELECT * FROM tag WHERE name = $1";

    for tag_ in tags {
        // check tag already exists
        let r = sqlx::query_as::<_, models::Tag>(sql_exist)
            .bind(&tag_.tag)
            .fetch_optional(pool)
            .await?;

        let tag_id = match r {
            Some(n) => n.id,
            None => {
                let r = sqlx::query_as::<_, models::Tag>(sql)
                    .bind(&tag_.tag)
                    .fetch_one(pool)
                    .await?;
                r.id
            }
        };

        tag_ids.push(tag_id)
    }
    Ok(tag_ids)
}

async fn insert_video_tag_relation(
    pool: &Pool<Postgres>,
    video_id: i32,
    tag_ids: &Vec<i32>,
) -> Result<(), sqlx::Error> {
    let sql = "
        INSERT INTO video_tag_relation (
            video_id
        ,   tag_id
        ) VALUES (
            $1,
            $2
        )
    ";

    let sql_exists = "
        SELECT
            id
        FROM
            video_tag_relation
        WHERE
            video_id = $1
            AND tag_id =$2
        ";
    for tag_id in tag_ids {
        // check relation already exists
        let r = sqlx::query(sql_exists)
            .bind(video_id)
            .bind(tag_id)
            .fetch_optional(pool)
            .await?;

        match r {
            Some(_) => continue,
            None => {
                sqlx::query(sql)
                    .bind(video_id)
                    .bind(tag_id)
                    .execute(pool)
                    .await?;
            }
        }
    }
    Ok(())
}
