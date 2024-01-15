
use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;
use crate::exec::pass as pm;
use crate::debug::metadata as md;

use super::context::Context;
use super::{values as val, types as typ};


// TODO Docs
wrapper!(Module, LLVMModuleRef);
impl Module {

    // TODO Docs
    pub fn new(name: &str) -> Self {
        unsafe { Self(llvm::LLVMModuleCreateWithName(str_to_cstr!(name))) }
    }

    // TODO Docs
    pub fn clone_module(&self) -> Self {
        unsafe { Self(llvm::LLVMCloneModule(self.0)) }
    }

    // TODO Docs
    pub fn link(&self, other: Self) -> Result<(), anyhow::Error> {
        let link = unsafe { llvm_sys::linker::LLVMLinkModules2(self.0, expose!(other)) };
        if link == 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Unable to link modules")) }
    }

    // TODO Docs
    pub fn context(&self) -> Context {
        unsafe { Context::wrap(llvm::LLVMGetModuleContext(self.0)) }
    }

    // TODO Docs
    pub fn set_identifier(&self, name: &str) {
        unsafe { llvm::LLVMSetModuleIdentifier(self.0, str_to_cstr!(name), name.len()) }
    }

    // TODO Docs
    pub fn set_inline_asm(&self, name: &str) {
        unsafe { llvm::LLVMSetModuleInlineAsm2(self.0, str_to_cstr!(name), name.len()) }
    }

    // TODO Docs
    pub fn append_inline_asm(&self, name: &str) {
        unsafe { llvm::LLVMAppendModuleInlineAsm(self.0, str_to_cstr!(name), name.len()) }
    }

    // TODO Docs
    //// TODO
    //pub fn print_to_file(&self, filename: &str) -> Result<(), anyhow::Error> {
    //    use std::os::raw::c_char;
    //    let err_msg = todo![];
    //    let print = unsafe { llvm::LLVMPrintModuleToFile(self.0, str_to_cstr!(filename), err_msg) };
    //    match print {
    //        0 => Err(anyhow::anyhow!("")),
    //        _ => Ok(())
    //    }
    //}

    // TODO Docs
    pub fn print_to_string(&self) -> String {
        cstr_to_str!(llvm::LLVMPrintModuleToString(self.0)).to_string()
    }

    // TODO Docs
    pub fn new_provider(&self) -> ModuleProvider {
        unsafe { ModuleProvider(llvm::LLVMCreateModuleProviderForExistingModule(self.0)) }
    }

    // TODO Docs
    pub fn dispose(self) {
        unsafe { llvm::LLVMDisposeModule(self.0) }
    }

    // TODO Docs
    pub fn dump(self) {
        unsafe { llvm::LLVMDumpModule(self.0) }
    }

    // TODO Docs
    pub fn name(&self) -> String {
        let ref mut len = 0usize;
        cstr_to_str!(llvm::LLVMGetModuleIdentifier(self.0, len)).to_string()
    }

    // TODO Docs
    pub fn get_inline_asm(&self) -> String {
        let ref mut len = 0usize;
        cstr_to_str!(llvm::LLVMGetModuleInlineAsm(self.0, len)).to_string()
    }

    // TODO Docs
    pub fn get_data_layout(&self) -> crate::support::target::TargetData {
        unsafe { crate::support::target::TargetData::wrap(
            llvm_sys::target::LLVMGetModuleDataLayout(self.0)
        )}
    }

    // TODO Docs
    pub fn set_data_layout(&self, data: crate::support::target::TargetData) {
        unsafe { llvm_sys::target::LLVMSetModuleDataLayout(self.0, expose!(data)) }
    }

    // TODO Docs
    pub fn get_flag(&self, key: &str) -> md::ActualMetadata {
        unsafe { md::ActualMetadata::wrap(
            llvm::LLVMGetModuleFlag(self.0, str_to_cstr!(key), key.len())
        )}
    }

