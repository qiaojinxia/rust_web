use validator::{Validate};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Validate)]
pub struct UserRegistrationDto {
    #[validate(length(min = 1, max = 255))]
    pub user_name: Option<String>,
    #[validate(length(min = 8, max = 255))]
    pub password: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub mobile: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistrationRespDto {
    pub user_name: String,
    pub token: String,
}



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