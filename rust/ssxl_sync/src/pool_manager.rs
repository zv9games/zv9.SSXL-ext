use tokio::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, error};
use tokio::runtime;
// Re-import the crossbeam channel for the handshake, since the new module doesn't use the top-level lib imports.
use crossbeam_channel::unbounded;

use ssxl_shared::tile_data::AnimationUpdate;
use ssxl_shared::chunk_data::ChunkData;
use ssxl_math::Vec2i;

// Import the types defined in the primitives module, including the new state object.
use super::primitives::{AnimationConductor, AnimationCommand, AnimationState, AnimationStatus};

// 🛑 REMOVED: Godot imports are no longer necessary since the handle is not stored/sent.
// use godot::obj::Gd;
// use godot::classes::Node; 

// --- Worker Thread Internal State ---

/// Persistent state held by the animation worker thread.
struct ChunkAnimationState {
    /// Registry of all chunks the animation worker is actively tracking.
    /// Key: Chunk's world-space coordinates. Value: Arc to the chunk's data.
    registry: HashMap<Vec2i, Arc<ChunkData>>,
    current_frame_index: i32,
    total_frames: i32,
    // 🛑 REMOVED: This field contained Gd<Node>, which is not Send, causing the E0277 error.
    // tilemap_handle: Option<Gd<Node>>, 
}

// ----------------------------------------

