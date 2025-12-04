// src/conductor/sync_get.rs
use super::Conductor;
use crate::task::{handle_chunk_unit, GenerationMessage};
use ssxl_math::prelude::Vec2i;
use ssxl_shared::ChunkData;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, error};

impl Conductor {
    pub fn get_chunk_data(&self, chunk_coords: &Vec2i) -> ChunkData {
        let active_generator_id = self.internal_state.get_active_generator_id();
        let (temp_sender, mut temp_receiver) = mpsc::channel(1);
        let state_arc = Arc::new(self.internal_state.clone());

        info!("Sync chunk request: {:?}", chunk_coords);

        handle_chunk_unit(
            *chunk_coords,
            &active_generator_id,
            self.generator_manager.get_map_ref(),
            &self.chunk_cache,
            &temp_sender,
            &state_arc,
        );

        loop {
            match temp_receiver.blocking_recv() {
                Some(GenerationMessage::Generated(_, chunk_data_arc)) => {
                    return Arc::try_unwrap(chunk_data_arc).unwrap_or_else(|arc| (*arc).clone());
                }
                Some(_) => continue,
                None => {
                    error!("Sync channel closed for {:?}", chunk_coords);
                    return ChunkData::new_at_coords(*chunk_coords);
                }
            }
        }
    }
}