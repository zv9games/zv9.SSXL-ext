use super::{
    run_godot_test, GODOT_TEST_SCENE, HEADLESS_ANIM_TEST_SCENE,
    HEADLESS_GEN_TEST_SCENE,
};

/// Runs the FFI Bridge/GDExtension validation test in Godot.
pub fn run_ffi_bridge_validation() {
    run_godot_test(
        "FFI Bridge and GDExtension Integration Validation",
        GODOT_TEST_SCENE,
        "FFI/GDExtension Bridge VALIDATION SUCCEEDED!",
    );
}

/// Runs the Headless Map Generation Integration Test (Full Pipeline Validation).
pub fn run_headless_generation_integration_test() {
    run_godot_test(
        "Headless Map Generation Integration Test",
        HEADLESS_GEN_TEST_SCENE,
        "Headless Generation Integration Test SUCCEEDED! Quantum alignment confirmed.",
    );
}

/// Runs the Headless Animation Conductor Tempo Test (Signal Latency Check).
pub fn run_headless_animation_tempo_test() {
    run_godot_test(
        "Headless Animation Conductor Tempo Test",
        HEADLESS_ANIM_TEST_SCENE,
        "Headless Animation Tempo Test SUCCEEDED! Faster-than-light speed confirmed.",
    );
}