// ============================================================================
// 🎼 Conductor Synchronous Chunk Retrieval (`Conductor::get_chunk_data`)
// ----------------------------------------------------------------------------
// This extension to the Conductor provides synchronous access to chunk data.
// While most of the Conductor system is designed around asynchronous task
// execution and streaming updates, there are scenarios where a caller needs
// immediate access to a chunk’s tile data. This method bridges that gap.
//
// Purpose:
//   • Allow synchronous retrieval of chunk data for given coordinates.
//   • Internally trigger chunk generation if data is not cached.
//   • Block until a `GenerationMessage::Generated` is received.
//   • Provide a fallback in case of channel errors.
//
// Key Components:
//   • active_generator_id
//       - Retrieved from conductor state.
//       - Determines which generator to use for chunk creation.
//
//   • temp_sender / temp_receiver
//       - Temporary bounded channel (capacity = 1).
//       - Used to send and receive the generated chunk synchronously.
//
//   • state_arc
//       - Arc-wrapped conductor state for safe sharing across tasks.
//
//   • handle_chunk_unit
//       - Core function that checks cache, generates chunk if missing,
//         and sends back a `GenerationMessage`.
//
// Workflow:
//   1. Retrieve active generator ID from conductor state.
//   2. Create a temporary bounded channel for synchronous communication.
//   3. Wrap conductor state in Arc for safe sharing.
//   4. Log the synchronous chunk request.
//   5. Call `handle_chunk_unit` to generate or fetch chunk data.
//   6. Enter blocking loop waiting for a `GenerationMessage`:
//        • If `Generated`: unwrap Arc<ChunkData> into owned ChunkData.
//        • If other message: ignore and continue waiting.
//        • If channel closed: log error and return fallback empty chunk.
//
// Design Choices:
//   • Blocking loop ensures synchronous semantics, even in async environment.
//   • Arc::try_unwrap optimizes ownership transfer; fallback clone ensures safety.
//   • Temporary channel isolates synchronous requests from async streams.
//   • Logging provides visibility into synchronous requests and errors.
//
// Educational Note:
//   • This method demonstrates how synchronous access can be layered on top of
//     an asynchronous system in Rust.
//   • By carefully using bounded channels and blocking receives, it provides
//     deterministic behavior without disrupting async workflows.
// ============================================================================


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
