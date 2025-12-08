// ============================================================================
// 🎼 FFI Module (`crate::ffi`)
// ----------------------------------------------------------------------------
// This module defines the Foreign Function Interface (FFI) layer for the SSXL
// engine. It organizes submodules that handle communication between Rust and
// Godot, and re-exports key types for ergonomic access.
//
// Purpose:
//   • Provide a namespace (`ffi`) for all Rust ↔ Godot bridging logic.
//   • Encapsulate signal definitions and oracle utilities in dedicated submodules.
//   • Re-export commonly used types so external code can import them directly
//     from `ffi` without drilling down into submodules.
//
// Key Components:
//   • Submodules
//       - oracle.rs
//           • Defines the `SSXLOracle` struct.
//           • Encapsulates logic for predictive or query-based FFI operations.
//       - signals.rs
//           • Defines the `SSXLSignals` struct.
//           • Contains signal definitions emitted from Rust to Godot.
//           • Provides the bridge between engine events and Godot script callbacks.
//
//   • Re-exports
//       - `pub use oracle::SSXLOracle;`
//           • Makes `SSXLOracle` available directly via `crate::ffi::SSXLOracle`.
//       - `pub use signals::SSXLSignals;`
//           • Makes `SSXLSignals` available directly via `crate::ffi::SSXLSignals`.
//       - This improves ergonomics and keeps external imports clean.
//
// Example Usage:
//   • Instead of writing:
//       use crate::ffi::oracle::SSXLOracle;
//       use crate::ffi::signals::SSXLSignals;
//     You can simply write:
//       use crate::ffi::{SSXLOracle, SSXLSignals};
//
// Design Choices:
//   • Modular separation ensures clarity: oracle logic and signal definitions
//     remain independent but accessible under a unified namespace.
//   • Re-exports flatten the API surface, reducing verbosity in external code.
//   • Clear FFI boundaries make it easier to maintain Rust ↔ Godot integration.
//
// Educational Note:
//   • This module demonstrates how Rust projects can use submodules and
//     re-exports to create clean, ergonomic APIs. By centralizing FFI-related
//     logic under `ffi`, SSXL ensures that external consumers have a simple,
//     intuitive interface for interacting with engine signals and oracle logic.
// ============================================================================


pub mod oracle;
pub mod signals;

pub use oracle::SSXLOracle;
pub use signals::SSXLSignals;
