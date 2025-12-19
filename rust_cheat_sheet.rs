// =========================
// 📦 MODULE + IMPORT SYSTEM
// =========================

// `use` pulls names into scope.
// `crate::` means "from this crate's root" (SSXL uses this everywhere).
use crate::noise::NoiseGenerator;
use crate::api_registry::export_api;
use tracing::{info, warn};

// =========================
// 🧱 STRUCT DEFINITION
// =========================

// `struct` defines a data type.
// Fields have names + types.
pub struct Chunk<'a, T> {
    pub x: i32,
    pub y: i32,
    pub data: Vec<T>,
    pub noise: &'a dyn NoiseGenerator, // trait object reference
}

// `'a` is a *lifetime* meaning "this reference must live at least as long as 'a".
// `T` is a generic type parameter.

// =========================
// 🎭 TRAIT DEFINITION
// =========================

// A trait is a "role" or "capability" a type can implement.
pub trait ChunkBuilder {
    fn build_chunk(&self, x: i32, y: i32) -> Chunk<'_, u8>;
}

// =========================
// 🛠️ IMPL BLOCK (METHODS)
// =========================

// `impl<T>` means "implement for any T".
// `where T: Trait` means "T must implement Trait".
impl<'a, T> Chunk<'a, T>
where
    T: Copy + Default,
{
    // &self = borrowed reference
    // -> ReturnType
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// =========================
// 🧪 IMPLEMENTING A TRAIT
// =========================

pub struct DefaultChunkBuilder<'a> {
    pub noise: &'a dyn NoiseGenerator,
}

impl<'a> ChunkBuilder for DefaultChunkBuilder<'a> {
    fn build_chunk(&self, x: i32, y: i32) -> Chunk<'_, u8> {
        info!("Building chunk at ({}, {})", x, y);

        let mut data = Vec::with_capacity(64 * 64);

        for iy in 0..64 {
            for ix in 0..64 {
                // world coords = chunk coords * size + local coords
                let wx = x * 64 + ix;
                let wy = y * 64 + iy;

                // sample noise (trait method)
                let v = self.noise.sample(wx as f32, wy as f32);

                data.push((v * 255.0) as u8);
            }
        }

        Chunk {
            x,
            y,
            data,
            noise: self.noise,
        }
    }
}

// =========================
// 🧩 MACRO USAGE (SSXL STYLE)
// =========================

// This registers a function in the API registry.
// SSXL uses this pattern heavily.
export_api! {
    fn get_chunk_value(x: i32, y: i32, index: usize) -> u8 {
        let builder = crate::runtime::get_chunk_builder();
        let chunk = builder.build_chunk(x, y);

        chunk.get(index).copied().unwrap_or(0)
    }
}

// =========================
// 🎛️ ERROR HANDLING
// =========================

pub fn load_config(path: &str) -> Result<String, std::io::Error> {
    // `?` means "if this errors, return the error immediately"
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

// =========================
// 🔀 MATCH (SSXL uses this everywhere)
// =========================

pub fn classify(v: u8) -> &'static str {
    match v {
        0..=50 => "low",
        51..=200 => "mid",
        _ => "high",
    }
}

// =========================
// ✅ CLOSURE (used in menu actions)
// =========================

pub fn run_menu() {
    let action = || {
        info!("Running GUT test...");
        crate::run_grand_unified_test();
    };

    action(); // call the closure
}




// =========================
// 📦 MODULE + IMPORT SYSTEM
// =========================

// `use` pulls names into scope.
// `crate::` means "from this crate's root" (SSXL uses this everywhere).
use crate::noise::NoiseGenerator;
use crate::api_registry::export_api;
use tracing::{info, warn};

// =========================
// 🧱 STRUCT DEFINITION
// =========================

// `pub`  -> visible outside this module
// `struct` -> defines a data type
// `Chunk`  -> name of the struct
// `<`'a, T`>` -> generic parameters: a lifetime `'a` and a type `T`
pub struct Chunk<'a, T> {
    // `pub x: i32` -> public field `x` of type `i32`
    pub x: i32,
    pub y: i32,

    // `Vec<T>` -> growable array (vector) of elements of type `T`
    pub data: Vec<T>,

    // `&'a dyn NoiseGenerator` ->
    //   &   : reference
    //  'a   : lifetime, must live at least as long as `'a`
    //  dyn  : dynamic dispatch (trait object)
    //  NoiseGenerator : trait name
    pub noise: &'a dyn NoiseGenerator,
}

