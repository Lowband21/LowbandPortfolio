use actix_web::{web, App, HttpServer, HttpResponse};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;
use actix_web::middleware::Logger;

mod models;
mod routes;
mod schema;
mod db;

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
                    .route("/getBio", web::get().to(get_bio)),
                    )
    })
    .bind(format!("0.0.0.0:{}", env::var("PORT").unwrap_or_else(|_| "5000".to_string())))?
    .run()
    .await
}
