SSXL-ext is a mythic core — a modular, dimension-agnostic  
procedural generation engine coded in Rust as a GDExtension  
for Godot 4.2+ →).

🪶 Manifest v9.0.seed


#files


PS C:\zv9\zv9.ssxl-ext> cd rust
PS C:\zv9\zv9.ssxl-ext\rust> Get-ChildItem -Recurse -Force |
>>   Select-Object FullName,
>>                 @{Name="Type";Expression={if($_.PSIsContainer){"Dir"}else{"File"}}},
>>                 Length |
>>   Format-Table -AutoSize

FullName                                                                     Type Length
--------                                                                     ---- ------
C:\zv9\zv9.ssxl-ext\rust\.cargo                                              Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_animate                                        Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_cache                                          Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli                                            Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_config                                         Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_engine_ffi                                     Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate                                       Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot                                          Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_math                                           Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared                                         Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_sync                                           Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_tools                                          Dir
C:\zv9\zv9.ssxl-ext\rust\Cargo.lock                                          File 46304
C:\zv9\zv9.ssxl-ext\rust\cargo.toml                                          File 3486
C:\zv9\zv9.ssxl-ext\rust\manifest.rs                                         File 12547
C:\zv9\zv9.ssxl-ext\rust\ssxl_animate\src                                    Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_animate\Cargo.toml                             File 615
C:\zv9\zv9.ssxl-ext\rust\ssxl_animate\src\animation_logic.rs                 File 1696
C:\zv9\zv9.ssxl-ext\rust\ssxl_animate\src\conductor.rs                       File 3503
C:\zv9\zv9.ssxl-ext\rust\ssxl_animate\src\lib.rs                             File 2751
C:\zv9\zv9.ssxl-ext\rust\ssxl_animate\src\worker.rs                          File 3072
C:\zv9\zv9.ssxl-ext\rust\ssxl_cache\src                                      Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_cache\Cargo.toml                               File 433
C:\zv9\zv9.ssxl-ext\rust\ssxl_cache\src\lib.rs                               File 5473
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src                                        Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\Cargo.toml                                 File 1353
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\actions                                Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\cli_util_bench.rs                      File 5917
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\cli_util_inspect.rs                    File 7543
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\cli_util_loc.rs                        File 6953
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\cli_util_menu.rs                       File 7871
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\main.rs                                File 6282
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\actions\actions.rs                     File 7267
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\actions\benchmarking.rs                File 3047
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\actions\godot_harness.rs               File 5751
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\actions\mod.rs                         File 3635
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\actions\testing.rs                     File 2774
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\actions\test_core_suites.rs            File 8648
C:\zv9\zv9.ssxl-ext\rust\ssxl_cli\src\actions\test_suites.rs                 File 5911
C:\zv9\zv9.ssxl-ext\rust\ssxl_config\src                                     Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_config\src\config.rs                           File 1350
C:\zv9\zv9.ssxl-ext\rust\ssxl_config\src\engine.toml                         File 599
C:\zv9\zv9.ssxl-ext\rust\ssxl_config\src\lib.rs                              File 166
C:\zv9\zv9.ssxl-ext\rust\ssxl_engine_ffi\src                                 Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_engine_ffi\Cargo.toml                          File 727
C:\zv9\zv9.ssxl-ext\rust\ssxl_engine_ffi\src\lib.rs                          File 7871
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src                                   Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\Cargo.toml                            File 979
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\ca                                Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\conductor                         Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\manager                           Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\perlin                            Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\task                              Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\lib.rs                            File 2627
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\ca\cellular_automata_generator.rs File 9334
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\ca\mod.rs                         File 1148
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\ca\neighbor_check.rs              File 2952
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\ca\rule_set.rs                    File 2898
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\conductor\conductor.rs            File 10462
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\conductor\conductor_state.rs      File 1923
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\conductor\mod.rs                  File 400
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\conductor\sync.rs                 File 1209
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\manager\config_validator.rs       File 2447
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\manager\generator.rs              File 1626
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\manager\generator_manager.rs      File 4430
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\manager\mod.rs                    File 318
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\manager\runtime_manager.rs        File 1114
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\perlin\mod.rs                     File 120
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\perlin\perlin_generator.rs        File 4531
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\task\batch_processor.rs           File 3172
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\task\benchmark_logic.rs           File 1007
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\task\mod.rs                       File 216
C:\zv9\zv9.ssxl-ext\rust\ssxl_generate\src\task\task_queue.rs                File 5672
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src                                      Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\Cargo.toml                               File 1031
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\anim                                 Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine                               Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\ffi                                  Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\tilemap                              Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\lib.rs                               File 2120
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\anim\animation_api.rs                File 1279
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\api.rs                        File 2868
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\api_initializers.rs           File 1544
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\cleanup.rs                    File 2321
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\commands.rs                   File 1705
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\init.rs                       File 6635
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\mod.rs                        File 910
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\poller.rs                     File 3570
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\query.rs                      File 1730
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\query_data.rs                 File 1314
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\render_batch.rs               File 2684
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\state.rs                      File 3274
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\engine\tick.rs                       File 3498
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\ffi\gde_api_defs.rs                  File 3018
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\ffi\mod.rs                           File 160
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\ffi\oracle.rs                        File 2082
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\ffi\signals.rs                       File 1542
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\tilemap\async_poll.rs                File 4352
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\tilemap\mod.rs                       File 310
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\tilemap\ssxl_tilemap.rs              File 3309
C:\zv9\zv9.ssxl-ext\rust\ssxl_godot\src\tilemap\status_reporter.rs           File 2797
C:\zv9\zv9.ssxl-ext\rust\ssxl_math\src                                       Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_math\Cargo.toml                                File 607
C:\zv9\zv9.ssxl-ext\rust\ssxl_math\src\coordinate_system.rs                  File 6677
C:\zv9\zv9.ssxl-ext\rust\ssxl_math\src\generation_utils.rs                   File 2729
C:\zv9\zv9.ssxl-ext\rust\ssxl_math\src\hashing.rs                            File 4662
C:\zv9\zv9.ssxl-ext\rust\ssxl_math\src\lib.rs                                File 2168
C:\zv9\zv9.ssxl-ext\rust\ssxl_math\src\primitives.rs                         File 2779
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src                                     Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\cargo.toml                              File 501
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\chunk                               Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\config                              Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\error                               Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\job                                 Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\math                                Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\message                             Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\tile                                Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\lib.rs                              File 1546
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\chunk\chunk_data.rs                 File 11996
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\chunk\grid_bounds.rs                File 2676
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\chunk\mod.rs                        File 76
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\config\config.rs                    File 1106
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\config\mod.rs                       File 51
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\error\errors.rs                     File 3182
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\error\mod.rs                        File 50
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\job\mod.rs                          File 1496
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\math\math_primitives.rs             File 2146
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\math\mod.rs                         File 58
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\message\generation_message.rs       File 1792
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\message\messages.rs                 File 4084
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\message\mod.rs                      File 271
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\tile\mod.rs                         File 72
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\tile\tile_data.rs                   File 4843
C:\zv9\zv9.ssxl-ext\rust\ssxl_shared\src\tile\tile_type.rs                   File 4701
C:\zv9\zv9.ssxl-ext\rust\ssxl_sync\src                                       Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_sync\Cargo.toml                                File 928
C:\zv9\zv9.ssxl-ext\rust\ssxl_sync\src\animation_conductor.rs                File 4721
C:\zv9\zv9.ssxl-ext\rust\ssxl_sync\src\lib.rs                                File 3176
C:\zv9\zv9.ssxl-ext\rust\ssxl_sync\src\pool.rs                               File 6426
C:\zv9\zv9.ssxl-ext\rust\ssxl_sync\src\primitives.rs                         File 1640
C:\zv9\zv9.ssxl-ext\rust\ssxl_tools\src                                      Dir
C:\zv9\zv9.ssxl-ext\rust\ssxl_tools\Cargo.toml                               File 293
C:\zv9\zv9.ssxl-ext\rust\ssxl_tools\src\lib.rs                               File 4463
