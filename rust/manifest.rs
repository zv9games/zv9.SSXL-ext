SSXL-ext is a mythic core — a modular, dimension-agnostic  
procedural generation engine coded in Rust as a GDExtension  
for Godot 4.2+ →).

🪶 Manifest v9.0.seed


#files


📦 zv9.SSXL-ext/ — Unified Workspace Root

C:/ZV9/zv9.SSXL-ext/rust

rust/                                # Workspace Member Root
├── ssxl_cache/             # 💾 Data Caching and Chunk Storage
	├── cargo.toml
	├── /src
		├──lib.rs
	
├── ssxl_cli/               # 🧰 Interactive Console, Benchmarks, and Diagnostics
	├── cargo.toml
	├── src/
		├──cli_util_actions.rs
		├──cli_util_bench.rs
		├──cli_util_inspect.rs
		├──cli_util_menu.rs
		├──main.rs
	
├── ssxl_engine_ffi/        # 🔗 Low-level C FFI Bridge for Godot Communication
	├──cargo.toml
	├── src/
		├──lib.rs
   
├── ssxl_generate/          # ⚙️ Core Generation Algorithms (Noise, Pattern Mapping)
	├──cargo.toml
	├── src/
		├──cellular_automata_generator.rs
		├──conductor.rs
		├──generator.rs
		├──lib.rs
		├──perlin_generator.rs
		├──benchmark_logic.rs
   
├── ssxl_godot/             # 🎮 High-level Godot Bindings (GDExtension API)
	├──cargo.toml
	├── src/
		├──lib.rs
   
├── ssxl_math/              # 📐 Mathematical Primitives and Coordinate Systems
	├──cargo.toml
	├── src/
		├──lib.rs
		├──coordinate_system.rs
		├──generation_utils.rs
		├──hashing.rs
		├──primitives.rs
			
   
├── ssxl_shared/            # 🧱 Global Data Primitives (Chunk, Tile, Grid Types)
	├──cargo.toml
	├── src/
		├──lib.rs
		├──chunk_data.rs
		├──errors.rs
		├──grid_bounds.rs
		├──math_primitives.rs
		├──tile_data.rs
		├──tile_type.rs
   
├── ssxl_sync/              # 🚦 Concurrency Primitives and Thread Safety
	├──cargo.toml
	├── src/
		├──lib.rs
   
├── ssxl_tools/             # 🔧 Utility Logic (Configuration, Logging, Profiling)
	├──cargo.toml
	├── src/
		├──lib.rs
   
└── iteration5/                  # 🗑️ Obsolete/Temporary Directory (For cleanup)

C:/ZV9/zv9.gdext/ //<-- local clone of fork of godot-rust master linked in toml's



