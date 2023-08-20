pub mod configuration;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
pub mod startup;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

type DbPool = Pool<ConnectionManager<PgConnection>>;
