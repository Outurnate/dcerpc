use std::ffi::{c_int, CString, c_char};
use idl_compiler_sys::idl_compile;

pub(crate) fn run_idl_compile<T: Into<Vec<u8>>>(args: impl IntoIterator<Item = T>)
{
  let mut args = args.into_iter().map(|arg| CString::new(arg).unwrap().into_raw()).collect::<Vec<*mut c_char>>();
  args.insert(0, CString::new("dceidl").unwrap().into_raw());
  let argc = args.len() as c_int;
  let argv = args.as_mut_ptr();
  unsafe { idl_compile(argc, argv) };
}