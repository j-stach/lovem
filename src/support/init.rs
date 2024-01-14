
use llvm_sys::prelude::LLVMPassRegistryRef;
use llvm_sys::initialization as llvm;

use crate::wrapper::Wrapper;


wrapper!(Initializer, LLVMPassRegistryRef);

macro_rules! initializer {
    ($initializer:ident, $fn:path) => {
        impl Initializer {
            pub fn $initializer(&self) {
                unsafe { $fn(self.0) }
            }
        }
    }
}

initializer!(aggressive_inst_combiner, llvm::LLVMInitializeAggressiveInstCombiner);
initializer!(analysis                , llvm::LLVMInitializeAnalysis);
initializer!(code_gen                , llvm::LLVMInitializeCodeGen);
initializer!(core                    , llvm::LLVMInitializeCore);
initializer!(ipa                     , llvm::LLVMInitializeIPA);
initializer!(ipo                     , llvm::LLVMInitializeIPO);
initializer!(inst_combine            , llvm::LLVMInitializeInstCombine);
initializer!(instrumentation         , llvm::LLVMInitializeInstrumentation);
initializer!(obj_carc_opts           , llvm::LLVMInitializeObjCARCOpts);
initializer!(scalar_opts             , llvm::LLVMInitializeScalarOpts);
initializer!(target                  , llvm::LLVMInitializeTarget);
initializer!(transform_utils         , llvm::LLVMInitializeTransformUtils);
initializer!(vectorization           , llvm::LLVMInitializeVectorization);

impl Initializer {
    // TODO??
}



