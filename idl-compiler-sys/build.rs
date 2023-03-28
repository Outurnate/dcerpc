use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::process::Command;
use std::fmt::Display;
use std::fs::read_to_string;
use std::fs::write;

fn flex(out_dir: &PathBuf, c: impl AsRef<Path>, h: impl AsRef<Path>, l: impl Display)
{
  Command::new("flex")
    .args([
      format!("--outfile={}", out_dir.join(c).display()),
      format!("--header-file={}", out_dir.join(h).display()),
      l.to_string()
    ])
    .current_dir("src")
    .output().unwrap();
}

fn bison(out_dir: &PathBuf, c: impl AsRef<Path>, h: impl AsRef<Path>, y: impl Display)
{
  Command::new("bison")
    .args([
      format!("--output={}", out_dir.join(c).display()),
      format!("--defines={}", out_dir.join(h).display()),
      y.to_string()
    ])
    .current_dir("src")
    .output().unwrap();
}

// really crummy, but oh well
fn configure_file<'a>(input: impl AsRef<Path>, output: impl AsRef<Path>, replacements: impl IntoIterator<Item = (&'a str, &'a str)>)
{
  let mut data = read_to_string(input).unwrap();
  for (from, to) in replacements
  {
    data = data.replace(from, to);
  }
  write(output, data).unwrap();
}

fn main()
{
  system_deps::Config::new().probe().unwrap();

  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  flex(&out_dir, "acf_l.c", "acf_l.h", "acf_l.l");
  flex(&out_dir, "nidl_l.c", "nidl_l.h", "nidl_l.l");
  bison(&out_dir, "acf_y.c", "acf_y.h", "acf_y.y");
  bison(&out_dir, "nidl_y.c", "nidl_y.h", "nidl_y.y");

  configure_file("src/sysdep.h.in", out_dir.join("sysdep.h"), vec![
    ("APPLE_LICENSE_HEADER_START", ""),
    ("APPLE_LICENSE_HEADER_END", ""),
    ("IDL_CPP", "$CPP"),
    ("IDL_CC", "$CC"),
    ("IDL_CFLAGS", ""),
    ("OBJEXT", "o")
  ]);

  cc::Build::new()
    .file(out_dir.join("acf_l.c"))
    .file(out_dir.join("nidl_l.c"))
    .file(out_dir.join("acf_y.c"))
    .file(out_dir.join("nidl_y.c"))
    .files(vec![
      "src/astp_com.c",
      "src/astp_exp.c",
      "src/checker.c",
      "src/command.c",
      "src/cspell.c",
      "src/ddspell.c",
      "src/files.c",
      "src/hdgen.c",
      "src/irepgen.c",
      "src/main.c",
      "src/mtspipes.c",
      "src/propagat.c",
      "src/user_exc.c",
      "src/astp_cpx.c",
      "src/astp_gbl.c",
      "src/chkichar.c",
      "src/comstmts.c",
      "src/cstubmts.c",
      "src/driver.c",
      "src/frontend.c",
      "src/icharsup.c",
      "src/irepscp.c",
      "src/message.c",
      "src/namdump.c",
      "src/sstubmts.c",
      "src/astp_dmp.c",
      "src/astp_sim.c",
      "src/clihamts.c",
      "src/cspeldcl.c",
      "src/ddbe.c",
      "src/errors.c",
      "src/getflags.c",
      "src/ifspemts.c",
      "src/keywds.c",
      "src/mtsbacke.c",
      "src/nametbl.c",
      "src/sysdep.c"
    ])
    .include(out_dir.clone())
    .include("src")
    .define("MIA", None)
    .define("DEFAULT_IDIR", "\"$(includedir)\"")
    .define("CATALOG_DIR", "\"$(pkgdatadir)\"")
    .define("YYERROR_VERBOSE", "1")
    .define("YYDEBUG", "1")
    .define("VERSION", "\"1.1.0.7\"")
    .define("ATTRIBUTE_UNUSED", "__attribute__((unused))")
    .define("_XOPEN_SOURCE", "500")
    .define("__EXTENSIONS__", None)
    .define("_POSIX_PTHREAD_SEMANTICS", None)
    .define("_ALL_SOURCE", "1")
    .define("_REENTRANT", None)
    //.define("HAVE_CONFIG_H", None)
    .compile("idl_compiler");
}