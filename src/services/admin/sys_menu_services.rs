use crate::common::error::MyError;
use crate::dto::admin::sys_menu_dto::{MenuCreateDto, MenuTreeResponseDto, MenuUpdateDto};
use crate::schemas::admin;
use crate::schemas::admin::prelude::SysMenu;
use crate::schemas::admin::sys_menu;
use chrono::Utc;
use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::{Rc};
use crate::common::enums;
use crate::schemas::admin::sys_menu::Model;

//create_menu 创建菜单
pub async fn create_menu(
    db: &DatabaseConnection,
    menu_create_req: MenuCreateDto,
    create_user: String,
) -> Result<sys_menu::Model, MyError> {
    let parent_id = if menu_create_req.parent_id == 0 {
        None
    } else {
        Some(menu_create_req.parent_id)
    };
    let mut menu = sys_menu::ActiveModel {
        parent_id: Set(parent_id),
        menu_name: Set(Some(menu_create_req.menu_name)),
        r#type: Set(admin::sea_orm_active_enums::Type::from_string(
            menu_create_req.menu_type.as_str(),
        )?),
        route_path: Set(Some(menu_create_req.route_path)),
        route_name: Set(Some(menu_create_req.route_name)),
        component: Set(menu_create_req.component),
        constant: Set(i8::from(menu_create_req.constant)),
        i18n_key: Set(menu_create_req.i18n_key),
        roles: Set(Some(String::new())),
        keep_alive: Set(Option::from(i8::from(menu_create_req.keep_alive))),
        order: Set(Some(menu_create_req.order)),
        href: Set(menu_create_req.href),
        hide_in_menu: Set(Option::from(i8::from(menu_create_req.hide_in_menu))),
        active_menu: Set(menu_create_req.active_menu),
        multi_tab: Set(Option::from(i8::from(menu_create_req.multi_tab))),
        fixed_index_in_tab: Set(menu_create_req.fixed_index_in_tab),
        query: Set(menu_create_req.query),
        status: Set(menu_create_req.status.parse::<i8>().unwrap_or(1)),
        create_user: Set(create_user),
        create_time: Set(Some(Utc::now())),
        ..Default::default()
    };
    if enums::IconType::LocalIcon.matches(menu_create_req.icon_type) {
        menu.local_icon = Set(menu_create_req.icon)
    }else{
        menu.icon = Set(menu_create_req.icon)
    }
    menu.insert(db).await.map_err(MyError::from)
}


//get_menus 获取菜单列表
pub async fn get_menus(db: &DatabaseConnection) -> Result<Vec<sys_menu::Model>, DbErr> {
    SysMenu::find().all(db).await
}

// 修改get_menus函数以支持分页
pub async fn get_menus_paged(
    db: &DatabaseConnection,
    page: u64,      // 当前页码，从1开始
    page_size: u64, // 每页条目数
) -> Result<(Vec<sys_menu::Model>, u64), MyError> {
    // 使用.find()开始构建查询
    let paginator = SysMenu::find().paginate(db, page_size); // 设置每页条目数
    let num_pages = paginator.num_pages().await?; // 获取总页数
    let menus = paginator.fetch_page(page - 1).await?; // 获取指定页的结果，页码从0开始，所以这里需要减1
    Ok((menus, num_pages))
}

//get_menu_by_id 获取单个菜单
pub async fn get_menu_by_id(
    db: &DatabaseConnection,
    menu_id: i32,
) -> Result<Option<sys_menu::Model>, DbErr> {
    SysMenu::find_by_id(menu_id).one(db).await
}


