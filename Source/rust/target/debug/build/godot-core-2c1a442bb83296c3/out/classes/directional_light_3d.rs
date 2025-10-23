#![doc = "Sidecar module for class [`DirectionalLight3D`][crate::classes::DirectionalLight3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DirectionalLight3D` enums](https://docs.godotengine.org/en/stable/classes/class_directionallight3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DirectionalLight3D.`\n\nInherits [`Light3D`][crate::classes::Light3D].\n\nRelated symbols:\n\n* [`directional_light_3d`][crate::classes::directional_light_3d]: sidecar module with related enum/flag types\n* [`IDirectionalLight3D`][crate::classes::IDirectionalLight3D]: virtual methods\n\n\nSee also [Godot docs for `DirectionalLight3D`](https://docs.godotengine.org/en/stable/classes/class_directionallight3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`DirectionalLight3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DirectionalLight3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`DirectionalLight3D`][crate::classes::DirectionalLight3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ILight3D`~~ > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `DirectionalLight3D` methods](https://docs.godotengine.org/en/stable/classes/class_directionallight3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IDirectionalLight3D: crate::obj::GodotClass < Base = DirectionalLight3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl DirectionalLight3D {
        pub fn set_shadow_mode(&mut self, mode: crate::classes::directional_light_3d::ShadowMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::directional_light_3d::ShadowMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirectionalLight3D", "set_shadow_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_mode(&self,) -> crate::classes::directional_light_3d::ShadowMode {
            type CallRet = crate::classes::directional_light_3d::ShadowMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirectionalLight3D", "get_shadow_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_splits(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirectionalLight3D", "set_blend_splits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_blend_splits_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirectionalLight3D", "is_blend_splits_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sky_mode(&mut self, mode: crate::classes::directional_light_3d::SkyMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::directional_light_3d::SkyMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirectionalLight3D", "set_sky_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sky_mode(&self,) -> crate::classes::directional_light_3d::SkyMode {
            type CallRet = crate::classes::directional_light_3d::SkyMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirectionalLight3D", "get_sky_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for DirectionalLight3D {
        type Base = crate::classes::Light3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"DirectionalLight3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for DirectionalLight3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Light3D > for DirectionalLight3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for DirectionalLight3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for DirectionalLight3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for DirectionalLight3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DirectionalLight3D {
        
    }
    impl crate::obj::cap::GodotDefault for DirectionalLight3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for DirectionalLight3D {
        type Target = crate::classes::Light3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DirectionalLight3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`DirectionalLight3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_DirectionalLight3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::DirectionalLight3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Light3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadowMode {
    ord: i32
}
impl ShadowMode {
    #[doc(alias = "SHADOW_ORTHOGONAL")]
    #[doc = "Godot enumerator name: `SHADOW_ORTHOGONAL`"]
    pub const ORTHOGONAL: ShadowMode = ShadowMode {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_PARALLEL_2_SPLITS")]
    #[doc = "Godot enumerator name: `SHADOW_PARALLEL_2_SPLITS`"]
    pub const PARALLEL_2_SPLITS: ShadowMode = ShadowMode {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_PARALLEL_4_SPLITS")]
    #[doc = "Godot enumerator name: `SHADOW_PARALLEL_4_SPLITS`"]
    pub const PARALLEL_4_SPLITS: ShadowMode = ShadowMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ShadowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowMode {
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
            Self::ORTHOGONAL => "ORTHOGONAL", Self::PARALLEL_2_SPLITS => "PARALLEL_2_SPLITS", Self::PARALLEL_4_SPLITS => "PARALLEL_4_SPLITS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadowMode::ORTHOGONAL, ShadowMode::PARALLEL_2_SPLITS, ShadowMode::PARALLEL_4_SPLITS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadowMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ORTHOGONAL", "SHADOW_ORTHOGONAL", ShadowMode::ORTHOGONAL), crate::meta::inspect::EnumConstant::new("PARALLEL_2_SPLITS", "SHADOW_PARALLEL_2_SPLITS", ShadowMode::PARALLEL_2_SPLITS), crate::meta::inspect::EnumConstant::new("PARALLEL_4_SPLITS", "SHADOW_PARALLEL_4_SPLITS", ShadowMode::PARALLEL_4_SPLITS)]
        }
    }
}
impl crate::meta::GodotConvert for ShadowMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadowMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SkyMode {
    ord: i32
}
impl SkyMode {
    #[doc(alias = "SKY_MODE_LIGHT_AND_SKY")]
    #[doc = "Godot enumerator name: `SKY_MODE_LIGHT_AND_SKY`"]
    pub const LIGHT_AND_SKY: SkyMode = SkyMode {
        ord: 0i32
    };
    #[doc(alias = "SKY_MODE_LIGHT_ONLY")]
    #[doc = "Godot enumerator name: `SKY_MODE_LIGHT_ONLY`"]
    pub const LIGHT_ONLY: SkyMode = SkyMode {
        ord: 1i32
    };
    #[doc(alias = "SKY_MODE_SKY_ONLY")]
    #[doc = "Godot enumerator name: `SKY_MODE_SKY_ONLY`"]
    pub const SKY_ONLY: SkyMode = SkyMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SkyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SkyMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SkyMode {
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
            Self::LIGHT_AND_SKY => "LIGHT_AND_SKY", Self::LIGHT_ONLY => "LIGHT_ONLY", Self::SKY_ONLY => "SKY_ONLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SkyMode::LIGHT_AND_SKY, SkyMode::LIGHT_ONLY, SkyMode::SKY_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SkyMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LIGHT_AND_SKY", "SKY_MODE_LIGHT_AND_SKY", SkyMode::LIGHT_AND_SKY), crate::meta::inspect::EnumConstant::new("LIGHT_ONLY", "SKY_MODE_LIGHT_ONLY", SkyMode::LIGHT_ONLY), crate::meta::inspect::EnumConstant::new("SKY_ONLY", "SKY_MODE_SKY_ONLY", SkyMode::SKY_ONLY)]
        }
    }
}
impl crate::meta::GodotConvert for SkyMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SkyMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SkyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::DirectionalLight3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for DirectionalLight3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}