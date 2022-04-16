use crate::models;
use sqlx::{Pool, Postgres};

pub(crate) async fn add_video(
    pool: &Pool<Postgres>,
    video_info: &models::VideoInfo,
) -> Result<(), sqlx::Error> {
    let video_id = upsert_video(&pool, &video_info).await?;
    let tag_ids = insert_tags(&pool, &video_info.tags).await?;
    insert_video_tag_relation(&pool, video_id, &tag_ids).await?;
    Ok(())
}

pub(crate) async fn upsert_video(
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

pub(crate) async fn check_video(
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

pub(crate) async fn insert_video(
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
        ,   category
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
        ,   $13
        )
        RETURNING *
    ";
    let r = sqlx::query_as::<_, models::Video>(sql)
        .bind(&video_info.video_id)
        .bind(&video_info.title)
        .bind(&video_info.description)
        .bind(&video_info.watch_num)
        .bind(&video_info.comment_num)
        .bind(&video_info.mylist_num)
        .bind(&video_info.thumbnail_url)
        .bind(&video_info.length)
        .bind(&video_info.file_type)
        .bind(&video_info.upload_time)
        .bind(&video_info.size_high)
        .bind(&video_info.size_low)
        .bind(&video_info.category)
        .fetch_one(pool)
        .await?;
    Ok(r.id)
}

pub(crate) async fn insert_tags(
    pool: &Pool<Postgres>,
    tags: &str,
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

    for tag in tags.split(" ") {
        // check tag already exists
        let r = sqlx::query_as::<_, models::Tag>(sql_exist)
            .bind(&tag)
            .fetch_optional(pool)
            .await?;

        let tag_id = match r {
            Some(n) => n.id,
            None => {
                let r = sqlx::query_as::<_, models::Tag>(sql)
                    .bind(&tag)
                    .fetch_one(pool)
                    .await?;
                r.id
            }
        };

        tag_ids.push(tag_id)
    }
    Ok(tag_ids)
}

pub(crate) async fn insert_video_tag_relation(
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
