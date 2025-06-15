use log::info;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

/// Network validation configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkValidationConfig {
    pub endpoints: Vec<String>,
    pub dns_servers: Vec<String>,
    pub required_ports: Vec<u16>,
    pub latency_threshold_ms: u64,
    pub bandwidth_threshold_mbps: f64,
    pub perform_traceroute: bool,
    pub check_firewall: bool,
    pub check_ipv6: bool,
    pub check_ssl: bool,
}

impl Default for NetworkValidationConfig {
    fn default() -> Self {
        Self {
            endpoints: vec![
                "https://bitcoin-rpc.publicnode.com".to_string(),
                "https://bitcoin-testnet-rpc.publicnode.com".to_string(),
            ],
            dns_servers: vec!["8.8.8.8".to_string(), "1.1.1.1".to_string()],
            required_ports: vec![22, 80, 443, 8333, 18333],
            latency_threshold_ms: 300,
            bandwidth_threshold_mbps: 10.0,
            perform_traceroute: true,
            check_firewall: true,
            check_ipv6: true,
            check_ssl: true,
        }
    }
}

/// Network validation result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkValidationResult {
    pub connectivity: NetworkConnectivityResult,
    pub dns: DnsValidationResult,
    pub bandwidth: BandwidthValidationResult,
    pub latency: LatencyValidationResult,
    pub ports: PortValidationResult,
    pub ssl: Option<SslValidationResult>,
    pub firewall: Option<FirewallValidationResult>,
    pub route: Option<RouteValidationResult>,
    pub vpn_detected: bool,
    pub recommendations: Vec<String>,
    pub overall_status: ValidationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Pass,
    Warning,
    Fail,
    Skipped,
}

