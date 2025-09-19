//! Enhanced Agent Communication System
//!
//! Provides a robust message bus for inter-agent communication with
//! routing, persistence, and event-driven architecture.

use crate::ml::agents::{AgentError, AgentId};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, RwLock};
use uuid::Uuid;

/// Message bus for agent communication
pub struct AgentMessageBus {
    /// Direct channels for agent-to-agent communication
    agent_channels: Arc<RwLock<HashMap<AgentId, mpsc::Sender<AgentMessage>>>>,
    /// Broadcast channel for system-wide messages
    broadcast_tx: broadcast::Sender<SystemMessage>,
    /// Message persistence layer
    message_store: Arc<dyn MessageStore>,
    /// Routing configuration
    router: Arc<MessageRouter>,
    /// Metrics for monitoring
    metrics: Arc<RwLock<MessageBusMetrics>>,
}

/// Agent message with routing and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: MessageId,
    pub from: AgentId,
    pub to: Option<AgentId>, // None for broadcast
    pub message_type: MessageType,
    pub payload: MessagePayload,
    pub timestamp: DateTime<Utc>,
    pub priority: MessagePriority,
    pub correlation_id: Option<String>,
    pub reply_to: Option<MessageId>,
    pub ttl: Option<Duration>,
}

/// System-wide messages for coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    pub id: MessageId,
    pub event_type: SystemEventType,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

/// Message types for classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Request for action or information
    Request,
    /// Response to a request
    Response,
    /// One-way notification
    Notification,
    /// Task delegation
    TaskAssignment,
    /// Status update
    StatusUpdate,
    /// Error or failure report
    Error,
    /// Coordination message
    Coordination,
}

/// Message payload containing actual data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    /// Text-based message
    Text(String),
    /// Structured data
    Json(serde_json::Value),
    /// Binary data
    Binary(Vec<u8>),
    /// Task specification
    Task(TaskSpec),
    /// Status information
    Status(StatusInfo),
    /// Error details
    Error(ErrorInfo),
}

/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
    Emergency = 5,
}

/// System event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEventType {
    AgentRegistered,
    AgentDeregistered,
    TaskCompleted,
    TaskFailed,
    SystemShutdown,
    ResourceAlert,
    PerformanceAlert,
    SecurityAlert,
}

/// Task specification for agent delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSpec {
    pub task_id: String,
    pub task_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub deadline: Option<DateTime<Utc>>,
    pub required_capabilities: Vec<String>,
}

/// Status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusInfo {
    pub component: String,
    pub status: String,
    pub health_score: f64,
    pub metrics: HashMap<String, f64>,
}

/// Error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub error_code: String,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub context: HashMap<String, serde_json::Value>,
}

/// Message router for intelligent routing
pub struct MessageRouter {
    /// Routing rules
    rules: Arc<RwLock<Vec<RoutingRule>>>,
    /// Agent capabilities registry
    capabilities: Arc<RwLock<HashMap<AgentId, Vec<String>>>>,
}

/// Routing rule for message forwarding
#[derive(Debug, Clone)]
pub struct RoutingRule {
    pub condition: RoutingCondition,
    pub action: RoutingAction,
    pub priority: u32,
}

/// Conditions for routing
pub enum RoutingCondition {
    MessageType(MessageType),
    AgentCapability(String),
    PayloadContains(String),
    Priority(MessagePriority),
    Custom(Box<dyn Fn(&AgentMessage) -> bool + Send + Sync>),
}

impl std::fmt::Debug for RoutingCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MessageType(mt) => f.debug_tuple("MessageType").field(mt).finish(),
            Self::AgentCapability(cap) => f.debug_tuple("AgentCapability").field(cap).finish(),
            Self::PayloadContains(payload) => {
                f.debug_tuple("PayloadContains").field(payload).finish()
            }
            Self::Priority(priority) => f.debug_tuple("Priority").field(priority).finish(),
            Self::Custom(_) => f.debug_tuple("Custom").field(&"<function>").finish(),
        }
    }
}

impl Clone for RoutingCondition {
    fn clone(&self) -> Self {
        match self {
            Self::MessageType(mt) => Self::MessageType(mt.clone()),
            Self::AgentCapability(cap) => Self::AgentCapability(cap.clone()),
            Self::PayloadContains(payload) => Self::PayloadContains(payload.clone()),
            Self::Priority(priority) => Self::Priority(priority.clone()),
            Self::Custom(_) => {
                // Cannot clone function pointers, so we create a placeholder
                Self::Custom(Box::new(|_| false))
            }
        }
    }
}

