use core::ffi::c_void;
use cpp::*;
use std::os::raw::c_char;

pub struct Logger {

}

cpp! {{
    #include <Nazara/Core/Log.hpp>
}}

pub unsafe fn enable(state: bool) {
    cpp!([state as "bool"] {
        Nz::Log::Enable(state);
    });
}

pub unsafe fn get_logger() {
    let logger = cpp!([] -> *mut c_void as "Nz::AbstractLogger*" {
        return Nz::Log::GetLogger();
    });
}

pub unsafe fn is_enabled() -> bool {
    cpp!([] -> bool as "bool" {
        return Nz::Log::IsEnabled();
    })
}

pub unsafe fn set_logger(logger: &'static Logger) {

}

pub unsafe fn write(message: *const c_char) {
    cpp!([message as "const char *"] {
        Nz::Log::Write(message);
    });
}
