use std::ptr::{null, null_mut};

use crate::glue;

pub static mut interface: *const glue::GDNativeInterface = null();
pub static mut library: glue::GDNativeExtensionClassLibraryPtr = null_mut();
pub static mut initialization: *mut glue::GDNativeInitialization = null_mut();