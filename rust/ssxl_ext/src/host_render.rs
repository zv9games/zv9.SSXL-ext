use flume::Receiver;
use crate::shared_chunk::Chunk;
use crate::generate_conductor_state::ConductorStateContainer;

#[cfg(feature = "godot-binding")]
use crate::generate_conductor_state::ConductorState;

// ✅ Updated import — now uses the native SSXLTileMap pipeline
#[cfg(feature = "godot-binding")]
use crate::host_tilemap::render_chunk_direct;

#[cfg(feature = "godot-binding")]
const MAX_CHUNKS_PER_FRAME: u32 = 16;

#[cfg(feature = "godot-binding")]
pub fn render_available_chunks(
    tilemap_id_raw: i64,
    chunk_receiver: &Receiver<Chunk>,
    state_container: &ConductorStateContainer,
) -> (u32, bool) {
    let mut chunks_rendered_this_frame = 0;

    while chunks_rendered_this_frame < MAX_CHUNKS_PER_FRAME {
        match chunk_receiver.try_recv() {
            Ok(chunk) => {
                match render_chunk_direct(tilemap_id_raw, chunk) {
                    Ok(_) => {
                        chunks_rendered_this_frame += 1;
                        state_container.increment_completed_chunks();
                    }
                    Err(e) => {
                        crate::ssxl_error!(
                            "Render Pacing Layer: Direct write failed: {:?}",
                            e
                        );
                    }
                }
            }
            Err(flume::TryRecvError::Empty) => break,
            Err(flume::TryRecvError::Disconnected) => {
                crate::ssxl_warn!(
                    "Render Pacing Layer: Worker channel disconnected unexpectedly."
                );
                break;
            }
        }
    }

    let metrics = state_container.get_metrics();
    let is_finished =
        metrics.completed_chunks == metrics.total_chunks && metrics.total_chunks > 0;

    if is_finished && state_container.get_state() == ConductorState::Generating {
        state_container.transition_to(ConductorState::Finished);
    }

    (chunks_rendered_this_frame, is_finished)
}

#[cfg(not(feature = "godot-binding"))]
pub fn render_available_chunks(
    _tilemap_id: i64,
    _chunk_receiver: &Receiver<Chunk>,
    _state_container: &ConductorStateContainer,
) -> (u32, bool) {
    (0, false)
}
