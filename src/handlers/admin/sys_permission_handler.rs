use actix_web::{delete, get, HttpResponse, post, put, Responder, web};
use crate::common::resp::ApiError;
use crate::config::globals;
use crate::create_response;
use crate::dto::admin::sys_permission_dto::{PermissionCreationDto, PermissionCreationRespDto,
                                            PermissionDeleteRespDto, PermissionDto, PermissionMenuDto,
                                            PermissionRespDto, PermissionUpdateDto, PermissionUpdateRespDto};
use crate::services::admin::sys_permission_services;
use crate::common::resp::ApiResponse;
use actix_web::ResponseError;
use crate::dto::admin::common_dto::{PaginationQueryDto, PaginationResponseDto};


// Create a new permission
#[post("/permissions")]
async fn create_permission(
    app_state: web::Data<globals::AppState>,
    permission_dto: web::Json<PermissionCreationDto>,
) -> impl Responder {
    let permission_req_data = permission_dto.into_inner();
    let result = sys_permission_services::create_permission(
        &*app_state.mysql_conn,
        permission_req_data.permission_code,
        permission_req_data.description.unwrap(),
        "admin".to_string(),
    ).await
        .map(|permission| PermissionCreationRespDto {
            base: PermissionDto::from(permission)
        } )
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
    let result =
        sys_permission_services::get_permissions_with_menus(
        &*app_state.mysql_conn, current as usize, page_size as usize).await
        .map(|(permissions, total)| {
            PaginationResponseDto::new(current, page_size, total as u64, permissions.into_iter()
                .map(|permission| PermissionMenuDto::from(permission)).collect::<Vec<PermissionMenuDto>>())
        })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Get a single permission by ID
#[get("/permissions/{id}")]
async fn get_permission_by_id(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let permission_id = path.into_inner();
    let result = sys_permission_services::get_permission_by_id(
        &*app_state.mysql_conn, permission_id).await
        .map(|permission| PermissionRespDto{
            base: PermissionDto::from(permission.unwrap()),
        }
        )
        .map_err(|error| ApiError::NotFound(error.to_string()));

    create_response!(result)
}

// Update a permission
#[put("/permissions/{id}")]
async fn update_permission(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
    permission_dto: web::Json<PermissionUpdateDto>,
) -> impl Responder {
    let permission_update_req_data = permission_dto.into_inner();
    let permission_id = path.into_inner();
    let result = sys_permission_services::update_permission(
        &*app_state.mysql_conn,
        permission_id,
        permission_update_req_data.permission_code,
        permission_update_req_data.description,
        "admin".to_string(),
    ).await
        .map(|permission| PermissionUpdateRespDto{
            base:None
            // base:PermissionDto::from(permission.unwrap())
        })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Delete a permission
#[delete("/permissions/{id}")]
async fn delete_permission(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let permission_id = path.into_inner();
    let result = sys_permission_services::delete_permission(
        &*app_state.mysql_conn, permission_id).await
        .map(|effects| PermissionDeleteRespDto{
            success: effects > 0
        })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Remember to add these handlers to your Actix Web app configuration
pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_permission)
        .service(get_permissions)
        .service(get_permission_by_id)
        .service(update_permission)
        .service(delete_permission);
}