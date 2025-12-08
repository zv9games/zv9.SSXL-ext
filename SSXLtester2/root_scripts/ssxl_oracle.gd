extends SSXLOracle # FIX: Extend the native GDExtension class name directly

# ============================================================================
# 1. ORACLE MONITORING SETTINGS
# ============================================================================

# How often (in seconds) to print status information for monitoring.
const MONITOR_INTERVAL: float = 0.5 
var time_elapsed: float = 0.0


# ============================================================================
# 2. LIFECYCLE AND MONITORING LOOP
# ============================================================================

func _process(delta: float):
	# The actual 'tick' that advances the engine state is called by the parent 
	# orchestrator script (main.gd). This script handles monitoring.
	
	time_elapsed += delta
	
	if time_elapsed >= MONITOR_INTERVAL:
		time_elapsed = 0.0
		
		# Check the Oracle's health and state using its exposed methods.
		# Now works because the script extends SSXLOracle.
		var status = get_status()
		var tick_count = get_tick()
		
		print("🧠 [Oracle Monitor] Status: %s | Tick: %d" % [status, tick_count])

# ============================================================================
# 3. UTILITY METHODS (Demonstrating API Access)
# ============================================================================

func perform_ping():
	# Now works because the script extends SSXLOracle.
	ping()
	print("🧠 [Oracle Monitor] Ping sent to native core.")

func force_reset():
	# Now works because the script extends SSXLOracle.
	reset()
	print("🧠 [Oracle Monitor] Oracle reset initiated.")
