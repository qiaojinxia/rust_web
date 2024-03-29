use chrono::Utc;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ColumnTrait, QueryFilter, JoinType, QuerySelect, RelationTrait};
use sea_orm::ActiveValue::Set;
use crate::common::auth::jwt::MenuInfo;
use crate::schemas::admin::{sys_menu, sys_permission, sys_role, sys_role_permission};
use crate::schemas::admin::prelude::{SysRolePermission};


//assign_permissions_to_role 为角色分配权限
pub async fn assign_permissions_to_role(
    db: &DatabaseConnection,
    role_id: i32,
    permission_ids: Vec<i32>,
    create_user: String,
) -> Result<Vec<sys_role_permission::Model>, DbErr> {
    // Prepare the role_permissions data for insertion
    let role_permissions: Vec<sys_role_permission::ActiveModel> = permission_ids
        .into_iter()
        .map(|permission_id| sys_role_permission::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(permission_id),
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
        .filter(sys_role_permission::Column::RoleId.eq(role_id))
        .all(db)
        .await?;

    Ok(inserted_permissions)
}


//get_role_permissions 获取角色的权限
pub async fn get_role_permissions(
    db: &DatabaseConnection,
    role_id: i32,
) -> Result<Vec<sys_role_permission::Model>, DbErr> {
    SysRolePermission::find()
        .filter(sys_role::Column::Id.eq(role_id))
        .select_only()
        .join(
            JoinType::InnerJoin,
            sys_role_permission::Relation::SysRole.def(),
        )
        .columns([sys_role_permission::Column::Id,sys_role_permission::Column::PermissionId,
            sys_role_permission::Column::RoleId, sys_role_permission::Column::CreateUser])
        .into_model::<sys_role_permission::Model>()
        .all(db)
        .await
}

//remove_permission_from_role 删除角色的权限

pub async fn remove_permission_from_role(
    db: &DatabaseConnection,
    role_id: i32,
    permission_id: i32,
) -> Result<u64, DbErr> {
    // 使用 delete_many 方法并结合过滤条件来删除记录
    SysRolePermission::delete_many()
        .filter(sys_role_permission::Column::RoleId.eq(role_id))
        .filter(sys_role_permission::Column::PermissionId.eq(permission_id))
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}



pub async fn get_menus_by_role_id(
    db: &DatabaseConnection,
    role_ids: Vec<i32>,
) -> Result<Vec<MenuInfo>, DbErr> {
    let menus = SysRolePermission::find()
        .filter(sys_role_permission::Column::RoleId.is_in(role_ids))
        .join(
            JoinType::InnerJoin,
            sys_role_permission::Relation::SysPermission.def(),
        )

        .select_only()
        .column_as(sys_menu::Column::Id, "id")
        .column_as(sys_menu::Column::MenuName, "menu_name")
        .column_as(sys_menu::Column::RoutePath, "route_path")
        .column_as(sys_menu::Column::RouteName, "route_name")
        .into_model::<MenuInfo>() // 确保 MenuInfo 匹配你想要的字段
        .all(db)
        .await;

    menus
}
