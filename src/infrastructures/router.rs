use crate::interfaces::controllers::Controller;
use crate::interfaces::requests::create_user_request::User;
use crate::{
    interfaces::controllers::users::get::GetUsersController,
    use_cases::users::find_user::FindUserOutputData,
};
use actix_web::{get, post, web, HttpResponse, Responder, Result};
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
pub async fn get_user(path: web::Path<(u32, String)>) -> web::Json<FindUserOutputData> {
    // pub async fn get_user(path: web::Path<(u32, String)>) -> Result<Json<FindUserOutputData>> {
    let user_controller = GetUsersController::new(String::from("GetUsers"));
    user_controller.log();

    let (id, _name) = path.into_inner();

    // pass input data to controller.
    let user = GetUsersController::find_one_by_id(id);
    // let user = user_controller.find_one_by_id(id);

    // let user = return_user(&name);

    // println!("{:#?}", user);
    // let full_name = user.full_name();
    // let full_name = user::UserEntity::full_name(&user.first_name);
    // println!("{:#?}", user::UserEntity::full_name(&user.first_name));
    // Ok(web::Json(user))
    web::Json(user)
}

#[post("/users")]
pub async fn create_user(body: web::Json<User>) -> impl Responder {
    let user_id = body.user_id;
    let name = &body.name;
    // TODO PostUsersController
    web::Json(json!({ "id": user_id + 1, "name": name }))
}

// // sample function
// fn return_user(name: &String) -> user::UserEntity {
//     user::UserEntity::new(
//         1,
//         "abc".to_string(),
//         name.to_owned(),
//         Some("a@example.com".to_string()),
//         // None,
//     )
// }
