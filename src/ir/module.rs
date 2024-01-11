
use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;
use super::context::Context;

use super::metadata as md;


wrapper!(Module, LLVMModuleRef);

impl Clone for Module {
    fn clone(&self) -> Self {
        Self(unsafe { llvm::LLVMCloneModule(self.0) })
    }
}

impl Module {
    pub fn new(name: &str) -> Self {
        unsafe { Self(llvm::LLVMModuleCreateWithName(str_to_cstr!(name))) }
    }

    pub fn context(&self) -> Context {
        Context::wrap(unsafe { llvm::LLVMGetModuleContext(self.0) })
    }

    pub fn set_identifier(&self, name: &str) {
        unsafe { llvm::LLVMSetModuleIdentifier(self.0, str_to_cstr!(name), name.len()) }
    }

    pub fn set_inline_asm(&self, name: &str) {
        unsafe { llvm::LLVMSetModuleInlineAsm2(self.0, str_to_cstr!(name), name.len()) }
    }

    pub fn append_inline_asm(&self, name: &str) {
        unsafe { llvm::LLVMAppendModuleInlineAsm(self.0, str_to_cstr!(name), name.len()) }
    }

    //pub fn print_to_file(&self, filename: &str) -> Result<(), anyhow::Error> {
    //    use std::os::raw::c_char;
    //    let err_msg = todo![];
    //    let print = unsafe { llvm::LLVMPrintModuleToFile(self.0, str_to_cstr!(filename), err_msg) };
    //    match print {
    //        0 => Err(anyhow::anyhow!("")),
    //        _ => Ok(())
    //    }
    //}

    pub fn print_to_string(&self) -> String {
        cstr_to_str!(llvm::LLVMPrintModuleToString(self.0)).to_string()
    }

    pub fn new_provider(&self) -> ModuleProvider {
        ModuleProvider(unsafe { llvm::LLVMCreateModuleProviderForExistingModule(self.0) })
    }

    pub fn dispose(self) {
        unsafe { llvm::LLVMDisposeModule(self.0) }
    }

    pub fn dump(self) {
        unsafe { llvm::LLVMDumpModule(self.0) }
    }

    pub fn name(&self) -> String {
        let ref mut len = 0usize;
        cstr_to_str!(llvm::LLVMGetModuleIdentifier(self.0, len)).to_string()
    }

    pub fn inline_asm(&self) -> String {
        let ref mut len = 0usize;
        cstr_to_str!(llvm::LLVMGetModuleInlineAsm(self.0, len)).to_string()
    }

    pub fn get_module_flag(&self, key: &str) -> md::ActualMetadata {
        md::ActualMetadata::wrap(unsafe {
            llvm::LLVMGetModuleFlag(self.0, str_to_cstr!(key), key.len())
        })
    }

    pub fn add_module_flag(&self, behavior: llvm_sys::LLVMModuleFlagBehavior, key: &str, metadata: md::ActualMetadata) {
        unsafe { llvm::LLVMAddModuleFlag(self.0, behavior, str_to_cstr!(key), key.len(), expose!(metadata)) }
    }
}

wrapper!(ModuleProvider, LLVMModuleProviderRef);
impl ModuleProvider {
    pub fn dispose(self) {
        unsafe { llvm::LLVMDisposeModuleProvider(self.0) }
    }
}

/*


    LLVMAddAlias⚠
    LLVMAddFunction⚠
    LLVMAddGlobal⚠
    LLVMAddGlobalIFunc⚠
    LLVMAddGlobalInAddressSpace⚠
    LLVMAddModuleFlag⚠
    LLVMAddNamedMetadataOperand⚠

/*
    LLVMGetNamedMetadata⚠
    LLVMGetNamedMetadataName⚠
    LLVMGetNamedMetadataNumOperands⚠
    LLVMGetNamedMetadataOperands⚠

    LLVMIntrinsicCopyOverloadedName2⚠ // module

    LLVMCreateFunctionPassManager⚠
    LLVMCreateFunctionPassManagerForModule⚠
    LLVMCreatePassManager⚠
    LLVMDisposePassManager⚠
    LLVMFinalizeFunctionPassManager⚠
    LLVMInitializeFunctionPassManager⚠
    LLVMRunFunctionPassManager⚠
    LLVMRunPassManager⚠
*/

*/

/*

    LLVMModuleCreateWithNameInContext⚠ // context

    LLVMModuleFlagEntriesGetFlagBehavior⚠
    LLVMModuleFlagEntriesGetKey⚠
    LLVMDisposeModuleFlagsMetadata⚠
    LLVMCopyModuleFlagsMetadata⚠
*/
