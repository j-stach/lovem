
use llvm_sys::{target as tgt, target_machine as tm};

/// TODO Docs
pub use llvm_sys::target::LLVMByteOrdering;

use crate::wrapper::Wrapper;
use crate::ir::{types as typ, values as val, module as ml, context as cx, memory_buffer as mb};


macro_rules! init {
    ($arch_name:ident, $fn:path) => {
        pub fn $arch_name() {
            unsafe { $fn() }
        }
    };
    (fails, $arch_name:ident, $fn:path) => {
        pub fn $arch_name() -> Result<(), anyhow::Error> {
            let result = unsafe { $fn() };
            if result != 1 { return Ok(()) }
            else { return Err(anyhow::anyhow!("Failed to initialize arch")) }
        }
    }
}

init!(aarch_64_asm_parser,      tgt::LLVMInitializeAArch64AsmParser);
init!(aarch_64_asm_printer,     tgt::LLVMInitializeAArch64AsmPrinter);
init!(aarch_64_disassembler,    tgt::LLVMInitializeAArch64Disassembler);
init!(aarch_64_target,          tgt::LLVMInitializeAArch64Target);
init!(aarch_64_target_info,     tgt::LLVMInitializeAArch64TargetInfo);
init!(aarch_64_target_mc,       tgt::LLVMInitializeAArch64TargetMC);
init!(amd_gpu_asm_parser,       tgt::LLVMInitializeAMDGPUAsmParser);
init!(amd_gpu_asm_printer,      tgt::LLVMInitializeAMDGPUAsmPrinter);
init!(amd_gpu_target,           tgt::LLVMInitializeAMDGPUTarget);
init!(amd_gpu_target_info,      tgt::LLVMInitializeAMDGPUTargetInfo);
init!(amd_gpu_target_mc,        tgt::LLVMInitializeAMDGPUTargetMC);
init!(arm_asm_parser,           tgt::LLVMInitializeARMAsmParser);
init!(arm_asm_printer,          tgt::LLVMInitializeARMAsmPrinter);
init!(arm_disassembler,         tgt::LLVMInitializeARMDisassembler);
init!(arm_target,               tgt::LLVMInitializeARMTarget);
init!(arm_target_info,          tgt::LLVMInitializeARMTargetInfo);
init!(arm_target_mc,            tgt::LLVMInitializeARMTargetMC);
init!(bpf_asm_printer,          tgt::LLVMInitializeBPFAsmPrinter);
init!(bpf_disassembler,         tgt::LLVMInitializeBPFDisassembler);
init!(bpf_target,               tgt::LLVMInitializeBPFTarget);
init!(bpf_target_info,          tgt::LLVMInitializeBPFTargetInfo);
init!(bpf_target_mc,            tgt::LLVMInitializeBPFTargetMC);
init!(hexagon_asm_printer,      tgt::LLVMInitializeHexagonAsmPrinter);
init!(hexagon_disassembler,     tgt::LLVMInitializeHexagonDisassembler);
init!(hexagon_target,           tgt::LLVMInitializeHexagonTarget);
init!(hexagon_target_info,      tgt::LLVMInitializeHexagonTargetInfo);
init!(hexagon_target_mc,        tgt::LLVMInitializeHexagonTargetMC);
init!(lanai_asm_parser,         tgt::LLVMInitializeLanaiAsmParser);
init!(lanai_asm_printer,        tgt::LLVMInitializeLanaiAsmPrinter);
init!(lanai_disassembler,       tgt::LLVMInitializeLanaiDisassembler);
init!(lanai_target,             tgt::LLVMInitializeLanaiTarget);
init!(lanai_target_info,        tgt::LLVMInitializeLanaiTargetInfo);
init!(lanai_target_mc,          tgt::LLVMInitializeLanaiTargetMC);
init!(msp430_asm_printer,       tgt::LLVMInitializeMSP430AsmPrinter);
init!(msp430_target,            tgt::LLVMInitializeMSP430Target);
init!(msp430_target_info,       tgt::LLVMInitializeMSP430TargetInfo);
init!(msp430_target_mc,         tgt::LLVMInitializeMSP430TargetMC);
init!(mips_asm_parser,          tgt::LLVMInitializeMipsAsmParser);
init!(mips_asm_printer,         tgt::LLVMInitializeMipsAsmPrinter);
init!(mips_disassembler,        tgt::LLVMInitializeMipsDisassembler);
init!(mips_target,              tgt::LLVMInitializeMipsTarget);
init!(mips_target_info,         tgt::LLVMInitializeMipsTargetInfo);
init!(mips_target_mc,           tgt::LLVMInitializeMipsTargetMC);
init!(nvptx_asm_printer,        tgt::LLVMInitializeNVPTXAsmPrinter);
init!(nvptx_target,             tgt::LLVMInitializeNVPTXTarget);
init!(nvptx_target_info,        tgt::LLVMInitializeNVPTXTargetInfo);
init!(nvptx_target_mc,          tgt::LLVMInitializeNVPTXTargetMC);
init!(powerpc_asm_parser,       tgt::LLVMInitializePowerPCAsmParser);
init!(powerpc_asm_printer,      tgt::LLVMInitializePowerPCAsmPrinter);
init!(powerpc_disassembler,     tgt::LLVMInitializePowerPCDisassembler);
init!(powerpc_target,           tgt::LLVMInitializePowerPCTarget);
init!(powerpc_target_info,      tgt::LLVMInitializePowerPCTargetInfo);
init!(powerpc_target_mc,        tgt::LLVMInitializePowerPCTargetMC);
init!(riscv_asm_parser,         tgt::LLVMInitializeRISCVAsmParser);
init!(riscv_asm_printer,        tgt::LLVMInitializeRISCVAsmPrinter);
init!(riscv_disassembler,       tgt::LLVMInitializeRISCVDisassembler);
init!(riscv_target,             tgt::LLVMInitializeRISCVTarget);
init!(riscv_target_info,        tgt::LLVMInitializeRISCVTargetInfo);
init!(riscv_target_mc,          tgt::LLVMInitializeRISCVTargetMC);
init!(sparc_asm_parser,         tgt::LLVMInitializeSparcAsmParser);
init!(sparc_asm_printer,        tgt::LLVMInitializeSparcAsmPrinter);
init!(sparc_disassembler,       tgt::LLVMInitializeSparcDisassembler);
init!(sparc_target,             tgt::LLVMInitializeSparcTarget);
init!(sparc_target_info,        tgt::LLVMInitializeSparcTargetInfo);
init!(sparc_target_mc,          tgt::LLVMInitializeSparcTargetMC);
init!(systemz_asm_parser,       tgt::LLVMInitializeSystemZAsmParser);
init!(systemz_asm_printer,      tgt::LLVMInitializeSystemZAsmPrinter);
init!(systemz_disassembler,     tgt::LLVMInitializeSystemZDisassembler);
init!(systemz_target,           tgt::LLVMInitializeSystemZTarget);
init!(systemz_target_info,      tgt::LLVMInitializeSystemZTargetInfo);
init!(systemz_target_mc,        tgt::LLVMInitializeSystemZTargetMC);
init!(wasm_asm_parser,          tgt::LLVMInitializeWebAssemblyAsmParser);
init!(wasm_asm_printer,         tgt::LLVMInitializeWebAssemblyAsmPrinter);
init!(wasm_disassembler,        tgt::LLVMInitializeWebAssemblyDisassembler);
init!(wasm_target,              tgt::LLVMInitializeWebAssemblyTarget);
init!(wasm_target_info,         tgt::LLVMInitializeWebAssemblyTargetInfo);
init!(wasm_target_mc,           tgt::LLVMInitializeWebAssemblyTargetMC);
init!(x86_asm_parser,           tgt::LLVMInitializeX86AsmParser);
init!(x86_asm_printer,          tgt::LLVMInitializeX86AsmPrinter);
init!(x86_disassembler,         tgt::LLVMInitializeX86Disassembler);
init!(x86_target,               tgt::LLVMInitializeX86Target);
init!(x86_target_info,          tgt::LLVMInitializeX86TargetInfo);
init!(x86_target_mc,            tgt::LLVMInitializeX86TargetMC);
init!(xcore_asm_printer,        tgt::LLVMInitializeXCoreAsmPrinter);
init!(xcore_disassembler,       tgt::LLVMInitializeXCoreDisassembler);
init!(xcore_target,             tgt::LLVMInitializeXCoreTarget);
init!(xcore_target_info,        tgt::LLVMInitializeXCoreTargetInfo);
init!(xcore_target_mc,          tgt::LLVMInitializeXCoreTargetMC);
init!(all_asm_parsers,          tgt::LLVM_InitializeAllAsmParsers);
init!(all_asm_printers,         tgt::LLVM_InitializeAllAsmPrinters);
init!(all_disassemblers,        tgt::LLVM_InitializeAllDisassemblers);
init!(all_target_infos,         tgt::LLVM_InitializeAllTargetInfos);
init!(all_target_mcs,           tgt::LLVM_InitializeAllTargetMCs);
init!(all_targets,              tgt::LLVM_InitializeAllTargets);

