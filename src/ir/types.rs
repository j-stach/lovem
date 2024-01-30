
use llvm_sys::{*, prelude::*};
use llvm_sys::core as llvm;

use crate::wrapper::Wrapper;

use crate::debug::metadata as md;


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

/// Represents the subtype of Type within opaque LLVM
pub use llvm_sys::LLVMTypeKind::*;
/// Wraps a naked LLVMTypeRef with the corresponding safe type & stores it on the heap
pub fn type_from_ref(typ_ref: LLVMTypeRef) -> Box<dyn Type> {
    unsafe { match llvm::LLVMGetTypeKind(typ_ref) {
        LLVMTypeKind::LLVMVoidTypeKind            => Box::new(Void(typ_ref)),
        LLVMTypeKind::LLVMHalfTypeKind            => Box::new(Half(typ_ref)),
        LLVMTypeKind::LLVMFloatTypeKind           => Box::new(Float(typ_ref)),
        LLVMTypeKind::LLVMDoubleTypeKind          => Box::new(Double(typ_ref)),
        LLVMTypeKind::LLVMX86_FP80TypeKind        => Box::new(X86FP80(typ_ref)),
        LLVMTypeKind::LLVMFP128TypeKind           => Box::new(FP128(typ_ref)),
        LLVMTypeKind::LLVMPPC_FP128TypeKind       => Box::new(PPCFP128(typ_ref)),
        LLVMTypeKind::LLVMLabelTypeKind           => unimplemented!(),
        LLVMTypeKind::LLVMTargetExtTypeKind       => unimplemented!(),
        LLVMTypeKind::LLVMIntegerTypeKind         => Box::new(Int(typ_ref)),
        LLVMTypeKind::LLVMFunctionTypeKind        => Box::new(Function(typ_ref)),
        LLVMTypeKind::LLVMStructTypeKind          => Box::new(Struct(typ_ref)),
        LLVMTypeKind::LLVMArrayTypeKind           => todo![], //Box::new(Array(typ_ref)),
        LLVMTypeKind::LLVMPointerTypeKind         => Box::new(Pointer(typ_ref)),
        LLVMTypeKind::LLVMVectorTypeKind          => Box::new(Vector(typ_ref)),
        LLVMTypeKind::LLVMMetadataTypeKind        => Box::new(md::MetadataType::wrap(typ_ref)),
        LLVMTypeKind::LLVMX86_MMXTypeKind         => Box::new(X86MMX(typ_ref)),
        LLVMTypeKind::LLVMTokenTypeKind           => Box::new(Token(typ_ref)),
        LLVMTypeKind::LLVMScalableVectorTypeKind  => Box::new(ScalableVector(typ_ref)),
        LLVMTypeKind::LLVMBFloatTypeKind          => Box::new(BFloat(typ_ref)),
        LLVMTypeKind::LLVMX86_AMXTypeKind         => Box::new(X86AMX(typ_ref)),
    }}
}
/// WARNING Unchecked cast to Type that does not use the heap
pub fn ref_to_type<T: Wrapper<Llvm = LLVMTypeRef>>(typ_ref: LLVMTypeRef) -> T {
    T::wrap(typ_ref)
}


/// Generates boilerplate impls of Type for wrappers of LLVMTypeRef
macro_rules! llvm_type {
    ($t:ident, $fn:path $(, $($argn:ident: $argv:path),*)?) => {
        // TODO Account for possible assoc type, & expose it for the fn
        wrapper!($t, LLVMTypeRef);
        impl Type for $t {}
        impl $t {
            #[doc = "TODO: Dynamically generate docs"]
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
            #[doc = "TODO: Dynamically generate docs"]
            pub fn new<T: Wrapper<Llvm = LLVMTypeRef>>(assoc: T, $($($argn: $argv),*)?) -> Self {
                unsafe { Self($fn(expose!(assoc)$(, $($argn),*)?)) }
            }
        }
    };
}

// Pointers
pub trait Reference: Type {}
llvm_type_with_assoc!(Pointer, llvm::LLVMPointerType, addr: u32);
llvm_type!(Void, llvm::LLVMVoidType);

// TODO Implement these for respective types, and see if any functions should fall under the trait
pub trait Number: Type {}

