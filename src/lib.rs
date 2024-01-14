
extern crate libc; // TODO Transition to std::ffi::* where possible
extern crate llvm_sys;

extern crate anyhow; // TODO Transition to thiserror

extern crate paste;


// Lovem features
#[macro_use] pub mod utils;
#[macro_use] pub mod wrapper;


// LLVM representations
pub mod ir;

pub mod error;
pub mod diagnostics;

pub mod support;
pub mod exec;
pub mod debug;









