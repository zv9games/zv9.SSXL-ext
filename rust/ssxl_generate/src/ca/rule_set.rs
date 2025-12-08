// ============================================================================
// 🧩 Cellular Automata Rule Set (`crate::ca::rule_set`)
// ----------------------------------------------------------------------------
// This module defines the rulesets and transition logic for the Cellular
// Automata (CA) subsystem of the SSXL engine. It determines how tiles evolve
// from one generation to the next based on their current state and the number
// of live neighbors.
//
// Purpose:
//   • Provide reusable constants for identifying different CA rulesets.
//   • Implement the core transition function (`get_next_tile_type`) that applies
//     Birth/Survival rules to evolve tile states.
//   • Support multiple terrain generation styles (caves, mazes, static fills).
//
// Rule Set Identifiers:
//   • RULE_BASIC_CAVE (0)
//       - Birth: 4–5 neighbors
//       - Survival: 1–7 neighbors
//       - Produces large, open cave-like structures.
//   • RULE_MAZE (1)
//       - Birth: exactly 3 neighbors
//       - Survival: 1–4 neighbors
//       - Produces thin, winding maze-like corridors.
//   • RULE_SOLID (2)
//       - Static rule: fills all tiles with Rock.
//   • RULE_CHECKERBOARD (3)
//       - Static rule: alternates Rock/Void in a checkerboard pattern.
//
// Function: get_next_tile_type
//   • Arguments:
//       - current_type: current tile state (`TileType`).
//       - live_neighbors: number of Rock neighbors (0–8).
//       - ruleset: identifier for which ruleset to apply.
//   • Returns:
//       - Next tile state (`TileType`).
//
// Workflow:
//   1. Select Birth/Survival ranges based on ruleset.
//   2. If tile is Rock (alive):
//        - Survives if neighbor count is within survival range.
//        - Otherwise becomes Void.
//   3. If tile is Void (dead):
//        - Becomes Rock if neighbor count is within birth range.
//        - Otherwise remains Void.
//   4. Other tile types (e.g., Water, Ore) remain unchanged.
//
// Design Choices:
//   • Encapsulates ruleset logic in a single function for clarity.
//   • Uses pattern matching to separate Rock/Void cases.
//   • Birth/Survival ranges are defined per ruleset for flexibility.
//   • Supports extensibility: new rulesets can be added easily.
//
// Educational Note:
//   • Cellular Automata rulesets are often expressed in "B/S notation"
//     (e.g., B3/S1-4 for Maze, B4-5/S1-7 for Cave).
//   • This function demonstrates how to translate those rules into Rust logic,
//     enabling reproducible terrain generation across chunks.
// ============================================================================


use ssxl_shared::TileType;

pub const RULE_BASIC_CAVE: u8 = 0;
pub const RULE_MAZE: u8 = 1;
pub const RULE_SOLID: u8 = 2;
pub const RULE_CHECKERBOARD: u8 = 3;

pub fn get_next_tile_type(current_type: TileType, live_neighbors: u8, ruleset: u8) -> TileType {
    let (birth_min, birth_max, survive_min, survive_max) = match ruleset {
        RULE_MAZE => (3, 3, 1, 4),
        RULE_BASIC_CAVE | _ => (4, 5, 1, 7),
    };

    match current_type {
        TileType::Rock => {
            if live_neighbors >= survive_min && live_neighbors <= survive_max {
                TileType::Rock
            } else {
                TileType::Void
            }
        }
        TileType::Void => {
            if live_neighbors >= birth_min && live_neighbors <= birth_max {
                TileType::Rock
            } else {
                TileType::Void
            }
        }
        _ => current_type,
    }
}
