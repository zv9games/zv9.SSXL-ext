extends Node2D

@onready var ssxl_oracle: Node = get_parent().get_node("SSXLOracle")
@onready var ssxl_engine: Node = get_parent().get_node("SSXLEngine")
@onready var ssxl_signals: Node = get_parent().get_node("SSXLSignals")

@onready var clock_timer: Timer = $/root/ssxltester/main/tilemap/clocktimer
@onready var engine_monitor: Node = $/root/ssxltester/main/EngineMonitor

func _ready() -> void:
	if is_instance_valid(clock_timer):
		clock_timer.timeout.connect(_on_clock_tick)
		clock_timer.start()
	else:
		push_error("❌ Clock timer not found. Main orchestration failed to start.")

func enter_idle_state() -> void:
	print("\n🎬 Main: Entering idle state. Systems standing by...")

	if is_instance_valid(ssxl_oracle) and is_instance_valid(ssxl_engine) and is_instance_valid(engine_monitor):
		
		print("🔗 Linking Oracle to Engine...")
		ssxl_oracle.set_engine(ssxl_engine) 
		print("🔗 Oracle: Engine link established.")

		print("📡 Linking EngineMonitor to Engine...")
		print("✅ EngineMonitor: Engine linked.")

		if is_instance_valid(ssxl_signals):
			print("📶 Connecting Engine to SSXLSignals...")
			ssxl_engine.set_signals_node(ssxl_signals) 
			
			print("✅ EngineMonitor connected to status signal.")
		else:
			push_warning("⚠️ SSXLSignals node not found. Signal emission connection skipped.")

		print("🔮 Oracle linked. Delivering first pulse...")
		ssxl_oracle.tick() 

		print("⚙️ Engine confirmed idle.")
	else:
		push_error("❌ Failed to link Core Systems (Oracle/Engine/Monitor). Invocation aborted.")


func _on_clock_tick() -> void:
	print("🕰️ Clock tick.")

	if is_instance_valid(ssxl_oracle):
		ssxl_oracle.tick() 

	if is_instance_valid(engine_monitor):
		pass
