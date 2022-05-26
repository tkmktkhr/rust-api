use crate::{
    interfaces::controllers::users::get::GetUsersController,
    interfaces::controllers::users::get::GetUsersControllerTrait,
    interfaces::requests::create_user_request::User,
    use_cases::users::find_user::FindUserOutputData,
};
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

#[get("/users/{id}")]
pub async fn get_user_by_id(path: web::Path<u32>) -> web::Json<FindUserOutputData> {
    // Type annotation(: GetUsersController) is necessary in this case.
    let user_controller: GetUsersController =
        GetUsersControllerTrait::new(String::from("get_user_by_id"));

    let id = path.into_inner();

    // pass input data to controller.
    let output = user_controller.find_one_by_id(id);

    let full_name = output.user.full_name();
    println!("{}", full_name);
    web::Json(output)
}

#[get("/users/{id}/{name}")]
pub async fn get_user(path: web::Path<(u32, String)>) -> web::Json<FindUserOutputData> {
    // Type annotation(: GetUsersController) is necessary in this case.
    let user_controller: GetUsersController =
        GetUsersControllerTrait::new(String::from("GetUsers"));

    let (id, _name) = path.into_inner();

    // pass input data to controller.
    let output = user_controller.find_one_by_id(id);

    let full_name = output.user.full_name();
    println!("{}", full_name);
    web::Json(output)
}

#[post("/users")]
pub async fn create_user(body: web::Json<User>) -> impl Responder {
    let user_id = body.user_id;
    let name = &body.name;
    // TODO PostUsersController
    web::Json(json!({ "id": user_id + 1, "name": name }))
}
