use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod routes;
mod schema;

use crate::routes::*;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Here");
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init_routes)
            .service(actix_files::Files::new("/", "./client/public").index_file("index.html"))
            .service(
                web::scope("/api")
                    .route("/getProjects", web::get().to(get_projects))
                    .route("/getSkills", web::get().to(get_skills))
                    .route("/getBio", web::get().to(get_bio))
                    .route("/chat", web::post().to(chat)),
            )
            .default_service(web::route().to(move |req: HttpRequest| {
                let path = req.path().to_owned();
                async move {
                    if path.starts_with("/api") {
                        HttpResponse::NotFound().finish()
                    } else {
                        match actix_files::NamedFile::open("./client/public/index.html") {
                            Ok(file) => file.into_response(&req),
                            Err(_) => HttpResponse::InternalServerError().finish(),
                        }
                    }
                }
            }))
    })
    .bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or_else(|_| "5000".to_string())
    ))?
    .run()
    .await
}
