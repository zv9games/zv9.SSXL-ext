//! Core hashing utilities for converting coordinate data into unique identifiers (hashes).
//! These hashes are primarily used as stable keys for the cache and storage layers.

use crate::primitives::SSXLResult;
// 📐 BULLDOZER FIX: Changed IVec3 (i32) to I64Vec3 (i64) to support coordinates beyond 2.1 billion chunks.
use glam::I64Vec3; 
use sha2::{Digest, Sha256};

/// Generates a unique SHA-256 hash string for a given 3D integer coordinate (chunk position).
///
/// This hash is deterministic and collision-resistant, making it ideal for use as a
/// primary key in the cache or database for persistent data lookup.
///
/// # Arguments
///
/// * `coords` - The 3D integer coordinates (e.g., chunk index).
///
/// # Returns
///
/// A `String` containing the hexadecimal representation of the SHA-256 hash.
pub fn hash_chunk_coords(coords: I64Vec3) -> SSXLResult<String> {
    // 1. Format the coordinate string: "x:y:z"
    let coord_string = format!("{}:{}:{}", coords.x, coords.y, coords.z);

    // 2. Hash the string using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(coord_string.as_bytes());
    let result = hasher.finalize();

    // 3. Convert the hash result to a hexadecimal string
    Ok(format!("{:x}", result))
}

/// Generates a unique content hash for a piece of data (e.g., a ChunkData struct).
///
/// This is a placeholder function that will be fully implemented later once we have the
/// full `ChunkData` structure. For now, it returns a simple hash based on a u64 key.
///
/// # Arguments
///
/// * `data_key` - A unique identifier or version number for the data.
///
/// # Returns
///
/// A `String` containing a placeholder hash.
pub fn hash_content_data(data_key: u64) -> SSXLResult<String> {
    let key_string = data_key.to_string();
    let mut hasher = Sha256::new();
    hasher.update(key_string.as_bytes());
    let result = hasher.finalize();

    Ok(format!("content_{:x}", result))
}

// ---------------------------
// IMPL: Unit Tests
// ---------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the SHA-256 hashing of coordinates is strictly deterministic.
    #[test]
    fn test_chunk_coords_determinism() {
        // Test with a coordinate that exceeds i32 limits to prove i64 is active.
        let coords = I64Vec3::new(3_000_000_000, -5, 100); 
        let hash1 = hash_chunk_coords(coords).unwrap();
        let hash2 = hash_chunk_coords(coords).unwrap();
        
        assert_eq!(hash1, hash2, "Deterministic hashing failed: hashes must be identical for same input.");
    }

    /// Tests that adjacent coordinates produce different hashes and verifies the format.
    #[test]
    fn test_chunk_coords_uniqueness_and_format() {
        let coords1 = I64Vec3::new(1, 1, 1);
        let coords2 = I64Vec3::new(1, 1, 2); // Only Z differs
        let hash1 = hash_chunk_coords(coords1).unwrap();
        let hash2 = hash_chunk_coords(coords2).unwrap();

        // SHA-256 produces 64 characters in hex format
        assert_eq!(hash1.len(), 64, "Chunk hash is not the expected SHA-256 hex length (64).");
        assert_ne!(hash1, hash2, "Uniqueness failed: adjacent coordinates produced same hash.");
    }

    /// Tests that the content hash is deterministic and prepends the 'content_' prefix.
    #[test]
    fn test_content_data_determinism_and_format() {
        let key: u64 = 987654321;
        
        let hash1 = hash_content_data(key).unwrap();
        let hash2 = hash_content_data(key).unwrap();

        // Determinism check
        assert_eq!(hash1, hash2, "Content hash determinism failed.");

        // Format check
        assert!(hash1.starts_with("content_"), "Content hash is missing the 'content_' prefix.");
        // The SHA-256 hex part is 64 chars long. "content_" is 8 chars. Total: 72 chars.
        assert_eq!(hash1.len(), 8 + 64, "Content hash is not the expected total length (72).");
    }
}