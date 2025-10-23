//! The core manager for the ssxl Engine, responsible for coordinating
//! generation, concurrency, and caching via the Tokio asynchronous runtime.

use tokio::runtime::{Runtime, Handle};
use tokio::sync::mpsc; 
use tracing::{info, error, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::io;

// NEW: Chunk size constant (assumed 64x64 from control_panel.gd)
const CHUNK_SIZE: usize = 64; 

// NEW: Cache and Math imports for Phase 6
use ssxl_cache::ChunkCache;
use ssxl_math::Vec2i;
use ssxl_math::prelude::ChunkKey; 
use glam::IVec3; 

// --- INTERNAL CRATE DEPENDENCIES ---
use crate::Generator;
use crate::perlin_generator::PerlinGenerator;
use crate::cellular_automata_generator::{
    CellularAutomataGenerator, 
    RULE_BASIC_CAVE, 
    RULE_MAZE
};

// --- EXTERNAL CRATE DEPENDENCIES ---
use ssxl_shared::chunk_data::ChunkData;
use ssxl_tools::get_config_from_path;

// FIX: Define a type alias that includes Send + Sync bounds for thread-safe trait objects
type DynGenerator = Box<dyn Generator + Send + Sync>;

// --- MPSC Channel Configuration ---
const PROGRESS_CHANNEL_BOUND: usize = 100;

// -----------------------------------------------------------------------------
// PHASE 8.4: GENERATION MESSAGES
// -----------------------------------------------------------------------------

/// Messages sent from the asynchronous generation task back to the main
/// thread (Godot's tick loop) for reporting progress and results.
#[derive(Debug)]
pub enum GenerationMessage {
    /// A general status update (e.g., "Initializing Noise", "Processing 5/100 Chunks").
    StatusUpdate(String),
    /// Reports the completion of a single chunk. Contains the chunk's coordinates.
    ChunkGenerated(Vec2i),
    /// Reports the final completion of the entire generation run.
    GenerationComplete,
    /// Reports a non-fatal error during generation.
    Error(String),
}


// -----------------------------------------------------------------------------
// PHASE 8.3: GENERATOR CONFIGURATION
// -----------------------------------------------------------------------------

/// Configuration data passed from the Godot API to the Conductor to start a
/// full map generation run.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub width: usize,
    pub height: usize,
    pub seed: String,
    pub generator_name: String,
}


// -----------------------------------------------------------------------------
// CONDUCTOR STATE AND STATUS (For Signal Inspector)
// -----------------------------------------------------------------------------

/// Represents the operational state of the Conductor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductorStatus {
    Initializing,
    Running,
    Paused,
    ShuttingDown,
    Error,
}

/// Shared, thread-safe state exposed to the CLI for monitoring.
#[derive(Clone)]
pub struct ConductorState {
    status: Arc<Mutex<ConductorStatus>>,
    // Represents the number of pending tasks in the generation queue
    queue_depth: Arc<AtomicUsize>, 
    // The ID of the currently active generation algorithm
    active_generator_id: Arc<Mutex<String>>,
}

impl ConductorState {
    pub fn new(initial_generator_id: String) -> Self {
        ConductorState {
            status: Arc::new(Mutex::new(ConductorStatus::Initializing)),
            queue_depth: Arc::new(AtomicUsize::new(0)),
            active_generator_id: Arc::new(Mutex::new(initial_generator_id)),
        }
    }

    // Public methods for the CLI to inspect the state
    pub fn get_status(&self) -> ConductorStatus {
        // Handle Mutex poisoning gracefully
        match self.status.lock() {
            Ok(guard) => *guard,
            Err(e) => {
                error!("Mutex poisoned when reading status: {}", e);
                ConductorStatus::Error
            }
        }
    }

    pub fn get_queue_depth(&self) -> usize {
        self.queue_depth.load(Ordering::Relaxed)
    }

    pub fn get_active_generator_id(&self) -> String {
        // Handle Mutex poisoning by unwrapping (less critical than status)
        self.active_generator_id.lock().unwrap().clone()
    }
    
    // Internal methods for the Conductor to update the state
    pub(crate) fn set_status(&self, new_status: ConductorStatus) {
        *self.status.lock().unwrap() = new_status;
    }
    
    pub(crate) fn set_active_generator_id(&self, id: &str) {
        *self.active_generator_id.lock().unwrap() = id.to_string();
    }
}

// -----------------------------------------------------------------------------
// CONDUCTOR MANAGER
// -----------------------------------------------------------------------------

/// The central manager for the procedural generation pipeline.
pub struct Conductor {
    runtime: Runtime,
    generators: HashMap<String, Arc<DynGenerator>>,
    internal_state: ConductorState,
    // PHASE 6: Thread-safe handle to the ChunkCache for persistence
    chunk_cache: Arc<Mutex<ChunkCache>>,
    // PHASE 8.4: The sender side of the progress channel. Cloned for each task.
    progress_sender: mpsc::Sender<GenerationMessage>,
}
            
