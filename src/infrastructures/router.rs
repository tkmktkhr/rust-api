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
    // TODO GetUsersController
    let (_id, name) = path.into_inner();

    let user = return_user(&name);

    println!("{:#?}", user);
    let full_name = user::User::full_name(&user);
    println!("{:#?}", user::User::full_name(&user));
    web::Json(json!({ "full_name": full_name }))
}

#[post("/users")]
pub async fn create_user(body: web::Json<User>) -> impl Responder {
    let user_id = body.user_id;
    let name = &body.name;
    // TODO GetUsersController
    web::Json(json!({ "id": user_id + 1, "name": name }))
}

// sample function
fn return_user(name: &String) -> user::User {
    user::User {
        id: Some(1),
        first_name: "abc".to_string(),
        last_name: name.to_owned(),
        email: "a@example.com".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::user;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    #[actix_rt::test]
    async fn test_return_user() {
        let name = String::from("rust");

        let expected_user = user::User {
            id: Some(1),
            first_name: "abc".to_string(),
            last_name: "rust".to_string(),
            email: "a@example.com".to_string(),
        };

        let user = return_user(&name);

        assert_eq!(user.id, expected_user.id);
        assert_eq!(user.first_name, expected_user.first_name);
        assert_eq!(user.last_name, expected_user.last_name);
        assert_eq!(user.email, expected_user.email);
    }
}
