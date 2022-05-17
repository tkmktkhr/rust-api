use crate::entities::user;

// DTO<Input>
pub struct FindUserInputData {
    pub id: u32,
}
// DTO<Output>
pub struct FindUserOutputData {
    pub user: user::UserEntity,
}

// Use Case implementation
pub struct FindUserInteractor {}

impl FindUserInteractor {
    pub fn get_user_by_id(input: FindUserInputData) -> FindUserOutputData {
        // TODO Application Logic
        let user = return_user(input.id);
        let output = FindUserOutputData { user };
        output
    }
}

// sample function
fn return_user(id: u32) -> user::UserEntity {
    user::UserEntity::new(
        id,
        "abc".to_string(),
        " def".to_owned(),
        Some("a@example.com".to_string()),
        // None,
    )
}

// For View
struct FindUserPresenter {
    pub id: u32,
}
