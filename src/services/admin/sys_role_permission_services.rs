use crate::common::auth::jwt::MenuInfo;
use crate::schemas::admin::prelude::SysRolePermission;
use crate::schemas::admin::{sys_menu, sys_role, sys_role_permission};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};

// Assign permissions to a role
pub async fn assign_permissions_to_role(
    db: &DatabaseConnection,
    role_code: String,
    permission_codes: Vec<String>,
    create_user: String,
) -> Result<Vec<sys_role_permission::Model>, DbErr> {
    // Prepare the role_permissions data for insertion
    let role_permissions: Vec<sys_role_permission::ActiveModel> = permission_codes
        .into_iter()
        .map(|permission_code| sys_role_permission::ActiveModel {
            role_code: Set(role_code.clone()),
            permission_code: Set(permission_code),
            create_user: Set(create_user.clone()),
            create_time: Set(Some(Utc::now())),
            ..Default::default()
        })
        .collect();

    // Perform the bulk insert operation
    sys_role_permission::Entity::insert_many(role_permissions)
        .exec(db)
        .await?;

    // Fetch the inserted records. This needs to be modified based on how your database and ORM handle it.
    let inserted_permissions = sys_role_permission::Entity::find()
        .filter(sys_role_permission::Column::RoleCode.eq(role_code))
        .all(db)
        .await?;

    Ok(inserted_permissions)
}

// Get a role's permissions
pub async fn get_role_permissions(
    db: &DatabaseConnection,
    role_code: String,
) -> Result<Vec<sys_role_permission::Model>, DbErr> {
    SysRolePermission::find()
        .filter(sys_role::Column::RoleCode.eq(role_code))
        .select_only()
        .join(
            JoinType::InnerJoin,
            sys_role_permission::Relation::SysRole.def(),
        )
        .columns([
            sys_role_permission::Column::Id,
            sys_role_permission::Column::PermissionCode,
            sys_role_permission::Column::RoleCode,
            sys_role_permission::Column::CreateUser,
        ])
        .into_model::<sys_role_permission::Model>()
        .all(db)
        .await
}

// Remove a permission from a role
pub async fn remove_permission_from_role(
    db: &DatabaseConnection,
    role_code: String,
    permission_code: String,
) -> Result<u64, DbErr> {
    // Use delete_many method with filtering to delete the records
    SysRolePermission::delete_many()
        .filter(sys_role_permission::Column::RoleCode.eq(role_code))
        .filter(sys_role_permission::Column::PermissionCode.eq(permission_code))
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}

pub async fn get_menus_by_role_codes(
    db: &DatabaseConnection,
    role_codes: Vec<String>,
) -> Result<Vec<MenuInfo>, DbErr> {
    let menus = SysRolePermission::find()
        .filter(sys_role_permission::Column::RoleCode.is_in(role_codes))
        .join(
            JoinType::InnerJoin,
            sys_role_permission::Relation::SysPermission.def(),
        )
        .select_only()
        .column_as(sys_menu::Column::Id, "id")
        .column_as(sys_menu::Column::MenuName, "menu_name")
        .column_as(sys_menu::Column::RoutePath, "route_path")
        .column_as(sys_menu::Column::RouteName, "route_name")
        .into_model::<MenuInfo>() // Ensure MenuInfo matches the desired fields
        .all(db)
        .await;

    menus
}
