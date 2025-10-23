#![doc = "Sidecar module for class [`FontVariation`][crate::classes::FontVariation].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FontVariation` enums](https://docs.godotengine.org/en/stable/classes/class_fontvariation.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FontVariation.`\n\nInherits [`Font`][crate::classes::Font].\n\nRelated symbols:\n\n* [`IFontVariation`][crate::classes::IFontVariation]: virtual methods\n\n\nSee also [Godot docs for `FontVariation`](https://docs.godotengine.org/en/stable/classes/class_fontvariation.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`FontVariation::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FontVariation {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`FontVariation`][crate::classes::FontVariation].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IFont`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `FontVariation` methods](https://docs.godotengine.org/en/stable/classes/class_fontvariation.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFontVariation: crate::obj::GodotClass < Base = FontVariation > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FontVariation {
        pub fn set_base_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font >> >,);
            let args = (font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "set_base_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "get_base_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_variation_opentype(&mut self, coords: &Dictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Dictionary >,);
            let args = (RefArg::new(coords),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "set_variation_opentype", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_variation_opentype(&self,) -> Dictionary {
            type CallRet = Dictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "get_variation_opentype", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_variation_embolden(&mut self, strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "set_variation_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_variation_embolden(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "get_variation_embolden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_variation_face_index(&mut self, face_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (face_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "set_variation_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_variation_face_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "get_variation_face_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_variation_transform(&mut self, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "set_variation_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_variation_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "get_variation_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_opentype_features(&mut self, features: &Dictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Dictionary >,);
            let args = (RefArg::new(features),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "set_opentype_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_spacing(&mut self, spacing: crate::classes::text_server::SpacingType, value: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::SpacingType, i32,);
            let args = (spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "set_spacing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_baseline_offset(&mut self, baseline_offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3499usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "set_baseline_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_baseline_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontVariation", "get_baseline_offset", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for FontVariation {
        type Base = crate::classes::Font;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"FontVariation"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FontVariation {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Font > for FontVariation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for FontVariation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FontVariation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FontVariation {
        
    }
    impl crate::obj::cap::GodotDefault for FontVariation {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FontVariation {
        type Target = crate::classes::Font;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FontVariation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FontVariation`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FontVariation__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FontVariation > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Font > for $Class {
                
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
    use super::re_export::FontVariation;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for FontVariation {
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