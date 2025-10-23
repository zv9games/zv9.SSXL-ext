use godot::prelude::*;
use std::str::FromStr;
use godot::classes::Control;

#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use crate::zv9_godot_interface_messaging_sync::{GodotDelivery, GodotSync};
use aetherion_core::pipeline::builder::spawn_map_builder;
use aetherion_core::generator::{NoiseConfig, NoiseType, GodotNoiseType};
use aetherion_shared::zv9_shared_types::SerializableVector2i;use aetherion_core::zv9_aetherion_pipeline_builder_streamer::{ChunkStreamer, SyncBridge};
use aetherion_shared::zv9_shared_logging::log_info;

/// 🧭 ControlPanel — UI node for interacting with the Aetherion runtime.
#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct ControlPanel {
    #[base]
    base: Base<Control>,

    #[export]
    terrain_mode: GString,

    #[export]
    structure_mode: GString,

    #[export]
    pacing_ms: i32,

    #[export]
    animate: bool,

    #[export]
    black: Vector2i,

    #[export]
    blue: Vector2i,
}

#[godot_api]
impl ControlPanel {
	#[allow(dead_code)]
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            terrain_mode: "Perlin".into(),
            structure_mode: "None".into(),
            pacing_ms: 2,
            animate: false,
            black: Vector2i::new(0, 0),
            blue: Vector2i::new(64, 64),
        }
    }

    #[func]
    fn _ready(&self) {
        godot_print!("🧭 ControlPanel ready.");
        log_info("ControlPanel", "UI node for interface controls initialized");
    }

    /// 🚀 Dispatches a map generation request to the engine using current settings.
    #[func]
    fn generate_map(&self) {
        let pacing = self.pacing_ms.clamp(1, 1000);

        let godot_mode = GodotNoiseType::from_str(&self.terrain_mode.to_string())
            .unwrap_or_else(|_| {
                godot_warn!("⚠️ Unknown terrain mode: {}", self.terrain_mode);
                GodotNoiseType::Perlin
            });

        let mode: NoiseType = godot_mode.into();

        let config = NoiseConfig {
            width: 128,
            height: 128,
            seed: 42,
            fill_ratio: 0.45,
            steps: 5,
            birth_limit: 4,
            survival_limit: 3,
        };

        let delivery = GodotDelivery {
		sync: GodotSync::init(),
		bridge: SyncBridge::default(),
	};

// supply est_cap explicitly
	let mut streamer = ChunkStreamer::new(delivery, pacing as u64, 1024);

	spawn_map_builder(
		&mut streamer,
		&config, // borrow here
		mode,
		self.animate,
		SerializableVector2i { x: self.black.x, y: self.black.y },
		SerializableVector2i { x: self.blue.x, y: self.blue.y },
	);

	self.base.to_init_gd().emit_signal("map_generation_requested", &[]);
	godot_print!("🧭 ControlPanel: Map generation triggered.");

    }

    /// 🕒 Sets the chunk pacing interval in milliseconds.
    #[func]
    fn set_pacing(&mut self, ms: i32) {
        self.pacing_ms = ms.clamp(1, 1000);
    }

    /// 🎛 Applies a preset configuration by name.
    #[func]
    fn apply_preset(&mut self, name: GString) {
        match name.to_string().as_str() {
            "island" => {
                self.terrain_mode = "Perlin".into();
                self.structure_mode = "None".into();
                self.pacing_ms = 5;
                self.animate = true;
            }
            "maze" => {
                self.terrain_mode = "Cellular".into();
                self.structure_mode = "Maze".into();
                self.pacing_ms = 2;
                self.animate = false;
            }
            "plains" => {
                self.terrain_mode = "Perlin".into();
                self.structure_mode = "Rooms".into();
                self.pacing_ms = 3;
                self.animate = true;
            }
            _ => {
                godot_warn!("⚠️ Unknown preset: {}", name);
            }
        }
    }

    /// 📋 Returns a summary of current settings.
    #[func]
    fn describe_settings(&self) -> String {
        format!(
            "Mode: {}, Structure: {}, Pacing: {}ms, Animate: {}, Black: {:?}, Blue: {:?}",
            self.terrain_mode, self.structure_mode, self.pacing_ms, self.animate, self.black, self.blue
        )
    }

    /// 📦 Converts current settings to a Godot Dictionary.
    #[func]
    fn to_config_dict(&self) -> Dictionary {
        let mut dict = Dictionary::new();
        let _ = dict.insert("terrain_mode", self.terrain_mode.clone());
        let _ = dict.insert("structure_mode", self.structure_mode.clone());
        let _ = dict.insert("pacing_ms", self.pacing_ms);
        let _ = dict.insert("animate", self.animate);
        let _ = dict.insert("black", self.black);
        let _ = dict.insert("blue", self.blue);
        dict
    }
}
