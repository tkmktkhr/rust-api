use crate::infrastructures::dbs::mysql::connection;
use crate::infrastructures::models::user::User;
use crate::{
    entities::user::UserEntity,
    interfaces::controllers::users::get::GetUsersController,
    interfaces::controllers::users::{
        get::GetUsersControllerTrait,
        post::{PostUsersController, PostUsersControllerTrait},
    },
    interfaces::requests::users::create_user_request::CreateUserReq,
    use_cases::users::find_user::FindUserOutputData,
};
use actix_web::{get, post, web, HttpResponse, Responder};

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
    // Controller Pattern 1
    // NOTE Type annotation(: GetUsersController) is necessary in this case.
    let user_controller: GetUsersController =
        GetUsersControllerTrait::new(String::from("get_user_by_id"));

    // Controller Pattern 2
    // let user_controller = GetUsersController::new(String::from("get_user_by_id"));

    let id = path.into_inner();

    let output = user_controller.find_one_by_id(id);
    println!("{:?}", output);

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
pub async fn create_user(body: web::Json<CreateUserReq>) -> impl Responder {
    // use diesel::prelude::*;
    // use rust_api::schema::users::dsl::users;

    // // // REFACTOR connection should be taken as global object.
    // let pool = connection::get_connection_pool();
    // let connection = pool.get().unwrap();

    let user_id = body.user_id;
    let first_name = &body.first_name;
    let last_name = &body.last_name;
    let email = &body.email;

    let user_controller: PostUsersController =
        PostUsersControllerTrait::new(String::from("PostUsers"));

    let output = user_controller.create_user(user_id);

    // TODO PostUsersController
    let user1 = UserEntity {
        id: Some(user_id + 1),
        first_name: Some(first_name.to_string()),
        last_name: Some(last_name.to_string()),
        email: email.clone(),
    };
    let user2 = UserEntity {
        id: Some(user_id + 2),
        first_name: Some(String::from("first")),
        last_name: Some(String::from("last")),
        email: Some(String::from("sample@email.com")),
    };
    let users = [user1, user2];

    web::Json(users)
}
