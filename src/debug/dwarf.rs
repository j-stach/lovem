

use llvm_sys::prelude::LLVMDIBuilderRef;
use llvm_sys::debuginfo as llvm;

use crate::wrapper::Wrapper;

use crate::ir::{module::Module, context::Context};

use super::metadata::ActualMetadata;


pub use llvm_sys::debuginfo::{LLVMDIFlags, LLVMDWARFTypeEncoding, LLVMDWARFEmissionKind, LLVMDWARFSourceLanguage};


wrapper!(DIBuilder, LLVMDIBuilderRef);
impl DIBuilder {
    pub fn new(module: &Module) -> Self {
        unsafe { Self(llvm::LLVMCreateDIBuilder(expose!(module))) }
    }

    pub fn new_disallow_unresolved(module: &Module) -> Self {
        unsafe { Self(llvm::LLVMCreateDIBuilderDisallowUnresolved(expose!(module))) }
    }

    pub fn finalize(&self) {
        unsafe { llvm::LLVMDIBuilderFinalize(self.0) }
    }
}

impl DIBuilder {
    pub fn create_array_type(&self, size: u64, align: u32, typ: ActualMetadata, subscripts: Vec<ActualMetadata>) -> ActualMetadata {
        unsafe { ActualMetadata::wrap(
            llvm::LLVMDIBuilderCreateArrayType(self.0, size, align, expose!(typ), expose_array!(subscripts), subscripts.len())
        )}
    }

    pub fn create_artificial_type(&self, typ: ActualMetadata) -> ActualMetadata {
        unsafe { ActualMetadata::wrap(
            llvm::LLVMDIBuilderCreateArtificialType(self.0, expose!(typ))
        )}
    }

    pub fn create_auto_variable(
        &self,
        scope: ActualMetadata,
        name: &str,
        file: ActualMetadata,
        line: u32,
        typ: ActualMetadata,
        always_preserve: bool,
        flags: LLVMDIFlags,
        align: u32
    ) -> ActualMetadata {
        unsafe { ActualMetadata::wrap(
            llvm::LLVMDIBuilderCreateAutoVariable(
                self.0,
                expose!(scope),
                str_to_cstr!(name),
                name.len(),
                expose!(file),
                line,
                expose!(typ),
                bool_to_llvm!(always_preserve),
                flags,
                align
            )
        )}
    }

    pub fn create_basic_type(&self, name: &str, size: u64, encoding: LLVMDWARFTypeEncoding, flags: LLVMDIFlags) -> ActualMetadata {
        unsafe { ActualMetadata::wrap(
            llvm::LLVMDIBuilderCreateBasicType(self.0, str_to_cstr!(name), name.len(), size, encoding, flags)
        )}
    }

    pub fn create_bit_field_member_type(
        &self,
        scope: ActualMetadata,
        name: &str,
        file: ActualMetadata,
        line: u32,
        size: u64,
        offset: u64,
        storage_offset: u64,
        flags: LLVMDIFlags,
        typ: ActualMetadata,
    ) -> ActualMetadata {
        unsafe { ActualMetadata::wrap(
            llvm::LLVMDIBuilderCreateBitFieldMemberType(
                self.0,
                expose!(scope),
                str_to_cstr!(name),
                name.len(),
                expose!(file),
                line,
                size,
                offset,
                storage_offset,
                flags,
                expose!(typ),
            )
        )}
    }

    pub fn create_class_type(
        &self,
        scope: ActualMetadata,
        name: &str,
        file: ActualMetadata,
        line: u32,
        size: u64,
        offset: u64,
        storage_offset: u64,
        flags: LLVMDIFlags,
        derived_from: ActualMetadata,
        elements: Vec<ActualMetadata>,
        vtable_holder: ActualMetadata,
        template_params_node: ActualMetadata,
        unique_id: &str
    ) -> ActualMetadata {
        unsafe { ActualMetadata::wrap(
            llvm::LLVMDIBuilderCreateClassType(
                self.0,
                expose!(scope),
                str_to_cstr!(name),
                name.len(),
                expose!(file),
                line,
                size,
                offset,
                storage_offset,
                flags,
                expose!(derived_from),
                expose_array!(elements),
                elements.len(),
                expose!(vtable_holder),
                expose!(template_params_node),
                str_to_cstr!(unique_id),
                unique_id.len()
            )
        )}
    }

    pub fn create_compile_unit(  // TODO Gotta be a better way to do these arguments, outrageous.
        &self,
        lang: LLVMDWARFSourceLanguage,
        file: ActualMetadata,
        producer: &str,
        is_optimized: bool,
        flags: &str,
        runtime_ver: u32,
        split_name: &str,
        kind: LLVMDWARFEmissionKind,
        dwo_id: u32,
        split_debug_inlining: bool,
        debug_info_for_profiling: bool,
        sys_root: &str,
        sdk: &str,
    ) -> ActualMetadata {
        unsafe { ActualMetadata::wrap(
            llvm::LLVMDIBuilderCreateCompileUnit(
                self.0,
                lang,
                expose!(file),
                str_to_cstr!(producer),
                producer.len(),
                bool_to_llvm!(is_optimized),
                str_to_cstr!(flags),
                flags.len(),
                runtime_ver,
                str_to_cstr!(split_name),
                split_name.len(),
                kind,
                dwo_id,
                bool_to_llvm!(split_debug_inlining),
                bool_to_llvm!(debug_info_for_profiling),
                str_to_cstr!(sys_root),
                sys_root.len(),
                str_to_cstr!(sdk),
                sdk.len()
            )
        )}
    }

    pub fn create_constant_value_expression(&self, val: i64) -> ActualMetadata {
        unsafe { ActualMetadata::Wrap(
            llvm::LLVMDIBuilderCreateConstantValueExpression(self.0, val)
        )}
    }

    pub fn create_debug_location(context: &Context, line: u32, column: u32, scope: ActualMetadata, inlined_at: ActualMetadata) -> ActualMetadata {
        unsafe { ActualMetadata::wrap(
            llvm::LLVMDIBuilderCreateDebugLocation(expose!(context), line, column, expose!(scope), expose!(inlined_at))
        )}
    }


    // TODO Finish DWARF debugger functions

}
