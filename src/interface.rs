use crate::glue;

pub struct GDNativeInterface {
    ptr: *const glue::GDNativeInterface,
}

impl GDNativeInterface {
    pub(crate) fn new(ptr: *const glue::GDNativeInterface) -> Self {
        GDNativeInterface { ptr }
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
