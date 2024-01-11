
use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;


// TODO Docs
wrapper!(MemoryBuffer, LLVMMemoryBufferRef);
impl MemoryBuffer {
    // TODO Docs
    pub fn new_from_file(filepath: &str) -> Result<Self, anyhow::Error> {
        let buf = std::ptr::null_mut();
        let msg = std::ptr::null_mut();
        let create = unsafe {
            llvm::LLVMCreateMemoryBufferWithContentsOfFile(str_to_cstr!(filepath), buf, msg) };
        if create > 0 { Ok(Self::wrap( unsafe { *buf })) }
        else { Err(anyhow::anyhow!("{}", cstr_to_str!(*msg))) }
    }

    // TODO Docs
    pub fn new_with_range(input: &str, name: &str, null_term: bool) -> Self {
        let buffer = unsafe {
            llvm::LLVMCreateMemoryBufferWithMemoryRange(
                str_to_cstr!(input),
                input.len(),
                str_to_cstr!(name),
                bool_to_llvm!(null_term))
        };
        Self(buffer)
    }

    // TODO Docs
    pub fn copy_with_range(input: &str, name: &str) -> Self {
        let buffer = unsafe {
            llvm::LLVMCreateMemoryBufferWithMemoryRangeCopy(str_to_cstr!(input), input.len(), str_to_cstr!(name))
        };
        Self(buffer)
    }

    // TODO Docs
    pub fn new_from_stdin() -> Result<Self, anyhow::Error> {
        let buf = std::ptr::null_mut();
        let msg = std::ptr::null_mut();
        let create = unsafe { llvm::LLVMCreateMemoryBufferWithSTDIN(buf, msg) };
        if create > 0 { Ok(Self::wrap( unsafe { *buf })) }
        else { Err(anyhow::anyhow!("{}", cstr_to_str!(*msg))) }
    }

    // TODO Docs
    pub fn dispose(self) {
        unsafe { llvm::LLVMDisposeMemoryBuffer(self.0) }
    }

    // TODO Docs
    pub fn start(&self) -> String { // TODO Revisit this, correct type?
        cstr_to_str!( llvm::LLVMGetBufferStart(self.0) ).to_string()
    }

    // TODO Docs
    pub fn size(&self) -> usize { // TODO Revisit this, correct type?
        unsafe { llvm::LLVMGetBufferSize(self.0) }
    }
}











