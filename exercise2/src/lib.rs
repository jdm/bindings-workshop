extern crate libc;

use std::ffi::CString;

#[allow(non_camel_case_types)]
pub mod ffi {
    use libc::{c_char, c_uint, c_int, c_void};

    #[repr(C)]
    pub struct string_t(c_void);
    #[repr(C)]
    pub struct slice_t(c_void);

    extern "C" {
        // Create a new string object by copying the null-terminated character buffer passed in.
        pub fn string_create(s: *const c_char) -> *mut string_t;

        // Add one to the reference count of the given string object.
        pub fn string_addref(string: *mut string_t);

        // Remove one oustanding reference of the given string object. Frees the object if the
        // count reaches zero.
        pub fn string_release(string: *mut string_t);

        // Attempt to create a slice from the provided string, starting at the given offset
        // and continuing for the given length. Returns null if the slice would exceed the
        // bounds of the original string. Adds a reference to the original string.
        pub fn string_slice(string: *mut string_t, offset: c_uint, length: c_uint) -> *mut slice_t;

        // Frees the provided slice object, and releases a reference to the underlying string.
        pub fn slice_free(slice: *mut slice_t);

        // Determine if either slice is a substring of the other.
        // Returns -N if `b` is a substring of `a` starting at position N,
        // N if `a` is a substring of `b` starting at position N,
        // and i32::min_value() if no match is found in either.
        pub fn substring(a: *const slice_t, b: *const slice_t) -> c_int;
    }
}

pub struct String {
    // What should this type contain?
}

impl String {
    pub fn new(s: &str) -> String {
        // Call the ffi::string_create function.
        unimplemented!()
    }

    // Can we always return a valid Slice object?
    pub fn slice(&self, offset: u32, length: u32) -> Slice {
        // Call the ffi::string_slice function.
        unimplemented!()
    }
}

// What other traits should this String type implement? Drop? Clone?

pub struct Slice {
    // What should this type contain?
}

// What other traits should this Slice type implement? Drop? Clone?

// Would an enum return type be more meaningful?
pub fn substring(a: &Slice, b: &Slice) -> i32 {
    // Call the ffi::substring function.
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let base = "this is a string buffer this is";
        let base2 = "some other buffer";
        let str1 = String::new(base);
        let str2 = String::new(base2);

        // Suggested tests:

        // Make a copy of str1.
        // let str3 = str1.clone();

        // Assert that identical slices of str1 and the copy are substrings.
        // let s1 = str1.slice(0, base.len() as u32).unwrap();
        // let s2 = str3.slice(0, base.len() as u32).unwrap();
        // assert_eq!(substring(&s1, &s2), Some(SliceResult::Left(0)));

        // Assert that complete slices of str1 and str2 substrings.
        // let s3 = str2.slice(0, base2.len() as u32).unwrap();
        // assert_eq!(substring(&s1, &s3), None);

        // Assert that a slice of str1 from position 5 of length 4 is a substring of str1.
        // let s4 = str1.slice(5, 4).unwrap();
        // assert_eq!(substring(&s4, &s1), Some(SliceResult::Right(5)));

    }
}
