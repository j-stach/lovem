
use llvm_sys::analysis as llvm;

use crate::wrapper::Wrapper;
use crate::ir::{values as val, module as ml};


/// TODO Docs
pub use llvm_sys::analysis::LLVMVerifierFailureAction;

/// TODO Docs
pub fn verify_function(function: &val::Function, action: LLVMVerifierFailureAction) -> Result<(), anyhow::Error> {
    let verify = unsafe { llvm::LLVMVerifyFunction(expose!(function), action) };
    if verify > 0 { return Ok(()) }
    else { return Err(anyhow::anyhow!("Unable to verify function")) }
}

/// TODO Docs
pub fn verify_module(module: &ml::Module, action: LLVMVerifierFailureAction) -> Result<(), anyhow::Error> {
    let ref mut msg: *mut std::ffi::c_char = std::ptr::null_mut();
    let verify = unsafe { llvm::LLVMVerifyModule(expose!(module), action, msg) };
    if verify > 0 { return Ok(()) }
    else {
        let msg = unsafe { std::ffi::CStr::from_ptr(*msg) };
        return Err(anyhow::anyhow!("Unable to verify module: {}", cstr_to_str!(msg.as_ptr())))
    }
}

/// TODO Docs
pub fn view_function_cfg(function: val::Function) {
    unsafe { llvm::LLVMViewFunctionCFG(expose!(function)) }
}

/// TODO Docs
pub fn view_function_cfg_only(function: val::Function) {
    unsafe { llvm::LLVMViewFunctionCFGOnly(expose!(function)) }
}
