// 引入所需的模块
use crate::common::{log as logger};
use crate::common::db;
use crate::config::globals;
use std::sync::Arc;
use log::{error, info};

// 一个初始化数据库连接的函数
pub async fn conn_db() {
    info!("connecting database config url: {}", globals::APP_CONFIG.database.url);
    match db::config::init(&globals::APP_CONFIG).await {
        Ok(pool) => {
            info!("Database connection established successfully.");
            let db_pool = Arc::new(pool);
            globals::DB_POOL.set(db_pool).expect("Failed to set DB_POOL");
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1); // 使用非零退出码表示错误
        }
    }
}

// 初始化日志系统
pub fn init_logger() {
    // 初始化日志逻辑
    logger::config::init();
}

// 一个总的初始化函数，用于集中调用所有初始化逻辑
pub async fn init() {
    init_logger();
    conn_db().await;
}
