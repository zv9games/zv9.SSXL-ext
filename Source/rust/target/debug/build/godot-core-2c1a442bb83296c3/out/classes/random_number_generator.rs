#![doc = "Sidecar module for class [`RandomNumberGenerator`][crate::classes::RandomNumberGenerator].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RandomNumberGenerator` enums](https://docs.godotengine.org/en/stable/classes/class_randomnumbergenerator.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RandomNumberGenerator.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`random_number_generator`][crate::classes::random_number_generator]: sidecar module with related enum/flag types\n* [`IRandomNumberGenerator`][crate::classes::IRandomNumberGenerator]: virtual methods\n\n\nSee also [Godot docs for `RandomNumberGenerator`](https://docs.godotengine.org/en/stable/classes/class_randomnumbergenerator.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RandomNumberGenerator::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RandomNumberGenerator {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RandomNumberGenerator`][crate::classes::RandomNumberGenerator].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `RandomNumberGenerator` methods](https://docs.godotengine.org/en/stable/classes/class_randomnumbergenerator.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRandomNumberGenerator: crate::obj::GodotClass < Base = RandomNumberGenerator > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RandomNumberGenerator {
        pub fn set_seed(&mut self, seed: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "set_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_seed(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "get_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_state(&mut self, state: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "set_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_state(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn randi(&mut self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "randi", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn randf(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "randf", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn randfn_full(&mut self, mean: f32, deviation: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (f32, f32,);
            let args = (mean, deviation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "randfn", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::randfn_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn randfn(&mut self,) -> f32 {
            self.randfn_ex() . done()
        }
        #[inline]
        pub fn randfn_ex < 'a > (&'a mut self,) -> ExRandfn < 'a > {
            ExRandfn::new(self,)
        }
        pub fn randf_range(&mut self, from: f32, to: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (f32, f32,);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "randf_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn randi_range(&mut self, from: i32, to: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "randi_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rand_weighted(&mut self, weights: &PackedFloat32Array,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(weights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "rand_weighted", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn randomize(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RandomNumberGenerator", "randomize", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RandomNumberGenerator {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"RandomNumberGenerator"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RandomNumberGenerator {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RandomNumberGenerator {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RandomNumberGenerator {
        
    }
    impl crate::obj::cap::GodotDefault for RandomNumberGenerator {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RandomNumberGenerator {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RandomNumberGenerator {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RandomNumberGenerator`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RandomNumberGenerator__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RandomNumberGenerator > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RandomNumberGenerator::randfn_ex`][super::RandomNumberGenerator::randfn_ex]."]
#[must_use]
pub struct ExRandfn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RandomNumberGenerator, mean: f32, deviation: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRandfn < 'a > {
    fn new(surround_object: &'a mut re_export::RandomNumberGenerator,) -> Self {
        let mean = 0f32;
        let deviation = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mean: mean, deviation: deviation,
        }
    }
    #[inline]
    pub fn mean(self, mean: f32) -> Self {
        Self {
            mean: mean, .. self
        }
    }
    #[inline]
    pub fn deviation(self, deviation: f32) -> Self {
        Self {
            deviation: deviation, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, mean, deviation,
        }
        = self;
        re_export::RandomNumberGenerator::randfn_full(surround_object, mean, deviation,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RandomNumberGenerator;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RandomNumberGenerator {
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