use crate::utils::jwt_util::validate;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Get the authorization header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_value) = auth_header.to_str() {
                    // Extract token from the header
                    if auth_value.starts_with("Bearer ") {
                        let token = &auth_value[7..]; // Remove "Bearer " prefix

                        // Validate the token
                        match validate(token) {
                            Ok(_) => return service.call(req).await, // Valid token, proceed with the request
                            Err(_) => {
                                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
                            }
                        }
                    }
                }
            }

            // If no valid token, respond with unauthorized
            Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
        })
    }
}
