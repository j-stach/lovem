
use llvm_sys::core as llvm;

// TODO Docs
pub fn lookup_intrinsic_id(name: &str) -> u32 {
    unsafe { llvm::LLVMLookupIntrinsicID(str_to_cstr!(name), name.len()) }
}

// TODO Docs
pub fn get_intrinsic_name(id: u32) -> String {
    let ref mut name: Vec<usize> = vec![];
    cstr_to_str!(llvm::LLVMIntrinsicGetName(id, name.as_mut_ptr())).to_string()
}

// TODO Docs
pub fn intrinsic_is_overloaded(id: u32) -> bool {
    bool_to_rust!(llvm::LLVMIntrinsicIsOverloaded(id))
}
