// rust/SSXL-ext/src/tools.rs

use godot::prelude::*;
use std::time::Instant;

// ----------------------------------------------------
// 1. CUSTOM LOGGING MACROS (CONDITIONALLY COMPILED)
// ----------------------------------------------------

/// Prints a standard information message.
#[macro_export]
macro_rules! ssxl_info {
    ($($arg:tt)*) => ({
        // === CLI MOCK IMPLEMENTATION (Safe) ===
        #[cfg(feature = "ssxl_cli")]
        eprintln!("INFO [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!());
        
        // === GDExtension IMPLEMENTATION ===
        #[cfg(not(feature = "ssxl_cli"))]
        godot::prelude::godot_print!("INFO [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!());
    });
}

/// Prints a warning message.
#[macro_export]
macro_rules! ssxl_warn {
    ($($arg:tt)*) => ({
        // === CLI MOCK IMPLEMENTATION (Safe) ===
        #[cfg(feature = "ssxl_cli")]
        eprintln!("WARN [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!());
        
        // === GDExtension IMPLEMENTATION ===
        #[cfg(not(feature = "ssxl_cli"))]
        godot::prelude::godot_warn!("WARN [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!());
    });
}

/// Prints an error message.
#[macro_export]
macro_rules! ssxl_error {
    ($($arg:tt)*) => ({
        // === CLI MOCK IMPLEMENTATION (Safe) ===
        #[cfg(feature = "ssxl_cli")]
        eprintln!("ERROR [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!());
        
        // === GDExtension IMPLEMENTATION ===
        #[cfg(not(feature = "ssxl_cli"))]
        godot::prelude::godot_error!("ERROR [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!());
    });
}

// ----------------------------------------------------
// 2. COORDINATE UTILITIES
// ----------------------------------------------------

/// Trait for converting Rust coordinate types to Godot Vector2i.
pub trait ToGodotVector {
    fn to_godot_vector(&self) -> Vector2i;
}

// Implementation for the standard (i32, i32) tuple used for chunk positions.
impl ToGodotVector for (i32, i32) {
    /// Converts a (x, y) tuple into a Godot Vector2i.
    fn to_godot_vector(&self) -> Vector2i {
        Vector2i::new(self.0, self.1)
    }
}

// ----------------------------------------------------
// 3. PROFILER UTILITY
// ----------------------------------------------------

/// A simple struct for timing code execution blocks.
// 🔥 FIX: Added 'pub' to make the struct accessible to other modules
pub struct Profiler {
    start: Instant,
    name: &'static str,
    enabled: bool,
}

impl Profiler {
    /// Starts a new profiler instance if profiling is globally enabled.
    // 🔥 FIX: Added 'pub' to make the method accessible
    pub fn start(name: &'static str) -> Self {
        // NOTE: In a real project, 'is_profiling_enabled' would be read from config.rs
        const IS_PROFILING_ENABLED: bool = true;
        
        Profiler {
            start: Instant::now(),
            name,
            enabled: IS_PROFILING_ENABLED,
        }
    }
}

/// The Drop implementation automatically logs the duration when the scope is exited.
impl Drop for Profiler {
    fn drop(&mut self) {
        if self.enabled {
            let duration = self.start.elapsed();
            // This already correctly uses eprintln!
            eprintln!(
                "PERF [{}]: Execution time: {:.3}ms",
                self.name,
                duration.as_secs_f64() * 1000.0
            );
        }
    }
}