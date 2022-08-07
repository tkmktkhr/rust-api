use crate::entities::user;
use crate::infrastructures::dbs::mysql::connection;
use crate::infrastructures::models::user::User;
use serde::{Deserialize, Serialize};

// DTO<Input> validation should be here?
pub struct CreateUserInputData {
    pub id: u32, // TODO this should be eliminated because it is auto_increment value.
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
        let connection = pool.get().unwrap();

        let new_users = User {
            id: input.id,
            first_name: input.first_name,
            last_name: Some(input.last_name),
            email: Some(input.email),
        };
        let results = diesel::insert_into(users)
            .values(&new_users)
            .execute(&connection)
            .unwrap();

        // TODO Application Logic
        let user = create_user(input.id);
        let output = CreateUserOutputData { user };
        output
    }
}

fn create_user(id: u32) -> user::UserEntity {
    // TODO db access.
    user::UserEntity::new(
        id,
        "abc".to_string(),
        " def".to_owned(),
        Some("a@example.com".to_string()),
        // None,
    )
}
