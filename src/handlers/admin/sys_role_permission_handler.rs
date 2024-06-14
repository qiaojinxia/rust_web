use crate::common::resp::{ApiError, ApiResponse};
use crate::config::globals;
use crate::create_response;
use crate::dto::admin::sys_role_permission_dto::{
    AssignPermissionsDto, AssignPermissionsRespDto, RemovePermissionRespDto, RolePermissionDto,
    RolePermissionsRespDto,
};
use crate::services::admin::sys_role_permission_services::{
    assign_permissions_to_role, get_role_permissions, remove_permission_from_role,
};
use actix_web::ResponseError;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

// Assign permissions to a role
#[post("/roles/{roleCode}/permissions")]
async fn assign_permissions(
    app_state: web::Data<globals::AppState>,
    path: web::Path<String>,
    permissions_dto: web::Json<AssignPermissionsDto>,
) -> impl Responder {
    let role_code = path.into_inner();
    let result = assign_permissions_to_role(
        &*app_state.mysql_conn,
        role_code,
        permissions_dto.permission_codes.clone(),
        "admin".to_string(),
    )
    .await
    .map(|_| AssignPermissionsRespDto { success: true })
    .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Get a role's permissions
#[get("/roles/{roleCode}/permissions")]
async fn get_permissions(
    app_state: web::Data<globals::AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let role_code = path.into_inner();
    let result = get_role_permissions(&*app_state.mysql_conn, role_code)
        .await
        .map(|permissions| {
            permissions
                .iter()
                .map(|permission| RolePermissionDto::from(permission.clone())) // Adjust mapping as necessary
                .collect::<Vec<RolePermissionDto>>()
        })
        .map(|permissions| RolePermissionsRespDto { permissions })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[delete("/roles/{roleCode}/permissions/{permissionCode}")]
async fn remove_permission(
    app_state: web::Data<globals::AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (role_code, permission_code) = path.into_inner();
    let result = remove_permission_from_role(&*app_state.mysql_conn, role_code, permission_code)
        .await
        .map(|_| RemovePermissionRespDto { success: true })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(assign_permissions)
        .service(get_permissions)
        .service(remove_permission);
}
