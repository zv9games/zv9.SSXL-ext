// ssxl_godot/src/lib.rs

// 🛑 Declare sub-modules (existing)
pub mod ssxl_engine;
pub mod ssxl_signals;
pub mod ssxl_oracle;

// 🛠️ FIX: New module declarations required by ssxl_engine.rs
// These modules contain the delegate structs (AsyncPoller, ChunkPresenter, etc.).
mod async_poll;
mod chunk_presenter;
mod channel_handler;
mod api_initializers;
mod generation_api;
mod animation_api;

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