use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

pub fn mobile_api_scope() -> Scope {
    web::scope(std::path::Path::new("/").join("mobile").to_string_lossy())
        .service(create_wallet)
        .service(get_balance)
        .service(send_transaction)
        .service(verify_payment)
}

#[derive(Deserialize)]
struct CreateWalletRequest {
    name: String,
    taproot_enabled: bool,
    security_level: SecurityLevel,
}

#[derive(Serialize)]
struct WalletResponse {
    id: String,
    address: String,
    balance: u64,
    security_info: SecurityInfo,
}

#[post(std::path::Path::new("/").join("wallets").to_string_lossy())]
async fn create_wallet(
    data: web::Data<AppState>,
    req: web::Json<CreateWalletRequest>
) -> HttpResponse {
    let wallet = data.wallet_manager
        .create_enhanced_wallet(&req.name, req.taproot_enabled, req.security_level)
        .await;

    match wallet {
        Ok(w) => HttpResponse::Ok().json(WalletResponse::from(w)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[get(std::path::Path::new("/").join("wallets/{id}/balance").to_string_lossy())]
async fn get_balance(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> HttpResponse {
    // Secure balance retrieval implementation
}

#[post(std::path::Path::new("/").join("transactions").to_string_lossy())] 
async fn send_transaction(
    data: web::Data<AppState>,
    req: web::Json<SendRequest>
) -> HttpResponse {
    // Secure transaction sending implementation
}

#[post(std::path::Path::new("/").join("verify").to_string_lossy())]
async fn verify_payment(
    data: web::Data<AppState>, 
    req: web::Json<VerifyRequest>
) -> HttpResponse {
    // Payment verification with SPV
}
