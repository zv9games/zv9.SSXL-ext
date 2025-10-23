#![doc = "Sidecar module for class [`GradientTexture2D`][crate::classes::GradientTexture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GradientTexture2D` enums](https://docs.godotengine.org/en/stable/classes/class_gradienttexture2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GradientTexture2D.`\n\nInherits [`Texture2D`][crate::classes::Texture2D].\n\nRelated symbols:\n\n* [`gradient_texture_2d`][crate::classes::gradient_texture_2d]: sidecar module with related enum/flag types\n* [`IGradientTexture2D`][crate::classes::IGradientTexture2D]: virtual methods\n\n\nSee also [Godot docs for `GradientTexture2D`](https://docs.godotengine.org/en/stable/classes/class_gradienttexture2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GradientTexture2D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GradientTexture2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GradientTexture2D`][crate::classes::GradientTexture2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITexture2D`][crate::classes::ITexture2D] > [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GradientTexture2D` methods](https://docs.godotengine.org/en/stable/classes/class_gradienttexture2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGradientTexture2D: crate::obj::GodotClass < Base = GradientTexture2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_width(&self,) -> i32;
        fn get_height(&self,) -> i32;
        fn is_pixel_opaque(&self, x: i32, y: i32,) -> bool {
            unimplemented !()
        }
        fn has_alpha(&self,) -> bool {
            unimplemented !()
        }
        fn draw(&self, to_canvas_item: Rid, pos: Vector2, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect(&self, to_canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect_region(&self, to_canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
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
    impl GradientTexture2D {
        pub fn set_gradient(&mut self, gradient: impl AsArg < Option < Gd < crate::classes::Gradient >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Gradient >> >,);
            let args = (gradient.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "set_gradient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gradient(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallRet = Option < Gd < crate::classes::Gradient > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "get_gradient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, height: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_hdr(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "set_use_hdr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_hdr(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "is_using_hdr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fill(&mut self, fill: crate::classes::gradient_texture_2d::Fill,) {
            type CallRet = ();
            type CallParams = (crate::classes::gradient_texture_2d::Fill,);
            let args = (fill,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "set_fill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fill(&self,) -> crate::classes::gradient_texture_2d::Fill {
            type CallRet = crate::classes::gradient_texture_2d::Fill;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "get_fill", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fill_from(&mut self, fill_from: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (fill_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "set_fill_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fill_from(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "get_fill_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fill_to(&mut self, fill_to: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (fill_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "set_fill_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fill_to(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "get_fill_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_repeat(&mut self, repeat: crate::classes::gradient_texture_2d::Repeat,) {
            type CallRet = ();
            type CallParams = (crate::classes::gradient_texture_2d::Repeat,);
            let args = (repeat,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "set_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_repeat(&self,) -> crate::classes::gradient_texture_2d::Repeat {
            type CallRet = crate::classes::gradient_texture_2d::Repeat;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GradientTexture2D", "get_repeat", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GradientTexture2D {
        type Base = crate::classes::Texture2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"GradientTexture2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GradientTexture2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture2D > for GradientTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for GradientTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GradientTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GradientTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GradientTexture2D {
        
    }
    impl crate::obj::cap::GodotDefault for GradientTexture2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GradientTexture2D {
        type Target = crate::classes::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GradientTexture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GradientTexture2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GradientTexture2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GradientTexture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture > for $Class {
                
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
pub struct Fill {
    ord: i32
}
impl Fill {
    #[doc(alias = "FILL_LINEAR")]
    #[doc = "Godot enumerator name: `FILL_LINEAR`"]
    pub const LINEAR: Fill = Fill {
        ord: 0i32
    };
    #[doc(alias = "FILL_RADIAL")]
    #[doc = "Godot enumerator name: `FILL_RADIAL`"]
    pub const RADIAL: Fill = Fill {
        ord: 1i32
    };
    #[doc(alias = "FILL_SQUARE")]
    #[doc = "Godot enumerator name: `FILL_SQUARE`"]
    pub const SQUARE: Fill = Fill {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Fill") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Fill {
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
            Self::LINEAR => "LINEAR", Self::RADIAL => "RADIAL", Self::SQUARE => "SQUARE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Fill::LINEAR, Fill::RADIAL, Fill::SQUARE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Fill >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR", "FILL_LINEAR", Fill::LINEAR), crate::meta::inspect::EnumConstant::new("RADIAL", "FILL_RADIAL", Fill::RADIAL), crate::meta::inspect::EnumConstant::new("SQUARE", "FILL_SQUARE", Fill::SQUARE)]
        }
    }
}
impl crate::meta::GodotConvert for Fill {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Fill {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Fill {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Repeat {
    ord: i32
}
impl Repeat {
    pub const REPEAT_NONE: Repeat = Repeat {
        ord: 0i32
    };
    pub const REPEAT: Repeat = Repeat {
        ord: 1i32
    };
    pub const REPEAT_MIRROR: Repeat = Repeat {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Repeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Repeat {
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
            Self::REPEAT_NONE => "REPEAT_NONE", Self::REPEAT => "REPEAT", Self::REPEAT_MIRROR => "REPEAT_MIRROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Repeat::REPEAT_NONE, Repeat::REPEAT, Repeat::REPEAT_MIRROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Repeat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("REPEAT_NONE", "REPEAT_NONE", Repeat::REPEAT_NONE), crate::meta::inspect::EnumConstant::new("REPEAT", "REPEAT", Repeat::REPEAT), crate::meta::inspect::EnumConstant::new("REPEAT_MIRROR", "REPEAT_MIRROR", Repeat::REPEAT_MIRROR)]
        }
    }
}
impl crate::meta::GodotConvert for Repeat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Repeat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Repeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GradientTexture2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GradientTexture2D {
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