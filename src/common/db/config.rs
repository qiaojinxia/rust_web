use sea_orm::{error::DbErr, ConnectOptions, DatabaseConnection, Database };
use std::time::Duration;
use crate::config::structs;
use log::LevelFilter;

// 函数来创建数据库连接池
pub async fn init(config:&structs::AppConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(config.database.url.clone());
    opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(config.database.connect_timeout))
        .acquire_timeout(Duration::from_secs(config.database.acquire_timeout))
        .idle_timeout(Duration::from_secs(config.database.idle_timeout))
        .max_lifetime(Duration::from_secs(config.database.max_lifetime))
        .sqlx_logging(config.database.sqlx_logging)
        .sqlx_logging_level(LevelFilter::Info); // 假设你想使用信息级别的日志

    Database::connect(opt).await
}
