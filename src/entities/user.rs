use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn full_name(user: &User) -> String {
        // Pattern1
        // let ref first = *user.first_name;
        // first.to_owned() + &user.last_name

        // Pattern2
        user.first_name.to_string() + &user.last_name

        // Pattern3
        // String::from(&user.first_name) + &user.last_name
    }
}
