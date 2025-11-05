// ssxl_sync/src/lib.rs

//! Provides the core synchronization and communication channels for the engine,
//! allowing data exchange between the main thread, Godot, and worker threads.

// Declare the new modules containing the split logic.
pub mod primitives;
mod pool_manager;

// Re-export core synchronization items.
pub use primitives::AtomicResource;
pub use primitives::{create_sync_channel, start_sync_worker};

// Re-export the worker conductor types.
pub use primitives::{AnimationConductor, AnimationCommand};

// FIX: Re-export the type aliases required by external crates (like ssxl_godot).
// These were defined in primitives.rs in the previous step.
pub use primitives::AnimationConductorHandle; 
pub use primitives::AnimationReceiver; 

// NOTE: We may need to re-export AnimationUpdate from ssxl_shared if the full crate structure is checked.