impl AnimationConductor {
    /// Fully implements the worker thread to process animation commands and send updates.
    /// This function is now **blocking** until the worker thread has successfully started its Tokio runtime.
    ///
    /// FIX: Function now returns the thread-safe AnimationState object to the caller.
    pub fn new() -> Result<(Self, mpsc::UnboundedReceiver<AnimationUpdate>, AnimationState), String> { // <--- UPDATED SIGNATURE
        // 1. Command Channel (Main thread sends, Worker thread receives)
        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel();

        // 2. Update Channel (Worker thread sends, Main thread receives)
        let (update_tx, update_rx) = mpsc::unbounded_channel();
        
        // 3. Handshake Channel: Used to confirm the worker thread is running.
        let (ready_tx, ready_rx) = unbounded();

        // 4. Create the shared state object
        let state_handle = AnimationState::new();
        let state_for_worker = state_handle.clone(); // Clone for worker thread (Moved into thread::spawn)

        // 5. Spawn a dedicated worker thread
        // This is the line where the Send requirement is enforced:
        thread::spawn(move || {
            // 6. Create a dedicated Tokio runtime for this worker
            let rt = runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime for animation worker.");
            
            // FIX (DEADLOCK): Signal the main thread that the runtime is active and we are ready.
            // This must be done BEFORE rt.block_on(async move { loop { ... } }) to unblock the main thread.
            if let Err(e) = ready_tx.send(()) {
                error!("Failed to send ready signal during worker startup: {:?}", e);
                state_for_worker.set_status(AnimationStatus::Error); // Set error status on failure (using state_for_worker)
                return;
            }

            // Clone the state handle for the async block, so state_for_worker remains for final synchronous cleanup.
            let state_for_async = state_for_worker.clone();
            
            rt.block_on(async move { // <--- state_for_async moved here
                use tokio::time::{self, Duration, MissedTickBehavior};

                info!("Animation Worker thread started and awaiting commands.");

                // Initialize the persistent worker state
                let mut state = ChunkAnimationState {
                    registry: HashMap::new(),
                    current_frame_index: 0,
                    total_frames: 4,
                    // 🛑 REMOVED: tilemap_handle field initialization
                };

                let mut is_running = false;
                let mut frame_duration = Duration::from_millis(100); // Default 10 FPS
                
                // Initialize interval timer
                let mut interval = time::interval(frame_duration);
                interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

                loop {
                    // Use tokio::select! to concurrently wait for commands AND the next animation tick.
                    tokio::select! {
                        // 1. Wait for the next command from the main thread (Godot)
                        cmd = cmd_rx.recv() => {
                            match cmd {
                                Some(AnimationCommand::Start) => {
                                    is_running = true;
                                    state_for_async.set_status(AnimationStatus::Running); // <--- Use state_for_async
                                    // Consume the immediate tick created by `interval.new()` to start the timer properly
                                    let _ = interval.tick().await;
                                    info!("Animation Worker: Received START command. Loop active.");
                                }
                                Some(AnimationCommand::Stop) => {
                                    is_running = false;
                                    state_for_async.set_status(AnimationStatus::Paused); // <--- Use state_for_async
                                    info!("Animation Worker: Received STOP command. Loop paused.");
                                }
                                Some(AnimationCommand::Complete) => { // <--- ADDED: Handle shutdown command
                                    info!("Animation Worker: Received COMPLETE command. Shutting down.");
                                    state_for_async.set_status(AnimationStatus::ShuttingDown); // <--- Use state_for_async
                                    return;
                                }
                                Some(AnimationCommand::UpdateFramerate(fps)) => {
                                    if fps > 0.0 {
                                        frame_duration = Duration::from_secs_f32(1.0 / fps);
                                        // Recreate interval with new duration
                                        interval = time::interval(frame_duration);
                                        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
                                        let _ = interval.tick().await; // Consume the immediate tick of the new interval
                                        info!("Animation Worker: Framerate set to {} FPS.", fps);
                                    }
                                }
                                // ✅ CRITICAL FIX: Match the unit variant (no tilemap_handle argument)
                                Some(AnimationCommand::StartTestAnimation) => {
                                    info!("Animation Worker: Received StartTestAnimation command. Starting test animation.");
                                    // 🛑 REMOVED: state.tilemap_handle = Some(tilemap_handle);
                                    state_for_async.set_status(AnimationStatus::Running);
                                    is_running = true;
                                }
                                Some(AnimationCommand::StopTestAnimation) => {
                                    info!("Animation Worker: Received StopTestAnimation command. Stopping test animation.");
                                    state_for_async.set_status(AnimationStatus::Paused);
                                    is_running = false;
                                    // 🛑 REMOVED: state.tilemap_handle = None;
                                }

                                Some(AnimationCommand::RegisterChunk(chunk)) => {
                                    info!("Animation Worker: Registering chunk bounds: {:?}", chunk.bounds);
                                
                                    // Manual conversion: Pull x/y from Coord2D and build Vec2i
                                    let chunk_key = Vec2i::new(chunk.bounds.min.x, chunk.bounds.min.y);
                                
                                    state.registry.insert(chunk_key, Arc::clone(&chunk));
                                    info!("Animation Worker: Registered {} chunks.", state.registry.len());
                                }

                                None => {
                                    state_for_async.set_status(AnimationStatus::ShuttingDown); // <--- Use state_for_async
                                    info!("Animation Worker: Command channel closed. Shutting down.");
                                    return; // Command channel closed, shut down worker.
                                }
                            }
                        }
                        
                        // 2. Perform animation tick only when 'is_running' is true
                        _ = interval.tick(), if is_running => {
                            // Animation State Update (The actual animation logic)

                            // Update global frame state
                            state.current_frame_index = (state.current_frame_index + 1) % state.total_frames;
                            
                            // Converted i32 to i64 using .into()
                            let new_atlas_coords = Vec2i::new(state.current_frame_index.into(), 10i64);

                            // Iterate over all registered chunks to generate updates
                            for (chunk_coord, _chunk_data) in state.registry.iter() {
                                // Placeholder Logic: Send one update per chunk using the chunk's coordinate
                                let tile_coord = *chunk_coord;
                                
                                let update = AnimationUpdate {
                                    tile_coords: tile_coord,
                                    new_atlas_coords,
                                    layer: 0,
                                    source_id: 0,
                                };

                                // Send update to be read by Godot
                                if update_tx.send(update).is_err() {
                                    info!("Animation update channel closed. Shutting down worker.");
                                    return; // Update channel closed, shut down worker.
                                }
                            }
                        }
                    }
                }
            });
            // This line is now only reached if the inner loop exits (which only happens on 'Complete' or channel close)
            state_for_worker.set_status(AnimationStatus::ShuttingDown); // Final status check (uses original state_for_worker)
            info!("Animation Worker thread gracefully shut down.");
        });

        // 7. Main Thread: Wait for the ready signal from the worker thread.
        match ready_rx.recv() {
            Ok(_) => info!("*** ANIMATION CONDUCTOR: Worker thread confirmed running. ***"),
            Err(e) => {
                state_handle.set_status(AnimationStatus::Error); // Set error status on join/recv failure
                return Err(format!("Failed to receive ready signal from worker: {:?}", e));
            }
        }

        // 8. Main Thread: The auto-start logic is intentionally commented out.

        // Return the Conductor and the update receiver to the main thread, along with the state handle.
        Ok((AnimationConductor { command_sender: cmd_tx }, update_rx, state_handle)) // <--- UPDATED RETURN VALUE
    }

    pub fn send_command(&self, command: AnimationCommand) -> Result<(), mpsc::error::SendError<AnimationCommand>> {
        self.command_sender.send(command)
    }
}