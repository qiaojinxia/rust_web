//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub role_code: String,
    #[sea_orm(unique)]
    pub role_name: String,
    pub description: Option<String>,
    pub status: i8,
    pub create_user: String,
    pub create_time: Option<DateTimeUtc>,
    pub update_user: Option<String>,
    pub update_time: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_role_permission::Entity")]
    SysRolePermission,
    #[sea_orm(has_many = "super::sys_user_role::Entity")]
    SysUserRole,
}

impl Related<super::sys_role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRolePermission.def()
    }
}

impl Related<super::sys_user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysUserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
