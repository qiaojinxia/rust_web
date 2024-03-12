use actix::MailboxError;
use crate::config::cfg;
use actix_redis::{RedisActor, RespValue, Error, Command, resp_array};
use actix::prelude::Addr;
use actix_redis::Error::Redis;
use actix_redis::RespError::{Remote, Unexpected};


// 假设你想要这个函数只在 crate 内部可见，你可以使用 `pub(crate)` 修饰符
pub async fn configure_redis(cfg: &cfg::RedisConfig) -> Result<Addr<RedisActor>, Error> {
    let redis_url = format!("{}:{}", cfg.host, cfg.port);
    let connection = RedisActor::start(redis_url);

    // 如果存在密码，则进行认证
    if let Some(password) = &cfg.password {
        if let Err(e) = authenticate(&connection, password).await {
            return Err(e);
        }
    }

    // 发送PING命令以检查连接
    let result = connection.send(Command(resp_array!["PING"])).await;
    if let Err(e) = check_ping(result) {
        return Err(e);
    }
    // 连接成功
    Ok(connection)
}

async fn authenticate(connection: &Addr<RedisActor>, password: &str) -> Result<(), Error> {
    match connection.send(Command(resp_array!["AUTH", password])).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(err)) => Err(err),
        Err(send_err) => {
            Err(Redis(Remote(send_err.to_string())))
        },
    }
}

fn check_ping(result: Result<Result<RespValue, Error>, MailboxError>) -> Result<(), Error> {
    match result {
        Ok(Ok(RespValue::SimpleString(s))) if s == "PONG" => Ok(()),
        Ok(Ok(response)) => Err(Redis(Unexpected(format!("{:?}", response)))) ,
        Ok(Err(err)) => Err(err),
        Err(send_err) => {
            Err(Redis(Remote(send_err.to_string())))
        },
    }
}