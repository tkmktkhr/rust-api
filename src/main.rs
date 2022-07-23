extern crate dotenv;

mod entities;
mod infrastructures;
mod interfaces;
mod use_cases;
mod utils;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

// -------------

// use diesel::r2d2::diesel_connection::{get_connection, PoolError};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .max_size(3)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

// fn get_connection() -> Result<(), PoolError> {
//     // DATABASE_URL can be set any time before the pool is lazily initialized on first use
//     dotenv().expect("Unable to load .env file");
//     env_logger::init();

//     let conn = get_connection()?;
// }

// dotenv().ok();
// use diesel::mysql::MysqlConnection;
// use diesel::prelude::*;
// use infrastructures::router;

// let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

// let mysql_connection = MysqlConnection::establish(&database_url);

// pub struct User {
//     pub id: u32,
//     pub first_name: String,
//     pub last_name: Option<String>,
//     pub email: Option<String>,
// }

// mod mysql_connection {
//     fn connection() {
//         dotenv().ok();
//         use diesel::mysql::MysqlConnection;
//         use diesel::prelude::*;
//         use infrastructures::router;

//         // REFACTOR Connection pool.
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         MysqlConnection::establish(&database_url)
//             .expect(&format!("Error connecting to {}", database_url));
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    use diesel::mysql::MysqlConnection;
    use diesel::prelude::*;
    use infrastructures::router;

    // REFACTOR Connection pool.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    HttpServer::new(|| {
        App::new()
            .service(router::health_check)
            .service(router::index)
            .service(router::get_user_by_id)
            .service(router::get_user)
            .service(router::create_user)
    })
    .bind(env::var("ADDRESS").unwrap())?
    .run()
    .await
}
