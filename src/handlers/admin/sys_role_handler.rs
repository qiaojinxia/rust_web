use crate::common::resp::ApiError;
use crate::common::resp::ApiResponse;
use crate::config::globals;
use crate::create_response;
use crate::dto::admin::common_dto::PaginationQueryDto;
use crate::dto::admin::sys_role_dto;
use crate::dto::admin::sys_role_dto::{RoleDeleteRespDto, RolesDeleteRespDto};
use crate::services::admin::sys_role_services;
use actix_web::ResponseError;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use validator::Validate;

// 创建角色
#[post("/roles")]
pub async fn create_role(
    app_state: web::Data<globals::AppState>,
    role_create_dto: web::Json<sys_role_dto::RoleCreationDto>,
) -> impl Responder {
    // 首先验证DTO
    if let Err(errors) = role_create_dto.0.validate() {
        return create_response!(Err::<sys_role_dto::RoleCreationResponseDto, ApiError>(
            ApiError::InvalidArgument(errors.to_string())
        ));
    }

    // 尝试创建角色
    let result = sys_role_services::create_role(
        &*app_state.mysql_conn,
        "admin".to_string(),
        role_create_dto.into_inner(),
    )
    .await
    .map(|role_resp| role_resp) // 返回创建后的完整数据
    .map_err(|error| ApiError::BadRequest(error.to_string()));

    create_response!(result)
}

// 获取角色列表
#[get("/roles")]
pub async fn get_roles(
    app_state: web::Data<globals::AppState>,
    web::Query(info): web::Query<PaginationQueryDto>,
) -> impl Responder {
    let current = info.current.unwrap_or(1);
    let size = info.size.unwrap_or(10);

    let result = sys_role_services::get_roles(&*app_state.mysql_conn, current as u32, size as u32)
        .await
        .map_err(|error| ApiError::BadRequest(error.to_string()));

    create_response!(result)
}

// Route to get all roles
#[get("/roles-options")]
pub async fn get_roles_options(app_state: web::Data<globals::AppState>) -> impl Responder {
    let result = sys_role_services::get_all_roles(&*app_state.mysql_conn)
        .await
        .map_err(|error| ApiError::BadRequest(error.to_string()));

    create_response!(result)
}

// 获取单个角色
#[get("/roles/{code}")]
pub async fn get_role_by_code(
    app_state: web::Data<globals::AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let role_code = path.into_inner();

    let result = sys_role_services::get_role_by_code(&*app_state.mysql_conn, role_code)
        .await
        .map(|opt_role_resp| opt_role_resp) // 返回完整的角色响应数据
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// 更新角色
#[put("/roles/{code}")]
pub async fn update_role(
    app_state: web::Data<globals::AppState>,
    path: web::Path<String>,
    role_update_dto: web::Json<sys_role_dto::RoleUpdateDto>,
) -> impl Responder {
    let role_code = path.into_inner();

    // 使用早返回处理验证错误
    if let Err(e) = role_update_dto.0.validate() {
        return create_response!(Err::<sys_role_dto::RoleCreationResponseDto, ApiError>(
            ApiError::InvalidArgument(e.to_string())
        ));
    }

    // 将业务逻辑处理结果映射到响应
    let result = sys_role_services::update_role(
        &*app_state.mysql_conn,
        role_code,
        role_update_dto.into_inner(),
        "admin".to_string(),
    )
    .await
    .map(|role_resp| role_resp) // 返回更新后的完整数据
    .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

// 删除角色
#[delete("/roles/{code}")]
pub async fn delete_role(
    app_state: web::Data<globals::AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let role_code = path.into_inner();
    let result: Result<Option<RoleDeleteRespDto>, ApiError>;
    match sys_role_services::delete_role(&*app_state.mysql_conn, role_code.clone()).await {
        Ok(rows) if rows > 0 => {
            result = Ok(Some(RoleDeleteRespDto {
                role_code: Some(role_code),
            }));
        }
        Ok(_) => {
            result = Err(ApiError::NotFound("Role not found".to_string()));
        }
        Err(error) => {
            result = Err(ApiError::InternalServerError(error.to_string()));
        }
    }
    create_response!(result)
}

//批量删除角色
#[delete("/roles")]
pub async fn delete_roles(
    app_state: web::Data<globals::AppState>,
    role_codes: web::Json<Vec<String>>,
) -> impl Responder {
    let role_codes = role_codes.into_inner();
    let result: Result<Option<RolesDeleteRespDto>, ApiError>;

    match sys_role_services::delete_roles(&*app_state.mysql_conn, role_codes.clone()).await {
        Ok(rows) if rows > 0 => {
            result = Ok(Some(RolesDeleteRespDto {
                deleted_role_codes: role_codes,
            }));
        }
        Ok(_) => {
            result = Err(ApiError::NotFound("Roles not found".to_string()));
        }
        Err(error) => {
            result = Err(ApiError::InternalServerError(error.to_string()));
        }
    }
    create_response!(result)
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_role)
        .service(get_roles)
        .service(get_roles_options)
        .service(get_role_by_code)
        .service(update_role)
        .service(delete_roles)
        .service(delete_role);
}
