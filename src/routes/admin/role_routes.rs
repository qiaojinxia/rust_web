use actix_web::{delete, get, put, post, web, HttpResponse, Responder};
use validator::{Validate};
use crate::services::admin::sys_role_services;
use crate::common::resp::ApiResponse;
use crate::common::resp::ApiError;
use crate::create_response;
use actix_web::ResponseError;
use crate::config::globals;
use crate::dto::admin::role_dto;
use crate::dto::admin::role_dto::RoleDeleteRespDto;


// 创建角色
#[post("/roles")]
pub async fn create_role(
    app_state: web::Data<globals::AppState>,
    role_create_dto: web::Json<role_dto::RoleCreationDto>,
) -> impl Responder {
    // 首先验证DTO
    if let Err(errors) = role_create_dto.0.validate() {
        return create_response!(Err::<role_dto::RoleCreationResponseDto, ApiError>(ApiError::InvalidArgument(errors.to_string())));
    }

    // 尝试创建角色
    let result = sys_role_services::create_role(
        &*app_state.mysql_conn,
        "admin".to_string(),
      role_create_dto.into_inner()
    )
        .await
        .map(|r| role_dto::RoleCreationResponseDto {
            base: role_dto::RoleDto {
                id: Some(r.id),
                role_name: Some(r.role_name),
                role_code: Some(r.role_code),
                description: r.description,
                status: r.status,
            },
        })
        .map_err(|error| ApiError::BadRequest(error.to_string()));

    create_response!(result)
}


// 获取角色列表
#[get("/roles")]
pub async fn get_roles(
    app_state: web::Data<globals::AppState>,
) -> impl Responder {
    let result = sys_role_services::get_roles(&*app_state.mysql_conn).await
        .map(|roles| {
            roles.into_iter().map(|role| role_dto::RoleDto {
                id: Some(role.id),
                role_name: Some(role.role_name),
                role_code: Some(role.role_code),
                description: role.description,
                status: role.status,
            }).collect::<Vec<role_dto::RoleDto>>()
        })
        .map(|vec_b| role_dto::RolesResponseDto { list: vec_b })
        .map_err(|error| ApiError::BadRequest(error.to_string()));

    create_response!(result)
}


// 获取单个角色
#[get("/roles/{id}")]
pub async fn get_role_by_id(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let role_id = path.into_inner();

    let result = sys_role_services::get_role_by_id(&*app_state.mysql_conn, role_id).await
        .map(|opt_role| {
            opt_role.map(|role| role_dto::RoleDto {
                id: Some(role.id),
                role_name: Some(role.role_name),
                role_code: Some(role.role_code),
                description: role.description,
                status: role.status,
            })
        })
        .map(|role_dto_opt| role_dto::RoleResponseDto { role: role_dto_opt })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}


// 更新角色
#[put("/roles/{id}")]
pub async fn update_role(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
    role_update_dto: web::Json<role_dto::RoleUpdateDto>,
) -> impl Responder {
    let role_id = path.into_inner();

    // 使用早返回处理验证错误
    if let Err(e) = role_update_dto.0.validate() {
        return create_response!(Err::<Option<role_dto::RoleUpdateRespDto>, ApiError>(ApiError::InvalidArgument(e.to_string())));
    }

    // 将业务逻辑处理结果映射到响应
    let result = sys_role_services::update_role(&*app_state.mysql_conn,
                                                role_id, role_update_dto.into_inner()).await
        .map(|opt_role| {
            opt_role.map(|role| role_dto::RoleUpdateRespDto {
                role: Some(role_dto::RoleDto {
                    id: Some(role.id),
                    role_name: Some(role.role_name),
                    role_code: Some(role.role_code),
                    description: role.description,
                    status: role.status,
                })
            })
        })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}


// 删除角色
#[delete("/roles/{id}")]
pub async fn delete_role(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let role_id = path.into_inner();
    let result:Result<Option<RoleDeleteRespDto>,ApiError>;
    match sys_role_services::delete_role(&*app_state.mysql_conn, role_id).await {
        Ok(rows) if rows > 0 => { result = Ok(None);},
        Ok(_) => { result = Err(ApiError::NotFound("Role not found".to_string())); },
        Err(error) => { result = Err(ApiError::InternalServerError(error.to_string())); },
    }
    create_response!(result)
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_role)
            .service(get_roles)
            .service(get_role_by_id)
            .service(update_role)
            .service(delete_role)
    );
}
