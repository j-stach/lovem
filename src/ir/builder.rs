
use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use crate::wrapper::{Wrapper, NonWrapper};

use crate::debug::metadata as md;
use super::{types as typ, values as val, block as bb};
use self::{typ::Type, val::Value};


wrapper!(Builder, LLVMBuilderRef);

impl Drop for Builder {
    /// TODO Docs
    fn drop(&mut self) {
        unsafe { llvm::LLVMDisposeBuilder(self.0) }
    }
}

impl Builder {
    /// TODO Docs
    pub fn new() -> Self {
        unsafe { Self(llvm::LLVMCreateBuilder()) }
    }

    /// TODO Docs
    pub fn insert(&self, instruction: val::Instruction) {
        unsafe { llvm::LLVMInsertIntoBuilder(self.0, expose!(instruction)) }
    }

    /// TODO Docs
    pub fn insert_with_name(&self, instruction: val::Instruction, name: &str) {
        unsafe { llvm::LLVMInsertIntoBuilderWithName(self.0, expose!(instruction), str_to_cstr!(name)) }
    }

    /// TODO Docs
    pub fn position(&self, block: bb::Block, instruction: val::Instruction) {
        unsafe { llvm::LLVMPositionBuilder(self.0, expose!(block), expose!(instruction)) }
    }

    /// TODO Docs
    pub fn position_at_end(&self, block: bb::Block) {
        unsafe { llvm::LLVMPositionBuilderAtEnd(self.0, expose!(block)) }
    }

    /// TODO Docs
    pub fn position_before(&self, instruction: val::Instruction) {
        unsafe { llvm::LLVMPositionBuilderBefore(self.0, expose!(instruction)) }
    }

    /// TODO Docs
    pub fn clear_insertion_position(&self) {
        unsafe { llvm::LLVMClearInsertionPosition(self.0) }
    }

    /// TODO Docs
    pub fn get_default_fp_math_tag(&self) -> md::ActualMetadata {
        unsafe { md::ActualMetadata::wrap(llvm::LLVMBuilderGetDefaultFPMathTag(self.0)) }
    }

    /// TODO Docs
    pub fn set_default_fp_math_tag(&self, fp_math_tag: md::ActualMetadata) {
        unsafe { llvm::LLVMBuilderSetDefaultFPMathTag(self.0, expose!(fp_math_tag)) }
    }
}


// TODO Gradually refine and distill -
// Not all of the code below needs to be a trait object.
// Not all of the code below needs to be an abstract type either - make sure Rust numbers work too
// TEST and consult documentation

pub use llvm_sys::LLVMOpcode;

