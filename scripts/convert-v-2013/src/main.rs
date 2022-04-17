mod ddl;
mod models;
mod v2013;
mod v2016;
mod v2021;

const DATABASE_URL: &str = "postgres://app_user:hogehoge@localhost:5432/defaultdb";

fn main() {
    v2021::create().unwrap();
}
