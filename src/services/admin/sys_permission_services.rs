use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, TransactionTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use crate::schemas::admin::{sys_menu, sys_permission, sys_role_permission};
use crate::schemas::admin::prelude::{SysMenu, SysPermission, SysRolePermission};
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

//create_permission 创建权限
pub async fn create_permission(
    db: &DatabaseConnection,
    permission_code: String,
    description: String,
    create_user: String,
) -> Result<sys_permission::Model, DbErr> {
    let permission = sys_permission::ActiveModel {
        permission_code: Set(permission_code),
        description: Set(Some(description)),
        create_user: Set(create_user),
        create_time: Set(Some(Utc::now())),
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
    permission_code: Option<String>,
    description: Option<String>,
    update_user: String,
) -> Result<Option<sys_permission::Model>, DbErr> {
    let mut permission: sys_permission::ActiveModel = SysPermission::find_by_id(permission_id).one(db).await?.unwrap().into();

    if let Some(p_code) = permission_code {
        permission.permission_code = Set(p_code);
    }
    if let Some(dsc) = description {
        permission.description = Set(Some(dsc));
    }

    permission.update_user = Set(Some(update_user));

    permission.update(db).await.map(Some)
}

//delete_permission 删除权限
pub async fn delete_permission(
    db: &DatabaseConnection,
    permission_id: i32,
) -> Result<u64, DbErr> {
    // 开始一个事务
    let txn = db.begin().await?;


    // 接着，更新所有引用该权限ID作为permission_id的sys_role_permission记录，
    // 将它们的permission_id设置为NULL
    let _ = SysRolePermission::update_many()
        .col_expr(sys_role_permission::Column::PermissionId, Expr::value(None::<i32>))
        .filter(sys_role_permission::Column::PermissionId.eq(permission_id))
        .exec(&txn)
        .await?;

    // 然后，尝试删除目标权限项
    let permission = sys_permission::ActiveModel {
        id: Set(permission_id),
        ..Default::default()
    };

    let rows_affected = SysPermission::delete(permission)
        .exec(&txn)
        .await?
        .rows_affected;

    // 提交事务
    txn.commit().await?;

    Ok(rows_affected)
}
