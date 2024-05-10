use crate::common::error::MyError;
use crate::schemas::admin::sea_orm_active_enums::{TargetType, Type};

impl Type {
    pub fn from_string(s: &str) -> Result<Self, MyError> {
        match s {
            "1" => Ok(Type::Directory),
            "2" => Ok(Type::Menu),
            "3" => Ok(Type::Button),
            _ => Err(MyError::InvalidTypeError(s.to_string())),
        }
    }
    pub fn get_serial_number(&self) -> Option<&'static str> {
        match self {
            Type::Directory => Some("1"),
            Type::Menu => Some("2"),
            Type::Button => Some("3"),
        }
    }
}

impl TargetType {
    pub fn from_string(s: &str) -> Result<Self, MyError> {
        match s {
            "1" => Ok(TargetType::Menu),
            "2" => Ok(TargetType::ApiGroup),
            _ => Err(MyError::InvalidTypeError(s.to_string())),
        }
    }
}
