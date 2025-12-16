use godot::prelude::*;
use godot::classes::{TileMap, TileMapLayer};
use godot::obj::InstanceId;

use crate::animate_events::AnimationEvent;
use crate::animate_conductor::AnimationConductor;
use crate::host_state::GodotError;
use crate::{ssxl_error, ssxl_warn, ssxl_info};

// Define a safe maximum number of animation events to process per frame
const MAX_ANIM_EVENTS_PER_FRAME: u32 = 256;

/// Polls the animation event channel and applies visual updates to the Godot world.
/// This must be called on the Godot main thread.
pub fn apply_animation_updates(conductor: &AnimationConductor, tilemap_id: i64) -> u32 {
    let mut events_processed = 0;

    while events_processed < MAX_ANIM_EVENTS_PER_FRAME {
        match conductor.event_receiver.try_recv() {
            Ok(event) => {
                if let Err(e) = handle_animation_event(tilemap_id, event) {
                    ssxl_error!("Host Anim: Failed to handle event: {:?}", e);
                }
                events_processed += 1;
            }
            Err(flume::TryRecvError::Empty) => {
                break;
            }
            Err(flume::TryRecvError::Disconnected) => {
                ssxl_warn!("Host Anim: Conductor event channel disconnected.");
                break;
            }
        }
    }

    events_processed
}

/// Dispatches the event to the appropriate Godot TileMap or Node API call.
fn handle_animation_event(tilemap_id: i64, event: AnimationEvent) -> Result<(), GodotError> {
    // Convert numeric ID to Godot's InstanceId internally.
    let instance_id = InstanceId::from_i64(tilemap_id);

    let tilemap_node = InstanceId::get_object(instance_id)
        .ok_or(GodotError::InvalidInstance)?
        .cast::<TileMap>();

    // Temporarily handling the unwrap() if the cast fails. In production this should be handled.
    let mut tilemap = tilemap_node.unwrap();

    // Mock implementation of to_godot_vector() and set_cell_tiledata_value()
    trait MockTileMapMethods {
        fn to_godot_vector(&self) -> godot::builtin::Vector2i;
        fn set_cell_tiledata_value(
            &mut self,
            layer: i32,
            coords: godot::builtin::Vector2i,
            data_id: TileMapLayer,
            value: Variant,
        );
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

        fn set_cell_tiledata_value(
            &mut self,
            _layer: i32,
            _coords: godot::builtin::Vector2i,
            _data_id: TileMapLayer,
            _value: Variant,
        ) {
            // Placeholder: Call the actual Godot API here in a real build
        }
    }

    match event {
        AnimationEvent::SetTileAnimation { layer, coords, frame_index } => {
            tilemap.set_cell_tiledata_value(
                layer,
                coords.to_godot_vector(),
                TileMapLayer::ANIMATION_FRAME,
                frame_index.to_variant(),
            );
        }

        AnimationEvent::SetLightColor { light_id, color } => {
            ssxl_info!("Applying color {:?} to light {}", color, light_id);
        }

        AnimationEvent::SpawnParticleEffect { effect_id, position } => {
            ssxl_info!("Spawning particle {} at {:?}", effect_id, position);
        }
        // ... other event types
    }

    Ok(())
}
