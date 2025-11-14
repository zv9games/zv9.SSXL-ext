// ssxl_shared/src/math_primitives.rs

//! # Math Primitives & Serde Helpers (`ssxl_shared::math_primitives`)
//!
//! This module contains custom logic, primarily Serde serialization/deserialization
//! functions, for mathematical or standard library types that require specific,
//! deterministic formatting (e.g., `SystemTime`) for cross-platform compatibility
//! and persistent caching.

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serializer, Deserializer, Deserialize};


/// Serde serialization/deserialization helper for `std::time::SystemTime`.
///
/// This custom serializer converts `SystemTime` to and from a `u64` representing
/// the time in milliseconds since the Unix epoch (1970-01-01). This guarantees
/// a **deterministic, platform-independent** representation for caching and networking.
pub mod system_time_serde {
    use super::*;

    /// Serializes a `SystemTime` into a `u64` representing milliseconds since epoch.
    /// This is used by Serde when writing data (e.g., saving a `ChunkData` to cache).
    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Calculate the duration from the UNIX_EPOCH to the current time.
        let duration = time.duration_since(UNIX_EPOCH).map_err(serde::ser::Error::custom)?;
        // Convert the duration to milliseconds (u64) to ensure a stable format.
        let ms = duration.as_millis() as u64;
        // Serialize the u64 millisecond count.
        serializer.serialize_u64(ms)
    }

    /// Deserializes a `u64` (milliseconds since epoch) back into a `SystemTime`.
    /// This is used by Serde when reading data (e.g., loading a `ChunkData` from cache).
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the u64 millisecond count.
        let ms = u64::deserialize(deserializer)?;
        // Convert the millisecond count into a Duration, then add it to UNIX_EPOCH.
        Ok(UNIX_EPOCH + std::time::Duration::from_millis(ms))
    }
}