#![feature(edition2021)]
// BOLT 12 Implementation (Offers Protocol)
use lightning::offers::offer::Offer;
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

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.inner.write(&mut buffer).unwrap();
        buffer
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