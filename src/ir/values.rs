
use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;

use super::types::{Type, type_from_ref};
use super::metadata as md;

use super::block as bb;


pub trait Value: Wrapper<Llvm = LLVMValueRef> {

    // TODO Docs
    fn type_of(&self) -> Box<dyn Type> {
        type_from_ref(unsafe { llvm::LLVMTypeOf(expose!(self)) })
    }

    // TODO Docs
    fn as_basic_block(&self) -> bb::Block {
        bb::Block::wrap(unsafe { llvm::LLVMValueAsBasicBlock(expose!(self)) })
    }

    // TODO Docs
    fn as_metadata(&self) -> md::ActualMetadata {
        md::ActualMetadata::wrap(unsafe { llvm::LLVMValueAsMetadata(expose!(self)) })
    }

    // TODO Docs
    fn is_basic_block(&self) -> bool {
        bool_to_rust!( llvm::LLVMValueIsBasicBlock(expose!(self)) )
    }

    // TODO Docs
    fn name(&self) -> String {
        let ref mut len: usize = 0;
        cstr_to_str!(llvm::LLVMGetValueName2(expose!(self), len)).to_string()
    }

    // TODO Docs
    fn rename(&self, name: &str) {
        unsafe { llvm::LLVMSetValueName2(expose!(self), str_to_cstr!(name), name.len()) }
    }

    // TODO Docs
    fn kind(&self) -> LLVMValueKind {
        unsafe { llvm::LLVMGetValueKind(expose!(self)) }
    }

    // TODO Docs
    fn to_string(&self) -> String {
        cstr_to_str!(llvm::LLVMPrintValueToString(expose!(self))).to_string()
    }

    /// Prints a textual representation of the type to the error stream
    fn dump(&self) {
        unsafe { llvm::LLVMDumpValue(expose!(self)) }
    }

    // TODO Docs
    fn num_blocks(&self) -> u32 {
        unsafe { llvm::LLVMCountBasicBlocks(expose!(self)) }
    }

    // TODO Docs
    fn get_alignment(&self) -> u32 {
        unsafe { llvm::LLVMGetAlignment(expose!(self)) }
    }

    // TODO Docs
    fn get_allocated_type(&self) -> Box<dyn Type> {
        type_from_ref(unsafe { llvm::LLVMGetAllocatedType(expose!(self)) })
    }

    // TODO Docs
    fn get_as_string(&self) -> String {
        let ref mut len: usize = 0;
        cstr_to_str!(llvm::LLVMGetAsString(expose!(self), len)).to_string()
    }
}

    // specific value kind
    //fn get_arg_operand(&self, i: u32) -> Self { // TODO Make note of how these references work, no borrow-checking technically
    //    Self(unsafe { llvm::LLVMGetArgOperand(expose!(self), i) })
    //}

/*
    LLVMGetAtomicRMWBinOp⚠
    LLVMGetAttributeCountAtIndex⚠ // passes ref to empty slice to populate
    LLVMGetAttributesAtIndex⚠     // these can return vec of attrs
*/

macro_rules! llvm_value {
    ($val_name:ident) => {
        wrapper!($val_name, LLVMValueRef);
        impl Value for $val_name {}
    };
}

llvm_value!(Argument);
llvm_value!(BasicBlock);
llvm_value!(MemoryUse);
llvm_value!(MemoryDef);
llvm_value!(MemoryPhi);
llvm_value!(Function);
llvm_value!(GlobalAlias);
llvm_value!(GlobalIFunc);
llvm_value!(GlobalVariable);
llvm_value!(BlockAddress);
llvm_value!(ConstantExpr);
llvm_value!(ConstantArray);
llvm_value!(ConstantStruct);
llvm_value!(ConstantVector);
llvm_value!(UndefValue);
llvm_value!(ConstantAggregateZero);
llvm_value!(ConstantDataArray);
llvm_value!(ConstantDataVector);
llvm_value!(ConstantInt);
llvm_value!(ConstantFP);
llvm_value!(ConstantPointerNull);
llvm_value!(ConstantTokenNone);
llvm_value!(MetadataAsValue);
llvm_value!(InlineAsm);
llvm_value!(Instruction);
llvm_value!(Poison);

