use actix_web::{get, post, Responder, web};
use crate::common::resp::{ApiError, ApiResponse};
use crate::config::globals;
use crate::create_response;
use crate::dto::admin::common_dto;
use crate::dto::admin::common_dto::PaginationResponseDto;
use crate::services::admin::{sys_user_role_services, sys_user_services};
use actix_web::HttpResponse;
use actix_web::ResponseError;
use validator::Validate;
use crate::dto::admin::sys_user_dto::{UserCreateDto, UserCreateRespDto, UserWithRolesDto};


#[post("/add")]
pub async fn create_user(
    app_state: web::Data<globals::AppState>,
    user_create_dto: web::Json<UserCreateDto>
) -> impl Responder {
    let result: Result<UserCreateRespDto, ApiError>;
    if let Err(err) = user_create_dto.0.validate() {
        result = Err(ApiError::InvalidArgument(err.to_string()));
        return create_response!(result);
    }
    let roles =  user_create_dto.user_roles.clone();

    // 创建用户
    let user = sys_user_services::create_user(
        &*app_state.mysql_conn, user_create_dto.into_inner(),
        "admin".to_string()).await.unwrap();
    match roles {
        None => {}
        Some(ref r) => {
            // 给用户添加权限
             match sys_user_role_services::assign_roles_to_user(
                &*app_state.mysql_conn, user.id, r.clone(),"admin".to_string()).await{
                Err(err) => {
                    result = Err(ApiError::InternalServerError(err.to_string()));
                    return create_response!(result);
                }
                _ => {}
            }
        }
    }
    result = Ok(UserCreateRespDto{base: UserWithRolesDto::from((user,roles))});
    // 创建并返回AuthResponse
    create_response!(result)
}


#[get("/users/roles")]
pub async fn get_users_with_roles(
    app_state: web::Data<globals::AppState>,
    query: web::Query<common_dto::PaginationQueryDto>,
) -> impl Responder {
    let current_page = query.page.unwrap_or(1);
    let page_size = query.size.unwrap_or(10);
    let result:Result<PaginationResponseDto<UserWithRolesDto>,ApiError>;
    // 查询总条数
    let total_count = match sys_user_services::get_total_users_count(&app_state.mysql_conn).await {
        Ok(count) => count,
        Err(error) => return  {
            result = Err(ApiError::InternalServerError(error.to_string()));
            create_response!(result)
        }
    };
    match sys_user_services::get_users_with_roles(&app_state.mysql_conn, current_page as usize, page_size as usize).await {
        Ok(users_with_roles) => {
            let response = PaginationResponseDto::new(
                current_page, page_size, total_count as usize as u64, users_with_roles);
            result = Ok(response)
        },
        Err(error) => result = Err(ApiError::InternalServerError(error.to_string())),
    }
    create_response!(result)
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_users_with_roles)
        .service(create_user);
}
