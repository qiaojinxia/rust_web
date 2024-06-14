use crate::common::auth;
use crate::dto::admin::sys_user_dto::{UserCreateDto, UserWithRolesDto};
use crate::schemas::admin::prelude::SysUser;
use crate::schemas::admin::sea_orm_active_enums::Gender;
use crate::schemas::admin::{sys_role, sys_user, sys_user_role};
use sea_orm::sea_query::{Alias, Expr, Query};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait,
    FromQueryResult, QueryFilter, QuerySelect,
};

//create_user 创建用户
pub async fn create_user(
    db: &DatabaseConnection,
    user_create_req: UserCreateDto,
    create_user: String,
) -> Result<sys_user::Model, DbErr> {
    let password_hash = auth::crypto::hash_password(Some(user_create_req.password)).unwrap(); // 假设这是一个外部函数，用于安全地散列密码
    let gender = user_create_req
        .user_gender
        .parse::<Gender>()
        .map_err(|_| DbErr::Custom("Invalid gender".to_string()))?;
    let user = sys_user::ActiveModel {
        user_name: Set(user_create_req.user_name),
        password: Set(password_hash),
        nick_name: Set(user_create_req.nick_name),
        email: Set(user_create_req.user_email),
        mobile: Set(Some(user_create_req.user_phone)),
        status: Set(user_create_req.status.parse().unwrap()),
        gender: Set(gender),
        create_user: Set(create_user),
        ..Default::default()
    };
    user.insert(db).await
}

// 定义一个表示自定义函数的结构体
pub async fn get_users_with_roles(
    db: &DatabaseConnection,
    current: usize,
    size: usize,
) -> Result<Vec<UserWithRolesDto>, DbErr> {
    let offset = (current.saturating_sub(1)) * size;
    let mut query = Query::select();
    query
        .columns(vec![
            (sys_user::Entity, sys_user::Column::Id),
            (sys_user::Entity, sys_user::Column::UserName),
            (sys_user::Entity, sys_user::Column::NickName),
            (sys_user::Entity, sys_user::Column::Email),
            (sys_user::Entity, sys_user::Column::Mobile),
            (sys_user::Entity, sys_user::Column::Gender),
            (sys_user::Entity, sys_user::Column::Status),
            (sys_user::Entity, sys_user::Column::CreateUser),
            (sys_user::Entity, sys_user::Column::CreateTime),
            (sys_user::Entity, sys_user::Column::UpdateUser),
            (sys_user::Entity, sys_user::Column::UpdateTime),
        ])
        .column((sys_role::Entity, sys_role::Column::Id))
        .expr_as(
            Expr::cust("GROUP_CONCAT(DISTINCT sys_role.role_code SEPARATOR ',')"),
            Alias::new("role_codes"),
        ) // Use expression with alias
        .from(sys_user::Entity)
        .left_join(
            sys_user_role::Entity,
            Expr::col((sys_user::Entity, sys_user::Column::Id))
                .equals((sys_user_role::Entity, sys_user_role::Column::UserId)),
        )
        .left_join(
            sys_role::Entity,
            Expr::col((sys_user_role::Entity, sys_user_role::Column::RoleCode))
                .equals((sys_role::Entity, sys_role::Column::RoleCode)),
        )
        .group_by_col((sys_user::Entity, sys_user::Column::Id)) // Group by to use aggregate function
        .limit(size as u64) // Pagination
        .offset(offset as u64);

    let builder = db.get_database_backend();
    let stmt = builder.build(&query); // 构建查询语句
    let rows = db.query_all(stmt).await?;
    let result: Vec<UserWithRolesDto> = rows
        .iter()
        .map(|row| {
            let user_id = row.try_get_by("id").unwrap_or_default();
            let user_name = row.try_get_by("user_name").unwrap_or_default();
            let nick_name = row.try_get_by("nick_name").unwrap_or_default();
            let user_email = row.try_get_by("email").unwrap_or_default();
            let user_phone = row.try_get_by("mobile").unwrap_or_default();
            let user_gender: String = row.try_get_by("gender").unwrap_or_default();
            let status: i32 = row.try_get_by("status").unwrap_or_default();
            let create_by = row.try_get_by("create_user").unwrap_or_default();
            let create_time: chrono::NaiveDateTime =
                row.try_get_by("create_time").unwrap_or_default();
            let update_by = row.try_get_by("update_user").unwrap_or_default();
            let update_time: chrono::NaiveDateTime =
                row.try_get_by("update_time").unwrap_or_default();
            let role_codes: String = row.try_get_by("role_codes").unwrap_or_default();
            let user_roles: Result<Vec<String>, _> = role_codes
                .split(',')
                .map(|code| code.trim().parse())
                .collect();
            UserWithRolesDto {
                id: user_id,
                user_name,
                nick_name,
                user_email,
                user_phone,
                user_gender,
                status: status.to_string(),
                create_by,
                create_time: format!("{}", create_time.format("%Y-%m-%d %H:%M:%S")),
                update_by,
                update_time: format!("{}", update_time.format("%Y-%m-%d %H:%M:%S")),
                user_roles: Some(user_roles.unwrap_or(vec![])),
            }
        })
        .collect();

    Ok(result)
}

//get_users 获取用户列表
pub async fn get_users(db: &DatabaseConnection) -> Result<Vec<sys_user::Model>, DbErr> {
    SysUser::find().all(db).await
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
    let mut user: sys_user::ActiveModel =
        SysUser::find_by_id(user_id).one(db).await?.unwrap().into();

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
pub async fn delete_user(db: &DatabaseConnection, user_id: i32) -> Result<u64, DbErr> {
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
    let query = SysUser::find()
        .filter(sys_user::Column::Email.eq(email))
        .filter(sys_user::Column::Mobile.eq(mobile));
    query.one(db).await
}

//find_user_by_username 根据用户名查找用户
pub async fn find_user_by_username(
    db: &DatabaseConnection,
    user_name: Option<String>,
) -> Result<Option<sys_user::Model>, DbErr> {
    let query = SysUser::find().filter(sys_user::Column::UserName.eq(user_name));
    query.one(db).await
}

#[derive(FromQueryResult)]
struct TotalCount {
    total_count: i32,
}

pub async fn get_total_users_count(db: &DatabaseConnection) -> Result<i32, DbErr> {
    let total = SysUser::find()
        .select_only()
        .column_as(sys_user::Column::Id.count(), "total_count")
        .into_model::<TotalCount>()
        .one(db)
        .await?
        .map(|total_count_model| total_count_model.total_count)
        .unwrap_or(0); // 如果没有结果，则默认为0

    Ok(total)
}
