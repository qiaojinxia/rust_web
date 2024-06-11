use serde::{Deserialize, Serialize};
use validator::ValidationError;
use regex::Regex;

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

pub fn validate_mobile(mobile: &str) -> Result<(), ValidationError> {
    // Define a regular expression for phone number validation.
    // This example assumes international phone numbers starting with '+' followed by 10 to 15 digits.
    let re = Regex::new(r"^\+?[1-9]\d{9,14}$").unwrap();

    if re.is_match(mobile) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_mobile");
        error.message = Some("The mobile number must be a valid international number starting with a '+' followed by 10 to 15 digits.".into());
        Err(error)
    }
}

pub fn validate_gender(gender: &str) -> Result<(), ValidationError> {
    match gender {
        "1" | "2" | "3" => Ok(()),
        _ => {
            let mut error = ValidationError::new("invalid_gender");
            error.message = Some("The gender must be either '1', '2', or '3'.".into());
            Err(error)
        }
    }
}