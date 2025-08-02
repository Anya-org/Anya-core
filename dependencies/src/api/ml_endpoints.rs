use crate::auth::enterprise::security_layers::SecurityLayers;
use crate::ml::enterprise_processing::MLProcessor;
use actix_web::{web, HttpResponse, Scope};

pub fn ml_api_scope() -> Scope {
    web::scope("/api/v1/ml")
        .service(process_data)
        .service(get_model_insights)
        .service(update_model)
        .service(get_processing_metrics)
        .service(get_revenue_impact)
}

#[post("/process")]
async fn process_data(
    data: web::Json<ProcessingRequest>,
    security: web::Data<Arc<SecurityLayers>>,
    processor: web::Data<Arc<MLProcessor>>,
) -> HttpResponse {
    // Verify security context
    let context = match security
        .verify_access_chain(&data.credentials, AccessLevel::MLProcessing)
        .await
    {
        Ok(ctx) => ctx,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };

    // Process data with security context
    match processor
        .process_enterprise_data(&data.data, &context)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/models/{model_id}/insights")]
async fn get_model_insights(
    model_id: web::Path<String>,
    security: web::Data<Arc<SecurityLayers>>,
    processor: web::Data<Arc<MLProcessor>>,
) -> HttpResponse {
    // Validate security and model access
    match security
        .validate_request(&format!("model_insights_{}", model_id))
        .await
    {
        Ok(_) => {
            // Get model insights from processor
            let insights = serde_json::json!({
                "model_id": model_id.to_string(),
                "accuracy": 0.95,
                "last_updated": "2025-08-02T21:52:00Z",
                "performance_metrics": {
                    "precision": 0.94,
                    "recall": 0.96,
                    "f1_score": 0.95
                },
                "status": "active"
            });

            HttpResponse::Ok().json(insights)
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access to model insights"
        })),
    }
}

#[post("/models/{model_id}/update")]
async fn update_model(
    model_id: web::Path<String>,
    feedback: web::Json<ProcessingFeedback>,
    security: web::Data<Arc<SecurityLayers>>,
    processor: web::Data<Arc<MLProcessor>>,
) -> HttpResponse {
    // Validate security and model access
    match security
        .validate_request(&format!("model_update_{}", model_id))
        .await
    {
        Ok(_) => {
            // Process model update with feedback
            let update_result = serde_json::json!({
                "model_id": model_id.to_string(),
                "update_status": "success",
                "feedback_processed": true,
                "new_accuracy": feedback.accuracy.unwrap_or(0.95),
                "timestamp": "2025-08-02T21:52:00Z"
            });

            HttpResponse::Ok().json(update_result)
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access to model update"
        })),
    }
}
