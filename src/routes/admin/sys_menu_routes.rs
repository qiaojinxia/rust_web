// use actix_web::{web, HttpResponse, ResponseError, Responder};
// use std::sync::Arc;
// use sea_orm::DatabaseConnection;
// use serde::Deserialize;
// use crate::services::admin::sys_menu;
// use crate::common::{ApiError, ApiResponse};
// use crate::create_response;
// use validator::{Validate, ValidationError};
//
// async fn create_menu_item(
//     db: web::Data<Arc<DatabaseConnection>>,
//     form: web::Form<CreateMenuItemRequest>,
// ) -> impl Responder {
//     let result = sys_menu::create_menu_item(
//         &*db,
//         form.menu_name.clone(),
//         form.permission_id,
//         form.url.clone(),
//         form.sort,
//         form.style.clone(),
//         form.parent_id,
//     )
//         .await
//         .map_err(|_| ApiError::InternalServerError("Failed to create menu item".to_string()));
//
//     create_response!(result)
// }
//
//
// // 更新菜单项
// async fn update_menu_item(
//     db: web::Data<Arc<DatabaseConnection>>,
//     form: web::Form<UpdateMenuItemRequest>, // 假设已定义UpdateMenuItemRequest
// ) -> impl Responder {
//     let result = sys_menu::update_menu_item(
//         &*db,
//         form.id,
//         form.menu_name.clone(),
//         form.permission_id,
//         form.url.clone(),
//         form.sort,
//         form.style.clone(),
//         form.parent_id,
//     )
//         .await
//         .map_err(|_| ApiError::InternalServerError("Failed to update menu item".to_string()));
//
//     create_response!(result)
// }
//
// // 删除菜单项
// async fn delete_menu_item(
//     db: web::Data<Arc<DatabaseConnection>>,
//     menu_id: web::Path<i32>,
// ) -> impl Responder {
//     let result = sys_menu::delete_menu_item(
//         &*db,
//         *menu_id,
//     )
//         .await
//         .map_err(|_| ApiError::InternalServerError("Failed to delete menu item".to_string()));
//
//     create_response!(result)
// }
//
// // 获取菜单项详情
// async fn get_menu_item_by_id(
//     db: web::Data<Arc<DatabaseConnection>>,
//     menu_id: web::Path<i32>,
// ) -> impl Responder {
//     let result = sys_menu::get_menu_item_by_id(
//         &*db,
//         *menu_id,
//     )
//         .await
//         .map_err(|_| ApiError::InternalServerError("Failed to get menu item details".to_string()));
//
//     create_response!(result)
// }
//
// // 获取菜单列表
// async fn get_menu_items(
//     db: web::Data<Arc<DatabaseConnection>>,
// ) -> impl Responder {
//     let result = sys_menu::get_menu_items(&*db)
//         .await
//         .map_err(|_| ApiError::InternalServerError("Failed to get menu items".to_string()));
//
//     create_response!(result)
// }
//
// // 获取菜单项关联的权限
// async fn get_permission_of_menu_item(
//     db: web::Data<Arc<DatabaseConnection>>,
//     menu_id: web::Path<i32>,
// ) -> impl Responder {
//     let result = sys_menu::get_permission_of_menu_item(
//         &*db,
//         *menu_id,
//     )
//         .await
//         .map_err(|_| ApiError::InternalServerError("Failed to get permissions of menu item".to_string()));
//
//     create_response!(result)
// }
//
// // 将此函数添加到您的API配置中
// pub fn api_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/api/menu")
//             .route("/create", web::post().to(create_menu_item))
//             .route("/update", web::put().to(update_menu_item))
//             .route("/delete/{menu_id}", web::delete().to(delete_menu_item))
//             .route("/{menu_id}", web::get().to(get_menu_item_by_id))
//             .route("/list", web::get().to(get_menu_items))
//             .route("/permissions/{menu_id}", web::get().to(get_permission_of_menu_item)),
//     );
// }
