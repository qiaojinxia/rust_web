use actix_web::{HttpResponse, ResponseError};
use serde::{Serialize, Deserialize};
use serde_json::json;
use thiserror::Error;
use actix_web::http::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(code: u16, message: &str, data: T) -> ApiResponse<T> {
        ApiResponse {
            code,
            message: message.to_string(),
            data,
        }
    }

    pub fn success(data: T) -> ApiResponse<T> {
        ApiResponse::new(200, "Success", data)
    }

    pub fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn from_error<E>(error: E) -> ApiResponse<serde_json::Value>
        where
            E: ResponseError,
    {
        ApiResponse {
            code: error.status_code().as_u16(),
            message: error.to_string(),
            data: json!({}),
        }
    }

}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Internal Server Error {0}")]
    InternalServerError(String),

    #[error("Bad Request {0}")]
    BadRequest(String),

    #[error("Unauthorized {0}")]
    Unauthorized(String),

    #[error("Not Found {0}")]
    NotFound(String),

    #[error("Custom Error: {0}")]
    CustomError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = match self {
            ApiError::InternalServerError(msg) => msg,
            ApiError::BadRequest(msg) => msg,
            ApiError::Unauthorized(msg) => msg,
            ApiError::NotFound(msg) => msg,
            ApiError::CustomError(msg) => msg,
        };
        let error_response = ApiResponse {
            code: status_code.as_u16(),
            message: error_message.to_string(),
            data: json!(null), // 通常错误响应的data是null
        };
        HttpResponse::build(status_code).json(error_response)
    }


}

#[macro_export]
macro_rules! create_response {
    ($result:expr) => {
        match $result {
            Ok(data) => HttpResponse::Ok().json(ApiResponse::success(data)),
            Err(e) => {
                let api_error: ApiError = e.into();
                let code = api_error.status_code();
                let error_response = ApiResponse::<serde_json::Value>::from_error(api_error);
                HttpResponse::build(code).json(error_response)
            }
        }
    };
}