use std::rc::Rc;
use std::task::{Context, Poll};
use futures::future::{ok, LocalBoxFuture, Ready};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::header::{HeaderName, HeaderValue};
use crate::utill::jwt::verify_token;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: From<BoxBody> + 'static,
{
    type Response = ServiceResponse<B>;
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

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: From<BoxBody> + 'static,
{
    type Response = ServiceResponse<B>;
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
                // Pass the request without checking for authentication
                return srv.call(req).await;
            }

            // Check for the Authorization header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..]; // Extract the token part

                        if verify_token(token).is_ok() {
                            // Token is valid, pass the request to the service
                            return srv.call(req).await;
                        }
                    }
                }
            }

            // Unauthorized response for missing/invalid token
            let response = HttpResponse::Unauthorized()
                .insert_header(("content-type", "text/plain"))
                .body("Unauthorized: Missing or invalid token");

            let (req_parts, _) = req.into_parts(); // Extract request parts
            let service_response = ServiceResponse::new(req_parts, response.map_into_boxed_body());
            let service_response = service_response.map_body(|_, body| B::from(body));

            Ok(service_response)
        })
    }

}
