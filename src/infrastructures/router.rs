use crate::entities::user;
use crate::interfaces::controllers::users::get::GetUsersController;
use crate::interfaces::controllers::Controller;
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
    let user_controller = GetUsersController {
        name: String::from("GetUsers"),
    };
    user_controller.log();

    let (_id, name) = path.into_inner();

    let user = return_user(&name);

    println!("{:#?}", user);
    let full_name = user.full_name();
    // let full_name = user::UserStruct::full_name(&user.first_name);
    // println!("{:#?}", user::UserStruct::full_name(&user.first_name));
    web::Json(json!({ "full_name": full_name }))
}

#[post("/users")]
pub async fn create_user(body: web::Json<User>) -> impl Responder {
    let user_id = body.user_id;
    let name = &body.name;
    // TODO PostUsersController
    web::Json(json!({ "id": user_id + 1, "name": name }))
}

// sample function
fn return_user(name: &String) -> user::UserStruct {
    user::UserStruct::new(
        1,
        "abc".to_string(),
        name.to_owned(),
        Some("a@example.com".to_string()),
        // None,
    )
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

        let expected_user = user::UserStruct::new(
            1,
            "abc".to_string(),
            name.to_owned(),
            // "a@example.com".to_string(),
            None,
        );

        let user = return_user(&name);

        assert_eq!(user.id, expected_user.id);
        assert_eq!(user.first_name, expected_user.first_name);
        assert_eq!(user.last_name, expected_user.last_name);
        assert_eq!(user.email, expected_user.email);
    }
}
