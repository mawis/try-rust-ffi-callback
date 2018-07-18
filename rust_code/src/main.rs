#[allow(non_camel_case_types)]
pub mod ccode;

#[link(name="ccode")]
extern{}

pub struct RustData {
    number: u16
}

unsafe extern "C" fn callback(usr: *mut ::std::os::raw::c_void) {
    let data = usr as *mut RustData;
    println!("Number is {}", (*data).number);
}

fn register_callback(data: *mut RustData) {
    unsafe {
        ccode::register_callback(Some(callback),
                                 data as *mut ::std::os::raw::c_void);
    }
}

fn main() {
    let mut data = RustData { number: 42 };
    register_callback(&mut data);
}
