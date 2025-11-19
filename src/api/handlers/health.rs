use actix_web::HttpResponse;
use crate::models::response::HealthResponse;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: "running".to_string(),
    })
}
