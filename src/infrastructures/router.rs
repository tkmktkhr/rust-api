pub mod router {
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