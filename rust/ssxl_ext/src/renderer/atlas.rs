use godot::prelude::*;

/// ------------------------------------------------------------
/// SSXL Atlas (Plan B)
///
/// Maps tile IDs → UV rectangles inside a texture atlas.
/// ------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct AtlasUV {
    pub uv_min: Vector2,   // bottom-left corner (0–1)
    pub uv_max: Vector2,   // top-right corner (0–1)
}

#[derive(Debug)]
pub struct SSXLAtlas {
    /// Map tile_id → UV rectangle
    uv_map: Vec<AtlasUV>,

    /// Atlas grid dimensions (for uniform atlases)
    pub tiles_x: i32,
    pub tiles_y: i32,
}

impl SSXLAtlas {
    /// Create a new uniform grid atlas.
    pub fn new_uniform(tiles_x: i32, tiles_y: i32) -> Self {
        if tiles_x <= 0 || tiles_y <= 0 {
            godot_warn!(
                "DEBUG SSXLAtlas: INVALID atlas size {}x{} — materials may appear blank",
                tiles_x, tiles_y
            );
        }

        let mut uv_map = Vec::new();
        uv_map.resize(
            (tiles_x * tiles_y) as usize,
            AtlasUV {
                uv_min: Vector2::ZERO,
                uv_max: Vector2::ONE,
            },
        );

        let tile_w = 1.0 / tiles_x as f32;
        let tile_h = 1.0 / tiles_y as f32;

        for ty in 0..tiles_y {
            for tx in 0..tiles_x {
                let id = (ty * tiles_x + tx) as usize;

                let u0 = tx as f32 * tile_w;
                let v0 = ty as f32 * tile_h;

                let uv_min = Vector2::new(u0, v0);
                let uv_max = Vector2::new(u0 + tile_w, v0 + tile_h);

                // DEBUG: UV sanity check
                if uv_min.x < 0.0 || uv_min.y < 0.0 || uv_max.x > 1.0 || uv_max.y > 1.0 {
                    godot_warn!(
                        "DEBUG SSXLAtlas: UV OUT OF RANGE for tile {} -> min={:?}, max={:?}",
                        id, uv_min, uv_max
                    );
                }

                uv_map[id] = AtlasUV { uv_min, uv_max };
            }
        }

        godot_print!(
            "DEBUG SSXLAtlas: created uniform atlas {}x{} ({} tiles)",
            tiles_x,
            tiles_y,
            uv_map.len()
        );

        SSXLAtlas {
            uv_map,
            tiles_x,
            tiles_y,
        }
    }

    /// Create an empty atlas (for custom UV assignment)
    pub fn new_empty() -> Self {
        godot_warn!(
            "DEBUG SSXLAtlas: new_empty() called — NO UVs defined. Material will appear uniform."
        );

        SSXLAtlas {
            uv_map: Vec::new(),
            tiles_x: 0,
            tiles_y: 0,
        }
    }

    /// Assign a custom UV rect to a tile ID.
    pub fn set_custom_uv(&mut self, tile_id: usize, uv_min: Vector2, uv_max: Vector2) {
        if tile_id >= self.uv_map.len() {
            godot_warn!(
                "DEBUG SSXLAtlas: expanding atlas for custom tile_id={} (old size={})",
                tile_id,
                self.uv_map.len()
            );

            self.uv_map.resize(
                tile_id + 1,
                AtlasUV {
                    uv_min: Vector2::ZERO,
                    uv_max: Vector2::ONE,
                },
            );
        }

        // DEBUG: UV sanity check
        if uv_min.x < 0.0 || uv_min.y < 0.0 || uv_max.x > 1.0 || uv_max.y > 1.0 {
            godot_warn!(
                "DEBUG SSXLAtlas: custom UV OUT OF RANGE for tile_id={} -> min={:?}, max={:?}",
                tile_id,
                uv_min,
                uv_max
            );
        }

        self.uv_map[tile_id] = AtlasUV { uv_min, uv_max };

        godot_print!(
            "DEBUG SSXLAtlas: set_custom_uv tile_id={} min={:?} max={:?}",
            tile_id,
            uv_min,
            uv_max
        );
    }

    /// Retrieve UVs for a tile ID.
    /// If out of range, returns a full‑atlas UV (safe fallback).
    pub fn get_uv(&self, tile_id: usize) -> AtlasUV {
        if tile_id < self.uv_map.len() {
            let uv = self.uv_map[tile_id].clone();

            // DEBUG: sanity check
            if uv.uv_min == Vector2::ZERO && uv.uv_max == Vector2::ONE {
                godot_warn!(
                    "DEBUG SSXLAtlas: tile_id={} using FULL-ATLAS UVs — texture may appear stretched",
                    tile_id
                );
            }

            return uv;
        }

        godot_warn!(
            "DEBUG SSXLAtlas: tile_id={} OUT OF RANGE (max={}) — using fallback UVs",
            tile_id,
            self.uv_map.len()
        );

        AtlasUV {
            uv_min: Vector2::ZERO,
            uv_max: Vector2::ONE,
        }
    }
}
