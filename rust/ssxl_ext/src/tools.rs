// rust/SSXL-ext/src/tools.rs

use godot::prelude::*;

/// Prints a standard information message to the Godot console.
#[macro_export]
macro_rules! ssxl_info {
    ($($arg:tt)*) => ({
        godot::prelude::godot_print!("INFO [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!())
    });
}

/// Prints a warning message to the Godot console.
#[macro_export]
macro_rules! ssxl_warn {
    ($($arg:tt)*) => ({
        godot::prelude::godot_warn!("WARN [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!())
    });
}

/// Prints an error message to the Godot console.
#[macro_export]
macro_rules! ssxl_error {
    ($($arg:tt)*) => ({
        godot::prelude::godot_error!("ERROR [SSXL]: {} ({}:{})", format!($($arg)*), file!(), line!())
    });
}

// rust/SSXL-ext/src/tools.rs

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

// Implementation for the shared_tile::TileData structure (if it holds coordinates)
// Or for a reference to the chunk position
// impl ToGodotVector for &ChunkPosition { ... }

// rust/SSXL-ext/src/tools.rs

use std::time::Instant;

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
            // Use standard print/error for micro-profiling to ensure data integrity
            eprintln!(
                "PERF [{}]: Execution time: {:.3}ms", 
                self.name, 
                duration.as_secs_f64() * 1000.0
            );
        }
    }
}

// Example usage:
// {
//     let _p = Profiler::start("CA Simulation Step");
//     // ... heavy computation here ...
// } // Duration logged automatically when _p goes out of scope.