
extern crate libc; // TODO Transition to std::ffi::* where possible
extern crate llvm_sys;

extern crate anyhow; // TODO Transition to thiserror

extern crate paste;


// Lovem features
#[macro_use] pub mod utils;
#[macro_use] pub mod wrapper;


// LLVM representations
pub mod ir;     // TODO Finish bulk fns

pub mod error;

pub mod support;
pub mod exec;  // TODO Complete
pub mod debug;  // TODO Complete









