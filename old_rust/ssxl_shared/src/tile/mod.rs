// ============================================================================
// 🧩 Tile Module Index
// File: ssxl_shared/src/tile/mod.rs
// ----------------------------------------------------------------------------
// This file acts as the *entry point* for all tile-related code in the
// `ssxl_shared` crate. In Rust, a `mod.rs` file inside a directory serves as
// the "table of contents" for that directory, declaring which submodules
// belong to it.
//
// Why this matters:
//   - Keeps the crate organized: all tile logic is grouped under `tile/`.
//   - Provides a clean namespace boundary: external code can import
//     `ssxl_shared::tile::TileData` or `ssxl_shared::tile::TileType` without
//     needing to know the internal file layout.
//   - Makes it easier to extend: new tile-related modules can be added here
//     and automatically become part of the `tile` namespace.
// ============================================================================

// -----------------------------------------------------------------------------
// 📦 Submodule: tile_data
// -----------------------------------------------------------------------------
// Contains the definition of `TileData`, the core struct representing the
// state of a single tile in a chunk. This typically includes information
// such as:
//   - Tile type (floor, wall, water, etc.)
//   - Metadata (flags, properties, animation state)
//   - Serialization logic for saving/loading tile state.
//
// Usage example:
//   use ssxl_shared::tile::tile_data::TileData;
// -----------------------------------------------------------------------------
pub mod tile_data;

// -----------------------------------------------------------------------------
// 📦 Submodule: tile_type
// -----------------------------------------------------------------------------
// Defines the `TileType` enumeration, which categorizes tiles by their role
// in the world (e.g., Grass, Stone, Water, Empty). This provides a strongly
// typed way to reason about tiles instead of relying on raw integers or
// strings.
//
// Usage example:
//   use ssxl_shared::tile::tile_type::TileType;
// -----------------------------------------------------------------------------
pub mod tile_type;
