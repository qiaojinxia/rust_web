use crate::config;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_name: String,       // 通常用于存放唯一用户标识
    pub exp: usize,              // Token的过期时间
    pub role_codes: Vec<String>, // 用户权限
}

// 假设的结构体表示菜单项
#[derive(Debug, FromQueryResult)]
pub struct MenuInfo {
    pub id: i32,
    pub menu_name: String,
    pub route: String,
    pub route_name: String,
}

impl Claims {
    pub fn is_expired(&self) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        current_time > self.exp
    }
    pub fn new() -> Self {
        Claims {
            user_name: "".to_string(),
            exp: 0,
            role_codes: vec![],
        }
    }
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config::globals::APP_CONFIG.jwt.secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}

pub fn generate_jwt(
    user_name: String,
    roles: Vec<String>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .checked_add(Duration::from_secs(
            config::globals::APP_CONFIG.jwt.expire_time,
        )) // Token有效期为1小时
        .expect("Invalid expiration time")
        .as_secs() as usize;

    let claims = Claims {
        user_name,
        exp: expiration,
        role_codes: roles,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config::globals::APP_CONFIG.jwt.secret.as_ref()),
    )
}