impl Conductor {
    /// Initializes the Conductor, starts the runtime, creates the MPSC channel,
    /// and returns the MPSC Receiver for the FFI consumer (Godot) to poll.
    pub fn new(config_path: Option<&str>) -> Result<(Self, ConductorState, mpsc::Receiver<GenerationMessage>), io::Error> {
        // Load configuration first
        let config = get_config_from_path(config_path)?; 

        // --- Runtime Setup ---
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4) 
            .enable_all()
            .build()?;

        // --- Generator Registration ---
        let mut generators: HashMap<String, Arc<DynGenerator>> = HashMap::new();
        
        // 1. Perlin Generator (MVG)
        let perlin: DynGenerator = Box::new(PerlinGenerator::new(64.0));
        let default_perlin_id = perlin.id().to_string();
        generators.insert(default_perlin_id.clone(), Arc::new(perlin));

        // 2. Cellular Automata Generator (Cave)
        let ca_cave: DynGenerator = Box::new(CellularAutomataGenerator::new(RULE_BASIC_CAVE));
        generators.insert(ca_cave.id().to_string(), Arc::new(ca_cave));
        
        // 3. Cellular Automata Generator (Maze)
        let ca_maze: DynGenerator = Box::new(CellularAutomataGenerator::new(RULE_MAZE));
        generators.insert(ca_maze.id().to_string(), Arc::new(ca_maze));
        
        // --- Cache Initialization (PHASE 6) ---
        let chunk_cache = match ChunkCache::new() {
            Ok(c) => {
                info!("ChunkCache initialized successfully.");
                Arc::new(Mutex::new(c))
            },
            Err(e) => {
                error!("Failed to initialize ChunkCache: {:?}", e);
                return Err(io::Error::new(io::ErrorKind::Other, "Cache initialization failed"));
            }
        };
        
        // --- MPSC Channel Setup (PHASE 8.4) ---
        let (progress_sender, progress_receiver) = mpsc::channel(PROGRESS_CHANNEL_BOUND);

        // --- Conductor State Initialization ---
        let mut initial_id = default_perlin_id.clone();
        
        // Use configured default, falling back to Perlin if not found
        let config_id = config.get_default_generator_id();
        if generators.contains_key(config_id) {
            initial_id = config_id.to_string();
        } else {
            warn!("Config default generator ID '{}' not found. Falling back to Perlin: {}", config_id, default_perlin_id);
        }

        let state = ConductorState::new(initial_id.clone());
        state.set_status(ConductorStatus::Running);
        
        info!("Conductor initialized. Active generator: {}", initial_id);

