
use std::ffi::CString;

use libc::c_void;

use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use derive_more::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
pub struct Context(LLVMContextRef);

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { llvm::LLVMContextDispose(self.0) }
    }
}

impl Context {
    // TODO Docs
    pub fn new() -> Self {
        unsafe { Self(llvm::LLVMContextCreate()) }
    }

    // TODO Docs
    pub fn wrap(context: LLVMContextRef) -> Self {
        Self(context)
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
    pub fn set_discard_value_names(&self, discard: LLVMBool) {
        unsafe { llvm::LLVMContextSetDiscardValueNames(self.0, discard) }
    }

    // TODO Docs
    pub fn should_discard_value_names(&self) -> LLVMBool { // TODO Make safe
        unsafe { llvm::LLVMContextShouldDiscardValueNames(self.0) }
    }

    // TODO Docs
    pub fn create_enum_attribute(&self, kind_id: u32, val: u64) -> LLVMAttributeRef {
        unsafe { llvm::LLVMCreateEnumAttribute(self.0, kind_id, val) }
    }

    // TODO Docs
    pub fn create_string_attribute(&self, k: &str, v: &str) -> LLVMAttributeRef {
        let c_k = CString::new(k).expect("Convert &str to CString");
        let c_v = CString::new(v).expect("Convert &str to CString");
        unsafe { llvm::LLVMCreateStringAttribute(self.0, c_k.as_ptr(), k.len() as u32, c_v.as_ptr(), v.len() as u32) }
    }

    // TODO Docs
    pub fn create_type_attribute(&self, kind_id: u32, typ: LLVMTypeRef) -> LLVMAttributeRef {
        unsafe { llvm::LLVMCreateTypeAttribute(self.0, kind_id, typ) }
    }

    // TODO Docs
    pub fn create_builder(&self) -> super::builder::Builder {
        super::builder::Builder::wrap(
            unsafe { llvm::LLVMCreateBuilderInContext(self.0) }
        )
    }

    // TODO Docs
    pub fn create_basic_block(&self, name: &str) -> LLVMBasicBlockRef {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe { llvm::LLVMCreateBasicBlockInContext(self.0, c_name.as_ptr()) }
    }

}





/*
    LLVMGetTypeByName2⚠

    LLVMTokenTypeInContext⚠
    LLVMFloatTypeInContext⚠
    LLVMHalfTypeInContext⚠
    LLVMPPCFP128TypeInContext⚠
    LLVMMDNodeInContext2⚠
    LLVMMDStringInContext2⚠
    LLVMInt16TypeInContext⚠
    LLVMInt8TypeInContext⚠
    LLVMInt1TypeInContext⚠
    LLVMInt64TypeInContext⚠
    LLVMInt32TypeInContext⚠
    LLVMIntTypeInContext⚠
    LLVMInt128TypeInContext⚠

    LLVMX86MMXTypeInContext⚠
    LLVMFP128TypeInContext⚠
    LLVMDoubleTypeInContext⚠
    LLVMBFloatTypeInContext⚠
    LLVMX86AMXTypeInContext⚠
    LLVMVoidTypeInContext⚠
    LLVMX86FP80TypeInContext⚠

 */
