use actix_session::storage::RedisSessionStore;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app_address = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:5000/health_check")
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
    let app_address = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:5000/fractal?x=0&y=0&zoom=1")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    let resp_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert!(resp_body.get("pixels").is_some());
    assert_eq!(resp_body["width"].as_u64().unwrap(), 800);
    assert_eq!(resp_body["height"].as_u64().unwrap(), 600);
}

use std::net::TcpListener;

async fn spawn_app() {
    let configuration = lowband_portfolio::configuration::get_configuration()
        .expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random address");
    let redis_connection_string =
        std::env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1:6379"));
    let redis_store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();
    let server = lowband_portfolio::startup::run(redis_store).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}
