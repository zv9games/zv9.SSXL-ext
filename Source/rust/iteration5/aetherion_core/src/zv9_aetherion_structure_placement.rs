use aetherion_shared::zv9_prelude::*;
use aetherion_shared::zv9_shared_pipeline_data_grid::{MapGrid, TileType};

// 🏗 Dummy structure placement function for testing and integration.
pub fn place_structure_stub(grid: &mut MapGrid, pos: Position) {
    // Define a 3×3 region starting from the given position
    let bounds = GridBounds::new(
        SerializableVector2i { x: pos.x, y: pos.y },
        SerializableVector2i { x: 3, y: 3 },
    );
    let tile_type = TileType::Chunk;

    // Place tiles within bounds
    for p in bounds.iter() {
        let position = Position { x: p.x, y: p.y };
        grid.set(position, tile_type);
    }

    // Optional: Log the placement event
    /*
    Trailkeeper::record(LogEntry {
        event_type: EventType::StructurePlacement,
        timestamp: chrono::Utc::now(),
        actor: "stub".into(),
        description: format!("Placed dummy structure at {:?}", pos),
        affected_components: vec![format!("{:?}", bounds)],
        status: LogStatus::Success,
    });
    */
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_structure_placement_centered() {
        let bounds = GridBounds::new(
            SerializableVector2i { x: 0, y: 0 },
            SerializableVector2i { x: 64, y: 64 },
        );
        let mut grid = MapGrid::new(bounds);
        let pos = Position { x: 30, y: 30 };
        place_structure_stub(&mut grid, pos);

        let structure_bounds = GridBounds::new(
            SerializableVector2i { x: pos.x, y: pos.y },
            SerializableVector2i { x: 3, y: 3 },
        );
        for p in structure_bounds.iter() {
            let placed = grid.get(Position { x: p.x, y: p.y });
            assert_eq!(placed, Some(TileType::Chunk));
        }
    }

    #[test]
    fn stress_structure_placement_near_edge() {
        let bounds = GridBounds::new(
            SerializableVector2i { x: 0, y: 0 },
            SerializableVector2i { x: 8, y: 8 },
        );
        let mut grid = MapGrid::new(bounds);
        let pos = Position { x: 6, y: 6 };
        place_structure_stub(&mut grid, pos);

        let structure_bounds = GridBounds::new(
            SerializableVector2i { x: pos.x, y: pos.y },
            SerializableVector2i { x: 3, y: 3 },
        );
        for p in structure_bounds.iter() {
            let _ = grid.get(Position { x: p.x, y: p.y }); // Should not panic
        }
    }

    #[test]
    fn stress_multiple_stub_placements() {
        let bounds = GridBounds::new(
            SerializableVector2i { x: 0, y: 0 },
            SerializableVector2i { x: 128, y: 128 },
        );
        let mut grid = MapGrid::new(bounds);

        for i in 0..10 {
            let pos = Position { x: i * 10, y: i * 10 };
            place_structure_stub(&mut grid, pos);
        }

        let total_chunks = grid.count_type(TileType::Chunk);
        assert!(total_chunks >= 90); // 10 placements × ~9 tiles each
    }
}
