extern crate libc;

use std::ffi::{CString, CStr};
use std::iter::Iterator;

pub mod ffi {
    use libc::{c_void, c_char, c_uint};

    #[repr(C)]
    pub struct query_result_t(c_void);

    extern "C" {
        // Accepts an SQL query string and returns a new instance of the corresponding
        // result set. The result set takes ownership of the query, and returns ownership
        // to the caller as part of free_query_result.
        pub fn execute_query(query: *const c_char) -> *mut query_result_t;

        // Returns the number of results present in this result set.
        pub fn num_query_results(results: *mut query_result_t) -> c_uint;

        // Returns the Nth result from the set (all results are null-terminated character strings).
        // The Nth result will be the string representation of N * 1000.
        pub fn get_nth_result(results: *mut query_result_t, nth: c_uint) -> *const c_char;

        // Free the memory associated with this result set as well as the individual results,
        // and return ownership of the query string to the caller.
        pub fn free_query_result(results: *mut query_result_t) -> *mut c_char;
    }
}

pub fn execute_query(query: &str) -> Results {
    let query = CString::new(query).unwrap();
    let ptr = unsafe {
        ffi::execute_query(query.into_raw())
    };
    assert!(!ptr.is_null());
    Results {
        ptr: ptr,
        num_rows: unsafe { ffi::num_query_results(ptr) },
    }
}

pub struct Results {
    ptr: *mut ffi::query_result_t,
    num_rows: u32,
}

impl Results {
    pub fn num_results(&self) -> u32 {
        self.num_rows
    }

    pub fn iter(&self) -> ResultsIterator {
        ResultsIterator {
            results: self,
            idx: 0,
        }
    }
}

impl Drop for Results {
    fn drop(&mut self) {
        unsafe {
            let q = ffi::free_query_result(self.ptr);
            let _ = CString::from_raw(q);
        }
    }
}

pub struct ResultsIterator<'a> {
    results: &'a Results,
    idx: u32,
}

impl<'a> Iterator for ResultsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.idx >= self.results.num_rows {
            return None;
        }
        let s = unsafe { ffi::get_nth_result(self.results.ptr, self.idx) };
        self.idx += 1;
        Some(unsafe { CStr::from_ptr(s) }.to_str().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_expected_values(r: &Results) -> Vec<String> {
        let mut v = vec![];
        for i in 0..r.num_results() {
            v.push((i * 1000).to_string());
        }
        v
    }

    #[test]
    fn it_works() {
        let results = execute_query("select values from table");
        let expected = make_expected_values(&results);
        for (result, expected) in results.iter().zip(expected.iter()) {
            assert_eq!(result, expected);
        }
    }
}
