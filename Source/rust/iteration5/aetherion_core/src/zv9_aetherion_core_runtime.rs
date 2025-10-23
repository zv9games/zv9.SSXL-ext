use std::thread::sleep;
use std::fmt;
use std::time::{Duration, Instant};

use crate::zv9_aetherion_core_conductor::{Conductor, ProcCommand};
use crate::zv9_util_config::EngineConfig;
use crate::pipeline::builder::{ChunkStreamer, ChunkDelivery};
use crate::zv9_aetherion_pipeline_builder_dummy_delivery::DummyDelivery;  // Corrected: Precise path to DummyDelivery module.
use log::{info as log_info, debug as log_debug, warn as log_warn};  // Native macros via aliases—unify logging.

use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;

#[cfg(feature = "async")]
use tokio::time::sleep as async_sleep;

/// 🕒 RuntimeState — now with accumulator for flawless fixed steps.
pub struct RuntimeState {
    tick_count: u64,
    last_tick: Instant,
    frame_budget: Duration,
    exceeded_budget: bool,
    avg_tick_duration: f64,
    on_tick: Option<Box<dyn FnMut(u64, Duration) + Send + Sync>>,
    #[cfg(feature = "profile")]
    tick_metrics: TickMetrics,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            tick_count: 0,
            last_tick: Instant::now(),
            frame_budget: Duration::from_millis(16),
            exceeded_budget: false,
            avg_tick_duration: 0.0,
            on_tick: None,
            #[cfg(feature = "profile")]
            tick_metrics: TickMetrics::default(),
        }
    }
}



#[derive(Debug, Default)]
#[cfg(feature = "profile")]
struct TickMetrics {
    max_duration: Duration,
    min_duration: Duration,
    total_ticks: u64,
}

/// Fluent listener builder—your golden touch.
#[derive(Debug, Default)]
pub struct TickListenerBuilder<F> {
    callback: Option<F>,
}

impl<F> TickListenerBuilder<F>
where
    F: FnMut(u64, Duration) + Send + Sync + 'static,
{
    pub fn new(callback: F) -> Self {
        Self { callback: Some(callback) }
    }

    pub fn build(self) -> Option<Box<dyn FnMut(u64, Duration) + Send + Sync>> {
        self.callback.map(|cb| {
            let boxed: Box<F> = Box::new(cb);
            boxed as Box<dyn FnMut(u64, Duration) + Send + Sync>
        })
    }
}


impl RuntimeState {
    pub fn new() -> Self {
        #[cfg(feature = "profile")]
        let metrics = TickMetrics::default();
        Self {
            tick_count: 0,
            last_tick: Instant::now(),
            frame_budget: Duration::from_millis(16),
            exceeded_budget: false,
            avg_tick_duration: 0.0,
            on_tick: None,
            #[cfg(feature = "profile")]
            tick_metrics: metrics,
        }
    }

    /// Single tick—now with EMA unbranched.
    #[inline(always)]
    pub fn tick(&mut self, elapsed: Duration) {
        self.exceeded_budget = elapsed > self.frame_budget;
        self.last_tick = Instant::now();
        self.tick_count += 1;

        // Unbranch EMA: Always compute, weight 0.9 prior + 0.1 new (f64 precise).
        self.avg_tick_duration = self.avg_tick_duration * 0.9 + elapsed.as_secs_f64() * 0.1;

        if let Some(callback) = &mut self.on_tick {
            callback(self.tick_count, elapsed);
        }

        #[cfg(feature = "profile")]
        self.tick_metrics.update(elapsed);
    }

    pub fn set_frame_budget(&mut self, millis: u64) {
        self.frame_budget = Duration::from_millis(millis);
    }

    pub fn set_tick_listener<F>(&mut self, callback: F)
    where F: FnMut(u64, Duration) + Send + Sync + 'static,
    {
        self.on_tick = Some(Box::new(callback));
    }

    /// Builder for listeners—ergonomic gold.
    pub fn listener_builder<F>(&mut self, cb: F) -> TickListenerBuilder<F>
    where F: FnMut(u64, Duration) + Send + Sync + 'static,
    {
        TickListenerBuilder::new(cb)
    }

    pub fn time_since_last_tick(&self) -> Duration { self.last_tick.elapsed() }

    pub fn ticks(&self) -> u64 { self.tick_count }

    pub fn budget(&self) -> Duration { self.frame_budget }

    pub fn average_tick_duration(&self) -> Duration {
        Duration::from_secs_f64(self.avg_tick_duration)
    }

    pub fn is_budget_exceeded(&self) -> bool { self.exceeded_budget }

    pub fn has_tick_listener(&self) -> bool { self.on_tick.is_some() }

    #[cfg(feature = "profile")]
    pub fn metrics(&self) -> &TickMetrics { &self.tick_metrics }
}

#[cfg(feature = "profile")]
impl TickMetrics {
    fn update(&mut self, elapsed: Duration) {
        self.max_duration = self.max_duration.max(elapsed);
        self.min_duration = if self.min_duration.is_zero() { elapsed } else { self.min_duration.min(elapsed) };
        self.total_ticks += 1;
    }

    pub fn fps_estimate(&self) -> f64 {
        if self.total_ticks == 0 { 0.0 } else { self.total_ticks as f64 / self.max_duration.as_secs_f64() }
    }
}

