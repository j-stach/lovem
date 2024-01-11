
extern crate libc; // TODO Transition to std::ffi::* where possible
extern crate llvm_sys;

extern crate anyhow; // TODO Transition to thiserror


#[macro_use]
pub mod convert;

#[macro_use]
pub mod wrapper;

pub mod error;
pub mod diagnostics;

pub mod ir;











