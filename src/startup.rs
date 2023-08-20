use crate::routes::chat::*;
use crate::routes::pathfinding::*;
use crate::routes::rsa::*;
use crate::routes::twentyfortyeight::*;

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use std::net::TcpListener;
use std::time::Duration;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn run(listener: TcpListener, redis_store: RedisSessionStore) -> std::io::Result<Server> {
    println!("Here");
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let secret_key = Key::generate();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(pool.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/generate_keys", web::get().to(generate_rsa_keys))
            .route("/bfs", web::post().to(bfs_route))
            .route("/dijkstra", web::post().to(dijkstra_route))
            .route("/a-star", web::post().to(a_star_route))
            .service(
                web::scope("/api")
                    .route("/getProjects", web::get().to(get_projects))
                    .route("/getSkills", web::get().to(get_skills))
                    .route("/getBio", web::get().to(get_bio))
                    .route("/chat", web::post().to(chat)),
            )
            .service(
                web::scope("/game")
                    .route("/new", web::post().to(new_game))
                    .route("/move", web::post().to(make_move))
                    .route("/undo", web::post().to(undo))
                    .route("/reset", web::post().to(reset)),
            )
            .service(actix_files::Files::new("/", "./client/public").index_file("index.html"))
            .default_service(web::route().to(move |req: HttpRequest| {
                let path = req.path().to_owned();
                async move {
                    if path.starts_with("/api") || path.starts_with("/game") {
                        HttpResponse::NotFound().finish()
                    } else {
                        match actix_files::NamedFile::open("./client/public/index.html") {
                            Ok(file) => file.into_response(&req),
                            Err(_) => HttpResponse::NotFound().finish(),
                        }
                    }
                }
            }))
    })
    .listen(listener)?
    .client_request_timeout(Duration::new(5 * 60, 0))
    .run();
    Ok(server)
}
