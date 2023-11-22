use thiserror::Error;
use actix_web::{HttpResponse, http::StatusCode, error::ResponseError};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("内部服务器错误")]
    InternalServerError,

    #[error("无效请求: {0}")]
    BadRequest(String),

    #[error("未授权")]
    Unauthorized,

    #[error("禁止访问")]
    Forbidden,

    #[error("未找到")]
    NotFound,

    #[error("方法不允许")]
    MethodNotAllowed,

    #[error("冲突")]
    Conflict,

    #[error("无法处理的实体: {0}")]
    UnprocessableEntity(String),
    
    // 可以继续添加其他错误类型...
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiError::InternalServerError => {
                HttpResponse::InternalServerError().json("内部服务器错误")
            },
            ApiError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(format!("无效请求: {}", message))
            },
            ApiError::Unauthorized => {
                HttpResponse::Unauthorized().json("未授权")
            },
            ApiError::Forbidden => {
                HttpResponse::Forbidden().json("禁止访问")
            },
            ApiError::NotFound => {
                HttpResponse::NotFound().json("未找到")
            },
            ApiError::MethodNotAllowed => {
                HttpResponse::MethodNotAllowed().json("方法不允许")
            },
            ApiError::Conflict => {
                HttpResponse::Conflict().json("冲突")
            },
            ApiError::UnprocessableEntity(ref message) => {
                HttpResponse::UnprocessableEntity().json(format!("无法处理的实体: {}", message))
            },
            // 处理其他错误...
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Conflict => StatusCode::CONFLICT,
            ApiError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            // 设置其他错误的状态码...
        }
    }
}
