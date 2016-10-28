use libc::{c_char, c_uint};
use std::ffi::{CString, CStr};

extern crate libc;

pub mod ffi {
    use libc::{c_char, c_uint};

    extern "C" {
        pub fn get_library_version() -> *const c_char;
        pub fn set_print_prefix(prefix: *const c_char);
        pub fn print(s: *const c_char) -> c_uint;
        pub fn print_many(s: *const *const c_char, n: c_uint) -> c_uint;
    }
}

pub fn set_print_prefix(prefix: &str) {
    let s = CString::new(prefix).unwrap();
    unsafe {
        ffi::set_print_prefix(s.as_ptr());
    }
}

pub fn get_library_version() -> String {
    let s = unsafe {
        CStr::from_ptr(ffi::get_library_version())
    };
    s.to_str().unwrap().to_string()
}

pub fn print(s: &str) -> Result<(), ()> {
    let s = CString::new(s).unwrap();
    let code = unsafe {
        ffi::print(s.as_ptr())
    };
    if code != 0 {
        Err(())
    } else {
        Ok(())
    }
}

pub fn print_many(v: &[&str]) -> Result<(), u32> {
    let v: Vec<CString> = v.iter().map(|s| CString::new(*s).unwrap()).collect();
    let v: Vec<*const c_char> = v.iter().map(|s| s.as_ptr()).collect();
    let code = unsafe {
        ffi::print_many(v.as_ptr(), v.len() as c_uint)
    };
    if code > 0 {
        Err(code)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoketest() {
        assert_eq!(&get_library_version(), "print-rs version 1.0");

        print("this line is not prefixed");

        let prefix = "this is a prefix -";
        set_print_prefix(prefix);

        print("prefixed line\n");

        print_many(&["another prefixed line\n", "and another prefixed line\n"]);
    }
}
