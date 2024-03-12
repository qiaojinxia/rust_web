use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ColumnTrait, QueryFilter, JoinType, QuerySelect, RelationTrait};
use sea_orm::ActiveValue::Set;
use crate::schema::admin::{sys_user, sys_user_role};
use crate::schema::admin::prelude::{SysUserRole};


//assign_roles_to_user，用于给用户分配角色
pub async fn assign_roles_to_user(
    db: &DatabaseConnection,
    user_id: i32,
    role_ids: Vec<i32>,
    create_user: String,
) -> Result<Vec<sys_user_role::Model>, DbErr> {
    let user_roles: Vec<sys_user_role::ActiveModel> = role_ids
        .into_iter()
        .map(|role_id| sys_user_role::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            create_user: Set(create_user.clone()),
            ..Default::default()
        })
        .collect();

    // Perform the insert operation
    sys_user_role::Entity::insert_many(user_roles)
        .exec(db)
        .await?;

    // Assuming you have some way to fetch the inserted records,
    // you might perform a query to get them or use the returning clause if your database supports it.
    // This is a hypothetical example and will depend on your schema and ORM's capabilities:
    let assigned_roles = sys_user_role::Entity::find()
        .filter(sys_user_role::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(assigned_roles)
}


//get_user_roles 获取用户的角色
pub async fn get_user_roles(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<sys_user_role::Model>, DbErr> {
    SysUserRole::find()
        // 假设您已经在 sys_user_role 和 sys_user 之间定义了正确的关系
        .filter(sys_user::Column::Id.eq(user_id))
        .select_only()
        .join(
            JoinType::InnerJoin,
            sys_user_role::Relation::SysUser.def(),
        )
        .columns([sys_user_role::Column::Id,sys_user_role::Column::RoleId,
            sys_user_role::Column::UserId,sys_user_role::Column::CreateUser])
        .into_model::<sys_user_role::Model>()
        .all(db)
        .await
}

//remove_role_from_user 删除用户的角色
pub async fn remove_role_from_user(
    db: &DatabaseConnection,
    user_id: i32,
    role_id: i32,
) -> Result<u64, DbErr> {
    SysUserRole::delete_many()
        .filter(sys_user_role::Column::UserId.eq(user_id))
        .filter(sys_user_role::Column::RoleId.eq(role_id))
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}