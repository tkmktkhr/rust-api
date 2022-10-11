use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserReq {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
}
