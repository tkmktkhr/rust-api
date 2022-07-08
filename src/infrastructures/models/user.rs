use diesel::Queryable;

// NOTE #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query.
#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
