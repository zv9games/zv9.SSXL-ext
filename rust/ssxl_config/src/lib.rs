// ============================================================================
// 📦 SSXL Configuration Module (`ssxl_config::lib`)
// ----------------------------------------------------------------------------
// This file defines the public API surface for the SSXL configuration crate.
// It declares the `config` submodule and re-exports key items so that other
// parts of the engine (or external crates) can access configuration logic
// directly from the crate root.
//
// Purpose:
//   • Provide a clean, centralized entry point for configuration management.
//   • Simplify usage by re-exporting commonly accessed items.
//   • Ensure that initialization and global access functions are available
//     without requiring deep module paths.
//
// Key Components:
//   • pub mod config
//       - Declares the `config` submodule.
//       - Contains the implementation of configuration structs, initialization,
//         and global state management.
//
//   • pub use config::config
//       - Re-exports the global accessor function `config()`.
//       - Allows consumers to call `ssxl_config::config()` directly.
//
//   • pub use config::init_config
//       - Re-exports the initialization function.
//       - Ensures configuration can be set up once at application startup.
//
//   • pub use config::SSXLConfig
//       - Re-exports the main configuration struct.
//       - Provides direct access to the strongly typed configuration object.
//
// Workflow:
//   1. The application calls `init_config()` to initialize global configuration.
//   2. Subsystems access configuration via `ssxl_config::config()`.
//   3. Developers can work with the `SSXLConfig` struct directly for debugging,
//      serialization, or extension.
//
// Design Choices:
//   • Re-exports reduce friction by avoiding verbose paths like
//     `use ssxl_config::config::SSXLConfig;`.
//   • Public module declaration ensures the configuration logic is modular
//     and can evolve independently.
//   • Clean API surface improves readability and maintainability.
//
// Educational Note:
//   • This pattern demonstrates how Rust crates can expose a simplified,
//     ergonomic API by re-exporting key items from internal modules.
//   • It balances modular organization with developer convenience.
// ============================================================================


pub mod config;

pub use config::config;
pub use config::init_config;
pub use config::SSXLConfig;
