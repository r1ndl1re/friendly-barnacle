mod crud;
mod ddl;
mod models;
mod v2013;
mod v2016;
mod v2021;

use glob::glob;
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;
use tokio::time::Instant;
use tokio_stream::{self, StreamExt};

use crate::crud::add_video;

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

    add_2021(&pool).await?;

    Ok(())
}

async fn add_2021(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let pattern = "./nicocomm/data.20211222/video/*.jsonl";

    for entry in glob(pattern).unwrap() {
        let start = Instant::now();
        let path = entry.unwrap();
        let video_infos = v2021::parse_video(&path);

        let mut stream = tokio_stream::iter(video_infos);
        while let Some(value) = stream.next().await {
            add_video(pool, &value).await?;
        }
        let duration = start.elapsed();
        println!("{}, time={}", &path.display(), duration.as_secs_f32());
    }
    Ok(())
}
