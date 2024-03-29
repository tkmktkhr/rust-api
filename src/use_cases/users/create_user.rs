use crate::entities::user;
use crate::infrastructures::dbs::mysql::connection;
use crate::infrastructures::models::user::NewUser;
use serde::Serialize;

// DTO<Input> validation should be here?
pub struct CreateUserInputData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// TODO Think about Serializer here. It seems to oppose to Clean Architecture.
// DTO<Output>
#[derive(Debug, Serialize)]
pub struct CreateUserOutputData {
    pub user: user::UserEntity,
}

// Use Case implementation
pub struct CreateUserInteractor {}

impl CreateUserInteractor {
    pub fn create(input: CreateUserInputData) -> CreateUserOutputData {
        use diesel::prelude::*;
        use rust_api::schema::users::dsl::users;

        // // REFACTOR connection should be taken as global object.
        let pool = connection::get_connection_pool();
        let mut connection = pool.get().unwrap();

        let new_users = NewUser {
            first_name: input.first_name,
            last_name: Some(input.last_name),
            email: input.email,
        };
        // NOTE diesel get_result method does not support MYSQL.
        // REFACTOR inert only one user.
        let results = diesel::insert_into(users)
            .values(&new_users)
            .execute(&mut connection);
        // TODO handle error pattern.
        let value = results.unwrap_or(0); // return value is the number of registered users.

        // TODO get created user. if possible.
        let user = create_user(123);
        let output = CreateUserOutputData { user };
        output
    }
}

fn create_user(id: u32) -> user::UserEntity {
    user::UserEntity::new(
        id,
        "abc".to_string(),
        Some(" def".to_owned()),
        "a@example.com".to_string(),
    )
}
