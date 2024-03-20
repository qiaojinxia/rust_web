use serde::{Deserialize, Serialize};
use validator_derive::Validate;
use crate::schema::admin::sys_user_role::Model;

// DTO for assigning roles to a user
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AssignRolesDto {
    pub role_ids: Vec<i32>,
}

// 修改后的DTO，用于列出用户的角色信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleDto {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub create_user: String,
}


// DTO for response after assigning roles to a user
#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRolesRespDto {
    pub success: bool,
}

// DTO for listing a user's roles
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRolesRespDto {
    pub roles: Vec<UserRoleDto>, // Assuming role IDs are sufficient, adjust according to needs
}

impl From<Model> for UserRoleDto {
    fn from(model: Model) -> Self {
        UserRoleDto {
            id: model.id,
            user_id: model.user_id,
            role_id: model.role_id,
            create_user: model.create_user,
        }
    }
}

// DTO for response after removing a role from a user
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveRoleRespDto {
    pub success: bool,
}
