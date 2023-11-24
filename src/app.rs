// 引入所需的模块
// use crate::config::{self, AppConfig};
// use crate::db::{self, DatabasePool};
use crate::utils::logger;
use crate::utils::database;
use crate::config::globals;
use log::{info, error}; // 导入日志宏
use std::sync::Arc;

// 一个初始化数据库连接的函数
pub async fn conn_db() {
    info!("connecting database config url: {}", globals::APP_CONFIG.database.url);
    match database::create_db_pool(&globals::APP_CONFIG).await {
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

// pub async fn create_db(){
//     let db = &match db.get_database_backend() {
//        DbBackend::MySql => {
//            db.execute(Statement::from_string(
//                db.get_database_backend(),
//                format!("CREATE DATABASE IF NOT EXISTS `{}`;", globals::APP_CONFIG.database.name),
//            ))
//            .await?;

//            let url = format!("{}/{}", DATABASE_URL, globals::APP_CONFIG.database.name);
//            Database::connect(&url).await?
//        }
// }

// 初始化日志系统
pub fn init_logger() {
    // 初始化日志逻辑
    logger::init();
}

// 一个总的初始化函数，用于集中调用所有初始化逻辑
pub async fn init() {
    init_logger();
    conn_db().await;
}
