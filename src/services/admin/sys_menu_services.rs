use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use serde_json::json;
use crate::dto::admin::sys_menu_dto::{MenuCreationDto, MenuUpdateDto};
use crate::schema::admin::{sys_menu};
use crate::schema::admin::prelude::{SysMenu};
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

//create_menu 创建菜单
pub async fn create_menu(
    db: &DatabaseConnection,
    menu_crate_req:MenuCreationDto,
    create_user:String,
) -> Result<sys_menu::Model, DbErr> {
    let mut menu = sys_menu::ActiveModel {
        menu_name: Set(menu_crate_req.base.name.unwrap()),
        route: Set(menu_crate_req.base.route.unwrap()),
        route_name:Set(menu_crate_req.base.route_name.unwrap()),
        sort: Set(Some(menu_crate_req.base.order)),
        parent_id: Set(menu_crate_req.base.parent_id),
        create_user:Set(create_user),
        status: Set(menu_crate_req.base.status),
        is_hidden: Set(menu_crate_req.base.is_hidden),
        create_time: Set(Some(Utc::now())),
        ..Default::default()
    };

    let mut meta = json!({"icon_type":menu_crate_req.base.icon_type});

    if let Some(v) = menu_crate_req.base.icon{
        if let Some(obj) = meta.as_object_mut() {
            // 使用insert方法添加新的键值对
            obj.insert("icon".to_string(), json!(v));
        }
    }

    if let Some(pid) = menu_crate_req.base.parent_id{
        menu.parent_id = Set(Some(pid));
    }
    menu.meta = Set(Some(meta));

    menu.insert(db).await
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

    if let Some(mn) = menu_update_req.base.name {
        menu.menu_name = Set(mn);
    }

    if let Some(perm_id) = menu_update_req.base.permission_id {
        menu.permission_id = Set(Some(perm_id));
    }
    menu.route = Set(menu_update_req.base.route.unwrap());
    menu.sort = Set(Some(menu_update_req.base.order));
    menu.r#type = Set(Some(menu_update_req.base.menu_type));

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
