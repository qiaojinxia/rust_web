use crate::dto::admin::common_dto::validate_status;
use crate::schemas::admin::sys_role;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleDto {
    pub id: Option<i32>,
    pub role_code: Option<String>,
    pub role_name: Option<String>,
    pub permission_ids: Option<Vec<i32>>,
    pub role_desc: Option<String>,
    pub status: String,
}

impl From<sys_role::Model> for RoleDto {
    fn from(model: sys_role::Model) -> Self {
        RoleDto {
            id: Some(model.id),
            role_code: Some(model.role_code),
            role_name: Some(model.role_name),
            permission_ids: None,
            role_desc: model.description,
            status: model.status.to_string(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoleCreationDto {
    #[validate(length(
        min = 1,
        max = 16,
        message = "role_code must be between 3 and 16 characters long"
    ))]
    pub role_code: String,
    #[validate(length(
        min = 1,
        max = 64,
        message = "role_name must be between 3 and 64 characters long"
    ))]
    pub role_name: String,
    #[validate(length(
        min = 1,
        max = 512,
        message = "description must be between 1 and 512 characters long"
    ))]
    pub role_desc: String,
    pub permission_ids: Option<Vec<i32>>,
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleCreationResponseDto {
    #[serde(flatten)]
    pub base: RoleDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesResponseDto {
    pub list: Vec<RoleDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponseDto {
    #[serde(flatten)]
    pub role: Option<RoleDto>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoleUpdateDto {
    #[validate(length(
        min = 1,
        max = 16,
        message = "role_code must be between 1 and 16 characters long"
    ))]
    pub role_code: Option<String>,
    #[validate(length(
        min = 1,
        max = 64,
        message = "role_name must be between 1 and 64 characters long"
    ))]
    pub role_name: Option<String>,
    #[validate(length(
        min = 1,
        max = 512,
        message = "description must be between 1 and 512 characters long"
    ))]
    pub permission_ids: Option<Vec<i32>>,
    pub role_desc: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleUpdateRespDto {
    #[serde(flatten)]
    pub role: Option<RoleDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDeleteRespDto {
    pub role_id: Option<i8>,
}

/// Role Deletion Response DTO
/// Method: DELETE
/// API: /roles
/// Description: DTO for the response after deleting roles.
#[derive(Debug, Serialize, Deserialize)]
pub struct RolesDeleteRespDto {
    pub deleted_role_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleOptionDto {
    pub id: Option<i32>,
    pub role_code: Option<String>,
    pub role_name: Option<String>,
}

impl From<(i32, String, String)> for RoleOptionDto {
    fn from(tuple: (i32, String, String)) -> Self {
        RoleOptionDto {
            id: Some(tuple.0),
            role_code: Some(tuple.1),
            role_name: Some(tuple.2),
        }
    }
}


#[derive(Serialize, Debug)]
pub struct RouteDto {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub component: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub children: Option<Vec<RouteDto>>
}

#[derive(Serialize, Debug)]
pub struct RoleMenuResponseDto {
    pub home: String,
    pub routes: Vec<RouteDto>,
}
