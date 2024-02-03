use std::sync::Arc;
use actix_web::{get, post, Responder, web};
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
use validator::Validate;


#[post("/register")]
pub async fn register(
    db: web::Data<Arc<DatabaseConnection>>,
    req: web::Json<auth_dto::RegisterRequestDto>
) -> impl Responder {
    let rs: Result<auth_dto::RegisterRespDto, ApiError>;
    match req.0.validate() {
        Ok(_) => {
            // 散列密码
            let password_hash = auth::crypto::hash_password(req.password.clone()); // 假设这是一个外部函数，用于安全地散列密码
            // 创建用户
            let user = sys_user_services::create_user(&*db, req.user_name.clone(), password_hash.unwrap(),
                                                          req.email.clone(), Gender::M, req.mobile.clone(), req.create_user.clone(),
                                                          req.update_user.clone()).await.unwrap();
            // 生成JWT
            let token = generate_jwt(&user.user_name, "user").unwrap(); // 假设这是一个外部函数，用于生成JWT
            // 创建并返回AuthResponse
            rs= Ok(auth_dto::RegisterRespDto{ token, user_name:user.user_name });
        },
        Err(errors) => {
            rs = Err(ApiError::BadRequest(errors.to_string()))
        },
    }
    create_response!(rs)
}


// 用户登录
#[post("/login")]
pub async fn login(
    db: web::Data<Arc<DatabaseConnection>>,
    req: web::Json<auth_dto::LoginRequestDto>
) -> impl Responder {
    let rs: Result<auth_dto::LoginRespDto, ApiError>;
    match req.0.validate() {
        Ok(_) => {
            // 查找用户
            let user_opt = sys_user_services::find_user_by_username(&*db, req.user_name.clone()).await.unwrap();
            // 验证用户是否存在
            let user = user_opt.ok_or(DbErr::Custom("Invalid username or password".to_string())).unwrap();
            // 验证密码
            if verify_password(&req.password.clone().unwrap(), &user.password).unwrap() {
                // 生成JWT
                let token = generate_jwt(&user.user_name, "user").unwrap(); // 假设这是一个外部函数，用于生成JWT

                rs = Ok(auth_dto::LoginRespDto { user_name: user.user_name, token });
            }else{
                rs = Err(ApiError::InternalServerError("Invalid username or password".to_string()));
            }
        },
        Err(errors) => {
            rs = Err(ApiError::BadRequest(errors.to_string()))
        },
    }
    create_response!(rs)
}

#[get("/health-checker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let rs: Result<&str, ApiError> = Ok(MESSAGE);

    create_response!(rs)
}


pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login)
            .service(register)
            .service(health_checker_handler)
    );
}
