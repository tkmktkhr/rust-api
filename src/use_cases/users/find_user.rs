use crate::entities::user::UserEntity;
use crate::infrastructures::dbs::mysql::connection;
use crate::infrastructures::models::user::User;
use diesel::result::Error;
use diesel::sql_query;
use diesel::sql_types::{Integer, Unsigned};
use rust_api::schema::users;
use serde::Serialize;

// DTO<Input> validation should be here?
pub struct FindUserInputData {
    pub id: u32,
}

pub struct FindUserAfterUserCreationInputData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
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
    pub fn get_user_by_id(input: FindUserInputData) -> Option<FindUserOutputData> {
        use diesel::prelude::*;

        // REFACTOR connection should be taken as global object.
        let pool = connection::get_connection_pool();
        let mut connection = pool.get().unwrap();

        // NOTE Application Logic is here

        // FIXME Dependency Inversion principle.
        let user_vec = sql_query(
            "
            SELECT
                id,
                first_name,
                last_name,
                email
            FROM
                users
            WHERE
                id = ?
            ",
        )
        .bind::<Unsigned<Integer>, _>(input.id)
        .load::<User>(&mut connection);
        // .get_results::<User>(&mut connection); // This is also fine.

        // NOTE response type is unknown.
        // let user_vec = users.find(input.id).get_results::<User>(&mut connection);

        let found_user = match user_vec {
            Ok(vec) => vec,
            Err(e) => panic!("Problem creating the file: {:?}", e), // TODO ERROR PROCESS
        };

        if found_user.is_empty() {
            return None;
        }

        // TODO user should be only one.
        let user_entity_output = &found_user[0];

        let user_output = UserEntity {
            id: Some(user_entity_output.id),
            first_name: Some(user_entity_output.first_name.clone()),
            last_name: user_entity_output.last_name.clone(),
            email: user_entity_output.email.clone(),
        };
        let output = FindUserOutputData { user: user_output };
        return Some(output);
    }

    // TODO TRY most of codes are same as get_user_by_id.
    // pub fn get_user_after_user_creation(
    //     input: FindUserAfterUserCreationInputData,
    // ) -> Option<FindUserOutputData> {
    //     // TODO return type is Result? it should return None for Not found.
    //     // pub fn get_user_by_id(input: FindUserAfterUserCreationInputData) -> Option<FindUserOutputData> {
    //     use diesel::prelude::*;

    //     // REFACTOR connection should be taken as global object.
    //     let pool = connection::get_connection_pool();
    //     let connection = pool.get().unwrap();

    //     // NOTE Application Logic is here

    //     // FIXME Dependency Inversion principle.
    //     let result: Result<Vec<User>, Error> = sql_query(
    //         "
    //       SELECT
    //           id,
    //           first_name,
    //           last_name,
    //           email
    //       FROM
    //           users
    //       WHERE
    //       first_name = ?,
    //       last_name = ?,
    //       email = ?
    //       ",
    //     )
    //     .bind::<String, _>(input.first_name)
    //     .bind::<String, _>(input.last_name)
    //     .bind::<String, _>(input.email)
    //     .load::<User>(&connection);
    //     print!("{:?}", result);

    //     let found_user = match result {
    //         Ok(vec) => vec,
    //         Err(e) => panic!("Problem creating the file: {:?}", e), // TODO ERROR PROCESS
    //     };

    //     if found_user.is_empty() {
    //         return None;
    //     }

    //     // TODO user should be only one.
    //     let user_entity_output = &found_user[0];

    //     let user_output = UserEntity {
    //         id: Some(user_entity_output.id),
    //         first_name: Some(user_entity_output.first_name.clone()),
    //         last_name: user_entity_output.last_name.clone(),
    //         email: user_entity_output.email.clone(),
    //     };
    //     let output = FindUserOutputData { user: user_output };
    //     return Some(output);
    // }

    // TODO GET plural users.
    // pub fn get_users(input: FindUserInputData) -> FindUserOutputData {
    // use rust_api::schema::users::dsl::users;
    // let results = users.load::<User>(&connection);
    // }
}

// sample function
fn get_user(id: u32) -> UserEntity {
    UserEntity::new(
        id,
        "abc".to_string(),
        " def".to_owned(),
        "a@example.com".to_string(),
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
            "a@example.com".to_string(),
        );

        let user = get_user(id);

        assert_eq!(user.id, expected_user.id);
        assert_eq!(user.first_name, expected_user.first_name);
        assert_eq!(user.last_name, expected_user.last_name);
        assert_eq!(user.email, expected_user.email);
    }
}
