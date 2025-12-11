// -----------------------------------------------------------------------------
// Module Declaration: ssxl_shared/src/config/mod.rs
// -----------------------------------------------------------------------------
// Purpose:
//   - This file acts as the *module root* for configuration logic in the
//     `ssxl_shared` crate.
//   - By declaring `pub mod config;`, it exposes the `config.rs` file as a
//     public submodule, making its types and functions available under
//     `ssxl_shared::config::...`.
//
// Why it matters:
//   - Keeps the crate organized: all configuration constants and structs live
//     in one place.
//   - Provides a clean namespace boundary, so other crates can import
//     `SSXLConfig` or constants without digging into file paths.
// -----------------------------------------------------------------------------
pub mod config;
