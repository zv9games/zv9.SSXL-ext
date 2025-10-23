//C:/ZV9/zv9.aetherion/rust/src/util/time.rs

#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use std::time::{Duration, Instant};

/// ⏲️ Manages fixed-rate ticking for runtime systems.
/// Tracks elapsed time and determines when a tick should occur.
pub struct TickTimer {
    last_tick: Instant,
    tick_rate: Duration,
}

impl TickTimer {
    /// Creates a new TickTimer with the given tick rate (ticks per second).
    pub fn new(ticks_per_second: u32) -> Self {
        Self {
            last_tick: Instant::now(),
            tick_rate: Duration::from_secs_f64(1.0 / ticks_per_second as f64),
        }
    }

    /// Returns true if enough time has passed to trigger a tick.
    /// Resets the internal timer if a tick occurs.
    pub fn should_tick(&mut self) -> bool {
        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
            true
        } else {
            false
        }
    }

    /// Returns the duration since the last tick.
    pub fn time_since_last_tick(&self) -> Duration {
        self.last_tick.elapsed()
    }

    /// Returns the configured tick rate as a duration.
    pub fn tick_interval(&self) -> Duration {
        self.tick_rate
    }
}



#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_tick_interval_accuracy() {
        let timer = TickTimer::new(60); // 60 ticks/sec
        let expected = Duration::from_millis(16);
        let actual = timer.tick_interval();
        assert!((actual.as_millis() as i64 - expected.as_millis() as i64).abs() <= 1);
    }

    #[test]
    fn stress_should_tick_behavior() {
        let mut timer = TickTimer::new(10); // 100ms interval
        std::thread::sleep(Duration::from_millis(120));
        assert!(timer.should_tick());
    }

    #[test]
    fn stress_should_tick_false_when_too_soon() {
        let mut timer = TickTimer::new(100); // 10ms interval
        std::thread::sleep(Duration::from_millis(5));
        assert!(!timer.should_tick());
    }

    #[test]
    fn stress_multiple_tick_cycles() {
        let mut timer = TickTimer::new(20); // 50ms interval
        let mut ticks = 0;

        for _ in 0..5 {
            std::thread::sleep(Duration::from_millis(55));
            if timer.should_tick() {
                ticks += 1;
            }
        }

        assert_eq!(ticks, 5);
    }

    #[test]
    fn stress_time_since_last_tick() {
        let timer = TickTimer::new(60);
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.time_since_last_tick();
        assert!(elapsed >= Duration::from_millis(10));
    }
}


//end time.rs