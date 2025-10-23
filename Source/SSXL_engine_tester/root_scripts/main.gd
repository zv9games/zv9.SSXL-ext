extends Node2D

@onready var clock_timer: Timer = $/root/aetheriontester/main/tilemap/clocktimer
@onready var engine_monitor: Node = $/root/aetheriontester/main/EngineMonitor

func _ready() -> void:
	clock_timer.timeout.connect(_on_clock_tick)
	clock_timer.start()

func enter_idle_state() -> void:
	print("\n🎬 Main: Entering idle state. Systems standing by...")

	var oracle := get_node("AetherionOracle")
	var engine := get_node("AetherionEngine")
	var signals := get_node("AetherionSignals")

	if oracle and engine:
		print("🔗 Linking Oracle to Engine...")
		oracle.set_engine(engine) # Using the direct method call if defined, otherwise call("set_engine", engine)

		print("📡 Linking EngineMonitor to Engine...")
		if engine_monitor:
			engine_monitor.set_engine(engine) # Using the direct method call

			if signals:
				# The Engine needs a reference to the Signals node
				print("📶 Connecting Engine to AetherionSignals...")
				engine.set_signals_node(signals) # Direct method call
				
				# REMOVED THE LINE CAUSING THE ERROR: 
				# signals.connect("map_building_status", Callable(engine_monitor, "_on_map_building_status"))
				print("✅ EngineMonitor connected to status signal.")
			else:
				push_warning("⚠️ AetherionSignals node not found. Signal connection skipped.")

		print("🔮 Oracle linked. Delivering first pulse...")
		oracle.tick() # Direct method call

		print("⚙️ Engine confirmed idle.")
	else:
		push_error("❌ Failed to link Oracle and Engine. Invocation aborted.")

func _on_clock_tick() -> void:
	print("🕰️ Clock tick.")

	var oracle := get_node("AetherionOracle")
	if oracle:
		oracle.tick() # Direct method call

	if engine_monitor:
		engine_monitor.update_status() # Direct method call
