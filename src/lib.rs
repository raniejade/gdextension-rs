use std::collections::HashMap;
use std::ffi::c_void;
use std::os::raw::c_uint;
use std::ptr::null_mut;

// re-exports
pub use interface::*;

pub mod glue;
mod interface;

pub type GDNativeExtensionClassLibraryPtr = glue::GDNativeExtensionClassLibraryPtr;

pub type GDLevelCallback = fn(&GDNativeInterface, GDNativeExtensionClassLibraryPtr, *mut c_void);

struct BindingState {
    initializers: HashMap<c_uint, GDLevelCallback>,
    finalizers: HashMap<c_uint, GDLevelCallback>,
    interface: GDNativeInterface,
    library: GDNativeExtensionClassLibraryPtr,
}

static mut binding_state: Option<BindingState> = None;

pub struct GDExtensionBinding {
    interface: *const glue::GDNativeInterface,
    library: glue::GDNativeExtensionClassLibraryPtr,
    initialization: *mut glue::GDNativeInitialization,
    initializers: HashMap<c_uint, GDLevelCallback>,
    finalizers: HashMap<c_uint, GDLevelCallback>,
    userdata: *mut c_void,
}

impl GDExtensionBinding {
    fn new(
        interface: *const glue::GDNativeInterface,
        library: glue::GDNativeExtensionClassLibraryPtr,
        initialization: *mut glue::GDNativeInitialization,
    ) -> Self {
        GDExtensionBinding {
            interface,
            library,
            initialization,
            initializers: HashMap::new(),
            finalizers: HashMap::new(),
            userdata: null_mut(),
        }
    }

    unsafe fn init(self) -> glue::GDNativeBool {
        (*self.initialization).minimum_initialization_level =
            glue::GDNativeInitializationLevel_GDNATIVE_MAX_INITIALIZATION_LEVEL;
        (*self.initialization).initialize = Some(initialize_level);
        (*self.initialization).deinitialize = Some(finalize_level);
        (*self.initialization).userdata = self.userdata;

        binding_state = Some(BindingState {
            initializers: self.initializers,
            finalizers: self.finalizers,
            interface: GDNativeInterface::new(self.interface),
            library: self.library,
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
        Some(cb) => cb(&state.interface, state.library, userdata),
        None => {}
    }
}

unsafe extern "C" fn finalize_level(
    userdata: *mut c_void,
    level: glue::GDNativeInitializationLevel,
) {
    let state = binding_state.as_ref().expect("binding state not set");
    match state.finalizers.get(&level) {
        Some(cb) => cb(&state.interface, state.library, userdata),
        None => {}
    }
}

pub unsafe fn init(
    interface: *const glue::GDNativeInterface,
    library: glue::GDNativeExtensionClassLibraryPtr,
    initialization: *mut glue::GDNativeInitialization,
    entry: fn(&mut GDExtensionBinding),
) -> glue::GDNativeBool {
    let mut binding = GDExtensionBinding::new(interface, library, initialization);
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
