//! Advanced Agent Orchestration System
//!
//! Provides sophisticated multi-agent workflow coordination inspired by LangGraph,
//! with support for complex execution graphs, conditional flows, and cross-protocol operations.

use crate::ml::agents::Agent;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

/// Command sent to an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCommand {
    pub action: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout_ms: Option<u64>,
}

/// Response from an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub success: bool,
    pub result: serde_json::Value,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// Workflow engine for orchestrating complex agent interactions
pub struct WorkflowEngine {
    workflows: Arc<RwLock<HashMap<String, WorkflowDefinition>>>,
    active_executions: Arc<RwLock<HashMap<String, WorkflowExecution>>>,
    agents: Arc<RwLock<HashMap<String, Box<dyn Agent>>>>,
    event_bus: mpsc::UnboundedSender<WorkflowEvent>,
}

/// Definition of a workflow with nodes and edges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub nodes: HashMap<String, WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
    pub entry_point: String,
    pub exit_points: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// A node in the workflow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: String,
    pub node_type: NodeType,
    pub agent_id: Option<String>,
    pub command: Option<AgentCommand>,
    pub condition: Option<String>, // Condition expression for conditional nodes
    pub retry_policy: Option<RetryPolicy>,
    pub timeout_ms: Option<u64>,
    pub metadata: HashMap<String, String>,
}

/// Types of workflow nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    /// Execute an agent action
    Action,
    /// Make a decision based on conditions
    Decision,
    /// Parallel execution of multiple branches
    Parallel,
    /// Wait for external event or condition
    Wait,
    /// Merge multiple execution paths
    Merge,
    /// Start of workflow
    Start,
    /// End of workflow
    End,
    /// Sub-workflow invocation
    SubWorkflow,
    /// Human-in-the-loop interaction
    Human,
}

/// Edge connecting two nodes with conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub from: String,
    pub to: String,
    pub condition: Option<String>, // Optional condition for edge traversal
    pub weight: Option<f32>,       // For weighted routing
    pub metadata: HashMap<String, String>,
}

/// Retry policy for failed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f32,
    pub retry_on_errors: Vec<String>, // Error types to retry on
}

/// Active execution of a workflow
#[derive(Debug)]
pub struct WorkflowExecution {
    pub id: String,
    pub workflow_id: String,
    pub status: ExecutionStatus,
    pub current_nodes: Vec<String>,
    pub execution_history: Vec<ExecutionStep>,
    pub context: WorkflowContext,
    pub started_at: std::time::SystemTime,
    pub completed_at: Option<std::time::SystemTime>,
    pub error: Option<String>,
}

/// Status of workflow execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Suspended,
}

/// Step in workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub node_id: String,
    pub step_type: StepType,
    pub started_at: std::time::SystemTime,
    pub completed_at: Option<std::time::SystemTime>,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub retry_count: u32,
}

/// Type of execution step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    NodeExecution,
    ConditionEvaluation,
    AgentCall,
    SubWorkflow,
    HumanInteraction,
    ExternalCall,
}

/// Context shared across workflow execution
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct WorkflowContext {
    pub variables: HashMap<String, serde_json::Value>,
    pub agent_states: HashMap<String, serde_json::Value>,
    pub external_data: HashMap<String, serde_json::Value>,
    pub execution_metadata: HashMap<String, String>,
}

/// Events emitted during workflow execution
#[derive(Debug, Clone)]
pub enum WorkflowEvent {
    WorkflowStarted {
        execution_id: String,
        workflow_id: String,
    },
    NodeStarted {
        execution_id: String,
        node_id: String,
    },
    NodeCompleted {
        execution_id: String,
        node_id: String,
        result: serde_json::Value,
    },
    NodeFailed {
        execution_id: String,
        node_id: String,
        error: String,
    },
    WorkflowCompleted {
        execution_id: String,
        result: serde_json::Value,
    },
    WorkflowFailed {
        execution_id: String,
        error: String,
    },
    ConditionalBranch {
        execution_id: String,
        node_id: String,
        condition: String,
        result: bool,
    },
    AgentResponse {
        execution_id: String,
        agent_id: String,
        response: AgentResponse,
    },
}

impl WorkflowEngine {
    /// Create new workflow engine
    pub fn new() -> (Self, mpsc::UnboundedReceiver<WorkflowEvent>) {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        let engine = Self {
            workflows: Arc::new(RwLock::new(HashMap::new())),
            active_executions: Arc::new(RwLock::new(HashMap::new())),
            agents: Arc::new(RwLock::new(HashMap::new())),
            event_bus: event_sender,
        };

        (engine, event_receiver)
    }

