use std::{path::Path, process::Command, ffi::OsStr};
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

  pub fn build(self, command: impl AsRef<OsStr>)
  {
    let mut command = Command::new(command);
    command.arg(self.idl);
    command.arg("-header");
    command.arg(self.header);
    command.arg(if self.cepv { "-cepv" } else { "-no_cepv" });
    command.arg(if self.mepv { "-mepv" } else { "-no_mepv" });
    command.arg("-cc_cmd");
    command.arg(&self.c_compiler);
    command.arg("-cc_opt");
    command.arg("-c -D_GNU_SOURCE -D_REENTRANT -D_POSIX_C_SOURCE=3");
    command.arg("-cpp_cmd");
    command.arg(self.c_compiler);
    command.arg("-cpp_opt");
    command.arg("-E -x c-header");
    for include in self.includes
    {
      command.arg("-I");
      command.arg(include);
    }
    if self.cstub.is_some() || self.sstub.is_some()
    {
      command.arg("-keep");
      command.arg("c_source");

      if let Some(cstub) = self.cstub
      {
        command.arg("-cstub");
        command.arg(cstub);
      }
      else
      {
        command.arg("-client");
        command.arg("none");
      }

      if let Some(sstub) = self.sstub
      {
        command.arg("-sstub");
        command.arg(sstub);
      }
      else
      {
        command.arg("-server");
        command.arg("none");
      }
    }
    if !self.preprocess
    {
      command.arg("-no_cpp");
    }


    for arg in command.get_args()
    {
      println!("{}", arg.to_string_lossy());
    }
    println!("{}", std::str::from_utf8(&command.output().unwrap().stderr).unwrap());
  }
}