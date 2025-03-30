use axum::{
    routing::{get, post},
    Router,
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use futures::stream::{self, Stream};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::broadcast;
use super::{McpError, McpTransport, McpRequest, McpResponse, McpNotification};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub uptime_seconds: u64,
    pub active_connections: usize,
    pub last_check_timestamp: u64,
    pub port: u16,
    pub host: String,
}

struct AppState {
    health_status: Arc<RwLock<HealthStatus>>,
    active_connections: Arc<RwLock<usize>>,
    tx: broadcast::Sender<String>,
}

pub struct HttpTransport {
    #[allow(dead_code)]
    tx: broadcast::Sender<String>,
    port: u16,
    host: String,
    #[allow(dead_code)]
    backlog: u32,
    health_status: Arc<RwLock<HealthStatus>>,
    #[allow(dead_code)]
    clients: Arc<RwLock<HashMap<String, tokio::sync::mpsc::Sender<String>>>>,
    active_connections: Arc<RwLock<usize>>,
}

impl HttpTransport {
    pub fn new(port: u16) -> Self {
        let (tx, _) = broadcast::channel(100);
        let start_time = Instant::now();
        
        let health_status = Arc::new(RwLock::new(HealthStatus {
            status: "initialized".to_string(),
            uptime_seconds: 0,
            active_connections: 0,
            last_check_timestamp: 0,
            port,
            host: "0.0.0.0".to_string(),
        }));
        
        // Start a background task to update the uptime
        let health_status_clone = health_status.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let elapsed = start_time.elapsed().as_secs();
                let mut health = health_status_clone.write().await;
                health.uptime_seconds = elapsed;
                health.last_check_timestamp = elapsed;
            }
        });
        
        Self { 
            tx, 
            port, 
            host: "0.0.0.0".to_string(),
            backlog: 1024,
            health_status,
            clients: Arc::new(RwLock::new(HashMap::new())),
            active_connections: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn start_server(self) -> Result<(), McpError> {
        // Update health status to starting
        {
            let mut health = self.health_status.write().await;
            health.status = "starting".to_string();
        }
        
        info!("===== STARTING MCP SERVER (HTTP TRANSPORT) =====");
        
        // Create the app state with health and connection tracking
        let app_state = Arc::new(AppState {
            tx: self.tx.clone(),
            health_status: self.health_status.clone(),
            active_connections: self.active_connections.clone(),
        });

        let app = Router::new()
            .route("/request", post(handle_request))
            .route("/events", get(handle_events))
            .route("/health", get(health_check))
            .route("/health/readiness", get(readiness_check))
            .route("/health/liveness", get(liveness_check))
            .route("/metrics", get(metrics_handler))
            .with_state(app_state)
            .layer(TraceLayer::new_for_http());

        // Use socket2 for more reliable socket binding with options
        let addr = format!("{}:{}", self.host, self.port);
        let addr = addr.parse::<SocketAddr>().map_err(|e| {
            error!("Failed to parse socket address: {}", e);
            McpError::TransportError(format!("Invalid address: {}", e))
        })?;
        
        // Create the socket with socket2
        let socket = Socket::new(
            Domain::for_address(addr),
            Type::STREAM,
            None
        ).map_err(|e| {
            error!("Failed to create socket: {}", e);
            McpError::TransportError(format!("Socket creation error: {}", e))
        })?;
        
        // Set socket options
        socket.set_reuse_address(true).map_err(|e| {
            error!("Failed to set reuse_address: {}", e);
            McpError::TransportError(format!("Socket option error: {}", e))
        })?;
        
        // Bind the socket
        socket.bind(&addr.into()).map_err(|e| {
            error!("Failed to bind socket to {}: {}", addr, e);
            McpError::TransportError(format!("Socket bind error: {}", e))
        })?;
        
        // Listen on the socket
        socket.listen(self.backlog.try_into().unwrap()).map_err(|e| {
            error!("Failed to listen on socket: {}", e);
            McpError::TransportError(format!("Socket listen error: {}", e))
        })?;
        
        // Get the std socket from socket2
        let std_socket = socket.into_tcp_listener();
        let local_addr = std_socket.local_addr().map_err(|e| {
            error!("Failed to get local address: {}", e);
            McpError::TransportError(format!("Socket error: {}", e))
        })?;
        
        info!("MCP server successfully bound to {}", local_addr);
        
        // Start the server
        let server = axum::Server::from_tcp(std_socket).map_err(|e| {
            error!("Failed to create server from socket: {}", e);
            McpError::TransportError(format!("Server error: {}", e))
        })?;
        
        // Update health status to running
        {
            let mut health = self.health_status.write().await;
            health.status = "running".to_string();
        }
        
        info!("MCP server is now running. Health status updated.");
        
        // Create a shutdown channel
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let server_handle = tokio::spawn(async move {
            server.serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .with_graceful_shutdown(async {
                    let _ = rx.await;
                    info!("Graceful shutdown signal received.");
                })
                .await
                .map_err(|e| McpError::TransportError(format!("Server error: {}", e)))
        });
        
        // Handle shutdown signal
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                info!("Shutdown signal received, stopping server...");
                let _ = tx.send(());
            }
            Err(err) => {
                error!("Failed to listen for shutdown signal: {}", err);
            }
        }
        
        // Wait for server to complete
        match server_handle.await {
            Ok(result) => {
                info!("Server task completed.");
                result
            }
            Err(e) => {
                error!("Server task panicked: {}", e);
                Err(McpError::TransportError(format!("Server task error: {}", e)))
            }
        }
    }
}