/// Wraps a naked LLVMValueRef with the corresponding safe type & stores it on the heap
pub fn value_from_ref(val_ref: LLVMValueRef) -> Box<dyn Value> {
    unsafe { match llvm::LLVMGetValueKind(val_ref) {
        LLVMValueKind::LLVMArgumentValueKind               => Box::new(Argument(val_ref)),
        LLVMValueKind::LLVMBasicBlockValueKind             => Box::new(BasicBlock(val_ref)),
        LLVMValueKind::LLVMMemoryUseValueKind              => Box::new(MemoryUse(val_ref)),
        LLVMValueKind::LLVMMemoryDefValueKind              => Box::new(MemoryDef(val_ref)),
        LLVMValueKind::LLVMMemoryPhiValueKind              => Box::new(MemoryPhi(val_ref)),
        LLVMValueKind::LLVMFunctionValueKind               => Box::new(Function(val_ref)),
        LLVMValueKind::LLVMGlobalAliasValueKind            => Box::new(GlobalAlias(val_ref)),
        LLVMValueKind::LLVMGlobalIFuncValueKind            => Box::new(GlobalIFunc(val_ref)),
        LLVMValueKind::LLVMGlobalVariableValueKind         => Box::new(GlobalVariable(val_ref)),
        LLVMValueKind::LLVMBlockAddressValueKind           => Box::new(BlockAddress(val_ref)),
        LLVMValueKind::LLVMConstantExprValueKind           => Box::new(ConstantExpr(val_ref)),
        LLVMValueKind::LLVMConstantArrayValueKind          => Box::new(ConstantArray(val_ref)),
        LLVMValueKind::LLVMConstantStructValueKind         => Box::new(ConstantStruct(val_ref)),
        LLVMValueKind::LLVMConstantVectorValueKind         => Box::new(ConstantVector(val_ref)),
        LLVMValueKind::LLVMUndefValueValueKind             => Box::new(UndefValue(val_ref)),
        LLVMValueKind::LLVMConstantAggregateZeroValueKind  => Box::new(ConstantAggregateZero(val_ref)),
        LLVMValueKind::LLVMConstantDataArrayValueKind      => Box::new(ConstantDataArray(val_ref)),
        LLVMValueKind::LLVMConstantDataVectorValueKind     => Box::new(ConstantDataVector(val_ref)),
        LLVMValueKind::LLVMConstantIntValueKind            => Box::new(ConstantInt(val_ref)),
        LLVMValueKind::LLVMConstantFPValueKind             => Box::new(ConstantFP(val_ref)),
        LLVMValueKind::LLVMConstantPointerNullValueKind    => Box::new(ConstantPointerNull(val_ref)),
        LLVMValueKind::LLVMConstantTokenNoneValueKind      => Box::new(ConstantTokenNone(val_ref)),
        LLVMValueKind::LLVMMetadataAsValueValueKind        => Box::new(MetadataAsValue(val_ref)),
        LLVMValueKind::LLVMInlineAsmValueKind              => Box::new(InlineAsm(val_ref)),
        LLVMValueKind::LLVMInstructionValueKind            => Box::new(Instruction(val_ref)),
        LLVMValueKind::LLVMPoisonValueKind                 => Box::new(Poison(val_ref)),
    }}
}

/// WARNING Unchecked cast to Value that does not use the heap
pub fn ref_to_value<V: Wrapper<Llvm = LLVMValueRef>>(val_ref: LLVMValueRef) -> V {
    V::wrap(val_ref)
}

// constructors

// Pointers

// Null

// atomics

// strings

// resize

// covert

// cast

// phi

// vecs/arrays/aggs

// gep

// math n binary sfu

// Logic

// functions
/*
    LLVMDeleteFunction⚠
    LLVMCountParams⚠
    LLVMEraseGlobalIFunc⚠
    LLVMGetBasicBlocks⚠ // function

*/





