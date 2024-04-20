//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub permission_code: String,
    pub description: Option<String>,
    pub create_user: String,
    pub create_time: Option<DateTimeUtc>,
    pub update_user: Option<String>,
    pub update_time: Option<DateTimeUtc>,
    pub status: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_permission_action::Entity")]
    SysPermissionAction,
    #[sea_orm(has_many = "super::sys_permission_target::Entity")]
    SysPermissionTarget,
    #[sea_orm(has_many = "super::sys_role_permission::Entity")]
    SysRolePermission,
}

impl Related<super::sys_permission_action::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysPermissionAction.def()
    }
}

impl Related<super::sys_permission_target::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysPermissionTarget.def()
    }
}

impl Related<super::sys_role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRolePermission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
