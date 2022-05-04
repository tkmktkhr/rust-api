use actix_web::{App, HttpServer};

mod infrastructures;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  use infrastructures::router::router;
    
  HttpServer::new(|| {App::new()
    .service(router::index)
    .service(router::sample_get)})
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
