
use std::ffi::CString;

use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use derive_more::{Deref, DerefMut};


// TODO Docs
#[derive(Deref, DerefMut)]
pub struct Value(LLVMValueRef);

impl Value {
    // TODO Docs
    pub fn as_basic_block(&self) -> LLVMBasicBlockRef {
        unsafe { llvm::LLVMValueAsBasicBlock(self.0) }
    }

    // TODO Docs
    pub fn as_metadata(&self) -> LLVMMetadataRef {
        unsafe { llvm::LLVMValueAsMetadata(self.0) }
    }

    // TODO Docs
    pub fn is_basic_block(&self) -> LLVMBool {
        unsafe { llvm::LLVMValueIsBasicBlock(self.0) }
    }

    // TODO Docs
    pub fn name(&self) -> String {
        let ref mut len: usize = 0;
        c_str_to_str!(llvm::LLVMGetValueName2(self.0, len)).to_string()
    }

    // TODO Docs
    pub fn rename(&self, name: &str) {
        let c_name = CString::new(name).expect("Convert &str to CString");
        unsafe { llvm::LLVMSetValueName2(self.0, c_name.as_ptr(), name.len()) }
    }

    // TODO Docs
    pub fn kind(&self) -> LLVMValueKind {
        unsafe { llvm::LLVMGetValueKind(self.0) }
    }

    // TODO Docs
    pub fn to_string(&self) -> String {
        c_str_to_str!(llvm::LLVMPrintValueToString(self.0)).to_string()
    }

    /// Prints a textual representation of the type to the error stream
    pub fn dump(&self) {
        unsafe { llvm::LLVMDumpValue(self.0) }
    }

    // TODO Docs
    pub fn num_blocks(&self) -> u32 {
        unsafe { llvm::LLVMCountBasicBlocks(self.0) }
    }

    // TODO Docs
    pub fn get_alignment(&self) -> u32 {
        unsafe { llvm::LLVMGetAlignment(self.0) }
    }

    // TODO Docs
    pub fn get_allocated_type(&self) -> LLVMTypeRef {
        unsafe { llvm::LLVMGetAllocatedType(self.0) }
    }

    // TODO Docs
    pub fn get_as_string(&self) -> String {
        let ref mut len: usize = 0;
        c_str_to_str!(llvm::LLVMGetAsString(self.0, len)).to_string()
    }
}



impl Value {
    // specific value kind
    pub fn get_arg_operand(&self, i: u32) -> Self { // TODO Make note of how these references work, no borrow-checking technically
        Self(unsafe { llvm::LLVMGetArgOperand(self.0, i) })
    }

/*
    LLVMGetAtomicRMWBinOp⚠
    LLVMGetAttributeCountAtIndex⚠ // passes ref to empty slice to populate
    LLVMGetAttributesAtIndex⚠     // these can return vec of attrs
*/

}

