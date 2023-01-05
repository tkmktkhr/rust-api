use crate::utils::string_util::string_util;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: Option<u32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
}

// Business Rules are here.
impl UserEntity {
    pub fn new(id: u32, first_name: String, last_name: String, email: String) -> UserEntity {
        UserEntity {
            id: Some(id),
            first_name: Some(first_name),
            last_name: Some(last_name),
            email,
        }
    }

    pub fn full_name(&self) -> String {
        let fname = string_util::check_string_return_string(&self.first_name);
        let lname = string_util::check_string_return_string(&self.last_name);
        fname + &lname.to_owned()

        // Pattern1
        // let ref first = *user.first_name;
        // first.to_owned() + &user.last_name

        // Pattern2
        // user.first_name.to_owned() + &user.last_name

        // Pattern3
        // user.first_name.to_string() + &user.last_name

        // Pattern4
        // String::from(&user.first_name) + &user.last_name
    }
}
