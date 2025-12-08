// ============================================================================
// 📊 StatusReporter (`crate::tilemap::status_reporter`)
// ----------------------------------------------------------------------------
// This module defines the `StatusReporter` utility, a stateless helper for
// compiling human-readable status reports from the SSXL engine. It separates
// reporting logic from the core engine, keeping `SSXLEngine` focused on
// orchestration while providing clean, script-friendly summaries.
//
// Purpose:
//   • Generate formatted status strings combining generation and animation states.
//   • Provide helper methods for querying tile counts from the conductor.
//   • Return results as Godot-native types (`GString`) for direct use in GDScript.
//
// Key Components:
//   • StatusReporter (struct)
//       - Stateless utility; contains no fields.
//       - Provides static-style methods for reporting.
//
//   • get_status_report(gen_state, anim_state)
//       - Constructs a formatted status string.
//       - Generation State:
//           • If present, calls `get_status()` on `ConductorState`.
//           • If absent, defaults to "Uninitialized".
//       - Animation State:
//           • If present, derives status from `time_scale`:
//               - > 0.0 → "Running (Scale: Xx)"
//               - == 0.0 → "Stopped"
//               - < 0.0 → "Error/Invalid Scale"
//           • If absent, defaults to "Uninitialized".
//       - Returns a `GString` formatted as:
//           "STATUS: Generation: <gen_status> | Animation: <anim_status>"
//
//   • get_current_tile_count_value(gen_state)
//       - Queries the total number of tiles placed by the generation conductor.
//       - If `gen_state` is present, calls `get_tiles_placed()`.
//       - If absent, defaults to 0.
//       - Returns a `u64` count.
//
// Design Choices:
//   • Stateless design ensures reporting logic is reusable and independent.
//   • Returning `GString` guarantees compatibility with Godot’s scripting layer.
//   • Clear formatting makes status reports easy to parse in logs or UI.
//
// Educational Note:
//   • This module demonstrates how Rust utilities can provide clean,
//     script-friendly reporting interfaces for Godot. By separating reporting
//     from orchestration, `StatusReporter` keeps the engine modular and
//     maintainable, while ensuring external scripts can easily monitor
//     generation and animation progress.
// ============================================================================


use godot::prelude::GString;
use ssxl_generate::conductor::ConductorState;
use ssxl_shared::AnimationState;

pub struct StatusReporter;

impl StatusReporter {
    pub fn get_status_report(
        gen_state: Option<&ConductorState>,
        anim_state: Option<&AnimationState>,
    ) -> GString {
        let gen_status = gen_state
            .map(|state| format!("{:?}", state.get_status()))
            .unwrap_or_else(|| String::from("Uninitialized"));

        let anim_status = anim_state
            .map(|state| {
                if state.time_scale > 0.0 {
                    format!("Running (Scale: {:.2}x)", state.time_scale)
                } else if state.time_scale == 0.0 {
                    String::from("Stopped")
                } else {
                    String::from("Error/Invalid Scale")
                }
            })
            .unwrap_or_else(|| String::from("Uninitialized"));

        let status = format!(
            "STATUS: Generation: {} | Animation: {}", 
            gen_status, 
            anim_status
        );

        GString::from(status.as_str())
    }

    pub fn get_current_tile_count_value(gen_state: Option<&ConductorState>) -> u64 {
        gen_state
            .map(|state| state.get_tiles_placed())
            .unwrap_or(0)
    }
}
