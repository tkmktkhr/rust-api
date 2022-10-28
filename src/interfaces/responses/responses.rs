use serde::Serialize;

// TODO abstract. Use <T>
// TODO NotFoundError -> General Error.
#[derive(Debug, Serialize)]
pub enum Res<T, E> {
    FindUserOutputData(Option<T>),
    CustomError(E),
}
// pub enum Res<T> {
//   FindUserOutputData(Option<T>),
//   NotFoundError(NotFoundError),
// }

#[derive(Debug, Serialize)]
pub struct ResponseStruct<T, E> {
    pub res: Res<T, E>,
}

// Errors
#[derive(Debug, Serialize)]
pub enum CustomError<T> {
    NotFoundError(NotFoundError),
    BadRequestError(BadRequestError),
}

#[derive(Debug, Serialize)]
pub struct NotFoundError {
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct BadRequestError {
    pub msg: String,
}
