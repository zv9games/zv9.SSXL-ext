//C:/ZV9/zv9.aetherion/rust/src/godot_interface__interface__diagnostics.rs

use godot::prelude::*;
use godot::classes::Label;
#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use aetherion_shared::zv9_shared_logging::log_info;

/// 📊 DiagnosticsOverlay — UI label for displaying engine metrics.
///
/// Used to visualize tick timing, queue length, and runtime status.
#[derive(GodotClass)]
#[class(init, base = Label)]
pub struct DiagnosticsOverlay {
    #[base]
    base: Base<Label>,
}

#[godot_api]
impl DiagnosticsOverlay {
	#[allow(dead_code)]
    fn init(base: Base<Label>) -> Self {
        Self { base }
    }

    /// Updates the overlay with current engine metrics.
    #[func]
    fn update_metrics(&mut self, tick: u64, avg_tick: f64, queue_len: i32) {
        let text = format!(
            "🧠 Tick: {}\n⏱ Avg Tick Duration: {:.2}ms\n📦 Chunk Queue: {}",
            tick,
            avg_tick,
            queue_len
        );
        self.to_gd().set_text(&text);
    }

    #[func]
    fn _ready(&self) {
		godot_print!("📊 DiagnosticsOverlay ready.");
		log_info("DiagnosticsOverlay", "UI label for diagnostics overlay initialized");
	}
}


//end diagnostics