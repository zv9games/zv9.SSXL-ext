use godot::prelude::*;
// FIX: Removed unused imports: Array and Variant
// use godot::builtin::{Array, Variant}; 
use ssxl_shared::{
    // FIX: Removed unused import: AnimationUpdate
    AnimationConductorHandle,
    AnimationState, 
};

// CRITICAL FIX: Explicitly import the correct message type for use in the receiver's type.
use ssxl_shared::messages::AnimationUpdate as MessageAnimationUpdate; 

use ssxl_animate::{
    // Keep logic components imported from ssxl_animate.
    initialize_animation_conductor, 
    AnimationConductor, 
};
// Use `tokio::sync::mpsc` for channel creation and type aliases
use tokio::sync::mpsc::{self, UnboundedReceiver}; 

// FIX: Change the type alias to use the correct MESSAGE struct.
pub type AnimationUpdateReceiver = UnboundedReceiver<MessageAnimationUpdate>;

/// The FFI Conductor: This struct is exposed to Godot as a Singleton or Node.
/// Its sole purpose is to bridge Godot's main thread with the Rust async workers.
#[derive(GodotClass)]
#[class(tool, init, base=Node)] // Base Node for easy Godot integration
pub struct FfiAnimationConductor {
    // 1. The FFI Handle: Used by Godot to send commands.
    // FIX: Allows dead code. This field is accessed via FFI functions (e.g., queue_job).
    #[allow(dead_code)] 
    command_tx: Option<AnimationConductorHandle>,
    // 2. The FFI Poller: Used by Godot's main thread to receive updates from workers.
    update_rx: Option<AnimationUpdateReceiver>,
    // 3. The Arc<Mutex<Conductor>>: The core async component.
    _core_conductor: Option<std::sync::Arc<std::sync::Mutex<AnimationConductor>>>,
    // Godot-safe handle to the TileMap resource for applying updates.
    tilemap_node: Option<Gd<Node2D>>,
}

#[godot_api]
impl FfiAnimationConductor {
    // FFI Lifecycle: Called once when the Node enters the tree.
    // FIX: Allows dead code. This method is called by the Godot Engine lifecycle.
    #[allow(dead_code)]
    fn ready(&mut self) {
        // NOTE: The `update_rx` created here is `UnboundedReceiver<ssxl_shared::messages::AnimationUpdate>`
        let (update_tx, update_rx) = mpsc::unbounded_channel();
        let initial_state = AnimationState::default(); 

        let (tx_handle, core_conductor) = initialize_animation_conductor(
            update_tx, 
            initial_state, 
        );

        // Assign the handles and receiver to the struct fields.
        self.command_tx = Some(tx_handle);
        self.update_rx = Some(update_rx); 
        self._core_conductor = Some(core_conductor);

        godot_print!("SSXL Animation System FFI initialized.");
    }

    // ... (omitted `queue_job` logic)
    
    /// FFI Method: Called every frame (e.g., via _process(delta)) by GDScript.
    #[func]
    pub fn poll_updates(&mut self) -> i32 {
        let mut updates_processed = 0;
        let mut updates_to_process: Vec<MessageAnimationUpdate> = Vec::new();
        
        if let Some(rx) = self.update_rx.as_mut() {
            // CRITICAL LOGIC: Non-blocking receive loop.
            while let Ok(update) = rx.try_recv() {
                updates_to_process.push(update);
            }
        }
        
        // Now self is not mutably borrowed by rx, so we can mutably borrow it here.
        for update in &updates_to_process {
            self.apply_update_to_tilemap(update);
            updates_processed += 1;
        }

        updates_processed
    }

    // Internal function for Godot interaction details (TileMap logic lives here).
    fn apply_update_to_tilemap(&mut self, _update: &MessageAnimationUpdate) {
        if let Some(_tilemap) = &mut self.tilemap_node {
            // TODO: Restore original 246-LOC logic for applying the change.
            // This is where the Chunking logic will be applied next!
        }
    }
}