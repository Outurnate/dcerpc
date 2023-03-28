conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
configure_file(input : 'config.h.in', output : 'config.h', configuration : conf)

std_idl_includes = [ meson.current_source_dir(), meson.current_build_dir() ]
install_subdir(
  meson.current_source_dir(),
  install_dir : '/',
  strip_directory : true,
  exclude_files :
  [
    'meson.build',
    'config.h.in',
    'compat/dcerpc.idl.include',
    'dce/dce_error.h.in',
    'dce/dce_utils.h.in',
    'dce/marshall.h.in',
    'dce/ndr_rep.h.in',
    'dce/ndrtypes.h.in',
    'dce/sec_authn.h.in'
  ]
)
install_subdir(
  meson.current_build_dir(),
  install_dir : '/',
  strip_directory : true,
  exclude_files : [ 'config.h' ]
)
