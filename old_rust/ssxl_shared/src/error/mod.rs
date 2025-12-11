// -----------------------------------------------------------------------------
// Module Declaration: ssxl_shared/src/error/mod.rs
// -----------------------------------------------------------------------------
// This file is the entry point for the `error` namespace in the `ssxl_shared` crate.
// By declaring `pub mod errors;`, it exposes the `errors.rs` submodule, which
// contains the canonical `SSXLError` enum and `SSXLResult` type alias.
// Purpose:
//   - Centralizes error handling for the engine.
//   - Provides a clean namespace so other crates can import errors via
//     `ssxl_shared::error::SSXLError` or `ssxl_shared::error::SSXLResult`.
// -----------------------------------------------------------------------------
pub mod errors;
