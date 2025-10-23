/// ✅ Suggestions for util/timer.rs

// 🔧 Add convenience methods:
//     - `fn elapsed_secs(&self) -> f64`
//     - `fn elapsed_millis(&self) -> u128`
//     - Useful for logging, diagnostics, and performance metrics

// 🧩 Add pause/resume support:
//     - `fn pause()`, `fn resume()` with internal offset tracking
//     - Enables controlled timing in animations or simulations

// 🚦 Improve precision control:
//     - Consider exposing `Instant` directly via `fn started_at() -> Instant`
//     - Useful for profiling or syncing with external systems

// 📚 Document intended use cases:
//     - Clarify that this is a monotonic timer for measuring durations
//     - Note that it’s not suitable for wall-clock or scheduled events

// 🧪 Add unit tests for `elapsed()` and `reset()`:
//     - Validate time progression and reset behavior
//     - Ensure consistency across platforms

// 🧼 Optional: Add `is_expired(duration: Duration)` helper:
//     - e.g. `fn is_expired(&self, timeout: Duration) -> bool`
//     - Useful for polling loops, timeouts, and pacing logic

// 🚀 Future: Add named or labeled timers:
//     - e.g. `pub struct LabeledTimer { label: String, timer: Timer }`
//     - Enables structured profiling or grouped diagnostics

// 🧠 Consider exposing timer to GDScript:
//     - Wrap in a Godot-friendly node or utility class
//     - Useful for runtime performance tracking or animation pacing


use std::time::{Duration, Instant};

/// A monotonic stopwatch for measuring elapsed time in Rust.
///
/// This utility is independent of Godot’s scene system and frame loop.
/// It is ideal for profiling, diagnostics, pacing logic, and internal cooldowns.
/// Not suitable for wall-clock time or scheduled events.
#[derive(Debug)]
pub struct Stopwatch {
    started: Instant,
    paused_at: Option<Instant>,
    offset: Duration,
}

impl Stopwatch {
    /// Creates and starts a new stopwatch.
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
            paused_at: None,
            offset: Duration::ZERO,
        }
    }

    /// Returns the total elapsed time, excluding any paused duration.
    pub fn elapsed(&self) -> Duration {
        match self.paused_at {
            Some(paused) => paused - self.started - self.offset,
            None => Instant::now() - self.started - self.offset,
        }
    }

    /// Returns elapsed time in seconds.
    pub fn elapsed_secs(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }

    /// Returns elapsed time in milliseconds.
    pub fn elapsed_millis(&self) -> u128 {
        self.elapsed().as_millis()
    }

    /// Resets the stopwatch to zero.
    pub fn reset(&mut self) {
        self.started = Instant::now();
        self.paused_at = None;
        self.offset = Duration::ZERO;
    }

    /// Pauses the stopwatch.
    pub fn pause(&mut self) {
        if self.paused_at.is_none() {
            self.paused_at = Some(Instant::now());
        }
    }

    /// Resumes the stopwatch from a paused state.
    pub fn resume(&mut self) {
        if let Some(paused) = self.paused_at.take() {
            self.offset += Instant::now() - paused;
        }
    }

    /// Returns the original start time.
    pub fn started_at(&self) -> Instant {
        self.started
    }

    /// Checks if the stopwatch has exceeded a given duration.
    pub fn is_expired(&self, timeout: Duration) -> bool {
        self.elapsed() >= timeout
    }

    /// Returns whether the stopwatch is currently paused.
    pub fn is_paused(&self) -> bool {
        self.paused_at.is_some()
    }
}
