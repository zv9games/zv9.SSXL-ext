use std::str::FromStr;

use godot::prelude::*;
use godot::classes::TileMap;

use aetherion_core::pipeline::builder::spawn_map_builder;
use aetherion_core::pipeline::data::{SerializableVector2i, TileInfo};
use aetherion_core::zv9_aetherion_core_conductor::{Conductor, ProcCommand};
use aetherion_core::zv9_aetherion_generator_noise::NoiseType;
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::{ChunkStreamer, SyncBridge};
use aetherion_shared::shared::EngineMessage;

use crate::MapDataChunk;
use crate::zv9_godot_interface_api_engine_signals::emit_pending_signals;
use crate::zv9_godot_interface_api_signals::AetherionSignals;
use crate::zv9_godot_interface_messaging_sync::{GodotDelivery, GodotSync};

/// 🧠 Godot-facing engine node for map generation and runtime coordination.
#[derive(GodotClass, Debug)]
#[class(init, base = Node)]
pub struct AetherionEngine {
    #[base]
    pub base: Base<Node>,

    pub sync: GodotSync,
    pub signals_node: Option<Gd<AetherionSignals>>,

    #[export]
    pub target_tilemap: Option<Gd<TileMap>>,

    pub current_status: String,
    pub conductor: Option<Conductor<GodotDelivery>>,
    pub chunk: Option<MapDataChunk>,
    pub last_reported_status: Option<String>,
    pub pending_tiles: Vec<(SerializableVector2i, TileInfo)>,
}