    /// Register a new workflow
    pub async fn register_workflow(&self, workflow: WorkflowDefinition) -> Result<()> {
        self.validate_workflow(&workflow)?;

        let mut workflows = self.workflows.write().await;
        workflows.insert(workflow.id.clone(), workflow);

        Ok(())
    }

    /// Register an agent for use in workflows
    pub async fn register_agent(&self, agent_id: String, agent: Box<dyn Agent>) -> Result<()> {
        let mut agents = self.agents.write().await;
        agents.insert(agent_id, agent);
        Ok(())
    }

    /// Start workflow execution
    pub async fn start_workflow(
        &self,
        workflow_id: &str,
        initial_context: WorkflowContext,
    ) -> Result<String> {
        let workflows = self.workflows.read().await;
        let workflow = workflows
            .get(workflow_id)
            .ok_or_else(|| anyhow!("Workflow not found: {}", workflow_id))?;

        let execution_id = Uuid::new_v4().to_string();
        let execution = WorkflowExecution {
            id: execution_id.clone(),
            workflow_id: workflow_id.to_string(),
            status: ExecutionStatus::Running,
            current_nodes: vec![workflow.entry_point.clone()],
            execution_history: Vec::new(),
            context: initial_context,
            started_at: std::time::SystemTime::now(),
            completed_at: None,
            error: None,
        };

        {
            let mut executions = self.active_executions.write().await;
            executions.insert(execution_id.clone(), execution);
        }

        // Emit start event
        let _ = self.event_bus.send(WorkflowEvent::WorkflowStarted {
            execution_id: execution_id.clone(),
            workflow_id: workflow_id.to_string(),
        });

        // Start execution in background
        let engine = self.clone();
        let exec_id = execution_id.clone();
        tokio::spawn(async move {
            if let Err(e) = engine.execute_workflow(&exec_id).await {
                let _ = engine.event_bus.send(WorkflowEvent::WorkflowFailed {
                    execution_id: exec_id.clone(),
                    error: e.to_string(),
                });

                // Update execution status
                let mut executions = engine.active_executions.write().await;
                if let Some(execution) = executions.get_mut(&exec_id) {
                    execution.status = ExecutionStatus::Failed;
                    execution.error = Some(e.to_string());
                    execution.completed_at = Some(std::time::SystemTime::now());
                }
            }
        });

        Ok(execution_id)
    }

    /// Execute workflow logic
    async fn execute_workflow(&self, execution_id: &str) -> Result<()> {
        loop {
            let current_nodes = {
                let executions = self.active_executions.read().await;
                let execution = executions
                    .get(execution_id)
                    .ok_or_else(|| anyhow!("Execution not found: {}", execution_id))?;

                if execution.status != ExecutionStatus::Running {
                    break;
                }

                execution.current_nodes.clone()
            };

            if current_nodes.is_empty() {
                // Workflow completed
                self.complete_workflow(execution_id).await?;
                break;
            }

            // Execute current nodes
            let next_nodes = self.execute_nodes(execution_id, &current_nodes).await?;

            // Update current nodes
            {
                let mut executions = self.active_executions.write().await;
                if let Some(execution) = executions.get_mut(execution_id) {
                    execution.current_nodes = next_nodes;
                }
            }
        }

        Ok(())
    }

    /// Execute a set of nodes
    async fn execute_nodes(&self, execution_id: &str, node_ids: &[String]) -> Result<Vec<String>> {
        let mut next_nodes = Vec::new();

        for node_id in node_ids {
            let result = self.execute_node(execution_id, node_id).await?;

            // Determine next nodes based on result
            let following_nodes = self.get_next_nodes(execution_id, node_id, &result).await?;
            next_nodes.extend(following_nodes);
        }

        // Remove duplicates
        next_nodes.sort();
        next_nodes.dedup();

        Ok(next_nodes)
    }

