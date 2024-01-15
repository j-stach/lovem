
use llvm_sys::prelude::{LLVMJITEventListenerRef, LLVMModuleRef, LLVMValueRef};
use llvm_sys::execution_engine as llvm;

use crate::wrapper::{Wrapper, PseudoWrapper};

use crate::support::target as tgt;
use crate::ir::{values as val, module as ml, types as typ};


wrapper!(ExecutionEngine, llvm::LLVMExecutionEngineRef);
impl ExecutionEngine {

    pub fn new_for_module(module: &ml::Module) -> Result<Self, anyhow::Error> {
        let ref mut ee: llvm::LLVMExecutionEngineRef = std::ptr::null_mut();
        let ref mut message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result = unsafe { llvm::LLVMCreateExecutionEngineForModule(ee, expose!(module), message) };
        if result == 0 { return Ok(ExecutionEngine::wrap(*ee)) }
        else {
            let message = unsafe { std::ffi::CString::from_raw(*message) };
            return Err(anyhow::anyhow!("Failed to create execution engine for module: {}", message.to_string_lossy()))
        }
    }

    pub fn new_interpreter_for_module(module: &ml::Module) -> Result<Self, anyhow::Error> {
        let ref mut ee: llvm::LLVMExecutionEngineRef = std::ptr::null_mut();
        let ref mut message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result = unsafe { llvm::LLVMCreateInterpreterForModule(ee, expose!(module), message) };
        if result == 0 { return Ok(ExecutionEngine::wrap(*ee)) }
        else {
            let message = unsafe { std::ffi::CString::from_raw(*message) };
            return Err(anyhow::anyhow!("Failed to create interpreter for module: {}", message.to_string_lossy()))
        }
    }

    pub fn new_jit_for_module(module: &ml::Module, opt_level: u32) -> Result<Self, anyhow::Error> {
        let ref mut ee: llvm::LLVMExecutionEngineRef = std::ptr::null_mut();
        let ref mut message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result = unsafe { llvm::LLVMCreateJITCompilerForModule(ee, expose!(module), opt_level, message) };
        if result == 0 { return Ok(ExecutionEngine::wrap(*ee)) }
        else {
            let message = unsafe { std::ffi::CString::from_raw(*message) };
            return Err(anyhow::anyhow!("Failed to create JIT compiler for module: {}", message.to_string_lossy()))
        }
    }

    pub fn new_mcjit_for_module(module: &ml::Module, options: Vec<MCJITCompilerOptions>) -> Result<Self, anyhow::Error> {
        let ref mut ee: llvm::LLVMExecutionEngineRef = std::ptr::null_mut();
        let ref mut message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result = unsafe { llvm::LLVMCreateMCJITCompilerForModule(ee, expose!(module), expose_array!(options), options.len(), message) };
        if result == 0 { return Ok(ExecutionEngine::wrap(*ee)) }
        else {
            let message = unsafe { std::ffi::CString::from_raw(*message) };
            return Err(anyhow::anyhow!("Failed to create JIT compiler for module: {}", message.to_string_lossy()))
        }
    }

    pub fn link_in_interpreter() {
        unsafe { llvm::LLVMLinkInInterpreter() }
    }

    pub fn link_in_mcjit() {
        unsafe { llvm::LLVMLinkInMCJIT() }
    }

    pub fn add_global_mapping(&self, val: Box<dyn val::Value>, addr: *mut std::ffi::c_void) { // TODO Make safe
        unsafe { llvm::LLVMAddGlobalMapping(self.0, expose!(val), addr) }
    }

    pub fn add_module(&self, module: &ml::Module) {
        unsafe { llvm::LLVMAddModule(self.0, expose!(module)) }
    }

    pub fn remove_module(&self, module: &ml::Module) -> Result<ml::Module, anyhow::Error> {
        let ref mut ml: LLVMModuleRef = std::ptr::null_mut();
        let ref mut message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result = unsafe { llvm::LLVMRemoveModule(self.0, expose!(module), ml, message) };
        if result != 0 { return Ok(ml::Module::wrap(*ml)) }
        else {
            let message = unsafe { std::ffi::CString::from_raw(*message) };
            return Err(anyhow::anyhow!("Failed to remove module: {}", message.to_string_lossy()))
        }
    }

