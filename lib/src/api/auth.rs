#![feature(edition2021)]
// OPSource API Authentication Module
// Implements JWT-based authentication middleware for Actix Web

use crate::api::Claims;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::{self, HeaderValue},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// JWT Authentication middleware for Actix Web
pub struct JwtAuth {
    jwt_secret: Arc<String>,
}

impl JwtAuth {
    /// Create new JWT authentication middleware
    pub fn new(jwt_secret: Arc<String>) -> Self {
        Self { jwt_secret }
    }
}

// Middleware factory implementation
impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
        })
    }
}

/// JWT Authentication middleware service
pub struct JwtAuthMiddleware<S> {
    service: S,
    jwt_secret: Arc<String>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract authorization header
        let auth_header = req.headers().get(header::AUTHORIZATION);
        let jwt_secret = self.jwt_secret.clone();
        let service = self.service.clone();

        Box::pin(async move {
            // Verify JWT token
            match auth_header {
                Some(auth_value) => {
                    let auth_str = auth_value.to_str().map_err(|_| {
                        ErrorUnauthorized("Invalid authorization header format")
                    })?;

                    // Extract token from "Bearer <token>"
                    let token = match auth_str.strip_prefix("Bearer ") {
                        Some(token) => token,
                        None => return Err(ErrorUnauthorized("Invalid token format")),
                    };

                    // Decode and validate token
                    let token_data = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(jwt_secret.as_bytes()),
                        &Validation::default(),
                    )
                    .map_err(|_| ErrorUnauthorized("Invalid token"))?;

                    // Add the claims to the request extensions for handlers to access
                    req.extensions_mut().insert(token_data.claims);
                    service.call(req).await
                }
                None => Err(ErrorUnauthorized("Missing authorization header")),
            }
        })
    }
}

/// Extract user ID from request
pub fn get_user_id(req: &ServiceRequest) -> Option<String> {
    req.extensions()
        .get::<Claims>()
        .map(|claims| claims.sub.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{ApiServer, ApiConfig};
    use actix_web::{http::StatusCode, test, web, App, HttpResponse};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[actix_web::test]
    async fn test_jwt_auth() {
        // Create a test server
        let server = ApiServer::new(ApiConfig::default());
        
        // Generate a test token
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
            
        let token = server.generate_token("test_user", "Test User", "user", 1)
            .unwrap();
            
        // Create a test endpoint
        let app = test::init_service(
            App::new()
                .service(
                    web::scope("/protected")
                        .wrap(JwtAuth::new(Arc::new(server.config.jwt_secret.clone())))
                        .route("", web::get().to(|| async { HttpResponse::Ok().body("protected") }))
                )
                .service(
                    web::scope("/public")
                        .route("", web::get().to(|| async { HttpResponse::Ok().body("public") }))
                )
        ).await;
        
        // Test public endpoint (no auth required)
        let req = test::TestRequest::get().uri("/public").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Test protected endpoint with valid token
        let req = test::TestRequest::get()
            .uri("/protected")
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Test protected endpoint without token
        let req = test::TestRequest::get()
            .uri("/protected")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
