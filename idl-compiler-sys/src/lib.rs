use std::os::raw::{c_int, c_char};

extern "C"
{
  fn main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

pub unsafe fn idl_compile(argc: c_int, argv: *mut *mut c_char) -> c_int
{
  main(argc, argv)
}