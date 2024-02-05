use actix::Addr;
use actix_redis::RedisActor;
use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use log::{info}; // 导入日志宏
use std::sync::Arc;
use crate::config::{loader, cfg}; // 确保导入了必要的模块

// 使用 OnceCell 在 globals 模块中设置 APP_STATE
use once_cell::sync::OnceCell;

pub static APP_STATE: OnceCell<AppState> = OnceCell::new();

pub fn set_app_state(state: AppState) -> Result<(), &'static str> {
    APP_STATE.set(state).map_err(|_| "APP_STATE already set")
}

pub struct AppState {
    pub redis_conn: Addr<RedisActor>,
    pub mysql_conn: Arc<DatabaseConnection>,
}

pub static APP_CONFIG: Lazy<cfg::AppConfig> = Lazy::new(|| {
    info!("load config success!");
    loader::with_config("config/dev.toml").unwrap()
});


impl AppState {
    pub fn new(redis_conn: Addr<RedisActor>, mysql_conn: Arc<DatabaseConnection>) -> Self {
        Self { redis_conn, mysql_conn }
    }
}
