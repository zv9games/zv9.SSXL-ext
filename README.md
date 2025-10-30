# 🪐 SSXL-ext: The Dimension-Agnostic Generation Engine

![SSXL-ext Banner](SSXL-ext.png)

**SSXL-ext** is a high-performance, modular procedural generation engine written in **Rust**, designed to integrate seamlessly with **Godot 4.2+** via **GDExtension**. It doesn’t simulate worlds—it forges them. In milliseconds.

> “SSXL-ext is not a game—it’s the genesis point for your game.”

---

## ⚡️ The Breakthrough

> 🧨 3.8 million tiles placed in 4 seconds.  
> 🚫 GDScript queue eliminated.  
> ✅ Rust writes directly to Godot’s `TileMap`.

The engine now bypasses Godot’s scripting layer entirely. Chunks are generated asynchronously in Rust and applied synchronously to the `TileMap`—no queue, no delay, no mercy.

---

## 🚀 Core Features

- **Direct TileMap Integration**  
  Rust sends `ChunkData` straight to Godot’s rendering layer. No intermediaries.

- **Multithreaded Generation**  
  Worker threads spawn via `ssxl_sync`, generating chunks in parallel.

- **Modular Algorithms**  
  Swap between Perlin, Cellular Automata, Maze, Cave, Solid Fill, Checkerboard—no engine changes required.

- **Zero-Cost Abstractions**  
  Rust’s performance profile ensures no overhead, even at scale.

- **Godot as a Visualizer**  
  SSXL treats Godot as a high-speed renderer—not a processor.

---

## 📦 Project Structure

| Module              | Purpose                                                                 |
|---------------------|-------------------------------------------------------------------------|
| `ssxl_generate`     | ⚙️ Core generation logic (noise, CA, conductor)                         |
| `ssxl_shared`       | 🧱 Universal data structures (`ChunkData`, `TileData`, grids)           |
| `ssxl_math`         | 📐 Vector math, hashing, coordinate systems                             |
| `ssxl_cache`        | 💾 Persistent and temporary chunk storage                               |
| `ssxl_sync`         | 🔄 Worker threads and cross-thread communication                        |
| `ssxl_godot`        | 🎮 High-level Godot API bindings (`ssxl_engine.rs`)                     |
| `ssxl_engine_ffi`   | 🔗 Raw C FFI layer for data exchange                                    |

---

## 🛠️ Getting Started

### Prerequisites

- Rust (latest stable)
- Godot 4.2+
- Set `GODOT4_BIN` environment variable to your Godot binary path

### Build Instructions

```bash
# Build the Rust core and GDExtension
cargo build --release

# The compiled library will appear in `target/release/`
# Open the Godot project tester (SSXL_engine_tester) and run!
```

---

## 🧠 Philosophy

SSXL-ext is built on the principle of **separation of concerns**.  
Rust handles the math, the noise, the concurrency.  
Godot handles the visuals, the physics, the scripting.  
Together, they form a symphony of speed and control.

> SSXL-ext doesn’t wait for the world to load. It builds the world while you play.

---

## 📝 Naming Conventions

| Current File Name             | Module         | Recommended Name       | Reasoning                                      |
|------------------------------|----------------|------------------------|------------------------------------------------|
| `cellular_automata_generator.rs` | ssxl_generate | `cellular_automata.rs` | `_generator` is implied by context             |
| `perlin_generator.rs`        | ssxl_generate   | `perlin_noise.rs`       | More specific, keeps generator context clear   |
| `cli_util_actions.rs`        | ssxl_cli        | `cli_actions.rs`        | `_util` is verbose and redundant               |
| `cli_util_bench.rs`          | ssxl_cli        | `cli_bench.rs`          | Concise and clear                              |
| `cli_util_inspect.rs`        | ssxl_cli        | `cli_inspector.rs`      | Noun form better reflects logic/commands       |
| `cli_util_menu.rs`           | ssxl_cli        | `cli_menu.rs`           | Streamlined                                    |
| `generation_utils.rs`        | ssxl_math       | `math_helpers.rs`       | Clarifies role as supplementary logic          |

---

## 🧊 Performance Notes

- **ChunkData is king**  
  Entire chunks are transferred as bulk payloads—no per-tile calls.

- **Rendering is synchronous**  
  TileMap updates happen on the main thread, instantly.

- **Cache-aware generation**  
  Chunks are loaded from cache or generated on-demand, then stored.

- **No queue, no lag**  
  The GDScript queue is gone. The bottleneck is gone. The legend begins.

---

