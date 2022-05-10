use crate::entities::user;
use crate::interfaces::requests::create_user_request::User;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn index() -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    let user = user::User {
        id: Some(1),
        first_name: "abc".to_string(),
        last_name: "def".to_string(),
        email: "a@example.com".to_string(),
    };
    println!("{:#?}", user);
    let full_name = user::User::full_name(&user);
    println!("{:#?}", user::User::full_name(&user));
    web::Json(json!({ "full_name": full_name }))
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