//update_menu 更新菜单
pub async fn update_menu(
    db: &DatabaseConnection,
    menu_id: i32,
    menu_update_req: MenuUpdateDto,
) -> Result<Option<Model>, MyError> {
    // 尝试检索现有菜单
    let mut menu: sys_menu::ActiveModel = SysMenu::find_by_id(menu_id)
        .one(db)
        .await?
        .ok_or(MyError::BadRequestError("Menu not found".to_string()))?
        .into();

    // 更新字段如果它们被提供
    if let Some(menu_name) = menu_update_req.menu_name {
        menu.menu_name = Set(Some(menu_name));
    }

    if let Some(route_name) = menu_update_req.route_name {
        menu.route_name = Set(Some(route_name));
    }

    if let Some(route_path) = menu_update_req.route_path {
        menu.route_path = Set(Some(route_path));
    }

    menu.parent_id = Set(if menu_update_req.parent_id == Some(0){
        None
    } else {
        menu_update_req.parent_id
    });

    menu.constant = Set(i8::from(menu_update_req.constant.unwrap_or(false)));

    if let Some(component) = menu_update_req.component {
        menu.component = Set(Some(component));
    }

    if let Some(menu_type) = menu_update_req.menu_type {
        menu.r#type = Set(admin::sea_orm_active_enums::Type::from_string(
            menu_type.as_str(),
        )?);
    }

    if let Some(status) = menu_update_req.status {
            menu.status = Set(status.parse().unwrap());
    }

    if let Some(hide_in_menu) = menu_update_req.hide_in_menu {
        menu.hide_in_menu = Set(Some(i8::from(hide_in_menu)));
    }

    if let Some(i18n_key) = menu_update_req.i18n_key {
        menu.i18n_key = Set(Some(i18n_key));
    }


    if let Some(keep_alive) = menu_update_req.keep_alive {
        menu.keep_alive = Set(Some(i8::from(keep_alive)));
    }

    if let Some(order) = menu_update_req.order {
        menu.order = Set(Some(order));
    }

    if let Some(href) = menu_update_req.href {
        menu.href = Set(Some(href));
    }

    if let Some(hide_in_menu) = menu_update_req.hide_in_menu {
        menu.hide_in_menu = Set(Some(i8::from(hide_in_menu)));
    }

    if let Some(active_menu) = menu_update_req.active_menu {
        menu.active_menu = Set(Some(active_menu));
    }

    if let Some(multi_tab) = menu_update_req.multi_tab {
        menu.multi_tab = Set(Some(i8::from(multi_tab)));
    }

    if let Some(fixed_index_in_tab) = menu_update_req.fixed_index_in_tab {
        menu.fixed_index_in_tab = Set(Some(fixed_index_in_tab));
    }

    if let Some(query) = menu_update_req.query {
        menu.query = Set(Some(query));
    }

    if let Some(icon_type) = menu_update_req.icon_type {
        if enums::IconType::LocalIcon.matches(icon_type) {
            menu.local_icon = Set(menu_update_req.icon);
            menu.icon = Set(Some("".to_string()));
        } else {
            menu.icon = Set(menu_update_req.icon);
            menu.local_icon = Set(Some("".to_string()));
        }
    }

    // 更新数据库中的菜单
    menu.update(db).await.map(Some).map_err(MyError::from)
}



//delete_menu 删除菜单
pub async fn delete_menu(db: &DatabaseConnection, menu_id: i32) -> Result<u64, DbErr> {
    // 首先，尝试更新所有引用该菜单ID作为parent_id的子菜单，
    // 将它们的parent_id设置为NULL（或者您可以选择删除这些子菜单）
    let _ = SysMenu::update_many()
        .col_expr(sys_menu::Column::ParentId, Expr::value(None::<i32>))
        .filter(sys_menu::Column::ParentId.eq(menu_id))
        .exec(db)
        .await?;

    // 然后，尝试删除目标菜单项
    let menu = sys_menu::ActiveModel {
        id: Set(menu_id),
        ..Default::default()
    };

    SysMenu::delete(menu)
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}

pub async fn delete_menus(db: &DatabaseConnection, menu_ids: Vec<i32>) -> Result<u64, DbErr> {
    // 步骤1: 更新所有引用这些菜单ID作为parent_id的子菜单
    let update_children_result = SysMenu::update_many()
        .col_expr(sys_menu::Column::ParentId, Expr::value(None::<i32>))
        .filter(sys_menu::Column::ParentId.is_in(menu_ids.clone()))
        .exec(db)
        .await?;
    let children_updated = update_children_result.rows_affected;

    // 步骤2: 删除这些菜单项
    let delete_menus_result = SysMenu::delete_many()
        .filter(sys_menu::Column::Id.is_in(menu_ids))
        .exec(db)
        .await?;
    let menus_deleted = delete_menus_result.rows_affected;

    // 返回总共影响的行数
    Ok(children_updated + menus_deleted)
}


pub fn build_menu_tree(menus: Vec<sys_menu::Model>) -> Rc<RefCell<MenuTreeResponseDto>> {
    let mut menu_map: HashMap<i32, Rc<RefCell<MenuTreeResponseDto>>> = HashMap::new();

    // 创建一个虚拟根节点
    let root = Rc::new(RefCell::new(MenuTreeResponseDto {
        id: 0, // 虚拟的根节点 ID
        p_id: None,
        label: "root".to_string(),
        children: None,
    }));

    // 创建所有 MenuTree 实例
    for menu in &menus {
        let menu_tree = Rc::new(RefCell::new(MenuTreeResponseDto {
            id: menu.id,
            p_id: menu.parent_id, // 父节点稍后设置
            label: menu.menu_name.clone().unwrap_or_default(),
            children: None,
        }));
        menu_map.insert(menu.id, Rc::clone(&menu_tree));
    }

    for menu in &menus {
        if let Some(cur_menu) = menu_map.get(&menu.id).cloned() {
            let parent_id = cur_menu.borrow().p_id;
            if let Some(parent_id) = parent_id {
                if let Some(parent_menu) = menu_map.get(&parent_id) {
                    parent_menu.borrow_mut().children.get_or_insert_with(Vec::new).push(cur_menu.clone());
                }
            } else {
                root.borrow_mut().children.get_or_insert_with(Vec::new).push(cur_menu.clone());
            }
        }
    }

    root
}
