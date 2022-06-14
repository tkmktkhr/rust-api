use diesel::{Insertable, Queryable};

// NOTE #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query.
// #[derive(Queryable, Insertable)]
#[derive(Queryable)]
// #[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: bool,
}