    /// Execute a single node
    async fn execute_node(&self, execution_id: &str, node_id: &str) -> Result<serde_json::Value> {
        // Emit node start event
        let _ = self.event_bus.send(WorkflowEvent::NodeStarted {
            execution_id: execution_id.to_string(),
            node_id: node_id.to_string(),
        });

        let (_workflow_id, node) = {
            let executions = self.active_executions.read().await;
            let execution = executions
                .get(execution_id)
                .ok_or_else(|| anyhow!("Execution not found: {}", execution_id))?;

            let workflows = self.workflows.read().await;
            let workflow = workflows
                .get(&execution.workflow_id)
                .ok_or_else(|| anyhow!("Workflow not found: {}", execution.workflow_id))?;

            let node = workflow
                .nodes
                .get(node_id)
                .ok_or_else(|| anyhow!("Node not found: {}", node_id))?;

            (execution.workflow_id.clone(), node.clone())
        };

        let result = match node.node_type {
            NodeType::Action => self.execute_action_node(execution_id, &node).await?,
            NodeType::Decision => self.execute_decision_node(execution_id, &node).await?,
            NodeType::Parallel => self.execute_parallel_node(execution_id, &node).await?,
            NodeType::Wait => self.execute_wait_node(execution_id, &node).await?,
            NodeType::Merge => self.execute_merge_node(execution_id, &node).await?,
            NodeType::Start => serde_json::json!({"status": "started"}),
            NodeType::End => serde_json::json!({"status": "completed"}),
            NodeType::SubWorkflow => self.execute_subworkflow_node(execution_id, &node).await?,
            NodeType::Human => self.execute_human_node(execution_id, &node).await?,
        };

        // Record execution step
        self.record_execution_step(
            execution_id,
            node_id,
            StepType::NodeExecution,
            Some(result.clone()),
            None,
        )
        .await?;

        // Emit node completion event
        let _ = self.event_bus.send(WorkflowEvent::NodeCompleted {
            execution_id: execution_id.to_string(),
            node_id: node_id.to_string(),
            result: result.clone(),
        });

        Ok(result)
    }

