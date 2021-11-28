extern crate gdextension_rs;

use gdextension_rs::*;
use std::ffi::c_void;

fn entry(binding: &mut GDExtensionBinding) {
    let s = Box::into_raw(Box::new("Hello"));
    binding.set_userdata(s as *mut c_void);

    binding.set_core_initializer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| unsafe {
            let b = Box::from_raw(userdata as *mut &str);
            println!("hi core {}", b);
        },
    );
    binding.set_core_finalizer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("bye core");
        },
    );

    binding.set_servers_initializer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("hi servers");
        },
    );
    binding.set_servers_finalizer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("bye servers");
        },
    );

    binding.set_scene_initializer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("hi scene");
        },
    );
    binding.set_scene_finalizer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("bye scene");
        },
    );

    binding.set_editor_initializer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("hi editor");
        },
    );
    binding.set_editor_finalizer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("bye editor");
        },
    );

    binding.set_driver_initializer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("hi driver");
        },
    );
    binding.set_driver_finalizer(
        |interface: &GDNativeInterface,
         library: &GDNativeExtensionClassLibraryPtr,
         userdata: *mut c_void| {
            println!("bye driver");
        },
    );
    println!("Hello Godot 4!");
}

gdnative_entry!(harness_init, entry);