// =========================
// 🎭 TRAIT DEFINITION
// =========================

// A trait is like an interface or "role" a type can implement.
pub trait ChunkBuilder {
    // `fn build_chunk(&self, x: i32, y: i32) -> Chunk<'_, u8>;`
    //    fn          : function
    //    &self       : borrowed reference to the implementor
    //    x: i32      : parameter `x` of type `i32`
    //    y: i32      : parameter `y` of type `i32`
    //    ->          : return type follows
    //    Chunk<'_,u8>: Chunk with inferred lifetime `'_' and element type `u8`
    fn build_chunk(&self, x: i32, y: i32) -> Chunk<'_, u8>;
}

// =========================
// 🛠️ IMPL BLOCK (METHODS)
// =========================

// `impl<'a, T>` -> implementation for any lifetime `'a` and type `T`
// `Chunk<'a, T>` -> the type we're implementing methods for
// `where T: Copy + Default` -> T must implement both `Copy` and `Default`
impl<'a, T> Chunk<'a, T>
where
    T: Copy + Default,
{
    // `pub fn len(&self) -> usize { ... }`
    //   pub   : method is visible outside this module
    //   fn    : function
    //   len   : method name
    //   &self : borrowed reference (doesn't take ownership)
    //   -> usize : returns a usize
    pub fn len(&self) -> usize {
        // `self.data.len()` ->
        //   self      : this Chunk instance
        //   .data     : its `data` field
        //   .len()    : vector length method
        self.data.len()
    }

    // Another example method using Option and borrowing
    pub fn get(&self, index: usize) -> Option<&T> {
        // `self.data.get(index)` returns `Option<&T>`
        self.data.get(index)
    }
}

// =========================
// 🧪 IMPLEMENTING A TRAIT
// =========================

// A concrete type that knows how to build chunks.
pub struct DefaultChunkBuilder<'a> {
    pub noise: &'a dyn NoiseGenerator,
}

// `impl<'a> ChunkBuilder for DefaultChunkBuilder<'a>`
// means: "for any lifetime `'a`, implement the `ChunkBuilder` trait
// for the type `DefaultChunkBuilder<'a>`"
impl<'a> ChunkBuilder for DefaultChunkBuilder<'a> {
    fn build_chunk(&self, x: i32, y: i32) -> Chunk<'_, u8> {
        info!("Building chunk at ({}, {})", x, y);

        // `let` bindings create local variables.
        // `Vec::with_capacity(64 * 64)` creates an empty Vec<u8> with room for 4096 elements.
        let mut data = Vec::with_capacity(64 * 64);

        // `for iy in 0..64` ->
        //   for   : loop
        //   iy    : loop variable
        //   0..64 : range from 0 (inclusive) to 64 (exclusive)
        for iy in 0..64 {
            for ix in 0..64 {
                // world coords = chunk coords * size + local coords
                // `let wx = x * 64 + ix;`
                //     wx : i32
                let wx = x * 64 + ix;
                let wy = y * 64 + iy;

                // sample noise (trait method)
                // `as f32` casts integer to f32
                let v = self.noise.sample(wx as f32, wy as f32);

                // push computed u8 into the data vector
                data.push((v * 255.0) as u8);
            }
        }

        // Return a Chunk instance using struct literal syntax:
        Chunk {
            x,
            y,
            data,
            noise: self.noise,
        }
    }
}

// =========================
// 🧩 MACRO USAGE (SSXL STYLE)
// =========================

