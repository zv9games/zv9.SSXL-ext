// rust/SSXL-ext/src/animate_conductor.rs
use crate::godot_print;
use flume::{Receiver, Sender};
use crate::animate_events::AnimationEvent;
use crate::animate_worker::AnimationWorker;
use crate::shared_config::AnimationConfig; // Note: This struct needs the field

/// Messages sent from the main thread (Conductor) to the worker threads.
#[derive(Debug)]
pub enum ControlMessage {
    /// Instructs the worker to pause its simulation loop.
    Pause,
    /// Instructs the worker to immediately shut down its thread.
    Stop,
    /// Instructs the worker to update its internal configuration.
    UpdateConfig(AnimationConfig), 
}

// rust/SSXL-ext/src/animate_conductor.rs

/// Orchestrates the background animation worker pool and manages communication.
pub struct AnimationConductor {
    // The channel receiving AnimationEvents from all workers (used by host_anim.rs).
    pub event_receiver: Receiver<AnimationEvent>, 
    
    // Senders used to dispatch control messages (Pause/Stop) to each worker.
    control_senders: Vec<Sender<ControlMessage>>,

    // The handles to the actual worker threads, managed by the conductor.
    workers: Vec<AnimationWorker>,
}

// rust/SSXL-ext/src/animate_conductor.rs

impl AnimationConductor {
    /// Initializes the conductor and launches the animation worker threads.
    pub fn new(config: &AnimationConfig) -> Self {
        // FIX: The necessary field (`animation_worker_count`) is missing from AnimationConfig.
        // Temporarily use a hardcoded default value of 4 to allow compilation.
        // This should be replaced with a proper field access once AnimationConfig is corrected.
        let num_workers = 4;
        
        godot_print!("AnimationConductor: Using default worker count of {} due to missing config field.", num_workers);
        
        // MPSC Channel setup: Workers are producers, Conductor is the single consumer (event_receiver)
        let (event_sender, event_receiver) = flume::unbounded();

        let mut workers = Vec::with_capacity(num_workers);
        let mut control_senders = Vec::with_capacity(num_workers);

        for id in 0..num_workers {
            // SPSC Channel setup: Conductor is the producer, Worker is the consumer
            let (control_sender, control_receiver) = flume::unbounded();
            
            let worker = AnimationWorker::new(
                id, 
                event_sender.clone(), // Clone the event sender for each worker
                control_receiver,
                config.clone(),
            );
            
            workers.push(worker);
            control_senders.push(control_sender);
        }

        godot_print!("Animation Conductor: Launched {} dedicated animation workers.", num_workers);
        Self { 
            event_receiver, 
            control_senders, 
            workers 
        }
    }

    /// Public method to command all workers to stop immediately and shut down.
    pub fn shutdown(self) {
        // ... (shutdown logic remains the same)
        godot_print!("Animation Conductor: Initiating graceful shutdown of workers.");
        
        // 1. Signal all workers to STOP
        for sender in self.control_senders.iter() {
            // Use try_send in case the worker has already disconnected
            let _ = sender.try_send(ControlMessage::Stop); 
        }

        // 2. Wait for all worker threads to join
        for worker in self.workers {
            worker.join();
        }

        godot_print!("Animation Conductor: All animation workers shut down.");
    }

    /// Exposes the main receiver to the Host Anim Poller.
    pub fn get_event_receiver(&self) -> &Receiver<AnimationEvent> {
        &self.event_receiver
    }
    
    /// Sends a Pause message to all workers.
    pub fn pause_workers(&self) {
        for sender in self.control_senders.iter() {
            let _ = sender.try_send(ControlMessage::Pause);
        }
    }
}