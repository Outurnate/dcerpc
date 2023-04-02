mod run;

use std::{path::Path, process::exit};

use fork::{fork, Fork};
use nix::{sys::wait::{waitpid, WaitStatus}, unistd::Pid};

use crate::run::run_idl_compile;

pub struct Builder
{
  includes: Vec<String>,
  c_compiler: String,
  idl: String,
  header: String,
  cepv: bool,
  mepv: bool,
  cstub: Option<String>,
  sstub: Option<String>,
  preprocess: bool
}

impl Default for Builder
{
  fn default() -> Self
  {
    Self
    {
      includes: Default::default(),
      c_compiler: Default::default(),
      idl: Default::default(),
      header: Default::default(),
      cepv: Default::default(),
      mepv: Default::default(),
      cstub: Default::default(),
      sstub: Default::default(),
      preprocess: true
    }
  }
}

impl Builder
{
  pub fn c_compiler(mut self, c_compiler: impl AsRef<Path>) -> Self
  {
    self.c_compiler = c_compiler.as_ref().to_string_lossy().into_owned();
    self
  }

  pub fn include(mut self, directory: impl AsRef<Path>) -> Self
  {
    self.includes.push(directory.as_ref().to_string_lossy().into_owned());
    self
  }

  pub fn idl(mut self, idl: impl AsRef<Path>) -> Self
  {
    self.idl = idl.as_ref().to_string_lossy().into_owned();
    self
  }

  pub fn header(mut self, header: impl AsRef<Path>) -> Self
  {
    self.header = header.as_ref().to_string_lossy().into_owned();
    self
  }

  pub fn cepv(mut self, enabled: bool) -> Self
  {
    self.cepv = enabled;
    self
  }

  pub fn mepv(mut self, enabled: bool) -> Self
  {
    self.mepv = enabled;
    self
  }

  pub fn cstub(mut self, file: impl AsRef<Path>) -> Self
  {
    self.cstub = Some(file.as_ref().to_string_lossy().into_owned());
    self
  }

  pub fn sstub(mut self, file: impl AsRef<Path>) -> Self
  {
    self.sstub = Some(file.as_ref().to_string_lossy().into_owned());
    self
  }

  pub fn preprocess(mut self, enabled: bool) -> Self
  {
    self.preprocess = enabled;
    self
  }

  pub fn build(self)
  {
    let mut args = Vec::new();
    args.push(self.idl);
    args.push("-header".to_owned());
    args.push(self.header);
    if self.cepv { args.push("-cepv".to_owned()) }
    args.push((if self.mepv { "-mepv" } else { "-no_mepv" }).to_owned());
    args.push("-cc_cmd".to_owned());
    args.push(self.c_compiler.clone());
    args.push("-cc_opt".to_owned());
    args.push("-c -D_GNU_SOURCE -D_REENTRANT -D_POSIX_C_SOURCE=3".to_owned());
    args.push("-cpp_cmd".to_owned());
    args.push(self.c_compiler);
    args.push("-cpp_opt".to_owned());
    args.push("-E -x c-header".to_owned());
    for include in self.includes
    {
      args.push("-I".to_owned());
      args.push(include);
    }
    if self.cstub.is_some() || self.sstub.is_some()
    {
      args.push("-keep".to_owned());
      args.push("c_source".to_owned());
    }
    
    if let Some(cstub) = self.cstub
    {
      args.push("-cstub".to_owned());
      args.push(cstub);
    }
    else
    {
      args.push("-client".to_owned());
      args.push("none".to_owned());
    }

    if let Some(sstub) = self.sstub
    {
      args.push("-sstub".to_owned());
      args.push(sstub);
    }
    else
    {
      args.push("-server".to_owned());
      args.push("none".to_owned());
    }

    if !self.preprocess
    {
      args.push("-no_cpp".to_owned());
    }
    
    match fork().unwrap()
    {
      Fork::Parent(child_pid) => loop
      {
        match waitpid(Pid::from_raw(child_pid), None).unwrap()
        {
          WaitStatus::Exited(_, _) => break,
          _ => continue
        }
      },
      Fork::Child =>
      {
        run_idl_compile(args);
        exit(1);
      }
    }
  }
}