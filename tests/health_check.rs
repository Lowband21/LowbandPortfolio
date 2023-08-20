use actix_session::storage::RedisSessionStore;
use diesel::{Connection, PgConnection};
use lowband_portfolio::configuration::get_configuration;

use diesel::r2d2::{ConnectionManager, Pool};

use lowband_portfolio::DbPool;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn fractal_works() {
    // Arrange
    let address = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/fractal?x_value=0&y_value=0&zoom=1", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    /*
    let resp_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert!(resp_body.get("pixels").is_some());
    assert_eq!(resp_body["width"].as_u64().unwrap(), 800);
    assert_eq!(resp_body["height"].as_u64().unwrap(), 600);
    */
}

use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let redis_connection_string =
        std::env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1:6379"));
    let redis_store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server = lowband_portfolio::startup::run(listener, redis_store, pool)
        .expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
