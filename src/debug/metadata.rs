
use llvm_sys::prelude::*;

use crate::wrapper::Wrapper;


// Wrapper for actual ref to metadata
wrapper!(ActualMetadata, LLVMMetadataRef);

// Metadata as type
wrapper!(MetadataType, LLVMTypeRef);
impl crate::ir::types::Type for MetadataType {}

// Metadata as value
pub use crate::ir::values::MetadataAsValue;

wrapper!(MetadataNode, LLVMNamedMDNodeRef);
impl MetadataNode {
    pub fn name(&self) -> String {
        let ref mut len: usize = 0;
        cstr_to_str!(llvm_sys::core::LLVMGetNamedMetadataName(self.0, len)).to_string()
    }
}

// Attribute
wrapper!(Attribute, LLVMAttributeRef);















