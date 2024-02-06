use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use jsonwebtoken::{decode, encode, Algorithm, Header, EncodingKey,DecodingKey, Validation, TokenData};
use crate::config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_name: String,  // 通常用于存放唯一用户标识
    pub exp: usize,   // Token的过期时间
    pub role:String, // 用户权限
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config::globals::APP_CONFIG.jwt.secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}

pub fn generate_jwt(user_name: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .checked_add(Duration::from_secs(config::globals::APP_CONFIG.jwt.expire_time)) // Token有效期为1小时
        .expect("Invalid expiration time")
        .as_secs() as usize;

    let claims = Claims {
        user_name: user_name.to_string(),
        exp: expiration,
        role: role.to_string(),
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config::globals::APP_CONFIG.jwt.secret.as_ref()),
    )
}

