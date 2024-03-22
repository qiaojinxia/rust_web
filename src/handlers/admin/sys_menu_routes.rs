use actix_web::{delete, get, put, post, web, HttpResponse, Responder};
use validator::Validate;
use crate::services::admin::sys_menu_services; // 假设您已经有一个处理数据库逻辑的服务层
use crate::common::resp::{ApiResponse, ApiError};
use crate::config::globals;
use crate::create_response;
use actix_web::ResponseError;
// 假设您已经定义了一个宏来简化响应的创建
use crate::dto::admin::sys_menu_dto::{MenuCreationDto, MenuUpdateDto, MenuDto,
                                      MenusResponseDto, MenuCreationResponseDto,
                                      MenuUpdateResponseDto, MenuDeleteResponseDto,
                                      MenuBaseDto};
#[post("/menus")]
pub async fn create_menu(
    app_state: web::Data<globals::AppState>,
    menu_creation_dto: web::Json<MenuCreationDto>,
) -> impl Responder {
    if let Err(errors) = menu_creation_dto.validate() {
        return create_response!(Err::<MenuCreationResponseDto, ApiError>(ApiError::InvalidArgument(errors.to_string())));
    }
    let result = sys_menu_services::create_menu(
        &*app_state.mysql_conn,
        menu_creation_dto.into_inner(), "admin".to_string())
        .await
        .map(|menu| MenuCreationResponseDto {
            base: MenuBaseDto::from(menu)
        })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[get("/menus")]
pub async fn get_menus(
    app_state: web::Data<globals::AppState>,
) -> impl Responder {
    let result = sys_menu_services::get_menus(&*app_state.mysql_conn).await
        .map(|menus| MenusResponseDto {
            list: menus.into_iter().map(|menu| MenuDto {
                base: MenuBaseDto::from(menu)
            }).collect(),
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
    let result = sys_menu_services::get_menu_by_id(&*app_state.mysql_conn, menu_id).await
        .map(|menu| MenuDto {
            base: MenuBaseDto::from(menu.unwrap())
        })
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
        return create_response!(Err::<MenuUpdateResponseDto, ApiError>(ApiError::InvalidArgument(errors.to_string())));
    }

    let result = sys_menu_services::update_menu(
        &*app_state.mysql_conn,
        menu_id,
        menu_update_dto.into_inner(),
    )
        .await
        .map(|menu| MenuUpdateResponseDto {
            base: MenuBaseDto::from(menu.unwrap())
        })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[delete("/menus/{id}")]
pub async fn delete_menu(
    app_state: web::Data<globals::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let menu_id = path.into_inner();
    let result = sys_menu_services::delete_menu(&*app_state.mysql_conn, menu_id).await
        .map(|success| MenuDeleteResponseDto { success:success != 0 })
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(create_menu)
        .service(get_menus)
        .service(get_menu_by_id)
        .service(update_menu)
        .service(delete_menu);
}