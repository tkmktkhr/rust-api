use crate::utils::check_string::check_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserStruct {
    pub id: Option<u32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

// Business Rules are here.
impl UserStruct {
    pub fn new(id: u32, first_name: String, last_name: String, email: String) -> UserStruct {
        UserStruct {
            id: Some(id),
            first_name: Some(first_name),
            last_name: Some(last_name),
            email: Some(email),
        }
    }

    pub fn full_name(&self) -> String {
        let fname = check_string(&self.first_name);
        let lname = check_string(&self.last_name);
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
