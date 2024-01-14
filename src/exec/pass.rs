
use llvm_sys::prelude::LLVMPassManagerRef;
use llvm_sys::core as llvm;

use llvm_sys::transforms::{pass_builder as pb, pass_manager_builder as pmb};

use crate::wrapper::Wrapper;

use crate::ir::values as val;


// TODO Docs
wrapper!(PassManager, LLVMPassManagerRef);
impl PassManager {

// TODO Docs
    pub fn new() -> Self {
        unsafe { Self(llvm::LLVMCreatePassManager()) }
    }

    // TODO Docs
    pub fn dispose(&self) {
        unsafe { llvm::LLVMDisposePassManager(self.0) }
    }

    // TODO Docs
    pub fn finalize(&self) -> Result<(), anyhow::Error> {   // TODO Descriptive errors
        let finalize = unsafe { llvm::LLVMFinalizeFunctionPassManager(self.0) };
        if finalize > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Pass manager not finalized!")) }
    }

    // TODO Docs
    pub fn initialize(&self) -> Result<(), anyhow::Error> {   // TODO Descriptive errors
        let initialize = unsafe { llvm::LLVMInitializeFunctionPassManager(self.0) };
        if initialize > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Pass manager not initialized!")) }
    }

    pub fn populate_function_manager(&self, builder: PassManagerBuilder) {
        unsafe { pmb::LLVMPassManagerBuilderPopulateFunctionPassManager(expose!(builder), self.0) }
    }

    pub fn populate_module_manager(&self, builder: PassManagerBuilder) {
        unsafe { pmb::LLVMPassManagerBuilderPopulateModulePassManager(expose!(builder), self.0) }
    }

    // TODO Docs
    pub fn run_function(&self, function: val::Function) -> Result<(), anyhow::Error> {   // TODO Descriptive errors
        let run = unsafe { llvm::LLVMRunFunctionPassManager(self.0, expose!(function)) };
        if run > 0 { return Ok(()) }
        else { return Err(anyhow::anyhow!("Function was not run!")) }
    }
}


use llvm_sys::transforms::*;

macro_rules! pass {
    ($pass_name:ident, $fn:path) => {
        impl PassManager {
            pub fn $pass_name(&self) {
                unsafe { $fn(self.0) }
            }
        }
    }
}

pass!(aggressive_inst_combining_pass,   aggressive_instcombine::LLVMAddAggressiveInstCombinerPass);
pass!(inst_combining_pass,              instcombine::LLVMAddInstructionCombiningPass);
pass!(coro_cleanup_pass,                coroutines::LLVMAddCoroCleanupPass);
pass!(coro_early_pass,                  coroutines::LLVMAddCoroEarlyPass);
pass!(coro_elide_pass,                  coroutines::LLVMAddCoroElidePass);
pass!(coro_split_pass,                  coroutines::LLVMAddCoroSplitPass);

pass!(always_inliner_pass,              ipo::LLVMAddAlwaysInlinerPass);
pass!(argument_promotion_pass,          ipo::LLVMAddArgumentPromotionPass);
pass!(called_value_propagation_pass,    ipo::LLVMAddCalledValuePropagationPass);
pass!(constant_merge_pass,              ipo::LLVMAddConstantMergePass);
pass!(dead_arg_elimination_pass,        ipo::LLVMAddDeadArgEliminationPass);
pass!(function_attrs_pass,              ipo::LLVMAddFunctionAttrsPass);
pass!(function_inlining_pass,           ipo::LLVMAddFunctionInliningPass);
pass!(global_dce_pass,                  ipo::LLVMAddGlobalDCEPass);
pass!(global_optimizer_pass,            ipo::LLVMAddGlobalOptimizerPass);
pass!(ipscc_pass,                       ipo::LLVMAddIPSCCPPass);
//TODO pass!(internalize_pass, ipo::LLVMAddInternalizePass);
//TODO pass!(internalize_pass_with_must_preserve, ipo::LLVMAddInternalizePassWithMustPreservePredicate);
pass!(merge_functions_pass,             ipo::LLVMAddMergeFunctionsPass);
pass!(prune_eh_pass,                    ipo::LLVMAddPruneEHPass);
pass!(strip_dead_prototypes_pass,       ipo::LLVMAddStripDeadPrototypesPass);
pass!(strip_symbols_pass,               ipo::LLVMAddStripSymbolsPass);

