// ============================================================================
// 🎼 Task Module Declaration (`crate::task`)
// ----------------------------------------------------------------------------
// This `mod.rs` file defines the structure and public API surface of the Task
// system in the SSXL engine. It organizes all task-related functionality,
// including batch generation, benchmarking, and task queue management.
//
// Purpose:
//   • Serve as the "table of contents" for task-related subsystems.
//   • Expose submodules that implement chunk batch processing, benchmarking,
//     and queue orchestration.
//   • Re-export commonly used items to simplify external imports.
//
// Submodules:
//   • batch_processor
//       - Handles sequential batch generation of chunks.
//       - Ensures stability with non-thread-safe generators and caches.
//   • benchmark_logic
//       - Provides benchmarking utilities for workload simulation.
//       - Useful for stress testing and performance validation.
//   • task_queue
//       - Manages the queue of chunk generation tasks.
//       - Supports both async and sync execution flows.
//
// Re-exports:
//   • pub use batch_processor::*
//       - Exposes batch generation functions directly from `crate::task`.
//   • pub use benchmark_logic::*
//       - Exposes benchmarking utilities directly from `crate::task`.
//   • pub use task_queue::*
//       - Exposes task queue management functions directly from `crate::task`.
//
// Workflow:
//   1. External code imports from `crate::task`.
//   2. Submodules provide specialized functionality (batch, benchmark, queue).
//   3. Re-exports flatten the hierarchy for ergonomic access.
//   4. Conductor orchestrates tasks using these utilities.
//
// Design Choices:
//   • Modular organization keeps batch, benchmark, and queue logic isolated.
//   • Re-exports simplify the public API, reducing boilerplate in external code.
//   • Clear separation of concerns ensures maintainability and scalability.
//
// Educational Note:
//   • This file demonstrates how Rust’s module system can be used to organize
//     complex task orchestration into clean, modular components.
//   • By re-exporting, developers gain ergonomic access while preserving
//     internal modularity and clarity.
// ============================================================================


pub mod batch_processor;
pub mod benchmark_logic;
pub mod task_queue;

pub use batch_processor::*;
pub use benchmark_logic::*;
pub use task_queue::*;
