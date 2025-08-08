use log::{error, info, warn};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

fn binary_available(bin: &str) -> bool {
    // Portable check without extra deps; works on Linux/macOS
    Command::new("which")
        .arg(bin)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn run_all() {
    info!("Running all ML tests...");

    // Test model loading
    match test_model_loading() {
        Ok(_) => info!("✅ Model loading test passed"),
        Err(e) => error!("❌ Model loading test failed: {}", e),
    }

    // Test inference
    match test_inference() {
        Ok(_) => info!("✅ Inference test passed"),
        Err(e) => error!("❌ Inference test failed: {}", e),
    }

    // Test telemetry
    match test_telemetry() {
        Ok(_) => info!("✅ Telemetry test passed"),
        Err(e) => error!("❌ Telemetry test failed: {}", e),
    }

    info!("ML tests completed");
}

fn test_model_loading() -> Result<(), String> {
    info!("Testing ML model loading...");

    // Skip if ML CLI isn't available
    if !binary_available("anya-ml") {
        warn!("Skipping ML model loading: 'anya-ml' CLI not found in PATH");
        return Ok(());
    }

    // Check if ML models directory exists
    let models_dir = "config/ml/models";
    if !Path::new(models_dir).exists() {
        warn!(
            "Skipping ML model loading: models directory not found: {}",
            models_dir
        );
        return Ok(());
    }

    // Check if at least one model exists
    let model_files = match fs::read_dir(models_dir) {
        Ok(entries) => {
            let files: Vec<_> = entries
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
                .collect();

            if files.is_empty() {
                return Err(format!("No model files found in {}", models_dir));
            }

            files
        }
        Err(e) => return Err(format!("Failed to read models directory: {}", e)),
    };

    // Attempt to load each model using the ML CLI tool
    for model_file in model_files {
        let model_path = model_file.path();
        let model_path_str = model_path.to_string_lossy();

        info!("Testing loading of model: {}", model_path_str);

        let output = Command::new("anya-ml")
            .args(&["model", "validate", "--path", &model_path_str])
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    return Err(format!(
                        "Failed to load model {}: {}",
                        model_path_str, error
                    ));
                }

                let validation_result = String::from_utf8_lossy(&output.stdout);
                info!(
                    "Model {} validated successfully: {}",
                    model_path_str, validation_result
                );
            }
            Err(e) => {
                return Err(format!(
                    "Failed to validate model {}: {}",
                    model_path_str, e
                ))
            }
        }
    }

    Ok(())
}

fn test_inference() -> Result<(), String> {
    info!("Testing ML inference...");

    if !binary_available("anya-ml") {
        warn!("Skipping ML inference: 'anya-ml' CLI not found in PATH");
        return Ok(());
    }

    // Create a simple test input
    let test_input = r#"{"text": "Test input for ML inference"}"#;
    let input_file = "ml_test_input.json";

    match fs::write(input_file, test_input) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create test input file: {}", e)),
    }

    // Run inference
    let output = Command::new("anya-ml")
        .args(&["inference", "--model", "default", "--input", input_file])
        .output();

    // Clean up input file
    fs::remove_file(input_file).ok();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Inference failed: {}", error));
            }

            let inference_result = String::from_utf8_lossy(&output.stdout);
            info!("Inference successful: {}", inference_result);

            // Parse the result to verify it's valid JSON
            match serde_json::from_str::<serde_json::Value>(&inference_result) {
                Ok(_) => (),
                Err(e) => return Err(format!("Inference result is not valid JSON: {}", e)),
            }

            Ok(())
        }
        Err(e) => Err(format!("Failed to run inference: {}", e)),
    }
}

fn test_telemetry() -> Result<(), String> {
    info!("Testing ML telemetry...");

    if !binary_available("anya-ml") {
        warn!("Skipping ML telemetry: 'anya-ml' CLI not found in PATH");
        return Ok(());
    }

    // Check if telemetry is enabled
    let output = Command::new("anya-ml")
        .args(&["telemetry", "status"])
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to check telemetry status: {}", error));
            }

            let status = String::from_utf8_lossy(&output.stdout);
            info!("Telemetry status: {}", status);

            // Test sending a telemetry event
            let telemetry_event = r#"{"event_type": "test", "metadata": {"test": true}}"#;
            let event_file = "ml_test_event.json";

            match fs::write(event_file, telemetry_event) {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to create test event file: {}", e)),
            }

            let send_output = Command::new("anya-ml")
                .args(&["telemetry", "send", "--input", event_file])
                .output();

            // Clean up event file
            fs::remove_file(event_file).ok();

            match send_output {
                Ok(output) => {
                    if !output.status.success() {
                        let error = String::from_utf8_lossy(&output.stderr);
                        return Err(format!("Failed to send telemetry event: {}", error));
                    }

                    let result = String::from_utf8_lossy(&output.stdout);
                    info!("Telemetry event sent successfully: {}", result);
                    Ok(())
                }
                Err(e) => Err(format!("Failed to send telemetry event: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to check telemetry status: {}", e)),
    }
}
