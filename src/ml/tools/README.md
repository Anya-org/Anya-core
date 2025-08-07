# Machine Learning Tools Module

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

## Introduction

The ML Tools module provides a comprehensive framework for integrating external tools with AI agents, enabling complex workflows and enhanced capabilities. It allows AI systems to safely interact with external systems, execute commands, and process data through a standardized interface.

## Features

- Unified tool execution framework
- Security and safety controls for tool usage
- Asynchronous tool execution with timeouts
- Parameter validation and type checking
- Tool capability management
- Execution history and logging
- Permission-based access control

## Core Components

### ToolResult

Results from tool execution:

```rust
pub struct ToolResult {
    pub tool_id: String,
    pub execution_id: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub metadata: HashMap<String, String>,
}
```

### ToolParameter

Definition of parameters accepted by tools:

```rust
pub struct ToolParameter {
    pub name: String,
    pub param_type: ToolParameterType,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
}

pub enum ToolParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
    File,
    Directory,
}
```

### ToolDefinition

Complete definition of a tool:

```rust
pub struct ToolDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub category: ToolCategory,
    pub parameters: Vec<ToolParameter>,
    pub execution_timeout_seconds: u64,
    pub requires_authentication: bool,
    pub safety_level: SafetyLevel,
    pub capabilities: Vec<String>,
}
```

### ToolRequest

Request to execute a tool:

```rust
pub struct ToolRequest {
    pub tool_id: String,
    pub parameters: HashMap<String, String>,
    pub agent_id: String,
    pub execution_context: ExecutionContext,
    pub safety_checks_disabled: bool,
}
```

## Usage Examples

### Registering a Tool

```rust
use anya::ml::tools::{ToolDefinition, ToolParameter, ToolParameterType, ToolRegistry, SafetyLevel, ToolCategory};

async fn register_file_search_tool() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ToolRegistry::global();

    // Define parameters
    let parameters = vec![
        ToolParameter {
            name: "pattern".to_string(),
            param_type: ToolParameterType::String,
            description: "File search pattern (e.g., *.txt)".to_string(),
            required: true,
            default_value: None,
            validation_pattern: Some(r"^[^/\\:*?\"<>|]+$".to_string()),
        },
        ToolParameter {
            name: "directory".to_string(),
            param_type: ToolParameterType::Directory,
            description: "Directory to search in".to_string(),
            required: false,
            default_value: Some(".".to_string()),
            validation_pattern: None,
        },
    ];

    // Create tool definition
    let tool = ToolDefinition {
        id: "file_search".to_string(),
        name: "File Search".to_string(),
        description: "Search for files matching a pattern".to_string(),
        version: "1.0.0".to_string(),
        category: ToolCategory::FileSystem,
        parameters,
        execution_timeout_seconds: 30,
        requires_authentication: false,
        safety_level: SafetyLevel::Safe,
        capabilities: vec!["file_system:read".to_string()],
    };

    // Register the tool
    registry.register_tool(tool).await?;
    println!("File search tool registered successfully");

    Ok(())
}
```

### Executing a Tool

```rust
use anya::ml::tools::{ToolRequest, ToolExecutor, ExecutionContext};
use std::collections::HashMap;

async fn execute_file_search() -> Result<(), Box<dyn std::error::Error>> {
    let executor = ToolExecutor::new();

    // Create request parameters
    let mut parameters = HashMap::new();
    parameters.insert("pattern".to_string(), "*.rs".to_string());
    parameters.insert("directory".to_string(), "./src".to_string());

    // Create execution context
    let mut env_vars = HashMap::new();
    env_vars.insert("LC_ALL".to_string(), "en_US.UTF-8".to_string());

    let context = ExecutionContext {
        working_directory: Some("/workspaces/project".to_string()),
        environment_variables: env_vars,
        timeout_override: None,
        resource_limits: None,
    };

    // Create the request
    let request = ToolRequest {
        tool_id: "file_search".to_string(),
        parameters,
        agent_id: "test_agent".to_string(),
        execution_context: context,
        safety_checks_disabled: false,
    };

    // Execute the tool
    let result = executor.execute_tool(request).await?;

    // Handle the result
    if result.success {
        println!("Search results: {}", result.output);
    } else {
        println!("Search failed: {}", result.error.unwrap_or_default());
    }

    println!("Execution took {} ms", result.execution_time_ms);

    Ok(())
}
```

### Implementing a Custom Tool

```rust
use anya::ml::tools::{Tool, ToolRequest, ToolResult, ToolRegistry};
use async_trait::async_trait;

struct CustomAnalysisTool;

#[async_trait]
impl Tool for CustomAnalysisTool {
    async fn execute(&self, request: ToolRequest) -> Result<ToolResult, Box<dyn std::error::Error + Send + Sync>> {
        // Extract parameters
        let input_file = request.parameters.get("input_file")
            .ok_or("Missing input_file parameter")?;

        let analysis_type = request.parameters.get("analysis_type")
            .unwrap_or(&"basic".to_string());

        // Perform analysis (actual implementation would depend on analysis type)
        println!("Performing {} analysis on {}", analysis_type, input_file);

        // Return result
        Ok(ToolResult {
            tool_id: "custom_analysis".to_string(),
            execution_id: Uuid::new_v4().to_string(),
            success: true,
            output: format!("Analysis complete on {}: Results look good!", input_file),
            error: None,
            execution_time_ms: 1250,
            metadata: HashMap::new(),
        })
    }
}

async fn register_custom_tool() -> Result<(), Box<dyn std::error::Error>> {
    // Register tool definition
    // ...

    // Register tool implementation
    let registry = ToolRegistry::global();
    registry.register_tool_implementation("custom_analysis".to_string(), Box::new(CustomAnalysisTool)).await?;

    Ok(())
}
```

## Security Considerations

1. **Safety Levels**: Tools are categorized by safety level to prevent unintended consequences
2. **Parameter Validation**: All parameters are validated against defined patterns before execution
3. **Capability Checks**: Tools are restricted to their declared capabilities
4. **Execution Isolation**: Tool execution occurs in isolated environments where appropriate
5. **Timeouts**: All executions have enforced timeouts to prevent resource exhaustion

## For More Information

- [Tool Development Guide](../../../docs/ml/tool_development.md)
- [Security Best Practices](../../../docs/security/tool_security.md)
- Project documentation
