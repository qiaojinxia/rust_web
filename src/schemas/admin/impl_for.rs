use crate::common::error::MyError;
use crate::schemas::admin::sea_orm_active_enums::{TargetType, Type};

impl Type {
    pub fn from_string(s: &str) -> Result<Self, MyError> {
        match s {
            "1" => Ok(Type::Directory),
            "2" => Ok(Type::Menu),
            "3" => Ok(Type::Button),
            _ => Err(MyError::InvalidType(s.to_string())),
        }
    }
}

impl TargetType {
    pub fn from_string(s: &str) -> Result<Self, MyError> {
        match s {
            "1" => Ok(TargetType::Menu),
            "2" => Ok(TargetType::ApiGroup),
            _ => Err(MyError::InvalidType(s.to_string())),
        }
    }
}