use crate::common::error::MyError;
use crate::schemas::admin::sea_orm_active_enums::{ActionCode, TargetType, Type};

impl Type {
    pub fn from_string(s: &str) -> Result<Self, MyError> {
        match s {
            "1" => Ok(Type::Directory),
            "2" => Ok(Type::Menu),
            "3" => Ok(Type::Button),
            _ => Err(MyError::InvalidTypeError(format!(
                "invalid Menu Type {}",
                s.to_string()
            ))),
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
            _ => Err(MyError::InvalidTypeError(format!(
                "invalid TargetType {}",
                s.to_string()
            ))),
        }
    }
}

impl ActionCode {
    pub fn from_string(s: &str) -> Result<Self, MyError> {
        match s {
            "1" => Ok(ActionCode::Create),
            "2" => Ok(ActionCode::Read),
            "3" => Ok(ActionCode::Update),
            "4" => Ok(ActionCode::Delete),
            _ => Err(MyError::InvalidTypeError(format!(
                "invalid ActionCode {}",
                s.to_string()
            ))),
        }
    }
    pub fn from_string_origin(s: &str) -> Result<&'static str, MyError> {
        match s {
            "CREATE" => Ok("1"),
            "READ" => Ok("2"),
            "UPDATE" => Ok("3"),
            "DELETE" => Ok("4"),
            _ => Err(MyError::InvalidTypeError(format!(
                "invalid ActionCode {}",
                s
            ))),
        }
    }
}
