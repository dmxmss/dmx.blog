use crate::lib::errors::AppError;

pub type Result<T> = std::result::Result<T, AppError>;
