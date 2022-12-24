use crate::{
    interfaces::controllers::users::get::GetUsersController,
    interfaces::{
        controllers::users::{
            get::GetUsersControllerTrait,
            post::{PostUsersController, PostUsersControllerTrait},
        },
        responses::responses::CustomError,
    },
    interfaces::{
        requests::users::create_user_request::CreateUserReq,
        responses::responses::{Res, ResponseStruct},
    },
    // use_cases::users::find_user::OutputData,
};
use actix_web::{
    get, http, post,
    web::{self, Json},
    HttpResponse, Responder,
};

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
// pub async fn get_user_by_id(path: web::Path<u32>) -> web::Json<Option<OutputData>> {
pub async fn get_user_by_id(path: web::Path<u32>) -> impl Responder {
    // Controller Pattern 1
    // NOTE Type annotation(: GetUsersController) is necessary in this case.
    let user_controller: GetUsersController =
        GetUsersControllerTrait::new(String::from("get_user_by_id"));

    // Controller Pattern 2
    // let user_controller = GetUsersController::new(String::from("get_user_by_id"));

    let id = path.into_inner();
    println!("{:?}", id);

    let output = user_controller.find_one_by_id(id);
    println!("{:?}", output);
    // let output_clone = output.clone();

    let not_found = CustomError {
        // code: http::StatusCode::NOT_FOUND,
        msg: "NOT FOUND".to_string(),
    };

    let res = match output {
        None => (
            web::Json(ResponseStruct {
                res: Res::CustomError(not_found),
            }),
            http::StatusCode::NOT_FOUND,
        ),
        Some(response) => (
            web::Json(ResponseStruct {
                res: Res::OutputData(Some(response)),
            }),
            http::StatusCode::OK,
        ),
    };

    res
}

#[get("/users/{id}/{name}")]
// pub async fn get_user(path: web::Path<(u32, String)>) -> web::Json<OutputData> {
pub async fn get_users(path: web::Path<(u32, String)>) -> impl Responder {
    // Type annotation(: GetUsersController) is necessary in this case.
    let user_controller: GetUsersController =
        GetUsersControllerTrait::new(String::from("GetUsers"));

    let (id, _name) = path.into_inner();

    let output = user_controller.find_one_by_id(id);

    web::Json(output)
}

#[post("/users")]
pub async fn create_user(body: web::Json<CreateUserReq>) -> impl Responder {
    let first_name = &body.first_name;
    let last_name = &body.last_name;
    let email = &body.email;

    let user_controller: PostUsersController =
        PostUsersControllerTrait::new(String::from("PostUsers"));

    let output = user_controller.create_user(first_name, last_name, email);

    let user = output.user;
    // TODO response code 200 with a created user object.
    (
        web::Json(ResponseStruct {
            res: Res::OutputData(Some(user)),
        }),
        http::StatusCode::CREATED,
    )
}
