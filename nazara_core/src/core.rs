use crate::sys;

pub fn initialize() -> bool {
    unsafe { sys::core::initialize() }
}

pub fn uninitialize() {
    unsafe { sys::core::uninitialize(); }
}

pub fn is_initialized() -> bool {
    unsafe { sys::core::is_initialized() }
}
