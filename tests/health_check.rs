use actix_session::storage::RedisSessionStore;
use redis::Commands;
use std::process::Command;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
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
async fn get_skills_works() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/get_skills", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    // TODO: add more specific assertions about the content if needed
}

#[tokio::test]
async fn get_bio_works() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/get_bio", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    // TODO: add more specific assertions about the content if needed
}

#[tokio::test]
async fn chat_works() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Manually create the JSON string (similar to what your client does)
    let request_data_json = r#"
    {
        "messages": [
            { 
                "text": "Hello, OpenAI!", 
                "user": true 
            }
        ]
    }
    "#;

    // Act
    let response = client
        .get(&format!("{}/chat", &address))
        .header("Content-Type", "application/json")
        .body(request_data_json)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    // TODO: add more specific assertions about the chat response if needed
}

use std::process::{Child, Stdio};
struct TempRedisServer {
    process: Child,
}

impl Drop for TempRedisServer {
    fn drop(&mut self) {
        // Kill the Redis process when the TempRedisServer instance goes out of scope
        let _ = self.process.kill();
    }
}

use std::net::TcpListener;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

type DbPool = Pool<ConnectionManager<PgConnection>>;

async fn spawn_app() -> String {
    // Start a Redis server on a random port
    let redis_port = get_random_port();
    // Build the Redis connection string
    let redis_connection_string = format!("redis://127.0.0.1:{}", redis_port);
    // Start Redis server
    let _redis_server = TempRedisServer {
        process: Command::new("redis-server")
            .arg("--port")
            .arg(redis_port.to_string())
            .stdout(Stdio::null()) // Redirect stdout
            .spawn()
            .expect("Failed to start Redis server"),
    };

    // Wait a little for the Redis server to start
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let redis_store = RedisSessionStore::new(&redis_connection_string)
        .await
        .unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let db_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let db_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let connection_string = format!("host={} user={}", db_host, db_user);
    // Connect to the default PostgreSQL database
    let (client, connection) = tokio_postgres::connect(&connection_string, tokio_postgres::NoTls)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Spawn connection task to handle background tasks (like keeping the connection alive)
    let connection_handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Create a temporary database
    let temp_db_name = format!("temp_test_db_{}", get_random_string(10)); // Implement `get_random_string` as you see fit
    client
        .execute(format!("CREATE DATABASE {}", temp_db_name).as_str(), &[])
        .await
        .expect("Failed to create temp database");

    // Now, use this database for your tests
    let database_url = format!("postgresql://localhost/{}", temp_db_name);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server =
        lowband_portfolio::run(redis_store, listener, pool).expect("Failed to bind address");

    // Launch the server as a background task
    let _ = tokio::spawn(server);

    // After the tests, you'd want to kill the Redis server
    // redis_server.kill().expect("Failed to kill Redis server");

    format!("http://127.0.0.1:{}", port)
}

fn get_random_port() -> u16 {
    // There are different ways to get a random port
    // This is just a basic example, and might have race conditions
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn get_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
