// // test.rs
// use actix_web::{web, HttpResponse, Responder};
// use std::sync::Arc;
// use sea_orm::{DatabaseConnection};
// use crate::common::{crypto, jwt};
//
// use serde::Deserialize;
//
// use crate::common::error::{ApiError, ApiResponse};
// use actix_web::ResponseError;
// use crate::create_response;
// use crate::schema::admin::response::user_response;
// // 定义一个宏来统一处理响应
//
//
// pub fn api_config(_: &mut web::ServiceConfig) {
//     // cfg.service(
//     //     web::scope("/api/auth")
//     //         .route("/register", web::post().to(register))
//     //         .route("/login", web::post().to(login)),
//     // );
// }
//
// // 首先定义一个结构体来表示你的表单数据
// #[derive(Deserialize)]
// pub struct RegisterRequest {
//    pub user_name:String,
//    pub password:String,
//    pub email:Option<String>,
//    pub gender:Option<sea_orm_active_enums::Gender>,
// }
//
// async fn register(
//     db: web::Data<Arc<DatabaseConnection>>,
//     form: web::Form<RegisterRequest>,
// ) -> impl Responder {
//     let result: Result<user_response::AuthResponse, ApiError> = async move {
//         // 尝试散列密码，如果失败则提早返回错误
//         let password_hash = crypto::hash_password(&form.password).map_err(|_|
//             ApiError::InternalServerError("Failed to hash password".to_string())
//         )?;
//
//         // 尝试创建用户，如果失败则提早返回错误
//         let mut user = user_services::create_user(
//             &*db,
//             form.user_name.clone(),
//             password_hash,
//             form.gender.clone(),
//             form.email.clone(),
//         ).await.map_err(|_|
//             ApiError::InternalServerError("Failed to create user".to_string())
//         )?;
//
//         // 尝试生成JWT，如果失败则提早返回错误
//         let token = jwt::generate_jwt(&user.username, "user").map_err(|_|
//             ApiError::InternalServerError("Failed to generate JWT".to_string())
//         )?;
//
//         // 屏蔽数据库返回密码
//         user.username = "".to_string();
//
//         // 创建并返回AuthResponse
//         let auth_response = user_response::AuthResponse { user, jwt: token };
//         Ok(auth_response)  // 注意这里返回的是 Result 类型
//     }.await;  // 等待异步块完成
//
//     create_response!(result)
// }
//
// // 登录的异步函数
// async fn login(
//     db: web::Data<Arc<DatabaseConnection>>,
//     form: web::Form<LoginRequest>,
// ) -> impl Responder {
//     // 使用 `and_then` 来链式处理正确的情况
//     let result = user_services::find_user(&*db, form.user_name.as_str()).await
//         .map_err(|_| ApiError::InternalServerError("Database error".to_string())) // 转换任何数据库错误为内部服务器错误
//         .and_then(|user_opt| {
//             user_opt.ok_or_else(|| ApiError::Unauthorized("Invalid username or password".to_string())) // 如果用户不存在，返回错误
//         })
//         .and_then(|user| {
//             // 验证密码
//             crypto::verify_password(&form.password, &user.password_hash)
//                 .map_err(|_| ApiError::InternalServerError("Server error".to_string())) // 如果密码错误，返回错误
//                 .and_then(|password_match| {
//                     if password_match {
//                         Ok(user) // 如果密码正确，继续链式调用
//                     } else {
//                         Err(ApiError::Unauthorized("Invalid username or password".to_string())) // 如果密码不匹配，返回错误
//                     }
//                 })
//         })
//         .and_then(|mut user| {
//             // 屏蔽数据库返回密码
//             user.password_hash = "".to_string();
//             // 生成JWT
//             jwt::generate_jwt(&user.username, "user")
//                 .map_err(|_| ApiError::InternalServerError("Failed to generate JWT".to_string()))
//                 .map(|token| user_response::AuthResponse { user, jwt: token }) // 创建 auth_response
//         });
//
//     create_response!(result)
// }
//
