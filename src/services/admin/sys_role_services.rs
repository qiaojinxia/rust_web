use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::dto;
use crate::dto::admin::sys_role_dto;
use crate::schema::admin::{sys_role};
use crate::schema::admin::prelude::{SysRole};
use sea_orm::QueryFilter;
//create_role 创建角色
pub async fn create_role(
    db: &DatabaseConnection,
    create_user: String,
    role_create_info: sys_role_dto::RoleCreationDto
) -> Result<sys_role::Model, DbErr> {
    let mut role = sys_role::ActiveModel {
        ..Default::default()
    };

    if let Some(rn) = role_create_info.role_name {
        role.role_name = Set(rn);
    }
    if let Some(dsc) = role_create_info.role_desc {
        role.description = Set(Some(dsc));
    }
    if let Some(code) = role_create_info.role_code {
        role.role_code = Set(code);
    }

    role.status = Set(role_create_info.status);

    role.create_user = Set(create_user);

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
    role_update_info: dto::admin::sys_role_dto::RoleUpdateDto
) -> Result<Option<sys_role::Model>, DbErr> {
    let mut role: sys_role::ActiveModel = SysRole::find_by_id(role_id).one(db).await?.unwrap().into();

    if let Some(rn) = role_update_info.role_name {
        role.role_name = Set(rn);
    }
    if let Some(dsc) = role_update_info.description {
        role.description = Set(Some(dsc));
    }
    if let Some(code) = role_update_info.role_code {
        role.role_code = Set(code);
    }
    if let Some(status) = role_update_info.status {
        role.status = Set(status);
    }
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

// 根据 role_code 数组返回所有匹配的 id
pub async fn get_role_ids_by_role_codes(
    db: &DatabaseConnection,
    role_codes: Vec<String>, // role_code 数组
) -> Result<Vec<i32>, DbErr> { // 假设 id 类型为 i32
    let roles = SysRole::find()
        .filter(sys_role::Column::RoleCode.is_in(role_codes)) // 使用 is_in 方法来过滤 role_code
        .all(db)
        .await?;

    let ids: Vec<i32> = roles.into_iter()
        .map(|role| role.id) // 假设 sys_role::Model 有一个 id 字段
        .collect();

    Ok(ids)
}