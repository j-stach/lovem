

pub trait Wrapper {
    type Llvm;
    fn wrap(llvm: Self::Llvm) -> Self where Self: Sized;
    unsafe fn expose(&self) -> &Self::Llvm;
}

macro_rules! wrapper {
    ($t:ident, $llvm:ty) => {
        #[doc = "TODO: Dynamically link to LLVM documentation"]
        #[derive(Debug)]
        pub struct $t ($llvm);
        impl Wrapper for $t {
            type Llvm = $llvm;
            fn wrap(llvm: $llvm) -> Self {
                Self(llvm)
            }
            unsafe fn expose(&self) -> &$llvm {
                &self.0
            }
        }
    }
}

macro_rules! expose_array {
    ($wrapper_array:expr) => {{
        let mut exposed_array = vec![];
        $wrapper_array.iter().enumerate().for_each(|(w, wrapper)| exposed_array[w] = *wrapper.expose());
        exposed_array.as_mut_ptr()
    }}
}

macro_rules! expose {
    ($wrapper:expr) => {{
        *$wrapper.expose()
    }}
}

#[allow(unused_macros)]
macro_rules! force_wrap {
    ($typ:ty, $typ_ref:expr) => {{
        <$typ>::wrap($typ_ref)
    }}
}

