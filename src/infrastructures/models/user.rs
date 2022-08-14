use diesel::{Insertable, Queryable};
use rust_api::schema::users;

// NOTE #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query.
#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    // pub id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
