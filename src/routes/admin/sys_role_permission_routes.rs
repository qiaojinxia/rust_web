use actix_web::{delete, get, post, web, HttpResponse, Responder};
use crate::services::admin::sys_role_permission_services::{assign_permissions_to_role,
                                                           get_role_permissions, remove_permission_from_role};
use crate::dto::admin::sys_role_permission_dto::{AssignPermissionsDto, AssignPermissionsRespDto,
                                                 RolePermissionsRespDto, RemovePermissionRespDto, RolePermissionDto};
use crate::common::resp::{ApiResponse, ApiError};
use crate::config::globals;
use crate::create_response;
use actix_web::ResponseError;

// Assign permissions to a role
#[post("/roles/{roleId}/permissions")]
async fn assign_permissions(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
    permissions_dto: web::Json<AssignPermissionsDto>,
) -> impl Responder {
    let role_id = path.into_inner();
    let result = assign_permissions_to_role(&*app_state.mysql_conn, role_id,
                                            permissions_dto.permission_ids.clone(), "admin".to_string()).await
        .map(|_| AssignPermissionsRespDto { success: true })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Get a role's permissions
#[get("/roles/{roleId}/permissions")]
async fn get_permissions(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let role_id = path.into_inner();
    let result = get_role_permissions(&*app_state.mysql_conn, role_id).await
        .map(|permissions| permissions.iter()
            .map(|permission| RolePermissionDto::from(permission.clone())) // Adjust mapping as necessary
            .collect::<Vec<RolePermissionDto>>())
        .map(|permissions| RolePermissionsRespDto { permissions })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[delete("/roles/{roleId}/permissions/{permissionId}")]
async fn remove_permission(
    app_state: web::Data<globals::AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (role_id, permission_id) = path.into_inner();
    let result = remove_permission_from_role(&*app_state.mysql_conn,
                                             role_id, permission_id).await
        .map(|_| RemovePermissionRespDto { success: true })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}


pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(assign_permissions)
        .service(get_permissions)
        .service(remove_permission);
}
