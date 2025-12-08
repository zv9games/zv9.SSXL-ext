// ============================================================================
// 🎼 Conductor Module Declaration (`crate::conductor`)
// ----------------------------------------------------------------------------
// This `mod.rs` file defines the structure and public API surface of the
// Conductor system. It organizes submodules and re-exports key items so that
// other parts of the crate can easily access them without needing to know the
// internal layout.
//
// Purpose:
//   • Serve as the "table of contents" for the Conductor system.
//   • Expose submodules that implement conductor logic, state management,
//     synchronization, setup, and internal utilities.
//   • Re-export commonly used items to simplify external imports.
//
// Submodules:
//   • conductor
//       - Core Conductor struct and its implementation.
//       - Provides constructors, lifecycle controls, and FFI integration.
//   • conductor_state
//       - Defines lifecycle states (Running, Stopping, etc.).
//       - Provides thread-safe state container for queue depth, generator ID,
//         and tile counters.
//   • sync
//       - Synchronization utilities for atomic resource sharing.
//       - Ensures safe concurrent access across async tasks.
//   • builder
//       - Setup and spawn logic for initializing conductor internals.
//       - Prepares channels, managers, and state before runtime starts.
//   • sync_get
//       - Helper functions for safely retrieving synchronized values.
//       - Improves ergonomics when working with atomic resources.
//   • internal_setup
//       - Defines `ConductorInternalSetup`, a bundle of channels, state, and
//         managers used during initialization.
//
// Re-exports:
//   • Conductor
//       - The main struct orchestrating chunk generation and lifecycle control.
//   • conductor_state::*
//       - Re-exports all items from `conductor_state` (status enum, state struct).
//   • sync::*
//       - Re-exports synchronization utilities for convenience.
//
// Workflow:
//   1. External code imports from `crate::conductor`.
//   2. Submodules provide specialized functionality (state, sync, setup).
//   3. Re-exports simplify usage by exposing key items directly.
//   4. Conductor orchestrates runtime, generators, and communication channels.
//
// Educational Note:
//   • This file demonstrates how Rust’s module system can be used to organize
//     complex subsystems into clear, modular components.
//   • By re-exporting key items, the public API remains clean and ergonomic,
//     while internal organization stays modular and maintainable.
// ============================================================================


pub mod conductor;
pub mod conductor_state;
pub mod sync;
pub mod builder;
pub mod sync_get;
pub mod internal_setup;

pub use conductor::Conductor;
pub use conductor_state::*;
pub use sync::*;
