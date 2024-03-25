use serde::{Deserialize, Serialize};
use validator_derive::Validate;
use crate::schemas::admin::sys_menu::Model;

/// Menu Base Information DTO
/// Description: Represents the foundational data for a menu item, including identifiers,
/// names, icons, handlers, and hierarchical information. It serves as a core component for various menu-related operations.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuBaseDto {
    pub id: Option<i32>,
    pub permission_id: Option<i32>,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    #[validate(range(min = 1, max = 2))]
    pub icon_type: i8,
    #[validate(length(min = 1, max = 64))]
    pub icon: Option<String>,
    #[validate(length(min = 1, max = 256))]
    pub route_name: Option<String>,
    #[validate(length(min = 1, max = 256))]
    pub route: Option<String>,
    pub parent_id: Option<i32>,
    #[validate(range(min = 1, max = 2))]
    pub menu_type: i8,
    #[validate(range(min = 0, max = 1))]
    pub status: i8,
    #[validate(range(min = 0, max = 1))]
    pub is_hidden: i8,
    pub order: i8,
}

impl From<Model> for MenuBaseDto {
    fn from(model: Model) -> Self {
        let meta_data = model.meta.unwrap();
        MenuBaseDto {
            id: Some(model.id),
            permission_id: Some(model.permission_id.unwrap_or(0)), // 处理 Option<i32> 到 i32 的转换
            name: Some(model.menu_name), // 注意这里是直接将 Option<String> 转换为 Option<String>
            icon_type: meta_data.get("icon_type").and_then(|v| v.as_i64()).unwrap_or(0) as i8,
            icon:  Some(meta_data["icon"].as_str().unwrap_or("").to_string()),
            route: Some(model.route), // 同样是将 Option<String> 转换为 Option<String>
            route_name: Some(model.route_name), // 同上
            parent_id: model.parent_id, // 直接复制 Option<i32>
            menu_type: model.r#type, // 处理 Option<i8> 到 i8 的转换
            status: model.status,
            is_hidden: model.is_hidden,
            order: model.sort, // 处理 Option<i8> 到 i8 的转换
        }
    }
}

impl Default for MenuBaseDto {
    fn default() -> Self {
        MenuBaseDto {
            id: None,
            permission_id: None,
            name: None,
            icon_type: 1, // 为非Option类型字段指定默认值
            icon: None,
            route_name: None,
            route: None,
            parent_id: None,
            menu_type: 1, // 为非Option类型字段指定默认值
            status: 1, // 为非Option类型字段指定默认值
            is_hidden: 2, // 为非Option类型字段指定默认值
            order: 0, // 为非Option类型字段指定默认值
        }
    }
}

/// Create Menu
///
/// Request Method: POST
/// API Path: /menus
///
/// Description:
/// This endpoint is used for creating a new menu item. It accepts necessary menu information and creates a new menu item in the system.
///
/// Parameters:
/// - `app_state`: The shared state of the application, containing global configurations and database connections.
/// - `menu_creation_dto`: The data required to create a menu, submitted in JSON format.
///
/// Response:
/// - Success: Returns the basic information of the created menu item, encapsulated in `MenuCreationResponseDto`.
/// - Error: Returns an error message if the request data does not meet requirements or in case of an internal server error.


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuCreationDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MenuCreationResponseDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}


/// Get Menus
///
/// Request Method: GET
/// API Path: /menus
///
/// Description:
/// This endpoint retrieves all menu items in the system. It returns a list of menu items for display or management purposes.
///
/// Parameters:
/// - `app_state`: The shared state of the application, containing global configurations and database connections.
///
/// Response:
/// - Success: Returns a list of all menu items in the system, encapsulated in `MenusResponseDto`.
/// - Error: Returns an error message in case of an internal server error.


#[derive(Debug, Serialize, Deserialize)]
pub struct MenusResponseDto {
    pub list: Vec<MenuDto>,
}


/// Get Menu by ID
///
/// Request Method: GET
/// API Path: /menus/{id}
///
/// Description:
/// This endpoint retrieves a specific menu item based on the given ID. It returns detailed information about the requested menu item.
///
/// Parameters:
/// - `app_state`: The shared state of the application, containing global configurations and database connections.
/// - `path`: Path parameter containing the ID of the menu item to retrieve.
///
/// Response:
/// - Success: Returns detailed information about the requested menu item, encapsulated in `MenuDto`.
/// - Error: Returns an error message if the corresponding menu item cannot be found or in case of an internal server error.

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}


/// Update Menu
///
/// Request Method: PUT
/// API Path: /menus/{id}
///
/// Description:
/// This endpoint updates information for an existing menu item. It accepts partial updates, allowing for flexible modifications.
///
/// Parameters:
/// - `app_state`: The shared state of the application, containing global configurations and database connections.
/// - `path`: Path parameter containing the ID of the menu item to update.
/// - `menu_update_dto`: The updated data for the menu item, submitted in JSON format. Fields are optional for partial updates.
///
/// Response:
/// - Success: Returns the updated information of the menu item, encapsulated in `MenuUpdateResponseDto`.
/// - Error: Returns an error message if the request data does not meet requirements, the menu item cannot be found, or in case of an internal server error.


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuUpdateDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MenuUpdateResponseDto {
    #[serde(flatten)]
    pub base: MenuBaseDto,
}


/// Delete Menu
///
/// Request Method: DELETE
/// API Path: /menus/{id}
///
/// Description:
/// This endpoint deletes a specific menu item based on the given ID. It is used for removing menu items from the system.
///
/// Parameters:
/// - `app_state`: The shared state of the application, containing global configurations and database connections.
/// - `path`: Path parameter containing the ID of the menu item to delete.
///
/// Response:
/// - Success: Returns a boolean flag indicating the success or failure of the deletion, encapsulated in `MenuDeleteResponseDto`.
/// - Error: Returns an error message in case of an internal server error or if the menu item cannot be found.

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDeleteResponseDto {
    pub success: bool,
}
