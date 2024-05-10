use crate::common::auth::crypto::verify_password;
use crate::common::auth::jwt::generate_jwt;
use crate::common::error::MyError;
use crate::common::error::MyError::AuthError;
use crate::common::resp::ApiError;
use crate::common::resp::ApiResponse;
use crate::dto::admin::sys_auth_dto::{SysLoginDto, SysLoginRespDto};
use crate::dto::admin::sys_user_dto::UserInfo;
use crate::schemas::admin::sys_user;
use crate::services::admin::sys_user_services;
use crate::{config, create_response};
use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use actix_web::{get, post, web, Responder};
use config::globals;
use sea_orm::DatabaseConnection;
use validator::Validate;

async fn authenticate_user(
    conn: &DatabaseConnection,
    sys_login_dto: &SysLoginDto,
) -> Result<sys_user::Model, MyError> {
    let user_opt =
        sys_user_services::find_user_by_username(&*conn, sys_login_dto.user_name.clone()).await?;
    let user = user_opt.ok_or(AuthError("Invalid username or password".to_string()))?;

    if verify_password(&sys_login_dto.password.clone().unwrap(), &user.password).unwrap() {
        Ok(user)
    } else {
        Err(AuthError("Invalid username or password".to_string()))
    }
}

// 用户登录
#[post("/login")]
pub async fn login(
    app_state: web::Data<globals::AppState>,
    sys_login_dto: web::Json<SysLoginDto>,
) -> impl Responder {
    let rs: Result<SysLoginRespDto, ApiError>;
    match sys_login_dto.0.validate() {
        Ok(_) => match authenticate_user(&app_state.mysql_conn, &sys_login_dto).await {
            Ok(user) => {
                let token = generate_jwt(user.user_name.clone(), vec!["user".to_string()]).unwrap();
                let resp_dto = SysLoginRespDto {
                    user_name: user.user_name,
                    token,
                    refresh_token: "xxx".to_string(),
                };
                rs = Ok(resp_dto);
            }
            Err(err) => rs = Err(ApiError::Unauthorized(err.to_string())),
        },
        Err(errors) => rs = Err(ApiError::BadRequest(errors.to_string())),
    }
    create_response!(rs)
}

#[get("/user-info")]
async fn user_info() -> impl Responder {
    // 创建User结构体实例并插入到HashMap中
    let user1 = UserInfo {
        user_id: "0".to_string(),
        user_name: "Soybean".to_string(),
        buttons: vec![
            "B_CODE1".to_string(),
            "B_CODE2".to_string(),
            "B_CODE3".to_string(),
        ],
        roles: vec!["R_SUPER".to_string()],
    };

    let rs: Result<ApiResponse<UserInfo>, ApiError> = Ok(ApiResponse::success(user1));

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
    let count = session
        .get::<i32>("counter")
        .unwrap()
        .map(|count| count + 1)
        .unwrap_or(1);
    match session.insert("counter", count) {
        Ok(_) => {
            let res = format!("Success {}", count.to_string());
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
    cfg.service(login)
        .service(index)
        .service(user_info)
        .service(health_checker_handler);
}
