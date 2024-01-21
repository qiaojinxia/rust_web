use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::schema::admin::{sys_menu};
use crate::schema::admin::prelude::{SysMenu};

//create_menu 创建菜单
pub async fn create_menu(
    db: &DatabaseConnection,
    menu_name: String,
    permission_id: i32,
    url: String,
    sort: i8,
    parent_id: Option<i32>,
    // ... 其他必要的字段
) -> Result<sys_menu::Model, DbErr> {
    let menu = sys_menu::ActiveModel {
        menu_name: Set(menu_name),
        permission_id: Set(Some(permission_id)),
        url: Set(url),
        sort: Set(Some(sort)),
        parent_id: Set(parent_id),
        // ... 设置其他字段
        ..Default::default()
    };
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
    menu_id: i32,
    menu_name: Option<String>,
    permission_id: Option<i32>,
    url: Option<String>,
    sort: Option<i8>,
    parent_id: Option<i32>,
    // ... 其他可选字段
) -> Result<Option<sys_menu::Model>, DbErr> {
    let mut menu: sys_menu::ActiveModel = SysMenu::find_by_id(menu_id).one(db).await?.unwrap().into();

    if let Some(mn) = menu_name {
        menu.menu_name = Set(mn);
    }
    if let Some(pid) = permission_id {
        menu.permission_id = Set(Some(pid));
    }
    if let Some(u) = url {
        menu.url = Set(u);
    }
    if let Some(s) = sort {
        menu.sort = Set(Some(s));
    }
    if let Some(pid) = parent_id {
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
