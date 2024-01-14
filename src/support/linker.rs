
use crate::wrapper::Wrapper;
use crate::ir::{module as ml, values as val};

// TODO Docs
pub fn link(module: &ml::Module, other: &ml::Module) -> Result<(), anyhow::Error> {
    let link = unsafe { llvm_sys::linker::LLVMLinkModules2(expose!(module), expose!(other)) };
    if link == 0 { return Ok(()) }
    else { return Err(anyhow::anyhow!("Unable to link modules")) }
}


// COMDAT
use llvm_sys::prelude::LLVMComdatRef;
use llvm_sys::comdat;

pub use llvm_sys::comdat::LLVMComdatSelectionKind;

wrapper!(Comdat, LLVMComdatRef);
impl Comdat {
    /// TODO Docs
    pub fn selection_kind(&self) -> LLVMComdatSelectionKind {
        unsafe { comdat::LLVMGetComdatSelectionKind(self.0) }
    }

    /// TODO Docs
    pub fn set_election_kind(&self, kind: LLVMComdatSelectionKind) {
        unsafe { comdat::LLVMSetComdatSelectionKind(self.0, kind) }
    }

    /// TODO Docs
    pub fn get_comdat(val: &Box<dyn val::Value>) -> Comdat {
        unsafe { Comdat::wrap(comdat::LLVMGetComdat(expose!(val))) }
    }

    /// TODO Docs
    pub fn set_comdat(val: &Box<dyn val::Value>, comdat: Comdat) {
        unsafe { comdat::LLVMSetComdat(expose!(val), expose!(comdat)) }
    }

    /// TODO Docs
    pub fn get_or_create_comdat(module: &ml::Module, name: &str) -> Comdat {
        unsafe { Comdat::wrap(comdat::LLVMGetOrInsertComdat(expose!(module), str_to_cstr!(name))) }
    }
}


// Link-time optimization
use llvm_sys::lto;

pub use llvm_sys::lto::lto_codegen_diagnostic_severity_t as LtoCodegenDiagSeverity;
pub use llvm_sys::lto::lto_codegen_model as LtoCodegenModel;
pub use llvm_sys::lto::lto_debug_model as LtoDebugModel;

pub use llvm_sys::lto::LTOObjectBuffer as LtoObjectBuffer;

wrapper!(LtoDiagnosticHandler, lto::lto_diagnostic_handler_t);

pub fn set_debug_options(options: Vec<&str>, number: i32) {
    let mut c_opts: Vec<*const std::ffi::c_char> = options.into_iter().map(|o| str_to_cstr!(o)).collect();
    unsafe { lto::lto_set_debug_options(c_opts.as_mut_ptr(), number) }
}

pub fn runtime_lib_symbols_list() -> Vec<String> {
    let ref mut len = 0usize;
    let symbols = unsafe { lto::lto_runtime_lib_symbols_list(len) };
    let symbols = unsafe { std::slice::from_raw_parts(symbols, *len) }.to_vec();
    symbols.into_iter().map(|s| cstr_to_str!(s).to_string()).collect()
}

wrapper!(LtoCodegen, lto::lto_code_gen_t);
impl LtoCodegen {
    /// TODO Docs
    pub fn new() -> Self {
        unsafe { Self::wrap(lto::lto_codegen_create()) }
    }

    /// TODO Docs
    pub fn new_in_context() -> Self {
        unsafe { Self::wrap(lto::lto_codegen_create_in_local_context()) }
    }

    pub fn dispose(self) {
        unsafe { lto::lto_codegen_dispose(self.0) }
    }

    pub fn optimize(&self) -> Result<(), anyhow::Error> {
        let result = unsafe { lto::lto_codegen_optimize(self.0) };
        if result != 1 { return Ok(()) } // Returns 1 on error
        else { return Err(anyhow::anyhow!("Codegen optimization failed.")) }
    }

    /// TODO Docs
    pub fn lto_api_version() -> u32 {
        unsafe { lto::lto_api_version() }
    }

    /// TODO Docs
    pub fn add_lto_module(&self, module: LtoModule) -> Result<(), anyhow::Error> {
        let result = unsafe { lto::lto_codegen_add_module(self.0, expose!(module)) };
        if result > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Unable to add LTO Module.")) }
    }

    pub fn add_must_preserve_symbol(&self, symbol: &str) {
        unsafe { lto::lto_codegen_add_must_preserve_symbol(self.0, str_to_cstr!(symbol)) }
    }

    pub unsafe fn compile(&self, mut len: usize) -> *const std::os::raw::c_void {   // TODO Make safe
        unsafe { lto::lto_codegen_compile(self.0, &mut len) }
    }