macro_rules! build_op {
    ($op_name:ident, $fn:path $(, $($arg_name:ident : $arg_typ:path),*)?) => {
        impl Builder {
            #[doc = "TODO: Dynamically link to LLVM documentation using croc"]
            pub fn $op_name(&self $(, $($arg_name: $arg_typ),*)?) -> val::Instruction {
                unsafe {
                    let value = $fn(self.0 $(, $(expose!($arg_name)),*)?);
                    val::ref_to_value(value)
                }
            }
        }
    };

    (named, $op_name:ident, $fn:path $(, $($arg_name:ident : $arg_typ:ty),*)?) => {
        impl Builder {
            #[doc = "TODO: Dynamically link to LLVM documentation using croc"]
            pub fn $op_name(&self $(, $($arg_name: $arg_typ),*)?, name: &str) -> val::Instruction {
                unsafe {
                    let value = $fn(self.0 $(, $(expose!($arg_name)),*)?, str_to_cstr!(name));
                    val::ref_to_value(value)
                }
            }
        }
    };

    (named string, $op_name:ident, $fn:path) => {
        impl Builder {
            #[doc = "TODO: Dynamically link to LLVM documentation using croc"]
            pub fn $op_name(&self, string: &str, name: &str) -> val::Instruction {
                unsafe {
                    let value = $fn(self.0, str_to_cstr!(string), str_to_cstr!(name));
                    val::ref_to_value(value)
                }
            }
        }
    };

    (named values, $op_name:ident, $fn:path $(, $($arg_name:ident : $arg_typ:ty),*)?) => {
        impl Builder {
            #[doc = "TODO: Dynamically link to LLVM documentation using croc"]
            pub fn $op_name(&self $(, $($arg_name: $arg_typ),*)?, values: Vec<Box<dyn Value>>, name: &str) -> val::Instruction {
                unsafe {
                    let value = $fn(self.0 $(, $(expose!($arg_name)),*)?, expose_array!(values), size!(values), str_to_cstr!(name));
                    val::ref_to_value(value)
                }
            }
        }
    };

    (function, $op_name:ident, $fn:path, $typ:ident: Box<$spec_typ:ty> $(, $($arg_name:ident : $arg_typ:ty),*)?) => {
        impl Builder {
            #[doc = "TODO: Dynamically link to LLVM documentation using croc"]
            pub fn $op_name(
                &self,
                $typ: Box<$spec_typ>,
                function: val::Function,
                args: Vec<val::Argument>
                $(, $($arg_name: $arg_typ),*)?,
                name: &str
            ) -> val::Instruction {
                unsafe {
                    let value = $fn(
                        self.0,
                        expose!($typ),
                        expose!(function),
                        expose_array!(args),
                        size!(args)
                        $(, $(expose!($arg_name)),*)?,
                        str_to_cstr!(name)
                    );
                    val::ref_to_value(value)
                }
            }
        }
    };

    (function, $op_name:ident, $fn:path $(, $($arg_name:ident : $arg_typ:ty),*)?) => {
        impl Builder {
            #[doc = "TODO: Dynamically link to LLVM documentation using croc"]
            pub fn $op_name(
                &self,
                function: val::Function,
                args: Vec<val::Argument>
                $(, $($arg_name: $arg_typ),*)?,
                name: &str
            ) -> val::Instruction {
                unsafe {
                    let value = $fn(
                        self.0,
                        expose!(function),
                        expose_array!(args),
                        size!(args)
                        $(, $(expose!($arg_name)),*)?,
                        str_to_cstr!(name)
                    );
                    val::ref_to_value(value)
                }
            }
        }
    };
}


// Memory allocation
build_op!(named, build_malloc, llvm::LLVMBuildMalloc, typ: Box<dyn Type>);
build_op!(named, build_alloca, llvm::LLVMBuildAlloca, typ: Box<dyn Type>);
build_op!(named, build_array_malloc, llvm::LLVMBuildArrayMalloc, typ: Box<dyn Type>, len: val::ConstantInt);
build_op!(named, build_array_alloca, llvm::LLVMBuildArrayAlloca, typ: Box<dyn Type>, len: val::ConstantInt);

// Working with memory
build_op!(build_mem_set, llvm::LLVMBuildMemSet, ptr: Box<dyn Value>, val: Box<dyn Value>, size: val::ConstantInt, align: u32);
build_op!(build_mem_move, llvm::LLVMBuildMemMove,
          dest: Box<dyn Value>, dest_align: u32, src: Box<dyn val::Value>, src_align: u32, size: val::ConstantInt);
build_op!(build_mem_copy, llvm::LLVMBuildMemCpy,
          dest: Box<dyn Value>, dest_align: u32, src: Box<dyn Value>, src_align: u32, size: val::ConstantInt);

// Pointer comparison
build_op!(named, build_ptr_diff, llvm::LLVMBuildPtrDiff, lhs: Box<dyn Value>, rhs: Box<dyn Value>);

// Check existence
build_op!(named, build_is_null, llvm::LLVMBuildIsNull, val: Box<dyn Value>);
build_op!(named, build_is_not_null, llvm::LLVMBuildIsNotNull, val: Box<dyn Value>);

//// TODO Atomics & Concurrency
//op_with_name!(build_fence, llvm::LLVMBuildFence, ordering: LLVMAtomicOrdering, singlethread: LLVMBool);
//op!(build_atomic_cmp_xchg, llvm::LLVMBuildAtomicCmpXchg,
//    ptr: Box<dyn Value>, cmp: LLVMValueRef, new: LLVMValueRef,  // TODO Atomics
//    succ_ord: LLVMAtomicOrdering, fail_ord: LLVMAtomicOrdering,
//    singlethread: LLVMBool);
//op!(build_atomic_rmw, llvm::LLVMBuildAtomicRMW,
//    op: LLVMAtomicRMWBinOp, ptr: LLVMValueRef, val: LLVMValueRef, ord: LLVMAtomicOrdering, singlethread: LLVMBool);