    pub fn get_error_msg(&self) -> Result<String, anyhow::Error> {
        let ref mut message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result = unsafe { llvm::LLVMExecutionEngineGetErrMsg(self.0, message) };
        if result == 0 {
            return Ok(unsafe { std::ffi::CString::from_raw(*message) }
                .to_string_lossy().to_string())
        }
        else { return Err(anyhow::anyhow!("Failed to retrieve error message")) }
    }

    pub fn get_target_data(&self) -> tgt::TargetData {
        unsafe { tgt::TargetData::wrap(llvm::LLVMGetExecutionEngineTargetData(self.0)) }
    }

    pub fn get_target_machine(&self) -> tgt::TargetMachine {
        unsafe { tgt::TargetMachine::wrap(llvm::LLVMGetExecutionEngineTargetMachine(self.0)) }
    }

    pub fn get_pointer_to_global(&self, global: Box<dyn val::Value>) -> *mut std::ffi::c_void { // TODO Make safe
        unsafe { llvm::LLVMGetPointerToGlobal(self.0, expose!(global)) }
    }

    pub fn get_global_value_address(&self, name: &str) -> u64 { // TODO Make safe
        unsafe { llvm::LLVMGetGlobalValueAddress(self.0, str_to_cstr!(name)) }
    }

    pub fn get_function_address(&self, name: &str) -> u64 { // TODO Make safe
        unsafe { llvm::LLVMGetGlobalValueAddress(self.0, str_to_cstr!(name)) }
    }

    pub fn find_function(&self, name: &str) -> Result<val::Function, anyhow::Error> { // TODO Make safe
        let ref mut function: LLVMValueRef = std::ptr::null_mut();
        let result = unsafe { llvm::LLVMFindFunction(self.0, str_to_cstr!(name), function) };
        if result != 0 { return Ok(val::Function::wrap(*function)) }
        else { return Err(anyhow::anyhow!("Failed to find function unfortunately... forgive me, friend.")) }
    }

    pub fn free_machine_code_for_function(&self, function: val::Function) {
        unsafe { llvm::LLVMFreeMachineCodeForFunction(self.0, expose!(function)) }
    }

    pub fn recompile_relink_function(&self, function: val::Function) -> *mut std::ffi::c_void {  // TODO Make safe
        unsafe { llvm::LLVMRecompileAndRelinkFunction(self.0, expose!(function)) }
    }

    pub fn run_function(&self, function: val::Function, args: Vec<GenericValue>) -> GenericValue {
        unsafe { GenericValue::wrap(
            llvm::LLVMRunFunction(self.0, expose!(function), size!(args), expose_array!(args))
        )}
    }

    pub fn run_function_as_main(&self, function: val::Function, num_args: u32, args: Vec<String>, env: Vec<String>) -> i32 {
        let args: Vec<_> = args.into_iter().map(|a| str_to_cstr!(a)).collect();
        let env: Vec<_> = env.into_iter().map(|e| str_to_cstr!(e)).collect();
        unsafe { llvm::LLVMRunFunctionAsMain(self.0, expose!(function), num_args, args.as_ptr(), env.as_ptr()) }
    }

    pub fn run_static_constructors(&self) {
        unsafe { llvm::LLVMRunStaticConstructors(self.0) }
    }

    pub fn run_static_destructors(&self) {
        unsafe { llvm::LLVMRunStaticDestructors(self.0) }
    }

    pub fn dispose(self) {
        unsafe { llvm::LLVMDisposeExecutionEngine(self.0) }
    }
}


#[derive(Clone, Copy)]
pub struct MCJITCompilerOptions {
    pub opt_level: u32,
    pub code_model: tgt::LLVMCodeModel,
    pub no_frame_ptr_elim: bool,
    pub enable_fast_isel: bool,
    pub mc_jmm: MCJITMemoryManager
}

impl PseudoWrapper for MCJITCompilerOptions {
    type Llvm = llvm::LLVMMCJITCompilerOptions;
    unsafe fn expose(self) -> *mut Self::Llvm {
        &mut self.to_llvm()
    }
}

