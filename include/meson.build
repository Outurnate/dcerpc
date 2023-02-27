conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
configure_file(input : 'config.h.in', output : 'config.h', configuration : conf)

std_idl_includes = [ meson.current_source_dir(), meson.current_build_dir() ]