
use libc::c_void;

use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;
use crate::diagnostics as dx;
use super::types::{Type, type_from_ref};
use super::{builder as br, values as val, metadata as md, block as bb, module as ml};


wrapper!(Context, LLVMContextRef);

// TODO WARNING DEBUG REVISIT THESE--- MAY NOT WANT TO DROP WITH WRAPPER?
// NEW WRAPPERS SOMETIMES ARE CREATED FOR A NEW REF, AND YOU WOULDN'T WANT TO DROP IT WHEN THEY DO
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
    pub fn get_diagnostic_context(&self) -> dx::DiagnosticContext {
        dx::DiagnosticContext::wrap(unsafe { llvm::LLVMContextGetDiagnosticContext(self.0) })
    }

    // TODO Docs
    pub fn get_diagnostic_handler(&self) -> dx::DiagnosticHandler {
        dx::DiagnosticHandler::wrap(unsafe { llvm::LLVMContextGetDiagnosticHandler(self.0) })
    }

    // TODO Docs
    pub fn set_diagnostic_handler(&self, handler: dx::DiagnosticHandler, diag_context: dx::DiagnosticContext) {
        unsafe { llvm::LLVMContextSetDiagnosticHandler(self.0, expose!(handler), expose!(diag_context)) }
    }

    // TODO Docs, What does this do?
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
    pub fn create_enum_attribute(&self, kind_id: u32, val: u64) -> md::Attribute {
        md::Attribute::wrap(unsafe { llvm::LLVMCreateEnumAttribute(self.0, kind_id, val) })
    }

    // TODO Docs
    pub fn create_string_attribute(&self, k: &str, v: &str) -> md::Attribute {
        md::Attribute::wrap(unsafe {
            llvm::LLVMCreateStringAttribute(self.0, str_to_cstr!(k), size!(k), str_to_cstr!(v), size!(v))
        })
    }

    // TODO Docs
    pub fn create_type_attribute<T: Type>(&self, kind_id: u32, typ: T) -> md::Attribute {
        md::Attribute::wrap(unsafe { llvm::LLVMCreateTypeAttribute(self.0, kind_id, expose!(typ)) })
    }

    // TODO Docs
    pub fn create_builder(&self) -> br::Builder {
        br::Builder::wrap(
            unsafe { llvm::LLVMCreateBuilderInContext(self.0) }
        )
    }

    // TODO Docs
    pub fn create_block(&self, name: &str) -> bb::Block {
        let b = unsafe { llvm::LLVMCreateBasicBlockInContext(self.0, str_to_cstr!(name)) };
        bb::Block::wrap(b)
    }

    // TODO Docs
    pub fn append_block(&self, function: val::Function, name: &str) -> bb::Block {
        let b = unsafe { llvm::LLVMAppendBasicBlockInContext(self.0, expose!(function), str_to_cstr!(name)) };
        bb::Block::wrap(b)
    }

    // TODO Docs
    pub fn get_type_by_name(&self, name: &str) -> Raw {
        Raw::wrap(unsafe { llvm::LLVMGetTypeByName2(self.0, str_to_cstr!(name)) })
    }

    // TODO Docs
    pub fn get_intrinsic_type(&self, id: u32, params: Vec<Box<dyn Type>>) -> Box<dyn Type> {
        let intrinsic = unsafe {
            llvm::LLVMIntrinsicGetType(self.0, id, expose_array!(params), params.len())
        };
        type_from_ref(intrinsic)
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

    // TODO REVISIT THIS, Move to metadata
    pub fn metadata_as_value(&self, metadata: md::ActualMetadata) -> md::MetadataAsValue {
        md::MetadataAsValue::wrap(unsafe { llvm::LLVMMetadataAsValue(self.0, expose!(metadata)) })
    }

   // TODO Docs, LLVMModuleCreateWithNameInContext⚠ // context
    pub fn create_module(&self, name: &str) -> ml::Module {
        unsafe { ml::Module::wrap(llvm::LLVMModuleCreateWithNameInContext(str_to_cstr!(name), self.0)) }
    }

}













