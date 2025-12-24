extends Node
class_name SSXLController

@onready var lbl_status: Label = $StatusLabel if has_node("StatusLabel") else null
@onready var lbl_progress: Label = $ProgressLabel if has_node("ProgressLabel") else null

var _last_progress_print := 0
var _progress_interval_ms := 200   # print progress at most every 0.2s
var _progress_done := false        # <-- prevents infinite 100% spam

func _ready():
	print("SSXLController: ready.")


# ------------------------------------------------------------
# Conductor lifecycle signals
# ------------------------------------------------------------
func _on_conductor_ready():
	print("SSXLController: Conductor ready.")
	if lbl_status:
		lbl_status.text = "Conductor ready"


func _on_generation_started(seed: int, generator_id: int):
	print("SSXLController: Generation started.")
	_progress_done = false  # <-- reset for next run
	if lbl_status:
		lbl_status.text = "Generation started"


func _on_generation_progress(done_chunks: int, total_chunks: int, metrics: Dictionary):
	if _progress_done:
		return

	var pct := int((float(done_chunks) / float(total_chunks)) * 100.0)

	# Throttle console spam
	var now := Time.get_ticks_msec()
	if now - _last_progress_print > _progress_interval_ms:
		print("SSXLController: Progress %s%%" % pct)
		_last_progress_print = now

	if lbl_progress:
		lbl_progress.text = "%s%%" % pct

	# Stop printing once we hit 100%
	if pct >= 100:
		_progress_done = true


func _on_chunk_rendered(cx: int, cy: int):
	# Comment out or keep minimal
	# print("Chunk rendered (%s, %s)" % [cx, cy])
	pass


func _on_generation_finished(result):
	print("SSXLController: Generation finished.")
	if lbl_status:
		lbl_status.text = "Generation complete"


func _on_generation_error(message: String):
	push_error("SSXLController: ERROR: %s" % message)
	if lbl_status:
		lbl_status.text = "Error: %s" % message


# ------------------------------------------------------------
# Debug + custom SSXL events
# ------------------------------------------------------------
func _on_debug_event(msg: String):
	# Uncomment if needed
	# print("SSXLController [debug]: %s" % msg)
	pass


func _on_ssxl_event(event_type: String, payload: Variant):
	# Uncomment if needed
	# print("SSXLController [event]: %s -> %s" % [event_type, payload])
	pass
