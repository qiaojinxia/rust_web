use serde::{Deserialize, Serialize};
use validator_derive::Validate;
use crate::schema::admin::sys_menu::Model;

/// 菜单基础信息DTO
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuBaseDto {
    pub id: i32,
    pub permission_id: i32,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub icon: Option<String>,
    #[validate(length(min = 1, max = 256))]
    pub route: Option<String>,
    #[validate(length(min = 1, max = 256))]
    pub route_name: Option<String>,
    pub parent_id: Option<i32>,
    pub menu_type: i8,
    pub status: i8,
    pub is_hidden: i8,
    pub sort: i8,
}

impl From<Model> for MenuBaseDto {
    fn from(model: Model) -> Self {
        MenuBaseDto {
            id: model.id,
            permission_id: model.permission_id.unwrap_or(0), // 处理 Option<i32> 到 i32 的转换
            name: Some(model.menu_name), // 注意这里是直接将 Option<String> 转换为 Option<String>
            icon: None, // 在 Model 中没有对应的字段，你需要根据实际情况处理
            route: Some(model.route), // 同样是将 Option<String> 转换为 Option<String>
            route_name: Some(model.route_name), // 同上
            parent_id: model.parent_id, // 直接复制 Option<i32>
            menu_type: model.r#type.unwrap_or_default(), // 处理 Option<i8> 到 i8 的转换
            status: model.status,
            is_hidden: model.is_hidden,
            sort: model.sort.unwrap_or_default(), // 处理 Option<i8> 到 i8 的转换
        }
    }
}

/// 菜单创建请求DTO
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuCreationDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}

/// 菜单创建响应DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuCreationResponseDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}

/// 菜单列表响应DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct MenusResponseDto {
    pub list: Vec<MenuDto>,
}

/// 单个菜单信息DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}

/// 菜单更新请求DTO
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuUpdateDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}

/// 菜单更新响应DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuUpdateResponseDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}

/// 菜单删除响应DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDeleteResponseDto {
    pub success: bool,
}
