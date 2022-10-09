use diesel::{Insertable, Queryable, QueryableByName};
use rust_api::schema::users;

// NOTE #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query.
#[derive(QueryableByName, Queryable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    // pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
