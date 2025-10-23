#![doc = "Sidecar module for class [`VisualShaderNodeDerivativeFunc`][crate::classes::VisualShaderNodeDerivativeFunc].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeDerivativeFunc` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodederivativefunc.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeDerivativeFunc.`\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_derivative_func`][crate::classes::visual_shader_node_derivative_func]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeDerivativeFunc`][crate::classes::IVisualShaderNodeDerivativeFunc]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeDerivativeFunc`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodederivativefunc.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeDerivativeFunc::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeDerivativeFunc {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShaderNodeDerivativeFunc`][crate::classes::VisualShaderNodeDerivativeFunc].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IVisualShaderNode`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShaderNodeDerivativeFunc` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodederivativefunc.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeDerivativeFunc: crate::obj::GodotClass < Base = VisualShaderNodeDerivativeFunc > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeDerivativeFunc {
        pub fn set_op_type(&mut self, type_: crate::classes::visual_shader_node_derivative_func::OpType,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_derivative_func::OpType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeDerivativeFunc", "set_op_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_op_type(&self,) -> crate::classes::visual_shader_node_derivative_func::OpType {
            type CallRet = crate::classes::visual_shader_node_derivative_func::OpType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeDerivativeFunc", "get_op_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_function(&mut self, func: crate::classes::visual_shader_node_derivative_func::Function,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_derivative_func::Function,);
            let args = (func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeDerivativeFunc", "set_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_function(&self,) -> crate::classes::visual_shader_node_derivative_func::Function {
            type CallRet = crate::classes::visual_shader_node_derivative_func::Function;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeDerivativeFunc", "get_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_precision(&mut self, precision: crate::classes::visual_shader_node_derivative_func::Precision,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_derivative_func::Precision,);
            let args = (precision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeDerivativeFunc", "set_precision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_precision(&self,) -> crate::classes::visual_shader_node_derivative_func::Precision {
            type CallRet = crate::classes::visual_shader_node_derivative_func::Precision;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeDerivativeFunc", "get_precision", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeDerivativeFunc {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"VisualShaderNodeDerivativeFunc"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeDerivativeFunc {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeDerivativeFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeDerivativeFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeDerivativeFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeDerivativeFunc {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeDerivativeFunc {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeDerivativeFunc {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeDerivativeFunc {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeDerivativeFunc`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeDerivativeFunc__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeDerivativeFunc > for $Class {
                
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
pub struct OpType {
    ord: i32
}
impl OpType {
    #[doc(alias = "OP_TYPE_SCALAR")]
    #[doc = "Godot enumerator name: `OP_TYPE_SCALAR`"]
    pub const SCALAR: OpType = OpType {
        ord: 0i32
    };
    #[doc(alias = "OP_TYPE_VECTOR_2D")]
    #[doc = "Godot enumerator name: `OP_TYPE_VECTOR_2D`"]
    pub const VECTOR_2D: OpType = OpType {
        ord: 1i32
    };
    #[doc(alias = "OP_TYPE_VECTOR_3D")]
    #[doc = "Godot enumerator name: `OP_TYPE_VECTOR_3D`"]
    pub const VECTOR_3D: OpType = OpType {
        ord: 2i32
    };
    #[doc(alias = "OP_TYPE_VECTOR_4D")]
    #[doc = "Godot enumerator name: `OP_TYPE_VECTOR_4D`"]
    pub const VECTOR_4D: OpType = OpType {
        ord: 3i32
    };
    #[doc(alias = "OP_TYPE_MAX")]
    #[doc = "Godot enumerator name: `OP_TYPE_MAX`"]
    pub const MAX: OpType = OpType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for OpType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OpType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OpType {
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
            Self::SCALAR => "SCALAR", Self::VECTOR_2D => "VECTOR_2D", Self::VECTOR_3D => "VECTOR_3D", Self::VECTOR_4D => "VECTOR_4D", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[OpType::SCALAR, OpType::VECTOR_2D, OpType::VECTOR_3D, OpType::VECTOR_4D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < OpType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALAR", "OP_TYPE_SCALAR", OpType::SCALAR), crate::meta::inspect::EnumConstant::new("VECTOR_2D", "OP_TYPE_VECTOR_2D", OpType::VECTOR_2D), crate::meta::inspect::EnumConstant::new("VECTOR_3D", "OP_TYPE_VECTOR_3D", OpType::VECTOR_3D), crate::meta::inspect::EnumConstant::new("VECTOR_4D", "OP_TYPE_VECTOR_4D", OpType::VECTOR_4D), crate::meta::inspect::EnumConstant::new("MAX", "OP_TYPE_MAX", OpType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for OpType {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for OpType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for OpType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OpType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Function {
    ord: i32
}
impl Function {
    #[doc(alias = "FUNC_SUM")]
    #[doc = "Godot enumerator name: `FUNC_SUM`"]
    pub const SUM: Function = Function {
        ord: 0i32
    };
    #[doc(alias = "FUNC_X")]
    #[doc = "Godot enumerator name: `FUNC_X`"]
    pub const X: Function = Function {
        ord: 1i32
    };
    #[doc(alias = "FUNC_Y")]
    #[doc = "Godot enumerator name: `FUNC_Y`"]
    pub const Y: Function = Function {
        ord: 2i32
    };
    #[doc(alias = "FUNC_MAX")]
    #[doc = "Godot enumerator name: `FUNC_MAX`"]
    pub const MAX: Function = Function {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Function") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Function {
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
            Self::SUM => "SUM", Self::X => "X", Self::Y => "Y", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Function::SUM, Function::X, Function::Y]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Function >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SUM", "FUNC_SUM", Function::SUM), crate::meta::inspect::EnumConstant::new("X", "FUNC_X", Function::X), crate::meta::inspect::EnumConstant::new("Y", "FUNC_Y", Function::Y), crate::meta::inspect::EnumConstant::new("MAX", "FUNC_MAX", Function::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Function {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for Function {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Function {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Function {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Precision {
    ord: i32
}
impl Precision {
    #[doc(alias = "PRECISION_NONE")]
    #[doc = "Godot enumerator name: `PRECISION_NONE`"]
    pub const NONE: Precision = Precision {
        ord: 0i32
    };
    #[doc(alias = "PRECISION_COARSE")]
    #[doc = "Godot enumerator name: `PRECISION_COARSE`"]
    pub const COARSE: Precision = Precision {
        ord: 1i32
    };
    #[doc(alias = "PRECISION_FINE")]
    #[doc = "Godot enumerator name: `PRECISION_FINE`"]
    pub const FINE: Precision = Precision {
        ord: 2i32
    };
    #[doc(alias = "PRECISION_MAX")]
    #[doc = "Godot enumerator name: `PRECISION_MAX`"]
    pub const MAX: Precision = Precision {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Precision {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Precision") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Precision {
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
            Self::NONE => "NONE", Self::COARSE => "COARSE", Self::FINE => "FINE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Precision::NONE, Precision::COARSE, Precision::FINE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Precision >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "PRECISION_NONE", Precision::NONE), crate::meta::inspect::EnumConstant::new("COARSE", "PRECISION_COARSE", Precision::COARSE), crate::meta::inspect::EnumConstant::new("FINE", "PRECISION_FINE", Precision::FINE), crate::meta::inspect::EnumConstant::new("MAX", "PRECISION_MAX", Precision::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Precision {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for Precision {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Precision {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Precision {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeDerivativeFunc;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeDerivativeFunc {
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