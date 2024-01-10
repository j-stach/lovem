
use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;


pub trait Type: Wrapper<Llvm = LLVMTypeRef> {
    /// Returns true if the type is sized TODO Clarify
    fn is_sized(&self) -> bool {
        bool_to_rust!(llvm::LLVMTypeIsSized(expose!(self)))
    }

    /// Prints a textual representation of the type to the error stream
    fn dump(&self) {
        unsafe { llvm::LLVMDumpType(expose!(self)) }
    }

    /// Get a new reference to the type's parent Context
    fn context(&self) -> super::context::Context {
        super::context::Context::wrap(
            unsafe { llvm::LLVMGetTypeContext(expose!(self)) }
        )
    }

    /// Returns the corresponding "kind" enum variant for the type
    fn kind(&self) -> LLVMTypeKind {
        unsafe { llvm::LLVMGetTypeKind(expose!(self)) }
    }

    /// Returns the name of the type as a string
    fn to_string(&self) -> String {
        cstr_to_str!(llvm::LLVMPrintTypeToString(expose!(self))).to_string()
    }
}

/// Wraps a naked LLVMTypeRef with the corresponding safe type & stores it on the heap
pub fn type_from_ref(typ_ref: LLVMTypeRef) -> Box<dyn Type> {
    unsafe { match llvm::LLVMGetTypeKind(typ_ref) {
        LLVMTypeKind::LLVMVoidTypeKind            => Box::new(Void::wrap(typ_ref)),
        LLVMTypeKind::LLVMHalfTypeKind            => Box::new(Half::wrap(typ_ref)),
        LLVMTypeKind::LLVMFloatTypeKind           => Box::new(Float::wrap(typ_ref)),
        LLVMTypeKind::LLVMDoubleTypeKind          => Box::new(Double::wrap(typ_ref)),
        LLVMTypeKind::LLVMX86_FP80TypeKind        => Box::new(X86FP80::wrap(typ_ref)),
        LLVMTypeKind::LLVMFP128TypeKind           => Box::new(FP128::wrap(typ_ref)),
        LLVMTypeKind::LLVMPPC_FP128TypeKind       => Box::new(PPCFP128::wrap(typ_ref)),
        LLVMTypeKind::LLVMLabelTypeKind           => unimplemented!(),                      // TODO
        LLVMTypeKind::LLVMIntegerTypeKind         => Box::new(Int::wrap(typ_ref)),
        LLVMTypeKind::LLVMFunctionTypeKind        => Box::new(Function::wrap(typ_ref)),
        LLVMTypeKind::LLVMStructTypeKind          => unimplemented!(),                      // TODO
        LLVMTypeKind::LLVMArrayTypeKind           => Box::new(Array::wrap(typ_ref)),
        LLVMTypeKind::LLVMPointerTypeKind         => Box::new(Pointer::wrap(typ_ref)),
        LLVMTypeKind::LLVMVectorTypeKind          => Box::new(Vector::wrap(typ_ref)),
        LLVMTypeKind::LLVMMetadataTypeKind        => unimplemented!(),                      // TODO
        LLVMTypeKind::LLVMX86_MMXTypeKind         => Box::new(X86MMX::wrap(typ_ref)),
        LLVMTypeKind::LLVMTokenTypeKind           => Box::new(Token::wrap(typ_ref)),
        LLVMTypeKind::LLVMScalableVectorTypeKind  => Box::new(ScalableVector::wrap(typ_ref)),
        LLVMTypeKind::LLVMBFloatTypeKind          => Box::new(BFloat::wrap(typ_ref)),
        LLVMTypeKind::LLVMX86_AMXTypeKind         => Box::new(X86AMX::wrap(typ_ref)),
    }}
}


/// Generates boilerplate impls of Type for wrappers of LLVMTypeRef
macro_rules! llvm_type {
    ($t:ident, $fn:path $(, $($argn:ident: $argv:path),*)?) => {
        // TODO Account for possible assoc type, & expose it for the fn
        wrapper!($t, LLVMTypeRef);
        impl Type for $t {}
        impl $t {
            pub fn new($($($argn: $argv),*)?) -> Self {
                unsafe { Self($fn($($($argn),*)?)) }
            }
        }
    };
}

/// Generates boilerplate for types with a single associated type
macro_rules! llvm_type_with_assoc {
    ($t:ident, $fn:path $(, $($argn:ident: $argv:path),*)?) => {
        // TODO Account for possible assoc type, & expose it for the fn
        wrapper!($t, LLVMTypeRef);
        impl Type for $t {}
        impl $t {
            pub fn new<T: Wrapper<Llvm = LLVMTypeRef>>(assoc: T, $($($argn: $argv),*)?) -> Self {
                unsafe { Self($fn(expose!(assoc)$(, $($argn),*)?)) }
            }
        }
    };
}

// Token type
wrapper!(Token, LLVMTypeRef);
impl Type for Token {}

// Pointers
llvm_type_with_assoc!(Pointer, llvm::LLVMPointerType, addr: u32);
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
llvm_type_with_assoc!(ScalableVector, llvm::LLVMScalableVectorType, size: u32);
llvm_type_with_assoc!(Vector, llvm::LLVMVectorType, size: u32);
llvm_type_with_assoc!(Array, llvm::LLVMArrayType, size: u32);

// X86 types
llvm_type!(X86FP80, llvm::LLVMX86FP80Type);
llvm_type!(X86AMX, llvm::LLVMX86AMXType);
llvm_type!(X86MMX, llvm::LLVMX86MMXType);


// Type for protecting temporary data retrieved as reference.
// Use sparingly.
wrapper!(Raw, LLVMTypeRef);
impl Type for Raw {}
impl Raw {
    /// Converts from Raw refrence to the correct named Type
    pub fn to_type(self) -> Box<dyn Type> {
        unsafe { type_from_ref(expose!(self)) }
    }
}

// Function type
wrapper!(Function, LLVMTypeRef);
impl Type for Function {}
impl Function {
    pub fn new(ret_typ: Box<dyn Type>, param_types: Vec<Box<dyn Type>>, is_var_arg: bool) -> Self {
        unsafe {
            Self(llvm::LLVMFunctionType(
                expose!(ret_typ),
                expose_array!(param_types),
                size!(param_types),
                bool_to_llvm!(is_var_arg)
            ))
        }
    }
}






