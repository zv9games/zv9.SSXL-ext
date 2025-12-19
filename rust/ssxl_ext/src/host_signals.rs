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

    pub fn poll_and_emit_signals(
        &mut self,
        conductor: &GenerateConductor,
        events: &ConductorEvents,
    ) {
        let metrics = conductor.get_metrics();
        let total = metrics.total_chunks as i32;
        let completed = metrics.completed_chunks as i32;

        if total > 0 {
            self.emit_conductor_ready();

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

            if events.generation_completed {
                let tilemap_id = events.finalized_tilemap_id.unwrap_or(-1);
                self.emit_generation_finished(tilemap_id);
            }
        } else {
            self.emit_conductor_idle();
        }
    }
}
