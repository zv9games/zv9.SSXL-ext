//C:/ZV9/zv9.aetherion/rust/src/zv9_util_profiling.rs
#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use std::time::{Duration, Instant};

/// ⏱ Lightweight profiling timer for scoped diagnostics.
pub struct Profiler {
    label: &'static str,
    start: Instant,
}

impl Profiler {
    /// Starts a new profiling session with a label.
    pub fn start(label: &'static str) -> Self {
        Self {
            label,
            start: Instant::now(),
        }
    }

    /// Ends the profiling session and prints the elapsed time.
    pub fn end(self) {
        let duration = self.start.elapsed();
        println!("[⏱ {}] took {:.2?}", self.label, duration);
        // Optional: record to Trailkeeper or diagnostics overlay
    }
}

/// 🧩 Scoped profiling macro for inline measurement.
#[macro_export]
macro_rules! profile_scope {
    ($label:expr) => {
        let _profiler = crate::util::profiling::Profiler::start($label);
    };
}

/// 🚦 Frame budget tracker for performance thresholds.
pub struct FrameBudget {
    pub max_duration: Duration,
}

impl FrameBudget {
    /// Returns true if the elapsed time exceeds the budget.
    pub fn is_exceeded(&self, elapsed: Duration) -> bool {
        elapsed > self.max_duration
    }
}


#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_profiler_duration_accuracy() {
        let profiler = Profiler::start("test");
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = profiler.start.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn stress_profiler_end_output() {
        let profiler = Profiler::start("output_check");
        std::thread::sleep(Duration::from_millis(5));
        profiler.end(); // Should print timing info
    }

    #[test]
    fn stress_frame_budget_enforcement() {
        let budget = FrameBudget {
            max_duration: Duration::from_millis(16),
        };

        let short = Duration::from_millis(10);
        let long = Duration::from_millis(20);

        assert!(!budget.is_exceeded(short));
        assert!(budget.is_exceeded(long));
    }

    #[test]
    fn stress_multiple_profilers() {
        for i in 0..5 {
            let label = match i {
                0 => "load",
                1 => "tick",
                2 => "render",
                3 => "sync",
                _ => "finalize",
            };
            let profiler = Profiler::start(label);
            std::thread::sleep(Duration::from_millis(2));
            profiler.end();
        }
    }
}



//end profiling.rs