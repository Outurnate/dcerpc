use std::os::raw::{c_int, c_char};

extern "C"
{
  pub fn idl_compile(argc: c_int, argv: *mut *mut c_char) -> c_int;
}