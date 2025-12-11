// ============================================================================
// 🧪 SSXL CLI: Godot Integration Tests (`godot_harness_tests`)
// ----------------------------------------------------------------------------
// This module defines a set of CLI-accessible integration tests that validate
// the connection between the Rust engine (via FFI/GDExtension) and the Godot
// runtime. Each test launches a specific Godot scene and checks for expected
// success signals, ensuring that critical subsystems are wired correctly.
//
// Key Functions:
//   • run_ffi_bridge_validation
//       - Validates the Foreign Function Interface (FFI) bridge between Rust
//         and Godot.
//       - Confirms that the compiled Rust dynamic library is correctly loaded
//         into Godot via GDExtension.
//       - Ensures bidirectional communication between Rust and Godot succeeds.
//
//   • run_headless_generation_integration_test
//       - Runs a headless Godot scene that exercises the full map generation
//         pipeline.
//       - Validates procedural generation, streaming, and integration without
//         requiring a graphical interface.
//       - Confirms that generated chunks are correctly processed and aligned.
//
//   • run_headless_animation_tempo_test
//       - Executes a headless Godot scene focused on animation conductor tempo.
//       - Validates high-frequency signal emission and latency handling.
//       - Ensures the conductor loop maintains expected performance under load.
//
// Workflow:
//   1. Each function calls `run_godot_test`, passing:
//        - A human-readable test description.
//        - The path to the Godot scene to execute.
//        - The expected success message for validation.
//   2. Godot runs the scene (editor or headless).
//   3. The CLI captures output and checks for the success string.
//   4. Results are logged, confirming subsystem integrity.
//
// Design Choices:
//   • Headless tests allow automated validation in CI/CD pipelines without GUI.
//   • Scene-based testing leverages Godot’s runtime to validate integration
//     rather than relying solely on unit tests.
//   • Success messages provide clear, human-readable confirmation of subsystem
//     health.
//
// Educational Note:
//   • These tests demonstrate how to bridge Rust and Godot using FFI and
//     GDExtension, ensuring that engine subsystems (map generation, animation,
//     conductor tempo) are validated in real runtime conditions.
//   • By centralizing these checks in the CLI, developers can quickly confirm
//     integration health before deploying or debugging in the editor.
// ============================================================================


use super::{
    run_godot_test, GODOT_TEST_SCENE, HEADLESS_ANIM_TEST_SCENE,
    HEADLESS_GEN_TEST_SCENE,
};

pub fn run_ffi_bridge_validation() {
    run_godot_test(
        "FFI Bridge and GDExtension Integration Validation",
        GODOT_TEST_SCENE,
        "FFI/GDExtension Bridge VALIDATION SUCCEEDED!",
    );
}

pub fn run_headless_generation_integration_test() {
    run_godot_test(
        "Headless Map Generation Integration Test",
        HEADLESS_GEN_TEST_SCENE,
        "Headless Generation Integration Test SUCCEEDED! Quantum alignment confirmed.",
    );
}

pub fn run_headless_animation_tempo_test() {
    run_godot_test(
        "Headless Animation Conductor Tempo Test",
        HEADLESS_ANIM_TEST_SCENE,
        "Headless Animation Tempo Test SUCCEEDED! Faster-than-light speed confirmed.",
    );
}
