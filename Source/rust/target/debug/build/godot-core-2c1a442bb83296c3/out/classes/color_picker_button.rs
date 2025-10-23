#![doc = "Sidecar module for class [`ColorPickerButton`][crate::classes::ColorPickerButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ColorPickerButton` enums](https://docs.godotengine.org/en/stable/classes/class_colorpickerbutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ColorPickerButton.`\n\nInherits [`Button`][crate::classes::Button].\n\nRelated symbols:\n\n* [`color_picker_button`][crate::classes::color_picker_button]: sidecar module with related enum/flag types\n* [`IColorPickerButton`][crate::classes::IColorPickerButton]: virtual methods\n* [`SignalsOfColorPickerButton`][crate::classes::color_picker_button::SignalsOfColorPickerButton]: signal collection\n\n\nSee also [Godot docs for `ColorPickerButton`](https://docs.godotengine.org/en/stable/classes/class_colorpickerbutton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ColorPickerButton::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ColorPickerButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ColorPickerButton`][crate::classes::ColorPickerButton].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IButton`][crate::classes::IButton] > [`IBaseButton`][crate::classes::IBaseButton] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ColorPickerButton` methods](https://docs.godotengine.org/en/stable/classes/class_colorpickerbutton.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IColorPickerButton: crate::obj::GodotClass < Base = ColorPickerButton > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn pressed(&mut self,) {
            unimplemented !()
        }
        fn toggled(&mut self, toggled_on: bool,) {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn accessibility_get_contextual_info(&self,) -> GString {
            unimplemented !()
        }
        fn get_accessibility_container_name(&self, node: Option < Gd < crate::classes::Node > >,) -> GString {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl ColorPickerButton {
        pub fn set_pick_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPickerButton", "set_pick_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pick_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPickerButton", "get_pick_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_picker(&mut self,) -> Option < Gd < crate::classes::ColorPicker > > {
            type CallRet = Option < Gd < crate::classes::ColorPicker > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPickerButton", "get_picker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_popup(&mut self,) -> Option < Gd < crate::classes::PopupPanel > > {
            type CallRet = Option < Gd < crate::classes::PopupPanel > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPickerButton", "get_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edit_alpha(&mut self, show: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPickerButton", "set_edit_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editing_alpha(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPickerButton", "is_editing_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edit_intensity(&mut self, show: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPickerButton", "set_edit_intensity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editing_intensity(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPickerButton", "is_editing_intensity", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ColorPickerButton {
        type Base = crate::classes::Button;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"ColorPickerButton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ColorPickerButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Button > for ColorPickerButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BaseButton > for ColorPickerButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ColorPickerButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ColorPickerButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ColorPickerButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ColorPickerButton {
        
    }
    impl crate::obj::cap::GodotDefault for ColorPickerButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ColorPickerButton {
        type Target = crate::classes::Button;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ColorPickerButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ColorPickerButton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ColorPickerButton__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ColorPickerButton > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Button > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BaseButton > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
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
    use super::re_export::ColorPickerButton;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ColorPickerButton`][crate::classes::ColorPickerButton] class."]
    pub struct SignalsOfColorPickerButton < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfColorPickerButton < 'c, C > {
        #[doc = "Signature: `(color: Color)`"]
        pub fn color_changed(&mut self) -> SigColorChanged < 'c, C > {
            SigColorChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "color_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn popup_closed(&mut self) -> SigPopupClosed < 'c, C > {
            SigPopupClosed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "popup_closed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn picker_created(&mut self) -> SigPickerCreated < 'c, C > {
            SigPickerCreated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "picker_created")
            }
        }
    }
    type TypedSigColorChanged < 'c, C > = TypedSignal < 'c, C, (Color,) >;
    pub struct SigColorChanged < 'c, C: WithSignals > {
        typed: TypedSigColorChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigColorChanged < 'c, C > {
        pub fn emit(&mut self, color: Color,) {
            self.typed.emit_tuple((color,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigColorChanged < 'c, C > {
        type Target = TypedSigColorChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigColorChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPopupClosed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPopupClosed < 'c, C: WithSignals > {
        typed: TypedSigPopupClosed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPopupClosed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPopupClosed < 'c, C > {
        type Target = TypedSigPopupClosed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPopupClosed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPickerCreated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPickerCreated < 'c, C: WithSignals > {
        typed: TypedSigPickerCreated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPickerCreated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPickerCreated < 'c, C > {
        type Target = TypedSigPickerCreated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPickerCreated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ColorPickerButton {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfColorPickerButton < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfColorPickerButton < 'c, C > {
        type Target = < < ColorPickerButton as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ColorPickerButton;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfColorPickerButton < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ColorPickerButton;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}