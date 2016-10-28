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
    ptr: *mut ffi::string_t,
}

impl String {
    pub fn new(s: &str) -> String {
        let s = CString::new(s).unwrap();

        let ptr = unsafe {
            ffi::string_create(s.as_ptr())
        };
        assert!(!ptr.is_null());

        unsafe {
            ffi::string_addref(ptr)
        };

        String {
            ptr: ptr,
        }
    }

    pub fn slice(&self, offset: u32, length: u32) -> Option<Slice> {
        let s = unsafe {
            ffi::string_slice(self.ptr, offset, length)
        };
        if s.is_null() {
            None
        } else {
            Some(Slice {
                ptr: s,
            })
        }
    }
}

impl Clone for String {
    fn clone(&self) -> String {
        unsafe {
            ffi::string_addref(self.ptr)
        };
        String {
            ptr: self.ptr
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            ffi::string_release(self.ptr)
        }
    }
}

pub struct Slice {
    ptr: *mut ffi::slice_t,
}

impl Drop for Slice {
    fn drop(&mut self) {
        unsafe {
            ffi::slice_free(self.ptr)
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SliceResult {
    Left(u32),
    Right(u32),
}

pub fn substring(a: &Slice, b: &Slice) -> Option<SliceResult> {
    let code = unsafe {
        ffi::substring(a.ptr, b.ptr)
    };
    match code {
        n if n == i32::min_value() =>
            None,

        n if n > 0 =>
            Some(SliceResult::Right(n as u32)),

        n if n <= 0 =>
            Some(SliceResult::Left(n as u32)),

        _ => unreachable!(),
    }
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
        let str3 = str1.clone();

        let s1 = str1.slice(0, base.len() as u32).unwrap();
        let s2 = str3.slice(0, base.len() as u32).unwrap();
        assert_eq!(substring(&s1, &s2), Some(SliceResult::Left(0)));

        let s3 = str2.slice(0, base2.len() as u32).unwrap();
        assert_eq!(substring(&s1, &s3), None);

        let s4 = str1.slice(5, 4).unwrap();
        assert_eq!(substring(&s4, &s1), Some(SliceResult::Right(5)));
    }
}
