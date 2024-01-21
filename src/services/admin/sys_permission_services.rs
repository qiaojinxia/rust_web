use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::schema::admin::{sys_permission};
use crate::schema::admin::prelude::{SysPermission};

//create_permission 创建权限
pub async fn create_permission(
    db: &DatabaseConnection,
    permission_name: String,
    description: String,
    // ... 其他必要的字段
) -> Result<sys_permission::Model, DbErr> {
    let permission = sys_permission::ActiveModel {
        permission_name: Set(permission_name),
        description: Set(Some(description)),
        // ... 设置其他字段
        ..Default::default()
    };
    permission.insert(db).await
}

//get_permissions 获取权限列表
pub async fn get_permissions(
    db: &DatabaseConnection,
) -> Result<Vec<sys_permission::Model>, DbErr> {
    SysPermission::find().all(db).await
}

//get_permission_by_id 获取单个权限
pub async fn get_permission_by_id(
    db: &DatabaseConnection,
    permission_id: i32,
) -> Result<Option<sys_permission::Model>, DbErr> {
    SysPermission::find_by_id(permission_id).one(db).await
}

//update_permission 更新权限
pub async fn update_permission(
    db: &DatabaseConnection,
    permission_id: i32,
    permission_name: Option<String>,
    description: Option<String>,
    // ... 其他可选字段
) -> Result<Option<sys_permission::Model>, DbErr> {
    let mut permission: sys_permission::ActiveModel = SysPermission::find_by_id(permission_id).one(db).await?.unwrap().into();

    if let Some(pn) = permission_name {
        permission.permission_name = Set(pn);
    }
    if let Some(dsc) = description {
        permission.description = Set(Some(dsc));
    }
    // ... 更新其他字段

    permission.update(db).await.map(Some)
}

//delete_permission 删除权限
pub async fn delete_permission(
    db: &DatabaseConnection,
    permission_id: i32,
) -> Result<u64, DbErr> {
    let permission = sys_permission::ActiveModel {
        id: Set(permission_id),
        ..Default::default()
    };
    SysPermission::delete(permission)
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}
