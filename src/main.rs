use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[get("/{id}/{name}")]
async fn sample_get(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
  HttpResponse::Ok().body(format!("Hello {}! id:{}", name, id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {App::new().service(sample_get).service(index)})
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
