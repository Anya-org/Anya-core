#![feature(edition2021)]
// OPSource Web and API Layer
// Replaces FastAPI with Actix Web for better performance and native Rust integration

use actix_web::{
    web, App, HttpServer, HttpResponse, Responder, 
    dev::ServiceRequest, Error, HttpMessage,
    middleware::{Logger, Compress, DefaultHeaders}
};
use chrono::{DateTime, Utc};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

mod routes;
mod websocket;
mod auth;

pub use routes::*;
pub use websocket::*;
pub use auth::*;

/// API server configuration
#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub enable_ssl: bool,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub jwt_secret: String,
    pub cors_allowed_origins: Vec<String>,
    pub enable_compression: bool,
    pub log_level: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
            workers: num_cpus::get(),
            enable_ssl: false,
            cert_file: None,
            key_file: None,
            jwt_secret: "change_me_in_production".to_string(),
            cors_allowed_origins: vec!["*".to_string()],
            enable_compression: true,
            log_level: "info".to_string(),
        }
    }
}

/// JWT Claims for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

/// API Server for OPSource
pub struct ApiServer {
    config: ApiConfig,
}

impl ApiServer {
    /// Create a new API server
    pub fn new(config: ApiConfig) -> Self {
        Self { config }
    }
    
    /// Run the API server
    pub async fn run(&self) -> std::io::Result<()> {
        // Initialize logging
        env_logger::init_from_env(env_logger::Env::new().default_filter_or(&self.config.log_level));
        
        let config = self.config.clone();
        let jwt_secret = Arc::new(config.jwt_secret.clone());
        
        // Configure CORS
        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                if config.cors_allowed_origins.contains(&"*".to_string()) {
                    return true;
                }
                
                let origin_str = match origin.to_str() {
                    Ok(origin) => origin,
                    Err(_) => return false,
                };
                
                config.cors_allowed_origins.iter().any(|allowed| allowed == origin_str)
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .supports_credentials()
            .max_age(3600);
        
        // Create and configure the HTTP server
        let server = HttpServer::new(move || {
            let mut app = App::new()
                .wrap(Logger::default())
                .wrap(cors.clone());
            
            // Add compression if enabled
            if config.enable_compression {
                app = app.wrap(Compress::default());
            }
            
            // Add default headers
            app = app.wrap(
                DefaultHeaders::new()
                    .add(("Server", "OPSource API"))
                    .add(("X-Content-Type-Options", "nosniff"))
            );
            
            // Configure authentication middleware for protected routes
            let jwt_secret = jwt_secret.clone();
            
            // Add routes
            app = app
                // Public routes (no authentication required)
                .service(web::scope("/api/v1/public")
                    .service(routes::status)
                    .service(routes::health)
                    .service(routes::login)
                    .service(routes::register)
                )
                
                // Protected routes (authentication required)
                .service(web::scope("/api/v1")
                    .wrap(auth::JwtAuth::new(jwt_secret.clone()))
                    
                    // User management
                    .service(web::scope("/users")
                        .service(routes::get_user)
                        .service(routes::update_user)
                        .service(routes::delete_user)
                        .service(routes::list_users)
                    )
                    
                    // Bitcoin functionality
                    .service(web::scope("/bitcoin")
                        .service(routes::get_address)
                        .service(routes::create_transaction)
                        .service(routes::get_transaction)
                        .service(routes::list_transactions)
                    )
                    
                    // RGB assets
                    .service(web::scope("/rgb")
                        .service(routes::issue_asset)
                        .service(routes::transfer_asset)
                        .service(routes::list_assets)
                        .service(routes::get_asset)
                    )
                    
                    // RSK smart contracts
                    .service(web::scope("/rsk")
                        .service(routes::deploy_contract)
                        .service(routes::call_contract)
                        .service(routes::get_contract)
                    )
                    
                    // Stacks smart contracts
                    .service(web::scope("/stacks")
                        .service(routes::deploy_stacks_contract)
                        .service(routes::call_stacks_contract)
                        .service(routes::get_stacks_contract)
                    )
                    
                    // Web5 DID functionality
                    .service(web::scope("/web5")
                        .service(routes::create_did)
                        .service(routes::resolve_did)
                        .service(routes::update_did)
                        .service(routes::verify_credential)
                    )
                    
                    // WebSocket API
                    .service(web::resource("/ws").route(web::get().to(websocket::websocket_handler)))
                )
                
                // The root route
                .service(routes::index);
            
            app
        })
        .workers(self.config.workers);
        
        // Set up with or without SSL
        if self.config.enable_ssl {
            if let (Some(cert_file), Some(key_file)) = (&self.config.cert_file, &self.config.key_file) {
                server
                    .bind_openssl(
                        format!("{}:{}", self.config.host, self.config.port),
                        self.create_ssl_config(cert_file, key_file)?,
                    )?
                    .run()
                    .await
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "SSL is enabled but cert_file or key_file is missing",
                ));
            }
        } else {
            server
                .bind(format!("{}:{}", self.config.host, self.config.port))?
                .run()
                .await
        }
    }
    
    /// Create SSL configuration
    fn create_ssl_config(
        &self,
        cert_file: &str,
        key_file: &str,
    ) -> std::io::Result<openssl::ssl::SslAcceptorBuilder> {
        use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
        
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        builder.set_private_key_file(key_file, SslFiletype::PEM)?;
        builder.set_certificate_chain_file(cert_file)?;
        
        Ok(builder)
    }
    
    /// Generate JWT token
    pub fn generate_token(&self, user_id: &str, name: &str, role: &str, expiry_days: usize) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;
        
        let expiry = now + (expiry_days * 24 * 60 * 60); // Convert days to seconds
        
        let claims = Claims {
            sub: user_id.to_owned(),
            name: name.to_owned(),
            role: role.to_owned(),
            exp: expiry,
            iat: now,
        };
        
        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.config.jwt_secret.as_bytes());
        
        encode(&header, &claims, &encoding_key)
    }
    
    /// Verify JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(self.config.jwt_secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}

/// Test the API server
pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing API server...");
    
    // Create test configuration
    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 9000, // Use a different port for testing
        workers: 1,
        enable_ssl: false,
        cert_file: None,
        key_file: None,
        jwt_secret: "test_secret_key".to_string(),
        cors_allowed_origins: vec!["*".to_string()],
        enable_compression: true,
        log_level: "debug".to_string(),
    };
    
    // Create API server
    let server = ApiServer::new(config);
    
    // Test JWT token generation and verification
    let token = server.generate_token("test_user", "Test User", "user", 1)?;
    println!("✓ Generated JWT token");
    
    let claims = server.verify_token(&token)?;
    println!("✓ Verified JWT token for user: {}", claims.name);
    
    // We don't start the server in the test, just verify we can create it
    println!("✓ API server test passed");
    
    Ok(())
}
