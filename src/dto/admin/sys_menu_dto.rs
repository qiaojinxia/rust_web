use std::cell::RefCell;
use std::rc::Rc;
use crate::dto::admin::common_dto::{validate_menu_type, validate_status, validate_icon_type};
use crate::schemas::admin::sys_menu::Model;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use validator::Validate;

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
    #[validate(length(min = 2, max = 256))]
    pub menu_name: String, //菜单名称
    #[validate(length(min = 2, max = 256))]
    pub route_name: String, //路由名称
    #[validate(length(min = 2, max = 256))]
    pub route_path: String, //路由路径
    pub layout: Option<String>,     //布局
    pub i18n_key: Option<String>,   //国际化key
    pub order: i32,                 //排序
    #[validate(length(min = 1), custom(function = "validate_icon_type"))]
    pub icon_type: String,          //图标类型
    pub icon: Option<String>,       //图标
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: String, //菜单状态
    pub keep_alive: bool,           //缓存路由
    pub constant: bool,             //常量路由
    pub href: Option<String>,       //外链
    pub hide_in_menu: bool, // 隐藏菜单
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
    pub i18n_key: Option<String>,
    pub keep_alive: bool,
    pub constant: bool,
    pub order: i32,
    pub href: Option<String>,
    pub hide_in_menu: bool,
    pub active_menu: Option<String>,
    pub multi_tab: bool,
    pub fixed_index_in_tab: Option<i32>,
    pub query: Option<Value>,
    pub status: String,
}

impl From<Model> for MenuBaseRespDto {
    fn from(model: Model) -> Self {
        // 确保 meta 字段不为空
        let mut menu_base_resp_dto = MenuBaseRespDto {
            id: model.id,
            parent_id: model.parent_id.unwrap_or_default(),
            menu_type: model.r#type.get_serial_number().unwrap_or("1").to_string(),
            menu_name: model.menu_name.unwrap_or_default(),
            route_name: model.route_name.unwrap_or_default(),
            route_path: model.route_path.unwrap_or_default(),
            component: model.component,
            order: model.order.unwrap_or(0),
            constant: model.constant == 1,
            icon: model.icon.clone().unwrap_or_default(),
            icon_type: "".to_string(),
            buttons: Some(vec![]),
            children: None,
            i18n_key: model.i18n_key.clone(),
            keep_alive: model.keep_alive.unwrap_or(0) == 1,
            href: model.href.clone(),
            hide_in_menu: model.hide_in_menu.unwrap_or(0) == 1,
            active_menu: model.active_menu.clone(),
            multi_tab: model.multi_tab.unwrap_or(0) == 1,
            fixed_index_in_tab: model.fixed_index_in_tab,
            query: model.query.clone(),
            status: model.status.to_string(),
        };
        let mut icon_type = "1".to_string();
        match model.local_icon{
            Some(icon) => {
                if icon != ""{
                    icon_type = "2".to_string();
                    menu_base_resp_dto.icon = icon;
                }
            },
            None  => {},
        }
        menu_base_resp_dto.icon_type = icon_type;
        menu_base_resp_dto
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
    #[validate(length(min = 2, max = 256))]
    pub menu_name: Option<String>, //菜单名称
    #[validate(length(min = 2, max = 256))]
    pub route_name: Option<String>, //路由名称
    #[validate(length(min = 2, max = 256))]
    pub route_path: Option<String>, //路由路径
    pub layout: Option<String>,     //布局
    pub i18n_key: Option<String>,   //国际化key
    pub order: Option<i32>,                 //排序
    #[validate(length(min = 1), custom(function = "validate_icon_type"))]
    pub icon_type: Option<String>,          //图标类型
    pub icon: Option<String>,       //图标
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: Option<String>, //菜单状态
    pub keep_alive: Option<bool>,           //缓存路由
    pub constant: Option<bool>,             //常量路由
    pub href: Option<String>,       //外链
    pub hide_in_menu: Option<bool>,         // 隐藏菜单
    pub multi_tab: Option<bool>,            // 支持多页签
    pub parent_id: Option<i32>,             //父菜单
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
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuTreeResponseDto {
    pub id: i32,
    pub label: String,
    pub p_id: Option<i32>,
    pub children: Option<Vec<Rc<RefCell<MenuTreeResponseDto>>>>,
}