// This macro registers a function into the SSXL API registry.
// The body looks like a normal function, but `export_api!` inspects it at compile time.
export_api! {
    fn get_chunk_value(x: i32, y: i32, index: usize) -> u8 {
        // `crate::runtime::get_chunk_builder()` -> call a function from the crate root.
        let builder = crate::runtime::get_chunk_builder();

        // use the trait method we implemented above
        let chunk = builder.build_chunk(x, y);

        // `chunk.get(index)` returns Option<&u8>
        // `.copied()` turns Option<&u8> into Option<u8>
        // `.unwrap_or(0)` returns the value, or 0 if None
        chunk.get(index).copied().unwrap_or(0)
    }
}

// =========================
// 🎛️ ERROR HANDLING
// =========================

// `Result<String, std::io::Error>` means this function can either:
// - return Ok(String) on success
// - or Err(std::io::Error) on failure
pub fn load_config(path: &str) -> Result<String, std::io::Error> {
    // `std::fs::read_to_string(path)` returns Result<String, std::io::Error>
    // The `?` operator means:
    //  - if it's Ok(v), use v
    //  - if it's Err(e), return Err(e) from this function immediately
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

// =========================
// 🔀 MATCH (PATTERN MATCHING)
// =========================

pub fn classify(v: u8) -> &'static str {
    match v {
        // exact value
        0 => "empty",
        // inclusive range 1 to 10
        1..=10 => "low",
        // wildcard (anything else)
        _ => "other",
    }
}

// =========================
// ✅ CLOSURE (USED IN MENU)
// =========================

pub fn run_menu() {
    // `let action = || { ... };`
    //   ||      : closure with no parameters
    //   { ... } : closure body
    let action = || {
        info!("Running GUT test...");
        crate::run_grand_unified_test();
    };

    // call the closure just like a function
    action();
}


// ===============================================================
// ✅ SCROLL 2 — SSXL FILE LAYOUT MIRROR (ONE‑PAGE REFERENCE)
// ===============================================================
//
// This is NOT real code — it’s a structural “map” of the SSXL-ext
// project, showing how modules, files, and responsibilities connect.
// Every section mirrors your actual architecture.
//
// Use this as a compass when navigating the repo.
// ===============================================================


// ===============================================================
// 📁 src/main.rs — ENTRY POINT
// ===============================================================

fn main() {
    // Initialize logging, runtime, and console
    ssxl_cli::init_logging();
    ssxl_cli::start_runtime();
    ssxl_cli::run_console();
}

// Re-export actions so the menu can call them
pub use ssxl_cli::{
    run_grand_unified_test,
    launch_godot_project,
    start_runtime,
};


// ===============================================================
// 📁 src/ssxl_cli/mod.rs — CLI ROOT MODULE
// ===============================================================

pub mod ssxl_menu;        // interactive console menu
pub mod ssxl_testing;     // GUT tests (bitmask, noise, boundaries)
pub mod ssxl_api_scan;    // auto-generated GDScript API printer

pub fn init_logging() { /* tracing setup */ }
pub fn start_runtime() -> bool { /* engine init */ }
pub fn run_console() { ssxl_menu::run_interactive_loop(...); }


// ===============================================================
// 📁 src/ssxl_cli/ssxl_menu.rs — MENU SYSTEM
// ===============================================================
//
// - Defines CliAction struct
// - Builds menu options (A, L, G, U)
// - Handles keyboard input
// - Calls closures that trigger engine actions
//

pub struct CliAction {
    pub key: char,
    pub label: &'static str,
    pub id: &'static str,
    pub action: Box<dyn Fn()>,
}

pub fn build_menu() -> Vec<CliAction> { /* menu items */ }
pub fn run_interactive_loop(menu: Vec<CliAction>) { /* event loop */ }


// ===============================================================
// 📁 src/ssxl_cli/ssxl_testing.rs — GRAND UNIFIED TEST (GUT)
// ===============================================================
//
// Contains all four major test phases:
//
// 1. Bitmask Conversion Test
// 2. Max Grid Heavy Noise Benchmark
// 3. Fast Concurrency Test
// 4. Full Integration Test (Chunk Boundary Coherence)
//
// This is where your boundary mismatch panic originates.
//

pub fn run_grand_unified_test() {
    run_bitmask_test();
    run_max_grid_benchmark();
    run_fast_concurrency_test();
    run_full_integration_test(); // ❌ failing here
}


