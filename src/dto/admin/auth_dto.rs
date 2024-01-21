
use serde::{Deserialize, Serialize};
use crate::schema::admin::sys_user::Model;

#[derive(Deserialize)]
pub struct RegisterRequestDto {
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub email: String,
    // ... 其他字段
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRespDto {
    pub user: Model,
    pub jwt: String,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestDto {
    pub email: String,
    pub mobile: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRespDto {
    pub user: Model,
    pub jwt: String,
}

