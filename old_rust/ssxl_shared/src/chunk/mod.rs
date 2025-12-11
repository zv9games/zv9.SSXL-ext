// -----------------------------------------------------------------------------
// Chunk Module Overview
// -----------------------------------------------------------------------------
// This file serves as the **module root** for all chunk-related functionality
// in the `ssxl_shared` crate. It organizes and exposes submodules that define
// the data structures and utilities for handling chunks in the SSXL engine.
//
// Why this matters:
//   - Rust modules are hierarchical. Declaring `pub mod ...` here tells the compiler
//     to look for corresponding files (or directories) and include them as part of
//     the `chunk` namespace.
//   - By centralizing these declarations, we ensure that all chunk-related logic
//     is grouped together and can be accessed via `ssxl_shared::chunk::...`.
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Submodule: chunk_data
// -----------------------------------------------------------------------------
// Contains the **ChunkData** struct, which represents a single chunk of the world.
// Responsibilities:
//   - Stores chunk ID, bounds, tiles, dimension tag, and timestamp.
//   - Provides methods for tile access, insertion, and coordinate hashing.
//   - Acts as the atomic unit of procedural generation and caching.
// Usage:
//   - Accessed via `ssxl_shared::chunk::chunk_data::ChunkData`.
pub mod chunk_data;

// -----------------------------------------------------------------------------
// Submodule: grid_bounds
// -----------------------------------------------------------------------------
// Contains the **GridBounds** struct and supporting types (Coord2D).
// Responsibilities:
//   - Defines rectangular bounding boxes in world space.
//   - Provides utilities for size calculation and containment checks.
//   - Used by ChunkData to represent the spatial extent of a chunk.
// Usage:
//   - Accessed via `ssxl_shared::chunk::grid_bounds::GridBounds`.
pub mod grid_bounds;

// -----------------------------------------------------------------------------
// Summary
// -----------------------------------------------------------------------------
// Together, `chunk_data` and `grid_bounds` form the foundation of chunk management:
//   - `chunk_data` handles the contents and metadata of a chunk.
//   - `grid_bounds` defines the spatial boundaries of a chunk.
// This modular design keeps responsibilities clear and makes the engine easier
// to maintain, extend, and test.
// -----------------------------------------------------------------------------
