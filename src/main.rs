//! main.rs
use actix_session::storage::RedisSessionStore;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use std::net::TcpListener;

use lowband_portfolio::run;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let redis_connection_string =
        std::env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1:6379"));
    let redis_store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind random port");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    run(redis_store, listener, pool)?.await
}
