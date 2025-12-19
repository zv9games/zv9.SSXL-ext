use once_cell::sync::Lazy;

/// SSXL-optimized bitmask → tile ID mapping.
/// Built once at runtime, then reused forever.
pub fn bitmask_to_id(bitmask: u8) -> i32 {
    static LUT: Lazy<[i32; 256]> = Lazy::new(|| {
        let mut table = [0i32; 256];

        for mask in 0..=255 {
            table[mask as usize] = match mask {
                0 => 0, // isolated

                // cardinal edges
                0b00001000 => 1, // top
                0b00000010 => 2, // right
                0b00000100 => 3, // bottom
                0b00000001 => 4, // left

                // outer corners
                0b00011000 => 5, // UL
                0b00101000 => 6, // UR
                0b01000100 => 7, // DR
                0b10000001 => 8, // DL

                // inner corners
                0b00001111 => 9,  // UL
                0b00011110 => 10, // UR
                0b00111100 => 11, // DR
                0b11110000 => 12, // DL

                // T-junctions
                0b00001110 => 13, // missing left
                0b00011100 => 14, // missing top
                0b00111000 => 15, // missing right
                0b01110000 => 16, // missing bottom

                // cross junctions
                0b00011111 => 17,
                0b00111110 => 18,
                0b01111100 => 19,
                0b11111000 => 20,

                // full surrounded
                0b11111111 => 21,

                // fallback
                _ => (mask % 48) as i32,
            };
        }

        table
    });

    LUT[bitmask as usize]
}
