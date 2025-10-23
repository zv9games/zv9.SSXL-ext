#![doc = "Sidecar module for class [`AStar3D`][crate::classes::AStar3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AStar3D` enums](https://docs.godotengine.org/en/stable/classes/class_astar3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AStar3D.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`a_star_3d`][crate::classes::a_star_3d]: sidecar module with related enum/flag types\n* [`IAStar3D`][crate::classes::IAStar3D]: virtual methods\n\n\nSee also [Godot docs for `AStar3D`](https://docs.godotengine.org/en/stable/classes/class_astar3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AStar3D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AStar3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AStar3D`][crate::classes::AStar3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AStar3D` methods](https://docs.godotengine.org/en/stable/classes/class_astar3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAStar3D: crate::obj::GodotClass < Base = AStar3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ObjectNotification) {
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
        fn filter_neighbor(&self, from_id: i64, neighbor_id: i64,) -> bool {
            unimplemented !()
        }
        fn estimate_cost(&self, from_id: i64, end_id: i64,) -> f32 {
            unimplemented !()
        }
        fn compute_cost(&self, from_id: i64, to_id: i64,) -> f32 {
            unimplemented !()
        }
    }
    impl AStar3D {
        pub fn get_available_point_id(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(29usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_available_point_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_point_full(&mut self, id: i64, position: Vector3, weight_scale: f32,) {
            type CallRet = ();
            type CallParams = (i64, Vector3, f32,);
            let args = (id, position, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(30usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "add_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_point(&mut self, id: i64, position: Vector3,) {
            self.add_point_ex(id, position,) . done()
        }
        #[inline]
        pub fn add_point_ex < 'a > (&'a mut self, id: i64, position: Vector3,) -> ExAddPoint < 'a > {
            ExAddPoint::new(self, id, position,)
        }
        pub fn get_point_position(&self, id: i64,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(31usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_position(&mut self, id: i64, position: Vector3,) {
            type CallRet = ();
            type CallParams = (i64, Vector3,);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(32usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "set_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_weight_scale(&self, id: i64,) -> f32 {
            type CallRet = f32;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(33usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_point_weight_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_weight_scale(&mut self, id: i64, weight_scale: f32,) {
            type CallRet = ();
            type CallParams = (i64, f32,);
            let args = (id, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(34usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "set_point_weight_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_point(&mut self, id: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(35usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "remove_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_point(&self, id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(36usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "has_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_connections(&mut self, id: i64,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(37usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_point_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_ids(&mut self,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(38usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_point_ids", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_point_disabled_full(&mut self, id: i64, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i64, bool,);
            let args = (id, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(39usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "set_point_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_point_disabled_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_point_disabled(&mut self, id: i64,) {
            self.set_point_disabled_ex(id,) . done()
        }
        #[inline]
        pub fn set_point_disabled_ex < 'a > (&'a mut self, id: i64,) -> ExSetPointDisabled < 'a > {
            ExSetPointDisabled::new(self, id,)
        }
        pub fn is_point_disabled(&self, id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(40usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "is_point_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_neighbor_filter_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(41usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "set_neighbor_filter_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_neighbor_filter_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(42usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "is_neighbor_filter_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn connect_points_full(&mut self, id: i64, to_id: i64, bidirectional: bool,) {
            type CallRet = ();
            type CallParams = (i64, i64, bool,);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(43usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "connect_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::connect_points_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn connect_points(&mut self, id: i64, to_id: i64,) {
            self.connect_points_ex(id, to_id,) . done()
        }
        #[inline]
        pub fn connect_points_ex < 'a > (&'a mut self, id: i64, to_id: i64,) -> ExConnectPoints < 'a > {
            ExConnectPoints::new(self, id, to_id,)
        }
        pub(crate) fn disconnect_points_full(&mut self, id: i64, to_id: i64, bidirectional: bool,) {
            type CallRet = ();
            type CallParams = (i64, i64, bool,);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(44usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "disconnect_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::disconnect_points_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn disconnect_points(&mut self, id: i64, to_id: i64,) {
            self.disconnect_points_ex(id, to_id,) . done()
        }
        #[inline]
        pub fn disconnect_points_ex < 'a > (&'a mut self, id: i64, to_id: i64,) -> ExDisconnectPoints < 'a > {
            ExDisconnectPoints::new(self, id, to_id,)
        }
        pub(crate) fn are_points_connected_full(&self, id: i64, to_id: i64, bidirectional: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i64, i64, bool,);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(45usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "are_points_connected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::are_points_connected_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn are_points_connected(&self, id: i64, to_id: i64,) -> bool {
            self.are_points_connected_ex(id, to_id,) . done()
        }
        #[inline]
        pub fn are_points_connected_ex < 'a > (&'a self, id: i64, to_id: i64,) -> ExArePointsConnected < 'a > {
            ExArePointsConnected::new(self, id, to_id,)
        }
        pub fn get_point_count(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(46usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_capacity(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(47usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_point_capacity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reserve_space(&mut self, num_nodes: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (num_nodes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(48usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "reserve_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(49usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_closest_point_full(&self, to_position: Vector3, include_disabled: bool,) -> i64 {
            type CallRet = i64;
            type CallParams = (Vector3, bool,);
            let args = (to_position, include_disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(50usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_closest_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_closest_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_closest_point(&self, to_position: Vector3,) -> i64 {
            self.get_closest_point_ex(to_position,) . done()
        }
        #[inline]
        pub fn get_closest_point_ex < 'a > (&'a self, to_position: Vector3,) -> ExGetClosestPoint < 'a > {
            ExGetClosestPoint::new(self, to_position,)
        }
        pub fn get_closest_position_in_segment(&self, to_position: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(51usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_closest_position_in_segment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_point_path_full(&mut self, from_id: i64, to_id: i64, allow_partial_path: bool,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (i64, i64, bool,);
            let args = (from_id, to_id, allow_partial_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(52usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_point_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_point_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_point_path(&mut self, from_id: i64, to_id: i64,) -> PackedVector3Array {
            self.get_point_path_ex(from_id, to_id,) . done()
        }
        #[inline]
        pub fn get_point_path_ex < 'a > (&'a mut self, from_id: i64, to_id: i64,) -> ExGetPointPath < 'a > {
            ExGetPointPath::new(self, from_id, to_id,)
        }
        pub(crate) fn get_id_path_full(&mut self, from_id: i64, to_id: i64, allow_partial_path: bool,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (i64, i64, bool,);
            let args = (from_id, to_id, allow_partial_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(53usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar3D", "get_id_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_id_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_id_path(&mut self, from_id: i64, to_id: i64,) -> PackedInt64Array {
            self.get_id_path_ex(from_id, to_id,) . done()
        }
        #[inline]
        pub fn get_id_path_ex < 'a > (&'a mut self, from_id: i64, to_id: i64,) -> ExGetIdPath < 'a > {
            ExGetIdPath::new(self, from_id, to_id,)
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
    impl crate::obj::GodotClass for AStar3D {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"AStar3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AStar3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AStar3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AStar3D {
        
    }
    impl crate::obj::cap::GodotDefault for AStar3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AStar3D {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AStar3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AStar3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AStar3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AStar3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AStar3D::add_point_ex`][super::AStar3D::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStar3D, id: i64, position: Vector3, weight_scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPoint < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, id: i64, position: Vector3,) -> Self {
        let weight_scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, position: position, weight_scale: weight_scale,
        }
    }
    #[inline]
    pub fn weight_scale(self, weight_scale: f32) -> Self {
        Self {
            weight_scale: weight_scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, position, weight_scale,
        }
        = self;
        re_export::AStar3D::add_point_full(surround_object, id, position, weight_scale,)
    }
}
#[doc = "Default-param extender for [`AStar3D::set_point_disabled_ex`][super::AStar3D::set_point_disabled_ex]."]
#[must_use]
pub struct ExSetPointDisabled < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStar3D, id: i64, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetPointDisabled < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, id: i64,) -> Self {
        let disabled = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, disabled: disabled,
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, disabled,
        }
        = self;
        re_export::AStar3D::set_point_disabled_full(surround_object, id, disabled,)
    }
}
#[doc = "Default-param extender for [`AStar3D::connect_points_ex`][super::AStar3D::connect_points_ex]."]
#[must_use]
pub struct ExConnectPoints < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStar3D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectPoints < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, id: i64, to_id: i64,) -> Self {
        let bidirectional = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, to_id: to_id, bidirectional: bidirectional,
        }
    }
    #[inline]
    pub fn bidirectional(self, bidirectional: bool) -> Self {
        Self {
            bidirectional: bidirectional, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, to_id, bidirectional,
        }
        = self;
        re_export::AStar3D::connect_points_full(surround_object, id, to_id, bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar3D::disconnect_points_ex`][super::AStar3D::disconnect_points_ex]."]
#[must_use]
pub struct ExDisconnectPoints < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStar3D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDisconnectPoints < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, id: i64, to_id: i64,) -> Self {
        let bidirectional = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, to_id: to_id, bidirectional: bidirectional,
        }
    }
    #[inline]
    pub fn bidirectional(self, bidirectional: bool) -> Self {
        Self {
            bidirectional: bidirectional, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, to_id, bidirectional,
        }
        = self;
        re_export::AStar3D::disconnect_points_full(surround_object, id, to_id, bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar3D::are_points_connected_ex`][super::AStar3D::are_points_connected_ex]."]
#[must_use]
pub struct ExArePointsConnected < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::AStar3D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExArePointsConnected < 'a > {
    fn new(surround_object: &'a re_export::AStar3D, id: i64, to_id: i64,) -> Self {
        let bidirectional = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, to_id: to_id, bidirectional: bidirectional,
        }
    }
    #[inline]
    pub fn bidirectional(self, bidirectional: bool) -> Self {
        Self {
            bidirectional: bidirectional, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, id, to_id, bidirectional,
        }
        = self;
        re_export::AStar3D::are_points_connected_full(surround_object, id, to_id, bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar3D::get_closest_point_ex`][super::AStar3D::get_closest_point_ex]."]
#[must_use]
pub struct ExGetClosestPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::AStar3D, to_position: Vector3, include_disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetClosestPoint < 'a > {
    fn new(surround_object: &'a re_export::AStar3D, to_position: Vector3,) -> Self {
        let include_disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position, include_disabled: include_disabled,
        }
    }
    #[inline]
    pub fn include_disabled(self, include_disabled: bool) -> Self {
        Self {
            include_disabled: include_disabled, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, to_position, include_disabled,
        }
        = self;
        re_export::AStar3D::get_closest_point_full(surround_object, to_position, include_disabled,)
    }
}
#[doc = "Default-param extender for [`AStar3D::get_point_path_ex`][super::AStar3D::get_point_path_ex]."]
#[must_use]
pub struct ExGetPointPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStar3D, from_id: i64, to_id: i64, allow_partial_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPointPath < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, from_id: i64, to_id: i64,) -> Self {
        let allow_partial_path = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_id: from_id, to_id: to_id, allow_partial_path: allow_partial_path,
        }
    }
    #[inline]
    pub fn allow_partial_path(self, allow_partial_path: bool) -> Self {
        Self {
            allow_partial_path: allow_partial_path, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedVector3Array {
        let Self {
            _phantom, surround_object, from_id, to_id, allow_partial_path,
        }
        = self;
        re_export::AStar3D::get_point_path_full(surround_object, from_id, to_id, allow_partial_path,)
    }
}
#[doc = "Default-param extender for [`AStar3D::get_id_path_ex`][super::AStar3D::get_id_path_ex]."]
#[must_use]
pub struct ExGetIdPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStar3D, from_id: i64, to_id: i64, allow_partial_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetIdPath < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, from_id: i64, to_id: i64,) -> Self {
        let allow_partial_path = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_id: from_id, to_id: to_id, allow_partial_path: allow_partial_path,
        }
    }
    #[inline]
    pub fn allow_partial_path(self, allow_partial_path: bool) -> Self {
        Self {
            allow_partial_path: allow_partial_path, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, from_id, to_id, allow_partial_path,
        }
        = self;
        re_export::AStar3D::get_id_path_full(surround_object, from_id, to_id, allow_partial_path,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AStar3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AStar3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}