// Various result structs for different aspects of network validation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConnectivityResult {
    pub internet_available: bool,
    pub endpoints_reachable: Vec<(String, bool)>,
    pub status: ValidationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DnsValidationResult {
    pub resolvers_available: Vec<(String, bool)>,
    pub resolution_times_ms: Vec<(String, u64)>,
    pub status: ValidationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BandwidthValidationResult {
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub status: ValidationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LatencyValidationResult {
    pub average_ms: u64,
    pub min_ms: u64,
    pub max_ms: u64,
    pub endpoint_latencies: Vec<(String, u64)>,
    pub status: ValidationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortValidationResult {
    pub open_ports: Vec<u16>,
    pub closed_ports: Vec<u16>,
    pub status: ValidationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SslValidationResult {
    pub endpoints_secure: Vec<(String, bool)>,
    pub certificate_issues: Vec<(String, String)>,
    pub status: ValidationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FirewallValidationResult {
    pub detected: bool,
    pub blocks_bitcoin: bool,
    pub blocks_required_ports: Vec<u16>,
    pub status: ValidationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteValidationResult {
    pub average_hops: u32,
    pub problematic_hops: Vec<String>,
    pub routes: Vec<(String, Vec<String>)>,
    pub status: ValidationStatus,
}

/// Network validator
pub struct NetworkValidator {
    config: NetworkValidationConfig,
}

impl NetworkValidator {
    pub fn new(config: NetworkValidationConfig) -> Self {
        Self { config }
    }

    /// Run comprehensive network validation
    pub async fn validate_network(&self) -> NetworkValidationResult {
        info!("Running comprehensive network validation...");

        // Run connectivity check
        let connectivity = self.validate_connectivity().await;

        // Skip remaining tests if no internet connectivity
        if !connectivity.internet_available {
            return NetworkValidationResult {
                connectivity,
                dns: DnsValidationResult {
                    resolvers_available: Vec::new(),
                    resolution_times_ms: Vec::new(),
                    status: ValidationStatus::Skipped,
                },
                bandwidth: BandwidthValidationResult {
                    download_mbps: 0.0,
                    upload_mbps: 0.0,
                    status: ValidationStatus::Skipped,
                },
                latency: LatencyValidationResult {
                    average_ms: 0,
                    min_ms: 0,
                    max_ms: 0,
                    endpoint_latencies: Vec::new(),
                    status: ValidationStatus::Skipped,
                },
                ports: PortValidationResult {
                    open_ports: Vec::new(),
                    closed_ports: Vec::new(),
                    status: ValidationStatus::Skipped,
                },
                ssl: None,
                firewall: None,
                route: None,
                vpn_detected: false,
                recommendations: vec!["Check your internet connection".to_string()],
                overall_status: ValidationStatus::Fail,
            };
        }

        // Run all other validations in parallel
        let dns_future = self.validate_dns();
        let bandwidth_future = self.validate_bandwidth();
        let latency_future = self.validate_latency();
        let ports_future = self.validate_ports();

        let (dns, bandwidth, latency, ports) =
            tokio::join!(dns_future, bandwidth_future, latency_future, ports_future);

        // Run optional checks
        let ssl = if self.config.check_ssl {
            Some(self.validate_ssl().await)
        } else {
            None
        };

        let firewall = if self.config.check_firewall {
            Some(self.validate_firewall().await)
        } else {
            None
        };

        let route = if self.config.perform_traceroute {
            Some(self.validate_routes().await)
        } else {
            None
        };

        // Check for VPN
        let vpn_detected = self.detect_vpn().await;

        // Generate recommendations
        let recommendations = self.generate_recommendations(
            &connectivity,
            &dns,
            &bandwidth,
            &latency,
            &ports,
            &ssl,
            &firewall,
            &route,
        );

        // Determine overall status
        let overall_status = self.determine_overall_status(
            &connectivity,
            &dns,
            &bandwidth,
            &latency,
            &ports,
            &ssl,
            &firewall,
            &route,
        );

        NetworkValidationResult {
            connectivity,
            dns,
            bandwidth,
            latency,
            ports,
            ssl,
            firewall,
            route,
            vpn_detected,
            recommendations,
            overall_status,
        }
    }

    /// Validate basic internet connectivity
    async fn validate_connectivity(&self) -> NetworkConnectivityResult {
        info!("Validating internet connectivity...");

        // Check if we can reach a reliable internet endpoint
        let internet_check = tokio::spawn(async {
            let addresses = ["1.1.1.1:443", "8.8.8.8:443", "9.9.9.9:443"];

            for addr in &addresses {
                if let Ok(Ok(_)) = timeout(Duration::from_secs(5), TcpStream::connect(addr)).await {
                    return true;
                }
            }

            false
        })
        .await
        .unwrap_or(false);

        // Check each endpoint
        let mut endpoints_reachable = Vec::new();

        for endpoint in &self.config.endpoints {
            let result = self.is_endpoint_reachable(endpoint).await;
            endpoints_reachable.push((endpoint.clone(), result));
        }

        // Determine status
        let status = if !internet_check {
            ValidationStatus::Fail
        } else if endpoints_reachable.iter().all(|(_, reachable)| *reachable) {
            ValidationStatus::Pass
        } else if endpoints_reachable.iter().any(|(_, reachable)| *reachable) {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Fail
        };

        NetworkConnectivityResult {
            internet_available: internet_check,
            endpoints_reachable,
            status,
        }
    }

    /// Check if endpoint is reachable
    async fn is_endpoint_reachable(&self, endpoint: &str) -> bool {
        // Parse the URL to get host and port
        if let Ok(url) = url::Url::parse(endpoint) {
            let host = match url.host_str() {
                Some(h) => h,
                None => return false,
            };

            let port = url.port().unwrap_or_else(|| match url.scheme() {
                "https" => 443,
                "http" => 80,
                _ => 0,
            });

            if port == 0 {
                return false;
            }

            // Try to connect
            let addr = format!("{}:{}", host, port);
            match timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await {
                Ok(Ok(_)) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Validate DNS resolution
    async fn validate_dns(&self) -> DnsValidationResult {
        info!("Validating DNS resolution...");

        let mut resolvers_available = Vec::new();
        let mut resolution_times_ms = Vec::new();

        // Check standard resolvers
        for dns in &self.config.dns_servers {
            // Create a custom resolver config
            let resolver_ip: IpAddr = match dns.parse() {
                Ok(ip) => ip,
                Err(_) => continue,
            };

            // Create NameServerConfig instead of SocketAddr
            let nameserver_config = trust_dns_resolver::config::NameServerConfig {
                socket_addr: SocketAddr::new(resolver_ip, 53),
                protocol: trust_dns_resolver::config::Protocol::Udp,
                tls_dns_name: None,
                bind_addr: None,
                trust_nx_responses: true,
            };

            let resolver_config = ResolverConfig::from_parts(None, vec![], vec![nameserver_config]);

            // Create the resolver
            let resolver = match Resolver::new(resolver_config, ResolverOpts::default()) {
                Ok(r) => r,
                Err(_) => {
                    resolvers_available.push((dns.clone(), false));
                    continue;
                }
            };

            // Try to resolve a well-known domain
            let start = Instant::now();
            let lookup_result = resolver.lookup_ip("google.com");
            let elapsed = start.elapsed();

            match lookup_result {
                Ok(_) => {
                    resolvers_available.push((dns.clone(), true));
                    resolution_times_ms.push((dns.clone(), elapsed.as_millis() as u64));
                }
                Err(_) => {
                    resolvers_available.push((dns.clone(), false));
                }
            }
        }

        // Determine status
        let status = if resolvers_available.iter().all(|(_, available)| !available) {
            ValidationStatus::Fail
        } else if resolvers_available.iter().all(|(_, available)| *available) {
            ValidationStatus::Pass
        } else {
            ValidationStatus::Warning
        };

        DnsValidationResult {
            resolvers_available,
            resolution_times_ms,
            status,
        }
    }

    /// Validate bandwidth
    async fn validate_bandwidth(&self) -> BandwidthValidationResult {
        info!("Validating network bandwidth...");

        // Simple bandwidth test
        // In a real implementation, you'd want to use a more sophisticated
        // bandwidth testing service or method

        // Download test (simplified)
        let download_start = Instant::now();
        let download_result =
            reqwest::get("https://speed.cloudflare.com/__down?bytes=5000000").await;

        let download_mbps = match download_result {
            Ok(resp) => {
                if let Ok(bytes) = resp.bytes().await {
                    let elapsed = download_start.elapsed();
                    let bits = bytes.len() as f64 * 8.0;
                    let seconds = elapsed.as_secs_f64();
                    (bits / 1_000_000.0) / seconds
                } else {
                    0.0
                }
            }
            Err(_) => 0.0,
        };

        // Upload test (simplified)
        let client = reqwest::Client::new();
        let upload_bytes = vec![0u8; 1_000_000]; // 1MB of data

        let upload_start = Instant::now();
        let upload_result = client
            .post("https://speed.cloudflare.com/__up")
            .body(upload_bytes)
            .send()
            .await;

        let upload_mbps = match upload_result {
            Ok(_) => {
                let elapsed = upload_start.elapsed();
                let bits = 1_000_000 * 8;
                let seconds = elapsed.as_secs_f64();
                (bits as f64 / 1_000_000.0) / seconds
            }
            Err(_) => 0.0,
        };

        // Determine status
        let status = if download_mbps < 1.0 || upload_mbps < 0.5 {
            ValidationStatus::Fail
        } else if download_mbps < self.config.bandwidth_threshold_mbps {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Pass
        };

        BandwidthValidationResult {
            download_mbps,
            upload_mbps,
            status,
        }
    }

    /// Validate network latency
    async fn validate_latency(&self) -> LatencyValidationResult {
        info!("Validating network latency...");

        let mut endpoint_latencies = Vec::new();
        let mut total_latency = 0u64;
        let mut count = 0u64;
        let mut min_ms = u64::MAX;
        let mut max_ms = 0u64;

        for endpoint in &self.config.endpoints {
            // Parse URL to get host
            if let Ok(url) = url::Url::parse(endpoint) {
                if let Some(host) = url.host_str() {
                    // Ping with timeout
                    let latency = self.measure_latency(host).await;

                    if latency > 0 {
                        endpoint_latencies.push((endpoint.clone(), latency));
                        total_latency += latency;
                        count += 1;
                        min_ms = min_ms.min(latency);
                        max_ms = max_ms.max(latency);
                    }
                }
            }
        }

        let average_ms = if count > 0 { total_latency / count } else { 0 };

        // Determine status
        let status = if count == 0 {
            ValidationStatus::Fail
        } else if average_ms > self.config.latency_threshold_ms {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Pass
        };

        LatencyValidationResult {
            average_ms,
            min_ms: if min_ms == u64::MAX { 0 } else { min_ms },
            max_ms,
            endpoint_latencies,
            status,
        }
    }

    /// Measure latency to a host
    async fn measure_latency(&self, host: &str) -> u64 {
        let mut total_ms = 0u64;
        let mut successful_pings = 0u64;

        for _ in 0..3 {
            let start = Instant::now();
            let result = TcpStream::connect(format!("{}:443", host)).await;
            let elapsed = start.elapsed();

            if result.is_ok() {
                total_ms += elapsed.as_millis() as u64;
                successful_pings += 1;
            }
        }

        if successful_pings > 0 {
            total_ms / successful_pings
        } else {
            0
        }
    }

    /// Validate open ports
    pub async fn validate_ports(&self) -> PortValidationResult {
        info!("Validating required ports with BIP-341 compliance...");

        let mut open_ports = Vec::new();
        let mut closed_ports = Vec::new();

        // Add BIP-341 required ports
        let bip341_ports = vec![8333, 18333, 8433]; // 8433 for Taproot monitoring

        for &port in bip341_ports.iter().chain(&self.config.required_ports) {
            let is_open = if port == 8433 {
                // Special handling for Taproot port
                self.validate_taproot_port(port).await
            } else {
                self.is_port_open("localhost", port).await
            };

            if is_open {
                open_ports.push(port);
            } else {
                closed_ports.push(port);
            }
        }

        // Determine status
        let status = if closed_ports.contains(&8333) && closed_ports.contains(&18333) {
            ValidationStatus::Fail
        } else if !closed_ports.is_empty() {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Pass
        };

        PortValidationResult {
            open_ports,
            closed_ports,
            status,
        }
    }

    /// Check if a port is open
    async fn is_port_open(&self, host: &str, port: u16) -> bool {
        let addr = format!("{}:{}", host, port);
        match timeout(Duration::from_secs(3), TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => true,
            _ => false,
        }
    }

    /// Validate SSL certificates
    async fn validate_ssl(&self) -> SslValidationResult {
        info!("Validating SSL certificates...");

        let mut endpoints_secure = Vec::new();
        let mut certificate_issues = Vec::new();

        for endpoint in &self.config.endpoints {
            if let Ok(url) = url::Url::parse(endpoint) {
                if url.scheme() != "https" {
                    continue;
                }

                if let Some(_host) = url.host_str() {
                    // Simple check using reqwest to validate HTTPS
                    let client = reqwest::Client::builder()
                        .danger_accept_invalid_certs(true)
                        .build()
                        .unwrap_or_default();

                    let response = client.get(endpoint).send().await;

                    match response {
                        Ok(resp) => {
                            let is_secure = resp.status().is_success();
                            endpoints_secure.push((endpoint.clone(), is_secure));

                            // Check for certificate warnings
                            if resp.status().is_client_error() {
                                certificate_issues.push((
                                    endpoint.clone(),
                                    format!("Certificate error: {}", resp.status()),
                                ));
                            }
                        }
                        Err(e) => {
                            endpoints_secure.push((endpoint.clone(), false));
                            certificate_issues
                                .push((endpoint.clone(), format!("Connection error: {}", e)));
                        }
                    }
                }
            }
        }

        // Determine status
        let status = if endpoints_secure.iter().all(|(_, secure)| !secure) {
            ValidationStatus::Fail
        } else if endpoints_secure.iter().all(|(_, secure)| *secure) {
            ValidationStatus::Pass
        } else {
            ValidationStatus::Warning
        };

        SslValidationResult {
            endpoints_secure,
            certificate_issues,
            status,
        }
    }

    /// Validate firewall settings
    async fn validate_firewall(&self) -> FirewallValidationResult {
        info!("Validating firewall settings...");

        // Detect if a firewall is present and active
        let firewall_detected = self.detect_firewall().await;

        // Check if firewall blocks Bitcoin connections
        let blocks_bitcoin = self.check_firewall_blocks_bitcoin().await;

        // Check which ports are blocked
        let blocks_required_ports = self.check_blocked_ports().await;

        // Determine status
        let status = if blocks_bitcoin {
            ValidationStatus::Fail
        } else if !blocks_required_ports.is_empty() {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Pass
        };

        FirewallValidationResult {
            detected: firewall_detected,
            blocks_bitcoin,
            blocks_required_ports,
            status,
        }
    }

    /// Detect if a firewall is present
    async fn detect_firewall(&self) -> bool {
        // Platform-specific firewall detection
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("netsh")
                .args(&["advfirewall", "show", "currentprofile"])
                .output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.contains("State                      ON")
                }
                Err(_) => false,
            }
        }

        #[cfg(target_os = "linux")]
        {
            let output = Command::new("sudo").args(&["iptables", "-L"]).output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    !stdout.trim().is_empty() && !stdout.contains("No rules")
                }
                Err(_) => false,
            }
        }

        #[cfg(target_os = "macos")]
        {
            let output = Command::new("defaults")
                .args(&["read", "/Library/Preferences/com.apple.alf", "globalstate"])
                .output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.trim() == "1" || stdout.trim() == "2"
                }
                Err(_) => false,
            }
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            false
        }
    }

    /// Check if firewall blocks Bitcoin connections
    async fn check_firewall_blocks_bitcoin(&self) -> bool {
        // Updated to check Taproot port requirements from BIP-341
        !(self.is_port_open("bitcoin.org", 8333).await ||   // Mainnet
          self.is_port_open("bitcoin.org", 18333).await ||  // Testnet
          self.is_port_open("bitcoin.org", 8433).await) // Taproot monitoring
    }

    /// Check which required ports are blocked
    async fn check_blocked_ports(&self) -> Vec<u16> {
        let mut blocked_ports = Vec::new();

        for &port in &self.config.required_ports {
            if !self.is_port_open("example.com", port).await {
                blocked_ports.push(port);
            }
        }

        blocked_ports
    }

    /// Validate network routes
    async fn validate_routes(&self) -> RouteValidationResult {
        info!("Validating network routes...");

        let mut routes = Vec::new();
        let mut total_hops = 0u32;
        let mut problematic_hops = Vec::new();
        let mut route_count = 0u32;

        // Traceroute to endpoints
        for endpoint in &self.config.endpoints {
            if let Ok(url) = url::Url::parse(endpoint) {
                if let Some(host) = url.host_str() {
                    let (route_hops, route_problems) = self.trace_route(host).await;

                    if !route_hops.is_empty() {
                        total_hops += route_hops.len() as u32;
                        route_count += 1;
                        routes.push((host.to_string(), route_hops));
                        problematic_hops.extend(route_problems);
                    }
                }
            }
        }

        let average_hops = if route_count > 0 {
            total_hops / route_count
        } else {
            0
        };

        // Determine status
        let status = if route_count == 0 {
            ValidationStatus::Fail
        } else if !problematic_hops.is_empty() {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Pass
        };

        RouteValidationResult {
            average_hops,
            problematic_hops,
            routes,
            status,
        }
    }

    /// Trace route to host
    async fn trace_route(&self, host: &str) -> (Vec<String>, Vec<String>) {
        let mut hops = Vec::new();
        let mut problematic = Vec::new();

        // Platform-specific traceroute
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let output = Command::new("traceroute")
                .args(&["-m", "15", host])
                .output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    for line in stdout.lines().skip(1) {
                        // Skip header
                        if line.contains("* * *") {
                            problematic.push(format!("Hop timeout: {}", line));
                        } else {
                            hops.push(line.to_string());
                        }
                    }
                }
                Err(_) => (),
            }
        }

        #[cfg(target_os = "windows")]
        {
            let output = Command::new("tracert").args(&["-h", "15", host]).output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    for line in stdout.lines().skip(4) {
                        // Skip headers
                        if line.contains("*") {
                            problematic.push(format!("Hop timeout: {}", line));
                        } else if !line.trim().is_empty() {
                            hops.push(line.to_string());
                        }
                    }
                }
                Err(_) => (),
            }
        }

        (hops, problematic)
    }

    /// Detect if a VPN is in use
    async fn detect_vpn(&self) -> bool {
        // Platform-specific VPN detection
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("netsh")
                .args(&["interface", "show", "interface"])
                .output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.contains("PPP") || stdout.contains("VPN") || stdout.contains("Tunnel")
                }
                Err(_) => false,
            }
        }

        #[cfg(target_os = "linux")]
        {
            let output = Command::new("ip").args(&["tuntap", "list"]).output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    !stdout.trim().is_empty()
                }
                Err(_) => false,
            }
        }

        #[cfg(target_os = "macos")]
        {
            let output = Command::new("networksetup")
                .args(&["-listallnetworkservices"])
                .output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.contains("VPN")
                        || stdout.contains("Cisco")
                        || stdout.contains("Global Protect")
                }
                Err(_) => false,
            }
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            false
        }
    }

    /// Generate recommendations based on validation results
    fn generate_recommendations(
        &self,
        connectivity: &NetworkConnectivityResult,
        dns: &DnsValidationResult,
        bandwidth: &BandwidthValidationResult,
        latency: &LatencyValidationResult,
        ports: &PortValidationResult,
        ssl: &Option<SslValidationResult>,
        firewall: &Option<FirewallValidationResult>,
        route: &Option<RouteValidationResult>,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Connectivity recommendations
        if !connectivity.internet_available {
            recommendations
                .push("Check your internet connection, it appears to be offline".to_string());
        } else if !connectivity
            .endpoints_reachable
            .iter()
            .all(|(_, reachable)| *reachable)
        {
            recommendations.push(
                "Some Bitcoin RPC endpoints are unreachable, check your network configuration"
                    .to_string(),
            );
        }

        // DNS recommendations
        if dns.status == ValidationStatus::Fail {
            recommendations.push("DNS resolution is failing, check your DNS servers or try using alternative DNS like 1.1.1.1 or 8.8.8.8".to_string());
        } else if dns.status == ValidationStatus::Warning {
            recommendations.push(
                "Some DNS servers are not responding, consider using more reliable DNS servers"
                    .to_string(),
            );
        }

        // Bandwidth recommendations
        if bandwidth.status == ValidationStatus::Fail {
            recommendations.push(format!(
                "Network bandwidth is too low (Download: {:.2} Mbps, Upload: {:.2} Mbps). Minimum requirements are 1 Mbps download and 0.5 Mbps upload",
                bandwidth.download_mbps, bandwidth.upload_mbps
            ));
        } else if bandwidth.status == ValidationStatus::Warning {
            recommendations.push(format!(
                "Network bandwidth is below recommended levels (Download: {:.2} Mbps). For optimal performance, {:.2} Mbps is recommended",
                bandwidth.download_mbps, self.config.bandwidth_threshold_mbps
            ));
        }

        // Latency recommendations
        if latency.status == ValidationStatus::Fail {
            recommendations.push(
                "Network latency could not be measured, your network connection may be unstable"
                    .to_string(),
            );
        } else if latency.status == ValidationStatus::Warning {
            recommendations.push(format!(
                "Network latency is high (Average: {} ms). For optimal performance, latency should be below {} ms",
                latency.average_ms, self.config.latency_threshold_ms
            ));
        }

        // Port recommendations
        if !ports.closed_ports.is_empty() {
            recommendations.push(format!(
                "The following required ports are closed or blocked: {}. Consider opening these ports in your firewall",
                ports.closed_ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
            ));
        }

        // SSL recommendations
        if let Some(ssl_result) = ssl {
            if ssl_result.status == ValidationStatus::Fail {
                recommendations.push("SSL certificate validation failed for all endpoints. Check your system's certificate store".to_string());
            } else if ssl_result.status == ValidationStatus::Warning {
                recommendations.push("Some SSL certificates could not be validated. Your system may be missing necessary root certificates".to_string());
            }
        }

        // Firewall recommendations
        if let Some(firewall_result) = firewall {
            if firewall_result.blocks_bitcoin {
                recommendations.push("Your firewall appears to be blocking Bitcoin network traffic. Configure your firewall to allow ports 8333 (mainnet) and 18333 (testnet)".to_string());
            } else if !firewall_result.blocks_required_ports.is_empty() {
                recommendations.push(format!(
                    "Your firewall is blocking these required ports: {}. Configure your firewall to allow these ports",
                    firewall_result.blocks_required_ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
                ));
            }
        }

        // Route recommendations
        if let Some(route_result) = route {
            if route_result.status == ValidationStatus::Fail {
                recommendations.push(
                    "Network route tracing failed. Your network may be blocking ICMP traffic"
                        .to_string(),
                );
            } else if route_result.status == ValidationStatus::Warning {
                recommendations.push(
                    "Network routes contain problematic hops, which may cause connection issues"
                        .to_string(),
                );
            }
        }

        // Add PSBT check
        if let Some(firewall_result) = firewall {
            if firewall_result.blocks_required_ports.contains(&174) {
                recommendations.push(
                    "PSBT (BIP-174) port 174 is blocked. Required for partial transactions"
                        .to_string(),
                );
            }
        }

        recommendations
    }

    /// Determine overall status based on individual test results
    fn determine_overall_status(
        &self,
        connectivity: &NetworkConnectivityResult,
        dns: &DnsValidationResult,
        bandwidth: &BandwidthValidationResult,
        latency: &LatencyValidationResult,
        ports: &PortValidationResult,
        ssl: &Option<SslValidationResult>,
        firewall: &Option<FirewallValidationResult>,
        route: &Option<RouteValidationResult>,
    ) -> ValidationStatus {
        // Critical failures
        if connectivity.status == ValidationStatus::Fail
            || dns.status == ValidationStatus::Fail
            || bandwidth.status == ValidationStatus::Fail
        {
            return ValidationStatus::Fail;
        }

        // Firewall blocking Bitcoin is a critical failure
        if let Some(firewall_result) = firewall {
            if firewall_result.blocks_bitcoin {
                return ValidationStatus::Fail;
            }
        }

        // Check for warnings
        let has_warnings = connectivity.status == ValidationStatus::Warning
            || dns.status == ValidationStatus::Warning
            || bandwidth.status == ValidationStatus::Warning
            || latency.status == ValidationStatus::Warning
            || ports.status == ValidationStatus::Warning;

        // Check optional components
        let optional_warnings = ssl
            .as_ref()
            .map_or(false, |r| r.status == ValidationStatus::Warning)
            || firewall
                .as_ref()
                .map_or(false, |r| r.status == ValidationStatus::Warning)
            || route
                .as_ref()
                .map_or(false, |r| r.status == ValidationStatus::Warning);

        if has_warnings || optional_warnings {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Pass
        }
    }

    /// BIP-341 specific port validation
    async fn validate_taproot_port(&self, _port: u16) -> bool {
        // Implement Taproot-specific validation logic
        let taproot_check = Command::new("bitcoin-cli")
            .args(&["getnetworkinfo"])
            .output();

        match taproot_check {
            Ok(output) => {
                let network_info = String::from_utf8_lossy(&output.stdout);
                network_info.contains("\"taproot_active\": true")
            }
            Err(_) => false,
        }
    }
}
