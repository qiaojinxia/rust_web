use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ColumnTrait, QueryFilter, JoinType, QuerySelect, RelationTrait};
use sea_orm::ActiveValue::Set;
use crate::schema::admin::{sys_role, sys_role_permission};
use crate::schema::admin::prelude::{SysRolePermission};

//assign_permissions_to_role 为角色分配权限
pub async fn assign_permissions_to_role(
    db: &DatabaseConnection,
    role_id: i32,
    permission_ids: Vec<i32>,
) -> Result<Vec<sys_role_permission::Model>, DbErr> {
    // Prepare the role_permissions data for insertion
    let role_permissions: Vec<sys_role_permission::ActiveModel> = permission_ids
        .into_iter()
        .map(|permission_id| sys_role_permission::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(permission_id),
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
        .join(
            JoinType::InnerJoin,
            sys_role::Relation::SysRolePermission.def(),
        )
        .filter(sys_role::Column::Id.eq(role_id))
        .select_only()
        .column(sys_role_permission::Column::PermissionId)
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
    let role_permission = sys_role_permission::ActiveModel {
        role_id: Set(role_id),
        permission_id: Set(permission_id),
        ..Default::default()
    };
    SysRolePermission::delete(role_permission)
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}
