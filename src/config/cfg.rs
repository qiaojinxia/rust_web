use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AppConfig {
    ///数据库配置
    pub database: DatabaseConfig,
    ///服务器配置
    pub server: ServerConfig,
    ///redis配置
    pub redis: RedisConfig,
    ///jwt配置
    pub jwt: JwtConfig,
}

/// DatabaseConfig 包含数据库连接池的配置参数。
#[derive(Deserialize)]
pub struct DatabaseConfig {
    /// 数据库的连接URL。
    pub url: String,

    /// 启动时运行的 SQL 初始化命令。
    pub init_sql: String,

    /// 连接池的最大连接数。
    pub max_connections: u32,

    /// 连接池的最小连接数。
    pub min_connections: u32,

    /// 建立新连接的超时时间（秒）。
    pub connect_timeout: u64,

    /// 从连接池获取连接的超时时间（秒）。
    pub acquire_timeout: u64,

    /// 连接在被视为闲置之前可以保持空闲状态的时间（秒）。
    pub idle_timeout: u64,

    /// 连接的最长生命周期（秒）。
    pub max_lifetime: u64,

    /// 是否启用 SQLx 的日志记录。
    pub sqlx_logging: bool,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    /// 服务器主机地址
    pub host: String,

    /// 服务器端口号
    pub port: u16,
}

// 定义 Redis 配置结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    /// Redis 服务器的主机地址
    pub host: String,

    /// Redis 服务器的端口号
    pub port: u16,

    /// Redis 服务器的密码（可选）
    pub password: Option<String>,

    /// 默认数据库编号
    pub db: i32,
}

// 定义 Jwt 配置结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expire_time: u64,
}
