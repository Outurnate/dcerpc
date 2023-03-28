mod run;

use std::env;
use run::run_idl_compile;

fn main()
{
  run_idl_compile(env::args());
}