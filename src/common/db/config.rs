use crate::config::cfg;
use log::LevelFilter;
use sea_orm::{error::DbErr, ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

// 函数来创建数据库连接池
pub async fn init(db_config: &cfg::DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(db_config.url.clone());
    opt.max_connections(db_config.max_connections)
        .min_connections(db_config.min_connections)
        .connect_timeout(Duration::from_secs(db_config.connect_timeout))
        .acquire_timeout(Duration::from_secs(db_config.acquire_timeout))
        .idle_timeout(Duration::from_secs(db_config.idle_timeout))
        .max_lifetime(Duration::from_secs(db_config.max_lifetime))
        .sqlx_logging(db_config.sqlx_logging)
        .sqlx_logging_level(LevelFilter::Info); // 假设你想使用信息级别的日志

    Database::connect(opt).await
}
