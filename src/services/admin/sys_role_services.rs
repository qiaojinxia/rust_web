use std::collections::{HashMap, HashSet};
use crate::dto::admin::common_dto::PaginationResponseDto;
use crate::dto::admin::sys_role_dto::{ RoleCreationDto, RoleCreationResponseDto, RoleDto, RoleMenuResponseDto, RoleOptionDto, RoleUpdateDto, RouteDto};
use crate::schemas::admin::prelude::SysRole;
use crate::schemas::admin::{sea_orm_active_enums, sys_menu, sys_permission_target, sys_role, sys_role_permission};
use sea_orm::sea_query::{MysqlQueryBuilder, Query};
use sea_orm::ActiveValue::Set;
use sea_orm::PaginatorTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::{ConnectionTrait, QueryFilter, QuerySelect, Statement, TransactionTrait};
use crate::common::error::MyError;

//create_role 创建角色
pub async fn create_role(
    db: &DatabaseConnection,
    create_user: String,
    role_create_info: RoleCreationDto,
) -> Result<RoleCreationResponseDto, DbErr> {
    // Start a transaction
    let txn = db.begin().await?;

    // Insert the role
    let role = sys_role::ActiveModel {
        role_name: Set(role_create_info.role_name.clone()),
        description: Set(Some(role_create_info.role_desc.clone())),
        role_code: Set(role_create_info.role_code.clone()),
        status: Set(role_create_info.status.parse().unwrap()),
        create_user: Set(create_user.clone()),
        ..Default::default()
    };

    let inserted_role = role.insert(&txn).await?;

    // Insert role permissions
    if let Some(permission_ids) = role_create_info.permission_ids {
        for permission_id in permission_ids {
            let role_permission = sys_role_permission::ActiveModel {
                role_id: Set(inserted_role.id),
                permission_id: Set(permission_id),
                create_user: Set(create_user.clone()),
                ..Default::default()
            };

            role_permission.insert(&txn).await?;
        }
    }

    // Commit the transaction
    txn.commit().await?;

    // Query inserted role with permissions
    let permissions = sys_role_permission::Entity::find()
        .filter(sys_role_permission::Column::RoleId.eq(inserted_role.id))
        .all(db)
        .await?;

    let permission_ids = permissions
        .into_iter()
        .map(|rp| rp.permission_id)
        .collect::<Vec<i32>>();

    let role_dto = RoleDto {
        id: Some(inserted_role.id),
        role_code: Some(inserted_role.role_code),
        role_name: Some(inserted_role.role_name),
        permission_ids: Some(permission_ids),
        role_desc: inserted_role.description,
        status: inserted_role.status.to_string(),
    };

    Ok(RoleCreationResponseDto { base: role_dto })
}

//get_roles 获取角色列表
// src/services/sys_role_services.rs
pub async fn get_roles(
    db: &DatabaseConnection,
    current: u32,
    size: u32,
) -> Result<PaginationResponseDto<RoleDto>, DbErr> {
    let offset = (current - 1) * size;
    let roles = sys_role::Entity::find()
        .limit(size as u64)
        .offset(offset as u64)
        .all(db)
        .await?;

    let total = sys_role::Entity::find().count(db).await?; // 查询总数

    let mut role_all_dto = Vec::new();

    for role in roles {
        let permissions = sys_role_permission::Entity::find()
            .filter(sys_role_permission::Column::RoleId.eq(role.id))
            .all(db)
            .await?;

        let permission_ids = permissions
            .into_iter()
            .map(|rp| rp.permission_id)
            .collect::<Vec<i32>>();

        let mut role_dto = RoleDto::from(role);
        role_dto.permission_ids = Some(permission_ids);

        role_all_dto.push(role_dto);
    }

    Ok(PaginationResponseDto::new(
        current as u64,
        size as u64,
        total,
        role_all_dto,
    ))
}

//get_role_by_id 获取单个角色
pub async fn get_role_by_id(
    db: &DatabaseConnection,
    role_id: i32,
) -> Result<Option<RoleCreationResponseDto>, DbErr> {
    // Find the role by ID
    if let Some(role) = sys_role::Entity::find_by_id(role_id).one(db).await? {
        // Find permissions associated with the role
        let permissions = sys_role_permission::Entity::find()
            .filter(sys_role_permission::Column::RoleId.eq(role.id))
            .all(db)
            .await?;

        let permission_ids = permissions
            .into_iter()
            .map(|rp| rp.permission_id)
            .collect::<Vec<i32>>();

        let mut role_dto = RoleDto::from(role);
        role_dto.permission_ids = Some(permission_ids);

        Ok(Some(RoleCreationResponseDto { base: role_dto }))
    } else {
        Ok(None)
    }
}

