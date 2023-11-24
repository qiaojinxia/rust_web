// use jsonwebtoken::{encode, EncodingKey, Header};
// use chrono::{Utc, Duration};

// fn generate_jwt(user_id: i64, secret_key:&str) -> Result<String, jsonwebtoken::errors::Error> {
//     let expiration = Utc::now()
//         .checked_add_signed(Duration::hours(24))
//         .expect("valid timestamp")
//         .timestamp();

//     let claims = MyClaims {
//         exp: expiration as usize,
//         iat: Utc::now().timestamp() as usize,
//         user_id,
//     };

//     let encoding_key = EncodingKey::from_secret(secret_key);
//     encode(&Header::default(), &claims, &encoding_key)
// }
