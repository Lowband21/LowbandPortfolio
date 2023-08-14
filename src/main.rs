//! main.rs
use actix_session::storage::RedisSessionStore;
use lowband_portfolio::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let redis_connection_string =
        std::env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1:6379"));
    let redis_store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();
    run(redis_store)?.await
}
