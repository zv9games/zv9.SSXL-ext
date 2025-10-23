use godot::prelude::*;
use godot::global::Error;

use crate::zv9_godot_interface_map_ext::MapDataChunkExt;
use crate::zv9_godot_interface_api_engine_core::AetherionEngine;
use aetherion_shared::shared::EngineMessage;

/// 📡 Emits all pending signals from engine and conductor into Godot.
pub fn emit_pending_signals(engine: &mut AetherionEngine) {
    // Drain signals first
    let mut signals = {
        let mut all = engine.sync_mut().drain_signals();
        if let Some(conductor) = engine.conductor_mut() {
            let conductor_signals = conductor.streamer_mut().delivery_mut().sync.drain_signals();
            all.extend(conductor_signals);
        }
        all
    };

    for signal_msg in signals.drain(..) {
        match signal_msg {
            EngineMessage::Status(status) => {
                // update engine state first
                engine.set_current_status(status.clone());

                // then borrow signals_node separately
                if let Some(signals_node) = engine.signals_node_mut() {
                    godot_print!("📡 Emitting 'map_building_status' → {}", status);
                    let result = signals_node.emit_signal(
                        "map_building_status",
                        &[GString::from(&status).to_variant()],
                    );
                    if result != Error::OK {
                        godot_warn!("⚠️ Signal emission failed: {:?}", result);
                    }
                }
            }
            EngineMessage::Progress(percent) => {
                if let Some(signals_node) = engine.signals_node_mut() {
                    godot_print!("📡 Emitting 'generation_progress' → {}%", percent);
                    let result = signals_node.emit_signal("generation_progress", &[percent.to_variant()]);
                    if result != Error::OK {
                        godot_warn!("⚠️ Signal emission failed: {:?}", result);
                    }
                }
            }
            EngineMessage::Complete { width, height, mode, animate, duration } => {
                if let Some(signals_node) = engine.signals_node_mut() {
                    godot_print!(
                        "📡 Emitting 'generation_complete' → {}x{}, mode={}, animate={}, duration={}",
                        width, height, mode, animate, duration
                    );
                    let mut dict = Dictionary::new();
                    dict.insert("width", width);
                    dict.insert("height", height);
                    dict.insert("mode", mode);
                    dict.insert("animate", animate);
                    dict.insert("duration", duration);
                    let result = signals_node.emit_signal("generation_complete", &[dict.to_variant()]);
                    if result != Error::OK {
                        godot_warn!("⚠️ Signal emission failed: {:?}", result);
                    }
                }
            }
            EngineMessage::ChunkReady(chunk) => {
                if let Some(signals_node) = engine.signals_node_mut() {
                    godot_print!("📡 Emitting 'chunk_ready'");
                    let dict = chunk.to_dictionary();
                    let result = signals_node.emit_signal("chunk_ready", &[dict.to_variant()]);
                    if result != Error::OK {
                        godot_warn!("⚠️ Signal emission failed: {:?}", result);
                    }
                }
            }
            _ => {}
        }
    }
}


pub fn apply_chunks_to_tilemap(engine: &mut AetherionEngine) {
    // Drain chunks first
    let chunks = engine.sync_mut().drain_chunks();

    if let Some(tilemap) = engine.target_tilemap_mut() {
        for chunk in chunks {
            godot_print!("🧱 Applying chunk with {} tiles", chunk.tiles.len());
            for (pos, tile_info) in chunk.tiles {
                tilemap
                    .set_cell_ex(0, Vector2i::new(pos.x, pos.y))
                    .source_id(tile_info.source_id)
                    .atlas_coords(Vector2i::new(tile_info.atlas_coords.x, tile_info.atlas_coords.y))
                    .alternative_tile(tile_info.alternate_id)
                    .done();
            }
        }
    } else {
        godot_warn!("⚠️ No TileMap assigned. Cannot apply chunks.");
    }
}


/// 🧪 Places a debug tile at the given coordinates.
pub fn debug_place_tile(engine: &mut AetherionEngine, x: i32, y: i32) {
    if let Some(tilemap) = engine.target_tilemap_mut() {
        godot_print!("🧪 Placing debug tile at ({}, {})", x, y);
        tilemap
            .set_cell_ex(0, Vector2i::new(x, y))
            .source_id(0)
            .atlas_coords(Vector2i::new(14, 13))
            .alternative_tile(0)
            .done();
    } else {
        godot_warn!("⚠️ No TileMap assigned. Cannot place debug tile.");
    }
}