#[godot_api]
impl AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        let sync = GodotSync::init();
        godot_print!("🧵 AetherionEngine initialized with Sync ID: {}", sync.sync_id());

        Self {
            base,
            sync,
            signals_node: None,
            target_tilemap: None,
            current_status: "Awaiting Oracle".into(),
            conductor: None,
            chunk: None,
            last_reported_status: None,
            pending_tiles: Vec::new(),
        }
    }

    #[signal]
    fn status_updated(status: String);

    #[func]
    fn init_engine(&mut self) {
        if self.conductor.is_none() {
            let delivery = GodotDelivery {
                sync: self.sync.clone(),
                bridge: SyncBridge::default(),
            };
            // supply est_cap explicitly
            let streamer = ChunkStreamer::new(delivery, 2, 1024);
            self.conductor = Some(Conductor::new(streamer));
            self.chunk = Some(MapDataChunk::new());

            godot_print!("🧵 AetherionEngine manual setup complete (conductor armed).");
        }

        self.pending_tiles.clear();
        self.base_mut().set_process(true);
        self.sync.push_status("🟢 Engine initialized. Awaiting map build...");
        self.last_reported_status = None;
        self.process(0.0);

        godot_print!("⚙️ AetherionEngine online. Systems nominal.");
        godot_print!("🧠 AetherionEngine boot sequence complete. Ready for map generation.");
    }

    #[func]
    fn _ready(&mut self) {
        self.init_engine();
    }

    #[func]
    fn process(&mut self, _delta: f64) {
        if let Some(conductor) = self.conductor.as_mut() {
            conductor.streamer_mut().delivery_mut().forward_bridge_signals();
        }

        for signal_msg in self.sync.drain_signals() {
            match signal_msg {
                EngineMessage::Status(status) => {
                    self.current_status = status.clone();
                    self.base_mut().emit_signal("status_updated", &[Variant::from(status.clone())]);
                }
                EngineMessage::Chunk(chunk) => {
                    self.queue_chunk_tiles(chunk);
                }
                other => {
                    godot_print!("⚠️ Unhandled EngineMessage: {:?}", other);
                }
            }
        }

        self.apply_batch_tiles(20);
        emit_pending_signals(self);
    }

    fn queue_chunk_tiles(&mut self, mut chunk: MapDataChunk) {
        let queued_count = chunk.tiles.len();
        for (key, tile) in chunk.tiles.drain() {
            self.pending_tiles.push((key, tile));
        }
        godot_print!("📦 Queued {} tiles (total pending: {})", queued_count, self.pending_tiles.len());
    }

    fn apply_batch_tiles(&mut self, batch_size: usize) {
        let Some(tilemap) = &mut self.target_tilemap else {
            return;
        };

        let to_apply = self.pending_tiles.len().min(batch_size);
        for _ in 0..to_apply {
            if let Some((key, tile)) = self.pending_tiles.pop() {
                let pos = Vector2i::new(key.x, key.y);
                let atlas = Vector2i::new(tile.atlas_coords.x, tile.atlas_coords.y);

                tilemap
                    .set_cell_ex(tile.layer as i32, pos)
                    .source_id(tile.source_id)
                    .atlas_coords(atlas)
                    .alternative_tile(tile.alternate_id)
                    .done();
            }
        }

        tilemap.force_update();

        if self.pending_tiles.is_empty() {
            self.sync.push_status("🟢 Map rendered complete");
        }
    }

    #[func]
    fn tick(&mut self, tick: u64) {
        self.sync.push_status("Idle");
        if let (Some(conductor), Some(chunk)) = (self.conductor.as_mut(), self.chunk.as_mut()) {
            conductor.tick(tick, chunk);
        }
        self.process(0.0);
    }

    #[func]
    fn build_map(
        &mut self,
        width: i32,
        height: i32,
        seed: i64,
        mode: String,
        animate: bool,
        black: Vector2i,
        blue: Vector2i,
    ) {
        let black_ser = SerializableVector2i { x: black.x, y: black.y };
        let blue_ser = SerializableVector2i { x: blue.x, y: blue.y };

        let config = aetherion_core::zv9_aetherion_generator_noise_config::NoiseConfig {
            width: width as usize,
            height: height as usize,
            seed: seed as u64,
            birth_limit: 4,
            survival_limit: 3,
            fill_ratio: 0.45,
            steps: 5,
        };

        let noise_type = NoiseType::from_str(&mode).unwrap_or(NoiseType::Basic);

        if let Some(conductor) = self.conductor.as_mut() {
            let streamer = conductor.streamer_mut();
            spawn_map_builder(streamer, &config, noise_type, animate, black_ser, blue_ser);
            conductor.enqueue(ProcCommand::GenerateTerrain { seed: seed as u64 });
            self.sync.push_status("🛠️ Map build requested");
        }
    }

    #[func]
    fn get_status(&mut self) -> String {
        if self.last_reported_status.as_deref() != Some(&self.current_status) {
            self.last_reported_status = Some(self.current_status.clone());
        }
        self.current_status.clone()
    }

    #[func]
    fn ping(&self) {
        godot_print!("⚙️ Engine: Ping received.");
    }

    #[func]
    fn set_signals_node(&mut self, node: Gd<AetherionSignals>) {
        self.signals_node = Some(node);
    }

    #[func]
    fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.target_tilemap = Some(tilemap);
    }

    fn apply_chunk_to_tilemap(&mut self, chunk: &MapDataChunk) {
        self.queue_chunk_tiles(chunk.clone());
        self.apply_batch_tiles(100);
    }

    pub fn sync(&self) -> &GodotSync {
        &self.sync
    }
    pub fn sync_mut(&mut self) -> &mut GodotSync {
        &mut self.sync
    }
	
    /// Mutable access to the conductor
    pub fn conductor_mut(&mut self) -> Option<&mut Conductor<GodotDelivery>> {
        self.conductor.as_mut()
    }

    /// Update current status string
    pub fn set_current_status(&mut self, status: String) {
        self.current_status = status;
    }

    /// Mutable access to the signals node
    pub fn signals_node_mut(&mut self) -> Option<&mut Gd<AetherionSignals>> {
        self.signals_node.as_mut()
    }

    /// Mutable access to the target TileMap
    pub fn target_tilemap_mut(&mut self) -> Option<&mut Gd<TileMap>> {
        self.target_tilemap.as_mut()
    }
}



