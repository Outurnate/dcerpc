dceidl = [
  dceidl_executable,
  '@INPUT@', '-header', '@OUTPUT0@', '-cstub', '@OUTPUT1@',
  '-I../include',
  '-keep', 'c_source', '-no_cpp', '-v', '-no_mepv', '-server', 'none',
  idl_flags
]

include_pickle = custom_target(
  'pickle',
  input : [ 'pickle.idl' ],
  output : [ 'pickle.h', 'pickle_cstub.c' ],
  command : dceidl,
  build_by_default : true
)

include_ep = custom_target(
  'ep',
  input : [ '../include/dce/ep.idl' ],
  output : [ 'ep.h', 'ep_cstub.c' ],
  command : [ dceidl, '-cepv' ],
  build_by_default : true
)

include_mgmt = custom_target(
  'mgmt',
  input : [ '../include/dce/mgmt.idl' ],
  output : [ 'mgmt.h', 'mgmt_cstub.c' ],
  command : [ dceidl, '-cepv', '-sstub', 'mgmt_sstub.c' ],
  build_by_default : true
)

include_convc = custom_target(
  'convc',
  input : [ '../include/dce/convc.idl' ],
  output : [ 'convc.h', 'convc_cstub.c' ],
  command : [ dceidl, '-cepv', '-sstub', 'convc_sstub.c' ],
  build_by_default : true
)

platform_includes = include_directories('include/linux-gnu')
com = static_library('com',
  'comauth.c',
  'combind.c',
  'comcall.c',
  'comcthd.c',
  'comep.c',
  'comif.c',
  'cominit.c',
  'cominit_ux.c',
  'comnaf.c',
  'comnet.c',
  'comnlsn.c',
  'comobj.c',
  'comp.c',
  'comtwr.c',
  'comtwrflr.c',
  'comtwrref.c',
  'comutil.c',
  'nsldap.c',
  'sec_id.c',
  include_directories: [ platform_includes, common_includes ],
  dependencies : [ thread_dependency, dcethread_dependency, dce_gen_dependency ],
  c_args : [ '-DATTRIBUTE_UNUSED=__attribute__((unused))', '-DNCK', '-DPIC', '-DIMAGE_DIR="/usr/local/lib"' ]
)

static_library('stubs',
  include_pickle,
  include_ep,
  dce_gen,
  include_directories: [ common_includes ]
)

rpc = static_library('rpc',
  'dce_error.c',
  'rpcclock.c',
  'rpcdbg.c',
  'rpclist.c',
  'rpclog.c',
  'rpcmem.c',
  'rpcmutex.c',
  'rpcrand.c',
  'rpctimer.c',
  include_directories: [ platform_includes, common_includes ],
  c_args : [ '-DCATALOG_DIR="/usr/local/share/dce-rpc"' ]
)

trans = static_library('trans',
  'comsoc.c',
  'comsoc_bsd.c',
  include_directories: [ platform_includes, common_includes ]
)

mgmt_stubs = static_library('mgmt_stubs',
  include_mgmt,
  dce_gen,
  include_directories: [ common_includes ]
)

prot_ncacn = static_library('prot_ncacn',
  'cnassoc.c',
  'cnbind.c',
  'cncall.c',
  'cncasgsm.c',
  'cncassm.c',
  'cncclsm.c',
  'cncthd.c',
  'cnfbuf.c',
  'cnid.c',
  'cninit.c',
  'cnmgmt.c',
  'cnnet.c',
  'cnp.c',
  'cnpkt.c',
  'cnrcvr.c',
  'cnsasgsm.c',
  'cnsassm.c',
  'cnsclsm.c',
  'cnsm.c',
  'cnxfer.c',
  include_directories: [ platform_includes, common_includes ]
)

ncadg_stubs = static_library('ncadg_stubs',
  include_convc,
  dce_gen,
  include_directories: [ common_includes ]
)

prot_ncadg = static_library('prot_ncadg',
  'conv.c',
  'dg.c',
  'dgcall.c',
  'dgccall.c',
  'dgccallt.c',
  'dgcct.c',
  'dgclive.c',
  'dgclsn.c',
  'dgexec.c',
  'dgfwd.c',
  'dgglob.c',
  'dghnd.c',
  'dginit.c',
  'dglossy.c',
  'dglsn.c',
  'dgpkt.c',
  'dgrq.c',
  'dgscall.c',
  'dgsct.c',
  'dgslive.c',
  'dgslsn.c',
  'dgsoc.c',
  'dgutl.c',
  'dgxq.c',
  dce_gen,
  include_directories: [ platform_includes, common_includes ],
  link_with : ncadg_stubs
)

naf_ip = static_library('naf_ip',
  'ipnaf.c',
  'ipnaf_linux.c',
  'twr_ip.c',
  include_directories: [ platform_includes, common_includes ]
)

naf_np = static_library('naf_np',
  'npnaf.c',
  'npnaf_linux.c',
  'twr_np.c',
  'twr_uxd.c',
  include_directories: [ platform_includes, common_includes ]
)

auth_gss_negotiate = static_library('auth_gss_negotiate',
  'gssauth.c',
  'gssauthcn.c',
  include_directories: [ platform_includes, common_includes ]
)

nck = static_library('nck',
  'comfork.c',
  'ndrglob.c',
  'mgmt.c',
  'rpcsvc.c',
  include_directories: [ platform_includes, common_includes ],
  link_with : [ com, rpc, trans, mgmt_stubs, prot_ncacn, prot_ncadg, naf_ip, naf_np, auth_gss_negotiate ]
)

nck_dependency = declare_dependency(
  link_with : nck,
  include_directories : [ platform_includes, common_includes, include_directories('.') ]
)