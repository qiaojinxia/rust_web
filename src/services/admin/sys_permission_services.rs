use crate::common::error::MyError;
use crate::dto::admin::sys_permission_dto::{ApiDetail, PermissionCreationDto, PermissionDetailsDto, PermissionDto};
use crate::schemas::admin::prelude::{SysPermission, SysRolePermission};
use crate::schemas::admin::{
    sea_orm_active_enums, sys_api, sys_menu, sys_permission, sys_permission_action,
    sys_permission_target, sys_role_permission,
};
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


//create_permission 创建权限
pub async fn create_permission(
    db: &DatabaseConnection,
    permission_creation_dto: PermissionCreationDto,
    create_user: String,
) -> Result<sys_permission::Model, MyError> {
    let transaction = db.begin().await?;
    let permission = sys_permission::ActiveModel {
        permission_name: Set(permission_creation_dto.permission_name),
        permission_code: Set(permission_creation_dto.permission_code),
        description: Set(permission_creation_dto.description),
        create_user: Set(create_user),
        create_time: Set(Some(Utc::now())),
        status: Set(permission_creation_dto.status.parse().unwrap()),
        ..Default::default()
    };
    let inserted_permission = permission.insert(&transaction).await?;

    insert_permission_targets_for_menus(&transaction, inserted_permission.id, permission_creation_dto.menus).await?;
    insert_permission_action_codes(&transaction, inserted_permission.id, permission_creation_dto.action_codes).await?;

    transaction.commit().await?;
    Ok(inserted_permission)
}



async fn insert_permission_targets_for_menus(
    transaction: &DatabaseTransaction,
    permission_id: i32,
    menu_ids: Option<Vec<i32>>,
) -> Result<(), MyError> {
    if let Some(menu_ids) = menu_ids {
        let menu_targets: Vec<sys_permission_target::ActiveModel> = menu_ids
            .into_iter()
            .map(|menu_id| sys_permission_target::ActiveModel {
                permission_id: Set(permission_id),
                target_id: Set(menu_id),
                target_type: Set(sea_orm_active_enums::TargetType::Menu),
                ..Default::default()
            })
            .collect();

        sys_permission_target::Entity::insert_many(menu_targets)
            .exec(transaction)
            .await?;
    }
    Ok(())
}

async fn insert_permission_action_codes(
    transaction: &DatabaseTransaction,
    permission_id: i32,
    action_codes: Option<Vec<String>>,
) -> Result<(), MyError> {
    if let Some(action_codes) = action_codes {
        let mut action_targets = Vec::new();
        for action_code in action_codes {
            match sea_orm_active_enums::ActionCode::from_string(action_code.as_str()) {
                Ok(parsed_code) => {
                    action_targets.push(sys_permission_action::ActiveModel {
                        permission_id: Set(permission_id),
                        action_code: Set(parsed_code),
                        ..Default::default()
                    });
                }
                Err(e) => return Err(MyError::ConversionError(e.to_string())), // Handle the conversion error appropriately
            }
        }

        sys_permission_action::Entity::insert_many(action_targets)
            .exec(transaction)
            .await?;
    }
    Ok(())
}


//get_permission_by_id 获取单个权限
pub async fn get_permission_by_id(
    db: &DatabaseConnection,
    permission_id: i32,
) -> Result<Option<sys_permission::Model>, DbErr> {
    SysPermission::find_by_id(permission_id).one(db).await
}


//get_permission_by_id 获取所有权限
pub async fn get_permissions(
    db: &DatabaseConnection,
) -> Result<Vec<sys_permission::Model>, DbErr> {
// 使用`find_all`方法获取所有权限记录
    let permissions = sys_permission::Entity::find()
        .all(db)
        .await?;
    Ok(permissions)
}


