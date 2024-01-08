
use std::ffi::CString;

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
    pub fn new() -> Self {
        unsafe { Context(llvm::LLVMContextCreate()) }
    }
    pub fn global() -> Self {
        unsafe { Context(llvm::LLVMGetGlobalContext()) }
    }
}

/*
    LLVMContextCreate⚠
    LLVMContextGetDiagnosticContext⚠
    LLVMContextGetDiagnosticHandler⚠
    LLVMContextSetDiagnosticHandler⚠
    LLVMContextSetDiscardValueNames⚠
    LLVMContextSetYieldCallback⚠
    LLVMContextShouldDiscardValueNames⚠
    LLVMCreateEnumAttribute⚠
    LLVMCreateStringAttribute⚠
    LLVMCreateBuilderInContext⚠

    LLVMAddAlias⚠
    LLVMAddFunction⚠
    LLVMAddGlobal⚠
    LLVMAddGlobalIFunc⚠
    LLVMAddGlobalInAddressSpace⚠
    LLVMAddModuleFlag⚠
    LLVMAddNamedMetadataOperand⚠

    LLVMTokenTypeInContext⚠

 */
