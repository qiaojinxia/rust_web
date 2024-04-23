use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::schemas::admin::sys_permission::Model;

// DTOs for Request and Response
#[derive(Deserialize, Validate)]
pub struct PermissionCreationDto {
    #[serde(rename = "permissionCode")]
    pub permission_code: String,
    pub description: Option<String>,
    pub actions: Vec<String>,
    #[serde(rename = "menusId")]
    pub menus_id:Option<Vec<i32>>,
    #[serde(rename = "apisId")]
    pub apis_id:Option<Vec<i32>>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionCreationRespDto{
    #[serde(flatten)]
    pub base: PermissionDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionRespDto {
    #[serde(flatten)]
    pub base: PermissionDto,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionUpdateDto {
    pub permission_code: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionUpdateRespDto {
    #[serde(flatten)]
    pub base: Option<PermissionDetailsDto>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionDto {
    pub id: i32,
    pub permission_code: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionDeleteRespDto {
    pub success: bool,
}


// 实现从Model到PermissionDto的转换
impl From<Model> for PermissionDto {
    fn from(model: Model) -> Self {
        PermissionDto {
            id: model.id,
            permission_code: model.permission_code,
            // 如果Model中的description是None，则转换为一个空字符串
            description: model.description.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuDetail {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDetail {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDetailsDto {
    pub permission_id: i32,
    pub permission_code: String,
    pub actions: Vec<String>, // Assuming action codes can be split into Vec<String>
    pub description: String,
    pub menus: Vec<MenuDetail>,
    pub apis: Vec<ApiDetail>,
    pub menu_status: i32,
}
