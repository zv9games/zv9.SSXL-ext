use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum RhythmPhase {
    Idle,
    CheckGenerationStatus,
    CleanUpStaleCaches,
    SyncAnimationStarts,
}

pub struct RhythmManager {
    _last_check_time: Instant,
    _current_phase: RhythmPhase,
}

impl RhythmManager {
    pub fn new() -> Self {
        RhythmManager {
            _last_check_time: Instant::now(),
            _current_phase: RhythmPhase::Idle,
        }
    }
}