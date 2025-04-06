#[openapi]
impl OpenBankingApi {
    /// PSD2-compliant account access
    #[get(std::path::Path::new("/").join("accounts/{account_id}").to_string_lossy())]
    async fn get_account(&self, account_id: String) -> Result<AccountInfo> {
        let proof = verify_psd2_headers()?;
        let data = self.ledger.get_account(account_id, proof).await?;
        Ok(AccountInfo::from(data))
    }

    /// Initiate SEPA payment
    #[post(std::path::Path::new("/").join("payments").to_string_lossy())]
    async fn create_payment(&self, payment: PaymentInitiation) -> Result<PaymentStatus> {
        let compliance = self.compliance.check_payment(&payment).await?;
        let tx_hash = self.settlement_engine.execute(payment, compliance).await?;
        Ok(PaymentStatus::new(tx_hash))
    }
}

#[openapi]
impl EnterpriseBankingApi {
    /// Bulk payment processing
    #[post(std::path::Path::new("/").join("bulk-payments").to_string_lossy())]
    async fn create_bulk_payments(
        &self,
        payments: Vec<EnterprisePayment>,
        approval_sig: ApprovalSignature
    ) -> Result<BulkPaymentReceipt> {
        verify_enterprise_signature(&approval_sig)?;
        
        let compliance_cert = self.compliance.bulk_screening(&payments)?;
        let batch_proof = self.settlement_engine.execute_batch(
            payments,
            compliance_cert
        ).await?;
        
        Ok(BulkPaymentReceipt {
            batch_id: batch_proof.batch_id,
            tx_count: batch_proof.tx_count,
            total_amount: batch_proof.total_amount,
            blockchain_proof: batch_proof.taproot_commitment
        })
    }

    /// Real-time liquidity management
    #[get(std::path::Path::new("/").join("liquidity").to_string_lossy())]
    async fn get_liquidity_dashboard(
        &self, 
        currencies: Vec<Currency>,
        risk_profile: RiskLevel
    ) -> Result<LiquidityAnalysis> {
        let analysis = self.liquidity_engine.analyze(
            currencies,
            risk_profile,
            self.fx_oracle.get_rates().await?
        )?;
        
        Ok(analysis)
    }
} 