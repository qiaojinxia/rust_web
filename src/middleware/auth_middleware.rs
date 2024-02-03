use actix_web::{error::ErrorUnauthorized, Error};
use std::future::{ready, Ready};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use futures_util::future::LocalBoxFuture;
use crate::common::resp::ApiResponse;

pub struct JWTAuth;
impl<S, B> Transform<S, ServiceRequest> for JWTAuth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JWTAuthHiMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTAuthHiMiddleware {
            service,
            verification_path: vec!["/api"],
            no_verification_path: vec!["/api/health-checker"],
        }))
    }
}

pub struct JWTAuthHiMiddleware<S> {
    service: S,
    verification_path: Vec<&'static str>,
    no_verification_path: Vec<&'static str>,
}

impl<S, B> JWTAuthHiMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    fn is_need_verification(&self, path: &str) -> bool {
        self.verification_path
            .iter()
            .any(|&vp| path.starts_with(vp))
            && !self
            .no_verification_path
            .iter()
            .any(|&vp| path.starts_with(vp))
    }
}

impl<S, B> Service<ServiceRequest> for JWTAuthHiMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.is_need_verification(req.path()) {
            let authorization = req.headers().get("Authorization");
            if authorization.is_none() {
                let json_error = ApiResponse{
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "api unauthorized err".to_string(),
                    data: (),
                };
                return Box::pin(async move { Err(ErrorUnauthorized(format!("{:?}",json_error))) });
            }

            let authorization = authorization.unwrap().to_str();

            if authorization.is_err() {
                let json_error = ApiResponse{
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: "api unauthorized err".to_string(),
                    data: (),
                };
                return Box::pin(async move { Err(ErrorUnauthorized(format!("{:?}",json_error))) });
            }

            let authorization = authorization.unwrap();

            let _token = &authorization[7..]; // 'Bearer ' + token

            // let token_data = Ok("".to_string());
            //
            // if let Err(err) = token_data {
            //     return Box::pin(async { Err(ErrorUnauthorized(err)) });
            // }
            //
            // let token_data = token_data.unwrap();

            // I need to pass this user_id to the next Handle
            // println!("user_id: {}", &token_data.claims.user_id);
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}