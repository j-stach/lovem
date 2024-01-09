
use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use derive_more::{Deref, DerefMut};


pub trait Type: std::ops::DerefMut<Target = LLVMTypeRef> {
    // TODO Docs
    fn is_sized(&self) -> LLVMBool {
        unsafe { llvm::LLVMTypeIsSized(*self.deref()) }
    }

    // TODO Docs
    fn type_of(val: LLVMValueRef) -> LLVMTypeRef { // TODO Match & wrap
        unsafe { llvm::LLVMTypeOf(val) }
    }

    /// Prints a textual representation of the type to the error stream
    fn dump(&self) {
        unsafe { llvm::LLVMDumpType(*self.deref()) }
    }

    // TODO Docs
    fn context(&self) -> super::context::Context {
        super::context::Context::wrap(
            unsafe { llvm::LLVMGetTypeContext(*self.deref()) }
        )
    }

    // TODO Docs
    fn kind(&self) -> LLVMTypeKind {
        unsafe { llvm::LLVMGetTypeKind(*self.deref()) }
    }

    // TODO Docs
    fn to_string(&self) -> String {
        c_str_to_str!(llvm::LLVMPrintTypeToString(*self.deref())).to_string()
    }
}


/// TODO Docs, Testing
macro_rules! llvm_type {
    ($t:ident, $fn:path $(, $($argn:ident: $argv:path),*)?) => {
        #[derive(Deref, DerefMut)]
        pub struct $t (LLVMTypeRef);
        impl $t {
            pub fn new($($($argn: $argv),*)?) -> Self {
                unsafe { Self($fn($($($argn),*)?)) }
            }
        }

        impl Type for $t {}
    };
}

// Pointers
llvm_type!(Pointer, llvm::LLVMPointerType, typ: LLVMTypeRef, addr: u32);
llvm_type!(Void, llvm::LLVMVoidType);

// Integer
llvm_type!(Int1, llvm::LLVMInt1Type);
llvm_type!(Int8, llvm::LLVMInt8Type);
llvm_type!(Int16, llvm::LLVMInt16Type);
llvm_type!(Int32, llvm::LLVMInt32Type);
llvm_type!(Int64, llvm::LLVMInt64Type);
llvm_type!(Int128, llvm::LLVMInt128Type);

// Floating Point
llvm_type!(Float, llvm::LLVMFloatType);
llvm_type!(BFloat, llvm::LLVMBFloatType);
llvm_type!(Double, llvm::LLVMDoubleType);
llvm_type!(Half, llvm::LLVMHalfType);
llvm_type!(FP128, llvm::LLVMFP128Type);
llvm_type!(PPCFP128, llvm::LLVMPPCFP128Type);

// Collections
llvm_type!(ScalableVector, llvm::LLVMScalableVectorType, elem_typ: LLVMTypeRef, size: u32);
llvm_type!(Vector, llvm::LLVMVectorType, elem_typ: LLVMTypeRef, size: u32);
llvm_type!(Array, llvm::LLVMArrayType, elem_typ: LLVMTypeRef, size: u32);

// Special types
llvm_type!(X86FP80, llvm::LLVMX86FP80Type);
llvm_type!(X86AMX, llvm::LLVMX86AMXType);
llvm_type!(X86MMX, llvm::LLVMX86MMXType);

// Function type
#[derive(Deref, DerefMut)]
pub struct Function(LLVMTypeRef);
impl Type for Function {}
impl Function {
    pub fn new(ret_typ: LLVMTypeRef, param_types: &mut [LLVMTypeRef], is_var_arg: LLVMBool) -> Self {
        unsafe {
            Self(llvm::LLVMFunctionType(ret_typ, param_types.as_mut_ptr(), param_types.len() as u32, is_var_arg))
        }
    }
}













