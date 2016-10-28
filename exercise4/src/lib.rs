use std::ffi::CString;
use libc::{c_void, c_char};
extern crate libc;

#[allow(non_camel_case_types)]
pub mod ffi {
    use libc::{c_void, c_char};

    #[repr(C)]
    pub struct logger_t(c_void);

    #[repr(C)]
    pub enum log_type_t {
        LOG_SUPPRESS = 0,
        LOG_STDOUT = 1,
        LOG_STDERR = 2,
    }

    pub type print_hook_t =
        Option<unsafe extern "C" fn(l: *mut logger_t, s: *const c_char) -> log_type_t>;

    extern "C" {
        // Create a new instance of a logger.
        pub fn logger_new(private: *mut c_void) -> *mut logger_t;
        // Free a logger instance.
        pub fn logger_free(logger: *mut logger_t);
        // Return the private data associated with this logger.
        pub fn logger_get_private(logger: *mut logger_t) -> *mut c_void;
        // Set the print hook for this logger.
        pub fn logger_set_print_hook(logger: *mut logger_t, print_hook: print_hook_t);
        // Log the given string using the given logger. The logger's print hook is
        // called, which can optionally return without logging anything or direct
        // the output towards stdout or stderr.
        pub fn logger_log(logger: *mut logger_t, s: *const c_char);
    }
}

pub struct Logger {
    // What should this type contain?
}

unsafe extern "C" fn print_hook(logger: *mut ffi::logger_t, _s: *const c_char) -> ffi::log_type_t {
    // How does this callback decide which value to return to C?
    unimplemented!()
}

impl Logger {
    pub fn new(/* some kind of configuration? */) -> Logger {
        // Call the ffi::logger_new and ffi::logger_set_print_hook functions.
        unimplemented!()
    }

    pub fn log(&self, s: &str) {
        // Call the ffi::logger_log function.
        unimplemented!()
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        // What memory needs to be freed by C?
        // What memory needs to be reclaimed by Rust?
    }
}

pub struct LoggerData {
    // What data should this type contain?
    // What does the `print_hook` callback need to access?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Suggested test:
        // Create a logger that is not suppressed, and another one that is.
        // Log messages using both, and manually verify that no suppressed
        // output appears in the terminal.
        //
        // let logger = Logger::new(...);
        // logger.log("should be visible");
        //
        // let logger_suppressed = Logger:new(...);
        // logger_suppressed.log("should not be visible");
        //
        // logger.log("should also be visible");
    }
}
