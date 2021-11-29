use std::ffi::CStr;
use std::fmt;

use crate::glue;

pub struct GDVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub str: String,
}

impl fmt::Display for GDVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

pub struct GDNativeInterface {
    ptr: *const glue::GDNativeInterface,
}

impl GDNativeInterface {
    pub(crate) fn new(ptr: *const glue::GDNativeInterface) -> Self {
        GDNativeInterface { ptr }
    }

    pub fn get_version(&self) -> GDVersion {
        let major: u32;
        let minor: u32;
        let patch: u32;
        let str: String;
        unsafe {
            major = (*self.ptr).version_major;
            minor = (*self.ptr).version_minor;
            patch = (*self.ptr).version_patch;
            str = String::from(CStr::from_ptr((*self.ptr).version_string).to_str().unwrap());
        }
        GDVersion {
            major,
            minor,
            patch,
            str,
        }
    }

    pub fn print_error(&self, description: &str, function: &str, file: &str, line_number: i32) {
        unsafe {
            (*self.ptr).print_error.unwrap()(
                description.as_ptr() as _,
                function.as_ptr() as _,
                file.as_ptr() as _,
                line_number,
            );
        }
    }

    pub fn print_warning(&self, description: &str, function: &str, file: &str, line_number: i32) {
        unsafe {
            (*self.ptr).print_warning.unwrap()(
                description.as_ptr() as _,
                function.as_ptr() as _,
                file.as_ptr() as _,
                line_number,
            );
        }
    }

    pub fn print_script_error(
        &self,
        description: &str,
        function: &str,
        file: &str,
        line_number: i32,
    ) {
        unsafe {
            (*self.ptr).print_script_error.unwrap()(
                description.as_ptr() as _,
                function.as_ptr() as _,
                file.as_ptr() as _,
                line_number,
            );
        }
    }
}
