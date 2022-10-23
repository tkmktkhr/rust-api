use serde::Serialize;

// TODO abstract. Use <T>
// TODO NotFoundError -> General Error.
#[derive(Debug, Serialize)]
pub enum Res<T> {
    FindUserOutputData(Option<T>),
    NotFoundError(NotFoundError),
}

#[derive(Debug, Serialize)]
pub struct ResponseStruct<T> {
    pub res: Res<T>,
}

// Errors
#[derive(Debug, Serialize)]
pub struct NotFoundError {
    pub msg: String,
}
