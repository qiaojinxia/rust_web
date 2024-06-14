use crate::schemas::admin::sys_role_permission::Model;
use serde::{Deserialize, Serialize};

// DTO for assigning permissions to a role
#[derive(Debug, Serialize, Deserialize)]
pub struct AssignPermissionsDto {
    pub permission_codes: Vec<String>,
}

// DTO for representing a single permission of a role
#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionDto {
    pub id: i32,
    pub role_code: String,
    pub permission_code: String,
    pub create_user: String,
}

impl From<Model> for RolePermissionDto {
    fn from(model: Model) -> Self {
        RolePermissionDto {
            id: model.id,
            role_code: model.role_code,
            permission_code: model.permission_code,
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
