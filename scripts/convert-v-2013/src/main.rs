use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let service = get(|| async { "Hello, world!" });
    let app = Router::new().route("/", service);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