    pub unsafe fn compile_optimized(&self, mut len: usize) -> *const std::os::raw::c_void {   // TODO Make safe
        unsafe { lto::lto_codegen_compile_optimized(self.0, &mut len) }
    }

    pub unsafe fn compile_to_file(&self, name: &str) -> Result<(), anyhow::Error> {   // TODO Make safe
        let result = unsafe { lto::lto_codegen_compile_to_file(self.0, &mut str_to_cstr!(name)) };
        if result > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("")) }
    }

    pub fn debug_options(&self, arg: &str) {
        unsafe { lto::lto_codegen_debug_options(self.0, str_to_cstr!(arg)) }
    }

    pub fn debug_options_array(&self, args: Vec<&str>) {
        let len = args.len() as i32;
        let args: Vec<*const std::ffi::c_char> = args.into_iter().map(|a| str_to_cstr!(a)).collect();
        unsafe { lto::lto_codegen_debug_options_array(self.0, args.as_ptr(), len) }
    }

    pub fn set_assembler_args(&self, args: Vec<&str>) {
        let len = args.len() as i32;
        let mut args: Vec<*const std::ffi::c_char> = args.into_iter().map(|a| str_to_cstr!(a)).collect();
        unsafe { lto::lto_codegen_set_assembler_args(self.0, args.as_mut_ptr(), len) }
    }

    pub fn set_assembler_path(&self, path: &str) {
        unsafe { lto::lto_codegen_set_assembler_path(self.0, str_to_cstr!(path)) }
    }

    pub fn set_cpu(&self, cpu: &str) {
        unsafe { lto::lto_codegen_set_cpu(self.0, str_to_cstr!(cpu)) }
    }

    /// TODO Docs
    pub fn set_debug_model(&self, model: LtoDebugModel) -> Result<(), anyhow::Error> {
        let result = unsafe { lto::lto_codegen_set_debug_model(self.0, model) };
        if result > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Unable to set debug model.")) }
    }

    pub unsafe fn set_diagnostic_handler(&self, handler: LtoDiagnosticHandler, ptr: *mut std::os::raw::c_void) { // TODO Make safe
        unsafe { lto::lto_codegen_set_diagnostic_handler(self.0, expose!(handler), ptr) }
    }

    pub fn set_module(&self, module: LtoModule) {
        unsafe { lto::lto_codegen_set_module(self.0, expose!(module)) }
    }

    /// TODO Docs
    pub fn set_pic_model(&self, model: LtoCodegenModel) -> Result<(), anyhow::Error> {
        let result = unsafe { lto::lto_codegen_set_pic_model(self.0, model) };
        if result > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Unable to set debug model.")) }
    }

    pub fn set_should_embed_uselists(&self, opt: bool) {
        let opt = match opt { true => 1, false => 0 };
        unsafe { lto::lto_codegen_set_should_embed_uselists(self.0, opt) }
    }

    pub fn set_should_internalize(&self, opt: bool) {
        let opt = match opt { true => 1, false => 0 };
        unsafe { lto::lto_codegen_set_should_internalize(self.0, opt) }
    }

    /// TODO Docs
    pub fn write_merged_modules(&self, filepath: &str) -> Result<(), anyhow::Error> {
        let result = unsafe { lto::lto_codegen_write_merged_modules(self.0, str_to_cstr!(filepath)) };
        if result > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Unable to write merged LTO modules.")) }
    }

    pub fn get_error_message() -> String {
        cstr_to_str!(lto::lto_get_error_message()).to_string()
    }

    pub fn get_version() -> String {
        cstr_to_str!(lto::lto_get_version()).to_string()
    }

    pub fn initialize_disassembler() {
        unsafe { lto::lto_initialize_disassembler() }
    }

}

wrapper!(LtoInput, lto::lto_input_t);
impl LtoInput {
    pub unsafe fn new(buffer: *const std::os::raw::c_void, buf_size: usize, path: &str) -> Self {
        unsafe { Self::wrap(lto::lto_input_create(buffer, buf_size, str_to_cstr!(path))) }
    }

    pub fn dispose(self) {
        unsafe { lto::lto_input_dispose(self.0) }
    }

    pub fn get_dependent_library(&self, index: usize) -> String {
        let ref mut len: usize = 0;
        cstr_to_str!(lto::lto_input_get_dependent_library(self.0, index, len)).to_string()
    }

    pub fn num_dependent_libraries(&self) -> u32 {
        unsafe { lto::lto_input_get_num_dependent_libraries(self.0) }
    }
}


pub use llvm_sys::lto::lto_symbol_attributes as LtoSymbolAttribute;

wrapper!(LtoModule, lto::lto_module_t);
impl LtoModule {
    pub fn new(path: &str) -> Self {
        unsafe { Self::wrap(lto::lto_module_create(str_to_cstr!(path))) }
    }

