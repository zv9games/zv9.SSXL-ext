// ssxl_shared/src/message/mod.rs

pub mod generation_message;
pub mod messages;

// FIX 1: Publicly re-export AnimationUpdate from the messages module.
// This makes the type available as `ssxl_shared::message::AnimationUpdate`.
pub use messages::AnimationUpdate;