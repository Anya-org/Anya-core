use std::error::Error;
// BOLT 12 Implementation (Offers Protocol)
use lightning::offers::offer::Offer;
use lightning::offers::invoice_request::InvoiceRequest;
use lightning::offers::invoice::Invoice;
use lightning::offers::refund::Refund;
use lightning::offers::payment::Payment;
use lightning::offers::parse::Bolt12ParseError;
use lightning::util::ser::Writeable;

#[derive(Debug, Clone)]
pub struct Bolt12Offer {
    inner: Offer,
    metadata: OfferMetadata,
}

impl Bolt12Offer {
    pub fn new(
        amount_msat: u64,
        description: String,
        expiry_secs: u32,
        issuer: String,
    ) -> Result<Self, Bolt12ParseError> {
        let builder = Offer::blank()
            .amount_msat(amount_msat)
            .description(description.clone())
            .expiry_time(expiry_secs)
            .issuer(issuer);
        
        Ok(Self {
            inner: builder.build()?,
            metadata: OfferMetadata::new(description, expiry_secs),
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        self.inner.write(&mut buffer)?;
        Ok(buffer)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Bolt12ParseError> {
        let offer = Offer::try_from(bytes)?;
        Ok(Self {
            inner: offer,
            metadata: OfferMetadata::default(),
        })
    }
}

#[derive(Debug, Clone)]
struct OfferMetadata {
    created_at: u64,
    description: String,
    expiry_secs: u32,
}

impl OfferMetadata {
    fn new(description: String, expiry_secs: u32) -> Self {
        Self {
            created_at: unix_time(),
            description,
            expiry_secs,
        }
    }
}

impl Default for OfferMetadata {
    fn default() -> Self {
        Self {
            created_at: unix_time(),
            description: String::new(),
            expiry_secs: 3600, // Default: 1 hour
        }
    }
}

/// Unix timestamp in seconds
fn unix_time() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// Invoice Request implementation for BOLT12
#[derive(Debug, Clone)]
pub struct Bolt12InvoiceRequest {
    inner: InvoiceRequest,
    metadata: RequestMetadata,
}

#[derive(Debug, Clone)]
struct RequestMetadata {
    created_at: u64,
    payer_note: Option<String>,
}

impl Default for RequestMetadata {
    fn default() -> Self {
        Self {
            created_at: unix_time(),
            payer_note: None,
        }
    }
}

impl Bolt12InvoiceRequest {
    pub fn new(
        offer: &Bolt12Offer,
        payer_id: [u8; 32],
        payer_note: Option<String>,
    ) -> Result<Self, Bolt12ParseError> {
        let builder = InvoiceRequest::from_offer(
            &offer.inner, 
            payer_id
        ).expect("Valid offer");
        
        let metadata = RequestMetadata {
            created_at: unix_time(),
            payer_note,
        };
        
        Ok(Self {
            inner: builder.build()?,
            metadata,
        })
    }
    
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        self.inner.write(&mut buffer)?;
        Ok(buffer)
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Bolt12ParseError> {
        let request = InvoiceRequest::try_from(bytes)?;
        Ok(Self {
            inner: request,
            metadata: RequestMetadata::default(),
        })
    }
}

// Invoice implementation for BOLT12
#[derive(Debug, Clone)]
pub struct Bolt12Invoice {
    inner: Invoice,
}

impl Bolt12Invoice {
    pub fn from_request(
        request: &Bolt12InvoiceRequest,
        payment_hash: [u8; 32],
        node_id: [u8; 33],
    ) -> Result<Self, Bolt12ParseError> {
        let invoice = Invoice::from_request(
            &request.inner, 
            payment_hash,
            node_id
        ).expect("Valid request");
        
        Ok(Self {
            inner: invoice,
        })
    }
    
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        self.inner.write(&mut buffer)?;
        Ok(buffer)
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Bolt12ParseError> {
        let invoice = Invoice::try_from(bytes)?;
        Ok(Self {
            inner: invoice,
        })
    }
    
    pub fn payment_hash(&self) -> [u8; 32] {
        self.inner.payment_hash()
    }
}

// Payment implementation for BOLT12
#[derive(Debug, Clone)]
pub struct Bolt12Payment {
    inner: Payment,
}

impl Bolt12Payment {
    pub fn new(
        invoice: &Bolt12Invoice,
        payment_preimage: [u8; 32],
    ) -> Result<Self, Bolt12ParseError> {
        let payment = Payment::for_invoice(
            &invoice.inner, 
            payment_preimage
        ).expect("Valid invoice");
        
        Ok(Self {
            inner: payment,
        })
    }
    
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        self.inner.write(&mut buffer)?;
        Ok(buffer)
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Bolt12ParseError> {
        let payment = Payment::try_from(bytes)?;
        Ok(Self {
            inner: payment,
        })
    }
}

// Refund implementation for BOLT12
#[derive(Debug, Clone)]
pub struct Bolt12Refund {
    inner: Refund,
}

impl Bolt12Refund {
    pub fn new(
        payment: &Bolt12Payment,
        refund_amount_msat: u64,
    ) -> Result<Self, Bolt12ParseError> {
        let refund = Refund::for_payment(
            &payment.inner, 
            refund_amount_msat
        ).expect("Valid payment");
        
        Ok(Self {
            inner: refund,
        })
    }
    
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        self.inner.write(&mut buffer)?;
        Ok(buffer)
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Bolt12ParseError> {
        let refund = Refund::try_from(bytes)?;
        Ok(Self {
            inner: refund,
        })
    }
}
