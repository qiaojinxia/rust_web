// 导入 SeaORM 核心特性
use sea_orm::*;
use crate::models::database::users;
use crate::models::database::sea_orm_active_enums;
use crate::models::database::prelude::Users;

// services/user_creation.rs
pub async fn create_user(
    db: &DatabaseConnection,
    user_name: String, 
    password_hash: String,
    gender: Option<sea_orm_active_enums::Gender>,
    email: Option<String>,
) -> Result<users::Model, DbErr> {
    // 创建一个新的激活模型（ActiveModel）
    let user = users::ActiveModel {
        username: Set(user_name),
        gender: Set(gender),
        email: Set(email),
        password_hash: Set(password_hash),
        ..Default::default()
    };
    // 插入记录到数据库并返回结果
    let insert_result = user.insert(db).await?;
    Ok(insert_result)
}

pub async fn find_user(db: &DatabaseConnection, user_id: i32) -> Result<Option<users::Model>, DbErr> {
    // 通过主键查找用户
    let user = Users::find_by_id(user_id).one(db).await?;
    Ok(user)
}