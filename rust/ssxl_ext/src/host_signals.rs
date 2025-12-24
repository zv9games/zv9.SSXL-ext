#[cfg(feature = "godot-binding")]
use godot::prelude::*;
#[cfg(feature = "godot-binding")]
use godot::builtin::{GString, VarDictionary};

#[cfg(feature = "godot-binding")]
use crate::generate_conductor::GenerateConductor;
#[cfg(feature = "godot-binding")]
use crate::host_poller::ConductorEvents;
#[cfg(feature = "godot-binding")]
use crate::host_conductor::SSXLConductor;

#[cfg(feature = "godot-binding")]
impl SSXLConductor {
    pub fn emit_conductor_ready(&mut self) {
        self.base_mut().emit_signal("conductor_ready", &[]);
    }

    pub fn emit_conductor_idle(&mut self) {
        self.base_mut().emit_signal("conductor_idle", &[]);
    }

    pub fn emit_generation_started(&mut self, tilemap_id: i64, total_chunks: i32) {
        self.base_mut().emit_signal(
            "generation_started",
            &[
                tilemap_id.to_variant(),
                total_chunks.to_variant(),
            ],
        );
    }

    pub fn emit_chunk_rendered(&mut self, completed: i32, total: i32) {
        self.base_mut().emit_signal(
            "chunk_rendered",
            &[
                completed.to_variant(),
                total.to_variant(),
            ],
        );
    }

    pub fn emit_chunk_failed(&mut self, error: &str) {
        self.base_mut().emit_signal(
            "chunk_failed",
            &[GString::from(error).to_variant()],
        );
    }

    pub fn emit_generation_progress(
        &mut self,
        completed: i32,
        total: i32,
        metrics: VarDictionary,
    ) {
        self.base_mut().emit_signal(
            "generation_progress",
            &[
                completed.to_variant(),
                total.to_variant(),
                metrics.to_variant(),
            ],
        );
    }

    pub fn emit_generation_finished(&mut self, tilemap_id: i64) {
        self.base_mut().emit_signal(
            "generation_finished",
            &[tilemap_id.to_variant()],
        );
    }

    pub fn emit_generation_error(&mut self, message: &str) {
        self.base_mut().emit_signal(
            "generation_error",
            &[GString::from(message).to_variant()],
        );
    }

    pub fn emit_debug_event(&mut self, message: &str) {
        self.base_mut().emit_signal(
            "debug_event",
            &[GString::from(message).to_variant()],
        );
    }

    // ----------------------------------------------------
    // ✅ PLAN B HOOKS (SSXL Renderer Integration)
    // ----------------------------------------------------

    /// Called when a chunk has been generated and is ready for rendering.
    /// In Plan B, this will forward raw tile data to your custom renderer.
    fn push_chunk_to_renderer(
        &mut self,
        _conductor: &GenerateConductor,
        _events: &ConductorEvents,
    ) {
        // ✅ PLAN B: This is where your renderer receives chunk data.
        // Example (future):
        // if let Some(renderer) = &self.tilemap_target {
        //     renderer.bind_mut().apply_chunk_mesh(...);
        // }
    }

    /// Called once when the entire generation is complete.
    /// In Plan B, this finalizes the mesh, batching, or GPU upload.
    fn finalize_renderer_output(&mut self) {
        // ✅ PLAN B: Finalize mesh, upload to GPU, build chunk instances, etc.
    }

    // ----------------------------------------------------
    // Main signal + renderer pipeline
    // ----------------------------------------------------
    pub fn poll_and_emit_signals(
        &mut self,
        conductor: &GenerateConductor,
        events: &ConductorEvents,
    ) {
        let metrics = conductor.get_metrics();
        let total = metrics.total_chunks as i32;
        let completed = metrics.completed_chunks as i32;

        if total > 0 {
            // ✅ SPAM BLOCKER: Only emit conductor_ready once per generation cycle
            if !self.has_emitted_ready {
                self.emit_conductor_ready();
                self.has_emitted_ready = true;
            }

            // ✅ PLAN B: Forward chunk data to renderer
            if events.chunks_rendered > 0 {
                self.push_chunk_to_renderer(conductor, events);
            }

            // ✅ Emit progress
            let mut dict = VarDictionary::new();
            let _ = dict.insert("completed", completed.to_variant());
            let _ = dict.insert("total", total.to_variant());
            let _ = dict.insert(
                "chunks_rendered_this_frame",
                events.chunks_rendered.to_variant(),
            );
            let _ = dict.insert(
                "generation_completed",
                events.generation_completed.to_variant(),
            );

            self.emit_generation_progress(completed, total, dict);

            if events.chunks_rendered > 0 {
                self.emit_chunk_rendered(completed, total);
            }

            // ✅ NEW SPAM BLOCKER: Only emit generation_finished once
            if events.generation_completed && !self.has_emitted_finished {
                // ✅ PLAN B: Finalize renderer output
                self.finalize_renderer_output();

                let tilemap_id = events.finalized_tilemap_id.unwrap_or(-1);
                self.emit_generation_finished(tilemap_id);
                self.has_emitted_finished = true;
            }
        } else {
            self.emit_conductor_idle();
        }
    }
}
