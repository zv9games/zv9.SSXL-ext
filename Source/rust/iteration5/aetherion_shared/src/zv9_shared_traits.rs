
#[allow(unused_imports)]
use crate::zv9_prelude::*;


pub trait Tickable {
    /// Called once per frame with the elapsed time in seconds.
    fn tick(&mut self, delta_time: f32);
}

/// Trait for serializable game data.
/// Allows conversion to and from string-based formats.
pub trait Serializable {
    /// Serializes the object into a string.
    fn serialize(&self) -> String;

    /// Deserializes the object from a string.
    fn deserialize(s: &str) -> Self
    where
        Self: Sized;
}
//end traits.rs