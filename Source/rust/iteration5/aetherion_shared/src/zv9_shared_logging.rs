use log::{info, warn, error, debug};

/// Logs a debug-level message with a tag.
pub fn log_debug(tag: &str, message: &str) {
    debug!("[{}] {}", tag, message);
}

/// Logs a warning-level message with a tag.
pub fn log_warn(tag: &str, message: &str) {
    warn!("[{}] ⚠️ {}", tag, message);
}

/// Logs an error-level message with a tag.
pub fn log_error(tag: &str, message: &str) {
    error!("[{}] ❌ {}", tag, message);
}

/// Logs an info-level message with a tag.
pub fn log_info(tag: &str, message: &str) {
    info!("[{}] {}", tag, message);
}
