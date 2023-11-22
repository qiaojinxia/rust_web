use once_cell::sync::Lazy;
use crate::config::loader;
use crate::config::structs;
use log::{info}; // 导入日志宏
use std::sync::{Arc, Mutex};
use sqlx::MySqlPool;

pub static APP_CONFIG: Lazy<structs::AppConfig> = Lazy::new(|| {
    info!("load config success!");
    loader::with_config("config/dev.toml").unwrap()
});

// 全局静态变量用于存储数据库连接池
pub static GLOBAL_DB_POOL: Lazy<Arc<Mutex<Option<MySqlPool>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

