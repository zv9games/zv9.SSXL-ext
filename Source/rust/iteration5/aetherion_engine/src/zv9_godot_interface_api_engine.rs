// zv9_godot_interface_api_engine.rs

use crate::zv9_godot_interface_api_engine_core;
use crate::zv9_godot_interface_api_engine_signals;
pub use crate::zv9_godot_interface_api_engine_util; // optional

pub use zv9_godot_interface_api_engine_core::*;
pub use zv9_godot_interface_api_engine_signals::*;



/*
use godot::prelude::*;
use godot::classes::TileMap;
use godot::global::Error;

#[allow(unused_imports)]
use crate::zv9_prelude::*;
use crate::zv9_godot_interface_map_ext::MapDataChunkExt;
use crate::zv9_godot_interface_messaging_sync::{GodotDelivery, GodotSync};

use aetherion_core::log_component;
use aetherion_core::pipeline::builder::spawn_builder_thread;
use aetherion_core::zv9_aetherion_core_conductor::Conductor;

/// 🚀 AetherionEngine — Godot-facing engine node for procedural generation and signal dispatch.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionEngine {
    #[base]
    base: Base<Node>,
    sync: GodotSync,
    signals_node: Option<Gd<AetherionSignals>>,

    #[export]
    target_tilemap: Option<Gd<TileMap>>,

    current_status: String,
    conductor: Option<Conductor<GodotDelivery>>,
    chunk: Option<MapDataChunk>,
}

// 🔧 Constructor (called by Godot's init) — Debug-enhanced
impl AetherionEngine {
    fn init(base: Base<Node>) -> Self {
        godot_print!("🔧 Engine init: Starting.");
        
        let sync = GodotSync::init();
        godot_print!("🔧 Engine init: Sync created.");
        
        let delivery = GodotDelivery {
            sync: sync.clone(),
            bridge: SyncBridge::default(),
        };
        godot_print!("🔧 Engine init: Delivery created.");
        
        let streamer = ChunkStreamer::new(delivery, 2);
        godot_print!("🔧 Engine init: Streamer created.");
        
        let conductor_raw = Conductor::new(streamer);
        godot_print!("🔧 Engine init: Conductor raw created.");
        
        let chunk_raw = MapDataChunk::new();
        godot_print!("🔧 Engine init: Chunk raw created.");
        
        godot_print!("🔧 Engine init: Building Self with Option<...>.");
        
        Self {
            base,
            sync,
            signals_node: None,
            target_tilemap: None,
            current_status: "Awaiting Oracle".into(),
            conductor: Some(conductor_raw),
            chunk: Some(chunk_raw),
        }
    }
}

#[godot_api]
impl AetherionEngine {
    // 🔧 Lifecycle
    #[func]
    fn enter_tree(&mut self) {
        godot_print!("🔧 AetherionEngine: enter_tree called.");
        // Fallback: Enable process here if _ready skips
        self.base_mut().set_process(true);
        godot_print!("⚙️ Process callback enabled (from enter_tree).");
    }
    
    #[func]
    fn _ready(&mut self) {
        godot_print!("🔧 AetherionEngine: _ready starting.");
        godot_print!("⚙️ AetherionEngine online. Systems nominal.");
        log_component!("AetherionEngine", "Engine node for procedural generation and signal dispatch");
        
        // ✅ Enable process callback directly
        self.base_mut().set_process(true);
        godot_print!("⚙️ Process callback enabled (from _ready).");
        
        // 🔍 Post-init state check
        let has_conductor = self.conductor.is_some();
        let has_chunk = self.chunk.is_some();
        godot_print!("🔍 _ready: Post-init check — Conductor: {}, Chunk: {}", has_conductor, has_chunk);
        
        self.sync.debug_id(); // 🧵 Trace sync ID on startup
        godot_print!("🔧 AetherionEngine: _ready complete.");
    }

    #[func]
    fn process(&mut self, _delta: f64) {
        self.sync.debug_id(); // 🧵 Trace sync ID per cycle
        godot_print!("⚙️ process() running");

        let drained = self.sync.drain_signals();
        godot_print!("📡 process() → drained {} signals", drained.len());

        for signal_msg in drained {
            godot_print!("📡 signal_msg → {:?}", signal_msg);
            if let EngineMessage::Status(status) = signal_msg {
                godot_print!("📡 Status signal received: {}", status);
                self.current_status = status;
            }
        }

        // 🔁 Unified signal dispatch (optional: comment out to separate)
        self.emit_pending_signals();
    }

    // 🔁 Signal Dispatch
    #[func]
    fn emit_pending_signals(&mut self) {
        if let Some(signals_node) = self.signals_node.as_mut() {
            let mut signals = Vec::new();

            if let Some(conductor) = self.conductor.as_mut() {
                let conductor_signals = conductor.streamer_mut().delivery_mut().sync.drain_signals();
                godot_print!("📡 Engine: Drained {} signals from conductor", conductor_signals.len());
                signals.extend(conductor_signals);
            }

            let engine_signals = self.sync.drain_signals();
            godot_print!("📡 Engine: Drained {} signals from engine", engine_signals.len());
            signals.extend(engine_signals);

            for signal_msg in signals {
                godot_print!("📡 Emitting signal: {:?}", signal_msg);

                let result = match signal_msg {
                    EngineMessage::Start => signals_node.emit_signal("build_map_start", &[]),
                    EngineMessage::Progress(percent) => signals_node.emit_signal("generation_progress", &[percent.to_variant()]),
                    EngineMessage::Status(status) => {
                        godot_print!("📡 Status signal received: {}", status);
                        self.current_status = status.clone();
                        signals_node.emit_signal("map_building_status", &[GString::from(status).to_variant()])
                    }
                    EngineMessage::Complete { width, height, mode, animate, duration } => {
                        let mut dict = Dictionary::new();
                        dict.insert("width", width);
                        dict.insert("height", height);
                        dict.insert("mode", mode);
                        dict.insert("animate", animate);
                        dict.insert("duration", duration);
                        signals_node.emit_signal("generation_complete", &[dict.to_variant()])
                    }
                    EngineMessage::MapChunkReady => signals_node.emit_signal("map_chunk_ready", &[]),
                    EngineMessage::ChunkReady(chunk) => {
                        let dict = chunk.to_dictionary();
                        signals_node.emit_signal("chunk_ready", &[dict.to_variant()])
                    }
                    EngineMessage::Cancelled => signals_node.emit_signal("map_build_cancelled", &[]),
                    EngineMessage::Diagnostics { memory_usage, thread_count, tick_rate } => {
                        signals_node.emit_signal("diagnostics", &[
                            memory_usage.to_variant(),
                            (thread_count as i32).to_variant(),
                            tick_rate.to_variant(),
                        ])
                    }
                    EngineMessage::Error(msg) => signals_node.emit_signal("rust_error", &[GString::from(msg).to_variant()]),
                    EngineMessage::Warning(msg) => signals_node.emit_signal("rust_warning", &[GString::from(msg).to_variant()]),
                    EngineMessage::Custom { name, payload } => signals_node.emit_signal("custom_event", &[
                        GString::from(name).to_variant(),
                        json_to_variant(payload),
                    ]),
                    EngineMessage::Paused => signals_node.emit_signal("engine_paused", &[]),
                    EngineMessage::Resumed => signals_node.emit_signal("engine_resumed", &[]),
                    EngineMessage::Retry => signals_node.emit_signal("engine_retry", &[]),
                };

                if result != Error::OK {
                    godot_warn!("⚠️ Engine: Signal emission failed: {:?}", result);
                }
            }
        } else {
            godot_warn!("⚠️ Engine: No signals_node assigned. Cannot emit signals.");
        }
    }

    // 🧭 Engine Control
    #[func]
    pub fn tick(&mut self, tick: u64) {
        self.sync.debug_id(); // 🧵 Trace sync ID per tick
        godot_print!("🔮 Engine: Tick {} received from Oracle.", tick);
        
        // 🔍 Granular: Pre-match state check
        let has_conductor = self.conductor.is_some();
        let has_chunk = self.chunk.is_some();
        godot_print!("🔍 Engine: Pre-tick check — Conductor: {}, Chunk: {}", has_conductor, has_chunk);
        
        match (self.conductor.as_mut(), self.chunk.as_mut()) {
            (Some(conductor), Some(chunk)) => {
                godot_print!("📡 Engine: Dispatching tick to conductor.");
                conductor.tick(tick, chunk); // ✅ Direct call—no unwind wrapper
                godot_print!("✅ Engine: Conductor.tick completed.");

                godot_print!("📡 Engine: Queuing status 'Idle' to sync.");
                self.sync.push_status("Idle");

                godot_print!("⚙️ Engine: Executing process cycle.");
                self.process(0.0);
                
                // 🔍 Post-tick snapshot
                godot_print!("📡 Post-tick status: {}", self.get_status());
            }
            _ => {
                godot_warn!("⚠️ Engine: Tick ignored — conductor or chunk not initialized. (Conductor: {:?}, Chunk: {:?})", 
                            has_conductor, has_chunk);
            }
        }
    }

    #[func]
    pub fn build_map(
        &mut self,
        width: i32,
        height: i32,
        seed: i64,
        mode: String,
        animate: bool,
        black: Vector2i,
        blue: Vector2i,
    ) {
        let mode_enum = mode.parse::<ExternalNoiseType>().unwrap_or(ExternalNoiseType::CellularAutomata);

        let config = MapBuildOptions {
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
        };

        godot_print!("⚙️ Engine: Launching map build thread...");

        if let Some(conductor) = self.conductor.as_mut() {
            let delivery = conductor.streamer_mut().delivery_mut();
            spawn_builder_thread(delivery.clone(), config);
        } else {
            godot_warn!("⚠️ Engine: Cannot build map. Conductor not initialized.");
        }
    }

    // 🧱 Tilemap Control
    #[func]
    pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.target_tilemap = Some(tilemap);
        godot_print!("⚙️ Engine: TileMap target assigned.");
    }

    #[func]
    pub fn apply_chunks_to_tilemap(&mut self) {
        if let Some(tilemap) = self.target_tilemap.as_mut() {
            for chunk in self.sync.drain_chunks() {
                for (pos, tile_info) in chunk.tiles {
                    let pos_vec = Vector2i::new(pos.x, pos.y);
                    let atlas_vec = Vector2i::new(tile_info.atlas_coords.x, tile_info.atlas_coords.y);

                    tilemap.set_cell_ex(0, pos_vec)
                        .source_id(tile_info.source_id)
                        .atlas_coords(atlas_vec)
                        .alternative_tile(tile_info.alternate_id)
                        .done();
                }
            }
        }
    }

    #[func]
    pub fn debug_place_tile(&mut self, x: i32, y: i32) {
        if let Some(tilemap) = self.target_tilemap.as_mut() {
            tilemap.set_cell_ex(0, Vector2i::new(x, y))
                .source_id(0)
                .atlas_coords(Vector2i::new(14, 13))
                .alternative_tile(0)
                .done();
            godot_print!("⚙️ Engine: Debug tile placed at ({}, {}).", x, y);
        } else {
            godot_warn!("⚠️ Engine: No TileMap assigned. Cannot place debug tile.");
        }
    }

    // 📊 Diagnostics
    #[func]
    pub fn ping(&self) {
        self.sync.debug_id(); // 🧵 Trace sync ID on ping
        godot_print!("⚙️ Engine: Ping received. Standing by.");
    }

    #[func]
    pub fn get_status(&self) -> String {
        let status = if self.current_status.is_empty() {
            "No status available"
        } else {
            &self.current_status
        };
        self.sync.debug_id(); // 🧵 Trace sync ID on status query
        godot_print!("📡 get_status() → {}", status);
        status.to_string()
    }

    #[func]
    pub fn set_signals_node(&mut self, node: Gd<AetherionSignals>) {
        self.signals_node = Some(node);
        godot_print!("📡 Engine: Signals node assigned.");
    }
}

*/