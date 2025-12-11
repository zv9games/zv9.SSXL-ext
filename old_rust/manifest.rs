SSXL-ext is a mythic core — a modular, dimension-agnostic  
procedural generation engine coded in Rust as a GDExtension  
for Godot 4.2+ →).

🪶 Manifest v9.1.seed (direct write to tilemap)

zv9.ssxl-ext/
├── rust/
│   ├── Cargo.toml                    # workspace + shared config
│   ├── SSXL-ext/                         # the only real crate (everything lives here)
│   │   ├── src/
│   │   │   ├── cache.rs                    # pure Rust cache
│   │   │   ├── config.rs                   # pure Rust settings
│   │   │   ├── math.rs                     # pure Rust math helpers
│   │   │   ├── tools.rs                    # pure Rust utilities
│   │   │   │
│   │   │   ├── animate_conductor.rs        # animation conductor
│   │   │   ├── animate_worker.rs           # worker threads
│   │   │   ├── animate_events.rs           # animation events
│   │   │   │
│   │   │   ├── generate_perlin.rs          # Perlin noise generator
│   │   │   ├── generate_ca.rs              # cellular automata (main + rules + neighbors)
│   │   │   ├── generate_ca_simulation.rs   # CA simulation helpers (split if needed)
│   │   │   ├── generate_conductor.rs       # generation conductor
│   │   │   ├── generate_conductor_state.rs # conductor state
│   │   │   ├── generate_conductor_sync.rs  # conductor sync helpers
│   │   │   ├── generate_manager.rs         # generator manager
│   │   │   ├── generate_runtime.rs         # runtime manager
│   │   │   ├── generate_task_queue.rs      # task queue
│   │   │   ├── generate_batch_processor.rs # batch processor
│   │   │   │
│   │   │   ├── shared_chunk.rs             # chunk data + bounds
│   │   │   ├── shared_tile.rs              # tile data + type
│   │   │   ├── shared_message.rs           # messages
│   │   │   ├── shared_config.rs            # shared config structs
│   │   │   ├── shared_error.rs             # errors
│   │   │   ├── shared_job.rs               # jobs
│   │   │   ├── shared_math.rs              # shared math primitives
│   │   │   │
│   │   │   ├── sync_pool.rs                # worker pool
│   │   │   ├── sync_rhythm.rs              # sync/timing
│   │   │   │
│   │   │   │ ────────────── Bridge / Host ──────────────
│   │   │   ├── bridge_ffi.rs               # raw FFI entry points (danger zone)
│   │   │   ├── bridge_signals.rs           # signal registration & emission
│   │   │   ├── bridge_oracle.rs            # oracle helper
│   │   │   │
│   │   │   ├── host_init.rs                # Godot engine initialization
│   │   │   ├── host_tick.rs                # Godot tick loop
│   │   │   ├── host_poller.rs              # Godot poller
│   │   │   ├── host_render.rs              # Godot render batch
│   │   │   ├── host_state.rs               # Godot engine state
│   │   │   ├── host_cleanup.rs             # Godot cleanup
│   │   │   ├── host_commands.rs            # Godot command handling
│   │   │   ├── host_tilemap.rs             # Godot TileMap integration
│   │   │   ├── host_tilemap_status.rs      # Godot status reporter
│   │   │   ├── host_anim.rs                # Godot animation API
│   │   │   │
│   │   │   └── lib.rs                      # re-exports only (super clean)
│   │   └── Cargo.toml
│   ├── ssxl_cli/                         # CLI stays separate
│   │   └── src/main.rs
│   └── ...