// Global strings
build_op!(named string, build_global_string, llvm::LLVMBuildGlobalString);
build_op!(named string, build_global_string_ptr, llvm::LLVMBuildGlobalStringPtr);

// Variable assignment and access
build_op!(build_store, llvm::LLVMBuildStore,  val: Box<dyn Value>, ptr: Box<dyn Value>);
build_op!(build_free, llvm::LLVMBuildFree, ptr: Box<dyn Value>);
build_op!(named, build_load, llvm::LLVMBuildLoad, ptr: Box<dyn Value>);
build_op!(named, build_load_2, llvm::LLVMBuildLoad2, typ: Box<dyn Type>, ptr: Box<dyn Value>);
build_op!(named, build_freeze, llvm::LLVMBuildFreeze, val: Box<dyn Value>);

// Integer resizing instructions
build_op!(named, build_trunc, llvm::LLVMBuildTrunc, int: val::ConstantInt, int_typ: Box<dyn typ::Integer>);
build_op!(named, build_zext, llvm::LLVMBuildZExt, int: val::ConstantInt, int_typ: Box<dyn typ::Integer>);
build_op!(named, build_sext, llvm::LLVMBuildSExt, int: val::ConstantInt, int_typ: Box<dyn typ::Integer>);
build_op!(named, build_fp_trunc, llvm::LLVMBuildFPTrunc, float: val::ConstantFP, float_typ: Box<dyn typ::FloatingPoint>);
build_op!(named, build_fp_ext, llvm::LLVMBuildFPExt, float: val::ConstantFP, float_typ: Box<dyn typ::FloatingPoint>);

// Number conversions
build_op!(named, build_fp_cast, llvm::LLVMBuildFPCast, float: val::ConstantFP, float_typ: Box<dyn typ::FloatingPoint>);
build_op!(named, build_fp_to_ui, llvm::LLVMBuildFPToUI, float: val::ConstantFP, int_typ: Box<dyn typ::Integer>);
build_op!(named, build_fp_to_si, llvm::LLVMBuildFPToSI, float: val::ConstantFP, int_typ: Box<dyn typ::Integer>);

build_op!(named, build_int_cast, llvm::LLVMBuildIntCast, val: val::ConstantInt, dest_typ: Box<dyn typ::Integer>);
//TODO build_op!(named, build_int_cast_2, llvm::LLVMBuildIntCast2, val: Box<dyn Value>, dest_typ: Box<dyn Type>), is_signed: LLVMBool);
build_op!(named, build_ui_to_fp, llvm::LLVMBuildUIToFP, int: val::ConstantInt, float_typ: Box<dyn typ::FloatingPoint>);
build_op!(named, build_si_to_fp, llvm::LLVMBuildSIToFP, int: val::ConstantInt, float_typ: Box<dyn typ::FloatingPoint>);
build_op!(named, build_ptr_to_int, llvm::LLVMBuildPtrToInt, val: Box<dyn Value>, int_typ: Box<dyn typ::Integer>); // TODO Revisit this
build_op!(named, build_int_to_ptr, llvm::LLVMBuildIntToPtr, int: val::ConstantInt, ptr_typ: typ::Pointer);  // And this

// Casts
build_op!(named, build_cast, llvm::LLVMBuildCast, op: LLVMOpcode, val: Box<dyn Value>, dest_typ: Box<dyn Type>);
build_op!(named, build_bit_cast, llvm::LLVMBuildBitCast, val: Box<dyn Value>, dest_typ: Box<dyn typ::Reference>);
build_op!(named, build_trunc_or_bit_cast, llvm::LLVMBuildTruncOrBitCast, val: Box<dyn Value>, dest_typ: Box<dyn Type>);
build_op!(named, build_zext_or_bit_cast, llvm::LLVMBuildZExtOrBitCast, val: val::ConstantInt, dest_typ: Box<dyn typ::Integer>);
build_op!(named, build_sext_or_bit_cast, llvm::LLVMBuildSExtOrBitCast, val: val::ConstantInt, dest_typ: Box<dyn typ::Integer>);
build_op!(named, build_addr_space_cast, llvm::LLVMBuildAddrSpaceCast, val: Box<dyn Value>, ptr_typ: typ::Pointer); // TODO Revsit this
build_op!(named, build_pointer_cast, llvm::LLVMBuildPointerCast, val: Box<dyn Value>, ptr_typ: typ::Pointer);  // And this

