use actix::prelude::*;
use actix_redis::{Command, Error, RedisActor, resp_array};
use actix_redis::RespError::Unexpected;
use actix_redis::RespValue;

struct RedisOps;

impl RedisOps {
    /// 设置键值对
    #[allow(dead_code)]
    async fn set(addr: &Addr<RedisActor>, key: &str, value: &str) -> Result<String, Error> {
        let res = addr
            .send(Command(resp_array!["SET", key, value]))
            .await
            .map_err(|e|Error::Redis(Unexpected(e.to_string())))?;

        match res {
            Ok(RespValue::SimpleString(s)) if s == "OK" => Ok(s),
            Ok(RespValue::Error(e)) => Err(Error::Redis(Unexpected(e))),
            _ => Err(Error::Redis(Unexpected("Unexpected response from Redis".to_string()))),
        }
    }

    /// 获取键的值
    #[allow(dead_code)]
    async fn get(addr: &Addr<RedisActor>, key: &str) -> Result<Option<String>, actix_redis::Error> {
        let res = addr
            .send(Command(resp_array!["GET", key]))
            .await
            .map_err(|e| Error::Redis(Unexpected(e.to_string())))?; // 转换MailboxError到actix_redis::Error

        match res {
            Ok(RespValue::BulkString(bytes)) => Ok(Some(String::from_utf8_lossy(&bytes).to_string())),
            Ok(RespValue::Nil) => Ok(None),
            _ => Ok(None),
        }
    }

    #[allow(dead_code)]
    async fn del(addr: &Addr<RedisActor>, key: &str) -> Result<usize, actix_redis::Error> {
        let res = addr
            .send(Command(resp_array!["DEL", key]))
            .await
            .map_err(|e| Error::Redis(Unexpected(e.to_string())))?; // 处理MailboxError

        match res {
            Ok(RespValue::Integer(count)) => Ok(count as usize),
            _ => Err(Error::Redis(Unexpected("Unexpected response type".to_string()))),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use actix::Actor;
    use crate::app;
    use crate::config::globals;

    #[actix_rt::test]
    async fn test_set_and_del() {
        // 启动Redis Actor

        app::init().await;
        let app_state = globals::APP_STATE.get().unwrap();
        let addr = &app_state.redis_conn;

        // 测试设置键值对
        let key = "test_key";
        let value = "test_value";
        match RedisOps::set(addr, key, value).await {
            Ok(response) => assert_eq!(response, "OK", "Expected OK response from SET command"),
            Err(e) => panic!("SET operation failed with error: {:?}", e),
        }
        // 测试获取刚设置的键值对以确认其存在
        let get_result = RedisOps::get(addr, key).await;
        match get_result {
            Ok(Some(val)) => assert_eq!(val, value, "Value mismatch for the key"),
            _ => panic!("Failed to get the value for the key"),
        }

        // 测试删除键
        let del_result =  RedisOps::del(addr, key).await;
        assert!(del_result.is_ok(), "Failed to delete key");
        let deleted_count = del_result.unwrap();
        assert_eq!(deleted_count, 1, "The key was not deleted");

        // 确认键已被删除
        let get_after_del_result =  RedisOps::get(addr, key).await;
        assert!(get_after_del_result.is_ok(), "Error when trying to confirm deletion");
        assert!(get_after_del_result.unwrap().is_none(), "Key still exists after deletion");
    }
}
