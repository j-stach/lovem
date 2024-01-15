
extern crate libc; // TODO Transition to std::ffi::* where possible
extern crate llvm_sys;

extern crate anyhow; // TODO Transition to thiserror
// TODO While doing error, change functions that could return null to options

extern crate paste;


// Lovem features
#[macro_use] pub mod utils;
#[macro_use] pub mod wrapper;


// LLVM representations
pub mod ir;     // TODO Finish bulk fns
// Continue to refine type and value traits; need a way to cast from trait object

pub mod error;

pub mod support;
pub mod exec;  // TODO Complete
pub mod debug;  // TODO Complete



#[cfg(test)] mod test;
// TODO Test doesn't build, need to update everything to the current version and try again
// Includes installing LLVM a better way than with llvmenv




