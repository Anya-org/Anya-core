# MCP Server Integration Guide [AIR-3][AIS-3][BPC-3]

This document outlines how to integrate the standalone MCP (Model Context Protocol) server with the main Anya Core project.

## Overview

The MCP server provides a standardized way for AI models to interact with tools and services. It supports both HTTP and Stdio transports, making it versatile for different deployment scenarios.

## Integration Options

### 1. As a Standalone Service

The standalone MCP server can be run independently from the main Anya Core project. This approach is recommended for:

- Development and testing
- Isolated environments
- When you need to minimize dependencies

#### Setup

```bash
# Clone the repository
git clone https://github.com/anya-im/mcp-server.git

# Build the server
cd mcp-server
cargo build --release

# Run with default configuration
cargo run --release

# Run with specific configuration
ANYA_MCP_CONFIG=/path/to/config.json cargo run --release
```

### 2. As an Integrated Component

The MCP server can be integrated directly into the Anya Core project using the library interface:

#### Cargo.toml

```toml
[dependencies]
anya-mcp-server = { git = "https://github.com/anya-im/mcp-server.git" }
```

#### Code Integration

```rust
use anya_mcp_server::{McpServer, McpServerConfig, TransportType};

async fn start_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    // Create server configuration
    let config = McpServerConfig {
        name: "integrated-mcp-server".into(),
        version: "1.0.0".into(),
        transport_type: TransportType::Http,
    };

    // Initialize server
    let mut server = McpServer::new(config);
    
    // Add resources
    server.add_resource(Resource {
        uri: "anya://core/tools/bitcoin".into(),
        name: "Bitcoin Tools".into(),
        resource_type: ResourceType::Tool,
    }).await?;

    // Initialize and start server
    server.initialize().await?;
    
    Ok(())
}
```

## Configuration

Both standalone and integrated deployments use the same configuration format. See [CONFIG.md](CONFIG.md) for details.

Example configuration:

```json
{
  "name": "anya-core-mcp",
  "version": "1.0.0",
  "transport": {
    "type": "http",
    "port": 8080
  },
  "resources": [
    {
      "uri": "anya://core/tools/bitcoin",
      "name": "Bitcoin Tools",
      "type": "tool"
    }
  ]
}
```

## Security Considerations

1. **Authentication**: When using HTTP transport, implement proper authentication mechanisms.
2. **Resource Validation**: Always validate resource URIs before processing.
3. **Sensitive Data**: Be careful with resource paths that might contain sensitive information.

## Testing Integration

To verify the integration is working correctly:

```bash
# Test HTTP transport
curl -X POST http://localhost:8080/request -d '{"method":"ping"}'

# Expected response
{"status":"success"}
```

## Troubleshooting

1. **Connection Issues**: Ensure the server is running and the port is accessible.
2. **Configuration Problems**: Verify the configuration file format and paths.
3. **Resource Not Found**: Check that resources are properly registered.

## Further Reading

- [MCP Protocol Specification](https://modelcontextprotocol.github.io/specification/)
- [Anya Core Architecture](../architecture/SYSTEM_MAP.md)
