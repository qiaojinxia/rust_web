use actix_web::{get, post, Responder, web};
use sea_orm::{DbErr};
use crate::dto::admin::sys_auth_dto;
use crate::common::auth;
use crate::common::auth::crypto::verify_password;
use crate::common::auth::jwt::generate_jwt;
use crate::common::resp::ApiResponse;
use crate::{config, create_response};
use actix_web::HttpResponse;
use crate::common::resp::ApiError;
use crate::schema::admin::sea_orm_active_enums::Gender;
use crate::services::admin::sys_user_services;
use actix_web::ResponseError;
use validator::Validate;
use config::globals;

#[post("/register")]
pub async fn register(
    app_state: web::Data<globals::AppState>,
    user_register_dto: web::Json<sys_auth_dto::UserRegistrationDto>
) -> impl Responder {
    let rs: Result<sys_auth_dto::UserRegistrationRespDto, ApiError>;
    match user_register_dto.0.validate() {
        Ok(_) => {
            // 散列密码
            let password_hash = auth::crypto::hash_password(user_register_dto.password.clone()); // 假设这是一个外部函数，用于安全地散列密码
            // 创建用户
            let user = sys_user_services::create_user(&*app_state.mysql_conn, user_register_dto.user_name.clone(), password_hash.unwrap(),
                                                      user_register_dto.email.clone(), Gender::M, user_register_dto.mobile.clone(), user_register_dto.create_user.clone(),
                                                      user_register_dto.update_user.clone()).await.unwrap();
            // 生成JWT
            let token = generate_jwt(&user.user_name, "user").unwrap(); // 假设这是一个外部函数，用于生成JWT
            // 创建并返回AuthResponse
            rs = Ok(sys_auth_dto::UserRegistrationRespDto { token, user_name:user.user_name });
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
    app_state: web::Data<globals::AppState>,
    sys_login_dto: web::Json<sys_auth_dto::SysLoginDto>
) -> impl Responder {
    let rs: Result<sys_auth_dto::SysLoginRespDto, ApiError>;
    match sys_login_dto.0.validate() {
        Ok(_) => {
            // 查找用户
            let user_opt = sys_user_services::find_user_by_username(&*app_state.mysql_conn, sys_login_dto.user_name.clone()).await.unwrap();
            // 验证用户是否存在
            let user = user_opt.ok_or(DbErr::Custom("Invalid username or password".to_string())).unwrap();
            // 验证密码
            if verify_password(&sys_login_dto.password.clone().unwrap(), &user.password).unwrap() {
                // 生成JWT
                let token = generate_jwt(&user.user_name, "user").unwrap(); // 假设这是一个外部函数，用于生成JWT

                rs = Ok(sys_auth_dto::SysLoginRespDto { user_name: user.user_name, token });
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
    cfg
        .service(login)
        .service(register)
        .service(health_checker_handler);
}
