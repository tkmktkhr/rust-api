use crate::{entities::user, utils::string_util::string_util::check_string_return_string_or_none};
use serde::{Deserialize, Serialize};

// DTO<Input> validation should be here?
pub struct CreateUserInputData {
    pub id: u32,
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
