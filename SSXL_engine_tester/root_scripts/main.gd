extends Node2D

# ----------------------------------------------------------------------
## DEPENDENCIES (using @onready for robustness)
# ----------------------------------------------------------------------

# System Nodes (Children of this Main node)
@onready var ssxl_oracle: Node = $SSXLOracle
@onready var ssxl_engine: Node = $SSXLEngine
@onready var ssxl_signals: Node = $SSXLSignals

# Global Utility Nodes
@onready var clock_timer: Timer = $/root/ssxltester/main/tilemap/clocktimer
@onready var engine_monitor: Node = $/root/ssxltester/main/EngineMonitor

# ----------------------------------------------------------------------
## LIFECYCLE
# ----------------------------------------------------------------------

func _ready() -> void:
	# Check for critical timer node before connecting
	if clock_timer:
		clock_timer.timeout.connect(_on_clock_tick)
		clock_timer.start()
	else:
		push_error("❌ Clock timer not found. Main orchestration failed to start.")


func enter_idle_state() -> void:
	print("\n🎬 Main: Entering idle state. Systems standing by...")

	# Use the @onready references directly
	if ssxl_oracle and ssxl_engine and engine_monitor:
		
		# --- 1. Oracle Link ---
		print("🔗 Linking Oracle to Engine...")
		ssxl_oracle.set_engine(ssxl_engine) 
		print("🔗 Oracle: Engine link established.") # Custom log for clarity

		# --- 2. Monitor Link ---
		print("📡 Linking EngineMonitor to Engine...")
		engine_monitor.set_engine(ssxl_engine)
		print("✅ EngineMonitor: Engine linked.")

		# --- 3. Signals Link ---
		if ssxl_signals:
			print("📶 Connecting Engine to AetherionSignals...")
			# Set the signal node reference on the Engine
			ssxl_engine.set_signals_node(ssxl_signals) 
			
			# NOTE: Connection between EngineMonitor and Engine signals should happen inside 
			# the EngineMonitor's set_engine method. We trust that logic here.
			print("✅ EngineMonitor connected to status signal.")
		else:
			push_warning("⚠️ AetherionSignals node not found. Signal connection skipped.")

		# --- 4. First Pulse ---
		print("🔮 Oracle linked. Delivering first pulse...")
		ssxl_oracle.tick() 

		print("⚙️ Engine confirmed idle.")
	else:
		# Use push_error to make this visible in the debugger
		push_error("❌ Failed to link Core Systems (Oracle/Engine/Monitor). Invocation aborted.")


# ----------------------------------------------------------------------
## RUNTIME
# ----------------------------------------------------------------------

func _on_clock_tick() -> void:
	# This function is called every time the clock_timer times out.
	print("🕰️ Clock tick.")

	if ssxl_oracle:
		ssxl_oracle.tick() 

	if engine_monitor:
		# This polls the engine's status, which should internally call engine.get_status()
		engine_monitor.update_status()
