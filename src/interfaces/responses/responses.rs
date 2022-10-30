use std::num::NonZeroU16;

use actix_web::http;
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
pub struct ResponseStruct<T, CustomError> {
    pub res: Res<T, CustomError>,
}

// Errors
#[derive(Debug, Serialize)]
pub enum CustomError {
    NotFoundError(NotFoundError),
    BadRequestError(BadRequestError),
}

#[derive(Debug, Serialize)]
pub struct NotFoundError {
    // pub code: http::StatusCode,
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct BadRequestError {
    pub msg: String,
}
