use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::path::Path;
use std::fs::read_to_string;
use std::fs::create_dir_all;

// really crummy, but oh well
fn configure_file<'a>(input: impl AsRef<Path>, output: impl AsRef<Path>, replacements: impl IntoIterator<Item = (&'a str, &'a str)>) -> Result<(), std::io::Error>
{
  let mut data = read_to_string(input)?;
  for (from, to) in replacements
  {
    data = data.replace(from, to);
  }
  
  let mut f = std::fs::OpenOptions::new().create(true).write(true).open(output)?;
  f.write_all(data.as_bytes())?;
  f.flush()?;

  Ok(())
}

fn idl_compile(input: impl AsRef<Path>, output: impl AsRef<Path>)
{
  idl_compiler::Builder::default()
    .include("src/include")
    .idl(input)
    .header(output)
    .cepv(true)
    .mepv(false)
    .c_compiler("/usr/bin/cc")
    .build("../target/debug/idl-compiler");
}

fn idl_compile_client(input: impl AsRef<Path>, header: impl AsRef<Path>, cstub: impl AsRef<Path>)
{
  idl_compiler::Builder::default() // -v ???
    .include("src/include")
    .idl(input)
    .header(header)
    .cstub(cstub)
    .mepv(false)
    .cepv(true)
    .c_compiler("/usr/bin/cc")
    .build("../target/debug/idl-compiler");
}

fn idl_compile_server(input: impl AsRef<Path>, header: impl AsRef<Path>, cstub: impl AsRef<Path>, sstub: impl AsRef<Path>)
{
  idl_compiler::Builder::default() // -v ???
    .include("src/include")
    .idl(input)
    .header(header)
    .cstub(cstub)
    .sstub(sstub)
    .mepv(false)
    .cepv(true)
    .c_compiler("/usr/bin/cc")
    .build("../target/debug/idl-compiler");
}

