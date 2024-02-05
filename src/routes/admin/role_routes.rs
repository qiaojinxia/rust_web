use actix_web::{delete, get, put, post, web, HttpResponse, Responder};
use validator::{Validate};
use crate::services::admin::sys_role_services;
use crate::common::resp::ApiResponse;
use crate::common::resp::ApiError;
use crate::create_response;
use crate::dto::admin::{role_dto};
use actix_web::ResponseError;
use crate::config::globals;

// 创建角色
#[post("/roles")]
pub async fn create_role(
    app_state: web::Data<globals::AppState>,
    role_create_dto: web::Json<role_dto::RoleCreationDto>
) -> impl Responder {
    let rs: Result<role_dto::RoleCreationResponseDto, ApiError>;
    match role_create_dto.0.validate() {
        Ok(_) => {
            let role = sys_role_services::create_role(&*app_state.mysql_conn, role_create_dto.role_name.clone().unwrap(),
                                                      role_create_dto.role_id.clone().unwrap(), role_create_dto.description.clone().unwrap()).await;
            match role {
                Ok(r) => {
                    rs = Ok(role_dto::RoleCreationResponseDto {
                        base: role_dto::RoleDto {
                            id: Some(r.id),
                            role_name: Some(r.role_name),
                            role_code: Some(r.role_code),
                            description: r.description,
                            status: r.status,
                        },
                    })
                }
                Err(errors) => {
                    rs = Err(ApiError::BadRequest(errors.to_string()))
                }
            }
        }
        Err(errors) => {
            rs = Err(ApiError::BadRequest(errors.to_string()))
        }
    }
    create_response!(rs)
}


// 获取角色列表
#[get("/roles")]
pub async fn get_roles(
    app_state: web::Data<globals::AppState>,
) -> impl Responder {
    let rs: Result<role_dto::RolesResponseDto, ApiError>;
    let roles = sys_role_services::get_roles(&*app_state.mysql_conn).await;
    match roles {
        Ok(roles) => {
            let vec_b: Vec<role_dto::RoleDto> = roles.into_iter().map(|s| role_dto::RoleDto{
                id: Some(s.id),
                role_name: Some(s.role_name),
                role_code: Some(s.role_code),
                description: s.description,
                status: s.status,
            }).collect();
            rs = Ok(role_dto::RolesResponseDto{list:vec_b});
        },
        Err(errors) => {
            rs = Err(ApiError::BadRequest(errors.to_string()));
        },
    }
    create_response!(rs)
}
//
// // 获取单个角色
// #[get("/roles/{id}")]
// pub async fn get_role_by_id(
//     db: web::Data<Arc<DatabaseConnection>>,
//     path: web::Path<i32>,
// ) -> impl Responder {
//     let role_id = path.into_inner();
//     let role = sys_role_services::get_role_by_id(&*db, role_id).await;
//     match role {
//         Ok(Some(role)) => HttpResponse::Ok().json(ApiResponse::from(role)),
//         Ok(None) => HttpResponse::NotFound().json(ApiResponse::from_error(ApiError::from(DbErr::RecordNotfound))),
//         Err(e) => HttpResponse::InternalServerError().json(ApiResponse::from_error(ApiError::from(e))),
//     }
// }
//

// // 更新角色
// #[put("/roles/{id}")]
// pub async fn update_role(
//     db: web::Data<Arc<DatabaseConnection>>,
//     path: web::Path<i32>,
//     role_name: web::Json<Option<String>>,
//     description: web::Json<Option<String>>,
// ) -> impl Responder {
//     let role_id = path.into_inner();
//     let role = sys_role_services::update_role(&*db, role_id, role_name.into_inner(), description.into_inner()).await;
//     match role {
//         Ok(Some(role)) => HttpResponse::Ok().json(ApiResponse::from(role)),
//         Ok(None) => HttpResponse::NotFound().json(ApiResponse::from_error(ApiError::from(DbErr::RecordNotfound))),
//         Err(e) => HttpResponse::InternalServerError().json(ApiResponse::from_error(ApiError::from(e))),
//     }
// }
//
// // 删除角色
// #[delete("/roles/{id}")]
// pub async fn delete_role(
//     db: web::Data<Arc<DatabaseConnection>>,
//     path: web::Path<i32>,
// ) -> impl Responder {
//     let role_id = path.into_inner();
//     let affected_rows = sys_role_services::delete_role(&*db, role_id).await;
//     match affected_rows {
//         Ok(rows) => {
//             if rows > 0 {
//                 HttpResponse::NoContent().finish()
//             } else {
//                 HttpResponse::NotFound().json(ApiResponse::from_error(ApiError::from(DbErr::RecordNotfound)))
//             }
//         },
//         Err(e) => HttpResponse::InternalServerError().json(ApiResponse::from_error(ApiError::from(e))),
//     }
// }

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_role)
            .service(get_roles)
            // .service(get_role_by_id)
            // .service(update_role)
            // .service(delete_role)
    );
}