/// Actions to take when routing
pub enum RoutingAction {
    ForwardTo(AgentId),
    Broadcast,
    LoadBalance(Vec<AgentId>),
    Drop,
    Transform(Box<dyn Fn(AgentMessage) -> AgentMessage + Send + Sync>),
}

impl std::fmt::Debug for RoutingAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForwardTo(agent_id) => f.debug_tuple("ForwardTo").field(agent_id).finish(),
            Self::Broadcast => write!(f, "Broadcast"),
            Self::LoadBalance(agents) => f.debug_tuple("LoadBalance").field(agents).finish(),
            Self::Drop => write!(f, "Drop"),
            Self::Transform(_) => f.debug_tuple("Transform").field(&"<function>").finish(),
        }
    }
}

impl Clone for RoutingAction {
    fn clone(&self) -> Self {
        match self {
            Self::ForwardTo(agent_id) => Self::ForwardTo(agent_id.clone()),
            Self::Broadcast => Self::Broadcast,
            Self::LoadBalance(agents) => Self::LoadBalance(agents.clone()),
            Self::Drop => Self::Drop,
            Self::Transform(_) => {
                // Cannot clone function pointers, so we create a placeholder
                Self::Transform(Box::new(|msg| msg))
            }
        }
    }
}

/// Message persistence trait
#[async_trait]
pub trait MessageStore: Send + Sync {
    async fn store_message(&self, message: &AgentMessage) -> Result<()>;
    async fn get_message(&self, id: &MessageId) -> Result<Option<AgentMessage>>;
    async fn get_conversation(
        &self,
        agent1: &AgentId,
        agent2: &AgentId,
    ) -> Result<Vec<AgentMessage>>;
    async fn get_messages_by_correlation(&self, correlation_id: &str) -> Result<Vec<AgentMessage>>;
    async fn cleanup_expired(&self) -> Result<u64>;
}

/// Metrics for message bus monitoring
#[derive(Debug, Default, Clone)]
pub struct MessageBusMetrics {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub messages_dropped: u64,
    pub messages_failed: u64,
    pub average_latency_ms: f64,
    pub active_conversations: u64,
    pub agent_count: u64,
}

/// Unique message identifier
pub type MessageId = Uuid;
pub type Duration = std::time::Duration;

impl AgentMessageBus {
    /// Create a new message bus
    pub fn new(message_store: Arc<dyn MessageStore>) -> Self {
        let (broadcast_tx, _) = broadcast::channel(1000);

        Self {
            agent_channels: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx,
            message_store,
            router: Arc::new(MessageRouter::new()),
            metrics: Arc::new(RwLock::new(MessageBusMetrics::default())),
        }
    }

    /// Register an agent for communication
    pub async fn register_agent(
        &self,
        agent_id: AgentId,
        capabilities: Vec<String>,
    ) -> Result<mpsc::Receiver<AgentMessage>> {
        let (tx, rx) = mpsc::channel(100);

        // Register channel
        {
            let mut channels = self.agent_channels.write().await;
            channels.insert(agent_id.clone(), tx);
        }

        // Register capabilities
        {
            let mut caps = self.router.capabilities.write().await;
            caps.insert(agent_id, capabilities);
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.agent_count += 1;
        }

        Ok(rx)
    }

    /// Deregister an agent
    pub async fn deregister_agent(&self, agent_id: &AgentId) -> Result<()> {
        // Remove channel
        {
            let mut channels = self.agent_channels.write().await;
            channels.remove(agent_id);
        }

        // Remove capabilities
        {
            let mut caps = self.router.capabilities.write().await;
            caps.remove(agent_id);
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.agent_count = metrics.agent_count.saturating_sub(1);
        }

        // Broadcast deregistration
        let system_msg = SystemMessage {
            id: Uuid::new_v4(),
            event_type: SystemEventType::AgentDeregistered,
            data: serde_json::json!({ "agent_id": agent_id }),
            timestamp: Utc::now(),
            source: "message_bus".to_string(),
        };

        let _ = self.broadcast_tx.send(system_msg);

        Ok(())
    }

