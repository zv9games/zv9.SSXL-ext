#![doc = "Sidecar module for class [`CpuParticles3D`][crate::classes::CpuParticles3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CPUParticles3D` enums](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `CPUParticles3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`cpu_particles_3d`][crate::classes::cpu_particles_3d]: sidecar module with related enum/flag types\n* [`ICpuParticles3D`][crate::classes::ICpuParticles3D]: virtual methods\n* [`SignalsOfCpuParticles3D`][crate::classes::cpu_particles_3d::SignalsOfCpuParticles3D]: signal collection\n\n\nSee also [Godot docs for `CPUParticles3D`](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CpuParticles3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CpuParticles3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CpuParticles3D`][crate::classes::CpuParticles3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D] > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CPUParticles3D` methods](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICpuParticles3D: crate::obj::GodotClass < Base = CpuParticles3D > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn get_property(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn set_property(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn get_property_list(&mut self) -> Vec < crate::meta::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn validate_property(&self, property: &mut crate::meta::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_accessibility_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn get_focused_accessibility_element(&self,) -> Rid {
            unimplemented !()
        }
    }
    impl CpuParticles3D {
        pub fn set_emitting(&mut self, emitting: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (emitting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_amount(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime(&mut self, secs: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_aabb(&mut self, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Aabb,);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_visibility_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lifetime_randomness(&mut self, random: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (random,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emitting(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "is_emitting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_amount(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_amount", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_lifetime", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pre_process_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_pre_process_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_explosiveness_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_explosiveness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_randomness_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_randomness_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_visibility_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lifetime_randomness(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_lifetime_randomness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_local_coordinates(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_use_local_coordinates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fixed_fps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_fixed_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractional_delta(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_fractional_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_order(&mut self, order: crate::classes::cpu_particles_3d::DrawOrder,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_3d::DrawOrder,);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_draw_order(&self,) -> crate::classes::cpu_particles_3d::DrawOrder {
            type CallRet = crate::classes::cpu_particles_3d::DrawOrder;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_draw_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mesh(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh >> >,);
            let args = (mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::classes::Mesh > > {
            type CallRet = Option < Gd < crate::classes::Mesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_fixed_seed(&mut self, use_fixed_seed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_fixed_seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_use_fixed_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_fixed_seed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_use_fixed_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_seed(&mut self, seed: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_seed(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn restart_full(&mut self, keep_seed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep_seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "restart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::restart_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn restart(&mut self,) {
            self.restart_ex() . done()
        }
        #[inline]
        pub fn restart_ex < 'a > (&'a mut self,) -> ExRestart < 'a > {
            ExRestart::new(self,)
        }
        pub fn request_particles_process(&mut self, process_time: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (process_time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "request_particles_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capture_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "capture_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_direction(&mut self, direction: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_direction(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spread(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_spread(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_spread", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flatness(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_flatness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_flatness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_flatness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_min(&mut self, param: crate::classes::cpu_particles_3d::Parameter, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_3d::Parameter, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_min(&self, param: crate::classes::cpu_particles_3d::Parameter,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::cpu_particles_3d::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_param_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_max(&mut self, param: crate::classes::cpu_particles_3d::Parameter, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_3d::Parameter, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1499usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_max(&self, param: crate::classes::cpu_particles_3d::Parameter,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::cpu_particles_3d::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_param_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_param_curve(&mut self, param: crate::classes::cpu_particles_3d::Parameter, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::cpu_particles_3d::Parameter, CowArg < 'a0, Option < Gd < crate::classes::Curve >> >,);
            let args = (param, curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1501usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_param_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param_curve(&self, param: crate::classes::cpu_particles_3d::Parameter,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = (crate::classes::cpu_particles_3d::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1502usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_param_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1503usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1504usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_ramp(&mut self, ramp: impl AsArg < Option < Gd < crate::classes::Gradient >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Gradient >> >,);
            let args = (ramp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1505usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallRet = Option < Gd < crate::classes::Gradient > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1506usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_initial_ramp(&mut self, ramp: impl AsArg < Option < Gd < crate::classes::Gradient >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Gradient >> >,);
            let args = (ramp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1507usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_initial_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallRet = Option < Gd < crate::classes::Gradient > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1508usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_color_initial_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_particle_flag(&mut self, particle_flag: crate::classes::cpu_particles_3d::ParticleFlags, enable: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_3d::ParticleFlags, bool,);
            let args = (particle_flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1509usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_particle_flag(&self, particle_flag: crate::classes::cpu_particles_3d::ParticleFlags,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::cpu_particles_3d::ParticleFlags,);
            let args = (particle_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1510usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_particle_flag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_shape(&mut self, shape: crate::classes::cpu_particles_3d::EmissionShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_3d::EmissionShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1511usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_shape(&self,) -> crate::classes::cpu_particles_3d::EmissionShape {
            type CallRet = crate::classes::cpu_particles_3d::EmissionShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1512usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_sphere_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1513usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_sphere_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1514usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_sphere_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_box_extents(&mut self, extents: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1515usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_box_extents(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1516usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_box_extents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_points(&mut self, array: &PackedVector3Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >,);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1517usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_points(&self,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1518usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_normals(&mut self, array: &PackedVector3Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >,);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_normals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_normals(&self,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_normals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_colors(&mut self, array: &PackedColorArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedColorArray >,);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_colors(&self,) -> PackedColorArray {
            type CallRet = PackedColorArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_axis(&mut self, axis: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_ring_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_axis(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_ring_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_height(&mut self, height: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_ring_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_height(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_ring_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_ring_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_ring_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_inner_radius(&mut self, inner_radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (inner_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_ring_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_inner_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_ring_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_ring_cone_angle(&mut self, cone_angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (cone_angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_emission_ring_cone_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_ring_cone_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_emission_ring_cone_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, accel_vec: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (accel_vec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1534usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_split_scale(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1535usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_split_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_split_scale(&mut self, split_scale: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (split_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1536usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_split_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_curve_x(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1537usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_scale_curve_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_curve_x(&mut self, scale_curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve >> >,);
            let args = (scale_curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_scale_curve_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_curve_y(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_scale_curve_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_curve_y(&mut self, scale_curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve >> >,);
            let args = (scale_curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_scale_curve_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_curve_z(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "get_scale_curve_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_curve_z(&mut self, scale_curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve >> >,);
            let args = (scale_curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "set_scale_curve_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert_from_particles(&mut self, particles: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node >> >,);
            let args = (particles.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles3D", "convert_from_particles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for CpuParticles3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"CPUParticles3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CpuParticles3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for CpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for CpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CpuParticles3D {
        
    }
    impl crate::obj::cap::GodotDefault for CpuParticles3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CpuParticles3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CpuParticles3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CpuParticles3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CpuParticles3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CpuParticles3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`CpuParticles3D::restart_ex`][super::CpuParticles3D::restart_ex]."]
#[must_use]
pub struct ExRestart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CpuParticles3D, keep_seed: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRestart < 'a > {
    fn new(surround_object: &'a mut re_export::CpuParticles3D,) -> Self {
        let keep_seed = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, keep_seed: keep_seed,
        }
    }
    #[inline]
    pub fn keep_seed(self, keep_seed: bool) -> Self {
        Self {
            keep_seed: keep_seed, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, keep_seed,
        }
        = self;
        re_export::CpuParticles3D::restart_full(surround_object, keep_seed,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DrawOrder {
    ord: i32
}
impl DrawOrder {
    #[doc(alias = "DRAW_ORDER_INDEX")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_INDEX`"]
    pub const INDEX: DrawOrder = DrawOrder {
        ord: 0i32
    };
    #[doc(alias = "DRAW_ORDER_LIFETIME")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_LIFETIME`"]
    pub const LIFETIME: DrawOrder = DrawOrder {
        ord: 1i32
    };
    #[doc(alias = "DRAW_ORDER_VIEW_DEPTH")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_VIEW_DEPTH`"]
    pub const VIEW_DEPTH: DrawOrder = DrawOrder {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DrawOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawOrder") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawOrder {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INDEX => "INDEX", Self::LIFETIME => "LIFETIME", Self::VIEW_DEPTH => "VIEW_DEPTH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DrawOrder::INDEX, DrawOrder::LIFETIME, DrawOrder::VIEW_DEPTH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DrawOrder >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INDEX", "DRAW_ORDER_INDEX", DrawOrder::INDEX), crate::meta::inspect::EnumConstant::new("LIFETIME", "DRAW_ORDER_LIFETIME", DrawOrder::LIFETIME), crate::meta::inspect::EnumConstant::new("VIEW_DEPTH", "DRAW_ORDER_VIEW_DEPTH", DrawOrder::VIEW_DEPTH)]
        }
    }
}
impl crate::meta::GodotConvert for DrawOrder {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DrawOrder {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawOrder {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Parameter {
    ord: i32
}
impl Parameter {
    #[doc(alias = "PARAM_INITIAL_LINEAR_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_INITIAL_LINEAR_VELOCITY`"]
    pub const INITIAL_LINEAR_VELOCITY: Parameter = Parameter {
        ord: 0i32
    };
    #[doc(alias = "PARAM_ANGULAR_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_VELOCITY`"]
    pub const ANGULAR_VELOCITY: Parameter = Parameter {
        ord: 1i32
    };
    #[doc(alias = "PARAM_ORBIT_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ORBIT_VELOCITY`"]
    pub const ORBIT_VELOCITY: Parameter = Parameter {
        ord: 2i32
    };
    #[doc(alias = "PARAM_LINEAR_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_ACCEL`"]
    pub const LINEAR_ACCEL: Parameter = Parameter {
        ord: 3i32
    };
    #[doc(alias = "PARAM_RADIAL_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_RADIAL_ACCEL`"]
    pub const RADIAL_ACCEL: Parameter = Parameter {
        ord: 4i32
    };
    #[doc(alias = "PARAM_TANGENTIAL_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_TANGENTIAL_ACCEL`"]
    pub const TANGENTIAL_ACCEL: Parameter = Parameter {
        ord: 5i32
    };
    #[doc(alias = "PARAM_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_DAMPING`"]
    pub const DAMPING: Parameter = Parameter {
        ord: 6i32
    };
    #[doc(alias = "PARAM_ANGLE")]
    #[doc = "Godot enumerator name: `PARAM_ANGLE`"]
    pub const ANGLE: Parameter = Parameter {
        ord: 7i32
    };
    #[doc(alias = "PARAM_SCALE")]
    #[doc = "Godot enumerator name: `PARAM_SCALE`"]
    pub const SCALE: Parameter = Parameter {
        ord: 8i32
    };
    #[doc(alias = "PARAM_HUE_VARIATION")]
    #[doc = "Godot enumerator name: `PARAM_HUE_VARIATION`"]
    pub const HUE_VARIATION: Parameter = Parameter {
        ord: 9i32
    };
    #[doc(alias = "PARAM_ANIM_SPEED")]
    #[doc = "Godot enumerator name: `PARAM_ANIM_SPEED`"]
    pub const ANIM_SPEED: Parameter = Parameter {
        ord: 10i32
    };
    #[doc(alias = "PARAM_ANIM_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_ANIM_OFFSET`"]
    pub const ANIM_OFFSET: Parameter = Parameter {
        ord: 11i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Parameter = Parameter {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Parameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Parameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INITIAL_LINEAR_VELOCITY => "INITIAL_LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "ANGULAR_VELOCITY", Self::ORBIT_VELOCITY => "ORBIT_VELOCITY", Self::LINEAR_ACCEL => "LINEAR_ACCEL", Self::RADIAL_ACCEL => "RADIAL_ACCEL", Self::TANGENTIAL_ACCEL => "TANGENTIAL_ACCEL", Self::DAMPING => "DAMPING", Self::ANGLE => "ANGLE", Self::SCALE => "SCALE", Self::HUE_VARIATION => "HUE_VARIATION", Self::ANIM_SPEED => "ANIM_SPEED", Self::ANIM_OFFSET => "ANIM_OFFSET", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Parameter::INITIAL_LINEAR_VELOCITY, Parameter::ANGULAR_VELOCITY, Parameter::ORBIT_VELOCITY, Parameter::LINEAR_ACCEL, Parameter::RADIAL_ACCEL, Parameter::TANGENTIAL_ACCEL, Parameter::DAMPING, Parameter::ANGLE, Parameter::SCALE, Parameter::HUE_VARIATION, Parameter::ANIM_SPEED, Parameter::ANIM_OFFSET]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Parameter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INITIAL_LINEAR_VELOCITY", "PARAM_INITIAL_LINEAR_VELOCITY", Parameter::INITIAL_LINEAR_VELOCITY), crate::meta::inspect::EnumConstant::new("ANGULAR_VELOCITY", "PARAM_ANGULAR_VELOCITY", Parameter::ANGULAR_VELOCITY), crate::meta::inspect::EnumConstant::new("ORBIT_VELOCITY", "PARAM_ORBIT_VELOCITY", Parameter::ORBIT_VELOCITY), crate::meta::inspect::EnumConstant::new("LINEAR_ACCEL", "PARAM_LINEAR_ACCEL", Parameter::LINEAR_ACCEL), crate::meta::inspect::EnumConstant::new("RADIAL_ACCEL", "PARAM_RADIAL_ACCEL", Parameter::RADIAL_ACCEL), crate::meta::inspect::EnumConstant::new("TANGENTIAL_ACCEL", "PARAM_TANGENTIAL_ACCEL", Parameter::TANGENTIAL_ACCEL), crate::meta::inspect::EnumConstant::new("DAMPING", "PARAM_DAMPING", Parameter::DAMPING), crate::meta::inspect::EnumConstant::new("ANGLE", "PARAM_ANGLE", Parameter::ANGLE), crate::meta::inspect::EnumConstant::new("SCALE", "PARAM_SCALE", Parameter::SCALE), crate::meta::inspect::EnumConstant::new("HUE_VARIATION", "PARAM_HUE_VARIATION", Parameter::HUE_VARIATION), crate::meta::inspect::EnumConstant::new("ANIM_SPEED", "PARAM_ANIM_SPEED", Parameter::ANIM_SPEED), crate::meta::inspect::EnumConstant::new("ANIM_OFFSET", "PARAM_ANIM_OFFSET", Parameter::ANIM_OFFSET), crate::meta::inspect::EnumConstant::new("MAX", "PARAM_MAX", Parameter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Parameter {
    const ENUMERATOR_COUNT: usize = 12usize;
    
}
impl crate::meta::GodotConvert for Parameter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Parameter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Parameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticleFlags {
    ord: i32
}
impl ParticleFlags {
    #[doc(alias = "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY`"]
    pub const ALIGN_Y_TO_VELOCITY: ParticleFlags = ParticleFlags {
        ord: 0i32
    };
    #[doc(alias = "PARTICLE_FLAG_ROTATE_Y")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_ROTATE_Y`"]
    pub const ROTATE_Y: ParticleFlags = ParticleFlags {
        ord: 1i32
    };
    #[doc(alias = "PARTICLE_FLAG_DISABLE_Z")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_DISABLE_Z`"]
    pub const DISABLE_Z: ParticleFlags = ParticleFlags {
        ord: 2i32
    };
    #[doc(alias = "PARTICLE_FLAG_MAX")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_MAX`"]
    pub const MAX: ParticleFlags = ParticleFlags {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ParticleFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticleFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticleFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ALIGN_Y_TO_VELOCITY => "ALIGN_Y_TO_VELOCITY", Self::ROTATE_Y => "ROTATE_Y", Self::DISABLE_Z => "DISABLE_Z", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticleFlags::ALIGN_Y_TO_VELOCITY, ParticleFlags::ROTATE_Y, ParticleFlags::DISABLE_Z]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticleFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALIGN_Y_TO_VELOCITY", "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY", ParticleFlags::ALIGN_Y_TO_VELOCITY), crate::meta::inspect::EnumConstant::new("ROTATE_Y", "PARTICLE_FLAG_ROTATE_Y", ParticleFlags::ROTATE_Y), crate::meta::inspect::EnumConstant::new("DISABLE_Z", "PARTICLE_FLAG_DISABLE_Z", ParticleFlags::DISABLE_Z), crate::meta::inspect::EnumConstant::new("MAX", "PARTICLE_FLAG_MAX", ParticleFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ParticleFlags {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ParticleFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ParticleFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticleFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EmissionShape {
    ord: i32
}
impl EmissionShape {
    #[doc(alias = "EMISSION_SHAPE_POINT")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_POINT`"]
    pub const POINT: EmissionShape = EmissionShape {
        ord: 0i32
    };
    #[doc(alias = "EMISSION_SHAPE_SPHERE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_SPHERE`"]
    pub const SPHERE: EmissionShape = EmissionShape {
        ord: 1i32
    };
    #[doc(alias = "EMISSION_SHAPE_SPHERE_SURFACE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_SPHERE_SURFACE`"]
    pub const SPHERE_SURFACE: EmissionShape = EmissionShape {
        ord: 2i32
    };
    #[doc(alias = "EMISSION_SHAPE_BOX")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_BOX`"]
    pub const BOX: EmissionShape = EmissionShape {
        ord: 3i32
    };
    #[doc(alias = "EMISSION_SHAPE_POINTS")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_POINTS`"]
    pub const POINTS: EmissionShape = EmissionShape {
        ord: 4i32
    };
    #[doc(alias = "EMISSION_SHAPE_DIRECTED_POINTS")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_DIRECTED_POINTS`"]
    pub const DIRECTED_POINTS: EmissionShape = EmissionShape {
        ord: 5i32
    };
    #[doc(alias = "EMISSION_SHAPE_RING")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_RING`"]
    pub const RING: EmissionShape = EmissionShape {
        ord: 6i32
    };
    #[doc(alias = "EMISSION_SHAPE_MAX")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_MAX`"]
    pub const MAX: EmissionShape = EmissionShape {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for EmissionShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EmissionShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EmissionShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::POINT => "POINT", Self::SPHERE => "SPHERE", Self::SPHERE_SURFACE => "SPHERE_SURFACE", Self::BOX => "BOX", Self::POINTS => "POINTS", Self::DIRECTED_POINTS => "DIRECTED_POINTS", Self::RING => "RING", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EmissionShape::POINT, EmissionShape::SPHERE, EmissionShape::SPHERE_SURFACE, EmissionShape::BOX, EmissionShape::POINTS, EmissionShape::DIRECTED_POINTS, EmissionShape::RING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EmissionShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINT", "EMISSION_SHAPE_POINT", EmissionShape::POINT), crate::meta::inspect::EnumConstant::new("SPHERE", "EMISSION_SHAPE_SPHERE", EmissionShape::SPHERE), crate::meta::inspect::EnumConstant::new("SPHERE_SURFACE", "EMISSION_SHAPE_SPHERE_SURFACE", EmissionShape::SPHERE_SURFACE), crate::meta::inspect::EnumConstant::new("BOX", "EMISSION_SHAPE_BOX", EmissionShape::BOX), crate::meta::inspect::EnumConstant::new("POINTS", "EMISSION_SHAPE_POINTS", EmissionShape::POINTS), crate::meta::inspect::EnumConstant::new("DIRECTED_POINTS", "EMISSION_SHAPE_DIRECTED_POINTS", EmissionShape::DIRECTED_POINTS), crate::meta::inspect::EnumConstant::new("RING", "EMISSION_SHAPE_RING", EmissionShape::RING), crate::meta::inspect::EnumConstant::new("MAX", "EMISSION_SHAPE_MAX", EmissionShape::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EmissionShape {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for EmissionShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EmissionShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EmissionShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CpuParticles3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CpuParticles3D`][crate::classes::CpuParticles3D] class."]
    pub struct SignalsOfCpuParticles3D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCpuParticles3D < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn finished(&mut self) -> SigFinished < 'c, C > {
            SigFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "finished")
            }
        }
    }
    type TypedSigFinished < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFinished < 'c, C: WithSignals > {
        typed: TypedSigFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFinished < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFinished < 'c, C > {
        type Target = TypedSigFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CpuParticles3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCpuParticles3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCpuParticles3D < 'c, C > {
        type Target = < < CpuParticles3D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CpuParticles3D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCpuParticles3D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CpuParticles3D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}