# 🪐 SSXL-ext: The Dimension-Agnostic Generation Engine

![SSXL-ext Banner](SSXL-ext.png)

**SSXL-ext** is a high-performance, modular procedural generation engine written in **Rust**, designed to integrate seamlessly with **Godot 4.2+** via **GDExtension**. It doesn’t simulate worlds—it forges them. In milliseconds.

> “SSXL-ext is not a game—it’s the genesis point for your game.”

---

## ⚡️ The Breakthrough

> 🧨 Generating massive worlds: The Godot client can now request up to **50,000 chunks** (over **204 million tiles**), leveraging the 100 million chunk Rust core capacity.
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

| Module | Purpose |
| :--- | :--- |
| `ssxl_generate` | ⚙️ Core generation logic (noise, CA, conductor) |
| `ssxl_shared` | 🧱 Universal data structures (`ChunkData`, `TileData`, grids) |
| `ssxl_math` | 📐 Vector math, hashing, coordinate systems |
| `ssxl_cache` | 💾 Persistent and temporary chunk storage |
| `ssxl_sync` | 🔄 Worker threads and cross-thread communication |
| `ssxl_godot` | 🎮 High-level Godot API bindings (`ssxl_engine.rs`) |
| `ssxl_engine_ffi` | 🔗 Raw C FFI layer for data exchange |

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