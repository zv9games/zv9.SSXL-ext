//! The core manager for the ssxl Engine, responsible for coordinating
//! generation, concurrency, and caching via the Tokio asynchronous runtime.

use tokio::runtime::{Runtime, Handle};
use tokio::sync::mpsc;
use tracing::{info, error, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::io;

// 🚀 TEMPO BOOST: Rayon for CPU parallelization
use rayon::prelude::*;
// 🚀 TEMPO BOOST: Use num_cpus to set worker threads dynamically
use num_cpus;

// 📐 BULLDOZER FIX: Imports updated to support i64 (64-bit) coordinates
use glam::I64Vec3;
use ssxl_math::{Vec2i, prelude::ChunkKey};

// --- CONSTANTS ---
const CHUNK_SIZE: usize = 64; // The size of a chunk in tiles

/// A safety measure to prevent the Conductor from trying to generate or track
/// an excessive number of chunks that could lead to memory exhaustion.
/// This limit (100,000) supports maps up to approximately 16192x16192 tiles (253x253 chunks).
const MAX_ACTIVE_CHUNKS: i64 = 100_000_000;

// --- INTERNAL CRATE DEPENDENCIES ---
use ssxl_cache::ChunkCache;
use crate::Generator;
use crate::perlin_generator::PerlinGenerator;
// FIX: No-Break Space (\u{a0}) removed from indentation on the following lines.
use crate::cellular_automata_generator::{
    CellularAutomataGenerator,
    RULE_BASIC_CAVE,
    RULE_MAZE,
    RULE_SOLID,
    RULE_CHECKERBOARD,
};

// --- EXTERNAL CRATE DEPENDENCIES ---
use ssxl_shared::chunk_data::ChunkData;
use ssxl_tools::get_config_from_path;

// Define a type alias that includes Send + Sync bounds for thread-safe trait objects
type DynGenerator = Box<dyn Generator + Send + Sync>;

// --- MPSC Channel Configuration ---
// 🚀 TEMPO BOOST: Increased bound to allow Rayon to get far ahead of the main thread consumer.
const PROGRESS_CHANNEL_BOUND: usize = 1024;

// -----------------------------------------------------------------------------
// GENERATION MESSAGES
// -----------------------------------------------------------------------------

/// Messages sent from the asynchronous generation task back to the main
/// thread (Godot's tick loop) for reporting progress and results.
#[derive(Debug)]
pub enum GenerationMessage {
    /// A general status update (e.g., "Initializing Noise", "Processing 5/100 Chunks").
    StatusUpdate(String),
    /// Reports the completion of a single chunk. Contains the chunk's coordinates AND the bulk data.
    /// 🚀 **TEMPO BOOST:** Uses Arc<ChunkData> for zero-copy transfer to the main thread.
    ChunkGenerated(Vec2i, Arc<ChunkData>),
    /// Reports the final completion of the entire generation run.
    GenerationComplete,
    /// Reports a non-fatal error during generation.
    Error(String),
}


// -----------------------------------------------------------------------------
// GENERATOR CONFIGURATION
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
// CONDUCTOR STATE AND STATUS
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
        // Handle Mutex poisoning by unwrapping
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
    // Thread-safe handle to the ChunkCache for persistence
    chunk_cache: Arc<Mutex<ChunkCache>>,
    // The sender side of the progress channel. Cloned for each task.
    progress_sender: mpsc::Sender<GenerationMessage>,
}

impl Conductor {
    /// Initializes the Conductor, starts the runtime, creates the MPSC channel,
    /// and returns the MPSC Receiver for the FFI consumer (Godot) to poll.
    pub fn new(config_path: Option<&str>) -> Result<(Self, ConductorState, mpsc::Receiver<GenerationMessage>), io::Error> {
        // Load configuration first
        let config = get_config_from_path(config_path)?;

        // --- Runtime Setup ---
        // 🚀 TEMPO BOOST: Dynamically set worker threads to match CPU cores.
        let num_cores = num_cpus::get();
        info!("Tokio Runtime initializing with {} worker threads (all logical cores).", num_cores);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_cores)
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

