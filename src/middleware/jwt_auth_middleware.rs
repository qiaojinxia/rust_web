use actix_web::{Error, HttpMessage};
use std::future::{Future, ready, Ready};
use std::pin::Pin;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use log::debug;
use crate::common;
use crate::common::resp::{create_error_response};

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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let need_verification = self.is_need_verification(req.path());
        if !need_verification    {
            let fut = self.service.call(req);
            return Box::pin(async move {
                fut.await
            });
        }
        let authorization = match req.headers().get("Authorization").and_then(|hv| hv.to_str().ok()) {
            Some(auth) => auth,
            None => return Box::pin(async { Err(create_error_response("Authorization header missing", StatusCode::UNAUTHORIZED)) }),
        };

        if !authorization.starts_with("Bearer ") {
            return Box::pin(async { Err(create_error_response("Invalid authorization scheme", StatusCode::UNAUTHORIZED)) });
        }

        let token = &authorization["Bearer ".len()..];
        match common::auth::jwt::decode_jwt(token) {
            Ok(jwt_info) if !jwt_info.claims.is_expired() => {
                debug!("user auth success user_name: {} user_role: {:?}", jwt_info.claims.user_name, jwt_info.claims.role_codes);

                req.extensions_mut().insert(jwt_info);

                let fut = self.service.call(req);
                Box::pin(async move {
                    fut.await
                })
            }
            Ok(_) => Box::pin(async { Err(create_error_response("Token expired", StatusCode::UNAUTHORIZED)) }),
            Err(_) => Box::pin(async { Err(create_error_response("Invalid token", StatusCode::UNAUTHORIZED)) }),
        }

    }
}