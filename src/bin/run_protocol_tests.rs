use anya_core::tests::layer2::protocol_tests::ProtocolTestSuite;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tracing::{error, info, warn};

#[derive(Debug, Serialize, Deserialize)]
struct TestReport {
    timestamp: String,
    total_protocols: usize,
    successful_protocols: usize,
    failed_protocols: usize,
    protocol_results: Vec<ProtocolResult>,
    total_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProtocolResult {
    name: String,
    status: String,
    completion_time: Option<f64>,
    milestones: Vec<MilestoneResult>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MilestoneResult {
    name: String,
    status: String,
    completion_time: Option<f64>,
    error: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let start_time = Instant::now();
    let mut report = TestReport {
        timestamp: chrono::Utc::now().to_rfc3339(),
        total_protocols: 0,
        successful_protocols: 0,
        failed_protocols: 0,
        protocol_results: Vec::new(),
        total_time: 0.0,
    };

    // Define protocols to test with their priorities
    let protocols = vec![
        // Only include protocols that are implemented and available
        // ("BOB", BobProtocol::new(), "High"),
        // ("Lightning Network", LightningProtocol::new(), "High"),
        // ("DLC", DlcProtocol::new(), "Medium"),
        // ("Liquid", LiquidProtocol::new(), "Medium"),
        // ("RGB", RgbProtocol::new(), "Medium"),
        // ("RSK", RskProtocol::new(), "Low"),
        // ("Stacks", StacksProtocol::new(), "Low"),
        // ("State Channels", StateChannelsProtocol::new(), "Low"),
        // ("Taproot Assets", TaprootAssetsProtocol::new(), "Low"),
    ];

    report.total_protocols = protocols.len();

    // Run tests for each protocol
    for (name, protocol, priority) in protocols {
        info!("Testing {} (Priority: {})...", name, priority);
        let mut suite = ProtocolTestSuite::new(name);

        // Add test milestones
        suite.add_milestone("initialization");
        suite.add_milestone("connection");
        suite.add_milestone("transaction_submission");
        suite.add_milestone("state_management");
        suite.add_milestone("asset_management");
        suite.add_milestone("security");
        suite.add_milestone("performance");

        let result = suite.run_protocol_tests(&protocol).await;

        let protocol_result = ProtocolResult {
            name: name.to_string(),
            status: if result.is_ok() {
                "Success".to_string()
            } else {
                "Failed".to_string()
            },
            completion_time: Some(suite.milestones.last()?.completion_time?.as_secs_f64()),
            milestones: suite
                .milestones
                .into_iter()
                .map(|m| MilestoneResult {
                    name: m.name,
                    status: match m.status {
                        MilestoneStatus::Completed => "Completed".to_string(),
                        MilestoneStatus::Failed => "Failed".to_string(),
                        _ => "Unknown".to_string(),
                    },
                    completion_time: m.completion_time.map(|d| d.as_secs_f64()),
                    error: m.error.clone(),
                })
                .collect(),
            error: result.err().map(|e| e.to_string()),
        };

        if result.is_ok() {
            report.successful_protocols += 1;
            info!("{} tests completed successfully", name);
        } else {
            report.failed_protocols += 1;
            error!("{} tests failed: {:?}", name, result.err());
        }

        report.protocol_results.push(protocol_result);
    }

    // Calculate total time
    report.total_time = start_time.elapsed().as_secs_f64();

    // Generate report
    let report_json = serde_json::to_string_pretty(&report)?;
    let report_dir = Path::new("test_reports");
    if !report_dir.exists() {
        fs::create_dir(report_dir)?;
    }

    let report_path = report_dir.join(format!(
        "protocol_test_report_{}.json",
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    ));
    fs::write(&report_path, report_json)?;

    // Print summary
    println!("\nTest Summary:");
    println!("Total Protocols: {}", report.total_protocols);
    println!("Successful: {}", report.successful_protocols);
    println!("Failed: {}", report.failed_protocols);
    println!("Total Time: {:.2?}", start_time.elapsed());
    println!("Report saved to: {}", report_path.display());

    // Print detailed results
    println!("\nDetailed Results:");
    for result in &report.protocol_results {
        println!("\n{}:", result.name);
        println!("  Status: {}", result.status);
        println!(
            "  Completion Time: {:.2?}",
            result
                .completion_time
                .map(|t| std::time::Duration::from_secs_f64(t))
        );
        if let Some(error) = &result.error {
            println!("  Error: {}", error);
        }

        println!("  Milestones:");
        for milestone in &result.milestones {
            println!(
                "    - {}: {} ({:.2?})",
                milestone.name,
                milestone.status,
                milestone
                    .completion_time
                    .map(|t| std::time::Duration::from_secs_f64(t))
            );
            if let Some(error) = &milestone.error {
                println!("      Error: {}", error);
            }
        }
    }

    // Exit with error if any tests failed
    if report.failed_protocols > 0 {
        std::process::exit(1);
    }
}
