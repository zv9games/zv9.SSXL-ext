// ─── Standard Library ──────────────────────────────────────────────────────────
pub use std::collections::*;
pub use std::fmt::{Debug, Display};
pub use std::sync::{Arc, Mutex, RwLock};
pub use std::time::{Duration, Instant};

// ─── External Crates ───────────────────────────────────────────────────────────
pub use chrono::{DateTime, Utc};
pub use log::{info, warn, error, debug};
pub use once_cell::sync::Lazy;
pub use rand::{Rng, SeedableRng};
pub use rand::rngs::SmallRng;
pub use rayon::prelude::*;
pub use serde::{Serialize, Deserialize};
pub use thiserror::Error;

// ─── Shared Modules ────────────────────────────────────────────────────────────
pub use crate::shared::*;
pub use crate::zv9_shared_math::{clamp, TAU};
pub use crate::zv9_shared_types::{Coord, EntityId, SerializableVector2i, Timestamp};
pub use crate::zv9_shared_traits::{Serializable, Tickable};
pub use crate::zv9_shared_spatial::{all_neighbors, cardinal_neighbors, in_bounds};
pub use crate::zv9_shared_grid2d::Grid2D;
pub use crate::zv9_shared_grid_bounds::GridBounds;
pub use crate::zv9_shared_pipeline_data_chunk::MapDataChunk;
pub use crate::zv9_shared_pipeline_data_tile::TileInfo;
pub use crate::zv9_shared_config::EngineConfig;


