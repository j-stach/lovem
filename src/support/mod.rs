
pub mod init;
pub mod linker;
pub mod target;
pub mod object;


use llvm_sys::support as llvm;

/// TODO Docs
pub type LLVMSymbolValue = *mut std::ffi::c_void;

/// TODO Docs
pub fn new_symbol(name: &str, val: LLVMSymbolValue) {   // TODO Make safe, or mark unsafe?
    unsafe { llvm::LLVMAddSymbol(str_to_cstr!(name), val) }
}

/// TODO Docs
pub fn find_symbol_address(name: &str) -> Option<LLVMSymbolValue> { // TODO Safe or unsafe?
    let addr = unsafe { llvm::LLVMSearchForAddressOfSymbol(str_to_cstr!(name)) };
    if addr.is_null() { return None }
    else { return Some(addr) }
}

/// TODO Docs
pub fn load_library_permanently(filepath: &str) -> Result<(), anyhow::Error> { // TODO revisit bool-error translations
    let load = unsafe { llvm::LLVMLoadLibraryPermanently(str_to_cstr!(filepath)) };
    if load > 0 { return Ok(()) }
    else { return Err(anyhow::anyhow!("Failed to load library '{}' from file", filepath)) }
}

/// TODO Docs
pub fn parse_cli_options(args: Vec<&str>, overview: &str) {
    let len = args.len() as i32;
    let c_args: Vec<*const std::ffi::c_char> = args.into_iter().map(|a| str_to_cstr!(a)).collect();
    unsafe { llvm::LLVMParseCommandLineOptions(len, c_args.as_ptr(), str_to_cstr!(overview)) }
}



