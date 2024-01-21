extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};

/// 使用bcrypt算法哈希密码。
pub fn hash_password(password: Option<String>) -> Result<String, bcrypt::BcryptError> {
    hash(password.unwrap(), DEFAULT_COST)
}

/// 验证密码与bcrypt哈希是否匹配。
pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed_password)
}
