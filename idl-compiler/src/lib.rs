mod run;

use run::run_idl_compile;

pub struct Builder
{
  args: Vec<String>
}

impl Builder
{
  pub fn new(c_compiler: &str) -> Self
  {
    let args = vec![
      "-cc_cmd", c_compiler,
      "-cc_opt", "-c -D_GNU_SOURCE -D_REENTRANT -D_POSIX_C_SOURCE=3",
      "-cpp_cmd", c_compiler,
      "-cpp_opt", "-E -x c-header"
    ].into_iter().map(|x| x.to_owned()).collect();

    Self { args }
  }

  pub fn include(mut self, directory: &str) -> Self
  {
    self.args.push("-I".to_owned());
    self.args.push(directory.to_owned());
    self
  }

  pub fn build(self)
  {
    run_idl_compile(self.args)
  }
}