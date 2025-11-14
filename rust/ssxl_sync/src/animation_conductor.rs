use godot::prelude::*;
use godot::builtin::{Array, Variant}; // Import Variant to use with Array
use ssxl_shared::{
    // We only import AnimationUpdate (the root export) because it's used in the
    // struct field type definition, but we need to ensure the receiver *type*
    // matches what's sent.
    AnimationUpdate,
    AnimationConductorHandle,
    AnimationState, 
};

// CRITICAL FIX: Explicitly import the correct message type for use in the receiver's type.
// This resolves the E0308 error by making the two type aliases compatible.
use ssxl_shared::messages::AnimationUpdate as MessageAnimationUpdate; 

use ssxl_animate::{
    // Keep logic components imported from ssxl_animate.
    initialize_animation_conductor, 
    AnimationConductor, 
};
// Use `tokio::sync::mpsc` for channel creation and type aliases
use tokio::sync::mpsc::{self, UnboundedReceiver}; 

// FIX: Change the type alias to use the correct MESSAGE struct, not the root export.
// The root type 'AnimationUpdate' is the *name* used in the FFI struct field,
// but the underlying *type* of the receiver must be the MessageAnimationUpdate.
pub type AnimationUpdateReceiver = UnboundedReceiver<MessageAnimationUpdate>;

/// The FFI Conductor: This struct is exposed to Godot as a Singleton or Node.
/// Its sole purpose is to bridge Godot's main thread with the Rust async workers.
#[derive(GodotClass)]
#[class(tool, init, base=Node)] // Base Node for easy Godot integration
pub struct FfiAnimationConductor {
    // 1. The FFI Handle: Used by Godot to send commands to the Rust async loop.
    command_tx: Option<AnimationConductorHandle>,
    // 2. The FFI Poller: Used by Godot's main thread to receive updates from workers.
    // The type of this field is what the compiler expects (UnboundedReceiver<ssxl_shared::AnimationUpdate>)
    // The type alias above now points the Receiver to the correct underlying type.
    update_rx: Option<AnimationUpdateReceiver>,
    // 3. The Arc<Mutex<Conductor>>: The core async component (needs to be run by the Tokio runtime).
    _core_conductor: Option<std::sync::Arc<std::sync::Mutex<AnimationConductor>>>,
    // Godot-safe handle to the TileMap resource for applying updates.
    tilemap_node: Option<Gd<Node2D>>,
}

#[godot_api]
impl FfiAnimationConductor {
    // FFI Lifecycle: Called once when the Node enters the tree.
    fn ready(&mut self) {
        // NOTE: The `update_rx` created here is `UnboundedReceiver<ssxl_shared::messages::AnimationUpdate>`
        let (update_tx, update_rx) = mpsc::unbounded_channel();
        let initial_state = AnimationState::default(); 

        let (tx_handle, core_conductor) = initialize_animation_conductor(
            update_tx, // Now uses the correct sender type
            initial_state, // Now uses the real state
        );

        // Assign the handles and receiver to the struct fields.
        self.command_tx = Some(tx_handle);
        
        // This line now compiles because `update_rx` is of type `AnimationUpdateReceiver`,
        // and the type alias now points to the correct underlying message struct.
        self.update_rx = Some(update_rx); 
        
        self._core_conductor = Some(core_conductor);

        godot_print!("SSXL Animation System FFI initialized.");
    }

    // ... (omitted `queue_job` and `poll_updates` logic as they are not the source of the error)
    
    /// FFI Method: Called every frame (e.g., via _process(delta)) by GDScript.
    #[func]
    pub fn poll_updates(&mut self) -> i32 {
        let mut updates_processed = 0;
        // The Vec here must be parameterized with the correct type as well.
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
            // ...
        }
    }
}