fn main()
{
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let dce_dir = out_dir.join("dce");
  let idl_dir = out_dir.join("idl");
  create_dir_all(dce_dir.clone()).unwrap();
  create_dir_all(idl_dir.clone()).unwrap();

  configure_file("src/include/config.h.in", out_dir.join("config.h"), vec![
    ("@VERSION@", "\"1.1.0.7\"")
  ]).unwrap();

  let conf = vec![
    ("@target_os@", "linux-gnu"),
    ("@target_cpu@", "x86_64")
  ];

  configure_file("src/include/dce/dce_error.h.in", dce_dir.join("dce_error.h"), conf.clone()).unwrap();
  configure_file("src/include/dce/dce_utils.h.in", dce_dir.join("dce_utils.h"), conf.clone()).unwrap();
  configure_file("src/include/dce/ndr_rep.h.in", dce_dir.join("ndr_rep.h"), conf.clone()).unwrap();
  configure_file("src/include/dce/sec_authn.h.in", dce_dir.join("sec_authn.h"), conf.clone()).unwrap();
  configure_file("src/include/dce/dce.h.in", dce_dir.join("dce.h"), conf.clone()).unwrap();
  configure_file("src/include/dce/marshall.h.in", dce_dir.join("marshall.h"), conf.clone()).unwrap();
  configure_file("src/include/dce/ndrtypes.h.in", dce_dir.join("ndrtypes.h"), conf).unwrap();

  idl_compile("src/include/dce/codesets.idl", dce_dir.join("codesets.h"));
  idl_compile("src/include/dce/conv.idl", dce_dir.join("conv.h"));
  idl_compile("src/include/dce/id_base.idl", dce_dir.join("id_base.h"));
  idl_compile("src/include/dce/lbase.idl", dce_dir.join("lbase.h"));
  idl_compile("src/include/dce/mgmt.idl", dce_dir.join("mgmt.h"));
  idl_compile("src/include/dce/ncastat.idl", dce_dir.join("ncastat.h"));
  idl_compile("src/include/dce/rpcbase.idl", dce_dir.join("rpcbase.h"));
  idl_compile("src/include/dce/rpcpvt.idl", dce_dir.join("rpcpvt.h"));
  idl_compile("src/include/dce/rpctypes.idl", dce_dir.join("rpctypes.h"));
  idl_compile("src/include/dce/smb.idl", dce_dir.join("smb.h"));
  idl_compile("src/include/dce/uuid.idl", dce_dir.join("uuid.h"));
  idl_compile("src/include/dce/convc.idl", dce_dir.join("convc.h"));
  idl_compile("src/include/dce/ep.idl", dce_dir.join("ep.h"));
  idl_compile("src/include/dce/iovector.idl", dce_dir.join("iovector.h"));
  idl_compile("src/include/dce/lrpc.idl", dce_dir.join("lrpc.h"));
  idl_compile("src/include/dce/nbase.idl", dce_dir.join("nbase.h"));
  idl_compile("src/include/dce/ndrold.idl", dce_dir.join("ndrold.h"));
  idl_compile("src/include/dce/rpc.idl", dce_dir.join("rpc.h"));
  idl_compile("src/include/dce/rpcsts.idl", dce_dir.join("rpcsts.h"));
  idl_compile("src/include/dce/schannel.idl", dce_dir.join("schannel.h"));
  idl_compile("src/include/dce/twr.idl", dce_dir.join("twr.h"));

  idl_compile_client("src/ncklib/pickle.idl", idl_dir.join("pickle.h"), idl_dir.join("pickle_cstub.c"));
  idl_compile_client("src/include/dce/ep.idl", idl_dir.join("ep.h"), idl_dir.join("ep_cstub.c"));
  idl_compile_server("src/include/dce/mgmt.idl", idl_dir.join("mgmt.h"), idl_dir.join("mgmt_cstub.c"), idl_dir.join("mgmt_sstub.c"));
  idl_compile_client("src/include/dce/convc.idl", idl_dir.join("convc.h"), idl_dir.join("convc_cstub.c"));
  idl_compile_client("src/include/dce/conv.idl", idl_dir.join("conv.h"), idl_dir.join("conv_cstub.c"));

  cc::Build::new()
    .files(vec![
      "src/dummyfuncs.c",
      "src/idl_lib/alfrsupp.c",
      "src/idl_lib/allocate.c",
      "src/idl_lib/autohndl.c",
      "src/idl_lib/bindcall.c",
      "src/idl_lib/ctxeecli.c",
      "src/idl_lib/ctxeectx.c",
      "src/idl_lib/ctxerrtl.c",
      "src/idl_lib/cvt_glob.c",
      "src/idl_lib/eebool.c",
      "src/idl_lib/eebyte.c",
      "src/idl_lib/eechar.c",
      "src/idl_lib/eedouble.c",
      "src/idl_lib/eeenum.c",
      "src/idl_lib/eefloat.c",
      "src/idl_lib/eehyper.c",
      "src/idl_lib/eelong.c",
      "src/idl_lib/eenodtbl.c",
      "src/idl_lib/eeshort.c",
      "src/idl_lib/eesmall.c",
      "src/idl_lib/eeuhyper.c",
      "src/idl_lib/eeulong.c",
      "src/idl_lib/eeushort.c",
      "src/idl_lib/eeusmall.c",
      "src/idl_lib/erbool.c",
      "src/idl_lib/erbyte.c",
      "src/idl_lib/erchar.c",
      "src/idl_lib/erdouble.c",
      "src/idl_lib/erenum.c",
      "src/idl_lib/erfloat.c",
      "src/idl_lib/erhyper.c",
      "src/idl_lib/erlong.c",
      "src/idl_lib/ernodtbl.c",
      "src/idl_lib/ershort.c",
      "src/idl_lib/ersmall.c",
      "src/idl_lib/eruhyper.c",
      "src/idl_lib/erulong.c",
      "src/idl_lib/erushort.c",
      "src/idl_lib/erusmall.c",
      "src/idl_lib/interpsh.c",
      "src/idl_lib/marbfman.c",
      "src/idl_lib/nbaseool.c",
      "src/idl_lib/ndrcharp.c",
      "src/idl_lib/ndrchars.c",
      "src/idl_lib/ndrfloat.c",
      "src/idl_lib/ndrmi.c",
      "src/idl_lib/ndrmi2.c",
      "src/idl_lib/ndrmi3.c",
      "src/idl_lib/ndrmi5.c",
      "src/idl_lib/ndrui.c",
      "src/idl_lib/ndrui2.c",
      "src/idl_lib/ndrui3.c",
      "src/idl_lib/ndrui5.c",
      "src/idl_lib/nidlalfr.c",
      "src/idl_lib/pickling.c",
      "src/idl_lib/pipesupp.c",
      "src/idl_lib/sscmaset.c",
      "src/idl_lib/sscmasrv.c",
      "src/libdcethread/dcethread_create.c",
      "src/libdcethread/dcethread_join.c",
      "src/libdcethread/dcethread_detach.c",
      "src/libdcethread/dcethread_interrupt.c",
      "src/libdcethread/dcethread_pause.c",
      "src/libdcethread/dcethread_cond_wait.c",
      "src/libdcethread/dcethread_atfork.c",
      "src/libdcethread/dcethread_kill.c",
      "src/libdcethread/dcethread_exit.c",
      "src/libdcethread/dcethread_read.c",
      "src/libdcethread/dcethread_write.c",
      "src/libdcethread/dcethread_send.c",
      "src/libdcethread/dcethread_sendto.c",
      "src/libdcethread/dcethread_sendmsg.c",
      "src/libdcethread/dcethread_recv.c",
      "src/libdcethread/dcethread_recvfrom.c",
      "src/libdcethread/dcethread_recvmsg.c",
      "src/libdcethread/dcethread_select.c",
      "src/libdcethread/dcethread_checkinterrupt.c",
      "src/libdcethread/dcethread-private.c",
      "src/libdcethread/dcethread-debug.c",
      "src/libdcethread/dcethread-util.c",
      "src/libdcethread/dcethread-exception.c",
      "src/libdcethread/dcethread_get_expiration.c",
      "src/libdcethread/dcethread_delay.c",
      "src/libdcethread/dcethread_lock_global.c",
      "src/libdcethread/dcethread_unlock_global.c",
      "src/libdcethread/dcethread_ismultithreaded.c",
      "src/libdcethread/dcethread_mutexattr_getkind.c",
      "src/libdcethread/dcethread_mutexattr_setkind.c",
      "src/libdcethread/dcethread_signal_to_interrupt.c",
      "src/libdcethread/dcethread_attr_create.c",
      "src/libdcethread/dcethread_attr_delete.c",
      "src/libdcethread/dcethread_attr_setprio.c",
      "src/libdcethread/dcethread_attr_getprio.c",
      "src/libdcethread/dcethread_attr_setsched.c",
      "src/libdcethread/dcethread_attr_getsched.c",
      "src/libdcethread/dcethread_attr_setinheritsched.c",
      "src/libdcethread/dcethread_attr_getinheritsched.c",
      "src/libdcethread/dcethread_attr_setstacksize.c",
      "src/libdcethread/dcethread_attr_getstacksize.c",
      "src/libdcethread/dcethread_setprio.c",
      "src/libdcethread/dcethread_getprio.c",
      "src/libdcethread/dcethread_mutexattr_create.c",
      "src/libdcethread/dcethread_mutexattr_delete.c",
      "src/libdcethread/dcethread_mutex_init.c",
      "src/libdcethread/dcethread_mutex_destroy.c",
      "src/libdcethread/dcethread_mutex_lock.c",
      "src/libdcethread/dcethread_mutex_unlock.c",
      "src/libdcethread/dcethread_mutex_trylock.c",
      "src/libdcethread/dcethread_condattr_create.c",
      "src/libdcethread/dcethread_condattr_delete.c",
      "src/libdcethread/dcethread_cond_init.c",
      "src/libdcethread/dcethread_cond_destroy.c",
      "src/libdcethread/dcethread_cond_broadcast.c",
      "src/libdcethread/dcethread_cond_signal.c",
      "src/libdcethread/dcethread_cond_timedwait.c",
      "src/libdcethread/dcethread_once.c",
      "src/libdcethread/dcethread_keycreate.c",
      "src/libdcethread/dcethread_setspecific.c",
      "src/libdcethread/dcethread_getspecific.c",
      "src/libdcethread/dcethread_enableasync.c",
      "src/libdcethread/dcethread_enableinterrupt.c",
      "src/libdcethread/dcethread_yield.c",
      "src/libdcethread/dcethread_equal.c",
      "src/libdcethread/dcethread_self.c",
      "src/libdcethread/dcethread_exc_init.c",
      "src/libdcethread/dcethread_exc_setstatus.c",
      "src/libdcethread/dcethread_exc_getstatus.c",
      "src/libdcethread/dcethread_exc_matches.c",
      "src/libdcethread/dcethread_exc_raise.c",
      "src/libdcethread/dcethread_frame_push.c",
      "src/libdcethread/dcethread_frame_pop.c",
      "src/uuid/uuid.c",
      "src/uuid/uuidsys.c",
      "src/uuid/get_802_addr.c",
      "src/ncklib/comauth.c",
      "src/ncklib/combind.c",
      "src/ncklib/comcall.c",
      "src/ncklib/comcthd.c",
      "src/ncklib/comep.c",
      "src/ncklib/comif.c",
      "src/ncklib/cominit.c",
      "src/ncklib/cominit_ux.c",
      "src/ncklib/comnaf.c",
      "src/ncklib/comnet.c",
      "src/ncklib/comnlsn.c",
      "src/ncklib/comobj.c",
      "src/ncklib/comp.c",
      "src/ncklib/comtwr.c",
      "src/ncklib/comtwrflr.c",
      "src/ncklib/comtwrref.c",
      "src/ncklib/comutil.c",
      "src/ncklib/nsldap.c",
      "src/ncklib/sec_id.c",
      "src/ncklib/dce_error.c",
      "src/ncklib/rpcclock.c",
      "src/ncklib/rpcdbg.c",
      "src/ncklib/rpclist.c",
      "src/ncklib/rpclog.c",
      "src/ncklib/rpcmem.c",
      "src/ncklib/rpcmutex.c",
      "src/ncklib/rpcrand.c",
      "src/ncklib/rpctimer.c",
      "src/ncklib/comsoc.c",
      "src/ncklib/comsoc_bsd.c",
      "src/ncklib/cnassoc.c",
      "src/ncklib/cnbind.c",
      "src/ncklib/cncall.c",
      "src/ncklib/cncasgsm.c",
      "src/ncklib/cncassm.c",
      "src/ncklib/cncclsm.c",
      "src/ncklib/cncthd.c",
      "src/ncklib/cnfbuf.c",
      "src/ncklib/cnid.c",
      "src/ncklib/cninit.c",
      "src/ncklib/cnmgmt.c",
      "src/ncklib/cnnet.c",
      "src/ncklib/cnp.c",
      "src/ncklib/cnpkt.c",
      "src/ncklib/cnrcvr.c",
      "src/ncklib/cnsasgsm.c",
      "src/ncklib/cnsassm.c",
      "src/ncklib/cnsclsm.c",
      "src/ncklib/cnsm.c",
      "src/ncklib/cnxfer.c",
      "src/ncklib/conv.c",
      "src/ncklib/dg.c",
      "src/ncklib/dgcall.c",
      "src/ncklib/dgccall.c",
      "src/ncklib/dgccallt.c",
      "src/ncklib/dgcct.c",
      "src/ncklib/dgclive.c",
      "src/ncklib/dgclsn.c",
      "src/ncklib/dgexec.c",
      "src/ncklib/dgfwd.c",
      "src/ncklib/dgglob.c",
      "src/ncklib/dghnd.c",
      "src/ncklib/dginit.c",
      "src/ncklib/dglossy.c",
      "src/ncklib/dglsn.c",
      "src/ncklib/dgpkt.c",
      "src/ncklib/dgrq.c",
      "src/ncklib/dgscall.c",
      "src/ncklib/dgsct.c",
      "src/ncklib/dgslive.c",
      "src/ncklib/dgslsn.c",
      "src/ncklib/dgsoc.c",
      "src/ncklib/dgutl.c",
      "src/ncklib/dgxq.c",
      "src/ncklib/ipnaf.c",
      "src/ncklib/ipnaf_linux.c",
      "src/ncklib/twr_ip.c",
      "src/ncklib/npnaf.c",
      "src/ncklib/npnaf_linux.c",
      "src/ncklib/twr_np.c",
      "src/ncklib/twr_uxd.c",
      "src/ncklib/gssauth.c",
      "src/ncklib/gssauthcn.c",
      "src/ncklib/comfork.c",
      "src/ncklib/ndrglob.c",
      "src/ncklib/mgmt.c",
      "src/ncklib/rpcsvc.c",
      &idl_dir.join("pickle_cstub.c").to_string_lossy(),
      &idl_dir.join("ep_cstub.c").to_string_lossy(),
      &idl_dir.join("mgmt_cstub.c").to_string_lossy(),
      &idl_dir.join("mgmt_sstub.c").to_string_lossy()
    ])
    .include(out_dir)
    .include("src/idl_lib")
    .include("src/ncklib")
    .include("src/ncklib/include/linux-gnu")
    .include("src/include")
    .include(idl_dir)
    .define("MIA", None)
    .define("API", None)
    .define("_POSIX_C_SOURCE", None)
    .define("DCETHREAD_ENFORCE_API", None)
    .define("_XOPEN_SOURCE", "500")
    .define("__EXTENSIONS__", None)
    .define("_POSIX_PTHREAD_SEMANTICS", None)
    .define("_ALL_SOURCE", "1")
    .define("_REENTRANT", None)
    .define("HAVE_CONFIG_H", None)
    .define("ATTRIBUTE_UNUSED", "__attribute__((unused))")
    .define("NCK", None)
    .define("PIC", None)
    .define("IMAGE_DIR", "\"/usr/local/lib\"")
    .define("CATALOG_DIR", "\"/usr/local/share/dce-rpc\"")
    .define("_FORTIFY_SOURCE", "2")
    .compile("idl_compiler");
}