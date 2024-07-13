use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use crate::common::error::MyError;
use crate::dto::admin::sys_route_dto::{RoleMenuResponseDto, RouteDto};
use crate::schemas::admin::{sea_orm_active_enums, sys_menu, sys_permission_target, sys_role, sys_role_permission};
use sea_orm::QueryFilter;
use serde_json::json;

// 通用函数：构建菜单树
fn build_menu_tree(menus: Vec<sys_menu::Model>) -> Vec<Rc<RefCell<RouteDto>>> {
    // 将菜单详情映射到 HashMap，以便快速查找
    let mut menu_map: HashMap<i32, Rc<RefCell<RouteDto>>> = HashMap::new();
    for menu in &menus {
        let mut new_meta = menu.meta.clone().unwrap_or_else(|| json!({}));
        new_meta["title"] = json!(menu.menu_name.clone().unwrap_or_default());
        let route_dto = RouteDto {
            id: menu.id,
            name: menu.menu_name.clone().unwrap_or_default(),
            path: menu.route_path.clone().unwrap_or_default(),
            component: menu.component.clone(),
            meta: Some(new_meta),
            children: None,
        };
        menu_map.insert(menu.id, Rc::new(RefCell::new(route_dto)));
    }

    // 构建菜单树
    let mut roots: Vec<Rc<RefCell<RouteDto>>> = Vec::new();
    for menu in menus {
        if let Some(parent_id) = menu.parent_id {
            if let Some(parent) = menu_map.get(&parent_id) {
                let mut parent_borrow = parent.borrow_mut();
                let children = parent_borrow.children.get_or_insert(Vec::new());
                if let Some(child) = menu_map.get(&menu.id) {
                    children.push(child.clone());
                }
            }
        } else {
            if let Some(root) = menu_map.get(&menu.id) {
                roots.push(root.clone());
            }
        }
    }

    roots
}

// 根据角色代码获取菜单
pub async fn get_menus_by_role_code(
    db: &DatabaseConnection,
    role_code: &str
) -> Result<RoleMenuResponseDto, MyError> {
    // 步骤1: 获取角色ID
    let role = sys_role::Entity::find()
        .filter(sys_role::Column::RoleCode.eq(role_code))
        .one(db)
        .await?
        .ok_or(MyError::NotFound("Role not found".to_string()))?;

    // 步骤2: 获取角色权限ID
    let permissions = sys_role_permission::Entity::find()
        .filter(sys_role_permission::Column::RoleId.eq(role.id))
        .all(db)
        .await?;

    let permission_ids: HashSet<i32> = permissions.into_iter().map(|rp| rp.permission_id).collect();

    // 步骤3: 获取权限对应的菜单ID
    let permission_targets = sys_permission_target::Entity::find()
        .filter(sys_permission_target::Column::PermissionId.is_in(permission_ids))
        .filter(sys_permission_target::Column::TargetType.eq(sea_orm_active_enums::TargetType::Menu))
        .all(db)
        .await?;

    let menu_ids: HashSet<i32> = permission_targets.into_iter().map(|pt| pt.target_id).collect();

    // 步骤4: 获取菜单详情
    let menus = sys_menu::Entity::find()
        .filter(sys_menu::Column::Id.is_in(menu_ids))
        .filter(sys_menu::Column::Constant.eq(false))
        .all(db)
        .await?;

    let roots = build_menu_tree(menus);

    let role_menu_resp = RoleMenuResponseDto{
        home: "home".to_string(),
        routes: roots,
    };
    Ok(role_menu_resp)
}

// 获取常量路由
pub async fn get_constant_menus(
    db: &DatabaseConnection,
) -> Result<Vec<Rc<RefCell<RouteDto>>>, MyError> {

    let menus = sys_menu::Entity::find()
        .filter(sys_menu::Column::Constant.eq(true))
        .all(db)
        .await?;

    let roots = build_menu_tree(menus);

    let role_menu_resp = RoleMenuResponseDto{
        home: "home".to_string(),
        routes: roots,
    };
    Ok(role_menu_resp.routes)
}
