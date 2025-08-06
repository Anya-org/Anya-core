# ml/orchestration Module

Advanced Agent Orchestration System

Provides sophisticated multi-agent workflow coordination inspired by LangGraph,
with support for complex execution graphs, conditional flows, and cross-protocol operations.

## Overview

The `ml/orchestration` module implements a comprehensive workflow engine for coordinating the actions of multiple AI agents within the Anya Core system. Based on directed graph architecture similar to LangGraph, this module enables complex multi-agent workflows with conditional branching, parallel execution paths, and sophisticated control flow.

## Key Components

### WorkflowEngine

The central orchestration system that:

- Manages workflow definitions and instances
- Coordinates agent execution
- Handles conditional logic and branching
- Provides event-based monitoring
- Supports parallel and sequential execution patterns

### Workflow Structure

Workflows are defined as directed graphs with:

- **Nodes**: Individual units of work (agent actions, decisions, etc.)
- **Edges**: Connections between nodes with optional conditions
- **Context**: Shared state accessible across the workflow
- **Events**: Signals emitted during workflow execution

### Node Types

- **Action**: Execute an agent task or operation
- **Decision**: Conditional branching based on state
- **Parallel**: Concurrent execution of multiple branches
- **Wait**: Pause for external events or conditions
- **Merge**: Consolidate results from multiple branches
- **SubWorkflow**: Nested workflow execution
- **Human**: Integration points for human-in-the-loop operations

### Error Handling

Robust error management with:

- Configurable retry policies
- Timeout handling
- Graceful failure modes
- Execution history tracking

## Usage Examples

```rust
// Create a workflow engine
let (engine, event_receiver) = WorkflowEngine::new();

// Build a workflow definition
let workflow = WorkflowBuilder::new()
    .add_node(WorkflowNode {
        id: "start".to_string(),
        node_type: NodeType::Start,
        agent_id: None,
        command: None,
        condition: None,
        retry_policy: None,
        timeout_ms: None,
        metadata: HashMap::new(),
    })
    .add_node(WorkflowNode {
        id: "analyze_data".to_string(),
        node_type: NodeType::Action,
        agent_id: Some("data_analyzer".to_string()),
        command: Some(AgentCommand {
            action: "analyze".to_string(),
            parameters: HashMap::new(),
            timeout_ms: Some(5000),
        }),
        condition: None,
        retry_policy: Some(RetryPolicy {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
            retry_on_errors: vec!["timeout".to_string()],
        }),
        timeout_ms: Some(10000),
        metadata: HashMap::new(),
    })
    .add_node(WorkflowNode {
        id: "decide_action".to_string(),
        node_type: NodeType::Decision,
        agent_id: None,
        command: None,
        condition: Some("$has_anomaly".to_string()),
        retry_policy: None,
        timeout_ms: None,
        metadata: HashMap::new(),
    })
    .add_node(WorkflowNode {
        id: "end".to_string(),
        node_type: NodeType::End,
        agent_id: None,
        command: None,
        condition: None,
        retry_policy: None,
        timeout_ms: None,
        metadata: HashMap::new(),
    })
    .add_edge(WorkflowEdge {
        from: "start".to_string(),
        to: "analyze_data".to_string(),
        condition: None,
        weight: None,
        metadata: HashMap::new(),
    })
    .add_edge(WorkflowEdge {
        from: "analyze_data".to_string(),
        to: "decide_action".to_string(),
        condition: None,
        weight: None,
        metadata: HashMap::new(),
    })
    .add_edge(WorkflowEdge {
        from: "decide_action".to_string(),
        to: "end".to_string(),
        condition: None,
        weight: None,
        metadata: HashMap::new(),
    })
    .build();

// Register workflow and agents
engine.register_workflow(workflow).await?;
engine.register_agent("data_analyzer".to_string(), Box::new(MyAgent::new())).await?;

// Execute workflow with initial context
let context = WorkflowContext::default();
let execution_id = engine.start_workflow("workflow_id", context).await?;

// Monitor workflow events
while let Some(event) = event_receiver.recv().await {
    match event {
        WorkflowEvent::WorkflowCompleted { execution_id, result } => {
            println!("Workflow completed: {}", execution_id);
            println!("Result: {:?}", result);
            break;
        },
        WorkflowEvent::NodeCompleted { execution_id, node_id, result } => {
            println!("Node {} completed in workflow {}", node_id, execution_id);
        },
        // Handle other events...
    }
}
```

## Event-Driven Architecture

The orchestration system uses an event-driven approach, emitting events for:

- Workflow lifecycle (started, completed, failed)
- Node execution (started, completed, failed)
- Agent responses
- Conditional branch decisions
- Error conditions and retries

This allows for real-time monitoring, logging, and integration with external systems.

## For more information

See the comprehensive documentation in the [docs/](../../../docs/) directory.
