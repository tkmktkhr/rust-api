use serde::Serialize;

#[derive(Debug, Serialize)]
struct NotFoundError {
    msg: String,
}

// TODO abstract. Use <T>
// TODO NotFoundError -> General Error.
#[derive(Debug, Serialize)]
pub enum Res<T> {
    FindUserOutputData(Option<T>),
    NotFoundError(NotFoundError),
}

#[derive(Debug, Serialize)]
pub struct ResponseStruct<T> {
    res: Res<T>,
}
