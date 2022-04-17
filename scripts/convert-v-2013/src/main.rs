mod crud;
mod ddl;
mod models;
mod v2013;
mod v2016;
mod v2021;

use sqlx::postgres::PgPoolOptions;

const DATABASE_URL: &str = "postgres://app_user:hogehoge@localhost:5432/defaultdb";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    _ = ddl::create_video_table(&pool)
        .await
        .expect("failed to create video table");

    _ = ddl::create_tag_table(&pool)
        .await
        .expect("failed to create tag table");

    _ = ddl::create_relation_table(&pool)
        .await
        .expect("failed to create video_tag_relation table");

    let video_infos = v2016::parse_video("./nicocomm/data.20161216/video/0000.zip");
    println!("{:?}", video_infos[0].tags);

    for video_info in video_infos {
        crud::add_video(&pool, &video_info).await?;
    }
    Ok(())
}
