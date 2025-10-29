// ssxl_godot/src/lib.rs

// 🛑 Declare sub-modules (no change needed here)
pub mod ssxl_engine;
pub mod ssxl_signals;
pub mod ssxl_oracle;

use godot::prelude::*;
use godot::init::{ExtensionLibrary, InitLevel}; 

// --- GDEXTENSION ENTRY POINT ---

struct SSXLExtension;

// 🛑 FIX: Use the simple, stable, declarative ExtensionLibrary implementation.
// This is the correct signature for your version and relies on the #[derive(GodotClass)] 
// in the sub-modules to perform registration.
#[gdextension]
unsafe impl ExtensionLibrary for SSXLExtension {
    fn on_level_init(_level: InitLevel) {
        // Leave the body empty. Registration happens automatically via macros.
    }
}