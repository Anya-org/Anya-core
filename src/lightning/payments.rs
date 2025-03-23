// Updated Payment Handling with BOLT 12
impl LightningNode {
    pub fn create_offer(&self, request: OfferRequest) -> Result<Bolt12Offer> {
        Bolt12Offer::new(
            request.amount_msat,
            request.description,
            request.expiry_secs,
            self.node_id.to_string(),
        )
    }

    pub fn request_invoice_from_offer(&self, offer: &Bolt12Offer) -> Result<Invoice> {
        let payment_paths = self.router.find_path_for_offer(
            offer.inner.clone(),
            self.channel_manager.list_channels(),
            self.network_graph.clone(),
        )?;
        
        InvoiceBuilder::from_offer(offer.inner.clone())
            .payment_paths(payment_paths)
            .build()
    }

    pub fn send_payment_for_offer(&self, offer: &Bolt12Offer) -> Result<PaymentHash> {
        let invoice = self.request_invoice_from_offer(offer)?;
        self.send_payment(&invoice)
    }
} 