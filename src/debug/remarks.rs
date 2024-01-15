
use llvm_sys::remarks as llvm;

use crate::wrapper::Wrapper;


pub use llvm_sys::remarks::LLVMRemarkType as LLVMRemarkKind;

wrapper!(RemarkArg, llvm::LLVMRemarkArgRef);
wrapper!(RemarkDebugLoc, llvm::LLVMRemarkDebugLocRef);
wrapper!(RemarkEntry, llvm::LLVMRemarkEntryRef);
wrapper!(RemarkParser, llvm::LLVMRemarkParserRef);
wrapper!(RemarkString, llvm::LLVMRemarkStringRef);


// TODO Holding off for now, after debugging and thiserror
