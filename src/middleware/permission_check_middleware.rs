use crate::common::auth::jwt::Claims;
use crate::common::resp::create_error_response;
use crate::config::globals;
use crate::services::admin::sys_role_permission_services::get_menus_by_role_id;
use crate::services::admin::sys_role_services::get_role_ids_by_role_codes;
use actix_service::{Service, Transform};
use actix_web::http::StatusCode;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web, Error, HttpMessage};
use futures::future::{ready, Ready};
use futures::Future;
use std::sync::Arc;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct PermissionCheck;

pub struct PermissionCheckMiddleware<S> {
    service: Arc<S>,
}

impl<S, B> Transform<S, ServiceRequest> for PermissionCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PermissionCheckMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionCheckMiddleware {
            service: Arc::new(service),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for PermissionCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone the database connection or the relevant part of app_state here. Assuming app_state.mysql_conn is an Arc or similar.
        let mysql_conn = req
            .app_data::<web::Data<globals::AppState>>()
            .unwrap()
            .mysql_conn
            .clone();
        let service = self.service.clone();

        let path = req.path().to_string();

        let roles = req
            .extensions()
            .get::<Claims>()
            .unwrap_or(&Claims::new())
            .role_codes
            .clone();

        Box::pin(async move {
            if roles.is_empty() {
                return Err(create_error_response(
                    "Authorization Failed: No Roles Found",
                    StatusCode::UNAUTHORIZED,
                ));
            }
            // 首先根据角色代码获取角色ID
            match get_role_ids_by_role_codes(&*mysql_conn, roles).await {
                Ok(role_ids) => {
                    // 然后根据角色ID获取菜单信息
                    match get_menus_by_role_id(&*mysql_conn, role_ids).await {
                        Ok(permissions) => {
                            // 检查请求路径是否在用户的权限内
                            let allowed =
                                permissions.iter().any(|perm| path.starts_with(&perm.route));
                            if allowed {
                                let fut = service.call(req);
                                fut.await
                            } else {
                                Err(create_error_response(
                                    "Authorization Failed",
                                    StatusCode::UNAUTHORIZED,
                                ))
                            }
                        }
                        Err(_) => Err(create_error_response(
                            "Authorization Failed",
                            StatusCode::UNAUTHORIZED,
                        )),
                    }
                }
                Err(_) => Err(create_error_response(
                    "Authorization Failed",
                    StatusCode::UNAUTHORIZED,
                )),
            }
        })
    }
}