    /// Execute an action node (agent call)
    async fn execute_action_node(
        &self,
        execution_id: &str,
        node: &WorkflowNode,
    ) -> Result<serde_json::Value> {
        let agent_id = node
            .agent_id
            .as_ref()
            .ok_or_else(|| anyhow!("Agent ID required for action node: {}", node.id))?;

        let command = node
            .command
            .as_ref()
            .ok_or_else(|| anyhow!("Command required for action node: {}", node.id))?;

        let agents = self.agents.read().await;
        let agent = agents
            .get(agent_id)
            .ok_or_else(|| anyhow!("Agent not found: {}", agent_id))?;

        // Execute with retry policy
        let retry_policy = node.retry_policy.as_ref();
        let mut attempts = 0;
        let max_attempts = retry_policy.map(|p| p.max_attempts).unwrap_or(1);
        let start_time = std::time::Instant::now();

        loop {
            attempts += 1;

            // Convert AgentCommand to Observation
            let observation = crate::ml::agents::Observation::Json(serde_json::to_value(command)?);

            match agent.process(observation).await {
                Ok(Some(action)) => {
                    let response = AgentResponse {
                        success: true,
                        result: serde_json::to_value(&action)?,
                        error: None,
                        execution_time_ms: start_time.elapsed().as_millis() as u64,
                    };

                    let _ = self.event_bus.send(WorkflowEvent::AgentResponse {
                        execution_id: execution_id.to_string(),
                        agent_id: agent_id.clone(),
                        response: response.clone(),
                    });

                    return Ok(serde_json::to_value(response)?);
                }
                Ok(None) => {
                    let response = AgentResponse {
                        success: true,
                        result: serde_json::Value::Null,
                        error: None,
                        execution_time_ms: start_time.elapsed().as_millis() as u64,
                    };
                    return Ok(serde_json::to_value(response)?);
                }
                Err(e) => {
                    if attempts >= max_attempts {
                        return Err(anyhow!(
                            "Agent execution failed after {} attempts: {}",
                            attempts,
                            e
                        ));
                    }

                    if let Some(policy) = retry_policy {
                        let delay = std::cmp::min(
                            policy.initial_delay_ms
                                * (policy.backoff_multiplier.powi(attempts as i32 - 1) as u64),
                            policy.max_delay_ms,
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                    }
                }
            }
        }
    }

    /// Execute a decision node
    async fn execute_decision_node(
        &self,
        execution_id: &str,
        node: &WorkflowNode,
    ) -> Result<serde_json::Value> {
        let condition = node
            .condition
            .as_ref()
            .ok_or_else(|| anyhow!("Condition required for decision node: {}", node.id))?;

        // Evaluate condition against current context
        let context = {
            let executions = self.active_executions.read().await;
            let execution = executions
                .get(execution_id)
                .ok_or_else(|| anyhow!("Execution not found: {}", execution_id))?;
            execution.context.clone()
        };

        let result = self.evaluate_condition(condition, &context).await?;

        let _ = self.event_bus.send(WorkflowEvent::ConditionalBranch {
            execution_id: execution_id.to_string(),
            node_id: node.id.clone(),
            condition: condition.clone(),
            result,
        });

        Ok(serde_json::json!({"condition_result": result}))
    }

    /// Execute parallel node (placeholder)
    async fn execute_parallel_node(
        &self,
        _execution_id: &str,
        node: &WorkflowNode,
    ) -> Result<serde_json::Value> {
        // In a full implementation, this would spawn parallel execution branches
        Ok(serde_json::json!({"parallel_branches": "started", "node_id": node.id}))
    }

    /// Execute wait node (placeholder)
    async fn execute_wait_node(
        &self,
        _execution_id: &str,
        node: &WorkflowNode,
    ) -> Result<serde_json::Value> {
        // In a full implementation, this would wait for external events
        let wait_time = node.timeout_ms.unwrap_or(1000);
        tokio::time::sleep(tokio::time::Duration::from_millis(wait_time)).await;
        Ok(serde_json::json!({"waited_ms": wait_time}))
    }

    /// Execute merge node (placeholder)
    async fn execute_merge_node(
        &self,
        _execution_id: &str,
        node: &WorkflowNode,
    ) -> Result<serde_json::Value> {
        // In a full implementation, this would merge results from parallel branches
        Ok(serde_json::json!({"merged": true, "node_id": node.id}))
    }

    /// Execute subworkflow node (placeholder)
    async fn execute_subworkflow_node(
        &self,
        _execution_id: &str,
        node: &WorkflowNode,
    ) -> Result<serde_json::Value> {
        // In a full implementation, this would start a sub-workflow
        Ok(serde_json::json!({"subworkflow": "completed", "node_id": node.id}))
    }

    /// Execute human interaction node (placeholder)
    async fn execute_human_node(
        &self,
        _execution_id: &str,
        node: &WorkflowNode,
    ) -> Result<serde_json::Value> {
        // In a full implementation, this would wait for human input
        Ok(serde_json::json!({"human_input": "received", "node_id": node.id}))
    }

    /// Evaluate a condition expression
    async fn evaluate_condition(&self, condition: &str, context: &WorkflowContext) -> Result<bool> {
        // Simple condition evaluation - in practice would use a proper expression evaluator
        // For now, just check if a variable exists and is true
        if let Some(var_name) = condition.strip_prefix("$") {
            if let Some(value) = context.variables.get(var_name) {
                return Ok(value.as_bool().unwrap_or(false));
            }
        }

        // Default evaluation
        Ok(true)
    }

    /// Get next nodes based on current node and execution result
    async fn get_next_nodes(
        &self,
        execution_id: &str,
        current_node: &str,
        _result: &serde_json::Value,
    ) -> Result<Vec<String>> {
        let executions = self.active_executions.read().await;
        let execution = executions
            .get(execution_id)
            .ok_or_else(|| anyhow!("Execution not found: {}", execution_id))?;

        let workflows = self.workflows.read().await;
        let workflow = workflows
            .get(&execution.workflow_id)
            .ok_or_else(|| anyhow!("Workflow not found: {}", execution.workflow_id))?;

        let mut next_nodes = Vec::new();

        for edge in &workflow.edges {
            if edge.from == current_node {
                // Check if edge condition is met
                if let Some(condition) = &edge.condition {
                    if !self
                        .evaluate_condition(condition, &execution.context)
                        .await?
                    {
                        continue;
                    }
                }

                next_nodes.push(edge.to.clone());
            }
        }

        Ok(next_nodes)
    }

    /// Record an execution step
    async fn record_execution_step(
        &self,
        execution_id: &str,
        node_id: &str,
        step_type: StepType,
        result: Option<serde_json::Value>,
        error: Option<String>,
    ) -> Result<()> {
        let mut executions = self.active_executions.write().await;
        if let Some(execution) = executions.get_mut(execution_id) {
            let step = ExecutionStep {
                node_id: node_id.to_string(),
                step_type,
                started_at: std::time::SystemTime::now(),
                completed_at: Some(std::time::SystemTime::now()),
                result,
                error,
                retry_count: 0,
            };
            execution.execution_history.push(step);
        }
        Ok(())
    }

    /// Complete workflow execution
    async fn complete_workflow(&self, execution_id: &str) -> Result<()> {
        let mut executions = self.active_executions.write().await;
        if let Some(execution) = executions.get_mut(execution_id) {
            execution.status = ExecutionStatus::Completed;
            execution.completed_at = Some(std::time::SystemTime::now());

            let _ = self.event_bus.send(WorkflowEvent::WorkflowCompleted {
                execution_id: execution_id.to_string(),
                result: serde_json::json!({"status": "completed"}),
            });
        }
        Ok(())
    }

    /// Validate workflow definition
    fn validate_workflow(&self, workflow: &WorkflowDefinition) -> Result<()> {
        // Check entry point exists
        if !workflow.nodes.contains_key(&workflow.entry_point) {
            return Err(anyhow!(
                "Entry point node not found: {}",
                workflow.entry_point
            ));
        }

        // Check all edge references exist
        for edge in &workflow.edges {
            if !workflow.nodes.contains_key(&edge.from) {
                return Err(anyhow!(
                    "Edge references non-existent from node: {}",
                    edge.from
                ));
            }
            if !workflow.nodes.contains_key(&edge.to) {
                return Err(anyhow!("Edge references non-existent to node: {}", edge.to));
            }
        }

        Ok(())
    }

    /// Get workflow execution status
    pub async fn get_execution_status(&self, execution_id: &str) -> Result<ExecutionStatus> {
        let executions = self.active_executions.read().await;
        let execution = executions
            .get(execution_id)
            .ok_or_else(|| anyhow!("Execution not found: {}", execution_id))?;
        Ok(execution.status.clone())
    }

    /// Cancel workflow execution
    pub async fn cancel_workflow(&self, execution_id: &str) -> Result<()> {
        let mut executions = self.active_executions.write().await;
        if let Some(execution) = executions.get_mut(execution_id) {
            execution.status = ExecutionStatus::Cancelled;
            execution.completed_at = Some(std::time::SystemTime::now());
        }
        Ok(())
    }
}

impl Clone for WorkflowEngine {
    fn clone(&self) -> Self {
        Self {
            workflows: self.workflows.clone(),
            active_executions: self.active_executions.clone(),
            agents: self.agents.clone(),
            event_bus: self.event_bus.clone(),
        }
    }
}

/// Builder for creating workflow definitions
pub struct WorkflowBuilder {
    nodes: Vec<WorkflowNode>,
    edges: Vec<WorkflowEdge>,
    metadata: HashMap<String, String>,
}

impl Default for WorkflowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowBuilder {
    /// Create a new workflow builder
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a node to the workflow
    pub fn add_node(mut self, node: WorkflowNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// Add an edge to the workflow
    pub fn add_edge(mut self, edge: WorkflowEdge) -> Self {
        self.edges.push(edge);
        self
    }

    /// Add metadata to the workflow
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Build the workflow definition
    pub fn build(self) -> WorkflowDefinition {
        // Convert Vec<WorkflowNode> to HashMap<String, WorkflowNode>
        let mut nodes_map = HashMap::new();
        for node in self.nodes {
            nodes_map.insert(node.id.clone(), node);
        }

        WorkflowDefinition {
            id: Uuid::new_v4().to_string(),
            name: "Generated Workflow".to_string(),
            description: "Generated using WorkflowBuilder".to_string(),
            nodes: nodes_map,
            edges: self.edges,
            entry_point: "start".to_string(),
            exit_points: vec!["end".to_string()],
            metadata: self.metadata,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_workflow_engine() {
        let (engine, mut _event_receiver) = WorkflowEngine::new();

        // Create simple workflow
        let workflow = WorkflowDefinition {
            id: "test_workflow".to_string(),
            name: "Test Workflow".to_string(),
            description: "Simple test workflow".to_string(),
            nodes: {
                let mut nodes = HashMap::new();
                nodes.insert(
                    "start".to_string(),
                    WorkflowNode {
                        id: "start".to_string(),
                        node_type: NodeType::Start,
                        agent_id: None,
                        command: None,
                        condition: None,
                        retry_policy: None,
                        timeout_ms: None,
                        metadata: HashMap::new(),
                    },
                );
                nodes.insert(
                    "end".to_string(),
                    WorkflowNode {
                        id: "end".to_string(),
                        node_type: NodeType::End,
                        agent_id: None,
                        command: None,
                        condition: None,
                        retry_policy: None,
                        timeout_ms: None,
                        metadata: HashMap::new(),
                    },
                );
                nodes
            },
            edges: vec![WorkflowEdge {
                from: "start".to_string(),
                to: "end".to_string(),
                condition: None,
                weight: None,
                metadata: HashMap::new(),
            }],
            entry_point: "start".to_string(),
            exit_points: vec!["end".to_string()],
            metadata: HashMap::new(),
        };

        // Register workflow
        engine.register_workflow(workflow).await.unwrap();

        // Start execution
        let context = WorkflowContext::default();
        let execution_id = engine
            .start_workflow("test_workflow", context)
            .await
            .unwrap();

        // Wait a bit for execution
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Check status
        let status = engine.get_execution_status(&execution_id).await.unwrap();
        println!("Execution status: {status:?}");
    }
}
