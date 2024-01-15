
use llvm_sys::prelude::*;
use llvm_sys::LLVMDiagnosticHandler;

use crate::wrapper::Wrapper;


// Diagnostics
wrapper!(DiagnosticHandler, LLVMDiagnosticHandler);
wrapper!(Diagnostics, LLVMDiagnosticInfoRef);

type LLVMDiagnosticContext = *mut std::os::raw::c_void; // TODO Refactor to "LlvmOpaque"
wrapper!(DiagnosticContext, LLVMDiagnosticContext);
