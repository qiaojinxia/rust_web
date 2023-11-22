// 引入所需的模块
// use crate::config::{self, AppConfig};
// use crate::db::{self, DatabasePool};
use crate::utils::logger;
use crate::utils::mysql;
use crate::config::globals;
use log::{info, error}; // 导入日志宏


// 一个初始化数据库连接的函数
pub async fn init_db() {
    let database_url = globals::APP_CONFIG.database.url.as_str();
    info!("connecting database config url: {}", database_url);
    match mysql::create_db_pool(database_url).await {
        Ok(pool) => {
            info!("Database connection established successfully.");
            let mut global_pool = globals::GLOBAL_DB_POOL.lock().unwrap();
            *global_pool = Some(pool);
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
    logger::init();
}

// 一个总的初始化函数，用于集中调用所有初始化逻辑
pub async fn init() {
    init_logger();
    init_db().await;
}
