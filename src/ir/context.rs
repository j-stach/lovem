
use libc::c_void;

use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;
use super::types::Type;
use super::{values as val, metadata as md};


wrapper!(Context, LLVMContextRef);

impl Drop for Context {
    fn drop(&mut self) {unsafe { llvm::LLVMContextDispose(self.0) }}
}

impl Context {
    // TODO Docs
    pub fn new() -> Self {
        unsafe { Self(llvm::LLVMContextCreate()) }
    }

    // TODO Docs
    pub fn global() -> Self {
        unsafe { Self(llvm::LLVMGetGlobalContext()) }
    }

    // TODO Docs
    pub fn get_diagnostic_context(&self) -> *mut c_void { // TODO Make safe
        unsafe { llvm::LLVMContextGetDiagnosticContext(self.0) }
    }

    // TODO Docs
    pub fn get_diagnostic_handler(&self) -> LLVMDiagnosticHandler {
        unsafe { llvm::LLVMContextGetDiagnosticHandler(self.0) }
    }

    // TODO Docs
    pub fn set_diagnostic_handler(&self, handler: LLVMDiagnosticHandler, diag_context: *mut c_void) { // TODO Make safe
        unsafe { llvm::LLVMContextSetDiagnosticHandler(self.0, handler, diag_context) }
    }

    // TODO Docs
    pub fn set_yield_callback(&self, callback: LLVMYieldCallback, opaque_handler: *mut c_void) { // TODO Make safe
        unsafe { llvm::LLVMContextSetYieldCallback(self.0, callback, opaque_handler) }
    }

    // TODO Docs
    pub fn set_discard_value_names(&self, discard: bool) {
        unsafe { llvm::LLVMContextSetDiscardValueNames(self.0, bool_to_llvm!(discard)) }
    }

    // TODO Docs
    pub fn should_discard_value_names(&self) -> bool { // TODO Make safe
        bool_to_rust!( llvm::LLVMContextShouldDiscardValueNames(self.0) )
    }

    // TODO Docs
    pub fn create_enum_attribute(&self, kind_id: u32, val: u64) -> LLVMAttributeRef {
        unsafe { llvm::LLVMCreateEnumAttribute(self.0, kind_id, val) }
    }

    // TODO Docs
    pub fn create_string_attribute(&self, k: &str, v: &str) -> LLVMAttributeRef {
        unsafe {
            llvm::LLVMCreateStringAttribute(self.0, str_to_cstr!(k), size!(k), str_to_cstr!(v), size!(v))
        }
    }

    // TODO Docs
    pub fn create_type_attribute<T: Type>(&self, kind_id: u32, typ: T) -> LLVMAttributeRef {
        unsafe { llvm::LLVMCreateTypeAttribute(self.0, kind_id, expose!(typ)) }
    }

    // TODO Docs
    pub fn create_builder(&self) -> super::builder::Builder {
        super::builder::Builder::wrap(
            unsafe { llvm::LLVMCreateBuilderInContext(self.0) }
        )
    }

    // TODO Docs
    pub fn create_block(&self, name: &str) -> LLVMBasicBlockRef {
        unsafe { llvm::LLVMCreateBasicBlockInContext(self.0, str_to_cstr!(name)) }
    }

    // TODO Docs
    pub fn append_block(&self, function: val::Function, name: &str) -> LLVMBasicBlockRef {
        unsafe { llvm::LLVMAppendBasicBlockInContext(self.0, expose!(function), str_to_cstr!(name)) }
    }

    // TODO Docs
    pub fn get_type_by_name(&self, name: &str) -> Raw {
        Raw::wrap(unsafe { llvm::LLVMGetTypeByName2(self.0, str_to_cstr!(name)) })
    }

}


use super::types::*;

