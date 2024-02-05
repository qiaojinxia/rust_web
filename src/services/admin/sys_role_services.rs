use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::schema::admin::{sys_role};
use crate::schema::admin::prelude::{SysRole};

//create_role 创建角色
pub async fn create_role(
    db: &DatabaseConnection,
    role_name: String,
    role_id: String,
    description: String,
) -> Result<sys_role::Model, DbErr> {
    let role = sys_role::ActiveModel {
        role_name: Set(role_name),
        description: Set(Some(description)),
        role_code:Set(role_id),
        ..Default::default()
    };
    role.insert(db).await
}

//get_roles 获取角色列表
pub async fn get_roles(
    db: &DatabaseConnection,
) -> Result<Vec<sys_role::Model>, DbErr> {
    SysRole::find().all(db).await
}

//get_role_by_id 获取单个角色
pub async fn get_role_by_id(
    db: &DatabaseConnection,
    role_id: i32,
) -> Result<Option<sys_role::Model>, DbErr> {
    SysRole::find_by_id(role_id).one(db).await
}

//update_role 更新角色
pub async fn update_role(
    db: &DatabaseConnection,
    role_id: i32,
    role_name: Option<String>,
    description: Option<String>,
    // ... 其他可选字段
) -> Result<Option<sys_role::Model>, DbErr> {
    let mut role: sys_role::ActiveModel = SysRole::find_by_id(role_id).one(db).await?.unwrap().into();

    if let Some(rn) = role_name {
        role.role_name = Set(rn);
    }
    if let Some(dsc) = description {
        role.description = Set(Some(dsc));
    }
    // ... 更新其他字段

    role.update(db).await.map(Some)
}

//delete_role 删除角色
pub async fn delete_role(
    db: &DatabaseConnection,
    role_id: i32,
) -> Result<u64, DbErr> {
    let role = sys_role::ActiveModel {
        id: Set(role_id),
        ..Default::default()
    };
    SysRole::delete(role)
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}
