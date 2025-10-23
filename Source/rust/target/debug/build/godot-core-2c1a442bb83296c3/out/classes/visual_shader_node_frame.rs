#![doc = "Sidecar module for class [`VisualShaderNodeFrame`][crate::classes::VisualShaderNodeFrame].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeFrame` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeframe.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeFrame.`\n\nInherits [`VisualShaderNodeResizableBase`][crate::classes::VisualShaderNodeResizableBase].\n\nRelated symbols:\n\n* [`IVisualShaderNodeFrame`][crate::classes::IVisualShaderNodeFrame]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeFrame`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeframe.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeFrame::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeFrame {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShaderNodeFrame`][crate::classes::VisualShaderNodeFrame].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IVisualShaderNodeResizableBase`~~ > ~~`IVisualShaderNode`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShaderNodeFrame` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeframe.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeFrame: crate::obj::GodotClass < Base = VisualShaderNodeFrame > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl VisualShaderNodeFrame {
        pub fn set_title(&mut self, title: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "set_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_title(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "get_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tint_color_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "set_tint_color_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tint_color_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "is_tint_color_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tint_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "set_tint_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tint_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "get_tint_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autoshrink_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "set_autoshrink_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_autoshrink_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "is_autoshrink_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_attached_node(&mut self, node: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "add_attached_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_attached_node(&mut self, node: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "remove_attached_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attached_nodes(&mut self, attached_nodes: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedInt32Array >,);
            let args = (RefArg::new(attached_nodes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "set_attached_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attached_nodes(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFrame", "get_attached_nodes", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeFrame {
        type Base = crate::classes::VisualShaderNodeResizableBase;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"VisualShaderNodeFrame"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeFrame {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNodeResizableBase > for VisualShaderNodeFrame {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeFrame {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeFrame {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeFrame {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeFrame {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeFrame {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeFrame {
        type Target = crate::classes::VisualShaderNodeResizableBase;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeFrame {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeFrame`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeFrame__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeFrame > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeResizableBase > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNode > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
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
    use super::re_export::VisualShaderNodeFrame;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeFrame {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}