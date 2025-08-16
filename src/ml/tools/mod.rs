//! Tool Integration Framework
//!
//! Provides a comprehensive framework for integrating external tools
//! with AI agents, enabling complex workflows and enhanced capabilities.

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_id: String,
    pub execution_id: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub metadata: HashMap<String, String>,
}

/// Tool parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: ToolParameterType,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCategory {
    FileSystem,
    Network,
    Database,
    Development,
    Analysis,
    Communication,
    SystemAdministration,
    DataProcessing,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SafetyLevel {
    Safe,       // Read-only operations
    Moderate,   // Limited write operations
    Dangerous,  // System-level operations
    Restricted, // Requires explicit approval
}

/// Tool execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub tool_id: String,
    pub parameters: HashMap<String, String>,
    pub agent_id: String,
    pub execution_context: ExecutionContext,
    pub safety_checks_disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub working_directory: Option<String>,
    pub environment_variables: HashMap<String, String>,
    pub timeout_override: Option<u64>,
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: Option<u64>,
    pub max_cpu_percent: Option<u8>,
    pub max_execution_time_seconds: Option<u64>,
    pub max_output_size_kb: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: Some(1024),
            max_cpu_percent: Some(80),
            max_execution_time_seconds: Some(300),
            max_output_size_kb: Some(10240),
        }
    }
}

/// Tool execution trait
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get tool definition
    fn definition(&self) -> &ToolDefinition;

    /// Execute the tool with given parameters
    async fn execute(&self, request: ToolRequest) -> Result<ToolResult>;

    /// Validate parameters before execution
    fn validate_parameters(&self, parameters: &HashMap<String, String>) -> Result<()>;

    /// Check if tool is available in current environment
    async fn health_check(&self) -> Result<bool>;

    /// Get tool-specific help information
    fn get_help(&self) -> String;
}

/// Tool registry for managing available tools
pub struct ToolRegistry {
    tools: RwLock<HashMap<String, Arc<dyn Tool>>>,
    execution_history: RwLock<Vec<ToolResult>>,
    safety_policies: RwLock<HashMap<SafetyLevel, Vec<String>>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    /// Create new tool registry
    pub fn new() -> Self {
        let mut safety_policies = HashMap::new();

        // Default safety policies
        safety_policies.insert(
            SafetyLevel::Safe,
            vec!["read_file".to_string(), "list_directory".to_string()],
        );
        safety_policies.insert(
            SafetyLevel::Moderate,
            vec!["write_file".to_string(), "create_directory".to_string()],
        );
        safety_policies.insert(
            SafetyLevel::Dangerous,
            vec!["execute_command".to_string(), "delete_file".to_string()],
        );
        safety_policies.insert(
            SafetyLevel::Restricted,
            vec!["system_shutdown".to_string(), "network_config".to_string()],
        );

        Self {
            tools: RwLock::new(HashMap::new()),
            execution_history: RwLock::new(Vec::new()),
            safety_policies: RwLock::new(safety_policies),
        }
    }

    /// Register a new tool
    pub async fn register_tool(&self, tool: Arc<dyn Tool>) -> Result<()> {
        let tool_id = tool.definition().id.clone();
        let mut tools = self.tools.write().await;
        tools.insert(tool_id, tool);
        Ok(())
    }

    /// Get tool by ID
    pub async fn get_tool(&self, tool_id: &str) -> Option<Arc<dyn Tool>> {
        let tools = self.tools.read().await;
        tools.get(tool_id).cloned()
    }

    /// List all available tools
    pub async fn list_tools(&self) -> Vec<ToolDefinition> {
        let tools = self.tools.read().await;
        tools.values().map(|t| t.definition().clone()).collect()
    }

