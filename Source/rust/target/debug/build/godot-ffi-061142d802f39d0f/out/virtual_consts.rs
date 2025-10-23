#![allow(non_snake_case, non_upper_case_globals, unused_imports)]
pub mod AesContext {
    pub use super::RefCounted::*;
    
}
pub mod AStar2D {
    pub use super::RefCounted::*;
    pub const filter_neighbor: (&'static str, u32) = ("_filter_neighbor", 2522259332u32);
    pub const estimate_cost: (&'static str, u32) = ("_estimate_cost", 3085491603u32);
    pub const compute_cost: (&'static str, u32) = ("_compute_cost", 3085491603u32);
    
}
pub mod AStar3D {
    pub use super::RefCounted::*;
    pub const filter_neighbor: (&'static str, u32) = ("_filter_neighbor", 2522259332u32);
    pub const estimate_cost: (&'static str, u32) = ("_estimate_cost", 3085491603u32);
    pub const compute_cost: (&'static str, u32) = ("_compute_cost", 3085491603u32);
    
}
pub mod AStarGrid2D {
    pub use super::RefCounted::*;
    pub const estimate_cost: (&'static str, u32) = ("_estimate_cost", 2153177966u32);
    pub const compute_cost: (&'static str, u32) = ("_compute_cost", 2153177966u32);
    
}
pub mod AcceptDialog {
    pub use super::Window::*;
    
}
pub mod AimModifier3D {
    pub use super::BoneConstraint3D::*;
    
}
pub mod AnimatableBody2D {
    pub use super::StaticBody2D::*;
    
}
pub mod AnimatableBody3D {
    pub use super::StaticBody3D::*;
    
}
pub mod AnimatedSprite2D {
    pub use super::Node2D::*;
    
}
pub mod AnimatedSprite3D {
    pub use super::SpriteBase3D::*;
    
}
pub mod AnimatedTexture {
    pub use super::Texture2D::*;
    
}
pub mod Animation {
    pub use super::Resource::*;
    
}
pub mod AnimationLibrary {
    pub use super::Resource::*;
    
}
pub mod AnimationMixer {
    pub use super::Node::*;
    pub const post_process_key_value: (&'static str, u32) = ("_post_process_key_value", 2716908335u32);
    
}
pub mod AnimationNode {
    pub use super::Resource::*;
    pub const get_child_nodes: (&'static str, u32) = ("_get_child_nodes", 3102165223u32);
    pub const get_parameter_list: (&'static str, u32) = ("_get_parameter_list", 3995934104u32);
    pub const get_child_by_name: (&'static str, u32) = ("_get_child_by_name", 625644256u32);
    pub const get_parameter_default_value: (&'static str, u32) = ("_get_parameter_default_value", 2760726917u32);
    pub const is_parameter_read_only: (&'static str, u32) = ("_is_parameter_read_only", 2619796661u32);
    pub const process: (&'static str, u32) = ("_process", 2139827523u32);
    pub const get_caption: (&'static str, u32) = ("_get_caption", 201670096u32);
    pub const has_filter: (&'static str, u32) = ("_has_filter", 36873697u32);
    
}
pub mod AnimationNodeAdd2 {
    pub use super::AnimationNodeSync::*;
    
}
pub mod AnimationNodeAdd3 {
    pub use super::AnimationNodeSync::*;
    
}
pub mod AnimationNodeAnimation {
    pub use super::AnimationRootNode::*;
    
}
pub mod AnimationNodeBlend2 {
    pub use super::AnimationNodeSync::*;
    
}
pub mod AnimationNodeBlend3 {
    pub use super::AnimationNodeSync::*;
    
}
pub mod AnimationNodeBlendSpace1D {
    pub use super::AnimationRootNode::*;
    
}
pub mod AnimationNodeBlendSpace2D {
    pub use super::AnimationRootNode::*;
    
}
pub mod AnimationNodeBlendTree {
    pub use super::AnimationRootNode::*;
    
}
pub mod AnimationNodeExtension {
    pub use super::AnimationNode::*;
    pub const process_animation_node: (&'static str, u32) = ("_process_animation_node", 912931771u32);
    
}
pub mod AnimationNodeOneShot {
    pub use super::AnimationNodeSync::*;
    
}
pub mod AnimationNodeOutput {
    pub use super::AnimationNode::*;
    
}
pub mod AnimationNodeStateMachine {
    pub use super::AnimationRootNode::*;
    
}
pub mod AnimationNodeStateMachinePlayback {
    pub use super::Resource::*;
    
}
pub mod AnimationNodeStateMachineTransition {
    pub use super::Resource::*;
    
}
pub mod AnimationNodeSub2 {
    pub use super::AnimationNodeSync::*;
    
}
pub mod AnimationNodeSync {
    pub use super::AnimationNode::*;
    
}
pub mod AnimationNodeTimeScale {
    pub use super::AnimationNode::*;
    
}
pub mod AnimationNodeTimeSeek {
    pub use super::AnimationNode::*;
    
}
pub mod AnimationNodeTransition {
    pub use super::AnimationNodeSync::*;
    
}
pub mod AnimationPlayer {
    pub use super::AnimationMixer::*;
    
}
pub mod AnimationRootNode {
    pub use super::AnimationNode::*;
    
}
pub mod AnimationTree {
    pub use super::AnimationMixer::*;
    
}
pub mod Area2D {
    pub use super::CollisionObject2D::*;
    
}
pub mod Area3D {
    pub use super::CollisionObject3D::*;
    
}
pub mod ArrayMesh {
    pub use super::Mesh::*;
    
}
pub mod ArrayOccluder3D {
    pub use super::Occluder3D::*;
    
}
pub mod AspectRatioContainer {
    pub use super::Container::*;
    
}
pub mod AtlasTexture {
    pub use super::Texture2D::*;
    
}
pub mod AudioBusLayout {
    pub use super::Resource::*;
    
}
pub mod AudioEffect {
    pub use super::Resource::*;
    pub const instantiate: (&'static str, u32) = ("_instantiate", 1659796816u32);
    
}
pub mod AudioEffectAmplify {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectBandLimitFilter {
    pub use super::AudioEffectFilter::*;
    
}
pub mod AudioEffectBandPassFilter {
    pub use super::AudioEffectFilter::*;
    
}
pub mod AudioEffectCapture {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectChorus {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectCompressor {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectDelay {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectDistortion {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectEq {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectEq10 {
    pub use super::AudioEffectEq::*;
    
}
pub mod AudioEffectEq21 {
    pub use super::AudioEffectEq::*;
    
}
pub mod AudioEffectEq6 {
    pub use super::AudioEffectEq::*;
    
}
pub mod AudioEffectFilter {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectHardLimiter {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectHighPassFilter {
    pub use super::AudioEffectFilter::*;
    
}
pub mod AudioEffectHighShelfFilter {
    pub use super::AudioEffectFilter::*;
    
}
pub mod AudioEffectInstance {
    pub use super::RefCounted::*;
    pub const process_rawptr: (&'static str, u32) = ("_process", 1649997291u32);
    pub const process_silence: (&'static str, u32) = ("_process_silence", 36873697u32);
    
}
pub mod AudioEffectLimiter {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectLowPassFilter {
    pub use super::AudioEffectFilter::*;
    
}
pub mod AudioEffectLowShelfFilter {
    pub use super::AudioEffectFilter::*;
    
}
pub mod AudioEffectNotchFilter {
    pub use super::AudioEffectFilter::*;
    
}
pub mod AudioEffectPanner {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectPhaser {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectPitchShift {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectRecord {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectReverb {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectSpectrumAnalyzer {
    pub use super::AudioEffect::*;
    
}
pub mod AudioEffectSpectrumAnalyzerInstance {
    pub use super::AudioEffectInstance::*;
    
}
pub mod AudioEffectStereoEnhance {
    pub use super::AudioEffect::*;
    
}
pub mod AudioListener2D {
    pub use super::Node2D::*;
    
}
pub mod AudioListener3D {
    pub use super::Node3D::*;
    
}
pub mod AudioServer {
    pub use super::Object::*;
    
}
pub mod AudioStream {
    pub use super::Resource::*;
    pub const instantiate_playback: (&'static str, u32) = ("_instantiate_playback", 3093715447u32);
    pub const get_stream_name: (&'static str, u32) = ("_get_stream_name", 201670096u32);
    pub const get_length: (&'static str, u32) = ("_get_length", 1740695150u32);
    pub const is_monophonic: (&'static str, u32) = ("_is_monophonic", 36873697u32);
    pub const get_bpm: (&'static str, u32) = ("_get_bpm", 1740695150u32);
    pub const get_beat_count: (&'static str, u32) = ("_get_beat_count", 3905245786u32);
    pub const get_tags: (&'static str, u32) = ("_get_tags", 3102165223u32);
    pub const get_parameter_list: (&'static str, u32) = ("_get_parameter_list", 3995934104u32);
    pub const has_loop: (&'static str, u32) = ("_has_loop", 36873697u32);
    pub const get_bar_beats: (&'static str, u32) = ("_get_bar_beats", 3905245786u32);
    
}
pub mod AudioStreamGenerator {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamGeneratorPlayback {
    pub use super::AudioStreamPlaybackResampled::*;
    
}
pub mod AudioStreamInteractive {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamMp3 {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamMicrophone {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamOggVorbis {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamPlayback {
    pub use super::RefCounted::*;
    pub const start: (&'static str, u32) = ("_start", 373806689u32);
    pub const stop: (&'static str, u32) = ("_stop", 3218959716u32);
    pub const is_playing: (&'static str, u32) = ("_is_playing", 36873697u32);
    pub const get_loop_count: (&'static str, u32) = ("_get_loop_count", 3905245786u32);
    pub const get_playback_position: (&'static str, u32) = ("_get_playback_position", 1740695150u32);
    pub const seek: (&'static str, u32) = ("_seek", 373806689u32);
    pub const mix_rawptr: (&'static str, u32) = ("_mix", 925936155u32);
    pub const tag_used_streams: (&'static str, u32) = ("_tag_used_streams", 3218959716u32);
    pub const set_parameter: (&'static str, u32) = ("_set_parameter", 3776071444u32);
    pub const get_parameter: (&'static str, u32) = ("_get_parameter", 2760726917u32);
    
}
pub mod AudioStreamPlaybackInteractive {
    pub use super::AudioStreamPlayback::*;
    
}
pub mod AudioStreamPlaybackOggVorbis {
    pub use super::AudioStreamPlaybackResampled::*;
    
}
pub mod AudioStreamPlaybackPlaylist {
    pub use super::AudioStreamPlayback::*;
    
}
pub mod AudioStreamPlaybackPolyphonic {
    pub use super::AudioStreamPlayback::*;
    
}
pub mod AudioStreamPlaybackResampled {
    pub use super::AudioStreamPlayback::*;
    pub const mix_resampled_rawptr: (&'static str, u32) = ("_mix_resampled", 50157827u32);
    pub const get_stream_sampling_rate: (&'static str, u32) = ("_get_stream_sampling_rate", 1740695150u32);
    
}
pub mod AudioStreamPlaybackSynchronized {
    pub use super::AudioStreamPlayback::*;
    
}
pub mod AudioStreamPlayer {
    pub use super::Node::*;
    
}
pub mod AudioStreamPlayer2D {
    pub use super::Node2D::*;
    
}
pub mod AudioStreamPlayer3D {
    pub use super::Node3D::*;
    
}
pub mod AudioStreamPlaylist {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamPolyphonic {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamRandomizer {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamSynchronized {
    pub use super::AudioStream::*;
    
}
pub mod AudioStreamWav {
    pub use super::AudioStream::*;
    
}
pub mod BackBufferCopy {
    pub use super::Node2D::*;
    
}
pub mod BaseButton {
    pub use super::Control::*;
    pub const pressed: (&'static str, u32) = ("_pressed", 3218959716u32);
    pub const toggled: (&'static str, u32) = ("_toggled", 2586408642u32);
    
}
pub mod BaseMaterial3D {
    pub use super::Material::*;
    
}
pub mod BitMap {
    pub use super::Resource::*;
    
}
pub mod Bone2D {
    pub use super::Node2D::*;
    
}
pub mod BoneAttachment3D {
    pub use super::Node3D::*;
    
}
pub mod BoneConstraint3D {
    pub use super::SkeletonModifier3D::*;
    
}
pub mod BoneMap {
    pub use super::Resource::*;
    
}
pub mod BoxContainer {
    pub use super::Container::*;
    
}
pub mod BoxMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod BoxOccluder3D {
    pub use super::Occluder3D::*;
    
}
pub mod BoxShape3D {
    pub use super::Shape3D::*;
    
}
pub mod Button {
    pub use super::BaseButton::*;
    
}
pub mod ButtonGroup {
    pub use super::Resource::*;
    
}
pub mod CpuParticles2D {
    pub use super::Node2D::*;
    
}
pub mod CpuParticles3D {
    pub use super::GeometryInstance3D::*;
    
}
pub mod CsgBox3D {
    pub use super::CsgPrimitive3D::*;
    
}
pub mod CsgCombiner3D {
    pub use super::CsgShape3D::*;
    
}
pub mod CsgCylinder3D {
    pub use super::CsgPrimitive3D::*;
    
}
pub mod CsgMesh3D {
    pub use super::CsgPrimitive3D::*;
    
}
pub mod CsgPolygon3D {
    pub use super::CsgPrimitive3D::*;
    
}
pub mod CsgPrimitive3D {
    pub use super::CsgShape3D::*;
    
}
pub mod CsgShape3D {
    pub use super::GeometryInstance3D::*;
    
}
pub mod CsgSphere3D {
    pub use super::CsgPrimitive3D::*;
    
}
pub mod CsgTorus3D {
    pub use super::CsgPrimitive3D::*;
    
}
pub mod CallbackTweener {
    pub use super::Tweener::*;
    
}
pub mod Camera2D {
    pub use super::Node2D::*;
    
}
pub mod Camera3D {
    pub use super::Node3D::*;
    
}
pub mod CameraAttributes {
    pub use super::Resource::*;
    
}
pub mod CameraAttributesPhysical {
    pub use super::CameraAttributes::*;
    
}
pub mod CameraAttributesPractical {
    pub use super::CameraAttributes::*;
    
}
pub mod CameraFeed {
    pub use super::RefCounted::*;
    pub const activate_feed: (&'static str, u32) = ("_activate_feed", 2240911060u32);
    pub const deactivate_feed: (&'static str, u32) = ("_deactivate_feed", 3218959716u32);
    
}
pub mod CameraServer {
    pub use super::Object::*;
    
}
pub mod CameraTexture {
    pub use super::Texture2D::*;
    
}
pub mod CanvasGroup {
    pub use super::Node2D::*;
    
}
pub mod CanvasItem {
    pub use super::Node::*;
    pub const draw: (&'static str, u32) = ("_draw", 3218959716u32);
    
}
pub mod CanvasItemMaterial {
    pub use super::Material::*;
    
}
pub mod CanvasLayer {
    pub use super::Node::*;
    
}
pub mod CanvasModulate {
    pub use super::Node2D::*;
    
}
pub mod CanvasTexture {
    pub use super::Texture2D::*;
    
}
pub mod CapsuleMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod CapsuleShape2D {
    pub use super::Shape2D::*;
    
}
pub mod CapsuleShape3D {
    pub use super::Shape3D::*;
    
}
pub mod CenterContainer {
    pub use super::Container::*;
    
}
pub mod CharFxTransform {
    pub use super::RefCounted::*;
    
}
pub mod CharacterBody2D {
    pub use super::PhysicsBody2D::*;
    
}
pub mod CharacterBody3D {
    pub use super::PhysicsBody3D::*;
    
}
pub mod CheckBox {
    pub use super::Button::*;
    
}
pub mod CheckButton {
    pub use super::Button::*;
    
}
pub mod CircleShape2D {
    pub use super::Shape2D::*;
    
}
pub mod ClassDb {
    pub use super::Object::*;
    
}
pub mod CodeEdit {
    pub use super::TextEdit::*;
    pub const confirm_code_completion: (&'static str, u32) = ("_confirm_code_completion", 2586408642u32);
    pub const request_code_completion: (&'static str, u32) = ("_request_code_completion", 2586408642u32);
    pub const filter_code_completion_candidates: (&'static str, u32) = ("_filter_code_completion_candidates", 2560709669u32);
    
}
pub mod CodeHighlighter {
    pub use super::SyntaxHighlighter::*;
    
}
pub mod CollisionObject2D {
    pub use super::Node2D::*;
    pub const input_event: (&'static str, u32) = ("_input_event", 1847696837u32);
    pub const mouse_enter: (&'static str, u32) = ("_mouse_enter", 3218959716u32);
    pub const mouse_exit: (&'static str, u32) = ("_mouse_exit", 3218959716u32);
    pub const mouse_shape_enter: (&'static str, u32) = ("_mouse_shape_enter", 1286410249u32);
    pub const mouse_shape_exit: (&'static str, u32) = ("_mouse_shape_exit", 1286410249u32);
    
}
pub mod CollisionObject3D {
    pub use super::Node3D::*;
    pub const input_event: (&'static str, u32) = ("_input_event", 2310605070u32);
    pub const mouse_enter: (&'static str, u32) = ("_mouse_enter", 3218959716u32);
    pub const mouse_exit: (&'static str, u32) = ("_mouse_exit", 3218959716u32);
    
}
pub mod CollisionPolygon2D {
    pub use super::Node2D::*;
    
}
pub mod CollisionPolygon3D {
    pub use super::Node3D::*;
    
}
pub mod CollisionShape2D {
    pub use super::Node2D::*;
    
}
pub mod CollisionShape3D {
    pub use super::Node3D::*;
    
}
pub mod ColorPalette {
    pub use super::Resource::*;
    
}
pub mod ColorPicker {
    pub use super::VBoxContainer::*;
    
}
pub mod ColorPickerButton {
    pub use super::Button::*;
    
}
pub mod ColorRect {
    pub use super::Control::*;
    
}
pub mod CompressedCubemap {
    pub use super::CompressedTextureLayered::*;
    
}
pub mod CompressedCubemapArray {
    pub use super::CompressedTextureLayered::*;
    
}
pub mod CompressedTexture2D {
    pub use super::Texture2D::*;
    
}
pub mod CompressedTexture2DArray {
    pub use super::CompressedTextureLayered::*;
    
}
pub mod CompressedTexture3D {
    pub use super::Texture3D::*;
    
}
pub mod CompressedTextureLayered {
    pub use super::TextureLayered::*;
    
}
pub mod ConcavePolygonShape2D {
    pub use super::Shape2D::*;
    
}
pub mod ConcavePolygonShape3D {
    pub use super::Shape3D::*;
    
}
pub mod ConeTwistJoint3D {
    pub use super::Joint3D::*;
    
}
pub mod ConfigFile {
    pub use super::RefCounted::*;
    
}
pub mod ConfirmationDialog {
    pub use super::AcceptDialog::*;
    
}
pub mod Container {
    pub use super::Control::*;
    pub const get_allowed_size_flags_horizontal: (&'static str, u32) = ("_get_allowed_size_flags_horizontal", 1930428628u32);
    pub const get_allowed_size_flags_vertical: (&'static str, u32) = ("_get_allowed_size_flags_vertical", 1930428628u32);
    
}
pub mod Control {
    pub use super::CanvasItem::*;
    pub const has_point: (&'static str, u32) = ("_has_point", 556197845u32);
    pub const structured_text_parser: (&'static str, u32) = ("_structured_text_parser", 1292548940u32);
    pub const get_minimum_size: (&'static str, u32) = ("_get_minimum_size", 3341600327u32);
    pub const get_tooltip: (&'static str, u32) = ("_get_tooltip", 3674420000u32);
    pub const get_drag_data: (&'static str, u32) = ("_get_drag_data", 2233896889u32);
    pub const can_drop_data: (&'static str, u32) = ("_can_drop_data", 2603004011u32);
    pub const drop_data: (&'static str, u32) = ("_drop_data", 3699746064u32);
    pub const make_custom_tooltip: (&'static str, u32) = ("_make_custom_tooltip", 1976279298u32);
    pub const accessibility_get_contextual_info: (&'static str, u32) = ("_accessibility_get_contextual_info", 201670096u32);
    pub const get_accessibility_container_name: (&'static str, u32) = ("_get_accessibility_container_name", 2174079723u32);
    pub const gui_input: (&'static str, u32) = ("_gui_input", 3754044979u32);
    
}
pub mod ConvertTransformModifier3D {
    pub use super::BoneConstraint3D::*;
    
}
pub mod ConvexPolygonShape2D {
    pub use super::Shape2D::*;
    
}
pub mod ConvexPolygonShape3D {
    pub use super::Shape3D::*;
    
}
pub mod CopyTransformModifier3D {
    pub use super::BoneConstraint3D::*;
    
}
pub mod Crypto {
    pub use super::RefCounted::*;
    
}
pub mod CryptoKey {
    pub use super::Resource::*;
    
}
pub mod Cubemap {
    pub use super::ImageTextureLayered::*;
    
}
pub mod CubemapArray {
    pub use super::ImageTextureLayered::*;
    
}
pub mod Curve {
    pub use super::Resource::*;
    
}
pub mod Curve2D {
    pub use super::Resource::*;
    
}
pub mod Curve3D {
    pub use super::Resource::*;
    
}
pub mod CurveTexture {
    pub use super::Texture2D::*;
    
}
pub mod CurveXyzTexture {
    pub use super::Texture2D::*;
    
}
pub mod CylinderMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod CylinderShape3D {
    pub use super::Shape3D::*;
    
}
pub mod DpiTexture {
    pub use super::Texture2D::*;
    
}
pub mod DtlsServer {
    pub use super::RefCounted::*;
    
}
pub mod DampedSpringJoint2D {
    pub use super::Joint2D::*;
    
}
pub mod Decal {
    pub use super::VisualInstance3D::*;
    
}
pub mod DirAccess {
    pub use super::RefCounted::*;
    
}
pub mod DirectionalLight2D {
    pub use super::Light2D::*;
    
}
pub mod DirectionalLight3D {
    pub use super::Light3D::*;
    
}
pub mod DisplayServer {
    pub use super::Object::*;
    
}
pub mod ENetConnection {
    pub use super::RefCounted::*;
    
}
pub mod ENetMultiplayerPeer {
    pub use super::MultiplayerPeer::*;
    
}
pub mod ENetPacketPeer {
    pub use super::PacketPeer::*;
    
}
pub mod EditorCommandPalette {
    pub use super::ConfirmationDialog::*;
    
}
pub mod EditorContextMenuPlugin {
    pub use super::RefCounted::*;
    pub const popup_menu: (&'static str, u32) = ("_popup_menu", 4015028928u32);
    
}
pub mod EditorDebuggerPlugin {
    pub use super::RefCounted::*;
    pub const setup_session: (&'static str, u32) = ("_setup_session", 1286410249u32);
    pub const has_capture: (&'static str, u32) = ("_has_capture", 3927539163u32);
    pub const capture: (&'static str, u32) = ("_capture", 2607901833u32);
    pub const goto_script_line: (&'static str, u32) = ("_goto_script_line", 1208513123u32);
    pub const breakpoints_cleared_in_tree: (&'static str, u32) = ("_breakpoints_cleared_in_tree", 3218959716u32);
    pub const breakpoint_set_in_tree: (&'static str, u32) = ("_breakpoint_set_in_tree", 2338735218u32);
    
}
pub mod EditorDebuggerSession {
    pub use super::RefCounted::*;
    
}
pub mod EditorExportPlatform {
    pub use super::RefCounted::*;
    
}
pub mod EditorExportPlatformAndroid {
    pub use super::EditorExportPlatform::*;
    
}
pub mod EditorExportPlatformAppleEmbedded {
    pub use super::EditorExportPlatform::*;
    
}
pub mod EditorExportPlatformExtension {
    pub use super::EditorExportPlatform::*;
    pub const get_preset_features: (&'static str, u32) = ("_get_preset_features", 1387456631u32);
    pub const is_executable: (&'static str, u32) = ("_is_executable", 3927539163u32);
    pub const get_export_options: (&'static str, u32) = ("_get_export_options", 3995934104u32);
    pub const should_update_export_options: (&'static str, u32) = ("_should_update_export_options", 2240911060u32);
    pub const get_export_option_visibility: (&'static str, u32) = ("_get_export_option_visibility", 969350244u32);
    pub const get_export_option_warning: (&'static str, u32) = ("_get_export_option_warning", 805886795u32);
    pub const get_os_name: (&'static str, u32) = ("_get_os_name", 201670096u32);
    pub const get_name: (&'static str, u32) = ("_get_name", 201670096u32);
    pub const get_logo: (&'static str, u32) = ("_get_logo", 3635182373u32);
    pub const poll_export: (&'static str, u32) = ("_poll_export", 2240911060u32);
    pub const get_options_count: (&'static str, u32) = ("_get_options_count", 3905245786u32);
    pub const get_options_tooltip: (&'static str, u32) = ("_get_options_tooltip", 201670096u32);
    pub const get_option_icon: (&'static str, u32) = ("_get_option_icon", 3536238170u32);
    pub const get_option_label: (&'static str, u32) = ("_get_option_label", 844755477u32);
    pub const get_option_tooltip: (&'static str, u32) = ("_get_option_tooltip", 844755477u32);
    pub const get_device_architecture: (&'static str, u32) = ("_get_device_architecture", 844755477u32);
    pub const cleanup: (&'static str, u32) = ("_cleanup", 3218959716u32);
    pub const run: (&'static str, u32) = ("_run", 1726914928u32);
    pub const get_run_icon: (&'static str, u32) = ("_get_run_icon", 3635182373u32);
    pub const can_export: (&'static str, u32) = ("_can_export", 493961987u32);
    pub const has_valid_export_configuration: (&'static str, u32) = ("_has_valid_export_configuration", 493961987u32);
    pub const has_valid_project_configuration: (&'static str, u32) = ("_has_valid_project_configuration", 3117166915u32);
    pub const get_binary_extensions: (&'static str, u32) = ("_get_binary_extensions", 1387456631u32);
    pub const export_project: (&'static str, u32) = ("_export_project", 1328957260u32);
    pub const export_pack: (&'static str, u32) = ("_export_pack", 1328957260u32);
    pub const export_zip: (&'static str, u32) = ("_export_zip", 1328957260u32);
    pub const export_pack_patch: (&'static str, u32) = ("_export_pack_patch", 454765315u32);
    pub const export_zip_patch: (&'static str, u32) = ("_export_zip_patch", 454765315u32);
    pub const get_platform_features: (&'static str, u32) = ("_get_platform_features", 1139954409u32);
    pub const get_debug_protocol: (&'static str, u32) = ("_get_debug_protocol", 201670096u32);
    
}
pub mod EditorExportPlatformIos {
    pub use super::EditorExportPlatformAppleEmbedded::*;
    
}
pub mod EditorExportPlatformLinuxBsd {
    pub use super::EditorExportPlatformPc::*;
    
}
pub mod EditorExportPlatformMacOs {
    pub use super::EditorExportPlatform::*;
    
}
pub mod EditorExportPlatformPc {
    pub use super::EditorExportPlatform::*;
    
}
pub mod EditorExportPlatformVisionOs {
    pub use super::EditorExportPlatformAppleEmbedded::*;
    
}
pub mod EditorExportPlatformWeb {
    pub use super::EditorExportPlatform::*;
    
}
pub mod EditorExportPlatformWindows {
    pub use super::EditorExportPlatformPc::*;
    
}
pub mod EditorExportPlugin {
    pub use super::RefCounted::*;
    pub const export_file: (&'static str, u32) = ("_export_file", 3533781844u32);
    pub const export_begin: (&'static str, u32) = ("_export_begin", 2765511433u32);
    pub const export_end: (&'static str, u32) = ("_export_end", 3218959716u32);
    pub const begin_customize_resources: (&'static str, u32) = ("_begin_customize_resources", 1312023292u32);
    pub const customize_resource: (&'static str, u32) = ("_customize_resource", 307917495u32);
    pub const begin_customize_scenes: (&'static str, u32) = ("_begin_customize_scenes", 1312023292u32);
    pub const customize_scene: (&'static str, u32) = ("_customize_scene", 498701822u32);
    pub const get_customization_configuration_hash: (&'static str, u32) = ("_get_customization_configuration_hash", 3905245786u32);
    pub const end_customize_scenes: (&'static str, u32) = ("_end_customize_scenes", 3218959716u32);
    pub const end_customize_resources: (&'static str, u32) = ("_end_customize_resources", 3218959716u32);
    pub const get_export_options: (&'static str, u32) = ("_get_export_options", 488349689u32);
    pub const get_export_options_overrides: (&'static str, u32) = ("_get_export_options_overrides", 2837326714u32);
    pub const should_update_export_options: (&'static str, u32) = ("_should_update_export_options", 1866233299u32);
    pub const get_export_option_visibility: (&'static str, u32) = ("_get_export_option_visibility", 3537301980u32);
    pub const get_export_option_warning: (&'static str, u32) = ("_get_export_option_warning", 3340251247u32);
    pub const get_export_features: (&'static str, u32) = ("_get_export_features", 1057664154u32);
    pub const get_name: (&'static str, u32) = ("_get_name", 201670096u32);
    pub const supports_platform: (&'static str, u32) = ("_supports_platform", 1866233299u32);
    pub const get_android_dependencies: (&'static str, u32) = ("_get_android_dependencies", 1057664154u32);
    pub const get_android_dependencies_maven_repos: (&'static str, u32) = ("_get_android_dependencies_maven_repos", 1057664154u32);
    pub const get_android_libraries: (&'static str, u32) = ("_get_android_libraries", 1057664154u32);
    pub const get_android_manifest_activity_element_contents: (&'static str, u32) = ("_get_android_manifest_activity_element_contents", 4013372917u32);
    pub const get_android_manifest_application_element_contents: (&'static str, u32) = ("_get_android_manifest_application_element_contents", 4013372917u32);
    pub const get_android_manifest_element_contents: (&'static str, u32) = ("_get_android_manifest_element_contents", 4013372917u32);
    pub const update_android_prebuilt_manifest: (&'static str, u32) = ("_update_android_prebuilt_manifest", 3304965187u32);
    
}
pub mod EditorExportPreset {
    pub use super::RefCounted::*;
    
}
pub mod EditorFeatureProfile {
    pub use super::RefCounted::*;
    
}
pub mod EditorFileDialog {
    pub use super::ConfirmationDialog::*;
    
}
pub mod EditorFileSystem {
    pub use super::Node::*;
    
}
pub mod EditorFileSystemDirectory {
    pub use super::Object::*;
    
}
pub mod EditorFileSystemImportFormatSupportQuery {
    pub use super::RefCounted::*;
    pub const is_active: (&'static str, u32) = ("_is_active", 36873697u32);
    pub const get_file_extensions: (&'static str, u32) = ("_get_file_extensions", 1139954409u32);
    pub const query: (&'static str, u32) = ("_query", 36873697u32);
    
}
pub mod EditorImportPlugin {
    pub use super::ResourceImporter::*;
    pub const get_importer_name: (&'static str, u32) = ("_get_importer_name", 201670096u32);
    pub const get_visible_name: (&'static str, u32) = ("_get_visible_name", 201670096u32);
    pub const get_preset_count: (&'static str, u32) = ("_get_preset_count", 3905245786u32);
    pub const get_preset_name: (&'static str, u32) = ("_get_preset_name", 844755477u32);
    pub const get_recognized_extensions: (&'static str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub const get_import_options: (&'static str, u32) = ("_get_import_options", 520498173u32);
    pub const get_save_extension: (&'static str, u32) = ("_get_save_extension", 201670096u32);
    pub const get_resource_type: (&'static str, u32) = ("_get_resource_type", 201670096u32);
    pub const get_priority: (&'static str, u32) = ("_get_priority", 1740695150u32);
    pub const get_import_order: (&'static str, u32) = ("_get_import_order", 3905245786u32);
    pub const get_format_version: (&'static str, u32) = ("_get_format_version", 3905245786u32);
    pub const get_option_visibility: (&'static str, u32) = ("_get_option_visibility", 240466755u32);
    pub const import: (&'static str, u32) = ("_import", 4108152118u32);
    pub const can_import_threaded: (&'static str, u32) = ("_can_import_threaded", 36873697u32);
    
}
pub mod EditorInspector {
    pub use super::ScrollContainer::*;
    
}
pub mod EditorInspectorPlugin {
    pub use super::RefCounted::*;
    pub const can_handle: (&'static str, u32) = ("_can_handle", 397768994u32);
    pub const parse_begin: (&'static str, u32) = ("_parse_begin", 3975164845u32);
    pub const parse_category: (&'static str, u32) = ("_parse_category", 357144787u32);
    pub const parse_group: (&'static str, u32) = ("_parse_group", 357144787u32);
    pub const parse_property: (&'static str, u32) = ("_parse_property", 1087679910u32);
    pub const parse_end: (&'static str, u32) = ("_parse_end", 3975164845u32);
    
}
pub mod EditorInterface {
    pub use super::Object::*;
    
}
pub mod EditorNode3DGizmo {
    pub use super::Node3DGizmo::*;
    pub const redraw: (&'static str, u32) = ("_redraw", 3218959716u32);
    pub const get_handle_name: (&'static str, u32) = ("_get_handle_name", 1868713439u32);
    pub const is_handle_highlighted: (&'static str, u32) = ("_is_handle_highlighted", 361316320u32);
    pub const get_handle_value: (&'static str, u32) = ("_get_handle_value", 2144196525u32);
    pub const begin_handle_action: (&'static str, u32) = ("_begin_handle_action", 300928843u32);
    pub const set_handle: (&'static str, u32) = ("_set_handle", 2210262157u32);
    pub const commit_handle: (&'static str, u32) = ("_commit_handle", 3655739840u32);
    pub const subgizmos_intersect_ray: (&'static str, u32) = ("_subgizmos_intersect_ray", 2055005479u32);
    pub const subgizmos_intersect_frustum: (&'static str, u32) = ("_subgizmos_intersect_frustum", 1653813165u32);
    pub const set_subgizmo_transform: (&'static str, u32) = ("_set_subgizmo_transform", 3616898986u32);
    pub const get_subgizmo_transform: (&'static str, u32) = ("_get_subgizmo_transform", 1965739696u32);
    pub const commit_subgizmos: (&'static str, u32) = ("_commit_subgizmos", 3411059856u32);
    
}
pub mod EditorNode3DGizmoPlugin {
    pub use super::Resource::*;
    pub const has_gizmo: (&'static str, u32) = ("_has_gizmo", 1905827158u32);
    pub const create_gizmo: (&'static str, u32) = ("_create_gizmo", 1418965287u32);
    pub const get_gizmo_name: (&'static str, u32) = ("_get_gizmo_name", 201670096u32);
    pub const get_priority: (&'static str, u32) = ("_get_priority", 3905245786u32);
    pub const can_be_hidden: (&'static str, u32) = ("_can_be_hidden", 36873697u32);
    pub const is_selectable_when_hidden: (&'static str, u32) = ("_is_selectable_when_hidden", 36873697u32);
    pub const redraw: (&'static str, u32) = ("_redraw", 173330131u32);
    pub const get_handle_name: (&'static str, u32) = ("_get_handle_name", 3888674840u32);
    pub const is_handle_highlighted: (&'static str, u32) = ("_is_handle_highlighted", 2665780718u32);
    pub const get_handle_value: (&'static str, u32) = ("_get_handle_value", 2887724832u32);
    pub const begin_handle_action: (&'static str, u32) = ("_begin_handle_action", 3363704593u32);
    pub const set_handle: (&'static str, u32) = ("_set_handle", 1249646868u32);
    pub const commit_handle: (&'static str, u32) = ("_commit_handle", 1939863962u32);
    pub const subgizmos_intersect_ray: (&'static str, u32) = ("_subgizmos_intersect_ray", 1781916302u32);
    pub const subgizmos_intersect_frustum: (&'static str, u32) = ("_subgizmos_intersect_frustum", 3514748524u32);
    pub const get_subgizmo_transform: (&'static str, u32) = ("_get_subgizmo_transform", 3700343508u32);
    pub const set_subgizmo_transform: (&'static str, u32) = ("_set_subgizmo_transform", 2435388792u32);
    pub const commit_subgizmos: (&'static str, u32) = ("_commit_subgizmos", 2282018236u32);
    
}
pub mod EditorPaths {
    pub use super::Object::*;
    
}
pub mod EditorPlugin {
    pub use super::Node::*;
    pub const forward_canvas_gui_input: (&'static str, u32) = ("_forward_canvas_gui_input", 1062211774u32);
    pub const forward_canvas_draw_over_viewport: (&'static str, u32) = ("_forward_canvas_draw_over_viewport", 1496901182u32);
    pub const forward_canvas_force_draw_over_viewport: (&'static str, u32) = ("_forward_canvas_force_draw_over_viewport", 1496901182u32);
    pub const forward_3d_gui_input: (&'static str, u32) = ("_forward_3d_gui_input", 1018736637u32);
    pub const forward_3d_draw_over_viewport: (&'static str, u32) = ("_forward_3d_draw_over_viewport", 1496901182u32);
    pub const forward_3d_force_draw_over_viewport: (&'static str, u32) = ("_forward_3d_force_draw_over_viewport", 1496901182u32);
    pub const get_plugin_name: (&'static str, u32) = ("_get_plugin_name", 201670096u32);
    pub const get_plugin_icon: (&'static str, u32) = ("_get_plugin_icon", 3635182373u32);
    pub const has_main_screen: (&'static str, u32) = ("_has_main_screen", 36873697u32);
    pub const make_visible: (&'static str, u32) = ("_make_visible", 2586408642u32);
    pub const edit: (&'static str, u32) = ("_edit", 3975164845u32);
    pub const handles: (&'static str, u32) = ("_handles", 397768994u32);
    pub const get_state: (&'static str, u32) = ("_get_state", 3102165223u32);
    pub const set_state: (&'static str, u32) = ("_set_state", 4155329257u32);
    pub const clear: (&'static str, u32) = ("_clear", 3218959716u32);
    pub const get_unsaved_status: (&'static str, u32) = ("_get_unsaved_status", 3135753539u32);
    pub const save_external_data: (&'static str, u32) = ("_save_external_data", 3218959716u32);
    pub const apply_changes: (&'static str, u32) = ("_apply_changes", 3218959716u32);
    pub const get_breakpoints: (&'static str, u32) = ("_get_breakpoints", 1139954409u32);
    pub const set_window_layout: (&'static str, u32) = ("_set_window_layout", 853519107u32);
    pub const get_window_layout: (&'static str, u32) = ("_get_window_layout", 853519107u32);
    pub const build: (&'static str, u32) = ("_build", 2240911060u32);
    pub const enable_plugin: (&'static str, u32) = ("_enable_plugin", 3218959716u32);
    pub const disable_plugin: (&'static str, u32) = ("_disable_plugin", 3218959716u32);
    
}
pub mod EditorProperty {
    pub use super::Container::*;
    pub const update_property: (&'static str, u32) = ("_update_property", 3218959716u32);
    pub const set_read_only: (&'static str, u32) = ("_set_read_only", 2586408642u32);
    
}
pub mod EditorResourceConversionPlugin {
    pub use super::RefCounted::*;
    pub const converts_to: (&'static str, u32) = ("_converts_to", 201670096u32);
    pub const handles: (&'static str, u32) = ("_handles", 3190994482u32);
    pub const convert: (&'static str, u32) = ("_convert", 325183270u32);
    
}
pub mod EditorResourcePicker {
    pub use super::HBoxContainer::*;
    pub const set_create_options: (&'static str, u32) = ("_set_create_options", 3975164845u32);
    pub const handle_menu_selected: (&'static str, u32) = ("_handle_menu_selected", 3067735520u32);
    
}
pub mod EditorResourcePreview {
    pub use super::Node::*;
    
}
pub mod EditorResourcePreviewGenerator {
    pub use super::RefCounted::*;
    pub const handles: (&'static str, u32) = ("_handles", 3927539163u32);
    pub const generate: (&'static str, u32) = ("_generate", 255939159u32);
    pub const generate_from_path: (&'static str, u32) = ("_generate_from_path", 1601192835u32);
    pub const generate_small_preview_automatically: (&'static str, u32) = ("_generate_small_preview_automatically", 36873697u32);
    pub const can_generate_small_preview: (&'static str, u32) = ("_can_generate_small_preview", 36873697u32);
    
}
pub mod EditorResourceTooltipPlugin {
    pub use super::RefCounted::*;
    pub const handles: (&'static str, u32) = ("_handles", 3927539163u32);
    pub const make_tooltip_for_path: (&'static str, u32) = ("_make_tooltip_for_path", 4100114520u32);
    
}
pub mod EditorSceneFormatImporter {
    pub use super::RefCounted::*;
    pub const get_extensions: (&'static str, u32) = ("_get_extensions", 1139954409u32);
    pub const import_scene: (&'static str, u32) = ("_import_scene", 3749238728u32);
    pub const get_import_options: (&'static str, u32) = ("_get_import_options", 83702148u32);
    pub const get_option_visibility: (&'static str, u32) = ("_get_option_visibility", 298836892u32);
    
}
pub mod EditorSceneFormatImporterBlend {
    pub use super::EditorSceneFormatImporter::*;
    
}
pub mod EditorSceneFormatImporterFbx2gltf {
    pub use super::EditorSceneFormatImporter::*;
    
}
pub mod EditorSceneFormatImporterGltf {
    pub use super::EditorSceneFormatImporter::*;
    
}
pub mod EditorSceneFormatImporterUfbx {
    pub use super::EditorSceneFormatImporter::*;
    
}
pub mod EditorScenePostImport {
    pub use super::RefCounted::*;
    pub const post_import: (&'static str, u32) = ("_post_import", 134930648u32);
    
}
pub mod EditorScenePostImportPlugin {
    pub use super::RefCounted::*;
    pub const get_internal_import_options: (&'static str, u32) = ("_get_internal_import_options", 1286410249u32);
    pub const get_internal_option_visibility: (&'static str, u32) = ("_get_internal_option_visibility", 3811255416u32);
    pub const get_internal_option_update_view_required: (&'static str, u32) = ("_get_internal_option_update_view_required", 3957349696u32);
    pub const internal_process: (&'static str, u32) = ("_internal_process", 3641982463u32);
    pub const get_import_options: (&'static str, u32) = ("_get_import_options", 83702148u32);
    pub const get_option_visibility: (&'static str, u32) = ("_get_option_visibility", 298836892u32);
    pub const pre_process: (&'static str, u32) = ("_pre_process", 1078189570u32);
    pub const post_process: (&'static str, u32) = ("_post_process", 1078189570u32);
    
}
pub mod EditorScript {
    pub use super::RefCounted::*;
    pub const run: (&'static str, u32) = ("_run", 3218959716u32);
    
}
pub mod EditorScriptPicker {
    pub use super::EditorResourcePicker::*;
    
}
pub mod EditorSelection {
    pub use super::Object::*;
    
}
pub mod EditorSettings {
    pub use super::Resource::*;
    
}
pub mod EditorSpinSlider {
    pub use super::Range::*;
    
}
pub mod EditorSyntaxHighlighter {
    pub use super::SyntaxHighlighter::*;
    pub const get_name: (&'static str, u32) = ("_get_name", 201670096u32);
    pub const get_supported_languages: (&'static str, u32) = ("_get_supported_languages", 1139954409u32);
    pub const create: (&'static str, u32) = ("_create", 3789807118u32);
    
}
pub mod EditorToaster {
    pub use super::HBoxContainer::*;
    
}
pub mod EditorTranslationParserPlugin {
    pub use super::RefCounted::*;
    pub const parse_file: (&'static str, u32) = ("_parse_file", 1576865988u32);
    pub const get_recognized_extensions: (&'static str, u32) = ("_get_recognized_extensions", 1139954409u32);
    
}
pub mod EditorUndoRedoManager {
    pub use super::Object::*;
    
}
pub mod EditorVcsInterface {
    pub use super::Object::*;
    pub const initialize: (&'static str, u32) = ("_initialize", 2323990056u32);
    pub const set_credentials: (&'static str, u32) = ("_set_credentials", 1336744649u32);
    pub const get_modified_files_data: (&'static str, u32) = ("_get_modified_files_data", 2915620761u32);
    pub const stage_file: (&'static str, u32) = ("_stage_file", 83702148u32);
    pub const unstage_file: (&'static str, u32) = ("_unstage_file", 83702148u32);
    pub const discard_file: (&'static str, u32) = ("_discard_file", 83702148u32);
    pub const commit: (&'static str, u32) = ("_commit", 83702148u32);
    pub const get_diff: (&'static str, u32) = ("_get_diff", 1366379175u32);
    pub const shut_down: (&'static str, u32) = ("_shut_down", 2240911060u32);
    pub const get_vcs_name: (&'static str, u32) = ("_get_vcs_name", 2841200299u32);
    pub const get_previous_commits: (&'static str, u32) = ("_get_previous_commits", 1171824711u32);
    pub const get_branch_list: (&'static str, u32) = ("_get_branch_list", 2915620761u32);
    pub const get_remotes: (&'static str, u32) = ("_get_remotes", 2915620761u32);
    pub const create_branch: (&'static str, u32) = ("_create_branch", 83702148u32);
    pub const remove_branch: (&'static str, u32) = ("_remove_branch", 83702148u32);
    pub const create_remote: (&'static str, u32) = ("_create_remote", 3186203200u32);
    pub const remove_remote: (&'static str, u32) = ("_remove_remote", 83702148u32);
    pub const get_current_branch_name: (&'static str, u32) = ("_get_current_branch_name", 2841200299u32);
    pub const checkout_branch: (&'static str, u32) = ("_checkout_branch", 2323990056u32);
    pub const pull: (&'static str, u32) = ("_pull", 83702148u32);
    pub const push: (&'static str, u32) = ("_push", 2678287736u32);
    pub const fetch: (&'static str, u32) = ("_fetch", 83702148u32);
    pub const get_line_diff: (&'static str, u32) = ("_get_line_diff", 2796572089u32);
    
}
pub mod EncodedObjectAsId {
    pub use super::RefCounted::*;
    
}
pub mod Engine {
    pub use super::Object::*;
    
}
pub mod EngineDebugger {
    pub use super::Object::*;
    
}
pub mod EngineProfiler {
    pub use super::RefCounted::*;
    pub const toggle: (&'static str, u32) = ("_toggle", 1157301103u32);
    pub const add_frame: (&'static str, u32) = ("_add_frame", 381264803u32);
    pub const tick: (&'static str, u32) = ("_tick", 3948312143u32);
    
}
pub mod Environment {
    pub use super::Resource::*;
    
}
pub mod Expression {
    pub use super::RefCounted::*;
    
}
pub mod ExternalTexture {
    pub use super::Texture2D::*;
    
}
pub mod FbxDocument {
    pub use super::GltfDocument::*;
    
}
pub mod FbxState {
    pub use super::GltfState::*;
    
}
pub mod FastNoiseLite {
    pub use super::Noise::*;
    
}
pub mod FileAccess {
    pub use super::RefCounted::*;
    
}
pub mod FileDialog {
    pub use super::ConfirmationDialog::*;
    
}
pub mod FileSystemDock {
    pub use super::VBoxContainer::*;
    
}
pub mod FlowContainer {
    pub use super::Container::*;
    
}
pub mod FogMaterial {
    pub use super::Material::*;
    
}
pub mod FogVolume {
    pub use super::VisualInstance3D::*;
    
}
pub mod FoldableContainer {
    pub use super::Container::*;
    
}
pub mod FoldableGroup {
    pub use super::Resource::*;
    
}
pub mod Font {
    pub use super::Resource::*;
    
}
pub mod FontFile {
    pub use super::Font::*;
    
}
pub mod FontVariation {
    pub use super::Font::*;
    
}
pub mod FramebufferCacheRd {
    pub use super::Object::*;
    
}
pub mod GDExtension {
    pub use super::Resource::*;
    
}
pub mod GDExtensionManager {
    pub use super::Object::*;
    
}
pub mod GDScript {
    pub use super::Script::*;
    
}
pub mod GDScriptSyntaxHighlighter {
    pub use super::EditorSyntaxHighlighter::*;
    
}
pub mod GltfAccessor {
    pub use super::Resource::*;
    
}
pub mod GltfAnimation {
    pub use super::Resource::*;
    
}
pub mod GltfBufferView {
    pub use super::Resource::*;
    
}
pub mod GltfCamera {
    pub use super::Resource::*;
    
}
pub mod GltfDocument {
    pub use super::Resource::*;
    
}
pub mod GltfDocumentExtension {
    pub use super::Resource::*;
    pub const import_preflight: (&'static str, u32) = ("_import_preflight", 412946943u32);
    pub const get_supported_extensions: (&'static str, u32) = ("_get_supported_extensions", 2981934095u32);
    pub const parse_node_extensions: (&'static str, u32) = ("_parse_node_extensions", 2067053794u32);
    pub const parse_image_data: (&'static str, u32) = ("_parse_image_data", 3201673288u32);
    pub const get_image_file_extension: (&'static str, u32) = ("_get_image_file_extension", 2841200299u32);
    pub const parse_texture_json: (&'static str, u32) = ("_parse_texture_json", 1624327185u32);
    pub const import_object_model_property: (&'static str, u32) = ("_import_object_model_property", 1446147484u32);
    pub const import_post_parse: (&'static str, u32) = ("_import_post_parse", 1704600462u32);
    pub const import_pre_generate: (&'static str, u32) = ("_import_pre_generate", 1704600462u32);
    pub const generate_scene_node: (&'static str, u32) = ("_generate_scene_node", 3810899026u32);
    pub const import_node: (&'static str, u32) = ("_import_node", 4064279746u32);
    pub const import_post: (&'static str, u32) = ("_import_post", 295478427u32);
    pub const export_preflight: (&'static str, u32) = ("_export_preflight", 295478427u32);
    pub const convert_scene_node: (&'static str, u32) = ("_convert_scene_node", 147612932u32);
    pub const export_post_convert: (&'static str, u32) = ("_export_post_convert", 295478427u32);
    pub const export_preserialize: (&'static str, u32) = ("_export_preserialize", 1704600462u32);
    pub const export_object_model_property: (&'static str, u32) = ("_export_object_model_property", 4111022730u32);
    pub const get_saveable_image_formats: (&'static str, u32) = ("_get_saveable_image_formats", 2981934095u32);
    pub const serialize_image_to_bytes: (&'static str, u32) = ("_serialize_image_to_bytes", 276886664u32);
    pub const save_image_at_path: (&'static str, u32) = ("_save_image_at_path", 1844337242u32);
    pub const serialize_texture_json: (&'static str, u32) = ("_serialize_texture_json", 2565166506u32);
    pub const export_node: (&'static str, u32) = ("_export_node", 4064279746u32);
    pub const export_post: (&'static str, u32) = ("_export_post", 1704600462u32);
    
}
pub mod GltfDocumentExtensionConvertImporterMesh {
    pub use super::GltfDocumentExtension::*;
    
}
pub mod GltfLight {
    pub use super::Resource::*;
    
}
pub mod GltfMesh {
    pub use super::Resource::*;
    
}
pub mod GltfNode {
    pub use super::Resource::*;
    
}
pub mod GltfObjectModelProperty {
    pub use super::RefCounted::*;
    
}
pub mod GltfPhysicsBody {
    pub use super::Resource::*;
    
}
pub mod GltfPhysicsShape {
    pub use super::Resource::*;
    
}
pub mod GltfSkeleton {
    pub use super::Resource::*;
    
}
pub mod GltfSkin {
    pub use super::Resource::*;
    
}
pub mod GltfSpecGloss {
    pub use super::Resource::*;
    
}
pub mod GltfState {
    pub use super::Resource::*;
    
}
pub mod GltfTexture {
    pub use super::Resource::*;
    
}
pub mod GltfTextureSampler {
    pub use super::Resource::*;
    
}
pub mod GpuParticles2D {
    pub use super::Node2D::*;
    
}
pub mod GpuParticles3D {
    pub use super::GeometryInstance3D::*;
    
}
pub mod GpuParticlesAttractor3D {
    pub use super::VisualInstance3D::*;
    
}
pub mod GpuParticlesAttractorBox3D {
    pub use super::GpuParticlesAttractor3D::*;
    
}
pub mod GpuParticlesAttractorSphere3D {
    pub use super::GpuParticlesAttractor3D::*;
    
}
pub mod GpuParticlesAttractorVectorField3D {
    pub use super::GpuParticlesAttractor3D::*;
    
}
pub mod GpuParticlesCollision3D {
    pub use super::VisualInstance3D::*;
    
}
pub mod GpuParticlesCollisionBox3D {
    pub use super::GpuParticlesCollision3D::*;
    
}
pub mod GpuParticlesCollisionHeightField3D {
    pub use super::GpuParticlesCollision3D::*;
    
}
pub mod GpuParticlesCollisionSdf3d {
    pub use super::GpuParticlesCollision3D::*;
    
}
pub mod GpuParticlesCollisionSphere3D {
    pub use super::GpuParticlesCollision3D::*;
    
}
pub mod Generic6DofJoint3D {
    pub use super::Joint3D::*;
    
}
pub mod Geometry2D {
    pub use super::Object::*;
    
}
pub mod Geometry3D {
    pub use super::Object::*;
    
}
pub mod GeometryInstance3D {
    pub use super::VisualInstance3D::*;
    
}
pub mod Gradient {
    pub use super::Resource::*;
    
}
pub mod GradientTexture1D {
    pub use super::Texture2D::*;
    
}
pub mod GradientTexture2D {
    pub use super::Texture2D::*;
    
}
pub mod GridContainer {
    pub use super::Container::*;
    
}
pub mod GridMap {
    pub use super::Node3D::*;
    
}
pub mod GridMapEditorPlugin {
    pub use super::EditorPlugin::*;
    
}
pub mod GrooveJoint2D {
    pub use super::Joint2D::*;
    
}
pub mod HBoxContainer {
    pub use super::BoxContainer::*;
    
}
pub mod HFlowContainer {
    pub use super::FlowContainer::*;
    
}
pub mod HmacContext {
    pub use super::RefCounted::*;
    
}
pub mod HScrollBar {
    pub use super::ScrollBar::*;
    
}
pub mod HSeparator {
    pub use super::Separator::*;
    
}
pub mod HSlider {
    pub use super::Slider::*;
    
}
pub mod HSplitContainer {
    pub use super::SplitContainer::*;
    
}
pub mod HttpClient {
    pub use super::RefCounted::*;
    
}
pub mod HttpRequest {
    pub use super::Node::*;
    
}
pub mod HashingContext {
    pub use super::RefCounted::*;
    
}
pub mod HeightMapShape3D {
    pub use super::Shape3D::*;
    
}
pub mod HingeJoint3D {
    pub use super::Joint3D::*;
    
}
pub mod Ip {
    pub use super::Object::*;
    
}
pub mod Image {
    pub use super::Resource::*;
    
}
pub mod ImageFormatLoader {
    pub use super::RefCounted::*;
    
}
pub mod ImageFormatLoaderExtension {
    pub use super::ImageFormatLoader::*;
    pub const get_recognized_extensions: (&'static str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub const load_image: (&'static str, u32) = ("_load_image", 3760540541u32);
    
}
pub mod ImageTexture {
    pub use super::Texture2D::*;
    
}
pub mod ImageTexture3D {
    pub use super::Texture3D::*;
    
}
pub mod ImageTextureLayered {
    pub use super::TextureLayered::*;
    
}
pub mod ImmediateMesh {
    pub use super::Mesh::*;
    
}
pub mod ImporterMesh {
    pub use super::Resource::*;
    
}
pub mod ImporterMeshInstance3D {
    pub use super::Node3D::*;
    
}
pub mod Input {
    pub use super::Object::*;
    
}
pub mod InputEvent {
    pub use super::Resource::*;
    
}
pub mod InputEventAction {
    pub use super::InputEvent::*;
    
}
pub mod InputEventFromWindow {
    pub use super::InputEvent::*;
    
}
pub mod InputEventGesture {
    pub use super::InputEventWithModifiers::*;
    
}
pub mod InputEventJoypadButton {
    pub use super::InputEvent::*;
    
}
pub mod InputEventJoypadMotion {
    pub use super::InputEvent::*;
    
}
pub mod InputEventKey {
    pub use super::InputEventWithModifiers::*;
    
}
pub mod InputEventMidi {
    pub use super::InputEvent::*;
    
}
pub mod InputEventMagnifyGesture {
    pub use super::InputEventGesture::*;
    
}
pub mod InputEventMouse {
    pub use super::InputEventWithModifiers::*;
    
}
pub mod InputEventMouseButton {
    pub use super::InputEventMouse::*;
    
}
pub mod InputEventMouseMotion {
    pub use super::InputEventMouse::*;
    
}
pub mod InputEventPanGesture {
    pub use super::InputEventGesture::*;
    
}
pub mod InputEventScreenDrag {
    pub use super::InputEventFromWindow::*;
    
}
pub mod InputEventScreenTouch {
    pub use super::InputEventFromWindow::*;
    
}
pub mod InputEventShortcut {
    pub use super::InputEvent::*;
    
}
pub mod InputEventWithModifiers {
    pub use super::InputEventFromWindow::*;
    
}
pub mod InputMap {
    pub use super::Object::*;
    
}
pub mod InstancePlaceholder {
    pub use super::Node::*;
    
}
pub mod IntervalTweener {
    pub use super::Tweener::*;
    
}
pub mod ItemList {
    pub use super::Control::*;
    
}
pub mod Json {
    pub use super::Resource::*;
    
}
pub mod JsonRpc {
    pub use super::Object::*;
    
}
pub mod Joint2D {
    pub use super::Node2D::*;
    
}
pub mod Joint3D {
    pub use super::Node3D::*;
    
}
pub mod KinematicCollision2D {
    pub use super::RefCounted::*;
    
}
pub mod KinematicCollision3D {
    pub use super::RefCounted::*;
    
}
pub mod Label {
    pub use super::Control::*;
    
}
pub mod Label3D {
    pub use super::GeometryInstance3D::*;
    
}
pub mod LabelSettings {
    pub use super::Resource::*;
    
}
pub mod Light2D {
    pub use super::Node2D::*;
    
}
pub mod Light3D {
    pub use super::VisualInstance3D::*;
    
}
pub mod LightOccluder2D {
    pub use super::Node2D::*;
    
}
pub mod LightmapGi {
    pub use super::VisualInstance3D::*;
    
}
pub mod LightmapGiData {
    pub use super::Resource::*;
    
}
pub mod LightmapProbe {
    pub use super::Node3D::*;
    
}
pub mod Lightmapper {
    pub use super::RefCounted::*;
    
}
pub mod LightmapperRd {
    pub use super::Lightmapper::*;
    
}
pub mod Line2D {
    pub use super::Node2D::*;
    
}
pub mod LineEdit {
    pub use super::Control::*;
    
}
pub mod LinkButton {
    pub use super::BaseButton::*;
    
}
pub mod Logger {
    pub use super::RefCounted::*;
    pub const log_error: (&'static str, u32) = ("_log_error", 27079556u32);
    pub const log_message: (&'static str, u32) = ("_log_message", 2678287736u32);
    
}
pub mod LookAtModifier3D {
    pub use super::SkeletonModifier3D::*;
    
}
pub mod MainLoop {
    pub use super::Object::*;
    pub const initialize: (&'static str, u32) = ("_initialize", 3218959716u32);
    pub const physics_process: (&'static str, u32) = ("_physics_process", 330693286u32);
    pub const process: (&'static str, u32) = ("_process", 330693286u32);
    pub const finalize: (&'static str, u32) = ("_finalize", 3218959716u32);
    
}
pub mod MarginContainer {
    pub use super::Container::*;
    
}
pub mod Marker2D {
    pub use super::Node2D::*;
    
}
pub mod Marker3D {
    pub use super::Node3D::*;
    
}
pub mod Marshalls {
    pub use super::Object::*;
    
}
pub mod Material {
    pub use super::Resource::*;
    pub const get_shader_rid: (&'static str, u32) = ("_get_shader_rid", 2944877500u32);
    pub const get_shader_mode: (&'static str, u32) = ("_get_shader_mode", 3392948163u32);
    pub const can_do_next_pass: (&'static str, u32) = ("_can_do_next_pass", 36873697u32);
    pub const can_use_render_priority: (&'static str, u32) = ("_can_use_render_priority", 36873697u32);
    
}
pub mod MenuBar {
    pub use super::Control::*;
    
}
pub mod MenuButton {
    pub use super::Button::*;
    
}
pub mod Mesh {
    pub use super::Resource::*;
    pub const get_surface_count: (&'static str, u32) = ("_get_surface_count", 3905245786u32);
    pub const surface_get_array_len: (&'static str, u32) = ("_surface_get_array_len", 923996154u32);
    pub const surface_get_array_index_len: (&'static str, u32) = ("_surface_get_array_index_len", 923996154u32);
    pub const surface_get_arrays: (&'static str, u32) = ("_surface_get_arrays", 663333327u32);
    pub const surface_get_blend_shape_arrays: (&'static str, u32) = ("_surface_get_blend_shape_arrays", 663333327u32);
    pub const surface_get_lods: (&'static str, u32) = ("_surface_get_lods", 3485342025u32);
    pub const surface_get_format: (&'static str, u32) = ("_surface_get_format", 923996154u32);
    pub const surface_get_primitive_type: (&'static str, u32) = ("_surface_get_primitive_type", 923996154u32);
    pub const surface_set_material: (&'static str, u32) = ("_surface_set_material", 3671737478u32);
    pub const surface_get_material: (&'static str, u32) = ("_surface_get_material", 2897466400u32);
    pub const get_blend_shape_count: (&'static str, u32) = ("_get_blend_shape_count", 3905245786u32);
    pub const get_blend_shape_name: (&'static str, u32) = ("_get_blend_shape_name", 659327637u32);
    pub const set_blend_shape_name: (&'static str, u32) = ("_set_blend_shape_name", 3780747571u32);
    pub const get_aabb: (&'static str, u32) = ("_get_aabb", 1068685055u32);
    
}
pub mod MeshConvexDecompositionSettings {
    pub use super::RefCounted::*;
    
}
pub mod MeshDataTool {
    pub use super::RefCounted::*;
    
}
pub mod MeshInstance2D {
    pub use super::Node2D::*;
    
}
pub mod MeshInstance3D {
    pub use super::GeometryInstance3D::*;
    
}
pub mod MeshLibrary {
    pub use super::Resource::*;
    
}
pub mod MeshTexture {
    pub use super::Texture2D::*;
    
}
pub mod MethodTweener {
    pub use super::Tweener::*;
    
}
pub mod MissingNode {
    pub use super::Node::*;
    
}
pub mod MissingResource {
    pub use super::Resource::*;
    
}
pub mod MobileVrInterface {
    pub use super::XrInterface::*;
    
}
pub mod ModifierBoneTarget3D {
    pub use super::SkeletonModifier3D::*;
    
}
pub mod MovieWriter {
    pub use super::Object::*;
    pub const get_audio_mix_rate: (&'static str, u32) = ("_get_audio_mix_rate", 3905245786u32);
    pub const get_audio_speaker_mode: (&'static str, u32) = ("_get_audio_speaker_mode", 2549190337u32);
    pub const handles_file: (&'static str, u32) = ("_handles_file", 3927539163u32);
    pub const write_begin: (&'static str, u32) = ("_write_begin", 1866453460u32);
    pub const write_frame_rawptr: (&'static str, u32) = ("_write_frame", 2784607037u32);
    pub const write_end: (&'static str, u32) = ("_write_end", 3218959716u32);
    
}
pub mod MultiMesh {
    pub use super::Resource::*;
    
}
pub mod MultiMeshInstance2D {
    pub use super::Node2D::*;
    
}
pub mod MultiMeshInstance3D {
    pub use super::GeometryInstance3D::*;
    
}
pub mod MultiplayerApi {
    pub use super::RefCounted::*;
    
}
pub mod MultiplayerApiExtension {
    pub use super::MultiplayerApi::*;
    pub const poll: (&'static str, u32) = ("_poll", 166280745u32);
    pub const set_multiplayer_peer: (&'static str, u32) = ("_set_multiplayer_peer", 3694835298u32);
    pub const get_multiplayer_peer: (&'static str, u32) = ("_get_multiplayer_peer", 3223692825u32);
    pub const get_unique_id: (&'static str, u32) = ("_get_unique_id", 3905245786u32);
    pub const get_peer_ids: (&'static str, u32) = ("_get_peer_ids", 1930428628u32);
    pub const rpc: (&'static str, u32) = ("_rpc", 3673574758u32);
    pub const get_remote_sender_id: (&'static str, u32) = ("_get_remote_sender_id", 3905245786u32);
    pub const object_configuration_add: (&'static str, u32) = ("_object_configuration_add", 1171879464u32);
    pub const object_configuration_remove: (&'static str, u32) = ("_object_configuration_remove", 1171879464u32);
    
}
pub mod MultiplayerPeer {
    pub use super::PacketPeer::*;
    
}
pub mod MultiplayerPeerExtension {
    pub use super::MultiplayerPeer::*;
    pub const get_packet_rawptr: (&'static str, u32) = ("_get_packet", 3099858825u32);
    pub const put_packet_rawptr: (&'static str, u32) = ("_put_packet", 3099858825u32);
    pub const get_available_packet_count: (&'static str, u32) = ("_get_available_packet_count", 3905245786u32);
    pub const get_max_packet_size: (&'static str, u32) = ("_get_max_packet_size", 3905245786u32);
    pub const get_packet_script: (&'static str, u32) = ("_get_packet_script", 2115431945u32);
    pub const put_packet_script: (&'static str, u32) = ("_put_packet_script", 680677267u32);
    pub const get_packet_channel: (&'static str, u32) = ("_get_packet_channel", 3905245786u32);
    pub const get_packet_mode: (&'static str, u32) = ("_get_packet_mode", 3369852622u32);
    pub const set_transfer_channel: (&'static str, u32) = ("_set_transfer_channel", 1286410249u32);
    pub const get_transfer_channel: (&'static str, u32) = ("_get_transfer_channel", 3905245786u32);
    pub const set_transfer_mode: (&'static str, u32) = ("_set_transfer_mode", 950411049u32);
    pub const get_transfer_mode: (&'static str, u32) = ("_get_transfer_mode", 3369852622u32);
    pub const set_target_peer: (&'static str, u32) = ("_set_target_peer", 1286410249u32);
    pub const get_packet_peer: (&'static str, u32) = ("_get_packet_peer", 3905245786u32);
    pub const is_server: (&'static str, u32) = ("_is_server", 36873697u32);
    pub const poll: (&'static str, u32) = ("_poll", 3218959716u32);
    pub const close: (&'static str, u32) = ("_close", 3218959716u32);
    pub const disconnect_peer: (&'static str, u32) = ("_disconnect_peer", 300928843u32);
    pub const get_unique_id: (&'static str, u32) = ("_get_unique_id", 3905245786u32);
    pub const set_refuse_new_connections: (&'static str, u32) = ("_set_refuse_new_connections", 2586408642u32);
    pub const is_refusing_new_connections: (&'static str, u32) = ("_is_refusing_new_connections", 36873697u32);
    pub const is_server_relay_supported: (&'static str, u32) = ("_is_server_relay_supported", 36873697u32);
    pub const get_connection_status: (&'static str, u32) = ("_get_connection_status", 2147374275u32);
    
}
pub mod MultiplayerSpawner {
    pub use super::Node::*;
    
}
pub mod MultiplayerSynchronizer {
    pub use super::Node::*;
    
}
pub mod NativeMenu {
    pub use super::Object::*;
    
}
pub mod NavigationMeshGenerator {
    pub use super::Object::*;
    
}
pub mod NinePatchRect {
    pub use super::Control::*;
    
}
pub mod Node {
    pub use super::Object::*;
    pub const process: (&'static str, u32) = ("_process", 373806689u32);
    pub const physics_process: (&'static str, u32) = ("_physics_process", 373806689u32);
    pub const enter_tree: (&'static str, u32) = ("_enter_tree", 3218959716u32);
    pub const exit_tree: (&'static str, u32) = ("_exit_tree", 3218959716u32);
    pub const ready: (&'static str, u32) = ("_ready", 3218959716u32);
    pub const get_configuration_warnings: (&'static str, u32) = ("_get_configuration_warnings", 1139954409u32);
    pub const get_accessibility_configuration_warnings: (&'static str, u32) = ("_get_accessibility_configuration_warnings", 1139954409u32);
    pub const input: (&'static str, u32) = ("_input", 3754044979u32);
    pub const shortcut_input: (&'static str, u32) = ("_shortcut_input", 3754044979u32);
    pub const unhandled_input: (&'static str, u32) = ("_unhandled_input", 3754044979u32);
    pub const unhandled_key_input: (&'static str, u32) = ("_unhandled_key_input", 3754044979u32);
    pub const get_focused_accessibility_element: (&'static str, u32) = ("_get_focused_accessibility_element", 2944877500u32);
    
}
pub mod Node2D {
    pub use super::CanvasItem::*;
    
}
pub mod Node3D {
    pub use super::Node::*;
    
}
pub mod Node3DGizmo {
    pub use super::RefCounted::*;
    
}
pub mod Noise {
    pub use super::Resource::*;
    
}
pub mod NoiseTexture2D {
    pub use super::Texture2D::*;
    
}
pub mod NoiseTexture3D {
    pub use super::Texture3D::*;
    
}
pub mod OrmMaterial3D {
    pub use super::BaseMaterial3D::*;
    
}
pub mod Os {
    pub use super::Object::*;
    
}
pub mod Object {
    
}
pub mod Occluder3D {
    pub use super::Resource::*;
    
}
pub mod OccluderInstance3D {
    pub use super::VisualInstance3D::*;
    
}
pub mod OccluderPolygon2D {
    pub use super::Resource::*;
    
}
pub mod OfflineMultiplayerPeer {
    pub use super::MultiplayerPeer::*;
    
}
pub mod OggPacketSequence {
    pub use super::Resource::*;
    
}
pub mod OggPacketSequencePlayback {
    pub use super::RefCounted::*;
    
}
pub mod OmniLight3D {
    pub use super::Light3D::*;
    
}
pub mod OpenXrApiExtension {
    pub use super::RefCounted::*;
    
}
pub mod OpenXrAction {
    pub use super::Resource::*;
    
}
pub mod OpenXrActionBindingModifier {
    pub use super::OpenXrBindingModifier::*;
    
}
pub mod OpenXrActionMap {
    pub use super::Resource::*;
    
}
pub mod OpenXrActionSet {
    pub use super::Resource::*;
    
}
pub mod OpenXrAnalogThresholdModifier {
    pub use super::OpenXrActionBindingModifier::*;
    
}
pub mod OpenXrBindingModifier {
    pub use super::Resource::*;
    pub const get_description: (&'static str, u32) = ("_get_description", 201670096u32);
    pub const get_ip_modification: (&'static str, u32) = ("_get_ip_modification", 2115431945u32);
    
}
pub mod OpenXrBindingModifierEditor {
    pub use super::PanelContainer::*;
    
}
pub mod OpenXrCompositionLayer {
    pub use super::Node3D::*;
    
}
pub mod OpenXrCompositionLayerCylinder {
    pub use super::OpenXrCompositionLayer::*;
    
}
pub mod OpenXrCompositionLayerEquirect {
    pub use super::OpenXrCompositionLayer::*;
    
}
pub mod OpenXrCompositionLayerQuad {
    pub use super::OpenXrCompositionLayer::*;
    
}
pub mod OpenXrDpadBindingModifier {
    pub use super::OpenXripBindingModifier::*;
    
}
pub mod OpenXrExtensionWrapper {
    pub use super::Object::*;
    pub const get_requested_extensions: (&'static str, u32) = ("_get_requested_extensions", 2382534195u32);
    pub const set_system_properties_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_system_properties_and_get_next_pointer", 3744713108u32);
    pub const set_instance_create_info_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_instance_create_info_and_get_next_pointer", 3744713108u32);
    pub const set_session_create_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_session_create_and_get_next_pointer", 3744713108u32);
    pub const set_swapchain_create_info_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_swapchain_create_info_and_get_next_pointer", 3744713108u32);
    pub const set_hand_joint_locations_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_hand_joint_locations_and_get_next_pointer", 50157827u32);
    pub const set_projection_views_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_projection_views_and_get_next_pointer", 50157827u32);
    pub const set_frame_wait_info_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_frame_wait_info_and_get_next_pointer", 3744713108u32);
    pub const set_frame_end_info_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_frame_end_info_and_get_next_pointer", 3744713108u32);
    pub const set_view_locate_info_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_view_locate_info_and_get_next_pointer", 3744713108u32);
    pub const set_reference_space_create_info_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_reference_space_create_info_and_get_next_pointer", 50157827u32);
    pub const get_composition_layer_count: (&'static str, u32) = ("_get_composition_layer_count", 2455072627u32);
    pub const get_composition_layer: (&'static str, u32) = ("_get_composition_layer", 3744713108u32);
    pub const get_composition_layer_order: (&'static str, u32) = ("_get_composition_layer_order", 3744713108u32);
    pub const get_suggested_tracker_names: (&'static str, u32) = ("_get_suggested_tracker_names", 2981934095u32);
    pub const on_register_metadata: (&'static str, u32) = ("_on_register_metadata", 3218959716u32);
    pub const on_before_instance_created: (&'static str, u32) = ("_on_before_instance_created", 3218959716u32);
    pub const on_instance_created: (&'static str, u32) = ("_on_instance_created", 1286410249u32);
    pub const on_instance_destroyed: (&'static str, u32) = ("_on_instance_destroyed", 3218959716u32);
    pub const on_session_created: (&'static str, u32) = ("_on_session_created", 1286410249u32);
    pub const on_process: (&'static str, u32) = ("_on_process", 3218959716u32);
    pub const on_sync_actions: (&'static str, u32) = ("_on_sync_actions", 3218959716u32);
    pub const on_pre_render: (&'static str, u32) = ("_on_pre_render", 3218959716u32);
    pub const on_main_swapchains_created: (&'static str, u32) = ("_on_main_swapchains_created", 3218959716u32);
    pub const on_pre_draw_viewport: (&'static str, u32) = ("_on_pre_draw_viewport", 2722037293u32);
    pub const on_post_draw_viewport: (&'static str, u32) = ("_on_post_draw_viewport", 2722037293u32);
    pub const on_session_destroyed: (&'static str, u32) = ("_on_session_destroyed", 3218959716u32);
    pub const on_state_idle: (&'static str, u32) = ("_on_state_idle", 3218959716u32);
    pub const on_state_ready: (&'static str, u32) = ("_on_state_ready", 3218959716u32);
    pub const on_state_synchronized: (&'static str, u32) = ("_on_state_synchronized", 3218959716u32);
    pub const on_state_visible: (&'static str, u32) = ("_on_state_visible", 3218959716u32);
    pub const on_state_focused: (&'static str, u32) = ("_on_state_focused", 3218959716u32);
    pub const on_state_stopping: (&'static str, u32) = ("_on_state_stopping", 3218959716u32);
    pub const on_state_loss_pending: (&'static str, u32) = ("_on_state_loss_pending", 3218959716u32);
    pub const on_state_exiting: (&'static str, u32) = ("_on_state_exiting", 3218959716u32);
    pub const on_event_polled_rawptr: (&'static str, u32) = ("_on_event_polled", 3067735520u32);
    pub const set_viewport_composition_layer_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_viewport_composition_layer_and_get_next_pointer", 2250464348u32);
    pub const get_viewport_composition_layer_extension_properties: (&'static str, u32) = ("_get_viewport_composition_layer_extension_properties", 2915620761u32);
    pub const get_viewport_composition_layer_extension_property_defaults: (&'static str, u32) = ("_get_viewport_composition_layer_extension_property_defaults", 2382534195u32);
    pub const on_viewport_composition_layer_destroyed_rawptr: (&'static str, u32) = ("_on_viewport_composition_layer_destroyed", 1286410249u32);
    pub const set_android_surface_swapchain_create_info_and_get_next_pointer_rawptr: (&'static str, u32) = ("_set_android_surface_swapchain_create_info_and_get_next_pointer", 3726637545u32);
    
}
pub mod OpenXrExtensionWrapperExtension {
    pub use super::OpenXrExtensionWrapper::*;
    
}
pub mod OpenXrFutureExtension {
    pub use super::OpenXrExtensionWrapper::*;
    
}
pub mod OpenXrFutureResult {
    pub use super::RefCounted::*;
    
}
pub mod OpenXrHand {
    pub use super::Node3D::*;
    
}
pub mod OpenXrHapticBase {
    pub use super::Resource::*;
    
}
pub mod OpenXrHapticVibration {
    pub use super::OpenXrHapticBase::*;
    
}
pub mod OpenXrIpBinding {
    pub use super::Resource::*;
    
}
pub mod OpenXripBindingModifier {
    pub use super::OpenXrBindingModifier::*;
    
}
pub mod OpenXrInteractionProfile {
    pub use super::Resource::*;
    
}
pub mod OpenXrInteractionProfileEditor {
    pub use super::OpenXrInteractionProfileEditorBase::*;
    
}
pub mod OpenXrInteractionProfileEditorBase {
    pub use super::HBoxContainer::*;
    
}
pub mod OpenXrInteractionProfileMetadata {
    pub use super::Object::*;
    
}
pub mod OpenXrInterface {
    pub use super::XrInterface::*;
    
}
pub mod OpenXrRenderModel {
    pub use super::Node3D::*;
    
}
pub mod OpenXrRenderModelExtension {
    pub use super::OpenXrExtensionWrapper::*;
    
}
pub mod OpenXrRenderModelManager {
    pub use super::Node3D::*;
    
}
pub mod OpenXrVisibilityMask {
    pub use super::VisualInstance3D::*;
    
}
pub mod OptimizedTranslation {
    pub use super::Translation::*;
    
}
pub mod OptionButton {
    pub use super::Button::*;
    
}
pub mod PckPacker {
    pub use super::RefCounted::*;
    
}
pub mod PackedDataContainer {
    pub use super::Resource::*;
    
}
pub mod PackedDataContainerRef {
    pub use super::RefCounted::*;
    
}
pub mod PackedScene {
    pub use super::Resource::*;
    
}
pub mod PacketPeer {
    pub use super::RefCounted::*;
    
}
pub mod PacketPeerDtls {
    pub use super::PacketPeer::*;
    
}
pub mod PacketPeerExtension {
    pub use super::PacketPeer::*;
    pub const get_packet_rawptr: (&'static str, u32) = ("_get_packet", 3099858825u32);
    pub const put_packet_rawptr: (&'static str, u32) = ("_put_packet", 3099858825u32);
    pub const get_available_packet_count: (&'static str, u32) = ("_get_available_packet_count", 3905245786u32);
    pub const get_max_packet_size: (&'static str, u32) = ("_get_max_packet_size", 3905245786u32);
    
}
pub mod PacketPeerStream {
    pub use super::PacketPeer::*;
    
}
pub mod PacketPeerUdp {
    pub use super::PacketPeer::*;
    
}
pub mod Panel {
    pub use super::Control::*;
    
}
pub mod PanelContainer {
    pub use super::Container::*;
    
}
pub mod PanoramaSkyMaterial {
    pub use super::Material::*;
    
}
pub mod ParallaxBackground {
    pub use super::CanvasLayer::*;
    
}
pub mod ParallaxLayer {
    pub use super::Node2D::*;
    
}
pub mod ParticleProcessMaterial {
    pub use super::Material::*;
    
}
pub mod Path2D {
    pub use super::Node2D::*;
    
}
pub mod Path3D {
    pub use super::Node3D::*;
    
}
pub mod PathFollow2D {
    pub use super::Node2D::*;
    
}
pub mod PathFollow3D {
    pub use super::Node3D::*;
    
}
pub mod Performance {
    pub use super::Object::*;
    
}
pub mod PhysicalBone2D {
    pub use super::RigidBody2D::*;
    
}
pub mod PhysicalBone3D {
    pub use super::PhysicsBody3D::*;
    pub const integrate_forces: (&'static str, u32) = ("_integrate_forces", 420958145u32);
    
}
pub mod PhysicalBoneSimulator3D {
    pub use super::SkeletonModifier3D::*;
    
}
pub mod PhysicalSkyMaterial {
    pub use super::Material::*;
    
}
pub mod PhysicsBody2D {
    pub use super::CollisionObject2D::*;
    
}
pub mod PhysicsBody3D {
    pub use super::CollisionObject3D::*;
    
}
pub mod PhysicsDirectBodyState2D {
    pub use super::Object::*;
    
}
pub mod PhysicsDirectBodyState2DExtension {
    pub use super::PhysicsDirectBodyState2D::*;
    pub const get_total_gravity: (&'static str, u32) = ("_get_total_gravity", 3341600327u32);
    pub const get_total_linear_damp: (&'static str, u32) = ("_get_total_linear_damp", 1740695150u32);
    pub const get_total_angular_damp: (&'static str, u32) = ("_get_total_angular_damp", 1740695150u32);
    pub const get_center_of_mass: (&'static str, u32) = ("_get_center_of_mass", 3341600327u32);
    pub const get_center_of_mass_local: (&'static str, u32) = ("_get_center_of_mass_local", 3341600327u32);
    pub const get_inverse_mass: (&'static str, u32) = ("_get_inverse_mass", 1740695150u32);
    pub const get_inverse_inertia: (&'static str, u32) = ("_get_inverse_inertia", 1740695150u32);
    pub const set_linear_velocity: (&'static str, u32) = ("_set_linear_velocity", 743155724u32);
    pub const get_linear_velocity: (&'static str, u32) = ("_get_linear_velocity", 3341600327u32);
    pub const set_angular_velocity: (&'static str, u32) = ("_set_angular_velocity", 373806689u32);
    pub const get_angular_velocity: (&'static str, u32) = ("_get_angular_velocity", 1740695150u32);
    pub const set_transform: (&'static str, u32) = ("_set_transform", 2761652528u32);
    pub const get_transform: (&'static str, u32) = ("_get_transform", 3814499831u32);
    pub const get_velocity_at_local_position: (&'static str, u32) = ("_get_velocity_at_local_position", 2656412154u32);
    pub const apply_central_impulse: (&'static str, u32) = ("_apply_central_impulse", 743155724u32);
    pub const apply_impulse: (&'static str, u32) = ("_apply_impulse", 3108078480u32);
    pub const apply_torque_impulse: (&'static str, u32) = ("_apply_torque_impulse", 373806689u32);
    pub const apply_central_force: (&'static str, u32) = ("_apply_central_force", 743155724u32);
    pub const apply_force: (&'static str, u32) = ("_apply_force", 3108078480u32);
    pub const apply_torque: (&'static str, u32) = ("_apply_torque", 373806689u32);
    pub const add_constant_central_force: (&'static str, u32) = ("_add_constant_central_force", 743155724u32);
    pub const add_constant_force: (&'static str, u32) = ("_add_constant_force", 3108078480u32);
    pub const add_constant_torque: (&'static str, u32) = ("_add_constant_torque", 373806689u32);
    pub const set_constant_force: (&'static str, u32) = ("_set_constant_force", 743155724u32);
    pub const get_constant_force: (&'static str, u32) = ("_get_constant_force", 3341600327u32);
    pub const set_constant_torque: (&'static str, u32) = ("_set_constant_torque", 373806689u32);
    pub const get_constant_torque: (&'static str, u32) = ("_get_constant_torque", 1740695150u32);
    pub const set_sleep_state: (&'static str, u32) = ("_set_sleep_state", 2586408642u32);
    pub const is_sleeping: (&'static str, u32) = ("_is_sleeping", 36873697u32);
    pub const set_collision_layer: (&'static str, u32) = ("_set_collision_layer", 1286410249u32);
    pub const get_collision_layer: (&'static str, u32) = ("_get_collision_layer", 3905245786u32);
    pub const set_collision_mask: (&'static str, u32) = ("_set_collision_mask", 1286410249u32);
    pub const get_collision_mask: (&'static str, u32) = ("_get_collision_mask", 3905245786u32);
    pub const get_contact_count: (&'static str, u32) = ("_get_contact_count", 3905245786u32);
    pub const get_contact_local_position: (&'static str, u32) = ("_get_contact_local_position", 2299179447u32);
    pub const get_contact_local_normal: (&'static str, u32) = ("_get_contact_local_normal", 2299179447u32);
    pub const get_contact_local_shape: (&'static str, u32) = ("_get_contact_local_shape", 923996154u32);
    pub const get_contact_local_velocity_at_position: (&'static str, u32) = ("_get_contact_local_velocity_at_position", 2299179447u32);
    pub const get_contact_collider: (&'static str, u32) = ("_get_contact_collider", 495598643u32);
    pub const get_contact_collider_position: (&'static str, u32) = ("_get_contact_collider_position", 2299179447u32);
    pub const get_contact_collider_id: (&'static str, u32) = ("_get_contact_collider_id", 923996154u32);
    pub const get_contact_collider_object: (&'static str, u32) = ("_get_contact_collider_object", 3332903315u32);
    pub const get_contact_collider_shape: (&'static str, u32) = ("_get_contact_collider_shape", 923996154u32);
    pub const get_contact_collider_velocity_at_position: (&'static str, u32) = ("_get_contact_collider_velocity_at_position", 2299179447u32);
    pub const get_contact_impulse: (&'static str, u32) = ("_get_contact_impulse", 2299179447u32);
    pub const get_step: (&'static str, u32) = ("_get_step", 1740695150u32);
    pub const integrate_forces: (&'static str, u32) = ("_integrate_forces", 3218959716u32);
    pub const get_space_state: (&'static str, u32) = ("_get_space_state", 2506717822u32);
    
}
pub mod PhysicsDirectBodyState3D {
    pub use super::Object::*;
    
}
pub mod PhysicsDirectBodyState3DExtension {
    pub use super::PhysicsDirectBodyState3D::*;
    pub const get_total_gravity: (&'static str, u32) = ("_get_total_gravity", 3360562783u32);
    pub const get_total_linear_damp: (&'static str, u32) = ("_get_total_linear_damp", 1740695150u32);
    pub const get_total_angular_damp: (&'static str, u32) = ("_get_total_angular_damp", 1740695150u32);
    pub const get_center_of_mass: (&'static str, u32) = ("_get_center_of_mass", 3360562783u32);
    pub const get_center_of_mass_local: (&'static str, u32) = ("_get_center_of_mass_local", 3360562783u32);
    pub const get_principal_inertia_axes: (&'static str, u32) = ("_get_principal_inertia_axes", 2716978435u32);
    pub const get_inverse_mass: (&'static str, u32) = ("_get_inverse_mass", 1740695150u32);
    pub const get_inverse_inertia: (&'static str, u32) = ("_get_inverse_inertia", 3360562783u32);
    pub const get_inverse_inertia_tensor: (&'static str, u32) = ("_get_inverse_inertia_tensor", 2716978435u32);
    pub const set_linear_velocity: (&'static str, u32) = ("_set_linear_velocity", 3460891852u32);
    pub const get_linear_velocity: (&'static str, u32) = ("_get_linear_velocity", 3360562783u32);
    pub const set_angular_velocity: (&'static str, u32) = ("_set_angular_velocity", 3460891852u32);
    pub const get_angular_velocity: (&'static str, u32) = ("_get_angular_velocity", 3360562783u32);
    pub const set_transform: (&'static str, u32) = ("_set_transform", 2952846383u32);
    pub const get_transform: (&'static str, u32) = ("_get_transform", 3229777777u32);
    pub const get_velocity_at_local_position: (&'static str, u32) = ("_get_velocity_at_local_position", 192990374u32);
    pub const apply_central_impulse: (&'static str, u32) = ("_apply_central_impulse", 3460891852u32);
    pub const apply_impulse: (&'static str, u32) = ("_apply_impulse", 1714681797u32);
    pub const apply_torque_impulse: (&'static str, u32) = ("_apply_torque_impulse", 3460891852u32);
    pub const apply_central_force: (&'static str, u32) = ("_apply_central_force", 3460891852u32);
    pub const apply_force: (&'static str, u32) = ("_apply_force", 1714681797u32);
    pub const apply_torque: (&'static str, u32) = ("_apply_torque", 3460891852u32);
    pub const add_constant_central_force: (&'static str, u32) = ("_add_constant_central_force", 3460891852u32);
    pub const add_constant_force: (&'static str, u32) = ("_add_constant_force", 1714681797u32);
    pub const add_constant_torque: (&'static str, u32) = ("_add_constant_torque", 3460891852u32);
    pub const set_constant_force: (&'static str, u32) = ("_set_constant_force", 3460891852u32);
    pub const get_constant_force: (&'static str, u32) = ("_get_constant_force", 3360562783u32);
    pub const set_constant_torque: (&'static str, u32) = ("_set_constant_torque", 3460891852u32);
    pub const get_constant_torque: (&'static str, u32) = ("_get_constant_torque", 3360562783u32);
    pub const set_sleep_state: (&'static str, u32) = ("_set_sleep_state", 2586408642u32);
    pub const is_sleeping: (&'static str, u32) = ("_is_sleeping", 36873697u32);
    pub const set_collision_layer: (&'static str, u32) = ("_set_collision_layer", 1286410249u32);
    pub const get_collision_layer: (&'static str, u32) = ("_get_collision_layer", 3905245786u32);
    pub const set_collision_mask: (&'static str, u32) = ("_set_collision_mask", 1286410249u32);
    pub const get_collision_mask: (&'static str, u32) = ("_get_collision_mask", 3905245786u32);
    pub const get_contact_count: (&'static str, u32) = ("_get_contact_count", 3905245786u32);
    pub const get_contact_local_position: (&'static str, u32) = ("_get_contact_local_position", 711720468u32);
    pub const get_contact_local_normal: (&'static str, u32) = ("_get_contact_local_normal", 711720468u32);
    pub const get_contact_impulse: (&'static str, u32) = ("_get_contact_impulse", 711720468u32);
    pub const get_contact_local_shape: (&'static str, u32) = ("_get_contact_local_shape", 923996154u32);
    pub const get_contact_local_velocity_at_position: (&'static str, u32) = ("_get_contact_local_velocity_at_position", 711720468u32);
    pub const get_contact_collider: (&'static str, u32) = ("_get_contact_collider", 495598643u32);
    pub const get_contact_collider_position: (&'static str, u32) = ("_get_contact_collider_position", 711720468u32);
    pub const get_contact_collider_id: (&'static str, u32) = ("_get_contact_collider_id", 923996154u32);
    pub const get_contact_collider_object: (&'static str, u32) = ("_get_contact_collider_object", 3332903315u32);
    pub const get_contact_collider_shape: (&'static str, u32) = ("_get_contact_collider_shape", 923996154u32);
    pub const get_contact_collider_velocity_at_position: (&'static str, u32) = ("_get_contact_collider_velocity_at_position", 711720468u32);
    pub const get_step: (&'static str, u32) = ("_get_step", 1740695150u32);
    pub const integrate_forces: (&'static str, u32) = ("_integrate_forces", 3218959716u32);
    pub const get_space_state: (&'static str, u32) = ("_get_space_state", 2069328350u32);
    
}
pub mod PhysicsDirectSpaceState2D {
    pub use super::Object::*;
    
}
pub mod PhysicsDirectSpaceState2DExtension {
    pub use super::PhysicsDirectSpaceState2D::*;
    pub const intersect_ray_rawptr: (&'static str, u32) = ("_intersect_ray", 2840492092u32);
    pub const intersect_point_rawptr: (&'static str, u32) = ("_intersect_point", 522407812u32);
    pub const intersect_shape_rawptr: (&'static str, u32) = ("_intersect_shape", 1584897015u32);
    pub const cast_motion_rawptr: (&'static str, u32) = ("_cast_motion", 1410701151u32);
    pub const collide_shape_rawptr: (&'static str, u32) = ("_collide_shape", 871510130u32);
    pub const rest_info_rawptr: (&'static str, u32) = ("_rest_info", 772675997u32);
    
}
pub mod PhysicsDirectSpaceState3D {
    pub use super::Object::*;
    
}
pub mod PhysicsDirectSpaceState3DExtension {
    pub use super::PhysicsDirectSpaceState3D::*;
    pub const intersect_ray_rawptr: (&'static str, u32) = ("_intersect_ray", 2022529123u32);
    pub const intersect_point_rawptr: (&'static str, u32) = ("_intersect_point", 3378904092u32);
    pub const intersect_shape_rawptr: (&'static str, u32) = ("_intersect_shape", 728953575u32);
    pub const cast_motion_rawptr: (&'static str, u32) = ("_cast_motion", 2320624824u32);
    pub const collide_shape_rawptr: (&'static str, u32) = ("_collide_shape", 2320624824u32);
    pub const rest_info_rawptr: (&'static str, u32) = ("_rest_info", 856242757u32);
    pub const get_closest_point_to_object_volume: (&'static str, u32) = ("_get_closest_point_to_object_volume", 2056183332u32);
    
}
pub mod PhysicsMaterial {
    pub use super::Resource::*;
    
}
pub mod PhysicsPointQueryParameters2D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsPointQueryParameters3D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsRayQueryParameters2D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsRayQueryParameters3D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsServer2D {
    pub use super::Object::*;
    
}
pub mod PhysicsServer2DExtension {
    pub use super::PhysicsServer2D::*;
    pub const world_boundary_shape_create: (&'static str, u32) = ("_world_boundary_shape_create", 529393457u32);
    pub const separation_ray_shape_create: (&'static str, u32) = ("_separation_ray_shape_create", 529393457u32);
    pub const segment_shape_create: (&'static str, u32) = ("_segment_shape_create", 529393457u32);
    pub const circle_shape_create: (&'static str, u32) = ("_circle_shape_create", 529393457u32);
    pub const rectangle_shape_create: (&'static str, u32) = ("_rectangle_shape_create", 529393457u32);
    pub const capsule_shape_create: (&'static str, u32) = ("_capsule_shape_create", 529393457u32);
    pub const convex_polygon_shape_create: (&'static str, u32) = ("_convex_polygon_shape_create", 529393457u32);
    pub const concave_polygon_shape_create: (&'static str, u32) = ("_concave_polygon_shape_create", 529393457u32);
    pub const shape_set_data: (&'static str, u32) = ("_shape_set_data", 3175752987u32);
    pub const shape_set_custom_solver_bias: (&'static str, u32) = ("_shape_set_custom_solver_bias", 1794382983u32);
    pub const shape_get_type: (&'static str, u32) = ("_shape_get_type", 1240598777u32);
    pub const shape_get_data: (&'static str, u32) = ("_shape_get_data", 4171304767u32);
    pub const shape_get_custom_solver_bias: (&'static str, u32) = ("_shape_get_custom_solver_bias", 866169185u32);
    pub const shape_collide_rawptr: (&'static str, u32) = ("_shape_collide", 738864683u32);
    pub const space_create: (&'static str, u32) = ("_space_create", 529393457u32);
    pub const space_set_active: (&'static str, u32) = ("_space_set_active", 1265174801u32);
    pub const space_is_active: (&'static str, u32) = ("_space_is_active", 4155700596u32);
    pub const space_set_param: (&'static str, u32) = ("_space_set_param", 949194586u32);
    pub const space_get_param: (&'static str, u32) = ("_space_get_param", 874111783u32);
    pub const space_get_direct_state: (&'static str, u32) = ("_space_get_direct_state", 3160173886u32);
    pub const space_set_debug_contacts: (&'static str, u32) = ("_space_set_debug_contacts", 3411492887u32);
    pub const space_get_contacts: (&'static str, u32) = ("_space_get_contacts", 2222557395u32);
    pub const space_get_contact_count: (&'static str, u32) = ("_space_get_contact_count", 2198884583u32);
    pub const area_create: (&'static str, u32) = ("_area_create", 529393457u32);
    pub const area_set_space: (&'static str, u32) = ("_area_set_space", 395945892u32);
    pub const area_get_space: (&'static str, u32) = ("_area_get_space", 3814569979u32);
    pub const area_add_shape: (&'static str, u32) = ("_area_add_shape", 888317420u32);
    pub const area_set_shape: (&'static str, u32) = ("_area_set_shape", 2310537182u32);
    pub const area_set_shape_transform: (&'static str, u32) = ("_area_set_shape_transform", 736082694u32);
    pub const area_set_shape_disabled: (&'static str, u32) = ("_area_set_shape_disabled", 2658558584u32);
    pub const area_get_shape_count: (&'static str, u32) = ("_area_get_shape_count", 2198884583u32);
    pub const area_get_shape: (&'static str, u32) = ("_area_get_shape", 1066463050u32);
    pub const area_get_shape_transform: (&'static str, u32) = ("_area_get_shape_transform", 1324854622u32);
    pub const area_remove_shape: (&'static str, u32) = ("_area_remove_shape", 3411492887u32);
    pub const area_clear_shapes: (&'static str, u32) = ("_area_clear_shapes", 2722037293u32);
    pub const area_attach_object_instance_id: (&'static str, u32) = ("_area_attach_object_instance_id", 3411492887u32);
    pub const area_get_object_instance_id: (&'static str, u32) = ("_area_get_object_instance_id", 2198884583u32);
    pub const area_attach_canvas_instance_id: (&'static str, u32) = ("_area_attach_canvas_instance_id", 3411492887u32);
    pub const area_get_canvas_instance_id: (&'static str, u32) = ("_area_get_canvas_instance_id", 2198884583u32);
    pub const area_set_param: (&'static str, u32) = ("_area_set_param", 1257146028u32);
    pub const area_set_transform: (&'static str, u32) = ("_area_set_transform", 1246044741u32);
    pub const area_get_param: (&'static str, u32) = ("_area_get_param", 3047435120u32);
    pub const area_get_transform: (&'static str, u32) = ("_area_get_transform", 213527486u32);
    pub const area_set_collision_layer: (&'static str, u32) = ("_area_set_collision_layer", 3411492887u32);
    pub const area_get_collision_layer: (&'static str, u32) = ("_area_get_collision_layer", 2198884583u32);
    pub const area_set_collision_mask: (&'static str, u32) = ("_area_set_collision_mask", 3411492887u32);
    pub const area_get_collision_mask: (&'static str, u32) = ("_area_get_collision_mask", 2198884583u32);
    pub const area_set_monitorable: (&'static str, u32) = ("_area_set_monitorable", 1265174801u32);
    pub const area_set_pickable: (&'static str, u32) = ("_area_set_pickable", 1265174801u32);
    pub const area_set_monitor_callback: (&'static str, u32) = ("_area_set_monitor_callback", 3379118538u32);
    pub const area_set_area_monitor_callback: (&'static str, u32) = ("_area_set_area_monitor_callback", 3379118538u32);
    pub const body_create: (&'static str, u32) = ("_body_create", 529393457u32);
    pub const body_set_space: (&'static str, u32) = ("_body_set_space", 395945892u32);
    pub const body_get_space: (&'static str, u32) = ("_body_get_space", 3814569979u32);
    pub const body_set_mode: (&'static str, u32) = ("_body_set_mode", 1658067650u32);
    pub const body_get_mode: (&'static str, u32) = ("_body_get_mode", 3261702585u32);
    pub const body_add_shape: (&'static str, u32) = ("_body_add_shape", 888317420u32);
    pub const body_set_shape: (&'static str, u32) = ("_body_set_shape", 2310537182u32);
    pub const body_set_shape_transform: (&'static str, u32) = ("_body_set_shape_transform", 736082694u32);
    pub const body_get_shape_count: (&'static str, u32) = ("_body_get_shape_count", 2198884583u32);
    pub const body_get_shape: (&'static str, u32) = ("_body_get_shape", 1066463050u32);
    pub const body_get_shape_transform: (&'static str, u32) = ("_body_get_shape_transform", 1324854622u32);
    pub const body_set_shape_disabled: (&'static str, u32) = ("_body_set_shape_disabled", 2658558584u32);
    pub const body_set_shape_as_one_way_collision: (&'static str, u32) = ("_body_set_shape_as_one_way_collision", 2556489974u32);
    pub const body_remove_shape: (&'static str, u32) = ("_body_remove_shape", 3411492887u32);
    pub const body_clear_shapes: (&'static str, u32) = ("_body_clear_shapes", 2722037293u32);
    pub const body_attach_object_instance_id: (&'static str, u32) = ("_body_attach_object_instance_id", 3411492887u32);
    pub const body_get_object_instance_id: (&'static str, u32) = ("_body_get_object_instance_id", 2198884583u32);
    pub const body_attach_canvas_instance_id: (&'static str, u32) = ("_body_attach_canvas_instance_id", 3411492887u32);
    pub const body_get_canvas_instance_id: (&'static str, u32) = ("_body_get_canvas_instance_id", 2198884583u32);
    pub const body_set_continuous_collision_detection_mode: (&'static str, u32) = ("_body_set_continuous_collision_detection_mode", 1882257015u32);
    pub const body_get_continuous_collision_detection_mode: (&'static str, u32) = ("_body_get_continuous_collision_detection_mode", 2661282217u32);
    pub const body_set_collision_layer: (&'static str, u32) = ("_body_set_collision_layer", 3411492887u32);
    pub const body_get_collision_layer: (&'static str, u32) = ("_body_get_collision_layer", 2198884583u32);
    pub const body_set_collision_mask: (&'static str, u32) = ("_body_set_collision_mask", 3411492887u32);
    pub const body_get_collision_mask: (&'static str, u32) = ("_body_get_collision_mask", 2198884583u32);
    pub const body_set_collision_priority: (&'static str, u32) = ("_body_set_collision_priority", 1794382983u32);
    pub const body_get_collision_priority: (&'static str, u32) = ("_body_get_collision_priority", 866169185u32);
    pub const body_set_param: (&'static str, u32) = ("_body_set_param", 2715630609u32);
    pub const body_get_param: (&'static str, u32) = ("_body_get_param", 3208033526u32);
    pub const body_reset_mass_properties: (&'static str, u32) = ("_body_reset_mass_properties", 2722037293u32);
    pub const body_set_state: (&'static str, u32) = ("_body_set_state", 1706355209u32);
    pub const body_get_state: (&'static str, u32) = ("_body_get_state", 4036367961u32);
    pub const body_apply_central_impulse: (&'static str, u32) = ("_body_apply_central_impulse", 3201125042u32);
    pub const body_apply_torque_impulse: (&'static str, u32) = ("_body_apply_torque_impulse", 1794382983u32);
    pub const body_apply_impulse: (&'static str, u32) = ("_body_apply_impulse", 2762675110u32);
    pub const body_apply_central_force: (&'static str, u32) = ("_body_apply_central_force", 3201125042u32);
    pub const body_apply_force: (&'static str, u32) = ("_body_apply_force", 2762675110u32);
    pub const body_apply_torque: (&'static str, u32) = ("_body_apply_torque", 1794382983u32);
    pub const body_add_constant_central_force: (&'static str, u32) = ("_body_add_constant_central_force", 3201125042u32);
    pub const body_add_constant_force: (&'static str, u32) = ("_body_add_constant_force", 2762675110u32);
    pub const body_add_constant_torque: (&'static str, u32) = ("_body_add_constant_torque", 1794382983u32);
    pub const body_set_constant_force: (&'static str, u32) = ("_body_set_constant_force", 3201125042u32);
    pub const body_get_constant_force: (&'static str, u32) = ("_body_get_constant_force", 2440833711u32);
    pub const body_set_constant_torque: (&'static str, u32) = ("_body_set_constant_torque", 1794382983u32);
    pub const body_get_constant_torque: (&'static str, u32) = ("_body_get_constant_torque", 866169185u32);
    pub const body_set_axis_velocity: (&'static str, u32) = ("_body_set_axis_velocity", 3201125042u32);
    pub const body_add_collision_exception: (&'static str, u32) = ("_body_add_collision_exception", 395945892u32);
    pub const body_remove_collision_exception: (&'static str, u32) = ("_body_remove_collision_exception", 395945892u32);
    pub const body_get_collision_exceptions: (&'static str, u32) = ("_body_get_collision_exceptions", 2684255073u32);
    pub const body_set_max_contacts_reported: (&'static str, u32) = ("_body_set_max_contacts_reported", 3411492887u32);
    pub const body_get_max_contacts_reported: (&'static str, u32) = ("_body_get_max_contacts_reported", 2198884583u32);
    pub const body_set_contacts_reported_depth_threshold: (&'static str, u32) = ("_body_set_contacts_reported_depth_threshold", 1794382983u32);
    pub const body_get_contacts_reported_depth_threshold: (&'static str, u32) = ("_body_get_contacts_reported_depth_threshold", 866169185u32);
    pub const body_set_omit_force_integration: (&'static str, u32) = ("_body_set_omit_force_integration", 1265174801u32);
    pub const body_is_omitting_force_integration: (&'static str, u32) = ("_body_is_omitting_force_integration", 4155700596u32);
    pub const body_set_state_sync_callback: (&'static str, u32) = ("_body_set_state_sync_callback", 3379118538u32);
    pub const body_set_force_integration_callback: (&'static str, u32) = ("_body_set_force_integration_callback", 2828036238u32);
    pub const body_collide_shape_rawptr: (&'static str, u32) = ("_body_collide_shape", 2131476465u32);
    pub const body_set_pickable: (&'static str, u32) = ("_body_set_pickable", 1265174801u32);
    pub const body_get_direct_state: (&'static str, u32) = ("_body_get_direct_state", 1191931871u32);
    pub const body_test_motion_rawptr: (&'static str, u32) = ("_body_test_motion", 104979818u32);
    pub const joint_create: (&'static str, u32) = ("_joint_create", 529393457u32);
    pub const joint_clear: (&'static str, u32) = ("_joint_clear", 2722037293u32);
    pub const joint_set_param: (&'static str, u32) = ("_joint_set_param", 3972556514u32);
    pub const joint_get_param: (&'static str, u32) = ("_joint_get_param", 4016448949u32);
    pub const joint_disable_collisions_between_bodies: (&'static str, u32) = ("_joint_disable_collisions_between_bodies", 1265174801u32);
    pub const joint_is_disabled_collisions_between_bodies: (&'static str, u32) = ("_joint_is_disabled_collisions_between_bodies", 4155700596u32);
    pub const joint_make_pin: (&'static str, u32) = ("_joint_make_pin", 2607799521u32);
    pub const joint_make_groove: (&'static str, u32) = ("_joint_make_groove", 438649616u32);
    pub const joint_make_damped_spring: (&'static str, u32) = ("_joint_make_damped_spring", 1276049561u32);
    pub const pin_joint_set_flag: (&'static str, u32) = ("_pin_joint_set_flag", 3520002352u32);
    pub const pin_joint_get_flag: (&'static str, u32) = ("_pin_joint_get_flag", 2647867364u32);
    pub const pin_joint_set_param: (&'static str, u32) = ("_pin_joint_set_param", 550574241u32);
    pub const pin_joint_get_param: (&'static str, u32) = ("_pin_joint_get_param", 348281383u32);
    pub const damped_spring_joint_set_param: (&'static str, u32) = ("_damped_spring_joint_set_param", 220564071u32);
    pub const damped_spring_joint_get_param: (&'static str, u32) = ("_damped_spring_joint_get_param", 2075871277u32);
    pub const joint_get_type: (&'static str, u32) = ("_joint_get_type", 4262502231u32);
    pub const free_rid: (&'static str, u32) = ("_free_rid", 2722037293u32);
    pub const set_active: (&'static str, u32) = ("_set_active", 2586408642u32);
    pub const init_ext: (&'static str, u32) = ("_init", 3218959716u32);
    pub const step: (&'static str, u32) = ("_step", 373806689u32);
    pub const sync: (&'static str, u32) = ("_sync", 3218959716u32);
    pub const flush_queries: (&'static str, u32) = ("_flush_queries", 3218959716u32);
    pub const end_sync: (&'static str, u32) = ("_end_sync", 3218959716u32);
    pub const finish: (&'static str, u32) = ("_finish", 3218959716u32);
    pub const is_flushing_queries: (&'static str, u32) = ("_is_flushing_queries", 36873697u32);
    pub const get_process_info: (&'static str, u32) = ("_get_process_info", 576496006u32);
    
}
pub mod PhysicsServer2DManager {
    pub use super::Object::*;
    
}
pub mod PhysicsServer3D {
    pub use super::Object::*;
    
}
pub mod PhysicsServer3DExtension {
    pub use super::PhysicsServer3D::*;
    pub const world_boundary_shape_create: (&'static str, u32) = ("_world_boundary_shape_create", 529393457u32);
    pub const separation_ray_shape_create: (&'static str, u32) = ("_separation_ray_shape_create", 529393457u32);
    pub const sphere_shape_create: (&'static str, u32) = ("_sphere_shape_create", 529393457u32);
    pub const box_shape_create: (&'static str, u32) = ("_box_shape_create", 529393457u32);
    pub const capsule_shape_create: (&'static str, u32) = ("_capsule_shape_create", 529393457u32);
    pub const cylinder_shape_create: (&'static str, u32) = ("_cylinder_shape_create", 529393457u32);
    pub const convex_polygon_shape_create: (&'static str, u32) = ("_convex_polygon_shape_create", 529393457u32);
    pub const concave_polygon_shape_create: (&'static str, u32) = ("_concave_polygon_shape_create", 529393457u32);
    pub const heightmap_shape_create: (&'static str, u32) = ("_heightmap_shape_create", 529393457u32);
    pub const custom_shape_create: (&'static str, u32) = ("_custom_shape_create", 529393457u32);
    pub const shape_set_data: (&'static str, u32) = ("_shape_set_data", 3175752987u32);
    pub const shape_set_custom_solver_bias: (&'static str, u32) = ("_shape_set_custom_solver_bias", 1794382983u32);
    pub const shape_set_margin: (&'static str, u32) = ("_shape_set_margin", 1794382983u32);
    pub const shape_get_margin: (&'static str, u32) = ("_shape_get_margin", 866169185u32);
    pub const shape_get_type: (&'static str, u32) = ("_shape_get_type", 3418923367u32);
    pub const shape_get_data: (&'static str, u32) = ("_shape_get_data", 4171304767u32);
    pub const shape_get_custom_solver_bias: (&'static str, u32) = ("_shape_get_custom_solver_bias", 866169185u32);
    pub const space_create: (&'static str, u32) = ("_space_create", 529393457u32);
    pub const space_set_active: (&'static str, u32) = ("_space_set_active", 1265174801u32);
    pub const space_is_active: (&'static str, u32) = ("_space_is_active", 4155700596u32);
    pub const space_set_param: (&'static str, u32) = ("_space_set_param", 2406017470u32);
    pub const space_get_param: (&'static str, u32) = ("_space_get_param", 1523206731u32);
    pub const space_get_direct_state: (&'static str, u32) = ("_space_get_direct_state", 2048616813u32);
    pub const space_set_debug_contacts: (&'static str, u32) = ("_space_set_debug_contacts", 3411492887u32);
    pub const space_get_contacts: (&'static str, u32) = ("_space_get_contacts", 808965560u32);
    pub const space_get_contact_count: (&'static str, u32) = ("_space_get_contact_count", 2198884583u32);
    pub const area_create: (&'static str, u32) = ("_area_create", 529393457u32);
    pub const area_set_space: (&'static str, u32) = ("_area_set_space", 395945892u32);
    pub const area_get_space: (&'static str, u32) = ("_area_get_space", 3814569979u32);
    pub const area_add_shape: (&'static str, u32) = ("_area_add_shape", 2153848567u32);
    pub const area_set_shape: (&'static str, u32) = ("_area_set_shape", 2310537182u32);
    pub const area_set_shape_transform: (&'static str, u32) = ("_area_set_shape_transform", 675327471u32);
    pub const area_set_shape_disabled: (&'static str, u32) = ("_area_set_shape_disabled", 2658558584u32);
    pub const area_get_shape_count: (&'static str, u32) = ("_area_get_shape_count", 2198884583u32);
    pub const area_get_shape: (&'static str, u32) = ("_area_get_shape", 1066463050u32);
    pub const area_get_shape_transform: (&'static str, u32) = ("_area_get_shape_transform", 1050775521u32);
    pub const area_remove_shape: (&'static str, u32) = ("_area_remove_shape", 3411492887u32);
    pub const area_clear_shapes: (&'static str, u32) = ("_area_clear_shapes", 2722037293u32);
    pub const area_attach_object_instance_id: (&'static str, u32) = ("_area_attach_object_instance_id", 3411492887u32);
    pub const area_get_object_instance_id: (&'static str, u32) = ("_area_get_object_instance_id", 2198884583u32);
    pub const area_set_param: (&'static str, u32) = ("_area_set_param", 2980114638u32);
    pub const area_set_transform: (&'static str, u32) = ("_area_set_transform", 3935195649u32);
    pub const area_get_param: (&'static str, u32) = ("_area_get_param", 890056067u32);
    pub const area_get_transform: (&'static str, u32) = ("_area_get_transform", 1128465797u32);
    pub const area_set_collision_layer: (&'static str, u32) = ("_area_set_collision_layer", 3411492887u32);
    pub const area_get_collision_layer: (&'static str, u32) = ("_area_get_collision_layer", 2198884583u32);
    pub const area_set_collision_mask: (&'static str, u32) = ("_area_set_collision_mask", 3411492887u32);
    pub const area_get_collision_mask: (&'static str, u32) = ("_area_get_collision_mask", 2198884583u32);
    pub const area_set_monitorable: (&'static str, u32) = ("_area_set_monitorable", 1265174801u32);
    pub const area_set_ray_pickable: (&'static str, u32) = ("_area_set_ray_pickable", 1265174801u32);
    pub const area_set_monitor_callback: (&'static str, u32) = ("_area_set_monitor_callback", 3379118538u32);
    pub const area_set_area_monitor_callback: (&'static str, u32) = ("_area_set_area_monitor_callback", 3379118538u32);
    pub const body_create: (&'static str, u32) = ("_body_create", 529393457u32);
    pub const body_set_space: (&'static str, u32) = ("_body_set_space", 395945892u32);
    pub const body_get_space: (&'static str, u32) = ("_body_get_space", 3814569979u32);
    pub const body_set_mode: (&'static str, u32) = ("_body_set_mode", 606803466u32);
    pub const body_get_mode: (&'static str, u32) = ("_body_get_mode", 2488819728u32);
    pub const body_add_shape: (&'static str, u32) = ("_body_add_shape", 2153848567u32);
    pub const body_set_shape: (&'static str, u32) = ("_body_set_shape", 2310537182u32);
    pub const body_set_shape_transform: (&'static str, u32) = ("_body_set_shape_transform", 675327471u32);
    pub const body_set_shape_disabled: (&'static str, u32) = ("_body_set_shape_disabled", 2658558584u32);
    pub const body_get_shape_count: (&'static str, u32) = ("_body_get_shape_count", 2198884583u32);
    pub const body_get_shape: (&'static str, u32) = ("_body_get_shape", 1066463050u32);
    pub const body_get_shape_transform: (&'static str, u32) = ("_body_get_shape_transform", 1050775521u32);
    pub const body_remove_shape: (&'static str, u32) = ("_body_remove_shape", 3411492887u32);
    pub const body_clear_shapes: (&'static str, u32) = ("_body_clear_shapes", 2722037293u32);
    pub const body_attach_object_instance_id: (&'static str, u32) = ("_body_attach_object_instance_id", 3411492887u32);
    pub const body_get_object_instance_id: (&'static str, u32) = ("_body_get_object_instance_id", 2198884583u32);
    pub const body_set_enable_continuous_collision_detection: (&'static str, u32) = ("_body_set_enable_continuous_collision_detection", 1265174801u32);
    pub const body_is_continuous_collision_detection_enabled: (&'static str, u32) = ("_body_is_continuous_collision_detection_enabled", 4155700596u32);
    pub const body_set_collision_layer: (&'static str, u32) = ("_body_set_collision_layer", 3411492887u32);
    pub const body_get_collision_layer: (&'static str, u32) = ("_body_get_collision_layer", 2198884583u32);
    pub const body_set_collision_mask: (&'static str, u32) = ("_body_set_collision_mask", 3411492887u32);
    pub const body_get_collision_mask: (&'static str, u32) = ("_body_get_collision_mask", 2198884583u32);
    pub const body_set_collision_priority: (&'static str, u32) = ("_body_set_collision_priority", 1794382983u32);
    pub const body_get_collision_priority: (&'static str, u32) = ("_body_get_collision_priority", 866169185u32);
    pub const body_set_user_flags: (&'static str, u32) = ("_body_set_user_flags", 3411492887u32);
    pub const body_get_user_flags: (&'static str, u32) = ("_body_get_user_flags", 2198884583u32);
    pub const body_set_param: (&'static str, u32) = ("_body_set_param", 910941953u32);
    pub const body_get_param: (&'static str, u32) = ("_body_get_param", 3385027841u32);
    pub const body_reset_mass_properties: (&'static str, u32) = ("_body_reset_mass_properties", 2722037293u32);
    pub const body_set_state: (&'static str, u32) = ("_body_set_state", 599977762u32);
    pub const body_get_state: (&'static str, u32) = ("_body_get_state", 1850449534u32);
    pub const body_apply_central_impulse: (&'static str, u32) = ("_body_apply_central_impulse", 3227306858u32);
    pub const body_apply_impulse: (&'static str, u32) = ("_body_apply_impulse", 3214966418u32);
    pub const body_apply_torque_impulse: (&'static str, u32) = ("_body_apply_torque_impulse", 3227306858u32);
    pub const body_apply_central_force: (&'static str, u32) = ("_body_apply_central_force", 3227306858u32);
    pub const body_apply_force: (&'static str, u32) = ("_body_apply_force", 3214966418u32);
    pub const body_apply_torque: (&'static str, u32) = ("_body_apply_torque", 3227306858u32);
    pub const body_add_constant_central_force: (&'static str, u32) = ("_body_add_constant_central_force", 3227306858u32);
    pub const body_add_constant_force: (&'static str, u32) = ("_body_add_constant_force", 3214966418u32);
    pub const body_add_constant_torque: (&'static str, u32) = ("_body_add_constant_torque", 3227306858u32);
    pub const body_set_constant_force: (&'static str, u32) = ("_body_set_constant_force", 3227306858u32);
    pub const body_get_constant_force: (&'static str, u32) = ("_body_get_constant_force", 531438156u32);
    pub const body_set_constant_torque: (&'static str, u32) = ("_body_set_constant_torque", 3227306858u32);
    pub const body_get_constant_torque: (&'static str, u32) = ("_body_get_constant_torque", 531438156u32);
    pub const body_set_axis_velocity: (&'static str, u32) = ("_body_set_axis_velocity", 3227306858u32);
    pub const body_set_axis_lock: (&'static str, u32) = ("_body_set_axis_lock", 2020836892u32);
    pub const body_is_axis_locked: (&'static str, u32) = ("_body_is_axis_locked", 587853580u32);
    pub const body_add_collision_exception: (&'static str, u32) = ("_body_add_collision_exception", 395945892u32);
    pub const body_remove_collision_exception: (&'static str, u32) = ("_body_remove_collision_exception", 395945892u32);
    pub const body_get_collision_exceptions: (&'static str, u32) = ("_body_get_collision_exceptions", 2684255073u32);
    pub const body_set_max_contacts_reported: (&'static str, u32) = ("_body_set_max_contacts_reported", 3411492887u32);
    pub const body_get_max_contacts_reported: (&'static str, u32) = ("_body_get_max_contacts_reported", 2198884583u32);
    pub const body_set_contacts_reported_depth_threshold: (&'static str, u32) = ("_body_set_contacts_reported_depth_threshold", 1794382983u32);
    pub const body_get_contacts_reported_depth_threshold: (&'static str, u32) = ("_body_get_contacts_reported_depth_threshold", 866169185u32);
    pub const body_set_omit_force_integration: (&'static str, u32) = ("_body_set_omit_force_integration", 1265174801u32);
    pub const body_is_omitting_force_integration: (&'static str, u32) = ("_body_is_omitting_force_integration", 4155700596u32);
    pub const body_set_state_sync_callback: (&'static str, u32) = ("_body_set_state_sync_callback", 3379118538u32);
    pub const body_set_force_integration_callback: (&'static str, u32) = ("_body_set_force_integration_callback", 2828036238u32);
    pub const body_set_ray_pickable: (&'static str, u32) = ("_body_set_ray_pickable", 1265174801u32);
    pub const body_test_motion_rawptr: (&'static str, u32) = ("_body_test_motion", 3627463434u32);
    pub const body_get_direct_state: (&'static str, u32) = ("_body_get_direct_state", 3029727957u32);
    pub const soft_body_create: (&'static str, u32) = ("_soft_body_create", 529393457u32);
    pub const soft_body_update_rendering_server: (&'static str, u32) = ("_soft_body_update_rendering_server", 2218179753u32);
    pub const soft_body_set_space: (&'static str, u32) = ("_soft_body_set_space", 395945892u32);
    pub const soft_body_get_space: (&'static str, u32) = ("_soft_body_get_space", 3814569979u32);
    pub const soft_body_set_ray_pickable: (&'static str, u32) = ("_soft_body_set_ray_pickable", 1265174801u32);
    pub const soft_body_set_collision_layer: (&'static str, u32) = ("_soft_body_set_collision_layer", 3411492887u32);
    pub const soft_body_get_collision_layer: (&'static str, u32) = ("_soft_body_get_collision_layer", 2198884583u32);
    pub const soft_body_set_collision_mask: (&'static str, u32) = ("_soft_body_set_collision_mask", 3411492887u32);
    pub const soft_body_get_collision_mask: (&'static str, u32) = ("_soft_body_get_collision_mask", 2198884583u32);
    pub const soft_body_add_collision_exception: (&'static str, u32) = ("_soft_body_add_collision_exception", 395945892u32);
    pub const soft_body_remove_collision_exception: (&'static str, u32) = ("_soft_body_remove_collision_exception", 395945892u32);
    pub const soft_body_get_collision_exceptions: (&'static str, u32) = ("_soft_body_get_collision_exceptions", 2684255073u32);
    pub const soft_body_set_state: (&'static str, u32) = ("_soft_body_set_state", 599977762u32);
    pub const soft_body_get_state: (&'static str, u32) = ("_soft_body_get_state", 1850449534u32);
    pub const soft_body_set_transform: (&'static str, u32) = ("_soft_body_set_transform", 3935195649u32);
    pub const soft_body_set_simulation_precision: (&'static str, u32) = ("_soft_body_set_simulation_precision", 3411492887u32);
    pub const soft_body_get_simulation_precision: (&'static str, u32) = ("_soft_body_get_simulation_precision", 2198884583u32);
    pub const soft_body_set_total_mass: (&'static str, u32) = ("_soft_body_set_total_mass", 1794382983u32);
    pub const soft_body_get_total_mass: (&'static str, u32) = ("_soft_body_get_total_mass", 866169185u32);
    pub const soft_body_set_linear_stiffness: (&'static str, u32) = ("_soft_body_set_linear_stiffness", 1794382983u32);
    pub const soft_body_get_linear_stiffness: (&'static str, u32) = ("_soft_body_get_linear_stiffness", 866169185u32);
    pub const soft_body_set_shrinking_factor: (&'static str, u32) = ("_soft_body_set_shrinking_factor", 1794382983u32);
    pub const soft_body_get_shrinking_factor: (&'static str, u32) = ("_soft_body_get_shrinking_factor", 866169185u32);
    pub const soft_body_set_pressure_coefficient: (&'static str, u32) = ("_soft_body_set_pressure_coefficient", 1794382983u32);
    pub const soft_body_get_pressure_coefficient: (&'static str, u32) = ("_soft_body_get_pressure_coefficient", 866169185u32);
    pub const soft_body_set_damping_coefficient: (&'static str, u32) = ("_soft_body_set_damping_coefficient", 1794382983u32);
    pub const soft_body_get_damping_coefficient: (&'static str, u32) = ("_soft_body_get_damping_coefficient", 866169185u32);
    pub const soft_body_set_drag_coefficient: (&'static str, u32) = ("_soft_body_set_drag_coefficient", 1794382983u32);
    pub const soft_body_get_drag_coefficient: (&'static str, u32) = ("_soft_body_get_drag_coefficient", 866169185u32);
    pub const soft_body_set_mesh: (&'static str, u32) = ("_soft_body_set_mesh", 395945892u32);
    pub const soft_body_get_bounds: (&'static str, u32) = ("_soft_body_get_bounds", 974181306u32);
    pub const soft_body_move_point: (&'static str, u32) = ("_soft_body_move_point", 831953689u32);
    pub const soft_body_get_point_global_position: (&'static str, u32) = ("_soft_body_get_point_global_position", 3440143363u32);
    pub const soft_body_remove_all_pinned_points: (&'static str, u32) = ("_soft_body_remove_all_pinned_points", 2722037293u32);
    pub const soft_body_pin_point: (&'static str, u32) = ("_soft_body_pin_point", 2658558584u32);
    pub const soft_body_is_point_pinned: (&'static str, u32) = ("_soft_body_is_point_pinned", 3120086654u32);
    pub const soft_body_apply_point_impulse: (&'static str, u32) = ("_soft_body_apply_point_impulse", 831953689u32);
    pub const soft_body_apply_point_force: (&'static str, u32) = ("_soft_body_apply_point_force", 831953689u32);
    pub const soft_body_apply_central_impulse: (&'static str, u32) = ("_soft_body_apply_central_impulse", 3227306858u32);
    pub const soft_body_apply_central_force: (&'static str, u32) = ("_soft_body_apply_central_force", 3227306858u32);
    pub const joint_create: (&'static str, u32) = ("_joint_create", 529393457u32);
    pub const joint_clear: (&'static str, u32) = ("_joint_clear", 2722037293u32);
    pub const joint_make_pin: (&'static str, u32) = ("_joint_make_pin", 4280171926u32);
    pub const pin_joint_set_param: (&'static str, u32) = ("_pin_joint_set_param", 810685294u32);
    pub const pin_joint_get_param: (&'static str, u32) = ("_pin_joint_get_param", 2817972347u32);
    pub const pin_joint_set_local_a: (&'static str, u32) = ("_pin_joint_set_local_a", 3227306858u32);
    pub const pin_joint_get_local_a: (&'static str, u32) = ("_pin_joint_get_local_a", 531438156u32);
    pub const pin_joint_set_local_b: (&'static str, u32) = ("_pin_joint_set_local_b", 3227306858u32);
    pub const pin_joint_get_local_b: (&'static str, u32) = ("_pin_joint_get_local_b", 531438156u32);
    pub const joint_make_hinge: (&'static str, u32) = ("_joint_make_hinge", 1684107643u32);
    pub const joint_make_hinge_simple: (&'static str, u32) = ("_joint_make_hinge_simple", 4069547571u32);
    pub const hinge_joint_set_param: (&'static str, u32) = ("_hinge_joint_set_param", 3165502333u32);
    pub const hinge_joint_get_param: (&'static str, u32) = ("_hinge_joint_get_param", 2129207581u32);
    pub const hinge_joint_set_flag: (&'static str, u32) = ("_hinge_joint_set_flag", 1601626188u32);
    pub const hinge_joint_get_flag: (&'static str, u32) = ("_hinge_joint_get_flag", 4165147865u32);
    pub const joint_make_slider: (&'static str, u32) = ("_joint_make_slider", 1684107643u32);
    pub const slider_joint_set_param: (&'static str, u32) = ("_slider_joint_set_param", 2264833593u32);
    pub const slider_joint_get_param: (&'static str, u32) = ("_slider_joint_get_param", 3498644957u32);
    pub const joint_make_cone_twist: (&'static str, u32) = ("_joint_make_cone_twist", 1684107643u32);
    pub const cone_twist_joint_set_param: (&'static str, u32) = ("_cone_twist_joint_set_param", 808587618u32);
    pub const cone_twist_joint_get_param: (&'static str, u32) = ("_cone_twist_joint_get_param", 1134789658u32);
    pub const joint_make_generic_6dof: (&'static str, u32) = ("_joint_make_generic_6dof", 1684107643u32);
    pub const generic_6dof_joint_set_param: (&'static str, u32) = ("_generic_6dof_joint_set_param", 2600081391u32);
    pub const generic_6dof_joint_get_param: (&'static str, u32) = ("_generic_6dof_joint_get_param", 467122058u32);
    pub const generic_6dof_joint_set_flag: (&'static str, u32) = ("_generic_6dof_joint_set_flag", 3570926903u32);
    pub const generic_6dof_joint_get_flag: (&'static str, u32) = ("_generic_6dof_joint_get_flag", 4158090196u32);
    pub const joint_get_type: (&'static str, u32) = ("_joint_get_type", 4290791900u32);
    pub const joint_set_solver_priority: (&'static str, u32) = ("_joint_set_solver_priority", 3411492887u32);
    pub const joint_get_solver_priority: (&'static str, u32) = ("_joint_get_solver_priority", 2198884583u32);
    pub const joint_disable_collisions_between_bodies: (&'static str, u32) = ("_joint_disable_collisions_between_bodies", 1265174801u32);
    pub const joint_is_disabled_collisions_between_bodies: (&'static str, u32) = ("_joint_is_disabled_collisions_between_bodies", 4155700596u32);
    pub const free_rid: (&'static str, u32) = ("_free_rid", 2722037293u32);
    pub const set_active: (&'static str, u32) = ("_set_active", 2586408642u32);
    pub const init_ext: (&'static str, u32) = ("_init", 3218959716u32);
    pub const step: (&'static str, u32) = ("_step", 373806689u32);
    pub const sync: (&'static str, u32) = ("_sync", 3218959716u32);
    pub const flush_queries: (&'static str, u32) = ("_flush_queries", 3218959716u32);
    pub const end_sync: (&'static str, u32) = ("_end_sync", 3218959716u32);
    pub const finish: (&'static str, u32) = ("_finish", 3218959716u32);
    pub const is_flushing_queries: (&'static str, u32) = ("_is_flushing_queries", 36873697u32);
    pub const get_process_info: (&'static str, u32) = ("_get_process_info", 1332958745u32);
    
}
pub mod PhysicsServer3DManager {
    pub use super::Object::*;
    
}
pub mod PhysicsServer3DRenderingServerHandler {
    pub use super::Object::*;
    pub const set_vertex: (&'static str, u32) = ("_set_vertex", 1530502735u32);
    pub const set_normal: (&'static str, u32) = ("_set_normal", 1530502735u32);
    pub const set_aabb: (&'static str, u32) = ("_set_aabb", 259215842u32);
    
}
pub mod PhysicsShapeQueryParameters2D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsShapeQueryParameters3D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsTestMotionParameters2D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsTestMotionParameters3D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsTestMotionResult2D {
    pub use super::RefCounted::*;
    
}
pub mod PhysicsTestMotionResult3D {
    pub use super::RefCounted::*;
    
}
pub mod PinJoint2D {
    pub use super::Joint2D::*;
    
}
pub mod PinJoint3D {
    pub use super::Joint3D::*;
    
}
pub mod PlaceholderCubemap {
    pub use super::PlaceholderTextureLayered::*;
    
}
pub mod PlaceholderCubemapArray {
    pub use super::PlaceholderTextureLayered::*;
    
}
pub mod PlaceholderMaterial {
    pub use super::Material::*;
    
}
pub mod PlaceholderMesh {
    pub use super::Mesh::*;
    
}
pub mod PlaceholderTexture2D {
    pub use super::Texture2D::*;
    
}
pub mod PlaceholderTexture2DArray {
    pub use super::PlaceholderTextureLayered::*;
    
}
pub mod PlaceholderTexture3D {
    pub use super::Texture3D::*;
    
}
pub mod PlaceholderTextureLayered {
    pub use super::TextureLayered::*;
    
}
pub mod PlaneMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod PointLight2D {
    pub use super::Light2D::*;
    
}
pub mod PointMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod Polygon2D {
    pub use super::Node2D::*;
    
}
pub mod PolygonOccluder3D {
    pub use super::Occluder3D::*;
    
}
pub mod PolygonPathFinder {
    pub use super::Resource::*;
    
}
pub mod Popup {
    pub use super::Window::*;
    
}
pub mod PopupMenu {
    pub use super::Popup::*;
    
}
pub mod PopupPanel {
    pub use super::Popup::*;
    
}
pub mod PortableCompressedTexture2D {
    pub use super::Texture2D::*;
    
}
pub mod PrimitiveMesh {
    pub use super::Mesh::*;
    pub const create_mesh_array: (&'static str, u32) = ("_create_mesh_array", 3995934104u32);
    
}
pub mod PrismMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod ProceduralSkyMaterial {
    pub use super::Material::*;
    
}
pub mod ProgressBar {
    pub use super::Range::*;
    
}
pub mod ProjectSettings {
    pub use super::Object::*;
    
}
pub mod PropertyTweener {
    pub use super::Tweener::*;
    
}
pub mod QuadMesh {
    pub use super::PlaneMesh::*;
    
}
pub mod QuadOccluder3D {
    pub use super::Occluder3D::*;
    
}
pub mod RdAttachmentFormat {
    pub use super::RefCounted::*;
    
}
pub mod RdFramebufferPass {
    pub use super::RefCounted::*;
    
}
pub mod RdPipelineColorBlendState {
    pub use super::RefCounted::*;
    
}
pub mod RdPipelineColorBlendStateAttachment {
    pub use super::RefCounted::*;
    
}
pub mod RdPipelineDepthStencilState {
    pub use super::RefCounted::*;
    
}
pub mod RdPipelineMultisampleState {
    pub use super::RefCounted::*;
    
}
pub mod RdPipelineRasterizationState {
    pub use super::RefCounted::*;
    
}
pub mod RdPipelineSpecializationConstant {
    pub use super::RefCounted::*;
    
}
pub mod RdSamplerState {
    pub use super::RefCounted::*;
    
}
pub mod RdShaderFile {
    pub use super::Resource::*;
    
}
pub mod RdShaderSpirv {
    pub use super::Resource::*;
    
}
pub mod RdShaderSource {
    pub use super::RefCounted::*;
    
}
pub mod RdTextureFormat {
    pub use super::RefCounted::*;
    
}
pub mod RdTextureView {
    pub use super::RefCounted::*;
    
}
pub mod RdUniform {
    pub use super::RefCounted::*;
    
}
pub mod RdVertexAttribute {
    pub use super::RefCounted::*;
    
}
pub mod RandomNumberGenerator {
    pub use super::RefCounted::*;
    
}
pub mod Range {
    pub use super::Control::*;
    pub const value_changed: (&'static str, u32) = ("_value_changed", 373806689u32);
    
}
pub mod RayCast2D {
    pub use super::Node2D::*;
    
}
pub mod RayCast3D {
    pub use super::Node3D::*;
    
}
pub mod RectangleShape2D {
    pub use super::Shape2D::*;
    
}
pub mod RefCounted {
    pub use super::Object::*;
    
}
pub mod ReferenceRect {
    pub use super::Control::*;
    
}
pub mod ReflectionProbe {
    pub use super::VisualInstance3D::*;
    
}
pub mod RegEx {
    pub use super::RefCounted::*;
    
}
pub mod RegExMatch {
    pub use super::RefCounted::*;
    
}
pub mod RemoteTransform2D {
    pub use super::Node2D::*;
    
}
pub mod RemoteTransform3D {
    pub use super::Node3D::*;
    
}
pub mod RenderData {
    pub use super::Object::*;
    
}
pub mod RenderDataExtension {
    pub use super::RenderData::*;
    pub const get_render_scene_buffers: (&'static str, u32) = ("_get_render_scene_buffers", 2793216201u32);
    pub const get_render_scene_data: (&'static str, u32) = ("_get_render_scene_data", 1288715698u32);
    pub const get_environment: (&'static str, u32) = ("_get_environment", 2944877500u32);
    pub const get_camera_attributes: (&'static str, u32) = ("_get_camera_attributes", 2944877500u32);
    
}
pub mod RenderDataRd {
    pub use super::RenderData::*;
    
}
pub mod RenderSceneBuffers {
    pub use super::RefCounted::*;
    
}
pub mod RenderSceneBuffersConfiguration {
    pub use super::RefCounted::*;
    
}
pub mod RenderSceneBuffersExtension {
    pub use super::RenderSceneBuffers::*;
    pub const configure: (&'static str, u32) = ("_configure", 3072623270u32);
    pub const set_fsr_sharpness: (&'static str, u32) = ("_set_fsr_sharpness", 373806689u32);
    pub const set_texture_mipmap_bias: (&'static str, u32) = ("_set_texture_mipmap_bias", 373806689u32);
    pub const set_anisotropic_filtering_level: (&'static str, u32) = ("_set_anisotropic_filtering_level", 1286410249u32);
    pub const set_use_debanding: (&'static str, u32) = ("_set_use_debanding", 2586408642u32);
    
}
pub mod RenderSceneBuffersRd {
    pub use super::RenderSceneBuffers::*;
    
}
pub mod RenderSceneData {
    pub use super::Object::*;
    
}
pub mod RenderSceneDataExtension {
    pub use super::RenderSceneData::*;
    pub const get_cam_transform: (&'static str, u32) = ("_get_cam_transform", 3229777777u32);
    pub const get_cam_projection: (&'static str, u32) = ("_get_cam_projection", 2910717950u32);
    pub const get_view_count: (&'static str, u32) = ("_get_view_count", 3905245786u32);
    pub const get_view_eye_offset: (&'static str, u32) = ("_get_view_eye_offset", 711720468u32);
    pub const get_view_projection: (&'static str, u32) = ("_get_view_projection", 3179846605u32);
    pub const get_uniform_buffer: (&'static str, u32) = ("_get_uniform_buffer", 2944877500u32);
    
}
pub mod RenderSceneDataRd {
    pub use super::RenderSceneData::*;
    
}
pub mod RenderingDevice {
    pub use super::Object::*;
    
}
pub mod RenderingServer {
    pub use super::Object::*;
    
}
pub mod Resource {
    pub use super::RefCounted::*;
    pub const setup_local_to_scene: (&'static str, u32) = ("_setup_local_to_scene", 3218959716u32);
    pub const get_rid: (&'static str, u32) = ("_get_rid", 2944877500u32);
    pub const reset_state: (&'static str, u32) = ("_reset_state", 3218959716u32);
    pub const set_path_cache: (&'static str, u32) = ("_set_path_cache", 3089850668u32);
    
}
pub mod ResourceFormatLoader {
    pub use super::RefCounted::*;
    pub const get_recognized_extensions: (&'static str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub const recognize_path: (&'static str, u32) = ("_recognize_path", 2594487047u32);
    pub const handles_type: (&'static str, u32) = ("_handles_type", 2619796661u32);
    pub const get_resource_type: (&'static str, u32) = ("_get_resource_type", 3135753539u32);
    pub const get_resource_script_class: (&'static str, u32) = ("_get_resource_script_class", 3135753539u32);
    pub const get_resource_uid: (&'static str, u32) = ("_get_resource_uid", 1321353865u32);
    pub const get_dependencies: (&'static str, u32) = ("_get_dependencies", 6257701u32);
    pub const rename_dependencies: (&'static str, u32) = ("_rename_dependencies", 223715120u32);
    pub const exists: (&'static str, u32) = ("_exists", 3927539163u32);
    pub const get_classes_used: (&'static str, u32) = ("_get_classes_used", 4291131558u32);
    pub const load: (&'static str, u32) = ("_load", 2885906527u32);
    
}
pub mod ResourceFormatSaver {
    pub use super::RefCounted::*;
    pub const save: (&'static str, u32) = ("_save", 2794699034u32);
    pub const set_uid: (&'static str, u32) = ("_set_uid", 993915709u32);
    pub const recognize: (&'static str, u32) = ("_recognize", 3190994482u32);
    pub const get_recognized_extensions: (&'static str, u32) = ("_get_recognized_extensions", 1567505034u32);
    pub const recognize_path: (&'static str, u32) = ("_recognize_path", 710996192u32);
    
}
pub mod ResourceImporter {
    pub use super::RefCounted::*;
    pub const get_build_dependencies: (&'static str, u32) = ("_get_build_dependencies", 4291131558u32);
    
}
pub mod ResourceImporterBmFont {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterBitMap {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterCsvTranslation {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterDynamicFont {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterImage {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterImageFont {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterLayeredTexture {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterMp3 {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterObj {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterOggVorbis {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterSvg {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterScene {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterShaderFile {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterTexture {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterTextureAtlas {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceImporterWav {
    pub use super::ResourceImporter::*;
    
}
pub mod ResourceLoader {
    pub use super::Object::*;
    
}
pub mod ResourcePreloader {
    pub use super::Node::*;
    
}
pub mod ResourceSaver {
    pub use super::Object::*;
    
}
pub mod ResourceUid {
    pub use super::Object::*;
    
}
pub mod RetargetModifier3D {
    pub use super::SkeletonModifier3D::*;
    
}
pub mod RibbonTrailMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod RichTextEffect {
    pub use super::Resource::*;
    pub const process_custom_fx: (&'static str, u32) = ("_process_custom_fx", 31984339u32);
    
}
pub mod RichTextLabel {
    pub use super::Control::*;
    
}
pub mod RigidBody2D {
    pub use super::PhysicsBody2D::*;
    pub const integrate_forces: (&'static str, u32) = ("_integrate_forces", 370287496u32);
    
}
pub mod RigidBody3D {
    pub use super::PhysicsBody3D::*;
    pub const integrate_forces: (&'static str, u32) = ("_integrate_forces", 420958145u32);
    
}
pub mod RootMotionView {
    pub use super::VisualInstance3D::*;
    
}
pub mod SceneMultiplayer {
    pub use super::MultiplayerApi::*;
    
}
pub mod SceneReplicationConfig {
    pub use super::Resource::*;
    
}
pub mod SceneState {
    pub use super::RefCounted::*;
    
}
pub mod SceneTree {
    pub use super::MainLoop::*;
    
}
pub mod SceneTreeTimer {
    pub use super::RefCounted::*;
    
}
pub mod Script {
    pub use super::Resource::*;
    
}
pub mod ScriptBacktrace {
    pub use super::RefCounted::*;
    
}
pub mod ScriptCreateDialog {
    pub use super::ConfirmationDialog::*;
    
}
pub mod ScriptEditor {
    pub use super::PanelContainer::*;
    
}
pub mod ScriptEditorBase {
    pub use super::VBoxContainer::*;
    
}
pub mod ScriptExtension {
    pub use super::Script::*;
    pub const editor_can_reload_from_file: (&'static str, u32) = ("_editor_can_reload_from_file", 2240911060u32);
    pub const placeholder_erased_rawptr: (&'static str, u32) = ("_placeholder_erased", 1286410249u32);
    pub const can_instantiate: (&'static str, u32) = ("_can_instantiate", 36873697u32);
    pub const get_base_script: (&'static str, u32) = ("_get_base_script", 278624046u32);
    pub const get_global_name: (&'static str, u32) = ("_get_global_name", 2002593661u32);
    pub const inherits_script: (&'static str, u32) = ("_inherits_script", 3669307804u32);
    pub const get_instance_base_type: (&'static str, u32) = ("_get_instance_base_type", 2002593661u32);
    pub const instance_create_rawptr: (&'static str, u32) = ("_instance_create", 1107568780u32);
    pub const placeholder_instance_create_rawptr: (&'static str, u32) = ("_placeholder_instance_create", 1107568780u32);
    pub const instance_has: (&'static str, u32) = ("_instance_has", 397768994u32);
    pub const has_source_code: (&'static str, u32) = ("_has_source_code", 36873697u32);
    pub const get_source_code: (&'static str, u32) = ("_get_source_code", 201670096u32);
    pub const set_source_code: (&'static str, u32) = ("_set_source_code", 83702148u32);
    pub const reload: (&'static str, u32) = ("_reload", 1413768114u32);
    pub const get_doc_class_name: (&'static str, u32) = ("_get_doc_class_name", 2002593661u32);
    pub const get_documentation: (&'static str, u32) = ("_get_documentation", 3995934104u32);
    pub const get_class_icon_path: (&'static str, u32) = ("_get_class_icon_path", 201670096u32);
    pub const has_method: (&'static str, u32) = ("_has_method", 2619796661u32);
    pub const has_static_method: (&'static str, u32) = ("_has_static_method", 2619796661u32);
    pub const get_script_method_argument_count: (&'static str, u32) = ("_get_script_method_argument_count", 2760726917u32);
    pub const get_method_info: (&'static str, u32) = ("_get_method_info", 4028089122u32);
    pub const is_tool: (&'static str, u32) = ("_is_tool", 36873697u32);
    pub const is_valid: (&'static str, u32) = ("_is_valid", 36873697u32);
    pub const is_abstract: (&'static str, u32) = ("_is_abstract", 36873697u32);
    pub const get_language: (&'static str, u32) = ("_get_language", 3096237657u32);
    pub const has_script_signal: (&'static str, u32) = ("_has_script_signal", 2619796661u32);
    pub const get_script_signal_list: (&'static str, u32) = ("_get_script_signal_list", 3995934104u32);
    pub const has_property_default_value: (&'static str, u32) = ("_has_property_default_value", 2619796661u32);
    pub const get_property_default_value: (&'static str, u32) = ("_get_property_default_value", 2760726917u32);
    pub const update_exports: (&'static str, u32) = ("_update_exports", 3218959716u32);
    pub const get_script_method_list: (&'static str, u32) = ("_get_script_method_list", 3995934104u32);
    pub const get_script_property_list: (&'static str, u32) = ("_get_script_property_list", 3995934104u32);
    pub const get_member_line: (&'static str, u32) = ("_get_member_line", 2458036349u32);
    pub const get_constants: (&'static str, u32) = ("_get_constants", 3102165223u32);
    pub const get_members: (&'static str, u32) = ("_get_members", 3995934104u32);
    pub const is_placeholder_fallback_enabled: (&'static str, u32) = ("_is_placeholder_fallback_enabled", 36873697u32);
    pub const get_rpc_config: (&'static str, u32) = ("_get_rpc_config", 1214101251u32);
    
}
pub mod ScriptLanguage {
    pub use super::Object::*;
    
}
pub mod ScriptLanguageExtension {
    pub use super::ScriptLanguage::*;
    pub const get_name: (&'static str, u32) = ("_get_name", 201670096u32);
    pub const init_ext: (&'static str, u32) = ("_init", 3218959716u32);
    pub const get_type: (&'static str, u32) = ("_get_type", 201670096u32);
    pub const get_extension: (&'static str, u32) = ("_get_extension", 201670096u32);
    pub const finish: (&'static str, u32) = ("_finish", 3218959716u32);
    pub const get_reserved_words: (&'static str, u32) = ("_get_reserved_words", 1139954409u32);
    pub const is_control_flow_keyword: (&'static str, u32) = ("_is_control_flow_keyword", 3927539163u32);
    pub const get_comment_delimiters: (&'static str, u32) = ("_get_comment_delimiters", 1139954409u32);
    pub const get_doc_comment_delimiters: (&'static str, u32) = ("_get_doc_comment_delimiters", 1139954409u32);
    pub const get_string_delimiters: (&'static str, u32) = ("_get_string_delimiters", 1139954409u32);
    pub const make_template: (&'static str, u32) = ("_make_template", 3583744548u32);
    pub const get_built_in_templates: (&'static str, u32) = ("_get_built_in_templates", 3147814860u32);
    pub const is_using_templates: (&'static str, u32) = ("_is_using_templates", 2240911060u32);
    pub const validate: (&'static str, u32) = ("_validate", 1697887509u32);
    pub const validate_path: (&'static str, u32) = ("_validate_path", 3135753539u32);
    pub const create_script: (&'static str, u32) = ("_create_script", 1981248198u32);
    pub const has_named_classes: (&'static str, u32) = ("_has_named_classes", 36873697u32);
    pub const supports_builtin_mode: (&'static str, u32) = ("_supports_builtin_mode", 36873697u32);
    pub const supports_documentation: (&'static str, u32) = ("_supports_documentation", 36873697u32);
    pub const can_inherit_from_file: (&'static str, u32) = ("_can_inherit_from_file", 36873697u32);
    pub const find_function: (&'static str, u32) = ("_find_function", 2878152881u32);
    pub const make_function: (&'static str, u32) = ("_make_function", 1243061914u32);
    pub const can_make_function: (&'static str, u32) = ("_can_make_function", 36873697u32);
    pub const open_in_external_editor: (&'static str, u32) = ("_open_in_external_editor", 552845695u32);
    pub const overrides_external_editor: (&'static str, u32) = ("_overrides_external_editor", 2240911060u32);
    pub const preferred_file_name_casing: (&'static str, u32) = ("_preferred_file_name_casing", 2969522789u32);
    pub const complete_code: (&'static str, u32) = ("_complete_code", 950756616u32);
    pub const lookup_code: (&'static str, u32) = ("_lookup_code", 3143837309u32);
    pub const auto_indent_code: (&'static str, u32) = ("_auto_indent_code", 2531480354u32);
    pub const add_global_constant: (&'static str, u32) = ("_add_global_constant", 3776071444u32);
    pub const add_named_global_constant: (&'static str, u32) = ("_add_named_global_constant", 3776071444u32);
    pub const remove_named_global_constant: (&'static str, u32) = ("_remove_named_global_constant", 3304788590u32);
    pub const thread_enter: (&'static str, u32) = ("_thread_enter", 3218959716u32);
    pub const thread_exit: (&'static str, u32) = ("_thread_exit", 3218959716u32);
    pub const debug_get_error: (&'static str, u32) = ("_debug_get_error", 201670096u32);
    pub const debug_get_stack_level_count: (&'static str, u32) = ("_debug_get_stack_level_count", 3905245786u32);
    pub const debug_get_stack_level_line: (&'static str, u32) = ("_debug_get_stack_level_line", 923996154u32);
    pub const debug_get_stack_level_function: (&'static str, u32) = ("_debug_get_stack_level_function", 844755477u32);
    pub const debug_get_stack_level_source: (&'static str, u32) = ("_debug_get_stack_level_source", 844755477u32);
    pub const debug_get_stack_level_locals: (&'static str, u32) = ("_debug_get_stack_level_locals", 335235777u32);
    pub const debug_get_stack_level_members: (&'static str, u32) = ("_debug_get_stack_level_members", 335235777u32);
    pub const debug_get_stack_level_instance_rawptr: (&'static str, u32) = ("_debug_get_stack_level_instance", 3744713108u32);
    pub const debug_get_globals: (&'static str, u32) = ("_debug_get_globals", 4123630098u32);
    pub const debug_parse_stack_level_expression: (&'static str, u32) = ("_debug_parse_stack_level_expression", 1135811067u32);
    pub const debug_get_current_stack_info: (&'static str, u32) = ("_debug_get_current_stack_info", 2915620761u32);
    pub const reload_all_scripts: (&'static str, u32) = ("_reload_all_scripts", 3218959716u32);
    pub const reload_scripts: (&'static str, u32) = ("_reload_scripts", 3156113851u32);
    pub const reload_tool_script: (&'static str, u32) = ("_reload_tool_script", 1957307671u32);
    pub const get_recognized_extensions: (&'static str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub const get_public_functions: (&'static str, u32) = ("_get_public_functions", 3995934104u32);
    pub const get_public_constants: (&'static str, u32) = ("_get_public_constants", 3102165223u32);
    pub const get_public_annotations: (&'static str, u32) = ("_get_public_annotations", 3995934104u32);
    pub const profiling_start: (&'static str, u32) = ("_profiling_start", 3218959716u32);
    pub const profiling_stop: (&'static str, u32) = ("_profiling_stop", 3218959716u32);
    pub const profiling_set_save_native_calls: (&'static str, u32) = ("_profiling_set_save_native_calls", 2586408642u32);
    pub const profiling_get_accumulated_data_rawptr: (&'static str, u32) = ("_profiling_get_accumulated_data", 50157827u32);
    pub const profiling_get_frame_data_rawptr: (&'static str, u32) = ("_profiling_get_frame_data", 50157827u32);
    pub const frame: (&'static str, u32) = ("_frame", 3218959716u32);
    pub const handles_global_class_type: (&'static str, u32) = ("_handles_global_class_type", 3927539163u32);
    pub const get_global_class_name: (&'static str, u32) = ("_get_global_class_name", 2248993622u32);
    
}
pub mod ScrollBar {
    pub use super::Range::*;
    
}
pub mod ScrollContainer {
    pub use super::Container::*;
    
}
pub mod SegmentShape2D {
    pub use super::Shape2D::*;
    
}
pub mod SeparationRayShape2D {
    pub use super::Shape2D::*;
    
}
pub mod SeparationRayShape3D {
    pub use super::Shape3D::*;
    
}
pub mod Separator {
    pub use super::Control::*;
    
}
pub mod Shader {
    pub use super::Resource::*;
    
}
pub mod ShaderGlobalsOverride {
    pub use super::Node::*;
    
}
pub mod ShaderInclude {
    pub use super::Resource::*;
    
}
pub mod ShaderIncludeDb {
    pub use super::Object::*;
    
}
pub mod ShaderMaterial {
    pub use super::Material::*;
    
}
pub mod Shape2D {
    pub use super::Resource::*;
    
}
pub mod Shape3D {
    pub use super::Resource::*;
    
}
pub mod ShapeCast2D {
    pub use super::Node2D::*;
    
}
pub mod ShapeCast3D {
    pub use super::Node3D::*;
    
}
pub mod Shortcut {
    pub use super::Resource::*;
    
}
pub mod Skeleton2D {
    pub use super::Node2D::*;
    
}
pub mod Skeleton3D {
    pub use super::Node3D::*;
    
}
pub mod SkeletonIk3d {
    pub use super::SkeletonModifier3D::*;
    
}
pub mod SkeletonModifier3D {
    pub use super::Node3D::*;
    pub const process_modification_with_delta: (&'static str, u32) = ("_process_modification_with_delta", 373806689u32);
    pub const process_modification: (&'static str, u32) = ("_process_modification", 3218959716u32);
    pub const skeleton_changed: (&'static str, u32) = ("_skeleton_changed", 2926744397u32);
    pub const validate_bone_names: (&'static str, u32) = ("_validate_bone_names", 3218959716u32);
    
}
pub mod SkeletonProfile {
    pub use super::Resource::*;
    
}
pub mod SkeletonProfileHumanoid {
    pub use super::SkeletonProfile::*;
    
}
pub mod Skin {
    pub use super::Resource::*;
    
}
pub mod SkinReference {
    pub use super::RefCounted::*;
    
}
pub mod Sky {
    pub use super::Resource::*;
    
}
pub mod Slider {
    pub use super::Range::*;
    
}
pub mod SliderJoint3D {
    pub use super::Joint3D::*;
    
}
pub mod SoftBody3D {
    pub use super::MeshInstance3D::*;
    
}
pub mod SphereMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod SphereOccluder3D {
    pub use super::Occluder3D::*;
    
}
pub mod SphereShape3D {
    pub use super::Shape3D::*;
    
}
pub mod SpinBox {
    pub use super::Range::*;
    
}
pub mod SplitContainer {
    pub use super::Container::*;
    
}
pub mod SpotLight3D {
    pub use super::Light3D::*;
    
}
pub mod SpringArm3D {
    pub use super::Node3D::*;
    
}
pub mod SpringBoneCollision3D {
    pub use super::Node3D::*;
    
}
pub mod SpringBoneCollisionCapsule3D {
    pub use super::SpringBoneCollision3D::*;
    
}
pub mod SpringBoneCollisionPlane3D {
    pub use super::SpringBoneCollision3D::*;
    
}
pub mod SpringBoneCollisionSphere3D {
    pub use super::SpringBoneCollision3D::*;
    
}
pub mod SpringBoneSimulator3D {
    pub use super::SkeletonModifier3D::*;
    
}
pub mod Sprite2D {
    pub use super::Node2D::*;
    
}
pub mod Sprite3D {
    pub use super::SpriteBase3D::*;
    
}
pub mod SpriteBase3D {
    pub use super::GeometryInstance3D::*;
    
}
pub mod SpriteFrames {
    pub use super::Resource::*;
    
}
pub mod StandardMaterial3D {
    pub use super::BaseMaterial3D::*;
    
}
pub mod StaticBody2D {
    pub use super::PhysicsBody2D::*;
    
}
pub mod StaticBody3D {
    pub use super::PhysicsBody3D::*;
    
}
pub mod StatusIndicator {
    pub use super::Node::*;
    
}
pub mod StreamPeer {
    pub use super::RefCounted::*;
    
}
pub mod StreamPeerBuffer {
    pub use super::StreamPeer::*;
    
}
pub mod StreamPeerExtension {
    pub use super::StreamPeer::*;
    pub const get_data_rawptr: (&'static str, u32) = ("_get_data", 298948178u32);
    pub const get_partial_data_rawptr: (&'static str, u32) = ("_get_partial_data", 298948178u32);
    pub const put_data_rawptr: (&'static str, u32) = ("_put_data", 298948178u32);
    pub const put_partial_data_rawptr: (&'static str, u32) = ("_put_partial_data", 298948178u32);
    pub const get_available_bytes: (&'static str, u32) = ("_get_available_bytes", 3905245786u32);
    
}
pub mod StreamPeerTcp {
    pub use super::StreamPeer::*;
    
}
pub mod StreamPeerTls {
    pub use super::StreamPeer::*;
    
}
pub mod StyleBox {
    pub use super::Resource::*;
    pub const draw: (&'static str, u32) = ("_draw", 2275962004u32);
    pub const get_draw_rect: (&'static str, u32) = ("_get_draw_rect", 408950903u32);
    pub const get_minimum_size: (&'static str, u32) = ("_get_minimum_size", 3341600327u32);
    pub const test_mask: (&'static str, u32) = ("_test_mask", 3735564539u32);
    
}
pub mod StyleBoxEmpty {
    pub use super::StyleBox::*;
    
}
pub mod StyleBoxFlat {
    pub use super::StyleBox::*;
    
}
pub mod StyleBoxLine {
    pub use super::StyleBox::*;
    
}
pub mod StyleBoxTexture {
    pub use super::StyleBox::*;
    
}
pub mod SubViewport {
    pub use super::Viewport::*;
    
}
pub mod SubViewportContainer {
    pub use super::Container::*;
    pub const propagate_input_event: (&'static str, u32) = ("_propagate_input_event", 3738334489u32);
    
}
pub mod SubtweenTweener {
    pub use super::Tweener::*;
    
}
pub mod SurfaceTool {
    pub use super::RefCounted::*;
    
}
pub mod SyntaxHighlighter {
    pub use super::Resource::*;
    pub const get_line_syntax_highlighting: (&'static str, u32) = ("_get_line_syntax_highlighting", 3485342025u32);
    pub const clear_highlighting_cache: (&'static str, u32) = ("_clear_highlighting_cache", 3218959716u32);
    pub const update_cache: (&'static str, u32) = ("_update_cache", 3218959716u32);
    
}
pub mod SystemFont {
    pub use super::Font::*;
    
}
pub mod TcpServer {
    pub use super::RefCounted::*;
    
}
pub mod TlsOptions {
    pub use super::RefCounted::*;
    
}
pub mod TabBar {
    pub use super::Control::*;
    
}
pub mod TabContainer {
    pub use super::Container::*;
    
}
pub mod TextEdit {
    pub use super::Control::*;
    pub const handle_unicode_input: (&'static str, u32) = ("_handle_unicode_input", 3937882851u32);
    pub const backspace: (&'static str, u32) = ("_backspace", 1286410249u32);
    pub const cut: (&'static str, u32) = ("_cut", 1286410249u32);
    pub const copy: (&'static str, u32) = ("_copy", 1286410249u32);
    pub const paste: (&'static str, u32) = ("_paste", 1286410249u32);
    pub const paste_primary_clipboard: (&'static str, u32) = ("_paste_primary_clipboard", 1286410249u32);
    
}
pub mod TextLine {
    pub use super::RefCounted::*;
    
}
pub mod TextMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod TextParagraph {
    pub use super::RefCounted::*;
    
}
pub mod TextServer {
    pub use super::RefCounted::*;
    
}
pub mod TextServerAdvanced {
    pub use super::TextServerExtension::*;
    
}
pub mod TextServerDummy {
    pub use super::TextServerExtension::*;
    
}
pub mod TextServerExtension {
    pub use super::TextServer::*;
    pub const has_feature: (&'static str, u32) = ("_has_feature", 3967367083u32);
    pub const get_name: (&'static str, u32) = ("_get_name", 201670096u32);
    pub const get_features: (&'static str, u32) = ("_get_features", 3905245786u32);
    pub const free_rid: (&'static str, u32) = ("_free_rid", 2722037293u32);
    pub const has: (&'static str, u32) = ("_has", 3521089500u32);
    pub const load_support_data: (&'static str, u32) = ("_load_support_data", 2323990056u32);
    pub const get_support_data_filename: (&'static str, u32) = ("_get_support_data_filename", 201670096u32);
    pub const get_support_data_info: (&'static str, u32) = ("_get_support_data_info", 201670096u32);
    pub const save_support_data: (&'static str, u32) = ("_save_support_data", 3927539163u32);
    pub const get_support_data: (&'static str, u32) = ("_get_support_data", 2362200018u32);
    pub const is_locale_right_to_left: (&'static str, u32) = ("_is_locale_right_to_left", 3927539163u32);
    pub const name_to_tag: (&'static str, u32) = ("_name_to_tag", 1321353865u32);
    pub const tag_to_name: (&'static str, u32) = ("_tag_to_name", 844755477u32);
    pub const create_font: (&'static str, u32) = ("_create_font", 529393457u32);
    pub const create_font_linked_variation: (&'static str, u32) = ("_create_font_linked_variation", 41030802u32);
    pub const font_set_data: (&'static str, u32) = ("_font_set_data", 1355495400u32);
    pub const font_set_data_ptr_rawptr: (&'static str, u32) = ("_font_set_data_ptr", 4288446313u32);
    pub const font_set_face_index: (&'static str, u32) = ("_font_set_face_index", 3411492887u32);
    pub const font_get_face_index: (&'static str, u32) = ("_font_get_face_index", 2198884583u32);
    pub const font_get_face_count: (&'static str, u32) = ("_font_get_face_count", 2198884583u32);
    pub const font_set_style: (&'static str, u32) = ("_font_set_style", 898466325u32);
    pub const font_get_style: (&'static str, u32) = ("_font_get_style", 3082502592u32);
    pub const font_set_name: (&'static str, u32) = ("_font_set_name", 2726140452u32);
    pub const font_get_name: (&'static str, u32) = ("_font_get_name", 642473191u32);
    pub const font_get_ot_name_strings: (&'static str, u32) = ("_font_get_ot_name_strings", 1882737106u32);
    pub const font_set_style_name: (&'static str, u32) = ("_font_set_style_name", 2726140452u32);
    pub const font_get_style_name: (&'static str, u32) = ("_font_get_style_name", 642473191u32);
    pub const font_set_weight: (&'static str, u32) = ("_font_set_weight", 3411492887u32);
    pub const font_get_weight: (&'static str, u32) = ("_font_get_weight", 2198884583u32);
    pub const font_set_stretch: (&'static str, u32) = ("_font_set_stretch", 3411492887u32);
    pub const font_get_stretch: (&'static str, u32) = ("_font_get_stretch", 2198884583u32);
    pub const font_set_antialiasing: (&'static str, u32) = ("_font_set_antialiasing", 958337235u32);
    pub const font_get_antialiasing: (&'static str, u32) = ("_font_get_antialiasing", 3389420495u32);
    pub const font_set_disable_embedded_bitmaps: (&'static str, u32) = ("_font_set_disable_embedded_bitmaps", 1265174801u32);
    pub const font_get_disable_embedded_bitmaps: (&'static str, u32) = ("_font_get_disable_embedded_bitmaps", 4155700596u32);
    pub const font_set_generate_mipmaps: (&'static str, u32) = ("_font_set_generate_mipmaps", 1265174801u32);
    pub const font_get_generate_mipmaps: (&'static str, u32) = ("_font_get_generate_mipmaps", 4155700596u32);
    pub const font_set_multichannel_signed_distance_field: (&'static str, u32) = ("_font_set_multichannel_signed_distance_field", 1265174801u32);
    pub const font_is_multichannel_signed_distance_field: (&'static str, u32) = ("_font_is_multichannel_signed_distance_field", 4155700596u32);
    pub const font_set_msdf_pixel_range: (&'static str, u32) = ("_font_set_msdf_pixel_range", 3411492887u32);
    pub const font_get_msdf_pixel_range: (&'static str, u32) = ("_font_get_msdf_pixel_range", 2198884583u32);
    pub const font_set_msdf_size: (&'static str, u32) = ("_font_set_msdf_size", 3411492887u32);
    pub const font_get_msdf_size: (&'static str, u32) = ("_font_get_msdf_size", 2198884583u32);
    pub const font_set_fixed_size: (&'static str, u32) = ("_font_set_fixed_size", 3411492887u32);
    pub const font_get_fixed_size: (&'static str, u32) = ("_font_get_fixed_size", 2198884583u32);
    pub const font_set_fixed_size_scale_mode: (&'static str, u32) = ("_font_set_fixed_size_scale_mode", 1029390307u32);
    pub const font_get_fixed_size_scale_mode: (&'static str, u32) = ("_font_get_fixed_size_scale_mode", 4113120379u32);
    pub const font_set_allow_system_fallback: (&'static str, u32) = ("_font_set_allow_system_fallback", 1265174801u32);
    pub const font_is_allow_system_fallback: (&'static str, u32) = ("_font_is_allow_system_fallback", 4155700596u32);
    pub const font_clear_system_fallback_cache: (&'static str, u32) = ("_font_clear_system_fallback_cache", 3218959716u32);
    pub const font_set_force_autohinter: (&'static str, u32) = ("_font_set_force_autohinter", 1265174801u32);
    pub const font_is_force_autohinter: (&'static str, u32) = ("_font_is_force_autohinter", 4155700596u32);
    pub const font_set_modulate_color_glyphs: (&'static str, u32) = ("_font_set_modulate_color_glyphs", 1265174801u32);
    pub const font_is_modulate_color_glyphs: (&'static str, u32) = ("_font_is_modulate_color_glyphs", 4155700596u32);
    pub const font_set_hinting: (&'static str, u32) = ("_font_set_hinting", 1520010864u32);
    pub const font_get_hinting: (&'static str, u32) = ("_font_get_hinting", 3971592737u32);
    pub const font_set_subpixel_positioning: (&'static str, u32) = ("_font_set_subpixel_positioning", 3830459669u32);
    pub const font_get_subpixel_positioning: (&'static str, u32) = ("_font_get_subpixel_positioning", 2752233671u32);
    pub const font_set_keep_rounding_remainders: (&'static str, u32) = ("_font_set_keep_rounding_remainders", 1265174801u32);
    pub const font_get_keep_rounding_remainders: (&'static str, u32) = ("_font_get_keep_rounding_remainders", 4155700596u32);
    pub const font_set_embolden: (&'static str, u32) = ("_font_set_embolden", 1794382983u32);
    pub const font_get_embolden: (&'static str, u32) = ("_font_get_embolden", 866169185u32);
    pub const font_set_spacing: (&'static str, u32) = ("_font_set_spacing", 1307259930u32);
    pub const font_get_spacing: (&'static str, u32) = ("_font_get_spacing", 1213653558u32);
    pub const font_set_baseline_offset: (&'static str, u32) = ("_font_set_baseline_offset", 1794382983u32);
    pub const font_get_baseline_offset: (&'static str, u32) = ("_font_get_baseline_offset", 866169185u32);
    pub const font_set_transform: (&'static str, u32) = ("_font_set_transform", 1246044741u32);
    pub const font_get_transform: (&'static str, u32) = ("_font_get_transform", 213527486u32);
    pub const font_set_variation_coordinates: (&'static str, u32) = ("_font_set_variation_coordinates", 1217542888u32);
    pub const font_get_variation_coordinates: (&'static str, u32) = ("_font_get_variation_coordinates", 1882737106u32);
    pub const font_set_oversampling: (&'static str, u32) = ("_font_set_oversampling", 1794382983u32);
    pub const font_get_oversampling: (&'static str, u32) = ("_font_get_oversampling", 866169185u32);
    pub const font_get_size_cache_list: (&'static str, u32) = ("_font_get_size_cache_list", 2684255073u32);
    pub const font_clear_size_cache: (&'static str, u32) = ("_font_clear_size_cache", 2722037293u32);
    pub const font_remove_size_cache: (&'static str, u32) = ("_font_remove_size_cache", 2450610377u32);
    pub const font_get_size_cache_info: (&'static str, u32) = ("_font_get_size_cache_info", 2684255073u32);
    pub const font_set_ascent: (&'static str, u32) = ("_font_set_ascent", 1892459533u32);
    pub const font_get_ascent: (&'static str, u32) = ("_font_get_ascent", 755457166u32);
    pub const font_set_descent: (&'static str, u32) = ("_font_set_descent", 1892459533u32);
    pub const font_get_descent: (&'static str, u32) = ("_font_get_descent", 755457166u32);
    pub const font_set_underline_position: (&'static str, u32) = ("_font_set_underline_position", 1892459533u32);
    pub const font_get_underline_position: (&'static str, u32) = ("_font_get_underline_position", 755457166u32);
    pub const font_set_underline_thickness: (&'static str, u32) = ("_font_set_underline_thickness", 1892459533u32);
    pub const font_get_underline_thickness: (&'static str, u32) = ("_font_get_underline_thickness", 755457166u32);
    pub const font_set_scale: (&'static str, u32) = ("_font_set_scale", 1892459533u32);
    pub const font_get_scale: (&'static str, u32) = ("_font_get_scale", 755457166u32);
    pub const font_get_texture_count: (&'static str, u32) = ("_font_get_texture_count", 1311001310u32);
    pub const font_clear_textures: (&'static str, u32) = ("_font_clear_textures", 2450610377u32);
    pub const font_remove_texture: (&'static str, u32) = ("_font_remove_texture", 3810512262u32);
    pub const font_set_texture_image: (&'static str, u32) = ("_font_set_texture_image", 2354485091u32);
    pub const font_get_texture_image: (&'static str, u32) = ("_font_get_texture_image", 2451761155u32);
    pub const font_set_texture_offsets: (&'static str, u32) = ("_font_set_texture_offsets", 3005398047u32);
    pub const font_get_texture_offsets: (&'static str, u32) = ("_font_get_texture_offsets", 3420028887u32);
    pub const font_get_glyph_list: (&'static str, u32) = ("_font_get_glyph_list", 46086620u32);
    pub const font_clear_glyphs: (&'static str, u32) = ("_font_clear_glyphs", 2450610377u32);
    pub const font_remove_glyph: (&'static str, u32) = ("_font_remove_glyph", 3810512262u32);
    pub const font_get_glyph_advance: (&'static str, u32) = ("_font_get_glyph_advance", 2555689501u32);
    pub const font_set_glyph_advance: (&'static str, u32) = ("_font_set_glyph_advance", 3219397315u32);
    pub const font_get_glyph_offset: (&'static str, u32) = ("_font_get_glyph_offset", 513728628u32);
    pub const font_set_glyph_offset: (&'static str, u32) = ("_font_set_glyph_offset", 1812632090u32);
    pub const font_get_glyph_size: (&'static str, u32) = ("_font_get_glyph_size", 513728628u32);
    pub const font_set_glyph_size: (&'static str, u32) = ("_font_set_glyph_size", 1812632090u32);
    pub const font_get_glyph_uv_rect: (&'static str, u32) = ("_font_get_glyph_uv_rect", 2274268786u32);
    pub const font_set_glyph_uv_rect: (&'static str, u32) = ("_font_set_glyph_uv_rect", 1973324081u32);
    pub const font_get_glyph_texture_idx: (&'static str, u32) = ("_font_get_glyph_texture_idx", 4292800474u32);
    pub const font_set_glyph_texture_idx: (&'static str, u32) = ("_font_set_glyph_texture_idx", 4254580980u32);
    pub const font_get_glyph_texture_rid: (&'static str, u32) = ("_font_get_glyph_texture_rid", 1451696141u32);
    pub const font_get_glyph_texture_size: (&'static str, u32) = ("_font_get_glyph_texture_size", 513728628u32);
    pub const font_get_glyph_contours: (&'static str, u32) = ("_font_get_glyph_contours", 2903964473u32);
    pub const font_get_kerning_list: (&'static str, u32) = ("_font_get_kerning_list", 1778388067u32);
    pub const font_clear_kerning_map: (&'static str, u32) = ("_font_clear_kerning_map", 3411492887u32);
    pub const font_remove_kerning: (&'static str, u32) = ("_font_remove_kerning", 2141860016u32);
    pub const font_set_kerning: (&'static str, u32) = ("_font_set_kerning", 3630965883u32);
    pub const font_get_kerning: (&'static str, u32) = ("_font_get_kerning", 1019980169u32);
    pub const font_get_glyph_index: (&'static str, u32) = ("_font_get_glyph_index", 1765635060u32);
    pub const font_get_char_from_glyph_index: (&'static str, u32) = ("_font_get_char_from_glyph_index", 2156738276u32);
    pub const font_has_char: (&'static str, u32) = ("_font_has_char", 3120086654u32);
    pub const font_get_supported_chars: (&'static str, u32) = ("_font_get_supported_chars", 642473191u32);
    pub const font_get_supported_glyphs: (&'static str, u32) = ("_font_get_supported_glyphs", 788230395u32);
    pub const font_render_range: (&'static str, u32) = ("_font_render_range", 4254580980u32);
    pub const font_render_glyph: (&'static str, u32) = ("_font_render_glyph", 3810512262u32);
    pub const font_draw_glyph: (&'static str, u32) = ("_font_draw_glyph", 404525066u32);
    pub const font_draw_glyph_outline: (&'static str, u32) = ("_font_draw_glyph_outline", 940535541u32);
    pub const font_is_language_supported: (&'static str, u32) = ("_font_is_language_supported", 3199320846u32);
    pub const font_set_language_support_override: (&'static str, u32) = ("_font_set_language_support_override", 2313957094u32);
    pub const font_get_language_support_override: (&'static str, u32) = ("_font_get_language_support_override", 2829184646u32);
    pub const font_remove_language_support_override: (&'static str, u32) = ("_font_remove_language_support_override", 2726140452u32);
    pub const font_get_language_support_overrides: (&'static str, u32) = ("_font_get_language_support_overrides", 2801473409u32);
    pub const font_is_script_supported: (&'static str, u32) = ("_font_is_script_supported", 3199320846u32);
    pub const font_set_script_support_override: (&'static str, u32) = ("_font_set_script_support_override", 2313957094u32);
    pub const font_get_script_support_override: (&'static str, u32) = ("_font_get_script_support_override", 2829184646u32);
    pub const font_remove_script_support_override: (&'static str, u32) = ("_font_remove_script_support_override", 2726140452u32);
    pub const font_get_script_support_overrides: (&'static str, u32) = ("_font_get_script_support_overrides", 2801473409u32);
    pub const font_set_opentype_feature_overrides: (&'static str, u32) = ("_font_set_opentype_feature_overrides", 1217542888u32);
    pub const font_get_opentype_feature_overrides: (&'static str, u32) = ("_font_get_opentype_feature_overrides", 1882737106u32);
    pub const font_supported_feature_list: (&'static str, u32) = ("_font_supported_feature_list", 1882737106u32);
    pub const font_supported_variation_list: (&'static str, u32) = ("_font_supported_variation_list", 1882737106u32);
    pub const font_get_global_oversampling: (&'static str, u32) = ("_font_get_global_oversampling", 1740695150u32);
    pub const font_set_global_oversampling: (&'static str, u32) = ("_font_set_global_oversampling", 373806689u32);
    pub const reference_oversampling_level: (&'static str, u32) = ("_reference_oversampling_level", 373806689u32);
    pub const unreference_oversampling_level: (&'static str, u32) = ("_unreference_oversampling_level", 373806689u32);
    pub const get_hex_code_box_size: (&'static str, u32) = ("_get_hex_code_box_size", 3016396712u32);
    pub const draw_hex_code_box: (&'static str, u32) = ("_draw_hex_code_box", 1602046441u32);
    pub const create_shaped_text: (&'static str, u32) = ("_create_shaped_text", 1431128392u32);
    pub const shaped_text_clear: (&'static str, u32) = ("_shaped_text_clear", 2722037293u32);
    pub const shaped_text_set_direction: (&'static str, u32) = ("_shaped_text_set_direction", 4276135416u32);
    pub const shaped_text_get_direction: (&'static str, u32) = ("_shaped_text_get_direction", 3065904362u32);
    pub const shaped_text_get_inferred_direction: (&'static str, u32) = ("_shaped_text_get_inferred_direction", 3065904362u32);
    pub const shaped_text_set_bidi_override: (&'static str, u32) = ("_shaped_text_set_bidi_override", 684822712u32);
    pub const shaped_text_set_custom_punctuation: (&'static str, u32) = ("_shaped_text_set_custom_punctuation", 2726140452u32);
    pub const shaped_text_get_custom_punctuation: (&'static str, u32) = ("_shaped_text_get_custom_punctuation", 642473191u32);
    pub const shaped_text_set_custom_ellipsis: (&'static str, u32) = ("_shaped_text_set_custom_ellipsis", 3411492887u32);
    pub const shaped_text_get_custom_ellipsis: (&'static str, u32) = ("_shaped_text_get_custom_ellipsis", 2198884583u32);
    pub const shaped_text_set_orientation: (&'static str, u32) = ("_shaped_text_set_orientation", 2306444742u32);
    pub const shaped_text_get_orientation: (&'static str, u32) = ("_shaped_text_get_orientation", 3142708106u32);
    pub const shaped_text_set_preserve_invalid: (&'static str, u32) = ("_shaped_text_set_preserve_invalid", 1265174801u32);
    pub const shaped_text_get_preserve_invalid: (&'static str, u32) = ("_shaped_text_get_preserve_invalid", 4155700596u32);
    pub const shaped_text_set_preserve_control: (&'static str, u32) = ("_shaped_text_set_preserve_control", 1265174801u32);
    pub const shaped_text_get_preserve_control: (&'static str, u32) = ("_shaped_text_get_preserve_control", 4155700596u32);
    pub const shaped_text_set_spacing: (&'static str, u32) = ("_shaped_text_set_spacing", 1307259930u32);
    pub const shaped_text_get_spacing: (&'static str, u32) = ("_shaped_text_get_spacing", 1213653558u32);
    pub const shaped_text_add_string: (&'static str, u32) = ("_shaped_text_add_string", 875249313u32);
    pub const shaped_text_add_object: (&'static str, u32) = ("_shaped_text_add_object", 2452224230u32);
    pub const shaped_text_resize_object: (&'static str, u32) = ("_shaped_text_resize_object", 2747466775u32);
    pub const shaped_get_text: (&'static str, u32) = ("_shaped_get_text", 642473191u32);
    pub const shaped_get_span_count: (&'static str, u32) = ("_shaped_get_span_count", 2198884583u32);
    pub const shaped_get_span_meta: (&'static str, u32) = ("_shaped_get_span_meta", 4069510997u32);
    pub const shaped_get_span_embedded_object: (&'static str, u32) = ("_shaped_get_span_embedded_object", 4069510997u32);
    pub const shaped_get_span_text: (&'static str, u32) = ("_shaped_get_span_text", 1464764419u32);
    pub const shaped_get_span_object: (&'static str, u32) = ("_shaped_get_span_object", 4069510997u32);
    pub const shaped_set_span_update_font: (&'static str, u32) = ("_shaped_set_span_update_font", 2569459151u32);
    pub const shaped_get_run_count: (&'static str, u32) = ("_shaped_get_run_count", 2198884583u32);
    pub const shaped_get_run_text: (&'static str, u32) = ("_shaped_get_run_text", 1464764419u32);
    pub const shaped_get_run_range: (&'static str, u32) = ("_shaped_get_run_range", 4069534484u32);
    pub const shaped_get_run_font_rid: (&'static str, u32) = ("_shaped_get_run_font_rid", 1066463050u32);
    pub const shaped_get_run_font_size: (&'static str, u32) = ("_shaped_get_run_font_size", 1120910005u32);
    pub const shaped_get_run_language: (&'static str, u32) = ("_shaped_get_run_language", 1464764419u32);
    pub const shaped_get_run_direction: (&'static str, u32) = ("_shaped_get_run_direction", 2413896864u32);
    pub const shaped_get_run_object: (&'static str, u32) = ("_shaped_get_run_object", 4069510997u32);
    pub const shaped_text_substr: (&'static str, u32) = ("_shaped_text_substr", 1937682086u32);
    pub const shaped_text_get_parent: (&'static str, u32) = ("_shaped_text_get_parent", 3814569979u32);
    pub const shaped_text_fit_to_width: (&'static str, u32) = ("_shaped_text_fit_to_width", 1426448222u32);
    pub const shaped_text_tab_align: (&'static str, u32) = ("_shaped_text_tab_align", 1283669550u32);
    pub const shaped_text_shape: (&'static str, u32) = ("_shaped_text_shape", 3521089500u32);
    pub const shaped_text_update_breaks: (&'static str, u32) = ("_shaped_text_update_breaks", 3521089500u32);
    pub const shaped_text_update_justification_ops: (&'static str, u32) = ("_shaped_text_update_justification_ops", 3521089500u32);
    pub const shaped_text_is_ready: (&'static str, u32) = ("_shaped_text_is_ready", 4155700596u32);
    pub const shaped_text_get_glyphs_rawptr: (&'static str, u32) = ("_shaped_text_get_glyphs", 2198884583u32);
    pub const shaped_text_sort_logical_rawptr: (&'static str, u32) = ("_shaped_text_sort_logical", 3917799429u32);
    pub const shaped_text_get_glyph_count: (&'static str, u32) = ("_shaped_text_get_glyph_count", 2198884583u32);
    pub const shaped_text_get_range: (&'static str, u32) = ("_shaped_text_get_range", 733700038u32);
    pub const shaped_text_get_line_breaks_adv: (&'static str, u32) = ("_shaped_text_get_line_breaks_adv", 1488467363u32);
    pub const shaped_text_get_line_breaks: (&'static str, u32) = ("_shaped_text_get_line_breaks", 3131311977u32);
    pub const shaped_text_get_word_breaks: (&'static str, u32) = ("_shaped_text_get_word_breaks", 2423529412u32);
    pub const shaped_text_get_trim_pos: (&'static str, u32) = ("_shaped_text_get_trim_pos", 2198884583u32);
    pub const shaped_text_get_ellipsis_pos: (&'static str, u32) = ("_shaped_text_get_ellipsis_pos", 2198884583u32);
    pub const shaped_text_get_ellipsis_glyph_count: (&'static str, u32) = ("_shaped_text_get_ellipsis_glyph_count", 2198884583u32);
    pub const shaped_text_get_ellipsis_glyphs_rawptr: (&'static str, u32) = ("_shaped_text_get_ellipsis_glyphs", 2198884583u32);
    pub const shaped_text_overrun_trim_to_width: (&'static str, u32) = ("_shaped_text_overrun_trim_to_width", 3364950921u32);
    pub const shaped_text_get_objects: (&'static str, u32) = ("_shaped_text_get_objects", 2684255073u32);
    pub const shaped_text_get_object_rect: (&'static str, u32) = ("_shaped_text_get_object_rect", 447978354u32);
    pub const shaped_text_get_object_range: (&'static str, u32) = ("_shaped_text_get_object_range", 2524675647u32);
    pub const shaped_text_get_object_glyph: (&'static str, u32) = ("_shaped_text_get_object_glyph", 1260085030u32);
    pub const shaped_text_get_size: (&'static str, u32) = ("_shaped_text_get_size", 2440833711u32);
    pub const shaped_text_get_ascent: (&'static str, u32) = ("_shaped_text_get_ascent", 866169185u32);
    pub const shaped_text_get_descent: (&'static str, u32) = ("_shaped_text_get_descent", 866169185u32);
    pub const shaped_text_get_width: (&'static str, u32) = ("_shaped_text_get_width", 866169185u32);
    pub const shaped_text_get_underline_position: (&'static str, u32) = ("_shaped_text_get_underline_position", 866169185u32);
    pub const shaped_text_get_underline_thickness: (&'static str, u32) = ("_shaped_text_get_underline_thickness", 866169185u32);
    pub const shaped_text_get_dominant_direction_in_range: (&'static str, u32) = ("_shaped_text_get_dominant_direction_in_range", 2156738276u32);
    pub const shaped_text_get_carets_rawptr: (&'static str, u32) = ("_shaped_text_get_carets", 1191777527u32);
    pub const shaped_text_get_selection: (&'static str, u32) = ("_shaped_text_get_selection", 3714187733u32);
    pub const shaped_text_hit_test_grapheme: (&'static str, u32) = ("_shaped_text_hit_test_grapheme", 3149310417u32);
    pub const shaped_text_hit_test_position: (&'static str, u32) = ("_shaped_text_hit_test_position", 3149310417u32);
    pub const shaped_text_draw: (&'static str, u32) = ("_shaped_text_draw", 2079930245u32);
    pub const shaped_text_draw_outline: (&'static str, u32) = ("_shaped_text_draw_outline", 601976754u32);
    pub const shaped_text_get_grapheme_bounds: (&'static str, u32) = ("_shaped_text_get_grapheme_bounds", 2546185844u32);
    pub const shaped_text_next_grapheme_pos: (&'static str, u32) = ("_shaped_text_next_grapheme_pos", 1120910005u32);
    pub const shaped_text_prev_grapheme_pos: (&'static str, u32) = ("_shaped_text_prev_grapheme_pos", 1120910005u32);
    pub const shaped_text_get_character_breaks: (&'static str, u32) = ("_shaped_text_get_character_breaks", 788230395u32);
    pub const shaped_text_next_character_pos: (&'static str, u32) = ("_shaped_text_next_character_pos", 1120910005u32);
    pub const shaped_text_prev_character_pos: (&'static str, u32) = ("_shaped_text_prev_character_pos", 1120910005u32);
    pub const shaped_text_closest_character_pos: (&'static str, u32) = ("_shaped_text_closest_character_pos", 1120910005u32);
    pub const format_number: (&'static str, u32) = ("_format_number", 315676799u32);
    pub const parse_number: (&'static str, u32) = ("_parse_number", 315676799u32);
    pub const percent_sign: (&'static str, u32) = ("_percent_sign", 3135753539u32);
    pub const strip_diacritics: (&'static str, u32) = ("_strip_diacritics", 3135753539u32);
    pub const is_valid_identifier: (&'static str, u32) = ("_is_valid_identifier", 3927539163u32);
    pub const is_valid_letter: (&'static str, u32) = ("_is_valid_letter", 1116898809u32);
    pub const string_get_word_breaks: (&'static str, u32) = ("_string_get_word_breaks", 3658450588u32);
    pub const string_get_character_breaks: (&'static str, u32) = ("_string_get_character_breaks", 2509056759u32);
    pub const is_confusable: (&'static str, u32) = ("_is_confusable", 1433197768u32);
    pub const spoof_check: (&'static str, u32) = ("_spoof_check", 3927539163u32);
    pub const string_to_upper: (&'static str, u32) = ("_string_to_upper", 315676799u32);
    pub const string_to_lower: (&'static str, u32) = ("_string_to_lower", 315676799u32);
    pub const string_to_title: (&'static str, u32) = ("_string_to_title", 315676799u32);
    pub const parse_structured_text: (&'static str, u32) = ("_parse_structured_text", 3310685015u32);
    pub const cleanup: (&'static str, u32) = ("_cleanup", 3218959716u32);
    
}
pub mod TextServerManager {
    pub use super::Object::*;
    
}
pub mod Texture {
    pub use super::Resource::*;
    
}
pub mod Texture2D {
    pub use super::Texture::*;
    pub const get_width: (&'static str, u32) = ("_get_width", 3905245786u32);
    pub const get_height: (&'static str, u32) = ("_get_height", 3905245786u32);
    pub const is_pixel_opaque: (&'static str, u32) = ("_is_pixel_opaque", 2522259332u32);
    pub const has_alpha: (&'static str, u32) = ("_has_alpha", 36873697u32);
    pub const draw: (&'static str, u32) = ("_draw", 1384643611u32);
    pub const draw_rect: (&'static str, u32) = ("_draw_rect", 3819628907u32);
    pub const draw_rect_region: (&'static str, u32) = ("_draw_rect_region", 4094143664u32);
    
}
pub mod Texture2DArray {
    pub use super::ImageTextureLayered::*;
    
}
pub mod Texture2DArrayRd {
    pub use super::TextureLayeredRd::*;
    
}
pub mod Texture2Drd {
    pub use super::Texture2D::*;
    
}
pub mod Texture3D {
    pub use super::Texture::*;
    pub const get_format: (&'static str, u32) = ("_get_format", 3847873762u32);
    pub const get_width: (&'static str, u32) = ("_get_width", 3905245786u32);
    pub const get_height: (&'static str, u32) = ("_get_height", 3905245786u32);
    pub const get_depth: (&'static str, u32) = ("_get_depth", 3905245786u32);
    pub const has_mipmaps: (&'static str, u32) = ("_has_mipmaps", 36873697u32);
    pub const get_data: (&'static str, u32) = ("_get_data", 3995934104u32);
    
}
pub mod Texture3Drd {
    pub use super::Texture3D::*;
    
}
pub mod TextureButton {
    pub use super::BaseButton::*;
    
}
pub mod TextureCubemapArrayRd {
    pub use super::TextureLayeredRd::*;
    
}
pub mod TextureCubemapRd {
    pub use super::TextureLayeredRd::*;
    
}
pub mod TextureLayered {
    pub use super::Texture::*;
    pub const get_format: (&'static str, u32) = ("_get_format", 3847873762u32);
    pub const get_layered_type: (&'static str, u32) = ("_get_layered_type", 3905245786u32);
    pub const get_width: (&'static str, u32) = ("_get_width", 3905245786u32);
    pub const get_height: (&'static str, u32) = ("_get_height", 3905245786u32);
    pub const get_layers: (&'static str, u32) = ("_get_layers", 3905245786u32);
    pub const has_mipmaps: (&'static str, u32) = ("_has_mipmaps", 36873697u32);
    pub const get_layer_data: (&'static str, u32) = ("_get_layer_data", 3655284255u32);
    
}
pub mod TextureLayeredRd {
    pub use super::TextureLayered::*;
    
}
pub mod TextureProgressBar {
    pub use super::Range::*;
    
}
pub mod TextureRect {
    pub use super::Control::*;
    
}
pub mod Theme {
    pub use super::Resource::*;
    
}
pub mod ThemeDb {
    pub use super::Object::*;
    
}
pub mod TileData {
    pub use super::Object::*;
    
}
pub mod TileMap {
    pub use super::Node2D::*;
    pub const use_tile_data_runtime_update: (&'static str, u32) = ("_use_tile_data_runtime_update", 3957903770u32);
    pub const tile_data_runtime_update: (&'static str, u32) = ("_tile_data_runtime_update", 4223434291u32);
    
}
pub mod TileMapLayer {
    pub use super::Node2D::*;
    pub const use_tile_data_runtime_update: (&'static str, u32) = ("_use_tile_data_runtime_update", 3715736492u32);
    pub const tile_data_runtime_update: (&'static str, u32) = ("_tile_data_runtime_update", 1627322126u32);
    pub const update_cells: (&'static str, u32) = ("_update_cells", 3156113851u32);
    
}
pub mod TileMapPattern {
    pub use super::Resource::*;
    
}
pub mod TileSet {
    pub use super::Resource::*;
    
}
pub mod TileSetAtlasSource {
    pub use super::TileSetSource::*;
    
}
pub mod TileSetScenesCollectionSource {
    pub use super::TileSetSource::*;
    
}
pub mod TileSetSource {
    pub use super::Resource::*;
    
}
pub mod Time {
    pub use super::Object::*;
    
}
pub mod Timer {
    pub use super::Node::*;
    
}
pub mod TorusMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod TouchScreenButton {
    pub use super::Node2D::*;
    
}
pub mod Translation {
    pub use super::Resource::*;
    pub const get_plural_message: (&'static str, u32) = ("_get_plural_message", 1970324172u32);
    pub const get_message: (&'static str, u32) = ("_get_message", 3639719779u32);
    
}
pub mod TranslationDomain {
    pub use super::RefCounted::*;
    
}
pub mod TranslationServer {
    pub use super::Object::*;
    
}
pub mod Tree {
    pub use super::Control::*;
    
}
pub mod TreeItem {
    pub use super::Object::*;
    
}
pub mod TriangleMesh {
    pub use super::RefCounted::*;
    
}
pub mod TubeTrailMesh {
    pub use super::PrimitiveMesh::*;
    
}
pub mod Tween {
    pub use super::RefCounted::*;
    
}
pub mod Tweener {
    pub use super::RefCounted::*;
    
}
pub mod UdpServer {
    pub use super::RefCounted::*;
    
}
pub mod Upnp {
    pub use super::RefCounted::*;
    
}
pub mod UpnpDevice {
    pub use super::RefCounted::*;
    
}
pub mod UndoRedo {
    pub use super::Object::*;
    
}
pub mod UniformSetCacheRd {
    pub use super::Object::*;
    
}
pub mod VBoxContainer {
    pub use super::BoxContainer::*;
    
}
pub mod VFlowContainer {
    pub use super::FlowContainer::*;
    
}
pub mod VScrollBar {
    pub use super::ScrollBar::*;
    
}
pub mod VSeparator {
    pub use super::Separator::*;
    
}
pub mod VSlider {
    pub use super::Slider::*;
    
}
pub mod VSplitContainer {
    pub use super::SplitContainer::*;
    
}
pub mod VehicleBody3D {
    pub use super::RigidBody3D::*;
    
}
pub mod VehicleWheel3D {
    pub use super::Node3D::*;
    
}
pub mod VideoStream {
    pub use super::Resource::*;
    pub const instantiate_playback: (&'static str, u32) = ("_instantiate_playback", 294648086u32);
    
}
pub mod VideoStreamPlayback {
    pub use super::Resource::*;
    pub const stop: (&'static str, u32) = ("_stop", 3218959716u32);
    pub const play: (&'static str, u32) = ("_play", 3218959716u32);
    pub const is_playing: (&'static str, u32) = ("_is_playing", 36873697u32);
    pub const set_paused: (&'static str, u32) = ("_set_paused", 2586408642u32);
    pub const is_paused: (&'static str, u32) = ("_is_paused", 36873697u32);
    pub const get_length: (&'static str, u32) = ("_get_length", 1740695150u32);
    pub const get_playback_position: (&'static str, u32) = ("_get_playback_position", 1740695150u32);
    pub const seek: (&'static str, u32) = ("_seek", 373806689u32);
    pub const set_audio_track: (&'static str, u32) = ("_set_audio_track", 1286410249u32);
    pub const get_texture: (&'static str, u32) = ("_get_texture", 3635182373u32);
    pub const update: (&'static str, u32) = ("_update", 373806689u32);
    pub const get_channels: (&'static str, u32) = ("_get_channels", 3905245786u32);
    pub const get_mix_rate: (&'static str, u32) = ("_get_mix_rate", 3905245786u32);
    
}
pub mod VideoStreamPlayer {
    pub use super::Control::*;
    
}
pub mod VideoStreamTheora {
    pub use super::VideoStream::*;
    
}
pub mod Viewport {
    pub use super::Node::*;
    
}
pub mod ViewportTexture {
    pub use super::Texture2D::*;
    
}
pub mod VisibleOnScreenEnabler2D {
    pub use super::VisibleOnScreenNotifier2D::*;
    
}
pub mod VisibleOnScreenEnabler3D {
    pub use super::VisibleOnScreenNotifier3D::*;
    
}
pub mod VisibleOnScreenNotifier2D {
    pub use super::Node2D::*;
    
}
pub mod VisibleOnScreenNotifier3D {
    pub use super::VisualInstance3D::*;
    
}
pub mod VisualInstance3D {
    pub use super::Node3D::*;
    pub const get_aabb: (&'static str, u32) = ("_get_aabb", 1068685055u32);
    
}
pub mod VisualShader {
    pub use super::Shader::*;
    
}
pub mod VisualShaderNode {
    pub use super::Resource::*;
    
}
pub mod VisualShaderNodeBillboard {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeBooleanConstant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeBooleanParameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeClamp {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeColorConstant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeColorFunc {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeColorOp {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeColorParameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeComment {
    pub use super::VisualShaderNodeFrame::*;
    
}
pub mod VisualShaderNodeCompare {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeConstant {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeCubemap {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeCubemapParameter {
    pub use super::VisualShaderNodeTextureParameter::*;
    
}
pub mod VisualShaderNodeCurveTexture {
    pub use super::VisualShaderNodeResizableBase::*;
    
}
pub mod VisualShaderNodeCurveXyzTexture {
    pub use super::VisualShaderNodeResizableBase::*;
    
}
pub mod VisualShaderNodeCustom {
    pub use super::VisualShaderNode::*;
    pub const get_name: (&'static str, u32) = ("_get_name", 201670096u32);
    pub const get_description: (&'static str, u32) = ("_get_description", 201670096u32);
    pub const get_category: (&'static str, u32) = ("_get_category", 201670096u32);
    pub const get_return_icon_type: (&'static str, u32) = ("_get_return_icon_type", 1287173294u32);
    pub const get_input_port_count: (&'static str, u32) = ("_get_input_port_count", 3905245786u32);
    pub const get_input_port_type: (&'static str, u32) = ("_get_input_port_type", 4102573379u32);
    pub const get_input_port_name: (&'static str, u32) = ("_get_input_port_name", 844755477u32);
    pub const get_input_port_default_value: (&'static str, u32) = ("_get_input_port_default_value", 4227898402u32);
    pub const get_default_input_port: (&'static str, u32) = ("_get_default_input_port", 1894493699u32);
    pub const get_output_port_count: (&'static str, u32) = ("_get_output_port_count", 3905245786u32);
    pub const get_output_port_type: (&'static str, u32) = ("_get_output_port_type", 4102573379u32);
    pub const get_output_port_name: (&'static str, u32) = ("_get_output_port_name", 844755477u32);
    pub const get_property_count: (&'static str, u32) = ("_get_property_count", 3905245786u32);
    pub const get_property_name: (&'static str, u32) = ("_get_property_name", 844755477u32);
    pub const get_property_default_index: (&'static str, u32) = ("_get_property_default_index", 923996154u32);
    pub const get_property_options: (&'static str, u32) = ("_get_property_options", 647634434u32);
    pub const get_code: (&'static str, u32) = ("_get_code", 4287175357u32);
    pub const get_func_code: (&'static str, u32) = ("_get_func_code", 1924221678u32);
    pub const get_global_code: (&'static str, u32) = ("_get_global_code", 3956542358u32);
    pub const is_highend: (&'static str, u32) = ("_is_highend", 36873697u32);
    pub const is_available: (&'static str, u32) = ("_is_available", 1932120545u32);
    
}
pub mod VisualShaderNodeDerivativeFunc {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeDeterminant {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeDistanceFade {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeDotProduct {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeExpression {
    pub use super::VisualShaderNodeGroupBase::*;
    
}
pub mod VisualShaderNodeFaceForward {
    pub use super::VisualShaderNodeVectorBase::*;
    
}
pub mod VisualShaderNodeFloatConstant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeFloatFunc {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeFloatOp {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeFloatParameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeFrame {
    pub use super::VisualShaderNodeResizableBase::*;
    
}
pub mod VisualShaderNodeFresnel {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeGlobalExpression {
    pub use super::VisualShaderNodeExpression::*;
    
}
pub mod VisualShaderNodeGroupBase {
    pub use super::VisualShaderNodeResizableBase::*;
    
}
pub mod VisualShaderNodeIf {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeInput {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeIntConstant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeIntFunc {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeIntOp {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeIntParameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeIs {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeLinearSceneDepth {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeMix {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeMultiplyAdd {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeOuterProduct {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeOutput {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParameter {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParameterRef {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParticleAccelerator {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParticleBoxEmitter {
    pub use super::VisualShaderNodeParticleEmitter::*;
    
}
pub mod VisualShaderNodeParticleConeVelocity {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParticleEmit {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParticleEmitter {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParticleMeshEmitter {
    pub use super::VisualShaderNodeParticleEmitter::*;
    
}
pub mod VisualShaderNodeParticleMultiplyByAxisAngle {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParticleOutput {
    pub use super::VisualShaderNodeOutput::*;
    
}
pub mod VisualShaderNodeParticleRandomness {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeParticleRingEmitter {
    pub use super::VisualShaderNodeParticleEmitter::*;
    
}
pub mod VisualShaderNodeParticleSphereEmitter {
    pub use super::VisualShaderNodeParticleEmitter::*;
    
}
pub mod VisualShaderNodeProximityFade {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeRandomRange {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeRemap {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeReroute {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeResizableBase {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeRotationByAxis {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeSdfRaymarch {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeSdfToScreenUv {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeSample3D {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeScreenNormalWorldSpace {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeScreenUvToSdf {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeSmoothStep {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeStep {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeSwitch {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeTexture {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeTexture2DArray {
    pub use super::VisualShaderNodeSample3D::*;
    
}
pub mod VisualShaderNodeTexture2DArrayParameter {
    pub use super::VisualShaderNodeTextureParameter::*;
    
}
pub mod VisualShaderNodeTexture2DParameter {
    pub use super::VisualShaderNodeTextureParameter::*;
    
}
pub mod VisualShaderNodeTexture3D {
    pub use super::VisualShaderNodeSample3D::*;
    
}
pub mod VisualShaderNodeTexture3DParameter {
    pub use super::VisualShaderNodeTextureParameter::*;
    
}
pub mod VisualShaderNodeTextureParameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeTextureParameterTriplanar {
    pub use super::VisualShaderNodeTextureParameter::*;
    
}
pub mod VisualShaderNodeTextureSdf {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeTextureSdfNormal {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeTransformCompose {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeTransformConstant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeTransformDecompose {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeTransformFunc {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeTransformOp {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeTransformParameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeTransformVecMult {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeUIntConstant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeUIntFunc {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeUIntOp {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeUIntParameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeUvFunc {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeUvPolarCoord {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeVarying {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeVaryingGetter {
    pub use super::VisualShaderNodeVarying::*;
    
}
pub mod VisualShaderNodeVaryingSetter {
    pub use super::VisualShaderNodeVarying::*;
    
}
pub mod VisualShaderNodeVec2Constant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeVec2Parameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeVec3Constant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeVec3Parameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeVec4Constant {
    pub use super::VisualShaderNodeConstant::*;
    
}
pub mod VisualShaderNodeVec4Parameter {
    pub use super::VisualShaderNodeParameter::*;
    
}
pub mod VisualShaderNodeVectorBase {
    pub use super::VisualShaderNode::*;
    
}
pub mod VisualShaderNodeVectorCompose {
    pub use super::VisualShaderNodeVectorBase::*;
    
}
pub mod VisualShaderNodeVectorDecompose {
    pub use super::VisualShaderNodeVectorBase::*;
    
}
pub mod VisualShaderNodeVectorDistance {
    pub use super::VisualShaderNodeVectorBase::*;
    
}
pub mod VisualShaderNodeVectorFunc {
    pub use super::VisualShaderNodeVectorBase::*;
    
}
pub mod VisualShaderNodeVectorLen {
    pub use super::VisualShaderNodeVectorBase::*;
    
}
pub mod VisualShaderNodeVectorOp {
    pub use super::VisualShaderNodeVectorBase::*;
    
}
pub mod VisualShaderNodeVectorRefract {
    pub use super::VisualShaderNodeVectorBase::*;
    
}
pub mod VisualShaderNodeWorldPositionFromDepth {
    pub use super::VisualShaderNode::*;
    
}
pub mod VoxelGi {
    pub use super::VisualInstance3D::*;
    
}
pub mod VoxelGiData {
    pub use super::Resource::*;
    
}
pub mod WeakRef {
    pub use super::RefCounted::*;
    
}
pub mod WebRtcDataChannel {
    pub use super::PacketPeer::*;
    
}
pub mod WebRtcDataChannelExtension {
    pub use super::WebRtcDataChannel::*;
    pub const get_packet_rawptr: (&'static str, u32) = ("_get_packet", 3099858825u32);
    pub const put_packet_rawptr: (&'static str, u32) = ("_put_packet", 3099858825u32);
    pub const get_available_packet_count: (&'static str, u32) = ("_get_available_packet_count", 3905245786u32);
    pub const get_max_packet_size: (&'static str, u32) = ("_get_max_packet_size", 3905245786u32);
    pub const poll: (&'static str, u32) = ("_poll", 166280745u32);
    pub const close: (&'static str, u32) = ("_close", 3218959716u32);
    pub const set_write_mode: (&'static str, u32) = ("_set_write_mode", 1999768052u32);
    pub const get_write_mode: (&'static str, u32) = ("_get_write_mode", 2848495172u32);
    pub const was_string_packet: (&'static str, u32) = ("_was_string_packet", 36873697u32);
    pub const get_ready_state: (&'static str, u32) = ("_get_ready_state", 3501143017u32);
    pub const get_label: (&'static str, u32) = ("_get_label", 201670096u32);
    pub const is_ordered: (&'static str, u32) = ("_is_ordered", 36873697u32);
    pub const get_id: (&'static str, u32) = ("_get_id", 3905245786u32);
    pub const get_max_packet_life_time: (&'static str, u32) = ("_get_max_packet_life_time", 3905245786u32);
    pub const get_max_retransmits: (&'static str, u32) = ("_get_max_retransmits", 3905245786u32);
    pub const get_protocol: (&'static str, u32) = ("_get_protocol", 201670096u32);
    pub const is_negotiated: (&'static str, u32) = ("_is_negotiated", 36873697u32);
    pub const get_buffered_amount: (&'static str, u32) = ("_get_buffered_amount", 3905245786u32);
    
}
pub mod WebRtcMultiplayerPeer {
    pub use super::MultiplayerPeer::*;
    
}
pub mod WebRtcPeerConnection {
    pub use super::RefCounted::*;
    
}
pub mod WebRtcPeerConnectionExtension {
    pub use super::WebRtcPeerConnection::*;
    pub const get_connection_state: (&'static str, u32) = ("_get_connection_state", 2275710506u32);
    pub const get_gathering_state: (&'static str, u32) = ("_get_gathering_state", 4262591401u32);
    pub const get_signaling_state: (&'static str, u32) = ("_get_signaling_state", 3342956226u32);
    pub const initialize: (&'static str, u32) = ("_initialize", 1494659981u32);
    pub const create_data_channel: (&'static str, u32) = ("_create_data_channel", 4111292546u32);
    pub const create_offer: (&'static str, u32) = ("_create_offer", 166280745u32);
    pub const set_remote_description: (&'static str, u32) = ("_set_remote_description", 852856452u32);
    pub const set_local_description: (&'static str, u32) = ("_set_local_description", 852856452u32);
    pub const add_ice_candidate: (&'static str, u32) = ("_add_ice_candidate", 3958950400u32);
    pub const poll: (&'static str, u32) = ("_poll", 166280745u32);
    pub const close: (&'static str, u32) = ("_close", 3218959716u32);
    
}
pub mod WebSocketMultiplayerPeer {
    pub use super::MultiplayerPeer::*;
    
}
pub mod WebSocketPeer {
    pub use super::PacketPeer::*;
    
}
pub mod WebXrInterface {
    pub use super::XrInterface::*;
    
}
pub mod Window {
    pub use super::Viewport::*;
    pub const get_contents_minimum_size: (&'static str, u32) = ("_get_contents_minimum_size", 3341600327u32);
    
}
pub mod WorkerThreadPool {
    pub use super::Object::*;
    
}
pub mod World2D {
    pub use super::Resource::*;
    
}
pub mod World3D {
    pub use super::Resource::*;
    
}
pub mod WorldBoundaryShape2D {
    pub use super::Shape2D::*;
    
}
pub mod WorldBoundaryShape3D {
    pub use super::Shape3D::*;
    
}
pub mod WorldEnvironment {
    pub use super::Node::*;
    
}
pub mod X509Certificate {
    pub use super::Resource::*;
    
}
pub mod XmlParser {
    pub use super::RefCounted::*;
    
}
pub mod XrAnchor3D {
    pub use super::XrNode3D::*;
    
}
pub mod XrCamera3D {
    pub use super::Camera3D::*;
    
}
pub mod XrController3D {
    pub use super::XrNode3D::*;
    
}
pub mod XrControllerTracker {
    pub use super::XrPositionalTracker::*;
    
}
pub mod XrHandModifier3D {
    pub use super::SkeletonModifier3D::*;
    
}
pub mod XrHandTracker {
    pub use super::XrPositionalTracker::*;
    
}
pub mod XrInterface {
    pub use super::RefCounted::*;
    
}
pub mod XrInterfaceExtension {
    pub use super::XrInterface::*;
    pub const get_name: (&'static str, u32) = ("_get_name", 2002593661u32);
    pub const get_capabilities: (&'static str, u32) = ("_get_capabilities", 3905245786u32);
    pub const is_initialized: (&'static str, u32) = ("_is_initialized", 36873697u32);
    pub const initialize: (&'static str, u32) = ("_initialize", 2240911060u32);
    pub const uninitialize: (&'static str, u32) = ("_uninitialize", 3218959716u32);
    pub const get_system_info: (&'static str, u32) = ("_get_system_info", 3102165223u32);
    pub const supports_play_area_mode: (&'static str, u32) = ("_supports_play_area_mode", 2693703033u32);
    pub const get_play_area_mode: (&'static str, u32) = ("_get_play_area_mode", 1615132885u32);
    pub const set_play_area_mode: (&'static str, u32) = ("_set_play_area_mode", 2693703033u32);
    pub const get_play_area: (&'static str, u32) = ("_get_play_area", 497664490u32);
    pub const get_render_target_size: (&'static str, u32) = ("_get_render_target_size", 1497962370u32);
    pub const get_view_count: (&'static str, u32) = ("_get_view_count", 2455072627u32);
    pub const get_camera_transform: (&'static str, u32) = ("_get_camera_transform", 4183770049u32);
    pub const get_transform_for_view: (&'static str, u32) = ("_get_transform_for_view", 518934792u32);
    pub const get_projection_for_view: (&'static str, u32) = ("_get_projection_for_view", 4067457445u32);
    pub const get_vrs_texture: (&'static str, u32) = ("_get_vrs_texture", 529393457u32);
    pub const get_vrs_texture_format: (&'static str, u32) = ("_get_vrs_texture_format", 1500923256u32);
    pub const process: (&'static str, u32) = ("_process", 3218959716u32);
    pub const pre_render: (&'static str, u32) = ("_pre_render", 3218959716u32);
    pub const pre_draw_viewport: (&'static str, u32) = ("_pre_draw_viewport", 3521089500u32);
    pub const post_draw_viewport: (&'static str, u32) = ("_post_draw_viewport", 1378122625u32);
    pub const end_frame: (&'static str, u32) = ("_end_frame", 3218959716u32);
    pub const get_suggested_tracker_names: (&'static str, u32) = ("_get_suggested_tracker_names", 1139954409u32);
    pub const get_suggested_pose_names: (&'static str, u32) = ("_get_suggested_pose_names", 1761182771u32);
    pub const get_tracking_status: (&'static str, u32) = ("_get_tracking_status", 167423259u32);
    pub const trigger_haptic_pulse: (&'static str, u32) = ("_trigger_haptic_pulse", 3752640163u32);
    pub const get_anchor_detection_is_enabled: (&'static str, u32) = ("_get_anchor_detection_is_enabled", 36873697u32);
    pub const set_anchor_detection_is_enabled: (&'static str, u32) = ("_set_anchor_detection_is_enabled", 2586408642u32);
    pub const get_camera_feed_id: (&'static str, u32) = ("_get_camera_feed_id", 3905245786u32);
    pub const get_color_texture: (&'static str, u32) = ("_get_color_texture", 529393457u32);
    pub const get_depth_texture: (&'static str, u32) = ("_get_depth_texture", 529393457u32);
    pub const get_velocity_texture: (&'static str, u32) = ("_get_velocity_texture", 529393457u32);
    
}
pub mod XrNode3D {
    pub use super::Node3D::*;
    
}
pub mod XrOrigin3D {
    pub use super::Node3D::*;
    
}
pub mod XrPose {
    pub use super::RefCounted::*;
    
}
pub mod XrPositionalTracker {
    pub use super::XrTracker::*;
    
}
pub mod XrServer {
    pub use super::Object::*;
    
}
pub mod XrTracker {
    pub use super::RefCounted::*;
    
}
pub mod Xrvrs {
    pub use super::Object::*;
    
}
pub mod ZipPacker {
    pub use super::RefCounted::*;
    
}
pub mod ZipReader {
    pub use super::RefCounted::*;
    
}