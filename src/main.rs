use actix_web::{App, HttpServer};

mod infrastructures;
mod interfaces;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use infrastructures::router;

    HttpServer::new(|| {
        App::new()
            .service(router::index)
            .service(router::sample_get)
            .service(router::sample_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
