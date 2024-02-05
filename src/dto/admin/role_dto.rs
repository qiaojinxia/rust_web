use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RoleCreationDto {
    #[validate(length(min = 1, max = 16))]
    pub role_id: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub role_name: Option<String>,
    pub description: Option<String>,
    pub status:i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDto {
    pub id: Option<i32>,
    pub role_code: Option<String>,
    pub role_name: Option<String>,
    pub description: Option<String>,
    pub status: i8,
}

// 用于创建操作的响应结构体，复用了BaseRoleResponseDto
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleCreationResponseDto {
    #[serde(flatten)]
    pub base: RoleDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesResponseDto {
    pub list: Vec<RoleDto>, // 使用 RoleDto 而不是特定于创建操作的 DTO
}
