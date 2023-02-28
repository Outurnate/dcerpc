dcerpc = static_library('dcerpc',
  'dummyfuncs.c',
  dependencies : [ nck_dependency, idl_dependency, uuid_dependency, dcethread_dependency, compat_dependency, nsl_dependency, dl_dependency, dce_gen_dependency ],
  install : true,
  install_dir : '/'
)

dcerpc_dependency = declare_dependency(
  link_with : dcerpc
)
