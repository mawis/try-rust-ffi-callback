/* automatically generated by rust-bindgen */

pub type callback_func =
    ::core::option::Option<unsafe extern "C" fn(usr: *mut ::c_interop::c_void)>;
extern "C" {
    pub fn register_callback(f: callback_func, usr: *mut ::c_interop::c_void);
}
