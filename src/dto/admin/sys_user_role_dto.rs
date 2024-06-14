use crate::schemas::admin::sys_user_role::Model;
use serde::{Deserialize, Serialize};
use validator::Validate;

// DTO for assigning roles to a user
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AssignRolesDto {
    pub role_codes: Vec<String>, // Updated to use role codes instead of role IDs
}

// DTO for user role information
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleDto {
    pub id: i32,
    pub user_id: i32,
    pub role_code: String, // Updated to use role code
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
    pub roles: Vec<UserRoleDto>, // Adjusted to list roles with role codes
}

impl From<Model> for UserRoleDto {
    fn from(model: Model) -> Self {
        UserRoleDto {
            id: model.id,
            user_id: model.user_id,
            role_code: model.role_code, // Updated to use role code
            create_user: model.create_user,
        }
    }
}

// DTO for response after removing a role from a user
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveRoleRespDto {
    pub success: bool,
}
