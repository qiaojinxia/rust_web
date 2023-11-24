use once_cell::sync::Lazy;
use crate::config::loader;
use crate::config::structs;
use log::{info}; // 导入日志宏
use sea_orm::{ DatabaseConnection };
use once_cell::sync::OnceCell;
use std::sync::Arc;

pub static APP_CONFIG: Lazy<structs::AppConfig> = Lazy::new(|| {
    info!("load config success!");
    loader::with_config("config/dev.toml").unwrap()
});

pub static DB_POOL: OnceCell<Arc<DatabaseConnection>> = OnceCell::new();
