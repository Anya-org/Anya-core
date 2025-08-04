//! Workflow Builder
//!
//! Provides a fluent API for building complex agent workflows with minimal boilerplate.

use super::*;
use std::collections::HashMap;

/// Builder for creating workflow definitions with a fluent API
pub struct WorkflowBuilder {
    id: String,
    name: String,
    description: String,
    nodes: HashMap<String, WorkflowNode>,
    edges: Vec<WorkflowEdge>,
    entry_point: Option<String>,
    exit_points: Vec<String>,
    metadata: HashMap<String, String>,
}

impl WorkflowBuilder {
    /// Create a new workflow builder
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: String::new(),
            description: String::new(),
            nodes: HashMap::new(),
            edges: Vec::new(),
            entry_point: None,
            exit_points: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set workflow name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set workflow description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Set entry point
    pub fn entry_point(mut self, node_id: impl Into<String>) -> Self {
        self.entry_point = Some(node_id.into());
        self
    }

    /// Add an exit point
    pub fn exit_point(mut self, node_id: impl Into<String>) -> Self {
        self.exit_points.push(node_id.into());
        self
    }

    /// Add a start node
    pub fn start_node(mut self, id: impl Into<String>) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::Start,
            agent_id: None,
            command: None,
            condition: None,
            retry_policy: None,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id.clone(), node);

        // Auto-set as entry point if not set
        if self.entry_point.is_none() {
            self.entry_point = Some(node_id);
        }

        self
    }

    /// Add an end node
    pub fn end_node(mut self, id: impl Into<String>) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::End,
            agent_id: None,
            command: None,
            condition: None,
            retry_policy: None,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id.clone(), node);
        self.exit_points.push(node_id);
        self
    }

    /// Add an action node
    pub fn action_node(
        mut self,
        id: impl Into<String>,
        agent_id: impl Into<String>,
        command: AgentCommand,
    ) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::Action,
            agent_id: Some(agent_id.into()),
            command: Some(command),
            condition: None,
            retry_policy: None,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id, node);
        self
    }

    /// Add an action node with retry policy
    pub fn action_node_with_retry(
        mut self,
        id: impl Into<String>,
        agent_id: impl Into<String>,
        command: AgentCommand,
        retry_policy: RetryPolicy,
    ) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::Action,
            agent_id: Some(agent_id.into()),
            command: Some(command),
            condition: None,
            retry_policy: Some(retry_policy),
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id, node);
        self
    }

    /// Add a decision node
    pub fn decision_node(mut self, id: impl Into<String>, condition: impl Into<String>) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::Decision,
            agent_id: None,
            command: None,
            condition: Some(condition.into()),
            retry_policy: None,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id, node);
        self
    }

    /// Add a parallel node
    pub fn parallel_node(mut self, id: impl Into<String>) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::Parallel,
            agent_id: None,
            command: None,
            condition: None,
            retry_policy: None,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id, node);
        self
    }

    /// Add a wait node
    pub fn wait_node(mut self, id: impl Into<String>, timeout_ms: u64) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::Wait,
            agent_id: None,
            command: None,
            condition: None,
            retry_policy: None,
            timeout_ms: Some(timeout_ms),
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id, node);
        self
    }

    /// Add a merge node
    pub fn merge_node(mut self, id: impl Into<String>) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::Merge,
            agent_id: None,
            command: None,
            condition: None,
            retry_policy: None,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id, node);
        self
    }

    /// Add a human interaction node
    pub fn human_node(mut self, id: impl Into<String>) -> Self {
        let node_id = id.into();
        let node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::Human,
            agent_id: None,
            command: None,
            condition: None,
            retry_policy: None,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        self.nodes.insert(node_id, node);
        self
    }

    /// Add a subworkflow node
    pub fn subworkflow_node(
        mut self,
        id: impl Into<String>,
        workflow_id: impl Into<String>,
    ) -> Self {
        let node_id = id.into();
        let mut node = WorkflowNode {
            id: node_id.clone(),
            node_type: NodeType::SubWorkflow,
            agent_id: None,
            command: None,
            condition: None,
            retry_policy: None,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        node.metadata
            .insert("workflow_id".to_string(), workflow_id.into());
        self.nodes.insert(node_id, node);
        self
    }

    /// Add an edge between two nodes
    pub fn edge(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        let edge = WorkflowEdge {
            from: from.into(),
            to: to.into(),
            condition: None,
            weight: None,
            metadata: HashMap::new(),
        };
        self.edges.push(edge);
        self
    }

    /// Add a conditional edge
    pub fn conditional_edge(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        condition: impl Into<String>,
    ) -> Self {
        let edge = WorkflowEdge {
            from: from.into(),
            to: to.into(),
            condition: Some(condition.into()),
            weight: None,
            metadata: HashMap::new(),
        };
        self.edges.push(edge);
        self
    }

    /// Add a weighted edge
    pub fn weighted_edge(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        weight: f32,
    ) -> Self {
        let edge = WorkflowEdge {
            from: from.into(),
            to: to.into(),
            condition: None,
            weight: Some(weight),
            metadata: HashMap::new(),
        };
        self.edges.push(edge);
        self
    }

    /// Build the workflow definition
    pub fn build(self) -> Result<WorkflowDefinition> {
        let entry_point = self
            .entry_point
            .ok_or_else(|| anyhow!("Entry point must be specified"))?;

        if self.exit_points.is_empty() {
            return Err(anyhow!("At least one exit point must be specified"));
        }

        let workflow = WorkflowDefinition {
            id: self.id,
            name: self.name,
            description: self.description,
            nodes: self.nodes,
            edges: self.edges,
            entry_point,
            exit_points: self.exit_points,
            metadata: self.metadata,
        };

        Ok(workflow)
    }
}

