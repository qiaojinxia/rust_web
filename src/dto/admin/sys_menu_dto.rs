use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::schemas::admin::sys_menu::Model;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuCreateDto {
    #[serde(rename = "permissionId", skip_serializing_if = "Option::is_none")]
    pub permission_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 64))]
    pub icon: Option<String>,

    #[serde(rename = "iconType")]
    pub icon_type: String,

    #[serde(rename = "menuName", skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 64))]
    pub menu_name: Option<String>,

    #[serde(rename = "menuType")]
    pub menu_type: String,

    #[serde(rename = "routeName")]
    #[validate(length(min = 1, max = 256))]
    pub route_name: String,

    #[serde(rename = "routePath", skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 256))]
    pub route_path: Option<String>,

    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,

    #[serde(rename = "status")]
    // #[validate(custom = "validate_status")]
    pub status: String,

    #[serde(rename = "hideInMenu")]
    pub is_hidden: bool,

    #[serde(rename = "order")]
    pub order: i8,

    #[serde(rename = "i18nKey", skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
}

fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "1" | "2" => Ok(()),
        _ => {
            let mut error = ValidationError::new("invalid_status");
            error.message = Some("The status must be either '1' or '2'.".into());
            Err(error)
        },
    }
}


#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuBaseRespDto {
    pub id: i32,
    pub permission_id: Option<i32>,
    pub icon: Option<String>,
    pub icon_type: String,
    pub menu_name: String,
    pub menu_type: String,
    pub order: i8,
    pub parent_id: i32,
    pub route_name: String,
    pub route_path: String,
    pub status: String,
    pub hide_in_menu: i8,
    pub i18n_key: String,
    pub component: String,
}

impl From<Model> for MenuBaseRespDto {
    fn from(model: Model) -> Self {
        let meta_data = model.meta.unwrap();
        MenuBaseRespDto {
            id: model.id,
            permission_id: Some(model.permission_id.unwrap_or(0)), // 处理 Option<i32> 到 i32 的转换
            menu_name: model.menu_name, // 注意这里是直接将 Option<String> 转换为 Option<String>
            icon_type: meta_data.get("icon_type").unwrap().as_str().unwrap_or("").to_string() ,
            icon:  Some(meta_data["icon"].as_str().unwrap_or("").to_string()),
            route_path: model.route_path, // 同样是将 Option<String> 转换为 Option<String>
            route_name: model.route_name, // 同上
            parent_id: model.parent_id.unwrap_or(0), // 直接复制 Option<i32>
            menu_type: model.r#type.to_string(), // 处理 Option<i8> 到 i8 的转换
            status: model.status.to_string(),
            hide_in_menu: model.is_hidden,
            order: model.sort, // 处理 Option<i8> 到 i8 的转换
            i18n_key: meta_data.get("i18nKey").unwrap().as_str().unwrap_or("").to_string(),
            component: model.component.unwrap(),
        }
    }
}

impl Default for MenuBaseRespDto {
    fn default() -> Self {
        MenuBaseRespDto {
            id: 0,
            permission_id: None,
            menu_name: "".to_string(),
            icon_type: "".to_string(),
            icon: None,
            route_name: "".to_string(),
            route_path: "".to_string(),
            parent_id: 0,
            menu_type: "".to_string(),
            status: "".to_string(),
            hide_in_menu: 2,
            order: 0,
            i18n_key: "".to_string(),
            component: "".to_string(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MenuCreationResponseDto {
    #[serde(flatten)]
    pub base: MenuBaseRespDto,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MenusResponseDto {
    pub list: Vec<MenuBaseRespDto>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDto {
    #[serde(flatten)]
    pub base: MenuBaseRespDto,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuUpdateDto {
    #[serde(flatten)]
    pub base: MenuCreateDto,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MenuUpdateResponseDto {
    #[serde(flatten)]
    pub base: MenuBaseRespDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDeleteResponseDto {
    pub success: bool,
}
