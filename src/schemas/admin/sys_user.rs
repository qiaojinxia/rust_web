//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use super::sea_orm_active_enums::Gender;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    #[sea_orm(unique)]
    pub email: String,
    pub gender: Gender,
    #[sea_orm(unique)]
    pub mobile: Option<String>,
    pub avatar: Option<String>,
    pub create_user: String,
    pub create_time: Option<DateTimeUtc>,
    pub update_user: Option<String>,
    pub update_time: Option<DateTimeUtc>,
    pub last_login: Option<DateTimeUtc>,
    #[sea_orm(column_name = "status_")]
    pub status: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_user_role::Entity")]
    SysUserRole,
}

impl Related<super::sys_user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysUserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
