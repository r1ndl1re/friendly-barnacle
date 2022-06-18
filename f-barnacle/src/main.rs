mod entity;

use std::net::SocketAddr;

use axum::extract::Query;
use axum::{extract::Extension, routing::get, Router};
use entity::prelude::*;
use entity::{tag, video};
use sea_orm::{
    ColumnTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, QueryFilter,
};

async fn read_video(db: &DatabaseConnection, video_id: &str) -> Option<video::Model> {
    let video = Video::find()
        .filter(video::Column::Code.eq(video_id))
        .one(db)
        .await
        .unwrap();
    video
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_video(
    Extension(db): Extension<DatabaseConnection>,
    video_id: Query<&str>,
) -> Option<video::Model> {
    let video = read_video(&db, &video_id).await;
    video
}

#[tokio::main]
async fn main() {
    let mut opt =
        ConnectOptions::new("postgres://app_user:hogehoge@localhost/defaultdb".to_owned());
    opt.max_connections(20)
        .min_connections(3)
        .sqlx_logging(true);

    let db = Database::connect(opt).await.unwrap();

    let video_id = "aaa";
    let video = read_video(&db, video_id).await;

    println!("{:?}", video);

    let app = Router::new()
        .route("/", get(root))
        .route("/video", get(get_video))
        .layer(Extension(db));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing_subscriber::fmt::init();
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
