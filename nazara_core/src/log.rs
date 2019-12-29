use crate::sys;
use std::ffi::CString;

struct Logger {

}

fn global_logger() -> Logger {
    unsafe { sys::log::get_logger(); }
    Logger { }
}

pub fn write(message: &str) {
    let message = CString::new(message).expect("Cannot log message");
    let message_ptr = message.as_ptr();
    unsafe { sys::log::write(message_ptr) };
}
