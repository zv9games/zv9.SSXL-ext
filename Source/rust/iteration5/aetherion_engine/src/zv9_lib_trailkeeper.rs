// 📚 Trailkeeper modules
pub mod zv9_trailkeeper_collector { include!("zv9_trailkeeper_collector.rs"); }
pub mod zv9_trailkeeper_config { include!("zv9_trailkeeper_config.rs"); }
pub mod zv9_trailkeeper_entry { include!("zv9_trailkeeper_entry.rs"); }
pub mod zv9_trailkeeper_export { include!("zv9_trailkeeper_export.rs"); }
pub mod zv9_trailkeeper_registry { include!("zv9_trailkeeper_registry.rs"); }
pub mod zv9_trailkeeper_scan { include!("zv9_trailkeeper_scan.rs"); }
pub mod zv9_trailkeeper_watch { include!("zv9_trailkeeper_watch.rs"); }

// 🧭 Trailkeeper re-exports
pub mod trailkeeper {
    pub use super::zv9_trailkeeper_collector::*;
    pub use super::zv9_trailkeeper_config::*;
    pub use super::zv9_trailkeeper_entry::*;
    pub use super::zv9_trailkeeper_export::*;
    pub use super::zv9_trailkeeper_registry::*;
    pub use super::zv9_trailkeeper_scan::*;
    pub use super::zv9_trailkeeper_watch::*;

    pub mod collector {
        pub use super::super::zv9_trailkeeper_collector::*;
    }
    pub mod config {
        pub use super::super::zv9_trailkeeper_config::*;
    }
    pub mod scan {
        pub use super::super::zv9_trailkeeper_scan::*;
    }
    pub mod entry {
        pub use super::super::zv9_trailkeeper_entry::*;
    }
    pub mod registry {
        pub use super::super::zv9_trailkeeper_registry::*;
    }
}
