// ============================================================================
// 🔐 Hashing Utilities (`crate::utils::hashing`)
// ----------------------------------------------------------------------------
// This module defines utility functions for generating deterministic,
// collision-resistant SHA-256 hashes for chunk coordinates and content data.
// These hashes serve as unique identifiers in caching, procedural generation,
// and content management systems.
//
// Purpose:
//   • Provide a reliable way to uniquely identify chunks in infinite world space.
//   • Generate consistent hashes for content keys used in caching or asset lookup.
//   • Ensure determinism: identical inputs always yield identical hashes.
//   • Ensure uniqueness: adjacent or different inputs yield distinct hashes.
//
// Key Functions:
//   • hash_chunk_coords(coords: I64Vec3) -> SSXLResult<String>
//       - Arguments:
//           • coords: 3D chunk coordinates in world space (I64Vec3).
//       - Behavior:
//           • Serializes coordinates into canonical string format: "x:y:z".
//           • Feeds the string into a SHA-256 hasher.
//           • Produces a 64-character lowercase hexadecimal string.
//       - Returns:
//           • SSXLResult<String> containing the hash.
//       - Use Cases:
//           • Unique identifiers for chunks in procedural generation.
//           • Cache keys for chunk data.
//
//   • hash_content_data(data_key: u64) -> SSXLResult<String>
//       - Arguments:
//           • data_key: numeric identifier for content (e.g., asset ID, seed).
//       - Behavior:
//           • Converts the key into a string.
//           • Feeds the string into a SHA-256 hasher.
//           • Produces a 64-character lowercase hexadecimal string.
//           • Prefixes the result with "content_" for easy identification.
//       - Returns:
//           • SSXLResult<String> containing the prefixed hash.
//       - Use Cases:
//           • Content caching and lookup.
//           • Asset or configuration uniqueness checks.
//
// Tests:
//   • test_chunk_coords_determinism
//       - Verifies identical coordinates always yield identical hashes.
//   • test_chunk_coords_uniqueness_and_format
//       - Verifies adjacent coordinates yield different hashes.
//       - Ensures hash length is exactly 64 characters.
//   • test_content_data_determinism_and_format
//       - Verifies identical content keys yield identical hashes.
//       - Ensures prefix "content_" is present.
//       - Ensures total length is 72 characters (8 prefix + 64 hash).
//
// Design Choices:
//   • SHA-256 chosen for collision resistance and deterministic output.
//   • Hexadecimal encoding ensures compact, human-readable identifiers.
//   • Prefixing content hashes improves clarity in cache systems.
//   • Wrapping results in SSXLResult maintains consistency with engine-wide error handling.
//
// Educational Note:
//   • This module demonstrates how cryptographic hashing can be applied to
//     procedural generation and content management. By using SHA-256, the
//     engine guarantees both determinism and uniqueness, critical for large-scale
//     worlds and distributed caching systems.
// ============================================================================


use crate::primitives::SSXLResult;
use glam::I64Vec3;
use sha2::{Digest, Sha256};

pub fn hash_chunk_coords(coords: I64Vec3) -> SSXLResult<String> {
    let coord_string = format!("{}:{}:{}", coords.x, coords.y, coords.z);

    let mut hasher = Sha256::new();
    hasher.update(coord_string.as_bytes());
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

pub fn hash_content_data(data_key: u64) -> SSXLResult<String> {
    let key_string = data_key.to_string();

    let mut hasher = Sha256::new();
    hasher.update(key_string.as_bytes());
    let result = hasher.finalize();

    Ok(format!("content_{:x}", result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_coords_determinism() {
        let coords = I64Vec3::new(3_000_000_000, -5, 100);
        let hash1 = hash_chunk_coords(coords).unwrap();
        let hash2 = hash_chunk_coords(coords).unwrap();

        assert_eq!(hash1, hash2, "Deterministic hashing failed: hashes must be identical for same input.");
    }

    #[test]
    fn test_chunk_coords_uniqueness_and_format() {
        let coords1 = I64Vec3::new(1, 1, 1);
        let coords2 = I64Vec3::new(1, 1, 2);
        let hash1 = hash_chunk_coords(coords1).unwrap();
        let hash2 = hash_chunk_coords(coords2).unwrap();

        assert_eq!(hash1.len(), 64, "Chunk hash is not the expected SHA-256 hex length (64).");
        assert_ne!(hash1, hash2, "Uniqueness failed: adjacent coordinates produced same hash.");
    }

    #[test]
    fn test_content_data_determinism_and_format() {
        let key: u64 = 987654321;

        let hash1 = hash_content_data(key).unwrap();
        let hash2 = hash_content_data(key).unwrap();

        assert_eq!(hash1, hash2, "Content hash determinism failed.");

        assert!(hash1.starts_with("content_"), "Content hash is missing the 'content_' prefix.");
        assert_eq!(hash1.len(), 8 + 64, "Content hash is not the expected total length (72).");
    }
}
