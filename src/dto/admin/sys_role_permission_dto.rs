use serde::{Deserialize, Serialize};
use crate::schemas::admin::sys_role_permission::Model;

// DTO for assigning permissions to a role
#[derive(Debug, Serialize, Deserialize)]
pub struct AssignPermissionsDto {
    pub permission_ids: Vec<i32>,
}

// DTO for representing a single permission of a role
#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionDto {
    pub id: i32,
    pub role_id: i32,
    pub permission_id: i32,
    pub create_user: String,
}

impl From<Model> for RolePermissionDto {
    fn from(model: Model) -> Self {
        RolePermissionDto {
            id: model.id,
            role_id: model.role_id,
            permission_id: model.permission_id,
            create_user: model.create_user,
        }
    }
}

// DTO for response listing a role's permissions
#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionsRespDto {
    pub permissions: Vec<RolePermissionDto>,
}

// DTO for response after assigning permissions to a role or removing a permission from a role
#[derive(Debug, Serialize, Deserialize)]
pub struct OperationSuccessRespDto {
    pub success: bool,
}

// DTO for response after removing a permission from a role
#[derive(Debug, Serialize, Deserialize)]
pub struct RemovePermissionRespDto {
    pub success: bool,
}

// DTO for response after performing an operation (e.g., assigning or removing permissions)
#[derive(Debug, Serialize, Deserialize)]
pub struct OperationResponseDto {
    pub success: bool,
}

// DTO for response after assigning permissions to a role
#[derive(Debug, Serialize, Deserialize)]
pub struct AssignPermissionsRespDto {
    pub success: bool,
}
