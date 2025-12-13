use std::thread;
use std::time::{Duration, Instant};
use flume::Sender;
use crate::animate_events::AnimationEvent;
use crate::shared_config::AnimationConfig;
use crate::animate_conductor::ControlMessage;
use crate::{ssxl_info, ssxl_warn}; // 🔥 FIX 1: Use custom logger macros
// use crate::{godot_print, godot_warn}; // REMOVED: Replaced by ssxl_*
use std::collections::VecDeque;

pub struct AnimationWorker {
    _id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl AnimationWorker {
    pub fn new(
        id: usize,
        event_sender: Sender<AnimationEvent>,
        control_receiver: flume::Receiver<ControlMessage>,
        initial_config: AnimationConfig,
    ) -> Self {
        let handle = thread::spawn(move || {
            let mut current_config = initial_config;
            let mut simulation_state = init_simulation_state(&current_config);
            
            let mut target_fps = current_config.simulation_fps;
            let mut target_frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
            
            // 🔥 FIX 2: Replaced godot_print! with ssxl_info!
            ssxl_info!("Anim Worker {}: Started with target FPS: {}", id, target_fps);
            
            let mut is_running = true;
            while is_running {
                let frame_start_time = Instant::now();

                if let Ok(msg) = control_receiver.try_recv() {
                    match msg {
                        ControlMessage::Pause => is_running = false,
                        ControlMessage::Stop => break,
                        ControlMessage::UpdateConfig(new_config) => {
                            // 🔥 FIX 3: Replaced godot_print! with ssxl_info!
                            ssxl_info!("Anim Worker {}: Applying new config. New FPS: {}", id, new_config.simulation_fps);
                            current_config = new_config;
                            target_fps = current_config.simulation_fps;
                            target_frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
                        }
                    }
                }
                
                if is_running {
                    let new_events = run_simulation_step(&mut simulation_state, &current_config);
                    
                    for event in new_events {
                        if event_sender.send(event).is_err() {
                            // 🔥 FIX 4: Replaced godot_warn! with ssxl_warn!
                            ssxl_warn!("Anim Worker {}: Conductor channel disconnected. Shutting down.", id);
                            break;
                        }
                    }
                }

                let elapsed_time = frame_start_time.elapsed();
                if elapsed_time < target_frame_duration {
                    thread::sleep(target_frame_duration - elapsed_time);
                }
            }
            // 🔥 FIX 5: Replaced godot_print! with ssxl_info!
            ssxl_info!("Anim Worker {} finished loop and exiting thread.", id);
        });

        AnimationWorker { _id: id, handle: Some(handle) }
    }
    
    pub fn join(mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

struct SimulationState {
    last_update_time: Instant,
}

fn init_simulation_state(_config: &AnimationConfig) -> SimulationState {
    SimulationState { last_update_time: Instant::now() }
}

fn run_simulation_step(state: &mut SimulationState, _config: &AnimationConfig) -> VecDeque<AnimationEvent> {
    let now = Instant::now();
    let events = if (now - state.last_update_time) > Duration::from_millis(100) {
        state.last_update_time = now;
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