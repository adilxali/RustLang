use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error};
use actix_service::Service;
use futures::future::{ok, Ready};
use jsonwebtoken::errors::ErrorKind;
use crate::jwt::{decode_jwt, Claims};

pub struct JWTMiddleware;

impl<S, B> actix_service::Transform<S, ServiceRequest> for JWTMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JWTMiddlewareService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Result<Self::Transform, Self::InitError> {
        Ok(JWTMiddlewareService { service })
    }
}

pub struct JWTMiddlewareService<S> {
    service: S,
}

impl<S, B> Service for JWTMiddlewareService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, ctx: &mut std::task::Context<'_>) -> Result<actix_service::Poll<Self::Error>, Self::Error> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    match decode_jwt(token) {
                        Ok(_claims) => return self.service.call(req),
                        Err(e) => {
                            // Handle invalid JWT token
                            eprintln!("Invalid JWT: {:?}", e);
                            return ok(req.error_response(actix_web::HttpResponse::Unauthorized().finish()));
                        }
                    }
                }
            }
        }

        // If there's no Authorization header or it's not a valid JWT, reject the request
        ok(req.error_response(actix_web::HttpResponse::Unauthorized().finish()))
    }
}
