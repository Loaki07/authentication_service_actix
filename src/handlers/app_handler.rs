use actix_web::{web, web::ServiceConfig, HttpResponse};
use serde_json::json;

pub fn app_config(config: &mut ServiceConfig) {
    let health_resource = web::resource("/").route(web::get().to(health));

    config.service(health_resource);
}

pub async fn health() -> HttpResponse {
    let response = json!({ "success": true, "message": "Authentication Server" });
    HttpResponse::Ok().json(response)
}