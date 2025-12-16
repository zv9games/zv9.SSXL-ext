// rust/SSXL-ext/src/tools.rs (FIXED)

// 🎯 CRITICAL FIX: Gate all Godot-related imports
#[cfg(feature = "godot-binding")]
use godot::prelude::*;

use std::time::Instant;

// ----------------------------------------------------
// 1. CUSTOM LOGGING MACROS (CONDITIONALLY COMPILED)
// ----------------------------------------------------

// NOTE: These macros are already correctly feature-gated internally.

/// Prints a standard information message.
#[macro_export]
macro_rules! ssxl_info {
    ($($arg:tt)*) => ({
        // === GDExtension IMPLEMENTATION ===
        #[cfg(feature = "godot-binding")]
        godot::prelude::godot_print!("INFO [SSXL]: {}", format!($($arg)*));
        
        // === CLI/STANDARD IMPLEMENTATION (Fallback) ===
        #[cfg(not(feature = "godot-binding"))]
        println!("INFO [SSXL]: {}", format!($($arg)*));
    });
}

/// Prints a warning message.
#[macro_export]
macro_rules! ssxl_warn {
    ($($arg:tt)*) => ({
        #[cfg(feature = "godot-binding")]
        godot::prelude::godot_warn!("WARN [SSXL]: {}", format!($($arg)*));
        
        #[cfg(not(feature = "godot-binding"))]
        eprintln!("WARN [SSXL]: {}", format!($($arg)*));
    });
}

/// Prints an error message.
#[macro_export]
macro_rules! ssxl_error {
    ($($arg:tt)*) => ({
        #[cfg(feature = "godot-binding")]
        godot::prelude::godot_error!("ERROR [SSXL]: {}", format!($($arg)*));
        
        #[cfg(not(feature = "godot-binding"))]
        eprintln!("ERROR [SSXL]: {}", format!($($arg)*));
    });
}

// ----------------------------------------------------
// 2. COORDINATE UTILITIES
// ----------------------------------------------------

// 🎯 CRITICAL FIX: Define a mock type for the CLI build (where Vector2i is missing).
// This is necessary because 'ToGodotVector' is not gated.
#[cfg(not(feature = "godot-binding"))]
pub type Vector2i = (i32, i32);


/// Trait for converting Rust coordinate types to Godot Vector2i.
// NOTE: Since this trait is used across the crate (impls are below) but its function 
// only makes sense in Godot, we keep the trait declaration ungated but the implementation gated.
pub trait ToGodotVector {
    fn to_godot_vector(&self) -> Vector2i;
}

// 🎯 CRITICAL FIX: Gate the implementation, as it uses Godot's Vector2i::new.
// If this is not gated, the compiler gets E0433: use of undeclared type `Vector2i`.
#[cfg(feature = "godot-binding")]
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
pub struct Profiler {
    start: Instant,
    name: &'static str,
    enabled: bool,
}

impl Profiler {
    /// Starts a new profiler instance if profiling is globally enabled.
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
            // This already correctly uses eprintln! and does not require macro conditionalization.
            eprintln!(
                "PERF [{}]: Execution time: {:.3}ms",
                self.name,
                duration.as_secs_f64() * 1000.0
            );
        }
    }
}