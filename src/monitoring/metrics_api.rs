use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use serde_json::json;

use crate::monitoring::blockchain_metrics;
use crate::monitoring::blockchain_alerts;
use crate::monitoring::metrics;

/// Create metrics API routes
pub fn create_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let metrics_route = warp::path!("metrics")
        .and(warp::get())
        .map(handle_metrics);
    
    let prometheus_route = warp::path!("metrics" / "prometheus")
        .and(warp::get())
        .map(handle_prometheus);
    
    let blockchain_metrics_route = warp::path!("metrics" / "blockchain")
        .and(warp::get())
        .map(handle_blockchain_metrics);
    
    let blockchain_historical_route = warp::path!("metrics" / "blockchain" / "historical" / String)
        .and(warp::get())
        .map(handle_blockchain_historical);
    
    let alerts_route = warp::path!("metrics" / "alerts")
        .and(warp::get())
        .map(handle_alerts);
    
    let alert_history_route = warp::path!("metrics" / "alerts" / "history")
        .and(warp::get())
        .map(handle_alert_history);
    
    let acknowledge_alert_route = warp::path!("metrics" / "alerts" / "acknowledge" / String)
        .and(warp::post())
        .map(handle_acknowledge_alert);
    
    metrics_route
        .or(prometheus_route)
        .or(blockchain_metrics_route)
        .or(blockchain_historical_route)
        .or(alerts_route)
        .or(alert_history_route)
        .or(acknowledge_alert_route)
}

/// Handle basic metrics endpoint
async fn handle_metrics() -> impl Reply {
    // Return all metrics as JSON
    let blockchain = blockchain_metrics::get_metrics_json();
    
    warp::reply::json(&json!({
        "status": "success",
        "data": {
            "blockchain": blockchain
        }
    }))
}

/// Handle Prometheus format metrics endpoint
async fn handle_prometheus() -> impl Reply {
    // Get Prometheus format metrics
    let metrics_text = metrics::export_metrics();
    
    warp::reply::with_header(
        metrics_text,
        "content-type",
        "text/plain; version=0.0.4",
    )
}

/// Handle blockchain metrics endpoint
async fn handle_blockchain_metrics() -> impl Reply {
    // Return blockchain metrics as JSON
    let blockchain = blockchain_metrics::get_metrics_json();
    
    warp::reply::json(&json!({
        "status": "success",
        "data": blockchain
    }))
}

/// Handle blockchain historical metrics endpoint
async fn handle_blockchain_historical(metric_name: String) -> impl Reply {
    // Get historical data for the requested metric
    let historical_data = blockchain_metrics::get_historical_data(&metric_name);
    
    match historical_data {
        Some(data) => {
            warp::reply::json(&json!({
                "status": "success",
                "metric": metric_name,
                "data": data
            }))
        }
        None => {
            warp::reply::json(&json!({
                "status": "error",
                "message": format!("Historical data not available for metric: {}", metric_name)
            }))
        }
    }
}

/// Handle alerts endpoint
async fn handle_alerts() -> impl Reply {
    // Get active alerts
    let active_alerts = blockchain_alerts::get_active_alerts();
    
    warp::reply::json(&json!({
        "status": "success",
        "count": active_alerts.len(),
        "data": active_alerts
    }))
}

/// Handle alert history endpoint
async fn handle_alert_history() -> impl Reply {
    // Get alert history
    let history = blockchain_alerts::get_alert_history();
    
    warp::reply::json(&json!({
        "status": "success",
        "count": history.len(),
        "data": history
    }))
}

/// Handle alert acknowledgement endpoint
async fn handle_acknowledge_alert(alert_id: String) -> impl Reply {
    // Acknowledge the alert
    blockchain_alerts::acknowledge_alert(&alert_id);
    
    warp::reply::json(&json!({
        "status": "success",
        "message": format!("Alert {} acknowledged", alert_id)
    }))
}