pass!(aggressive_dce_pass,                  scalar::LLVMAddAggressiveDCEPass);
pass!(alignment_from_assumptions_pass,      scalar::LLVMAddAlignmentFromAssumptionsPass);
pass!(basic_alias_analysis_pass,            scalar::LLVMAddBasicAliasAnalysisPass);
pass!(bit_tracking_dce_pass,                scalar::LLVMAddBitTrackingDCEPass);
pass!(cfg_simplification_pass,              scalar::LLVMAddCFGSimplificationPass);
pass!(correlated_value_propagation_pass,    scalar::LLVMAddCorrelatedValuePropagationPass);
pass!(dce_pass,                             scalar::LLVMAddDCEPass);
pass!(dead_store_elimination_pass,          scalar::LLVMAddDeadStoreEliminationPass);
pass!(demote_memory_to_register,            scalar::LLVMAddDemoteMemoryToRegisterPass);
pass!(early_cse_mem_ssa_pass,               scalar::LLVMAddEarlyCSEMemSSAPass);
pass!(early_cse_pass,                       scalar::LLVMAddEarlyCSEPass);
pass!(gvn_pass,                             scalar::LLVMAddGVNPass);
pass!(ind_var_simplifify_pass,              scalar::LLVMAddIndVarSimplifyPass);
pass!(instruction_combining_pass,           scalar::LLVMAddInstructionCombiningPass);
pass!(instruction_simplify_pass,            scalar::LLVMAddInstructionSimplifyPass);
pass!(jump_threading_pass,                  scalar::LLVMAddJumpThreadingPass);
pass!(licm_pass,                            scalar::LLVMAddLICMPass);
pass!(loop_deletion_pass,                   scalar::LLVMAddLoopDeletionPass);
pass!(loop_idiom_pass,                      scalar::LLVMAddLoopIdiomPass);
pass!(loop_reroll_pass,                     scalar::LLVMAddLoopRerollPass);
pass!(loop_rotate_pass,                     scalar::LLVMAddLoopRotatePass);
pass!(loop_unroll_and_jam_pass,             scalar::LLVMAddLoopUnrollAndJamPass);
pass!(loop_unroll_pass,                     scalar::LLVMAddLoopUnrollPass);
pass!(loop_unswitch_pass,                   scalar::LLVMAddLoopUnswitchPass);
pass!(lower_atomic_pass,                    scalar::LLVMAddLowerAtomicPass);
pass!(lower_constant_intrinsic_pass,        scalar::LLVMAddLowerConstantIntrinsicsPass);
pass!(lower_expect_intrinsic_pass,          scalar::LLVMAddLowerExpectIntrinsicPass);
pass!(mem_cpy_opt_pass,                     scalar::LLVMAddMemCpyOptPass);
pass!(merged_load_store_motion_pass,        scalar::LLVMAddMergedLoadStoreMotionPass);
pass!(new_gvn_pass,                         scalar::LLVMAddNewGVNPass);
pass!(partially_inline_lib_calls_pass,      scalar::LLVMAddPartiallyInlineLibCallsPass);
pass!(reassociate_pass,                     scalar::LLVMAddReassociatePass);
pass!(sccp_pass,                            scalar::LLVMAddSCCPPass);
pass!(scalar_repl_aggregates_pass,          scalar::LLVMAddScalarReplAggregatesPass);
pass!(scalar_repl_aggregates_pass_ssa,      scalar::LLVMAddScalarReplAggregatesPassSSA);
//TODO pass!(scalar_repl_aggregates_pass_with_threshold, scalar::LLVMAddScalarReplAggregatesPassWithThreshold);
pass!(scalarize_pass,                       scalar::LLVMAddScalarizerPass);
pass!(scoped_no_alias_aa_pass,              scalar::LLVMAddScopedNoAliasAAPass);
pass!(simplify_lib_calls_pass,              scalar::LLVMAddSimplifyLibCallsPass);
pass!(tail_call_elimination_pass,           scalar::LLVMAddTailCallEliminationPass);
pass!(type_based_alias_analysis_pass,       scalar::LLVMAddTypeBasedAliasAnalysisPass);
pass!(unify_function_exit_nodes_pass,       scalar::LLVMAddUnifyFunctionExitNodesPass);
pass!(verifier_pass,                        scalar::LLVMAddVerifierPass);

pass!(discriminators_pass,              util::LLVMAddAddDiscriminatorsPass);
pass!(lower_switch_pass,                util::LLVMAddLowerSwitchPass);
pass!(promote_memory_to_register_pass,  util::LLVMAddPromoteMemoryToRegisterPass);

pass!(loop_vectorize_pass,              vectorize::LLVMAddLoopVectorizePass);
pass!(slp_vectorize_pass,               vectorize::LLVMAddSLPVectorizePass);

/*
 * TODO
pub unsafe extern "C" fn LLVMPassManagerBuilderAddCoroutinePassesToExtensionPoints(
    PMB: LLVMPassManagerBuilderRef
)
 */

