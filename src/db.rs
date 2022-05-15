// #[macro_use]
// extern crate lazy_static;
//
// use crate::errors::ApiError;
//
// use r2d2;
// use std::env;
// use actix_web::connect;
// use diesel::PgConnection;
// use diesel::r2d2::ConnectionManager;
// use diesel::test_helpers::pg_connection;
// use lazy_static::lazy_static;
// use diesel_migrations::embed_migrations;
//
// type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
//
// lazy_static! {
//     static ref POOL: Pool = {
//         let db_url = env::var("DATABASE_URL").expect("Database url environment variable undefined");
//         let manager = ConnectionManager::<PgConnection>::new(db_url);
//         Pool::new(manager).expect("Failed to create a db pool");
//     }
// }
//
// pub fn get_connection() -> DbConnection {
//     POOL.get().map_err(|error| ApiError::new(500, format!("Failed to get a db connection with error: {}", error)))?
// }
//
// pub fn init() {
//     info!("initializing db");
//     lazy_static::initialize(&Pool);
// }

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;
use crate::errors::ApiError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        // info!(format!("{}", env::var("DATABASE_URL").as_str()));
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);

    // let t = env::var("DATABASE_URL").as_str();
}

pub fn get_connection() -> Result<DbConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}
