use crate::common::resp::{ApiError, ApiResponse};
use crate::config::globals;
use crate::create_response;
use crate::dto::admin::sys_user_role_dto::{
    AssignRolesDto, AssignRolesRespDto, RemoveRoleRespDto, UserRoleDto, UserRolesRespDto,
};
use crate::services::admin::sys_user_role_services::{
    assign_roles_to_user, get_user_roles, remove_role_from_user,
};
use actix_web::ResponseError;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use validator::Validate;

// Assign roles to a user
#[post("/users/{userId}/roles")]
async fn assign_roles(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
    roles_dto: web::Json<AssignRolesDto>,
) -> impl Responder {
    let user_id = path.into_inner();
    if let Err(errors) = roles_dto.0.validate() {
        return create_response!(Err::<AssignRolesRespDto, ApiError>(
            ApiError::InvalidArgument(errors.to_string())
        ));
    }
    let role_codes = roles_dto.into_inner().role_codes;
    let result = assign_roles_to_user(
        &*app_state.mysql_conn,
        user_id,
        role_codes,
        "admin".to_string(),
    )
    .await
    .map(|_| AssignRolesRespDto { success: true })
    .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Get a user's roles
#[get("/users/{userId}/roles")]
async fn get_roles(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let result = get_user_roles(&*app_state.mysql_conn, user_id)
        .await
        .map(|roles| {
            roles
                .iter()
                .map(|role| UserRoleDto::from(role.clone())) // Convert each role to UserRoleDto
                .collect::<Vec<UserRoleDto>>()
        })
        .map(|roles| UserRolesRespDto { roles })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Remove a role from a user
#[delete("/users/{userId}/roles/{roleCode}")]
async fn remove_role(
    app_state: web::Data<globals::AppState>,
    path: web::Path<(i32, String)>,
) -> impl Responder {
    let (user_id, role_code) = path.into_inner();
    let result = remove_role_from_user(&*app_state.mysql_conn, user_id, role_code)
        .await
        .map(|_| RemoveRoleRespDto { success: true })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// Register API handlers in Actix Web app configuration
pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(assign_roles)
        .service(get_roles)
        .service(remove_role);
}
