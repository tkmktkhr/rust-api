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

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .max_size(3)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

// let mysql_pool = get_connection_pool();

// pub struct DbPool {
//     pub mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
// }

// impl DbPool {
//     fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let manager = ConnectionManager::<MysqlConnection>::new(database_url);

//         Pool::builder()
//             .max_size(3)
//             .test_on_check_out(true)
//             .build(manager)
//             .expect("Could not build connection pool")
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // use diesel::mysql::MysqlConnection;
    // use diesel::prelude::*;
    use infrastructures::router;

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // MysqlConnection::establish(&database_url)
    //     .expect(&format!("Error connecting to {}", database_url));

    // REFACTOR make Connection pool global object.
    get_connection_pool();

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
