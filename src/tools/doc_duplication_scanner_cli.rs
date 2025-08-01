use crate::tools::doc_duplication_scanner::{scan_for_duplicates, DuplicationReport, ScanOptions};
use crate::tools::source_of_truth_registry::{get_global_registry, initialize_global_registry};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

/// CLI entrypoint for documentation duplication scanner
pub async fn run_cli() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Process command line arguments
    let options = parse_cli_args(&args)?;

    println!(
        "Starting documentation duplication scan with threshold: {}",
        options.similarity_threshold
    );
    println!("Scanning directory: {}", options.scan_path.display());

    // Initialize the registry if needed
    if get_global_registry().await.read().await.is_none() {
        initialize_global_registry().await?;
    }

    // Run the scan
    let report = scan_for_duplicates(&options).await?;

    // Output the report
    print_report(&report, options.output_format.as_deref())?;

    // Set exit code based on findings and threshold
    if !report.duplications.is_empty() && options.fail_on_duplicates {
        println!(
            "\nDuplication scan failed: {} duplicate groups found",
            report.duplications.len()
        );
        process::exit(1);
    }

    Ok(())
}

fn parse_cli_args(args: &[String]) -> Result<ScanOptions, Box<dyn Error>> {
    let mut options = ScanOptions {
        scan_path: PathBuf::from("."),
        file_extensions: vec![
            "md".to_string(),
            "rst".to_string(),
            "html".to_string(),
            "txt".to_string(),
        ],
        similarity_threshold: 0.85,
        ignore_patterns: Vec::new(),
        output_format: Some("text".to_string()),
        fail_on_duplicates: false,
        max_results: None,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--path" | "-p" => {
                i += 1;
                if i < args.len() {
                    options.scan_path = PathBuf::from(&args[i]);
                }
            }
            "--threshold" | "-t" => {
                i += 1;
                if i < args.len() {
                    options.similarity_threshold = args[i].parse::<f32>()?;
                }
            }
            "--format" | "-f" => {
                i += 1;
                if i < args.len() {
                    options.output_format = Some(args[i].clone());
                }
            }
            "--extensions" | "-e" => {
                i += 1;
                if i < args.len() {
                    options.file_extensions =
                        args[i].split(',').map(|s| s.trim().to_string()).collect();
                }
            }
            "--ignore" | "-i" => {
                i += 1;
                if i < args.len() {
                    options.ignore_patterns =
                        args[i].split(',').map(|s| s.trim().to_string()).collect();
                }
            }
            "--fail-on-duplicates" => {
                options.fail_on_duplicates = true;
            }
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            _ => {
                println!("Unknown option: {}", args[i]);
                print_help();
                process::exit(1);
            }
        }
        i += 1;
    }

    Ok(options)
}

fn print_report(report: &DuplicationReport, format: Option<&str>) -> Result<(), Box<dyn Error>> {
    match format {
        Some("json") => {
            println!("{}", serde_json::to_string_pretty(report)?);
        }
        Some("markdown") | Some("md") => {
            println!("# Documentation Duplication Report\n");
            println!("## Summary\n");
            println!("- Total files scanned: {}", report.files_scanned);
            println!("- Duplication groups found: {}", report.duplications.len());
            println!("- Total sections analyzed: {}", report.sections_analyzed);
            println!("\n## Duplications Found\n");

            for (i, dup) in report.duplications.iter().enumerate() {
                println!("### Group {}\n", i + 1);
                println!("Similarity: {}%\n", (dup.similarity * 100.0) as u32);

                for entry in &dup.entries {
                    println!("- **File:** `{}`", entry.file_path);
                    println!("  - **Section:** {}", entry.section);
                    println!("  - **Snippet:** ```\n{}\n  ```\n", entry.content_snippet);
                }
            }
        }
        _ => {
            // Default text format
            println!("Documentation Duplication Report");
            println!("==============================\n");
            println!("Files scanned: {}", report.files_scanned);
            println!("Sections analyzed: {}", report.sections_analyzed);
            println!("Duplication groups found: {}\n", report.duplications.len());

            for (i, dup) in report.duplications.iter().enumerate() {
                println!(
                    "Group {} (Similarity: {}%)",
                    i + 1,
                    (dup.similarity * 100.0) as u32
                );
                println!("----------------------------------------");

                for entry in &dup.entries {
                    println!("File: {}", entry.file_path);
                    println!("Section: {}", entry.section);
                    println!("Snippet: \n{}\n", entry.content_snippet);
                }
                println!();
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("Documentation Duplication Scanner");
    println!("Usage: doc_duplication_scanner [options]");
    println!();
    println!("Options:");
    println!("  --path, -p PATH           Path to scan for documentation files");
    println!("  --threshold, -t FLOAT     Similarity threshold (0.0-1.0), default: 0.85");
    println!("  --format, -f FORMAT       Output format (text, json, markdown)");
    println!("  --extensions, -e LIST     Comma-separated file extensions to scan");
    println!("  --ignore, -i PATTERNS     Comma-separated patterns to ignore");
    println!("  --fail-on-duplicates      Exit with error code if duplicates found");
    println!("  --help, -h                Show this help message");
}
