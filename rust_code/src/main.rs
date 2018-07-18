#[allow(non_camel_case_types)]
pub mod ccode;

#[link(name="ccode")]
extern{}

pub struct RustData {
}

unsafe extern "C" fn callback(usr: *mut ::std::os::raw::c_void) {
    println!("Inside callback!");
}

fn register_callback(data: *mut RustData) {
    unsafe {
        ccode::register_callback(Some(callback),
                                 data as *mut ::std::os::raw::c_void);
    }
}

fn main() {
    let mut data_box = Box::new(RustData {});
    register_callback(&mut *data_box);
}
