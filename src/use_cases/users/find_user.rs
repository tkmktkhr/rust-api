use crate::entities::user::UserEntity;
use serde::{Deserialize, Serialize};

// DTO<Input> validation should be here?
pub struct FindUserInputData {
    pub id: u32,
}

// CHECK Think about Serializer here. It seems to oppose to Clean Architecture.
// DTO<Output>
#[derive(Debug, Serialize)]
pub struct FindUserOutputData {
    pub user: UserEntity,
}

// Use Case implementation
pub struct FindUserInteractor {}

impl FindUserInteractor {
    pub fn get_user_by_id(input: FindUserInputData) -> FindUserOutputData {
        // NOTE Application Logic is here

        // FIXME Dependency Inversion principle.
        let user = get_user(input.id);
        let output = FindUserOutputData { user };
        return output;
    }

    // pub fn get_users(input: FindUserInputData) -> FindUserOutputData {}
}

// sample function
fn get_user(id: u32) -> UserEntity {
    UserEntity::new(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::user::UserEntity;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    #[actix_rt::test]
    async fn test_get_user() {
        let name = String::from(" def");
        let id = 1;

        let expected_user = UserEntity::new(
            1,
            "abc".to_string(),
            name.to_owned(),
            Some("a@example.com".to_string()),
            // None,
        );

        let user = get_user(id);

        assert_eq!(user.id, expected_user.id);
        assert_eq!(user.first_name, expected_user.first_name);
        assert_eq!(user.last_name, expected_user.last_name);
        assert_eq!(user.email, expected_user.email);
    }
}
