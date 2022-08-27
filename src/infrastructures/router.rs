use crate::{
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
pub async fn index(path: web::Path<(i32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    HttpResponse::Ok().body(format!("Hello {}! id:{}", name, id))
}

#[get("/users/{id}")]
pub async fn get_user_by_id(path: web::Path<i32>) -> web::Json<FindUserOutputData> {
    // Controller Pattern 1
    // NOTE Type annotation(: GetUsersController) is necessary in this case.
    let user_controller: GetUsersController =
        GetUsersControllerTrait::new(String::from("get_user_by_id"));

    // Controller Pattern 2
    // let user_controller = GetUsersController::new(String::from("get_user_by_id"));

    let id = path.into_inner();

    let output = user_controller.find_one_by_id(id);
    println!("{:?}", output);

    let res = match output {
        // TODO NOT Found
        None => web::Json(Some(output)),
        Some(res) => web::Json(Some(output)),
    };
    res
    // let full_name = output.user.full_name();
    // println!("{}", full_name);
    // web::Json(output)
}

#[get("/users/{id}/{name}")]
pub async fn get_user(path: web::Path<(i32, String)>) -> web::Json<FindUserOutputData> {
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
