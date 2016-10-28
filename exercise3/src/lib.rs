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
    // Call the ffi::execute_query function.
    unimplemented!()
}

pub struct Results {
    // What should this type contain?
}

impl Results {
    pub fn num_results(&self) -> u32 {
        // Call the ffi::num_query_results function.
        unimplemented!()
    }

    pub fn iter(&self) -> ResultsIterator {
        unimplemented!()
    }
}

impl Drop for Results {
    fn drop(&mut self) {
        // What memory needs to be freed via C?
        // What memory needs to be returned to Rust?
    }
}

pub struct ResultsIterator {
    // What should this type contain?
}

// How should ResultsIterator implement the Iterator trait?
// What item type should it return? &CStr? &str? String?

#[cfg(test)]
mod tests {
    use super::*;

    // Create a vector that contains the same values that exist in the list of results.
    fn make_expected_values(r: &Results) -> Vec<String> {
        let mut v = vec![];
        for i in 0..r.num_results() {
            v.push((i * 1000).to_string());
        }
        v
    }

    #[test]
    fn it_works() {
        // Suggested test:
        // Create a results set; iterate over it; assert that each value
        // from the iterator corresponds with the matching value from
        // `make_expected_values`:
        //
        // let results = execute_query("select values from table");
        // let expected = make_expected_values(results);
        // for (value, expected) in results.iter().zip(expected.iter()) {
        //     assert_eq!(value, expected);
        // }
    }
}
