//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_user_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub create_user: String,
    pub create_time: Option<DateTimeUtc>,
    pub update_user: Option<String>,
    pub update_time: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_role::Entity",
        from = "Column::RoleId",
        to = "super::sys_role::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    SysRole,
    #[sea_orm(
        belongs_to = "super::sys_user::Entity",
        from = "Column::UserId",
        to = "super::sys_user::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    SysUser,
}

impl Related<super::sys_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRole.def()
    }
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
