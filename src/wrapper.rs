

pub trait Wrapper {
    type Llvm: Copy;
    fn wrap(llvm: Self::Llvm) -> Self where Self: Sized;
    unsafe fn expose(&self) -> &Self::Llvm;
    fn is_wrapper(&self) -> bool { true }
}

macro_rules! wrapper {
    ($t:ident, $llvm:ty) => {
        #[doc = "TODO: Dynamically link to LLVM documentation"]
        #[derive(Debug, Clone)]
        pub struct $t ($llvm);
        impl Wrapper for $t {
            type Llvm = $llvm;
            fn wrap(llvm: $llvm) -> Self {
                Self(llvm) // TODO Make wrap contain the unsafe block?
            }
            unsafe fn expose(&self) -> &$llvm {
                &self.0
            }
        }
    }
}

macro_rules! expose_array {
    ($wrapper_array:expr) => {{
        let mut exposed_array = vec![]; // TODO Test
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
macro_rules! wrap {
    ($typ:ty, $typ_ref:expr) => {{
        <$typ>::wrap($typ_ref)
    }}
}

pub trait NonWrapper {
    type Llvm;
    unsafe fn expose(&self) -> &Self::Llvm;
}

impl NonWrapper for u32 {
    type Llvm = Self;
    unsafe fn expose(&self) -> &Self { &self }
}

pub trait WrapperCollection {
    type Llvm;
    unsafe fn expose(self) -> *mut Self::Llvm;
}

impl<W: Wrapper> WrapperCollection for Vec<W> {
    type Llvm = W::Llvm;
    unsafe fn expose(self) -> *mut Self::Llvm {
        let mut exposed = vec![]; // TODO Test
        self.iter().enumerate().for_each(|(w, wrapper)| exposed[w] = *wrapper.expose());
        exposed.as_mut_ptr()
    }
}