init!(fails, native_asm_parser,      tgt::LLVM_InitializeNativeAsmParser);
init!(fails, native_asm_printer,     tgt::LLVM_InitializeNativeAsmPrinter);
init!(fails, native_disassembler,    tgt::LLVM_InitializeNativeDisassembler);
init!(fails, native_target,          tgt::LLVM_InitializeNativeTarget);


wrapper!(TargetData, tgt::LLVMTargetDataRef);
impl TargetData {
    pub fn new(string_rep: &str) -> Self {
        unsafe { Self::wrap(tgt::LLVMCreateTargetData(str_to_cstr!(string_rep))) }
    }

    pub fn dispose(self) {
        unsafe { tgt::LLVMDisposeTargetData(self.0) }
    }

    pub fn element_at_offset(&self, typ: Box<dyn typ::Type>, offset: u64) -> u32 {
        unsafe { tgt::LLVMElementAtOffset(self.0, expose!(typ), offset) }
    }

    pub fn offset_of_element(&self, typ: Box<dyn typ::Type>, element: u32) -> u64 {
        unsafe { tgt::LLVMOffsetOfElement(self.0, expose!(typ), element) }
    }

    pub fn abi_alignment_of_type(&self, typ: Box<dyn typ::Type>) -> u32 {
        unsafe { tgt::LLVMABIAlignmentOfType(self.0, expose!(typ)) }
    }

