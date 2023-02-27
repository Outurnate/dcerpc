project('DCE RPC', 'c', version : '1.1.0.7', default_options : [ 'warning_level=2', 'buildtype=release' ])

subdir('include')

add_project_arguments('-fno-strict-aliasing', language : 'c')
add_project_arguments('-Wmissing-format-attribute', language : 'c')
add_project_arguments('-Wbad-function-cast', language : 'c')
add_project_arguments('-Wstrict-prototypes', language : 'c')
add_project_arguments('-Wpointer-arith', language : 'c')
add_project_arguments('-Wcast-align', language : 'c')
add_project_arguments('-Wwrite-strings', language : 'c')

if meson.get_compiler('c').has_argument('-fstack-protector')
  add_project_arguments('-fstack-protector', language : 'c')
  add_project_arguments('-D_FORTIFY_SOURCE=2', language : 'c')
endif

add_project_arguments('-D_XOPEN_SOURCE=500', language : 'c')
add_project_arguments('-D__EXTENSIONS__', language : 'c')
add_project_arguments('-D_POSIX_PTHREAD_SEMANTICS', language : 'c')
add_project_arguments('-D_ALL_SOURCE=1', language : 'c')
add_project_arguments('-D_REENTRANT', language : 'c')
add_project_arguments('-DHAVE_CONFIG_H', language : 'c')

thread_dependency = dependency('threads')
dl_dependency = meson.get_compiler('c').find_library('dl')
nsl_dependency = dependency('libnsl')

common_includes = include_directories('include')

idl_flags = [
  '-cc_cmd', meson.get_compiler('c', native : false).cmd_array(),
  '-cc_opt', '-c -D_GNU_SOURCE -D_REENTRANT -D_POSIX_C_SOURCE=3',
  '-cpp_cmd', meson.get_compiler('c', native : false).cmd_array(),
  '-cpp_opt', '-E -x c-header',
  '-I', std_idl_includes[0],
  '-I', std_idl_includes[1]
]

subdir('compat')
subdir('libdcethread')
subdir('idl_compiler')
subdir('include/dce')
subdir('ncklib')
subdir('idl_lib')
subdir('uuid')
subdir('dcelib')