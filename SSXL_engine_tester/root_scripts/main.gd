extends Node2D

# ----------------------------------------------------------------------
## DEPENDENCIES (using @onready for robustness)
# ----------------------------------------------------------------------

# System Nodes (Children of this Main node, must match GDExtension names)
@onready var ssxl_oracle: Node = $SSXLOracle
@onready var ssxl_engine: Node = $SSXLEngine
@onready var ssxl_signals: Node = $SSXLSignals

# Global Utility Nodes (Explicitly typed for performance)
@onready var clock_timer: Timer = $/root/ssxltester/main/tilemap/clocktimer
@onready var engine_monitor: Node = $/root/ssxltester/main/EngineMonitor

# ----------------------------------------------------------------------
## LIFECYCLE
# ----------------------------------------------------------------------

func _ready() -> void:
	# Check for critical timer node before connecting
	if is_instance_valid(clock_timer):
		# Start the primary orchestration loop
		clock_timer.timeout.connect(_on_clock_tick)
		clock_timer.start()
	else:
		push_error("❌ Clock timer not found. Main orchestration failed to start.")
	
	# We call this here to ensure all engine links are established immediately
	# after the clock timer is configured.
	enter_idle_state()


# Initializes the links between the Godot-side controllers and the Rust GDExtension engine.
func enter_idle_state() -> void:
	print("\n🎬 Main: Entering idle state. Systems standing by...")

	# Validate all core systems before proceeding
	if is_instance_valid(ssxl_oracle) and is_instance_valid(ssxl_engine) and is_instance_valid(engine_monitor):
		
		# --- 1. Oracle Link (Tells the Oracle which Engine to command) ---
		print("🔗 Linking Oracle to Engine...")
		ssxl_oracle.set_engine(ssxl_engine) 
		print("🔗 Oracle: Engine link established.")

		# --- 2. Monitor Link (Tells the Monitor which Engine to observe) ---
		print("📡 Linking EngineMonitor to Engine...")
		engine_monitor.set_engine(ssxl_engine)
		print("✅ EngineMonitor: Engine linked.")

		# --- 3. Signals Link (Tells the Engine which Godot Node to use for signal emission) ---
		if is_instance_valid(ssxl_signals):
			# 💡 FIX APPLIED: Changed AetherionSignals to SSXLSignals
			print("📶 Connecting Engine to SSXLSignals...")
			ssxl_engine.set_signals_node(ssxl_signals) 
			
			# The Monitor-Signal connection is trusted to happen within set_engine()
			print("✅ EngineMonitor connected to status signal.")
		else:
			push_warning("⚠️ SSXLSignals node not found. Signal emission connection skipped.")

		# --- 4. First Pulse ---
		# The Oracle's initial tick primes the Rust system's state machine.
		print("🔮 Oracle linked. Delivering first pulse...")
		ssxl_oracle.tick() 

		print("⚙️ Engine confirmed idle.")
	else:
		# Use push_error to make this visible in the debugger
		push_error("❌ Failed to link Core Systems (Oracle/Engine/Monitor). Invocation aborted.")


# ----------------------------------------------------------------------
## RUNTIME (Clock Tick)
# ----------------------------------------------------------------------

# Drives the core update loop for the engine and monitor systems.
func _on_clock_tick() -> void:
	print("🕰️ Clock tick.")

	if is_instance_valid(ssxl_oracle):
		# Tick the Oracle, which in turn commands the Rust engine
		ssxl_oracle.tick() 

	if is_instance_valid(engine_monitor):
		# Polls the engine for its current status (should be throttled or non-blocking)
		engine_monitor.update_status()
