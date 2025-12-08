// ============================================================================
// 🎮 SSXL Engine Godot Extension (`ssxl_godot::engine`)
// ----------------------------------------------------------------------------
// This module defines the `SSXLEngine` Godot node, which acts as the bridge
// between the Rust-based SSXL engine and the Godot game engine via GDExtension.
//
// Purpose:
//   • Provide a Godot-facing node that orchestrates procedural generation tasks.
//   • Manage communication channels between Rust async tasks and Godot runtime.
//   • Render generated chunks into a Godot `TileMap`.
//   • Expose methods and signals for Godot scripts to interact with the engine.
//
// Key Components:
//   • Feature Flag
//       - `#![feature(int_roundings)]` enables nightly integer division helpers
//         like `div_ceil`, used for chunk calculations.
//
//   • Imports
//       - Godot prelude, Node, TileMap, INode trait for GDExtension integration.
//       - Tokio channels for async task communication.
//       - `tracing` for structured logging.
//       - SSXL subsystems: `Conductor`, `ConductorState`, `GenerationTask`,
//         `GenerationMessage`, math utilities, and shared constants.
//
//   • create_dummy_engine_state
//       - Provides a fallback engine state if the Conductor fails to start.
//       - Ensures the node remains valid even without generation capability.
//
//   • SSXLEngine Struct
//       - Annotated with `#[derive(GodotClass)]` to register as a Godot node.
//       - Fields:
//           • base: underlying Godot Node.
//           • conductor: orchestrates generation tasks.
//           • request_sender: channel for sending tasks.
//           • progress_rx: channel for receiving updates.
//           • tilemap: optional Godot TileMap reference.
//           • state: conductor state tracking activity.
//
//   • INode Implementation
//       - `init`: initializes shared data, starts conductor, or falls back.
//       - `process`: runs every frame, consumes progress messages, applies
//         generated chunks to the TileMap, and emits signals.
//       - `exit_tree`: gracefully shuts down conductor when node is removed.
//
//   • Public Methods
//       - `set_tilemap`: assigns a Godot TileMap for rendering.
//       - `build_map`: schedules generation tasks for a given width/height,
//         dividing into chunks using `div_ceil`.
//       - `is_active`: checks conductor activity state.
//       - `chunk_applied`: signal emitted when a chunk is rendered.
//
//   • Extension Entry Point
//       - `SSXLExtension` struct implements `ExtensionLibrary` to register
//         the engine node with Godot.
//
// Workflow:
//   1. Godot instantiates `SSXLEngine` as a node.
//   2. Rust initializes conductor and shared resources.
//   3. Godot requests map generation via `build_map`.
//   4. Rust sends tasks, processes updates, and applies chunks to TileMap.
//   5. Signals notify Godot scripts when chunks are applied.
//   6. Node shuts down gracefully when removed.
//
// Design Choices:
//   • Async channels decouple generation tasks from rendering loop.
//   • Arc ensures safe sharing of chunk data across threads.
//   • Signals provide a clean Godot-side API for reacting to generation events.
//   • Fallback dummy state prevents crashes if conductor initialization fails.
//
// Educational Note:
//   • This module demonstrates how Rust can extend Godot with high-performance
//     procedural generation, while maintaining safe concurrency and ergonomic
//     scripting interfaces.
//   • By combining async task orchestration with Godot signals, developers gain
//     a powerful workflow for integrating complex systems into game engines.
// ============================================================================


#![feature(int_roundings)]

use godot::{
    prelude::*,
    classes::{Node, TileMap, INode},
    obj::{Base, Gd},
};

use std::sync::Arc;
use tokio::sync::mpsc::{
    channel,
    Receiver,
    UnboundedSender,
    unbounded_channel,
    error::TryRecvError
};
use tracing::{info, error, Level, span};

use ssxl_generate::Conductor;
use ssxl_generate::conductor::ConductorState;
use ssxl_generate::task::task_queue::{GenerationMessage, GenerationTask};

use ssxl_math::prelude::Vec2i;
use ssxl_shared::{initialize_shared_data, CHUNK_SIZE};

