

#[allow(unused_imports)]

use aetherion_shared::zv9_prelude::*;


#[derive(Default)]
pub struct Config {
    /// Emit `#[derive(Debug)]`
    pub derive_debug: bool,

    /// Emit `#[derive(Clone)]`
    pub derive_clone: bool,

    /// Emit `impl Display` block
    pub emit_display_impl: bool,

    /// Include `#[derive(Serialize, Deserialize)]`
    pub use_serde: bool,
}

// the end