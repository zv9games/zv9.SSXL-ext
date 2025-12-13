// rust/SSXL-ext/src/host_anim.rs

use godot::prelude::*;
use godot::classes::{TileMap, TileMapLayer}; // Added TileMapLayer for the mock
use crate::animate_events::AnimationEvent;
use crate::animate_conductor::AnimationConductor;
use crate::host_state::GodotError;
use crate::{ssxl_error, ssxl_warn, ssxl_info}; // 🔥 FIX 1: Import custom logger macros

// Define a safe maximum number of animation events to process per frame
const MAX_ANIM_EVENTS_PER_FRAME: u32 = 256; 

/// Polls the animation event channel and applies visual updates to the Godot world.
/// This must be called on the Godot main thread.
pub fn apply_animation_updates(conductor: &AnimationConductor, tilemap_id: InstanceId) -> u32 {
    let mut events_processed = 0;
    
    // Use a try_recv loop with a frame budget to ensure non-blocking execution
    while events_processed < MAX_ANIM_EVENTS_PER_FRAME {
        
        // 1. Receive the next completed event from the worker threads
        match conductor.event_receiver.try_recv() {
            Ok(event) => {
                // 2. Apply the visual change based on the event type
                if let Err(e) = handle_animation_event(tilemap_id, event) {
                    // 🔥 FIX 2: Replaced godot_error! with ssxl_error!
                    ssxl_error!("Host Anim: Failed to handle event: {:?}", e);
                }
                events_processed += 1;
            },
            Err(flume::TryRecvError::Empty) => {
                // No more events this frame
                break;
            },
            Err(flume::TryRecvError::Disconnected) => {
                // 🔥 FIX 3: Replaced godot_warn! with ssxl_warn!
                ssxl_warn!("Host Anim: Conductor event channel disconnected.");
                break;
            }
        }
    }
    
    events_processed
}

/// Dispatches the event to the appropriate Godot TileMap or Node API call.
fn handle_animation_event(tilemap_id: InstanceId, event: AnimationEvent) -> Result<(), GodotError> {
    
    // We assume the tilemap_id can be safely resolved to a Gd<TileMap>
    let tilemap_node = InstanceId::get_object(tilemap_id)
        .ok_or(GodotError::InvalidInstance)?
        .cast::<TileMap>();

    // Temporarily handling the unwrap() if the cast fails. In production this should be handled.
    let mut tilemap = tilemap_node.unwrap(); 

    // Mock implementation of to_godot_vector() and set_cell_tiledata_value()
    trait MockTileMapMethods {
        fn to_godot_vector(&self) -> godot::builtin::Vector2i;
        fn set_cell_tiledata_value(&mut self, layer: i32, coords: godot::builtin::Vector2i, data_id: TileMapLayer, value: Variant);
    }
    
    impl MockTileMapMethods for (i32, i32) {
        fn to_godot_vector(&self) -> godot::builtin::Vector2i {
            godot::builtin::Vector2i::new(self.0, self.1)
        }
    }
    
    impl MockTileMapMethods for Gd<TileMap> {
        fn to_godot_vector(&self) -> godot::builtin::Vector2i {
            // Not used, but required for trait completeness
            godot::builtin::Vector2i::new(0, 0)
        }
        fn set_cell_tiledata_value(&mut self, layer: i32, coords: godot::builtin::Vector2i, data_id: TileMapLayer, value: Variant) {
            // Placeholder: Call the actual Godot API here in a real build
        }
    }
    
    match event {
        AnimationEvent::SetTileAnimation { layer, coords, frame_index } => {
            // Update the tile's animated frame (lightweight API call)
            tilemap.set_cell_tiledata_value(
                layer,
                coords.to_godot_vector(), // Helper to convert Rust coords to Godot Vector2i
                TileMapLayer::ANIMATION_FRAME,
                frame_index.to_variant(),
            );
        }
        
        AnimationEvent::SetLightColor { light_id, color } => {
            // Example: update_light_node(light_id, color);
            // 🔥 FIX 4: Replaced godot_print! with ssxl_info!
            ssxl_info!("Applying color {:?} to light {}", color, light_id);
        }
        
        AnimationEvent::SpawnParticleEffect { effect_id, position } => {
            // Example: spawn_particle_at(effect_id, position);
            // 🔥 FIX 5: Replaced godot_print! with ssxl_info!
            ssxl_info!("Spawning particle {} at {:?}", effect_id, position);
        }
        // ... other event types
    }

    Ok(())
}