wrapper!(PassManagerBuilder, pmb::LLVMPassManagerBuilderRef);
impl PassManagerBuilder {
    pub fn new() -> Self {
        unsafe { Self::wrap(pmb::LLVMPassManagerBuilderCreate()) }
    }

    pub fn dispose(self) {
        unsafe { pmb::LLVMPassManagerBuilderDispose(self.0) }
    }

    pub fn populate_function_pass_manager(&self, passman: PassManager) {
        unsafe { pmb::LLVMPassManagerBuilderPopulateFunctionPassManager(self.0, expose!(passman)) }
    }

    pub fn populate_lto_pass_manager(&self, passman: PassManager, internalize: bool, inliner: bool) {
        let internalize = match internalize { true => 1, false => 0 };
        let inliner = match inliner { true => 1, false => 0 };
        unsafe { pmb::LLVMPassManagerBuilderPopulateLTOPassManager(self.0, expose!(passman), internalize, inliner) }
    }

    pub fn populate_module_pass_manager(&self, passman: PassManager) {
        unsafe { pmb::LLVMPassManagerBuilderPopulateModulePassManager(self.0, expose!(passman)) }
    }

    pub fn set_disable_disable_lib_calls(&self, disable: bool) {
        let disable = match disable { true => 1, false => 0 };
        unsafe { pmb::LLVMPassManagerBuilderSetDisableSimplifyLibCalls(self.0, disable) }
    }

    pub fn set_disable_unit_at_a_time(&self, disable: bool) {
        let disable = match disable { true => 1, false => 0 };
        unsafe { pmb::LLVMPassManagerBuilderSetDisableUnitAtATime(self.0, disable) }
    }

    pub fn set_disable_unroll_loops(&self, disable: bool) {
        let disable = match disable { true => 1, false => 0 };
        unsafe { pmb::LLVMPassManagerBuilderSetDisableUnrollLoops(self.0, disable) }
    }

    pub fn set_opt_level(&self, opt_level: u32) {
        unsafe { pmb::LLVMPassManagerBuilderSetOptLevel(self.0, opt_level) }
    }

    pub fn set_size_level(&self, size_level: u32) {
        unsafe { pmb::LLVMPassManagerBuilderSetSizeLevel(self.0, size_level) }
    }

    pub fn use_inliner_with_threshold(&self, threshold: u32) {
        unsafe { pmb::LLVMPassManagerBuilderUseInlinerWithThreshold(self.0, threshold) }
    }
}


wrapper!(PassBuilder, pb::LLVMPassBuilderOptionsRef);
impl PassBuilder {
    pub fn new() -> Self {
        unsafe { Self::wrap(pb::LLVMCreatePassBuilderOptions()) }
    }

    pub fn dispose(self) {
        unsafe { pb::LLVMDisposePassBuilderOptions(self.0) }
    }
}

macro_rules! pass_option {
    (boolean, $option_name:ident, $fn:path) => {
        impl PassBuilder {
            pub fn $option_name(&self, enable: bool) {
                let enable = match enable { true => 1, false => 0 };
                unsafe { $fn(self.0, enable) }
            }
        }
    };
    (cap, $option_name:ident, $fn:path) => {
        impl PassBuilder {
            pub fn $option_name(&self, cap: u32) {
                unsafe { $fn(self.0, cap) }
            }
        }
    }
}

pass_option!(boolean, set_call_graph_profile,           pb::LLVMPassBuilderOptionsSetCallGraphProfile);
pass_option!(boolean, set_debug_logging,                pb::LLVMPassBuilderOptionsSetDebugLogging);
pass_option!(boolean, set_forget_scev_in_unroll,        pb::LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll);
pass_option!(boolean, set_loop_interleaving,            pb::LLVMPassBuilderOptionsSetLoopInterleaving);
pass_option!(boolean, set_loop_unrolling,               pb::LLVMPassBuilderOptionsSetLoopUnrolling);
pass_option!(boolean, set_loop_vectorization,           pb::LLVMPassBuilderOptionsSetLoopVectorization);
pass_option!(boolean, set_loop_merge_functions,         pb::LLVMPassBuilderOptionsSetMergeFunctions);
pass_option!(boolean, set_slp_vectorization,            pb::LLVMPassBuilderOptionsSetSLPVectorization);
pass_option!(boolean, set_verify_each,                  pb::LLVMPassBuilderOptionsSetVerifyEach);
pass_option!(cap, set_licm_mssa_no_acc_promotion_cap,   pb::LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap);
pass_option!(cap, set_licm_mssa_opt_cap,                pb::LLVMPassBuilderOptionsSetLicmMssaOptCap);










