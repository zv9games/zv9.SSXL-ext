#![doc = "Sidecar module for class [`VisualShaderNodeOutput`][crate::classes::VisualShaderNodeOutput].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeOutput` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeoutput.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeOutput.`\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `VisualShaderNodeOutput`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeoutput.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNodeOutput>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeOutput {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl VisualShaderNodeOutput {
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
    impl crate::obj::GodotClass for VisualShaderNodeOutput {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"VisualShaderNodeOutput"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeOutput {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeOutput {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeOutput {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeOutput {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeOutput {
        
    }
    impl std::ops::Deref for VisualShaderNodeOutput {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeOutput {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeOutput__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `VisualShaderNodeOutput` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeOutput;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeOutput {
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