fn create_dummy_engine_state(base: Base<Node>) -> SSXLEngine {
    let (dummy_tx, _) = unbounded_channel();
    let (_, dummy_rx) = channel(1);

    SSXLEngine {
        base,
        conductor: None,
        state: ConductorState::new(String::new()),
        request_sender: dummy_tx,
        progress_rx: dummy_rx,
        tilemap: None,
    }
}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct SSXLEngine {
    base: Base<Node>,
    conductor: Option<Conductor>,
    request_sender: UnboundedSender<GenerationTask>,
    progress_rx: Receiver<GenerationMessage>,
    tilemap: Option<Gd<TileMap>>,
    state: ConductorState,
}

#[godot_api]
impl INode for SSXLEngine {
    fn init(base: Base<Node>) -> Self {
        let _span = span!(Level::INFO, "SSXLEngine::init").entered();

        initialize_shared_data();

        match Conductor::new(None) {
            Ok((conductor, state, request_sender, progress_rx)) => {
                info!("SSXLEngine → Conductor ONLINE");
                Self {
                    base,
                    conductor: Some(conductor),
                    state,
                    request_sender,
                    progress_rx,
                    tilemap: None,
                }
            }
            Err(e) => {
                error!("Failed to start Conductor: {:?}", e);
                create_dummy_engine_state(base)
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        loop {
            let message = match self.progress_rx.try_recv() {
                Ok(msg) => msg,
                Err(TryRecvError::Empty) => break,
                Err(e) => {
                    error!("Progress channel error: {:?}", e);
                    break;
                }
            };

            match message {
                GenerationMessage::Generated(key, chunk_data) => {
                    let Some(tilemap) = self.tilemap.as_mut() else { continue; };

                    let origin_x = (key.x as i32) * CHUNK_SIZE as i32;
                    let origin_y = (key.y as i32) * CHUNK_SIZE as i32;
                    let layer = 0;

                    let tiles = Arc::try_unwrap(chunk_data)
                        .unwrap_or_else(|arc| (*arc).clone())
                        .tiles;

                    for (idx, _tile) in tiles.iter().enumerate() {
                        let local_x = (idx as u32 % CHUNK_SIZE) as i32;
                        let local_y = (idx as u32 / CHUNK_SIZE) as i32;

                        let world_x = origin_x + local_x;
                        let world_y = origin_y + local_y;

                        tilemap.set_cell(
                            layer,
                            Vector2i::new(world_x, world_y),
                        );
                    }

                    self.base_mut().emit_signal(
                        "chunk_applied",
                        &[key.x.to_variant(), key.y.to_variant()],
                    );
                }

                GenerationMessage::GenerationComplete => {
                    info!("Generation Task Queue signaled completion.");
                }

                GenerationMessage::StatusUpdate(status) => {
                    info!("Generation status update: {}", status);
                }
            }
        }
    }

    fn exit_tree(&mut self) {
        if let Some(conductor) = self.conductor.take() {
            conductor.signal_shutdown_graceful();
        }
    }
}

#[godot_api]
impl SSXLEngine {
    #[func]
    pub fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.tilemap = Some(tilemap);
    }

    #[func]
    pub fn build_map(&self, width: i32, height: i32, generator_id: GString) {
        if self.request_sender.is_closed() {
            error!("Conductor is shut down. Cannot request map.");
            return;
        }

        let chunks_x = width.div_ceil(CHUNK_SIZE as i32);
        let chunks_y = height.div_ceil(CHUNK_SIZE as i32);

        for x in 0..chunks_x {
            for y in 0..chunks_y {
                let task = GenerationTask {
                    chunk_coords: Vec2i::new(x as i64, y as i64),
                    generator_id: generator_id.to_string(),
                };
                let _ = self.request_sender.send(task);
            }
        }

        info!("Sent {} generation tasks.", chunks_x * chunks_y);
    }

    #[func]
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    #[signal]
    fn chunk_applied(key_x: i64, key_y: i64);
}

struct SSXLExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SSXLExtension {}
