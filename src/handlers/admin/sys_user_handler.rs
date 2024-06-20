use crate::common::resp::{ApiError, ApiResponse};
use crate::config::globals;
use crate::create_response;
use crate::dto::admin::common_dto;
use crate::dto::admin::common_dto::PaginationResponseDto;
use crate::services::admin::{sys_user_role_services, sys_user_services};
use actix_web::ResponseError;
use actix_web::{delete, put, HttpResponse};
use actix_web::{get, post, web, Responder};
use validator::Validate;
use crate::dto::admin::sys_user_dto::{
    UserCreateDto, UserCreateRespDto, UserUpdateDto, UserWithRolesDto,
};

#[post("/users")]
pub async fn create_user(
    app_state: web::Data<globals::AppState>,
    user_create_dto: web::Json<UserCreateDto>,
) -> impl Responder {
    let result: Result<UserCreateRespDto, ApiError>;

    // Validate the incoming request
    if let Err(err) = user_create_dto.0.validate() {
        result = Err(ApiError::InvalidArgument(err.to_string()));
        return create_response!(result);
    }

    let roles = user_create_dto.user_roles.clone();

    // Create the user
    let user = match sys_user_services::create_user(
        &*app_state.mysql_conn,
        user_create_dto.into_inner(),
        "admin".to_string(),
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            result = Err(ApiError::InternalServerError(err.to_string()));
            return create_response!(result);
        }
    };

    // Assign roles to the user if roles are provided
    if let Some(ref r) = roles {
        if let Err(err) = sys_user_role_services::assign_roles_to_user(
            &*app_state.mysql_conn,
            user.id,
            r.clone(),
            "admin".to_string(),
        )
        .await
        {
            result = Err(ApiError::InternalServerError(err.to_string()));
            return create_response!(result);
        }
    }

    // Create the response DTO
    result = Ok(UserCreateRespDto {
        base: UserWithRolesDto::from((user, roles)),
    });

    // Create and return the response
    create_response!(result)
}

#[get("/users")]
pub async fn get_users_with_roles(
    app_state: web::Data<globals::AppState>,
    query: web::Query<common_dto::PaginationQueryDto>,
) -> impl Responder {
    let current_page = query.current.unwrap_or(1);
    let page_size = query.size.unwrap_or(10);
    let result: Result<PaginationResponseDto<UserWithRolesDto>, ApiError>;
    // 查询总条数
    let total_count = match sys_user_services::get_total_users_count(&app_state.mysql_conn).await {
        Ok(count) => count,
        Err(error) => {
            return {
                result = Err(ApiError::InternalServerError(error.to_string()));
                create_response!(result)
            }
        }
    };
    match sys_user_services::get_users_with_roles(
        &app_state.mysql_conn,
        current_page as usize,
        page_size as usize,
    )
    .await
    {
        Ok(users_with_roles) => {
            let response = PaginationResponseDto::new(
                current_page,
                page_size,
                total_count as usize as u64,
                users_with_roles,
            );
            result = Ok(response)
        }
        Err(error) => result = Err(ApiError::InternalServerError(error.to_string())),
    }
    create_response!(result)
}

#[put("/users/{user_id}")]
pub async fn update_user(
    app_state: web::Data<globals::AppState>,
    user_id: web::Path<i32>,
    user_update_dto: web::Json<UserUpdateDto>,
) -> impl Responder {
    let result: Result<UserWithRolesDto, ApiError>;

    // Validate the incoming request
    if let Err(err) = user_update_dto.validate() {
        result = Err(ApiError::InvalidArgument(err.to_string()));
        return create_response!(result);
    }
    let roles = user_update_dto.user_roles.clone();
    // Update the user
    let updated_user = match sys_user_services::update_user(
        &*app_state.mysql_conn,
        *user_id,
        user_update_dto.into_inner(),
        "admin".to_string(),
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            result = Err(ApiError::InternalServerError(err.to_string()));
            return create_response!(result);
        }
    };

    // Optionally, update roles if provided
    if let Some(roles) = roles {
        if let Err(err) = sys_user_role_services::assign_roles_to_user(
            &*app_state.mysql_conn,
            user_id.into_inner(),
            roles,
            "admin".to_string(),
        )
        .await
        {
            result = Err(ApiError::InternalServerError(err.to_string()));
            return create_response!(result);
        }
    }

    // Return the updated user data
    result = Ok(UserWithRolesDto::from((updated_user.unwrap(), None)));
    create_response!(result)
}

#[delete("/users/{user_id}")]
pub async fn delete_user_handler(
    app_state: web::Data<globals::AppState>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let result: Result<u64, ApiError> =
        sys_user_services::delete_user(&app_state.mysql_conn, *user_id)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()));

    create_response!(result)
}

#[delete("/users")]
pub async fn batch_delete_users_handler(
    app_state: web::Data<globals::AppState>,
    user_ids: web::Json<Vec<i32>>,
) -> impl Responder {
    let result: Result<u64, ApiError> =
        sys_user_services::batch_delete_users(&app_state.mysql_conn, user_ids.into_inner())
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()));

    create_response!(result)
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users_with_roles)
        .service(create_user)
        .service(update_user)
        .service(delete_user_handler)
        .service(batch_delete_users_handler);
}
