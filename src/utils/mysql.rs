use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};
use std::fs;

// 函数来创建数据库连接池
pub async fn create_db_pool(database_url: &str) -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new()
        .connect(&database_url)
        .await
}

// 数据库初始化函数
pub async fn initialize_database(sql_url:&str, pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    let init_script = fs::read_to_string(sql_url)
        .expect("Failed to read init_db.sql");

    sqlx::query(&init_script)
        .execute(pool)
        .await?;

    Ok(())
}
