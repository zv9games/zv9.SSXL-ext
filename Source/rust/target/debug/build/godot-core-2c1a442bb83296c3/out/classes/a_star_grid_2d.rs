#![doc = "Sidecar module for class [`AStarGrid2D`][crate::classes::AStarGrid2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AStarGrid2D` enums](https://docs.godotengine.org/en/stable/classes/class_astargrid2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AStarGrid2D.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`a_star_grid_2d`][crate::classes::a_star_grid_2d]: sidecar module with related enum/flag types\n* [`IAStarGrid2D`][crate::classes::IAStarGrid2D]: virtual methods\n\n\nSee also [Godot docs for `AStarGrid2D`](https://docs.godotengine.org/en/stable/classes/class_astargrid2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AStarGrid2D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AStarGrid2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AStarGrid2D`][crate::classes::AStarGrid2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AStarGrid2D` methods](https://docs.godotengine.org/en/stable/classes/class_astargrid2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAStarGrid2D: crate::obj::GodotClass < Base = AStarGrid2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn estimate_cost(&self, from_id: Vector2i, end_id: Vector2i,) -> f32 {
            unimplemented !()
        }
        fn compute_cost(&self, from_id: Vector2i, to_id: Vector2i,) -> f32 {
            unimplemented !()
        }
    }
    impl AStarGrid2D {
        pub fn set_region(&mut self, region: Rect2i,) {
            type CallRet = ();
            type CallParams = (Rect2i,);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(54usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_region(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(55usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(56usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(57usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(58usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(59usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_size(&mut self, cell_size: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (cell_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(60usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(61usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_cell_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cell_shape(&mut self, cell_shape: crate::classes::a_star_grid_2d::CellShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::a_star_grid_2d::CellShape,);
            let args = (cell_shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(62usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_cell_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_shape(&self,) -> crate::classes::a_star_grid_2d::CellShape {
            type CallRet = crate::classes::a_star_grid_2d::CellShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(63usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_cell_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_bounds(&self, x: i32, y: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(64usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "is_in_bounds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_boundsv(&self, id: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(65usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "is_in_boundsv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_dirty(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(66usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "is_dirty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(67usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_jumping_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(68usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_jumping_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_jumping_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(69usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "is_jumping_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_diagonal_mode(&mut self, mode: crate::classes::a_star_grid_2d::DiagonalMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::a_star_grid_2d::DiagonalMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(70usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_diagonal_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_diagonal_mode(&self,) -> crate::classes::a_star_grid_2d::DiagonalMode {
            type CallRet = crate::classes::a_star_grid_2d::DiagonalMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(71usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_diagonal_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_compute_heuristic(&mut self, heuristic: crate::classes::a_star_grid_2d::Heuristic,) {
            type CallRet = ();
            type CallParams = (crate::classes::a_star_grid_2d::Heuristic,);
            let args = (heuristic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(72usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_default_compute_heuristic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_compute_heuristic(&self,) -> crate::classes::a_star_grid_2d::Heuristic {
            type CallRet = crate::classes::a_star_grid_2d::Heuristic;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(73usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_default_compute_heuristic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_estimate_heuristic(&mut self, heuristic: crate::classes::a_star_grid_2d::Heuristic,) {
            type CallRet = ();
            type CallParams = (crate::classes::a_star_grid_2d::Heuristic,);
            let args = (heuristic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(74usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_default_estimate_heuristic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_estimate_heuristic(&self,) -> crate::classes::a_star_grid_2d::Heuristic {
            type CallRet = crate::classes::a_star_grid_2d::Heuristic;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(75usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_default_estimate_heuristic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_point_solid_full(&mut self, id: Vector2i, solid: bool,) {
            type CallRet = ();
            type CallParams = (Vector2i, bool,);
            let args = (id, solid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(76usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_point_solid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_point_solid_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_point_solid(&mut self, id: Vector2i,) {
            self.set_point_solid_ex(id,) . done()
        }
        #[inline]
        pub fn set_point_solid_ex < 'a > (&'a mut self, id: Vector2i,) -> ExSetPointSolid < 'a > {
            ExSetPointSolid::new(self, id,)
        }
        pub fn is_point_solid(&self, id: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(77usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "is_point_solid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_weight_scale(&mut self, id: Vector2i, weight_scale: f32,) {
            type CallRet = ();
            type CallParams = (Vector2i, f32,);
            let args = (id, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(78usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "set_point_weight_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_weight_scale(&self, id: Vector2i,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2i,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(79usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_point_weight_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn fill_solid_region_full(&mut self, region: Rect2i, solid: bool,) {
            type CallRet = ();
            type CallParams = (Rect2i, bool,);
            let args = (region, solid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(80usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "fill_solid_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::fill_solid_region_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn fill_solid_region(&mut self, region: Rect2i,) {
            self.fill_solid_region_ex(region,) . done()
        }
        #[inline]
        pub fn fill_solid_region_ex < 'a > (&'a mut self, region: Rect2i,) -> ExFillSolidRegion < 'a > {
            ExFillSolidRegion::new(self, region,)
        }
        pub fn fill_weight_scale_region(&mut self, region: Rect2i, weight_scale: f32,) {
            type CallRet = ();
            type CallParams = (Rect2i, f32,);
            let args = (region, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(81usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "fill_weight_scale_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(82usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_position(&self, id: Vector2i,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2i,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(83usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_data_in_region(&self, region: Rect2i,) -> Array < Dictionary > {
            type CallRet = Array < Dictionary >;
            type CallParams = (Rect2i,);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(84usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_point_data_in_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_point_path_full(&mut self, from_id: Vector2i, to_id: Vector2i, allow_partial_path: bool,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (Vector2i, Vector2i, bool,);
            let args = (from_id, to_id, allow_partial_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(85usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_point_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_point_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_point_path(&mut self, from_id: Vector2i, to_id: Vector2i,) -> PackedVector2Array {
            self.get_point_path_ex(from_id, to_id,) . done()
        }
        #[inline]
        pub fn get_point_path_ex < 'a > (&'a mut self, from_id: Vector2i, to_id: Vector2i,) -> ExGetPointPath < 'a > {
            ExGetPointPath::new(self, from_id, to_id,)
        }
        pub(crate) fn get_id_path_full(&mut self, from_id: Vector2i, to_id: Vector2i, allow_partial_path: bool,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Vector2i, Vector2i, bool,);
            let args = (from_id, to_id, allow_partial_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(86usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStarGrid2D", "get_id_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_id_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_id_path(&mut self, from_id: Vector2i, to_id: Vector2i,) -> Array < Vector2i > {
            self.get_id_path_ex(from_id, to_id,) . done()
        }
        #[inline]
        pub fn get_id_path_ex < 'a > (&'a mut self, from_id: Vector2i, to_id: Vector2i,) -> ExGetIdPath < 'a > {
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
    impl crate::obj::GodotClass for AStarGrid2D {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"AStarGrid2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AStarGrid2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AStarGrid2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AStarGrid2D {
        
    }
    impl crate::obj::cap::GodotDefault for AStarGrid2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AStarGrid2D {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AStarGrid2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AStarGrid2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AStarGrid2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AStarGrid2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AStarGrid2D::set_point_solid_ex`][super::AStarGrid2D::set_point_solid_ex]."]
#[must_use]
pub struct ExSetPointSolid < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStarGrid2D, id: Vector2i, solid: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetPointSolid < 'a > {
    fn new(surround_object: &'a mut re_export::AStarGrid2D, id: Vector2i,) -> Self {
        let solid = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, solid: solid,
        }
    }
    #[inline]
    pub fn solid(self, solid: bool) -> Self {
        Self {
            solid: solid, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, solid,
        }
        = self;
        re_export::AStarGrid2D::set_point_solid_full(surround_object, id, solid,)
    }
}
#[doc = "Default-param extender for [`AStarGrid2D::fill_solid_region_ex`][super::AStarGrid2D::fill_solid_region_ex]."]
#[must_use]
pub struct ExFillSolidRegion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStarGrid2D, region: Rect2i, solid: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFillSolidRegion < 'a > {
    fn new(surround_object: &'a mut re_export::AStarGrid2D, region: Rect2i,) -> Self {
        let solid = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, region: region, solid: solid,
        }
    }
    #[inline]
    pub fn solid(self, solid: bool) -> Self {
        Self {
            solid: solid, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, region, solid,
        }
        = self;
        re_export::AStarGrid2D::fill_solid_region_full(surround_object, region, solid,)
    }
}
#[doc = "Default-param extender for [`AStarGrid2D::get_point_path_ex`][super::AStarGrid2D::get_point_path_ex]."]
#[must_use]
pub struct ExGetPointPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStarGrid2D, from_id: Vector2i, to_id: Vector2i, allow_partial_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPointPath < 'a > {
    fn new(surround_object: &'a mut re_export::AStarGrid2D, from_id: Vector2i, to_id: Vector2i,) -> Self {
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
    pub fn done(self) -> PackedVector2Array {
        let Self {
            _phantom, surround_object, from_id, to_id, allow_partial_path,
        }
        = self;
        re_export::AStarGrid2D::get_point_path_full(surround_object, from_id, to_id, allow_partial_path,)
    }
}
#[doc = "Default-param extender for [`AStarGrid2D::get_id_path_ex`][super::AStarGrid2D::get_id_path_ex]."]
#[must_use]
pub struct ExGetIdPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AStarGrid2D, from_id: Vector2i, to_id: Vector2i, allow_partial_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetIdPath < 'a > {
    fn new(surround_object: &'a mut re_export::AStarGrid2D, from_id: Vector2i, to_id: Vector2i,) -> Self {
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
    pub fn done(self) -> Array < Vector2i > {
        let Self {
            _phantom, surround_object, from_id, to_id, allow_partial_path,
        }
        = self;
        re_export::AStarGrid2D::get_id_path_full(surround_object, from_id, to_id, allow_partial_path,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Heuristic {
    ord: i32
}
impl Heuristic {
    #[doc(alias = "HEURISTIC_EUCLIDEAN")]
    #[doc = "Godot enumerator name: `HEURISTIC_EUCLIDEAN`"]
    pub const EUCLIDEAN: Heuristic = Heuristic {
        ord: 0i32
    };
    #[doc(alias = "HEURISTIC_MANHATTAN")]
    #[doc = "Godot enumerator name: `HEURISTIC_MANHATTAN`"]
    pub const MANHATTAN: Heuristic = Heuristic {
        ord: 1i32
    };
    #[doc(alias = "HEURISTIC_OCTILE")]
    #[doc = "Godot enumerator name: `HEURISTIC_OCTILE`"]
    pub const OCTILE: Heuristic = Heuristic {
        ord: 2i32
    };
    #[doc(alias = "HEURISTIC_CHEBYSHEV")]
    #[doc = "Godot enumerator name: `HEURISTIC_CHEBYSHEV`"]
    pub const CHEBYSHEV: Heuristic = Heuristic {
        ord: 3i32
    };
    #[doc(alias = "HEURISTIC_MAX")]
    #[doc = "Godot enumerator name: `HEURISTIC_MAX`"]
    pub const MAX: Heuristic = Heuristic {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Heuristic {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Heuristic") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Heuristic {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::EUCLIDEAN => "EUCLIDEAN", Self::MANHATTAN => "MANHATTAN", Self::OCTILE => "OCTILE", Self::CHEBYSHEV => "CHEBYSHEV", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Heuristic::EUCLIDEAN, Heuristic::MANHATTAN, Heuristic::OCTILE, Heuristic::CHEBYSHEV]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Heuristic >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EUCLIDEAN", "HEURISTIC_EUCLIDEAN", Heuristic::EUCLIDEAN), crate::meta::inspect::EnumConstant::new("MANHATTAN", "HEURISTIC_MANHATTAN", Heuristic::MANHATTAN), crate::meta::inspect::EnumConstant::new("OCTILE", "HEURISTIC_OCTILE", Heuristic::OCTILE), crate::meta::inspect::EnumConstant::new("CHEBYSHEV", "HEURISTIC_CHEBYSHEV", Heuristic::CHEBYSHEV), crate::meta::inspect::EnumConstant::new("MAX", "HEURISTIC_MAX", Heuristic::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Heuristic {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for Heuristic {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Heuristic {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Heuristic {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DiagonalMode {
    ord: i32
}
impl DiagonalMode {
    #[doc(alias = "DIAGONAL_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `DIAGONAL_MODE_ALWAYS`"]
    pub const ALWAYS: DiagonalMode = DiagonalMode {
        ord: 0i32
    };
    #[doc(alias = "DIAGONAL_MODE_NEVER")]
    #[doc = "Godot enumerator name: `DIAGONAL_MODE_NEVER`"]
    pub const NEVER: DiagonalMode = DiagonalMode {
        ord: 1i32
    };
    #[doc(alias = "DIAGONAL_MODE_AT_LEAST_ONE_WALKABLE")]
    #[doc = "Godot enumerator name: `DIAGONAL_MODE_AT_LEAST_ONE_WALKABLE`"]
    pub const AT_LEAST_ONE_WALKABLE: DiagonalMode = DiagonalMode {
        ord: 2i32
    };
    #[doc(alias = "DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES")]
    #[doc = "Godot enumerator name: `DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES`"]
    pub const ONLY_IF_NO_OBSTACLES: DiagonalMode = DiagonalMode {
        ord: 3i32
    };
    #[doc(alias = "DIAGONAL_MODE_MAX")]
    #[doc = "Godot enumerator name: `DIAGONAL_MODE_MAX`"]
    pub const MAX: DiagonalMode = DiagonalMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DiagonalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DiagonalMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DiagonalMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::ALWAYS => "ALWAYS", Self::NEVER => "NEVER", Self::AT_LEAST_ONE_WALKABLE => "AT_LEAST_ONE_WALKABLE", Self::ONLY_IF_NO_OBSTACLES => "ONLY_IF_NO_OBSTACLES", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DiagonalMode::ALWAYS, DiagonalMode::NEVER, DiagonalMode::AT_LEAST_ONE_WALKABLE, DiagonalMode::ONLY_IF_NO_OBSTACLES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DiagonalMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALWAYS", "DIAGONAL_MODE_ALWAYS", DiagonalMode::ALWAYS), crate::meta::inspect::EnumConstant::new("NEVER", "DIAGONAL_MODE_NEVER", DiagonalMode::NEVER), crate::meta::inspect::EnumConstant::new("AT_LEAST_ONE_WALKABLE", "DIAGONAL_MODE_AT_LEAST_ONE_WALKABLE", DiagonalMode::AT_LEAST_ONE_WALKABLE), crate::meta::inspect::EnumConstant::new("ONLY_IF_NO_OBSTACLES", "DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES", DiagonalMode::ONLY_IF_NO_OBSTACLES), crate::meta::inspect::EnumConstant::new("MAX", "DIAGONAL_MODE_MAX", DiagonalMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DiagonalMode {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for DiagonalMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DiagonalMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DiagonalMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellShape {
    ord: i32
}
impl CellShape {
    #[doc(alias = "CELL_SHAPE_SQUARE")]
    #[doc = "Godot enumerator name: `CELL_SHAPE_SQUARE`"]
    pub const SQUARE: CellShape = CellShape {
        ord: 0i32
    };
    #[doc(alias = "CELL_SHAPE_ISOMETRIC_RIGHT")]
    #[doc = "Godot enumerator name: `CELL_SHAPE_ISOMETRIC_RIGHT`"]
    pub const ISOMETRIC_RIGHT: CellShape = CellShape {
        ord: 1i32
    };
    #[doc(alias = "CELL_SHAPE_ISOMETRIC_DOWN")]
    #[doc = "Godot enumerator name: `CELL_SHAPE_ISOMETRIC_DOWN`"]
    pub const ISOMETRIC_DOWN: CellShape = CellShape {
        ord: 2i32
    };
    #[doc(alias = "CELL_SHAPE_MAX")]
    #[doc = "Godot enumerator name: `CELL_SHAPE_MAX`"]
    pub const MAX: CellShape = CellShape {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CellShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CellShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CellShape {
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
            Self::SQUARE => "SQUARE", Self::ISOMETRIC_RIGHT => "ISOMETRIC_RIGHT", Self::ISOMETRIC_DOWN => "ISOMETRIC_DOWN", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CellShape::SQUARE, CellShape::ISOMETRIC_RIGHT, CellShape::ISOMETRIC_DOWN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CellShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SQUARE", "CELL_SHAPE_SQUARE", CellShape::SQUARE), crate::meta::inspect::EnumConstant::new("ISOMETRIC_RIGHT", "CELL_SHAPE_ISOMETRIC_RIGHT", CellShape::ISOMETRIC_RIGHT), crate::meta::inspect::EnumConstant::new("ISOMETRIC_DOWN", "CELL_SHAPE_ISOMETRIC_DOWN", CellShape::ISOMETRIC_DOWN), crate::meta::inspect::EnumConstant::new("MAX", "CELL_SHAPE_MAX", CellShape::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CellShape {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for CellShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CellShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CellShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AStarGrid2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AStarGrid2D {
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