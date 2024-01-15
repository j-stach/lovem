

/// Convert CStr to str safely, for both static and dynamic strings.
/// Panics if ptr is not a CStr
// TODO Mark this as panicking
macro_rules! cstr_to_str {
    ($s:expr) => {{
        unsafe { std::ffi::CStr::from_ptr($s) }
            .to_str().expect("Convert CStr to &str")
    }}
}

// TODO Other conversion methods for crate use


macro_rules! str_to_cstr {
    ($s:expr) => {{
        let c_str = std::ffi::CString::new($s).expect("Convert &str to Cstring");
        c_str.as_ptr()
    }}
}


macro_rules! size {
    ($a:expr) => {{
        $a.len() as u32
    }}
}

#[allow(unused_macros)]
macro_rules! bool_to_rust {
    ($b:expr) => {{
        unsafe {
            $b > 0
        }
    }}
}

#[allow(unused_macros)]
macro_rules! bool_to_llvm {
    ($b:expr) => {{
        match $b {
            false => 0,
            true => 1
        }
    }}
}

macro_rules! find_size {
    ($e:expr) => {{
        let mut len = 0;
        while !$e.add(len).is_null() {
            len += 1;
        }
        len
    }}
}


