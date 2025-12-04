// FILE: ssxl_shared/src/message/mod.rs

pub mod generation_message;
pub mod messages;

// FIX: Publicly re-export all necessary message and state types from the messages submodule.
// This resolves the unresolved imports (E0432) in ssxl_godot/api_initializers.rs.
pub use messages::{
    // New re-exports to fix current error
    AnimationCommand,
    AnimationState,
    
    // Existing re-exports
    AnimationUpdate,
    GenerationCommand, 
    GenerationResponse,
};