    /// Execute a tool with safety checks
    pub async fn execute_tool(&self, request: ToolRequest) -> Result<ToolResult> {
        let tool = self
            .get_tool(&request.tool_id)
            .await
            .ok_or_else(|| anyhow!("Tool not found: {}", request.tool_id))?;

        // Safety checks
        if !request.safety_checks_disabled {
            self.perform_safety_checks(&tool, &request).await?;
        }

        // Validate parameters
        tool.validate_parameters(&request.parameters)?;

        // Execute tool
        let start_time = std::time::Instant::now();
        let result = tool.execute(request).await;
        let execution_time = start_time.elapsed().as_millis() as u64;

        // Update execution time in result
        let mut final_result = result?;
        final_result.execution_time_ms = execution_time;

        // Store in history
        {
            let mut history = self.execution_history.write().await;
            history.push(final_result.clone());

            // Keep only last 1000 executions
            if history.len() > 1000 {
                history.drain(0..100);
            }
        }

        Ok(final_result)
    }

    /// Perform safety checks before tool execution
    async fn perform_safety_checks(
        &self,
        tool: &Arc<dyn Tool>,
        request: &ToolRequest,
    ) -> Result<()> {
        let definition = tool.definition();

        match definition.safety_level {
            SafetyLevel::Restricted => {
                return Err(anyhow!("Tool {} requires explicit approval", definition.id));
            }
            SafetyLevel::Dangerous => {
                log::warn!("Executing dangerous tool: {}", definition.id);
            }
            _ => {}
        }

        // Check resource limits
        if let Some(max_time) = request
            .execution_context
            .resource_limits
            .max_execution_time_seconds
        {
            if max_time > 3600 {
                return Err(anyhow!(
                    "Execution time limit too high: {} seconds",
                    max_time
                ));
            }
        }

        Ok(())
    }

    /// Get execution history
    pub async fn get_execution_history(&self) -> Vec<ToolResult> {
        let history = self.execution_history.read().await;
        history.clone()
    }

    /// Get safety policies for a specific safety level
    pub async fn get_safety_policies(&self, level: SafetyLevel) -> Vec<String> {
        let policies = self.safety_policies.read().await;
        policies.get(&level).cloned().unwrap_or_default()
    }

    /// Search tools by category or capability
    pub async fn search_tools(&self, query: &str) -> Vec<ToolDefinition> {
        let tools = self.tools.read().await;
        tools
            .values()
            .map(|t| t.definition())
            .filter(|def| {
                def.name.to_lowercase().contains(&query.to_lowercase())
                    || def
                        .description
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                    || def
                        .capabilities
                        .iter()
                        .any(|cap| cap.to_lowercase().contains(&query.to_lowercase()))
            })
            .cloned()
            .collect()
    }
}

/// Built-in file system tool
pub struct FileSystemTool {
    definition: ToolDefinition,
}

impl Default for FileSystemTool {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemTool {
    pub fn new() -> Self {
        let definition = ToolDefinition {
            id: "filesystem".to_string(),
            name: "File System Operations".to_string(),
            description: "Read, write, and manage files and directories".to_string(),
            version: "1.0.0".to_string(),
            category: ToolCategory::FileSystem,
            parameters: vec![
                ToolParameter {
                    name: "operation".to_string(),
                    param_type: ToolParameterType::String,
                    description: "Operation type: read, write, list, create_dir, delete"
                        .to_string(),
                    required: true,
                    default_value: None,
                    validation_pattern: Some("^(read|write|list|create_dir|delete)$".to_string()),
                },
                ToolParameter {
                    name: "path".to_string(),
                    param_type: ToolParameterType::String,
                    description: "File or directory path".to_string(),
                    required: true,
                    default_value: None,
                    validation_pattern: None,
                },
                ToolParameter {
                    name: "content".to_string(),
                    param_type: ToolParameterType::String,
                    description: "Content to write (for write operations)".to_string(),
                    required: false,
                    default_value: None,
                    validation_pattern: None,
                },
            ],
            execution_timeout_seconds: 60,
            requires_authentication: false,
            safety_level: SafetyLevel::Moderate,
            capabilities: vec![
                "file_operations".to_string(),
                "directory_management".to_string(),
            ],
        };

        Self { definition }
    }
}

#[async_trait]
impl Tool for FileSystemTool {
    fn definition(&self) -> &ToolDefinition {
        &self.definition
    }

