use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserReq {
    pub user_id: u32,
    pub name: String,
}
