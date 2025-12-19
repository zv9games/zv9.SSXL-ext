use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Global registry of exported GDScript API methods.
pub static API_REGISTRY: Lazy<Mutex<Vec<String>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

/// Called by macros to register a method signature.
pub fn register_api(method: &str) {
    API_REGISTRY.lock().unwrap().push(method.to_string());
}

/// Returns a copy of the registered API list.
pub fn list_api() -> Vec<String> {
    API_REGISTRY.lock().unwrap().clone()
}

#[macro_export]
macro_rules! export_api {
    ($sig:expr) => {{
        $crate::api_registry::register_api($sig);
    }};
}
