
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    // 可以添加更多的配置字段
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub init_sql: String,
    // 其他数据库相关配置
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    // 其他服务器相关配置
}
