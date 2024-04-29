//! main.rs
use actix_session::storage::RedisSessionStore;
use dotenv::dotenv;

use lowband_portfolio::startup::run;
use sqlx::PgPool;
use std::env;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    //let configuration = get_configuration().expect("Failed to read configuration.");
    let port = env::var("PORT").unwrap_or("5005".to_string());

    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(address)?;

    let redis_connection_string =
        std::env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1:6379"));
    let redis_store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    //let connection_string = configuration.database.connection_string();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let connection = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, redis_store, connection)?.await
}
