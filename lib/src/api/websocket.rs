// OPSource WebSocket Implementation
// Provides real-time communication for blockchain events and notifications

use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// WebSocket message types
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    Text(String),
    Bitcoin(BitcoinEvent),
    RGB(RgbEvent),
    RSK(RskEvent),
    Stacks(StacksEvent),
    Subscription(SubscriptionEvent),
}

/// Bitcoin-related events
#[derive(Serialize, Deserialize)]
pub struct BitcoinEvent {
    pub event: String,
    pub txid: Option<String>,
    pub address: Option<String>,
    pub amount: Option<f64>,
    pub confirmations: Option<u64>,
}

/// RGB asset events
#[derive(Serialize, Deserialize)]
pub struct RgbEvent {
    pub event: String,
    pub contract_id: Option<String>,
    pub asset_name: Option<String>,
    pub amount: Option<u64>,
}

/// RSK contract events
#[derive(Serialize, Deserialize)]
pub struct RskEvent {
    pub event: String,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub block_number: Option<u64>,
}

/// Stacks contract events
#[derive(Serialize, Deserialize)]
pub struct StacksEvent {
    pub event: String,
    pub contract_id: Option<String>,
    pub transaction_id: Option<String>,
    pub block_height: Option<u64>,
}

/// Subscription events
#[derive(Serialize, Deserialize)]
pub struct SubscriptionEvent {
    pub action: String,
    pub topic: String,
    pub status: String,
}

/// WebSocket connection state
pub struct WebSocketConnection {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    last_heartbeat: Instant,
    /// Subscribed topics
    subscriptions: Vec<String>,
    /// User ID if authenticated
    user_id: Option<String>,
}

impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }
}

/// Handler for ws::Message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                // Handle text messages - typically JSON commands
                if let Ok(message) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(command) = message.get("command").and_then(|c| c.as_str()) {
                        match command {
                            "subscribe" => {
                                if let Some(topic) = message.get("topic").and_then(|t| t.as_str()) {
                                    self.subscriptions.push(topic.to_string());
                                    let response = SubscriptionEvent {
                                        action: "subscribe".to_string(),
                                        topic: topic.to_string(),
                                        status: "success".to_string(),
                                    };
                                    if let Ok(response_json) = serde_json::to_string(&WsMessage::Subscription(response)) {
                                        ctx.text(response_json);
                                    }
                                }
                            }
                            "unsubscribe" => {
                                if let Some(topic) = message.get("topic").and_then(|t| t.as_str()) {
                                    self.subscriptions.retain(|t| t != topic);
                                    let response = SubscriptionEvent {
                                        action: "unsubscribe".to_string(),
                                        topic: topic.to_string(),
                                        status: "success".to_string(),
                                    };
                                    if let Ok(response_json) = serde_json::to_string(&WsMessage::Subscription(response)) {
                                        ctx.text(response_json);
                                    }
                                }
                            }
                            _ => {
                                // Unknown command
                                ctx.text(format!("{{\"error\": \"Unknown command: {}\"}}", command));
                            }
                        }
                    }
                } else {
                    // Invalid JSON
                    ctx.text("{\"error\": \"Invalid JSON format\"}");
                }
            }
            Ok(ws::Message::Binary(_)) => {
                // Not handling binary messages
                ctx.text("{\"error\": \"Binary messages not supported\"}");
            }
            Ok(ws::Message::Close(reason)) => {
                // Client disconnected
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WebSocketConnection {
    pub fn new(user_id: Option<String>) -> Self {
        Self {
            last_heartbeat: Instant::now(),
            subscriptions: Vec::new(),
            user_id,
        }
    }

    /// Send ping to client every 5 seconds (HEARTBEAT_INTERVAL)
    /// and check if client has sent us a ping in the last 10 seconds (CLIENT_TIMEOUT)
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // Check client heartbeat
            if Instant::now().duration_since(act.last_heartbeat) > CLIENT_TIMEOUT {
                // Heartbeat timed out
                println!("WebSocket Client heartbeat failed, disconnecting!");
                
                // Stop actor
                ctx.stop();
                return;
            }
            
            // Send ping
            ctx.ping(b"");
        });
    }
}

/// WebSocket route handler
pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    // Optional: extract user ID from query or JWT
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    // Extract user ID if provided
    let user_id = query.get("user_id").map(|id| id.to_string());
    
    // Create WebSocket connection
    ws::start(WebSocketConnection::new(user_id), &req, stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    
    #[actix_web::test]
    async fn test_websocket() {
        // Create test app
        let app = test::init_service(
            App::new().route("/ws", web::get().to(websocket_handler))
        ).await;
        
        // WebSocket testing is complex and typically requires integration tests
        // This is a placeholder for a proper WebSocket test
        let req = test::TestRequest::get().uri("/ws").to_request();
        let resp = test::call_service(&app, req).await;
        
        // Should return a switching protocols response for WebSocket
        assert!(resp.status().is_success() || resp.status().as_u16() == 101);
    }
}
