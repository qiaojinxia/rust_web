use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use serde_json::json;
use crate::dto::admin::sys_menu_dto::{MenuCreateDto, MenuUpdateDto};
use crate::schemas::admin::{sys_menu};
use crate::schemas::admin::prelude::{SysMenu};
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use crate::common::error::MyError;

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

    let mut menu = sys_menu::ActiveModel {
        menu_name: Set(menu_name),
        r#type: Set(menu_create_req.menu_type.parse::<i8>().unwrap()),
        route_path: Set(route_path),
        route_name: Set(menu_create_req.route_name),
        parent_id: Set(menu_create_req.parent_id),
        create_user: Set(create_user),
        status: Set(menu_create_req.status.parse::<i8>().unwrap()),
        is_hidden: Set(menu_create_req.is_hidden as i8),
        create_time: Set(Some(Utc::now())),
        permission_id: Set(menu_create_req.permission_id),
        sort: Set(menu_create_req.order),
        ..Default::default()
    };

    let mut meta = json!({"icon": menu_create_req.icon});

    let meta_obj = meta.as_object_mut().
        ok_or(MyError::ValidationError("Failed to create meta object".to_string()))?;

    meta_obj.insert("icon_type".to_string(), json!(menu_create_req.icon_type));

    if let Some(i18n_key) = menu_create_req.i18n_key {
        meta_obj.insert("i18n_key".to_string(), json!(i18n_key));
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
) -> Result<(Vec<sys_menu::Model>, u64), DbErr> {
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

) -> Result<Option<sys_menu::Model>, DbErr> {

    let mut menu: sys_menu::ActiveModel = SysMenu::find_by_id(menu_id).one(db).await?.unwrap().into();

    // menu.menu_name = Set(menu_update_req.base.menu_name);
    //
    // if let Some(perm_id) = menu_update_req.base.permission_id {
    //     menu.permission_id = Set(Some(perm_id));
    // }
    // menu.route_path = Set(menu_update_req.base.route_path);
    //
    // menu.sort = Set(menu_update_req.base.order);
    //
    // menu.r#type = Set(menu_update_req.base.menu_type);

    if let Some(pid) = menu_update_req.base.parent_id {
        menu.parent_id = Set(Some(pid));
    }

    menu.update(db).await.map(Some)
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