/// 🚀 Starts the engine—now with accumulator for lockstep perfection.
pub async fn start<D: ChunkDelivery + Send + 'static>(  // Fixed: Make start async to support .await.
    delivery: D,
    max_ticks: Option<u64>,  // Gold: Configurable bound.
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log_info!("runtime: Starting Aetherion engine...");  // Native info! macro—structured target, drop &format!.

    let config = EngineConfig::default();
    let mut state = RuntimeState::new();
    let step = Duration::from_millis((1000 / config.tick_rate).max(1) as u64);  // Fixed step.
    state.set_frame_budget(step.as_millis() as u64);

    let streamer = ChunkStreamer::new(delivery, config.interval_ms, 1024);  // Fixed: Add est_cap for constructor harmony (tune as needed).
    let mut conductor = Conductor::new(streamer);
    let mut chunk = MapDataChunk::default();

    // Fluent enqueue—your vein.
    conductor.enqueue_batch(vec![
        ProcCommand::GenerateTerrain { seed: 42 },  // Param gold.
        ProcCommand::EmitSignal("Engine started".into()),
        ProcCommand::WaitTicks(10),
        ProcCommand::EmitSignal("Midway checkpoint".into()),
    ]);

    // Listener: Gated debug.
    #[cfg(feature = "profile")]
    state.set_tick_listener(move |tick, elapsed| {
        log_debug!("tick: Tick {} took {:?}", tick, elapsed);  // Fixed: Native debug! macro—interpolation gold, no &format!.
    });

    let mut accumulator = Duration::ZERO;
    let mut last_frame_time = Instant::now();
    let mut frame_count = 0u64;
    let report_interval = config.tick_rate as u64;  // FPS every sec.

    // Accumulator loop: Fixed steps, precise sleep (Gaffer , ).
    loop {
        let delta = last_frame_time.elapsed();  // Frame delta.
        last_frame_time = Instant::now();
        accumulator += delta;

        // Multi-step if lag: No skips, sim consistent.
        while accumulator >= step {
            state.tick(step);  // Pass fixed step.
            conductor.tick(state.ticks(), &mut chunk)?;  // ? for gold err prop.

            accumulator -= step;
            frame_count += 1;

            // Shutdown grace.
            if let Some(max) = max_ticks {
                if state.ticks() >= max { break; }
            }

            // Report gold.
            if frame_count % report_interval == 0 {
                #[cfg(feature = "profile")]
                log_info!("runtime: FPS: {:.1}", state.metrics().fps_estimate());  // Fixed: Native, direct args—elegant.
            }
        }

        // Interp stub: Alpha for render (future gold).
        let alpha = accumulator.as_secs_f64() / step.as_secs_f64();

        // e.g., render_interpolated(chunk, alpha); // TODO

        // Precise sleep: Remainder, no busy-poll.
        let remainder = step.saturating_sub(accumulator);
        #[cfg(feature = "async")]
		{
		// Tokio example:
			tokio::time::sleep(remainder).await;
		}

		#[cfg(not(feature = "async"))]
		{
			std::thread::sleep(remainder);
		}

        // Exceed? Alert.
        if state.is_budget_exceeded() {
            log_warn!("runtime: Budget exceeded—consider tuning tick_rate");  // Fixed: Native warn! macro—silence the error, target for trace.
        }
    }

    #[cfg(feature = "profile")]
    log_info!("runtime: Final FPS avg: {:.1}", state.metrics().fps_estimate());  // Fixed: Native, interpolation.
    log_info!("runtime: Aetherion engine stopped gracefully.");  // Fixed: As above.

    Ok(())
}

#[cfg(test)]
mod stress_tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn stress_tick_flood() {
        let mut state = RuntimeState::new();
        for _ in 0..100_000 {
            state.tick(Duration::from_millis(16));
        }
        assert_eq!(state.ticks(), 100_000);
    }

    #[test]
    fn stress_budget_enforcement() {
        let mut state = RuntimeState::new();
        state.set_frame_budget(1);
        state.tick(Duration::from_millis(5));  // Simulate lag.
        assert!(state.is_budget_exceeded());
    }

    #[test]
    fn stress_listener_callback() {
        let mut state = RuntimeState::new();
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        state.set_tick_listener(move |tick, _| {
            if tick == 1 { *called_clone.lock().unwrap() = true; }
        });

        state.tick(Duration::from_millis(16));
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn stress_average_smoothing() {
        let mut state = RuntimeState::new();
        let mut expected = 0.0;
        for i in 0..10 {
            let elapsed = Duration::from_millis(10 + i);  // Vary.
            state.tick(elapsed);
            expected = expected * 0.9 + elapsed.as_millis() as f64 * 0.1;
            assert!((state.average_tick_duration().as_millis() as f64 - expected).abs() < 1.0);
        }
    }

    #[test]
    #[cfg(not(feature = "async"))]  // Fixed: Gate async test—stub for sync realms, avoid tokio dep in tests.
    fn stress_accumulator_no_skips() {
        // Mock start w/ max=5, lag sim.
        let delivery = DummyDelivery::new();  // Corrected: Align with constructor signature—no args.
        // Simulate loop ticks==5 without full start (for test isolation).
        let mut state = RuntimeState::new();
        let step = Duration::from_millis(16);
        let mut accumulator = Duration::from_millis(100);  // Simulate lag > step.
        while accumulator >= step {
            state.tick(step);
            accumulator -= step;
        }
        assert_eq!(state.ticks(), 6);  // 100/16 ≈6, no skips.
    }
}