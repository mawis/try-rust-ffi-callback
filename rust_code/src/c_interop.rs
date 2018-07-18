// some types we need for C interop
// we have to define them on our own as we build without the standard library
pub enum c_void {}

pub type c_uchar = u8;
pub type c_ushort = u16;
