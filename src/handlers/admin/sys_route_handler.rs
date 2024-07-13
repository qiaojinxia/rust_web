use actix_web::{get, Responder, web};
use crate::common::resp::ApiError;
use crate::config::globals;
use crate::create_response;
use crate::handlers::admin::sys_role_handler::{ create_role };
use crate::services::admin::{sys_route_services};
use actix_web::HttpResponse;
use crate::common::resp::ApiResponse;
use actix_web::ResponseError;

#[get("/get-user-routes")]
pub async fn get_user_menus_by_role_code(
    app_state: web::Data<globals::AppState>,
) -> impl Responder {
    let role_code:&'static str = "admin";
    let result = sys_route_services::get_menus_by_role_code(&*app_state.mysql_conn, &role_code)
        .await
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}

#[get("/get-constant-routes")]
pub async fn get_constant_menus_by_role_code(
    app_state: web::Data<globals::AppState>,
) -> impl Responder {
    let result = sys_route_services::get_constant_menus(&*app_state.mysql_conn)
        .await
        .map_err(|error| ApiError::InternalServerError(error.to_string()));

    create_response!(result)
}


pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_role)
        .service(get_user_menus_by_role_code)
        .service(get_constant_menus_by_role_code);
}
