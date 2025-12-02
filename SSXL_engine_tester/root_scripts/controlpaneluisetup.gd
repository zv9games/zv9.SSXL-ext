class_name control_panel_ui_setup
extends RefCounted

var controller: Control

func _init(p_controller: Control) -> void:
	controller = p_controller

func setup_all() -> void:
	setup_timers_properties()
	setup_ui_elements()
	connect_ui_signals()

func setup_timers_properties() -> void:
	if is_instance_valid(controller.engine_timer):
		controller.engine_timer.wait_time = 0.5
		controller.engine_timer.one_shot = false
		controller.engine_timer.autostart = false
	
	if is_instance_valid(controller.animation_timer):
		controller.animation_timer.wait_time = 0.5
		controller.animation_timer.one_shot = false
		controller.animation_timer.autostart = false
	
	if is_instance_valid(controller.redraw_throttle_timer):
		controller.redraw_throttle_timer.one_shot = true
		controller.redraw_throttle_timer.wait_time = 0.1

func link_ffi_nodes() -> void:
	var ssxl_engine: Node = controller.ssxl_engine
	var ssxl_signals: Node = controller.ssxl_signals
	var ssxl_oracle: Node = controller.ssxl_oracle
	var tilemap: TileMap = controller.ssxl_tilemap   # FIX: use Rust SSXLTileMap

	if is_instance_valid(ssxl_engine):
		if is_instance_valid(ssxl_signals) and ssxl_engine.has_method("set_signals_node"):
			ssxl_engine.set_signals_node(ssxl_signals)
			print("DEBUG: Engine → Signals linked")

		if is_instance_valid(tilemap) and ssxl_engine.has_method("set_tilemap"):
			ssxl_engine.set_tilemap(tilemap)
			print("DEBUG: Engine → SSXLTileMap linked")

		if is_instance_valid(ssxl_oracle) and is_instance_valid(ssxl_signals) and ssxl_oracle.has_method("set_signals_node"):
			ssxl_oracle.set_signals_node(ssxl_signals)
			print("DEBUG: Oracle → Signals linked")

func start_ffi_conductors() -> void:
	var ssxl_engine: Node = controller.ssxl_engine
	if is_instance_valid(ssxl_engine) and ssxl_engine.has_method("start_async_conductors"):
		ssxl_engine.start_async_conductors()
		print("DEBUG: Async conductors started")

func setup_ui_elements() -> void:
	var width_spinbox: SpinBox = controller.grid_width
	var height_spinbox: SpinBox = controller.grid_height
	
	if is_instance_valid(width_spinbox) and is_instance_valid(height_spinbox):
		width_spinbox.max_value = 1_000_000_000.0
		width_spinbox.step = float(controller.CHUNK_SIZE_X)
		width_spinbox.value = float(controller.CHUNK_SIZE_X * controller.DEFAULT_CHUNKS)
		
		height_spinbox.max_value = 1_000_000_000.0
		height_spinbox.step = float(controller.CHUNK_SIZE_Y)
		height_spinbox.value = float(controller.CHUNK_SIZE_Y * controller.DEFAULT_CHUNKS)

	var tile_selector: OptionButton = controller.tile_type_selector
	if is_instance_valid(tile_selector):
		tile_selector.clear()
		tile_selector.add_item("perlin_basic_2d", 0)
		tile_selector.add_item("cellular_automata_solid", 1)
		tile_selector.select(0)
	
	var placement_selector: OptionButton = controller.placement_mode_selector
	if is_instance_valid(placement_selector):
		placement_selector.clear()
		placement_selector.add_item("perlin_basic_2d", 0)
		placement_selector.add_item("cellular_automata_checkerboard", 1)
		placement_selector.add_item("drunkards_walk", 2)
		placement_selector.add_item("maze_recursive_division", 3)
		placement_selector.select(0)
		
	if is_instance_valid(controller.progress_bar):
		controller.progress_bar.visible = false
	
	if is_instance_valid(controller.status_label):
		controller.status_label.text = "IDLE: Ready for Ignition."

func connect_ui_signals() -> void:
	if is_instance_valid(controller.grid_width):
		controller.grid_width.value_changed.connect(self._on_grid_size_value_changed)
		
	if is_instance_valid(controller.grid_height):
		controller.grid_height.value_changed.connect(self._on_grid_size_value_changed)

	if is_instance_valid(controller.placement_mode_selector):
		controller.placement_mode_selector.item_selected.connect(self._on_mode_selector_item_selected)

	if is_instance_valid(controller.tile_type_selector):
		controller.tile_type_selector.item_selected.connect(self._on_tile_type_selector_item_selected)

func _on_grid_size_value_changed(value: float) -> void:
	if is_instance_valid(controller.ssxl_tilemap) and not controller.is_generating:
		controller.ssxl_tilemap.clear()
		print("DEBUG: Tilemap cleared on grid size change")

func _on_mode_selector_item_selected(index: int) -> void:
	var mode: int = controller.placement_mode_selector.get_item_id(index)
	print("DEBUG: Placement mode selected →", mode)

func _on_tile_type_selector_item_selected(index: int) -> void:
	var type_id: int = controller.tile_type_selector.get_item_id(index)
	print("DEBUG: Tile type selected →", type_id)
