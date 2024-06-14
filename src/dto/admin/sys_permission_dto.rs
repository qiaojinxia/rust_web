use crate::schemas::admin::sys_permission::Model;
use serde::{Deserialize, Serialize};
use validator::Validate;

// DTOs for Request and Response
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionCreationDto {
    pub permission_name: String,
    pub permission_code: String,
    pub description: Option<String>,
    pub action_codes: Option<Vec<String>>,
    pub menus: Option<Vec<i32>>,
    pub apis: Option<Vec<i32>>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionCreationRespDto {
    #[serde(flatten)]
    pub base: PermissionDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionRespDto {
    #[serde(flatten)]
    pub base: PermissionDto,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionSimpleRespDto {
    pub id: i32,
    pub permission_name: String,
    pub permission_code: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionUpdateDto {
    #[serde(flatten)]
    pub base: PermissionDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionUpdateRespDto {
    #[serde(flatten)]
    pub base: Option<PermissionDetailsDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDto {
    pub permission_name: Option<String>,
    pub permission_code: Option<String>,
    pub description: Option<String>,
    pub action_codes: Option<Vec<String>>,
    pub menus: Option<Vec<i32>>,
    pub apis: Option<Vec<i32>>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionDeleteRespDto {
    pub success: bool,
}

// 实现从Model到PermissionDto的转换
impl From<Model> for PermissionDto {
    fn from(model: Model) -> Self {
        PermissionDto {
            permission_name: Some(model.permission_name),
            permission_code: Some(model.permission_code),
            // 如果Model中的description是None，则转换为一个空字符串
            description: model.description,
            action_codes: None,
            menus: None,
            apis: None,
            status: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDetail {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDetailsDto {
    pub id: i32,
    pub permission_name: String,
    pub permission_code: String,
    pub action_codes: Vec<String>,
    pub description: String,
    pub menus: Vec<String>,
    pub apis: Vec<ApiDetail>,
    pub status: String,
}
