#![doc = "Sidecar module for class [`StreamPeer`][crate::classes::StreamPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StreamPeer` enums](https://docs.godotengine.org/en/stable/classes/class_streampeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StreamPeer.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`stream_peer`][crate::classes::stream_peer]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `StreamPeer`](https://docs.godotengine.org/en/stable/classes/class_streampeer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<StreamPeer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StreamPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl StreamPeer {
        pub fn put_data(&mut self, data: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_partial_data(&mut self, data: &PackedByteArray,) -> VariantArray {
            type CallRet = VariantArray;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_partial_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data(&mut self, bytes: i32,) -> VariantArray {
            type CallRet = VariantArray;
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_partial_data(&mut self, bytes: i32,) -> VariantArray {
            type CallRet = VariantArray;
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_partial_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_available_bytes(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_available_bytes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_big_endian(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "set_big_endian", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_big_endian_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "is_big_endian_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_8(&mut self, value: i8,) {
            type CallRet = ();
            type CallParams = (i8,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u8(&mut self, value: u8,) {
            type CallRet = ();
            type CallParams = (u8,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_16(&mut self, value: i16,) {
            type CallRet = ();
            type CallParams = (i16,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u16(&mut self, value: u16,) {
            type CallRet = ();
            type CallParams = (u16,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_32(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u32(&mut self, value: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_64(&mut self, value: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u64(&mut self, value: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_half(&mut self, value: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_half", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_float(&mut self, value: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_double(&mut self, value: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_string(&mut self, value: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (value.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_utf8_string(&mut self, value: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (value.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_utf8_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn put_var_full(&mut self, value: RefArg < Variant >, full_objects: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (value, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::put_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn put_var(&mut self, value: &Variant,) {
            self.put_var_ex(value,) . done()
        }
        #[inline]
        pub fn put_var_ex < 'a > (&'a mut self, value: &'a Variant,) -> ExPutVar < 'a > {
            ExPutVar::new(self, value,)
        }
        pub fn get_8(&mut self,) -> i8 {
            type CallRet = i8;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u8(&mut self,) -> u8 {
            type CallRet = u8;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_16(&mut self,) -> i16 {
            type CallRet = i16;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u16(&mut self,) -> u16 {
            type CallRet = u16;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_32(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u32(&mut self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_64(&mut self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u64(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_half(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_half", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_float(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_double(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_string_full(&mut self, bytes: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_string(&mut self,) -> GString {
            self.get_string_ex() . done()
        }
        #[inline]
        pub fn get_string_ex < 'a > (&'a mut self,) -> ExGetString < 'a > {
            ExGetString::new(self,)
        }
        pub(crate) fn get_utf8_string_full(&mut self, bytes: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_utf8_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_utf8_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_utf8_string(&mut self,) -> GString {
            self.get_utf8_string_ex() . done()
        }
        #[inline]
        pub fn get_utf8_string_ex < 'a > (&'a mut self,) -> ExGetUtf8String < 'a > {
            ExGetUtf8String::new(self,)
        }
        pub(crate) fn get_var_full(&mut self, allow_objects: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams = (bool,);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_var(&mut self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[inline]
        pub fn get_var_ex < 'a > (&'a mut self,) -> ExGetVar < 'a > {
            ExGetVar::new(self,)
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
    impl crate::obj::GodotClass for StreamPeer {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_ascii(c"StreamPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StreamPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StreamPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StreamPeer {
        
    }
    impl std::ops::Deref for StreamPeer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StreamPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_StreamPeer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `StreamPeer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`StreamPeer::put_var_ex`][super::StreamPeer::put_var_ex]."]
#[must_use]
pub struct ExPutVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeer, value: CowArg < 'a, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPutVar < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer, value: &'a Variant,) -> Self {
        let full_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, value: CowArg::Borrowed(value), full_objects: full_objects,
        }
    }
    #[inline]
    pub fn full_objects(self, full_objects: bool) -> Self {
        Self {
            full_objects: full_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, value, full_objects,
        }
        = self;
        re_export::StreamPeer::put_var_full(surround_object, value.cow_as_arg(), full_objects,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_string_ex`][super::StreamPeer::get_string_ex]."]
#[must_use]
pub struct ExGetString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeer, bytes: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetString < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
        let bytes = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: bytes,
        }
    }
    #[inline]
    pub fn bytes(self, bytes: i32) -> Self {
        Self {
            bytes: bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, bytes,
        }
        = self;
        re_export::StreamPeer::get_string_full(surround_object, bytes,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_utf8_string_ex`][super::StreamPeer::get_utf8_string_ex]."]
#[must_use]
pub struct ExGetUtf8String < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeer, bytes: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUtf8String < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
        let bytes = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: bytes,
        }
    }
    #[inline]
    pub fn bytes(self, bytes: i32) -> Self {
        Self {
            bytes: bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, bytes,
        }
        = self;
        re_export::StreamPeer::get_utf8_string_full(surround_object, bytes,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_var_ex`][super::StreamPeer::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeer, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVar < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
        let allow_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, allow_objects: allow_objects,
        }
    }
    #[inline]
    pub fn allow_objects(self, allow_objects: bool) -> Self {
        Self {
            allow_objects: allow_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, allow_objects,
        }
        = self;
        re_export::StreamPeer::get_var_full(surround_object, allow_objects,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::StreamPeer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for StreamPeer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}