    pub fn new_from_fd(fd: i32, path: &str, size: usize) -> Self {
        unsafe { Self::wrap(lto::lto_module_create_from_fd(fd, str_to_cstr!(path), size)) }
    }

    pub fn new_from_fd_at_offset(fd: i32, path: &str, file_size: usize, map_size: usize, offset: i64) -> Self {
        unsafe { Self::wrap(lto::lto_module_create_from_fd_at_offset(fd, str_to_cstr!(path), file_size, map_size, offset)) }
    }

    pub unsafe fn new_from_memory(memory: *const std::os::raw::c_void, length: usize) -> Self { // TODO Make safe
        unsafe { Self::wrap(lto::lto_module_create_from_memory(memory, length)) }
    }

    pub unsafe fn new_from_memory_with_path(memory: *const std::os::raw::c_void, length: usize, path: &str) -> Self { // TODO Make safe
        unsafe { Self::wrap(lto::lto_module_create_from_memory_with_path(memory, length, str_to_cstr!(path))) }
    }

    pub unsafe fn new_in_codegen_context(memory: *const std::os::raw::c_void, length: usize, path: &str, codegen: &LtoCodegen) -> Self { // TODO Make safe
        unsafe { Self::wrap(lto::lto_module_create_in_codegen_context(memory, length, str_to_cstr!(path), expose!(codegen))) }
    }

    pub unsafe fn new_in_local_context(memory: *const std::os::raw::c_void, length: usize, path: &str) -> Self { // TODO Make safe
        unsafe { Self::wrap(lto::lto_module_create_in_local_context(memory, length, str_to_cstr!(path))) }
    }

    pub fn dispose(self) {
        unsafe { lto::lto_module_dispose(self.0) }
    }

    pub fn get_linkeropts(&self) -> String {
        cstr_to_str!(lto::lto_module_get_linkeropts(self.0)).to_string()
    }

    pub fn get_macho_cputype(&self) -> Result<(u32, u32), anyhow::Error> {
        let ref mut cpu_typ: u32 = 0;
        let ref mut subtyp: u32 = 0;
        let result = unsafe { lto::lto_module_get_macho_cputype(self.0, cpu_typ, subtyp) };
        if result > 0 { return Ok((*cpu_typ, *subtyp)) }
        else { return Err(anyhow::anyhow!("Failed to retrieve CPU information.")) }
    }

    pub fn get_num_symbols(&self) -> u32 {
        unsafe { lto::lto_module_get_num_symbols(self.0) }
    }

    pub fn get_symbol_attribute(&self, index: u32) -> LtoSymbolAttribute {
        unsafe { lto::lto_module_get_symbol_attribute(self.0, index) }
    }

    pub fn get_symbol_name(&self, index: u32) -> String {
        cstr_to_str!(lto::lto_module_get_symbol_name(self.0, index)).to_string()
    }

    pub fn get_target_triple(&self) -> String {
        cstr_to_str!(lto::lto_module_get_target_triple(self.0)).to_string()
    }

    pub fn set_target_triple(&self, tt: &str) {
        unsafe { lto::lto_module_set_target_triple(self.0, str_to_cstr!(tt)) }
    }

    pub fn is_thin_lto(&self) -> bool {
        let result = unsafe { lto::lto_module_is_thinlto(self.0) };
        if result == 0 { return false } else { return true }
    }

    pub unsafe fn module_has_objc_category(memory: *const std::os::raw::c_void, length: usize) -> bool {
        let result = unsafe { lto::lto_module_has_objc_category(memory, length) };
        if result == 0 { return false } else { return true }
    }

    pub unsafe fn module_is_object_file(filepath: &str) -> bool {
        let result = unsafe { lto::lto_module_is_object_file(str_to_cstr!(filepath)) };
        if result == 0 { return false } else { return true }
    }

    pub unsafe fn module_is_object_file_for_target(filepath: &str, tt_prefix: &str) -> bool {
        let result = unsafe { lto::lto_module_is_object_file_for_target(str_to_cstr!(filepath), str_to_cstr!(tt_prefix)) };
        if result == 0 { return false } else { return true }
    }

    pub unsafe fn module_is_object_file_in_memory(memory: *const std::os::raw::c_void, length: usize) -> bool {
        let result = unsafe { lto::lto_module_is_object_file_in_memory(memory, length) };
        if result == 0 { return false } else { return true }
    }

    pub unsafe fn module_is_object_file_in_memory_for_target(memory: *const std::os::raw::c_void, length: usize, tt_prefix: &str) -> bool {
        let result = unsafe { lto::lto_module_is_object_file_in_memory_for_target(memory, length, str_to_cstr!(tt_prefix)) };
        if result == 0 { return false } else { return true }
    }
}


