use actix_web::{delete, get, HttpResponse, post, put, Responder, web};
use crate::common::resp::ApiError;
use crate::config::globals;
use crate::create_response;
use crate::dto::admin::sys_permission_dto::{PermissionCreationDto, PermissionCreationRespDto,
                                            PermissionDeleteRespDto, PermissionDto, PermissionRespDto,
                                            PermissionsRespDto, PermissionUpdateDto, PermissionUpdateRespDto};
use crate::services::admin::sys_permission_services;
use crate::common::resp::ApiResponse;
use actix_web::ResponseError;


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
) -> impl Responder {
    let result = sys_permission_services::get_permissions(
        &*app_state.mysql_conn).await.map(|permissions| permissions.iter()
            .map(|permission| PermissionDto::from(permission.clone())) // 在这里进行解引用
            .collect::<Vec<PermissionDto>>()).map(|permissions|PermissionsRespDto{
            base:permissions,
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
            base:PermissionDto::from(permission.unwrap())
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