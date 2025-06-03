/// [AIR-3][AIS-3][BPC-3][RES-3] Execute an async operation with timeout and progress tracking
pub async fn execute_with_monitoring<T, F>(
    operation_name: &str,
    timeout_duration: Duration,
    operation: F
) -> AnyaResult<T> 
where
    F: Future<Output = AnyaResult<T>> {
    // Create watchdog
    let watchdog = Watchdog::new(operation_name, timeout_duration);
    
    // Execute with timeout
    match tokio::time::timeout(timeout_duration, operation).await {
        Ok(result) => {
            // Operation completed within timeout
            watchdog.stop();
            result
        }
        Err(_) => {
            // Operation timed out
            watchdog.trigger_alert();
            let error_msg = format!("Operation '{}' timed out after {:?}", operation_name, timeout_duration);
            error!("{}", error_msg);
            Err(AnyaError::Timeout(error_msg))
        }
    }
}

/// [AIR-3][AIS-3][BPC-3][RES-3] Execute with recovery attempt on timeout
pub async fn execute_with_recovery<T, F, R>(
    operation_name: &str,
    primary_timeout: Duration,
    recovery_timeout: Duration,
    primary_operation: F,
    recovery_operation: R
) -> AnyaResult<T>
where
    F: Future<Output = AnyaResult<T>>,
    R: Future<Output = AnyaResult<T>> {
    // Create watchdog for the entire operation
    let watchdog = Watchdog::new(operation_name, primary_timeout + recovery_timeout + Duration::from_secs(1));
    
    // Try primary operation with timeout
    match tokio::time::timeout(primary_timeout, primary_operation).await {
        Ok(result) => {
            // Primary operation completed within timeout
            watchdog.stop();
            return result;
        }
        Err(_) => {
            // Primary operation timed out, try recovery
            warn!(
                "Operation '{}' timed out after {:?}, attempting recovery",
                operation_name, primary_timeout
            );
            
            // Try recovery operation with timeout
            match tokio::time::timeout(recovery_timeout, recovery_operation).await {
                Ok(result) => {
                    // Recovery completed within timeout
                    watchdog.stop();
                    info!("Recovery for '{}' succeeded", operation_name);
                    result
                }
                Err(_) => {
                    // Recovery also timed out
                    watchdog.trigger_alert();
                    let error_msg = format!(
                        "Operation '{}' and recovery both timed out (after {:?} and {:?})",
                        operation_name, primary_timeout, recovery_timeout
                    );
                    error!("{}", error_msg);
                    Err(AnyaError::Timeout(error_msg))
                }
            }
        }
    }
}