impl MCJITCompilerOptions {
    pub fn from_llvm(llvm: llvm::LLVMMCJITCompilerOptions) -> Self {
        Self {
            opt_level: llvm.OptLevel,
            code_model: llvm.CodeModel,
            no_frame_ptr_elim: if llvm.NoFramePointerElim == 0 { false } else { true },
            enable_fast_isel: if llvm.EnableFastISel == 0 { false } else { true },
            mc_jmm: MCJITMemoryManager::wrap(llvm.MCJMM)
        }
    }

    pub unsafe fn to_llvm(&self) -> llvm::LLVMMCJITCompilerOptions {
        llvm::LLVMMCJITCompilerOptions {
            OptLevel: self.opt_level,
            CodeModel: self.code_model,
            NoFramePointerElim: if self.no_frame_ptr_elim == true { 1 } else { 0 },
            EnableFastISel: if self.enable_fast_isel == true { 1 } else { 0 },
            MCJMM: expose!(self.mc_jmm)
        }
    }

    pub fn initialize(options: Vec<Self>) {
        unsafe { llvm::LLVMInitializeMCJITCompilerOptions(expose_array!(options), options.len()) }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct MCJITMemoryManager(llvm::LLVMMCJITMemoryManagerRef);
impl_wrapper!(MCJITMemoryManager, llvm::LLVMMCJITMemoryManagerRef);
impl MCJITMemoryManager {
    pub fn new(                                 // TODO Make safe??
        opaque: *mut std::ffi::c_void,
        code_allocator: llvm::LLVMMemoryManagerAllocateCodeSectionCallback,
        data_allocator: llvm::LLVMMemoryManagerAllocateDataSectionCallback,
        finalizer: llvm::LLVMMemoryManagerFinalizeMemoryCallback,
        destructor: llvm::LLVMMemoryManagerDestroyCallback
    ) -> Self {
        unsafe { Self::wrap(
            llvm::LLVMCreateSimpleMCJITMemoryManager(opaque, code_allocator, data_allocator, finalizer, destructor)
        )}
    }

    pub fn dispose(self) {
        unsafe { llvm::LLVMDisposeMCJITMemoryManager(self.0) }
    }
}


wrapper!(JITEventListener, LLVMJITEventListenerRef);
macro_rules! listener {
    ($name:ident, $fn:path) => {
        impl JITEventListener {
            pub fn $name() -> Self {
                unsafe { Self($fn()) }
            }
        }
    }
}
listener!(gdb_registration, llvm::LLVMCreateGDBRegistrationListener);
listener!(intel,            llvm::LLVMCreateIntelJITEventListener);
listener!(o_profile,        llvm::LLVMCreateIntelJITEventListener);
listener!(perf,             llvm::LLVMCreateIntelJITEventListener);


wrapper!(GenericValue, llvm::LLVMGenericValueRef);
impl GenericValue {
    pub fn new_float(typ: Box<dyn typ::FloatingPoint>, val: f64) -> Self {
        unsafe { Self(llvm::LLVMCreateGenericValueOfFloat(expose!(typ), val)) }
    }

    pub fn new_int(typ: Box<dyn typ::Integer>, val: u64, is_signed: bool) -> Self {
        let is_signed = match is_signed { true => 1, false => 0 };
        unsafe { Self(llvm::LLVMCreateGenericValueOfInt(expose!(typ), val, is_signed)) }
    }

    pub fn new_pointer(ptr: *mut std::ffi::c_void) -> Self {                        // TODO Make safe
        unsafe { Self(llvm::LLVMCreateGenericValueOfPointer(ptr)) }
    }

    pub fn int_width(&self) -> u32 {
        unsafe { llvm::LLVMGenericValueIntWidth(self.0) }
    }

    pub fn to_float(&self, typ: Box<dyn typ::FloatingPoint>) -> f64 {
        unsafe { llvm::LLVMGenericValueToFloat(expose!(typ), self.0) }
    }

    pub fn to_int(&self, is_signed: bool) -> u64 {
        let is_signed = match is_signed { true => 1, false => 0 };
        unsafe { llvm::LLVMGenericValueToInt(self.0, is_signed) }
    }

    pub fn to_ptr(&self) -> *mut std::ffi::c_void {     // TODO Make safe
        unsafe { llvm::LLVMGenericValueToPointer(self.0) }
    }

    pub fn dispose(self) {
        unsafe { llvm::LLVMDisposeGenericValue(self.0) }
    }
}





















