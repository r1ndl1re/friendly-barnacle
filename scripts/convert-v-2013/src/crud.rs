use crate::models;
use sqlx::{Pool, Postgres};

pub(crate) async fn add_video(
    pool: &Pool<Postgres>,
    video_info: &models::VideoInfo,
) -> Result<(), sqlx::Error> {
    let video_id = upsert_video(&pool, &video_info).await?;
    let tag_ids = insert_tags(&pool, &video_info.tags).await?;
    insert_video_tag_relation(&pool, video_id, tag_ids).await?;
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
        RETURNING id
    ";
    let r = sqlx::query_as::<_, models::Video>(sql)
        .bind(&video_info.video_id)
        .bind(&video_info.title)
        .bind(&video_info.description)
        .bind(&video_info.watch_num)
        .bind(&video_info.comment_num)
        .bind(&video_info.mylist_num)
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

async fn insert_tags(pool: &Pool<Postgres>, tags: &str) -> Result<Vec<i32>, sqlx::Error> {
    let mut tag_ids: Vec<i32> = Vec::with_capacity(tags.len());
    let sql = "
        INSERT INTO tag (
            name
        ) VALUES (
            $1
        )
        RETURNING *
    ";

    let tags: Vec<&str> = tags.split(" ").collect();
    // check tag already exists
    // https://github.com/launchbadge/sqlx/blob/master/FAQ.md#how-can-i-do-a-select--where-foo-in--query
    let sql_exist = "SELECT * FROM tag WHERE name = ANY($1)";
    let r = sqlx::query_as::<_, models::Tag>(sql_exist)
        .bind(&tags)
        .fetch_all(pool)
        .await?;

    for tag in tags.into_iter() {
        let is_contains = r.iter().any(|e| e.name == tag);
        if !is_contains {
            let tag_id = sqlx::query_as::<_, models::Tag>(sql)
                .bind(&tag)
                .fetch_one(pool)
                .await?
                .id;
            tag_ids.push(tag_id);
        }
    }
    Ok(tag_ids)
}

async fn insert_video_tag_relation(
    pool: &Pool<Postgres>,
    video_id: i32,
    tag_ids: Vec<i32>,
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

    for tag_id in tag_ids {
        sqlx::query(sql)
            .bind(video_id)
            .bind(tag_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