/// Helper for creating common retry policies
impl RetryPolicy {
    /// Create exponential backoff retry policy
    pub fn exponential_backoff(
        max_attempts: u32,
        initial_delay_ms: u64,
        max_delay_ms: u64,
    ) -> Self {
        Self {
            max_attempts,
            initial_delay_ms,
            max_delay_ms,
            backoff_multiplier: 2.0,
            retry_on_errors: vec!["timeout".to_string(), "network".to_string()],
        }
    }

    /// Create linear backoff retry policy
    pub fn linear_backoff(max_attempts: u32, delay_ms: u64) -> Self {
        Self {
            max_attempts,
            initial_delay_ms: delay_ms,
            max_delay_ms: delay_ms,
            backoff_multiplier: 1.0,
            retry_on_errors: vec!["timeout".to_string(), "network".to_string()],
        }
    }

    /// Create immediate retry policy
    pub fn immediate(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            initial_delay_ms: 0,
            max_delay_ms: 0,
            backoff_multiplier: 1.0,
            retry_on_errors: vec!["timeout".to_string()],
        }
    }
}

/// Pre-built workflow templates
pub struct WorkflowTemplates;

impl WorkflowTemplates {
    /// Create a simple sequential workflow
    pub fn sequential(
        id: impl Into<String>,
        name: impl Into<String>,
        actions: Vec<(String, String, AgentCommand)>, // (node_id, agent_id, command)
    ) -> Result<WorkflowDefinition> {
        let mut builder = WorkflowBuilder::new(id)
            .name(name)
            .start_node("start")
            .end_node("end");

        let mut prev_node = "start".to_string();

        for (i, (node_id, agent_id, command)) in actions.into_iter().enumerate() {
            builder = builder.action_node(&node_id, agent_id, command);
            builder = builder.edge(&prev_node, &node_id);
            prev_node = node_id;
        }

        builder = builder.edge(&prev_node, "end");
        builder.build()
    }

    /// Create a conditional workflow with branching
    pub fn conditional_branch(
        id: impl Into<String>,
        name: impl Into<String>,
        condition: impl Into<String>,
        true_branch: (String, String, AgentCommand), // (node_id, agent_id, command)
        false_branch: (String, String, AgentCommand), // (node_id, agent_id, command)
    ) -> Result<WorkflowDefinition> {
        let builder = WorkflowBuilder::new(id)
            .name(name)
            .start_node("start")
            .decision_node("decision", condition)
            .action_node(&true_branch.0, &true_branch.1, true_branch.2)
            .action_node(&false_branch.0, &false_branch.1, false_branch.2)
            .end_node("end")
            .edge("start", "decision")
            .conditional_edge("decision", &true_branch.0, "$condition_result")
            .conditional_edge("decision", &false_branch.0, "!$condition_result")
            .edge(&true_branch.0, "end")
            .edge(&false_branch.0, "end");

        builder.build()
    }

