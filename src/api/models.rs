use serde::{Deserialize, Serialize};

// Authentication
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_at: i64,
}

// Bitcoin wallet
#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub name: Option<String>,
    pub wallet_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WalletResponse {
    pub id: String,
    pub network: String,
    pub address_count: usize,
    pub wallet_type: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct GenerateAddressRequest {
    pub wallet_id: String,
}

#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub address: String,
    pub path: String,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct SendTransactionRequest {
    pub wallet_id: String,
    pub to_address: String,
    pub amount_sats: u64,
    pub fee_rate: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub txid: String,
    pub hex: String,
    pub fee: u64,
}

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub confirmed: u64,
    pub unconfirmed: u64,
    pub total: u64,
}

// Web5/DID
#[derive(Debug, Deserialize)]
pub struct CreateIdentityRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IdentityResponse {
    pub id: String,
    pub did: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateCredentialRequest {
    pub subject_did: String,
    pub claims: serde_json::Value,
    pub expiration: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CredentialResponse {
    pub id: String,
    pub vc: serde_json::Value,
    pub issuer: String,
    pub subject: String,
    pub created_at: i64,
}

// DLC
#[derive(Debug, Deserialize)]
pub struct CreateDlcRequest {
    pub oracle_pubkey: String,
    pub collateral_amount: u64,
    pub outcomes: Vec<DlcOutcomeRequest>,
}

#[derive(Debug, Deserialize)]
pub struct DlcOutcomeRequest {
    pub outcome_value: String,
    pub payout_to_offerer: u64,
}

#[derive(Debug, Serialize)]
pub struct DlcResponse {
    pub id: String,
    pub status: String,
    pub contract_id: String,
    pub funding_txid: Option<String>,
    pub created_at: i64,
}

// System info
#[derive(Debug, Serialize)]
pub struct SystemInfoResponse {
    pub version: String,
    pub network: String,
    pub block_height: u32,
    pub uptime: u64,
    pub peer_count: usize,
}