    async fn execute(&self, request: ToolRequest) -> Result<ToolResult> {
        let execution_id = Uuid::new_v4().to_string();
        let operation = request
            .parameters
            .get("operation")
            .ok_or_else(|| anyhow!("Missing operation parameter"))?;
        let path = request
            .parameters
            .get("path")
            .ok_or_else(|| anyhow!("Missing path parameter"))?;

        let result = match operation.as_str() {
            "read" => match tokio::fs::read_to_string(path).await {
                Ok(content) => ToolResult {
                    tool_id: self.definition.id.clone(),
                    execution_id,
                    success: true,
                    output: content,
                    error: None,
                    execution_time_ms: 0,
                    metadata: HashMap::new(),
                },
                Err(e) => ToolResult {
                    tool_id: self.definition.id.clone(),
                    execution_id,
                    success: false,
                    output: String::new(),
                    error: Some(e.to_string()),
                    execution_time_ms: 0,
                    metadata: HashMap::new(),
                },
            },
            "write" => {
                let default_content = String::new();
                let content = request
                    .parameters
                    .get("content")
                    .unwrap_or(&default_content);
                match tokio::fs::write(path, content).await {
                    Ok(_) => ToolResult {
                        tool_id: self.definition.id.clone(),
                        execution_id,
                        success: true,
                        output: format!("Successfully wrote to {path}"),
                        error: None,
                        execution_time_ms: 0,
                        metadata: HashMap::new(),
                    },
                    Err(e) => ToolResult {
                        tool_id: self.definition.id.clone(),
                        execution_id,
                        success: false,
                        output: String::new(),
                        error: Some(e.to_string()),
                        execution_time_ms: 0,
                        metadata: HashMap::new(),
                    },
                }
            }
            "list" => match tokio::fs::read_dir(path).await {
                Ok(mut entries) => {
                    let mut files = Vec::new();
                    while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                        files.push(entry.file_name().to_string_lossy().to_string());
                    }
                    ToolResult {
                        tool_id: self.definition.id.clone(),
                        execution_id,
                        success: true,
                        output: files.join("\n"),
                        error: None,
                        execution_time_ms: 0,
                        metadata: HashMap::new(),
                    }
                }
                Err(e) => ToolResult {
                    tool_id: self.definition.id.clone(),
                    execution_id,
                    success: false,
                    output: String::new(),
                    error: Some(e.to_string()),
                    execution_time_ms: 0,
                    metadata: HashMap::new(),
                },
            },
            "create_dir" => match tokio::fs::create_dir_all(path).await {
                Ok(_) => ToolResult {
                    tool_id: self.definition.id.clone(),
                    execution_id,
                    success: true,
                    output: format!("Successfully created directory {path}"),
                    error: None,
                    execution_time_ms: 0,
                    metadata: HashMap::new(),
                },
                Err(e) => ToolResult {
                    tool_id: self.definition.id.clone(),
                    execution_id,
                    success: false,
                    output: String::new(),
                    error: Some(e.to_string()),
                    execution_time_ms: 0,
                    metadata: HashMap::new(),
                },
            },
            _ => ToolResult {
                tool_id: self.definition.id.clone(),
                execution_id,
                success: false,
                output: String::new(),
                error: Some(format!("Unknown operation: {operation}")),
                execution_time_ms: 0,
                metadata: HashMap::new(),
            },
        };

        Ok(result)
    }

    fn validate_parameters(&self, parameters: &HashMap<String, String>) -> Result<()> {
        if !parameters.contains_key("operation") {
            return Err(anyhow!("Missing required parameter: operation"));
        }
        if !parameters.contains_key("path") {
            return Err(anyhow!("Missing required parameter: path"));
        }

        let operation = parameters.get("operation").unwrap();
        if !["read", "write", "list", "create_dir", "delete"].contains(&operation.as_str()) {
            return Err(anyhow!("Invalid operation: {}", operation));
        }

        Ok(())
    }

    async fn health_check(&self) -> Result<bool> {
        // Test basic file system access
        let temp_dir = std::env::temp_dir();
        Ok(temp_dir.exists())
    }

