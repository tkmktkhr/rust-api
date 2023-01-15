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
    pub err: CustomError, // TODO array
}

#[derive(Debug, Serialize)]
pub struct CustomError {
    // REFACTOR if possible, use pub code: http::StatusCode,
    code: u16,
    msg: String,
}

pub trait TCustomError {
    fn new(code: u16, msg: String) -> Self;
}

impl TCustomError for CustomError {
    fn new(code: u16, msg: String) -> Self {
        Self { code, msg }
    }
}

// #[derive(Debug, Serialize)]
// pub enum CustomError1 {
//     NotFoundError(NotFoundError),
//     BadRequestError(BadRequestError),
// }

// #[derive(Debug, Serialize)]
// pub struct NotFoundError {
//     // TODO add code.
//     // pub code: http::StatusCode,
//     pub msg: String,
// }

// #[derive(Debug, Serialize)]
// pub struct BadRequestError {
//     pub msg: String,
// }
