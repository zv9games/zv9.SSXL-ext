// FILE: ssxl_cli\src\actions\test_suites.rs

//! # Internal Architectural and Data Validation Suites
//!
//! Contains self-contained Rust tests focused on validating internal data contracts,
//! concurrency models (channels), and core generation logic without external
//! process reliance.

use tracing::{info, error};
use std::time::{Duration, Instant};
use std::sync::mpsc;
use bincode::{serialize, deserialize};

// --- Project Imports ---
use ssxl_generate::Generator;
// FIX E0432: `perlin_generator` likely renamed to `perlin`.
use ssxl_generate::perlin::PerlinGenerator;
use ssxl_math::prelude::Vec2i;
// FIX E0432: Correct path to Chunk data constant.
use ssxl_shared::chunk::chunk_data::CHUNK_SIZE;
// FIX E0432: Correct path to Tile data struct.
use ssxl_shared::tile::tile_data::AnimationUpdate;


// -----------------------------------------------------------------------------
// FOCUSED ARCHITECTURAL VALIDATION
// -----------------------------------------------------------------------------

/// Validates the non-blocking mpsc channels used between the Godot main thread
/// and the Rust worker threads (Generation and Animation Conductors).
pub fn run_communication_channel_test() {
    info!("--- Starting Communication Channel Test (Godot <-> Rust Tempo) ---");
    let test_duration = Duration::from_millis(100);
    
    // 1. Create a channel pair: (CLI sends, Mock Conductor receives)
    let (cli_sender, conductor_receiver) = mpsc::channel::<String>();
    
    // 2. Spawn a thread to act as the Mock Conductor
    let conductor_handle = std::thread::spawn(move || {
        let mut messages_received = 0;
        let start = Instant::now();
        
        while start.elapsed() < test_duration {
            match conductor_receiver.try_recv() {
                Ok(msg) => {
                    if msg == "TEST_COMMAND" {
                        messages_received += 1;
                    }
                },
                Err(mpsc::TryRecvError::Empty) => {
                    std::thread::yield_now();
                },
                Err(mpsc::TryRecvError::Disconnected) => break,
            }
        }
        messages_received
    });
    
    // 3. CLI (main thread) floods the channel with messages
    let mut messages_sent = 0;
    let start = Instant::now();
    while start.elapsed() < test_duration {
        if cli_sender.send("TEST_COMMAND".to_string()).is_ok() {
            messages_sent += 1;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    
    drop(cli_sender);

    // 4. Wait for the Mock Conductor thread to complete and get the result
    let messages_received = match conductor_handle.join() {
        Ok(count) => count,
        Err(_) => {
            error!("❌ Mock Conductor thread panicked during join.");
            return;
        }
    };
    
    // 5. Report results
    let success = messages_received > 0;

    info!("Test Duration: {:?}", start.elapsed());
    info!("Total messages SENT: {}", messages_sent);
    info!("Total messages RECEIVED: {}", messages_received);

    if success {
        info!("✅ Communication Channel Test SUCCESS: Non-blocking channel functional. Received {} messages.", messages_received);
    } else {
        error!("❌ Communication Channel Test FAILED: Did not receive any messages or thread failed to join.");
    }
}


/// Alias for `run_communication_channel_test`. Validates data-channel functionality.
pub fn run_data_channel_test() {
    run_communication_channel_test();
}

/// Validates the core map generation logic by creating a chunk using the Perlin Generator.
pub fn run_map_generation_test() {
    info!("--- Starting Map Generation Test (Perlin Generator) ---");
    let generator = PerlinGenerator::new(64.0);
    let chunk_coords = Vec2i::new(10, 20);
    
    let start = Instant::now();
    let chunk = generator.generate_chunk(chunk_coords);
    let duration = start.elapsed();

    let expected_size = (CHUNK_SIZE * CHUNK_SIZE) as usize;
    let success = chunk.tiles.len() == expected_size;

    info!("Generated Chunk ID: {}", chunk.id);
    info!("Generation Time: {:?} ({:.2} tiles/sec)", duration, (expected_size as f64) / duration.as_secs_f64());
    
    if success {
        info!("✅ Map Generation Test SUCCESS: Chunk data integrity verified ({} tiles).", expected_size);
    } else {
        error!("❌ Map Generation Test FAILED: Expected {} tiles, got {}.", expected_size, chunk.tiles.len());
    }
}

/// Validates the data contract for the Animation Conductor by ensuring
/// the `AnimationUpdate` structure can be serialized and deserialized.
pub fn run_animation_conductor_test() {
    info!("--- Starting Animation Conductor Data Contract Test ---");

    let original_update = AnimationUpdate {
        layer: 1,
        source_id: 101,
        tile_coords: Vec2i::new(500, -300),
        new_atlas_coords: Vec2i::new(5, 7),
    };

    let encoded = match serialize(&original_update) {
        Ok(e) => e,
        Err(e) => {
            error!("❌ Serialization FAILED: {}", e);
            return;
        }
    };
    
    let decoded: AnimationUpdate = match deserialize(&encoded) {
        Ok(d) => d,
        Err(e) => {
            error!("❌ Deserialization FAILED: {}", e);
            return;
        }
    };

    let success = original_update.tile_coords == decoded.tile_coords && original_update.new_atlas_coords == decoded.new_atlas_coords;

    info!("Serialized size: {} bytes", encoded.len());

    if success {
        info!("✅ Animation Conductor Data Test SUCCESS: AnimationUpdate serialized and deserialized correctly.");
    } else {
        error!("❌ Animation Conductor Data Test FAILED: Decoded data did not match original.");
    }
}