async fn handle_request(
    State(state): State<Arc<AppState>>,
    Json(request): Json<McpRequest>,
) -> Json<McpResponse> {
    // Process the request and generate response
    let response = McpResponse {
        result: Some(serde_json::json!({ "status": "success" })),
        error: None,
    };

    // Broadcast notification to SSE clients if needed
    let _ = state.tx.send(serde_json::to_string(&request).unwrap());

    Json(response)
}

async fn handle_events(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();

    let stream = stream::unfold(rx, |mut rx| async move {
        let msg = rx.recv().await.ok()?;
        let event = Event::default().data(msg);
        Some((Ok(event), rx))
    });
    
    // Increment active connections counter
    let mut connections = state.active_connections.write().await;
    *connections += 1;

    Sse::new(stream)
}

// Health check endpoint handler
async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Json<HealthStatus> {
    Json(state.health_status.read().await.clone())
}

// Readiness check endpoint handler
async fn readiness_check(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let health = state.health_status.read().await;
    
    if health.status == "running" {
        (StatusCode::OK, Json(serde_json::json!({ "status": "ready" })))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({ 
            "status": "not_ready",
            "reason": health.status.clone()
        })))
    }
}

// Liveness check endpoint handler
async fn liveness_check() -> (StatusCode, Json<serde_json::Value>) {
    // Basic liveness check - if this handler executes, the server is alive
    (StatusCode::OK, Json(serde_json::json!({ "status": "alive" })))
}

// Metrics endpoint handler for enhanced monitoring
async fn metrics_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let health = state.health_status.read().await;
    let active = *state.active_connections.read().await;
    
    let metrics = serde_json::json!({
        "status": health.status,
        "uptime_seconds": health.uptime_seconds,
        "active_connections": active,
        "memory_usage_mb": get_memory_usage(),
        "system_load": get_system_load(),
        "port": health.port,
        "host": health.host,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    (StatusCode::OK, Json(metrics))
}

// Helper function to estimate memory usage (simplified)
fn get_memory_usage() -> f64 {
    // In a real implementation, this would use platform-specific APIs
    // For demo purposes, returning a placeholder value
    let usage = std::process::Command::new("ps")
        .args(["-o", "rss=", &format!("{}", std::process::id())])
        .output();
    
    match usage {
        Ok(output) => {
            let s = String::from_utf8_lossy(&output.stdout);
            let rss = s.trim().parse::<f64>().unwrap_or(0.0);
            rss / 1024.0 // Convert KB to MB
        }
        Err(_) => 0.0,
    }
}

// Helper function to get system load
fn get_system_load() -> Vec<f64> {
    #[cfg(target_os = "linux")]
    {
        let load = std::fs::read_to_string("/proc/loadavg");
        match load {
            Ok(contents) => {
                let parts: Vec<&str> = contents.split_whitespace().collect();
                let mut result = Vec::new();
                for i in 0..3 {
                    if i < parts.len() {
                        if let Ok(val) = parts[i].parse::<f64>() {
                            result.push(val);
                        }
                    }
                }
                if !result.is_empty() {
                    return result;
                }
            }
            Err(_) => {}
        }
    }
    
    // Fallback for non-Linux systems or if reading failed
    vec![0.0, 0.0, 0.0]
}

#[async_trait::async_trait]
impl McpTransport for HttpTransport {
    async fn send_request(&self, request: McpRequest) -> Result<McpResponse, McpError> {
        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:8080/request")
            .json(&request)
            .send()
            .await
            .map_err(|e| McpError::TransportError(format!("HTTP request failed: {}", e)))?;

        let response: McpResponse = response
            .json()
            .await
            .map_err(|e| McpError::ProtocolError(format!("Failed to parse response: {}", e)))?;

        Ok(response)
    }

    async fn send_notification(&self, notification: McpNotification) -> Result<(), McpError> {
        let notification_json = serde_json::to_string(&notification)
            .map_err(|e| McpError::ProtocolError(format!("Failed to serialize notification: {}", e)))?;

        self.tx
            .send(notification_json)
            .map_err(|e| McpError::TransportError(format!("Failed to broadcast notification: {}", e)))?;

        Ok(())
    }
}
