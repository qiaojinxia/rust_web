use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_name: String,  // 通常用于存放唯一用户标识
    exp: usize,   // Token的过期时间
    role:String, // 用户权限
}

pub fn generate_jwt(user_name: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .checked_add(Duration::from_secs(60 * 60)) // Token有效期为1小时
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
        &EncodingKey::from_secret("your-secret-key".as_ref()),
    )
}


