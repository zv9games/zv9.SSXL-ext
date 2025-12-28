use flume::Receiver;
use crate::generate_conductor_state::{ConductorStateContainer, ConductorState};
use crate::shared_message::GenerationDataMessage;
use crate::host_tilemap::render_chunk_direct;
use crate::config::DEBUG_RENDER_PACING;   // <-- NEW debug flag

const MAX_CHUNKS_PER_FRAME: u32 = 16;

#[cfg(feature = "godot-binding")]
pub fn render_available_chunks(
    tilemap_id_raw: i64,
    chunk_receiver: &Receiver<GenerationDataMessage>,
    state_container: &ConductorStateContainer,
) -> (u32, bool) {
    let mut chunks_rendered_this_frame = 0;

    if DEBUG_RENDER_PACING {
        crate::ssxl_info!(
            "RenderPacing: BEGIN frame (max={})",
            MAX_CHUNKS_PER_FRAME
        );
    }

    while chunks_rendered_this_frame < MAX_CHUNKS_PER_FRAME {
        match chunk_receiver.try_recv() {
            Ok(GenerationDataMessage::CompletedChunk(chunk)) => {
                if DEBUG_RENDER_PACING {
                    crate::ssxl_info!(
                        "RenderPacing: Received CompletedChunk ({}, {})",
                        chunk.position.0,
                        chunk.position.1
                    );
                }

                if let Err(e) = render_chunk_direct(tilemap_id_raw, chunk) {
                    // Errors always print
                    crate::ssxl_error!("Direct write failed: {:?}", e);
                } else {
                    chunks_rendered_this_frame += 1;
                    state_container.increment_completed_chunks();

                    if DEBUG_RENDER_PACING {
                        crate::ssxl_info!(
                            "RenderPacing: Chunk rendered. Frame count = {}",
                            chunks_rendered_this_frame
                        );
                    }
                }
            }

            Ok(GenerationDataMessage::JobFailure(e)) => {
                // Errors always print
                crate::ssxl_error!("Worker job failed: {:?}", e);
            }

            Ok(GenerationDataMessage::Ack) => {
                if DEBUG_RENDER_PACING {
                    crate::ssxl_info!("RenderPacing: Ack received (ignored)");
                }
            }

            Err(flume::TryRecvError::Empty) => break,

            Err(flume::TryRecvError::Disconnected) => {
                // Warnings always print
                crate::ssxl_warn!("Worker channel disconnected unexpectedly.");
                break;
            }
        }
    }

    let metrics = state_container.get_metrics();
    let is_finished =
        metrics.completed_chunks == metrics.total_chunks && metrics.total_chunks > 0;

    if DEBUG_RENDER_PACING {
        crate::ssxl_info!(
            "RenderPacing: Frame done. Rendered={} / Total={}",
            chunks_rendered_this_frame,
            metrics.total_chunks
        );
    }

    if is_finished && state_container.get_state() == ConductorState::Generating {
        if DEBUG_RENDER_PACING {
            crate::ssxl_info!("RenderPacing: All chunks complete → transitioning to Finished");
        }
        state_container.transition_to(ConductorState::Finished);
    }

    (chunks_rendered_this_frame, is_finished)
}

#[cfg(not(feature = "godot-binding"))]
pub fn render_available_chunks(
    _tilemap_id: i64,
    _chunk_receiver: &Receiver<GenerationDataMessage>,
    _state_container: &ConductorStateContainer,
) -> (u32, bool) {
    (0, false)
}
