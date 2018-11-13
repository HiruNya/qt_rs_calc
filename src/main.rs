extern crate libc;

mod implementation;
mod calculator;
#[allow(dead_code)]
pub mod interface {
    include!(concat!(env!("OUT_DIR"), "/src/interface.rs"));
}

extern {
    fn main_cpp(app: *const ::std::os::raw::c_char);
}

fn main() {
    use std::{env, ffi::CString};
    let app_name = env::args().next().unwrap();
    let app_name = CString::new(app_name).unwrap();
    
    // Use the `flat` style
    env::set_var("QT_QUICK_CONTROLS_1_STYLE", "Flat");

    unsafe {
        main_cpp(app_name.as_ptr());
    }
}
