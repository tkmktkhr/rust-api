use crate::{
    interfaces::controllers::users::get::GetUsersController,
    interfaces::controllers::users::{
        get::GetUsersControllerTrait,
        post::{PostUsersController, PostUsersControllerTrait},
    },
    interfaces::requests::users::create_user_request::CreateUserReq,
    use_cases::users::find_user::FindUserOutputData,
};
use actix_web::{
    get, http, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use serde::Serialize;

#[get("/healthCheck")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/")]
pub async fn index(path: web::Path<(i32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    HttpResponse::Ok().body(format!("Hello {}! id:{}", name, id))
}

#[derive(Debug, Serialize)]
struct NotFoundError {
    msg: String,
}

fn check_obj_or_none(
    original: Option<FindUserOutputData>,
) -> Option<Json<Option<FindUserOutputData>>> {
    match original {
        None => None,
        Some(i) => Some(web::Json(Some(i))),
    }
}

pub enum Res {
    FindUserOutputData,
    NotFoundError,
}

#[derive(Debug, Serialize)]
pub struct ResponseStruct {
    res: Res,
}

#[get("/users/{id}")]
// pub async fn get_user_by_id(path: web::Path<i32>) -> web::Json<Option<FindUserOutputData>> {
pub async fn get_user_by_id(path: web::Path<i32>) -> impl Responder {
    // Controller Pattern 1
    // NOTE Type annotation(: GetUsersController) is necessary in this case.
    let user_controller: GetUsersController =
        GetUsersControllerTrait::new(String::from("get_user_by_id"));

    // Controller Pattern 2
    // let user_controller = GetUsersController::new(String::from("get_user_by_id"));

    let id = path.into_inner();

    let output = user_controller.find_one_by_id(id);
    println!("{:?}", output);
    // let output_clone = output.clone();

    let obj = NotFoundError {
        msg: "NOT FOUND".to_string(),
    };

    let res = match output {
        // TODO NOT Found
        // None => web::Json(obj),
        None => (
            web::Json(ResponseStruct { res: obj }),
            http::StatusCode::CREATED,
        ),
        Some(response) => (
            web::Json(ResponseStruct { res: response }),
            http::StatusCode::CREATED,
        ),
    };

    // NOTE This works.
    // let res = match output {
    //     // TODO NOT Found
    //     // None => web::Json(obj),
    //     None => web::Json(output),
    //     Some(res) => web::Json(Some(res)),
    // };

    // let res = if output.is_none() {
    //     let obj = NotFoundError {
    //         msg: "NOT FOUND".to_string(),
    //     };
    //     let e_res = web::Json(obj);
    //     return (e_res, http::StatusCode::NOT_FOUND);
    // } else {
    //     let ok_res = web::Json(output);
    //     return (ok_res, http::StatusCode::CREATED);
    // };

    res
    // NOTE returning with indicated status code.
    // (res, http::StatusCode::CREATED)
    // (web::Json(Some(output)), http::StatusCode::CREATED)

    // let full_name = output.user.full_name();
    // web::Json(output)
}

#[get("/users/{id}/{name}")]
// pub async fn get_user(path: web::Path<(i32, String)>) -> web::Json<FindUserOutputData> {
pub async fn get_user(path: web::Path<(i32, String)>) -> impl Responder {
    // Type annotation(: GetUsersController) is necessary in this case.
    let user_controller: GetUsersController =
        GetUsersControllerTrait::new(String::from("GetUsers"));

    let (id, _name) = path.into_inner();

    let output = user_controller.find_one_by_id(id);

    // let full_name = output.user.full_name();
    // println!("{}", full_name);
    web::Json(output)
}

#[post("/users")]
pub async fn create_user(body: web::Json<CreateUserReq>) -> impl Responder {
    let first_name = &body.first_name;
    let last_name = &body.last_name;
    let email = &body.email;
    println!("{:?}", email);

    let user_controller: PostUsersController =
        PostUsersControllerTrait::new(String::from("PostUsers"));

    let output = user_controller.create_user(first_name, last_name, email);

    let user = output.user;
    // TODO response code 201 or 200.
    web::Json(user)
}
