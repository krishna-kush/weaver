use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .app_data(web::PayloadConfig::default().limit(500_000_000)) // 500MB global payload limit for large overload binaries
        .route("/health", web::get().to(handlers::health::health))
        .route("/merge", web::post().to(handlers::merge::merge_binaries))
        .route("/merge/stop-on-exit", web::post().to(handlers::merge_stop_on_exit::merge_stop_on_exit))
        .route("/merge/v2/stop-on-exit", web::post().to(handlers::merge_v2::merge_v2_stop_on_exit))
        .route("/download/{id}", web::get().to(handlers::download::download_binary));
}
