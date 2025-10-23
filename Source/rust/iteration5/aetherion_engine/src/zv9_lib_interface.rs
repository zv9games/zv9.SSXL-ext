// ─── Godot Interface Modules ───────────────────────────────────────────────────

// API
pub mod zv9_godot_interface_api_config         { include!("zv9_godot_interface_api_config.rs"); }
pub mod zv9_godot_interface_api_generator      { include!("zv9_godot_interface_api_generator.rs"); }
pub mod zv9_godot_interface_api_map            { include!("zv9_godot_interface_api_map.rs"); }
pub mod zv9_godot_interface_api_oracle         { include!("zv9_godot_interface_api_oracle.rs"); }
pub mod zv9_godot_interface_api_signals        { include!("zv9_godot_interface_api_signals.rs"); }

// Bindings
pub mod zv9_godot_interface_bindings_godot_types { include!("zv9_godot_interface_bindings_godot_types.rs"); }

// Controls & Diagnostics
pub mod zv9_godot_interface_interface_controls    { include!("zv9_godot_interface_interface_controls.rs"); }
pub mod zv9_godot_interface_interface_diagnostics { include!("zv9_godot_interface_interface_diagnostics.rs"); }

// Messaging
pub mod zv9_godot_interface_messaging_messages    { include!("zv9_godot_interface_messaging_messages.rs"); }
pub mod zv9_godot_interface_messaging_sync        { include!("zv9_godot_interface_messaging_sync.rs"); }

// Signals
pub mod zv9_godot_interface_signals_definitions   { include!("zv9_godot_interface_signals_definitions.rs"); }
pub mod zv9_godot_interface_signals_dispatch      { include!("zv9_godot_interface_signals_dispatch.rs"); }

// ─── Godot Interface Re-exports ────────────────────────────────────────────────
pub mod interface {
    // API
    pub use crate::zv9_lib_interface::zv9_godot_interface_api_config::*;
    pub use crate::zv9_lib_interface::zv9_godot_interface_api_generator::*;
    pub use crate::zv9_lib_interface::zv9_godot_interface_api_map::*;
    pub use crate::zv9_lib_interface::zv9_godot_interface_api_oracle::*;
    pub use crate::zv9_lib_interface::zv9_godot_interface_api_signals::*;

    // Bindings
    //9pub use crate::zv9_lib_interface::zv9_godot_interface_bindings_godot_types::*;

    // Controls & Diagnostics
    pub use crate::zv9_lib_interface::zv9_godot_interface_interface_controls::*;
    pub use crate::zv9_lib_interface::zv9_godot_interface_interface_diagnostics::*;

    // Messaging
    pub use crate::zv9_lib_interface::zv9_godot_interface_messaging_messages::*;
    pub use crate::zv9_lib_interface::zv9_godot_interface_messaging_sync::*;

    // Signals
    pub use crate::zv9_lib_interface::zv9_godot_interface_signals_definitions::*;
    pub use crate::zv9_lib_interface::zv9_godot_interface_signals_dispatch::*;
}
