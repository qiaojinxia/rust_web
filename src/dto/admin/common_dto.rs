use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Serialize, Deserialize)]
pub struct PaginationResponseDto<T> {
    current: u64,
    size: u64,
    total: u64,
    records: Vec<T>,
}

impl<T> PaginationResponseDto<T> {
    pub fn new(current: u64, size: u64, total: u64, records: Vec<T>) -> Self {
        PaginationResponseDto {
            current,
            size,
            total,
            records,
        }
    }
}

#[derive(Deserialize, Copy, Clone)]
pub struct PaginationQueryDto {
    pub current: Option<u64>,
    pub size: Option<u64>,
}

pub fn validate_menu_type(status: &str) -> Result<(), ValidationError> {
    match status {
        "1" | "2" => Ok(()),
        _ => {
            let mut error = ValidationError::new("invalid_menu_type");
            error.message = Some("The menuType must be either '1' or '2'.".into());
            Err(error)
        }
    }
}

pub fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "1" | "2" => Ok(()),
        _ => {
            let mut error = ValidationError::new("invalid_status");
            error.message = Some("The status must be either '1' or '2'.".into());
            Err(error)
        }
    }
}
