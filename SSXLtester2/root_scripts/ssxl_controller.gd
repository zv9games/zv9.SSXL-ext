extends Node
class_name SSXLController

@onready var lbl_status: Label = $StatusLabel if has_node("StatusLabel") else null
@onready var lbl_progress: Label = $ProgressLabel if has_node("ProgressLabel") else null

var _last_progress_print := 0
var _progress_interval_ms := 200
var _progress_done := false

# Performance metrics
var _gen_start_time_ms := 0
var _gen_end_time_ms := 0
var _total_tiles := 0


func _ready():
	#print("SSXLController: ready.")
	pass


# Called by SSXLMain before generation starts
func setup_world_metrics(w: int, h: int) -> void:
	_total_tiles = w * h


# ------------------------------------------------------------
# Conductor lifecycle signals
# ------------------------------------------------------------
func _on_conductor_ready():
	print("SSXLController: Conductor ready.")
	if lbl_status:
		lbl_status.text = "Conductor ready"


func _on_generation_started(seed: int, generator_id: int):
	#print("SSXLController: Generation started.")
	_progress_done = false
	_gen_start_time_ms = Time.get_ticks_msec()

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

	if pct >= 100:
		_progress_done = true


# ------------------------------------------------------------
# CRITICAL FIX: Apply chunks as they are rendered
# ------------------------------------------------------------
func _on_chunk_rendered(cx: int, cy: int):
	#print("SSXLController: chunk rendered (%s, %s)" % [cx, cy])

	var renderer := get_node_or_null("../Node3D/SSXLRenderer")
	if renderer == null:
		push_error("SSXLController: ERROR — SSXLRenderer node not found at ../Node3D/SSXLRenderer")
		return

	renderer.apply_chunk(cx, cy)


func _on_generation_finished(result):
	_gen_end_time_ms = Time.get_ticks_msec()

	var duration_ms := _gen_end_time_ms - _gen_start_time_ms
	var duration_sec := duration_ms / 1000.0
	var tiles_per_sec := 0.0

	if duration_sec > 0.0:
		tiles_per_sec = float(_total_tiles) / duration_sec

	print("=== SSXL PERFORMANCE REPORT ===")
	print("Map Gen Start:   %s ms" % _gen_start_time_ms)
	print("Map Gen Finish:  %s ms" % _gen_end_time_ms)
	print("Total Time:      %s ms (%.3f sec)" % [duration_ms, duration_sec])
	print("Total Tiles:     %s" % _total_tiles)
	print("Tiles / Second:  %.2f" % tiles_per_sec)
	print("FPS (stabilizing...):")
	print("================================")

	# Wait TWO frames for accurate FPS
	call_deferred("_print_fps_after_two_frames")

	if lbl_status:
		lbl_status.text = "Generation complete"


func _print_fps_after_two_frames():
	# Wait for two full render frames
	await get_tree().process_frame
	await get_tree().process_frame

	var fps := Engine.get_frames_per_second()
	print("FPS (stabilized): %s" % fps)


func _on_generation_error(message: String):
#	push_error("SSXLController: ERROR: %s" % message)
	if lbl_status:
		lbl_status.text = "Error: %s" % message


# ------------------------------------------------------------
# Debug + custom SSXL events
# ------------------------------------------------------------
func _on_debug_event(msg: String):
	pass


func _on_ssxl_event(event_type: String, payload: Variant):
	pass
