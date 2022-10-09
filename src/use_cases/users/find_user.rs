use crate::entities::user::UserEntity;
use crate::infrastructures::dbs::mysql::connection;
use crate::infrastructures::models::user::User;
use diesel::result::Error;
use diesel::sql_query;
use diesel::sql_types::Integer;
use serde::Serialize;

// DTO<Input> validation should be here?
pub struct FindUserInputData {
    pub id: i32,
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
    // TODO return type is Result? it should return None for Not found.
    pub fn get_user_by_id(input: FindUserInputData) -> Option<FindUserOutputData> {
        use diesel::prelude::*;
        use rust_api::schema::users::dsl::users;

        // REFACTOR connection should be taken as global object.
        let pool = connection::get_connection_pool();
        let connection = pool.get().unwrap();

        // NOTE Application Logic is here

        // FIXME Dependency Inversion principle.
        // FIXME Get Indicated user.
        let results = users.load::<User>(&connection);
        // let results = users.filter(users::id.eq(input.id)).first(&connection);
        // let user_vec = match results {
        //     Ok(user) => user,
        //     Err(error) => {
        //         // TODO return results as a zero value.
        //         panic!("There was not that user: {:?}", error)
        //     }
        // };

        let users_res: Result<Vec<User>, Error> = sql_query(
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
        .bind::<Integer, _>(input.id)
        .load::<User>(&connection);
        // .load::<(i32, String, Option<String>, Option<String>)>(&connection);
        print!("{:?}", users_res);

        let vec_res = match users_res {
            Ok(vec) => vec,
            Err(e) => panic!("Problem creating the file: {:?}", e), // TODO ERROR PROCESS
        };

        if vec_res.is_empty() {
            // TODO return None?
            return None;
        }

        // let user_output = UserEntity {
        //     id: Some(user_entity_output.id),
        //     first_name: Some(user_entity_output.first_name.clone()),
        //     last_name: user_entity_output.last_name.clone(),
        //     email: user_entity_output.email.clone(),
        // };

        // if user_vec.is_empty() {
        //     // TODO return None?
        //     return None;
        // }

        // // TODO fix.
        // let user_entity_output = &user_vec[0];
        let user_entity_output = &vec_res[0];

        let user_output = UserEntity {
            id: Some(user_entity_output.id),
            first_name: Some(user_entity_output.first_name.clone()),
            last_name: user_entity_output.last_name.clone(),
            email: user_entity_output.email.clone(),
        };
        let output = FindUserOutputData { user: user_output };
        return Some(output);
    }

    // pub fn get_users(input: FindUserInputData) -> FindUserOutputData {}
}

// sample function
fn get_user(id: i32) -> UserEntity {
    UserEntity::new(
        id,
        "abc".to_string(),
        " def".to_owned(),
        // Some("a@example.com".to_string()),
        None,
    )
}

// For View
struct FindUserPresenter {
    pub id: i32,
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
        );

        let user = get_user(id);

        assert_eq!(user.id, expected_user.id);
        assert_eq!(user.first_name, expected_user.first_name);
        assert_eq!(user.last_name, expected_user.last_name);
        assert_eq!(user.email, expected_user.email);
    }
}