    /// Send a message to a specific agent
    pub async fn send_message(&self, message: AgentMessage) -> Result<()> {
        // Store message for persistence
        self.message_store.store_message(&message).await?;

        // Apply routing rules
        let routed_message = self.router.route_message(message).await?;

        // Send based on routing decision
        match &routed_message.to {
            Some(agent_id) => {
                // Direct message
                let channels = self.agent_channels.read().await;
                if let Some(tx) = channels.get(agent_id) {
                    if (tx.send(routed_message.clone()).await).is_err() {
                        // Channel closed, remove it
                        drop(channels);
                        let mut channels = self.agent_channels.write().await;
                        channels.remove(agent_id);

                        let mut metrics = self.metrics.write().await;
                        metrics.messages_failed += 1;

                        return Err(AgentError::CommunicationError(format!(
                            "Agent {} is not reachable",
                            agent_id.0
                        ))
                        .into());
                    }
                } else {
                    let mut metrics = self.metrics.write().await;
                    metrics.messages_dropped += 1;

                    return Err(AgentError::CommunicationError(format!(
                        "Agent {} not found",
                        agent_id.0
                    ))
                    .into());
                }
            }
            None => {
                // Broadcast message
                self.broadcast_to_all_agents(routed_message).await?;
            }
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.messages_sent += 1;
        }

        Ok(())
    }

    /// Broadcast message to all registered agents
    async fn broadcast_to_all_agents(&self, message: AgentMessage) -> Result<()> {
        let channels = self.agent_channels.read().await;
        let mut failed_count = 0;

        for (agent_id, tx) in channels.iter() {
            if (tx.send(message.clone()).await).is_err() {
                // Agent channel is closed
                failed_count += 1;
                log::warn!(
                    "Failed to deliver broadcast message to agent {}",
                    agent_id.0
                );
            }
        }

        if failed_count > 0 {
            let mut metrics = self.metrics.write().await;
            metrics.messages_failed += failed_count;
        }

        Ok(())
    }

    /// Subscribe to system messages
    pub fn subscribe_system_messages(&self) -> broadcast::Receiver<SystemMessage> {
        self.broadcast_tx.subscribe()
    }

    /// Send a system message
    pub async fn send_system_message(&self, message: SystemMessage) -> Result<()> {
        let _ = self.broadcast_tx.send(message);
        Ok(())
    }

    /// Get conversation history between two agents
    pub async fn get_conversation(
        &self,
        agent1: &AgentId,
        agent2: &AgentId,
    ) -> Result<Vec<AgentMessage>> {
        self.message_store.get_conversation(agent1, agent2).await
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> MessageBusMetrics {
        self.metrics.read().await.clone()
    }

    /// Cleanup expired messages
    pub async fn cleanup_expired_messages(&self) -> Result<u64> {
        self.message_store.cleanup_expired().await
    }
}

impl Default for MessageRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageRouter {
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            capabilities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_rule(&self, rule: RoutingRule) {
        let mut rules = self.rules.write().await;
        rules.push(rule);
        rules.sort_by_key(|r| r.priority);
    }

    pub async fn route_message(&self, mut message: AgentMessage) -> Result<AgentMessage> {
        let rules = self.rules.read().await;

        for rule in rules.iter() {
            if self.matches_condition(&rule.condition, &message).await {
                match &rule.action {
                    RoutingAction::ForwardTo(agent_id) => {
                        message.to = Some(agent_id.clone());
                        break;
                    }
                    RoutingAction::Broadcast => {
                        message.to = None;
                        break;
                    }
                    RoutingAction::LoadBalance(agents) => {
                        // Simple round-robin for now
                        if !agents.is_empty() {
                            let index = message.id.as_u128() as usize % agents.len();
                            message.to = Some(agents[index].clone());
                        }
                        break;
                    }
                    RoutingAction::Drop => {
                        return Err(AgentError::CommunicationError(
                            "Message dropped by routing rule".to_string(),
                        )
                        .into());
                    }
                    RoutingAction::Transform(transform) => {
                        message = transform(message);
                    }
                }
            }
        }

        Ok(message)
    }

