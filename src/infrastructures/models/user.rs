use diesel::{Insertable, Queryable};

// NOTE #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query.
#[derive(Queryable, PartialEq, Debug, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
