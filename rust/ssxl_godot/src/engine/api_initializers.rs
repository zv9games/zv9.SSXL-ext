// FILE: ssxl_godot/src/engine/api_initializers.rs

use tokio::sync::mpsc;
use ssxl_shared::config::config::get_config_from_path;
use ssxl_generate::{
    Conductor,
    ConductorProgressReceiver,
    conductor::ConductorState,
};
use ssxl_sync::AnimationConductorHandle;
use ssxl_shared::message::{
    AnimationCommand,
    AnimationState,
    GenerationCommand,
    GenerationResponse,
    AnimationUpdate,
};
use std::error::Error;
use tracing::info;

pub type AnimationUpdateReceiver = mpsc::UnboundedReceiver<AnimationUpdate>;
pub type GenerationCommandTx = mpsc::UnboundedSender<GenerationCommand>;
pub type GenerationResponseRx = mpsc::UnboundedReceiver<GenerationResponse>;

pub struct GenesisHandles {
    pub gen_state: ConductorState,
    pub anim_state: AnimationState,
    pub gen_progress_rx: ConductorProgressReceiver,
    pub anim_update_rx: AnimationUpdateReceiver,
    pub anim_command_tx: AnimationConductorHandle,
    pub gen_command_tx: GenerationCommandTx,
    pub gen_response_rx: GenerationResponseRx,
    pub(crate) _gen_conductor: Conductor,
    pub(crate) _anim_rx: mpsc::UnboundedReceiver<AnimationCommand>,
    pub(crate) _anim_update_tx: mpsc::UnboundedSender<AnimationUpdate>,
    pub(crate) _gen_resp_tx: mpsc::UnboundedSender<GenerationResponse>,
}

pub fn execute_channel_and_state_setup(
    config_path: Option<&str>,
) -> Result<GenesisHandles, Box<dyn Error>> {
    // We only need the config to pass down — no need to bind it if unused
    // But we keep it readable: we are loading config for Conductor
    let _config = get_config_from_path(config_path);

    // External Godot-facing channels
    let (gen_command_tx, _gen_cmd_rx) = mpsc::unbounded_channel::<GenerationCommand>();
    let (_gen_resp_tx, gen_response_rx) = mpsc::unbounded_channel::<GenerationResponse>();

    // Let the Conductor manage its own internal runtime + channels
    let (
        gen_conductor,
        gen_state,
        _internal_gen_cmd_tx,   // unused — Godot uses gen_command_tx
        _internal_gen_resp_rx,  // unused — Godot uses gen_response_rx
        gen_progress_rx_inner,
    ) = Conductor::new_for_ffi(config_path)?;

    // Animation system channels
    let (anim_command_tx_inner, anim_rx) = mpsc::unbounded_channel::<AnimationCommand>();
    let (anim_update_tx, anim_update_rx) = mpsc::unbounded_channel::<AnimationUpdate>();

    let anim_state = AnimationState::default();

    info!("Genesis engine initialized: Conductor + Animation systems ready.");

    Ok(GenesisHandles {
        gen_state,
        anim_state,
        gen_progress_rx: ConductorProgressReceiver::new(gen_progress_rx_inner),
        anim_update_rx,
        anim_command_tx: AnimationConductorHandle::new(anim_command_tx_inner),
        gen_command_tx,
        gen_response_rx,
        _gen_conductor: gen_conductor,
        _anim_rx: anim_rx,
        _anim_update_tx: anim_update_tx,
        _gen_resp_tx,
    })
}