// BOLT12 Implementation Tests
use crate::lightning::bolt12::{
    Bolt12Offer,
    Bolt12InvoiceRequest,
    Bolt12Invoice,
    Bolt12Payment,
    Bolt12Refund
};
use std::error::Error;

#[test]
fn test_bolt12_offer_creation() -> Result<(), Box<dyn Error>> {
    let offer = Bolt12Offer::new(
        1_000_000,           // 1000 sats
        "Test Payment".into(),
        3600,               // 1 hour expiry
        "Test Merchant".into()
    )?;
    
    let serialized = offer.serialize()?;
    assert!(!serialized.is_empty(), "Serialized offer should not be empty");
    
    let deserialized = Bolt12Offer::from_bytes(&serialized)?;
    let re_serialized = deserialized.serialize()?;
    
    assert_eq!(serialized, re_serialized, "Re-serialized offer should match original");
    
    Ok(())
}

#[test]
fn test_bolt12_invoice_request() -> Result<(), Box<dyn Error>> {
    let offer = Bolt12Offer::new(
        1_000_000,
        "Test Payment".into(),
        3600,
        "Test Merchant".into()
    )?;
    
    let payer_id = [0u8; 32];  // Sample payer ID
    let invoice_request = Bolt12InvoiceRequest::new(
        &offer, 
        payer_id,
        Some("Payment for goods".into())
    )?;
    
    let serialized = invoice_request.serialize()?;
    assert!(!serialized.is_empty(), "Serialized invoice request should not be empty");
    
    let deserialized = Bolt12InvoiceRequest::from_bytes(&serialized)?;
    let re_serialized = deserialized.serialize()?;
    
    assert_eq!(serialized, re_serialized, "Re-serialized invoice request should match original");
    
    Ok(())
}

#[test]
fn test_bolt12_invoice() -> Result<(), Box<dyn Error>> {
    let offer = Bolt12Offer::new(
        1_000_000,
        "Test Payment".into(),
        3600,
        "Test Merchant".into()
    )?;
    
    let payer_id = [0u8; 32];  // Sample payer ID
    let invoice_request = Bolt12InvoiceRequest::new(
        &offer, 
        payer_id,
        Some("Payment for goods".into())
    )?;
    
    let payment_hash = [0u8; 32];  // Sample payment hash
    let node_id = [0u8; 33];       // Sample node ID
    
    let invoice = Bolt12Invoice::from_request(
        &invoice_request,
        payment_hash,
        node_id
    )?;
    
    let serialized = invoice.serialize()?;
    assert!(!serialized.is_empty(), "Serialized invoice should not be empty");
    
    let deserialized = Bolt12Invoice::from_bytes(&serialized)?;
    let re_serialized = deserialized.serialize()?;
    
    assert_eq!(serialized, re_serialized, "Re-serialized invoice should match original");
    assert_eq!(invoice.payment_hash(), payment_hash, "Payment hash should match");
    
    Ok(())
}

#[test]
fn test_bolt12_payment() -> Result<(), Box<dyn Error>> {
    let offer = Bolt12Offer::new(
        1_000_000,
        "Test Payment".into(),
        3600,
        "Test Merchant".into()
    )?;
    
    let payer_id = [0u8; 32];  // Sample payer ID
    let invoice_request = Bolt12InvoiceRequest::new(
        &offer, 
        payer_id,
        Some("Payment for goods".into())
    )?;
    
    let payment_hash = [0u8; 32];  // Sample payment hash
    let node_id = [0u8; 33];       // Sample node ID
    
    let invoice = Bolt12Invoice::from_request(
        &invoice_request,
        payment_hash,
        node_id
    )?;
    
    let payment_preimage = [0u8; 32];  // Sample payment preimage
    let payment = Bolt12Payment::new(&invoice, payment_preimage)?;
    
    let serialized = payment.serialize()?;
    assert!(!serialized.is_empty(), "Serialized payment should not be empty");
    
    let deserialized = Bolt12Payment::from_bytes(&serialized)?;
    let re_serialized = deserialized.serialize()?;
    
    assert_eq!(serialized, re_serialized, "Re-serialized payment should match original");
    
    Ok(())
}

#[test]
fn test_bolt12_refund() -> Result<(), Box<dyn Error>> {
    let offer = Bolt12Offer::new(
        1_000_000,
        "Test Payment".into(),
        3600,
        "Test Merchant".into()
    )?;
    
    let payer_id = [0u8; 32];  // Sample payer ID
    let invoice_request = Bolt12InvoiceRequest::new(
        &offer, 
        payer_id,
        Some("Payment for goods".into())
    )?;
    
    let payment_hash = [0u8; 32];  // Sample payment hash
    let node_id = [0u8; 33];       // Sample node ID
    
    let invoice = Bolt12Invoice::from_request(
        &invoice_request,
        payment_hash,
        node_id
    )?;
    
    let payment_preimage = [0u8; 32];  // Sample payment preimage
    let payment = Bolt12Payment::new(&invoice, payment_preimage)?;
    
    let refund_amount = 500_000;  // 500 sats refund
    let refund = Bolt12Refund::new(&payment, refund_amount)?;
    
    let serialized = refund.serialize()?;
    assert!(!serialized.is_empty(), "Serialized refund should not be empty");
    
    let deserialized = Bolt12Refund::from_bytes(&serialized)?;
    let re_serialized = deserialized.serialize()?;
    
    assert_eq!(serialized, re_serialized, "Re-serialized refund should match original");
    
    Ok(())
}

#[test]
fn test_complete_bolt12_flow() -> Result<(), Box<dyn Error>> {
    // 1. Create an offer
    let offer = Bolt12Offer::new(
        1_000_000,
        "Complete BOLT12 Test".into(),
        3600,
        "Test Merchant".into()
    )?;
    
    // 2. Customer creates an invoice request
    let payer_id = [1u8; 32];
    let invoice_request = Bolt12InvoiceRequest::new(
        &offer, 
        payer_id,
        Some("Testing full BOLT12 flow".into())
    )?;
    
    // 3. Merchant creates an invoice
    let payment_hash = [2u8; 32];
    let node_id = [3u8; 33];
    
    let invoice = Bolt12Invoice::from_request(
        &invoice_request,
        payment_hash,
        node_id
    )?;
    
    // 4. Customer creates a payment
    let payment_preimage = [4u8; 32];
    let payment = Bolt12Payment::new(&invoice, payment_preimage)?;
    
    // 5. If needed, merchant creates a refund
    let refund_amount = 250_000;  // Partial refund
    let refund = Bolt12Refund::new(&payment, refund_amount)?;
    
    // Verify all serializations work
    assert!(!offer.serialize()?.is_empty());
    assert!(!invoice_request.serialize()?.is_empty());
    assert!(!invoice.serialize()?.is_empty());
    assert!(!payment.serialize()?.is_empty());
    assert!(!refund.serialize()?.is_empty());
    
    Ok(())
}
