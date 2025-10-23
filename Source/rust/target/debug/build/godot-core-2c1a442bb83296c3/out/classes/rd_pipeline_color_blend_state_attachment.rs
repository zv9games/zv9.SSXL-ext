#![doc = "Sidecar module for class [`RdPipelineColorBlendStateAttachment`][crate::classes::RdPipelineColorBlendStateAttachment].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDPipelineColorBlendStateAttachment` enums](https://docs.godotengine.org/en/stable/classes/class_rdpipelinecolorblendstateattachment.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDPipelineColorBlendStateAttachment.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IRdPipelineColorBlendStateAttachment`][crate::classes::IRdPipelineColorBlendStateAttachment]: virtual methods\n\n\nSee also [Godot docs for `RDPipelineColorBlendStateAttachment`](https://docs.godotengine.org/en/stable/classes/class_rdpipelinecolorblendstateattachment.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RdPipelineColorBlendStateAttachment::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdPipelineColorBlendStateAttachment {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RdPipelineColorBlendStateAttachment`][crate::classes::RdPipelineColorBlendStateAttachment].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `RDPipelineColorBlendStateAttachment` methods](https://docs.godotengine.org/en/stable/classes/class_rdpipelinecolorblendstateattachment.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdPipelineColorBlendStateAttachment: crate::obj::GodotClass < Base = RdPipelineColorBlendStateAttachment > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdPipelineColorBlendStateAttachment {
        pub fn set_as_mix(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_as_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_blend(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_enable_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_blend(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_enable_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_src_color_blend_factor(&mut self, p_member: crate::classes::rendering_device::BlendFactor,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::BlendFactor,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_src_color_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_src_color_blend_factor(&self,) -> crate::classes::rendering_device::BlendFactor {
            type CallRet = crate::classes::rendering_device::BlendFactor;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_src_color_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dst_color_blend_factor(&mut self, p_member: crate::classes::rendering_device::BlendFactor,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::BlendFactor,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_dst_color_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dst_color_blend_factor(&self,) -> crate::classes::rendering_device::BlendFactor {
            type CallRet = crate::classes::rendering_device::BlendFactor;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_dst_color_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_blend_op(&mut self, p_member: crate::classes::rendering_device::BlendOperation,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::BlendOperation,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_color_blend_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_blend_op(&self,) -> crate::classes::rendering_device::BlendOperation {
            type CallRet = crate::classes::rendering_device::BlendOperation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_color_blend_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_src_alpha_blend_factor(&mut self, p_member: crate::classes::rendering_device::BlendFactor,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::BlendFactor,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_src_alpha_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_src_alpha_blend_factor(&self,) -> crate::classes::rendering_device::BlendFactor {
            type CallRet = crate::classes::rendering_device::BlendFactor;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_src_alpha_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dst_alpha_blend_factor(&mut self, p_member: crate::classes::rendering_device::BlendFactor,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::BlendFactor,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_dst_alpha_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dst_alpha_blend_factor(&self,) -> crate::classes::rendering_device::BlendFactor {
            type CallRet = crate::classes::rendering_device::BlendFactor;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_dst_alpha_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_blend_op(&mut self, p_member: crate::classes::rendering_device::BlendOperation,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::BlendOperation,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_alpha_blend_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_blend_op(&self,) -> crate::classes::rendering_device::BlendOperation {
            type CallRet = crate::classes::rendering_device::BlendOperation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_alpha_blend_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_r(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_write_r", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_r(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_write_r", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_g(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_write_g", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_g(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_write_g", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_b(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_write_b", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_b(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_write_b", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_a(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "set_write_a", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_a(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineColorBlendStateAttachment", "get_write_a", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdPipelineColorBlendStateAttachment {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"RDPipelineColorBlendStateAttachment"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdPipelineColorBlendStateAttachment {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RdPipelineColorBlendStateAttachment {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RdPipelineColorBlendStateAttachment {
        
    }
    impl crate::obj::cap::GodotDefault for RdPipelineColorBlendStateAttachment {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdPipelineColorBlendStateAttachment {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdPipelineColorBlendStateAttachment {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RdPipelineColorBlendStateAttachment`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RdPipelineColorBlendStateAttachment__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RdPipelineColorBlendStateAttachment > for $Class {
                
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
    use super::re_export::RdPipelineColorBlendStateAttachment;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RdPipelineColorBlendStateAttachment {
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