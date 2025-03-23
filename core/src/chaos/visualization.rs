//! Network visualization dashboard
use libp2p::PeerId;
use serde_json::json;
use actix_web::{web, App, HttpResponse, HttpServer};

pub async fn run_server(port: u16) -> anyhow::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(serve_dashboard))
            .route("/data", web::get().to(network_data))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;
    
    Ok(())
}

async fn serve_dashboard() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("viz_template.html"))
}

async fn network_data() -> HttpResponse {
    let nodes = vec![/* Real-time network data */];
    let edges = vec![/* Connection data */];
    
    HttpResponse::Ok().json(json!({
        "nodes": nodes,
        "edges": edges,
        "metrics": {
            "tps": 0.0,
            "latency": 0.0,
            "partition_status": "stable"
        }
    }))
} 