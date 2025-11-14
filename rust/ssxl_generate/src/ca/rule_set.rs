// ssxl_generate/src/ca/rule_set.rs
use ssxl_shared::tile_type::TileType;

// --- 1. Rule Set Identifiers ---

/// Identifier for the standard cave generation ruleset (B4-5/S1-7).
pub const RULE_BASIC_CAVE: u8 = 0;
/// Identifier for the maze-like generation ruleset (B3/S1-4).
pub const RULE_MAZE: u8 = 1;
/// Placeholder/future rule for generating a solid block.
pub const RULE_SOLID: u8 = 2;
/// Placeholder/future rule for generating a checkerboard pattern.
pub const RULE_CHECKERBOARD: u8 = 3;

// --- 2. Core Rule Application Function ---

/// Determines the next state of a tile based on the current state, live neighbor count, and a specific ruleset.
///
/// This implements the standard Birth/Survival (B/S) rules for cellular automata.
///
/// # Arguments
/// * `current_type`: The tile's state at the current CA iteration (Void or Rock).
/// * `live_neighbors`: The count of surrounding `TileType::Rock` tiles (0-8).
/// * `ruleset`: The identifier defining the B/S parameters to use.
///
/// # Returns
/// The tile's state for the next CA iteration.
pub fn get_next_tile_type(current_type: TileType, live_neighbors: u8, ruleset: u8) -> TileType {

    // Define the specific Birth (B) and Survival (S) parameters based on the ruleset ID.
    // Bx-y: Tile will be born (become Rock) if live_neighbors is in range [x, y] and current state is Void.
    // Sx-y: Tile will survive (remain Rock) if live_neighbors is in range [x, y] and current state is Rock.
    let (birth_min, birth_max, survive_min, survive_max) = match ruleset {
        RULE_MAZE => (3, 3, 1, 4),               // B3/S1-4 (Favors thin, complex structures with few dead ends)
        RULE_BASIC_CAVE | _ => (4, 5, 1, 7),     // B4-5/S1-7 (Favors large, open, robust cave systems)
    };

    match current_type {
        // --- Survival Check: If the tile is currently Rock (alive) ---
        TileType::Rock => {
            if live_neighbors >= survive_min && live_neighbors <= survive_max {
                // Within survival range: Rock survives.
                TileType::Rock
            } else {
                // Outside survival range: Rock dies (becomes Void).
                TileType::Void
            }
        }
        // --- Birth Check: If the tile is currently Void (dead) ---
        TileType::Void => {
            if live_neighbors >= birth_min && live_neighbors <= birth_max {
                // Within birth range: Void becomes Rock (birth).
                TileType::Rock
            } else {
                // Outside birth range: Void remains Void.
                TileType::Void
            }
        }
        // Handle any other TileTypes outside the CA simulation (e.g., Water, Ore) by leaving them unchanged.
        _ => current_type,
    }
}