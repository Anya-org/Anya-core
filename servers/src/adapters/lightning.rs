impl LightningAdapter {
    pub async fn new() -> Self {
        // Direct network access violates hexagonal boundaries
        let client = reqwest::Client::new(); 
        // Should use network port interface
    }
} 