// test.rs
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use sea_orm::{ DatabaseConnection };
use crate::utils::crypto;
use crate::models::database::sea_orm_active_enums;
use serde::Deserialize;
use crate::services::user_services;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}

// 首先定义一个结构体来表示你的表单数据
#[derive(Deserialize)]
pub struct RegisterRequest {
   pub user_name:String,
   pub password:String,
   pub email:Option<String>,
   pub gender:Option<sea_orm_active_enums::Gender>,
}

async fn register(
    db: web::Data<Arc<DatabaseConnection>>,
    form: web::Form<RegisterRequest>,
) -> impl Responder {
      match crypto::hash_password(&form.password.clone()) {
                Ok(password_hash) => {
                    match user_services::create_user(
                        &*db,
                        form.user_name.clone(),
                        password_hash,
                        form.gender.clone(),
                        form.email.clone(),
                    )
                    .await
                    {
                        Ok(user) => HttpResponse::Ok().json(user), // 假设你要返回 JSON 格式的 user
                        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
                    }
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to hash password"),
            }
}

// 登录的异步函数（暂未修改）
async fn login(_db: web::Data<Arc<DatabaseConnection>>) -> impl Responder {
    HttpResponse::Ok().body("are you ok")
}

