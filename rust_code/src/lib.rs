#![feature(panic_implementation)]
#![no_std]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_implementation]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[allow(non_camel_case_types)]
pub mod ccode;

#[allow(non_camel_case_types)]
pub mod c_interop;

#[link(name="ccode")]
extern{}

pub struct RustData {
    number: u16
}

unsafe extern "C" fn callback(usr: *mut c_interop::c_void) {
    let data = usr as *mut RustData;
    (*data).number = (*data).number + 1;
}

fn register_callback(data: *mut RustData) {
    unsafe {
        ccode::register_callback(Some(callback),
                                 data as *mut c_interop::c_void);
    }
}

pub extern fn main() {
    let mut data = RustData { number: 42 };
    register_callback(&mut data);
}
