#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use anyhow::{Context, Result};  // Gold: Err elegy.
use image::{open, ImageBuffer, Luma};
use rayon::prelude::*;
use log::info;  // Added: Native info! macro for consistent logging.

use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;
use aetherion_shared::zv9_shared_pipeline_data_tile::TileInfo;
use aetherion_shared::shared::SerializableVector2i;

#[cfg(feature = "png")]  // Feat-gate: Image png only.
use image::io::Reader as ImageReader;

#[derive(Debug, Clone)]
pub struct ChunkBuilder {
    blue_atlas: SerializableVector2i,
    black_atlas: SerializableVector2i,
    thresh: u8,
}

impl Default for ChunkBuilder {
    fn default() -> Self {
        Self {
            blue_atlas: SerializableVector2i { x: 0, y: 0 },
            black_atlas: SerializableVector2i { x: 0, y: 0 },
            thresh: 128,
        }
    }
}

// 👇 Move your builder methods into a separate inherent impl
impl ChunkBuilder {
    pub fn blue(mut self, x: i32, y: i32) -> Self {
        self.blue_atlas = SerializableVector2i { x, y };
        self
    }

    pub fn black(mut self, x: i32, y: i32) -> Self {
        self.black_atlas = SerializableVector2i { x, y };
        self
    }

    pub fn threshold(mut self, t: u8) -> Self {
        self.thresh = t;
        self
    }

    pub fn build(self) -> impl Fn(&str, u32) -> Result<MapDataChunk> {
        move |path: &str, scale: u32| {
            convert_world_png_to_chunk_with(
                path,
                scale,
                &self.blue_atlas,
                &self.black_atlas,
                self.thresh,
            )
        }
    }
}

/// 🖼️ Converts: Raw rite, batch birth—velocity vow.
#[inline]
pub fn convert_world_png_to_chunk(path: &str, scale: u32) -> Result<MapDataChunk> {
    convert_world_png_to_chunk_with(path, scale, &SerializableVector2i { x: 1, y: 0 }, &SerializableVector2i { x: 0, y: 0 }, 128)
}

fn convert_world_png_to_chunk_with(
    path: &str,
    scale: u32,
    blue: &SerializableVector2i,
    black: &SerializableVector2i,
    thresh: u8,
) -> Result<MapDataChunk> {
    if scale == 0 {
        anyhow::bail!("Scale must >0");
    }

    #[cfg(feature = "png")]
    {
        let img = ImageReader::open(path)
            .context("Failed to open PNG")?
            .with_guessed_format()?
            .decode()?
            .to_luma8();

        let (orig_w, orig_h) = img.dimensions();
        let scaled_w = orig_w as u64 * scale as u64;
        let scaled_h = orig_h as u64 * scale as u64;
        let total_tiles = scaled_w * scaled_h;
        if total_tiles > 1_000_000_000 {
            anyhow::bail!("Output too large: {} tiles", total_tiles);
        }

        let raw = img.into_raw();
        let orig_w_usize = orig_w as usize;

        let use_par = scaled_h > 100;
        let tiles: Vec<(SerializableVector2i, TileInfo)> = if use_par {
            (0..scaled_h as usize)
                .into_par_iter()
                .flat_map(|y| {
                    let mut row_tiles = Vec::with_capacity(scaled_w as usize);
                    let src_y = (y as u64 / scale as u64) as usize;
                    for x in 0..scaled_w as usize {
                        let src_x = (x as u64 / scale as u64) as usize;
                        let pixel_idx = src_y * orig_w_usize + src_x;
                        let pixel = raw[pixel_idx];

                        let atlas = if pixel > thresh { blue.clone() } else { black.clone() };
                        let tile = TileInfo::new(atlas, 0);
                        let pos = SerializableVector2i { x: x as i32, y: y as i32 };
                        row_tiles.push((pos, tile));
                    }
                    row_tiles
                })
                .collect()
        } else {
            let mut all_tiles = Vec::with_capacity(total_tiles as usize);
            for y in 0..scaled_h as usize {
                let src_y = (y as u64 / scale as u64) as usize;
                for x in 0..scaled_w as usize {
                    let src_x = (x as u64 / scale as u64) as usize;
                    let pixel_idx = src_y * orig_w_usize + src_x;
                    let pixel = raw[pixel_idx];

                    let atlas = if pixel > thresh { blue.clone() } else { black.clone() };
                    let tile = TileInfo::new(atlas, 0);
                    let pos = SerializableVector2i { x: x as i32, y: y as i32 };
                    all_tiles.push((pos, tile));
                }
            }
            all_tiles
        };

        let start = std::time::Instant::now();
        let mut chunk = MapDataChunk::new();
        chunk.tiles.extend(tiles.into_iter());

        let elapsed = start.elapsed();
        info!(
            "bitmask: Converted {} tiles in {:.2?} (~{} tiles/sec)",
            chunk.len(),
            elapsed,
            chunk.len() as f64 / elapsed.as_secs_f64()
        );

        Ok(chunk)
    }

    #[cfg(not(feature = "png"))]
    {
        anyhow::bail!("PNG support disabled—enable 'png' feature");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_png_convert() {
        // Assume test.png 2x2 white/black.
        let chunk = convert_world_png_to_chunk("test.png", 1).unwrap();
        assert_eq!(chunk.len(), 4);  // Tiles match.
    }

    #[bench]
    fn bench_convert_512(b: &mut test::Bencher) {
        b.iter(|| convert_world_png_to_chunk("test_512.png", 1));
    }
}