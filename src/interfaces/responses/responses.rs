use serde::Serialize;

// TODO abstract. Use <T>
// TODO NotFoundError -> General Error.
#[derive(Debug, Serialize)]
pub enum Res<T> {
    OutputData(Option<T>), // to general object.
    CustomError(CustomError),
    // NotFoundError(NotFoundError), // todo arrange all errors here?
}

#[derive(Debug, Serialize)]
pub struct ResponseStruct<T> {
    pub res: Res<T>,
}

// Errors
#[derive(Debug, Serialize)]
pub struct CustomErrorStruct<CustomError> {
    // pub code: http::StatusCode,
    pub err: CustomError,
}

#[derive(Debug, Serialize)]
pub struct CustomError {
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub enum CustomError1 {
    NotFoundError(NotFoundError),
    BadRequestError(BadRequestError),
}

#[derive(Debug, Serialize)]
pub struct NotFoundError {
    // TODO add code.
    // pub code: http::StatusCode,
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct BadRequestError {
    pub msg: String,
}
