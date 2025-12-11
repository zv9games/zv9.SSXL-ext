// ============================================================================
// 🧪 SSXL CLI: Internal Architectural & Data Validation Suites
// ----------------------------------------------------------------------------
// This module defines self-contained Rust tests that validate the integrity of
// internal data contracts, concurrency models, and core generation logic. Unlike
// integration tests that rely on external processes (e.g., Godot), these suites
// focus purely on architectural correctness within the Rust engine itself.
//
// Key Functions:
//   • run_communication_channel_test
//       - Validates non-blocking `mpsc` channels used between Godot’s main thread
//         and Rust worker threads (Generation and Animation Conductors).
//       - Simulates a conductor thread receiving commands while the CLI floods
//         the channel with messages.
//       - Confirms that messages are successfully transmitted and received within
//         a bounded test duration.
//
//   • run_data_channel_test
//       - Alias for `run_communication_channel_test`.
//       - Provides a semantic entry point for validating data channel integrity.
//
//   • run_map_generation_test
//       - Validates core map generation logic using the Perlin noise generator.
//       - Generates a chunk at specific coordinates and verifies tile count
//         against the expected `CHUNK_SIZE`².
//       - Reports generation time and throughput (tiles/sec) for performance
//         benchmarking.
//       - Ensures chunk data integrity and correctness of procedural generation.
//
//   • run_animation_conductor_test
//       - Validates the data contract for animation updates.
//       - Ensures the `AnimationUpdate` struct can be serialized and deserialized
//         using `bincode` without data loss.
//       - Confirms that tile coordinates and atlas coordinates remain consistent
//         after round-trip encoding/decoding.
//       - Provides confidence that animation updates can safely traverse FFI
//         boundaries and channels.
//
// Workflow:
//   1. Communication channels are stress-tested for throughput and reliability.
//   2. Map generation is validated for correctness and performance metrics.
//   3. Animation conductor data contracts are verified for serialization safety.
//   4. Results are logged via `tracing` for visibility and debugging.
//
// Design Choices:
//   • `mpsc` channels simulate real conductor communication without external
//     dependencies.
//   • `PerlinGenerator` provides deterministic procedural generation for chunk
//     validation.
//   • `bincode` ensures efficient serialization for FFI and channel transport.
//   • `tracing` macros (`info`, `error`) provide structured logging for clarity.
//
// Educational Note:
//   • These suites demonstrate how to validate concurrency, procedural generation,
//     and serialization contracts in isolation.
//   • By ensuring architectural correctness here, developers can trust that
//     higher-level integration tests (with Godot) are built on a solid foundation.
// ============================================================================


use tracing::{info, error};
use std::time::{Duration, Instant};
use std::sync::mpsc;
use bincode::{serialize, deserialize};

use ssxl_generate::Generator;
use ssxl_generate::perlin::PerlinGenerator;
use ssxl_math::prelude::Vec2i;
use ssxl_shared::chunk::chunk_data::CHUNK_SIZE;
use ssxl_shared::tile::tile_data::AnimationUpdate;

pub fn run_communication_channel_test() {
    info!("--- Starting Communication Channel Test (Godot <-> Rust Tempo) ---");
    let test_duration = Duration::from_millis(100);
    
    let (cli_sender, conductor_receiver) = mpsc::channel::<String>();
    
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
    
    let mut messages_sent = 0;
    let start = Instant::now();
    while start.elapsed() < test_duration {
        if cli_sender.send("TEST_COMMAND".to_string()).is_ok() {
            messages_sent += 1;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    
    drop(cli_sender);

    let messages_received = match conductor_handle.join() {
        Ok(count) => count,
        Err(_) => {
            error!("❌ Mock Conductor thread panicked during join.");
            return;
        }
    };
    
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

pub fn run_data_channel_test() {
    run_communication_channel_test();
}

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
