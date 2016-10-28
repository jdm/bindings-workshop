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

pub enum Destination {
    Stdout,
    Stderr,
}

pub struct Logger {
    ptr: *mut ffi::logger_t,
}

impl Drop for Logger {
    fn drop(&mut self) {
        unsafe {
            let p = ffi::logger_get_private(self.ptr) as *mut LoggerData;
            let _ = Box::from_raw(p);
            ffi::logger_free(self.ptr);
        }
    }
}

unsafe extern "C" fn print_hook(logger: *mut ffi::logger_t, _s: *const c_char) -> ffi::log_type_t {
    let p = ffi::logger_get_private(logger) as *const LoggerData;
    match (*p).action {
        Some(Destination::Stdout) => ffi::log_type_t::LOG_STDOUT,
        Some(Destination::Stderr) => ffi::log_type_t::LOG_STDERR,
        None => ffi::log_type_t::LOG_SUPPRESS,
    }
}

impl Logger {
    pub fn new(dest: Option<Destination>) -> Logger {
        let data = Box::new(LoggerData { action: dest });
        let ptr = unsafe {
            ffi::logger_new(Box::into_raw(data) as *mut c_void)
        };
        assert!(!ptr.is_null());
        unsafe {
            ffi::logger_set_print_hook(ptr, Some(print_hook));
        }
        Logger {
            ptr: ptr,
        }
    }

    pub fn log(&self, s: &str) {
        let s = CString::new(s).unwrap();
        unsafe {
            ffi::logger_log(self.ptr, s.as_ptr());
        }
    }
}

pub struct LoggerData {
    action: Option<Destination>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let l = Logger::new(Some(Destination::Stderr));
        l.log("hi there\n");
        let l2 = Logger::new(None);
        l2.log("hi there #2\n");
        l.log("hi there #3\n");
    }
}
