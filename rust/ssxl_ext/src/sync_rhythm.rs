// rust/SSXL-ext/src/sync_rhythm.rs

use std::time::{Instant, Duration};
use crate::generate_conductor_state::ConductorState;
use crate::host_state::get_host_state;
// --- FIX: Import logging macro from the crate root ---
use crate::{ssxl_info, ssxl_error};

// Define the interval at which the complex rhythm checks run (e.g., 4 times per second)
const RHYTHM_CHECK_INTERVAL: Duration = Duration::from_millis(250);

/// Manages the timing and synchronization points between the generation and simulation layers.
pub struct RhythmManager {
    last_check_time: Instant,
    // The current phase of the synchronization loop (if staggered)
    current_phase: RhythmPhase, 
}

/// Defines the stages in a multi-step synchronization cycle.
#[derive(Debug, Clone, Copy, PartialEq)]
enum RhythmPhase {
    Idle,
    CheckGenerationStatus,
    CleanUpStaleCaches,
    SyncAnimationStarts,
}

// rust/SSXL-ext/src/sync_rhythm.rs

impl RhythmManager {
    pub fn new() -> Self {
        RhythmManager {
            last_check_time: Instant::now(),
            current_phase: RhythmPhase::Idle,
        }
    }

    /// Checks if it's time to run a complex synchronization check.
    pub fn poll_rhythm(&mut self) {
        if self.last_check_time.elapsed() >= RHYTHM_CHECK_INTERVAL {
            self.last_check_time = Instant::now();
            self.execute_rhythm_check();
        }
    }

    /// Executes the staged synchronization logic.
    fn execute_rhythm_check(&mut self) {
        // Access the global state, where the conductors and caches reside
        let host_state = match get_host_state() {
            Ok(state) => state,
            Err(_) => {
                // If HostState isn't initialized, we can't run the rhythm check.
                ssxl_error!("RhythmManager tried to poll but HostState is uninitialized.");
                return;
            }
        };

        // Get the state container safely
        let generation_state = host_state.conductor.get_state_container();
        
        match self.current_phase {
            RhythmPhase::Idle => {
                // Check if any system needs attention
                if generation_state.get_state() == ConductorState::Finished {
                    self.current_phase = RhythmPhase::CheckGenerationStatus;
                } else {
                    // Nothing urgent, remain idle
                }
            },
            
            RhythmPhase::CheckGenerationStatus => {
                // Action: Generation is FINISHED. This is a synchronization point.
                ssxl_info!("Rhythm Check: Generation finished. Starting post-gen cleanup.");

                // 1. Flush caches that depend on generation tasks
                // NOTE: Assumes host_state has a public `noise_cache` field and it has a `clear` method.
                // host_state.noise_cache.clear(); 
                
                self.current_phase = RhythmPhase::SyncAnimationStarts;
            },
            
            RhythmPhase::SyncAnimationStarts => {
                // Action: Start the Animation workers now that the world is ready.
                ssxl_info!("Rhythm Check: Starting animation workers...");
                // host_state.anim_conductor.start_workers(); 

                self.current_phase = RhythmPhase::CleanUpStaleCaches;
            },

            RhythmPhase::CleanUpStaleCaches => {
                // Action: Run garbage collection/stale data cleanup
                // host_state.chunk_cache.prune_distant_chunks(host_state.player_position); 

                self.current_phase = RhythmPhase::Idle; // Cycle complete
            }
        }
    }
}