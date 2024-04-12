use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, TransactionTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use crate::schemas::admin::{sys_menu, sys_permission, sys_permission_action, sys_role_permission};
use crate::schemas::admin::prelude::{SysPermission, SysRolePermission};
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use sea_orm::sea_query::{Alias, Query};
use crate::dto::admin::sys_permission_dto::PermissionMenuDto;

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

pub async fn get_total_permissions_count(
    db: &DatabaseConnection,
) -> Result<i64, DbErr> {
    let mut query = Query::select();

    query.from(sys_permission::Entity)
        .expr_as(
            Expr::cust("COUNT(DISTINCT id)"),
            Alias::new("total_count"),
        );

    let builder = db.get_database_backend();
    let stmt = builder.build(&query);
    // Execute the query
    let result = db.query_one(stmt).await?;

    // Extract the count directly from the result
    if let Some(row) = result {
        // Try to get the "total_count" column from the row
        let total_count: i64 = row.try_get_by("total_count").unwrap_or(0);
        Ok(total_count)
    } else {
        Err(DbErr::Custom("Failed to fetch total count".into()))
    }
}

pub async fn get_paginated_permissions_with_menus(
    db: &DatabaseConnection,
    current: usize,
    size: usize,
) -> Result<Vec<PermissionMenuDto>, DbErr> {
    let offset = (current.saturating_sub(1)) * size;

    let mut query = Query::select();
    query.columns(vec![
            (sys_permission::Entity, sys_permission::Column::Id),
            (sys_permission::Entity, sys_permission::Column::PermissionCode),
            (sys_permission::Entity, sys_permission::Column::Description),
        ])
        .column((sys_menu::Entity, sys_menu::Column::MenuName))
        .column((sys_menu::Entity, sys_menu::Column::Id))
        .column((sys_menu::Entity, sys_menu::Column::Status))
        .expr_as(
            Expr::cust("GROUP_CONCAT(DISTINCT sys_permission_action.action_code SEPARATOR ',')"),
            Alias::new("actions"),
        ) // Custom expression for actions
        .from(sys_permission::Entity)
        .left_join(
            sys_permission_action::Entity,
            Expr::col((sys_permission::Entity, sys_permission::Column::Id))
                .equals((sys_permission_action::Entity, sys_permission_action::Column::PermissionId)),
        )
        // .left_join(
        //     sys_menu_permission::Entity,
        //     Expr::col((sys_permission::Entity, sys_permission::Column::Id))
        //         .equals((sys_menu_permission::Entity, sys_menu_permission::Column::PermissionId)),
        // )
        // .left_join(
        //     sys_menu::Entity,
        //     Expr::col((sys_menu_permission::Entity, sys_menu_permission::Column::MenuId))
        //         .equals((sys_menu::Entity, sys_menu::Column::Id)),
        // )
        .group_by_col((sys_permission::Entity, sys_permission::Column::Id))
        .limit(size as u64)
        .offset(offset as u64);

    let builder = db.get_database_backend();
    let stmt = builder.build(&query);
    let rows = db.query_all(stmt).await?;
    let result = rows.iter().map(|row| {
        // Parse each field from the row. Ensure you handle potential errors or missing values appropriately.
        let permission_id: i32 = row.try_get_by("permission_id").unwrap_or_default();
        let permission_code: String = row.try_get_by("permission_code").unwrap_or_default();
        let description: String = row.try_get_by("description").unwrap_or_default();
        let actions: String = row.try_get_by("actions").unwrap_or_default();
        let menu_name: String = row.try_get_by("menu_name").unwrap_or_default();
        let menu_id: i32 = row.try_get_by("menu_id").unwrap_or_default();
        let menu_status: String = row.try_get_by("menu_status").unwrap_or_default();

        // Split the actions string into a Vec<String>, assuming they are separated by commas
        let action_codes: Vec<String> = actions.split(',').map(String::from).collect();

        // Construct your DTO
        PermissionMenuDto {
            permission_id,
            permission_code,
            description,
            actions: action_codes,
            menu_name,
            menu_id,
            menu_status,
        }
    }).collect();

    Ok(result)
}

pub async fn get_permissions_with_menus(
    db: &DatabaseConnection,
    current: usize,
    page_size: usize,
) -> Result<(Vec<PermissionMenuDto>, i64), DbErr> {
    // Fetch total count of permissions
    let total_count = get_total_permissions_count(db).await?;

    // Fetch paginated permissions with menus
    let permissions = get_paginated_permissions_with_menus(db, current, page_size).await?;

    Ok((permissions, total_count))
}
