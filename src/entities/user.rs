use serde::Deserialize;

#[derive(Debug)]
#[derive(Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
  pub fn full_name(user: &User) -> String {
    let ref first = *user.first_name;
    first.to_owned() + &user.last_name
  }
}