        // Return the Conductor instance, the State, and the MPSC Receiver
        Ok((Conductor {
            runtime,
            generators,
            internal_state: state.clone(),
            chunk_cache,
            progress_sender,
        }, state, progress_receiver)) 
    }

    /// Public method to get a handle to the Runtime for task spawning.
    pub fn get_handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    /// Provides public read-access to the active generator ID.
    pub fn get_active_generator_id(&self) -> String {
        self.internal_state.get_active_generator_id()
    }
    
    // -------------------------------------------------------------------------
    // PHASE 8.3: CORE GENERATION COMMAND ENTRY POINT
    // -------------------------------------------------------------------------

    /// Handles the main map generation command from an FFI wrapper (like Godot).
    pub fn start_generation(&mut self, config: GeneratorConfig) -> Result<(), Box<dyn std::error::Error>> {
        info!("Conductor received main generation command. Config: {:?}", config);

        // 1. Set the active generator based on the config name
        match self.set_active_generator(&config.generator_name) {
            Ok(_) => info!("Generator successfully set to '{}'", config.generator_name),
            Err(e) => {
                error!("Failed to set generator: {}", e);
                return Err(e.into()); 
            }
        }
        
        // 2. Prepare for Asynchronous Generation
        let internal_state_clone = self.internal_state.clone();
        let runtime_handle = self.get_handle();
        let progress_sender_clone = self.progress_sender.clone(); // PHASE 8.4: Clone Sender for the task.

        runtime_handle.spawn(async move {
            info!("Async generation task spawned with config: {:?}", config);
            
            // Send initial status update
            let _ = progress_sender_clone.send(GenerationMessage::StatusUpdate("Starting generation task...".into())).await;

            // Increment queue depth to signal a task has started
            internal_state_clone.queue_depth.fetch_add(1, Ordering::Relaxed);

            // --- IMPLEMENTATION: FULL CHUNK LOOP ---
            // Calculate the grid dimensions in chunks (using ceiling division)
            let width_in_chunks = (config.width + CHUNK_SIZE - 1) / CHUNK_SIZE;
            let height_in_chunks = (config.height + CHUNK_SIZE - 1) / CHUNK_SIZE;
            let total_chunks = width_in_chunks * height_in_chunks;
            
            if total_chunks == 0 {
                error!("Generation failed: Calculated chunk count is zero for size {}x{}. Sending error signal.", config.width, config.height);
                let _ = progress_sender_clone.send(GenerationMessage::Error("Map dimensions are too small to contain a single chunk.".into())).await;
            } else {
                info!("Generation starting: {} chunks ({}x{}) to process.", total_chunks, width_in_chunks, height_in_chunks);
                
                // Outer loop (Y-axis)
                for chunk_y in 0..height_in_chunks {
                    // Inner loop (X-axis)
                    for chunk_x in 0..width_in_chunks {
                        let chunk_coords = Vec2i::new(chunk_x as i32, chunk_y as i32);
                        
                        // Emit signal to Godot main thread to trigger synchronous generate_chunk() call
                        let send_result = progress_sender_clone.send(
                            GenerationMessage::ChunkGenerated(chunk_coords)
                        ).await;
                        
                        if send_result.is_err() {
                            // If the receiver (Godot) dropped the channel, gracefully stop the work.
                            warn!("Progress channel disconnected. Stopping generation task.");
                            
                            // Decrease queue depth before exiting the task.
                            internal_state_clone.queue_depth.fetch_sub(1, Ordering::Relaxed);
                            return; 
                        }
                    }
                }
            }
            
            // Send final completion message
            let _ = progress_sender_clone.send(GenerationMessage::GenerationComplete).await;

            // Decrement queue depth on completion
            internal_state_clone.queue_depth.fetch_sub(1, Ordering::Relaxed);

            info!("Async generation task finished processing command: {:?}", config);
        });

        Ok(())
    }

    // --- Shutdown Management (The Fix) ---

    /// Signals the Conductor's internal state to begin a graceful shutdown process.
    pub fn signal_shutdown_graceful(&self) {
        info!("Aetherion Conductor signaled for shutdown. Setting status to ShuttingDown.");
        self.internal_state.set_status(ConductorStatus::ShuttingDown);
    }
    
    /// Performs a full, graceful teardown of the Conductor, signaling shutdown and
    /// stopping the underlying Tokio runtime. This method consumes `self`.
    pub fn graceful_teardown(self) {
        self.signal_shutdown_graceful();
        
        // This stops the Tokio runtime pool, freeing its resources.
        self.runtime.shutdown_background(); 
        info!("SSXL Conductor full teardown complete.");
    }
    
    // --- Generator Management & Core Pipeline ---

    /// Changes the algorithm used for subsequent generation tasks.
    pub fn set_active_generator(&mut self, id: &str) -> Result<(), String> {
        if self.generators.contains_key(id) {
            info!("Active generator set to: {}", id);
            self.internal_state.set_active_generator_id(id);
            Ok(())
        } else {
            let err = format!("Generator ID '{}' not found. Available IDs: {:?}", id, self.generators.keys());
            error!("{}", err);
            Err(err)
        }
    }
    
    /// The primary synchronous function used to generate a chunk (blocking for now).
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        let key_vec3 = IVec3::new(chunk_coords.x, chunk_coords.y, 0);
        let chunk_key = ChunkKey(key_vec3);

        // --- 1. Attempt to load from cache ---
        match self.chunk_cache.lock() {
            Ok(cache_lock) => {
                match cache_lock.load_chunk(&chunk_key) {
                    Ok(Some(data)) => {
                        info!("Conductor retrieved chunk {:?} (Key: {:?}) from cache.", chunk_coords, chunk_key);
                        return data;
                    },
                    Ok(None) => {
                        info!("Chunk {:?} not found in cache. Generating...", chunk_coords);
                    },
                    Err(e) => {
                        warn!("Cache load failed for {:?}: {:?}. Generating instead.", chunk_coords, e);
                    }
                }
            },
            Err(e) => {
                error!("Cache Mutex poisoned during load: {}", e);
            }
        }
        
        // --- 2. Generate the Chunk ---
        let active_id = self.internal_state.get_active_generator_id();
        let generator_arc = self.generators
            .get(&active_id)
            .expect("Active generator ID must be registered in Conductor.");
            
        info!("Conductor dispatching generation of chunk {:?} using '{}'", 
              chunk_coords, active_id);
            
        let chunk_data = generator_arc.generate_chunk(chunk_coords);
        
        // --- 3. Save to cache ---
        match self.chunk_cache.lock() {
            Ok(cache_lock) => {
                if let Err(e) = cache_lock.save_chunk(&chunk_key, &chunk_data) {
                    error!("Failed to save chunk {:?} (Key: {:?}) to cache: {:?}", chunk_coords, chunk_key, e);
                } else {
                    info!("Conductor saved chunk {:?} to cache.", chunk_coords);
                }
            },
            Err(e) => {
                error!("Cache Mutex poisoned during save: {}", e);
            }
        }
        
        chunk_data
    }
}