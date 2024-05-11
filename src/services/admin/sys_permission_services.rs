use crate::common::error::MyError;
use crate::dto::admin::sys_permission_dto::{ApiDetail, PermissionCreationDto, PermissionDetailsDto};
use crate::schemas::admin::prelude::{SysPermission, SysRolePermission};
use crate::schemas::admin::{sea_orm_active_enums, sys_api, sys_menu, sys_permission, sys_permission_action, sys_permission_target, sys_role_permission};
use chrono::Utc;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, Query};
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    TransactionTrait,
};

async fn insert_permission_target(
    transaction: &DatabaseTransaction,
    permission_id: i32,
    target_id: i32,
    target_type: sea_orm_active_enums::TargetType,
) -> Result<(), MyError> {
    let permission_target = sys_permission_target::ActiveModel {
        permission_id: Set(permission_id),
        target_id: Set(target_id),
        target_type: Set(target_type),
        ..Default::default()
    };
    permission_target.insert(transaction).await?;
    Ok(())
}

async fn insert_permission_action_target(
    transaction: &DatabaseTransaction,
    permission_id: i32,
    action_code: String,
) -> Result<(), MyError> {
    let permission_target =  sys_permission_action::ActiveModel {
        permission_id: Set(permission_id),
        action_code: Set(sea_orm_active_enums::ActionCode::from_string(action_code.as_str())?), // 假设ActionCode是一个枚举
        ..Default::default()
    };
    permission_target.insert(transaction).await?;
    Ok(())
}


