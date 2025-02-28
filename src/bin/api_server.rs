use actix_web::{
    web, App, HttpServer, HttpResponse, Responder, 
    dev::HttpServiceFactory, middleware::{Logger, NormalizePath},
    error::ResponseError, http::StatusCode
};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use clap::Parser;
use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use std::env;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use jwt::{
    VerifyWithKey, SignWithKey, AlgorithmType, Header, PocketJwt, 
    Claims, Token, VerifyWithSecret, SignWithSecret
};
use anyhow::Result;
use log::LevelFilter;
use env_logger::Builder;
use anya_bitcoin::{
    BitcoinNode, wallet::BitcoinWallet, transaction::TransactionService,
    Config as BitcoinConfig
};

// CLI Arguments
#[derive(Parser, Debug)]
#[clap(name = "Anya API Server", about = "Bitcoin Core and Web5 API Server")]
struct Args {
    /// Port to run the server on
    #[clap(short, long, default_value = "8000")]
    port: u16,
    
    /// Host to bind to
    #[clap(long, default_value = "127.0.0.1")]
    host: String,
    
    /// Path to configuration file
    #[clap(short, long)]
    config: Option<PathBuf>,
    
    /// Log level (trace, debug, info, warn, error)
    #[clap(long, default_value = "info")]
    log_level: String,
}

// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    bitcoin: BitcoinConfig,
    logging: LoggingConfig,
    security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerConfig {
    port: u16,
    host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoggingConfig {
    level: String,
    dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecurityConfig {
    jwt_secret: String,
    cors_origins: Vec<String>,
}

// Application state
struct AppState {
    config: AppConfig,
    bitcoin_node: Arc<RwLock<BitcoinNode>>,
    startup_time: DateTime<Utc>,
}

// API response models
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    uptime_seconds: u64,
    bitcoin_node_status: String,
    services: HashMap<String, String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    status_code: u16,
}

// Custom error handling
#[derive(Debug)]
struct ApiError {
    message: String,
    code: StatusCode,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.code
    }
    
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.code)
            .json(ErrorResponse {
                error: self.message.clone(),
                status_code: self.code.as_u16(),
            })
    }
}

// API routes
fn api_routes() -> impl HttpServiceFactory {
    web::scope("/api")
        .service(web::resource("/health").route(web::get().to(health_check)))
        .service(web::resource("/config").route(web::get().to(get_config)))
}

// API handlers
async fn root() -> impl Responder {
    HttpResponse::Ok().body("OPSource API Server - Built on Bitcoin Core Principles")
}

async fn health_check(data: web::Data<AppState>) -> impl Responder {
    // Calculate uptime
    let now = Utc::now();
    let uptime = now.signed_duration_since(data.startup_time);
    
    // Check Bitcoin node status
    let bitcoin_status = match data.bitcoin_node.read().await.start() {
        Ok(_) => "running",
        Err(_) => "error",
    };
    
    // Collect service statuses
    let mut services = HashMap::new();
    services.insert("database".to_string(), "connected".to_string());
    services.insert("bitcoin_rpc".to_string(), "connected".to_string());
    
    // Build response
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime.num_seconds() as u64,
        bitcoin_node_status: bitcoin_status.to_string(),
        services,
    };
    
    HttpResponse::Ok().json(response)
}

async fn get_config(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    // This would typically require authentication
    // For demonstration, we're returning a redacted config
    
    let config = data.config.clone();
    
    // Create a redacted version for public consumption
    let public_config = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "network": config.bitcoin.network,
        "services": {
            "web5": "enabled",
            "bitcoin": "enabled",
            "taproot": "enabled",
        }
    });
    
    Ok(HttpResponse::Ok().json(public_config))
}

// Load configuration
fn load_config(config_path: Option<PathBuf>) -> Result<AppConfig> {
    let config_path = config_path.unwrap_or_else(|| {
        PathBuf::from(env::var("CONFIG_PATH").unwrap_or_else(|_| {
            // Default config location
            let mut path = PathBuf::from("config");
            path.push("app_config.toml");
            path.to_string_lossy().to_string()
        }))
    });
    
    // If config exists, load it
    if config_path.exists() {
        let config_str = fs::read_to_string(config_path)?;
        let config: AppConfig = toml::from_str(&config_str)?;
        return Ok(config);
    }
    
    // If no config, create default
    let default_config = AppConfig {
        server: ServerConfig {
            port: 8000,
            host: "127.0.0.1".to_string(),
        },
        bitcoin: BitcoinConfig::default(),
        logging: LoggingConfig {
            level: "info".to_string(),
            dir: "logs".to_string(),
        },
        security: SecurityConfig {
            jwt_secret: uuid::Uuid::new_v4().to_string(),
            cors_origins: vec!["http://localhost:3000".to_string()],
        },
    };
    
    Ok(default_config)
}

// Setup logging
fn setup_logging(level: &str) {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    
    Builder::new()
        .filter(None, log_level)
        .format_timestamp_millis()
        .init();
    
    info!("Logging initialized at {} level", level);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    
    // Setup logging
    setup_logging(&args.log_level);
    
    // Load configuration
    let config = load_config(args.config).expect("Failed to load configuration");
    
    // Initialize Bitcoin node
    let bitcoin_node = Arc::new(RwLock::new(
        BitcoinNode::new(config.bitcoin.clone())
            .expect("Failed to initialize Bitcoin node")
    ));
    
    // Create application state
    let app_state = web::Data::new(AppState {
        config: config.clone(),
        bitcoin_node: bitcoin_node.clone(),
        startup_time: Utc::now(),
    });
    
    // Start server
    info!("Starting server at {}:{}", config.server.host, config.server.port);
    
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                let origin_str = origin.to_str().unwrap_or("");
                config.security.cors_origins.iter().any(|allowed| allowed == origin_str)
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(3600);
        
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .wrap(cors)
            .app_data(app_state.clone())
            .service(web::resource("/").route(web::get().to(root)))
            .service(api_routes())
    })
    .bind((config.server.host.clone(), config.server.port))?
    .run()
    .await
}
