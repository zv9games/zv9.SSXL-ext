// rust/SSXL-ext/src/animate_worker.rs

use std::thread;
use std::time::{Duration, Instant};
use flume::Sender;
use crate::animate_events::AnimationEvent;
use crate::shared_config::AnimationConfig;
use crate::animate_conductor::ControlMessage; // For rhythmic control

// --- FIX: Import Godot macros from the crate root ---
use crate::{godot_print, godot_warn}; 

/// Represents a single thread dedicated to running a continuous simulation.
pub struct AnimationWorker {
    id: usize,
    // FIX 1: Remove these fields. They were moved into the thread closure and are not accessible 
    // from the main thread (where AnimationWorker lives) after spawning.
    // event_sender: Sender<AnimationEvent>, 
    // control_receiver: flume::Receiver<ControlMessage>,
    handle: Option<thread::JoinHandle<()>>,
}

impl AnimationWorker {
    pub fn new(
        id: usize, 
        // FIX 2: These parameters are intended to be moved directly into the thread.
        event_sender: Sender<AnimationEvent>, // Moved into thread
        control_receiver: flume::Receiver<ControlMessage>, // Moved into thread
        initial_config: AnimationConfig, // Initial simulation parameters
    ) -> Self {
        let handle = thread::spawn(move || {
            // Worker state: Simulation parameters and internal state
            let mut current_config = initial_config; // FIX 1: Make config mutable in the closure
            let mut simulation_state = init_simulation_state(&current_config);
            
            let mut target_fps = current_config.simulation_fps;
            let mut target_frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
            
            godot_print!("Anim Worker {}: Started with target FPS: {}", id, target_fps);
            
            let mut is_running = true;
            while is_running {
                let frame_start_time = Instant::now();

                // 1. Check for Control Messages (Non-blocking check)
                if let Ok(msg) = control_receiver.try_recv() {
                    match msg {
                        ControlMessage::Pause => is_running = false,
                        ControlMessage::Stop => break, // Exit the loop entirely
                        
                        // FIX 2: Handle config updates and recalculate timing
                        ControlMessage::UpdateConfig(new_config) => {
                            godot_print!("Anim Worker {}: Applying new config. New FPS: {}", id, new_config.simulation_fps);
                            current_config = new_config;
                            target_fps = current_config.simulation_fps;
                            target_frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
                            // TODO: Re-initialize or adjust simulation_state based on other config changes
                        }
                    }
                }
                
                if is_running {
                    // 2. Execute the Simulation Step
                    // Pass the current config for frame-specific parameters (if needed)
                    let new_events = run_simulation_step(&mut simulation_state, &current_config); 
                    
                    // 3. Deliver Results (The Finisher Payload)
                    for event in new_events {
                        if event_sender.send(event).is_err() { // event_sender is used successfully inside the thread
                            godot_warn!("Anim Worker {}: Conductor channel disconnected. Shutting down.", id);
                            break;
                        }
                    }
                }

                // 4. Enforce Simulation Rhythm (Sleep to hit target FPS)
                let elapsed_time = frame_start_time.elapsed();
                if elapsed_time < target_frame_duration {
                    thread::sleep(target_frame_duration - elapsed_time);
                }
            }
            godot_print!("Anim Worker {} finished loop and exiting thread.", id);
        });

        // FIX 3: event_sender and control_receiver were moved above, so we cannot use them here.
        AnimationWorker { id, handle: Some(handle) } 
    }
    
    // Safety: Necessary for clean shutdown during host_cleanup.rs
    pub fn join(mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

// rust/SSXL-ext/src/animate_worker.rs

use std::collections::VecDeque;

// Dummy struct to represent the internal state of a simulation (e.g., the fluid grid).
struct SimulationState {
    // ... complex data structures for CA, fluid, etc.
    last_update_time: Instant,
}

fn init_simulation_state(config: &AnimationConfig) -> SimulationState {
    // Setup initial data structures based on config
    SimulationState { last_update_time: Instant::now() }
}

/// Executes the core computational step for the simulation.
// FIX 3: Add config parameter to run_simulation_step, as it may be needed internally.
fn run_simulation_step(state: &mut SimulationState, _config: &AnimationConfig) -> VecDeque<AnimationEvent> {
    // --- THIS IS THE OFFLINE COMPUTATION ---
    // Example: Calculate the next frame of the fluid simulation.
    // Example: Determine if a particle needs to move or spawn an effect.
    // ... heavy math operations ...

    // Generate output events only when visual changes occur.
    let now = Instant::now();
    let events = if (now - state.last_update_time) > Duration::from_millis(100) {
        state.last_update_time = now;
        // Example event generation
        let event = AnimationEvent::SetTileAnimation { 
            layer: 0, 
            coords: (10, 5), 
            frame_index: (now.elapsed().as_secs() % 4) as i32 
        };
        VecDeque::from([event])
    } else {
        VecDeque::new()
    };
    
    events
}