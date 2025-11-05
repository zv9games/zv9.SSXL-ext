// ssxl_generate/src/ca/rule_set.rs

use ssxl_shared::tile_type::TileType;

// --- RULESET DEFINITIONS ---
pub const RULE_BASIC_CAVE: u8 = 0; // Generates large, open cave systems.
pub const RULE_MAZE: u8 = 1;        // Generates thin, winding maze/pillar structures.
pub const RULE_SOLID: u8 = 2;       // Fills the entire chunk with a solid tile.
pub const RULE_CHECKERBOARD: u8 = 3; // Generates a checkerboard pattern.

/// Determines the next tile type based on the current type, live neighbors, and the active ruleset.
pub fn get_next_tile_type(current_type: TileType, live_neighbors: u8, ruleset: u8) -> TileType {
    // NOTE: Only handles Rock/Void transitions.

    // Define Birth (B) and Survival (S) conditions based on the ruleset
    // Bx/Sy: Birth if live neighbors in [x], Survive if live neighbors in [y].
    let (birth_min, birth_max, survive_min, survive_max) = match ruleset {
        RULE_MAZE => (3, 3, 1, 4),          // B3/S1-4 (Favors thin, complex structures)
        RULE_BASIC_CAVE | _ => (4, 5, 1, 7), // B4-5/S1-7 (Favors large, open caves)
    };

    match current_type {
        TileType::Rock => {
            // Survival Rule: Rock stays Rock
            if live_neighbors >= survive_min && live_neighbors <= survive_max {
                TileType::Rock
            } else {
                TileType::Void // Dies (becomes Void)
            }
        }
        TileType::Void => {
            // Birth Rule: Void becomes Rock
            if live_neighbors >= birth_min && live_neighbors <= birth_max {
                TileType::Rock
            } else {
                TileType::Void // Stays Void
            }
        }
        // Preserve any other tile types
        _ => current_type,
    }
}