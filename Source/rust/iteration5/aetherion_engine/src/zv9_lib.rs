/*use godot::prelude::*;
use godot_macros::gdextension;

#[allow(unused_imports)]
use crate::zv9_prelude::*;
mod zv9_lib_interface;

/// 📦 Version info
pub const VERSION: &str = "0.1.0";

/// 📦 Prelude
pub mod zv9_prelude {
    include!("zv9_prelude.rs");
}

/// 🧩 Macros
#[macro_use]
pub mod zv9_trailkeeper_macros {
    include!("zv9_trailkeeper_macros.rs");
}

/// 🧩 Centralized init hook
pub fn init_all() {
    log_event!(
        EventType::System,
        "engine",
        format!("Aetherion boot sequence started (v{})", VERSION)
    );
}

/// 📚 Modular includes
#[path = "zv9_lib_core.rs"]
mod zv9_lib_core;
pub use zv9_lib_core::*;

#[path = "zv9_lib_interface.rs"]
mod zv9_lib_interface;
pub use zv9_lib_interface::*;

#[path = "zv9_lib_util.rs"]
mod zv9_lib_util;
pub use zv9_lib_util::*;

#[path = "zv9_lib_trailkeeper.rs"]
mod zv9_lib_trailkeeper;
pub use zv9_lib_trailkeeper::*;

#[path = "zv9_godot_interface_emulator.rs"]
mod zv9_godot_interface_emulator;
pub use zv9_godot_interface_emulator::{
    FakeTileMap,
    TileMapInterface,
    test_generation_and_placement_cli,
};

/// 🧪 Tests
#[cfg(test)]
mod integration_tests {
    // Add test modules here
}

/// 🚀 Godot Extension Entry
struct AetherionEXT;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionEXT {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 Aetherion is summoned.");
        }
    }
}

/// 🔓 Re-exports for binary access

// Prelude
pub use zv9_prelude::*;

// Core
pub use crate::core::*;
pub use crate::core::runtime::start as start_runtime;
pub use Conductor;
pub use ProcCommand;
pub use start;

// Interface
pub use interface::*;
pub use GodotSync;

// Util
pub use util::*;

// Trailkeeper
pub use trailkeeper::*;

// Pipeline
pub use MapDataChunk;

pub mod pipeline_builder {
    pub mod bitmask {
        pub use crate::zv9_lib_core::zv9_aetherion_pipeline_builder_bitmask::*;
    }
}
*/