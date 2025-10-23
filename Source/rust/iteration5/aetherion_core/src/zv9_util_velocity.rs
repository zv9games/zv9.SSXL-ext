
#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Velocity { dx, dy }
    }

    pub fn zero() -> Self {
        Velocity { dx: 0.0, dy: 0.0 }
    }

    pub fn apply(&self, position: &mut crate::zv9_util_position::Position) {
		position.x += self.dx as i32;
		position.y += self.dy as i32;
	}

	
	pub fn scale(&self, factor: f64) -> Velocity {

        Velocity {
            dx: (self.dx as f64 * factor) as f32,
            dy: (self.dy as f64 * factor) as f32,
        }
    }

    pub fn increase(&mut self, amount: f64) {
		self.dx += amount as f32;
		self.dy += amount as f32;
	} // ← this was missing
}

// the end