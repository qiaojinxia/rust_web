use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::{Expr};
use serde_json::{json};
use crate::dto::admin::sys_menu_dto::{MenuCreateDto, MenuTreeResponseDto, MenuUpdateDto};
use crate::schemas::admin::{sys_menu};
use crate::schemas::admin::prelude::{SysMenu};
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use serde::Deserialize;
use crate::common::error::MyError;
use crate::schemas::admin;

//create_menu 创建菜单
pub async fn create_menu(
    db: &DatabaseConnection,
    menu_create_req: MenuCreateDto,
    create_user: String,
) -> Result<sys_menu::Model, MyError> {
    let menu_name = menu_create_req.menu_name.
        ok_or(MyError::ValidationError("menu_name is required".to_string()))?;
    let route_path = menu_create_req.route_path.
        ok_or(MyError::ValidationError("route_path is required".to_string()))?;
    let parent_id = match menu_create_req.parent_id {
        0 => None,
        id => Some(id),
    };
    let mut menu = sys_menu::ActiveModel {
        menu_name: Set(menu_name),
        r#type: Set(admin::sea_orm_active_enums::Type::from_string(menu_create_req.menu_type.as_str())?),
        route_path: Set(route_path),
        route_name: Set(menu_create_req.route_name),
        parent_id: Set(parent_id),
        create_user: Set(create_user),
        status: Set(menu_create_req.status.parse::<i8>().unwrap()),
        is_hidden: Set(i8::from(menu_create_req.is_hidden)),
        create_time: Set(Some(Utc::now())),
        sort: Set(menu_create_req.order),
        component: Set(Some(menu_create_req.component)),
        ..Default::default()
    };

    let mut meta = json!({"icon": menu_create_req.icon});

    let meta_obj = meta.as_object_mut().
        ok_or(MyError::ValidationError("Failed to create meta object".to_string()))?;

    meta_obj.insert("icon_type".to_string(), json!(menu_create_req.icon_type));

    if let Some(i18n_key) = menu_create_req.i18n_key {
        meta_obj.insert("i18n_key".to_string(), json!(i18n_key));
    }

    if let Some(layout) = menu_create_req.layout {
        meta_obj.insert("layout".to_string(), json!(layout));
    }else{
        meta_obj.insert("layout".to_string(), json!("base".to_string()));
    }


    menu.meta = Set(Some(meta));

    menu.insert(db).await.map_err(MyError::from)
}

//get_menus 获取菜单列表
pub async fn get_menus(
    db: &DatabaseConnection,
) -> Result<Vec<sys_menu::Model>, DbErr> {
    SysMenu::find().all(db).await
}


// 修改get_menus函数以支持分页
pub async fn get_menus_paged(
    db: &DatabaseConnection,
    page: u64, // 当前页码，从1开始
    page_size: u64, // 每页条目数
) -> Result<(Vec<sys_menu::Model>, u64), MyError> {
    // 使用.find()开始构建查询
    let paginator = SysMenu::find()
        .paginate(db, page_size); // 设置每页条目数
    let num_pages = paginator.num_pages().await?; // 获取总页数

    let menus = paginator
        .fetch_page(page - 1).await?; // 获取指定页的结果，页码从0开始，所以这里需要减1

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
    menu_id:i32,
    menu_update_req:MenuUpdateDto,

) -> Result<Option<sys_menu::Model>, MyError> {
    let mut menu: sys_menu::ActiveModel =
        SysMenu::find_by_id(menu_id).one(db).await?.unwrap().into();

    let mut meta = json!({"": ""});
    // 记录meta的初始状态
    let initial_meta = meta.clone();
    let meta_obj = meta.as_object_mut().
        ok_or(MyError::ValidationError("Failed to create meta object".to_string()))?;
    if let Some(icon) = menu_update_req.icon {
        meta_obj.insert("icon".to_string(), json!(icon));
    }
    if let Some(icon_type) = menu_update_req.icon_type {
        meta_obj.insert("icon_type".to_string(), json!(icon_type));
    }
    if let Some(i18n_key) = menu_update_req.i18n_key {
        meta_obj.insert("i18n_key".to_string(), json!(i18n_key));
    }
    if let Some(layout) = menu_update_req.layout {
        meta_obj.insert("layout".to_string(), json!(layout));
    }
    if let Some(menu_name) = menu_update_req.menu_name {
        menu.menu_name = Set(menu_name);
    }
    if let Some(component) = menu_update_req.component {
        menu.component = Set(Some(component));
    }
    if let Some(menu_type) = menu_update_req.menu_type {
        menu.r#type = Set(admin::sea_orm_active_enums::Type::from_string(menu_type.as_str())?); // 需要转换为期望的数据类型
    }
    if let Some(route_name) = menu_update_req.route_name {
        menu.route_name = Set(route_name);
    }
    if let Some(route_path) = menu_update_req.route_path {
        menu.route_path = Set(route_path);
    }
    menu.parent_id = Set(menu_update_req.parent_id);
    if let Some(status) = menu_update_req.status {
        menu.status = Set(status.parse().unwrap_or_default()); // 需要转换为期望的数据类型
    }
    if let Some(is_hidden) = menu_update_req.is_hidden {
        menu.is_hidden = Set(is_hidden as i8); // 根据需要转换布尔值
    }
    if let Some(order) = menu_update_req.order {
        menu.sort = Set(order);
    }
    if let Some(id) = menu_update_req.parent_id {
        if id != 0 {
            menu.parent_id = Set(Some(id));
        } else {
            menu.parent_id = Set(None); // or whatever logic you want when id is 0
        }
    }
    if meta != initial_meta {
        menu.meta = Set(Some(meta));
    }
    menu.update(db).await.map(Some).map_err(MyError::from)
}

