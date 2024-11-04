use std::{error, fmt};

#[derive(Debug)]
pub enum AppError {
    TokenValidationError(jsonwebtoken::errors::Error),
    TokenEncodingError(jsonwebtoken::errors::Error)
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AppError::TokenValidationError(e) => write!(f, "Token is not valid: {}", e),
            AppError::TokenEncodingError(e) => write!(f, "Token encoding error: {}", e)
        }
    }
}

