use std::collections::HashMap;
use std::ffi::c_void;
use std::os::raw::c_uint;
use std::ptr::null_mut;

pub mod glue;
mod sys;

pub struct GDNativeInterface {
    ptr: *const glue::GDNativeInterface,
}

pub struct GDNativeExtensionClassLibraryPtr {
    ptr: glue::GDNativeExtensionClassLibraryPtr,
}

pub type GDLevelCallback = fn(&GDNativeInterface, &GDNativeExtensionClassLibraryPtr, *mut c_void);

impl GDNativeExtensionClassLibraryPtr {
    fn new(ptr: glue::GDNativeExtensionClassLibraryPtr) -> Self {
        GDNativeExtensionClassLibraryPtr { ptr }
    }
}

impl GDNativeInterface {
    fn new(ptr: *const glue::GDNativeInterface) -> Self {
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

struct BindingState {
    initializers: HashMap<c_uint, GDLevelCallback>,
    finalizers: HashMap<c_uint, GDLevelCallback>,
    interface: GDNativeInterface,
    library: GDNativeExtensionClassLibraryPtr,
}

static mut binding_state: Option<BindingState> = None;

pub struct GDExtensionBinding {
    initializers: HashMap<c_uint, GDLevelCallback>,
    finalizers: HashMap<c_uint, GDLevelCallback>,
    userdata: *mut c_void,
}

impl GDExtensionBinding {
    fn new() -> Self {
        GDExtensionBinding {
            initializers: HashMap::new(),
            finalizers: HashMap::new(),
            userdata: null_mut(),
        }
    }

    unsafe fn init(self) -> glue::GDNativeBool {
        (*sys::initialization).minimum_initialization_level =
            glue::GDNativeInitializationLevel_GDNATIVE_MAX_INITIALIZATION_LEVEL;
        (*sys::initialization).initialize = Some(initialize_level);
        (*sys::initialization).deinitialize = Some(finalize_level);
        (*sys::initialization).userdata = self.userdata;

        binding_state = Some(BindingState {
            initializers: self.initializers,
            finalizers: self.finalizers,
            interface: GDNativeInterface::new(sys::interface),
            library: GDNativeExtensionClassLibraryPtr::new(sys::library),
        });
        return 1;
    }

    pub fn set_userdata(&mut self, userdata: *mut c_void) {
        self.userdata = userdata;
    }

    pub fn set_core_initializer(&mut self, cb: GDLevelCallback) {
        self.initializers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_CORE,
            cb,
        );
    }
    pub fn set_core_finalizer(&mut self, cb: GDLevelCallback) {
        self.finalizers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_CORE,
            cb,
        );
    }

    pub fn set_servers_initializer(&mut self, cb: GDLevelCallback) {
        self.initializers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_SERVERS,
            cb,
        );
    }
    pub fn set_servers_finalizer(&mut self, cb: GDLevelCallback) {
        self.finalizers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_SERVERS,
            cb,
        );
    }

    pub fn set_scene_initializer(&mut self, cb: GDLevelCallback) {
        self.initializers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_SCENE,
            cb,
        );
    }
    pub fn set_scene_finalizer(&mut self, cb: GDLevelCallback) {
        self.finalizers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_SCENE,
            cb,
        );
    }

    pub fn set_editor_initializer(&mut self, cb: GDLevelCallback) {
        self.initializers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_EDITOR,
            cb,
        );
    }
    pub fn set_editor_finalizer(&mut self, cb: GDLevelCallback) {
        self.finalizers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_EDITOR,
            cb,
        );
    }

    pub fn set_driver_initializer(&mut self, cb: GDLevelCallback) {
        self.initializers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_DRIVER,
            cb,
        );
    }
    pub fn set_driver_finalizer(&mut self, cb: GDLevelCallback) {
        self.finalizers.insert(
            glue::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_DRIVER,
            cb,
        );
    }
}

unsafe extern "C" fn initialize_level(
    userdata: *mut c_void,
    level: glue::GDNativeInitializationLevel,
) {
    let state = binding_state.as_ref().expect("binding state not set");
    match state.initializers.get(&level) {
        Some(cb) => cb(&state.interface, &state.library, userdata),
        None => {}
    }
}

unsafe extern "C" fn finalize_level(
    userdata: *mut c_void,
    level: glue::GDNativeInitializationLevel,
) {
    let state = binding_state.as_ref().expect("binding state not set");
    match state.finalizers.get(&level) {
        Some(cb) => cb(&state.interface, &state.library, userdata),
        None => {}
    }
}

pub unsafe fn init(
    interface: *const glue::GDNativeInterface,
    library: glue::GDNativeExtensionClassLibraryPtr,
    initialization: *mut glue::GDNativeInitialization,
    entry: fn(&mut GDExtensionBinding),
) -> glue::GDNativeBool {
    sys::interface = interface;
    sys::library = library;
    sys::initialization = initialization;
    let mut binding = GDExtensionBinding::new();
    entry(&mut binding);
    return binding.init();
}

#[macro_export]
macro_rules! gdnative_entry {
    ($name: ident, $entry: ident) => {
        #[no_mangle]
        unsafe fn $name(
            _interface: *const gdextension_rs::glue::GDNativeInterface,
            _library: gdextension_rs::glue::GDNativeExtensionClassLibraryPtr,
            _initialization: *mut gdextension_rs::glue::GDNativeInitialization,
        ) -> gdextension_rs::glue::GDNativeBool {
            return gdextension_rs::init(_interface, _library, _initialization, $entry);
        }
    };
}
