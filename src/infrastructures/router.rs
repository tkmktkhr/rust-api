use crate::entities::user;
use crate::interfaces::requests::create_user_request::User;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/healthCheck")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/")]
pub async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    HttpResponse::Ok().body(format!("Hello {}! id:{}", name, id))
}

#[get("/users/{id}/{name}")]
pub async fn get_user(path: web::Path<(u32, String)>) -> impl Responder {
    // TODO getUserController
    let (_id, name) = path.into_inner();

    let user = user::User {
        id: Some(1),
        first_name: "abc".to_string(),
        last_name: name,
        email: "a@example.com".to_string(),
    };
    println!("{:#?}", user);
    let full_name = user::User::full_name(&user);
    println!("{:#?}", user::User::full_name(&user));
    web::Json(json!({ "full_name": full_name }))
}

#[post("/users")]
pub async fn create_user(body: web::Json<User>) -> impl Responder {
    let user_id = body.user_id;
    let name = &body.name;
    // TODO createUserController
    web::Json(json!({ "id": user_id + 1, "name": name }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