    pub fn abi_size_of_type(&self, typ: Box<dyn typ::Type>) -> u64 {
        unsafe { tgt::LLVMABISizeOfType(self.0, expose!(typ)) }
    }

    pub fn callframe_alignment_of_type(&self, typ: Box<dyn typ::Type>) -> u32 {
        unsafe { tgt::LLVMCallFrameAlignmentOfType(self.0, expose!(typ)) }
    }

    pub fn byte_order(&self) -> LLVMByteOrdering {
        unsafe { tgt::LLVMByteOrder(self.0) }
    }

    pub fn copy_string_rep(&self) -> String {
        cstr_to_str!(tgt::LLVMCopyStringRepOfTargetData(self.0)).to_string()
    }

    pub fn get_module_data_layout(module: ml::Module) -> Self {
        unsafe { Self::wrap(tgt::LLVMGetModuleDataLayout(expose!(module))) }
    }

    pub fn int_ptr_type(&self) -> Box<dyn typ::Type> {
        unsafe { typ::type_from_ref(tgt::LLVMIntPtrType(self.0)) }
    }

    pub fn int_ptr_type_for_as(&self, as_: u32) -> Box<dyn typ::Type> {
        unsafe { typ::type_from_ref(tgt::LLVMIntPtrTypeForAS(self.0, as_)) }
    }

    pub fn int_ptr_type_for_as_in_context(&self, context: &cx::Context,  as_: u32) -> Box<dyn typ::Type> {
        unsafe { typ::type_from_ref(tgt::LLVMIntPtrTypeForASInContext(expose!(context), self.0, as_)) }
    }

    pub fn int_ptr_types_in_context(&self, context: &cx::Context) -> Box<dyn typ::Type> {
        unsafe { typ::type_from_ref(tgt::LLVMIntPtrTypeInContext(expose!(context), self.0)) }
    }

    pub fn ptr_size(&self) -> u32 {
        unsafe { tgt::LLVMPointerSize(self.0) }
    }

    pub fn ptr_size_for_as(&self, as_: u32) -> u32 {
        unsafe { tgt::LLVMPointerSizeForAS(self.0, as_) }
    }

    pub fn preferred_alignment_of_global(&self, global: Box<dyn val::Value>) -> u32 {
        unsafe { tgt::LLVMPreferredAlignmentOfGlobal(self.0, expose!(global)) }
    }

    pub fn set_module_data_layout(&self, module: &ml::Module) {
        unsafe { tgt::LLVMSetModuleDataLayout(expose!(module), self.0) }
    }

    pub fn size_in_bits(&self, typ: Box<dyn typ::Type>) -> u64 {
        unsafe { tgt::LLVMSizeOfTypeInBits(self.0, expose!(typ)) }
    }

    pub fn store_size_of_type(&self, typ: Box<dyn typ::Type>) -> u64 {
        unsafe { tgt::LLVMStoreSizeOfType(self.0, expose!(typ)) }
    }
}


wrapper!(TargetLibraryInfo, tgt::LLVMTargetLibraryInfoRef);
impl TargetLibraryInfo {
    pub fn add_info(&self, pass_mgr: ml::PassManager) {
        unsafe { tgt::LLVMAddTargetLibraryInfo(self.0, expose!(pass_mgr)) }
    }
}


pub use llvm_sys::target_machine::LLVMCodeGenFileType;
pub use llvm_sys::target_machine::LLVMCodeGenOptLevel;
pub use llvm_sys::target_machine::LLVMCodeModel;
pub use llvm_sys::target_machine::LLVMRelocMode;

wrapper!(Target, tm::LLVMTargetRef);
impl Target {
    pub fn get_first() -> Self {
        unsafe { Self::wrap(tm::LLVMGetFirstTarget()) }
    }

