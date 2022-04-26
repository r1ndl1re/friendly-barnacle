mod entity;

use entity::{prelude::*, video};
use sea_orm::{ColumnTrait, ConnectOptions, Database, EntityTrait, QueryFilter};

#[tokio::main]
async fn main() {
    let mut opt =
        ConnectOptions::new("postgres://app_user:hogehoge@localhost/defaultdb".to_owned());
    opt.max_connections(20)
        .min_connections(3)
        .sqlx_logging(true);

    let db = Database::connect(opt).await.unwrap();

    let video = Video::find()
        .filter(video::Column::Code.eq("aaaa"))
        .one(&db)
        .await
        .unwrap();

    println!("{:?}", video);
}
