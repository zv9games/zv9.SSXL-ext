// rust/SSXL-ext/src/host_anim.rs

use godot::prelude::*;
use crate::animate_events::AnimationEvent;
use crate::animate_conductor::AnimationConductor;
use crate::host_state::GodotError;

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
                    godot_error!("Host Anim: Failed to handle event: {:?}", e);
                }
                events_processed += 1;
            },
            Err(flume::TryRecvError::Empty) => {
                // No more events this frame
                break;
            },
            Err(flume::TryRecvError::Disconnected) => {
                godot_warn!("Host Anim: Conductor event channel disconnected.");
                break;
            }
        }
    }
    
    events_processed
}

// rust/SSXL-ext/src/host_anim.rs

/// Dispatches the event to the appropriate Godot TileMap or Node API call.
fn handle_animation_event(tilemap_id: InstanceId, event: AnimationEvent) -> Result<(), GodotError> {
    
    // We assume the tilemap_id can be safely resolved to a Gd<TileMap>
    let tilemap_node = InstanceId::get_object(tilemap_id)
        .ok_or(GodotError::InvalidInstance)?
        .cast::<TileMap>();

    let mut tilemap = tilemap_node.unwrap(); // Assume cast is successful for brevity

    match event {
        AnimationEvent::SetTileAnimation { layer, coords, frame_index } => {
            // Update the tile's animated frame (lightweight API call)
            // Note: This often requires Godot's TileMap::set_cell_tiledata function
            
            // Simplified call to set the tile's frame (assuming `set_frame` exists)
            tilemap.set_cell_tiledata_value(
                layer,
                coords.to_godot_vector(), // Helper to convert Rust coords to Godot Vector2i
                TileMapLayer::ANIMATION_FRAME,
                frame_index.to_variant(),
            );
        }
        
        AnimationEvent::SetLightColor { light_id, color } => {
            // Find a separate light node managed by the system and update its color.
            // This event handles non-tile-based dynamic assets.
            // Example: update_light_node(light_id, color);
            godot_print!("Applying color {:?} to light {}", color, light_id);
        }
        
        AnimationEvent::SpawnParticleEffect { effect_id, position } => {
            // Instance and position a particle system node
            // Example: spawn_particle_at(effect_id, position);
            godot_print!("Spawning particle {} at {:?}", effect_id, position);
        }
        // ... other event types
    }

    Ok(())
}