use actix_web::{HttpResponse, ResponseError, http::StatusCode, web, Responder, Error as ActixError};
use actix_web::error::BlockingError;

use crate::models::{Bio, Project, Skill};

//use crate::schema::projects::dsl::*;
//use crate::schema::{bio::dsl::*, skills::dsl::*};
//use crate::schema::{bio, projects, skills};

use crate::DbPool;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use diesel::r2d2::PooledConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use serde::Serialize;

use std::fmt;

use actix_web::{get, post, put, delete, Error};
use diesel::result::Error as DieselError;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn get_projects(pool: web::Data<DbPool>) -> Result<HttpResponse, ActixError> {
    let mut conn = pool.get().expect("Failed to get db connection from pool");

    let projects = web::block(move || crate::db::get_all_projects(&mut conn)).await;

    match projects {
        Ok(Ok(projects)) => {
            Ok(HttpResponse::Ok().json(projects))
        }
        Ok(Err(e)) => {
            // Handle the case when there was an error in the inner Result
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
        Err(e) => {
            // Handle the case when there was an error in the outer Result
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn get_skills(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let skills = web::block(move || crate::db::get_all_skills(&mut conn)).await;

    match skills {
        Ok(Ok(skills)) => {
            Ok(HttpResponse::Ok().json(skills))
        }
        Ok(Err(e)) => {
            // Handle the case when there was an error in the inner Result
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
        Err(e) => {
            // Handle the case when there was an error in the outer Result
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn get_bio(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let bio = web::block(move || crate::db::get_all_bio(&mut conn)).await;

    match bio {
        Ok(Ok(bio)) => {
            Ok(HttpResponse::Ok().json(bio))
        }
        Ok(Err(e)) => {
            // Handle the case when there was an error in the inner Result
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
        Err(e) => {
            // Handle the case when there was an error in the outer Result
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/getProjects", web::get().to(get_projects))
            .route("/getSkills", web::get().to(get_skills))
            .route("/getBio", web::get().to(get_bio)),
    );
}
