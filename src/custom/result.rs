use crate::custom::errors::AppError;

pub type AppResult<T> = Result<T, AppError>;
