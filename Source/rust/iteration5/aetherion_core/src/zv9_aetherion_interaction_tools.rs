

use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;

/// Marker trait for editor tools that operate on map chunks.
pub trait TileTool {
    fn name(&self) -> &'static str;
    fn apply(&self, _chunk: &mut MapDataChunk) {
        // No-op placeholder
    }
}

/// Dummy tool to silence unused import warning and establish structure.
pub struct NullTool;

impl TileTool for NullTool {
    fn name(&self) -> &'static str {
        "null"
    }
}



// the end