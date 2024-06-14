use crate::dto::admin::common_dto::PaginationResponseDto;
use crate::dto::admin::sys_role_dto::{
    RoleCreationDto, RoleCreationResponseDto, RoleDto, RoleOptionDto, RoleUpdateDto,
};
use crate::schemas::admin::prelude::SysRole;
use crate::schemas::admin::{sys_role, sys_role_permission};
use sea_orm::sea_query::{MysqlQueryBuilder, Query};
use sea_orm::ActiveValue::Set;
use sea_orm::PaginatorTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::{
    ConnectionTrait, IntoActiveModel, QueryFilter, QuerySelect, Statement, TransactionTrait,
};

//create_role 创建角色
pub async fn create_role(
    db: &DatabaseConnection,
    create_user: String,
    role_create_info: RoleCreationDto,
) -> Result<RoleCreationResponseDto, DbErr> {
    let txn = db.begin().await?;

    let role = sys_role::ActiveModel {
        role_name: Set(role_create_info.role_name.clone()),
        description: Set(Some(role_create_info.role_desc.clone())),
        role_code: Set(role_create_info.role_code.clone()),
        status: Set(role_create_info.status.parse().unwrap()),
        create_user: Set(create_user.clone()),
        ..Default::default()
    };

    let inserted_role = role.insert(&txn).await?;

    if let Some(permission_codes) = role_create_info.permission_codes {
        for permission_code in permission_codes {
            let role_permission = sys_role_permission::ActiveModel {
                role_code: Set(inserted_role.role_code.clone()),
                permission_code: Set(permission_code),
                create_user: Set(create_user.clone()),
                ..Default::default()
            };

            role_permission.insert(&txn).await?;
        }
    }

    txn.commit().await?;

    let permissions = sys_role_permission::Entity::find()
        .filter(sys_role_permission::Column::RoleCode.eq(inserted_role.role_code.clone()))
        .all(db)
        .await?;

    let permission_codes = permissions
        .into_iter()
        .map(|rp| rp.permission_code)
        .collect::<Vec<String>>();

    let role_dto = RoleDto {
        id: Some(inserted_role.id),
        role_code: Some(inserted_role.role_code),
        role_name: Some(inserted_role.role_name),
        permission_ids: Some(permission_codes),
        role_desc: inserted_role.description,
        status: inserted_role.status.to_string(),
    };

    Ok(RoleCreationResponseDto { base: role_dto })
}

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

    let total = sys_role::Entity::find().count(db).await?;

    let mut role_all_dto = Vec::new();

    for role in roles {
        let permissions = sys_role_permission::Entity::find()
            .filter(sys_role_permission::Column::RoleCode.eq(role.role_code.clone()))
            .all(db)
            .await?;

        let permission_codes = permissions
            .into_iter()
            .map(|rp| rp.permission_code)
            .collect::<Vec<String>>();

        let mut role_dto = RoleDto::from(role);
        role_dto.permission_ids = Some(permission_codes);

        role_all_dto.push(role_dto);
    }

    Ok(PaginationResponseDto::new(
        current as u64,
        size as u64,
        total,
        role_all_dto,
    ))
}