pub async fn update_permission(
    db: &DatabaseConnection,
    permission_id: i32,
    permission_update_dto: PermissionDto,
    update_user: String,
) -> Result<(), MyError> {
    let transaction = db.begin().await?;

    let permission = SysPermission::find_by_id(permission_id)
        .one(db)
        .await?
        .ok_or(MyError::NotFound("db select error".to_string()))?;

    let mut is_modified = false;

    let mut active_permission: sys_permission::ActiveModel = permission.into();
    if let Some(permission_name) = permission_update_dto.permission_name {
        active_permission.permission_name = Set(permission_name);
        is_modified = true;
    }
    if let Some(permission_code) = permission_update_dto.permission_code {
        active_permission.permission_code = Set(permission_code);
        is_modified = true;
    }
    if let Some(description) = permission_update_dto.description {
        active_permission.description = Set(Some(description));
        is_modified = true;
    }
    if let Some(status) = permission_update_dto.status {
        active_permission.status = Set(status.parse().unwrap());
        is_modified = true;
    }

    if is_modified {
        active_permission.update_user = Set(Some(update_user));
        active_permission.update_time = Set(Some(Utc::now()));
        active_permission.update(&transaction).await?;
    }

    if let Some(_) = permission_update_dto.menus {
        delete_permission_targets(&transaction, permission_id,sea_orm_active_enums::TargetType::Menu).await?;
        insert_permission_targets_for_menus(&transaction, permission_id, permission_update_dto.menus).await?;
    }

    if let Some(ref actions_codes) = permission_update_dto.action_codes {
        if !actions_codes.is_empty() {
            delete_permission_action_codes(&transaction, permission_id).await?;
            insert_permission_action_codes(&transaction, permission_id, permission_update_dto.action_codes).await?;
        }
    }

    transaction.commit().await?;
    Ok(())
}


//delete_permission_targets 删除关联菜单
async fn delete_permission_targets(
    transaction: &DatabaseTransaction,
    permission_id: i32,
    target_type: sea_orm_active_enums::TargetType,
) -> Result<(), MyError> {
    sys_permission_target::Entity::delete_many()
        .filter(sys_permission_target::Column::PermissionId.eq(permission_id))
        .filter(sys_permission_target::Column::TargetType.eq(target_type))
        .exec(transaction)
        .await?;

    Ok(())
}

//delete_permission_action_targets 删除关联操作权限
async fn delete_permission_action_codes(
    transaction: &DatabaseTransaction,
    permission_id: i32,
) -> Result<(), MyError> {
    sys_permission_action::Entity::delete_many()
        .filter(sys_permission_action::Column::PermissionId.eq(permission_id))
        .exec(transaction)
        .await?;

    Ok(())
}

//delete_permission 删除权限
pub async fn delete_permission(db: &DatabaseConnection, permission_id: i32) -> Result<u64, DbErr> {
    // 开始一个事务
    let txn = db.begin().await?;

    // 更新所有引用该权限ID作为permission_id的sys_role_permission记录，将它们的permission_id设置为NULL
    let _ = SysRolePermission::update_many()
        .col_expr(
            sys_role_permission::Column::PermissionId,
            Expr::value(None::<i32>),
        )
        .filter(sys_role_permission::Column::PermissionId.eq(permission_id))
        .exec(&txn)
        .await?;

    // 删除关联的 sys_permission_action 记录
    let _ = sys_permission_action::Entity::delete_many()
        .filter(sys_permission_action::Column::PermissionId.eq(permission_id))
        .exec(&txn)
        .await?;

    // 删除关联的 sys_permission_target 记录
    let _ = sys_permission_target::Entity::delete_many()
        .filter(sys_permission_target::Column::PermissionId.eq(permission_id))
        .exec(&txn)
        .await?;

    // 删除目标权限项
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
            Expr::cust("GROUP_CONCAT(DISTINCT sys_menu.id SEPARATOR ',')"),
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
            let action_codes_list = action_codes
                .split(',')
                .filter_map(|s| {
                    let s = s.trim();
                    if !s.is_empty() {
                        let s = sea_orm_active_enums::ActionCode::from_string_origin(s).ok()?;
                        Some(s.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            // Parse the menus and apis fields into Vecs of (name, id)
            let menu_details: Vec<String> = menus
                .split(',')
                .filter_map(|s| s.parse().ok())
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
                action_codes: action_codes_list,
                description,
                menus: menu_details,
                apis: api_details,
                status: status.to_string(),
            }
        })
        .collect();

    Ok(result)
}
