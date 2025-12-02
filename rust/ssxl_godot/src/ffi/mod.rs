pub mod oracle;
pub mod signals;        // this loads src/ffi/signals.rs

pub use oracle::SSXLOracle;
pub use signals::SSXLSignals;   // re-export the class