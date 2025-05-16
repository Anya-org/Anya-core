# Error Handling in Anya Core

This document outlines the error handling strategy and patterns used throughout the Anya Core codebase.

## Table of Contents
- [Error Types](#error-types)
- [Error Handling Patterns](#error-handling-patterns)
- [Error Propagation](#error-propagation)
- [Logging and Monitoring](#logging-and-monitoring)
- [Best Practices](#best-practices)
- [Common Error Scenarios](#common-error-scenarios)

## Error Types

### 1. Domain Errors

Errors that represent business logic failures.

```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid input: {0}")]
    ValidationError(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Authentication failed")]
    AuthenticationError,
    
    #[error("Authorization failed: {0}")]
    AuthorizationError(String),
}
```

### 2. Infrastructure Errors

Errors from external systems and infrastructure.

```rust
#[derive(Debug, thiserror::Error)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
}
```

### 3. Application Errors

Errors specific to application logic and use cases.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Request timeout")]
    Timeout,
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}
```

## Error Handling Patterns

### 1. Using `thiserror` for Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    Validation(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
```

### 2. Result Type Alias

```rust
pub type Result<T> = std::result::Result<T, MyError>;

fn process_data(data: &[u8]) -> Result<ProcessedData> {
    // Function implementation
}
```

### 3. Error Conversion

```rust
impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError::NetworkError(err.to_string())
    }
}
```

## Error Propagation

### Using the `?` Operator

```rust
fn process_file(path: &str) -> Result<Data> {
    let content = std::fs::read_to_string(path)?;  // Automatically converts io::Error to MyError
    let data: Data = serde_json::from_str(&content)?;  // Automatically handles serde_json::Error
    Ok(data)
}
```

### Contextual Errors

```rust
use anyhow::{Context, Result};

fn process_config() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    
    let _settings: Settings = toml::from_str(&config)
        .context("Failed to parse config file")?;
    
    Ok(())
}
```

## Logging and Monitoring

### Structured Logging

```rust
use tracing::{error, info, warn};

async fn process_request(request: Request) -> Result<Response> {
    info!(request_id = %request.id, "Processing request");
    
    let result = some_operation().await
        .map_err(|e| {
            error!(
                error = %e,
                request_id = %request.id,
                "Failed to process request"
            );
            e
        })?;
    
    Ok(Response::new(result))
}
```

### Metrics

```rust
use metrics::{counter, histogram};

pub async fn process_item(item: Item) -> Result<()> {
    let start = std::time::Instant::now();
    
    let result = process(item).await;
    
    let elapsed = start.elapsed();
    histogram!("process_item_duration_seconds", elapsed.as_secs_f64());
    
    match &result {
        Ok(_) => counter!("process_item_success_total", 1),
        Err(e) => {
            counter!("process_item_error_total", 1);
            error!("Failed to process item: {}", e);
        }
    }
    
    result
}
```

## Best Practices

### 1. Use Descriptive Error Messages

```rust
// Bad
#[error("Error")]

// Good
#[error("Failed to parse configuration file: {path}")]
```

### 2. Include Context

```rust
// Bad
read_file(path).await?;

// Good
read_file(path).await
    .with_context(|| format!("Failed to read file: {}", path))?;
```

### 3. Handle Errors Appropriately

```rust
match some_operation().await {
    Ok(result) => process(result),
    Err(MyError::NotFound(_)) => handle_not_found(),
    Err(MyError::RateLimitExceeded) => retry_after_delay().await,
    Err(e) => return Err(e.into()),
}
```

## Common Error Scenarios

### 1. Database Errors

```rust
pub async fn get_user(user_id: Uuid) -> Result<User> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| DomainError::NotFound(format!("User {} not found", user_id)))
}
```

### 2. Network Requests

```rust
pub async fn fetch_data(url: &str) -> Result<Data> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| InfrastructureError::NetworkError(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(InfrastructureError::NetworkError(
            format!("Request failed with status: {}", response.status())
        ));
    }
    
    response.json().await
        .map_err(|e| InfrastructureError::NetworkError(e.to_string()))
}
```

### 3. Input Validation

```rust
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self> {
        if !email.contains('@') {
            return Err(DomainError::ValidationError(
                "Invalid email format".to_string()
            ));
        }
        Ok(Self(email.to_string()))
    }
}
```

## Testing Error Handling

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    
    #[test]
    fn test_invalid_email() {
        let result = Email::new("invalid-email");
        assert_matches!(result, Err(DomainError::ValidationError(_)));
    }
    
    #[tokio::test]
    async fn test_user_not_found() {
        let result = get_user(Uuid::new_v4()).await;
        assert_matches!(result, Err(DomainError::NotFound(_)));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_network_failure() {
    let server = mockito::Server::new();
    let _m = server
        .mock("GET", "/data")
        .with_status(500)
        .create();
    
    let result = fetch_data(&server.url()).await;
    assert_matches!(result, Err(InfrastructureError::NetworkError(_)));
}
```

## Monitoring and Alerting

### Log Analysis

```bash
# Search for errors in logs
journalctl -u anya-core --since "1 hour ago" | grep -i error

# Count errors by type
cat app.log | grep ERROR | awk '{print $5}' | sort | uniq -c | sort -nr
```

### Alerting Rules

```yaml
groups:
- name: anya-core-errors
  rules:
  - alert: HighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
    for: 10m
    labels:
      severity: critical
    annotations:
      summary: "High error rate on {{ $labels.instance }}"
      description: "{{ $value }}% of requests are failing"
```

## Conclusion

Proper error handling is crucial for building reliable and maintainable applications. By following these patterns and best practices, we can ensure that Anya Core handles errors gracefully and provides meaningful feedback to users and developers.
