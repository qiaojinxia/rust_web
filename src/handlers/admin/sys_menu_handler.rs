use actix_web::{delete, get, put, post, web, HttpResponse, Responder};
use validator::Validate;
use crate::services::admin::sys_menu_services; // 假设您已经有一个处理数据库逻辑的服务层
use crate::common::resp::{ApiResponse, ApiError};
use crate::config::globals;
use crate::create_response;
use actix_web::ResponseError;
use actix_web::web::service;
use crate::dto::admin::common_dto::{PaginationQueryDto, PaginationResponseDto};
// 假设您已经定义了一个宏来简化响应的创建
use crate::dto::admin::sys_menu_dto::{MenuCreateDto, MenuUpdateDto,
                                      MenuCreationResponseDto,
                                      MenuUpdateResponseDto, MenuDeleteResponseDto,
                                      MenuBaseRespDto};
#[post("/menus")]
pub async fn create_menu(
    app_state: web::Data<globals::AppState>,
    menu_create_dto: web::Json<MenuCreateDto>,
) -> impl Responder {
    if let Err(errors) = menu_create_dto.0.validate() {
        return create_response!(
            Err::<MenuCreationResponseDto, ApiError>(ApiError::InvalidArgument(errors.to_string())));
    }
    let result = sys_menu_services::create_menu(
        &*app_state.mysql_conn,
        menu_create_dto.into_inner(), "admin".to_string())
        .await
        .map(|menu| MenuBaseRespDto::from(menu))
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[get("/menus/paged")]
pub async fn get_menus_paged(
    app_state: web::Data<globals::AppState>,
    web::Query(info): web::Query<PaginationQueryDto>,
) -> impl Responder {
    let current = info.current.unwrap_or(1);
    let page_size = info.size.unwrap_or(10);

    let result = sys_menu_services::get_menus_paged(
        &*app_state.mysql_conn, current, page_size).await
        .map(|(menus, total_menus)| {
            PaginationResponseDto::new(current, page_size, total_menus, menus.into_iter()
                .map(|menu| MenuBaseRespDto::from(menu)).collect::<Vec<MenuBaseRespDto>>())
        })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));
    create_response!(result)
}

#[get("/menus")]
pub async fn get_menus(
    app_state: web::Data<globals::AppState>,
) -> impl Responder {
    let result = sys_menu_services::get_menus(&*app_state.mysql_conn).await
        .map(|menus| {
            PaginationResponseDto::new(1, 10, menus.len() as u64,
            menus.into_iter().map(|menu| MenuBaseRespDto::from(menu)).collect::<Vec<MenuBaseRespDto>>())
        })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[get("/menus/{id}")]
pub async fn get_menu_by_id(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let menu_id = path.into_inner();
    let result = sys_menu_services::get_menu_by_id(
        &*app_state.mysql_conn, menu_id).await
        .map(|menu| MenuBaseRespDto::from(menu.unwrap()))
        .map_err(|error| ApiError::NotFound(error.to_string()));

    create_response!(result)
}

#[put("/menus/{id}")]
pub async fn update_menu(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
    menu_update_dto: web::Json<MenuUpdateDto>,
) -> impl Responder {
    let menu_id = path.into_inner();
    if let Err(errors) = menu_update_dto.validate() {
        return create_response!(Err::<MenuUpdateResponseDto, ApiError>
            (ApiError::InvalidArgument(errors.to_string())));
    }

    let result = sys_menu_services::update_menu(
        &*app_state.mysql_conn,
        menu_id,
        menu_update_dto.into_inner(),
    )
        .await
        .map(|menu|MenuBaseRespDto::from(menu.unwrap()))
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[delete("/menus/{id}")]
pub async fn delete_menu(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let menu_id = path.into_inner();
    let result = sys_menu_services::delete_menu(
        &*app_state.mysql_conn, menu_id).await
        .map(|success| MenuDeleteResponseDto { success:success != 0 })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[delete("/menus")]
pub async fn delete_menus(
    app_state: web::Data<globals::AppState>,
    menu_ids: web::Json<Vec<i32>>,
) -> impl Responder {
    let menu_ids = menu_ids.into_inner();
    let result = sys_menu_services::delete_menus(
        &*app_state.mysql_conn, menu_ids).await
        .map(|success| MenuDeleteResponseDto { success:success != 0 })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_menu)
        .service(get_menus)
        .service(get_menus_paged)
        .service(get_menu_by_id)
        .service(update_menu)
        .service(delete_menu)
        .service(delete_menus);
}