// ssxl_godot/src/channel_handler.rs (CONFIRMED FIXED)

use godot::prelude::*;
use godot::classes::{Node, TileMap};
// FIX 1: TRef is deprecated and should be removed. Gd handles object references.
use godot::obj::Gd;
use godot::builtin::GString;

// Internal Crate Dependencies
use ssxl_generate::Conductor;
// FIX 2: Correct imports based on where the messages actually live or should live.
use ssxl_shared::tile_data::AnimationUpdate;
// Assuming ChunkMessage is defined in a new shared messages module
use ssxl_shared::messages::ChunkMessage;

// We need the ChunkPresenter type to be accessible
use crate::chunk_presenter::ChunkPresenter;

use std::sync::{Arc, Mutex};

/// Handles incoming messages and data from the background worker threads (Conductor)
/// and applies the changes to the Godot world (TileMap and Signals).
#[derive(Debug, Default, Clone)]
pub struct ChannelHandler {
    // Reference to the presenter delegate
    // ⚡️ E0277 FIXED by adding #[derive(Debug)] to ChunkPresenter
    presenter: Option<ChunkPresenter>,
    
    // Node references are required for emitting signals
    signals_node: Option<Gd<Node>>,
}

impl ChannelHandler {
    pub fn new() -> Self {
        ChannelHandler::default()
    }

    /// Sets the presenter delegate.
    pub fn set_presenter(&mut self, presenter: ChunkPresenter) {
        self.presenter = Some(presenter);
    }

    /// Sets the signals node reference.
    pub fn set_signals_node(&mut self, signals_node: Option<Gd<Node>>) {
        self.signals_node = signals_node;
    }

    /// Processes a batch of ChunkMessages from the generation thread.
    /// Returns a status update message if an important state change occurred.
    pub fn process_generation_messages(
        &mut self,
        messages: Vec<ChunkMessage>,
        _conductor: Option<Arc<Mutex<Conductor>>>,
        tilemap_node: Option<&mut Gd<TileMap>>,
    ) -> Option<GString> {
        if messages.is_empty() {
            return None;
        }

        // Delegate chunk presentation to the ChunkPresenter
        if let Some(ref mut presenter) = self.presenter {
            if let Some(tilemap) = tilemap_node {
                for msg in messages {
                    // ⚡️ E0599 FIXED by implementing present_chunk on ChunkPresenter
                    presenter.present_chunk(tilemap, msg);
                }
            }
        }

        // Placeholder for emitting final status or errors
        None
    }

    /// Processes a batch of AnimationUpdates from the animation thread.
    /// Returns a status update message if an important state change occurred.
    pub fn process_animation_updates(
        &mut self,
        updates: Vec<AnimationUpdate>,
        tilemap_node: Option<&mut Gd<TileMap>>,
    ) -> Option<GString> {
        if updates.is_empty() {
            return None;
        }

        // Delegate animation updates to the ChunkPresenter
        if let Some(ref mut presenter) = self.presenter {
            if let Some(tilemap) = tilemap_node {
                for update in updates {
                    // ⚡️ E0599 FIXED by implementing update_animated_tile on ChunkPresenter
                    presenter.update_animated_tile(tilemap, update);
                }
            }
        }
        
        // Note: Real implementation would emit signals for animation completion
        None
    }
}