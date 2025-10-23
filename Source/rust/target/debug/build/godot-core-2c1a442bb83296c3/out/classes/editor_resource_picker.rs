#![doc = "Sidecar module for class [`EditorResourcePicker`][crate::classes::EditorResourcePicker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorResourcePicker` enums](https://docs.godotengine.org/en/stable/classes/class_editorresourcepicker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorResourcePicker.`\n\nInherits [`HBoxContainer`][crate::classes::HBoxContainer].\n\nRelated symbols:\n\n* [`editor_resource_picker`][crate::classes::editor_resource_picker]: sidecar module with related enum/flag types\n* [`IEditorResourcePicker`][crate::classes::IEditorResourcePicker]: virtual methods\n* [`SignalsOfEditorResourcePicker`][crate::classes::editor_resource_picker::SignalsOfEditorResourcePicker]: signal collection\n\n\nSee also [Godot docs for `EditorResourcePicker`](https://docs.godotengine.org/en/stable/classes/class_editorresourcepicker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorResourcePicker::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorResourcePicker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorResourcePicker`][crate::classes::EditorResourcePicker].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IHBoxContainer`][crate::classes::IHBoxContainer] > [`IBoxContainer`][crate::classes::IBoxContainer] > [`IContainer`][crate::classes::IContainer] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `EditorResourcePicker` methods](https://docs.godotengine.org/en/stable/classes/class_editorresourcepicker.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorResourcePicker: crate::obj::GodotClass < Base = EditorResourcePicker > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
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
        fn set_create_options(&mut self, menu_node: Option < Gd < crate::classes::Object > >,) {
            unimplemented !()
        }
        fn handle_menu_selected(&mut self, id: i32,) -> bool {
            unimplemented !()
        }
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
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
    impl EditorResourcePicker {
        pub fn set_base_type(&mut self, base_type: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (base_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "set_base_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_type(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "get_base_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allowed_types(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "get_allowed_types", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edited_resource(&mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource >> >,);
            let args = (resource.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "set_edited_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_resource(&mut self,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "get_edited_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_toggle_mode(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "set_toggle_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_toggle_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "is_toggle_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_toggle_pressed(&mut self, pressed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "set_toggle_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editable(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "set_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePicker", "is_editable", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorResourcePicker {
        type Base = crate::classes::HBoxContainer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"EditorResourcePicker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorResourcePicker {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::HBoxContainer > for EditorResourcePicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoxContainer > for EditorResourcePicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for EditorResourcePicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for EditorResourcePicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for EditorResourcePicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorResourcePicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorResourcePicker {
        
    }
    impl crate::obj::cap::GodotDefault for EditorResourcePicker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorResourcePicker {
        type Target = crate::classes::HBoxContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorResourcePicker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorResourcePicker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorResourcePicker__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorResourcePicker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::HBoxContainer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BoxContainer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Container > for $Class {
                
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
    use super::re_export::EditorResourcePicker;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorResourcePicker`][crate::classes::EditorResourcePicker] class."]
    pub struct SignalsOfEditorResourcePicker < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorResourcePicker < 'c, C > {
        #[doc = "Signature: `(resource: Gd<Resource>, inspect: bool)`"]
        pub fn resource_selected(&mut self) -> SigResourceSelected < 'c, C > {
            SigResourceSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "resource_selected")
            }
        }
        #[doc = "Signature: `(resource: Gd<Resource>)`"]
        pub fn resource_changed(&mut self) -> SigResourceChanged < 'c, C > {
            SigResourceChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "resource_changed")
            }
        }
    }
    type TypedSigResourceSelected < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Resource >, bool,) >;
    pub struct SigResourceSelected < 'c, C: WithSignals > {
        typed: TypedSigResourceSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourceSelected < 'c, C > {
        pub fn emit(&mut self, resource: Gd < crate::classes::Resource >, inspect: bool,) {
            self.typed.emit_tuple((resource, inspect,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourceSelected < 'c, C > {
        type Target = TypedSigResourceSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourceSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigResourceChanged < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Resource >,) >;
    pub struct SigResourceChanged < 'c, C: WithSignals > {
        typed: TypedSigResourceChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourceChanged < 'c, C > {
        pub fn emit(&mut self, resource: Gd < crate::classes::Resource >,) {
            self.typed.emit_tuple((resource,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourceChanged < 'c, C > {
        type Target = TypedSigResourceChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourceChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorResourcePicker {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorResourcePicker < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorResourcePicker < 'c, C > {
        type Target = < < EditorResourcePicker as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorResourcePicker;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorResourcePicker < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorResourcePicker;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}