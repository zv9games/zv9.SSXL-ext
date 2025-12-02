use std::sync::{Arc, atomic::{AtomicUsize, AtomicU64, Ordering}};
use ssxl_sync::AtomicResource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductorStatus {
	Initializing,
	Running,
	Paused,
	Generating,
	Stopping,
	ShuttingDown,
	Error,
}

#[derive(Clone)]
pub struct ConductorState {
	status: AtomicResource<ConductorStatus>,
	queue_depth: Arc<AtomicUsize>,
	active_generator_id: AtomicResource<String>,
	tile_counter: Arc<AtomicU64>,
}

impl ConductorState {
	pub fn new(initial_generator_id: String) -> Self {
		ConductorState {
			status: AtomicResource::new(ConductorStatus::Initializing),
			queue_depth: Arc::new(AtomicUsize::new(0)),
			active_generator_id: AtomicResource::new(initial_generator_id),
			tile_counter: Arc::new(AtomicU64::new(0)),
		}
	}

	pub fn get_status(&self) -> ConductorStatus {
        *self.status.read()
	}
	
	pub fn is_active(&self) -> bool {
		let current_status = self.get_status();
		current_status == ConductorStatus::Running || current_status == ConductorStatus::Generating
	}

	pub fn get_queue_depth(&self) -> usize {
		self.queue_depth.load(Ordering::Relaxed)
	}

	pub fn get_tiles_placed(&self) -> u64 {
		self.tile_counter.load(Ordering::Relaxed)
	}

	pub fn get_active_generator_id(&self) -> String {
		self.active_generator_id.read().clone()
	}

	pub fn increment_queue_depth(&self) {
		self.queue_depth.fetch_add(1, Ordering::Relaxed);
	}

	pub fn decrement_queue_depth(&self) {
		self.queue_depth.fetch_sub(1, Ordering::Relaxed);
	}
	
	pub(crate) fn increment_tile_count(&self, amount: u64) {
		self.tile_counter.fetch_add(amount, Ordering::Relaxed);
	}

	pub(crate) fn set_status(&self, new_status: ConductorStatus) {
        *self.status.write() = new_status;
	}

	pub(crate) fn set_active_generator_id(&self, id: &str) {
        *self.active_generator_id.write() = id.to_string();
	}
}