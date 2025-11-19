use actix_web::{web, HttpResponse, Error};
use std::collections::HashMap;
use std::sync::Mutex;
use chrono::Utc;

use crate::models::{binary::StoredBinary, response::ErrorResponse};

pub async fn download_binary(
    path: web::Path<String>,
    binary_store: web::Data<Mutex<HashMap<String, StoredBinary>>>,
) -> Result<HttpResponse, Error> {
    let binary_id = path.into_inner();
    
    let stored = {
        let store = binary_store.lock().unwrap();
        store.get(&binary_id).cloned()
    };
    
    match stored {
        Some(binary) => {
            // Check if expired
            if Utc::now() > binary.expires_at {
                log::warn!("Binary {} has expired", binary_id);
                return Ok(HttpResponse::Gone().json(ErrorResponse {
                    error: "Binary has expired".to_string(),
                    details: None,
                }));
            }
            
            match std::fs::read(&binary.path) {
                Ok(data) => {
                    log::info!("📥 Downloading binary: {} ({} bytes)", binary_id, data.len());
                    Ok(HttpResponse::Ok()
                        .content_type("application/octet-stream")
                        .insert_header(("Content-Disposition", format!("attachment; filename=\"merged_binary\"")))
                        .body(data))
                }
                Err(e) => {
                    log::error!("Failed to read binary {}: {}", binary_id, e);
                    Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Failed to read binary".to_string(),
                        details: Some(e.to_string()),
                    }))
                }
            }
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "Binary not found".to_string(),
                details: Some(format!("ID: {}", binary_id)),
            }))
        }
    }
}
