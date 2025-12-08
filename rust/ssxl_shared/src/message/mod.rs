// ============================================================================
// 📡 Message Module Index
// File: ssxl_shared/src/message/mod.rs
// ----------------------------------------------------------------------------
// This file serves as the *entry point* for all messaging-related code in the
// `ssxl_shared` crate. It organizes submodules and re-exports key types so that
// other parts of the engine (and external crates like `ssxl_godot`) can access
// messaging functionality through a clean, consistent API surface.
//
// Why this matters:
//   - Keeps the crate structure organized: `generation_message.rs` handles
//     worker-to-conductor communication, while `messages.rs` defines animation
//     and generation command types.
//   - Provides a single place to re-export commonly used types, avoiding deep
//     import paths in downstream code.
//   - Ensures that external modules can compile without unresolved import errors
//     (e.g., E0432 in `ssxl_godot/api_initializers.rs`).
// ============================================================================

pub mod generation_message; // Defines messages returned from generation workers
pub mod messages;           // Defines animation and generation command structures

// -----------------------------------------------------------------------------
// 🔗 Public Re-exports
// -----------------------------------------------------------------------------
// The following types are re-exported from `messages.rs` so that other crates
// can import them directly from `ssxl_shared::message`.
//
// Benefits:
//   - Simplifies usage: external code can write `use ssxl_shared::message::AnimationCommand;`
//     instead of navigating into `messages::AnimationCommand`.
//   - Provides a stable, curated API surface for messaging-related types.
// -----------------------------------------------------------------------------
pub use messages::{
    // Newly added re-exports to resolve current import errors
    AnimationCommand,
    AnimationState,
    
    // Existing re-exports for generation and animation updates
    AnimationUpdate,
    GenerationCommand, 
    GenerationResponse,
};
