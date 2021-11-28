extern crate gdextension_rs;

use std::ffi::{c_void, CString};
use gdextension_rs::*;

fn entry(binding: &mut GDExtensionBinding) {
    let s = Box::into_raw(Box::new("Hello"));
    binding.set_userdata(s as *mut c_void);

    GDNativeInterface::print_error("some error", "foo_bar", "bar.java", 45);

    binding.set_core_initializer(|userdata: *mut c_void| unsafe {
        let b = Box::from_raw(userdata as *mut &str);
        println!("hi core {}", b);
    });
    binding.set_core_finalizer(|userdata: *mut c_void| {
        println!("bye core");
    });

    binding.set_servers_initializer(|userdata: *mut c_void| {
        println!("hi servers");
    });
    binding.set_servers_finalizer(|userdata: *mut c_void| {
        println!("bye servers");
    });

    binding.set_scene_initializer(|userdata: *mut c_void| {
        println!("hi scene");
    });
    binding.set_scene_finalizer(|userdata: *mut c_void| {
        println!("bye scene");
    });

    binding.set_editor_initializer(|userdata: *mut c_void| {
        println!("hi editor");
    });
    binding.set_editor_finalizer(|userdata: *mut c_void| {
        println!("bye editor");
    });

    binding.set_driver_initializer(|userdata: *mut c_void| {
        println!("hi driver");
    });
    binding.set_driver_finalizer(|userdata: *mut c_void| {
        println!("bye driver");
    });
    println!("Hello Godot 45!");
}

gdnative_entry!(harness_init, entry);
