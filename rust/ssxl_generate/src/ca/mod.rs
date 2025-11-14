// ssxl_generate/src/ca/mod.rs

//! Core module for the Cellular Automata (CA) generation algorithms.
//!
//! This logic is used by the `CellularAutomataGenerator` to simulate environment
//! growth (e.g., cave systems, walls, and borders) based on local neighborhood rules.
//! This module provides the tools necessary to enforce a state-based system
//! for structured, yet emergent, world design.

// --- Sub-Modules ---

/// Defines the specific CA rules (e.g., Survival and Birth rules like 4/5).
/// This module abstracts the core generation logic, allowing for easy experimentation
/// and swapping of different CA rule sets to achieve diverse world patterns.
pub mod rule_set;

/// Contains logic for checking a cell's immediate surroundings and calculating
/// the neighbor count based on the current state. This is the low-level
/// component that feeds the local state into the `rule_set`.
pub mod neighbor_check;