use actix_web::{get, post, Responder, web};
use sea_orm::{DbErr};
use crate::common::auth::crypto::verify_password;
use crate::common::auth::jwt::generate_jwt;
use crate::common::resp::ApiResponse;
use crate::{config, create_response};
use actix_web::HttpResponse;
use crate::common::resp::ApiError;
use crate::services::admin::sys_user_services;
use validator::Validate;
use config::globals;
use actix_session::{Session};
use crate::dto::admin::sys_auth_dto::{SysLoginDto, SysLoginRespDto};
use actix_web::ResponseError;

// 用户登录
#[post("/login")]
pub async fn login(
    app_state: web::Data<globals::AppState>,
    sys_login_dto: web::Json<SysLoginDto>
) -> impl Responder {
    let rs: Result<SysLoginRespDto, ApiError>;
    match sys_login_dto.0.validate() {
        Ok(_) => {
            // 查找用户
            let user_opt = sys_user_services::find_user_by_username(
                &*app_state.mysql_conn, sys_login_dto.user_name.clone()).await.unwrap();
            // 验证用户是否存在
            let user = user_opt.ok_or(DbErr::Custom("Invalid username or password".to_string())).unwrap();
            // 验证密码
            if verify_password(&sys_login_dto.password.clone().unwrap(), &user.password).unwrap() {
                // 生成JWT
                let token = generate_jwt(user.user_name.clone(), vec!["user".to_string()]).unwrap(); // 假设这是一个外部函数，用于生成JWT

                rs = Ok(SysLoginRespDto { user_name: user.user_name, token });
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


#[get("/count")]
async fn index(session: Session) -> impl Responder {
    let count = session.get::<i32>("counter").
        unwrap().map(|count| count + 1).unwrap_or(1);
    match session.insert("counter", count ) {
        Ok(_) => {
            let res= format!("Success {}",count.to_string());
            let rs: Result<&str, ApiError> = Ok(res.as_str());
            create_response!(rs)
        }
        Err(e) => {
            // 这里处理错误，例如返回一个内部服务器错误
            let rs: Result<&str, ApiError> = Err(ApiError::InternalServerError(e.to_string()));
            create_response!(rs)
        }
    }
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(login)
        .service(index)
        .service(health_checker_handler);
}