// Integer
pub trait Integer: Number {} // TODO Do these all create the same type under the hood? Can i condense?
llvm_type!(Int1, llvm::LLVMInt1Type);
llvm_type!(Int8, llvm::LLVMInt8Type);
llvm_type!(Int16, llvm::LLVMInt16Type);
llvm_type!(Int32, llvm::LLVMInt32Type);
llvm_type!(Int64, llvm::LLVMInt64Type);
llvm_type!(Int128, llvm::LLVMInt128Type);
llvm_type!(Int, llvm::LLVMIntType, num_bits: u32);

// Floating Point
pub trait FloatingPoint: Number {}
llvm_type!(Float, llvm::LLVMFloatType);
llvm_type!(BFloat, llvm::LLVMBFloatType);
llvm_type!(Double, llvm::LLVMDoubleType);
llvm_type!(Half, llvm::LLVMHalfType);
llvm_type!(FP128, llvm::LLVMFP128Type);
llvm_type!(PPCFP128, llvm::LLVMPPCFP128Type);

// Collections
pub trait Collection: Reference {}
llvm_type_with_assoc!(ScalableVector, llvm::LLVMScalableVectorType, size: u32);
llvm_type_with_assoc!(Vector, llvm::LLVMVectorType, size: u32);
// TODO UPDATE llvm_type_with_assoc!(Array, llvm::LLVMArrayType2, size: u32);

// X86 types
llvm_type!(X86FP80, llvm::LLVMX86FP80Type);
llvm_type!(X86AMX, llvm::LLVMX86AMXType);
llvm_type!(X86MMX, llvm::LLVMX86MMXType);


// Token type
wrapper!(Token, LLVMTypeRef);
impl Type for Token {}

// Type for protecting temporary data retrieved as reference.
// Use sparingly.
wrapper!(Raw, LLVMTypeRef);
impl Type for Raw {}
impl Raw {
    /// Converts from Raw refrence to the correct named Type
    pub fn type_of(self) -> Box<dyn Type> {
        unsafe { type_from_ref(expose!(self)) }
    }

    /// WARNING Unchecked cast to Type
    pub fn to_type<T: Wrapper<Llvm = LLVMTypeRef>>(self) -> T {
        T::wrap(unsafe { expose!(self) })
    }
}

// Function type
wrapper!(Function, LLVMTypeRef);
impl Type for Function {}
impl Function {
    pub fn new(ret_typ: Box<dyn Type>, param_types: Vec<Box<dyn Type>>, is_var_arg: bool) -> Self {
        unsafe { Self(llvm::LLVMFunctionType(
            expose!(ret_typ),
            expose_array!(param_types),
            size!(param_types),
            bool_to_llvm!(is_var_arg)
        ))}
    }

    pub fn num_params(&self) -> u32 {
        unsafe { llvm::LLVMCountParamTypes(self.0) }
    }

}

wrapper!(Struct, LLVMTypeRef);
impl Type for Struct {}
impl Struct {
    // TODO Docs
    pub fn new(elements: Vec<Box<dyn Type>>, packed: bool) -> Self {
        unsafe { Self(llvm::LLVMStructType(
            expose_array!(elements),
            size!(elements),
            bool_to_llvm!(packed)
        ))}
    }

    // TODO Docs
    pub fn type_of_index(&self, i: u32) -> Box<dyn Type> {
        type_from_ref(unsafe { llvm::LLVMStructGetTypeAtIndex(self.0, i) })
    }

    // TODO Docs
    pub fn set_body(&self, elements: Vec<Box<dyn Type>>, packed: bool) {
        unsafe { llvm::LLVMStructSetBody(
            self.0,
            expose_array!(elements),
            size!(elements),
            bool_to_llvm!(packed)
        )}
    }

    // TODO Docs
    pub fn num_elements(&self) -> usize {
        unsafe { llvm::LLVMCountStructElementTypes(self.0) as usize }
    }

    // TODO Docs
    pub fn get_struct_element_types(&self) -> Vec<Box<dyn Type>> {
        unsafe {
            let mut elements: Vec<LLVMTypeRef> = vec![];
            llvm::LLVMGetStructElementTypes(self.0, elements.as_mut_ptr());
            elements.iter().map(|e| type_from_ref(*e)).collect()
        }
    }

    // TODO Docs
    pub fn name(&self) -> String {
        cstr_to_str!(llvm::LLVMGetStructName(self.0)).to_string()
    }
}





