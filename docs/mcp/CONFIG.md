# Anya Core MCP Server Configuration

This document describes the configuration format for the Anya Core MCP (Model Context Protocol) server.

## Configuration File

The MCP server configuration is stored in JSON format. The default configuration file locations (in order of precedence):

1. Path specified in `ANYA_MCP_CONFIG` environment variable
2. `./mcp_config.json` in the current directory
3. `/etc/anya/mcp_config.json`
4. `$CONFIG_DIR/anya/mcp_config.json` (platform-specific user config directory)

## Configuration Format

```json
{
  "mcpServers": {
    "anya-core": {
      "command": "cargo",
      "args": [
        "run",
        "--bin",
        "anya-core-mcp"
      ],
      "env": {
        "RUST_LOG": "debug"
      },
      "config": {
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
    }
  }
}
```

### Configuration Fields

#### Server Configuration

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Name of the MCP server instance |
| `version` | string | Version of the server |
| `transport` | object | Transport configuration |
| `resources` | array | List of available resources |

#### Transport Configuration

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Transport type: "http" or "stdio" |
| `port` | number | (Optional) Port number for HTTP transport |

#### Resource Configuration

| Field | Type | Description |
|-------|------|-------------|
| `uri` | string | Unique URI for the resource |
| `name` | string | Human-readable name |
| `type` | string | Resource type: "file", "directory", "service", or "tool" |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `ANYA_MCP_CONFIG` | Path to configuration file |
| `RUST_LOG` | Logging level (error, warn, info, debug, trace) |

## Example Usage

1. Create configuration file:
   ```bash
   mkdir -p ~/.config/anya
   cp mcp_config.json ~/.config/anya/
   ```

2. Run the server:
   ```bash
   cargo run --bin anya-core-mcp
   ```

## Integration with Other MCP Servers

The configuration file supports multiple MCP servers. Common integrations:

- Sequential Thinking Server
- GitHub MCP Server
- Custom Tool Servers

Each server can be configured with its own command, arguments, environment variables, and configuration.

## Security Considerations

1. Avoid exposing sensitive information in the configuration file
2. Use appropriate file permissions
3. When using HTTP transport, consider network security
4. Validate resource URIs and access controls
