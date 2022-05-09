extern crate dotenv;

use dotenv::dotenv;
use std::env;
use actix_web::{App, HttpServer};

mod entities;
mod infrastructures;
mod interfaces;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    use infrastructures::router;

    HttpServer::new(|| {
        App::new()
            .service(router::index)
            .service(router::sample_get)
            .service(router::sample_post)
    })
    .bind(env::var("ADDRESS").unwrap())?
    .run()
    .await
}
