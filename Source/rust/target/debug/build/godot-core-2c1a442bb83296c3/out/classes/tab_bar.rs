#![doc = "Sidecar module for class [`TabBar`][crate::classes::TabBar].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TabBar` enums](https://docs.godotengine.org/en/stable/classes/class_tabbar.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TabBar.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`tab_bar`][crate::classes::tab_bar]: sidecar module with related enum/flag types\n* [`ITabBar`][crate::classes::ITabBar]: virtual methods\n* [`SignalsOfTabBar`][crate::classes::tab_bar::SignalsOfTabBar]: signal collection\n\n\nSee also [Godot docs for `TabBar`](https://docs.godotengine.org/en/stable/classes/class_tabbar.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TabBar::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TabBar {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TabBar`][crate::classes::TabBar].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TabBar` methods](https://docs.godotengine.org/en/stable/classes/class_tabbar.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITabBar: crate::obj::GodotClass < Base = TabBar > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TabBar {
        pub fn set_tab_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_tab(&mut self, tab_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_current_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_tab(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_current_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_previous_tab(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_previous_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_previous_available(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "select_previous_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_next_available(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "select_next_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_title(&mut self, tab_idx: i32, title: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (tab_idx, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_title(&self, tab_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_tooltip(&mut self, tab_idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (tab_idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_tooltip(&self, tab_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_text_direction(&mut self, tab_idx: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (tab_idx, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_text_direction(&self, tab_idx: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_language(&mut self, tab_idx: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (tab_idx, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_language(&self, tab_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_icon(&mut self, tab_idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D >> >,);
            let args = (tab_idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_icon(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_icon_max_width(&mut self, tab_idx: i32, width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (tab_idx, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_icon_max_width(&self, tab_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_button_icon(&mut self, tab_idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D >> >,);
            let args = (tab_idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_button_icon(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_disabled(&mut self, tab_idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (tab_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tab_disabled(&self, tab_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "is_tab_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_hidden(&mut self, tab_idx: i32, hidden: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (tab_idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tab_hidden(&self, tab_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "is_tab_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_metadata(&mut self, tab_idx: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (tab_idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_metadata(&self, tab_idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_tab(&mut self, tab_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "remove_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_tab_full(&mut self, title: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D >> >,);
            let args = (title, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "add_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_tab_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_tab(&mut self,) {
            self.add_tab_ex() . done()
        }
        #[inline]
        pub fn add_tab_ex < 'a > (&'a mut self,) -> ExAddTab < 'a > {
            ExAddTab::new(self,)
        }
        pub fn get_tab_idx_at_point(&self, point: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_idx_at_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_alignment(&mut self, alignment: crate::classes::tab_bar::AlignmentMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tab_bar::AlignmentMode,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_alignment(&self,) -> crate::classes::tab_bar::AlignmentMode {
            type CallRet = crate::classes::tab_bar::AlignmentMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_tabs(&mut self, clip_tabs: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (clip_tabs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_clip_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_tabs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_clip_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_offset(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset_buttons_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_offset_buttons_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ensure_tab_visible(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "ensure_tab_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_rect(&self, tab_idx: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_tab(&mut self, from: i32, to: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "move_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_close_with_middle_mouse(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_close_with_middle_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_close_with_middle_mouse(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_close_with_middle_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_close_display_policy(&mut self, policy: crate::classes::tab_bar::CloseButtonDisplayPolicy,) {
            type CallRet = ();
            type CallParams = (crate::classes::tab_bar::CloseButtonDisplayPolicy,);
            let args = (policy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_close_display_policy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_close_display_policy(&self,) -> crate::classes::tab_bar::CloseButtonDisplayPolicy {
            type CallRet = crate::classes::tab_bar::CloseButtonDisplayPolicy;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_close_display_policy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_tab_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_max_tab_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_tab_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_max_tab_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scrolling_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_scrolling_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scrolling_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_scrolling_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_to_rearrange_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_drag_to_rearrange_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drag_to_rearrange_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_drag_to_rearrange_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tabs_rearrange_group(&mut self, group_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tabs_rearrange_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tabs_rearrange_group(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tabs_rearrange_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scroll_to_selected(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_scroll_to_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scroll_to_selected(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_scroll_to_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_select_with_rmb(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_select_with_rmb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_select_with_rmb(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_select_with_rmb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deselect_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_deselect_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_deselect_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_deselect_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_tabs(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "clear_tabs", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TabBar {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"TabBar"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TabBar {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TabBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TabBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TabBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TabBar {
        
    }
    impl crate::obj::cap::GodotDefault for TabBar {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TabBar {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TabBar {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TabBar`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TabBar__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TabBar > for $Class {
                
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
#[doc = "Default-param extender for [`TabBar::add_tab_ex`][super::TabBar::add_tab_ex]."]
#[must_use]
pub struct ExAddTab < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TabBar, title: CowArg < 'a, GString >, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D >> >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTab < 'a > {
    fn new(surround_object: &'a mut re_export::TabBar,) -> Self {
        let title = GString::from("");
        let icon = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: CowArg::Owned(title), icon: icon.into_arg(),
        }
    }
    #[inline]
    pub fn title(self, title: impl AsArg < GString > + 'a) -> Self {
        Self {
            title: title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, title, icon,
        }
        = self;
        re_export::TabBar::add_tab_full(surround_object, title, icon,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlignmentMode {
    ord: i32
}
impl AlignmentMode {
    #[doc(alias = "ALIGNMENT_LEFT")]
    #[doc = "Godot enumerator name: `ALIGNMENT_LEFT`"]
    pub const LEFT: AlignmentMode = AlignmentMode {
        ord: 0i32
    };
    #[doc(alias = "ALIGNMENT_CENTER")]
    #[doc = "Godot enumerator name: `ALIGNMENT_CENTER`"]
    pub const CENTER: AlignmentMode = AlignmentMode {
        ord: 1i32
    };
    #[doc(alias = "ALIGNMENT_RIGHT")]
    #[doc = "Godot enumerator name: `ALIGNMENT_RIGHT`"]
    pub const RIGHT: AlignmentMode = AlignmentMode {
        ord: 2i32
    };
    #[doc(alias = "ALIGNMENT_MAX")]
    #[doc = "Godot enumerator name: `ALIGNMENT_MAX`"]
    pub const MAX: AlignmentMode = AlignmentMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AlignmentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlignmentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlignmentMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::LEFT => "LEFT", Self::CENTER => "CENTER", Self::RIGHT => "RIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AlignmentMode::LEFT, AlignmentMode::CENTER, AlignmentMode::RIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AlignmentMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LEFT", "ALIGNMENT_LEFT", AlignmentMode::LEFT), crate::meta::inspect::EnumConstant::new("CENTER", "ALIGNMENT_CENTER", AlignmentMode::CENTER), crate::meta::inspect::EnumConstant::new("RIGHT", "ALIGNMENT_RIGHT", AlignmentMode::RIGHT), crate::meta::inspect::EnumConstant::new("MAX", "ALIGNMENT_MAX", AlignmentMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for AlignmentMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for AlignmentMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AlignmentMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlignmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CloseButtonDisplayPolicy {
    ord: i32
}
impl CloseButtonDisplayPolicy {
    #[doc(alias = "CLOSE_BUTTON_SHOW_NEVER")]
    #[doc = "Godot enumerator name: `CLOSE_BUTTON_SHOW_NEVER`"]
    pub const SHOW_NEVER: CloseButtonDisplayPolicy = CloseButtonDisplayPolicy {
        ord: 0i32
    };
    #[doc(alias = "CLOSE_BUTTON_SHOW_ACTIVE_ONLY")]
    #[doc = "Godot enumerator name: `CLOSE_BUTTON_SHOW_ACTIVE_ONLY`"]
    pub const SHOW_ACTIVE_ONLY: CloseButtonDisplayPolicy = CloseButtonDisplayPolicy {
        ord: 1i32
    };
    #[doc(alias = "CLOSE_BUTTON_SHOW_ALWAYS")]
    #[doc = "Godot enumerator name: `CLOSE_BUTTON_SHOW_ALWAYS`"]
    pub const SHOW_ALWAYS: CloseButtonDisplayPolicy = CloseButtonDisplayPolicy {
        ord: 2i32
    };
    #[doc(alias = "CLOSE_BUTTON_MAX")]
    #[doc = "Godot enumerator name: `CLOSE_BUTTON_MAX`"]
    pub const MAX: CloseButtonDisplayPolicy = CloseButtonDisplayPolicy {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CloseButtonDisplayPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CloseButtonDisplayPolicy") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CloseButtonDisplayPolicy {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::SHOW_NEVER => "SHOW_NEVER", Self::SHOW_ACTIVE_ONLY => "SHOW_ACTIVE_ONLY", Self::SHOW_ALWAYS => "SHOW_ALWAYS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CloseButtonDisplayPolicy::SHOW_NEVER, CloseButtonDisplayPolicy::SHOW_ACTIVE_ONLY, CloseButtonDisplayPolicy::SHOW_ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CloseButtonDisplayPolicy >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SHOW_NEVER", "CLOSE_BUTTON_SHOW_NEVER", CloseButtonDisplayPolicy::SHOW_NEVER), crate::meta::inspect::EnumConstant::new("SHOW_ACTIVE_ONLY", "CLOSE_BUTTON_SHOW_ACTIVE_ONLY", CloseButtonDisplayPolicy::SHOW_ACTIVE_ONLY), crate::meta::inspect::EnumConstant::new("SHOW_ALWAYS", "CLOSE_BUTTON_SHOW_ALWAYS", CloseButtonDisplayPolicy::SHOW_ALWAYS), crate::meta::inspect::EnumConstant::new("MAX", "CLOSE_BUTTON_MAX", CloseButtonDisplayPolicy::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CloseButtonDisplayPolicy {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for CloseButtonDisplayPolicy {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CloseButtonDisplayPolicy {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CloseButtonDisplayPolicy {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TabBar;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TabBar`][crate::classes::TabBar] class."]
    pub struct SignalsOfTabBar < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTabBar < 'c, C > {
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_selected(&mut self) -> SigTabSelected < 'c, C > {
            SigTabSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_selected")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_changed(&mut self) -> SigTabChanged < 'c, C > {
            SigTabChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_changed")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_clicked(&mut self) -> SigTabClicked < 'c, C > {
            SigTabClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_clicked")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_rmb_clicked(&mut self) -> SigTabRmbClicked < 'c, C > {
            SigTabRmbClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_rmb_clicked")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_close_pressed(&mut self) -> SigTabClosePressed < 'c, C > {
            SigTabClosePressed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_close_pressed")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_button_pressed(&mut self) -> SigTabButtonPressed < 'c, C > {
            SigTabButtonPressed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_button_pressed")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_hovered(&mut self) -> SigTabHovered < 'c, C > {
            SigTabHovered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_hovered")
            }
        }
        #[doc = "Signature: `(idx_to: i64)`"]
        pub fn active_tab_rearranged(&mut self) -> SigActiveTabRearranged < 'c, C > {
            SigActiveTabRearranged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "active_tab_rearranged")
            }
        }
    }
    type TypedSigTabSelected < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabSelected < 'c, C: WithSignals > {
        typed: TypedSigTabSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabSelected < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabSelected < 'c, C > {
        type Target = TypedSigTabSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabChanged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabChanged < 'c, C: WithSignals > {
        typed: TypedSigTabChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabChanged < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabChanged < 'c, C > {
        type Target = TypedSigTabChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabClicked < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabClicked < 'c, C: WithSignals > {
        typed: TypedSigTabClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabClicked < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabClicked < 'c, C > {
        type Target = TypedSigTabClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabRmbClicked < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabRmbClicked < 'c, C: WithSignals > {
        typed: TypedSigTabRmbClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabRmbClicked < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabRmbClicked < 'c, C > {
        type Target = TypedSigTabRmbClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabRmbClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabClosePressed < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabClosePressed < 'c, C: WithSignals > {
        typed: TypedSigTabClosePressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabClosePressed < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabClosePressed < 'c, C > {
        type Target = TypedSigTabClosePressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabClosePressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabButtonPressed < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabButtonPressed < 'c, C: WithSignals > {
        typed: TypedSigTabButtonPressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabButtonPressed < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabButtonPressed < 'c, C > {
        type Target = TypedSigTabButtonPressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabButtonPressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabHovered < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabHovered < 'c, C: WithSignals > {
        typed: TypedSigTabHovered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabHovered < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabHovered < 'c, C > {
        type Target = TypedSigTabHovered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabHovered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigActiveTabRearranged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigActiveTabRearranged < 'c, C: WithSignals > {
        typed: TypedSigActiveTabRearranged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigActiveTabRearranged < 'c, C > {
        pub fn emit(&mut self, idx_to: i64,) {
            self.typed.emit_tuple((idx_to,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigActiveTabRearranged < 'c, C > {
        type Target = TypedSigActiveTabRearranged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigActiveTabRearranged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for TabBar {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTabBar < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTabBar < 'c, C > {
        type Target = < < TabBar as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TabBar;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTabBar < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TabBar;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}