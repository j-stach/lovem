
use llvm_sys::prelude::LLVMBasicBlockRef;
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;
use super::values as val;

wrapper!(Block, LLVMBasicBlockRef);
impl Block {

    //TODO Docs
    pub fn delete(self) {
        unsafe { llvm::LLVMDeleteBasicBlock(self.0) };
        drop(self)
    }

    //TODO Docs
    pub fn append(function: val::Function, name: &str) -> Block {
        Block(unsafe { llvm::LLVMAppendBasicBlock(expose!(function), str_to_cstr!(name)) })
    }

    //TODO Docs
    pub fn append_existing(function: val::Function, block: Block) {
        unsafe { llvm::LLVMAppendExistingBasicBlock(expose!(function), expose!(block)) }
    }

    //TODO Docs
    pub fn name(&self) -> String {
        cstr_to_str!(llvm::LLVMGetBasicBlockName(self.0)).to_string()
    }

    //TODO Docs
    pub fn parent(&self) -> val::BasicBlock {
        val::BasicBlock::wrap(unsafe { llvm::LLVMGetBasicBlockParent(self.0) })
    }

    //TODO Docs
    pub fn terminator(&self) -> val::Instruction {
        val::Instruction::wrap(unsafe { llvm::LLVMGetBasicBlockTerminator(self.0) })
    }

    //TODO Docs
    pub fn as_value(&self) -> val::BasicBlock {
        val::BasicBlock::wrap(unsafe { llvm::LLVMBasicBlockAsValue(self.0) })
    }

    //TODO Docs
    pub fn remove_from_parent(&self) {
        unsafe { llvm::LLVMRemoveBasicBlockFromParent(self.0) }
    }

    //TODO Docs
    pub fn move_after(&self, other: Block) {
        unsafe { llvm::LLVMMoveBasicBlockAfter(self.0, expose!(other)) }
    }

    //TODO Docs
    pub fn move_before(&self, other: Block) {
        unsafe { llvm::LLVMMoveBasicBlockBefore(self.0, expose!(other)) }
    }
}
