// ssxl_cli/src/scan/mod.rs 

//! # Codebase Scan Utilities (`ssxl_cli::scan`)
//!
//! This module groups all functionality related to scanning and analyzing the
//! SSXL-ext codebase, such as calculating Lines of Code (LOC) and generating
//! reports. It serves as a façade to its internal sub-modules.

// Removed all unused imports: tracing macros, std::io, std::env, std::fs, std::process::Command, and std::process::Stdio.

/// Internal module responsible for recursively walking the file system.
mod file_walker;

/// Internal module responsible for writing the report file contents.
/// FIX: Declare report_writer as a sibling here, allowing it to be imported in report_formatter.rs via `super::`.
mod report_writer;

/// Internal module responsible for orchestrating the scan and delegating report formatting.
mod report_formatter;


// Publicly re-export the main entry point for the LOC scan utility.
pub use report_formatter::execute_loc_scan;