// ===============================================================
// 📁 ssxl_ext/lib.rs — ENGINE ROOT
// ===============================================================
//
// Re-exports engine modules so CLI can access them cleanly.
//

pub mod noise;
pub mod chunk;
pub mod api_registry;
pub mod runtime;

pub use noise::*;
pub use chunk::*;
pub use api_registry::*;
pub use runtime::*;


// ===============================================================
// 📁 ssxl_ext/noise.rs — NOISE TRAITS + IMPLEMENTATIONS
// ===============================================================
//
// - Defines NoiseGenerator trait
// - Provides Perlin/Simplex/etc implementations
// - SSXL chunk generation depends on this
//

pub trait NoiseGenerator {
    fn sample(&self, x: f32, y: f32) -> f32;
}

pub struct PerlinNoise { /* fields */ }
impl NoiseGenerator for PerlinNoise { /* sample() */ }


// ===============================================================
// 📁 ssxl_ext/chunk.rs — CHUNK STRUCT + BUILDER TRAIT
// ===============================================================
//
// - Defines Chunk<T>
// - Defines ChunkBuilder trait
// - Implements DefaultChunkBuilder
// - Handles world coordinate math
//

pub struct Chunk<'a, T> {
    pub x: i32,
    pub y: i32,
    pub data: Vec<T>,
    pub noise: &'a dyn NoiseGenerator,
}

pub trait ChunkBuilder {
    fn build_chunk(&self, x: i32, y: i32) -> Chunk<'_, u8>;
}

pub struct DefaultChunkBuilder<'a> {
    pub noise: &'a dyn NoiseGenerator,
}


// ===============================================================
// 📁 ssxl_ext/api_registry.rs — REFLECTION-LIKE API SYSTEM
// ===============================================================
//
// - Defines global registry Vec<String>
// - Provides export_api! macro
// - Provides list_api() for CLI menu option A
//

pub fn list_api() -> Vec<String> { /* returns registered methods */ }

#[macro_export]
macro_rules! export_api {
    (fn $name:ident ( $($args:tt)* ) -> $ret:ty $body:block) => {
        // registers function name in global registry
        // generates wrapper function
    };
}


// ===============================================================
// 📁 ssxl_ext/runtime.rs — ENGINE RUNTIME
// ===============================================================
//
// - Holds global noise generator
// - Holds global chunk builder
// - Provides get_chunk_builder()
// - Used by export_api! functions
//

static mut RUNTIME: Option<Runtime> = None;

pub struct Runtime {
    pub noise: Box<dyn NoiseGenerator>,
    pub builder: Box<dyn ChunkBuilder>,
}

pub fn get_chunk_builder() -> &'static dyn ChunkBuilder { /* ... */ }


// ===============================================================
// 📁 ssxl_cli/ssxl_api_scan.rs — AUTO-GENERATED API PRINTER
// ===============================================================
//
// - Calls list_api()
// - Prints all exported API methods
//

pub fn print_godot_api_surface() {
    let api = ssxl_ext::api_registry::list_api();
    for method in api {
        println!("• {}", method);
    }
}


// ===============================================================
// ✅ SUMMARY OF SSXL ARCHITECTURE
// ===============================================================
//
// main.rs
//   → ssxl_cli (menu, tests, API printer)
//   → ssxl_ext (engine: noise, chunks, registry, runtime)
//
// ssxl_cli
//   ├── ssxl_menu.rs
//   ├── ssxl_testing.rs
//   └── ssxl_api_scan.rs
//
// ssxl_ext
//   ├── noise.rs
//   ├── chunk.rs
//   ├── api_registry.rs
//   └── runtime.rs
//
// The CLI triggers engine actions.
// The engine exposes API via macros.
// The runtime holds global state.
// The GUT tests validate chunk/noise correctness.
//
// ===============================================================
// ✅ END OF SCROLL 2
// ===============================================================



// ============================================================================
// ✅ SCROLL 3 — PROCEDURAL MACRO ANATOMY (SSXL’s export_api! EXPLAINED)
// ============================================================================
//
// This is a *teardown* of how SSXL’s procedural macro system works.
// It mirrors the structure of your real export_api! macro, but simplified
// into a single-page, readable, annotated reference.
//
// This is NOT your real macro — it’s a conceptual Rosetta Stone.
// ============================================================================


