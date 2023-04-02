use std::ffi::{NulError, CString};
use std::ptr::null_mut;
use std::fmt::Display;
use thiserror::Error;
use crate::msgs::ERROR_MESSAGES;

pub mod ms_icpr;
mod msgs;

#[derive(Error, Debug)]
pub enum RpcError
{
  #[error("error converting string: early null terminator")]
  StringError(#[from] NulError),
  #[error("{0}")]
  DceError(String)
}

pub enum Protocol
{
  NamedPipes,
  Tcp,
  Udp,
  LocalRPC
}

const STATUS_CODE_MASK: u32 = 0x00000FFF;
const STATUS_CODE_SHIFT: u32 = 0;

fn error_code(code: u32) -> String
{
  let status_code = (code & STATUS_CODE_MASK) >> STATUS_CODE_SHIFT;

  return ERROR_MESSAGES[status_code as usize - 1].to_owned();
}

fn check_error(code: u32, func: impl AsRef<str> + Display) -> Result<(), RpcError>
{
  if code != libdcerpc_sys::error_status_ok
  {
    Err(RpcError::DceError(format!("error {} ({:#10X}) in {}", error_code(code), code, func)))
  }
  else
  {
    Ok(())
  }
}

struct DceString
{
  string: *mut u8
}

impl DceString
{
  fn compose_binding(protocol: Protocol, netaddr: &str) -> Result<DceString, RpcError>
  {
    let prot_string = match protocol
    {
      Protocol::NamedPipes => CString::new("ncacn_np"),
      Protocol::Tcp        => CString::new("ncacn_ip_tcp"),
      Protocol::Udp        => CString::new("ncadg_ip_udp"),
      Protocol::LocalRPC   => CString::new("ncalrpc")
    }?.into_raw();
    let netaddr = CString::new(netaddr.as_bytes())?.into_raw();
    let mut string: *mut u8 = null_mut();
    let mut status = 0u32;
    unsafe
    {
      libdcerpc_sys::rpc_string_binding_compose(
        null_mut(),
        prot_string as *mut u8,
        netaddr as *mut u8,
        null_mut(),
        null_mut(),
        &mut string as *mut *mut u8,
        &mut status as *mut u32);
      drop(CString::from_raw(netaddr));
      drop(CString::from_raw(prot_string));
    }
    check_error(status, "rpc_string_binding_compose")?;
    Ok(Self { string })
  }
}

impl Drop for DceString
{
  fn drop(&mut self)
  {
    let mut status = 0u32;
    unsafe { libdcerpc_sys::rpc_string_free(&mut self.string, &mut status); }
  }
}

struct RpcBinding
{
  handle: *mut libdcerpc_sys::rpc_handle_s_t
}

impl RpcBinding
{
  fn new(string_binding: DceString) -> Result<Self, RpcError>
  {
    let mut handle = null_mut();
    let mut status = 0u32;
    unsafe
    {
      libdcerpc_sys::rpc_binding_from_string_binding(
        string_binding.string,
        &mut handle,
        &mut status as *mut u32);
    }
    check_error(status, "rpc_binding_from_string_binding")?;
    Ok(Self { handle })
  }

  fn ep_resolve(&mut self, if_spec: libdcerpc_sys::rpc_if_handle_t) -> Result<(), RpcError>
  {
    let mut status = 0u32;
    unsafe
    {
      libdcerpc_sys::rpc_ep_resolve_binding(
        self.handle,
        if_spec,
        &mut status);
    }
    check_error(status, "rpc_ep_resolve_binding")
  }

  fn set_auth_info(&mut self, spn: &str) -> Result<(), RpcError>
  {
    let spn = CString::new(spn.as_bytes())?.into_raw();
    let mut status = 0u32;
    unsafe
    {
      libdcerpc_sys::rpc_binding_set_auth_info(
        self.handle,
        spn as *mut u8,
        libdcerpc_sys::rpc_c_protect_level_pkt_privacy,
        libdcerpc_sys::rpc_c_authn_gss_mskrb,
        null_mut(),
        libdcerpc_sys::rpc_c_authz_name,
        &mut status);
      drop(CString::from_raw(spn));
    }
    check_error(status, "rpc_binding_set_auth_info")
  }
}

impl Drop for RpcBinding
{
  fn drop(&mut self)
  {
    let mut status = 0u32;
    unsafe { libdcerpc_sys::rpc_binding_free(&mut self.handle, &mut status); }
  }
}

fn clone_to_utf16(string: &str, null_terminate: bool) -> Vec<u16>
{
  let mut result = string.encode_utf16().into_iter().collect::<Vec<u16>>();
  if null_terminate
  {
    result.push(0);
  }
  result
}

fn clone_to_utf16_le(string: &str, null_terminate: bool) -> Vec<u8>
{
  let mut result = string.encode_utf16().into_iter().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>();
  if null_terminate
  {
    result.push(0);
  }
  result
}