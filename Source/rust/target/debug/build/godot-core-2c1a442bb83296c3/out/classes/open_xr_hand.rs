#![doc = "Sidecar module for class [`OpenXrHand`][crate::classes::OpenXrHand].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRHand` enums](https://docs.godotengine.org/en/stable/classes/class_openxrhand.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRHand.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`open_xr_hand`][crate::classes::open_xr_hand]: sidecar module with related enum/flag types\n* [`IOpenXrHand`][crate::classes::IOpenXrHand]: virtual methods\n\n\nSee also [Godot docs for `OpenXRHand`](https://docs.godotengine.org/en/stable/classes/class_openxrhand.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OpenXrHand::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrHand {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrHand`][crate::classes::OpenXrHand].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRHand` methods](https://docs.godotengine.org/en/stable/classes/class_openxrhand.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrHand: crate::obj::GodotClass < Base = OpenXrHand > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrHand {
        pub fn set_hand(&mut self, hand: crate::classes::open_xr_hand::Hands,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_hand::Hands,);
            let args = (hand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrHand", "set_hand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand(&self,) -> crate::classes::open_xr_hand::Hands {
            type CallRet = crate::classes::open_xr_hand::Hands;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrHand", "get_hand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_range(&mut self, motion_range: crate::classes::open_xr_hand::MotionRange,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_hand::MotionRange,);
            let args = (motion_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrHand", "set_motion_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_range(&self,) -> crate::classes::open_xr_hand::MotionRange {
            type CallRet = crate::classes::open_xr_hand::MotionRange;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrHand", "get_motion_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skeleton_rig(&mut self, skeleton_rig: crate::classes::open_xr_hand::SkeletonRig,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_hand::SkeletonRig,);
            let args = (skeleton_rig,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrHand", "set_skeleton_rig", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skeleton_rig(&self,) -> crate::classes::open_xr_hand::SkeletonRig {
            type CallRet = crate::classes::open_xr_hand::SkeletonRig;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrHand", "get_skeleton_rig", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_update(&mut self, bone_update: crate::classes::open_xr_hand::BoneUpdate,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_hand::BoneUpdate,);
            let args = (bone_update,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrHand", "set_bone_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_update(&self,) -> crate::classes::open_xr_hand::BoneUpdate {
            type CallRet = crate::classes::open_xr_hand::BoneUpdate;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrHand", "get_bone_update", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrHand {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"OpenXRHand"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrHand {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for OpenXrHand {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for OpenXrHand {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrHand {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrHand {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrHand {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrHand {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrHand`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrHand__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrHand > for $Class {
                
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
pub struct Hands {
    ord: i32
}
impl Hands {
    #[doc(alias = "HAND_LEFT")]
    #[doc = "Godot enumerator name: `HAND_LEFT`"]
    pub const LEFT: Hands = Hands {
        ord: 0i32
    };
    #[doc(alias = "HAND_RIGHT")]
    #[doc = "Godot enumerator name: `HAND_RIGHT`"]
    pub const RIGHT: Hands = Hands {
        ord: 1i32
    };
    #[doc(alias = "HAND_MAX")]
    #[doc = "Godot enumerator name: `HAND_MAX`"]
    pub const MAX: Hands = Hands {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Hands {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Hands") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Hands {
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
            Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Hands::LEFT, Hands::RIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Hands >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LEFT", "HAND_LEFT", Hands::LEFT), crate::meta::inspect::EnumConstant::new("RIGHT", "HAND_RIGHT", Hands::RIGHT), crate::meta::inspect::EnumConstant::new("MAX", "HAND_MAX", Hands::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Hands {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for Hands {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Hands {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Hands {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MotionRange {
    ord: i32
}
impl MotionRange {
    #[doc(alias = "MOTION_RANGE_UNOBSTRUCTED")]
    #[doc = "Godot enumerator name: `MOTION_RANGE_UNOBSTRUCTED`"]
    pub const UNOBSTRUCTED: MotionRange = MotionRange {
        ord: 0i32
    };
    #[doc(alias = "MOTION_RANGE_CONFORM_TO_CONTROLLER")]
    #[doc = "Godot enumerator name: `MOTION_RANGE_CONFORM_TO_CONTROLLER`"]
    pub const CONFORM_TO_CONTROLLER: MotionRange = MotionRange {
        ord: 1i32
    };
    #[doc(alias = "MOTION_RANGE_MAX")]
    #[doc = "Godot enumerator name: `MOTION_RANGE_MAX`"]
    pub const MAX: MotionRange = MotionRange {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MotionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MotionRange") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MotionRange {
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
            Self::UNOBSTRUCTED => "UNOBSTRUCTED", Self::CONFORM_TO_CONTROLLER => "CONFORM_TO_CONTROLLER", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MotionRange::UNOBSTRUCTED, MotionRange::CONFORM_TO_CONTROLLER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MotionRange >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNOBSTRUCTED", "MOTION_RANGE_UNOBSTRUCTED", MotionRange::UNOBSTRUCTED), crate::meta::inspect::EnumConstant::new("CONFORM_TO_CONTROLLER", "MOTION_RANGE_CONFORM_TO_CONTROLLER", MotionRange::CONFORM_TO_CONTROLLER), crate::meta::inspect::EnumConstant::new("MAX", "MOTION_RANGE_MAX", MotionRange::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MotionRange {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for MotionRange {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MotionRange {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MotionRange {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SkeletonRig {
    ord: i32
}
impl SkeletonRig {
    #[doc(alias = "SKELETON_RIG_OPENXR")]
    #[doc = "Godot enumerator name: `SKELETON_RIG_OPENXR`"]
    pub const OPENXR: SkeletonRig = SkeletonRig {
        ord: 0i32
    };
    #[doc(alias = "SKELETON_RIG_HUMANOID")]
    #[doc = "Godot enumerator name: `SKELETON_RIG_HUMANOID`"]
    pub const HUMANOID: SkeletonRig = SkeletonRig {
        ord: 1i32
    };
    #[doc(alias = "SKELETON_RIG_MAX")]
    #[doc = "Godot enumerator name: `SKELETON_RIG_MAX`"]
    pub const MAX: SkeletonRig = SkeletonRig {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SkeletonRig {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SkeletonRig") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SkeletonRig {
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
            Self::OPENXR => "OPENXR", Self::HUMANOID => "HUMANOID", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SkeletonRig::OPENXR, SkeletonRig::HUMANOID]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SkeletonRig >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OPENXR", "SKELETON_RIG_OPENXR", SkeletonRig::OPENXR), crate::meta::inspect::EnumConstant::new("HUMANOID", "SKELETON_RIG_HUMANOID", SkeletonRig::HUMANOID), crate::meta::inspect::EnumConstant::new("MAX", "SKELETON_RIG_MAX", SkeletonRig::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SkeletonRig {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for SkeletonRig {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SkeletonRig {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SkeletonRig {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BoneUpdate {
    ord: i32
}
impl BoneUpdate {
    #[doc(alias = "BONE_UPDATE_FULL")]
    #[doc = "Godot enumerator name: `BONE_UPDATE_FULL`"]
    pub const FULL: BoneUpdate = BoneUpdate {
        ord: 0i32
    };
    #[doc(alias = "BONE_UPDATE_ROTATION_ONLY")]
    #[doc = "Godot enumerator name: `BONE_UPDATE_ROTATION_ONLY`"]
    pub const ROTATION_ONLY: BoneUpdate = BoneUpdate {
        ord: 1i32
    };
    #[doc(alias = "BONE_UPDATE_MAX")]
    #[doc = "Godot enumerator name: `BONE_UPDATE_MAX`"]
    pub const MAX: BoneUpdate = BoneUpdate {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for BoneUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BoneUpdate") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BoneUpdate {
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
            Self::FULL => "FULL", Self::ROTATION_ONLY => "ROTATION_ONLY", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BoneUpdate::FULL, BoneUpdate::ROTATION_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BoneUpdate >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FULL", "BONE_UPDATE_FULL", BoneUpdate::FULL), crate::meta::inspect::EnumConstant::new("ROTATION_ONLY", "BONE_UPDATE_ROTATION_ONLY", BoneUpdate::ROTATION_ONLY), crate::meta::inspect::EnumConstant::new("MAX", "BONE_UPDATE_MAX", BoneUpdate::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for BoneUpdate {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for BoneUpdate {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BoneUpdate {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BoneUpdate {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrHand;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for OpenXrHand {
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