// ============================================================================
// 📁 api_registry.rs — GLOBAL REGISTRY + MACRO ENTRY POINT
// ============================================================================

use std::sync::Mutex;

// A global list of exported API method names.
// SSXL uses this for the GDScript API printer.
static API_REGISTRY: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

pub fn list_api() -> Vec<String> {
    API_REGISTRY
        .lock()
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect()
}


// ============================================================================
// ✅ THE MACRO ITSELF (SIMPLIFIED MODEL OF SSXL’s export_api!)
// ============================================================================
//
// This macro does THREE THINGS:
//
// 1. Registers the function name in the global API registry
// 2. Generates a wrapper function with the same signature
// 3. Leaves your original function body intact
//
// The real SSXL macro is more complex, but this captures the structure.
// ============================================================================

#[macro_export]
macro_rules! export_api {
    // Pattern: a function with name, args, return type, and body
    (fn $name:ident ( $($args:tt)* ) -> $ret:ty $body:block) => {
        // ------------------------------------------------------------
        // 1. REGISTER THE FUNCTION NAME
        // ------------------------------------------------------------
        {
            // SAFETY: This is compile-time code generation.
            // The macro injects this into the module where it's used.
            let mut reg = $crate::api_registry::API_REGISTRY
                .lock()
                .expect("API registry poisoned");

            // stringify!($name) → "get_chunk_value"
            reg.push(stringify!($name));
        }

        // ------------------------------------------------------------
        // 2. GENERATE THE WRAPPER FUNCTION
        // ------------------------------------------------------------
        //
        // This wrapper is what gets called from Godot or CLI.
        // It has the same signature as the user-defined function.
        //
        // The wrapper simply calls the user’s function body.
        //
        // ------------------------------------------------------------
        pub fn $name( $($args)* ) -> $ret {
            // The user’s function body is injected here.
            $body
        }
    };
}


// ============================================================================
// ✅ HOW THE MACRO IS USED IN SSXL
// ============================================================================
//
// This is exactly how your SSXL API functions look.
// The macro expands into registry insertion + wrapper generation.
// ============================================================================

export_api! {
    fn get_chunk_value(x: i32, y: i32, index: usize) -> u8 {
        let builder = crate::runtime::get_chunk_builder();
        let chunk = builder.build_chunk(x, y);

        chunk.get(index)
            .copied()
            .unwrap_or(0)
    }
}


// ============================================================================
// ✅ WHAT THE MACRO EXPANDS INTO (CONCEPTUAL EXPANSION)
// ============================================================================
//
// The following is what the macro *effectively* generates:
//
// (This is NOT real code — it’s a conceptual expansion.)
// ============================================================================

/*

// 1. Register the function name
{
    let mut reg = API_REGISTRY.lock().unwrap();
    reg.push("get_chunk_value");
}

// 2. Generate the wrapper function
pub fn get_chunk_value(x: i32, y: i32, index: usize) -> u8 {
    let builder = crate::runtime::get_chunk_builder();
    let chunk = builder.build_chunk(x, y);

    chunk.get(index)
        .copied()
        .unwrap_or(0)
}

*/


// ============================================================================
// ✅ SUMMARY OF HOW SSXL’S MACRO SYSTEM WORKS
// ============================================================================
//
// • The macro is invoked with a normal function definition
// • It extracts the function name using `$name:ident`
// • It stringifies the name and inserts it into a global registry
// • It generates a wrapper function with the same signature
// • The wrapper contains the user’s original function body
//
// This gives SSXL:
//
// ✅ Reflection-like API discovery  
// ✅ Auto-generated GDScript API surface  
// ✅ Zero manual bookkeeping  
// ✅ No drift between code and documentation  
//
// ============================================================================
// ✅ END OF SCROLL 3
// ============================================================================


