
use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;

//
//        match llvm::LLVMGetTypeKind($typ_ref) {
//            LLVMTypeKind::LLVMVoidTypeKind            => Void,
//            LLVMTypeKind::LLVMHalfTypeKind            => Half,
//            LLVMTypeKind::LLVMFloatTypeKind           => Float,
//            LLVMTypeKind::LLVMDoubleTypeKind          => Double,
//            LLVMTypeKind::LLVMX86_FP80TypeKind        => X86FP80,
//            LLVMTypeKind::LLVMFP128TypeKind           => FP128,
//            LLVMTypeKind::LLVMPPC_FP128TypeKind       => PPCFP128,
//            LLVMTypeKind::LLVMLabelTypeKind           => Label,
//            LLVMTypeKind::LLVMIntegerTypeKind         => Int,
//            LLVMTypeKind::LLVMFunctionTypeKind        => Function,
//            LLVMTypeKind::LLVMStructTypeKind          => Struct,
//            LLVMTypeKind::LLVMArrayTypeKind           => Array,
//            LLVMTypeKind::LLVMPointerTypeKind         => Pointer,
//            LLVMTypeKind::LLVMVectorTypeKind          => Vector,
//            LLVMTypeKind::LLVMMetadataTypeKind        => Metadata,
//            LLVMTypeKind::LLVMX86_MMXTypeKind         => X86MMX,
//            LLVMTypeKind::LLVMTokenTypeKind           => Token,
//            LLVMTypeKind::LLVMScalableVectorTypeKind  => ScalableVector,
//            LLVMTypeKind::LLVMBFloatTypeKind          => BFloat,
//            LLVMTypeKind::LLVMX86_AMXTypeKind         => X86AMX,
//        })
//

pub trait Type: Wrapper<Llvm = LLVMTypeRef> {
    // TODO Docs
    fn is_sized(&self) -> LLVMBool {
        unsafe { llvm::LLVMTypeIsSized(*self.expose()) }
    }

    /// Prints a textual representation of the type to the error stream
    fn dump(&self) {
        unsafe { llvm::LLVMDumpType(*self.expose()) }
    }

    // TODO Docs
    fn context(&self) -> super::context::Context {
        super::context::Context::wrap(
            unsafe { llvm::LLVMGetTypeContext(*self.expose()) }
        )
    }

    // TODO Docs
    fn kind(&self) -> LLVMTypeKind {
        unsafe { llvm::LLVMGetTypeKind(*self.expose()) }
    }

    // TODO Docs
    fn to_string(&self) -> String {
        cstr_to_str!(llvm::LLVMPrintTypeToString(*self.expose())).to_string()
    }

    fn upcast(typ_ref: LLVMTypeRef) -> Option<Box<dyn Type>> {
        todo![]
    }
}


/// TODO Docs, Testing
macro_rules! llvm_type {
    ($t:ident, $fn:path $(, $($argn:ident: $argv:path),*)?) => {
        wrapper!($t, LLVMTypeRef);
        impl Type for $t {}
        impl $t {
            pub fn new($($($argn: $argv),*)?) -> Self {
                unsafe { Self($fn($($($argn),*)?)) }
            }
        }
    };
}

// Token type
wrapper!(Token, LLVMTypeRef);
impl Type for Token {}

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
llvm_type!(Int, llvm::LLVMIntType, num_bits: u32);

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
wrapper!(Function, LLVMTypeRef);
impl Type for Function {}
impl Function {
    pub fn new(ret_typ: LLVMTypeRef, param_types: &mut [LLVMTypeRef], is_var_arg: LLVMBool) -> Self {
        unsafe {
            Self(llvm::LLVMFunctionType(ret_typ, param_types.as_mut_ptr(), param_types.len() as u32, is_var_arg))
        }
    }
}

// Raw type for protecting data retrieved from reference
wrapper!(RawTypeRef, LLVMTypeRef);
impl Type for RawTypeRef {}
impl RawTypeRef {
    // TODO Need a function that can recontextualize the type
}