//create_permission 创建权限
pub async fn create_permission(
    db: &DatabaseConnection,
    permission_creation_dto: PermissionCreationDto,
    create_user: String,

) -> Result<sys_permission::Model, MyError> {
    let transaction = db.begin().await?;
    let permission = sys_permission::ActiveModel {
        permission_name:Set(permission_creation_dto.permission_name),
        permission_code: Set(permission_creation_dto.permission_code),
        description: Set(permission_creation_dto.description),
        create_user: Set(create_user),
        create_time: Set(Some(Utc::now())),
        status: Set(permission_creation_dto.status.parse().unwrap()),
        ..Default::default()
    };
    let inserted_permission = permission.insert(&transaction).await?;

    // Insert permission targets for menus
    if let Some(menu_ids) = permission_creation_dto.menus {
        for menu_id in menu_ids {
            insert_permission_target(
                &transaction,
                inserted_permission.id,
                menu_id,
                sea_orm_active_enums::TargetType::Menu,
            ).await?;
        }
    }

    // Insert permission targets for menus
    if let Some(action_codes) = permission_creation_dto.action_codes {
        for action_code in action_codes {
            insert_permission_action_target(
                &transaction,
                inserted_permission.id,
                action_code,
            ).await?;
        }
    }

    transaction.commit().await?;
    Ok(inserted_permission)
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
    let mut permission: sys_permission::ActiveModel = SysPermission::find_by_id(permission_id)
        .one(db)
        .await?
        .unwrap()
        .into();

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
pub async fn delete_permission(db: &DatabaseConnection, permission_id: i32) -> Result<u64, DbErr> {
    // 开始一个事务
    let txn = db.begin().await?;

    // 接着，更新所有引用该权限ID作为permission_id的sys_role_permission记录，
    // 将它们的permission_id设置为NULL
    let _ = SysRolePermission::update_many()
        .col_expr(
            sys_role_permission::Column::PermissionId,
            Expr::value(None::<i32>),
        )
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

pub async fn get_total_permissions_count(db: &DatabaseConnection) -> Result<i64, DbErr> {
    let mut query = Query::select();

    query
        .from(sys_permission::Entity)
        .expr_as(Expr::cust("COUNT(DISTINCT id)"), Alias::new("total_count"));

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

pub async fn get_paginated_permissions_with_menus_apis(
    db: &DatabaseConnection,
    current: usize,
    size: usize,
) -> Result<Vec<PermissionDetailsDto>, MyError> {
    let offset = (current.saturating_sub(1)) * size;

    let mut query = Query::select();
    query.columns(vec![
        (sys_permission::Entity, sys_permission::Column::Id),
        (sys_permission::Entity, sys_permission::Column::PermissionName),
        (sys_permission::Entity, sys_permission::Column::PermissionCode),
        (sys_permission::Entity, sys_permission::Column::Description),
        (sys_permission::Entity, sys_permission::Column::Status),
    ])
        .expr_as(
            Expr::cust("GROUP_CONCAT(DISTINCT CASE WHEN target_type = 'MENU' THEN CONCAT(sys_menu.menu_name, ':', sys_menu.id) END SEPARATOR ',')"),
            Alias::new("menus")
        )
        .expr_as(
            Expr::cust("GROUP_CONCAT(DISTINCT CASE WHEN target_type = 'API' THEN CONCAT(sys_api.api_name, ':', sys_api.id) END SEPARATOR ',')"),
            Alias::new("apis")
        ).expr_as(
            Expr::cust("GROUP_CONCAT(DISTINCT sys_permission_action.action_code SEPARATOR ',')"),
            Alias::new("action_codes")
         )
        .from(sys_permission::Entity)
        .left_join(
            sys_permission_target::Entity,
            Expr::col((sys_permission::Entity, sys_permission::Column::Id))
                .equals((sys_permission_target::Entity, sys_permission_target::Column::PermissionId)),
        )
        .left_join(
            sys_menu::Entity,
            Expr::col((sys_permission_target::Entity, sys_permission_target::Column::TargetId))
                .equals((sys_menu::Entity, sys_menu::Column::Id))
                .and(Expr::col((sys_permission_target::Entity, sys_permission_target::Column::TargetType))
                    .eq(sea_orm_active_enums::TargetType::Menu)),
        )
        .left_join(
            sys_api::Entity,
            Expr::col((sys_permission_target::Entity, sys_permission_target::Column::TargetId))
                .equals((sys_api::Entity, sys_api::Column::Id))
                .and(Expr::col((sys_permission_target::Entity, sys_permission_target::Column::TargetType))
                    .eq(sea_orm_active_enums::TargetType::ApiGroup)),
        ).left_join(
        sys_permission_action::Entity,
        Expr::col((sys_permission::Entity, sys_permission::Column::Id))
            .equals((sys_permission_action::Entity, sys_permission_action::Column::PermissionId)),
         )
        .group_by_col((sys_permission::Entity, sys_permission::Column::Id))
        .limit(size as u64)
        .offset(offset as u64);

    let builder = db.get_database_backend();
    let stmt = builder.build(&query);
    let rows = db.query_all(stmt).await?;
    let result = rows
        .iter()
        .map(|row| {
            let permission_id: i32 = row.try_get_by("id").unwrap_or_default();
            let status: i32 = row.try_get_by("status").unwrap_or_default();
            let permission_name: String = row.try_get_by("permission_name").unwrap_or_default();
            let permission_code: String = row.try_get_by("permission_code").unwrap_or_default();
            let description: String = row.try_get_by("description").unwrap_or_default();
            let menus: String = row.try_get_by("menus").unwrap_or_default();
            let apis: String = row.try_get_by("apis").unwrap_or_default();
            let action_codes: String = row.try_get_by("action_codes").unwrap_or_default();
            let action_codes_list = action_codes.split(',')
                .filter_map(|s| {
                    let s = s.trim();
                    if !s.is_empty() {
                        let s =  sea_orm_active_enums::ActionCode::from_string_origin(s).ok()?;
                        Some(s.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            // Parse the menus and apis fields into Vecs of (name, id)
            let menu_details: Vec<String> = menus
                .split(',')
                .filter_map(|s| {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 2 {
                        Some( parts[1].parse().unwrap_or_default())
                    } else {
                        None
                    }
                })
                .collect();

            let api_details: Vec<ApiDetail> = apis
                .split(',')
                .filter_map(|s| {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 2 {
                        Some(ApiDetail {
                            name: parts[0].to_string(),
                            id: parts[1].parse().unwrap_or_default(),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            // Construct your DTO
            PermissionDetailsDto {
                id: permission_id,
                permission_name,
                permission_code,
                action_codes:action_codes_list,
                description,
                menus: menu_details,
                apis: api_details,
                status: status.to_string(),
            }
        })
        .collect();

    Ok(result)
}