/*
    LLVMBlockAddress⚠ // value
*/


/*
Binary ops:

    LLVMConstAShr⚠
    LLVMConstAdd⚠
    LLVMConstAnd⚠
    LLVMConstExactSDiv⚠
    LLVMConstExactUDiv⚠

    LLVMConstFAdd⚠
    LLVMConstFCmp⚠
    LLVMConstFDiv⚠
    LLVMConstFMul⚠
    LLVMConstFNeg⚠
*/

/*
Data inits & misc
    LLVMConstAddrSpaceCast⚠
    LLVMConstAllOnes⚠

    LLVMConstBitCast⚠

    LLVMConstFPCast⚠
    LLVMConstFPExt⚠
    LLVMConstFPToSI⚠
    LLVMConstFPToUI⚠
    LLVMConstFPTrunc⚠
*/

/*
Const constructors
    LLVMConstArray⚠

    vector only?
    LLVMConstExtractElement⚠
    LLVMConstExtractValue⚠

    LLVMConstFRem⚠
    LLVMConstFSub⚠
    LLVMConstGEP⚠
    LLVMConstGEP2⚠
    LLVMConstICmp⚠
    LLVMConstInBoundsGEP⚠
    LLVMConstInBoundsGEP2⚠
    LLVMConstInlineAsm⚠Deprecated
    LLVMConstInsertElement⚠
    LLVMConstInsertValue⚠
    LLVMConstInt⚠
    LLVMConstIntCast⚠
    LLVMConstIntGetSExtValue⚠
    LLVMConstIntGetZExtValue⚠
    LLVMConstIntOfArbitraryPrecision⚠
    LLVMConstIntOfString⚠
    LLVMConstIntOfStringAndSize⚠
    LLVMConstIntToPtr⚠
    LLVMConstLShr⚠
    LLVMConstMul⚠
    LLVMConstNSWAdd⚠
    LLVMConstNSWMul⚠
    LLVMConstNSWNeg⚠
    LLVMConstNSWSub⚠
    LLVMConstNUWAdd⚠
    LLVMConstNUWMul⚠
    LLVMConstNUWNeg⚠
    LLVMConstNUWSub⚠
    LLVMConstNamedStruct⚠
    LLVMConstNeg⚠
    LLVMConstNot⚠
    LLVMConstNull⚠
    LLVMConstOr⚠
    LLVMConstPointerCast⚠
    LLVMConstPointerNull⚠
    LLVMConstPtrToInt⚠
    LLVMConstReal⚠
    LLVMConstRealGetDouble⚠
    LLVMConstRealOfString⚠
    LLVMConstRealOfStringAndSize⚠
    LLVMConstSDiv⚠
    LLVMConstSExt⚠
    LLVMConstSExtOrBitCast⚠
    LLVMConstSIToFP⚠
    LLVMConstSRem⚠
    LLVMConstSelect⚠
    LLVMConstShl⚠
    LLVMConstShuffleVector⚠
    LLVMConstString⚠
    LLVMConstStringInContext⚠
    LLVMConstStruct⚠
    LLVMConstStructInContext⚠
    LLVMConstSub⚠
    LLVMConstTrunc⚠
    LLVMConstTruncOrBitCast⚠
    LLVMConstUDiv⚠
    LLVMConstUIToFP⚠
    LLVMConstURem⚠
    LLVMConstVector⚠
    LLVMConstXor⚠
    LLVMConstZExt⚠
    LLVMConstZExtOrBitCast⚠

    LLVMGlobalClearMetadata⚠
    LLVMGlobalCopyAllMetadata⚠
    LLVMGlobalEraseMetadata⚠
    LLVMGlobalGetValueType⚠
    LLVMGlobalSetMetadata⚠
    LLVMHasMetadata⚠

    LLVMValueMetadataEntriesGetKind⚠
    LLVMValueMetadataEntriesGetMetadata⚠
    LLVMDisposeValueMetadataEntries⚠
*/