#[allow(unused_macros)]
macro_rules! type_in_context {
    ($op_name:ident, $fn:path, $ret_typ:ty $(, $($argn:ident: $argv:path),*)?) => {
        impl Context {
            #[doc = "TODO: Dynamically link to LLVM documentation"]
            pub fn $op_name(&self $(, $($argn: $argv),*)?) -> $ret_typ {
                unsafe {
                    <$ret_typ>::wrap( $fn(self.0 $(, $($argn),*)?) )
                }
            }
        }
    }
}

type_in_context!(create_void, llvm::LLVMVoidTypeInContext, Void);

type_in_context!(create_float, llvm::LLVMFloatTypeInContext, Float);
type_in_context!(create_half, llvm::LLVMHalfTypeInContext, Half);
type_in_context!(create_double, llvm::LLVMDoubleTypeInContext, Double);
type_in_context!(create_b_float, llvm::LLVMBFloatTypeInContext, BFloat);
type_in_context!(create_fp_128, llvm::LLVMFP128TypeInContext, FP128);
type_in_context!(create_ppc_fp_128, llvm::LLVMPPCFP128TypeInContext, PPCFP128);

type_in_context!(create_int_1, llvm::LLVMInt1TypeInContext, Int1);
type_in_context!(create_int_8, llvm::LLVMInt8TypeInContext, Int8);
type_in_context!(create_int_16, llvm::LLVMInt16TypeInContext, Int16);
type_in_context!(create_int_32, llvm::LLVMInt32TypeInContext, Int32);
type_in_context!(create_int_64, llvm::LLVMInt64TypeInContext, Int64);
type_in_context!(create_int_128, llvm::LLVMInt128TypeInContext, Int128);
type_in_context!(create_int, llvm::LLVMIntTypeInContext, Int, num_bits: u32);

type_in_context!(create_x86_mmx, llvm::LLVMX86MMXTypeInContext, X86MMX);
type_in_context!(create_x86_amx, llvm::LLVMX86AMXTypeInContext, X86AMX);
type_in_context!(create_x86_fp_80, llvm::LLVMX86FP80TypeInContext, X86FP80);

impl Context {
    // TODO Docs
    pub fn create_token(&self) -> Token {
        Token::wrap(unsafe { llvm::LLVMTokenTypeInContext(self.0) })
    }

    //  TODO Docs -- LLVMMetadataTypeInContext
    pub fn create_metadata(&self) -> md::MetadataType {
        md::MetadataType::wrap(unsafe { llvm::LLVMMetadataTypeInContext(self.0) })
    }

    // TODO Docs
    pub fn create_metadata_node(&self, metadata: Vec<md::ActualMetadata>) -> md::ActualMetadata {
        let meta = unsafe { llvm::LLVMMDNodeInContext2(self.0, expose_array!(metadata), metadata.len() ) };
        md::ActualMetadata::wrap(meta)
    }

    // TODO Docs
    pub fn create_metadata_string(&self, string: &str) -> md::ActualMetadata {
        let meta = unsafe { llvm::LLVMMDStringInContext2(self.0, str_to_cstr!(string), string.len() ) };
        md::ActualMetadata::wrap(meta)
    }

    // TODO Docs
    pub fn create_struct(&self, elements: Vec<Box<dyn Type>>, is_packed: bool) -> Struct {
        let structo = unsafe {
            llvm::LLVMStructTypeInContext(
                self.0,
                expose_array!(elements),
                size!(elements),
                bool_to_llvm!(is_packed)
            )
        };
        Struct::wrap(structo)
    }

    // TODO REVISIT THIS    LLVMStructCreateNamed
    pub fn create_struct_named(&self, name: &str) -> Struct {
        Struct::wrap(unsafe { llvm::LLVMStructCreateNamed(self.0, str_to_cstr!(name)) })
    }

    // TODO REVISIT THIS
    pub fn metadata_as_value(&self, metadata: md::ActualMetadata) -> md::MetadataAsValue {
        md::MetadataAsValue::wrap(unsafe { llvm::LLVMMetadataAsValue(self.0, expose!(metadata)) })
    }
}













