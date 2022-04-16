mod crud;
mod ddl;
mod models;
mod v2013;
mod v2021;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let postgress_info = "postgres://app_user:hogehoge@localhost:5432/defaultdb";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(postgress_info)
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

    let video_infos = v2013::parse_video("0000.dat.gz");

    for video_info in video_infos {
        crud::add_video(&pool, &video_info).await?;
    }
    Ok(())
}