// Comparison operations
build_op!(named, build_icmp, llvm::LLVMBuildICmp, op: LLVMIntPredicate, lhs: Box<dyn Value>, rhs: Box<dyn Value>); // And these
build_op!(named, build_fcmp, llvm::LLVMBuildFCmp, op: LLVMRealPredicate, lhs: Box<dyn Value>, rhs: Box<dyn Value>);

// Phi node
build_op!(named, build_phi, llvm::LLVMBuildPhi, typ: Box<dyn Type>);

// Variable list access
build_op!(named, build_vaarg, llvm::LLVMBuildVAArg, list: Box<dyn Value>, typ: Box<dyn Type>);

// Working with vectors
build_op!(named, build_extract_element, llvm::LLVMBuildExtractElement, vec: Box<dyn val::Collection>, index: val::ConstantInt);
build_op!(named, build_insert_element, llvm::LLVMBuildInsertElement,
          vec: Box<dyn val::Collection>, val: Box<dyn Value>, index: val::ConstantInt);
build_op!(named, build_shuffle_vector, llvm::LLVMBuildShuffleVector,
          v1: Box<dyn val::Collection>, v2: Box<dyn val::Collection>, mask: Box<dyn val::Collection>);

// Working with aggregates
build_op!(named, build_extract_value, llvm::LLVMBuildExtractValue, agg: Box<dyn val::Aggregate>, index: u32);
build_op!(named, build_insert_value, llvm::LLVMBuildInsertValue, agg: Box<dyn val::Aggregate>, val: Box<dyn Value>, index: u32);

// Referencing elements (Get Element Pointer)
build_op!(named values, build_gep, llvm::LLVMBuildGEP, val: Box<dyn Value>);
build_op!(named values, build_gep_2, llvm::LLVMBuildGEP2, typ: Box<dyn Type>, val: Box<dyn Value>);
build_op!(named values, build_in_bounds_gep, llvm::LLVMBuildInBoundsGEP, val: Box<dyn Value>);
build_op!(named values, build_in_bounds_gep_2, llvm::LLVMBuildInBoundsGEP2, typ: Box<dyn Type>, val: Box<dyn Value>);

// Binary operation from Opcode
build_op!(named, build_bin_op, llvm::LLVMBuildBinOp, op: LLVMOpcode, lhs: Box<dyn Value>, rhs: Box<dyn Value>);

// Integer math
build_op!(named, build_neg, llvm::LLVMBuildNeg, val: Box<dyn val::Number>);
build_op!(named, build_add, llvm::LLVMBuildAdd, lhs: Box<dyn val::Number>, rhs: Box<dyn val::Number>);
build_op!(named, build_sub, llvm::LLVMBuildSub, lhs: Box<dyn val::Number>, rhs: Box<dyn val::Number>);
build_op!(named, build_mul, llvm::LLVMBuildMul, lhs: Box<dyn val::Number>, rhs: Box<dyn val::Number>);
build_op!(named, build_sdiv, llvm::LLVMBuildSDiv, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_srem, llvm::LLVMBuildSRem, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_exact_sdiv, llvm::LLVMBuildExactSDiv, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);

build_op!(named, build_udiv, llvm::LLVMBuildUDiv, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_urem, llvm::LLVMBuildURem, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_exact_udiv, llvm::LLVMBuildExactUDiv, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);


// Floating point math
build_op!(named, build_fneg, llvm::LLVMBuildFNeg, val: Box<dyn val::FloatingPoint>);
build_op!(named, build_fadd, llvm::LLVMBuildFAdd, lhs: Box<dyn val::FloatingPoint>, rhs: Box<dyn val::FloatingPoint>);
build_op!(named, build_fsub, llvm::LLVMBuildFSub, lhs: Box<dyn val::FloatingPoint>, rhs: Box<dyn val::FloatingPoint>);
build_op!(named, build_fmul, llvm::LLVMBuildFMul, lhs: Box<dyn val::FloatingPoint>, rhs: Box<dyn val::FloatingPoint>);
build_op!(named, build_fdiv, llvm::LLVMBuildFDiv, lhs: Box<dyn val::FloatingPoint>, rhs: Box<dyn val::FloatingPoint>);
build_op!(named, build_frem, llvm::LLVMBuildFRem, lhs: Box<dyn val::FloatingPoint>, rhs: Box<dyn val::FloatingPoint>);

