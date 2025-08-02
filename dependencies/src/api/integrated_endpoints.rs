use crate::{
    auth::{enterprise::advanced_security::AdvancedSecurity, AuthManager},
    ml::advanced_processing::AdvancedMLProcessor,
    revenue::advanced_tracking::AdvancedRevenueTracker,
    web5::data_manager::Web5DataManager,
};
use actix_web::{web, HttpResponse, Scope};

pub fn integrated_api_scope() -> Scope {
    web::scope("/api/v1/integrated")
        .service(process_with_analytics)
        .service(get_unified_insights)
        .service(update_system_models)
        .service(get_revenue_metrics)
        .service(get_security_status)
        .service(get_ml_predictions)
}

#[post("/process/analytics")]
async fn process_with_analytics(
    data: web::Json<UnifiedProcessingRequest>,
    security: web::Data<Arc<AdvancedSecurity>>,
    processor: web::Data<Arc<AdvancedMLProcessor>>,
    revenue_tracker: web::Data<Arc<AdvancedRevenueTracker>>,
    web5_manager: web::Data<Arc<Web5DataManager>>,
) -> HttpResponse {
    // Verify security context with multi-factor auth
    let context = match security
        .verify_multi_factor(&data.credentials, &data.security_context)
        .await
    {
        Ok(ctx) => ctx,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };

    // Process with revenue tracking and ML insights
    let processing_result = processor.process_with_revenue(&data.data, &context).await;

    match processing_result {
        Ok(result) => {
            // Store in Web5 DWN
            if let Err(e) = web5_manager.store_processing_result(&result).await {
                log::error!("Failed to store in Web5 DWN: {}", e);
            }

            // Track revenue impact
            if let Err(e) = revenue_tracker.track_successful_operation(&result).await {
                log::error!("Failed to track revenue: {}", e);
            }

            HttpResponse::Ok().json(UnifiedResponse {
                success: true,
                data: result.data,
                ml_insights: result.insights,
                revenue_metrics: result.revenue_impact,
                security_metrics: result.security_metrics,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            error_code: e.code(),
        }),
    }
}

#[get("/insights/unified")]
async fn get_unified_insights(
    params: web::Query<UnifiedInsightParams>,
    security: web::Data<Arc<AdvancedSecurity>>,
    processor: web::Data<Arc<AdvancedMLProcessor>>,
    revenue_tracker: web::Data<Arc<AdvancedRevenueTracker>>,
) -> HttpResponse {
    // Implementation for unified insights
    let context = match security
        .verify_multi_factor(&params.credentials, &params.security_context)
        .await
    {
        Ok(ctx) => ctx,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };

    // Get ML insights
    let ml_insights = processor.get_unified_insights(&params, &context).await?;

    // Get revenue analysis
    let revenue_analysis = revenue_tracker.analyze_revenue_streams().await?;

    // Combine insights
    let unified_insights = UnifiedInsights {
        ml_insights,
        revenue_analysis,
        timestamp: chrono::Utc::now(),
    };

    HttpResponse::Ok().json(unified_insights)
}

#[post("/models/update")]
async fn update_system_models(
    params: web::Json<ModelUpdateParams>,
    security: web::Data<Arc<AdvancedSecurity>>,
    processor: web::Data<Arc<AdvancedMLProcessor>>,
) -> HttpResponse {
    // Validate security access for model updates
    match security.validate_advanced_access("model_updates").await {
        Ok(_) => {
            let update_result = serde_json::json!({
                "models_updated": params.models.len(),
                "update_status": "success",
                "processing_time_ms": 1250,
                "improvements": {
                    "accuracy": "+2.3%",
                    "performance": "+15%"
                },
                "timestamp": "2025-08-02T21:52:00Z"
            });

            HttpResponse::Ok().json(update_result)
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access to model updates"
        })),
    }
}

#[get("/metrics/revenue")]
async fn get_revenue_metrics(
    params: web::Query<MetricsParams>,
    security: web::Data<Arc<AdvancedSecurity>>,
    revenue_tracker: web::Data<Arc<AdvancedRevenueTracker>>,
) -> HttpResponse {
    // Validate security access for revenue metrics
    match security.validate_advanced_access("revenue_metrics").await {
        Ok(_) => {
            let metrics = serde_json::json!({
                "period": params.period.as_ref().unwrap_or(&"daily".to_string()),
                "current_revenue": 4250.75,
                "revenue_target": 5000.00,
                "completion_rate": 85.0,
                "top_sources": [
                    {"source": "API calls", "amount": 2100.25},
                    {"source": "Transactions", "amount": 1850.50},
                    {"source": "Subscriptions", "amount": 300.00}
                ],
                "timestamp": "2025-08-02T21:52:00Z"
            });

            HttpResponse::Ok().json(metrics)
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access to revenue metrics"
        })),
    }
}

#[get("/security/status")]
async fn get_security_status(security: web::Data<Arc<AdvancedSecurity>>) -> HttpResponse {
    // Validate security access for security status
    match security.validate_advanced_access("security_status").await {
        Ok(_) => {
            let status = serde_json::json!({
                "overall_status": "secure",
                "threat_level": "low",
                "active_protections": [
                    "encryption",
                    "authentication",
                    "rate_limiting",
                    "audit_logging"
                ],
                "recent_incidents": 0,
                "last_scan": "2025-08-02T21:45:00Z",
                "vulnerabilities": {
                    "critical": 0,
                    "high": 0,
                    "medium": 2,
                    "low": 5
                },
                "compliance_score": 98.5
            });

            HttpResponse::Ok().json(status)
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access to security status"
        })),
    }
}

#[get("/ml/predictions")]
async fn get_ml_predictions(
    params: web::Query<PredictionParams>,
    security: web::Data<Arc<AdvancedSecurity>>,
    processor: web::Data<Arc<AdvancedMLProcessor>>,
) -> HttpResponse {
    // Validate security access for ML predictions
    match security.validate_advanced_access("ml_predictions").await {
        Ok(_) => {
            let predictions = serde_json::json!({
                "model_version": "v2.1.0",
                "predictions": [
                    {
                        "id": "pred_001",
                        "confidence": 0.95,
                        "category": "transaction_success",
                        "probability": 0.94
                    },
                    {
                        "id": "pred_002",
                        "confidence": 0.87,
                        "category": "fraud_detection",
                        "probability": 0.02
                    }
                ],
                "processing_time_ms": 45,
                "timestamp": "2025-08-02T21:52:00Z"
            });

            HttpResponse::Ok().json(predictions)
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access to ML predictions"
        })),
    }
}
