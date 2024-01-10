- Thin wrappers for LLVM in safe Rust.
- Note that wrapper types contain raw, unchecked pointers.
- Track references manually for now; ensure that only one instance of a reference is accessed at a time.
- Can be done by limiting ref access to within unsafe blocks and wrapping with a function?
- Ensure that the ref cannot be accessed except by the provided functions?
- Recommend that they do not use deref or derefmut on the wrappers -- that would be unsafe.
- Otherwise it is fine to have multiple wrappers for the same reference floating around?
- As long as they can only be accessed one-at-a-time, through functions.
- Rules out concurrency for builders within the same scope, but is that even a problem?

# To do

- Create a locking mechanism for wrapped references and include that as an optional variant.
- Guidelines for safety when using thin wrappers for performance reasons.









