use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub user_id: u32,
    pub name: String,
}
