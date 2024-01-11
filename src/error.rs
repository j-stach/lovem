
use llvm_sys::{error as err, error_handling as eh};

use super::wrapper::Wrapper;

///! Note:
///! "LLVMErrorRef" is an alias for "*mut LLVMOpaqueError"
///! "LLVMErrorTypeId" is an alias for "*const c_void"

/// TODO Docs
pub const SUCCESS: libc::c_int = err::LLVMErrorSuccess; // Just zero with extra steps


wrapper!(Error, err::LLVMErrorRef);
wrapper!(ErrorType, err::LLVMErrorTypeId);

// TODO Revaluate, may be a bug
impl Drop for Error {
    fn drop(&mut self) {
        unsafe { err::LLVMConsumeError(self.0) }
    }
}

impl Error {
    // TODO Docs
    pub fn consume(self) {
        unsafe { err::LLVMConsumeError(self.0) };
        drop(self)
    }

    // TODO Docs
    pub fn create_string_error(message: &str) -> Self {
        Self(unsafe { err::LLVMCreateStringError(str_to_cstr!(message)) })
    }

    // TODO Docs: Removes msg from error
    pub fn dispose_message(&mut self) {
        unsafe {
            let msg = err::LLVMGetErrorMessage(self.0);
            err::LLVMDisposeErrorMessage(msg)
        }
    }

    // TODO Docs
    pub fn message(&mut self) -> &str {
        cstr_to_str!(err::LLVMGetErrorMessage(self.0))
    }

    // TODO Docs, plus, does this need a Rusty return type?
    pub fn type_id(&self) -> ErrorType {
        ErrorType::wrap(unsafe { err::LLVMGetErrorTypeId(self.0) })
    }

    // TODO Why is this useful exactly?
    pub fn get_string_error_type_id() -> ErrorType {
        ErrorType::wrap(unsafe { err::LLVMGetStringErrorTypeId() })
    }
}

///! Note:
///! "LLVMFatalErrorHandler" is a type alias for Option<extern "C" fn(Reason: *const c_char)>"

/// TODO Docs
pub struct FatalErrorHandler(eh::LLVMFatalErrorHandler);

// TODO Revisit this, how can i create a new one safely? Would i ever need to do that?
impl FatalErrorHandler {
    // TODO Docs
    pub fn enable_pretty_stack_trace() {
        unsafe { eh::LLVMEnablePrettyStackTrace() }
    }

    // TODO Docs
    pub fn install(self) {
        unsafe { eh::LLVMInstallFatalErrorHandler(self.0) }
        drop(self)
    }

    // TODO Docs
    pub fn reset() {
        unsafe { eh::LLVMResetFatalErrorHandler() }
    }
}