    fn get_help(&self) -> String {
        r#"File System Tool Help:

Operations:
- read: Read file content (requires 'path')
- write: Write content to file (requires 'path' and 'content')
- list: List directory contents (requires 'path')
- create_dir: Create directory (requires 'path')
- delete: Delete file or directory (requires 'path')

Examples:
- Read file: {"operation": "read", "path": "/path/to/file.txt"}
- Write file: {"operation": "write", "path": "/path/to/file.txt", "content": "Hello World"}
- List directory: {"operation": "list", "path": "/path/to/directory"}
"#
        .to_string()
    }
}

/// Command execution tool
pub struct CommandTool {
    definition: ToolDefinition,
}

impl Default for CommandTool {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandTool {
    pub fn new() -> Self {
        let definition = ToolDefinition {
            id: "command".to_string(),
            name: "Command Execution".to_string(),
            description: "Execute system commands and scripts".to_string(),
            version: "1.0.0".to_string(),
            category: ToolCategory::SystemAdministration,
            parameters: vec![
                ToolParameter {
                    name: "command".to_string(),
                    param_type: ToolParameterType::String,
                    description: "Command to execute".to_string(),
                    required: true,
                    default_value: None,
                    validation_pattern: None,
                },
                ToolParameter {
                    name: "args".to_string(),
                    param_type: ToolParameterType::Array,
                    description: "Command arguments".to_string(),
                    required: false,
                    default_value: None,
                    validation_pattern: None,
                },
                ToolParameter {
                    name: "working_dir".to_string(),
                    param_type: ToolParameterType::Directory,
                    description: "Working directory".to_string(),
                    required: false,
                    default_value: None,
                    validation_pattern: None,
                },
            ],
            execution_timeout_seconds: 300,
            requires_authentication: false,
            safety_level: SafetyLevel::Dangerous,
            capabilities: vec![
                "command_execution".to_string(),
                "script_running".to_string(),
            ],
        };

        Self { definition }
    }
}

#[async_trait]
impl Tool for CommandTool {
    fn definition(&self) -> &ToolDefinition {
        &self.definition
    }

    async fn execute(&self, request: ToolRequest) -> Result<ToolResult> {
        let execution_id = Uuid::new_v4().to_string();
        let command = request
            .parameters
            .get("command")
            .ok_or_else(|| anyhow!("Missing command parameter"))?;

        let mut cmd = Command::new(command);

        // Add arguments if provided
        if let Some(args) = request.parameters.get("args") {
            let args_vec: Vec<&str> = args.split_whitespace().collect();
            cmd.args(args_vec);
        }

        // Set working directory if provided
        if let Some(working_dir) = request.parameters.get("working_dir") {
            cmd.current_dir(working_dir);
        } else if let Some(working_dir) = &request.execution_context.working_directory {
            cmd.current_dir(working_dir);
        }

        // Set environment variables
        for (key, value) in &request.execution_context.environment_variables {
            cmd.env(key, value);
        }

        // Configure stdio
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        // Execute command
        match cmd.output().await {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                let success = output.status.success();
                let combined_output = if stderr.is_empty() {
                    stdout
                } else {
                    format!("STDOUT:\n{stdout}\n\nSTDERR:\n{stderr}")
                };

                Ok(ToolResult {
                    tool_id: self.definition.id.clone(),
                    execution_id,
                    success,
                    output: combined_output,
                    error: if success { None } else { Some(stderr) },
                    execution_time_ms: 0,
                    metadata: HashMap::new(),
                })
            }
            Err(e) => Ok(ToolResult {
                tool_id: self.definition.id.clone(),
                execution_id,
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: 0,
                metadata: HashMap::new(),
            }),
        }
    }

    fn validate_parameters(&self, parameters: &HashMap<String, String>) -> Result<()> {
        if !parameters.contains_key("command") {
            return Err(anyhow!("Missing required parameter: command"));
        }
        Ok(())
    }

    async fn health_check(&self) -> Result<bool> {
        // Test basic command execution
        let output = Command::new("echo").arg("test").output().await;

        Ok(output.is_ok())
    }

