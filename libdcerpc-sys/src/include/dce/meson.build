
conf = configuration_data()
conf.set('target_os', 'linux-gnu')
conf.set('target_cpu', 'x86_64')

configure_file(input : 'dce_error.h.in', output : 'dce_error.h', configuration : conf)
configure_file(input : 'dce_utils.h.in', output : 'dce_utils.h', configuration : conf)
configure_file(input : 'ndr_rep.h.in', output : 'ndr_rep.h', configuration : conf)
configure_file(input : 'sec_authn.h.in', output : 'sec_authn.h', configuration : conf)
configure_file(input : 'dce.h.in', output : 'dce.h', configuration : conf)
configure_file(input : 'marshall.h.in', output : 'marshall.h', configuration : conf)
configure_file(input : 'ndrtypes.h.in', output : 'ndrtypes.h', configuration : conf)
dceidl = [
  dceidl_executable,
  '@INPUT@', '-header', '@OUTPUT@',
  '-cepv', '-client', 'none', '-server', 'none', '-no_mepv',
  idl_flags
]

dce_gen = [
  custom_target('codesets', input : [ 'codesets.idl' ], output : [ 'codesets.h' ], command : dceidl, build_by_default : true),
  custom_target('conv', input : [ 'conv.idl' ], output : [ 'conv.h' ], command : dceidl, build_by_default : true),
  custom_target('id_base', input : [ 'id_base.idl' ], output : [ 'id_base.h' ], command : dceidl, build_by_default : true),
  custom_target('lbase', input : [ 'lbase.idl' ], output : [ 'lbase.h' ], command : dceidl, build_by_default : true),
  custom_target('mgmt', input : [ 'mgmt.idl' ], output : [ 'mgmt.h' ], command : dceidl, build_by_default : true),
  custom_target('ncastat', input : [ 'ncastat.idl' ], output : [ 'ncastat.h' ], command : dceidl, build_by_default : true),
  custom_target('rpcbase', input : [ 'rpcbase.idl' ], output : [ 'rpcbase.h' ], command : dceidl, build_by_default : true),
  custom_target('rpcpvt', input : [ 'rpcpvt.idl' ], output : [ 'rpcpvt.h' ], command : dceidl, build_by_default : true),
  custom_target('rpctypes', input : [ 'rpctypes.idl' ], output : [ 'rpctypes.h' ], command : dceidl, build_by_default : true),
  custom_target('smb', input : [ 'smb.idl' ], output : [ 'smb.h' ], command : dceidl, build_by_default : true),
  custom_target('uuid', input : [ 'uuid.idl' ], output : [ 'uuid.h' ], command : dceidl, build_by_default : true),
  custom_target('convc', input : [ 'convc.idl' ], output : [ 'convc.h' ], command : dceidl, build_by_default : true),
  custom_target('ep', input : [ 'ep.idl' ], output : [ 'ep.h' ], command : dceidl, build_by_default : true),
  custom_target('iovector', input : [ 'iovector.idl' ], output : [ 'iovector.h' ], command : dceidl, build_by_default : true),
  custom_target('lrpc', input : [ 'lrpc.idl' ], output : [ 'lrpc.h' ], command : dceidl, build_by_default : true),
  custom_target('nbase', input : [ 'nbase.idl' ], output : [ 'nbase.h' ], command : dceidl, build_by_default : true),
  custom_target('ndrold', input : [ 'ndrold.idl' ], output : [ 'ndrold.h' ], command : dceidl, build_by_default : true),
  custom_target('rpc', input : [ 'rpc.idl' ], output : [ 'rpc.h' ], command : dceidl, build_by_default : true),
  custom_target('rpcsts', input : [ 'rpcsts.idl' ], output : [ 'rpcsts.h' ], command : dceidl, build_by_default : true),
  custom_target('schannel', input : [ 'schannel.idl' ], output : [ 'schannel.h' ], command : dceidl, build_by_default : true),
  custom_target('twr', input : [ 'twr.idl' ], output : [ 'twr.h' ], command : dceidl, build_by_default : true)
]
dce_gen_dependency = declare_dependency(sources : dce_gen)