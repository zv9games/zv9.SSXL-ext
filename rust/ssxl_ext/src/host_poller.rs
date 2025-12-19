use crate::generate_conductor::GenerateConductor;
use crate::ssxl_info;

pub struct ConductorEvents {
    pub chunks_rendered: u32,
    pub generation_completed: bool,
    pub finalized_tilemap_id: Option<i64>,
}

pub fn poll_conductor_status(conductor: &mut GenerateConductor) -> ConductorEvents {
    let (chunks_rendered, generation_completed) = conductor.poll_chunks_and_render();

    if chunks_rendered > 0 {
        ssxl_info!("Poller: Rendered {} chunks this frame.", chunks_rendered);
    }

    let finalized_tilemap_id = conductor.try_finalize_and_get_target_id();

    if let Some(id_for_logging) = finalized_tilemap_id {
        ssxl_info!(
            "Poller: All chunks rendered. Finalization for ID: {}",
            id_for_logging
        );
    }

    ConductorEvents {
        chunks_rendered,
        generation_completed,
        finalized_tilemap_id,
    }
}
