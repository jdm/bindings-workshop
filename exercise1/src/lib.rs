extern crate libc;

use std::ffi::{CString, CStr};

pub mod ffi {
    use libc::*;

    extern "C" {
        // Add declarations for the C functions listed in `library.h`.
    }
}

pub fn set_print_prefix(prefix: &str) {
    // Call the ffi::set_print_prefix function.
    unimplemented!()
}

pub fn get_library_version() -> String {
    // Call the ffi::get_library_version function.
    unimplemented!()
}

// Can this return something more meaningful than u32?
pub fn print(s: &str) -> u32 {
    // Call the ffi::print function.
    unimplemented!()
}

// Can this return something more meaningful than u32?
pub fn print_many(v: &[&str]) -> u32 {
    // Call the ffi::print_many function.
    //
    // The C function wants `*const *const c_char`.
    // Calling `v.as_ptr()` yields `*const &str` which C will not understand.
    // What should we do instead?
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoketest() {
        assert_eq!(&get_library_version(), "print-rs version 1.0");

        print("this line is not prefixed");

        let prefix = "this is a prefix - ";
        set_print_prefix(prefix);

        print("prefixed line\n");

        print_many(&["another prefixed line\n", "and another prefixed line\n"]);

        // Expected output:
        //   this line is not prefixed
        //   this is a prefix - prefixed line
        //   this is a prefix - another prefixed line
        //   this is a prefix - and another prefixed line
    }
}
