
use llvm_sys::prelude::*;

use crate::wrapper::Wrapper;

// Wrapper for actual ref to metadata
wrapper!(ActualMetadata, LLVMMetadataRef);


// Metadata as type
wrapper!(MetadataType, LLVMTypeRef);
impl super::types::Type for MetadataType {}


// Metadata as value
pub use super::values::MetadataAsValue;
