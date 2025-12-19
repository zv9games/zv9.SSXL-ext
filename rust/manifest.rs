# 🪶 SSXL‑ext — Manifest v9.1.seed
### (direct‑write‑to‑tilemap edition)

**SSXL‑ext** is a mythic core — a modular, dimension‑agnostic  
procedural generation engine coded in **Rust** as a **GDExtension**  
for **Godot 4.2+**.

## ✅ Required Godot Extension Files (Native Core)


SSXL.gdextension  
SSXL_ext.dll


## ✅ Required Plugin Files (Editor‑Side Plugin)


addons/<plugin_name>/

plugin.cfg  
plugin.gd
SSXL.gd





PS C:\zv9\zv9.ssxl-ext\rust> Get-ChildItem -Path "C:\zv9\zv9.ssxl-ext\rust" -Recurse | Select-Object FullName

FullName
--------
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext
C:\zv9\zv9.ssxl-ext\rust\Cargo.lock
C:\zv9\zv9.ssxl-ext\rust\cargo.toml
C:\zv9\zv9.ssxl-ext\rust\LOC_scan.ps1
C:\zv9\zv9.ssxl-ext\rust\manifest.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_config.toml
C:\zv9\zv9.ssxl-ext\rust\SSXL_forward.rs
C:\zv9\zv9.ssxl-ext\rust\SSXL_manual.rs
C:\zv9\zv9.ssxl-ext\rust\SSXL_noob_survival_guide.gd
C:\zv9\zv9.ssxl-ext\rust\verbose.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\Cargo.toml
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\main.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\pipeline.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\run_ssxl.bat
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\ssxl_api_scan.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\ssxl_godot.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\ssxl_menu.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\ssxl_source_scan.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\ssxl_testing.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\Cargo.toml
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\animate_conductor.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\animate_events.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\animate_worker.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\api_registry.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\bridge_ffi.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\bridge_oracle.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\bridge_signals.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\cache.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\config.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_anim_conductor.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_batch_processor.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_ca.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_ca_simulation.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_conductor.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_conductor_state.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_conductor_sync.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_manager.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_perlin.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_runtime.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\generate_task_queue.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_anim.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_api.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_cleanup.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_commands.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_conductor.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_init.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_poller.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_render.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_signals.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_state.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_tilemap.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\host_tilemap_status.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\lib.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\math.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\rhythm_manager.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\shared_chunk.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\shared_config.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\shared_error.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\shared_job.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\shared_math.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\shared_message.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\shared_tile.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\shared_types.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\sync_pool.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\sync_rhythm.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\tile_conversion.rs
C:\zv9\zv9.ssxl-ext\rust\ssxl_ext\src\tools.rs


PS C:\zv9\zv9.SSXL-ext\rust>