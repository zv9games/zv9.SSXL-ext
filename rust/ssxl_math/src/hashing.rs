
// ssxl_math/src/hashing.rs

/// # Hashing Utilities for SSXL-ext
///
/// This module provides **deterministic, collision-resistant** hashing functions
/// essential for identifying and retrieving procedural data.
///
/// Hashing is crucial for:
/// 1.  Generating unique, stable IDs for **world chunks** based on their coordinates.
/// 2.  Creating cache keys for **generated content** to ensure persistence and integrity.
use crate::primitives::SSXLResult;
use glam::I64Vec3;
use sha2::{Digest, Sha256};

/// Generates a **deterministic SHA-256 hash** for a given 3D chunk coordinate.
///
/// This hash is used as a unique identifier (a key) for world chunks, ensuring that
/// the same coordinate always yields the identical hash string, which is crucial
/// for cache lookups and procedural generation stability.
///
/// # Arguments
///
/// * `coords` - The 3D world coordinate of the chunk, using 64-bit integers (`I64Vec3`)
///              to support extremely large, dimension-agnostic worlds.
///
/// # Returns
///
/// A `SSXLResult<String>` containing the 64-character hexadecimal SHA-256 hash.
pub fn hash_chunk_coords(coords: I64Vec3) -> SSXLResult<String> {
    // 1. Serialize the coordinates into a canonical string format (e.g., "100:5:25").
    // This fixed format ensures deterministic input for the hash function.
    let coord_string = format!("{}:{}:{}", coords.x, coords.y, coords.z);

    // 2. Compute the SHA-256 hash.
    let mut hasher = Sha256::new();
    hasher.update(coord_string.as_bytes());
    let result = hasher.finalize();

    // 3. Convert the hash bytes into a lowercase 64-character hexadecimal string.
    Ok(format!("{:x}", result))
}

/// Generates a **deterministic SHA-256 hash** for a generic 64-bit data key.
///
/// This is typically used to create unique cache keys for generated content or
/// assets, prefixed with `"content_"` for easy identification in the cache system
/// (`ssxl_cache`).
///
/// # Arguments
///
/// * `data_key` - A generic `u64` identifier for the content (e.g., a seed,
///                a configuration ID, or a tile type index).
///
/// # Returns
///
/// A `SSXLResult<String>` containing the hash prefixed with `"content_"`.
pub fn hash_content_data(data_key: u64) -> SSXLResult<String> {
    // 1. Convert the u64 key into a string.
    let key_string = data_key.to_string();

    // 2. Compute the SHA-256 hash.
    let mut hasher = Sha256::new();
    hasher.update(key_string.as_bytes());
    let result = hasher.finalize();

    // 3. Format the result with the required "content_" prefix.
    Ok(format!("content_{:x}", result))
}


/// Unit tests to ensure the hashing functions are **deterministic** and produce
/// the expected **format** and **uniqueness** required for stable world generation.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests that the same coordinates always produce the identical hash (determinism).
    fn test_chunk_coords_determinism() {
        let coords = I64Vec3::new(3_000_000_000, -5, 100);
        let hash1 = hash_chunk_coords(coords).unwrap();
        let hash2 = hash_chunk_coords(coords).unwrap();

        assert_eq!(hash1, hash2, "Deterministic hashing failed: hashes must be identical for same input.");
    }

    #[test]
    /// Tests the expected hash length (64 chars) and confirms that a small coordinate change
    /// results in a completely different hash (uniqueness/avalanche effect).
    fn test_chunk_coords_uniqueness_and_format() {
        let coords1 = I64Vec3::new(1, 1, 1);
        let coords2 = I64Vec3::new(1, 1, 2); // Only Z differs
        let hash1 = hash_chunk_coords(coords1).unwrap();
        let hash2 = hash_chunk_coords(coords2).unwrap();

        assert_eq!(hash1.len(), 64, "Chunk hash is not the expected SHA-256 hex length (64).");
        assert_ne!(hash1, hash2, "Uniqueness failed: adjacent coordinates produced same hash.");
    }

    #[test]
    /// Tests determinism, the 'content_' prefix, and the total expected length (72 chars).
    fn test_content_data_determinism_and_format() {
        let key: u64 = 987654321;

        let hash1 = hash_content_data(key).unwrap();
        let hash2 = hash_content_data(key).unwrap();

        assert_eq!(hash1, hash2, "Content hash determinism failed.");

        assert!(hash1.starts_with("content_"), "Content hash is missing the 'content_' prefix.");
        // 'content_' (8 chars) + SHA-256 hex (64 chars) = 72
        assert_eq!(hash1.len(), 8 + 64, "Content hash is not the expected total length (72).");
    }
}