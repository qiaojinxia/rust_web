//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub menu_name: String,
    pub permission_id: Option<i32>,
    pub url: String,
    pub sort: Option<i8>,
    pub parent_id: Option<i32>,
    pub redirect: Option<String>,
    pub guards: Option<i8>,
    pub component: Option<String>,
    pub meta: Option<Json>,
    pub hidden: i8,
    pub create_user: String,
    pub create_time: Option<DateTimeUtc>,
    pub update_user: String,
    pub update_time: Option<DateTimeUtc>,
    pub is_visible: i8,
    pub is_deleted: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    SelfRef,
    #[sea_orm(
        belongs_to = "super::sys_permission::Entity",
        from = "Column::PermissionId",
        to = "super::sys_permission::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    SysPermission,
}

impl Related<super::sys_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysPermission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