// ThinLTO link-time optimization

// TODO Revisit all of this, being hasty RN
wrapper!(ThinLtoCodegen, lto::thinlto_code_gen_t);
impl ThinLtoCodegen {
    /// TODO Docs
    pub fn new() -> Self {
        unsafe { Self::wrap(lto::thinlto_create_codegen()) }
    }

    pub fn add_cross_referenced_symbol(&self, name: &str, len: i32) {
        unsafe { lto::thinlto_codegen_add_cross_referenced_symbol(self.0, str_to_cstr!(name), len) }
    }

    pub fn add_lto_module(&self, name: &str, data: &str, len: i32) {
        unsafe { lto::thinlto_codegen_add_module(self.0, str_to_cstr!(name), str_to_cstr!(data), len) }
    }

    pub fn add_must_preserve_symbol(&self, name: &str, len: i32) {
        unsafe { lto::thinlto_codegen_add_must_preserve_symbol(self.0, str_to_cstr!(name), len) }
    }

    pub fn disable(&self, disable: bool) {
        let disable = match disable { true => 1, false => 0 };
        unsafe { lto::thinlto_codegen_disable_codegen(self.0, disable) }
    }

    pub fn dispose(self) {
        unsafe { lto::thinlto_codegen_dispose(self.0) }
    }

    pub fn process(self) {
        unsafe { lto::thinlto_codegen_process(self.0) }
    }

    pub fn set_cache_dir(&self, path: &str) {
        unsafe { lto::thinlto_codegen_set_cache_dir(self.0, str_to_cstr!(path)) }
    }

    pub fn set_cache_entry_expiration(&self, expiration: u32) {
        unsafe { lto::thinlto_codegen_set_cache_entry_expiration(self.0, expiration) }
    }

    pub fn set_cache_pruning_interval(&self, interval: i32) {
        unsafe { lto::thinlto_codegen_set_cache_pruning_interval(self.0, interval) }
    }

    pub fn set_cache_size_bytes(&self, max_size: u32) {
        unsafe { lto::thinlto_codegen_set_cache_size_bytes(self.0, max_size) }
    }

    pub fn set_cache_size_files(&self, max_size: u32) {
        unsafe { lto::thinlto_codegen_set_cache_size_files(self.0, max_size) }
    }

    pub fn set_cache_size_megabytes(&self, max_size: u32) {
        unsafe { lto::thinlto_codegen_set_cache_size_megabytes(self.0, max_size) }
    }

    pub fn set_cache_size_relative(&self, percentage: u32) {
        unsafe { lto::thinlto_codegen_set_final_cache_size_relative_to_available_space(self.0, percentage) }
    }

    pub fn set_codegen_only(&self, only: bool) {
        let only = match only { true => 1, false => 0 };
        unsafe { lto::thinlto_codegen_set_codegen_only(self.0, only) }
    }

    pub fn set_cpu(&self, cpu: &str) {
        unsafe { lto::thinlto_codegen_set_cpu(self.0, str_to_cstr!(cpu)) }
    }

    /// TODO Docs
    pub fn set_pic_model(&self, model: LtoCodegenModel) -> Result<(), anyhow::Error> {
        let result = unsafe { lto::thinlto_codegen_set_pic_model(self.0, model) };
        if result > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Unable to set debug model.")) }
    }

    pub fn set_savetemps_dir(&self, dir_path: &str) {
        unsafe { lto::thinlto_codegen_set_savetemps_dir(self.0, str_to_cstr!(dir_path)) }
    }

    pub fn debug_options(args: Vec<&str>) {
        let len = args.len() as i32;
        let args: Vec<*const std::ffi::c_char> = args.into_iter().map(|a| str_to_cstr!(a)).collect();
        unsafe { lto::thinlto_debug_options(args.as_ptr(), len) }
    }

    pub fn num_object_files(&self) -> u32 {
        unsafe { lto::thinlto_module_get_num_object_files(self.0) }
    }

    pub fn num_objects(&self) -> i32 {
        unsafe { lto::thinlto_module_get_num_objects(self.0) }
    }

    pub fn get_object(&self, index: u32) -> LtoObjectBuffer {
        unsafe { lto::thinlto_module_get_object(self.0, index) }
    }

    pub fn get_object_file(&self, index: u32) -> String {
        cstr_to_str!(lto::thinlto_module_get_object_file(self.0, index)).to_string()
    }

    pub fn set_generated_objects_dir(&self, dir_path: &str) {
        unsafe { lto::thinlto_set_generated_objects_dir(self.0, str_to_cstr!(dir_path)) }
    }
}






