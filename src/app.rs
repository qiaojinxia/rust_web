// 引入所需的模块
use crate::common::{log as logger};
use crate::common::redis;
use crate::common::db;
use crate::config::globals;
use std::sync::Arc;
use actix::Addr;
use actix_redis::{Error, RedisActor};
use log::{error, info};

use sea_orm::{DatabaseConnection, DbErr};
use crate::config::cfg::{AppConfig, DatabaseConfig, RedisConfig};

// 一个初始化数据库连接的函数
pub async fn conn_db(db_cfg: &DatabaseConfig) -> Result<Arc<DatabaseConnection>, DbErr> {
    info!("Connecting to database with config url: {}", db_cfg.url);
    match db::config::init(db_cfg).await {
        Ok(pool) => {
            info!("Database connection established successfully.");
            Ok(Arc::new(pool)) // 成功时返回 Ok 包裹的 Arc<DatabaseConnection>
        },
        Err(e) => {
            info!("Failed to connect to the database: {}", e);
            Err(e) // 错误时返回 Err 包裹的 DbErr
        }
    }
}

pub async fn init_redis(cfg: &RedisConfig) ->  Result<Addr<RedisActor>, Error>{
    redis::config::configure_redis(cfg).await
}

// 一个总的初始化函数，用于集中调用所有初始化逻辑
pub async fn init() {
    // 初始化日志系统
    logger::config::init();

    // 初始化 Redis
    let redis_conn = match init_redis(&globals::APP_CONFIG.redis).await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Redis initialization failed: {}", e);
            std::process::exit(1);
        }
    };

    // 初始化数据库
    let mysql_conn = match conn_db(&globals::APP_CONFIG.database).await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection failed: {}", e);
            std::process::exit(1);
        }
    };

    // 初始化全局 AppState
    let app_state = globals::AppState::new(redis_conn, mysql_conn);

    globals::set_app_state(app_state).expect("Failed to set global APP_STATE");

}
