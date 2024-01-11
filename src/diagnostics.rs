
use llvm_sys::prelude::*;
use llvm_sys::LLVMDiagnosticHandler;

use super::wrapper::Wrapper;


// Diagnostics
wrapper!(DiagnosticHandler, LLVMDiagnosticHandler);
wrapper!(Diagnostics, LLVMDiagnosticInfoRef);
