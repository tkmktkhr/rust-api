use crate::interfaces::requests::sample_post::User;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn index() -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    web::Json(json!({ "height": 174.3 }))
}

#[get("/{id}/{name}")]
pub async fn sample_get(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    HttpResponse::Ok().body(format!("Hello {}! id:{}", name, id))
}

#[post("/users")]
pub async fn sample_post(body: web::Json<User>) -> impl Responder {
    let user_id = body.user_id;
    let name = &body.name;
    web::Json(json!({ "id": user_id + 1, "name": name }))
}
