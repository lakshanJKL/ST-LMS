use std::rc::Rc;
use std::task::{Context, Poll};
use futures::future::{ok, LocalBoxFuture, Ready};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use crate::utill::jwt::extract_and_check_role_from_token;

pub struct JwtMiddleware;

impl<S> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<EitherBody<BoxBody>>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody>>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<EitherBody<BoxBody>>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let path = req.path().to_string();

        Box::pin(async move {
            // Exclude certain paths from middleware
            if path == "/signup" || path == "/login" {
                return srv.call(req).await;
            }

            if extract_and_check_role_from_token(&req) {
                return srv.call(req).await;
            }

            // Unauthorized response for missing/invalid token
            let response = HttpResponse::Unauthorized()
                .insert_header(("content-type", "text/plain"))
                .body("Unauthorized: Missing or invalid token");

            let (req_parts, _) = req.into_parts();
            let service_response =
                ServiceResponse::new(req_parts, response.map_into_right_body());

            Ok(service_response)
        })
    }
}
