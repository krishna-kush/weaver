use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub temp_dir: String,
    pub binary_expiration_hours: i64,
    pub cleanup_interval: u64,
    pub redis_url: String,
    pub main_server_url: String,
    pub max_file_size: usize,
    pub binary_ttl: i64,
    pub enable_qemu_testing: bool,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("WEAVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("WEAVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            temp_dir: env::var("WEAVER_TEMP_DIR").unwrap_or_else(|_| "/tmp/weaver".to_string()),
            binary_expiration_hours: env::var("WEAVER_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            cleanup_interval: env::var("WEAVER_CLEANUP_INTERVAL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            main_server_url: env::var("MAIN_SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
            max_file_size: env::var("WEAVER_MAX_SIZE")
                .unwrap_or_else(|_| "209715200".to_string())
                .parse()
                .unwrap_or(209715200),
            binary_ttl: env::var("WEAVER_BINARY_TTL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
            enable_qemu_testing: env::var("WEAVER_ENABLE_CROSS_HOST_TESTING")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
}
