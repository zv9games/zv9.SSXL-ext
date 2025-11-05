SSXL-ext is a mythic core — a modular, dimension-agnostic  
procedural generation engine coded in Rust as a GDExtension  
for Godot 4.2+ →).

🪶 Manifest v9.0.seed


#files


C:/ZV9/zv9.SSXL-ext/rust

Crate: ssxl_cache/src
├── [CORE]  lib.rs $

🔍 Crate: ssxl_engine_ffi/src
├── [CORE]  lib.rs $

🔍 Crate: ssxl_generate/src
│   └──  batch_processor.rs
│   └──  benchmark_logic.rs
│   └──  ca\mod.rs
│   └──  ca\neighbor_check.rs
│   └──  ca\rule_set.rs
│   └──  cellular_automata_generator.rs
│   └──  conductor.rs
│   └──  conductor_state.rs
│   └──  config_validator.rs
│   └──  generator.rs
│   └──  generator_manager.rs
├── [CORE]  lib.rs
│   └──  perlin_generator.rs
│   └──  runtime_manager.rs
│   └──  sync.rs
│   └──  task_queue.rs

🔍 Crate: ssxl_godot/src
│   └──  animation_api.rs
│   └──  api_initializers.rs
│   └──  async_poll.rs
│   └──  build.rs
│   └──  channel_handler.rs
│   └──  chunk_presenter.rs
│   └──  gde_api_defs.rs
│   └──  generation_api.rs
├── [CORE]  lib.rs
│   └──  ssxl_engine.rs
│   └──  ssxl_oracle.rs
│   └──  ssxl_signals.rs

🔍 Crate: ssxl_math/src
│   └──  coordinate_system.rs
│   └──  generation_utils.rs
│   └──  hashing.rs
├── [CORE]  lib.rs
│   └──  primitives.rs

🔍 Crate: ssxl_shared/src
│   └──  chunk_data.rs
│   └──  config.rs
│   └──  errors.rs
│   └──  generation_message.rs
│   └──  grid_bounds.rs
├── [CORE]  lib.rs
│   └──  math_primitives.rs
│   └──  messages.rs
│   └──  tile_data.rs
│   └──  tile_type.rs

🔍 Crate: ssxl_sync/src
│   └──  animation_conductor.rs
├── [CORE]  lib.rs
│   └──  pool_manager.rs
│   └──  primitives.rs

🔍 Crate: ssxl_tools/src
├── [CORE]  lib.rs

🔍 Crate: ssxl_cli/src
│   └──  actions\benchmarking.rs
│   └──  actions\godot_harness.rs
│   └──  actions\mod.rs
│   └──  actions\testing.rs
│   └──  cli_util_bench.rs
│   └──  cli_util_inspect.rs
│   └──  cli_util_menu.rs
├── [CORE]  main.rs
│   └──  scan\file_walker.rs
│   └──  scan\mod.rs
│   └──  scan\report_formatter.rs


