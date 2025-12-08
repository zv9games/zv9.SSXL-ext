// ============================================================================
// 🎼 Genesis Channel and State Setup (`crate::genesis`)
// ----------------------------------------------------------------------------
// This module wires together the foundational communication channels and state
// objects required to initialize the SSXL engine. It bundles conductor and
// animation subsystems into a unified "genesis kit" for orchestration.
//
// Purpose:
//   • Establish all channels for generation and animation communication.
//   • Initialize conductor and animation state objects.
//   • Provide a single struct (`GenesisHandles`) that encapsulates all handles.
//   • Simplify engine startup by returning a fully wired configuration.
//
// Key Components:
//   • Type Aliases
//       - AnimationUpdateReceiver: channel receiver for animation updates.
//       - GenerationCommandTx: channel sender for generation commands.
//       - GenerationResponseRx: channel receiver for generation responses.
//       - These aliases improve readability of channel signatures.
//
//   • GenesisHandles (struct)
//       - Bundles all handles, channels, and state objects created during setup.
//       - Fields:
//           • gen_state: snapshot of conductor state.
//           • anim_state: snapshot of animation state.
//           • gen_progress_rx: receiver for generation progress updates.
//           • anim_update_rx: receiver for animation updates.
//           • anim_command_tx: sender for animation commands.
//           • gen_command_tx: sender for generation commands.
//           • gen_response_rx: receiver for generation responses.
//           • _gen_conductor: internal conductor instance (not exposed).
//           • _anim_rx: internal receiver for animation commands.
//           • _anim_update_tx: internal sender for animation updates.
//           • _gen_resp_tx: internal sender for generation responses.
//
//   • execute_channel_and_state_setup (function)
//       - Initializes all channels and state objects required for the engine.
//       - Arguments:
//           • config_path: optional path to configuration file.
//       - Workflow:
//           1. Load configuration from path.
//           2. Create generation command + response channels.
//           3. Initialize conductor via `Conductor::new_for_ffi`.
//           4. Create animation command + update channels.
//           5. Initialize animation state.
//           6. Log successful initialization.
//           7. Return `GenesisHandles` with all handles and channels wired.
//
// Design Choices:
//   • Separation of conductor and animation subsystems ensures modularity.
//   • Internal fields (`_gen_conductor`, `_anim_rx`, etc.) are retained for
//     lifecycle management but hidden from external API.
//   • Logging provides visibility into initialization success.
//   • Returning a single struct simplifies downstream orchestration.
//
// Educational Note:
//   • This module demonstrates how Rust can bundle complex initialization
//     logic into a clean, ergonomic API. By combining channels, state objects,
//     and conductor instances into `GenesisHandles`, it provides a reproducible
//     and maintainable foundation for engine startup.
// ============================================================================


use tokio::sync::mpsc;
use ssxl_shared::config::config::get_config_from_path;
use ssxl_generate::{
    Conductor,
    ConductorProgressReceiver,
    conductor::ConductorState,
};
use ssxl_shared::AnimationConductorHandle;
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
    let _config = get_config_from_path(config_path);

    let (gen_command_tx, _gen_cmd_rx) = mpsc::unbounded_channel::<GenerationCommand>();
    let (_gen_resp_tx, gen_response_rx) = mpsc::unbounded_channel::<GenerationResponse>();

    let (
        gen_conductor,
        gen_state,
        _internal_gen_cmd_tx,
        _internal_gen_resp_rx,
        gen_progress_rx_inner,
    ) = Conductor::new_for_ffi(config_path)?;

    let (anim_command_tx_inner, anim_rx) = mpsc::unbounded_channel::<AnimationCommand>();
    let (anim_update_tx, anim_update_rx) = mpsc::unbounded_channel::<AnimationUpdate>();

    let anim_state = AnimationState::default();

    info!("Genesis engine initialized: Conductor + Animation systems ready.");

    Ok(GenesisHandles {
        gen_state,
        anim_state,
        gen_progress_rx: ConductorProgressReceiver::new(gen_progress_rx_inner),
        anim_update_rx,
        anim_command_tx: anim_command_tx_inner,

        gen_command_tx,
        gen_response_rx,
        _gen_conductor: gen_conductor,
        _anim_rx: anim_rx,
        _anim_update_tx: anim_update_tx,
        _gen_resp_tx,
    })
}