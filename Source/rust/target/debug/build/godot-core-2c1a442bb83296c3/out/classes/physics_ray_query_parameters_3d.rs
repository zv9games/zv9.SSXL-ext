#![doc = "Sidecar module for class [`PhysicsRayQueryParameters3D`][crate::classes::PhysicsRayQueryParameters3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsRayQueryParameters3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsrayqueryparameters3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsRayQueryParameters3D.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`physics_ray_query_parameters_3d`][crate::classes::physics_ray_query_parameters_3d]: sidecar module with related enum/flag types\n* [`IPhysicsRayQueryParameters3D`][crate::classes::IPhysicsRayQueryParameters3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsRayQueryParameters3D`](https://docs.godotengine.org/en/stable/classes/class_physicsrayqueryparameters3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PhysicsRayQueryParameters3D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsRayQueryParameters3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PhysicsRayQueryParameters3D`][crate::classes::PhysicsRayQueryParameters3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `PhysicsRayQueryParameters3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsrayqueryparameters3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsRayQueryParameters3D: crate::obj::GodotClass < Base = PhysicsRayQueryParameters3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl PhysicsRayQueryParameters3D {
        pub(crate) fn create_full(from: Vector3, to: Vector3, collision_mask: u32, exclude: RefArg < Array < Rid > >,) -> Option < Gd < crate::classes::PhysicsRayQueryParameters3D > > {
            type CallRet = Option < Gd < crate::classes::PhysicsRayQueryParameters3D > >;
            type CallParams < 'a0, > = (Vector3, Vector3, u32, RefArg < 'a0, Array < Rid > >,);
            let args = (from, to, collision_mask, exclude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "create", std::ptr::null_mut(), None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create(from: Vector3, to: Vector3,) -> Option < Gd < crate::classes::PhysicsRayQueryParameters3D > > {
            Self::create_ex(from, to,) . done()
        }
        #[inline]
        pub fn create_ex < 'a > (from: Vector3, to: Vector3,) -> ExCreate < 'a > {
            ExCreate::new(from, to,)
        }
        pub fn set_from(&mut self, from: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "set_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_from(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "get_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_to(&mut self, to: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "set_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_to(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "get_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, collision_mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (collision_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exclude(&mut self, exclude: &Array < Rid >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Rid > >,);
            let args = (RefArg::new(exclude),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "set_exclude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_exclude(&self,) -> Array < Rid > {
            type CallRet = Array < Rid >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "get_exclude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_bodies(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "set_collide_with_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_bodies_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "is_collide_with_bodies_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_areas(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "set_collide_with_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_areas_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "is_collide_with_areas_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hit_from_inside(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "set_hit_from_inside", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hit_from_inside_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "is_hit_from_inside_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hit_back_faces(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "set_hit_back_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hit_back_faces_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsRayQueryParameters3D", "is_hit_back_faces_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsRayQueryParameters3D {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"PhysicsRayQueryParameters3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsRayQueryParameters3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PhysicsRayQueryParameters3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsRayQueryParameters3D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsRayQueryParameters3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsRayQueryParameters3D {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsRayQueryParameters3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsRayQueryParameters3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsRayQueryParameters3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsRayQueryParameters3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsRayQueryParameters3D::create_ex`][super::PhysicsRayQueryParameters3D::create_ex]."]
#[must_use]
pub struct ExCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, from: Vector3, to: Vector3, collision_mask: u32, exclude: CowArg < 'a, Array < Rid > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreate < 'a > {
    fn new(from: Vector3, to: Vector3,) -> Self {
        let collision_mask = 4294967295u32;
        let exclude = Array::new();
        Self {
            _phantom: std::marker::PhantomData, from: from, to: to, collision_mask: collision_mask, exclude: CowArg::Owned(exclude),
        }
    }
    #[inline]
    pub fn collision_mask(self, collision_mask: u32) -> Self {
        Self {
            collision_mask: collision_mask, .. self
        }
    }
    #[inline]
    pub fn exclude(self, exclude: &'a Array < Rid >) -> Self {
        Self {
            exclude: CowArg::Borrowed(exclude), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::PhysicsRayQueryParameters3D > > {
        let Self {
            _phantom, from, to, collision_mask, exclude,
        }
        = self;
        re_export::PhysicsRayQueryParameters3D::create_full(from, to, collision_mask, exclude.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsRayQueryParameters3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsRayQueryParameters3D {
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