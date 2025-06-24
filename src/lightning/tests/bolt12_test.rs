#[cfg(test)]
mod tests {
    use super::super::*;
    use std::error::Error;

    #[test]
    fn test_offer_creation_and_serialization() -> Result<(), Box<dyn Error>> {
        let offer = Bolt12Offer::new(
            100_000, // 100k sats
            "Test BOLT12 Offer".to_string(),
            3600, // 1 hour
            "Test Issuer".to_string(),
        )?;
        
        let serialized = offer.serialize()?;
        assert!(!serialized.is_empty(), "Offer serialization should not be empty");
        
        let deserialized = Bolt12Offer::from_bytes(&serialized)?;
        assert_eq!(
            deserialized.metadata.description,
            "Test BOLT12 Offer".to_string(),
            "Deserialized offer should have the same description"
        );
        
        Ok(())
    }
    
    #[test]
    fn test_invoice_request_flow() -> Result<(), Box<dyn Error>> {
        // Create an offer
        let offer = Bolt12Offer::new(
            200_000, // 200k sats
            "Test Service".to_string(),
            7200, // 2 hours
            "Service Provider".to_string(),
        )?;
        
        // Create a payer ID (just for testing)
        let payer_id = [0u8; 32];
        
        // Create an invoice request from the offer
        let request = Bolt12InvoiceRequest::new(
            &offer,
            payer_id,
            Some("Please process my order #12345".to_string()),
        )?;
        
        let serialized = request.serialize()?;
        assert!(!serialized.is_empty(), "Request serialization should not be empty");
        
        // Check that we can deserialize it
        let _deserialized = Bolt12InvoiceRequest::from_bytes(&serialized)?;
        
        Ok(())
    }
    
    #[test]
    fn test_complete_payment_flow() -> Result<(), Box<dyn Error>> {
        // Create an offer
        let offer = Bolt12Offer::new(
            150_000, // 150k sats
            "Complete Payment Test".to_string(),
            3600, // 1 hour
            "Merchant".to_string(),
        )?;
        
        // Create a payer ID and other required values (just for testing)
        let payer_id = [1u8; 32];
        let payment_hash = [2u8; 32];
        let payment_preimage = [3u8; 32];
        let node_id = [4u8; 33];
        
        // 1. Create invoice request from offer
        let request = Bolt12InvoiceRequest::new(&offer, payer_id, None)?;
        
        // 2. Create invoice from request
        let invoice = Bolt12Invoice::from_request(&request, payment_hash, node_id)?;
        assert_eq!(invoice.payment_hash(), payment_hash, "Payment hash should match");
        
        // 3. Create payment from invoice
        let payment = Bolt12Payment::new(&invoice, payment_preimage)?;
        let serialized_payment = payment.serialize()?;
        assert!(!serialized_payment.is_empty(), "Payment serialization should not be empty");
        
        // 4. Create refund from payment
        let refund_amount = 50_000; // Partial refund
        let refund = Bolt12Refund::new(&payment, refund_amount)?;
        let serialized_refund = refund.serialize()?;
        assert!(!serialized_refund.is_empty(), "Refund serialization should not be empty");
        
        Ok(())
    }
}