//delete_menu 删除菜单
pub async fn delete_menu(
    db: &DatabaseConnection,
    menu_id: i32,
) -> Result<u64, DbErr> {
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

pub async fn delete_menus(
    db: &DatabaseConnection,
    menu_ids: Vec<i32>,
) -> Result<u64, DbErr> {
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


#[derive( Clone, Deserialize)]
pub struct MenuTree {
    pub id: i32,
    #[serde(rename = "pId")]
    pub parent_id: Option<Weak<RefCell<MenuTree>>>,
    pub label: String,
    pub children: Option<RefCell<Vec<Rc<RefCell<MenuTree>>>>>,
}
impl MenuTree {
    // 添加一个方法来转换为可序列化的结构体
    fn to_serializable(&self) -> MenuTreeResponseDto {
        MenuTreeResponseDto {
            id: self.id,
            label: self.label.clone(),
            children: self.children
                .as_ref()
                .map_or(vec![], |children| {
                    children.borrow().iter().map(|child| {
                        child.borrow().to_serializable()
                    }).collect()
                }),
        }
    }
}
pub fn build_menu_tree(menus: Vec<sys_menu::Model>) -> Option<MenuTreeResponseDto> {
    let mut menu_map: HashMap<i32, Rc<RefCell<MenuTree>>> = HashMap::new();
    let mut root: Option<Rc<RefCell<MenuTree>>> = None;

    for menu in menus.iter() {
        let menu_tree_dto = Rc::new(RefCell::new(MenuTree {
            id: menu.id,
            parent_id: None,
            label: menu.menu_name.clone(),
            children: Some(RefCell::new(Vec::new())),
        }));
        menu_map.insert(menu.id, Rc::clone(&menu_tree_dto));
    }

    for menu in menus.iter() {
        if let Some(parent_id) = menu.parent_id {
            if let Some(parent) = menu_map.get(&parent_id) {
                let child = menu_map.get(&menu.id).unwrap();

                child.borrow_mut().parent_id = Some(Rc::downgrade(&parent));

                if let Some(children) = parent.borrow_mut().children.as_mut() {
                    children.borrow_mut().push(Rc::clone(&child));
                } else {
                    parent.borrow_mut().children = Some(RefCell::new(vec![Rc::clone(&child)]));
                }
            }
        } else {
            if root.is_some() {
                // Handle the case where there is more than one root. Options include logging an error or choosing the first one.
                // For now, let's assume we only keep the first root.
                continue;
            }
            root = Some(menu_map.get(&menu.id).unwrap().clone());
        }
    }

    root.map(|tree_rc| tree_rc.borrow().to_serializable())
}