// No overflow signed wrapping math
build_op!(named, build_nswneg, llvm::LLVMBuildNSWNeg, val: Box<dyn val::Integer>);
build_op!(named, build_nswadd, llvm::LLVMBuildNSWAdd, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_nswsub, llvm::LLVMBuildNSWSub, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_nswmul, llvm::LLVMBuildNSWMul, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);

// No overflow unsigned wrapping math
build_op!(named, build_nuwneg, llvm::LLVMBuildNUWNeg, val: Box<dyn val::Integer>);
build_op!(named, build_nuwadd, llvm::LLVMBuildNUWAdd, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_nuwsub, llvm::LLVMBuildNUWSub, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_nuwmul, llvm::LLVMBuildNUWMul, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);

// Bitshifting operations
build_op!(named, build_shl, llvm::LLVMBuildShl, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_lshr, llvm::LLVMBuildLShr, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_ashr, llvm::LLVMBuildAShr, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);

// Bitwise logical operators
build_op!(named, build_and, llvm::LLVMBuildAnd, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_or, llvm::LLVMBuildOr, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_xor, llvm::LLVMBuildXor, lhs: Box<dyn val::Integer>, rhs: Box<dyn val::Integer>);
build_op!(named, build_not, llvm::LLVMBuildNot, val: Box<dyn val::Integer>);

// Flow control & branching
build_op!(named, build_select, llvm::LLVMBuildSelect, cond: Box<dyn val::Integer>, then: Box<dyn Value>, els: Box<dyn Value>);
build_op!(build_cond_br, llvm::LLVMBuildCondBr, cond: Box<dyn val::Integer>, then: bb::Block, els: bb::Block);
build_op!(build_switch, llvm::LLVMBuildSwitch, val: Box<dyn Value>, els: bb::Block, num_cases: u32);
build_op!(build_br, llvm::LLVMBuildBr, dest: bb::Block);
build_op!(build_indirect_br, llvm::LLVMBuildIndirectBr, addr: val::BlockAddress, num_dests: u32);
build_op!(build_unreachable, llvm::LLVMBuildUnreachable);

// Functions
build_op!(function, build_call, llvm::LLVMBuildCall);
build_op!(function, build_call_2, llvm::LLVMBuildCall2, typ: Box<dyn Type>);

// Return statements
build_op!(build_ret_void, llvm::LLVMBuildRetVoid);
build_op!(build_ret, llvm::LLVMBuildRet, ret_val: Box<dyn Value>);
impl Builder {
    // TODO Docs, macro?
    pub fn build_aggregate_ret(&self, ret_aggr: Vec<Box<dyn Value>>) -> val::Instruction {
        unsafe { val::Instruction::wrap(
            llvm::LLVMBuildAggregateRet(self.0, expose_array!(ret_aggr), size!(ret_aggr))
        )}
    }
}

// Function components involving exceptions
build_op!(build_resume, llvm::LLVMBuildResume, exception: Box<dyn Value>);
build_op!(build_catch_ret, llvm::LLVMBuildCatchRet, pad: val::BasicBlock, block: bb::Block);
build_op!(build_cleanup_ret, llvm::LLVMBuildCleanupRet, pad: val::BasicBlock, block: bb::Block);
build_op!(named, build_landing_pad, llvm::LLVMBuildLandingPad, typ: Box<dyn Type>, pers_fun: val::Function, num_clauses: u32);
build_op!(named, build_catch_switch, llvm::LLVMBuildCatchSwitch, pad: val::BasicBlock, unwind: bb::Block, num_handler: u32);

build_op!(function, build_invoke, llvm::LLVMBuildInvoke, then: bb::Block, catch: bb::Block);
build_op!(function, build_invoke_2, llvm::LLVMBuildInvoke2, typ: Box<dyn Type>, then: bb::Block, catch: bb::Block);

build_op!(named values, build_catch_pad, llvm::LLVMBuildCatchPad, pad: val::BasicBlock);
build_op!(named values, build_cleanup_pad, llvm::LLVMBuildCleanupPad, pad: val::BasicBlock);














