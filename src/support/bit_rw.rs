
use crate::wrapper::Wrapper;

use llvm_sys::{object as obj, bit_writer as bw, bit_reader as br};

use crate::ir::{module::Module, memory_buffer as mb, context as cx};


pub fn get_bitcode_module(buffer: &mb::MemoryBuffer) -> Result<Module, anyhow::Error> {
    let ref mut module: llvm_sys::prelude::LLVMModuleRef = std::ptr::null_mut();
    let result = unsafe { br::LLVMGetBitcodeModule2(expose!(buffer), module) };
    if result == 0 { return Ok(Module::wrap(*module)) }
    else { return Err(anyhow::anyhow!("Failed to get bitcode module")) }
}

pub fn get_bitcode_module_in_context(context: &cx::Context, buffer: &mb::MemoryBuffer) -> Result<Module, anyhow::Error> {
    let ref mut module: llvm_sys::prelude::LLVMModuleRef = std::ptr::null_mut();
    let result = unsafe { br::LLVMGetBitcodeModuleInContext2(expose!(context), expose!(buffer), module) };
    if result == 0 { return Ok(Module::wrap(*module)) }
    else { return Err(anyhow::anyhow!("Failed to get bitcode module")) }
}

pub fn parse_bitcode(buffer: &mb::MemoryBuffer) -> Result<Module, anyhow::Error> {
    let ref mut module: llvm_sys::prelude::LLVMModuleRef = std::ptr::null_mut();
    let result = unsafe { br::LLVMParseBitcode2(expose!(buffer), module) };
    if result == 0 { return Ok(Module::wrap(*module)) }
    else { return Err(anyhow::anyhow!("Failed to get bitcode module")) }
}

pub fn parse_bitcode_in_context(context: &cx::Context, buffer: &mb::MemoryBuffer) -> Result<Module, anyhow::Error> {
    let ref mut module: llvm_sys::prelude::LLVMModuleRef = std::ptr::null_mut();
    let result = unsafe { br::LLVMParseBitcodeInContext2(expose!(context), expose!(buffer), module) };
    if result == 0 { return Ok(Module::wrap(*module)) }
    else { return Err(anyhow::anyhow!("Failed to get bitcode module")) }
}

pub fn write_bitcode_to_file_descriptor(module: &Module, fd: i32, should_close: bool, unbuffered: bool) -> Result<(), anyhow::Error> {
    let should_close = match should_close { true => 1, false => 0 };
    let unbuffered = match unbuffered { true => 1, false => 0 };
    let result = unsafe { bw::LLVMWriteBitcodeToFD(expose!(module), fd, should_close, unbuffered) };
    if result == 0 { return Ok(()) }
    else { return Err(anyhow::anyhow!("Failed to write module bitcode to file descriptor")) }
}

pub fn write_bitcode_to_file(module: &Module, path: &str) -> Result<(), anyhow::Error> {
    let result = unsafe { bw::LLVMWriteBitcodeToFile(expose!(module), str_to_cstr!(path)) };
    if result == 0 { return Ok(()) }
    else { return Err(anyhow::anyhow!("Failed to write module bitcode to file")) }
}

pub fn write_bitcode_to_file_handle(module: &Module, handle: i32) -> Result<(), anyhow::Error> {
    let result = unsafe { bw::LLVMWriteBitcodeToFileHandle(expose!(module), handle) };
    if result == 0 { return Ok(()) }
    else { return Err(anyhow::anyhow!("Failed to write module bitcode to file handle")) }
}

pub fn write_bitcode_to_memory_buffer(module: &Module) -> mb::MemoryBuffer {
    unsafe { mb::MemoryBuffer::wrap(bw::LLVMWriteBitcodeToMemoryBuffer(expose!(module))) }
}


pub use llvm_sys::object::LLVMBinaryType as LLVMBinaryKind;

wrapper!(Binary, obj::LLVMBinaryRef);
impl Binary {
    pub fn new(buffer: &mb::MemoryBuffer, context: &cx::Context) -> Result<Binary, anyhow::Error> {
        let ref mut message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result = unsafe { obj::LLVMCreateBinary(expose!(buffer), expose!(context), message) };
        if message.is_null() { return Ok(Binary::wrap(result)) }
        else {
            let message = unsafe { std::ffi::CString::from_raw(*message) };
            return Err(anyhow::anyhow!("Failed to create binary: {}", message.to_string_lossy()))
        }
    }

    pub fn copy_memory_buffer(&self) -> mb::MemoryBuffer {
        unsafe { mb::MemoryBuffer::wrap(obj::LLVMBinaryCopyMemoryBuffer(self.0)) }
    }

    pub fn kind(&self) -> LLVMBinaryKind {
        unsafe { obj::LLVMBinaryGetType(self.0) }
    }

    pub fn dispose(self) {
        unsafe { obj::LLVMDisposeBinary(self.0) }
    }

