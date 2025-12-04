// ssxl_config/src/lib.rs
pub mod config;

// Re-export for simplified access
pub use config::config; 
pub use config::init_config; 
pub use config::SSXLConfig;