        // 4. Cellular Automata Generator (Solid Fill)
        let ca_solid: DynGenerator = Box::new(CellularAutomataGenerator::new(RULE_SOLID));
        generators.insert(ca_solid.id().to_string(), Arc::new(ca_solid));

        // 5. Cellular Automata Generator (Checkerboard)
        let ca_checkerboard: DynGenerator = Box::new(CellularAutomataGenerator::new(RULE_CHECKERBOARD));
        generators.insert(ca_checkerboard.id().to_string(), Arc::new(ca_checkerboard));

        // --- Cache Initialization ---
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

        // --- MPSC Channel Setup ---
        // Bound set high to prevent backpressure on fast Rayon threads
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
    // CORE GENERATION COMMAND ENTRY POINT
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
        let runtime_handle = self.get_handle();
        // Clone all resources needed for the *blocking* task's move closure
        let generators_clone = self.generators.clone();
        let chunk_cache_clone = self.chunk_cache.clone();
        let active_generator_id = self.internal_state.get_active_generator_id();
        let progress_sender_clone = self.progress_sender.clone();
        let internal_state_clone = self.internal_state.clone();
        let config_clone = config.clone(); // Clone the config for the task

        // --- 3. Offload CPU-Intensive Work to Blocking Pool (THE TEMPO BOOST) ---
        runtime_handle.spawn_blocking(move || { // ⚠️ CRITICAL CHANGE: Use spawn_blocking for CPU work
            info!("Blocking generation task started with config: {:?}", config_clone);

            // Increment queue depth to signal a task has started
            internal_state_clone.queue_depth.fetch_add(1, Ordering::Relaxed);
            
            // Calculate grid dimensions using i64 types
            let chunk_size_i64 = CHUNK_SIZE as i64;
            let width_in_chunks = (config_clone.width as i64 + chunk_size_i64 - 1) / chunk_size_i64;
            let height_in_chunks = (config_clone.height as i64 + chunk_size_i64 - 1) / chunk_size_i64;
            // The total number of chunks to process
            let total_chunks = width_in_chunks * height_in_chunks;
            
            // 🛑 SAFETY CHECK: Re-introducing the chunk limit to prevent memory exhaustion
            if total_chunks > MAX_ACTIVE_CHUNKS {
                let error_msg = format!("max chunks limit exceeded: requested {} chunks ({}x{}) but the limit is {}. Increase MAX_ACTIVE_CHUNKS in conductor.rs if needed.",
                    total_chunks, width_in_chunks, height_in_chunks, MAX_ACTIVE_CHUNKS);
                error!("{}", error_msg);
                let _ = progress_sender_clone.blocking_send(GenerationMessage::Error(error_msg.into()));
                // CRITICAL FIX: Decrement the queue depth since we are aborting the task
                internal_state_clone.queue_depth.fetch_sub(1, Ordering::Relaxed);
                return; // Exit the blocking task gracefully on error
            }

            // --- 1. Create a flattened collection of all chunk coordinates (i64) ---
            let all_chunk_coords: Vec<Vec2i> = (0..height_in_chunks)
                .flat_map(|y| (0..width_in_chunks).map(move |x| Vec2i::new(x, y)))
                .collect();

            info!("Generation starting: {} chunks to process.", all_chunk_coords.len());

            if total_chunks == 0 {
                error!("Generation failed: Calculated chunk count is zero for size {}x{}. Sending error signal.", config_clone.width, config_clone.height);
                let _ = progress_sender_clone.blocking_send(GenerationMessage::Error("Map dimensions are too small to contain a single chunk.".into()));
            } else {
                // --- 2. Parallelize the entire generation workload using Rayon ---
                all_chunk_coords
                    .par_iter()
                    .for_each(|&chunk_coords| { // Use for_each for fire-and-forget generation
                        // The generation logic below now executes in parallel across CPU cores!
                        
                        // 📐 BULLDOZER FIX: Use I64Vec3 (i64) to prevent coordinate overflow on key creation.
                        let key_vec3 = I64Vec3::new(chunk_coords.x, chunk_coords.y, 0);
                        
                        let chunk_key = ChunkKey(key_vec3);
                        let chunk_data: ChunkData;

                        // ⚠️ CORE TEMPO ALIGNMENT: GENERATE & CACHE IN PARALLEL

                        // --- 1. Attempt to load from cache ---
                        // Mutex Lock contention will occur here, but Rayon handles this better than Tokio threads
                        match chunk_cache_clone.lock() {
                            // FIX: Removed 'mut' from 'cache_lock' to resolve 'unused_mut' warning.
                            Ok(cache_lock) => {
                                // Caching logic remains the same: load, generate, save
                                match cache_lock.load_chunk(&chunk_key) {
                                    Ok(Some(data)) => {
                                        info!("Blocking: Retrieved chunk {:?} from cache.", chunk_coords);
                                        chunk_data = data;
                                    },
                                    Ok(None) => {
                                        info!("Blocking: Chunk {:?} not found in cache. Generating...", chunk_coords);

                                        // --- 2. Generate the Chunk ---
                                        let generator_arc = generators_clone
                                            .get(&active_generator_id)
                                            .expect("Active generator ID must be registered in Conductor.");

                                        chunk_data = generator_arc.generate_chunk(chunk_coords); // ⚠️ Vec2i (i64 based)

                                        // --- 3. Save to cache ---
                                        // The MutexGuard is locked in the surrounding scope, so we can access `cache_lock`
                                        if let Err(e) = cache_lock.save_chunk(&chunk_key, &chunk_data) {
                                            error!("Blocking: Failed to save chunk {:?} to cache: {:?}", chunk_coords, e);
                                        } else {
                                            info!("Blocking: Saved chunk {:?} to cache.", chunk_coords);
                                        }
                                    },
                                    Err(e) => {
                                        warn!("Blocking: Cache load failed for {:?}: {:?}. Forcing generation without caching.", chunk_coords, e);
                                        let generator_arc = generators_clone
                                            .get(&active_generator_id)
                                            .expect("Active generator ID must be registered in Conductor.");
                                        chunk_data = generator_arc.generate_chunk(chunk_coords);
                                    }
                                }
                            },
                            Err(e) => {
                                error!("Blocking: Cache Mutex poisoned during load/save: {}", e);
                                warn!("Blocking: Falling back to generation without caching.");
                                let generator_arc = generators_clone
                                    .get(&active_generator_id)
                                    .expect("Active generator ID must be registered in Conductor.");
                                chunk_data = generator_arc.generate_chunk(chunk_coords);
                            }
                        }
                        
                        // 🚀 **TEMPO BOOST:** Wrap ChunkData in Arc for zero-copy transfer
                        let chunk_data_arc = Arc::new(chunk_data);

                        // 4. Emit signal with the zero-copy bulk data payload
                        let send_result = progress_sender_clone.blocking_send( // ⚠️ Use blocking_send
                            GenerationMessage::ChunkGenerated(chunk_coords, chunk_data_arc) // 👈 SENDING ARC<BULK DATA>
                        );

                        if send_result.is_err() {
                            // If the receiver (Godot) dropped the channel, gracefully stop the work.
                            warn!("Progress channel disconnected. Stopping generation task.");
                            // Note: We can't break from a for_each loop, so we return early
                            return;
                        }
                    }); // END of rayon parallel processing
            }

            // Send final completion message
            let _ = progress_sender_clone.blocking_send(GenerationMessage::GenerationComplete);

            // Decrement queue depth on completion
            internal_state_clone.queue_depth.fetch_sub(1, Ordering::Relaxed);

            info!("Blocking generation task finished processing command: {:?}", config_clone);
        });

        Ok(())
    }

    // --- Shutdown Management ---

    /// Signals the Conductor's internal state to begin a graceful shutdown process.
    pub fn signal_shutdown_graceful(&self) {
        info!("SSXL Conductor signaled for shutdown. Setting status to ShuttingDown.");
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

    /// **SYNCHRONOUS PATHWAY (For CLI/Benchmarking):**
    /// Synchronously generates a single chunk using the currently active generator.
    /// This method is blocking and bypasses the asynchronous runtime, MPSC channel,
    /// and internal caching mechanism.
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        let active_id = self.internal_state.get_active_generator_id();

        let generator_arc = self.generators
            .get(&active_id)
            .unwrap_or_else(|| panic!("Cannot find active generator with ID: {}", active_id));

        generator_arc.generate_chunk(chunk_coords)
    }
}