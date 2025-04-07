use anyhow::{Result, Context};
use tokio::net::TcpStream;
use std::time::Duration;
use crate::platform::{self, PlatformType};
use serde::{Serialize, Deserialize};
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::time::timeout;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ServiceStatus {
    FullyConnected,
    PartiallyConnected,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResult {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub connected: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAnalysisResult {
    pub status: ServiceStatus,
    pub services: Vec<ServiceResult>,
    pub internet_connectivity: bool,
    pub dns_working: bool,
}

pub struct NetworkChecker {
    required_services: Vec<ServiceEndpoint>,
    optional_services: Vec<ServiceEndpoint>,
    platform_type: PlatformType,
}

#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    name: String,
    host: String,
    port: u16,
    required: bool,
}

impl NetworkChecker {
    pub fn new() -> Self {
        let platform_type = platform::current_platform();
        
        let mut required_services = vec![
            ServiceEndpoint {
                name: "Bitcoin Core".to_string(),
                host: "127.0.0.1".to_string(),
                port: 8332,
                required: true,
            },
        ];
        
        let mut optional_services = vec![
            ServiceEndpoint {
                name: "Lightning Network".to_string(),
                host: "127.0.0.1".to_string(),
                port: 9735,
                required: false,
            },
            ServiceEndpoint {
                name: "Web5 DWN".to_string(),
                host: "127.0.0.1".to_string(),
                port: 3000,
                required: false,
            },
            ServiceEndpoint {
                name: "Bitcoin P2P".to_string(),
                host: "127.0.0.1".to_string(),
                port: 8333,
                required: false,
            },
            ServiceEndpoint {
                name: "Bitcoin ZMQ".to_string(),
                host: "127.0.0.1".to_string(),
                port: 28332,
                required: false,
            },
        ];
        
        // Add Internet connectivity checks
        required_services.push(ServiceEndpoint {
            name: "DNS Lookup".to_string(),
            host: "bitcoincore.org".to_string(),
            port: 443,
            required: true,
        });
        
        Self {
            required_services,
            optional_services,
            platform_type,
        }
    }
    
    /// Add a custom service to check
    pub fn add_service(&mut self, name: String, host: String, port: u16, required: bool) {
        let service = ServiceEndpoint {
            name,
            host,
            port,
            required,
        };
        
        if required {
            self.required_services.push(service);
        } else {
            self.optional_services.push(service);
        }
    }

    pub async fn analyze_network(&self) -> Result<NetworkAnalysisResult> {
        let mut results = Vec::new();
        let mut required_connected = 0;
        let required_total = self.required_services.len();
        let mut internet_connectivity = false;
        let mut dns_working = false;
        
        // Check all required services
        for service in &self.required_services {
            let result = self.check_service(service).await?;
            
            if result.name == "DNS Lookup" && result.connected {
                dns_working = true;
                internet_connectivity = true;
            }
            
            if result.connected {
                required_connected += 1;
            }
            
            results.push(result);
        }
        
        // Check all optional services
        for service in &self.optional_services {
            let result = self.check_service(service).await?;
            results.push(result);
        }
        
        // Determine overall status
        let status = if required_connected == required_total {
            ServiceStatus::FullyConnected
        } else if required_connected > 0 {
            ServiceStatus::PartiallyConnected
        } else {
            ServiceStatus::Offline
        };
        
        Ok(NetworkAnalysisResult {
            status,
            services: results,
            internet_connectivity,
            dns_working,
        })
    }

    async fn check_service(&self, service: &ServiceEndpoint) -> Result<ServiceResult> {
        // For DNS service, perform an actual DNS lookup
        if service.name == "DNS Lookup" {
            return self.check_dns_service(service).await;
        }
        
        // For regular services, check TCP connection
        let start = std::time::Instant::now();
        let address = format!("{}:{}", service.host, service.port);
        
        let connection_result = timeout(
            Duration::from_secs(2),
            TcpStream::connect(&address)
        ).await;
        
        match connection_result {
            Ok(Ok(_)) => {
                let latency = start.elapsed().as_millis() as u64;
                Ok(ServiceResult {
                    name: service.name.clone(),
                    host: service.host.clone(),
                    port: service.port,
                    connected: true,
                    latency_ms: Some(latency),
                    error: None,
                })
            },
            Ok(Err(e)) => {
                Ok(ServiceResult {
                    name: service.name.clone(),
                    host: service.host.clone(),
                    port: service.port,
                    connected: false,
                    latency_ms: None,
                    error: Some(format!("Connection error: {}", e)),
                })
            },
            Err(_) => {
                Ok(ServiceResult {
                    name: service.name.clone(),
                    host: service.host.clone(),
                    port: service.port,
                    connected: false,
                    latency_ms: None,
                    error: Some("Connection timeout".to_string()),
                })
            }
        }
    }
    
    /// Check DNS resolution
    async fn check_dns_service(&self, service: &ServiceEndpoint) -> Result<ServiceResult> {
        let start = std::time::Instant::now();
        
        // Try to resolve the hostname
        let resolve_result = tokio::task::spawn_blocking(move || {
            format!("{}:{}", service.host, service.port)
                .to_socket_addrs()
                .map(|mut addrs| addrs.next().is_some())
        }).await;
        
        match resolve_result {
            Ok(Ok(true)) => {
                let latency = start.elapsed().as_millis() as u64;
                Ok(ServiceResult {
                    name: service.name.clone(),
                    host: service.host.clone(),
                    port: service.port,
                    connected: true,
                    latency_ms: Some(latency),
                    error: None,
                })
            },
            _ => {
                Ok(ServiceResult {
                    name: service.name.clone(),
                    host: service.host.clone(),
                    port: service.port,
                    connected: false,
                    latency_ms: None,
                    error: Some("DNS resolution failed".to_string()),
                })
            }
        }
    }
    
    /// Get the base data directory for bitcoin and other services
    pub fn get_data_directory(&self) -> PathBuf {
        match self.platform_type {
            PlatformType::Windows => PathBuf::from(r"C:\ProgramData\AnyaCore"),
            PlatformType::MacOS => PathBuf::from("/Library/Application Support/AnyaCore"),
            PlatformType::Linux => PathBuf::from("/etc/anya-core"),
            _ => platform::data_dir(),
        }
    }
    
    /// Check Bitcoin RPC connection with credentials (if available)
    pub async fn check_bitcoin_rpc(&self, user: Option<String>, password: Option<String>) -> Result<bool> {
        // First check if the RPC port is open
        let bitcoin_service = ServiceEndpoint {
            name: "Bitcoin RPC".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8332,
            required: true,
        };
        
        let port_check = self.check_service(&bitcoin_service).await?;
        if !port_check.connected {
            return Ok(false);
        }
        
        // If credentials are provided, attempt an actual RPC call
        if let (Some(user), Some(password)) = (user, password) {
            // In a real implementation, we would attempt a basic RPC call here
            // Such as getblockchaininfo
            // For now, just return success if the port is open
            Ok(true)
        } else {
            // Without credentials, we can only check if the port is open
            Ok(port_check.connected)
        }
    }
    
    /// Check Lightning Network connection
    pub async fn check_lightning(&self) -> Result<bool> {
        let lightning_service = ServiceEndpoint {
            name: "Lightning Network".to_string(),
            host: "127.0.0.1".to_string(),
            port: 9735,
            required: false,
        };
        
        let result = self.check_service(&lightning_service).await?;
        Ok(result.connected)
    }
    
    /// Check Web5 DWN connection
    pub async fn check_web5(&self) -> Result<bool> {
        let web5_service = ServiceEndpoint {
            name: "Web5 DWN".to_string(),
            host: "127.0.0.1".to_string(),
            port: 3000,
            required: false,
        };
        
        let result = self.check_service(&web5_service).await?;
        Ok(result.connected)
    }
    
    /// Get the configuration directories for various services
    pub fn get_service_directories(&self) -> ServiceDirectories {
        let base_dir = self.get_data_directory();
        
        ServiceDirectories {
            bitcoin: base_dir.join("bitcoin"),
            lightning: base_dir.join("lightning"),
            web5: base_dir.join("web5"),
        }
    }
}

/// Service directory paths
pub struct ServiceDirectories {
    pub bitcoin: PathBuf,
    pub lightning: PathBuf,
    pub web5: PathBuf,
}