// ============================================================================
// ✅ SCROLL 4 — SSXL API REGISTRY INTERNALS (ONE‑PAGE REFERENCE)
// ============================================================================
//
// This scroll explains the *entire internal machinery* behind SSXL’s
// reflection‑like API registry. This is the system that:
//
//   • Collects exported API method names
//   • Stores them globally
//   • Lets the CLI print them (menu option A)
//   • Lets Godot discover callable Rust functions
//
// This is a conceptual, readable, SSXL‑accurate model.
// ============================================================================


// ============================================================================
// 📁 api_registry.rs — GLOBAL REGISTRY STORAGE
// ============================================================================
//
// The registry is a global Vec<String> protected by a Mutex.
// This allows safe mutation from anywhere in the program.
// ============================================================================

use std::sync::Mutex;

// Global registry of exported API method names.
// The real SSXL uses a similar pattern.
pub static API_REGISTRY: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());


// ============================================================================
// ✅ PUBLIC FUNCTION: list_api()
// ============================================================================
//
// The CLI uses this to print the GDScript API surface.
// The menu option A calls this directly.
// ============================================================================

pub fn list_api() -> Vec<String> {
    API_REGISTRY
        .lock()
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect()
}


// ============================================================================
// ✅ THE MACRO ENTRY POINT (export_api!)
// ============================================================================
//
// This is the heart of SSXL’s reflection-like system.
// The macro:
//
//   1. Registers the function name in API_REGISTRY
//   2. Generates a wrapper function with the same signature
//   3. Leaves your function body intact
//
// The real macro is more complex, but this captures the architecture.
// ============================================================================

#[macro_export]
macro_rules! export_api {
    // Pattern: function with name, args, return type, and body
    (fn $name:ident ( $($args:tt)* ) -> $ret:ty $body:block) => {
        // ------------------------------------------------------------
        // 1. REGISTER THE FUNCTION NAME
        // ------------------------------------------------------------
        {
            let mut reg = $crate::api_registry::API_REGISTRY
                .lock()
                .expect("API registry poisoned");

            // stringify!($name) → "get_chunk_value"
            reg.push(stringify!($name));
        }

        // ------------------------------------------------------------
        // 2. GENERATE THE WRAPPER FUNCTION
        // ------------------------------------------------------------
        //
        // This wrapper is what Godot and the CLI call.
        // It has the same signature as the user-defined function.
        //
        // ------------------------------------------------------------
        pub fn $name( $($args)* ) -> $ret {
            $body
        }
    };
}


// ============================================================================
// ✅ HOW THE MACRO IS USED IN SSXL
// ============================================================================
//
// This is exactly how your SSXL API functions look.
// The macro expands into registry insertion + wrapper generation.
// ============================================================================

export_api! {
    fn get_chunk_value(x: i32, y: i32, index: usize) -> u8 {
        let builder = crate::runtime::get_chunk_builder();
        let chunk = builder.build_chunk(x, y);

        chunk.get(index)
            .copied()
            .unwrap_or(0)
    }
}


// ============================================================================
// ✅ WHAT THE MACRO EXPANDS INTO (CONCEPTUAL EXPANSION)
// ============================================================================
//
// This is NOT real code — it’s a readable model of what the macro generates.
// ============================================================================

/*

{
    let mut reg = API_REGISTRY.lock().unwrap();
    reg.push("get_chunk_value");
}

pub fn get_chunk_value(x: i32, y: i32, index: usize) -> u8 {
    let builder = crate::runtime::get_chunk_builder();
    let chunk = builder.build_chunk(x, y);

    chunk.get(index)
        .copied()
        .unwrap_or(0)
}

*/


// ============================================================================
// ✅ HOW THE REGISTRY CONNECTS TO THE CLI
// ============================================================================
//
// The CLI menu option A calls:
//
//     ssxl_api_scan::print_godot_api_surface()
//
// That function calls:
//
//     list_api()
//
// Which returns the Vec<String> of all exported API methods.
//
// This gives you:
//
//   ✅ Auto-generated GDScript API surface
//   ✅ No manual bookkeeping
//   ✅ No drift between code and documentation
//   ✅ Reflection-like behavior without reflection
//
// ============================================================================
// ✅ END OF SCROLL 4
// ============================================================================




