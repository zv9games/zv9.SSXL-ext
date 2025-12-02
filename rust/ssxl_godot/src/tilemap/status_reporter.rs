use godot::prelude::GString;
// FIX: Change import path from conductor_state::ConductorState to conductor::ConductorState
// Assuming the ConductorState type is exported via the 'conductor' module in ssxl_generate.
use ssxl_generate::conductor::ConductorState;
use ssxl_shared::AnimationState;

/// # StatusReporter
/// 
/// A stateless utility struct responsible for querying the various core states
/// (Generation, Animation) and compiling human-readable status reports
/// for the Godot scripting layer.
pub struct StatusReporter;

impl StatusReporter {
    /// Constructs a new, human-readable status string combining the state of
    /// the Generation and Animation cores.
    /// 
    /// This removes complex string formatting logic from SSXLEngine.
    /// 
    /// # Arguments
    /// * `gen_state` - Read-only reference to the Generation process state.
    /// * `anim_state` - Read-only reference to the Animation process state.
    /// 
    /// # Returns
    /// A Godot `GString` containing the formatted status message.
    pub fn get_status_report(
        gen_state: Option<&ConductorState>,
        anim_state: Option<&AnimationState>,
    ) -> GString {
        let gen_status = gen_state
            // This line is assumed correct for ConductorState
            .map(|state| format!("{:?}", state.get_status()))
            .unwrap_or_else(|| String::from("Uninitialized"));

        let anim_status = anim_state
            .map(|state| {
                // FIX: AnimationState does not have a get_status() method.
                // We derive the status from the publicly available `time_scale` field.
                if state.time_scale > 0.0 {
                    format!("Running (Scale: {:.2}x)", state.time_scale)
                } else if state.time_scale == 0.0 {
                    String::from("Stopped")
                } else {
                    // Handle negative or otherwise unexpected scales
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

    /// Retrieves the total number of tiles placed by the engine during generation.
    /// 
    /// # Arguments
    /// * `gen_state` - Read-only reference to the Generation process state.
    pub fn get_current_tile_count_value(gen_state: Option<&ConductorState>) -> u64 {
        gen_state
            .map(|state| state.get_tiles_placed())
            // Fails gracefully to 0 if the core is not yet initialized.
            .unwrap_or(0)
    }
}