    /// Create a parallel execution workflow
    pub fn parallel_execution(
        id: impl Into<String>,
        name: impl Into<String>,
        parallel_actions: Vec<(String, String, AgentCommand)>, // (node_id, agent_id, command)
    ) -> Result<WorkflowDefinition> {
        let mut builder = WorkflowBuilder::new(id)
            .name(name)
            .start_node("start")
            .parallel_node("parallel")
            .merge_node("merge")
            .end_node("end")
            .edge("start", "parallel");

        // Add parallel branches
        for (node_id, agent_id, command) in parallel_actions {
            builder = builder
                .action_node(&node_id, agent_id, command)
                .edge("parallel", &node_id)
                .edge(&node_id, "merge");
        }

        builder = builder.edge("merge", "end");
        builder.build()
    }

    /// Create a retry workflow with error handling
    pub fn retry_workflow(
        id: impl Into<String>,
        name: impl Into<String>,
        main_action: (String, String, AgentCommand),
        fallback_action: (String, String, AgentCommand),
        max_retries: u32,
    ) -> Result<WorkflowDefinition> {
        let retry_policy = RetryPolicy::exponential_backoff(max_retries, 1000, 10000);

        let builder = WorkflowBuilder::new(id)
            .name(name)
            .start_node("start")
            .action_node_with_retry(&main_action.0, &main_action.1, main_action.2, retry_policy)
            .action_node(&fallback_action.0, &fallback_action.1, fallback_action.2)
            .end_node("end")
            .edge("start", &main_action.0)
            .conditional_edge(&main_action.0, "end", "$success")
            .conditional_edge(&main_action.0, &fallback_action.0, "$failure")
            .edge(&fallback_action.0, "end");

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::agents::AgentCommand;

    #[test]
    fn test_workflow_builder() {
        let workflow = WorkflowBuilder::new("test")
            .name("Test Workflow")
            .description("A test workflow")
            .start_node("start")
            .action_node(
                "process",
                "processor_agent",
                AgentCommand::Process {
                    data: vec![1.0, 2.0, 3.0],
                    operation: "transform".to_string(),
                    parameters: HashMap::new(),
                },
            )
            .end_node("end")
            .edge("start", "process")
            .edge("process", "end")
            .build()
            .unwrap();

        assert_eq!(workflow.id, "test");
        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.nodes.len(), 3);
        assert_eq!(workflow.edges.len(), 2);
        assert_eq!(workflow.entry_point, "start");
        assert_eq!(workflow.exit_points, vec!["end"]);
    }

    #[test]
    fn test_sequential_template() {
        let actions = vec![
            (
                "step1".to_string(),
                "agent1".to_string(),
                AgentCommand::Process {
                    data: vec![1.0],
                    operation: "step1".to_string(),
                    parameters: HashMap::new(),
                },
            ),
            (
                "step2".to_string(),
                "agent2".to_string(),
                AgentCommand::Process {
                    data: vec![2.0],
                    operation: "step2".to_string(),
                    parameters: HashMap::new(),
                },
            ),
        ];

        let workflow = WorkflowTemplates::sequential("seq", "Sequential", actions).unwrap();

        assert_eq!(workflow.nodes.len(), 4); // start, step1, step2, end
        assert_eq!(workflow.edges.len(), 3); // start->step1, step1->step2, step2->end
    }

    #[test]
    fn test_conditional_template() {
        let true_branch = (
            "true_action".to_string(),
            "agent1".to_string(),
            AgentCommand::Process {
                data: vec![1.0],
                operation: "true_path".to_string(),
                parameters: HashMap::new(),
            },
        );

        let false_branch = (
            "false_action".to_string(),
            "agent2".to_string(),
            AgentCommand::Process {
                data: vec![2.0],
                operation: "false_path".to_string(),
                parameters: HashMap::new(),
            },
        );

        let workflow = WorkflowTemplates::conditional_branch(
            "conditional",
            "Conditional Workflow",
            "$should_take_true_path",
            true_branch,
            false_branch,
        )
        .unwrap();

        assert_eq!(workflow.nodes.len(), 5); // start, decision, true_action, false_action, end
        assert!(workflow.edges.len() >= 5);
    }
}
