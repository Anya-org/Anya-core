use anya_core::protocols::mcp::{McpServer, McpServerConfig, TransportType, Resource, ResourceType};
use std::path::PathBuf;
use tokio;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    version: String,
    transport: TransportConfig,
    resources: Vec<ResourceConfig>,
}

#[derive(Debug, Deserialize)]
struct TransportConfig {
    #[serde(rename = "type")]
    transport_type: String,
    port: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct ResourceConfig {
    uri: String,
    name: String,
    #[serde(rename = "type")]
    resource_type: String,
}

impl From<ResourceConfig> for Resource {
    fn from(config: ResourceConfig) -> Self {
        Self {
            uri: config.uri,
            name: config.name,
            resource_type: match config.resource_type.as_str() {
                "file" => ResourceType::File,
                "directory" => ResourceType::Directory,
                "service" => ResourceType::Service,
                _ => ResourceType::Tool,
            },
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config_path = get_config_path()?;
    let config: Config = load_config(&config_path)?;

    // Create server configuration
    let server_config = McpServerConfig {
        name: config.name,
        version: config.version,
        transport_type: match config.transport.transport_type.as_str() {
            "http" => TransportType::Http,
            _ => TransportType::Stdio,
        },
    };

    // Initialize server
    let server = McpServer::new(server_config);

    // Add configured resources
    for resource_config in config.resources {
        server.add_resource(resource_config.into()).await?;
    }

    // Initialize and start server
    server.initialize().await?;

    // If using HTTP transport, start the server on the configured port
    if let TransportType::Http = server_config.transport_type {
        if let Some(port) = config.transport.port {
            use anya_core::protocols::mcp::http::HttpTransport;
            let transport = HttpTransport::new(port);
            transport.start_server().await?;
        }
    }

    // Keep the server running
    tokio::signal::ctrl_c().await?;
    Ok(())
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Try environment variable first
    if let Ok(path) = std::env::var("ANYA_MCP_CONFIG") {
        return Ok(PathBuf::from(path));
    }

    // Then try default locations
    let default_paths = vec![
        PathBuf::from("./mcp_config.json"),
        PathBuf::from("/etc/anya/mcp_config.json"),
        dirs::config_dir().map(|p| p.join("anya/mcp_config.json")).unwrap_or_default(),
    ];

    for path in default_paths {
        if path.exists() {
            return Ok(path);
        }
    }

    Err("Could not find configuration file".into())
}

fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}
