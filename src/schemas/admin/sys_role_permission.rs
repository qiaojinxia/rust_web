//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_role_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role_id: i32,
    pub permission_id: i32,
    pub create_user: String,
    pub create_time: Option<DateTimeUtc>,
    pub update_user: Option<String>,
    pub update_time: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_permission::Entity",
        from = "Column::PermissionId",
        to = "super::sys_permission::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    SysPermission,
    #[sea_orm(
        belongs_to = "super::sys_role::Entity",
        from = "Column::RoleId",
        to = "super::sys_role::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    SysRole,
}

impl Related<super::sys_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysPermission.def()
    }
}

impl Related<super::sys_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}