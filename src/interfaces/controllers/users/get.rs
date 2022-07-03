use crate::{
    infrastructures::models::user::User, interfaces::controllers::called_log,
    use_cases::users::find_user, use_cases::users::find_user::FindUserOutputData,
};

// kind of class without func
pub struct GetUsersController {
    pub name: String,
}

// Traits are similar to a feature often called interfaces in other languages, although with some differences.
pub trait GetUsersControllerTrait {
    fn new(name: String) -> Self;
    fn find_one_by_id(&self, id: u32) -> FindUserOutputData; // remove here but it will be error in Controller because there is no abstract concept in Rust.
}

// methods impl in struct
impl GetUsersControllerTrait for GetUsersController {
    fn new(name: String) -> Self {
        called_log(&name);

        Self { name }
    }

    fn find_one_by_id(&self, id: u32) -> FindUserOutputData {
        // use self::schema::users::dsl::*;
        use rust_api::schema::users::dsl::users;

        // ここから消す-------------------------------
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        // use diesel::mysql::MysqlConnection;
        use diesel::prelude::*;

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        // ここまで消す-------------------------------

        // NOTE Clean Architecture Sample.
        // var inputData = new UserCreateInputData(userName);
        // userCreateUseCase.Handle(inputData);

        let input_data = find_user::FindUserInputData { id };
        let output_data = find_user::FindUserInteractor::get_user_by_id(input_data);
        let results = users.load::<User>(&connection);
        // let results = users
        //     .limit(5)
        //     .load::<User>(&connection)
        //     .expect("Error loading posts");
        println!("------------------------\n");
        println!("Displaying {:?} users", &results);
        // for user in results {
        //     println!("{}", user.id);
        //     println!("----------\n");
        //     println!("{}", user.body);
        //     println!("{}", user.first_name);
        //     println!("{}", user.last_name);
        //     println!("{}", user.email);
        // }
        println!("------------------------\n");
        // if there is no user, return Not found.
        // let output_data = match output_data {
        //     Ok(user) => user,
        //     Err(error) => {
        //         // Error
        //         panic!("There was a problem opening the file: {:?}", error)
        //     }
        // };
        output_data
    }
}