    // TODO Docs
    pub fn add_flag(&self, behavior: FlagBehavior, key: &str, metadata: md::ActualMetadata) {
        unsafe { llvm::LLVMAddModuleFlag(self.0, behavior.to_llvm(), str_to_cstr!(key), key.len(), expose!(metadata)) }
    }

    // TODO Docs
    pub fn copy_flags(&self) -> Vec<ModuleFlag> {
        let ref mut len: usize = 0;
        let flags = unsafe { std::slice::from_raw_parts(llvm::LLVMCopyModuleFlagsMetadata(self.0, len), *len) };
        flags.iter().map(|o| ModuleFlag(*o)).collect()
    }

    // TODO Docs
    pub fn get_metadata_node(&self, name: &str) -> md::MetadataNode {
        unsafe { md::MetadataNode::wrap(
            llvm::LLVMGetNamedMetadata(self.0, str_to_cstr!(name), name.len())
        )}
    }

    // TODO Docs
    pub fn get_metadata_operands(&self, name: &str) -> Vec<Box<dyn val::Value>> {
        let ref mut operands: Vec<LLVMValueRef> = vec![];
        unsafe { llvm::LLVMGetNamedMetadataOperands(self.0, str_to_cstr!(name), operands.as_mut_ptr()) }
        operands.iter().map(|o| val::value_from_ref(*o)).collect()
    }

    // TODO Docs
    pub fn get_metadata_num_operands(&self, name: &str) -> u32 {
        unsafe { llvm::LLVMGetNamedMetadataNumOperands(self.0, str_to_cstr!(name)) }
    }

    // TODO Docs
    pub fn add_metadata_operand(&self, name: &str, val: Box<dyn val::Value>) {
        unsafe { llvm::LLVMAddNamedMetadataOperand(self.0, str_to_cstr!(name), expose!(val)) }
    }

    // TODO Docs
    pub fn intrinsic_copy_overloaded_name(&self, id: u32, params: Vec<Box<dyn typ::Type>>) -> String {
        let ref mut name_len: usize = 0;
        cstr_to_str!(
            llvm::LLVMIntrinsicCopyOverloadedName2(self.0, id, expose_array!(params), params.len(), name_len)
        ).to_string()
    }

    // TODO Docs
    pub fn create_function_pass_manager(&self) -> pm::PassManager {
        unsafe { pm::PassManager::wrap(llvm::LLVMCreateFunctionPassManagerForModule(self.0)) }
    }

    // TODO Docs
    pub fn run_pass_manager(&self, pass_manager: pm::PassManager) -> Result<(), anyhow::Error> {
        let run = unsafe { llvm::LLVMRunPassManager(expose!(pass_manager), self.0) };
        if run > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Function was not run!")) }
    }

    // TODO Docs
    pub fn add_alias(&self, typ: Box<dyn typ::Type>, aliasee: Box<dyn val::Value>, name: &str) -> val::GlobalAlias {
        unsafe { val::GlobalAlias::wrap(
            llvm::LLVMAddAlias(self.0, expose!(typ), expose!(aliasee), str_to_cstr!(name))
        )}
    }

    // TODO Docs
    pub fn add_function(&self, function: typ::Function, name: &str) -> val::Function {
        unsafe { val::Function::wrap(
            llvm::LLVMAddFunction(self.0, str_to_cstr!(name), expose!(function))
        )}
    }

    // TODO Docs
    pub fn add_global(&self, typ: Box<dyn typ::Type>, name: &str) -> val::GlobalVariable {
        unsafe { val::GlobalVariable::wrap(
            llvm::LLVMAddGlobal(self.0, expose!(typ), str_to_cstr!(name))
        )}
    }

    // TODO Docs
    pub fn add_global_in_address(&self, typ: Box<dyn typ::Type>, name: &str, addr: u32) -> val::GlobalVariable {
        unsafe { val::GlobalVariable::wrap(
            llvm::LLVMAddGlobalInAddressSpace(self.0, expose!(typ), str_to_cstr!(name), addr)
        )}
    }