    fn get_help(&self) -> String {
        r#"Command Execution Tool Help:

Execute system commands with arguments and environment control.

Parameters:
- command: The command to execute (required)
- args: Space-separated command arguments (optional)
- working_dir: Working directory for command execution (optional)

Examples:
- List files: {"command": "ls", "args": "-la"}
- Check disk usage: {"command": "df", "args": "-h"}
- Run in specific directory: {"command": "pwd", "working_dir": "/tmp"}

Warning: This tool can execute any system command and should be used with caution.
"#
        .to_string()
    }
}

/// Tool manager for coordinating multiple tools
pub struct ToolManager {
    registry: Arc<ToolRegistry>,
    active_executions: Arc<Mutex<HashMap<String, tokio::task::JoinHandle<()>>>>,
}

impl ToolManager {
    /// Create new tool manager
    pub async fn new() -> Self {
        let registry = Arc::new(ToolRegistry::new());

        // Register built-in tools
        registry
            .register_tool(Arc::new(FileSystemTool::new()))
            .await
            .unwrap();
        registry
            .register_tool(Arc::new(CommandTool::new()))
            .await
            .unwrap();

        Self {
            registry,
            active_executions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get the tool registry
    pub fn registry(&self) -> Arc<ToolRegistry> {
        self.registry.clone()
    }

    /// Execute multiple tools in parallel
    pub async fn execute_parallel(&self, requests: Vec<ToolRequest>) -> Vec<Result<ToolResult>> {
        let futures: Vec<_> = requests
            .into_iter()
            .map(|req| self.registry.execute_tool(req))
            .collect();

        // Wait for all executions to complete
        futures::future::join_all(futures).await
    }

    /// Execute tools in sequence (useful for dependent operations)
    pub async fn execute_sequence(&self, requests: Vec<ToolRequest>) -> Result<Vec<ToolResult>> {
        let mut results = Vec::new();

        for request in requests {
            let result = self.registry.execute_tool(request).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Cancel a running tool execution
    pub async fn cancel_execution(&self, execution_id: &str) -> Result<bool> {
        let mut executions = self.active_executions.lock().await;
        if let Some(handle) = executions.remove(execution_id) {
            handle.abort();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get tool usage statistics
    pub async fn get_statistics(&self) -> ToolStatistics {
        let history = self.registry.get_execution_history().await;

        let total_executions = history.len();
        let successful_executions = history.iter().filter(|r| r.success).count();
        let failed_executions = total_executions - successful_executions;

        let avg_execution_time = if total_executions > 0 {
            history.iter().map(|r| r.execution_time_ms).sum::<u64>() / total_executions as u64
        } else {
            0
        };

        let mut tool_usage: HashMap<String, u32> = HashMap::new();
        for result in &history {
            *tool_usage.entry(result.tool_id.clone()).or_insert(0) += 1;
        }

        ToolStatistics {
            total_executions,
            successful_executions,
            failed_executions,
            average_execution_time_ms: avg_execution_time,
            tool_usage_count: tool_usage,
        }
    }
}

/// Tool usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatistics {
    pub total_executions: usize,
    pub successful_executions: usize,
    pub failed_executions: usize,
    pub average_execution_time_ms: u64,
    pub tool_usage_count: HashMap<String, u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_registry() {
        let registry = ToolRegistry::new();
        let fs_tool = Arc::new(FileSystemTool::new());

        registry.register_tool(fs_tool.clone()).await.unwrap();

        let tools = registry.list_tools().await;
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].id, "filesystem");
    }

    #[tokio::test]
    async fn test_filesystem_tool() {
        let tool = FileSystemTool::new();

        // Test validation
        let mut params = HashMap::new();
        params.insert("operation".to_string(), "read".to_string());
        params.insert("path".to_string(), "/tmp/test.txt".to_string());

        assert!(tool.validate_parameters(&params).is_ok());

        // Test health check
        assert!(tool.health_check().await.unwrap());
    }

    #[tokio::test]
    async fn test_tool_manager() {
        let manager = ToolManager::new().await;
        let tools = manager.registry().list_tools().await;

        // Should have at least the built-in tools
        assert!(tools.len() >= 2);

        let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();
        assert!(tool_names.contains(&"File System Operations".to_string()));
        assert!(tool_names.contains(&"Command Execution".to_string()));
    }
}
