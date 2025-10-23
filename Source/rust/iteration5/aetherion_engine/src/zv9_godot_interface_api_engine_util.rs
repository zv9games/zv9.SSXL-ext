// zv9_godot_interface_api_engine_util.rs

use aetherion_shared::zv9_prelude::*;
use aetherion_core::structure::ExternalNoiseType;
use aetherion_core::structure::MapBuildOptions;
use godot::builtin::Vector2i;


pub fn build_map_config(
    width: i32,
    height: i32,
    seed: i64,
    mode: String,
    animate: bool,
    black: Vector2i,
    blue: Vector2i,
) -> MapBuildOptions {
    let mode_enum = mode.parse::<ExternalNoiseType>().unwrap_or(ExternalNoiseType::CellularAutomata);

    MapBuildOptions {
        width,
        height,
        seed: seed.try_into().unwrap_or_default(),
        mode: mode_enum,
        animate,
        black: SerializableVector2i { x: black.x, y: black.y },
        blue: SerializableVector2i { x: blue.x, y: blue.y },
        birth_limit: 4,
        survival_limit: 3,
        fill_ratio: 0.45,
        steps: 5,
        delivery_interval_ms: Some(2),
    }
}
