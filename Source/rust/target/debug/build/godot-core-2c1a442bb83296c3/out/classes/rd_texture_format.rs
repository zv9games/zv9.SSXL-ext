#![doc = "Sidecar module for class [`RdTextureFormat`][crate::classes::RdTextureFormat].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDTextureFormat` enums](https://docs.godotengine.org/en/stable/classes/class_rdtextureformat.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDTextureFormat.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IRdTextureFormat`][crate::classes::IRdTextureFormat]: virtual methods\n\n\nSee also [Godot docs for `RDTextureFormat`](https://docs.godotengine.org/en/stable/classes/class_rdtextureformat.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RdTextureFormat::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdTextureFormat {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RdTextureFormat`][crate::classes::RdTextureFormat].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `RDTextureFormat` methods](https://docs.godotengine.org/en/stable/classes/class_rdtextureformat.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdTextureFormat: crate::obj::GodotClass < Base = RdTextureFormat > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdTextureFormat {
        pub fn set_format(&mut self, p_member: crate::classes::rendering_device::DataFormat,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::DataFormat,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_format(&self,) -> crate::classes::rendering_device::DataFormat {
            type CallRet = crate::classes::rendering_device::DataFormat;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_width(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_width(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_array_layers(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_array_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_array_layers(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_array_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mipmaps(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mipmaps(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_type(&mut self, p_member: crate::classes::rendering_device::TextureType,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::TextureType,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_texture_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_type(&self,) -> crate::classes::rendering_device::TextureType {
            type CallRet = crate::classes::rendering_device::TextureType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_texture_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_samples(&mut self, p_member: crate::classes::rendering_device::TextureSamples,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::TextureSamples,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_samples", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_samples(&self,) -> crate::classes::rendering_device::TextureSamples {
            type CallRet = crate::classes::rendering_device::TextureSamples;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_samples", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_usage_bits(&mut self, p_member: crate::classes::rendering_device::TextureUsageBits,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::TextureUsageBits,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_usage_bits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_usage_bits(&self,) -> crate::classes::rendering_device::TextureUsageBits {
            type CallRet = crate::classes::rendering_device::TextureUsageBits;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_usage_bits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_is_resolve_buffer(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_is_resolve_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_is_resolve_buffer(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_is_resolve_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_is_discardable(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "set_is_discardable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_is_discardable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "get_is_discardable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_shareable_format(&mut self, format: crate::classes::rendering_device::DataFormat,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::DataFormat,);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "add_shareable_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_shareable_format(&mut self, format: crate::classes::rendering_device::DataFormat,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::DataFormat,);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdTextureFormat", "remove_shareable_format", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdTextureFormat {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"RDTextureFormat"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdTextureFormat {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RdTextureFormat {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RdTextureFormat {
        
    }
    impl crate::obj::cap::GodotDefault for RdTextureFormat {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdTextureFormat {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdTextureFormat {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RdTextureFormat`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RdTextureFormat__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RdTextureFormat > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RdTextureFormat;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RdTextureFormat {
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