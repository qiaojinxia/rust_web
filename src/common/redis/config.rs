use crate::config::cfg;
use actix_redis::{RedisActor, Error, Command};
use actix::prelude::Addr;
use redis_async::resp::RespValue;
use redis_async::resp_array;

// 假设你想要这个函数只在 crate 内部可见，你可以使用 `pub(crate)` 修饰符
pub async fn configure_redis(config: &cfg::RedisConfig) -> Result<Addr<RedisActor>, Error> {
    let redis_url = if let Some(password) = &config.password {
        if !password.is_empty() {
            // 如果密码存在且不为空，则包含密码部分
            format!("redis://{}@{}:{}/{}", password, config.host, config.port, config.db)
        } else {
            // 密码为空，省略密码部分
            format!("redis://{}:{}/{}", config.host, config.port, config.db)
        }
    } else {
        // 没有密码，省略密码部分
        format!("redis://{}:{}/{}", config.host, config.port, config.db)
    };

    let redis_addr = RedisActor::start(redis_url);

    // 发送PING命令以检查连接
    let result = redis_addr.send(Command(resp_array!["PING"])).await;

    match result {
        Ok(Ok(RespValue::SimpleString(s))) if s == "PONG" => {
            // 连接成功
            Ok(redis_addr)
        },
        Ok(Ok(_)) | Ok(Err(_)) => {
            // 收到了响应，但不是预期的PONG，或者命令执行出错
            Err(Error::Redis(redis_async::error::Error::Unexpected( "Unexpected Redis response or command error".to_string())))
        },
        Err(_) => {
            // 发送命令时出错（可能是连接问题）
            Err(Error::Disconnected)
        }
    }
}