mod entity;

use std::net::SocketAddr;

use axum::{
    extract::{Extension, Path},
    response::Json,
    routing::get,
    Router,
};
use entity::{
    prelude::*,
    video::{self, VideoToTag},
};
use sea_orm::{
    ColumnTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};
use serde::Serialize;
use tower::ServiceBuilder;

#[derive(Serialize)]
struct VideoResponse {
    video: video::Model,
    tags: Vec<String>,
}

async fn read_video(db: &DatabaseConnection, video_id: &str) -> VideoResponse {
    let video = Video::find()
        .filter(video::Column::Code.eq(video_id))
        .one(db)
        .await
        .expect("unable to query video information");
    let tags = match &video {
        Some(x) => x
            .find_linked(VideoToTag)
            .all(db)
            .await
            .expect("unable to query tag information"),
        None => Vec::new(),
    };
    VideoResponse {
        video: video.unwrap(),
        tags: tags.into_iter().map(|x| x.name).collect(),
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_video(
    Extension(db): Extension<DatabaseConnection>,
    Path(video_id): Path<String>,
) -> Json<VideoResponse> {
    let video = read_video(&db, &video_id).await;
    Json(video)
}

#[tokio::main]
async fn main() {
    let mut opt =
        ConnectOptions::new("postgres://app_user:hogehoge@localhost/defaultdb".to_owned());
    opt.max_connections(20)
        .min_connections(3)
        .sqlx_logging(true);

    let db = Database::connect(opt)
        .await
        .expect("Database connection failed");

    let app = Router::new()
        .route("/", get(root))
        .route("/video/:video_id", get(get_video))
        .layer(ServiceBuilder::new().layer(Extension(db)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing_subscriber::fmt::init();
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
