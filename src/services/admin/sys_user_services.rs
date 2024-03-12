use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use crate::schema::admin::{sys_user};
use crate::schema::admin::prelude::{SysUser};
use crate::schema::admin::sea_orm_active_enums::Gender;

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

//get_users 获取用户列表
pub async fn get_users(
    db: &DatabaseConnection,
) -> Result<Vec<sys_user::Model>, DbErr> {
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
