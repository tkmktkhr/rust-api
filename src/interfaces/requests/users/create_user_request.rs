use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserReq {
    pub user_id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
}
