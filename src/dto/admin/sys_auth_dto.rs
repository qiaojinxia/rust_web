use validator::{Validate};

use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SysLoginDto {
    #[validate(length(min = 1, max = 255))]
    pub user_name: Option<String>,
    #[validate(length(min = 8, max = 255))]
    pub password: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SysLoginRespDto {
    pub user_name: String,
    pub token: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoDto {
    pub user_name: String,
    pub user_info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