    // TODO Docs
    pub fn add_global_indirect_function(&self, name: &str, function: typ::Function, addr: u32, resolver: val::Function) -> val::GlobalIFunc {
        unsafe { val::GlobalIFunc::wrap(
            llvm::LLVMAddGlobalIFunc(self.0, str_to_cstr!(name), name.len(), expose!(function), addr, expose!(resolver))
        )}
    }
}


// TODO Docs
wrapper!(ModuleProvider, LLVMModuleProviderRef);
impl ModuleProvider {
    // TODO Docs
    pub fn dispose(self) {
        unsafe { llvm::LLVMDisposeModuleProvider(self.0) }
    }

    // TODO Docs
    pub fn create_function_pass_manager(&self) -> pm::PassManager {
        unsafe { pm::PassManager::wrap(llvm::LLVMCreateFunctionPassManager(self.0)) }
    }
}


// TODO Docs
wrapper!(ModuleFlag, LLVMModuleFlagEntry);
impl ModuleFlag {

    // TODO Docs
    pub fn dispose(self) {
        Self::dispose_flags(vec![self])
    }

    // TODO Docs
    pub fn behavior(&self) -> FlagBehavior {
        Self::get_flag_behavior(vec![self.to_owned()], 0)
    }

    // TODO Docs
    pub fn key(&self) -> String {
        Self::get_flag_key(vec![self.to_owned()], 0)
    }

    // TODO Docs
    pub fn metadata(&self) -> md::ActualMetadata {
        Self::get_flag_metadata(vec![self.to_owned()], 0)
    }

    // TODO Docs
    pub fn dispose_flags(flags: Vec<Self>) {
        unsafe { llvm::LLVMDisposeModuleFlagsMetadata(expose_array!(flags)) }
    }

    // TODO Docs
    pub fn get_flag_behavior(flags: Vec<Self>, index: u32) -> FlagBehavior {
        unsafe { FlagBehavior::from_llvm(
            llvm::LLVMModuleFlagEntriesGetFlagBehavior(expose_array!(flags), index)
        )}
    }

    // TODO Docs
    pub fn get_flag_key(flags: Vec<Self>, index: u32) -> String {
        let ref mut len: usize = 0;
        cstr_to_str!(llvm::LLVMModuleFlagEntriesGetKey(expose_array!(flags), index, len)).to_string()
    }

    // TODO Docs
    pub fn get_flag_metadata(flags: Vec<Self>, index: u32) -> md::ActualMetadata {
        unsafe { md::ActualMetadata::wrap(
            llvm::LLVMModuleFlagEntriesGetMetadata(expose_array!(flags), index)
        )}
    }
}

/// More legible representation of LLVMModuleFlagBehavior
pub enum FlagBehavior {
    Error,
    Warning,
    Require,
    Override,
    Append,
    AppendUnique,
}

use llvm_sys::LLVMModuleFlagBehavior::{self, *};
impl FlagBehavior {
    pub fn from_llvm(llvm: LLVMModuleFlagBehavior) -> FlagBehavior {
        match llvm {
            LLVMModuleFlagBehaviorError          => FlagBehavior::Error,
            LLVMModuleFlagBehaviorWarning        => FlagBehavior::Warning,
            LLVMModuleFlagBehaviorRequire        => FlagBehavior::Require,
            LLVMModuleFlagBehaviorOverride       => FlagBehavior::Override,
            LLVMModuleFlagBehaviorAppend         => FlagBehavior::Append,
            LLVMModuleFlagBehaviorAppendUnique   => FlagBehavior::AppendUnique,
        }
    }
    pub fn to_llvm(&self) -> LLVMModuleFlagBehavior {
        match self {
            FlagBehavior::Error          => LLVMModuleFlagBehaviorError,
            FlagBehavior::Warning        => LLVMModuleFlagBehaviorWarning,
            FlagBehavior::Require        => LLVMModuleFlagBehaviorRequire,
            FlagBehavior::Override       => LLVMModuleFlagBehaviorOverride,
            FlagBehavior::Append         => LLVMModuleFlagBehaviorAppend,
            FlagBehavior::AppendUnique   => LLVMModuleFlagBehaviorAppendUnique,
        }
    }
}









