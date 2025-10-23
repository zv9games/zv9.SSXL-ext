use std::time::Instant;

/// 🧩 Trait abstraction for tilemap-like behavior
pub trait TileMapInterface {
    fn set_cell_ex(&mut self, layer: i32, pos: (i32, i32));
    fn source_id(&mut self, id: i32);
    fn atlas_coords(&mut self, coords: (i32, i32));
    fn alternative_tile(&mut self, alt: i32);
    fn done(&mut self);
}

/// 🧪 FakeTileMap — CLI-safe emulator for tile placement
pub struct FakeTileMap;

#[derive(Clone, Copy, Debug)]
pub struct Vec2i(pub i32, pub i32);

impl TileMapInterface for FakeTileMap {
    fn set_cell_ex(&mut self, layer: i32, pos: (i32, i32)) {
        println!("🧪 [FakeTileMap] set_cell_ex({}, {:?})", layer, pos);
    }

    fn source_id(&mut self, id: i32) {
        println!("🧪 [FakeTileMap] source_id({})", id);
    }

    fn atlas_coords(&mut self, coords: (i32, i32)) {
        println!("🧪 [FakeTileMap] atlas_coords({:?})", coords);
    }

    fn alternative_tile(&mut self, alt: i32) {
        println!("🧪 [FakeTileMap] alternative_tile({})", alt);
    }

    fn done(&mut self) {
        println!("🧪 [FakeTileMap] done()");
    }
}

/// 🧱 Pure Rust stand-in for TileInfo (no FFI)
#[allow(dead_code)]
#[derive(Debug)]
pub struct TileInfoStub {
    pub source_id: i32,
    pub atlas_coords: Vec2i,
    pub alternate_id: i32,
    pub rotation: u8,
    pub layer: u8,
    pub flags: u32,
}

/// 🧪 CLI test for tile generation and placement with metrics
#[allow(dead_code)]
pub fn test_generation_and_placement_cli() {
    println!("🧪 Running CLI test for generation and placement...");

    let mut tilemap = FakeTileMap;
    let total_tiles = 100;
    let layers = 3;
    let bytes_per_tile = std::mem::size_of::<TileInfoStub>();
    let total_bytes = bytes_per_tile * total_tiles * layers;

    let start = Instant::now();

    for layer in 0..layers {
        for i in 0..total_tiles {
            let pos = Vec2i((i % 8) as i32, (i / 8) as i32);
            let tile = TileInfoStub {
                source_id: 0,
                atlas_coords: pos,
                alternate_id: 0,
                rotation: 0,
                layer: layer as u8,
                flags: 0,
            };

            tilemap.set_cell_ex(layer as i32, (pos.0, pos.1));
            tilemap.source_id(tile.source_id);
            tilemap.atlas_coords((tile.atlas_coords.0, tile.atlas_coords.1));
            tilemap.alternative_tile(tile.alternate_id);
            tilemap.done();
        }
    }

    let duration = start.elapsed();
    let tiles_per_sec = (total_tiles * layers) as f64 / duration.as_secs_f64();

    println!("\n✅ CLI test completed with emulated tilemap.");
    println!("📊 Metrics Summary:");
    println!("• Layers simulated: {}", layers);
    println!("• Tiles placed: {}", total_tiles * layers);
    println!("• Duration: {:.2?}", duration);
    println!("• Throughput: {:.2} tiles/sec", tiles_per_sec);
    println!("• Estimated memory usage: {} bytes", total_bytes);
}
