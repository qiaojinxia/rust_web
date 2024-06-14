use crate::common::resp::ApiError;
use crate::common::resp::ApiResponse;
use crate::config::globals;
use crate::create_response;
use crate::dto::admin::common_dto::{PaginationQueryDto, PaginationResponseDto};
use crate::dto::admin::sys_permission_dto::{
    PermissionCreationDto, PermissionCreationRespDto, PermissionDeleteRespDto, PermissionDto,
    PermissionRespDto, PermissionSimpleRespDto, PermissionUpdateDto, PermissionUpdateRespDto,
};
use crate::services::admin::sys_permission_services;
use actix_web::ResponseError;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use validator::Validate;

// Create a new permission
#[post("/permissions")]
async fn create_permission(
    app_state: web::Data<globals::AppState>,
    permission_dto: web::Json<PermissionCreationDto>,
) -> impl Responder {
    if let Err(errors) = permission_dto.0.validate() {
        return create_response!(Err::<PermissionCreationRespDto, ApiError>(
            ApiError::InvalidArgument(errors.to_string())
        ));
    }
    let permission_req_data = permission_dto.into_inner();
    let result = sys_permission_services::create_permission(
        &*app_state.mysql_conn,
        permission_req_data,
        "admin".to_string(),
    )
    .await
    .map(|permission| PermissionCreationRespDto {
        base: PermissionDto::from(permission),
    })
    .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Get all permissions
#[get("/permissions")]
async fn get_permissions(
    app_state: web::Data<globals::AppState>,
    web::Query(info): web::Query<PaginationQueryDto>,
) -> impl Responder {
    let current = info.current.unwrap_or(1);
    let page_size = info.size.unwrap_or(10);

    let result = sys_permission_services::get_paginated_permissions_with_menus_apis(
        &*app_state.mysql_conn,
        current as usize,
        page_size as usize,
    )
    .await
    .map(|permissions| {
        let total = permissions.len(); // 取得返回的权限总数，适用于小数据量，对于大数据量需要另外计算总数
        PaginationResponseDto::new(current, page_size, total as u64, permissions)
    })
    .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Get a single permission by code
#[get("/permissions/{code}")]
async fn get_permission_by_code(
    app_state: web::Data<globals::AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let permission_code = path.into_inner();
    // Call the service to get the permission by code
    let data =
        sys_permission_services::get_permission_by_code(&*app_state.mysql_conn, &permission_code)
            .await;
    let result;
    // Handle the result
    match data {
        Ok(permission) => {
            // Check if permission is None
            if let Some(permission) = permission {
                let response = PermissionRespDto {
                    base: PermissionDto::from(permission),
                };
                result = Ok(response);
            } else {
                result = Err(ApiError::NotFound(format!(
                    "Permission with code {} not found",
                    permission_code
                )));
            }
        }
        Err(error) => {
            result = Err(ApiError::NotFound(error.to_string()));
        }
    }
    create_response!(result)
}

#[get("/permissions-options")]
async fn get_simple_permission(app_state: web::Data<globals::AppState>) -> impl Responder {
    let result = sys_permission_services::get_permissions(&*app_state.mysql_conn)
        .await
        .map(|permissions| {
            permissions
                .iter()
                .map(|permission| PermissionSimpleRespDto {
                    id: permission.id,
                    permission_code: permission.permission_code.clone(),
                    permission_name: permission.permission_name.clone(),
                    status: permission.status.to_string(),
                })
                .collect::<Vec<_>>()
        })
        .map_err(|error| ApiError::NotFound(error.to_string()));
    create_response!(result)
}

// Update a permission
#[put("/permissions/{code}")]
async fn update_permission(
    app_state: web::Data<globals::AppState>,
    path: web::Path<String>,
    permission_dto: web::Json<PermissionUpdateDto>,
) -> impl Responder {
    let permission_update_req_data = permission_dto.into_inner();
    let permission_code = path.into_inner();
    let result = sys_permission_services::update_permission(
        &*app_state.mysql_conn,
        &permission_code,
        permission_update_req_data.base,
        "admin".to_string(),
    )
    .await
    .map(|_| PermissionUpdateRespDto {
        base: None, // base:PermissionDto::from(permission.unwrap())
    })
    .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Delete a permission
#[delete("/permissions/{code}")]
async fn delete_permission(
    app_state: web::Data<globals::AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let permission_code = path.into_inner();
    let result =
        sys_permission_services::delete_permission(&*app_state.mysql_conn, permission_code)
            .await
            .map(|effects| PermissionDeleteRespDto {
                success: effects > 0,
            })
            .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_permission)
        .service(get_permissions)
        .service(get_simple_permission)
        .service(get_permission_by_code)
        .service(update_permission)
        .service(delete_permission);
}
