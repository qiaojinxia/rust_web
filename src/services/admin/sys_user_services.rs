use std::fmt::Write;
use actix_web::web::BufMut;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, Iden, JoinType, PaginatorTrait, QueryFilter, RelationTrait, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Alias, Expr, MysqlQueryBuilder, Query};
use crate::schemas::admin::{sys_role, sys_role_permission, sys_user, sys_user_role};
use crate::schemas::admin::prelude::{SysUser};
use crate::schemas::admin::sea_orm_active_enums::Gender;

//create_user 创建用户
pub async fn create_user(
    db: &DatabaseConnection,
    user_name: Option<String>,
    password: String,
    email: Option<String>,
    gender: Gender,
    mobile: Option<String>,
    create_user: Option<String>,
    update_user: Option<String>,
) -> Result<sys_user::Model, DbErr> {

    let user = sys_user::ActiveModel {
        user_name: Set(user_name.unwrap()),
        password: Set(password),
        email: Set(email.unwrap()),
        gender: Set(gender),
        mobile: Set(mobile),
        create_user: Set(create_user.unwrap()),
        update_user: Set(update_user),
        // ... 设置其他字段
        ..Default::default()
    };
    user.insert(db).await
}
// 定义一个表示自定义函数的结构体

// pub async fn get_users_with_roles(
//     db: &DatabaseConnection,
//     current: usize,
//     size: usize,
// ) -> Result<Vec<(i32, String, String, String)>, DbErr> {
//     let offset = (current.saturating_sub(1)) * size;
//
//     let mut query = Query::select();
//     query.columns(vec![
//         sys_user::Column::Id.into(),
//         sys_user::Column::UserName.into(),
//         // sys_user::Column::NickName.into(),
//         sys_user::Column::Email.into(),
//         sys_user::Column::Mobile.into(),
//         sys_user::Column::Gender.into(),
//         // sys_user::Column::Status.into(),
//         sys_user::Column::CreateUser.into(),
//         sys_user::Column::CreateTime.into(),
//         sys_user::Column::UpdateUser.into(),
//         sys_user::Column::UpdateTime.into(),
//     ])
//         .expr_as(
//             Expr::cust("GROUP_CONCAT(DISTINCT sys_role.role_code SEPARATOR ',')"),
//             Alias::new("role_codes"),
//         ) // 使用表达式与别名
//         .from(sys_user::Entity)
//         .inner_join(
//             sys_user_role::Entity,
//             Expr::col((sys_user::Entity, sys_user::Column::Id))
//                 .equals((sys_user_role::Entity, sys_user_role::Column::UserId)),
//         )
//         .inner_join(
//             sys_role::Entity,
//             Expr::col((sys_user_role::Entity, sys_user_role::Column::RoleId))
//                 .equals((sys_role::Entity, sys_role::Column::Id)),
//         )
//         .group_by_col(sys_user::Column::Id) // 分组以便使用聚合函数
//         .limit(size as u64) // 分页
//         .offset(offset as u64);
//
//     let builder = db.get_database_backend();
//     let stmt = builder.build(&query); // 构建查询语句
//     let rows = db.query_all(stmt).await?;
//
//     let result: Vec<(i32, String, String, String)> = rows.iter().map(|row| {
//         let id = row.try_get_by("id").unwrap_or_default();
//         let user_name = row.try_get_by("user_name").unwrap_or_default();
//         let email = row.try_get_by("email").unwrap_or_default();
//         let role_codes = row.try_get_by("role_codes").unwrap_or_default();
//         (id, user_name, email, role_codes)
//     }).collect();
//
//     Ok(result)
// }

//get_users 获取用户列表
pub async fn get_users(
    db: &DatabaseConnection,
) -> Result<Vec<sys_user::Model>, DbErr> {
    SysUser::find().all(db).await
}

// 修改后的get_users函数，添加分页参数current和size
pub async fn get_users_paginated(
    db: &DatabaseConnection,
    current: u64, // 当前页码
    size: u64,    // 每页数量
) -> Result<Vec<sys_user::Model>, DbErr> {
    // 使用SysUser::find()开始构建查询
    let paginator = SysUser::find()
        .paginate(db, size); // 分页器，指定每页数量

    let result = paginator.
        fetch_page(current - 1).await; // 获取指定页的数据，页码从0开始，所以这里用current - 1
    result
}

//get_user_by_id 获取单个用户
pub async fn get_user_by_id(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<sys_user::Model>, DbErr> {
    SysUser::find_by_id(user_id).one(db).await
}

//update_user 更新用户
pub async fn update_user(
    db: &DatabaseConnection,
    user_id: i32,
    user_name: Option<String>,
    password: Option<String>,
    email: Option<String>,
    gender: Option<Gender>,
    mobile: Option<String>,
    // ... 其他可选字段
) -> Result<Option<sys_user::Model>, DbErr> {
    let mut user: sys_user::ActiveModel = SysUser::find_by_id(user_id).one(db).await?.unwrap().into();

    if let Some(un) = user_name {
        user.user_name = Set(un);
    }
    if let Some(pwd) = password {
        user.password = Set(pwd);
    }
    if let Some(em) = email {
        user.email = Set(em);
    }
    if let Some(gen) = gender {
        user.gender = Set(gen);
    }
    if let Some(mb) = mobile {
        user.mobile = Set(Some(mb));
    }
    user.update(db).await.map(Some)
}

//delete_user 删除用户
pub async fn delete_user(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<u64, DbErr> {
    let user = sys_user::ActiveModel {
        id: Set(user_id),
        ..Default::default()
    };
    SysUser::delete(user)
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}

//get_user_by_id_pure 根据用户ID获取用户信息
pub async fn get_user_by_id_pure(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<sys_user::Model>, DbErr> {
    // 使用 SeaORM 的查询方法
    SysUser::find_by_id(user_id).one(db).await
}

//find_user_by_email_or_mobile 根据邮箱或手机号查询用户
pub async fn find_user_by_email_or_mobile(
    db: &DatabaseConnection,
    email: Option<String>,
    mobile: Option<String>,
) -> Result<Option<sys_user::Model>, DbErr> {
    let  query = SysUser::find()
        .filter(sys_user::Column::Email.eq(email)).filter(sys_user::Column::Mobile.eq(mobile));
    query.one(db).await
}

//find_user_by_username 根据用户名查找用户
pub async fn find_user_by_username(
    db: &DatabaseConnection,
    user_name: Option<String>,
) -> Result<Option<sys_user::Model>, DbErr> {
    let  query = SysUser::find()
        .filter(sys_user::Column::UserName.eq(user_name));
    query.one(db).await
}
