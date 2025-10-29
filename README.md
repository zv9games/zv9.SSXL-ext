# 🪐 SSXL-ext: The Dimension-Agnostic Generation Engine

![SSXL-ext Banner](SSXL-ext.jpg)

**SSXL-ext** is a high-performance, modular procedural generation engine written in **Rust**, designed to integrate seamlessly with **Godot 4.2+** via **GDExtension**. It separates the heavy lifting of world generation from the rendering pipeline, enabling blazing-fast content creation without sacrificing visual fidelity.

> “SSXL-ext is not a game—it’s the genesis point for your game.”

---

## 🚀 Key Features

- **Subzero Cold Performance**  
  Rust-powered generation algorithms with zero-cost abstractions and multithreaded execution.

- **Modular Workspace**  
  Swap out generation logic (Perlin, Cellular Automata, etc.) without touching the engine interface.

- **GDExtension Bridge**  
  Fast, safe, low-latency data transfer between Rust and Godot using the `gdext` ecosystem.

- **Multithreaded Execution**  
  `ssxl_sync` spawns worker threads to generate chunks asynchronously, keeping the Godot main thread smooth.

- **Core Algorithms**  
  Includes:
  - Perlin & Simplex Noise
  - 2D & 3D Cellular Automata
  - Custom biome blending via the **Conductor** pipeline

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

## 📝 Script Naming Refinements

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

## ❄️ Performance Alignment: Taming the Godot Bulldozer

### 1. Asynchronous Chunk Generation

Use `ssxl_sync` to spawn Rust worker threads for chunk generation.  
Return `ChunkData` via lock-free queue or channel to `ssxl_godot::SSXLEngine`.  
Result: High FPS, smooth gameplay, chunks pop in as they complete.

### 2. High-Speed Data Transfer

❌ Avoid per-tile GDExtension calls.  
✅ Transfer entire chunk as a flat array (`PackedByteArray`, `Vec<u8>`).  
Godot unpacks in one go—less FFI overhead, more speed.

### 3. Godot-Side Rendering Optimization

❌ Don’t create a new Node3D/Sprite2D per tile.  
✅ Use:

- **2D**: `TileMap` or custom `_draw()` with `RenderingServer`  
- **3D**: `MultiMeshInstance3D` with batched transforms/colors

> Treat Godot as a high-speed data visualizer—not a processor.

---

## 🧠 Philosophy

SSXL-ext is built on the principle of **separation of concerns**.  
Rust handles the math, the noise, the concurrency.  
Godot handles the visuals, the physics, the scripting.  
Together, they form a symphony of speed and control.

---