    pub fn macho_universal_binary_copy_for_arch(&self, arch: &str) -> Result<Binary, anyhow::Error> {
        let ref mut message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result = unsafe { obj::LLVMMachOUniversalBinaryCopyObjectForArch(self.0, str_to_cstr!(arch), arch.len(), message) };
        if message.is_null() { return Ok(Binary::wrap(result)) }
        else {
            let message = unsafe { std::ffi::CString::from_raw(*message) };
            return Err(anyhow::anyhow!("Failed to copy binary: {}", message.to_string_lossy()))
        }
    }

    pub fn copy_section_iterator(&self) -> SectionIterator {
        unsafe { SectionIterator::wrap(obj::LLVMObjectFileCopySectionIterator(self.0)) }
    }

    pub fn copy_symbol_iterator(&self) -> SymbolIterator {
        unsafe { SymbolIterator::wrap(obj::LLVMObjectFileCopySymbolIterator(self.0)) }
    }

    pub fn ends_in_section_iterator(&self, section: &SectionIterator) -> bool {
        let result = unsafe { obj::LLVMObjectFileIsSectionIteratorAtEnd(self.0, expose!(section)) };
        if result != 0 { return true } else { return false }
    }

    pub fn ends_in_symbol_iterator(&self, symbol: &SymbolIterator) -> bool {
        let result = unsafe { obj::LLVMObjectFileIsSymbolIteratorAtEnd(self.0, expose!(symbol)) };
        if result != 0 { return true } else { return false }
    }
}


wrapper!(RelocationIterator, obj::LLVMRelocationIteratorRef);
impl RelocationIterator {

    pub fn dispose(self) {
        unsafe { obj::LLVMDisposeRelocationIterator(self.0) }
    }

    pub fn offset(&self) -> u64 {
        unsafe { obj::LLVMGetRelocationOffset(self.0) }
    }

    pub fn symbol(&self) -> SymbolIterator {
        unsafe { SymbolIterator::wrap(obj::LLVMGetRelocationSymbol(self.0)) }
    }

    pub fn get_type(&self) -> u64 {
        unsafe { obj::LLVMGetRelocationType(self.0) }
    }

    pub fn get_type_name(&self) -> String {
        cstr_to_str!(obj::LLVMGetRelocationTypeName(self.0)).to_string()
    }

    pub fn get_value_string(&self) -> String {
        cstr_to_str!(obj::LLVMGetRelocationValueString(self.0)).to_string()
    }

    pub fn move_to_next(&self) {
        unsafe { obj::LLVMMoveToNextRelocation(self.0) }
    }

}


wrapper!(SectionIterator, obj::LLVMSectionIteratorRef);
impl SectionIterator {

    pub fn dispose(self) {
        unsafe { obj::LLVMDisposeSectionIterator(self.0) }
    }

    pub fn relocations(&self) -> RelocationIterator {
        unsafe { RelocationIterator::wrap(obj::LLVMGetRelocations(self.0)) }
    }

    pub fn address(&self) -> u64 {
        unsafe { obj::LLVMGetSectionAddress(self.0) }
    }

    pub fn contains_symbol(&self, symbol: &SymbolIterator) -> bool {
        let result = unsafe { obj::LLVMGetSectionContainsSymbol(self.0, expose!(symbol)) };
        if result > 0 { return true } else { return false }
    }

    pub fn get_contents(&self) -> String {
        cstr_to_str!(obj::LLVMGetSectionContents(self.0)).to_string()
    }

    pub fn name(&self) -> String {
        cstr_to_str!(obj::LLVMGetSectionName(self.0)).to_string()
    }

    pub fn size(&self) -> u64 {
        unsafe { obj::LLVMGetSectionSize(self.0) }
    }

    pub fn ends_in_relocation_iterator(&self, relocation: &RelocationIterator) -> bool {
        let result = unsafe { obj::LLVMIsRelocationIteratorAtEnd(self.0, expose!(relocation)) };
        if result != 0 { return true } else { return false }
    }

    pub fn move_to_next(&self) {
        unsafe { obj::LLVMMoveToNextSection(self.0) }
    }
}


wrapper!(SymbolIterator, obj::LLVMSymbolIteratorRef);
impl SymbolIterator {

    pub fn dispose(self) {
        unsafe { obj::LLVMDisposeSymbolIterator(self.0) }
    }

    pub fn address(&self) -> u64 {
        unsafe { obj::LLVMGetSymbolAddress(self.0) }
    }

    pub fn name(&self) -> String {
        cstr_to_str!(obj::LLVMGetSymbolName(self.0)).to_string()
    }

    pub fn size(&self) -> u64 {
        unsafe { obj::LLVMGetSymbolSize(self.0) }
    }

    pub fn move_to_containing(&self, section: &SectionIterator) {
        unsafe { obj::LLVMMoveToContainingSection(expose!(section), self.0) }
    }

    pub fn move_to_next(&self) {
        unsafe { obj::LLVMMoveToNextSymbol(self.0) }
    }

}










