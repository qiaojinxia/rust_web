use crate::schemas::admin::prelude::SysUserRole;
use crate::schemas::admin::sys_user_role;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

// assign_roles_to_user assigns roles to a user
pub async fn assign_roles_to_user(
    db: &DatabaseConnection,
    user_id: i32,
    role_codes: Vec<String>,
    create_user: String,
) -> Result<Vec<sys_user_role::Model>, DbErr> {
    let user_roles: Vec<sys_user_role::ActiveModel> = role_codes
        .into_iter()
        .map(|role_code| sys_user_role::ActiveModel {
            user_id: Set(user_id),
            role_code: Set(role_code),
            create_user: Set(create_user.clone()),
            ..Default::default()
        })
        .collect();

    // Perform the insert operation
    sys_user_role::Entity::insert_many(user_roles)
        .exec(db)
        .await?;

    // Retrieve the assigned roles
    let assigned_roles = sys_user_role::Entity::find()
        .filter(sys_user_role::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(assigned_roles)
}

// get_user_roles retrieves the roles assigned to a user
pub async fn get_user_roles(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<sys_user_role::Model>, DbErr> {
    SysUserRole::find()
        .filter(sys_user_role::Column::UserId.eq(user_id))
        .all(db)
        .await
}

// remove_role_from_user removes a specific role from a user
pub async fn remove_role_from_user(
    db: &DatabaseConnection,
    user_id: i32,
    role_code: String,
) -> Result<u64, DbErr> {
    SysUserRole::delete_many()
        .filter(sys_user_role::Column::UserId.eq(user_id))
        .filter(sys_user_role::Column::RoleCode.eq(role_code))
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}
