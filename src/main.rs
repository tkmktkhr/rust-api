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
