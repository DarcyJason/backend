use crate::custom::errors::app_error::AppError;

pub type AppResult<T> = Result<T, AppError>;
