use crate::error::AppError;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

pub const DATABASE_URL: &str = "DATABASE_URL";
pub const BIND_ADDRESS: &str = "BIND_ADDRESS";