use crate::common::value::{extract_bool, extract_i32, extract_json, extract_string};
use crate::schemas::admin::sys_menu::Model;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::{Validate};
use crate::dto::admin::common_dto::{validate_menu_type, validate_status};

// 定义一个结构体来表示菜单按钮
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuButton {
    code: String, // 按钮代码
    desc: String, // 按钮描述
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuCreateDto {
    #[validate(length(min = 1), custom(function = "validate_menu_type"))]
    pub menu_type: String, // 菜单类型
    #[validate(length(min = 3, max = 256))]
    pub menu_name: String, //菜单名称
    #[validate(length(min = 3, max = 256))]
    pub route_name: String, //路由名称
    #[validate(length(min = 3, max = 256))]
    pub route_path: String, //路由路径
    pub path_param: Option<String>, //路径参数
    pub layout: Option<String>,     //布局
    pub i18n_key: Option<String>,   //国际化key
    pub order: i32,                 //排序
    pub icon_type: String,          //图标类型
    pub icon: Option<String>,       //图标
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: String, //菜单状态
    pub keep_alive: bool,           //缓存路由
    pub constant: bool,             //常量路由
    pub href: Option<String>,       //外链
    #[serde(rename = "hideInMenu")]
    pub is_hidden: bool, // 隐藏菜单
    pub multi_tab: bool,            // 支持多页签
    pub parent_id: i32,             //父菜单
    pub fixed_index_in_tab: Option<i32>, //固定在页签中的序号
    pub component: Option<String>,  //组建路径
    pub active_menu: Option<String>, // 高亮的菜单
    pub query: Option<Value>,       //路由参数
    pub buttons: Option<Vec<MenuButton>>, //按钮
}

// 定义菜单基础响应数据传输对象（DTO）
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuBaseRespDto {
    pub id: i32,
    pub parent_id: i32,
    pub menu_type: String,
    pub menu_name: String,
    pub route_name: String,
    pub route_path: String,
    pub component: Option<String>,
    pub icon: String,
    pub icon_type: String,
    pub buttons: Option<Vec<MenuButton>>,
    pub children: Option<Vec<MenuBaseRespDto>>,
    // 下面是 MenuPropsOfRoute 的字段
    pub i18n_key: Option<String>,
    pub keep_alive: bool,
    pub constant: bool,
    pub order: i32,
    pub href: Option<String>,
    pub hide_in_menu: bool,
    pub active_menu: Option<String>,
    pub path_param: Option<String>,
    pub multi_tab: bool,
    pub fixed_index_in_tab: Option<i32>,
    pub query: Option<Value>,
    pub status: String,
}

impl From<Model> for MenuBaseRespDto {
    fn from(model: Model) -> Self {
        // 确保 meta 字段不为空
        let meta = model.meta.unwrap_or_default();
        MenuBaseRespDto {
            id: model.id,
            parent_id: model.parent_id.unwrap_or_default(),
            menu_type: model.r#type.get_serial_number().unwrap_or("1").to_string(),
            menu_name: model.menu_name.unwrap_or_default(),
            route_name: model.route_name.unwrap_or_default(),
            route_path: model.route_path.unwrap_or_default(),
            component: model.component,
            path_param: model.path_param,
            order: model.sort.unwrap_or(0),
            constant: model.constant == 1,
            icon_type: extract_string(&meta, "icon_type"),
            icon: extract_string(&meta, "icon"),
            buttons: meta.get("buttons").and_then(|v| {
                v.as_array().map(|a| {
                    a.iter()
                        .map(|v| MenuButton {
                            code: v
                                .get("code")
                                .map_or("".to_string(), |v| v.as_str().unwrap_or("").to_string()),
                            desc: v
                                .get("desc")
                                .map_or("".to_string(), |v| v.as_str().unwrap_or("").to_string()),
                        })
                        .collect()
                })
            }),
            children: None,
            // 下面是 MenuPropsOfRoute 的字段
            i18n_key: Some(extract_string(&meta, "i18n_key")),
            keep_alive: extract_bool(&meta, "keep_alive").unwrap_or(false),
            href: Some(extract_string(&meta, "href")),
            hide_in_menu: model.is_hidden == 1,
            active_menu: Some(extract_string(&meta, "active_menu")),
            multi_tab: extract_bool(&meta, "multi_tab").unwrap_or(false),
            fixed_index_in_tab: extract_i32(&meta, "fixed_index_in_tab"),
            query: extract_json(&meta, "query"),
            status: model.status.to_string(),
        }
    }
}

impl Default for MenuBaseRespDto {
    fn default() -> Self {
        MenuBaseRespDto {
            id: 0,
            parent_id: 0,
            menu_type: "1".to_string(),
            menu_name: "empty".to_string(),
            route_name: "".to_string(),
            route_path: "".to_string(),
            path_param: None,
            component: None,
            icon: "".to_string(),
            icon_type: "".to_string(),
            buttons: None,
            children: None,
            i18n_key: None,
            keep_alive: false,
            constant: false,
            order: 0,
            href: None,
            hide_in_menu: false,
            active_menu: None,
            multi_tab: false,
            fixed_index_in_tab: None,
            query: None,
            status: "2".to_string(),
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
#[serde(rename_all = "camelCase")]
pub struct MenuUpdateDto {
    #[validate(length(min = 1), custom(function = "validate_menu_type"))]
    pub menu_type: Option<String>, // 菜单类型
    #[validate(length(min = 3, max = 256))]
    pub menu_name: Option<String>, //菜单名称
    #[validate(length(min = 3, max = 256))]
    pub route_name: Option<String>, //路由名称
    #[validate(length(min = 3, max = 256))]
    pub route_path: String, //路由路径
    pub path_param: Option<String>, //路径参数
    pub layout: Option<String>,     //布局
    pub i18n_key: Option<String>,   //国际化key
    pub order: i32,                 //排序
    pub icon_type: String,          //图标类型
    pub icon: Option<String>,       //图标
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: String, //菜单状态
    pub keep_alive: bool,           //缓存路由
    pub constant: bool,             //常量路由
    pub href: Option<String>,       //外链
    #[serde(rename = "hideInMenu")]
    pub is_hidden: bool, // 隐藏菜单
    pub multi_tab: bool,            // 支持多页签
    pub parent_id: i32,             //父菜单
    pub fixed_index_in_tab: Option<i32>, //固定在页签中的序号
    pub component: Option<String>,  //组建路径
    pub active_menu: Option<String>, // 高亮的菜单
    pub query: Option<Value>,       //路由参数
    pub buttons: Option<Vec<MenuButton>>, //按钮
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

// 定义一个用于序列化的简化版本的结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuTreeResponseDto {
    pub id: i32,
    pub label: String,
    pub children: Vec<MenuTreeResponseDto>,
}
