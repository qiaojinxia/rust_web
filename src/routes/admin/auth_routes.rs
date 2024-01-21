use std::sync::Arc;
use actix_web::{Responder, web};
use sea_orm::{DatabaseConnection, DbErr};
use crate::dto::admin::auth_dto;
use crate::common::auth;
use crate::common::auth::crypto::verify_password;
use crate::common::auth::jwt::generate_jwt;
use crate::common::resp::ApiResponse;
use crate::create_response;
use actix_web::HttpResponse;
use crate::common::resp::ApiError;
use crate::schema::admin::sea_orm_active_enums::Gender;
use crate::services::admin::sys_user_services;
use actix_web::ResponseError;
pub async fn register(
    db: web::Data<Arc<DatabaseConnection>>,
    form: web::Form<auth_dto::RegisterRequestDto>,
) -> impl Responder {
    // 散列密码
    let password_hash = auth::crypto::hash_password(form.password.clone()); // 假设这是一个外部函数，用于安全地散列密码
    // 创建用户
    let mut user = sys_user_services::create_user(&*db, form.user_name.clone(), password_hash.unwrap(),
                                                  form.email.clone(), Gender::M, "".to_string()).await.unwrap();
    // 生成JWT
    let token = generate_jwt(&user.user_name, "user").unwrap(); // 假设这是一个外部函数，用于生成JWT
    // 屏蔽数据库返回的密码
    user.password = "".to_string();
    // 创建并返回AuthResponse
    let rs: Result<auth_dto::RegisterRespDto, ApiError> = Ok(auth_dto::RegisterRespDto{ user, jwt: token });
    create_response!(rs)
}


// 用户登录
pub async fn login(
    db: web::Data<Arc<DatabaseConnection>>,
    form: web::Form<auth_dto::LoginRequestDto>,
) -> impl Responder {
    let rs: Result<auth_dto::LoginRespDto, ApiError>;
    // 查找用户
    let user_opt = sys_user_services::find_user_by_email_or_mobile(&*db, Some(form.email.clone()), Some(form.mobile.clone())).await.unwrap();
    // 验证用户是否存在
    let mut user = user_opt.ok_or(DbErr::Custom("Invalid username or password".to_string())).unwrap();
    // 验证密码
    if verify_password(&form.password, &user.password).unwrap() {
        // 生成JWT
        let token = generate_jwt(&user.user_name, "user").unwrap(); // 假设这是一个外部函数，用于生成JWT
        // 屏蔽数据库返回的密码
        user.password = "".to_string();

        rs = Ok(auth_dto::LoginRespDto { user, jwt: token });
    }else{
        rs = Err(ApiError::InternalServerError("Invalid username or password".to_string()));
    }
    create_response!(rs)
}


pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))

    );
}
