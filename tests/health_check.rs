use actix_session::storage::RedisSessionStore;
use lowband_portfolio::configuration::get_configuration;
use num_bigint::BigUint;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
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
    let app = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!(
            "{}/fractal?x_value=0&y_value=0&zoom=1",
            &app.address
        ))
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

#[tokio::test]
async fn generate_rsa_keys_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/generate_keys", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let resp_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body.");

    assert!(resp_body.get("prime1").is_some());
    assert!(resp_body.get("prime2").is_some());
    assert!(resp_body.get("oddNumbersTried").is_some());
    assert!(resp_body.get("valuesUsed").is_some());
    assert!(resp_body.get("confidence").is_some());

    // Extract the primes from the response
    let _prime1: BigUint =
        BigUint::parse_bytes(resp_body["prime1"].as_str().unwrap().as_bytes(), 10)
            .expect("Failed to parse prime1");
    let _prime2: BigUint =
        BigUint::parse_bytes(resp_body["prime2"].as_str().unwrap().as_bytes(), 10)
            .expect("Failed to parse prime2");
}

#[tokio::test]
async fn test_new_game() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/game/new", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);

    let resp_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body.");

    assert_eq!(resp_body["score"], 0);
    // Additional assertions about the board state if necessary.
}

#[tokio::test]
async fn test_make_move() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // We should ideally start with creating a new game here before making a move.

    // Act
    use lowband_portfolio::routes::game::oxydized2048::Action;
    use lowband_portfolio::routes::MoveInfo;
    let move_info = MoveInfo {
        action: Action::Left, // For instance
    };
    let response = client
        .post(&format!("{}/game/new", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    let response = client
        .post(&format!("{}/game/move", &app.address))
        .json(&move_info)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    let status = response.status();
    assert!(
        status == 200 || status == 400,
        "Unexpected response status: {}",
        status
    );
    // Additional assertions about the board state and score after making the move.

    // We should also test for invalid moves.
}

#[tokio::test]
async fn test_undo() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Ideally, create a game, make a move, and then undo that move.

    // Act
    let response = client
        .post(&format!("{}/game/undo", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);
    // Additional assertions about the board state after undoing.
}

#[tokio::test]
async fn test_reset() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Ideally, create a game, make a move, and then reset the game.

    // Act
    let response = client
        .post(&format!("{}/game/reset", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);
    // Additional assertions about the board state after resetting.
}

async fn spawn_app() -> TestApp {
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
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let server = lowband_portfolio::startup::run(listener, redis_store, connection_pool.clone())
        .expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
