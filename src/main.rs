extern crate dotenv;

mod entities;
mod infrastructures;
mod interfaces;
mod use_cases;
mod utils;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    use infrastructures::router;

    // FROM here - previous db connection impl.
    // use diesel::mysql::MysqlConnection;
    // use diesel::prelude::*;
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // MysqlConnection::establish(&database_url)
    //     .expect(&format!("Error connecting to {}", database_url));
    // UNTIL here - previous db connection impl.

    // REFACTOR make Connection pool global object.
    infrastructures::dbs::mysql::connection::get_connection_pool();

    HttpServer::new(|| {
        App::new()
            .service(router::health_check)
            .service(router::index)
            .service(router::get_user_by_id)
            .service(router::get_users)
            .service(router::create_user)
    })
    .bind(env::var("ADDRESS").unwrap())?
    .run()
    .await
}
