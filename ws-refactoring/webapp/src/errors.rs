use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum MyError {
    ActixError(String),
    NotFound(String), //这里没有用到这个变体，编译器会发出警告，所以上面使用dead_code
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for MyError {}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::TeraError(msg) => {
                println!("Error in rendering the template: {:?}", msg);
                msg.into()
            }
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                msg.into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::TeraError(_) | MyError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}
