use crate::common::error::MyError;
use crate::schemas::admin::sea_orm_active_enums::Type;

impl Type {
    pub fn from_string(s: &str) -> Result<Self, MyError> {
        match s {
            "DIRECTORY" => Ok(Type::Directory),
            "MENU" => Ok(Type::Menu),
            "BUTTON" => Ok(Type::Button),
            _ => Err(MyError::InvalidType(s.to_string())),
        }
    }
}