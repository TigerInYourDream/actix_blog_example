// hide diesel derive warning
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

pub struct DbConnecting;

impl DbConnecting {
    pub fn establish_connection() -> diesel::pg::PgConnection {
        dotenv().ok();
//        let s = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_url = "postgres://zhaoyue:000000@127.0.0.1/actix_web_dev";
        diesel::pg::PgConnection::establish(&db_url).expect("Failed to create connection.")
    }
    pub fn establish_pool() -> diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>> {
        dotenv().ok();
//        let s = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_url = "postgres://zhaoyue:000000@127.0.0.1/actix_web_dev";
        let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(db_url);
        let pool = diesel::r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    }
}

pub type DbConn = diesel::pg::PgConnection;
pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

pub mod models;
pub mod schema;
