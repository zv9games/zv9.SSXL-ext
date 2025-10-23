use aetherion_core::zv9_aetherion_core_conductor::{Conductor, ProcCommand};
use aetherion_core::zv9_aetherion_pipeline_builder_streamer::{ChunkStreamer, SyncBridge};
use crate::zv9_godot_interface_messaging_sync::{GodotDelivery, GodotSync};

/// 📋 Inspects the procedural command queue using GodotSync
pub fn inspect_pending_queue() {
    let sync = GodotSync::init();
    let delivery = GodotDelivery {
        sync: sync.clone(),
        bridge: SyncBridge::default(),
    };
    let streamer = ChunkStreamer::new(delivery, 2, 1024); // 2ms interval, capacity 1024

    let mut conductor = Conductor::new(streamer);
    conductor.enqueue(ProcCommand::EmitSignal("Pending check".into()));

    println!("📋 Queue length: {}", conductor.queue_len());
    println!("⏳ Has pending: {}", conductor.has_pending());
    println!("✅ Queue inspection complete.\n");
}
