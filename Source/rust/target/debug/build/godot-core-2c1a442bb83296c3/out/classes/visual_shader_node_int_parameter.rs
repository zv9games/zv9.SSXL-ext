#![doc = "Sidecar module for class [`VisualShaderNodeIntParameter`][crate::classes::VisualShaderNodeIntParameter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeIntParameter` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeintparameter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeIntParameter.`\n\nInherits [`VisualShaderNodeParameter`][crate::classes::VisualShaderNodeParameter].\n\nRelated symbols:\n\n* [`visual_shader_node_int_parameter`][crate::classes::visual_shader_node_int_parameter]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeIntParameter`][crate::classes::IVisualShaderNodeIntParameter]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeIntParameter`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeintparameter.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeIntParameter::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeIntParameter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShaderNodeIntParameter`][crate::classes::VisualShaderNodeIntParameter].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IVisualShaderNodeParameter`~~ > ~~`IVisualShaderNode`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShaderNodeIntParameter` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeintparameter.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeIntParameter: crate::obj::GodotClass < Base = VisualShaderNodeIntParameter > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeIntParameter {
        pub fn set_hint(&mut self, hint: crate::classes::visual_shader_node_int_parameter::Hint,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_int_parameter::Hint,);
            let args = (hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "set_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hint(&self,) -> crate::classes::visual_shader_node_int_parameter::Hint {
            type CallRet = crate::classes::visual_shader_node_int_parameter::Hint;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "get_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "set_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "get_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "set_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "get_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_step(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "set_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_step(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "get_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enum_names(&mut self, names: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "set_enum_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enum_names(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "get_enum_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_value_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "set_default_value_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_default_value_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "is_default_value_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_value(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "set_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_value(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeIntParameter", "get_default_value", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeIntParameter {
        type Base = crate::classes::VisualShaderNodeParameter;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"VisualShaderNodeIntParameter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeIntParameter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNodeParameter > for VisualShaderNodeIntParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeIntParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeIntParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeIntParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeIntParameter {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeIntParameter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeIntParameter {
        type Target = crate::classes::VisualShaderNodeParameter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeIntParameter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeIntParameter`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeIntParameter__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeIntParameter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeParameter > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Hint {
    ord: i32
}
impl Hint {
    #[doc(alias = "HINT_NONE")]
    #[doc = "Godot enumerator name: `HINT_NONE`"]
    pub const NONE: Hint = Hint {
        ord: 0i32
    };
    #[doc(alias = "HINT_RANGE")]
    #[doc = "Godot enumerator name: `HINT_RANGE`"]
    pub const RANGE: Hint = Hint {
        ord: 1i32
    };
    #[doc(alias = "HINT_RANGE_STEP")]
    #[doc = "Godot enumerator name: `HINT_RANGE_STEP`"]
    pub const RANGE_STEP: Hint = Hint {
        ord: 2i32
    };
    #[doc(alias = "HINT_ENUM")]
    #[doc = "Godot enumerator name: `HINT_ENUM`"]
    pub const ENUM: Hint = Hint {
        ord: 3i32
    };
    #[doc(alias = "HINT_MAX")]
    #[doc = "Godot enumerator name: `HINT_MAX`"]
    pub const MAX: Hint = Hint {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Hint {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Hint") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Hint {
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
            Self::NONE => "NONE", Self::RANGE => "RANGE", Self::RANGE_STEP => "RANGE_STEP", Self::ENUM => "ENUM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Hint::NONE, Hint::RANGE, Hint::RANGE_STEP, Hint::ENUM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Hint >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "HINT_NONE", Hint::NONE), crate::meta::inspect::EnumConstant::new("RANGE", "HINT_RANGE", Hint::RANGE), crate::meta::inspect::EnumConstant::new("RANGE_STEP", "HINT_RANGE_STEP", Hint::RANGE_STEP), crate::meta::inspect::EnumConstant::new("ENUM", "HINT_ENUM", Hint::ENUM), crate::meta::inspect::EnumConstant::new("MAX", "HINT_MAX", Hint::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Hint {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for Hint {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Hint {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Hint {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeIntParameter;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeIntParameter {
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