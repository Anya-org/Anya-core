use anya_core::tools::doc_duplication_scanner_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the CLI tool for documentation duplication scanning
    doc_duplication_scanner_cli::run_cli().await?;
    Ok(())
}
