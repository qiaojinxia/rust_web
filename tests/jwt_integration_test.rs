// tests/jwt_integration_test.rs

use my_gpt::common::auth::jwt;
#[test]
fn test_jwt_integration() {
    let user_name = "integration_test_user";
    let role = "user";
    let jwt = jwt::generate_jwt(user_name, role).expect("Failed to generate JWT");

    assert!(!jwt.is_empty(), "JWT should not be empty");
}

#[test]
fn test_encryption_decryption() {
    let original_data = "Hello, Rust!";
    let key = "secret_key"; // 加解密使用的密钥

    // 假设 generate_jwt 函数接受 Claims 结构体作为参数
    let claims = jwt::Claims {
        user_name: original_data.to_owned(),
        exp: 0, // 示例过期时间
        role: "admin".to_owned(),
    };

    // 加密原始数据
    let encrypted_data = jwt::generate_jwt(&claims.user_name, &claims.role).unwrap(); // 确保generate_jwt返回Result<String, Error>

    // 尝试解密
    let token_data = jwt::decode_jwt(&encrypted_data).expect("Decryption failed"); // 确保decode_jwt正确处理并返回TokenData<Claims>
    let decrypted_claims = token_data.claims;

    // 验证解密后的数据是否与原始数据相同
    assert_eq!(claims.user_name, decrypted_claims.user_name, "Decrypted data does not match original");
    assert_eq!(claims.role, decrypted_claims.role, "Decrypted data does not match original");
}