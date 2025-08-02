use hickory_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use hickory_resolver::Resolver;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::net::{IpAddr, SocketAddr};
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::info;

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

/// Group validation results into a single struct to avoid too many arguments
pub(crate) struct ValidationResults<'a> {
    pub connectivity: &'a NetworkConnectivityResult,
    pub dns: &'a DnsValidationResult,
    pub bandwidth: &'a BandwidthValidationResult,
    pub latency: &'a LatencyValidationResult,
    pub ports: &'a PortValidationResult,
    pub ssl: &'a Option<SslValidationResult>,
    pub firewall: &'a Option<FirewallValidationResult>,
    pub route: &'a Option<RouteValidationResult>,
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

        let connectivity = self.validate_connectivity().await;

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

        // Run DNS, bandwidth, latency, and ports checks concurrently for better performance
        let (dns, bandwidth, latency, ports) = futures::join!(
            self.validate_dns(),
            self.validate_bandwidth(),
            self.validate_latency(),
            self.validate_ports()
        );

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

        let vpn_detected = self.detect_vpn().await;

        let results = ValidationResults {
            connectivity: &connectivity,
            dns: &dns,
            bandwidth: &bandwidth,
            latency: &latency,
            ports: &ports,
            ssl: &ssl,
            firewall: &firewall,
            route: &route,
        };

        let recommendations = self.generate_recommendations(&results);
        let overall_status = self.determine_overall_status(&results);

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

    async fn validate_connectivity(&self) -> NetworkConnectivityResult {
        info!("Validating internet connectivity...");

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

        let mut endpoints_reachable = Vec::new();
        for endpoint in &self.config.endpoints {
            let result = self.is_endpoint_reachable(endpoint).await;
            endpoints_reachable.push((endpoint.clone(), result));
        }

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

    async fn is_endpoint_reachable(&self, endpoint: &str) -> bool {
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
            let addr = format!("{host}:{port}");
            matches!(
                timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await,
                Ok(Ok(_))
            )
        } else {
            false
        }
    }

    async fn validate_dns(&self) -> DnsValidationResult {
        info!("Validating DNS resolution...");

        let mut resolvers_available = Vec::new();
        let mut resolution_times_ms = Vec::new();

        for dns in &self.config.dns_servers {
            let resolver_ip: IpAddr = match dns.parse() {
                Ok(ip) => ip,
                Err(_) => continue,
            };

            let nameserver_config = NameServerConfig {
                socket_addr: SocketAddr::new(resolver_ip, 53),
                protocol: Protocol::Udp,
                tls_dns_name: None,
                tls_config: None,
                bind_addr: None,
                trust_negative_responses: true,
            };

            let resolver_config = ResolverConfig::from_parts(None, vec![], vec![nameserver_config]);

            let resolver = match Resolver::new(resolver_config, ResolverOpts::default()) {
                Ok(r) => r,
                Err(_) => {
                    resolvers_available.push((dns.clone(), false));
                    continue;
                }
            };

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

    async fn validate_bandwidth(&self) -> BandwidthValidationResult {
        info!("Validating network bandwidth...");

        let download_start = Instant::now();
        // Use ureq for MIT-compliant HTTP requests
        let download_result = tokio::task::spawn_blocking(|| {
            match ureq::get("https://speed.cloudflare.com/__down?bytes=5000000").call() {
                Ok(resp) => {
                    let mut bytes = Vec::new();
                    match resp.into_reader().read_to_end(&mut bytes) {
                        Ok(size) => Ok(size),
                        Err(_) => Err("IO Error"),
                    }
                }
                Err(_) => Err("HTTP Error"),
            }
        })
        .await;

        let download_mbps = match download_result {
            Ok(Ok(_bytes_len)) => {
                let elapsed = download_start.elapsed();
                let bits = 5000000.0 * 8.0; // We know the expected size
                let seconds = elapsed.as_secs_f64();
                (bits / 1_000_000.0) / seconds
            }
            _ => 0.0,
        };

        // Upload test using ureq
        let upload_bytes = vec![0u8; 1_000_000];
        let upload_start = Instant::now();
        let upload_result = tokio::task::spawn_blocking(move || {
            ureq::post("https://speed.cloudflare.com/__up").send_bytes(&upload_bytes)
        })
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

    async fn validate_latency(&self) -> LatencyValidationResult {
        info!("Validating network latency...");

        let mut endpoint_latencies = Vec::new();
        let mut total_latency = 0u64;
        let mut count = 0u64;
        let mut min_ms = u64::MAX;
        let mut max_ms = 0u64;

        for endpoint in &self.config.endpoints {
            if let Ok(url) = url::Url::parse(endpoint) {
                if let Some(host) = url.host_str() {
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

    async fn measure_latency(&self, host: &str) -> u64 {
        let mut total_ms = 0u64;
        let mut successful_pings = 0u64;

        for _ in 0..3 {
            let start = Instant::now();
            let result = TcpStream::connect(format!("{host}:443")).await;
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

    pub async fn validate_ports(&self) -> PortValidationResult {
        info!("Validating required ports with BIP-341 compliance...");

        let mut open_ports = Vec::new();
        let mut closed_ports = Vec::new();

        let bip341_ports = [8333, 18333, 8433];

        for &port in bip341_ports.iter().chain(&self.config.required_ports) {
            let is_open = if port == 8433 {
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

    async fn is_port_open(&self, host: &str, port: u16) -> bool {
        let addr = format!("{host}:{port}");
        matches!(
            timeout(Duration::from_secs(3), TcpStream::connect(&addr)).await,
            Ok(Ok(_))
        )
    }

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
                    // Use ureq for MIT-compliant HTTP requests
                    let endpoint_clone = endpoint.clone();
                    let response =
                        tokio::task::spawn_blocking(move || ureq::get(&endpoint_clone).call())
                            .await;

                    match response {
                        Ok(Ok(resp)) => {
                            let is_secure = resp.status() < 400;
                            endpoints_secure.push((endpoint.clone(), is_secure));
                            if resp.status() >= 400 {
                                certificate_issues.push((
                                    endpoint.clone(),
                                    format!("Certificate error: {}", resp.status()),
                                ));
                            }
                        }
                        Ok(Err(e)) => {
                            endpoints_secure.push((endpoint.clone(), false));
                            certificate_issues.push((endpoint.clone(), format!("HTTP error: {e}")));
                        }
                        Err(e) => {
                            endpoints_secure.push((endpoint.clone(), false));
                            certificate_issues
                                .push((endpoint.clone(), format!("Connection error: {e}")));
                        }
                    }
                }
            }
        }

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

    async fn validate_firewall(&self) -> FirewallValidationResult {
        info!("Validating firewall settings...");

        let firewall_detected = self.detect_firewall().await;
        let blocks_bitcoin = self.check_firewall_blocks_bitcoin().await;
        let blocks_required_ports = self.check_blocked_ports().await;

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

    async fn detect_firewall(&self) -> bool {
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
            let output = Command::new("sudo").args(["iptables", "-L"]).output();

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

    async fn check_firewall_blocks_bitcoin(&self) -> bool {
        !(self.is_port_open("bitcoin.org", 8333).await
            || self.is_port_open("bitcoin.org", 18333).await
            || self.is_port_open("bitcoin.org", 8433).await)
    }

    async fn check_blocked_ports(&self) -> Vec<u16> {
        let mut blocked_ports = Vec::new();
        for &port in &self.config.required_ports {
            if !self.is_port_open("example.com", port).await {
                blocked_ports.push(port);
            }
        }
        blocked_ports
    }

    async fn validate_routes(&self) -> RouteValidationResult {
        info!("Validating network routes...");

        let mut routes = Vec::new();
        let mut total_hops = 0u32;
        let mut problematic_hops = Vec::new();
        let mut route_count = 0u32;

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

    async fn trace_route(&self, host: &str) -> (Vec<String>, Vec<String>) {
        let mut hops = Vec::new();
        let mut problematic = Vec::new();

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let output = Command::new("traceroute").args(["-m", "15", host]).output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines().skip(1) {
                    if line.contains("* * *") {
                        problematic.push(format!("Hop timeout: {line}"));
                    } else {
                        hops.push(line.to_string());
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            let output = Command::new("tracert").args(&["-h", "15", host]).output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines().skip(4) {
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

    async fn detect_vpn(&self) -> bool {
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
            let output = Command::new("ip").args(["tuntap", "list"]).output();

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

    fn generate_recommendations(&self, results: &ValidationResults) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !results.connectivity.internet_available {
            recommendations
                .push("Check your internet connection, it appears to be offline".to_string());
        } else if !results
            .connectivity
            .endpoints_reachable
            .iter()
            .all(|(_, reachable)| *reachable)
        {
            recommendations.push(
                "Some Bitcoin RPC endpoints are unreachable, check your network configuration"
                    .to_string(),
            );
        }

        if results.dns.status == ValidationStatus::Fail {
            recommendations.push("DNS resolution is failing, check your DNS servers or try using alternative DNS like 1.1.1.1 or 8.8.8.8".to_string());
        } else if results.dns.status == ValidationStatus::Warning {
            recommendations.push(
                "Some DNS servers are not responding, consider using more reliable DNS servers"
                    .to_string(),
            );
        }

        if results.bandwidth.status == ValidationStatus::Fail {
            recommendations.push(format!(
                "Network bandwidth is too low (Download: {:.2} Mbps, Upload: {:.2} Mbps). Minimum requirements are 1 Mbps download and 0.5 Mbps upload",
                results.bandwidth.download_mbps, results.bandwidth.upload_mbps
            ));
        } else if results.bandwidth.status == ValidationStatus::Warning {
            recommendations.push(format!(
                "Network bandwidth is below recommended levels (Download: {:.2} Mbps). For optimal performance, {:.2} Mbps is recommended",
                results.bandwidth.download_mbps, self.config.bandwidth_threshold_mbps
            ));
        }

        if results.latency.status == ValidationStatus::Fail {
            recommendations.push(
                "Network latency could not be measured, your network connection may be unstable"
                    .to_string(),
            );
        } else if results.latency.status == ValidationStatus::Warning {
            recommendations.push(format!(
                "Network latency is high (Average: {} ms). For optimal performance, latency should be below {} ms",
                results.latency.average_ms, self.config.latency_threshold_ms
            ));
        }

        if !results.ports.closed_ports.is_empty() {
            recommendations.push(format!(
                "The following required ports are closed or blocked: {}. Consider opening these ports in your firewall",
                results.ports.closed_ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
            ));
        }

        if let Some(ssl_result) = &results.ssl {
            if ssl_result.status == ValidationStatus::Fail {
                recommendations.push("SSL certificate validation failed for all endpoints. Check your system's certificate store".to_string());
            } else if ssl_result.status == ValidationStatus::Warning {
                recommendations.push("Some SSL certificates could not be validated. Your system may be missing necessary root certificates".to_string());
            }
        }

        if let Some(firewall_result) = &results.firewall {
            if firewall_result.blocks_bitcoin {
                recommendations.push("Your firewall appears to be blocking Bitcoin network traffic. Configure your firewall to allow ports 8333 (mainnet) and 18333 (testnet)".to_string());
            } else if !firewall_result.blocks_required_ports.is_empty() {
                recommendations.push(format!(
                    "Your firewall is blocking these required ports: {}. Configure your firewall to allow these ports",
                    firewall_result.blocks_required_ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
                ));
            }
            if firewall_result.blocks_required_ports.contains(&174) {
                recommendations.push(
                    "PSBT (BIP-174) port 174 is blocked. Required for partial transactions"
                        .to_string(),
                );
            }
        }

        if let Some(route_result) = &results.route {
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

        recommendations
    }

    fn determine_overall_status(&self, results: &ValidationResults) -> ValidationStatus {
        if results.connectivity.status == ValidationStatus::Fail
            || results.dns.status == ValidationStatus::Fail
            || results.bandwidth.status == ValidationStatus::Fail
        {
            return ValidationStatus::Fail;
        }

        if let Some(firewall_result) = results.firewall {
            if firewall_result.blocks_bitcoin {
                return ValidationStatus::Fail;
            }
        }

        let has_warnings = results.connectivity.status == ValidationStatus::Warning
            || results.dns.status == ValidationStatus::Warning
            || results.bandwidth.status == ValidationStatus::Warning
            || results.latency.status == ValidationStatus::Warning
            || results.ports.status == ValidationStatus::Warning;

        let optional_warnings = results
            .ssl
            .as_ref()
            .map_or(false, |r| r.status == ValidationStatus::Warning)
            || results
                .firewall
                .as_ref()
                .map_or(false, |r| r.status == ValidationStatus::Warning)
            || results
                .route
                .as_ref()
                .map_or(false, |r| r.status == ValidationStatus::Warning);

        if has_warnings || optional_warnings {
            ValidationStatus::Warning
        } else {
            ValidationStatus::Pass
        }
    }

    async fn validate_taproot_port(&self, _port: u16) -> bool {
        let taproot_check = Command::new("bitcoin-cli")
            .args(["getnetworkinfo"])
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
