use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::ActiveValue::Set;
use serde_json::json;
use crate::dto::admin::sys_menu_dto::{MenuCreationDto, MenuUpdateDto};
use crate::schema::admin::{sys_menu};
use crate::schema::admin::prelude::{SysMenu};

//create_menu 创建菜单
pub async fn create_menu(
    db: &DatabaseConnection,
    menu_crate_req:MenuCreationDto,
    create_user:String,
) -> Result<sys_menu::Model, DbErr> {
    let mut menu = sys_menu::ActiveModel {
        menu_name: Set(menu_crate_req.base.name.unwrap()),
        permission_id: Set(Some(menu_crate_req.base.permission_id)),
        route: Set(menu_crate_req.base.route.unwrap()),
        meta: Set(Some(json!({"icon": ""}))),
        sort: Set(Some(menu_crate_req.base.sort)),
        parent_id: Set(menu_crate_req.base.parent_id),
        create_user:Set(create_user),
        status: Set(menu_crate_req.base.status),
        is_hidden: Set(menu_crate_req.base.is_hidden),
        create_time: Set(Some(Utc::now())),
        ..Default::default()
    };
    // if let icon = Some(menu_crate_req.base.icon){
    //     menu.meta = Set(Some(json!({"icon":icon})))
    // }
    menu.insert(db).await
}

//get_menus 获取菜单列表
pub async fn get_menus(
    db: &DatabaseConnection,
) -> Result<Vec<sys_menu::Model>, DbErr> {
    SysMenu::find().all(db).await
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
    menu_update_req:MenuUpdateDto,
    // ... 其他可选字段
) -> Result<Option<sys_menu::Model>, DbErr> {

    let menu_id = menu_update_req.base.id;

    let mut menu: sys_menu::ActiveModel = SysMenu::find_by_id(menu_id).one(db).await?.unwrap().into();

    if let Some(mn) = menu_update_req.base.name {
        menu.menu_name = Set(mn);
    }

    menu.permission_id = Set(Some(menu_update_req.base.permission_id));


    menu.route = Set(menu_update_req.base.route.unwrap());

    menu.sort = Set(Some(menu_update_req.base.sort));

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
    let menu = sys_menu::ActiveModel {
        id: Set(menu_id),
        ..Default::default()
    };
    SysMenu::delete(menu)
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}
