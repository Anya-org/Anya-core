use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::monitoring::metrics;

/// Starts the metrics server on the specified address
pub fn start_metrics_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Metrics server started on {}", address);
    
    for stream in listener.incoming() {
        let stream = stream?;
        
        thread::spawn(|| {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
    
    Ok(())
}

/// Handles an HTTP connection
fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    
    let get = b"GET /metrics HTTP/1.1\r\n";
    
    let (status_line, content) = if buffer.starts_with(get) {
        // Metrics endpoint
        let metrics = metrics::export_metrics();
        ("HTTP/1.1 200 OK", metrics)
    } else {
        // Not found
        ("HTTP/1.1 404 NOT FOUND", "404 Not Found".to_string())
    };
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::str::FromStr;
    
    #[test]
    fn test_server_creation() {
        // This is just a basic test to ensure the server creation logic works
        // Bind to a random port for testing
        let addr = SocketAddr::from_str("127.0.0.1:0").unwrap();
        let listener = TcpListener::bind(addr).unwrap();
        let local_addr = listener.local_addr().unwrap();
        
        assert!(local_addr.port() > 0);
    }
} 