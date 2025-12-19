use std::time::{Instant, Duration};
use crate::generate_conductor_state::ConductorState;
use crate::host_state::get_host_state;
use crate::{ssxl_info, ssxl_error};

const RHYTHM_CHECK_INTERVAL: Duration = Duration::from_millis(250);

pub struct RhythmManager {
    last_check_time: Instant,
    current_phase: RhythmPhase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RhythmPhase {
    Idle,
    CheckGenerationStatus,
    CleanUpStaleCaches,
    SyncAnimationStarts,
}

impl RhythmManager {
    pub fn new() -> Self {
        RhythmManager {
            last_check_time: Instant::now(),
            current_phase: RhythmPhase::Idle,
        }
    }

    pub fn poll_rhythm(&mut self) {
        if self.last_check_time.elapsed() >= RHYTHM_CHECK_INTERVAL {
            self.last_check_time = Instant::now();
            self.execute_rhythm_check();
        }
    }

    fn execute_rhythm_check(&mut self) {
        let host_state = match get_host_state() {
            Ok(state) => state,
            Err(_) => {
                ssxl_error!("RhythmManager tried to poll but HostState is uninitialized.");
                return;
            }
        };

        let generation_state = host_state.conductor.get_state_container();

        match self.current_phase {
            RhythmPhase::Idle => {
                if generation_state.get_state() == ConductorState::Finished {
                    self.current_phase = RhythmPhase::CheckGenerationStatus;
                }
            }

            RhythmPhase::CheckGenerationStatus => {
                ssxl_info!("Rhythm Check: Generation finished. Starting post-gen cleanup.");
                self.current_phase = RhythmPhase::SyncAnimationStarts;
            }

            RhythmPhase::SyncAnimationStarts => {
                ssxl_info!("Rhythm Check: Starting animation workers...");
                self.current_phase = RhythmPhase::CleanUpStaleCaches;
            }

            RhythmPhase::CleanUpStaleCaches => {
                self.current_phase = RhythmPhase::Idle;
            }
        }
    }
}
