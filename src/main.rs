use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// mod infrastructures;

mod routers {
  use actix_web::{get, web, HttpResponse, Responder};

  #[get("/")]
  pub async fn index() -> impl Responder {
      HttpResponse::Ok().body("Hello world!")
      // web::Json(json!({ "temperature": 42.3 }))
  }
  
  
  #[get("/{id}/{name}")]
  pub async fn sample_get(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! id:{}", name, id))
  }
}

// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }


// #[get("/{id}/{name}")]
// async fn sample_get(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//   HttpResponse::Ok().body(format!("Hello {}! id:{}", name, id))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // use router;
    use routers;
    
    HttpServer::new(|| {App::new()
        .service(routers::sample_get)
        .service(routers::index)})
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
