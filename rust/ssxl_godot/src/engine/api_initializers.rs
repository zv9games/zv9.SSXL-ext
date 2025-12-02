// ssxl_godot/src/engine/api_initializers.rs

use tokio::sync::mpsc;

use ssxl_generate::{
    Conductor,
    ConductorProgressReceiver,
    // FIX 1: ConductorState is found under ssxl_generate::conductor
    conductor::ConductorState, 
};
use ssxl_sync::AnimationConductorHandle;

// CRITICAL FIX E0308: Alias the specific message type to the simple 
// name 'AnimationUpdate' to match the type expected by the AsyncPoller.
use ssxl_shared::{
    AnimationCommand, 
    message::messages::AnimationUpdate as AnimationUpdate, // <-- RENAMED ALIAS
    AnimationState,
};

// Use the corrected alias for the receiver type.
pub type AnimationUpdateReceiver = mpsc::UnboundedReceiver<AnimationUpdate>;

/// Returned after Phase 1 — contains everything needed to spawn threads in Phase 2
pub struct GenesisHandles {
    pub gen_state: ConductorState,
    pub anim_state: AnimationState,
    pub gen_progress_rx: ConductorProgressReceiver,
    pub anim_update_rx: AnimationUpdateReceiver,
    pub anim_command_tx: AnimationConductorHandle,
    
    // FIX: Prefix unused internal fields with `_` to suppress dead_code warnings.
    // Internal — only used to spawn threads
    pub(crate) _gen_conductor: Conductor,
    pub(crate) _anim_rx: mpsc::UnboundedReceiver<AnimationCommand>,
    // Use the corrected alias for the sender type.
    pub(crate) _anim_update_tx: mpsc::UnboundedSender<AnimationUpdate>, 
}

// NOTE: The rest of the module (e.g., `build_engine_genesis` function) 
// would follow here.