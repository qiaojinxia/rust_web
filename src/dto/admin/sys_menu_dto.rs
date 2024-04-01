use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::schemas::admin::sys_menu::Model;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MenuCreateDto {

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

    #[serde(rename = "parentId")]
    pub parent_id: i32,

    #[serde(rename = "status")]
    // #[validate(custom = "validate_status")]
    pub status: String,

    pub component: String,

    #[serde(rename = "hideInMenu")]
    pub is_hidden: bool,

    #[serde(rename = "order")]
    pub order: i8,

    #[serde(rename = "i18nKey", skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,

    pub layout: Option<String>,

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
    pub icon: Option<String>,
    pub icon_type: String,
    pub menu_name: String,
    pub menu_type: String,
    pub order: i8,
    pub parent_id: i32,
    pub route_name: String,
    pub route_path: String,
    pub status: String,
    pub hide_in_menu: bool,
    pub i18n_key: String,
    pub component: String,
    pub layout: String,
}

impl From<Model> for MenuBaseRespDto {
    fn from(model: Model) -> Self {
        // Assuming `meta_data` is an Option<HashMap<String, Value>> or similar.
        let meta_data = model.meta.unwrap_or_default();

        MenuBaseRespDto {
            id: model.id,
            menu_name: model.menu_name,
            icon_type: meta_data.get("icon_type").map_or_else(|| "".to_string(), |v| v.as_str().unwrap_or("").to_string()),
            icon:  meta_data.get("icon").map(|v| v.as_str().unwrap_or("").to_string()),
            route_path: model.route_path,
            route_name: model.route_name,
            parent_id: model.parent_id.unwrap_or(0),
            menu_type: model.r#type.to_string(),
            status: model.status.to_string(),
            hide_in_menu: model.is_hidden == 1,
            order: model.sort,
            i18n_key: meta_data.get("i18nKey")
                .map_or_else(|| "".to_string(), |v| v.as_str()
                    .unwrap_or("").to_string()),
            component: model.component.unwrap(),
            layout: meta_data.get("layout")
                .map_or_else(|| "base".to_string(), |v| v.as_str()
                    .unwrap_or("").to_string()),
        }
    }
}

impl Default for MenuBaseRespDto {
    fn default() -> Self {
        MenuBaseRespDto {
            id: 0,
            menu_name: "".to_string(),
            icon_type: "".to_string(),
            icon: None,
            route_name: "".to_string(),
            route_path: "".to_string(),
            parent_id: 0,
            menu_type: "".to_string(),
            status: "".to_string(),
            hide_in_menu: false,
            order: 0,
            i18n_key: "".to_string(),
            component: "".to_string(),
            layout: "default".to_string(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 64))]
    pub icon: Option<String>,

    #[serde(rename = "iconType")]
    pub icon_type: Option<String>,

    #[serde(rename = "menuName", skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 64))]
    pub menu_name: Option<String>,

    #[serde(rename = "menuType")]
    pub menu_type: Option<String>,

    #[serde(rename = "routeName")]
    #[validate(length(min = 1, max = 256))]
    pub route_name: Option<String>,

    #[serde(rename = "routePath", skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 256))]
    pub route_path: Option<String>,

    #[serde(rename = "parentId")]
    pub parent_id: Option<i32>,

    #[serde(rename = "status")]
    // #[validate(custom = "validate_status")]
    pub status: Option<String>,

    pub component: Option<String>,

    #[serde(rename = "hideInMenu")]
    pub is_hidden: Option<bool>,

    #[serde(rename = "order")]
    pub order: Option<i8>,

    #[serde(rename = "i18nKey", skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,

    pub layout: Option<String>,
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
