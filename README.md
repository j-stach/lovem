
Wrappers for LLVM in safe Rust.

### WIP: Untested, should be presumed malfunctional
**Expect breaking changes in all new versions before 0.1.0**
- The current approach with boxed dynamic types makes me want to commit to reference counting,
since we are relying so heavily on the heap anyway.
- This tempts me to put a Mutex on everything as well, to make lovem into a concurrency-focused library
- Will explore the best way to pursue this; maybe interchangeable wrapper types for sync and async contexts
------
### Notes to self
Simplest safe interface?
- Note that wrapper types contain raw, unchecked pointers.
- Mutexes would ensure that only one instance of a reference is accessed at a time.
- Could also be done by limiting ref access to within unsafe blocks, within safe functions.
- Recommend do not use deref or derefmut on the wrappers -- that would be unsafe.
- Otherwise it is fine to have multiple wrappers for the same reference floating around?
- Downside to Arc is that it gives the wrong impression count-wise if a new wrapper/ref is created by an LLVM function
  
Performance?
- Currently high level of abstraction, use of dynamic type checking will affect compiling times
but should not impact performance of the code compiled with compilers built using this crate.
- This is for simplicity an ease of use on the Rust end
- A more closely-wrapped `closer` version of the functions will be added later; these will maintain safety but will not
obscure function arguments that are found in native LLVM

Standardize value-handling?
- intend to restrict types where possible; `values::Function` instead of `Box<dyn values::Value>`, for example.
- they're both `LLVMValueRef` under the hood, but this makes it easier to work with, safer, more rusty
- may standardize to all boxed values, eg `Box<values::Function>` or `Arc<...>`






