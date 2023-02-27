idl_compiler_includes = include_directories('.')

flex = find_program('flex', required : true)
bison = find_program('bison', required : true)
flex_generator  = generator(flex,  output : [ '@BASENAME@.c', '@BASENAME@.h' ], arguments : [ '--outfile=@OUTPUT0@', '--header-file=@OUTPUT1@', '@INPUT@' ])
bison_generator = generator(bison, output : [ '@BASENAME@.c', '@BASENAME@.h' ], arguments : [  '--output=@OUTPUT0@', '--defines=@OUTPUT1@',     '@INPUT@' ])

lfiles = flex_generator.process('acf_l.l', 'nidl_l.l')
pfiles = bison_generator.process('acf_y.y', 'nidl_y.y')

conf = configuration_data()
conf.set('APPLE_LICENSE_HEADER_START', '')
conf.set('APPLE_LICENSE_HEADER_END', '')
conf.set('IDL_CPP', '$CPP')
conf.set('IDL_CC', '$CC')
conf.set('IDL_CFLAGS', '')
conf.set('OBJEXT', 'o')
configure_file(input : 'sysdep.h.in', output : 'sysdep.h', configuration : conf)

dceidl_executable = executable('dceidl',
  lfiles,
  pfiles,
  'astp_com.c',
  'astp_exp.c',
  'checker.c',
  'command.c',
  'cspell.c',
  'ddspell.c',
  'files.c',
  'hdgen.c',
  'irepgen.c',
  'main.c',
  'mtspipes.c',
  'propagat.c',
  'user_exc.c',
  'astp_cpx.c',
  'astp_gbl.c',
  'chkichar.c',
  'comstmts.c',
  'cstubmts.c',
  'driver.c',
  'frontend.c',
  'icharsup.c',
  'irepscp.c',
  'message.c',
  'namdump.c',
  'sstubmts.c',
  'astp_dmp.c',
  'astp_sim.c',
  'clihamts.c',
  'cspeldcl.c',
  'ddbe.c',
  'errors.c',
  'getflags.c',
  'ifspemts.c',
  'keywds.c',
  'mtsbacke.c',
  'nametbl.c',
  'sysdep.c',
  include_directories: [ idl_compiler_includes ],
  dependencies: compat_native_dependency,
  native: true,
  c_args:
  [
    '-DMIA',
    '-DDEFAULT_IDIR="$(includedir)"',
    '-DCATALOG_DIR="$(pkgdatadir)"',
    '-DYYERROR_VERBOSE=1',
    '-DYYDEBUG=1',
    '-DVERSION="' + meson.project_version() + '"',
    '-DATTRIBUTE_UNUSED=__attribute__((unused))',
    '-I' + meson.global_build_root() # HACK - flex seems to ignore --header-file and drop headers in the current working directory
  ]
)