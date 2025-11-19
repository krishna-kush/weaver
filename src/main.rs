// Module declarations
mod api;
mod core;
mod models;
mod config;

use actix_web::{web, App, HttpServer, middleware};
use actix_multipart::form::MultipartFormConfig;
use std::sync::Mutex;
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let config = config::Config::from_env();
    
    log::info!("🕸️  Starting Weaver Binary Weaving Service");
    log::info!("📍 Listening on {}:{}", config.host, config.port);
    log::info!("📁 Temp directory: {}", config.temp_dir);
    
    let bind_addr = (config.host.clone(), config.port);
    
    // Shared state for storing merged binaries
    let binary_store = web::Data::new(Mutex::new(HashMap::<String, models::StoredBinary>::new()));
    let max_upload_size = config.max_file_size;
    let config_data = web::Data::new(config);
    
    HttpServer::new(move || {
        App::new()
            .app_data(MultipartFormConfig::default().total_limit(max_upload_size))
            .app_data(binary_store.clone())
            .app_data(config_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(api::configure_routes)
    })
    .bind(bind_addr)?
    .run()
    .await
}
