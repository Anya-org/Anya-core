#![feature(edition2021)]
use anyhow::Result;
use tokio::net::TcpStream;
use std::time::Duration;

#[derive(Debug)]
pub enum ServiceStatus {
    FullyConnected,
    PartiallyConnected,
    Offline,
}

pub struct NetworkChecker {
    required_services: Vec<ServiceEndpoint>,
}

struct ServiceEndpoint {
    name: String,
    host: String,
    port: u16,
}

impl NetworkChecker {
    pub fn new() -> Self {
        Self {
            required_services: vec![
                ServiceEndpoint {
                    name: "Bitcoin Core".to_string(),
                    host: "127.0.0.1".to_string(),
                    port: 8332,
                },
                ServiceEndpoint {
                    name: "Lightning".to_string(),
                    host: "127.0.0.1".to_string(),
                    port: 9735,
                },
                ServiceEndpoint {
                    name: "Web5".to_string(),
                    host: "127.0.0.1".to_string(),
                    port: 3000,
                },
            ],
        }
    }

    pub async fn analyze_network(&self) -> Result<ServiceStatus> {
        let mut connected_count = 0;

        for service in &self.required_services {
            if self.check_service(service).await? {
                connected_count += 1;
            }
        }

        Ok(match connected_count {
            0 => ServiceStatus::Offline,
            n if n == self.required_services.len() => ServiceStatus::FullyConnected,
            _ => ServiceStatus::PartiallyConnected,
        })
    }

    async fn check_service(&self, service: &ServiceEndpoint) -> Result<bool> {
        match TcpStream::connect((service.host.as_str(), service.port))
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
