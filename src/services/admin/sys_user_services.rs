use crate::common::auth;
use crate::common::error::MyError;
use crate::dto::admin::sys_user_dto::{UserCreateDto, UserUpdateDto, UserWithRolesDto};
use crate::schemas::admin::prelude::SysUser;
use crate::schemas::admin::sea_orm_active_enums::Gender;
use crate::schemas::admin::{sys_user, sys_user_role};
use chrono::{DateTime, Utc};
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult,
    JoinType, QueryFilter, QuerySelect, RelationTrait,
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

#[derive(Debug, FromQueryResult)]
pub struct UserWithRoles {
    pub id: i32,
    pub user_name: String,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub gender: Option<String>,
    pub status: Option<i32>,
    pub create_user: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_user: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub role_codes: Option<String>,
}

pub async fn get_users_with_roles(
    db: &DatabaseConnection,
    current: usize,
    page_size: usize,
) -> Result<Vec<UserWithRolesDto>, DbErr> {
    let offset = (current.saturating_sub(1)) * page_size;
    let users_with_roles: Vec<UserWithRolesDto> = sys_user::Entity::find()
        .select_only()
        .column(sys_user::Column::Id)
        .column(sys_user::Column::UserName)
        .column(sys_user::Column::NickName)
        .column(sys_user::Column::Email)
        .column(sys_user::Column::Mobile)
        .column(sys_user::Column::Gender)
        .column(sys_user::Column::Status)
        .column(sys_user::Column::CreateUser)
        .column(sys_user::Column::CreateTime)
        .column(sys_user::Column::UpdateUser)
        .column(sys_user::Column::UpdateTime)
        .join(JoinType::LeftJoin, sys_user::Relation::SysUserRole.def())
        .join(JoinType::LeftJoin, sys_user_role::Relation::SysRole.def())
        .column_as(
            Expr::cust("GROUP_CONCAT(DISTINCT sys_role.id SEPARATOR ',')"),
            "role_codes",
        )
        .group_by(sys_user::Column::Id)
        .limit(Some(page_size as u64))
        .offset(Some(offset as u64))
        .into_model::<UserWithRoles>()
        .all(db)
        .await? // Execute the query
        .into_iter() // Iterate over the results
        .map(|user| UserWithRolesDto {
            id: user.id,
            user_name: user.user_name,
            nick_name: user.nick_name.unwrap_or("".to_string()),
            user_email: user.email.unwrap_or("".to_string()),
            user_phone: user.mobile.unwrap_or("".to_string()),
            user_gender: user.gender.unwrap_or("1".to_string()),
            status: user.status.unwrap_or(1).to_string(),
            create_by: user.create_user.unwrap_or("".to_string()),
            create_time: user
                .create_time
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            update_by: user.update_user.unwrap_or("".to_string()),
            update_time: user
                .update_time
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            user_roles: user.role_codes.as_ref().map(|codes| {
                codes
                    .split(',')
                    .filter_map(|code| code.trim().parse::<i32>().ok())
                    .collect()
            }),
        })
        .collect();
    Ok(users_with_roles)
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
    update_dto: UserUpdateDto,
    update_user: String,
) -> Result<Option<sys_user::Model>, MyError> {
    let mut user: sys_user::ActiveModel = SysUser::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(MyError::NotFound("User not found".to_string()))?
        .into();

    if let Some(un) = update_dto.user_name {
        user.user_name = Set(un);
    }
    if let Some(nn) = update_dto.nick_name {
        user.nick_name = Set(nn);
    }
    if let Some(pwd) = update_dto.password {
        user.password = Set(auth::crypto::hash_password(Some(pwd))?);
    }
    if let Some(em) = update_dto.user_email {
        user.email = Set(em);
    }
    if let Some(gen) = update_dto
        .user_gender
        .unwrap_or("1".to_string())
        .parse::<Gender>()
        .ok()
    {
        user.gender = Set(gen);
    }
    if let Some(mb) = update_dto.user_phone {
        user.mobile = Set(Some(mb));
    }
    if let Some(st) = update_dto
        .status
        .unwrap_or("1".to_string())
        .parse::<i8>()
        .ok()
    {
        user.status = Set(st);
    }

    user.update_user = Set(Some(update_user));

    Ok(user.update(db).await.map(Some)?)
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

pub async fn batch_delete_users(db: &DatabaseConnection, user_ids: Vec<i32>) -> Result<u64, DbErr> {
    let delete_query = SysUser::delete_many().filter(sys_user::Column::Id.is_in(user_ids));

    let result = delete_query.exec(db).await?;
    Ok(result.rows_affected)
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
