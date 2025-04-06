// Add telemetry support
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use std::collections::HashMap;

/// Telemetry data collected for Silent Payments
/// 
/// Designed with privacy in mind - no identifiable information is included
#[derive(Debug, Clone, Default)]
pub struct SilentPaymentTelemetry {
    /// Number of transactions scanned
    pub transactions_scanned: AtomicU64,
    
    /// Number of detected payments
    pub payments_detected: AtomicU64,
    
    /// Total time spent scanning in milliseconds
    pub scanning_time_ms: AtomicU64,
    
    /// Number of scanning errors
    pub scanning_errors: AtomicU64,
}

impl SilentPaymentTelemetry {
    /// Create new telemetry instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record a transaction scan
    pub fn record_scan(&self, duration_ms: u64, payments_found: u64, error: bool) {
        self.transactions_scanned.fetch_add(1, Ordering::Relaxed);
        self.scanning_time_ms.fetch_add(duration_ms, Ordering::Relaxed);
        self.payments_detected.fetch_add(payments_found, Ordering::Relaxed);
        
        if error {
            self.scanning_errors.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    /// Get average scan time in milliseconds
    pub fn average_scan_time(&self) -> f64 {
        let total_time = self.scanning_time_ms.load(Ordering::Relaxed) as f64;
        let total_scans = self.transactions_scanned.load(Ordering::Relaxed) as f64;
        
        if total_scans > 0.0 {
            total_time / total_scans
        } else {
            0.0
        }
    }
    
    /// Get success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        let total_scans = self.transactions_scanned.load(Ordering::Relaxed) as f64;
        let errors = self.scanning_errors.load(Ordering::Relaxed) as f64;
        
        if total_scans > 0.0 {
            ((total_scans - errors) / total_scans) * 100.0
        } else {
            100.0
        }
    }
    
    /// Get detection rate (payments per transaction)
    pub fn detection_rate(&self) -> f64 {
        let total_scans = self.transactions_scanned.load(Ordering::Relaxed) as f64;
        let payments = self.payments_detected.load(Ordering::Relaxed) as f64;
        
        if total_scans > 0.0 {
            payments / total_scans
        } else {
            0.0
        }
    }
    
    /// Reset all telemetry counters
    pub fn reset(&self) {
        self.transactions_scanned.store(0, Ordering::Relaxed);
        self.payments_detected.store(0, Ordering::Relaxed);
        self.scanning_time_ms.store(0, Ordering::Relaxed);
        self.scanning_errors.store(0, Ordering::Relaxed);
    }
}

/// Scanner for detecting Silent Payments according to BIP-353
///
/// Implements the receiving side of the Silent Payments protocol,
/// allowing users to scan blocks and transactions for payments
/// sent to their Silent Payment address.
#[derive(Debug)]
pub struct SilentPaymentScanner {
    /// The scanning secret key
    scan_secret: SecretKey,
    
    /// The spending public key
    spend_pubkey: XOnlyPublicKey,
    
    /// The secp256k1 context
    secp: Secp256k1<bitcoin::secp256k1::All>,
    
    /// Detected payment information
    detected_payments: HashMap<OutPoint, SilentPaymentInfo>,
    
    /// Optional telemetry data for monitoring performance and statistics
    /// while preserving privacy (no identifying information is collected)
    telemetry: Option<SilentPaymentTelemetry>,
}

impl SilentPaymentScanner {
    /// Create a new Silent Payment scanner
    ///
    /// # Arguments
    /// * `scan_secret` - The scan secret key
    /// * `spend_pubkey` - The spending public key
    ///
    /// # Returns
    /// A new Silent Payment scanner instance
    pub fn new(scan_secret: SecretKey, spend_pubkey: XOnlyPublicKey) -> Result<Self> {
        Ok(Self {
            scan_secret,
            spend_pubkey,
            secp: Secp256k1::new(),
            detected_payments: HashMap::new(),
            telemetry: None, // Telemetry disabled by default for privacy
        })
    }
    
    /// Modify the scan_transaction method to include telemetry
    pub fn scan_transaction(&self, tx: &Transaction, block_height: Option<u32>) -> Result<Vec<SilentPaymentInfo>> {
        let start = Instant::now();
        let mut payments_found = 0;
        let mut error_occurred = false;
        
        // Main scanning logic 
        let result = match self.internal_scan_transaction(tx, block_height) {
            Ok(payments) => {
                payments_found = payments.len() as u64;
                Ok(payments)
            },
            Err(e) => {
                error_occurred = true;
                Err(e)
            }
        };
        
        // Record telemetry data with privacy preservation
        if let Some(telemetry) = &self.telemetry {
            let duration = start.elapsed().as_millis() as u64;
            telemetry.record_scan(duration, payments_found, error_occurred);
        }
        
        result
    }
    
    /// Internal method for transaction scanning
    fn internal_scan_transaction(&self, tx: &Transaction, block_height: Option<u32>) -> Result<Vec<SilentPaymentInfo>> {
        // Existing scanning logic...
    }
    
    /// Enable telemetry collection
    pub fn enable_telemetry(&mut self) {
        self.telemetry = Some(SilentPaymentTelemetry::new());
    }
    
    /// Disable telemetry collection
    pub fn disable_telemetry(&mut self) {
        self.telemetry = None;
    }
    
    /// Get telemetry data if enabled
    pub fn get_telemetry(&self) -> Option<&SilentPaymentTelemetry> {
        self.telemetry.as_ref()
    }
} 