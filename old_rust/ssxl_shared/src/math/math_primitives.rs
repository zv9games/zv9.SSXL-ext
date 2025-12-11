// ============================================================================
// 🧮 Math Primitives & Serde Helpers
// File: ssxl_shared/src/math_primitives.rs
// ----------------------------------------------------------------------------
// Purpose:
//   - Provides custom serialization/deserialization logic for mathematical and
//     standard library types that require deterministic formatting.
//   - Ensures cross-platform compatibility and stable persistence when caching
//     or transmitting data across the SSXL engine ecosystem.
//
// Why it matters:
//   - Default Serde behavior for certain types (like SystemTime) can vary
//     depending on platform or implementation.
//   - By enforcing a canonical format (milliseconds since UNIX epoch), we
//     guarantee consistency across all crates (math, generate, cache, godot).
// ============================================================================

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serializer, Deserializer, Deserialize};

// -----------------------------------------------------------------------------
// ⏱️ Module: system_time_serde
// -----------------------------------------------------------------------------
// Custom Serde helpers for `std::time::SystemTime`.
// Converts SystemTime <-> u64 (milliseconds since epoch).
// This ensures:
//   - Deterministic representation (always the same format).
//   - Platform independence (no OS-specific quirks).
//   - Stable caching and networking (safe to persist and reload).
// -----------------------------------------------------------------------------
pub mod system_time_serde {
    use super::*;

    // -------------------------------------------------------------------------
    // Function: serialize
    // -------------------------------------------------------------------------
    // Converts a SystemTime into a u64 millisecond count since UNIX_EPOCH.
    // Steps:
    //   1. Compute duration since epoch.
    //   2. Convert duration to milliseconds.
    //   3. Serialize as u64.
    // Usage:
    //   - Called automatically by Serde when writing data structures containing
    //     SystemTime (e.g., saving ChunkData to disk or sending over network).
    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Duration since epoch; errors if time < UNIX_EPOCH.
        let duration = time.duration_since(UNIX_EPOCH).map_err(serde::ser::Error::custom)?;
        // Convert to milliseconds (u64).
        let ms = duration.as_millis() as u64;
        // Serialize the millisecond count.
        serializer.serialize_u64(ms)
    }

    // -------------------------------------------------------------------------
    // Function: deserialize
    // -------------------------------------------------------------------------
    // Converts a u64 millisecond count back into a SystemTime.
    // Steps:
    //   1. Deserialize u64 from input.
    //   2. Create Duration from milliseconds.
    //   3. Add Duration to UNIX_EPOCH to reconstruct SystemTime.
    // Usage:
    //   - Called automatically by Serde when reading data structures containing
    //     SystemTime (e.g., loading ChunkData from cache).
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Read the millisecond count.
        let ms = u64::deserialize(deserializer)?;
        // Convert back into SystemTime.
        Ok(UNIX_EPOCH + std::time::Duration::from_millis(ms))
    }
}
