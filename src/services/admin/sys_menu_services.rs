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
use serde::Deserialize;
use serde_json::json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

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
        menu_name: Set(Some(menu_create_req.menu_name)),
        r#type: Set(admin::sea_orm_active_enums::Type::from_string(
            menu_create_req.menu_type.as_str(),
        )?),
        route_path: Set(Some(menu_create_req.route_path)),
        route_name: Set(Some(menu_create_req.route_name)),
        path_param: Set(menu_create_req.path_param),
        parent_id: Set(parent_id),
        create_user: Set(create_user),
        status: Set(menu_create_req.status.parse::<i8>().unwrap_or(1)),
        is_hidden: Set(i8::from(menu_create_req.is_hidden)),
        create_time: Set(Some(Utc::now())),
        sort: Set(Some(menu_create_req.order)), // Assuming order is present in MenuCreateDto
        component: Set(menu_create_req.component),
        constant: Set(i8::from(menu_create_req.constant)),
        ..Default::default()
    };

    let mut meta = json!({
        "icon": menu_create_req.icon,
        "icon_type": menu_create_req.icon_type,
        "layout": menu_create_req.layout.unwrap_or_else(|| "base".to_string()),
        "href": menu_create_req.href,
        "keep_alive": menu_create_req.keep_alive,
        "multi_tab": menu_create_req.multi_tab,
        "fixed_index_in_tab": menu_create_req.fixed_index_in_tab,
    });

    if let Some(i18n_key) = menu_create_req.i18n_key {
        meta["i18n_key"] = json!(i18n_key);
    }

    if let Some(active_menu) = menu_create_req.active_menu {
        meta["active_menu"] = json!(active_menu);
    }

    if let Some(query) = menu_create_req.query {
        meta["query"] = json!(query);
    }

    if let Some(buttons) = menu_create_req.buttons {
        meta["buttons"] = json!(buttons);
    }

    menu.meta = Set(Some(meta));

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
) -> Result<Option<sys_menu::Model>, MyError> {
    // Attempt to retrieve the existing menu
    let mut menu: sys_menu::ActiveModel = SysMenu::find_by_id(menu_id)
        .one(db)
        .await?
        .ok_or(MyError::BadRequestError("Menu not found".to_string()))?
        .into();

    // Update fields if they are provided
    menu.menu_name = Set(menu_update_req.menu_name);
    menu.route_name = Set(menu_update_req.route_name);
    menu.route_path = Set(Some(menu_update_req.route_path));
    menu.parent_id = Set(if menu_update_req.parent_id == 0 {
        None
    } else {
        Some(menu_update_req.parent_id)
    });
    menu.sort = Set(Some(menu_update_req.order));
    menu.path_param = Set(menu_update_req.path_param);
    menu.constant = Set(i8::from(menu_update_req.constant));
    if let Some(component) = menu_update_req.component {
        menu.component = Set(Some(component));
    }

    if let Some(menu_type) = menu_update_req.menu_type {
        menu.r#type = Set(admin::sea_orm_active_enums::Type::from_string(
            menu_type.as_str(),
        )?);
    }

    if let Ok(status) = menu_update_req.status.parse() {
        menu.status = Set(status);
    }

    menu.is_hidden = Set(i8::from(menu_update_req.is_hidden));

    // Handle the metaObject
    let mut meta = json!({
        "icon": menu_update_req.icon,
        "icon_type": menu_update_req.icon_type,
        "layout": menu_update_req.layout.unwrap_or_else(|| "base".to_string()),
        "href": menu_update_req.href,
        "keep_alive": menu_update_req.keep_alive,
        "multi_tab": menu_update_req.multi_tab,
        "fixed_index_in_tab": menu_update_req.fixed_index_in_tab,
    });

    if let Some(i18n_key) = menu_update_req.i18n_key {
        meta["i18n_key"] = json!(i18n_key);
    }

    if let Some(active_menu) = menu_update_req.active_menu {
        meta["active_menu"] = json!(active_menu);
    }
    if let Some(query) = menu_update_req.query {
        meta["query"] = json!(query);
    }

    if let Some(buttons) = menu_update_req.buttons {
        meta["buttons"] = json!(buttons);
    }

    menu.meta = Set(Some(meta));

    // Update the menu in the database
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

#[derive(Clone, Deserialize)]
pub struct MenuTree {
    pub id: i32,
    #[serde(rename = "pId")]
    pub parent: Option<Weak<RefCell<MenuTree>>>,
    pub label: String,
    pub children: Option<RefCell<Vec<Rc<RefCell<MenuTree>>>>>,
}
impl MenuTree {
    // 添加一个方法来转换为可序列化的结构体
    fn to_serializable(&self) -> MenuTreeResponseDto {
        MenuTreeResponseDto {
            id: self.id,
            label: self.label.clone(),
            children: self.children.as_ref().map_or(vec![], |children| {
                children
                    .borrow()
                    .iter()
                    .map(|child| child.borrow().to_serializable())
                    .collect()
            }),
        }
    }
}

pub fn build_menu_tree(menus: Vec<sys_menu::Model>) -> MenuTreeResponseDto {
    let mut menu_map: HashMap<i32, Rc<RefCell<MenuTree>>> = HashMap::new();

    // 创建一个虚拟根节点
    let root = Rc::new(RefCell::new(MenuTree {
        id: 0, // 虚拟的根节点 ID
        parent: None,
        label: "root".to_string(),
        children: Some(RefCell::new(Vec::new())),
    }));

    // 创建所有 MenuTree 实例
    for menu in &menus {
        let menu_tree = Rc::new(RefCell::new(MenuTree {
            id: menu.id,
            parent: None, // 父节点稍后设置
            label: menu.menu_name.clone().unwrap_or_default(),
            children: Some(RefCell::new(Vec::new())),
        }));
        menu_map.insert(menu.id, Rc::clone(&menu_tree));
        // 先将所有节点作为根节点的直接子节点（稍后调整）
        root.borrow_mut()
            .children
            .as_ref()
            .unwrap()
            .borrow_mut()
            .push(Rc::clone(&menu_tree));
    }

    // 根据 parent_id 设置真正的父子关系
    for menu in &menus {
        if let Some(parent_id) = menu.parent_id {
            if let (Some(child_tree), Some(parent_tree)) =
                (menu_map.get(&menu.id), menu_map.get(&parent_id))
            {
                // 设置子节点的 parent 引用
                let weak_parent = Rc::downgrade(parent_tree);
                child_tree.borrow_mut().parent = Some(weak_parent);

                // 将子节点加入到正确的父节点的 children 集合中
                parent_tree
                    .borrow_mut()
                    .children
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .push(Rc::clone(child_tree));

                // 从根节点的直接子节点中移除，只保留不正确的父节点的子节点
                if let Some(children) = root.borrow_mut().children.as_mut() {
                    let mut children_borrow = children.borrow_mut();
                    let pos = children_borrow
                        .iter()
                        .position(|r| Rc::ptr_eq(r, child_tree));
                    if let Some(idx) = pos {
                        children_borrow.remove(idx);
                    }
                }
            }
        }
    }

    let resp = root.borrow().to_serializable();
    resp
}