    async fn matches_condition(
        &self,
        condition: &RoutingCondition,
        message: &AgentMessage,
    ) -> bool {
        match condition {
            RoutingCondition::MessageType(msg_type) => {
                std::mem::discriminant(&message.message_type) == std::mem::discriminant(msg_type)
            }
            RoutingCondition::Priority(priority) => message.priority >= *priority,
            RoutingCondition::PayloadContains(text) => match &message.payload {
                MessagePayload::Text(content) => content.contains(text),
                MessagePayload::Json(value) => value.to_string().contains(text),
                _ => false,
            },
            RoutingCondition::AgentCapability(capability) => {
                let capabilities = self.capabilities.read().await;
                if let Some(agent_caps) = capabilities.get(&message.from) {
                    agent_caps.contains(capability)
                } else {
                    false
                }
            }
            RoutingCondition::Custom(func) => func(message),
        }
    }
}

/// Helper for creating messages
impl AgentMessage {
    pub fn new(
        from: AgentId,
        to: Option<AgentId>,
        message_type: MessageType,
        payload: MessagePayload,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            message_type,
            payload,
            timestamp: Utc::now(),
            priority: MessagePriority::Normal,
            correlation_id: None,
            reply_to: None,
            ttl: None,
        }
    }

    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn as_reply_to(mut self, original_message_id: MessageId) -> Self {
        self.reply_to = Some(original_message_id);
        self
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Simple in-memory message store for testing
    struct InMemoryMessageStore {
        messages: Arc<RwLock<Vec<AgentMessage>>>,
    }

    impl InMemoryMessageStore {
        fn new() -> Self {
            Self {
                messages: Arc::new(RwLock::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl MessageStore for InMemoryMessageStore {
        async fn store_message(&self, message: &AgentMessage) -> Result<()> {
            let mut messages = self.messages.write().await;
            messages.push(message.clone());
            Ok(())
        }

        async fn get_message(&self, id: &MessageId) -> Result<Option<AgentMessage>> {
            let messages = self.messages.read().await;
            Ok(messages.iter().find(|m| m.id == *id).cloned())
        }

        async fn get_conversation(
            &self,
            agent1: &AgentId,
            agent2: &AgentId,
        ) -> Result<Vec<AgentMessage>> {
            let messages = self.messages.read().await;
            Ok(messages
                .iter()
                .filter(|m| {
                    (m.from == *agent1 && m.to.as_ref() == Some(agent2))
                        || (m.from == *agent2 && m.to.as_ref() == Some(agent1))
                })
                .cloned()
                .collect())
        }

        async fn get_messages_by_correlation(
            &self,
            correlation_id: &str,
        ) -> Result<Vec<AgentMessage>> {
            let messages = self.messages.read().await;
            Ok(messages
                .iter()
                .filter(|m| m.correlation_id.as_ref() == Some(&correlation_id.to_string()))
                .cloned()
                .collect())
        }

        async fn cleanup_expired(&self) -> Result<u64> {
            // For testing, just return 0
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_message_bus_creation() {
        let store = Arc::new(InMemoryMessageStore::new());
        let bus = AgentMessageBus::new(store);

        let metrics = bus.get_metrics().await;
        assert_eq!(metrics.agent_count, 0);
    }

    #[tokio::test]
    async fn test_agent_registration() {
        let store = Arc::new(InMemoryMessageStore::new());
        let bus = AgentMessageBus::new(store);

        let agent_id = AgentId("test-agent".to_string());
        let capabilities = vec!["text-processing".to_string()];

        let _rx = bus
            .register_agent(agent_id.clone(), capabilities)
            .await
            .unwrap();

        let metrics = bus.get_metrics().await;
        assert_eq!(metrics.agent_count, 1);
    }

    #[tokio::test]
    async fn test_message_sending() {
        let store = Arc::new(InMemoryMessageStore::new());
        let bus = AgentMessageBus::new(store);

        let agent1 = AgentId("agent1".to_string());
        let agent2 = AgentId("agent2".to_string());

        let mut rx1 = bus.register_agent(agent1.clone(), vec![]).await.unwrap();
        let _rx2 = bus.register_agent(agent2.clone(), vec![]).await.unwrap();

        let message = AgentMessage::new(
            agent2.clone(),
            Some(agent1.clone()),
            MessageType::Request,
            MessagePayload::Text("Hello".to_string()),
        );

        bus.send_message(message).await.unwrap();

        // Check if message was received
        let received = rx1.recv().await.unwrap();
        assert_eq!(received.from, agent2);
        match received.payload {
            MessagePayload::Text(text) => assert_eq!(text, "Hello"),
            _ => panic!("Wrong payload type"),
        }
    }
}
