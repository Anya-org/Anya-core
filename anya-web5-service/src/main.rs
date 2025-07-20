use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

// Example request/response types for DID
#[derive(Deserialize)]
pub struct CreateDidRequest {
    pub method: String,
}

#[derive(Serialize)]
pub struct DidDocumentResponse {
    pub did: String,
    pub document: serde_json::Value,
}

async fn create_did(req: web::Json<CreateDidRequest>) -> impl Responder {
    // TODO: Call web5-rust logic here
    // let did_doc = ...
    HttpResponse::Ok().json(DidDocumentResponse {
        did: "did:example:123".to_string(),
        document: serde_json::json!({"example": true}),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/did/create", web::post().to(create_did))
        // TODO: Add more routes for DWN, VC, etc.
    })
    .bind(("0.0.0.0", 8085))?
    .run()
    .await
}