pub async fn update_role(
    db: &DatabaseConnection,
    role_id: i32,
    role_update_info: RoleUpdateDto,
) -> Result<RoleCreationResponseDto, DbErr> {
    // Start a transaction
    let txn = db.begin().await?;

    // Find the role by ID
    let role_opt = SysRole::find_by_id(role_id).one(&txn).await?;
    let mut role: sys_role::ActiveModel = match role_opt {
        Some(role) => role.into(),
        None => return Err(DbErr::RecordNotFound("Role not found".to_string())),
    };

    // Update the role fields
    if let Some(rn) = role_update_info.role_name {
        role.role_name = Set(rn);
    }
    if let Some(dsc) = role_update_info.role_desc {
        role.description = Set(Some(dsc));
    }
    if let Some(code) = role_update_info.role_code {
        role.role_code = Set(code);
    }
    if let Some(status) = role_update_info.status {
        role.status = Set(status.parse().unwrap());
    }
    role.update(&txn).await?;

    // Update role permissions
    if let Some(permission_ids) = role_update_info.permission_ids {
        // Remove existing permissions
        sys_role_permission::Entity::delete_many()
            .filter(sys_role_permission::Column::RoleId.eq(role_id))
            .exec(&txn)
            .await?;

        // Add new permissions
        for permission_id in permission_ids {
            let role_permission = sys_role_permission::ActiveModel {
                role_id: Set(role_id),
                permission_id: Set(permission_id),
                create_user: Set("update_user".to_string()), // Update this to the actual user if needed
                ..Default::default()
            };

            role_permission.insert(&txn).await?;
        }
    }

    // Commit the transaction
    txn.commit().await?;

    // Query the updated role with permissions
    let updated_role = SysRole::find_by_id(role_id).one(db).await?.unwrap();
    let permissions = sys_role_permission::Entity::find()
        .filter(sys_role_permission::Column::RoleId.eq(updated_role.id))
        .all(db)
        .await?;

    let permission_ids = permissions
        .into_iter()
        .map(|rp| rp.permission_id)
        .collect::<Vec<i32>>();

    let mut role_dto = RoleDto::from(updated_role);
    role_dto.permission_ids = Some(permission_ids);

    Ok(RoleCreationResponseDto { base: role_dto })
}

//delete_role 删除角色
pub async fn delete_role(db: &DatabaseConnection, role_id: i32) -> Result<u64, DbErr> {
    let role = sys_role::ActiveModel {
        id: Set(role_id),
        ..Default::default()
    };
    SysRole::delete(role)
        .exec(db)
        .await
        .map(|res| res.rows_affected)
}

// src/services/sys_role_services.rs
pub async fn get_all_roles(db: &DatabaseConnection) -> Result<Vec<RoleOptionDto>, DbErr> {
    let roles = sys_role::Entity::find()
        .select_only()
        .column(sys_role::Column::Id)
        .column(sys_role::Column::RoleCode)
        .column(sys_role::Column::RoleName)
        .into_tuple::<(i32, String, String)>()
        .all(db)
        .await?;

    let role_all_dto: Vec<RoleOptionDto> = roles.into_iter().map(RoleOptionDto::from).collect();

    Ok(role_all_dto)
}

pub async fn delete_roles(db: &DatabaseConnection, role_ids: Vec<i32>) -> Result<u64, DbErr> {
    // 构建 SQL 删除语句
    let delete_statement = Query::delete()
        .from_table(sys_role::Entity)
        .and_where(sys_role::Column::Id.is_in(role_ids))
        .to_owned();

    // 将查询语句转换为 SQL 语句和参数
    let (sql, values) = delete_statement.build(MysqlQueryBuilder);

    // 执行 SQL 语句
    let result = db
        .execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::MySql,
            &sql,
            values,
        ))
        .await?;

    // 返回影响的行数
    Ok(result.rows_affected())
}

// 根据角色代码获取菜单
pub async fn get_menus_by_role_code(
    db: &DatabaseConnection,
    role_code: &str,
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
        .all(db)
        .await?;



    // 将菜单详情映射到 HashMap，以便快速查找
    let mut menu_map: HashMap<i32, RouteDto> = HashMap::new();
    for menu in &menus {
        let route_dto = RouteDto {
            id: menu.id,
            name: menu.menu_name.clone().unwrap_or_default(),
            path: menu.route_path.clone().unwrap_or_default(),
            component: menu.component.clone(),
            meta: menu.meta.clone(),
            children: None,
        };
        menu_map.insert(menu.id, route_dto);
    }

    // 构建菜单树
    let mut roots: Vec<RouteDto> = Vec::new();
    // for menu in menus {
    //     if let Some(parent_id) = menu.parent_id {
    //         if let Some(mut parent) = menu_map.remove(&menu.id) {
    //             let children = parent.children.get_or_insert(Vec::new());
    //             if let Some(child) = menu_map.remove(&menu.id) {
    //                 children.push(child);
    //             }
    //             menu_map.insert(parent_id, parent);
    //         }
    //     } else {
    //         if let Some(root) = menu_map.remove(&menu.id) {
    //             roots.push(root);
    //         }
    //     }
    // }

    let role_menu_resp = RoleMenuResponseDto{
        home: "home".to_string(),
        routes: roots,
    };
    Ok(role_menu_resp)
}

// 根据 role_code 数组返回所有匹配的 id
pub async fn get_role_ids_by_role_codes(
    db: &DatabaseConnection,
    role_codes: Vec<String>, // role_code 数组
) -> Result<Vec<i32>, DbErr> {
    // 假设 id 类型为 i32
    let roles = SysRole::find()
        .filter(sys_role::Column::RoleCode.is_in(role_codes)) // 使用 is_in 方法来过滤 role_code
        .all(db)
        .await?;

    let ids: Vec<i32> = roles
        .into_iter()
        .map(|role| role.id) // 假设 sys_role::Model 有一个 id 字段
        .collect();

    Ok(ids)
}
