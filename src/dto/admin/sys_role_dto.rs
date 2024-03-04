use serde::{Deserialize, Serialize};
use validator_derive::Validate;

/// Role DTO
/// Method: General
/// Description: Used for general role information.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDto {
    pub id: Option<i32>,
    pub role_code: Option<String>,
    pub role_name: Option<String>,
    pub description: Option<String>,
    pub status: i8,
}

/// Role Creation Request DTO
/// Method: POST
/// API: /roles
/// Description: DTO for creating a role with field validations.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RoleCreationDto {
    #[validate(length(min = 1, max = 16, message = "role_code must be between 1 and 16 characters long"))]
    pub role_code: Option<String>,
    #[validate(length(min = 1, max = 64, message = "role_name must be between 1 and 64 characters long"))]
    pub role_name: Option<String>,
    #[validate(length(min = 1, max = 512, message = "description must be between 1 and 512 characters long"))]
    pub description: Option<String>,
    pub status: i8,
}

/// Role Creation Response DTO
/// Method: POST
/// API: /roles
/// Description: DTO for the response after creating a role.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleCreationResponseDto {
    #[serde(flatten)]
    pub base: RoleDto,
}

/// Roles List Response DTO
/// Method: GET
/// API: /roles
/// Description: DTO for the response containing a list of roles.
#[derive(Debug, Serialize, Deserialize)]
pub struct RolesResponseDto {
    pub list: Vec<RoleDto>,
}

/// Single Role Response DTO
/// Method: GET
/// API: /roles/{id}
/// Description: DTO for the response containing information of a single role.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponseDto {
    #[serde(flatten)]
    pub role: Option<RoleDto>,
}

/// Role Update Request DTO
/// Method: PUT
/// API: /roles/{id}
/// Description: DTO for updating role information with field validations.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RoleUpdateDto {
    #[validate(length(min = 1, max = 16, message = "role_code must be between 1 and 16 characters long"))]
    pub role_code: Option<String>,
    #[validate(length(min = 1, max = 64, message = "role_name must be between 1 and 64 characters long"))]
    pub role_name: Option<String>,
    #[validate(length(min = 1, max = 512, message = "description must be between 1 and 512 characters long"))]
    pub description: Option<String>,
    pub status: Option<i8>,
}

/// Role Update Response DTO
/// Method: PUT
/// API: /roles/{id}
/// Description: DTO for the response after updating role information.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleUpdateRespDto {
    #[serde(flatten)]
    pub role: Option<RoleDto>,
}


/// Role Deletion Response DTO
/// Method: DELETE
/// API: /roles/{id}
/// Description: DTO for the response after deleting a role.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDeleteRespDto {
    pub role_id: Option<i8>,
}