pub async fn get_role_by_code(
    db: &DatabaseConnection,
    role_code: String,
) -> Result<Option<RoleCreationResponseDto>, DbErr> {
    // Find the role by role_code
    if let Some(role) = sys_role::Entity::find()
        .filter(sys_role::Column::RoleCode.eq(role_code))
        .one(db)
        .await?
    {
        // Find permissions associated with the role using role_code
        let permissions = sys_role_permission::Entity::find()
            .filter(sys_role_permission::Column::RoleCode.eq(role.role_code.clone()))
            .all(db)
            .await?;

        // Collect permission codes
        let permission_codes = permissions
            .into_iter()
            .map(|rp| rp.permission_code)
            .collect::<Vec<String>>();

        // Convert the role to RoleDto and set permission codes
        let mut role_dto = RoleDto::from(role);
        role_dto.permission_ids = Some(permission_codes);

        // Return the role with its permissions
        Ok(Some(RoleCreationResponseDto { base: role_dto }))
    } else {
        // Return None if the role is not found
        Ok(None)
    }
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

pub async fn update_role(
    db: &DatabaseConnection,
    role_code: String,
    role_update_info: RoleUpdateDto,
    update_user: String,
) -> Result<RoleCreationResponseDto, DbErr> {
    // Start a transaction
    let txn = db.begin().await?;

    // Find the role by role_code
    let role_opt = SysRole::find()
        .filter(sys_role::Column::RoleCode.eq(role_code.clone()))
        .one(&txn)
        .await?;

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
        role.role_code = Set(code.clone());
    }
    if let Some(status) = role_update_info.status {
        role.status = Set(status.parse().unwrap());
    }
    role.update(&txn).await?;

    // Update role permissions
    if let Some(permission_codes) = role_update_info.permission_codes {
        // Remove existing permissions
        sys_role_permission::Entity::delete_many()
            .filter(sys_role_permission::Column::RoleCode.eq(role_code.clone()))
            .exec(&txn)
            .await?;

        // Add new permissions
        for permission_code in permission_codes {
            let role_permission = sys_role_permission::ActiveModel {
                role_code: Set(role_code.clone()),
                permission_code: Set(permission_code),
                create_user: Set(update_user.clone()),
                ..Default::default()
            };

            role_permission.insert(&txn).await?;
        }
    }

    // Commit the transaction
    txn.commit().await?;

    // Query the updated role with permissions
    let updated_role = SysRole::find()
        .filter(sys_role::Column::RoleCode.eq(role_code.clone()))
        .one(db)
        .await?
        .unwrap();

    let permissions = sys_role_permission::Entity::find()
        .filter(sys_role_permission::Column::RoleCode.eq(updated_role.role_code.clone()))
        .all(db)
        .await?;

    let permission_codes = permissions
        .into_iter()
        .map(|rp| rp.permission_code)
        .collect::<Vec<String>>();

    let mut role_dto = RoleDto::from(updated_role);
    role_dto.permission_ids = Some(permission_codes);

    Ok(RoleCreationResponseDto { base: role_dto })
}

pub async fn delete_role(db: &DatabaseConnection, role_code: String) -> Result<u64, DbErr> {
    let role_opt = SysRole::find()
        .filter(sys_role::Column::RoleCode.eq(role_code))
        .one(db)
        .await?;
    if let Some(role) = role_opt {
        sys_role_permission::Entity::delete_many()
            .filter(sys_role_permission::Column::RoleCode.eq(role.role_code.clone()))
            .exec(db)
            .await?;
        SysRole::delete(role.into_active_model())
            .exec(db)
            .await
            .map(|res| res.rows_affected)
    } else {
        Ok(0)
    }
}

pub async fn delete_roles(db: &DatabaseConnection, role_codes: Vec<String>) -> Result<u64, DbErr> {
    // Fetch the roles using the provided role codes
    let roles = SysRole::find()
        .filter(sys_role::Column::RoleCode.is_in(role_codes.clone()))
        .all(db)
        .await?;

    // Extract the role IDs from the fetched roles
    let role_ids: Vec<i32> = roles.into_iter().map(|role| role.id).collect();

    // Delete the associated role permissions using the role codes
    sys_role_permission::Entity::delete_many()
        .filter(sys_role_permission::Column::RoleCode.is_in(role_codes))
        .exec(db)
        .await?;

    // Delete the roles using the role IDs
    let delete_statement = Query::delete()
        .from_table(sys_role::Entity)
        .and_where(sys_role::Column::Id.is_in(role_ids))
        .to_owned();

    let (sql, values) = delete_statement.build(MysqlQueryBuilder);

    let result = db
        .execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::MySql,
            &sql,
            values,
        ))
        .await?;

    Ok(result.rows_affected())
}

