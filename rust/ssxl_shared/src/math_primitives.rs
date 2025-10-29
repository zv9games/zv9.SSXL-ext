// ssxl_shared/src/math_primitives.rs
//! This module holds utility functions and constants needed by shared data structures,
//! primarily serialization helpers for standard library types.

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serializer, Deserializer, Deserialize};

// NOTE: CHUNK_SIZE_I32 and F32_EPSILON are removed from here.
// CHUNK_SIZE_I32 should be moved to aetherion_math/src/primitives.rs to break the cycle.
// F32_EPSILON is a generic math constant and should also live in aetherion_math.

// -----------------------------------------------------------------------------
// SERDE HELPERS (Remains in Shared, as it aids shared data structures)
// -----------------------------------------------------------------------------

/// Serde serialization and deserialization helpers for `std::time::SystemTime`.
///
/// This allows `SystemTime` to be used in structs with `#[derive(Serialize/Deserialize)]`
/// by converting it to and from a serializable `u64` (milliseconds since epoch).
pub mod system_time_serde {
    use super::*;

    /// Serializes `SystemTime` as milliseconds elapsed since the Unix epoch (u64).
    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH).map_err(serde::ser::Error::custom)?;
        let ms = duration.as_millis() as u64;
        serializer.serialize_u64(ms)
    }

    /// Deserializes milliseconds (u64) back into a `SystemTime` struct.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_millis(ms))
    }
}