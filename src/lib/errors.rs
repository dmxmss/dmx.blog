use std::{error, fmt};
use std::error::Error;
use rocket::{
    http::Status,
    request::Outcome
};
use jsonwebtoken::errors::ErrorKind;


#[derive(Debug)]
pub enum AppError {
    TokenValidationError(jsonwebtoken::errors::Error),
    InternalServerError(Box<dyn Error + Send + Sync>)
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AppError::TokenValidationError(e) => write!(f, "token is not valid: {}", e),
            AppError::InternalServerError(e) => write!(f, "internal error: {}", *e)
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::InternalServerError(Box::new(error))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::InternalServerError(Box::new(error))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> AppError {
        match e.kind() {
            ErrorKind::ExpiredSignature | 
            ErrorKind::InvalidToken     | 
            ErrorKind::InvalidSignature => AppError::TokenValidationError(e),

            _ => AppError::InternalServerError(Box::new(e))
        }
    }
}

impl From<AppError> for Status {
    fn from(e: AppError) -> Self {
        match e {
            AppError::TokenValidationError(_)=> Status::Unauthorized,
            AppError::InternalServerError(_) => Status::InternalServerError
        }
    }
}

impl<T> From<AppError> for Outcome<T, AppError> 
{
    fn from(e: AppError) -> Self {
        match e {
            AppError::TokenValidationError(_) => Outcome::Error((Status::Unauthorized, e)),
            AppError::InternalServerError(_) => Outcome::Error((Status::InternalServerError, e)),
        }
    }
}