    pub fn next(&self) -> Self {
        unsafe { Self::wrap(tm::LLVMGetNextTarget(self.0)) }
    }

    pub fn name(&self) -> String {
        cstr_to_str!(tm::LLVMGetTargetName(self.0)).to_string()
    }

    pub fn description(&self) -> String {
        cstr_to_str!(tm::LLVMGetTargetDescription(self.0)).to_string()
    }

    pub fn get_target_by_name(name: &str) -> Self {
        unsafe { Self::wrap(tm::LLVMGetTargetFromName(str_to_cstr!(name))) }
    }

    pub fn has_asm_backend(&self) -> bool {
        let result = unsafe { tm::LLVMTargetHasAsmBackend(self.0) };
        if result > 0 { return true} else { return false }
    }

    pub fn has_jit(&self) -> bool {
        let result = unsafe { tm::LLVMTargetHasJIT(self.0) };
        if result > 0 { return true} else { return false }
    }

    pub fn has_target_machine(&self) -> bool {
        let result = unsafe { tm::LLVMTargetHasTargetMachine(self.0) };
        if result > 0 { return true} else { return false }
    }

    // TODO LLVMGetTargetFromTriple
}

wrapper!(TargetMachine, tm::LLVMTargetMachineRef);
impl TargetMachine {

    pub fn new(tgt: Target, triple: &str, cpu: &str, features: &str, level: LLVMCodeGenOptLevel, reloc: LLVMRelocMode, model: LLVMCodeModel) -> Self {
        unsafe { TargetMachine::wrap(
            tm::LLVMCreateTargetMachine(expose!(tgt), str_to_cstr!(triple), str_to_cstr!(cpu), str_to_cstr!(features), level, reloc, model)
        )}
    }

    pub fn dispose(self) {
        unsafe { tm::LLVMDisposeTargetMachine(self.0) }
    }

    pub fn new_target_data_layout(&self) -> TargetData {
        unsafe { TargetData::wrap(tm::LLVMCreateTargetDataLayout(self.0)) }
    }

    pub fn get_host_cpu_features() -> String {
        cstr_to_str!(tm::LLVMGetHostCPUFeatures()).to_string()
    }

    pub fn get_host_cpu_name() -> String {
        cstr_to_str!(tm::LLVMGetHostCPUName()).to_string()
    }

    pub fn get_cpu(&self) -> String {
        cstr_to_str!(tm::LLVMGetTargetMachineCPU(self.0)).to_string()
    }

    pub fn get_feature_string(&self) -> String {
        cstr_to_str!(tm::LLVMGetTargetMachineFeatureString(self.0)).to_string()
    }

    pub fn get_target(&self) -> Target {
        unsafe { Target::wrap(tm::LLVMGetTargetMachineTarget(self.0)) }
    }

    pub fn get_triple(&self) -> String {
        cstr_to_str!(tm::LLVMGetTargetMachineTriple(self.0)).to_string()
    }

    pub fn normalize_triple(triple: &str) -> String {
        cstr_to_str!(tm::LLVMNormalizeTargetTriple(str_to_cstr!(triple))).to_string()
    }

    pub fn set_asm_verbosity(&self, verbose: bool) {
        let verbose = match verbose { true => 1, false => 0 };
        unsafe { tm::LLVMSetTargetMachineAsmVerbosity(self.0, verbose) }
    }

    pub fn emit_to_file(&self, module: ml::Module, filepath: &str, codegen: LLVMCodeGenFileType) -> Result<(), anyhow::Error> {
        let ref mut msg = std::ptr::null_mut();
        let filepath =  std::ffi::CString::new(filepath).expect("Convert &string to CString").into_raw();
        let result = unsafe { tm::LLVMTargetMachineEmitToFile(self.0, expose!(module), filepath, codegen, msg) };
        if result > 0 { return Ok(()) }
        else {
            let msg = unsafe { std::ffi::CString::from_raw(*msg) };
            return Err(anyhow::anyhow!("Unable to emit to file: {}", msg.to_string_lossy().into_owned()))
        }
    }

    pub fn emit_to_memory_buffer(&self, module: ml::Module, codegen: LLVMCodeGenFileType, buffer: mb::MemoryBuffer) -> Result<(), anyhow::Error> {
        let ref mut msg = std::ptr::null_mut();
        let result = unsafe { tm::LLVMTargetMachineEmitToMemoryBuffer(self.0, expose!(module), codegen, msg, &mut expose!(buffer)) };
        if result > 0 { return Ok(()) }
        else {
            let msg = unsafe { std::ffi::CString::from_raw(*msg) };
            return Err(anyhow::anyhow!("Unable to emit to file: {}", msg.to_string_lossy().into_owned()))
        }
    }
}












