extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
use std::cell::UnsafeCell;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type redisAsyncContext;
    pub type sockaddr;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: core::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(_: libc::c_int) -> libc::c_int;
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn time(__timer: *mut time_t) -> time_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn lround(_: libc::c_double) -> libc::c_long;
    fn hi_sdsnewlen(init: *const libc::c_void, initlen: size_t) -> hisds;
    fn hi_sdsnew(init: *const libc::c_char) -> hisds;
    fn hi_sdsempty() -> hisds;
    fn hi_sdsdup(s: hisds) -> hisds;
    fn hi_sdsfree(s: hisds);
    fn hi_sdscatlen(s: hisds, t: *const libc::c_void, len: size_t) -> hisds;
    fn hi_sdscat(s: hisds, t: *const libc::c_char) -> hisds;
    fn hi_sdscatsds(s: hisds, t: hisds) -> hisds;
    fn hi_sdscpy(s: hisds, t: *const libc::c_char) -> hisds;
    fn hi_sdscatprintf(s: hisds, fmt: *const libc::c_char, _: ...) -> hisds;
    fn hi_sdscatfmt(s: hisds, fmt: *const libc::c_char, _: ...) -> hisds;
    fn hi_sdsrange(s: hisds, start: ssize_t, end: ssize_t) -> libc::c_int;
    fn hi_sdscmp(s1: hisds, s2: hisds) -> libc::c_int;
    fn hi_sdsfreesplitres(tokens: *mut hisds, count: libc::c_int);
    fn hi_sdstolower(s: hisds);
    fn hi_sdstoupper(s: hisds);
    fn hi_sdsfromlonglong(value: libc::c_longlong) -> hisds;
    fn hi_sdscatrepr(s: hisds, p: *const libc::c_char, len: size_t) -> hisds;
    fn hi_sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut hisds;
    fn hi_sds_malloc(size: size_t) -> *mut libc::c_void;
    fn hi_sds_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn freeReplyObject(reply: *mut libc::c_void);
    fn redisConnect(ip: *const libc::c_char, port: libc::c_int) -> *mut redisContext;
    fn redisConnectUnix(path: *const libc::c_char) -> *mut redisContext;
    fn redisSetPushCallback(
        c: *mut redisContext,
        fn_0: Option::<redisPushFn>,
    ) -> Option::<redisPushFn>;
    fn redisFree(c: *mut redisContext);
    fn redisBufferRead(c: *mut redisContext) -> libc::c_int;
    fn redisGetReply(c: *mut redisContext, reply: *mut *mut libc::c_void) -> libc::c_int;
    fn redisAppendCommand(
        c: *mut redisContext,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn redisAppendCommandArgv(
        c: *mut redisContext,
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
        argvlen: *const size_t,
    ) -> libc::c_int;
    fn redisvCommand(
        c: *mut redisContext,
        format: *const libc::c_char,
        ap: core::ffi::VaList,
    ) -> *mut libc::c_void;
    fn redisCommand(
        c: *mut redisContext,
        format: *const libc::c_char,
        _: ...
    ) -> *mut libc::c_void;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictReplace(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn init_genrand64(seed: libc::c_ulonglong);
    fn dictGenHashFunction(key: *const libc::c_void, len: size_t) -> uint64_t;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listEmpty(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn linenoiseSetCompletionCallback(_: Option::<linenoiseCompletionCallback>);
    fn linenoiseSetHintsCallback(_: Option::<linenoiseHintsCallback>);
    fn linenoiseSetFreeHintsCallback(_: Option::<linenoiseFreeHintsCallback>);
    fn linenoiseAddCompletion(_: *mut linenoiseCompletions, _: *const libc::c_char);
    fn linenoise(prompt: *const libc::c_char) -> *mut libc::c_char;
    fn linenoiseFree(ptr: *mut libc::c_void);
    fn linenoiseHistoryAdd(line: *const libc::c_char) -> libc::c_int;
    fn linenoiseHistorySave(filename: *const libc::c_char) -> libc::c_int;
    fn linenoiseHistoryLoad(filename: *const libc::c_char) -> libc::c_int;
    fn linenoiseClearScreen();
    fn linenoiseSetMultiLine(ml: libc::c_int);
    fn linenoiseMaskModeEnable();
    fn linenoiseMaskModeDisable();
    fn anetResolve(
        err: *mut libc::c_char,
        host: *mut libc::c_char,
        ipbuf: *mut libc::c_char,
        ipbuf_len: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn anetNonBlock(err: *mut libc::c_char, fd: libc::c_int) -> libc::c_int;
    fn anetKeepAlive(
        err: *mut libc::c_char,
        fd: libc::c_int,
        interval: libc::c_int,
    ) -> libc::c_int;
    fn anetFormatAddr(
        fmt: *mut libc::c_char,
        fmt_len: size_t,
        ip: *mut libc::c_char,
        port: libc::c_int,
    ) -> libc::c_int;
    fn aeWait(
        fd: libc::c_int,
        mask: libc::c_int,
        milliseconds: libc::c_longlong,
    ) -> libc::c_int;
    fn cliSecureConnection(
        c: *mut redisContext,
        config_0: cliSSLconfig,
        err: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn cliWriteConn(
        c: *mut redisContext,
        buf: *const libc::c_char,
        buf_len: size_t,
    ) -> ssize_t;
    fn readArgFromStdin() -> hisds;
    fn getSdsArrayFromArgv(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        quoted: libc::c_int,
    ) -> *mut hisds;
    fn unquoteCString(str: *mut libc::c_char) -> hisds;
    fn parseRedisUri(
        uri: *const libc::c_char,
        tool_name: *const libc::c_char,
        connInfo: *mut cliConnInfo,
        tls_flag: *mut libc::c_int,
    );
    fn escapeJsonString(s: hisds, p: *const libc::c_char, len: size_t) -> hisds;
    fn redisGitSHA1() -> *mut libc::c_char;
    fn redisGitDirty() -> *mut libc::c_char;
    fn crc16(buf: *const libc::c_char, len: libc::c_int) -> uint16_t;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReadTask {
    pub type_0: libc::c_int,
    pub elements: libc::c_longlong,
    pub idx: libc::c_int,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReplyObjectFunctions {
    pub createString: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createArray: Option::<
        unsafe extern "C" fn(*const redisReadTask, size_t) -> *mut libc::c_void,
    >,
    pub createInteger: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_longlong) -> *mut libc::c_void,
    >,
    pub createDouble: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            libc::c_double,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createNil: Option::<
        unsafe extern "C" fn(*const redisReadTask) -> *mut libc::c_void,
    >,
    pub createBool: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_int) -> *mut libc::c_void,
    >,
    pub freeObject: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReader {
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub buf: *mut libc::c_char,
    pub pos: size_t,
    pub len: size_t,
    pub maxbuf: size_t,
    pub maxelements: libc::c_longlong,
    pub task: *mut *mut redisReadTask,
    pub tasks: libc::c_int,
    pub ridx: libc::c_int,
    pub reply: *mut libc::c_void,
    pub fn_0: *mut redisReplyObjectFunctions,
    pub privdata: *mut libc::c_void,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type hisds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hisdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hisdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hisdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hisdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContext {
    pub funcs: *const redisContextFuncs,
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub fd: redisFD,
    pub flags: libc::c_int,
    pub obuf: *mut libc::c_char,
    pub reader: *mut redisReader,
    pub connection_type: redisConnectionType,
    pub connect_timeout: *mut timeval,
    pub command_timeout: *mut timeval,
    pub tcp: C2RustUnnamed_1,
    pub unix_sock: C2RustUnnamed_0,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub privctx: *mut libc::c_void,
    pub push_cb: Option::<redisPushFn>,
}
pub type redisPushFn = unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub host: *mut libc::c_char,
    pub source_addr: *mut libc::c_char,
    pub port: libc::c_int,
}
pub type redisConnectionType = libc::c_uint;
pub const REDIS_CONN_USERFD: redisConnectionType = 2;
pub const REDIS_CONN_UNIX: redisConnectionType = 1;
pub const REDIS_CONN_TCP: redisConnectionType = 0;
pub type redisFD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContextFuncs {
    pub free_privctx: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub async_read: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub async_write: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub read: Option::<
        unsafe extern "C" fn(*mut redisContext, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<unsafe extern "C" fn(*mut redisContext) -> ssize_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReply {
    pub type_0: libc::c_int,
    pub integer: libc::c_longlong,
    pub dval: libc::c_double,
    pub len: size_t,
    pub str_0: *mut libc::c_char,
    pub vtype: [libc::c_char; 4],
    pub elements: size_t,
    pub element: *mut *mut redisReply,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictEntry {
    pub key: *mut libc::c_void,
    pub v: C2RustUnnamed_2,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub val: *mut libc::c_void,
    pub u64_0: uint64_t,
    pub s64: int64_t,
    pub d: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dict {
    pub type_0: *mut dictType,
    pub ht_table: [*mut *mut dictEntry; 2],
    pub ht_used: [libc::c_ulong; 2],
    pub rehashidx: libc::c_long,
    pub pauserehash: int16_t,
    pub ht_size_exp: [libc::c_schar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictType {
    pub hashFunction: Option::<unsafe extern "C" fn(*const libc::c_void) -> uint64_t>,
    pub keyDup: Option::<
        unsafe extern "C" fn(*mut dict, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub valDup: Option::<
        unsafe extern "C" fn(*mut dict, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub keyCompare: Option::<
        unsafe extern "C" fn(
            *mut dict,
            *const libc::c_void,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub keyDestructor: Option::<
        unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
    >,
    pub valDestructor: Option::<
        unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
    >,
    pub expandAllowed: Option::<
        unsafe extern "C" fn(size_t, libc::c_double) -> libc::c_int,
    >,
    pub dictEntryMetadataBytes: Option::<unsafe extern "C" fn(*mut dict) -> size_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictIterator {
    pub d: *mut dict,
    pub index: libc::c_long,
    pub table: libc::c_int,
    pub safe: libc::c_int,
    pub entry: *mut dictEntry,
    pub nextEntry: *mut dictEntry,
    pub fingerprint: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listNode {
    pub prev: *mut listNode,
    pub next: *mut listNode,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listIter {
    pub next: *mut listNode,
    pub direction: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub head: *mut listNode,
    pub tail: *mut listNode,
    pub dup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub match_0: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub len: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linenoiseCompletions {
    pub len: size_t,
    pub cvec: *mut *mut libc::c_char,
}
pub type linenoiseCompletionCallback = unsafe extern "C" fn(
    *const libc::c_char,
    *mut linenoiseCompletions,
) -> ();
pub type linenoiseHintsCallback = unsafe extern "C" fn(
    *const libc::c_char,
    *mut libc::c_int,
    *mut libc::c_int,
) -> *mut libc::c_char;
pub type linenoiseFreeHintsCallback = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct commandHelp {
    pub name: *mut libc::c_char,
    pub params: *mut libc::c_char,
    pub summary: *mut libc::c_char,
    pub group: libc::c_int,
    pub since: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cliSSLconfig {
    pub sni: *mut libc::c_char,
    pub cacert: *mut libc::c_char,
    pub cacertdir: *mut libc::c_char,
    pub skip_cert_verify: libc::c_int,
    pub cert: *mut libc::c_char,
    pub key: *mut libc::c_char,
    pub ciphers: *mut libc::c_char,
    pub ciphersuites: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cliConnInfo {
    pub hostip: *mut libc::c_char,
    pub hostport: libc::c_int,
    pub input_dbnum: libc::c_int,
    pub auth: *mut libc::c_char,
    pub user: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct commandDocs {
    pub name: *mut libc::c_char,
    pub params: *mut libc::c_char,
    pub summary: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub since: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterManagerCommand {
    pub name: *mut libc::c_char,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub stdin_arg: hisds,
    pub flags: libc::c_int,
    pub replicas: libc::c_int,
    pub from: *mut libc::c_char,
    pub to: *mut libc::c_char,
    pub weight: *mut *mut libc::c_char,
    pub weight_argc: libc::c_int,
    pub master_id: *mut libc::c_char,
    pub slots: libc::c_int,
    pub timeout: libc::c_int,
    pub pipeline: libc::c_int,
    pub threshold: libc::c_float,
    pub backup_dir: *mut libc::c_char,
    pub from_user: *mut libc::c_char,
    pub from_pass: *mut libc::c_char,
    pub from_askpass: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub conn_info: cliConnInfo,
    pub hostsocket: *mut libc::c_char,
    pub tls: libc::c_int,
    pub sslconfig: cliSSLconfig,
    pub repeat: libc::c_long,
    pub interval: libc::c_long,
    pub dbnum: libc::c_int,
    pub interactive: libc::c_int,
    pub shutdown: libc::c_int,
    pub monitor_mode: libc::c_int,
    pub pubsub_mode: libc::c_int,
    pub blocking_state_aborted: libc::c_int,
    pub latency_mode: libc::c_int,
    pub latency_dist_mode: libc::c_int,
    pub latency_history: libc::c_int,
    pub lru_test_mode: libc::c_int,
    pub lru_test_sample_size: libc::c_longlong,
    pub cluster_mode: libc::c_int,
    pub cluster_reissue_command: libc::c_int,
    pub cluster_send_asking: libc::c_int,
    pub slave_mode: libc::c_int,
    pub pipe_mode: libc::c_int,
    pub pipe_timeout: libc::c_int,
    pub getrdb_mode: libc::c_int,
    pub get_functions_rdb_mode: libc::c_int,
    pub stat_mode: libc::c_int,
    pub scan_mode: libc::c_int,
    pub intrinsic_latency_mode: libc::c_int,
    pub intrinsic_latency_duration: libc::c_int,
    pub pattern: hisds,
    pub rdb_filename: *mut libc::c_char,
    pub bigkeys: libc::c_int,
    pub memkeys: libc::c_int,
    pub memkeys_samples: libc::c_uint,
    pub hotkeys: libc::c_int,
    pub stdin_lastarg: libc::c_int,
    pub stdin_tag_arg: libc::c_int,
    pub stdin_tag_name: *mut libc::c_char,
    pub askpass: libc::c_int,
    pub quoted_input: libc::c_int,
    pub output: libc::c_int,
    pub push_output: libc::c_int,
    pub mb_delim: hisds,
    pub cmd_delim: hisds,
    pub prompt: [libc::c_char; 128],
    pub eval: *mut libc::c_char,
    pub eval_ldb: libc::c_int,
    pub eval_ldb_sync: libc::c_int,
    pub eval_ldb_end: libc::c_int,
    pub enable_ldb_on_eval: libc::c_int,
    pub last_cmd_type: libc::c_int,
    pub verbose: libc::c_int,
    pub set_errcode: libc::c_int,
    pub cluster_manager_command: clusterManagerCommand,
    pub no_auth_warning: libc::c_int,
    pub resp2: libc::c_int,
    pub resp3: libc::c_int,
    pub in_multi: libc::c_int,
    pub pre_multi_dbnum: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pref {
    pub hints: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct helpEntry {
    pub type_0: libc::c_int,
    pub argc: libc::c_int,
    pub argv: *mut hisds,
    pub full: hisds,
    pub org: commandDocs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterManager {
    pub nodes: *mut list,
    pub errors: *mut list,
    pub unreachable_masters: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterManagerNode {
    pub context: *mut redisContext,
    pub name: hisds,
    pub ip: *mut libc::c_char,
    pub port: libc::c_int,
    pub bus_port: libc::c_int,
    pub current_epoch: uint64_t,
    pub ping_sent: time_t,
    pub ping_recv: time_t,
    pub flags: libc::c_int,
    pub flags_str: *mut list,
    pub replicate: hisds,
    pub dirty: libc::c_int,
    pub slots: [uint8_t; 16384],
    pub slots_count: libc::c_int,
    pub replicas_count: libc::c_int,
    pub friends: *mut list,
    pub migrating: *mut hisds,
    pub importing: *mut hisds,
    pub migrating_count: libc::c_int,
    pub importing_count: libc::c_int,
    pub weight: libc::c_float,
    pub balance: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterManagerNodeArray {
    pub nodes: *mut *mut clusterManagerNode,
    pub alloc: *mut *mut clusterManagerNode,
    pub len: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterManagerReshardTableItem {
    pub source: *mut clusterManagerNode,
    pub slot: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterManagerLink {
    pub node_name: hisds,
    pub node_addr: hisds,
    pub connected: libc::c_int,
    pub handshaking: libc::c_int,
}
pub type clusterManagerCommandProc = unsafe extern "C" fn(
    libc::c_int,
    *mut *mut libc::c_char,
) -> libc::c_int;
pub type clusterManagerOnReplyError = Option::<
    unsafe extern "C" fn(
        *mut redisReply,
        *mut clusterManagerNode,
        libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterManagerOptionDef {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterManagerCommandDef {
    pub name: *mut libc::c_char,
    pub proc_0: Option::<clusterManagerCommandProc>,
    pub arity: libc::c_int,
    pub args: *mut libc::c_char,
    pub options: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct distsamples {
    pub max: libc::c_longlong,
    pub count: libc::c_longlong,
    pub character: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct typeinfo {
    pub name: *mut libc::c_char,
    pub sizecmd: *mut libc::c_char,
    pub sizeunit: *mut libc::c_char,
    pub biggest: libc::c_ulonglong,
    pub count: libc::c_ulonglong,
    pub totalsize: libc::c_ulonglong,
    pub biggest_key: hisds,
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    return strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn hi_sdslen(s: hisds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(core::mem::size_of::<hisdshdr8>() as libc::c_ulong as isize))
                as *mut hisdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s
                .offset(
                    -(core::mem::size_of::<hisdshdr16>() as libc::c_ulong as isize),
                ) as *mut hisdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s
                .offset(
                    -(core::mem::size_of::<hisdshdr32>() as libc::c_ulong as isize),
                ) as *mut hisdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s
                .offset(
                    -(core::mem::size_of::<hisdshdr64>() as libc::c_ulong as isize),
                ) as *mut hisdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
static mut commandGroups: [*mut libc::c_char; 16] = [
    b"generic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sorted-set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pubsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"transactions\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"connection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"server\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"scripting\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hyperloglog\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"geo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"stream\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bitmap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut commandHelp: [commandHelp; 366] = [
    {
        let mut init = commandHelp {
            name: b"ACL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for Access List Control commands \0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL CAT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[categoryname]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"List the ACL categories or the commands inside a category\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL DELUSER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"username [username ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove the specified ACL users and the associated rules\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL DRYRUN\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"username command [arg [arg ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Returns whether the user can execute the given command without executing the command.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL GENPASS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[bits]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Generate a pseudorandom secure password to use for ACL users\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL GETUSER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"username\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the rules for a specific ACL user\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL HELP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL LIST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"List the current ACL rules in ACL config file format\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL LOAD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Reload the ACLs from the configured ACL file\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL LOG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[count|RESET]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"List latest events denied because of ACLs in place\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL SAVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Save the current ACL rules in the configured ACL file\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL SETUSER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"username [rule [rule ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Modify or create the rules for a specific ACL user\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL USERS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"List the username of all the configured ACL rules\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ACL WHOAMI\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the name of the user associated to the current connection\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"APPEND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Append a value to a key\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ASKING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Sent by cluster clients after an -ASK redirect\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"AUTH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[username] password\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Authenticate to the server\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BGREWRITEAOF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Asynchronously rewrite the append-only file\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BGSAVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[SCHEDULE]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Asynchronously save the dataset to disk\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BITCOUNT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [start end [BYTE|BIT]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Count set bits in a string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 15 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BITFIELD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key GET encoding offset|[OVERFLOW WRAP|SAT|FAIL] SET encoding offset value|INCRBY encoding offset increment [GET encoding offset|[OVERFLOW WRAP|SAT|FAIL] SET encoding offset value|INCRBY encoding offset increment ...]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Perform arbitrary bitfield integer operations on strings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 15 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BITFIELD_RO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key GET encoding offset [GET encoding offset ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Perform arbitrary bitfield integer operations on strings. Read-only variant of BITFIELD\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 15 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BITOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"operation destkey key [key ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Perform bitwise operations between strings\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 15 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BITPOS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key bit [start [end [BYTE|BIT]]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Find first bit set or clear in a string\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 15 as libc::c_int,
            since: b"2.8.7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BLMOVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"source destination LEFT|RIGHT LEFT|RIGHT timeout\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Pop an element from a list, push it to another list and return it; or block until one is available\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BLMPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"timeout numkeys key [key ...] LEFT|RIGHT [COUNT count]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Pop elements from a list, or block until one is available\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BLPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...] timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and get the first element in a list, or block until one is available\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BRPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...] timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and get the last element in a list, or block until one is available\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BRPOPLPUSH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"source destination timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Pop an element from a list, push it to another list and return it; or block until one is available\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BZMPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"timeout numkeys key [key ...] MIN|MAX [COUNT count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Remove and return members with scores in a sorted set or block until one is available\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BZPOPMAX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...] timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and return the member with the highest score from one or more sorted sets, or block until one is available\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"BZPOPMIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...] timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and return the member with the lowest score from one or more sorted sets, or block until one is available\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for client connection commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"2.4.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT CACHING\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"YES|NO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Instruct the server about tracking or not keys in the next request\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT GETNAME\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the current connection name\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"2.6.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT GETREDIR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get tracking notifications redirection client ID if any\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT ID\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Returns the client ID for the current connection\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT INFO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Returns information about the current client connection.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT KILL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"ip:port|[ID client-id]|[TYPE NORMAL|MASTER|SLAVE|REPLICA|PUBSUB]|[USER username]|[ADDR ip:port]|[LADDR ip:port]|[SKIPME yes/no] [[ID client-id]|[TYPE NORMAL|MASTER|SLAVE|REPLICA|PUBSUB]|[USER username]|[ADDR ip:port]|[LADDR ip:port]|[SKIPME yes/no] ...]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Kill the connection of a client\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"2.4.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT LIST\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[TYPE NORMAL|MASTER|REPLICA|PUBSUB] [ID client-id [client-id ...]]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the list of client connections\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"2.4.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT NO-EVICT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"ON|OFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Set client eviction mode for the current connection\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT PAUSE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"timeout [WRITE|ALL]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Stop processing commands from clients for some time\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"2.9.50\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT REPLY\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"ON|OFF|SKIP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Instruct the server whether to reply to commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT SETNAME\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"connection-name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the current connection name\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"2.6.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT TRACKING\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"ON|OFF [REDIRECT client-id] [PREFIX prefix [PREFIX prefix ...]] [BCAST] [OPTIN] [OPTOUT] [NOLOOP]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Enable or disable server assisted client side caching support\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT TRACKINGINFO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return information about server assisted client side caching for the current connection\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT UNBLOCK\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"client-id [TIMEOUT|ERROR]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Unblock a client blocked in a blocking command from a different connection\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLIENT UNPAUSE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Resume processing of clients that were paused\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for cluster commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER ADDSLOTS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"slot [slot ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Assign new hash slots to receiving node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER ADDSLOTSRANGE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"start-slot end-slot [start-slot end-slot ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Assign new hash slots to receiving node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER BUMPEPOCH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Advance the cluster config epoch\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER COUNT-FAILURE-REPORTS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"node-id\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Return the number of failure reports active for a given node\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER COUNTKEYSINSLOT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"slot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the number of local keys in the specified hash slot\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER DELSLOTS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"slot [slot ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set hash slots as unbound in receiving node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER DELSLOTSRANGE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"start-slot end-slot [start-slot end-slot ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Set hash slots as unbound in receiving node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER FAILOVER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[FORCE|TAKEOVER]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Forces a replica to perform a manual failover of its master.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER FLUSHSLOTS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Delete a node's own slots information\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER FORGET\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"node-id\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove a node from the nodes table\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER GETKEYSINSLOT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"slot count\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Return local key names in the specified hash slot\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER INFO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Provides info about Redis Cluster node state\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER KEYSLOT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Returns the hash slot of the specified key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER LINKS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Returns a list of all TCP links to and from peer nodes in cluster\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER MEET\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"ip port [cluster_bus_port]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Force a node cluster to handshake with another node\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER MYID\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the node id\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER NODES\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get Cluster config for the node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER REPLICAS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"node-id\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"List replica nodes of the specified master node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER REPLICATE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"node-id\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Reconfigure a node as a replica of the specified master node\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER RESET\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[HARD|SOFT]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Reset a Redis Cluster node\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER SAVECONFIG\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Forces the node to save cluster state on disk\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER SET-CONFIG-EPOCH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"config-epoch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the configuration epoch in a new node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER SETSLOT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"slot IMPORTING node-id|MIGRATING node-id|NODE node-id|STABLE\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Bind a hash slot to a specific node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER SHARDS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get array of cluster slots to node mappings\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER SLAVES\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"node-id\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"List replica nodes of the specified master node\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CLUSTER SLOTS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get array of Cluster slot to node mappings\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COMMAND\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get array of Redis command details\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COMMAND COUNT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get total number of Redis commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COMMAND DOCS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[command-name [command-name ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Get array of specific Redis command documentation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COMMAND GETKEYS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Extract keys given a full Redis command\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COMMAND GETKEYSANDFLAGS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Extract keys and access flags given a full Redis command\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COMMAND HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COMMAND INFO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[command-name [command-name ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Get array of specific Redis command details, or all when no argument is given.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COMMAND LIST\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[FILTERBY MODULE module-name|ACLCAT category|PATTERN pattern]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get an array of Redis command names\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CONFIG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for server configuration commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CONFIG GET\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"parameter [parameter ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the values of configuration parameters\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CONFIG HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CONFIG RESETSTAT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Reset the stats returned by INFO\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CONFIG REWRITE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Rewrite the configuration file with the in memory configuration\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"CONFIG SET\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"parameter value [parameter value ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Set configuration parameters to the given values\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"COPY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"source destination [DB destination-db] [REPLACE]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Copy a key\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"DBSIZE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the number of keys in the selected database\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"DEBUG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for debugging commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"DECR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Decrement the integer value of a key by one\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"DECRBY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key decrement\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Decrement the integer value of a key by the given number\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"DEL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Delete a key\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"DISCARD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Discard all commands issued after MULTI\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 7 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"DUMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a serialized version of the value stored at the specified key.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ECHO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"message\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Echo the given string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EVAL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"script numkeys [key [key ...]] [arg [arg ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Execute a Lua script server side\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EVALSHA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"sha1 numkeys [key [key ...]] [arg [arg ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Execute a Lua script server side\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EVALSHA_RO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"sha1 numkeys [key [key ...]] [arg [arg ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Execute a read-only Lua script server side\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EVAL_RO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"script numkeys [key [key ...]] [arg [arg ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Execute a read-only Lua script server side\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EXEC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Execute all commands issued after MULTI\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 7 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EXISTS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Determine if a key exists\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EXPIRE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key seconds [NX|XX|GT|LT]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set a key's time to live in seconds\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EXPIREAT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key unix-time-seconds [NX|XX|GT|LT]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Set the expiration for a key as a UNIX timestamp\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"EXPIRETIME\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the expiration Unix timestamp for a key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FAILOVER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[TO host port [FORCE]] [ABORT] [TIMEOUT milliseconds]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Start a coordinated failover between this server and one of its replicas.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FCALL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"function numkeys [key [key ...]] [arg [arg ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Invoke a function\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FCALL_RO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"function numkeys [key [key ...]] [arg [arg ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Invoke a read-only function\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FLUSHALL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[ASYNC|SYNC]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove all keys from all databases\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FLUSHDB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[ASYNC|SYNC]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove all keys from the current database\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for function commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION DELETE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"library-name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Delete a function by name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION DUMP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Dump all functions into a serialized binary payload\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION FLUSH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[ASYNC|SYNC]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Deleting all functions\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION KILL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Kill the function currently in execution.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION LIST\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[LIBRARYNAME library-name-pattern] [WITHCODE]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"List information about all the functions\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION LOAD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[REPLACE] function-code\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Create a function with the given arguments (name, code, description)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION RESTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"serialized-value [FLUSH|APPEND|REPLACE]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Restore all the functions on the given payload\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"FUNCTION STATS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return information about the function currently running (name, description, duration)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEOADD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [NX|XX] [CH] longitude latitude member [longitude latitude member ...]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Add one or more geospatial items in the geospatial index represented using a sorted set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEODIST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member1 member2 [M|KM|FT|MI]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Returns the distance between two members of a geospatial index\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEOHASH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member [member ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Returns members of a geospatial index as standard geohash strings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEOPOS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member [member ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Returns longitude and latitude of members of a geospatial index\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEORADIUS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key longitude latitude radius M|KM|FT|MI [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT count [ANY]] [ASC|DESC] [STORE key] [STOREDIST key]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Query a sorted set representing a geospatial index to fetch members matching a given maximum distance from a point\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEORADIUSBYMEMBER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key member radius M|KM|FT|MI [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT count [ANY]] [ASC|DESC] [STORE key] [STOREDIST key]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Query a sorted set representing a geospatial index to fetch members matching a given maximum distance from a member\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEORADIUSBYMEMBER_RO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key member radius M|KM|FT|MI [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT count [ANY]] [ASC|DESC]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A read-only variant for GEORADIUSBYMEMBER\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"3.2.10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEORADIUS_RO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key longitude latitude radius M|KM|FT|MI [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT count [ANY]] [ASC|DESC]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A read-only variant for GEORADIUS\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"3.2.10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEOSEARCH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key FROMMEMBER member|FROMLONLAT longitude latitude BYRADIUS radius M|KM|FT|MI|BYBOX width height M|KM|FT|MI [ASC|DESC] [COUNT count [ANY]] [WITHCOORD] [WITHDIST] [WITHHASH]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Query a sorted set representing a geospatial index to fetch members inside an area of a box or a circle.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GEOSEARCHSTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"destination source FROMMEMBER member|FROMLONLAT longitude latitude BYRADIUS radius M|KM|FT|MI|BYBOX width height M|KM|FT|MI [ASC|DESC] [COUNT count [ANY]] [STOREDIST]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Query a sorted set representing a geospatial index to fetch members inside an area of a box or a circle, and store the result in another key.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 13 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the value of a key\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GETBIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key offset\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Returns the bit value at offset in the string value stored at key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 15 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GETDEL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the value of a key and delete the key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GETEX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [EX seconds|PX milliseconds|EXAT unix-time-seconds|PXAT unix-time-milliseconds|PERSIST]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the value of a key and optionally set its expiration\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GETRANGE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key start end\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get a substring of the string stored at a key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"2.4.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"GETSET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the string value of a key and return its old value\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HDEL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field [field ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Delete one or more hash fields\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HELLO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[protover [AUTH username password] [SETNAME clientname]]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Handshake with Redis\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"6.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HEXISTS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Determine if a hash field exists\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HGET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the value of a hash field\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HGETALL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get all the fields and values in a hash\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HINCRBY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field increment\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Increment the integer value of a hash field by the given number\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HINCRBYFLOAT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key field increment\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Increment the float value of a hash field by the given amount\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HKEYS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get all the fields in a hash\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the number of fields in a hash\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HMGET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field [field ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the values of all the given hash fields\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HMSET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field value [field value ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Set multiple hash fields to multiple values\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HRANDFIELD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key [count [WITHVALUES]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get one or multiple random fields from a hash\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HSCAN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key cursor [MATCH pattern] [COUNT count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Incrementally iterate hash fields and associated values\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HSET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field value [field value ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Set the string value of a hash field\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HSETNX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the value of a hash field, only if the field does not exist\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HSTRLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key field\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the length of the value of a hash field\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"HVALS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get all the values in a hash\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 5 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"INCR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Increment the integer value of a key by one\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"INCRBY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key increment\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Increment the integer value of a key by the given amount\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"INCRBYFLOAT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key increment\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Increment the float value of a key by the given amount\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"INFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[section [section ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get information and statistics about the server\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"KEYS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"pattern\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Find all keys matching the given pattern\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LASTSAVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the UNIX time stamp of the last successful save to disk\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LATENCY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for latency diagnostics commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LATENCY DOCTOR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a human readable latency analysis report.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LATENCY GRAPH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"event\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a latency graph for the event.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LATENCY HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LATENCY HISTOGRAM\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[command [command ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Return the cumulative distribution of latencies of a subset of commands or all.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LATENCY HISTORY\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"event\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return timestamp-latency samples for the event.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LATENCY LATEST\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the latest latency samples for all events.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LATENCY RESET\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[event [event ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Reset latency data for one or more events.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LCS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key1 key2 [LEN] [IDX] [MINMATCHLEN len] [WITHMATCHLEN]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Find longest common substring\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LINDEX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key index\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get an element from a list by its index\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LINSERT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key BEFORE|AFTER pivot element\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Insert an element before or after another element in a list\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the length of a list\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LMOVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"source destination LEFT|RIGHT LEFT|RIGHT\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Pop an element from a list, push it to another list and return it\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LMPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"numkeys key [key ...] LEFT|RIGHT [COUNT count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Pop elements from a list\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LOLWUT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[VERSION version]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Display some computer art and the Redis version\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and get the first elements in a list\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LPOS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key element [RANK rank] [COUNT num-matches] [MAXLEN len]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the index of matching elements on a list\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"6.0.6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LPUSH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key element [element ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Prepend one or multiple elements to a list\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LPUSHX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key element [element ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Prepend an element to a list, only if the list exists\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LRANGE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key start stop\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get a range of elements from a list\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LREM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key count element\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove elements from a list\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LSET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key index element\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the value of an element in a list by its index\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"LTRIM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key start stop\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Trim a list to the specified range\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MEMORY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for memory diagnostics commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MEMORY DOCTOR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Outputs memory problems report\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MEMORY HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MEMORY MALLOC-STATS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show allocator internal stats\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MEMORY PURGE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Ask the allocator to release memory\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MEMORY STATS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show memory usage details\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MEMORY USAGE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key [SAMPLES count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Estimate the memory usage of a key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MGET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the values of all the given keys\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MIGRATE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"host port key| destination-db timeout [COPY] [REPLACE] [[AUTH password]|[AUTH2 username password]] [KEYS key [key ...]]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Atomically transfer a key from a Redis instance to another one.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MODULE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for module commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MODULE HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MODULE LIST\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"List all modules loaded by the server\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MODULE LOAD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"path [arg [arg ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Load a module\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MODULE LOADEX\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"path [CONFIG name value [CONFIG name value ...]] [ARGS arg [arg ...]]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Load a module with extended parameters\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MODULE UNLOAD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Unload a module\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MONITOR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Listen for all requests received by the server in real time\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MOVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key db\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Move a key to another database\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MSET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key value [key value ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set multiple keys to multiple values\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MSETNX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key value [key value ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set multiple keys to multiple values, only if none of the keys exist\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"MULTI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Mark the start of a transaction block\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 7 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"OBJECT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for object introspection commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.2.3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"OBJECT ENCODING\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Inspect the internal encoding of a Redis object\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.2.3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"OBJECT FREQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the logarithmic access frequency counter of a Redis object\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"OBJECT HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"OBJECT IDLETIME\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the time since a Redis object was last accessed\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.2.3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"OBJECT REFCOUNT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the number of references to the value of the key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.2.3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PERSIST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Remove the expiration from a key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PEXPIRE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key milliseconds [NX|XX|GT|LT]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Set a key's time to live in milliseconds\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PEXPIREAT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key unix-time-milliseconds [NX|XX|GT|LT]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Set the expiration for a key as a UNIX timestamp specified in milliseconds\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PEXPIRETIME\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the expiration Unix timestamp for a key in milliseconds\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PFADD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [element [element ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Adds the specified elements to the specified HyperLogLog.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 11 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PFCOUNT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Return the approximated cardinality of the set(s) observed by the HyperLogLog at key(s).\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 11 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PFDEBUG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"subcommand key\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Internal commands for debugging HyperLogLog values\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 11 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PFMERGE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"destkey sourcekey [sourcekey ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Merge N different HyperLogLogs into a single one.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 11 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PFSELFTEST\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"An internal command for testing HyperLogLog values\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 11 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[message]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Ping the server\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PSETEX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key milliseconds value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the value and expiration in milliseconds of a key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PSUBSCRIBE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"pattern [pattern ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Listen for messages published to channels matching the given patterns\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PSYNC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"replicationid offset\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Internal command used for replication\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PTTL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the time to live for a key in milliseconds\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUBLISH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"channel message\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Post a message to a channel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUBSUB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for Pub/Sub commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUBSUB CHANNELS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[pattern]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"List active channels\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUBSUB HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUBSUB NUMPAT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the count of unique patterns pattern subscriptions\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUBSUB NUMSUB\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[channel [channel ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the count of subscribers for channels\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUBSUB SHARDCHANNELS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[pattern]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"List active shard channels\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUBSUB SHARDNUMSUB\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[shardchannel [shardchannel ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the count of subscribers for shard channels\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"PUNSUBSCRIBE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[pattern [pattern ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Stop listening for messages posted to channels matching the given patterns\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"QUIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Close the connection\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RANDOMKEY\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a random key from the keyspace\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"READONLY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Enables read queries for a connection to a cluster replica node\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"READWRITE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Disables read queries for a connection to a cluster replica node\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 12 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RENAME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key newkey\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Rename a key\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RENAMENX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key newkey\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Rename a key, only if the new key does not exist\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"REPLCONF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"An internal command for configuring the replication stream\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"REPLICAOF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"host port\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Make the server a replica of another instance, or promote it as master.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RESET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Reset the connection\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RESTORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key ttl serialized-value [REPLACE] [ABSTTL] [IDLETIME seconds] [FREQ frequency]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Create a key using the provided serialized value, previously obtained using DUMP.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RESTORE-ASKING\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key ttl serialized-value [REPLACE] [ABSTTL] [IDLETIME seconds] [FREQ frequency]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"An internal command for migrating keys in a cluster\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ROLE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the role of the instance in the context of replication\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.8.12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and get the last elements in a list\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RPOPLPUSH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"source destination\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove the last element in a list, prepend it to another list and return it\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RPUSH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key element [element ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Append one or multiple elements to a list\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"RPUSHX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key element [element ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Append an element to a list, only if the list exists\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 2 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SADD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member [member ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Add one or more members to a set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SAVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Synchronously save the dataset to disk\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCAN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"cursor [MATCH pattern] [COUNT count] [TYPE type]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Incrementally iterate the keys space\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCARD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the number of members in a set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCRIPT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for Lua scripts management commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCRIPT DEBUG\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"YES|SYNC|NO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the debug mode for executed scripts.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"3.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCRIPT EXISTS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"sha1 [sha1 ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Check existence of scripts in the script cache.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCRIPT FLUSH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[ASYNC|SYNC]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove all the scripts from the script cache.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCRIPT HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCRIPT KILL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Kill the script currently in execution.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SCRIPT LOAD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"script\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Load the specified Lua script into the script cache.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 10 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SDIFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Subtract multiple sets\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SDIFFSTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"destination key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Subtract multiple sets and store the resulting set in a key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SELECT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"index\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Change the selected database for the current connection\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 8 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key value [NX|XX] [GET] [EX seconds|PX milliseconds|EXAT unix-time-seconds|PXAT unix-time-milliseconds|KEEPTTL]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Set the string value of a key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SETBIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key offset value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Sets or clears the bit at offset in the string value stored at key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 15 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SETEX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key seconds value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the value and expiration of a key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SETNX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Set the value of a key, only if the key does not exist\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SETRANGE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key offset value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Overwrite part of a string at key starting at the specified offset\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SHUTDOWN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[NOSAVE|SAVE] [NOW] [FORCE] [ABORT]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Synchronously save the dataset to disk and then shut down the server\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SINTER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Intersect multiple sets\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SINTERCARD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"numkeys key [key ...] [LIMIT limit]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Intersect multiple sets and return the cardinality of the result\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SINTERSTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"destination key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Intersect multiple sets and store the resulting set in a key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SISMEMBER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key member\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Determine if a given value is a member of a set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SLAVEOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"host port\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Make the server a replica of another instance, or promote it as master.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SLOWLOG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for slow log commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.2.12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SLOWLOG GET\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the slow log's entries\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.2.12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SLOWLOG HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SLOWLOG LEN\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the slow log's length\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.2.12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SLOWLOG RESET\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Clear all entries from the slow log\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.2.12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SMEMBERS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get all the members in a set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SMISMEMBER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key member [member ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Returns the membership associated with the given elements for a set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SMOVE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"source destination member\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Move a member from one set to another\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SORT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [BY pattern] [LIMIT offset count] [GET pattern [GET pattern ...]] [ASC|DESC] [ALPHA] [STORE destination]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Sort the elements in a list, set or sorted set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SORT_RO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [BY pattern] [LIMIT offset count] [GET pattern [GET pattern ...]] [ASC|DESC] [ALPHA]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Sort the elements in a list, set or sorted set. Read-only variant of SORT.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and return one or multiple random members from a set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SPUBLISH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"shardchannel message\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Post a message to a shard channel\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SRANDMEMBER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key [count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get one or multiple random members from a set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SREM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member [member ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove one or more members from a set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SSCAN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key cursor [MATCH pattern] [COUNT count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Incrementally iterate Set elements\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SSUBSCRIBE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"shardchannel [shardchannel ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Listen for messages published to the given shard channels\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"STRLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the length of the value stored in a key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SUBSCRIBE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"channel [channel ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Listen for messages published to the given channels\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SUBSTR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key start end\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get a substring of the string stored at a key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 1 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SUNION\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Add multiple sets\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SUNIONSTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"destination key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Add multiple sets and store the resulting set in a key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 3 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SUNSUBSCRIBE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[shardchannel [shardchannel ...]]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Stop listening for messages posted to the given shard channels\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SWAPDB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"index1 index2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Swaps two Redis databases\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"SYNC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Internal command used for replication\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"TIME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the current server time\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 9 as libc::c_int,
            since: b"2.6.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"TOUCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Alters the last access time of a key(s). Returns the number of existing keys specified.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"3.2.1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"TTL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the time to live for a key in seconds\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"TYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Determine the type stored at key\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"1.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"UNLINK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Delete a key asynchronously in another thread. Otherwise it is just as DEL, but non blocking.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"4.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"UNSUBSCRIBE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"[channel [channel ...]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Stop listening for messages posted to the given channels\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 6 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"UNWATCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Forget about all watched keys\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 7 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"WAIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"numreplicas timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Wait for the synchronous replication of all the write commands sent in the context of the current connection\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 0 as libc::c_int,
            since: b"3.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"WATCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [key ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Watch the given keys to determine execution of the MULTI/EXEC block\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 7 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XACK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key group id [id ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Marks a pending message as correctly processed, effectively removing it from the pending entries list of the consumer group. Return value of the command is the number of messages successfully acknowledged, that is, the IDs we were actually able to resolve in the PEL.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XADD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [NOMKSTREAM] [MAXLEN|MINID [=|~] threshold [LIMIT count]] *|id field value [field value ...]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Appends a new entry to a stream\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XAUTOCLAIM\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key group consumer min-idle-time start [COUNT count] [JUSTID]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Changes (or acquires) ownership of messages in a consumer group, as if the messages were delivered to the specified consumer.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XCLAIM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key group consumer min-idle-time id [id ...] [IDLE ms] [TIME unix-time-milliseconds] [RETRYCOUNT count] [FORCE] [JUSTID] [LASTID id]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Changes (or acquires) ownership of a message in a consumer group, as if the message was delivered to the specified consumer.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XDEL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key id [id ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Removes the specified entries from the stream. Returns the number of items actually deleted, that may be different from the number of IDs passed in case certain IDs do not exist.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XGROUP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for consumer groups commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XGROUP CREATE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key groupname id|$ [MKSTREAM] [ENTRIESREAD entries_read]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Create a consumer group.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XGROUP CREATECONSUMER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key groupname consumername\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Create a consumer in a consumer group.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XGROUP DELCONSUMER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key groupname consumername\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Delete a consumer from a consumer group.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XGROUP DESTROY\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key groupname\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Destroy a consumer group.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XGROUP HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XGROUP SETID\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key groupname id|$ [ENTRIESREAD entries_read]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Set a consumer group to an arbitrary last delivered ID value.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XINFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"A container for stream introspection commands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XINFO CONSUMERS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key groupname\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"List the consumers in a consumer group\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XINFO GROUPS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"List the consumer groups of a stream\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XINFO HELP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Show helpful text about the different subcommands\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XINFO STREAM\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key [FULL [COUNT count]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get information about a stream\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return the number of entries in a stream\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XPENDING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key group [[IDLE min-idle-time] start end count [consumer]]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return information and entries from a stream consumer group pending entries list, that are messages fetched but never acknowledged.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XRANGE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key start end [COUNT count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Return a range of elements in a stream, with IDs matching the specified IDs interval\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XREAD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"[COUNT count] [BLOCK milliseconds] STREAMS key [key ...] id [id ...]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return never seen elements in multiple streams, with IDs greater than the ones reported by the caller for each stream. Can block.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XREADGROUP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"GROUP group consumer [COUNT count] [BLOCK milliseconds] [NOACK] STREAMS key [key ...] id [id ...]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return new entries from a stream using a consumer group, or access the history of the pending entries for a given consumer. Can block.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XREVRANGE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key end start [COUNT count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Return a range of elements in a stream, with IDs matching the specified IDs interval, in reverse order (from greater to smaller IDs) compared to XRANGE\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XSETID\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key last-id [ENTRIESADDED entries_added] [MAXDELETEDID max_deleted_entry_id]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"An internal command for replicating stream values\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"XTRIM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key MAXLEN|MINID [=|~] threshold [LIMIT count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Trims the stream to (approximately if '~' is passed) a certain size\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 14 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZADD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [NX|XX] [GT|LT] [CH] [INCR] score member [score member ...]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Add one or more members to a sorted set, or update its score if it already exists\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZCARD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Get the number of members in a sorted set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZCOUNT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key min max\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Count the members in a sorted set with scores within the given values\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZDIFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"numkeys key [key ...] [WITHSCORES]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Subtract multiple sorted sets\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZDIFFSTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"destination numkeys key [key ...]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Subtract multiple sorted sets and store the resulting sorted set in a new key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZINCRBY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key increment member\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Increment the score of a member in a sorted set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZINTER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX] [WITHSCORES]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Intersect multiple sorted sets\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZINTERCARD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"numkeys key [key ...] [LIMIT limit]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Intersect multiple sorted sets and return the cardinality of the result\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZINTERSTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"destination numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Intersect multiple sorted sets and store the resulting sorted set in a new key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZLEXCOUNT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key min max\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Count the number of members in a sorted set between a given lexicographical range\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZMPOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"numkeys key [key ...] MIN|MAX [COUNT count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Remove and return members with scores in a sorted set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"7.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZMSCORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member [member ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the score associated with the given members in a sorted set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZPOPMAX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and return members with the highest scores in a sorted set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZPOPMIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key [count]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove and return members with the lowest scores in a sorted set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZRANDMEMBER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key [count [WITHSCORES]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get one or multiple random elements from a sorted set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZRANGE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key start stop [BYSCORE|BYLEX] [REV] [LIMIT offset count] [WITHSCORES]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a range of members in a sorted set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZRANGEBYLEX\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key min max [LIMIT offset count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a range of members in a sorted set, by lexicographical range\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZRANGEBYSCORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key min max [WITHSCORES] [LIMIT offset count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a range of members in a sorted set, by score\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.0.5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZRANGESTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"dst src min max [BYSCORE|BYLEX] [REV] [LIMIT offset count]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Store a range of members from sorted set into another key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZRANK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Determine the index of a member in a sorted set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZREM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member [member ...]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove one or more members from a sorted set\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZREMRANGEBYLEX\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key min max\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove all members in a sorted set between the given lexicographical range\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZREMRANGEBYRANK\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key start stop\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove all members in a sorted set within the given indexes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZREMRANGEBYSCORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key min max\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Remove all members in a sorted set within the given scores\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZREVRANGE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key start stop [WITHSCORES]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Return a range of members in a sorted set, by index, with scores ordered from high to low\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZREVRANGEBYLEX\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key max min [LIMIT offset count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a range of members in a sorted set, by lexicographical range, ordered from higher to lower strings.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.8.9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZREVRANGEBYSCORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"key max min [WITHSCORES] [LIMIT offset count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Return a range of members in a sorted set, by score, with scores ordered from high to low\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZREVRANK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Determine the index of a member in a sorted set, with scores ordered from high to low\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZSCAN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key cursor [MATCH pattern] [COUNT count]\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            summary: b"Incrementally iterate sorted sets elements and associated scores\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.8.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZSCORE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"key member\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            summary: b"Get the score associated with the given member in a sorted set\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"1.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZUNION\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            params: b"numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX] [WITHSCORES]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Add multiple sorted sets\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"6.2.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = commandHelp {
            name: b"ZUNIONSTORE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            params: b"destination numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX]\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            summary: b"Add multiple sorted sets and store the resulting sorted set in a new key\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            group: 4 as libc::c_int,
            since: b"2.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub static mut spectrum_palette_color_size: libc::c_int = 19 as libc::c_int;
#[no_mangle]
pub static mut spectrum_palette_color: [libc::c_int; 19] = [
    0 as libc::c_int,
    233 as libc::c_int,
    234 as libc::c_int,
    235 as libc::c_int,
    237 as libc::c_int,
    239 as libc::c_int,
    241 as libc::c_int,
    243 as libc::c_int,
    245 as libc::c_int,
    247 as libc::c_int,
    144 as libc::c_int,
    143 as libc::c_int,
    142 as libc::c_int,
    184 as libc::c_int,
    226 as libc::c_int,
    214 as libc::c_int,
    208 as libc::c_int,
    202 as libc::c_int,
    196 as libc::c_int,
];
#[no_mangle]
pub static mut spectrum_palette_mono_size: libc::c_int = 13 as libc::c_int;
#[no_mangle]
pub static mut spectrum_palette_mono: [libc::c_int; 13] = [
    0 as libc::c_int,
    233 as libc::c_int,
    234 as libc::c_int,
    235 as libc::c_int,
    237 as libc::c_int,
    239 as libc::c_int,
    241 as libc::c_int,
    243 as libc::c_int,
    245 as libc::c_int,
    247 as libc::c_int,
    249 as libc::c_int,
    251 as libc::c_int,
    253 as libc::c_int,
];
#[no_mangle]
pub static mut spectrum_palette: *mut libc::c_int = 0 as *const libc::c_int
    as *mut libc::c_int;
#[no_mangle]
pub static mut spectrum_palette_size: libc::c_int = 0;
static mut context: *mut redisContext = 0 as *const redisContext as *mut redisContext;
pub static mut config: config = config {
    conn_info: cliConnInfo {
        hostip: 0 as *const libc::c_char as *mut libc::c_char,
        hostport: 0,
        input_dbnum: 0,
        auth: 0 as *const libc::c_char as *mut libc::c_char,
        user: 0 as *const libc::c_char as *mut libc::c_char,
    },
    hostsocket: 0 as *const libc::c_char as *mut libc::c_char,
    tls: 0,
    sslconfig: cliSSLconfig {
        sni: 0 as *mut libc::c_char,
        cacert: 0 as *mut libc::c_char,
        cacertdir: 0 as *mut libc::c_char,
        skip_cert_verify: 0,
        cert: 0 as *mut libc::c_char,
        key: 0 as *mut libc::c_char,
        ciphers: 0 as *mut libc::c_char,
        ciphersuites: 0 as *mut libc::c_char,
    },
    repeat: 0,
    interval: 0,
    dbnum: 0,
    interactive: 0,
    shutdown: 0,
    monitor_mode: 0,
    pubsub_mode: 0,
    blocking_state_aborted: 0,
    latency_mode: 0,
    latency_dist_mode: 0,
    latency_history: 0,
    lru_test_mode: 0,
    lru_test_sample_size: 0,
    cluster_mode: 0,
    cluster_reissue_command: 0,
    cluster_send_asking: 0,
    slave_mode: 0,
    pipe_mode: 0,
    pipe_timeout: 0,
    getrdb_mode: 0,
    get_functions_rdb_mode: 0,
    stat_mode: 0,
    scan_mode: 0,
    intrinsic_latency_mode: 0,
    intrinsic_latency_duration: 0,
    pattern: 0 as *const libc::c_char as *mut libc::c_char,
    rdb_filename: 0 as *const libc::c_char as *mut libc::c_char,
    bigkeys: 0,
    memkeys: 0,
    memkeys_samples: 0,
    hotkeys: 0,
    stdin_lastarg: 0,
    stdin_tag_arg: 0,
    stdin_tag_name: 0 as *const libc::c_char as *mut libc::c_char,
    askpass: 0,
    quoted_input: 0,
    output: 0,
    push_output: 0,
    mb_delim: 0 as *const libc::c_char as *mut libc::c_char,
    cmd_delim: 0 as *const libc::c_char as *mut libc::c_char,
    prompt: [0; 128],
    eval: 0 as *const libc::c_char as *mut libc::c_char,
    eval_ldb: 0,
    eval_ldb_sync: 0,
    eval_ldb_end: 0,
    enable_ldb_on_eval: 0,
    last_cmd_type: 0,
    verbose: 0,
    set_errcode: 0,
    cluster_manager_command: clusterManagerCommand {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        argc: 0,
        argv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        stdin_arg: 0 as *const libc::c_char as *mut libc::c_char,
        flags: 0,
        replicas: 0,
        from: 0 as *const libc::c_char as *mut libc::c_char,
        to: 0 as *const libc::c_char as *mut libc::c_char,
        weight: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        weight_argc: 0,
        master_id: 0 as *const libc::c_char as *mut libc::c_char,
        slots: 0,
        timeout: 0,
        pipeline: 0,
        threshold: 0.,
        backup_dir: 0 as *const libc::c_char as *mut libc::c_char,
        from_user: 0 as *const libc::c_char as *mut libc::c_char,
        from_pass: 0 as *const libc::c_char as *mut libc::c_char,
        from_askpass: 0,
    },
    no_auth_warning: 0,
    resp2: 0,
    resp3: 0,
    in_multi: 0,
    pre_multi_dbnum: 0,
};
static mut pref: pref = pref { hints: 0 };
static mut force_cancel_loop: sig_atomic_t = 0 as libc::c_int;
unsafe extern "C" fn ustime() -> libc::c_longlong {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ust: libc::c_longlong = 0;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    ust = tv.tv_sec as libc::c_longlong * 1000000 as libc::c_int as libc::c_longlong;
    ust += tv.tv_usec as libc::c_longlong;
    return ust;
}
unsafe extern "C" fn mstime() -> libc::c_longlong {
    return ustime() / 1000 as libc::c_int as libc::c_longlong;
}
unsafe extern "C" fn cliRefreshPrompt() {
    if config.eval_ldb != 0 {
        return;
    }
    let mut prompt: hisds = hi_sdsempty();
    if !(config.hostsocket).is_null() {
        prompt = hi_sdscatfmt(
            prompt,
            b"redis %s\0" as *const u8 as *const libc::c_char,
            config.hostsocket,
        );
    } else {
        let mut addr: [libc::c_char; 256] = [0; 256];
        anetFormatAddr(
            addr.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            config.conn_info.hostip,
            config.conn_info.hostport,
        );
        prompt = hi_sdscatlen(
            prompt,
            addr.as_mut_ptr() as *const libc::c_void,
            strlen(addr.as_mut_ptr()),
        );
    }
    if config.dbnum != 0 as libc::c_int {
        prompt = hi_sdscatfmt(
            prompt,
            b"[%i]\0" as *const u8 as *const libc::c_char,
            config.dbnum,
        );
    }
    if config.in_multi != 0 {
        prompt = hi_sdscatlen(
            prompt,
            b"(TX)\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
    }
    prompt = hi_sdscatlen(
        prompt,
        b"> \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    snprintf(
        (config.prompt).as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        prompt,
    );
    hi_sdsfree(prompt);
}
unsafe extern "C" fn getDotfilePath(
    mut envoverride: *mut libc::c_char,
    mut dotfilename: *mut libc::c_char,
) -> hisds {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dotPath: hisds = 0 as hisds;
    path = getenv(envoverride);
    if !path.is_null() && *path as libc::c_int != '\0' as i32 {
        if strcmp(b"/dev/null\0" as *const u8 as *const libc::c_char, path) == 0 {
            return 0 as hisds;
        }
        dotPath = hi_sdsnew(path);
    } else {
        let mut home: *mut libc::c_char = getenv(
            b"HOME\0" as *const u8 as *const libc::c_char,
        );
        if !home.is_null() && *home as libc::c_int != '\0' as i32 {
            dotPath = hi_sdscatprintf(
                hi_sdsempty(),
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                home,
                dotfilename,
            );
        }
    }
    return dotPath;
}
unsafe extern "C" fn dictSdsHash(mut key: *const libc::c_void) -> uint64_t {
    return dictGenHashFunction(
        key as *mut libc::c_uchar as *const libc::c_void,
        hi_sdslen(key as *mut libc::c_char),
    );
}
unsafe extern "C" fn dictSdsKeyCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = hi_sdslen(key1 as hisds) as libc::c_int;
    l2 = hi_sdslen(key2 as hisds) as libc::c_int;
    if l1 != l2 {
        return 0 as libc::c_int;
    }
    return (memcmp(key1, key2, l1 as libc::c_ulong) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn dictSdsDestructor(mut d: *mut dict, mut val: *mut libc::c_void) {
    hi_sdsfree(val as hisds);
}
unsafe extern "C" fn dictListDestructor(mut d: *mut dict, mut val: *mut libc::c_void) {
    listRelease(val as *mut list);
}
static mut helpEntries: *mut helpEntry = 0 as *const helpEntry as *mut helpEntry;
static mut helpEntriesLen: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn cliVersion() -> hisds {
    let mut version: hisds = 0 as *mut libc::c_char;
    version = hi_sdscatprintf(
        hi_sdsempty(),
        b"%s\0" as *const u8 as *const libc::c_char,
        b"7.0.8\0" as *const u8 as *const libc::c_char,
    );
    if strtoll(redisGitSHA1(), 0 as *mut *mut libc::c_char, 16 as libc::c_int) != 0 {
        version = hi_sdscatprintf(
            version,
            b" (git:%s\0" as *const u8 as *const libc::c_char,
            redisGitSHA1(),
        );
        if strtoll(redisGitDirty(), 0 as *mut *mut libc::c_char, 10 as libc::c_int) != 0
        {
            version = hi_sdscatprintf(
                version,
                b"-dirty\0" as *const u8 as *const libc::c_char,
            );
        }
        version = hi_sdscat(version, b")\0" as *const u8 as *const libc::c_char);
    }
    return version;
}
unsafe extern "C" fn cliOldInitHelp() {
    let mut commandslen: libc::c_int = (core::mem::size_of::<[commandHelp; 366]>()
        as libc::c_ulong)
        .wrapping_div(core::mem::size_of::<commandHelp>() as libc::c_ulong)
        as libc::c_int;
    let mut groupslen: libc::c_int = (core::mem::size_of::<[*mut libc::c_char; 16]>()
        as libc::c_ulong)
        .wrapping_div(core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut tmp: helpEntry = helpEntry {
        type_0: 0,
        argc: 0,
        argv: 0 as *mut hisds,
        full: 0 as *mut libc::c_char,
        org: commandDocs {
            name: 0 as *mut libc::c_char,
            params: 0 as *mut libc::c_char,
            summary: 0 as *mut libc::c_char,
            group: 0 as *mut libc::c_char,
            since: 0 as *mut libc::c_char,
        },
    };
    len = commandslen + groupslen;
    helpEntriesLen = len;
    helpEntries = zmalloc(
        (core::mem::size_of::<helpEntry>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong),
    ) as *mut helpEntry;
    i = 0 as libc::c_int;
    while i < groupslen {
        tmp.argc = 1 as libc::c_int;
        tmp
            .argv = zmalloc(core::mem::size_of::<hisds>() as libc::c_ulong)
            as *mut hisds;
        let ref mut fresh0 = *(tmp.argv).offset(0 as libc::c_int as isize);
        *fresh0 = hi_sdscatprintf(
            hi_sdsempty(),
            b"@%s\0" as *const u8 as *const libc::c_char,
            commandGroups[i as usize],
        );
        tmp.full = *(tmp.argv).offset(0 as libc::c_int as isize);
        tmp.type_0 = 2 as libc::c_int;
        tmp.org.name = 0 as *mut libc::c_char;
        tmp.org.params = 0 as *mut libc::c_char;
        tmp.org.summary = 0 as *mut libc::c_char;
        tmp.org.since = 0 as *mut libc::c_char;
        tmp.org.group = 0 as *mut libc::c_char;
        let fresh1 = pos;
        pos = pos + 1;
        *helpEntries.offset(fresh1 as isize) = tmp;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < commandslen {
        tmp.argv = hi_sdssplitargs(commandHelp[i as usize].name, &mut tmp.argc);
        tmp.full = hi_sdsnew(commandHelp[i as usize].name);
        tmp.type_0 = 1 as libc::c_int;
        tmp.org.name = commandHelp[i as usize].name;
        tmp.org.params = commandHelp[i as usize].params;
        tmp.org.summary = commandHelp[i as usize].summary;
        tmp.org.since = commandHelp[i as usize].since;
        tmp.org.group = commandGroups[commandHelp[i as usize].group as usize];
        let fresh2 = pos;
        pos = pos + 1;
        *helpEntries.offset(fresh2 as isize) = tmp;
        i += 1;
    }
}
unsafe extern "C" fn cliOldIntegrateHelp() {
    if cliConnect((1 as libc::c_int) << 1 as libc::c_int) == -(1 as libc::c_int) {
        return;
    }
    let mut reply: *mut redisReply = redisCommand(
        context,
        b"COMMAND\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    if reply.is_null() || (*reply).type_0 != 2 as libc::c_int {
        return;
    }
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < (*reply).elements {
        let mut entry: *mut redisReply = *((*reply).element).offset(j as isize);
        if (*entry).type_0 != 2 as libc::c_int
            || (*entry).elements < 4 as libc::c_int as libc::c_ulong
            || (**((*entry).element).offset(0 as libc::c_int as isize)).type_0
                != 1 as libc::c_int
            || (**((*entry).element).offset(1 as libc::c_int as isize)).type_0
                != 3 as libc::c_int
            || (**((*entry).element).offset(3 as libc::c_int as isize)).type_0
                != 3 as libc::c_int
        {
            return;
        }
        let mut cmdname: *mut libc::c_char = (**((*entry).element)
            .offset(0 as libc::c_int as isize))
            .str_0;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < helpEntriesLen {
            let mut he: *mut helpEntry = helpEntries.offset(i as isize);
            if strcasecmp(
                *((*he).argv).offset(0 as libc::c_int as isize) as *const libc::c_char,
                cmdname,
            ) == 0
            {
                break;
            }
            i += 1;
        }
        if !(i != helpEntriesLen) {
            helpEntriesLen += 1;
            helpEntries = zrealloc(
                helpEntries as *mut libc::c_void,
                (core::mem::size_of::<helpEntry>() as libc::c_ulong)
                    .wrapping_mul(helpEntriesLen as libc::c_ulong),
            ) as *mut helpEntry;
            let mut new: *mut helpEntry = helpEntries
                .offset((helpEntriesLen - 1 as libc::c_int) as isize);
            (*new).argc = 1 as libc::c_int;
            (*new)
                .argv = zmalloc(core::mem::size_of::<hisds>() as libc::c_ulong)
                as *mut hisds;
            let ref mut fresh3 = *((*new).argv).offset(0 as libc::c_int as isize);
            *fresh3 = hi_sdsnew(cmdname);
            (*new).full = *((*new).argv).offset(0 as libc::c_int as isize);
            (*new).type_0 = 1 as libc::c_int;
            hi_sdstoupper(*((*new).argv).offset(0 as libc::c_int as isize));
            (*new).org.name = *((*new).argv).offset(0 as libc::c_int as isize);
            (*new).org.params = hi_sdsempty();
            let mut args: libc::c_int = llabs(
                (**((*entry).element).offset(1 as libc::c_int as isize)).integer,
            ) as libc::c_int;
            args -= 1;
            if (**((*entry).element).offset(3 as libc::c_int as isize)).integer
                == 1 as libc::c_int as libc::c_longlong
            {
                (*new)
                    .org
                    .params = hi_sdscat(
                    (*new).org.params,
                    b"key \0" as *const u8 as *const libc::c_char,
                );
                args -= 1;
            }
            loop {
                let fresh4 = args;
                args = args - 1;
                if !(fresh4 > 0 as libc::c_int) {
                    break;
                }
                (*new)
                    .org
                    .params = hi_sdscat(
                    (*new).org.params,
                    b"arg \0" as *const u8 as *const libc::c_char,
                );
            }
            if (**((*entry).element).offset(1 as libc::c_int as isize)).integer
                < 0 as libc::c_int as libc::c_longlong
            {
                (*new)
                    .org
                    .params = hi_sdscat(
                    (*new).org.params,
                    b"...options...\0" as *const u8 as *const libc::c_char,
                );
            }
            (*new)
                .org
                .summary = b"Help not available\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            (*new)
                .org
                .since = b"Not known\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            (*new).org.group = commandGroups[0 as libc::c_int as usize];
        }
        j = j.wrapping_add(1);
    }
    freeReplyObject(reply as *mut libc::c_void);
}
unsafe extern "C" fn sdscat_orempty(
    mut params: hisds,
    mut value: *mut libc::c_char,
) -> hisds {
    if *value.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return hi_sdscat(params, b"\"\"\0" as *const u8 as *const libc::c_char);
    }
    return hi_sdscat(params, value);
}
unsafe extern "C" fn cliConcatArguments(
    mut params: hisds,
    mut arguments: *mut redisReply,
    mut separator: *mut libc::c_char,
) -> hisds {
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < (*arguments).elements {
        params = cliAddArgument(params, *((*arguments).element).offset(j as isize));
        if j != ((*arguments).elements).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            params = hi_sdscat(params, separator);
        }
        j = j.wrapping_add(1);
    }
    return params;
}
unsafe extern "C" fn cliAddArgument(
    mut params: hisds,
    mut argMap: *mut redisReply,
) -> hisds {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut optional: libc::c_int = 0 as libc::c_int;
    let mut multiple: libc::c_int = 0 as libc::c_int;
    let mut multipleToken: libc::c_int = 0 as libc::c_int;
    let mut arguments: *mut redisReply = 0 as *mut redisReply;
    let mut tokenPart: hisds = hi_sdsempty();
    let mut repeatPart: hisds = hi_sdsempty();
    if (*argMap).type_0 != 9 as libc::c_int && (*argMap).type_0 != 2 as libc::c_int {
        return params;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*argMap).elements {
        if (**((*argMap).element).offset(i as isize)).type_0 == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"argMap->element[i]->type == REDIS_REPLY_STRING\0" as *const u8
                    as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                567 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"hisds cliAddArgument(hisds, redisReply *)\0"))
                    .as_ptr(),
            );
        };
        let mut key: *mut libc::c_char = (**((*argMap).element).offset(i as isize))
            .str_0;
        if strcmp(key, b"name\0" as *const u8 as *const libc::c_char) == 0 {
            if (**((*argMap).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .type_0 == 1 as libc::c_int
            {} else {
                __assert_fail(
                    b"argMap->element[i + 1]->type == REDIS_REPLY_STRING\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    570 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"hisds cliAddArgument(hisds, redisReply *)\0"))
                        .as_ptr(),
                );
            };
            name = (**((*argMap).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .str_0;
        } else if strcmp(key, b"token\0" as *const u8 as *const libc::c_char) == 0 {
            if (**((*argMap).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .type_0 == 1 as libc::c_int
            {} else {
                __assert_fail(
                    b"argMap->element[i + 1]->type == REDIS_REPLY_STRING\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    573 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"hisds cliAddArgument(hisds, redisReply *)\0"))
                        .as_ptr(),
                );
            };
            let mut token: *mut libc::c_char = (**((*argMap).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .str_0;
            tokenPart = sdscat_orempty(tokenPart, token);
        } else if strcmp(key, b"type\0" as *const u8 as *const libc::c_char) == 0 {
            if (**((*argMap).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .type_0 == 1 as libc::c_int
            {} else {
                __assert_fail(
                    b"argMap->element[i + 1]->type == REDIS_REPLY_STRING\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    577 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"hisds cliAddArgument(hisds, redisReply *)\0"))
                        .as_ptr(),
                );
            };
            type_0 = (**((*argMap).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .str_0;
        } else if strcmp(key, b"arguments\0" as *const u8 as *const libc::c_char) == 0 {
            arguments = *((*argMap).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        } else if strcmp(key, b"flags\0" as *const u8 as *const libc::c_char) == 0 {
            let mut flags: *mut redisReply = *((*argMap).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            if (*flags).type_0 == 10 as libc::c_int
                || (*flags).type_0 == 2 as libc::c_int
            {} else {
                __assert_fail(
                    b"flags->type == REDIS_REPLY_SET || flags->type == REDIS_REPLY_ARRAY\0"
                        as *const u8 as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    583 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"hisds cliAddArgument(hisds, redisReply *)\0"))
                        .as_ptr(),
                );
            };
            let mut j: size_t = 0 as libc::c_int as size_t;
            while j < (*flags).elements {
                if (**((*flags).element).offset(j as isize)).type_0 == 5 as libc::c_int
                {} else {
                    __assert_fail(
                        b"flags->element[j]->type == REDIS_REPLY_STATUS\0" as *const u8
                            as *const libc::c_char,
                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                        585 as libc::c_int as libc::c_uint,
                        (*core::mem::transmute::<
                            &[u8; 42],
                            &[libc::c_char; 42],
                        >(b"hisds cliAddArgument(hisds, redisReply *)\0"))
                            .as_ptr(),
                    );
                };
                let mut flag: *mut libc::c_char = (**((*flags).element)
                    .offset(j as isize))
                    .str_0;
                if strcmp(flag, b"optional\0" as *const u8 as *const libc::c_char) == 0 {
                    optional = 1 as libc::c_int;
                } else if strcmp(flag, b"multiple\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                    multiple = 1 as libc::c_int;
                } else if strcmp(
                    flag,
                    b"multiple_token\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    multipleToken = 1 as libc::c_int;
                }
                j = j.wrapping_add(1);
            }
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    if strcmp(type_0, b"key\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(type_0, b"string\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(type_0, b"integer\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(type_0, b"double\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(type_0, b"pattern\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(type_0, b"unix-time\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(type_0, b"token\0" as *const u8 as *const libc::c_char) == 0
    {
        repeatPart = sdscat_orempty(repeatPart, name);
    } else if strcmp(type_0, b"oneof\0" as *const u8 as *const libc::c_char) == 0 {
        repeatPart = cliConcatArguments(
            repeatPart,
            arguments,
            b"|\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if strcmp(type_0, b"block\0" as *const u8 as *const libc::c_char) == 0 {
        repeatPart = cliConcatArguments(
            repeatPart,
            arguments,
            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if strcmp(type_0, b"pure-token\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Unknown type '%s' set for argument '%s'\n\0" as *const u8
                as *const libc::c_char,
            type_0,
            name,
        );
    }
    if *tokenPart.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        && strcmp(type_0, b"pure-token\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
    {
        tokenPart = hi_sdscat(tokenPart, b" \0" as *const u8 as *const libc::c_char);
    }
    if optional != 0 {
        params = hi_sdscat(params, b"[\0" as *const u8 as *const libc::c_char);
    }
    params = hi_sdscat(params, tokenPart as *const libc::c_char);
    params = hi_sdscat(params, repeatPart as *const libc::c_char);
    if multiple != 0 {
        params = hi_sdscat(params, b" [\0" as *const u8 as *const libc::c_char);
        if multipleToken != 0 {
            params = hi_sdscat(params, tokenPart as *const libc::c_char);
        }
        params = hi_sdscat(params, repeatPart as *const libc::c_char);
        params = hi_sdscat(params, b" ...]\0" as *const u8 as *const libc::c_char);
    }
    if optional != 0 {
        params = hi_sdscat(params, b"]\0" as *const u8 as *const libc::c_char);
    }
    hi_sdsfree(tokenPart);
    hi_sdsfree(repeatPart);
    return params;
}
unsafe extern "C" fn cliFillInCommandHelpEntry(
    mut help: *mut helpEntry,
    mut cmdname: *mut libc::c_char,
    mut subcommandname: *mut libc::c_char,
) {
    (*help)
        .argc = if !subcommandname.is_null() {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    (*help)
        .argv = zmalloc(
        (core::mem::size_of::<hisds>() as libc::c_ulong)
            .wrapping_mul((*help).argc as libc::c_ulong),
    ) as *mut hisds;
    let ref mut fresh5 = *((*help).argv).offset(0 as libc::c_int as isize);
    *fresh5 = hi_sdsnew(cmdname);
    hi_sdstoupper(*((*help).argv).offset(0 as libc::c_int as isize));
    if !subcommandname.is_null() {
        let ref mut fresh6 = *((*help).argv).offset(1 as libc::c_int as isize);
        *fresh6 = hi_sdsnew(
            (strchr(subcommandname, '|' as i32)).offset(1 as libc::c_int as isize),
        );
        hi_sdstoupper(*((*help).argv).offset(1 as libc::c_int as isize));
    }
    let mut fullname: hisds = hi_sdsnew(
        *((*help).argv).offset(0 as libc::c_int as isize) as *const libc::c_char,
    );
    if !subcommandname.is_null() {
        fullname = hi_sdscat(fullname, b" \0" as *const u8 as *const libc::c_char);
        fullname = hi_sdscat(
            fullname,
            *((*help).argv).offset(1 as libc::c_int as isize) as *const libc::c_char,
        );
    }
    (*help).full = fullname;
    (*help).type_0 = 1 as libc::c_int;
    (*help).org.name = (*help).full;
    (*help).org.params = hi_sdsempty();
    (*help).org.since = 0 as *mut libc::c_char;
}
unsafe extern "C" fn cliInitCommandHelpEntry(
    mut cmdname: *mut libc::c_char,
    mut subcommandname: *mut libc::c_char,
    mut next: *mut helpEntry,
    mut specs: *mut redisReply,
    mut groups: *mut dict,
) -> *mut helpEntry {
    let fresh7 = next;
    next = next.offset(1);
    let mut help: *mut helpEntry = fresh7;
    cliFillInCommandHelpEntry(help, cmdname, subcommandname);
    if (*specs).type_0 == 9 as libc::c_int || (*specs).type_0 == 2 as libc::c_int
    {} else {
        __assert_fail(
            b"specs->type == REDIS_REPLY_MAP || specs->type == REDIS_REPLY_ARRAY\0"
                as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            677 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
            ))
                .as_ptr(),
        );
    };
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < (*specs).elements {
        if (**((*specs).element).offset(j as isize)).type_0 == 1 as libc::c_int {} else {
            __assert_fail(
                b"specs->element[j]->type == REDIS_REPLY_STRING\0" as *const u8
                    as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                679 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
                ))
                    .as_ptr(),
            );
        };
        let mut key: *mut libc::c_char = (**((*specs).element).offset(j as isize)).str_0;
        if strcmp(key, b"summary\0" as *const u8 as *const libc::c_char) == 0 {
            let mut reply: *mut redisReply = *((*specs).element)
                .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            if (*reply).type_0 == 1 as libc::c_int {} else {
                __assert_fail(
                    b"reply->type == REDIS_REPLY_STRING\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    683 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
                    ))
                        .as_ptr(),
                );
            };
            (*help).org.summary = hi_sdsnew((*reply).str_0);
        } else if strcmp(key, b"since\0" as *const u8 as *const libc::c_char) == 0 {
            let mut reply_0: *mut redisReply = *((*specs).element)
                .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            if (*reply_0).type_0 == 1 as libc::c_int {} else {
                __assert_fail(
                    b"reply->type == REDIS_REPLY_STRING\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    687 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
                    ))
                        .as_ptr(),
                );
            };
            (*help).org.since = hi_sdsnew((*reply_0).str_0);
        } else if strcmp(key, b"group\0" as *const u8 as *const libc::c_char) == 0 {
            let mut reply_1: *mut redisReply = *((*specs).element)
                .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            if (*reply_1).type_0 == 1 as libc::c_int {} else {
                __assert_fail(
                    b"reply->type == REDIS_REPLY_STRING\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    691 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
                    ))
                        .as_ptr(),
                );
            };
            (*help).org.group = hi_sdsnew((*reply_1).str_0);
            let mut group: hisds = hi_sdsdup((*help).org.group);
            if dictAdd(groups, group as *mut libc::c_void, 0 as *mut libc::c_void)
                != 0 as libc::c_int
            {
                hi_sdsfree(group);
            }
        } else if strcmp(key, b"arguments\0" as *const u8 as *const libc::c_char) == 0 {
            let mut args: *mut redisReply = *((*specs).element)
                .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            if (*args).type_0 == 2 as libc::c_int {} else {
                __assert_fail(
                    b"args->type == REDIS_REPLY_ARRAY\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    699 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
                    ))
                        .as_ptr(),
                );
            };
            (*help)
                .org
                .params = cliConcatArguments(
                (*help).org.params,
                args,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if strcmp(key, b"subcommands\0" as *const u8 as *const libc::c_char) == 0
        {
            let mut subcommands: *mut redisReply = *((*specs).element)
                .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            if (*subcommands).type_0 == 9 as libc::c_int
                || (*subcommands).type_0 == 2 as libc::c_int
            {} else {
                __assert_fail(
                    b"subcommands->type == REDIS_REPLY_MAP || subcommands->type == REDIS_REPLY_ARRAY\0"
                        as *const u8 as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    703 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
                    ))
                        .as_ptr(),
                );
            };
            let mut i: size_t = 0 as libc::c_int as size_t;
            while i < (*subcommands).elements {
                if (**((*subcommands).element).offset(i as isize)).type_0
                    == 1 as libc::c_int
                {} else {
                    __assert_fail(
                        b"subcommands->element[i]->type == REDIS_REPLY_STRING\0"
                            as *const u8 as *const libc::c_char,
                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                        705 as libc::c_int as libc::c_uint,
                        (*core::mem::transmute::<
                            &[u8; 86],
                            &[libc::c_char; 86],
                        >(
                            b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                let mut subcommandname_0: *mut libc::c_char = (**((*subcommands).element)
                    .offset(i as isize))
                    .str_0;
                let mut subcommand: *mut redisReply = *((*subcommands).element)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                if (*subcommand).type_0 == 9 as libc::c_int
                    || (*subcommand).type_0 == 2 as libc::c_int
                {} else {
                    __assert_fail(
                        b"subcommand->type == REDIS_REPLY_MAP || subcommand->type == REDIS_REPLY_ARRAY\0"
                            as *const u8 as *const libc::c_char,
                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                        708 as libc::c_int as libc::c_uint,
                        (*core::mem::transmute::<
                            &[u8; 86],
                            &[libc::c_char; 86],
                        >(
                            b"helpEntry *cliInitCommandHelpEntry(char *, char *, helpEntry *, redisReply *, dict *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                next = cliInitCommandHelpEntry(
                    cmdname,
                    subcommandname_0,
                    next,
                    subcommand,
                    groups,
                );
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
        }
        j = (j as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return next;
}
unsafe extern "C" fn cliCountCommands(mut commandTable: *mut redisReply) -> size_t {
    let mut numCommands: size_t = ((*commandTable).elements)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*commandTable).elements {
        if (**((*commandTable).element).offset(i as isize)).type_0 == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"commandTable->element[i]->type == REDIS_REPLY_STRING\0" as *const u8
                    as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                722 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"size_t cliCountCommands(redisReply *)\0"))
                    .as_ptr(),
            );
        };
        if (**((*commandTable).element)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
            .type_0 == 9 as libc::c_int
            || (**((*commandTable).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .type_0 == 2 as libc::c_int
        {} else {
            __assert_fail(
                b"commandTable->element[i + 1]->type == REDIS_REPLY_MAP || commandTable->element[i + 1]->type == REDIS_REPLY_ARRAY\0"
                    as *const u8 as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                724 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"size_t cliCountCommands(redisReply *)\0"))
                    .as_ptr(),
            );
        };
        let mut map: *mut redisReply = *((*commandTable).element)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < (*map).elements {
            if (**((*map).element).offset(j as isize)).type_0 == 1 as libc::c_int
            {} else {
                __assert_fail(
                    b"map->element[j]->type == REDIS_REPLY_STRING\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    727 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 38],
                        &[libc::c_char; 38],
                    >(b"size_t cliCountCommands(redisReply *)\0"))
                        .as_ptr(),
                );
            };
            let mut key: *mut libc::c_char = (**((*map).element).offset(j as isize))
                .str_0;
            if strcmp(key, b"subcommands\0" as *const u8 as *const libc::c_char) == 0 {
                let mut subcommands: *mut redisReply = *((*map).element)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                if (*subcommands).type_0 == 9 as libc::c_int
                    || (*subcommands).type_0 == 2 as libc::c_int
                {} else {
                    __assert_fail(
                        b"subcommands->type == REDIS_REPLY_MAP || subcommands->type == REDIS_REPLY_ARRAY\0"
                            as *const u8 as *const libc::c_char,
                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                        731 as libc::c_int as libc::c_uint,
                        (*core::mem::transmute::<
                            &[u8; 38],
                            &[libc::c_char; 38],
                        >(b"size_t cliCountCommands(redisReply *)\0"))
                            .as_ptr(),
                    );
                };
                numCommands = (numCommands as libc::c_ulong)
                    .wrapping_add(
                        ((*subcommands).elements)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            }
            j = (j as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return numCommands;
}
#[no_mangle]
pub unsafe extern "C" fn helpEntryCompare(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> libc::c_int {
    let mut i1: *mut helpEntry = entry1 as *mut helpEntry;
    let mut i2: *mut helpEntry = entry2 as *mut helpEntry;
    return strcmp((*i1).full as *const libc::c_char, (*i2).full as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn cliInitGroupHelpEntries(mut groups: *mut dict) {
    let mut iter: *mut dictIterator = dictGetIterator(groups);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut tmp: helpEntry = helpEntry {
        type_0: 0,
        argc: 0,
        argv: 0 as *mut hisds,
        full: 0 as *mut libc::c_char,
        org: commandDocs {
            name: 0 as *mut libc::c_char,
            params: 0 as *mut libc::c_char,
            summary: 0 as *mut libc::c_char,
            group: 0 as *mut libc::c_char,
            since: 0 as *mut libc::c_char,
        },
    };
    let mut numGroups: libc::c_int = ((*groups).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*groups).ht_used[1 as libc::c_int as usize]) as libc::c_int;
    let mut pos: libc::c_int = helpEntriesLen;
    helpEntriesLen += numGroups;
    helpEntries = zrealloc(
        helpEntries as *mut libc::c_void,
        (core::mem::size_of::<helpEntry>() as libc::c_ulong)
            .wrapping_mul(helpEntriesLen as libc::c_ulong),
    ) as *mut helpEntry;
    entry = dictNext(iter);
    while !entry.is_null() {
        tmp.argc = 1 as libc::c_int;
        tmp
            .argv = zmalloc(core::mem::size_of::<hisds>() as libc::c_ulong)
            as *mut hisds;
        let ref mut fresh8 = *(tmp.argv).offset(0 as libc::c_int as isize);
        *fresh8 = hi_sdscatprintf(
            hi_sdsempty(),
            b"@%s\0" as *const u8 as *const libc::c_char,
            (*entry).key as *mut libc::c_char,
        );
        tmp.full = *(tmp.argv).offset(0 as libc::c_int as isize);
        tmp.type_0 = 2 as libc::c_int;
        tmp.org.name = 0 as *mut libc::c_char;
        tmp.org.params = 0 as *mut libc::c_char;
        tmp.org.summary = 0 as *mut libc::c_char;
        tmp.org.since = 0 as *mut libc::c_char;
        tmp.org.group = 0 as *mut libc::c_char;
        let fresh9 = pos;
        pos = pos + 1;
        *helpEntries.offset(fresh9 as isize) = tmp;
        entry = dictNext(iter);
    }
    dictReleaseIterator(iter);
}
#[no_mangle]
pub unsafe extern "C" fn cliInitCommandHelpEntries(
    mut commandTable: *mut redisReply,
    mut groups: *mut dict,
) {
    let mut next: *mut helpEntry = helpEntries;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*commandTable).elements {
        if (**((*commandTable).element).offset(i as isize)).type_0 == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"commandTable->element[i]->type == REDIS_REPLY_STRING\0" as *const u8
                    as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                780 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void cliInitCommandHelpEntries(redisReply *, dict *)\0"))
                    .as_ptr(),
            );
        };
        let mut cmdname: *mut libc::c_char = (**((*commandTable).element)
            .offset(i as isize))
            .str_0;
        if (**((*commandTable).element)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
            .type_0 == 9 as libc::c_int
            || (**((*commandTable).element)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                .type_0 == 2 as libc::c_int
        {} else {
            __assert_fail(
                b"commandTable->element[i + 1]->type == REDIS_REPLY_MAP || commandTable->element[i + 1]->type == REDIS_REPLY_ARRAY\0"
                    as *const u8 as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                784 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void cliInitCommandHelpEntries(redisReply *, dict *)\0"))
                    .as_ptr(),
            );
        };
        let mut cmdspecs: *mut redisReply = *((*commandTable).element)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        next = cliInitCommandHelpEntry(
            cmdname,
            0 as *mut libc::c_char,
            next,
            cmdspecs,
            groups,
        );
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
}
unsafe extern "C" fn cliInitHelp() {
    let mut groupsdt: dictType = {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                dictSdsDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    };
    let mut commandTable: *mut redisReply = 0 as *mut redisReply;
    let mut groups: *mut dict = 0 as *mut dict;
    if cliConnect((1 as libc::c_int) << 1 as libc::c_int) == -(1 as libc::c_int) {
        cliOldInitHelp();
        return;
    }
    commandTable = redisCommand(
        context,
        b"COMMAND DOCS\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    if commandTable.is_null() || (*commandTable).type_0 == 6 as libc::c_int {
        freeReplyObject(commandTable as *mut libc::c_void);
        cliOldInitHelp();
        cliOldIntegrateHelp();
        return;
    }
    if (*commandTable).type_0 != 9 as libc::c_int
        && (*commandTable).type_0 != 2 as libc::c_int
    {
        return;
    }
    helpEntriesLen = cliCountCommands(commandTable) as libc::c_int;
    helpEntries = zmalloc(
        (core::mem::size_of::<helpEntry>() as libc::c_ulong)
            .wrapping_mul(helpEntriesLen as libc::c_ulong),
    ) as *mut helpEntry;
    groups = dictCreate(&mut groupsdt);
    cliInitCommandHelpEntries(commandTable, groups);
    cliInitGroupHelpEntries(groups);
    qsort(
        helpEntries as *mut libc::c_void,
        helpEntriesLen as size_t,
        core::mem::size_of::<helpEntry>() as libc::c_ulong,
        Some(
            helpEntryCompare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    freeReplyObject(commandTable as *mut libc::c_void);
    dictRelease(groups);
}
unsafe extern "C" fn cliOutputCommandHelp(
    mut help: *mut commandDocs,
    mut group: libc::c_int,
) {
    printf(
        b"\r\n  \x1B[1m%s\x1B[0m \x1B[90m%s\x1B[0m\r\n\0" as *const u8
            as *const libc::c_char,
        (*help).name,
        (*help).params,
    );
    printf(
        b"  \x1B[33msummary:\x1B[0m %s\r\n\0" as *const u8 as *const libc::c_char,
        (*help).summary,
    );
    if !((*help).since).is_null() {
        printf(
            b"  \x1B[33msince:\x1B[0m %s\r\n\0" as *const u8 as *const libc::c_char,
            (*help).since,
        );
    }
    if group != 0 {
        printf(
            b"  \x1B[33mgroup:\x1B[0m %s\r\n\0" as *const u8 as *const libc::c_char,
            (*help).group,
        );
    }
}
unsafe extern "C" fn cliOutputGenericHelp() {
    let mut version: hisds = cliVersion();
    printf(
        b"redis-cli %s\nTo get help about Redis commands type:\n      \"help @<group>\" to get a list of commands in <group>\n      \"help <command>\" for help on <command>\n      \"help <tab>\" to get a list of possible help topics\n      \"quit\" to exit\n\nTo set redis-cli preferences:\n      \":set hints\" enable online hints\n      \":set nohints\" disable online hints\nSet your preferences in ~/.redisclirc\n\0"
            as *const u8 as *const libc::c_char,
        version,
    );
    hi_sdsfree(version);
}
unsafe extern "C" fn cliOutputHelp(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut group: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut entry: *mut helpEntry = 0 as *mut helpEntry;
    let mut help: *mut commandDocs = 0 as *mut commandDocs;
    if argc == 0 as libc::c_int {
        cliOutputGenericHelp();
        return;
    } else {
        if argc > 0 as libc::c_int
            && *(*argv.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
        {
            group = (*argv.offset(0 as libc::c_int as isize))
                .offset(1 as libc::c_int as isize);
        }
    }
    if helpEntries.is_null() {
        cliInitHelp();
    }
    if argc > 0 as libc::c_int {} else {
        __assert_fail(
            b"argc > 0\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            888 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void cliOutputHelp(int, char **)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int;
    while i < helpEntriesLen {
        entry = &mut *helpEntries.offset(i as isize) as *mut helpEntry;
        if !((*entry).type_0 != 1 as libc::c_int) {
            help = &mut (*entry).org;
            if group.is_null() {
                if argc <= (*entry).argc {
                    j = 0 as libc::c_int;
                    while j < argc {
                        if strcasecmp(
                            *argv.offset(j as isize),
                            *((*entry).argv).offset(j as isize) as *const libc::c_char,
                        ) != 0 as libc::c_int
                        {
                            break;
                        }
                        j += 1;
                    }
                    if j == argc {
                        cliOutputCommandHelp(help, 1 as libc::c_int);
                    }
                }
            } else if strcasecmp(group, (*help).group) == 0 as libc::c_int {
                cliOutputCommandHelp(help, 0 as libc::c_int);
            }
        }
        i += 1;
    }
    printf(b"\r\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn completionCallback(
    mut buf: *const libc::c_char,
    mut lc: *mut linenoiseCompletions,
) {
    let mut startpos: size_t = 0 as libc::c_int as size_t;
    let mut mask: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut matchlen: size_t = 0;
    let mut tmp: hisds = 0 as *mut libc::c_char;
    if strncasecmp(
        buf,
        b"help \0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
    ) == 0 as libc::c_int
    {
        startpos = 5 as libc::c_int as size_t;
        while *(*__ctype_b_loc())
            .offset(*buf.offset(startpos as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            startpos = startpos.wrapping_add(1);
        }
        mask = 1 as libc::c_int | 2 as libc::c_int;
    } else {
        mask = 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < helpEntriesLen {
        if !((*helpEntries.offset(i as isize)).type_0 & mask == 0) {
            matchlen = strlen(buf.offset(startpos as isize));
            if strncasecmp(
                buf.offset(startpos as isize),
                (*helpEntries.offset(i as isize)).full as *const libc::c_char,
                matchlen,
            ) == 0 as libc::c_int
            {
                tmp = hi_sdsnewlen(buf as *const libc::c_void, startpos);
                tmp = hi_sdscat(
                    tmp,
                    (*helpEntries.offset(i as isize)).full as *const libc::c_char,
                );
                linenoiseAddCompletion(lc, tmp as *const libc::c_char);
                hi_sdsfree(tmp);
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn hintsCallback(
    mut buf: *const libc::c_char,
    mut color: *mut libc::c_int,
    mut bold: *mut libc::c_int,
) -> *mut libc::c_char {
    if pref.hints == 0 {
        return 0 as *mut libc::c_char;
    }
    let mut i: libc::c_int = 0;
    let mut rawargc: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut buflen: libc::c_int = strlen(buf) as libc::c_int;
    let mut matchlen: libc::c_int = 0 as libc::c_int;
    let mut rawargv: *mut hisds = 0 as *mut hisds;
    let mut argv: *mut hisds = hi_sdssplitargs(buf, &mut argc);
    let mut endspace: libc::c_int = (buflen != 0
        && *(*__ctype_b_loc())
            .offset(
                *buf.offset((buflen - 1 as libc::c_int) as isize) as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0) as libc::c_int;
    let mut entry: *mut helpEntry = 0 as *mut helpEntry;
    if argc == 0 as libc::c_int {
        hi_sdsfreesplitres(argv, argc);
        return 0 as *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    while i < helpEntriesLen {
        if !((*helpEntries.offset(i as isize)).type_0 & 1 as libc::c_int == 0) {
            rawargv = hi_sdssplitargs(
                (*helpEntries.offset(i as isize)).full as *const libc::c_char,
                &mut rawargc,
            );
            if rawargc <= argc {
                let mut j: libc::c_int = 0;
                j = 0 as libc::c_int;
                while j < rawargc {
                    if strcasecmp(
                        *rawargv.offset(j as isize) as *const libc::c_char,
                        *argv.offset(j as isize) as *const libc::c_char,
                    ) != 0
                    {
                        break;
                    }
                    j += 1;
                }
                if j == rawargc && rawargc > matchlen {
                    matchlen = rawargc;
                    entry = &mut *helpEntries.offset(i as isize) as *mut helpEntry;
                }
            }
            hi_sdsfreesplitres(rawargv, rawargc);
        }
        i += 1;
    }
    hi_sdsfreesplitres(argv, argc);
    if !entry.is_null() {
        *color = 90 as libc::c_int;
        *bold = 0 as libc::c_int;
        let mut hint: hisds = hi_sdsnew((*entry).org.params);
        let mut toremove: libc::c_int = argc - matchlen;
        while toremove > 0 as libc::c_int && hi_sdslen(hint) != 0 {
            if *hint.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
                break;
            }
            if *hint.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                toremove -= 1;
            }
            hi_sdsrange(
                hint,
                1 as libc::c_int as ssize_t,
                -(1 as libc::c_int) as ssize_t,
            );
        }
        if endspace == 0 {
            let mut newhint: hisds = hi_sdsnewlen(
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            newhint = hi_sdscatsds(newhint, hint);
            hi_sdsfree(hint);
            hint = newhint;
        }
        return hint;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn freeHintsCallback(mut ptr: *mut libc::c_void) {
    hi_sdsfree(ptr as hisds);
}
unsafe extern "C" fn cliAuth(
    mut ctx: *mut redisContext,
    mut user: *mut libc::c_char,
    mut auth: *mut libc::c_char,
) -> libc::c_int {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    if auth.is_null() {
        return 0 as libc::c_int;
    }
    if user.is_null() {
        reply = redisCommand(ctx, b"AUTH %s\0" as *const u8 as *const libc::c_char, auth)
            as *mut redisReply;
    } else {
        reply = redisCommand(
            ctx,
            b"AUTH %s %s\0" as *const u8 as *const libc::c_char,
            user,
            auth,
        ) as *mut redisReply;
    }
    if reply.is_null() {
        fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0 as libc::c_int;
    if (*reply).type_0 == 6 as libc::c_int {
        result = -(1 as libc::c_int);
        fprintf(
            stderr,
            b"AUTH failed: %s\n\0" as *const u8 as *const libc::c_char,
            (*reply).str_0,
        );
    }
    freeReplyObject(reply as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn cliSelect() -> libc::c_int {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    if config.conn_info.input_dbnum == config.dbnum {
        return 0 as libc::c_int;
    }
    reply = redisCommand(
        context,
        b"SELECT %d\0" as *const u8 as *const libc::c_char,
        config.conn_info.input_dbnum,
    ) as *mut redisReply;
    if reply.is_null() {
        fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0 as libc::c_int;
    if (*reply).type_0 == 6 as libc::c_int {
        result = -(1 as libc::c_int);
        fprintf(
            stderr,
            b"SELECT %d failed: %s\n\0" as *const u8 as *const libc::c_char,
            config.conn_info.input_dbnum,
            (*reply).str_0,
        );
    } else {
        config.dbnum = config.conn_info.input_dbnum;
        cliRefreshPrompt();
    }
    freeReplyObject(reply as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn cliSwitchProto() -> libc::c_int {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    if config.resp3 == 0 || config.resp2 != 0 {
        return 0 as libc::c_int;
    }
    reply = redisCommand(context, b"HELLO 3\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if reply.is_null() {
        fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0 as libc::c_int;
    if (*reply).type_0 == 6 as libc::c_int {
        fprintf(
            stderr,
            b"HELLO 3 failed: %s\n\0" as *const u8 as *const libc::c_char,
            (*reply).str_0,
        );
        if config.resp3 == 1 as libc::c_int {
            result = -(1 as libc::c_int);
        } else if config.resp3 == 2 as libc::c_int {
            result = 0 as libc::c_int;
        }
    }
    freeReplyObject(reply as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn cliConnect(mut flags: libc::c_int) -> libc::c_int {
    if context.is_null() || flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        if !context.is_null() {
            redisFree(context);
            config.dbnum = 0 as libc::c_int;
            config.in_multi = 0 as libc::c_int;
            cliRefreshPrompt();
        }
        if (config.hostsocket).is_null()
            || config.cluster_mode != 0 && config.cluster_reissue_command != 0
        {
            context = redisConnect(config.conn_info.hostip, config.conn_info.hostport);
        } else {
            context = redisConnectUnix(config.hostsocket);
        }
        if (*context).err == 0 && config.tls != 0 {
            let mut err: *const libc::c_char = 0 as *const libc::c_char;
            if cliSecureConnection(context, config.sslconfig, &mut err)
                == -(1 as libc::c_int) && !err.is_null()
            {
                fprintf(
                    stderr,
                    b"Could not negotiate a TLS connection: %s\n\0" as *const u8
                        as *const libc::c_char,
                    err,
                );
                redisFree(context);
                context = 0 as *mut redisContext;
                return -(1 as libc::c_int);
            }
        }
        if (*context).err != 0 {
            if flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                fprintf(
                    stderr,
                    b"Could not connect to Redis at \0" as *const u8
                        as *const libc::c_char,
                );
                if (config.hostsocket).is_null()
                    || config.cluster_mode != 0 && config.cluster_reissue_command != 0
                {
                    fprintf(
                        stderr,
                        b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                        config.conn_info.hostip,
                        config.conn_info.hostport,
                        ((*context).errstr).as_mut_ptr(),
                    );
                } else {
                    fprintf(
                        stderr,
                        b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                        config.hostsocket,
                        ((*context).errstr).as_mut_ptr(),
                    );
                }
            }
            redisFree(context);
            context = 0 as *mut redisContext;
            return -(1 as libc::c_int);
        }
        anetKeepAlive(0 as *mut libc::c_char, (*context).fd, 15 as libc::c_int);
        if cliAuth(context, config.conn_info.user, config.conn_info.auth)
            != 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if cliSelect() != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if cliSwitchProto() != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if config.push_output != 0 {
        redisSetPushCallback(
            context,
            Some(
                cliPushHandler
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cliSendAsking() -> libc::c_int {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    config.cluster_send_asking = 0 as libc::c_int;
    if context.is_null() {
        return -(1 as libc::c_int);
    }
    reply = redisCommand(context, b"ASKING\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if reply.is_null() {
        fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0 as libc::c_int;
    if (*reply).type_0 == 6 as libc::c_int {
        result = -(1 as libc::c_int);
        fprintf(
            stderr,
            b"ASKING failed: %s\n\0" as *const u8 as *const libc::c_char,
            (*reply).str_0,
        );
    }
    freeReplyObject(reply as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn cliPrintContextError() {
    if context.is_null() {
        return;
    }
    fprintf(
        stderr,
        b"Error: %s\n\0" as *const u8 as *const libc::c_char,
        ((*context).errstr).as_mut_ptr(),
    );
}
unsafe extern "C" fn isInvalidateReply(mut reply: *mut redisReply) -> libc::c_int {
    return ((*reply).type_0 == 12 as libc::c_int
        && (*reply).elements == 2 as libc::c_int as libc::c_ulong
        && (**((*reply).element).offset(0 as libc::c_int as isize)).type_0
            == 1 as libc::c_int
        && strncmp(
            (**((*reply).element).offset(0 as libc::c_int as isize)).str_0,
            b"invalidate\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as libc::c_ulong,
        ) == 0
        && (**((*reply).element).offset(1 as libc::c_int as isize)).type_0
            == 2 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn cliFormatInvalidateTTY(mut r: *mut redisReply) -> hisds {
    let mut out: hisds = hi_sdsnew(
        b"-> invalidate: \0" as *const u8 as *const libc::c_char,
    );
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (**((*r).element).offset(1 as libc::c_int as isize)).elements {
        let mut key: *mut redisReply = *((**((*r).element)
            .offset(1 as libc::c_int as isize))
            .element)
            .offset(i as isize);
        if (*key).type_0 == 1 as libc::c_int {} else {
            __assert_fail(
                b"key->type == REDIS_REPLY_STRING\0" as *const u8 as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                1198 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"hisds cliFormatInvalidateTTY(redisReply *)\0"))
                    .as_ptr(),
            );
        };
        out = hi_sdscatfmt(
            out,
            b"'%s'\0" as *const u8 as *const libc::c_char,
            (*key).str_0,
            (*key).len,
        );
        if i
            < ((**((*r).element).offset(1 as libc::c_int as isize)).elements)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            out = hi_sdscatlen(
                out,
                b", \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
        }
        i = i.wrapping_add(1);
    }
    return hi_sdscatlen(
        out,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn cliIsMultilineValueTTY(mut r: *mut redisReply) -> libc::c_int {
    match (*r).type_0 {
        2 | 10 | 12 => {
            if (*r).elements == 0 as libc::c_int as libc::c_ulong {
                return 0 as libc::c_int;
            }
            if (*r).elements > 1 as libc::c_int as libc::c_ulong {
                return 1 as libc::c_int;
            }
            return cliIsMultilineValueTTY(
                *((*r).element).offset(0 as libc::c_int as isize),
            );
        }
        9 => {
            if (*r).elements == 0 as libc::c_int as libc::c_ulong {
                return 0 as libc::c_int;
            }
            if (*r).elements > 2 as libc::c_int as libc::c_ulong {
                return 1 as libc::c_int;
            }
            return cliIsMultilineValueTTY(
                *((*r).element).offset(1 as libc::c_int as isize),
            );
        }
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn cliFormatReplyTTY(
    mut r: *mut redisReply,
    mut prefix: *mut libc::c_char,
) -> hisds {
    let mut out: hisds = hi_sdsempty();
    match (*r).type_0 {
        6 => {
            out = hi_sdscatprintf(
                out,
                b"(error) %s\n\0" as *const u8 as *const libc::c_char,
                (*r).str_0,
            );
        }
        5 => {
            out = hi_sdscat(out, (*r).str_0);
            out = hi_sdscat(out, b"\n\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            out = hi_sdscatprintf(
                out,
                b"(integer) %lld\n\0" as *const u8 as *const libc::c_char,
                (*r).integer,
            );
        }
        7 => {
            out = hi_sdscatprintf(
                out,
                b"(double) %s\n\0" as *const u8 as *const libc::c_char,
                (*r).str_0,
            );
        }
        1 | 14 => {
            if (*r).type_0 == 1 as libc::c_int {
                out = hi_sdscatrepr(out, (*r).str_0, (*r).len);
                out = hi_sdscat(out, b"\n\0" as *const u8 as *const libc::c_char);
            } else {
                out = hi_sdscatlen(out, (*r).str_0 as *const libc::c_void, (*r).len);
                out = hi_sdscat(out, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        4 => {
            out = hi_sdscat(out, b"(nil)\n\0" as *const u8 as *const libc::c_char);
        }
        8 => {
            out = hi_sdscat(
                out,
                if (*r).integer != 0 {
                    b"(true)\n\0" as *const u8 as *const libc::c_char
                } else {
                    b"(false)\n\0" as *const u8 as *const libc::c_char
                },
            );
        }
        2 | 9 | 10 | 12 => {
            if (*r).elements == 0 as libc::c_int as libc::c_ulong {
                if (*r).type_0 == 2 as libc::c_int {
                    out = hi_sdscat(
                        out,
                        b"(empty array)\n\0" as *const u8 as *const libc::c_char,
                    );
                } else if (*r).type_0 == 9 as libc::c_int {
                    out = hi_sdscat(
                        out,
                        b"(empty hash)\n\0" as *const u8 as *const libc::c_char,
                    );
                } else if (*r).type_0 == 10 as libc::c_int {
                    out = hi_sdscat(
                        out,
                        b"(empty set)\n\0" as *const u8 as *const libc::c_char,
                    );
                } else if (*r).type_0 == 12 as libc::c_int {
                    out = hi_sdscat(
                        out,
                        b"(empty push)\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    out = hi_sdscat(
                        out,
                        b"(empty aggregate type)\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                let mut i: libc::c_uint = 0;
                let mut idxlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut _prefixlen: [libc::c_char; 16] = [0; 16];
                let mut _prefixfmt: [libc::c_char; 16] = [0; 16];
                let mut _prefix: hisds = 0 as *mut libc::c_char;
                let mut tmp: hisds = 0 as *mut libc::c_char;
                i = (*r).elements as libc::c_uint;
                if (*r).type_0 == 9 as libc::c_int {
                    i = i.wrapping_div(2 as libc::c_int as libc::c_uint);
                }
                loop {
                    idxlen = idxlen.wrapping_add(1);
                    i = i.wrapping_div(10 as libc::c_int as libc::c_uint);
                    if !(i != 0) {
                        break;
                    }
                }
                memset(
                    _prefixlen.as_mut_ptr() as *mut libc::c_void,
                    ' ' as i32,
                    idxlen.wrapping_add(2 as libc::c_int as libc::c_uint)
                        as libc::c_ulong,
                );
                _prefixlen[idxlen.wrapping_add(2 as libc::c_int as libc::c_uint)
                    as usize] = '\0' as i32 as libc::c_char;
                _prefix = hi_sdscat(hi_sdsnew(prefix), _prefixlen.as_mut_ptr());
                let mut numsep: libc::c_char = 0;
                if (*r).type_0 == 10 as libc::c_int {
                    numsep = '~' as i32 as libc::c_char;
                } else if (*r).type_0 == 9 as libc::c_int {
                    numsep = '#' as i32 as libc::c_char;
                } else {
                    numsep = ')' as i32 as libc::c_char;
                }
                snprintf(
                    _prefixfmt.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    b"%%s%%%ud%c \0" as *const u8 as *const libc::c_char,
                    idxlen,
                    numsep as libc::c_int,
                );
                i = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong) < (*r).elements {
                    let mut human_idx: libc::c_uint = if (*r).type_0 == 9 as libc::c_int
                    {
                        i.wrapping_div(2 as libc::c_int as libc::c_uint)
                    } else {
                        i
                    };
                    human_idx = human_idx.wrapping_add(1);
                    out = hi_sdscatprintf(
                        out,
                        _prefixfmt.as_mut_ptr(),
                        if i == 0 as libc::c_int as libc::c_uint {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            prefix as *const libc::c_char
                        },
                        human_idx,
                    );
                    tmp = cliFormatReplyTTY(*((*r).element).offset(i as isize), _prefix);
                    out = hi_sdscatlen(out, tmp as *const libc::c_void, hi_sdslen(tmp));
                    hi_sdsfree(tmp);
                    if (*r).type_0 == 9 as libc::c_int {
                        i = i.wrapping_add(1);
                        hi_sdsrange(
                            out,
                            0 as libc::c_int as ssize_t,
                            -(2 as libc::c_int) as ssize_t,
                        );
                        out = hi_sdscat(
                            out,
                            b" => \0" as *const u8 as *const libc::c_char,
                        );
                        if cliIsMultilineValueTTY(*((*r).element).offset(i as isize))
                            != 0
                        {
                            out = hi_sdscat(
                                out,
                                b"\n\0" as *const u8 as *const libc::c_char,
                            );
                            out = hi_sdscat(out, _prefix as *const libc::c_char);
                        }
                        tmp = cliFormatReplyTTY(
                            *((*r).element).offset(i as isize),
                            _prefix,
                        );
                        out = hi_sdscatlen(
                            out,
                            tmp as *const libc::c_void,
                            hi_sdslen(tmp),
                        );
                        hi_sdsfree(tmp);
                    }
                    i = i.wrapping_add(1);
                }
                hi_sdsfree(_prefix);
            }
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown reply type: %d\n\0" as *const u8 as *const libc::c_char,
                (*r).type_0,
            );
            exit(1 as libc::c_int);
        }
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn isColorTerm() -> libc::c_int {
    let mut t: *mut libc::c_char = getenv(b"TERM\0" as *const u8 as *const libc::c_char);
    return (!t.is_null()
        && !(strstr(t, b"xterm\0" as *const u8 as *const libc::c_char)).is_null())
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sdscatcolor(
    mut o: hisds,
    mut s: *mut libc::c_char,
    mut len: size_t,
    mut color: *mut libc::c_char,
) -> hisds {
    if isColorTerm() == 0 {
        return hi_sdscatlen(o, s as *const libc::c_void, len);
    }
    let mut bold: libc::c_int = (strstr(
        color,
        b"bold\0" as *const u8 as *const libc::c_char,
    ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    let mut ccode: libc::c_int = 37 as libc::c_int;
    if !(strstr(color, b"red\0" as *const u8 as *const libc::c_char)).is_null() {
        ccode = 31 as libc::c_int;
    } else if !(strstr(color, b"green\0" as *const u8 as *const libc::c_char)).is_null()
    {
        ccode = 32 as libc::c_int;
    } else if !(strstr(color, b"yellow\0" as *const u8 as *const libc::c_char)).is_null()
    {
        ccode = 33 as libc::c_int;
    } else if !(strstr(color, b"blue\0" as *const u8 as *const libc::c_char)).is_null() {
        ccode = 34 as libc::c_int;
    } else if !(strstr(color, b"magenta\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        ccode = 35 as libc::c_int;
    } else if !(strstr(color, b"cyan\0" as *const u8 as *const libc::c_char)).is_null() {
        ccode = 36 as libc::c_int;
    } else if !(strstr(color, b"white\0" as *const u8 as *const libc::c_char)).is_null()
    {
        ccode = 37 as libc::c_int;
    }
    o = hi_sdscatfmt(
        o,
        b"\x1B[%i;%i;49m\0" as *const u8 as *const libc::c_char,
        bold,
        ccode,
    );
    o = hi_sdscatlen(o, s as *const libc::c_void, len);
    o = hi_sdscat(o, b"\x1B[0m\0" as *const u8 as *const libc::c_char);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn sdsCatColorizedLdbReply(
    mut o: hisds,
    mut s: *mut libc::c_char,
    mut len: size_t,
) -> hisds {
    let mut color: *mut libc::c_char = b"white\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    if !(strstr(s, b"<debug>\0" as *const u8 as *const libc::c_char)).is_null() {
        color = b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !(strstr(s, b"<redis>\0" as *const u8 as *const libc::c_char)).is_null() {
        color = b"green\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !(strstr(s, b"<reply>\0" as *const u8 as *const libc::c_char)).is_null() {
        color = b"cyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !(strstr(s, b"<error>\0" as *const u8 as *const libc::c_char)).is_null() {
        color = b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !(strstr(s, b"<hint>\0" as *const u8 as *const libc::c_char)).is_null() {
        color = b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !(strstr(s, b"<value>\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(s, b"<retval>\0" as *const u8 as *const libc::c_char)).is_null()
    {
        color = b"magenta\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if len > 4 as libc::c_int as libc::c_ulong
        && *(*__ctype_b_loc())
            .offset(*s.offset(3 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        if *s.offset(1 as libc::c_int as isize) as libc::c_int == '>' as i32 {
            color = b"yellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if *s.offset(2 as libc::c_int as isize) as libc::c_int == '#' as i32 {
            color = b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    return sdscatcolor(o, s, len, color);
}
unsafe extern "C" fn cliFormatReplyRaw(mut r: *mut redisReply) -> hisds {
    let mut out: hisds = hi_sdsempty();
    let mut tmp: hisds = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    match (*r).type_0 {
        4 => {}
        6 => {
            out = hi_sdscatlen(out, (*r).str_0 as *const libc::c_void, (*r).len);
            out = hi_sdscatlen(
                out,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        5 | 1 | 14 => {
            if (*r).type_0 == 5 as libc::c_int && config.eval_ldb != 0 {
                if strstr(
                    (*r).str_0,
                    b"<endsession>\0" as *const u8 as *const libc::c_char,
                ) == (*r).str_0
                {
                    config.enable_ldb_on_eval = 0 as libc::c_int;
                    config.eval_ldb = 0 as libc::c_int;
                    config.eval_ldb_end = 1 as libc::c_int;
                    config.output = 0 as libc::c_int;
                    cliRefreshPrompt();
                } else {
                    out = sdsCatColorizedLdbReply(out, (*r).str_0, (*r).len);
                }
            } else {
                out = hi_sdscatlen(out, (*r).str_0 as *const libc::c_void, (*r).len);
            }
        }
        8 => {
            out = hi_sdscat(
                out,
                if (*r).integer != 0 {
                    b"(true)\0" as *const u8 as *const libc::c_char
                } else {
                    b"(false)\0" as *const u8 as *const libc::c_char
                },
            );
        }
        3 => {
            out = hi_sdscatprintf(
                out,
                b"%lld\0" as *const u8 as *const libc::c_char,
                (*r).integer,
            );
        }
        7 => {
            out = hi_sdscatprintf(
                out,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*r).str_0,
            );
        }
        10 | 2 | 12 => {
            i = 0 as libc::c_int as size_t;
            while i < (*r).elements {
                if i > 0 as libc::c_int as libc::c_ulong {
                    out = hi_sdscat(out, config.mb_delim as *const libc::c_char);
                }
                tmp = cliFormatReplyRaw(*((*r).element).offset(i as isize));
                out = hi_sdscatlen(out, tmp as *const libc::c_void, hi_sdslen(tmp));
                hi_sdsfree(tmp);
                i = i.wrapping_add(1);
            }
        }
        9 => {
            i = 0 as libc::c_int as size_t;
            while i < (*r).elements {
                if i > 0 as libc::c_int as libc::c_ulong {
                    out = hi_sdscat(out, config.mb_delim as *const libc::c_char);
                }
                tmp = cliFormatReplyRaw(*((*r).element).offset(i as isize));
                out = hi_sdscatlen(out, tmp as *const libc::c_void, hi_sdslen(tmp));
                hi_sdsfree(tmp);
                out = hi_sdscatlen(
                    out,
                    b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
                tmp = cliFormatReplyRaw(
                    *((*r).element)
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                );
                out = hi_sdscatlen(out, tmp as *const libc::c_void, hi_sdslen(tmp));
                hi_sdsfree(tmp);
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown reply type: %d\n\0" as *const u8 as *const libc::c_char,
                (*r).type_0,
            );
            exit(1 as libc::c_int);
        }
    }
    return out;
}
unsafe extern "C" fn cliFormatReplyCSV(mut r: *mut redisReply) -> hisds {
    let mut i: libc::c_uint = 0;
    let mut out: hisds = hi_sdsempty();
    match (*r).type_0 {
        6 => {
            out = hi_sdscat(out, b"ERROR,\0" as *const u8 as *const libc::c_char);
            out = hi_sdscatrepr(out, (*r).str_0, strlen((*r).str_0));
        }
        5 => {
            out = hi_sdscatrepr(out, (*r).str_0, (*r).len);
        }
        3 => {
            out = hi_sdscatprintf(
                out,
                b"%lld\0" as *const u8 as *const libc::c_char,
                (*r).integer,
            );
        }
        7 => {
            out = hi_sdscatprintf(
                out,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*r).str_0,
            );
        }
        1 | 14 => {
            out = hi_sdscatrepr(out, (*r).str_0, (*r).len);
        }
        4 => {
            out = hi_sdscat(out, b"NULL\0" as *const u8 as *const libc::c_char);
        }
        8 => {
            out = hi_sdscat(
                out,
                if (*r).integer != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        2 | 10 | 12 | 9 => {
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong) < (*r).elements {
                let mut tmp: hisds = cliFormatReplyCSV(
                    *((*r).element).offset(i as isize),
                );
                out = hi_sdscatlen(out, tmp as *const libc::c_void, hi_sdslen(tmp));
                if i as libc::c_ulong
                    != ((*r).elements).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    out = hi_sdscat(out, b",\0" as *const u8 as *const libc::c_char);
                }
                hi_sdsfree(tmp);
                i = i.wrapping_add(1);
            }
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown reply type: %d\n\0" as *const u8 as *const libc::c_char,
                (*r).type_0,
            );
            exit(1 as libc::c_int);
        }
    }
    return out;
}
unsafe extern "C" fn jsonStringOutput(
    mut out: hisds,
    mut p: *const libc::c_char,
    mut len: libc::c_int,
    mut mode: libc::c_int,
) -> hisds {
    if mode == 3 as libc::c_int {
        return escapeJsonString(out, p, len as size_t)
    } else {
        if mode == 4 as libc::c_int {
            let mut tmp: hisds = hi_sdscatrepr(hi_sdsempty(), p, len as size_t);
            let mut tmplen: libc::c_int = hi_sdslen(tmp) as libc::c_int;
            let mut n: *mut libc::c_char = tmp;
            loop {
                let fresh10 = tmplen;
                tmplen = tmplen - 1;
                if !(fresh10 != 0) {
                    break;
                }
                if *n as libc::c_int == '\\' as i32 {
                    out = hi_sdscatlen(
                        out,
                        b"\\\\\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        2 as libc::c_int as size_t,
                    );
                } else {
                    out = hi_sdscatlen(
                        out,
                        n as *const libc::c_void,
                        1 as libc::c_int as size_t,
                    );
                }
                n = n.offset(1);
            }
            hi_sdsfree(tmp);
            return out;
        } else {
            if 0 as libc::c_int != 0 {} else {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    1524 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"hisds jsonStringOutput(hisds, const char *, int, int)\0"))
                        .as_ptr(),
                );
            };
        }
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn cliFormatReplyJson(
    mut out: hisds,
    mut r: *mut redisReply,
    mut mode: libc::c_int,
) -> hisds {
    let mut i: libc::c_uint = 0;
    match (*r).type_0 {
        6 => {
            out = hi_sdscat(out, b"error:\0" as *const u8 as *const libc::c_char);
            out = jsonStringOutput(
                out,
                (*r).str_0,
                strlen((*r).str_0) as libc::c_int,
                mode,
            );
        }
        5 => {
            out = jsonStringOutput(out, (*r).str_0, (*r).len as libc::c_int, mode);
        }
        3 => {
            out = hi_sdscatprintf(
                out,
                b"%lld\0" as *const u8 as *const libc::c_char,
                (*r).integer,
            );
        }
        7 => {
            out = hi_sdscatprintf(
                out,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*r).str_0,
            );
        }
        1 | 14 => {
            out = jsonStringOutput(out, (*r).str_0, (*r).len as libc::c_int, mode);
        }
        4 => {
            out = hi_sdscat(out, b"null\0" as *const u8 as *const libc::c_char);
        }
        8 => {
            out = hi_sdscat(
                out,
                if (*r).integer != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        2 | 10 | 12 => {
            out = hi_sdscat(out, b"[\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong) < (*r).elements {
                out = cliFormatReplyJson(out, *((*r).element).offset(i as isize), mode);
                if i as libc::c_ulong
                    != ((*r).elements).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    out = hi_sdscat(out, b",\0" as *const u8 as *const libc::c_char);
                }
                i = i.wrapping_add(1);
            }
            out = hi_sdscat(out, b"]\0" as *const u8 as *const libc::c_char);
        }
        9 => {
            out = hi_sdscat(out, b"{\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong) < (*r).elements {
                let mut key: *mut redisReply = *((*r).element).offset(i as isize);
                if (*key).type_0 == 6 as libc::c_int || (*key).type_0 == 5 as libc::c_int
                    || (*key).type_0 == 1 as libc::c_int
                    || (*key).type_0 == 14 as libc::c_int
                {
                    out = cliFormatReplyJson(out, key, mode);
                } else {
                    let mut keystr: hisds = cliFormatReplyJson(hi_sdsempty(), key, mode);
                    if *keystr.offset(0 as libc::c_int as isize) as libc::c_int
                        == '"' as i32
                    {
                        out = hi_sdscatsds(out, keystr);
                    } else {
                        out = hi_sdscatfmt(
                            out,
                            b"\"%S\"\0" as *const u8 as *const libc::c_char,
                            keystr,
                        );
                    }
                    hi_sdsfree(keystr);
                }
                out = hi_sdscat(out, b":\0" as *const u8 as *const libc::c_char);
                out = cliFormatReplyJson(
                    out,
                    *((*r).element)
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ),
                    mode,
                );
                if i as libc::c_ulong
                    != ((*r).elements).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                {
                    out = hi_sdscat(out, b",\0" as *const u8 as *const libc::c_char);
                }
                i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
            }
            out = hi_sdscat(out, b"}\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown reply type: %d\n\0" as *const u8 as *const libc::c_char,
                (*r).type_0,
            );
            exit(1 as libc::c_int);
        }
    }
    return out;
}
unsafe extern "C" fn cliFormatReply(
    mut reply: *mut redisReply,
    mut mode: libc::c_int,
    mut verbatim: libc::c_int,
) -> hisds {
    let mut out: hisds = 0 as *mut libc::c_char;
    if verbatim != 0 {
        out = cliFormatReplyRaw(reply);
    } else if mode == 0 as libc::c_int {
        out = cliFormatReplyTTY(
            reply,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if mode == 1 as libc::c_int {
        out = cliFormatReplyRaw(reply);
        out = hi_sdscatsds(out, config.cmd_delim);
    } else if mode == 2 as libc::c_int {
        out = cliFormatReplyCSV(reply);
        out = hi_sdscatlen(
            out,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    } else if mode == 3 as libc::c_int || mode == 4 as libc::c_int {
        out = cliFormatReplyJson(hi_sdsempty(), reply, mode);
        out = hi_sdscatlen(
            out,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    } else {
        fprintf(
            stderr,
            b"Error:  Unknown output encoding %d\n\0" as *const u8
                as *const libc::c_char,
            mode,
        );
        exit(1 as libc::c_int);
    }
    return out;
}
unsafe extern "C" fn cliPushHandler(
    mut privdata: *mut libc::c_void,
    mut reply: *mut libc::c_void,
) {
    let mut out: hisds = 0 as *mut libc::c_char;
    if config.output == 0 as libc::c_int
        && isInvalidateReply(reply as *mut redisReply) != 0
    {
        out = cliFormatInvalidateTTY(reply as *mut redisReply);
    } else {
        out = cliFormatReply(reply as *mut redisReply, config.output, 0 as libc::c_int);
    }
    fwrite(
        out as *const libc::c_void,
        hi_sdslen(out),
        1 as libc::c_int as libc::c_ulong,
        stdout,
    );
    freeReplyObject(reply);
    hi_sdsfree(out);
}
unsafe extern "C" fn cliReadReply(mut output_raw_strings: libc::c_int) -> libc::c_int {
    let mut _reply: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut out: hisds = 0 as hisds;
    let mut output: libc::c_int = 1 as libc::c_int;
    if redisGetReply(context, &mut _reply) != 0 as libc::c_int {
        if config.blocking_state_aborted != 0 {
            config.blocking_state_aborted = 0 as libc::c_int;
            config.monitor_mode = 0 as libc::c_int;
            config.pubsub_mode = 0 as libc::c_int;
            return cliConnect((1 as libc::c_int) << 0 as libc::c_int);
        }
        if config.shutdown != 0 {
            redisFree(context);
            context = 0 as *mut redisContext;
            return 0 as libc::c_int;
        }
        if config.interactive != 0 {
            if (*context).err == 1 as libc::c_int
                && (*__errno_location() == 104 as libc::c_int
                    || *__errno_location() == 32 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            if (*context).err == 3 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        cliPrintContextError();
        exit(1 as libc::c_int);
    }
    reply = _reply as *mut redisReply;
    config.last_cmd_type = (*reply).type_0;
    if config.cluster_mode != 0 && (*reply).type_0 == 6 as libc::c_int
        && (strncmp(
            (*reply).str_0,
            b"MOVED \0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
            || strncmp(
                (*reply).str_0,
                b"ASK \0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0)
    {
        let mut p: *mut libc::c_char = (*reply).str_0;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut slot: libc::c_int = 0;
        output = 0 as libc::c_int;
        s = strchr(p, ' ' as i32);
        p = strchr(s.offset(1 as libc::c_int as isize), ' ' as i32);
        *p = '\0' as i32 as libc::c_char;
        slot = atoi(s.offset(1 as libc::c_int as isize));
        s = strrchr(p.offset(1 as libc::c_int as isize), ':' as i32);
        *s = '\0' as i32 as libc::c_char;
        hi_sdsfree(config.conn_info.hostip);
        config.conn_info.hostip = hi_sdsnew(p.offset(1 as libc::c_int as isize));
        config.conn_info.hostport = atoi(s.offset(1 as libc::c_int as isize));
        if config.interactive != 0 {
            printf(
                b"-> Redirected to slot [%d] located at %s:%d\n\0" as *const u8
                    as *const libc::c_char,
                slot,
                config.conn_info.hostip,
                config.conn_info.hostport,
            );
        }
        config.cluster_reissue_command = 1 as libc::c_int;
        if strncmp(
            (*reply).str_0,
            b"ASK \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            config.cluster_send_asking = 1 as libc::c_int;
        }
        cliRefreshPrompt();
    } else if config.interactive == 0 && config.set_errcode != 0
        && (*reply).type_0 == 6 as libc::c_int
    {
        fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, (*reply).str_0);
        exit(1 as libc::c_int);
    }
    if output != 0 {
        out = cliFormatReply(reply, config.output, output_raw_strings);
        fwrite(
            out as *const libc::c_void,
            hi_sdslen(out),
            1 as libc::c_int as libc::c_ulong,
            stdout,
        );
        fflush(stdout);
        hi_sdsfree(out);
    }
    freeReplyObject(reply as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn cliSendCommand(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut repeat: libc::c_long,
) -> libc::c_int {
    let mut command: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut argvlen: *mut size_t = 0 as *mut size_t;
    let mut j: libc::c_int = 0;
    let mut output_raw: libc::c_int = 0;
    if context.is_null() {
        return -(1 as libc::c_int);
    }
    output_raw = 0 as libc::c_int;
    if strcasecmp(command, b"info\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(command, b"lolwut\0" as *const u8 as *const libc::c_char) == 0
        || argc >= 2 as libc::c_int
            && strcasecmp(command, b"debug\0" as *const u8 as *const libc::c_char) == 0
            && strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"htstats\0" as *const u8 as *const libc::c_char,
            ) == 0
        || argc >= 2 as libc::c_int
            && strcasecmp(command, b"debug\0" as *const u8 as *const libc::c_char) == 0
            && strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"htstats-key\0" as *const u8 as *const libc::c_char,
            ) == 0
        || argc >= 2 as libc::c_int
            && strcasecmp(command, b"debug\0" as *const u8 as *const libc::c_char) == 0
            && strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"client-eviction\0" as *const u8 as *const libc::c_char,
            ) == 0
        || argc >= 2 as libc::c_int
            && strcasecmp(command, b"memory\0" as *const u8 as *const libc::c_char) == 0
            && (strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"malloc-stats\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(1 as libc::c_int as isize),
                    b"doctor\0" as *const u8 as *const libc::c_char,
                ) == 0)
        || argc == 2 as libc::c_int
            && strcasecmp(command, b"cluster\0" as *const u8 as *const libc::c_char) == 0
            && (strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"nodes\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(1 as libc::c_int as isize),
                    b"info\0" as *const u8 as *const libc::c_char,
                ) == 0)
        || argc >= 2 as libc::c_int
            && strcasecmp(command, b"client\0" as *const u8 as *const libc::c_char) == 0
            && (strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"list\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(1 as libc::c_int as isize),
                    b"info\0" as *const u8 as *const libc::c_char,
                ) == 0)
        || argc == 3 as libc::c_int
            && strcasecmp(command, b"latency\0" as *const u8 as *const libc::c_char) == 0
            && strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"graph\0" as *const u8 as *const libc::c_char,
            ) == 0
        || argc == 2 as libc::c_int
            && strcasecmp(command, b"latency\0" as *const u8 as *const libc::c_char) == 0
            && strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"doctor\0" as *const u8 as *const libc::c_char,
            ) == 0
        || argc >= 2 as libc::c_int
            && strcasecmp(command, b"proxy\0" as *const u8 as *const libc::c_char) == 0
            && strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"info\0" as *const u8 as *const libc::c_char,
            ) == 0
    {
        output_raw = 1 as libc::c_int;
    }
    if strcasecmp(command, b"shutdown\0" as *const u8 as *const libc::c_char) == 0 {
        config.shutdown = 1 as libc::c_int;
    }
    if strcasecmp(command, b"monitor\0" as *const u8 as *const libc::c_char) == 0 {
        config.monitor_mode = 1 as libc::c_int;
    }
    if strcasecmp(command, b"subscribe\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(command, b"psubscribe\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(command, b"ssubscribe\0" as *const u8 as *const libc::c_char) == 0
    {
        config.pubsub_mode = 1 as libc::c_int;
    }
    if strcasecmp(command, b"sync\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(command, b"psync\0" as *const u8 as *const libc::c_char) == 0
    {
        config.slave_mode = 1 as libc::c_int;
    }
    if argc == 3 as libc::c_int
        && strcasecmp(
            *argv.offset(0 as libc::c_int as isize),
            b"script\0" as *const u8 as *const libc::c_char,
        ) == 0
        && strcasecmp(
            *argv.offset(1 as libc::c_int as isize),
            b"debug\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        if strcasecmp(
            *argv.offset(2 as libc::c_int as isize),
            b"yes\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcasecmp(
                *argv.offset(2 as libc::c_int as isize),
                b"sync\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            config.enable_ldb_on_eval = 1 as libc::c_int;
        } else {
            config.enable_ldb_on_eval = 0 as libc::c_int;
        }
    }
    if strcasecmp(command, b"eval\0" as *const u8 as *const libc::c_char) == 0
        && config.enable_ldb_on_eval != 0
    {
        config.eval_ldb = 1 as libc::c_int;
        config.output = 1 as libc::c_int;
    }
    argvlen = zmalloc(
        (argc as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    j = 0 as libc::c_int;
    while j < argc {
        *argvlen.offset(j as isize) = hi_sdslen(*argv.offset(j as isize));
        j += 1;
    }
    while repeat < 0 as libc::c_int as libc::c_long
        || {
            let fresh11 = repeat;
            repeat = repeat - 1;
            fresh11 > 0 as libc::c_int as libc::c_long
        }
    {
        redisAppendCommandArgv(context, argc, argv as *mut *const libc::c_char, argvlen);
        if config.monitor_mode != 0 {
            loop {
                if cliReadReply(output_raw) != 0 as libc::c_int {
                    cliPrintContextError();
                    exit(1 as libc::c_int);
                }
                fflush(stdout);
                if config.last_cmd_type == 6 as libc::c_int {
                    config.monitor_mode = 0 as libc::c_int;
                }
                if !(config.monitor_mode != 0) {
                    break;
                }
            }
            zfree(argvlen as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        if config.pubsub_mode != 0 {
            if config.output != 1 as libc::c_int {
                printf(
                    b"Reading messages... (press Ctrl-C to quit)\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            redisSetPushCallback(context, None);
            while config.pubsub_mode != 0 {
                if cliReadReply(output_raw) != 0 as libc::c_int {
                    cliPrintContextError();
                    exit(1 as libc::c_int);
                }
                fflush(stdout);
                if config.pubsub_mode == 0 || config.last_cmd_type == 6 as libc::c_int {
                    if config.push_output != 0 {
                        redisSetPushCallback(
                            context,
                            Some(
                                cliPushHandler
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                        *mut libc::c_void,
                                    ) -> (),
                            ),
                        );
                    }
                    config.pubsub_mode = 0 as libc::c_int;
                }
            }
        } else {
            if config.slave_mode != 0 {
                printf(
                    b"Entering replica output mode...  (press Ctrl-C to quit)\n\0"
                        as *const u8 as *const libc::c_char,
                );
                slaveMode();
                config.slave_mode = 0 as libc::c_int;
                zfree(argvlen as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            if cliReadReply(output_raw) != 0 as libc::c_int {
                zfree(argvlen as *mut libc::c_void);
                return -(1 as libc::c_int);
            } else {
                if strcasecmp(command, b"select\0" as *const u8 as *const libc::c_char)
                    == 0 && argc == 2 as libc::c_int
                    && config.last_cmd_type != 6 as libc::c_int
                {
                    config.dbnum = atoi(*argv.offset(1 as libc::c_int as isize));
                    config.conn_info.input_dbnum = config.dbnum;
                    cliRefreshPrompt();
                } else if strcasecmp(
                    command,
                    b"auth\0" as *const u8 as *const libc::c_char,
                ) == 0 && (argc == 2 as libc::c_int || argc == 3 as libc::c_int)
                {
                    cliSelect();
                } else if strcasecmp(
                    command,
                    b"multi\0" as *const u8 as *const libc::c_char,
                ) == 0 && argc == 1 as libc::c_int
                    && config.last_cmd_type != 6 as libc::c_int
                {
                    config.in_multi = 1 as libc::c_int;
                    config.pre_multi_dbnum = config.dbnum;
                    cliRefreshPrompt();
                } else if strcasecmp(
                    command,
                    b"exec\0" as *const u8 as *const libc::c_char,
                ) == 0 && argc == 1 as libc::c_int && config.in_multi != 0
                {
                    config.in_multi = 0 as libc::c_int;
                    if config.last_cmd_type == 6 as libc::c_int
                        || config.last_cmd_type == 4 as libc::c_int
                    {
                        config.dbnum = config.pre_multi_dbnum;
                        config.conn_info.input_dbnum = config.dbnum;
                    }
                    cliRefreshPrompt();
                } else if strcasecmp(
                    command,
                    b"discard\0" as *const u8 as *const libc::c_char,
                ) == 0 && argc == 1 as libc::c_int
                    && config.last_cmd_type != 6 as libc::c_int
                {
                    config.in_multi = 0 as libc::c_int;
                    config.dbnum = config.pre_multi_dbnum;
                    config.conn_info.input_dbnum = config.dbnum;
                    cliRefreshPrompt();
                } else if strcasecmp(
                    command,
                    b"reset\0" as *const u8 as *const libc::c_char,
                ) == 0 && argc == 1 as libc::c_int
                    && config.last_cmd_type != 6 as libc::c_int
                {
                    config.in_multi = 0 as libc::c_int;
                    config.dbnum = 0 as libc::c_int;
                    config.conn_info.input_dbnum = 0 as libc::c_int;
                    config.resp3 = 0 as libc::c_int;
                    cliRefreshPrompt();
                }
            }
            if config.cluster_reissue_command != 0 {
                break;
            }
            if config.interval != 0 {
                usleep(config.interval as __useconds_t);
            }
            fflush(stdout);
        }
    }
    zfree(argvlen as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn reconnectingRedisCommand(
    mut c: *mut redisContext,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut redisReply {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut tries: libc::c_int = 0 as libc::c_int;
    let mut ap: core::ffi::VaListImpl;
    if (*c).err == 0 {} else {
        __assert_fail(
            b"!c->err\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            1905 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"redisReply *reconnectingRedisCommand(redisContext *, const char *, ...)\0",
            ))
                .as_ptr(),
        );
    };
    while reply.is_null() {
        while (*c).err & (1 as libc::c_int | 3 as libc::c_int) != 0 {
            printf(b"\r\x1B[0K\0" as *const u8 as *const libc::c_char);
            tries += 1;
            printf(b"Reconnecting... %d\r\0" as *const u8 as *const libc::c_char, tries);
            fflush(stdout);
            redisFree(c);
            c = redisConnect(config.conn_info.hostip, config.conn_info.hostport);
            if (*c).err == 0 && config.tls != 0 {
                let mut err: *const libc::c_char = 0 as *const libc::c_char;
                if cliSecureConnection(c, config.sslconfig, &mut err)
                    == -(1 as libc::c_int) && !err.is_null()
                {
                    fprintf(
                        stderr,
                        b"TLS Error: %s\n\0" as *const u8 as *const libc::c_char,
                        err,
                    );
                    exit(1 as libc::c_int);
                }
            }
            usleep(1000000 as libc::c_int as __useconds_t);
        }
        ap = args.clone();
        reply = redisvCommand(c, fmt, ap.as_va_list()) as *mut redisReply;
        if (*c).err != 0 && (*c).err & (1 as libc::c_int | 3 as libc::c_int) == 0 {
            fprintf(
                stderr,
                b"Error: %s\n\0" as *const u8 as *const libc::c_char,
                ((*c).errstr).as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        } else {
            if tries > 0 as libc::c_int {
                printf(b"\r\x1B[0K\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    context = c;
    return reply;
}
unsafe extern "C" fn parseOptions(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < argc {
        let mut lastarg: libc::c_int = (i == argc - 1 as libc::c_int) as libc::c_int;
        if strcmp(*argv.offset(i as isize), b"-h\0" as *const u8 as *const libc::c_char)
            == 0 && lastarg == 0
        {
            hi_sdsfree(config.conn_info.hostip);
            i += 1;
            config.conn_info.hostip = hi_sdsnew(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg != 0
        {
            usage(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            usage(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-x\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.stdin_lastarg = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-X\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            config.stdin_tag_arg = 1 as libc::c_int;
            i += 1;
            config.stdin_tag_name = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-p\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.conn_info.hostport = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"-s\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.hostsocket = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-r\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config
                .repeat = strtoll(
                *argv.offset(i as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            ) as libc::c_long;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-i\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            let mut seconds: libc::c_double = atof(*argv.offset(i as isize));
            config
                .interval = (seconds * 1000000 as libc::c_int as libc::c_double)
                as libc::c_long;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-n\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.conn_info.input_dbnum = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-auth-warning\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.no_auth_warning = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--askpass\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.askpass = 1 as libc::c_int;
        } else if (strcmp(
            *argv.offset(i as isize),
            b"-a\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--pass\0" as *const u8 as *const libc::c_char,
            ) == 0) && lastarg == 0
        {
            i += 1;
            config.conn_info.auth = hi_sdsnew(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--user\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.conn_info.user = hi_sdsnew(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"-u\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            parseRedisUri(
                *argv.offset(i as isize),
                b"redis-cli\0" as *const u8 as *const libc::c_char,
                &mut config.conn_info,
                &mut config.tls,
            );
        } else if strcmp(
            *argv.offset(i as isize),
            b"--raw\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.output = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-raw\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.output = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--quoted-input\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.quoted_input = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--csv\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.output = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--json\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if config.resp3 == 0 as libc::c_int {
                config.resp3 = 2 as libc::c_int;
            }
            config.output = 3 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--quoted-json\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if config.resp3 == 0 as libc::c_int {
                config.resp3 = 2 as libc::c_int;
            }
            config.output = 4 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--latency\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.latency_mode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--latency-dist\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.latency_dist_mode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--mono\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            spectrum_palette = spectrum_palette_mono.as_mut_ptr();
            spectrum_palette_size = spectrum_palette_mono_size;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--latency-history\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.latency_mode = 1 as libc::c_int;
            config.latency_history = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--lru-test\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            config.lru_test_mode = 1 as libc::c_int;
            i += 1;
            config
                .lru_test_sample_size = strtoll(
                *argv.offset(i as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            );
        } else if strcmp(
            *argv.offset(i as isize),
            b"--slave\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.slave_mode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--replica\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.slave_mode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--stat\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.stat_mode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--scan\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.scan_mode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--pattern\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            hi_sdsfree(config.pattern);
            i += 1;
            config.pattern = hi_sdsnew(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--quoted-pattern\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            hi_sdsfree(config.pattern);
            i += 1;
            config.pattern = unquoteCString(*argv.offset(i as isize));
            if (config.pattern).is_null() {
                fprintf(
                    stderr,
                    b"Invalid quoted string specified for --quoted-pattern.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"--intrinsic-latency\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            config.intrinsic_latency_mode = 1 as libc::c_int;
            i += 1;
            config.intrinsic_latency_duration = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--rdb\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            config.getrdb_mode = 1 as libc::c_int;
            i += 1;
            config.rdb_filename = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--functions-rdb\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            config.get_functions_rdb_mode = 1 as libc::c_int;
            i += 1;
            config.rdb_filename = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--pipe\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.pipe_mode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--pipe-timeout\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.pipe_timeout = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--bigkeys\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.bigkeys = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--memkeys\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.memkeys = 1 as libc::c_int;
            config.memkeys_samples = 0 as libc::c_int as libc::c_uint;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--memkeys-samples\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.memkeys = 1 as libc::c_int;
            i += 1;
            config.memkeys_samples = atoi(*argv.offset(i as isize)) as libc::c_uint;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--hotkeys\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.hotkeys = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--eval\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.eval = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--ldb\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.eval_ldb = 1 as libc::c_int;
            config.output = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--ldb-sync-mode\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.eval_ldb = 1 as libc::c_int;
            config.eval_ldb_sync = 1 as libc::c_int;
            config.output = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-c\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_mode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            hi_sdsfree(config.mb_delim);
            i += 1;
            config.mb_delim = hi_sdsnew(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"-D\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            hi_sdsfree(config.cmd_delim);
            i += 1;
            config.cmd_delim = hi_sdsnew(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"-e\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.set_errcode = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--verbose\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.verbose = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            if !(config.cluster_manager_command.name).is_null() {
                usage(1 as libc::c_int);
            }
            i += 1;
            let mut cmd: *mut libc::c_char = *argv.offset(i as isize);
            let mut j: libc::c_int = i;
            while j < argc
                && *(*argv.offset(j as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int != '-' as i32
            {
                j += 1;
            }
            if j > i {
                j -= 1;
            }
            let mut err: libc::c_int = createClusterManagerCommand(
                cmd,
                j - i,
                argv.offset(i as isize).offset(1 as libc::c_int as isize),
            );
            if err != 0 {
                exit(err);
            }
            i = j;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg != 0
        {
            usage(1 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-only-masters\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 11 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-only-replicas\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 12 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-replicas\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.replicas = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-master-id\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.master_id = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-from\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.from = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-to\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.to = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-from-user\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.from_user = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-from-pass\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.from_pass = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-from-askpass\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.from_askpass = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-weight\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            if !(config.cluster_manager_command.weight).is_null() {
                fprintf(
                    stderr,
                    b"WARNING: you cannot use --cluster-weight more than once.\nYou can set more weights by adding them as a space-separated list, ie:\n--cluster-weight n1=w n2=w\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            let mut widx: libc::c_int = i + 1 as libc::c_int;
            let mut weight: *mut *mut libc::c_char = argv.offset(widx as isize);
            let mut wargc: libc::c_int = 0 as libc::c_int;
            while widx < argc {
                if strstr(
                    *argv.offset(widx as isize),
                    b"--\0" as *const u8 as *const libc::c_char,
                ) == *argv.offset(widx as isize)
                {
                    break;
                }
                if (strchr(*argv.offset(widx as isize), '=' as i32)).is_null() {
                    break;
                }
                wargc += 1;
                widx += 1;
            }
            if wargc > 0 as libc::c_int {
                config.cluster_manager_command.weight = weight;
                config.cluster_manager_command.weight_argc = wargc;
                i += wargc;
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-slots\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.slots = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-timeout\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.timeout = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-pipeline\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config.cluster_manager_command.pipeline = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-threshold\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            config
                .cluster_manager_command
                .threshold = atof(*argv.offset(i as isize)) as libc::c_float;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-yes\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-simulate\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 5 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-replace\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 6 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-copy\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 7 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-slave\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-use-empty-masters\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 4 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-search-multiple-owners\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 9 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--cluster-fix-with-unreachable-masters\0" as *const u8
                as *const libc::c_char,
        ) == 0
        {
            config.cluster_manager_command.flags
                |= (1 as libc::c_int) << 10 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-v\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--version\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            let mut version: hisds = cliVersion();
            printf(b"redis-cli %s\n\0" as *const u8 as *const libc::c_char, version);
            hi_sdsfree(version);
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-2\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.resp2 = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-3\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.resp3 = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--show-pushes\0" as *const u8 as *const libc::c_char,
        ) == 0 && lastarg == 0
        {
            i += 1;
            let mut argval: *mut libc::c_char = *argv.offset(i as isize);
            if strncasecmp(
                argval,
                b"n\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            ) == 0
            {
                config.push_output = 0 as libc::c_int;
            } else if strncasecmp(
                argval,
                b"y\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            ) == 0
            {
                config.push_output = 1 as libc::c_int;
            } else {
                fprintf(
                    stderr,
                    b"Unknown --show-pushes value '%s' (valid: '[y]es', '[n]o')\n\0"
                        as *const u8 as *const libc::c_char,
                    argval,
                );
            }
        } else if !(config.cluster_manager_command.name).is_null()
            && *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int != '-' as i32
        {
            if config.cluster_manager_command.argc == 0 as libc::c_int {
                let mut j_0: libc::c_int = i + 1 as libc::c_int;
                while j_0 < argc
                    && *(*argv.offset(j_0 as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int != '-' as i32
                {
                    j_0 += 1;
                }
                let mut cmd_argc: libc::c_int = j_0 - i;
                config.cluster_manager_command.argc = cmd_argc;
                config.cluster_manager_command.argv = argv.offset(i as isize);
                if cmd_argc > 1 as libc::c_int {
                    i = j_0 - 1 as libc::c_int;
                }
            }
        } else {
            if !(*(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32)
            {
                break;
            }
            fprintf(
                stderr,
                b"Unrecognized option or bad number of args for: '%s'\n\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(i as isize),
            );
            exit(1 as libc::c_int);
        }
        i += 1;
    }
    if !(config.hostsocket).is_null() && config.cluster_mode != 0 {
        fprintf(
            stderr,
            b"Options -c and -s are mutually exclusive.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if config.resp2 != 0 && config.resp3 == 1 as libc::c_int {
        fprintf(
            stderr,
            b"Options -2 and -3 are mutually exclusive.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if config.eval_ldb != 0 && (config.eval).is_null() {
        fprintf(
            stderr,
            b"Options --ldb and --ldb-sync-mode require --eval.\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Try %s --help for more information.\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    if config.no_auth_warning == 0 && !(config.conn_info.auth).is_null() {
        fputs(
            b"Warning: Using a password with '-a' or '-u' option on the command line interface may not be safe.\n\0"
                as *const u8 as *const libc::c_char,
            stderr,
        );
    }
    if config.get_functions_rdb_mode != 0 && config.getrdb_mode != 0 {
        fprintf(
            stderr,
            b"Option --functions-rdb and --rdb are mutually exclusive.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if config.stdin_lastarg != 0 && config.stdin_tag_arg != 0 {
        fprintf(
            stderr,
            b"Options -x and -X are mutually exclusive.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return i;
}
unsafe extern "C" fn parseEnv() {
    let mut auth: *mut libc::c_char = getenv(
        b"REDISCLI_AUTH\0" as *const u8 as *const libc::c_char,
    );
    if !auth.is_null() && (config.conn_info.auth).is_null() {
        config.conn_info.auth = auth;
    }
    let mut cluster_yes: *mut libc::c_char = getenv(
        b"REDISCLI_CLUSTER_YES\0" as *const u8 as *const libc::c_char,
    );
    if !cluster_yes.is_null()
        && strcmp(cluster_yes, b"1\0" as *const u8 as *const libc::c_char) == 0
    {
        config.cluster_manager_command.flags |= (1 as libc::c_int) << 2 as libc::c_int;
    }
}
unsafe extern "C" fn usage(mut err: libc::c_int) {
    let mut version: hisds = cliVersion();
    let mut target: *mut FILE = if err != 0 { stderr } else { stdout };
    fprintf(
        target,
        b"redis-cli %s\n\nUsage: redis-cli [OPTIONS] [cmd [arg [arg ...]]]\n  -h <hostname>      Server hostname (default: 127.0.0.1).\n  -p <port>          Server port (default: 6379).\n  -s <socket>        Server socket (overrides hostname and port).\n  -a <password>      Password to use when connecting to the server.\n                     You can also use the REDISCLI_AUTH environment\n                     variable to pass this password more safely\n                     (if both are used, this argument takes precedence).\n  --user <username>  Used to send ACL style 'AUTH username pass'. Needs -a.\n  --pass <password>  Alias of -a for consistency with the new --user option.\n  --askpass          Force user to input password with mask from STDIN.\n                     If this argument is used, '-a' and REDISCLI_AUTH\n                     environment variable will be ignored.\n  -u <uri>           Server URI.\n  -r <repeat>        Execute specified command N times.\n  -i <interval>      When -r is used, waits <interval> seconds per command.\n                     It is possible to specify sub-second times like -i 0.1.\n                     This interval is also used in --scan and --stat per cycle.\n                     and in --bigkeys, --memkeys, and --hotkeys per 100 cycles.\n  -n <db>            Database number.\n  -2                 Start session in RESP2 protocol mode.\n  -3                 Start session in RESP3 protocol mode.\n  -x                 Read last argument from STDIN (see example below).\n  -X                 Read <tag> argument from STDIN (see example below).\n  -d <delimiter>     Delimiter between response bulks for raw formatting (default: \\n).\n  -D <delimiter>     Delimiter between responses for raw formatting (default: \\n).\n  -c                 Enable cluster mode (follow -ASK and -MOVED redirections).\n  -e                 Return exit error code when command execution fails.\n  --raw              Use raw formatting for replies (default when STDOUT is\n                     not a tty).\n  --no-raw           Force formatted output even when STDOUT is not a tty.\n  --quoted-input     Force input to be handled as quoted strings.\n  --csv              Output in CSV format.\n  --json             Output in JSON format (default RESP3, use -2 if you want to use with RESP2).\n  --quoted-json      Same as --json, but produce ASCII-safe quoted strings, not Unicode.\n  --show-pushes <yn> Whether to print RESP3 PUSH messages.  Enabled by default when\n                     STDOUT is a tty but can be overridden with --show-pushes no.\n  --stat             Print rolling stats about server: mem, clients, ...\n\0"
            as *const u8 as *const libc::c_char,
        version,
    );
    fprintf(
        target,
        b"  --latency          Enter a special mode continuously sampling latency.\n                     If you use this mode in an interactive session it runs\n                     forever displaying real-time stats. Otherwise if --raw or\n                     --csv is specified, or if you redirect the output to a non\n                     TTY, it samples the latency for 1 second (you can use\n                     -i to change the interval), then produces a single output\n                     and exits.\n  --latency-history  Like --latency but tracking latency changes over time.\n                     Default time interval is 15 sec. Change it using -i.\n  --latency-dist     Shows latency as a spectrum, requires xterm 256 colors.\n                     Default time interval is 1 sec. Change it using -i.\n  --lru-test <keys>  Simulate a cache workload with an 80-20 distribution.\n  --replica          Simulate a replica showing commands received from the master.\n  --rdb <filename>   Transfer an RDB dump from remote server to local file.\n                     Use filename of \"-\" to write to stdout.\n --functions-rdb <filename> Like --rdb but only get the functions (not the keys)\n                     when getting the RDB dump file.\n  --pipe             Transfer raw Redis protocol from stdin to server.\n  --pipe-timeout <n> In --pipe mode, abort with error if after sending all data.\n                     no reply is received within <n> seconds.\n                     Default timeout: %d. Use 0 to wait forever.\n\0"
            as *const u8 as *const libc::c_char,
        30 as libc::c_int,
    );
    fprintf(
        target,
        b"  --bigkeys          Sample Redis keys looking for keys with many elements (complexity).\n  --memkeys          Sample Redis keys looking for keys consuming a lot of memory.\n  --memkeys-samples <n> Sample Redis keys looking for keys consuming a lot of memory.\n                     And define number of key elements to sample\n  --hotkeys          Sample Redis keys looking for hot keys.\n                     only works when maxmemory-policy is *lfu.\n  --scan             List all keys using the SCAN command.\n  --pattern <pat>    Keys pattern when using the --scan, --bigkeys or --hotkeys\n                     options (default: *).\n  --quoted-pattern <pat> Same as --pattern, but the specified string can be\n                         quoted, in order to pass an otherwise non binary-safe string.\n  --intrinsic-latency <sec> Run a test to measure intrinsic system latency.\n                     The test will run for the specified amount of seconds.\n  --eval <file>      Send an EVAL command using the Lua script at <file>.\n  --ldb              Used with --eval enable the Redis Lua debugger.\n  --ldb-sync-mode    Like --ldb but uses the synchronous Lua debugger, in\n                     this mode the server is blocked and script changes are\n                     not rolled back from the server memory.\n  --cluster <command> [args...] [opts...]\n                     Cluster Manager command and arguments (see below).\n  --verbose          Verbose mode.\n  --no-auth-warning  Don't show warning message when using password on command\n                     line interface.\n  --help             Output this help and exit.\n  --version          Output version and exit.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        target,
        b"Cluster Manager Commands:\n  Use --cluster help to list all available cluster manager commands.\n\nExamples:\n  cat /etc/passwd | redis-cli -x set mypasswd\n  redis-cli -D \"\" --raw dump key > key.dump && redis-cli -X dump_tag restore key2 0 dump_tag replace < key.dump\n  redis-cli -r 100 lpush mylist x\n  redis-cli -r 100 -i 1 info | grep used_memory_human:\n  redis-cli --quoted-input set '\"null-\\x00-separated\"' value\n  redis-cli --eval myscript.lua key1 key2 , arg1 arg2 arg3\n  redis-cli --scan --pattern '*:12345*'\n\n  (Note: when using --eval the comma separates KEYS[] from ARGV[] items)\n\nWhen no command is given, redis-cli starts in interactive mode.\nType \"help\" in interactive mode for information on available commands\nand settings.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    hi_sdsfree(version);
    exit(err);
}
unsafe extern "C" fn confirmWithYes(
    mut msg: *mut libc::c_char,
    mut ignore_force: libc::c_int,
) -> libc::c_int {
    if ignore_force == 0
        && config.cluster_manager_command.flags & (1 as libc::c_int) << 2 as libc::c_int
            != 0
    {
        return 1 as libc::c_int;
    }
    printf(b"%s (type 'yes' to accept): \0" as *const u8 as *const libc::c_char, msg);
    fflush(stdout);
    let mut buf: [libc::c_char; 4] = [0; 4];
    let mut nread: libc::c_int = read(
        fileno(stdin),
        buf.as_mut_ptr() as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) as libc::c_int;
    buf[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    return (nread != 0 as libc::c_int
        && strcmp(b"yes\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr()) == 0)
        as libc::c_int;
}
unsafe extern "C" fn issueCommandRepeat(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut repeat: libc::c_long,
) -> libc::c_int {
    if config.eval_ldb == 0
        && (strcasecmp(
            *argv.offset(0 as libc::c_int as isize),
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcasecmp(
                *argv.offset(0 as libc::c_int as isize),
                b"?\0" as *const u8 as *const libc::c_char,
            ) == 0)
    {
        argc -= 1;
        argv = argv.offset(1);
        cliOutputHelp(argc, argv);
        return 0 as libc::c_int;
    }
    loop {
        if config.cluster_reissue_command != 0 || context.is_null()
            || (*context).err == 1 as libc::c_int || (*context).err == 3 as libc::c_int
        {
            if cliConnect((1 as libc::c_int) << 0 as libc::c_int) != 0 as libc::c_int {
                cliPrintContextError();
                config.cluster_reissue_command = 0 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
        config.cluster_reissue_command = 0 as libc::c_int;
        if config.cluster_send_asking != 0 {
            if cliSendAsking() != 0 as libc::c_int {
                cliPrintContextError();
                return -(1 as libc::c_int);
            }
        }
        if cliSendCommand(argc, argv, repeat) != 0 as libc::c_int {
            cliPrintContextError();
            redisFree(context);
            context = 0 as *mut redisContext;
            return -(1 as libc::c_int);
        }
        if !(config.cluster_mode != 0 && config.cluster_reissue_command != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn issueCommand(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    return issueCommandRepeat(argc, argv, config.repeat);
}
unsafe extern "C" fn cliSplitArgs(
    mut line: *mut libc::c_char,
    mut argc: *mut libc::c_int,
) -> *mut hisds {
    if config.eval_ldb != 0
        && (strstr(line, b"eval \0" as *const u8 as *const libc::c_char) == line
            || strstr(line, b"e \0" as *const u8 as *const libc::c_char) == line)
    {
        let mut argv: *mut hisds = hi_sds_malloc(
            (core::mem::size_of::<hisds>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut hisds;
        *argc = 2 as libc::c_int;
        let mut len: libc::c_int = strlen(line) as libc::c_int;
        let mut elen: libc::c_int = if *line.offset(1 as libc::c_int as isize)
            as libc::c_int == ' ' as i32
        {
            2 as libc::c_int
        } else {
            5 as libc::c_int
        };
        let ref mut fresh12 = *argv.offset(0 as libc::c_int as isize);
        *fresh12 = hi_sdsnewlen(
            line as *const libc::c_void,
            (elen - 1 as libc::c_int) as size_t,
        );
        let ref mut fresh13 = *argv.offset(1 as libc::c_int as isize);
        *fresh13 = hi_sdsnewlen(
            line.offset(elen as isize) as *const libc::c_void,
            (len - elen) as size_t,
        );
        return argv;
    } else {
        return hi_sdssplitargs(line, argc)
    };
}
#[no_mangle]
pub unsafe extern "C" fn cliSetPreferences(
    mut argv: *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut interactive: libc::c_int,
) {
    if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b":set\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc >= 2 as libc::c_int
    {
        if strcasecmp(
            *argv.offset(1 as libc::c_int as isize),
            b"hints\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            pref.hints = 1 as libc::c_int;
        } else if strcasecmp(
            *argv.offset(1 as libc::c_int as isize),
            b"nohints\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            pref.hints = 0 as libc::c_int;
        } else {
            printf(
                b"%sunknown redis-cli preference '%s'\n\0" as *const u8
                    as *const libc::c_char,
                if interactive != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b".redisclirc: \0" as *const u8 as *const libc::c_char
                },
                *argv.offset(1 as libc::c_int as isize),
            );
        }
    } else {
        printf(
            b"%sunknown redis-cli internal command '%s'\n\0" as *const u8
                as *const libc::c_char,
            if interactive != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b".redisclirc: \0" as *const u8 as *const libc::c_char
            },
            *argv.offset(0 as libc::c_int as isize),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn cliLoadPreferences() {
    let mut rcfile: hisds = getDotfilePath(
        b"REDISCLI_RCFILE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".redisclirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if rcfile.is_null() {
        return;
    }
    let mut fp: *mut FILE = fopen(
        rcfile as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if !fp.is_null() {
        while !(fgets(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            fp,
        ))
            .is_null()
        {
            let mut argv: *mut hisds = 0 as *mut hisds;
            let mut argc: libc::c_int = 0;
            argv = hi_sdssplitargs(buf.as_mut_ptr(), &mut argc);
            if argc > 0 as libc::c_int {
                cliSetPreferences(argv, argc, 0 as libc::c_int);
            }
            hi_sdsfreesplitres(argv, argc);
        }
        fclose(fp);
    }
    hi_sdsfree(rcfile);
}
unsafe extern "C" fn isSensitiveCommand(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"auth\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 1 as libc::c_int
    } else {
        if argc > 1 as libc::c_int
            && strcasecmp(
                *argv.offset(0 as libc::c_int as isize),
                b"acl\0" as *const u8 as *const libc::c_char,
            ) == 0
            && strcasecmp(
                *argv.offset(1 as libc::c_int as isize),
                b"setuser\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            return 1 as libc::c_int
        } else {
            if argc > 2 as libc::c_int
                && strcasecmp(
                    *argv.offset(0 as libc::c_int as isize),
                    b"config\0" as *const u8 as *const libc::c_char,
                ) == 0
                && strcasecmp(
                    *argv.offset(1 as libc::c_int as isize),
                    b"set\0" as *const u8 as *const libc::c_char,
                ) == 0
                && (strcasecmp(
                    *argv.offset(2 as libc::c_int as isize),
                    b"masterauth\0" as *const u8 as *const libc::c_char,
                ) == 0
                    || strcasecmp(
                        *argv.offset(2 as libc::c_int as isize),
                        b"masteruser\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || strcasecmp(
                        *argv.offset(2 as libc::c_int as isize),
                        b"requirepass\0" as *const u8 as *const libc::c_char,
                    ) == 0)
            {
                return 1 as libc::c_int
            } else {
                if argc > 4 as libc::c_int
                    && strcasecmp(
                        *argv.offset(0 as libc::c_int as isize),
                        b"hello\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    let mut j: libc::c_int = 2 as libc::c_int;
                    while j < argc {
                        let mut moreargs: libc::c_int = argc - 1 as libc::c_int - j;
                        if strcasecmp(
                            *argv.offset(j as isize),
                            b"AUTH\0" as *const u8 as *const libc::c_char,
                        ) == 0 && moreargs >= 2 as libc::c_int
                        {
                            return 1 as libc::c_int
                        } else {
                            if strcasecmp(
                                *argv.offset(j as isize),
                                b"SETNAME\0" as *const u8 as *const libc::c_char,
                            ) == 0 && moreargs != 0
                            {
                                j += 1;
                            } else {
                                return 0 as libc::c_int
                            }
                        }
                        j += 1;
                    }
                } else if argc > 7 as libc::c_int
                    && strcasecmp(
                        *argv.offset(0 as libc::c_int as isize),
                        b"migrate\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    let mut j_0: libc::c_int = 6 as libc::c_int;
                    while j_0 < argc {
                        let mut moreargs_0: libc::c_int = argc - 1 as libc::c_int - j_0;
                        if strcasecmp(
                            *argv.offset(j_0 as isize),
                            b"auth\0" as *const u8 as *const libc::c_char,
                        ) == 0 && moreargs_0 != 0
                        {
                            return 1 as libc::c_int
                        } else {
                            if strcasecmp(
                                *argv.offset(j_0 as isize),
                                b"auth2\0" as *const u8 as *const libc::c_char,
                            ) == 0 && moreargs_0 >= 2 as libc::c_int
                            {
                                return 1 as libc::c_int
                            } else {
                                if strcasecmp(
                                    *argv.offset(j_0 as isize),
                                    b"keys\0" as *const u8 as *const libc::c_char,
                                ) == 0 && moreargs_0 != 0
                                {
                                    return 0 as libc::c_int;
                                }
                            }
                        }
                        j_0 += 1;
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn repl() {
    let mut historyfile: hisds = 0 as hisds;
    let mut history: libc::c_int = 0 as libc::c_int;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut argv: *mut hisds = 0 as *mut hisds;
    if config.eval_ldb == 0 {
        cliInitHelp();
    }
    config.interactive = 1 as libc::c_int;
    linenoiseSetMultiLine(1 as libc::c_int);
    linenoiseSetCompletionCallback(
        Some(
            completionCallback
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut linenoiseCompletions,
                ) -> (),
        ),
    );
    linenoiseSetHintsCallback(
        Some(
            hintsCallback
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_int,
                    *mut libc::c_int,
                ) -> *mut libc::c_char,
        ),
    );
    linenoiseSetFreeHintsCallback(
        Some(freeHintsCallback as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if isatty(fileno(stdin)) != 0 {
        historyfile = getDotfilePath(
            b"REDISCLI_HISTFILE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b".rediscli_history\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        history = 1 as libc::c_int;
        if !historyfile.is_null() {
            linenoiseHistoryLoad(historyfile as *const libc::c_char);
        }
        cliLoadPreferences();
    }
    cliRefreshPrompt();
    loop {
        line = linenoise(
            if !context.is_null() {
                (config.prompt).as_mut_ptr() as *const libc::c_char
            } else {
                b"not connected> \0" as *const u8 as *const libc::c_char
            },
        );
        if line.is_null() {
            break;
        }
        if *line.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            let mut repeat: libc::c_long = 1 as libc::c_int as libc::c_long;
            let mut skipargs: libc::c_int = 0 as libc::c_int;
            let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
            argv = cliSplitArgs(line, &mut argc);
            if argv.is_null() {
                printf(b"Invalid argument(s)\n\0" as *const u8 as *const libc::c_char);
                fflush(stdout);
                if history != 0 {
                    linenoiseHistoryAdd(line);
                }
                if !historyfile.is_null() {
                    linenoiseHistorySave(historyfile as *const libc::c_char);
                }
                linenoiseFree(line as *mut libc::c_void);
                continue;
            } else if argc == 0 as libc::c_int {
                hi_sdsfreesplitres(argv, argc);
                linenoiseFree(line as *mut libc::c_void);
                continue;
            } else {
                *__errno_location() = 0 as libc::c_int;
                repeat = strtol(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    &mut endptr,
                    10 as libc::c_int,
                );
                if argc > 1 as libc::c_int && *endptr as libc::c_int == '\0' as i32 {
                    if *__errno_location() == 34 as libc::c_int
                        || *__errno_location() == 22 as libc::c_int
                        || repeat <= 0 as libc::c_int as libc::c_long
                    {
                        fputs(
                            b"Invalid redis-cli repeat command option value.\n\0"
                                as *const u8 as *const libc::c_char,
                            stdout,
                        );
                        hi_sdsfreesplitres(argv, argc);
                        linenoiseFree(line as *mut libc::c_void);
                        continue;
                    } else {
                        skipargs = 1 as libc::c_int;
                    }
                } else {
                    repeat = 1 as libc::c_int as libc::c_long;
                }
                if isSensitiveCommand(argc - skipargs, argv.offset(skipargs as isize))
                    == 0
                {
                    if history != 0 {
                        linenoiseHistoryAdd(line);
                    }
                    if !historyfile.is_null() {
                        linenoiseHistorySave(historyfile as *const libc::c_char);
                    }
                }
                if strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"quit\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"exit\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    exit(0 as libc::c_int);
                } else if *(*argv.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                {
                    cliSetPreferences(argv, argc, 1 as libc::c_int);
                    hi_sdsfreesplitres(argv, argc);
                    linenoiseFree(line as *mut libc::c_void);
                    continue;
                } else {
                    if strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"restart\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        if !(config.eval).is_null() {
                            config.eval_ldb = 1 as libc::c_int;
                            config.output = 1 as libc::c_int;
                            hi_sdsfreesplitres(argv, argc);
                            linenoiseFree(line as *mut libc::c_void);
                            return;
                        } else {
                            printf(
                                b"Use 'restart' only in Lua debugging mode.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            fflush(stdout);
                        }
                    } else if argc == 3 as libc::c_int
                        && strcasecmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            b"connect\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        hi_sdsfree(config.conn_info.hostip);
                        config
                            .conn_info
                            .hostip = hi_sdsnew(
                            *argv.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        config
                            .conn_info
                            .hostport = atoi(
                            *argv.offset(2 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        cliRefreshPrompt();
                        cliConnect((1 as libc::c_int) << 0 as libc::c_int);
                    } else if argc == 1 as libc::c_int
                        && strcasecmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            b"clear\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        linenoiseClearScreen();
                    } else {
                        let mut start_time: libc::c_longlong = mstime();
                        let mut elapsed: libc::c_longlong = 0;
                        issueCommandRepeat(
                            argc - skipargs,
                            argv.offset(skipargs as isize),
                            repeat,
                        );
                        if config.eval_ldb_end != 0 {
                            config.eval_ldb_end = 0 as libc::c_int;
                            cliReadReply(0 as libc::c_int);
                            printf(
                                b"\n(Lua debugging session ended%s)\n\n\0" as *const u8
                                    as *const libc::c_char,
                                if config.eval_ldb_sync != 0 {
                                    b"\0" as *const u8 as *const libc::c_char
                                } else {
                                    b" -- dataset changes rolled back\0" as *const u8
                                        as *const libc::c_char
                                },
                            );
                            cliInitHelp();
                        }
                        elapsed = mstime() - start_time;
                        if elapsed >= 500 as libc::c_int as libc::c_longlong
                            && config.output == 0 as libc::c_int
                        {
                            printf(
                                b"(%.2fs)\n\0" as *const u8 as *const libc::c_char,
                                elapsed as libc::c_double
                                    / 1000 as libc::c_int as libc::c_double,
                            );
                        }
                    }
                    hi_sdsfreesplitres(argv, argc);
                }
            }
        }
        linenoiseFree(line as *mut libc::c_void);
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn noninteractive(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut sds_args: *mut hisds = getSdsArrayFromArgv(argc, argv, config.quoted_input);
    if sds_args.is_null() {
        printf(b"Invalid quoted string\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if config.stdin_lastarg != 0 {
        sds_args = hi_sds_realloc(
            sds_args as *mut libc::c_void,
            ((argc + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(core::mem::size_of::<hisds>() as libc::c_ulong),
        ) as *mut hisds;
        let ref mut fresh14 = *sds_args.offset(argc as isize);
        *fresh14 = readArgFromStdin();
        argc += 1;
    } else if config.stdin_tag_arg != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut tag_match: libc::c_int = 0 as libc::c_int;
        while i < argc {
            if strcmp(
                config.stdin_tag_name,
                *sds_args.offset(i as isize) as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                i += 1;
            } else {
                tag_match = 1 as libc::c_int;
                hi_sdsfree(*sds_args.offset(i as isize));
                let ref mut fresh15 = *sds_args.offset(i as isize);
                *fresh15 = readArgFromStdin();
                break;
            }
        }
        if tag_match == 0 {
            hi_sdsfreesplitres(sds_args, argc);
            fprintf(
                stderr,
                b"Using -X option but stdin tag not match.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    retval = issueCommand(argc, sds_args);
    hi_sdsfreesplitres(sds_args, argc);
    return if retval == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int };
}
unsafe extern "C" fn evalMode(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut script: hisds = 0 as hisds;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut nread: size_t = 0;
    let mut argv2: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut got_comma: libc::c_int = 0;
    let mut keys: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    loop {
        if config.eval_ldb != 0 {
            printf(
                b"Lua debugging session started, please use:\nquit    -- End the session.\nrestart -- Restart the script in debug mode again.\nhelp    -- Show Lua script debugging commands.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        hi_sdsfree(script);
        script = hi_sdsempty();
        got_comma = 0 as libc::c_int;
        keys = 0 as libc::c_int;
        fp = fopen(config.eval, b"r\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            fprintf(
                stderr,
                b"Can't open file '%s': %s\n\0" as *const u8 as *const libc::c_char,
                config.eval,
                strerror(*__errno_location()),
            );
            exit(1 as libc::c_int);
        }
        loop {
            nread = fread(
                buf.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                fp,
            );
            if !(nread != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            script = hi_sdscatlen(
                script,
                buf.as_mut_ptr() as *const libc::c_void,
                nread,
            );
        }
        fclose(fp);
        if config.eval_ldb != 0 {
            let mut reply: *mut redisReply = redisCommand(
                context,
                if config.eval_ldb_sync != 0 {
                    b"SCRIPT DEBUG sync\0" as *const u8 as *const libc::c_char
                } else {
                    b"SCRIPT DEBUG yes\0" as *const u8 as *const libc::c_char
                },
            ) as *mut redisReply;
            if !reply.is_null() {
                freeReplyObject(reply as *mut libc::c_void);
            }
        }
        argv2 = zmalloc(
            (core::mem::size_of::<hisds>() as libc::c_ulong)
                .wrapping_mul((argc + 3 as libc::c_int) as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        let ref mut fresh16 = *argv2.offset(0 as libc::c_int as isize);
        *fresh16 = hi_sdsnew(b"EVAL\0" as *const u8 as *const libc::c_char);
        let ref mut fresh17 = *argv2.offset(1 as libc::c_int as isize);
        *fresh17 = script;
        j = 0 as libc::c_int;
        while j < argc {
            if got_comma == 0
                && *(*argv.offset(j as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == ',' as i32
                && *(*argv.offset(j as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == 0 as libc::c_int
            {
                got_comma = 1 as libc::c_int;
            } else {
                let ref mut fresh18 = *argv2
                    .offset((j + 3 as libc::c_int - got_comma) as isize);
                *fresh18 = hi_sdsnew(*argv.offset(j as isize));
                if got_comma == 0 {
                    keys += 1;
                }
            }
            j += 1;
        }
        let ref mut fresh19 = *argv2.offset(2 as libc::c_int as isize);
        *fresh19 = hi_sdscatprintf(
            hi_sdsempty(),
            b"%d\0" as *const u8 as *const libc::c_char,
            keys,
        );
        let mut eval_ldb: libc::c_int = config.eval_ldb;
        retval = issueCommand(argc + 3 as libc::c_int - got_comma, argv2);
        if !(eval_ldb != 0) {
            break;
        }
        if config.eval_ldb == 0 {
            printf(
                b"Eval debugging session can't start:\n\0" as *const u8
                    as *const libc::c_char,
            );
            cliReadReply(0 as libc::c_int);
            break;
        } else {
            strncpy(
                (config.prompt).as_mut_ptr(),
                b"lua debugger> \0" as *const u8 as *const libc::c_char,
                core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            );
            repl();
            cliConnect((1 as libc::c_int) << 0 as libc::c_int);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return if retval == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int };
}
static mut cluster_manager: clusterManager = clusterManager {
    nodes: 0 as *const list as *mut list,
    errors: 0 as *const list as *mut list,
    unreachable_masters: 0,
};
#[no_mangle]
pub static mut clusterManagerUncoveredSlots: *mut dict = 0 as *const dict as *mut dict;
static mut clusterManagerDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: None,
            valDestructor: Some(
                dictSdsDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
static mut clusterManagerLinkDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                dictSdsDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            valDestructor: Some(
                dictListDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut clusterManagerCommands: [clusterManagerCommandDef; 13] = unsafe {
    [
        {
            let mut init = clusterManagerCommandDef {
                name: b"create\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandCreate
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: -(2 as libc::c_int),
                args: b"host1:port1 ... hostN:portN\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                options: b"replicas <arg>\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"check\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandCheck
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: -(1 as libc::c_int),
                args: b"<host:port> or <host> <port> - separated by either colon or space\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                options: b"search-multiple-owners\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"info\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandInfo
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: -(1 as libc::c_int),
                args: b"<host:port> or <host> <port> - separated by either colon or space\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                options: 0 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"fix\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandFix
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: -(1 as libc::c_int),
                args: b"<host:port> or <host> <port> - separated by either colon or space\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                options: b"search-multiple-owners,fix-with-unreachable-masters\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"reshard\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandReshard
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: -(1 as libc::c_int),
                args: b"<host:port> or <host> <port> - separated by either colon or space\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                options: b"from <arg>,to <arg>,slots <arg>,yes,timeout <arg>,pipeline <arg>,replace\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"rebalance\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandRebalance
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: -(1 as libc::c_int),
                args: b"<host:port> or <host> <port> - separated by either colon or space\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                options: b"weight <node1=w1...nodeN=wN>,use-empty-masters,timeout <arg>,simulate,pipeline <arg>,threshold <arg>,replace\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"add-node\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandAddNode
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: 2 as libc::c_int,
                args: b"new_host:new_port existing_host:existing_port\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                options: b"slave,master-id <arg>\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"del-node\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandDeleteNode
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: 2 as libc::c_int,
                args: b"host:port node_id\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                options: 0 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"call\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandCall
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: -(2 as libc::c_int),
                args: b"host:port command arg arg .. arg\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                options: b"only-masters,only-replicas\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"set-timeout\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandSetTimeout
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: 2 as libc::c_int,
                args: b"host:port milliseconds\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                options: 0 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"import\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandImport
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: 1 as libc::c_int,
                args: b"host:port\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                options: b"from <arg>,from-user <arg>,from-pass <arg>,from-askpass,copy,replace\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"backup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandBackup
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: 2 as libc::c_int,
                args: b"host:port backup_directory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                options: 0 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = clusterManagerCommandDef {
                name: b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                proc_0: Some(
                    clusterManagerCommandHelp
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                arity: 0 as libc::c_int,
                args: 0 as *const libc::c_char as *mut libc::c_char,
                options: 0 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut clusterManagerOptions: [clusterManagerOptionDef; 1] = [
    {
        let mut init = clusterManagerOptionDef {
            name: b"--cluster-yes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"Automatic yes to cluster commands prompts\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn createClusterManagerCommand(
    mut cmdname: *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cmd: *mut clusterManagerCommand = &mut config.cluster_manager_command;
    (*cmd).name = cmdname;
    (*cmd).argc = argc;
    (*cmd).argv = if argc != 0 { argv } else { 0 as *mut *mut libc::c_char };
    if isColorTerm() != 0 {
        (*cmd).flags |= (1 as libc::c_int) << 8 as libc::c_int;
    }
    if config.stdin_lastarg != 0 {
        let mut new_argv: *mut *mut libc::c_char = zmalloc(
            (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(((*cmd).argc + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        memcpy(
            new_argv as *mut libc::c_void,
            (*cmd).argv as *const libc::c_void,
            (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*cmd).argc as libc::c_ulong),
        );
        (*cmd).stdin_arg = readArgFromStdin();
        let fresh20 = (*cmd).argc;
        (*cmd).argc = (*cmd).argc + 1;
        let ref mut fresh21 = *new_argv.offset(fresh20 as isize);
        *fresh21 = (*cmd).stdin_arg;
        (*cmd).argv = new_argv;
    } else if config.stdin_tag_arg != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut tag_match: libc::c_int = 0 as libc::c_int;
        (*cmd).stdin_arg = readArgFromStdin();
        while i < argc {
            if strcmp(*argv.offset(i as isize), config.stdin_tag_name)
                != 0 as libc::c_int
            {
                i += 1;
            } else {
                tag_match = 1 as libc::c_int;
                let ref mut fresh22 = *((*cmd).argv).offset(i as isize);
                *fresh22 = (*cmd).stdin_arg;
                break;
            }
        }
        if tag_match == 0 {
            hi_sdsfree((*cmd).stdin_arg);
            fprintf(
                stderr,
                b"Using -X option but stdin tag not match.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn validateClusterManagerCommand() -> Option::<
    clusterManagerCommandProc,
> {
    let mut i: libc::c_int = 0;
    let mut commands_count: libc::c_int = (core::mem::size_of::<
        [clusterManagerCommandDef; 13],
    >() as libc::c_ulong)
        .wrapping_div(
            core::mem::size_of::<clusterManagerCommandDef>() as libc::c_ulong,
        ) as libc::c_int;
    let mut proc_0: Option::<clusterManagerCommandProc> = None;
    let mut cmdname: *mut libc::c_char = config.cluster_manager_command.name;
    let mut argc: libc::c_int = config.cluster_manager_command.argc;
    i = 0 as libc::c_int;
    while i < commands_count {
        let mut cmddef: clusterManagerCommandDef = clusterManagerCommands[i as usize];
        if strcmp(cmddef.name, cmdname) == 0 {
            if cmddef.arity > 0 as libc::c_int && argc != cmddef.arity
                || cmddef.arity < 0 as libc::c_int
                    && argc < cmddef.arity * -(1 as libc::c_int)
            {
                fprintf(
                    stderr,
                    b"[ERR] Wrong number of arguments for specified --cluster sub command\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return None;
            }
            proc_0 = cmddef.proc_0;
        }
        i += 1;
    }
    if proc_0.is_none() {
        fprintf(
            stderr,
            b"Unknown --cluster subcommand\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return proc_0;
}
unsafe extern "C" fn parseClusterNodeAddress(
    mut addr: *mut libc::c_char,
    mut ip_ptr: *mut *mut libc::c_char,
    mut port_ptr: *mut libc::c_int,
    mut bus_port_ptr: *mut libc::c_int,
) -> libc::c_int {
    let mut c: *mut libc::c_char = strrchr(addr, '@' as i32);
    if !c.is_null() {
        *c = '\0' as i32 as libc::c_char;
        if !bus_port_ptr.is_null() {
            *bus_port_ptr = atoi(c.offset(1 as libc::c_int as isize));
        }
    }
    c = strrchr(addr, ':' as i32);
    if !c.is_null() {
        *c = '\0' as i32 as libc::c_char;
        *ip_ptr = addr;
        c = c.offset(1);
        *port_ptr = atoi(c);
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn getClusterHostFromCmdArgs(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut ip_ptr: *mut *mut libc::c_char,
    mut port_ptr: *mut libc::c_int,
) -> libc::c_int {
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc == 1 as libc::c_int {
        let mut addr: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
        if parseClusterNodeAddress(addr, &mut ip, &mut port, 0 as *mut libc::c_int) == 0
        {
            return 0 as libc::c_int;
        }
    } else {
        ip = *argv.offset(0 as libc::c_int as isize);
        port = atoi(*argv.offset(1 as libc::c_int as isize));
    }
    if ip.is_null() || port == 0 {
        return 0 as libc::c_int
    } else {
        *ip_ptr = ip;
        *port_ptr = port;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn freeClusterManagerNodeFlags(mut flags: *mut list) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(flags, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut flag: hisds = (*ln).value as hisds;
        hi_sdsfree(flag);
    }
    listRelease(flags);
}
unsafe extern "C" fn freeClusterManagerNode(mut node: *mut clusterManagerNode) {
    if !((*node).context).is_null() {
        redisFree((*node).context);
    }
    if !((*node).friends).is_null() {
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind((*node).friends, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut fn_0: *mut clusterManagerNode = (*ln).value
                as *mut clusterManagerNode;
            freeClusterManagerNode(fn_0);
        }
        listRelease((*node).friends);
        (*node).friends = 0 as *mut list;
    }
    if !((*node).name).is_null() {
        hi_sdsfree((*node).name);
    }
    if !((*node).replicate).is_null() {
        hi_sdsfree((*node).replicate);
    }
    if (*node).flags & (1 as libc::c_int) << 2 as libc::c_int != 0
        && !((*node).ip).is_null()
    {
        hi_sdsfree((*node).ip);
    }
    let mut i: libc::c_int = 0;
    if !((*node).migrating).is_null() {
        i = 0 as libc::c_int;
        while i < (*node).migrating_count {
            hi_sdsfree(*((*node).migrating).offset(i as isize));
            i += 1;
        }
        zfree((*node).migrating as *mut libc::c_void);
    }
    if !((*node).importing).is_null() {
        i = 0 as libc::c_int;
        while i < (*node).importing_count {
            hi_sdsfree(*((*node).importing).offset(i as isize));
            i += 1;
        }
        zfree((*node).importing as *mut libc::c_void);
    }
    if !((*node).flags_str).is_null() {
        freeClusterManagerNodeFlags((*node).flags_str);
        (*node).flags_str = 0 as *mut list;
    }
    zfree(node as *mut libc::c_void);
}
unsafe extern "C" fn freeClusterManager() {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    if !(cluster_manager.nodes).is_null() {
        listRewind(cluster_manager.nodes, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
            freeClusterManagerNode(n);
        }
        listRelease(cluster_manager.nodes);
        cluster_manager.nodes = 0 as *mut list;
    }
    if !(cluster_manager.errors).is_null() {
        listRewind(cluster_manager.errors, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut err: hisds = (*ln).value as hisds;
            hi_sdsfree(err);
        }
        listRelease(cluster_manager.errors);
        cluster_manager.errors = 0 as *mut list;
    }
    if !clusterManagerUncoveredSlots.is_null() {
        dictRelease(clusterManagerUncoveredSlots);
    }
}
unsafe extern "C" fn clusterManagerNewNode(
    mut ip: *mut libc::c_char,
    mut port: libc::c_int,
    mut bus_port: libc::c_int,
) -> *mut clusterManagerNode {
    let mut node: *mut clusterManagerNode = zmalloc(
        core::mem::size_of::<clusterManagerNode>() as libc::c_ulong,
    ) as *mut clusterManagerNode;
    (*node).context = 0 as *mut redisContext;
    (*node).name = 0 as hisds;
    (*node).ip = ip;
    (*node).port = port;
    (*node)
        .bus_port = if bus_port != 0 { bus_port } else { port + 10000 as libc::c_int };
    (*node).current_epoch = 0 as libc::c_int as uint64_t;
    (*node).ping_sent = 0 as libc::c_int as time_t;
    (*node).ping_recv = 0 as libc::c_int as time_t;
    (*node).flags = 0 as libc::c_int;
    (*node).flags_str = 0 as *mut list;
    (*node).replicate = 0 as hisds;
    (*node).dirty = 0 as libc::c_int;
    (*node).friends = 0 as *mut list;
    (*node).migrating = 0 as *mut hisds;
    (*node).importing = 0 as *mut hisds;
    (*node).migrating_count = 0 as libc::c_int;
    (*node).importing_count = 0 as libc::c_int;
    (*node).replicas_count = 0 as libc::c_int;
    (*node).weight = 1.0f32;
    (*node).balance = 0 as libc::c_int;
    clusterManagerNodeResetSlots(node);
    return node;
}
unsafe extern "C" fn clusterManagerGetNodeRDBFilename(
    mut node: *mut clusterManagerNode,
) -> hisds {
    if !(config.cluster_manager_command.backup_dir).is_null() {} else {
        __assert_fail(
            b"config.cluster_manager_command.backup_dir\0" as *const u8
                as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            3238 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"hisds clusterManagerGetNodeRDBFilename(clusterManagerNode *)\0"))
                .as_ptr(),
        );
    };
    let mut filename: hisds = hi_sdsnew(config.cluster_manager_command.backup_dir);
    if *filename
        .offset(
            (hi_sdslen(filename)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as isize,
        ) as libc::c_int != '/' as i32
    {
        filename = hi_sdscat(filename, b"/\0" as *const u8 as *const libc::c_char);
    }
    filename = hi_sdscatprintf(
        filename,
        b"redis-node-%s-%d-%s.rdb\0" as *const u8 as *const libc::c_char,
        (*node).ip,
        (*node).port,
        (*node).name,
    );
    return filename;
}
unsafe extern "C" fn clusterManagerCheckRedisReply(
    mut n: *mut clusterManagerNode,
    mut r: *mut redisReply,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut is_err: libc::c_int = 0 as libc::c_int;
    if r.is_null()
        || {
            is_err = ((*r).type_0 == 6 as libc::c_int) as libc::c_int;
            is_err != 0
        }
    {
        if is_err != 0 {
            if !err.is_null() {
                *err = zmalloc(
                    ((*r).len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                strcpy(*err, (*r).str_0);
            } else {
                clusterManagerLog(
                    3 as libc::c_int,
                    b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                        as *const libc::c_char,
                    (*n).ip,
                    (*n).port,
                    (*r).str_0,
                );
            }
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn clusterManagerStartTransaction(
    mut node: *mut clusterManagerNode,
) -> libc::c_int {
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"MULTI\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    let mut success: libc::c_int = clusterManagerCheckRedisReply(
        node,
        reply,
        0 as *mut *mut libc::c_char,
    );
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerExecTransaction(
    mut node: *mut clusterManagerNode,
    mut onerror: clusterManagerOnReplyError,
) -> libc::c_int {
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"EXEC\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    let mut success: libc::c_int = clusterManagerCheckRedisReply(
        node,
        reply,
        0 as *mut *mut libc::c_char,
    );
    if success != 0 {
        if (*reply).type_0 != 2 as libc::c_int {
            success = 0 as libc::c_int;
        } else {
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*reply).elements {
                let mut r: *mut redisReply = *((*reply).element).offset(i as isize);
                let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
                success = clusterManagerCheckRedisReply(node, r, &mut err);
                if success == 0 && onerror.is_some() {
                    success = onerror
                        .expect("non-null function pointer")(r, node, i as libc::c_int);
                }
                if !err.is_null() {
                    if success == 0 {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                                as *const libc::c_char,
                            (*node).ip,
                            (*node).port,
                            err,
                        );
                    }
                    zfree(err as *mut libc::c_void);
                }
                if success == 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerNodeConnect(
    mut node: *mut clusterManagerNode,
) -> libc::c_int {
    if !((*node).context).is_null() {
        redisFree((*node).context);
    }
    (*node).context = redisConnect((*node).ip, (*node).port);
    if (*(*node).context).err == 0 && config.tls != 0 {
        let mut err: *const libc::c_char = 0 as *const libc::c_char;
        if cliSecureConnection((*node).context, config.sslconfig, &mut err)
            == -(1 as libc::c_int) && !err.is_null()
        {
            fprintf(
                stderr,
                b"TLS Error: %s\n\0" as *const u8 as *const libc::c_char,
                err,
            );
            redisFree((*node).context);
            (*node).context = 0 as *mut redisContext;
            return 0 as libc::c_int;
        }
    }
    if (*(*node).context).err != 0 {
        fprintf(
            stderr,
            b"Could not connect to Redis at \0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            (*node).ip,
            (*node).port,
            ((*(*node).context).errstr).as_mut_ptr(),
        );
        redisFree((*node).context);
        (*node).context = 0 as *mut redisContext;
        return 0 as libc::c_int;
    }
    anetKeepAlive(0 as *mut libc::c_char, (*(*node).context).fd, 15 as libc::c_int);
    if !(config.conn_info.auth).is_null() {
        let mut reply: *mut redisReply = 0 as *mut redisReply;
        if (config.conn_info.user).is_null() {
            reply = redisCommand(
                (*node).context,
                b"AUTH %s\0" as *const u8 as *const libc::c_char,
                config.conn_info.auth,
            ) as *mut redisReply;
        } else {
            reply = redisCommand(
                (*node).context,
                b"AUTH %s %s\0" as *const u8 as *const libc::c_char,
                config.conn_info.user,
                config.conn_info.auth,
            ) as *mut redisReply;
        }
        let mut ok: libc::c_int = clusterManagerCheckRedisReply(
            node,
            reply,
            0 as *mut *mut libc::c_char,
        );
        if !reply.is_null() {
            freeReplyObject(reply as *mut libc::c_void);
        }
        if ok == 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn clusterManagerRemoveNodeFromList(
    mut nodelist: *mut list,
    mut node: *mut clusterManagerNode,
) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(nodelist, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        if !(node == (*ln).value as *mut clusterManagerNode) {
            continue;
        }
        listDelNode(nodelist, ln);
        break;
    };
}
unsafe extern "C" fn clusterManagerNodeByName(
    mut name: *const libc::c_char,
) -> *mut clusterManagerNode {
    if (cluster_manager.nodes).is_null() {
        return 0 as *mut clusterManagerNode;
    }
    let mut found: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut lcname: hisds = hi_sdsempty();
    lcname = hi_sdscpy(lcname, name);
    hi_sdstolower(lcname);
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if !(!((*n).name).is_null() && hi_sdscmp((*n).name, lcname) == 0) {
            continue;
        }
        found = n;
        break;
    }
    hi_sdsfree(lcname);
    return found;
}
unsafe extern "C" fn clusterManagerNodeByAbbreviatedName(
    mut name: *const libc::c_char,
) -> *mut clusterManagerNode {
    if (cluster_manager.nodes).is_null() {
        return 0 as *mut clusterManagerNode;
    }
    let mut found: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut lcname: hisds = hi_sdsempty();
    lcname = hi_sdscpy(lcname, name);
    hi_sdstolower(lcname);
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if !(!((*n).name).is_null()
            && strstr((*n).name as *const libc::c_char, lcname as *const libc::c_char)
                == (*n).name)
        {
            continue;
        }
        found = n;
        break;
    }
    hi_sdsfree(lcname);
    return found;
}
unsafe extern "C" fn clusterManagerNodeResetSlots(mut node: *mut clusterManagerNode) {
    memset(
        ((*node).slots).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[uint8_t; 16384]>() as libc::c_ulong,
    );
    (*node).slots_count = 0 as libc::c_int;
}
unsafe extern "C" fn clusterManagerGetNodeRedisInfo(
    mut node: *mut clusterManagerNode,
    mut err: *mut *mut libc::c_char,
) -> *mut redisReply {
    let mut info: *mut redisReply = redisCommand(
        (*node).context,
        b"INFO\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    if !err.is_null() {
        *err = 0 as *mut libc::c_char;
    }
    if info.is_null() {
        return 0 as *mut redisReply;
    }
    if (*info).type_0 == 6 as libc::c_int {
        if !err.is_null() {
            *err = zmalloc(
                ((*info).len)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            strcpy(*err, (*info).str_0);
        }
        freeReplyObject(info as *mut libc::c_void);
        return 0 as *mut redisReply;
    }
    return info;
}
unsafe extern "C" fn clusterManagerNodeIsCluster(
    mut node: *mut clusterManagerNode,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut info: *mut redisReply = clusterManagerGetNodeRedisInfo(node, err);
    if info.is_null() {
        return 0 as libc::c_int;
    }
    let mut is_cluster: libc::c_int = getLongInfoField(
        (*info).str_0,
        b"cluster_enabled\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    freeReplyObject(info as *mut libc::c_void);
    return is_cluster;
}
unsafe extern "C" fn clusterManagerNodeIsEmpty(
    mut node: *mut clusterManagerNode,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut known_nodes: libc::c_long = 0;
    let mut info: *mut redisReply = clusterManagerGetNodeRedisInfo(node, err);
    let mut is_empty: libc::c_int = 1 as libc::c_int;
    if info.is_null() {
        return 0 as libc::c_int;
    }
    if !(strstr((*info).str_0, b"db0:\0" as *const u8 as *const libc::c_char)).is_null()
    {
        is_empty = 0 as libc::c_int;
    } else {
        freeReplyObject(info as *mut libc::c_void);
        info = redisCommand(
            (*node).context,
            b"CLUSTER INFO\0" as *const u8 as *const libc::c_char,
        ) as *mut redisReply;
        if !err.is_null() {
            *err = 0 as *mut libc::c_char;
        }
        if clusterManagerCheckRedisReply(node, info, err) == 0 {
            is_empty = 0 as libc::c_int;
        } else {
            known_nodes = getLongInfoField(
                (*info).str_0,
                b"cluster_known_nodes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            is_empty = (known_nodes == 1 as libc::c_int as libc::c_long) as libc::c_int;
        }
    }
    freeReplyObject(info as *mut libc::c_void);
    return is_empty;
}
unsafe extern "C" fn clusterManagerGetAntiAffinityScore(
    mut ipnodes: *mut clusterManagerNodeArray,
    mut ip_count: libc::c_int,
    mut offending: *mut *mut *mut clusterManagerNode,
    mut offending_len: *mut libc::c_int,
) -> libc::c_int {
    let mut score: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut node_len: libc::c_int = (*cluster_manager.nodes).len as libc::c_int;
    let mut offending_p: *mut *mut clusterManagerNode = 0
        as *mut *mut clusterManagerNode;
    if !offending.is_null() {
        *offending = zcalloc(
            (node_len as libc::c_ulong)
                .wrapping_mul(
                    core::mem::size_of::<*mut clusterManagerNode>() as libc::c_ulong,
                ),
        ) as *mut *mut clusterManagerNode;
        offending_p = *offending;
    }
    i = 0 as libc::c_int;
    while i < ip_count {
        let mut node_array: *mut clusterManagerNodeArray = &mut *ipnodes
            .offset(i as isize) as *mut clusterManagerNodeArray;
        let mut related: *mut dict = dictCreate(&mut clusterManagerDictType);
        let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
        j = 0 as libc::c_int;
        while j < (*node_array).len {
            let mut node: *mut clusterManagerNode = *((*node_array).nodes)
                .offset(j as isize);
            if !node.is_null() {
                if ip.is_null() {
                    ip = (*node).ip;
                }
                let mut types: hisds = 0 as *mut libc::c_char;
                let mut key: hisds = if ((*node).replicate).is_null() {
                    (*node).name
                } else {
                    (*node).replicate
                };
                if !key.is_null() {} else {
                    __assert_fail(
                        b"key != NULL\0" as *const u8 as *const libc::c_char,
                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                        3512 as libc::c_int as libc::c_uint,
                        (*core::mem::transmute::<
                            &[u8; 102],
                            &[libc::c_char; 102],
                        >(
                            b"int clusterManagerGetAntiAffinityScore(clusterManagerNodeArray *, int, clusterManagerNode ***, int *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                let mut entry: *mut dictEntry = dictFind(
                    related,
                    key as *const libc::c_void,
                );
                if !entry.is_null() {
                    types = hi_sdsdup((*entry).v.val as hisds);
                } else {
                    types = hi_sdsempty();
                }
                if !((*node).replicate).is_null() {
                    types = hi_sdscat(types, b"s\0" as *const u8 as *const libc::c_char);
                } else {
                    let mut s: hisds = hi_sdscatsds(
                        hi_sdsnew(b"m\0" as *const u8 as *const libc::c_char),
                        types,
                    );
                    hi_sdsfree(types);
                    types = s;
                }
                dictReplace(
                    related,
                    key as *mut libc::c_void,
                    types as *mut libc::c_void,
                );
            }
            j += 1;
        }
        let mut iter: *mut dictIterator = dictGetIterator(related);
        let mut entry_0: *mut dictEntry = 0 as *mut dictEntry;
        loop {
            entry_0 = dictNext(iter);
            if entry_0.is_null() {
                break;
            }
            let mut types_0: hisds = (*entry_0).v.val as hisds;
            let mut name: hisds = (*entry_0).key as hisds;
            let mut typeslen: libc::c_int = hi_sdslen(types_0) as libc::c_int;
            if typeslen < 2 as libc::c_int {
                continue;
            }
            if *types_0.offset(0 as libc::c_int as isize) as libc::c_int == 'm' as i32 {
                score += 10000 as libc::c_int * (typeslen - 1 as libc::c_int);
            } else {
                score += 1 as libc::c_int * typeslen;
            }
            if offending.is_null() {
                continue;
            }
            let mut li: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            let mut ln: *mut listNode = 0 as *mut listNode;
            listRewind(cluster_manager.nodes, &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut n: *mut clusterManagerNode = (*ln).value
                    as *mut clusterManagerNode;
                if ((*n).replicate).is_null() {
                    continue;
                }
                if !(strcmp(
                    (*n).replicate as *const libc::c_char,
                    name as *const libc::c_char,
                ) == 0 && strcmp((*n).ip, ip) == 0)
                {
                    continue;
                }
                let fresh23 = offending_p;
                offending_p = offending_p.offset(1);
                *fresh23 = n;
                if !offending_len.is_null() {
                    *offending_len += 1;
                }
                break;
            }
        }
        dictReleaseIterator(iter);
        dictRelease(related);
        i += 1;
    }
    return score;
}
unsafe extern "C" fn clusterManagerOptimizeAntiAffinity(
    mut ipnodes: *mut clusterManagerNodeArray,
    mut ip_count: libc::c_int,
) {
    let mut node_len: libc::c_int = 0;
    let mut maxiter: libc::c_int = 0;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut perfect: libc::c_int = 0;
    let mut log_level: libc::c_int = 0;
    let mut offenders: *mut *mut clusterManagerNode = 0 as *mut *mut clusterManagerNode;
    let mut score: libc::c_int = clusterManagerGetAntiAffinityScore(
        ipnodes,
        ip_count,
        0 as *mut *mut *mut clusterManagerNode,
        0 as *mut libc::c_int,
    );
    if !(score == 0 as libc::c_int) {
        clusterManagerLog(
            1 as libc::c_int,
            b">>> Trying to optimize slaves allocation for anti-affinity\n\0"
                as *const u8 as *const libc::c_char,
        );
        node_len = (*cluster_manager.nodes).len as libc::c_int;
        maxiter = 500 as libc::c_int * node_len;
        srand(time(0 as *mut time_t) as libc::c_uint);
        while maxiter > 0 as libc::c_int {
            let mut offending_len: libc::c_int = 0 as libc::c_int;
            if !offenders.is_null() {
                zfree(offenders as *mut libc::c_void);
                offenders = 0 as *mut *mut clusterManagerNode;
            }
            score = clusterManagerGetAntiAffinityScore(
                ipnodes,
                ip_count,
                &mut offenders,
                &mut offending_len,
            );
            if score == 0 as libc::c_int || offending_len == 0 as libc::c_int {
                break;
            }
            let mut rand_idx: libc::c_int = rand() % offending_len;
            let mut first: *mut clusterManagerNode = *offenders
                .offset(rand_idx as isize);
            let mut second: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
            let mut other_replicas: *mut *mut clusterManagerNode = zcalloc(
                ((node_len - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        core::mem::size_of::<*mut clusterManagerNode>()
                            as libc::c_ulong,
                    ),
            ) as *mut *mut clusterManagerNode;
            let mut other_replicas_count: libc::c_int = 0 as libc::c_int;
            let mut li: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            let mut ln: *mut listNode = 0 as *mut listNode;
            listRewind(cluster_manager.nodes, &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut n: *mut clusterManagerNode = (*ln).value
                    as *mut clusterManagerNode;
                if n != first && !((*n).replicate).is_null() {
                    let fresh24 = other_replicas_count;
                    other_replicas_count = other_replicas_count + 1;
                    let ref mut fresh25 = *other_replicas.offset(fresh24 as isize);
                    *fresh25 = n;
                }
            }
            if other_replicas_count == 0 as libc::c_int {
                zfree(other_replicas as *mut libc::c_void);
                break;
            } else {
                rand_idx = rand() % other_replicas_count;
                second = *other_replicas.offset(rand_idx as isize);
                let mut first_master: *mut libc::c_char = (*first).replicate;
                let mut second_master: *mut libc::c_char = (*second).replicate;
                (*first).replicate = second_master;
                (*first).dirty = 1 as libc::c_int;
                (*second).replicate = first_master;
                (*second).dirty = 1 as libc::c_int;
                let mut new_score: libc::c_int = clusterManagerGetAntiAffinityScore(
                    ipnodes,
                    ip_count,
                    0 as *mut *mut *mut clusterManagerNode,
                    0 as *mut libc::c_int,
                );
                if new_score > score {
                    (*first).replicate = first_master;
                    (*second).replicate = second_master;
                }
                zfree(other_replicas as *mut libc::c_void);
                maxiter -= 1;
            }
        }
        score = clusterManagerGetAntiAffinityScore(
            ipnodes,
            ip_count,
            0 as *mut *mut *mut clusterManagerNode,
            0 as *mut libc::c_int,
        );
        msg = 0 as *mut libc::c_char;
        perfect = (score == 0 as libc::c_int) as libc::c_int;
        log_level = if perfect != 0 { 4 as libc::c_int } else { 2 as libc::c_int };
        if perfect != 0 {
            msg = b"[OK] Perfect anti-affinity obtained!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        } else if score >= 10000 as libc::c_int {
            msg = b"[WARNING] Some slaves are in the same host as their master\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            msg = b"[WARNING] Some slaves of the same master are in the same host\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        clusterManagerLog(log_level, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    }
    zfree(offenders as *mut libc::c_void);
}
unsafe extern "C" fn clusterManagerNodeFlagString(
    mut node: *mut clusterManagerNode,
) -> hisds {
    let mut flags: hisds = hi_sdsempty();
    if ((*node).flags_str).is_null() {
        return flags;
    }
    let mut empty: libc::c_int = 1 as libc::c_int;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind((*node).flags_str, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut flag: hisds = (*ln).value as hisds;
        if strcmp(
            flag as *const libc::c_char,
            b"myself\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            continue;
        }
        if empty == 0 {
            flags = hi_sdscat(flags, b",\0" as *const u8 as *const libc::c_char);
        }
        flags = hi_sdscatfmt(flags, b"%S\0" as *const u8 as *const libc::c_char, flag);
        empty = 0 as libc::c_int;
    }
    return flags;
}
unsafe extern "C" fn clusterManagerNodeSlotsString(
    mut node: *mut clusterManagerNode,
) -> hisds {
    let mut slots: hisds = hi_sdsempty();
    let mut first_range_idx: libc::c_int = -(1 as libc::c_int);
    let mut last_slot_idx: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16384 as libc::c_int {
        let mut has_slot: libc::c_int = (*node).slots[i as usize] as libc::c_int;
        if has_slot != 0 {
            if first_range_idx == -(1 as libc::c_int) {
                if hi_sdslen(slots) != 0 {
                    slots = hi_sdscat(slots, b",\0" as *const u8 as *const libc::c_char);
                }
                first_range_idx = i;
                slots = hi_sdscatfmt(
                    slots,
                    b"[%u\0" as *const u8 as *const libc::c_char,
                    i,
                );
            }
            last_slot_idx = i;
        } else {
            if last_slot_idx >= 0 as libc::c_int {
                if first_range_idx == last_slot_idx {
                    slots = hi_sdscat(slots, b"]\0" as *const u8 as *const libc::c_char);
                } else {
                    slots = hi_sdscatfmt(
                        slots,
                        b"-%u]\0" as *const u8 as *const libc::c_char,
                        last_slot_idx,
                    );
                }
            }
            last_slot_idx = -(1 as libc::c_int);
            first_range_idx = -(1 as libc::c_int);
        }
        i += 1;
    }
    if last_slot_idx >= 0 as libc::c_int {
        if first_range_idx == last_slot_idx {
            slots = hi_sdscat(slots, b"]\0" as *const u8 as *const libc::c_char);
        } else {
            slots = hi_sdscatfmt(
                slots,
                b"-%u]\0" as *const u8 as *const libc::c_char,
                last_slot_idx,
            );
        }
    }
    return slots;
}
unsafe extern "C" fn clusterManagerNodeGetJSON(
    mut node: *mut clusterManagerNode,
    mut error_count: libc::c_ulong,
) -> hisds {
    let mut json: hisds = hi_sdsempty();
    let mut replicate: hisds = hi_sdsempty();
    if !((*node).replicate).is_null() {
        replicate = hi_sdscatprintf(
            replicate,
            b"\"%s\"\0" as *const u8 as *const libc::c_char,
            (*node).replicate,
        );
    } else {
        replicate = hi_sdscat(replicate, b"null\0" as *const u8 as *const libc::c_char);
    }
    let mut slots: hisds = clusterManagerNodeSlotsString(node);
    let mut flags: hisds = clusterManagerNodeFlagString(node);
    let mut p: *mut libc::c_char = slots;
    loop {
        p = strchr(p, '-' as i32);
        if p.is_null() {
            break;
        }
        let fresh26 = p;
        p = p.offset(1);
        *fresh26 = ',' as i32 as libc::c_char;
    }
    json = hi_sdscatprintf(
        json,
        b"  {\n    \"name\": \"%s\",\n    \"host\": \"%s\",\n    \"port\": %d,\n    \"replicate\": %s,\n    \"slots\": [%s],\n    \"slots_count\": %d,\n    \"flags\": \"%s\",\n    \"current_epoch\": %llu\0"
            as *const u8 as *const libc::c_char,
        (*node).name,
        (*node).ip,
        (*node).port,
        replicate,
        slots,
        (*node).slots_count,
        flags,
        (*node).current_epoch as libc::c_ulonglong,
    );
    if error_count > 0 as libc::c_int as libc::c_ulong {
        json = hi_sdscatprintf(
            json,
            b",\n    \"cluster_errors\": %lu\0" as *const u8 as *const libc::c_char,
            error_count,
        );
    }
    if (*node).migrating_count > 0 as libc::c_int && !((*node).migrating).is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut migrating: hisds = hi_sdsempty();
        while i < (*node).migrating_count {
            let mut slot: hisds = *((*node).migrating).offset(i as isize);
            let mut dest: hisds = *((*node).migrating)
                .offset((i + 1 as libc::c_int) as isize);
            if !slot.is_null() && !dest.is_null() {
                if hi_sdslen(migrating) > 0 as libc::c_int as libc::c_ulong {
                    migrating = hi_sdscat(
                        migrating,
                        b",\0" as *const u8 as *const libc::c_char,
                    );
                }
                migrating = hi_sdscatfmt(
                    migrating,
                    b"\"%S\": \"%S\"\0" as *const u8 as *const libc::c_char,
                    slot,
                    dest,
                );
            }
            i += 2 as libc::c_int;
        }
        if hi_sdslen(migrating) > 0 as libc::c_int as libc::c_ulong {
            json = hi_sdscatfmt(
                json,
                b",\n    \"migrating\": {%S}\0" as *const u8 as *const libc::c_char,
                migrating,
            );
        }
        hi_sdsfree(migrating);
    }
    if (*node).importing_count > 0 as libc::c_int && !((*node).importing).is_null() {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        let mut importing: hisds = hi_sdsempty();
        while i_0 < (*node).importing_count {
            let mut slot_0: hisds = *((*node).importing).offset(i_0 as isize);
            let mut from: hisds = *((*node).importing)
                .offset((i_0 + 1 as libc::c_int) as isize);
            if !slot_0.is_null() && !from.is_null() {
                if hi_sdslen(importing) > 0 as libc::c_int as libc::c_ulong {
                    importing = hi_sdscat(
                        importing,
                        b",\0" as *const u8 as *const libc::c_char,
                    );
                }
                importing = hi_sdscatfmt(
                    importing,
                    b"\"%S\": \"%S\"\0" as *const u8 as *const libc::c_char,
                    slot_0,
                    from,
                );
            }
            i_0 += 2 as libc::c_int;
        }
        if hi_sdslen(importing) > 0 as libc::c_int as libc::c_ulong {
            json = hi_sdscatfmt(
                json,
                b",\n    \"importing\": {%S}\0" as *const u8 as *const libc::c_char,
                importing,
            );
        }
        hi_sdsfree(importing);
    }
    json = hi_sdscat(json, b"\n  }\0" as *const u8 as *const libc::c_char);
    hi_sdsfree(replicate);
    hi_sdsfree(slots);
    hi_sdsfree(flags);
    return json;
}
unsafe extern "C" fn clusterManagerKeyHashSlot(
    mut key: *mut libc::c_char,
    mut keylen: libc::c_int,
) -> libc::c_uint {
    let mut s: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s < keylen {
        if *key.offset(s as isize) as libc::c_int == '{' as i32 {
            break;
        }
        s += 1;
    }
    if s == keylen {
        return (crc16(key, keylen) as libc::c_int & 0x3fff as libc::c_int)
            as libc::c_uint;
    }
    e = s + 1 as libc::c_int;
    while e < keylen {
        if *key.offset(e as isize) as libc::c_int == '}' as i32 {
            break;
        }
        e += 1;
    }
    if e == keylen || e == s + 1 as libc::c_int {
        return (crc16(key, keylen) as libc::c_int & 0x3fff as libc::c_int)
            as libc::c_uint;
    }
    return (crc16(
        key.offset(s as isize).offset(1 as libc::c_int as isize),
        e - s - 1 as libc::c_int,
    ) as libc::c_int & 0x3fff as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn clusterManagerNodeInfo(
    mut node: *mut clusterManagerNode,
    mut indent: libc::c_int,
) -> hisds {
    let mut info: hisds = hi_sdsempty();
    let mut spaces: hisds = hi_sdsempty();
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < indent {
        spaces = hi_sdscat(spaces, b" \0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    if indent != 0 {
        info = hi_sdscat(info, spaces as *const libc::c_char);
    }
    let mut is_master: libc::c_int = ((*node).flags
        & (1 as libc::c_int) << 1 as libc::c_int == 0) as libc::c_int;
    let mut role: *mut libc::c_char = (if is_master != 0 {
        b"M\0" as *const u8 as *const libc::c_char
    } else {
        b"S\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut slots: hisds = 0 as hisds;
    if (*node).dirty != 0 && !((*node).replicate).is_null() {
        info = hi_sdscatfmt(
            info,
            b"S: %S %s:%u\0" as *const u8 as *const libc::c_char,
            (*node).name,
            (*node).ip,
            (*node).port,
        );
    } else {
        slots = clusterManagerNodeSlotsString(node);
        let mut flags: hisds = clusterManagerNodeFlagString(node);
        info = hi_sdscatfmt(
            info,
            b"%s: %S %s:%u\n%s   slots:%S (%u slots) %S\0" as *const u8
                as *const libc::c_char,
            role,
            (*node).name,
            (*node).ip,
            (*node).port,
            spaces,
            slots,
            (*node).slots_count,
            flags,
        );
        hi_sdsfree(slots);
        hi_sdsfree(flags);
    }
    if !((*node).replicate).is_null() {
        info = hi_sdscatfmt(
            info,
            b"\n%s   replicates %S\0" as *const u8 as *const libc::c_char,
            spaces,
            (*node).replicate,
        );
    } else if (*node).replicas_count != 0 {
        info = hi_sdscatfmt(
            info,
            b"\n%s   %U additional replica(s)\0" as *const u8 as *const libc::c_char,
            spaces,
            (*node).replicas_count,
        );
    }
    hi_sdsfree(spaces);
    return info;
}
unsafe extern "C" fn clusterManagerShowNodes() {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut node: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        let mut info: hisds = clusterManagerNodeInfo(node, 0 as libc::c_int);
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, info);
        hi_sdsfree(info);
    };
}
unsafe extern "C" fn clusterManagerShowClusterInfo() {
    let mut masters: libc::c_int = 0 as libc::c_int;
    let mut keys: libc::c_int = 0 as libc::c_int;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut node: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if !((*node).flags & (1 as libc::c_int) << 1 as libc::c_int == 0) {
            continue;
        }
        if ((*node).name).is_null() {
            continue;
        }
        let mut replicas: libc::c_int = 0 as libc::c_int;
        let mut dbsize: libc::c_int = -(1 as libc::c_int);
        let mut name: [libc::c_char; 9] = [0; 9];
        memcpy(
            name.as_mut_ptr() as *mut libc::c_void,
            (*node).name as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        name[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        let mut ri: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut rn: *mut listNode = 0 as *mut listNode;
        listRewind(cluster_manager.nodes, &mut ri);
        loop {
            rn = listNext(&mut ri);
            if rn.is_null() {
                break;
            }
            let mut n: *mut clusterManagerNode = (*rn).value as *mut clusterManagerNode;
            if n == node || (*n).flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                continue;
            }
            if !((*n).replicate).is_null()
                && strcmp(
                    (*n).replicate as *const libc::c_char,
                    (*node).name as *const libc::c_char,
                ) == 0
            {
                replicas += 1;
            }
        }
        let mut reply: *mut redisReply = redisCommand(
            (*node).context,
            b"DBSIZE\0" as *const u8 as *const libc::c_char,
        ) as *mut redisReply;
        if !reply.is_null() && (*reply).type_0 == 3 as libc::c_int {
            dbsize = (*reply).integer as libc::c_int;
        }
        if dbsize < 0 as libc::c_int {
            let mut err: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            if !reply.is_null() && (*reply).type_0 == 6 as libc::c_int {
                err = (*reply).str_0;
            }
            clusterManagerLog(
                3 as libc::c_int,
                b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                    as *const libc::c_char,
                (*node).ip,
                (*node).port,
                err,
            );
            if !reply.is_null() {
                freeReplyObject(reply as *mut libc::c_void);
            }
            return;
        }
        if !reply.is_null() {
            freeReplyObject(reply as *mut libc::c_void);
        }
        printf(
            b"%s:%d (%s...) -> %d keys | %d slots | %d slaves.\n\0" as *const u8
                as *const libc::c_char,
            (*node).ip,
            (*node).port,
            name.as_mut_ptr(),
            dbsize,
            (*node).slots_count,
            replicas,
        );
        masters += 1;
        keys += dbsize;
    }
    clusterManagerLog(
        4 as libc::c_int,
        b"[OK] %d keys in %d masters.\n\0" as *const u8 as *const libc::c_char,
        keys,
        masters,
    );
    let mut keys_per_slot: libc::c_float = keys as libc::c_float
        / 16384 as libc::c_int as libc::c_float;
    printf(
        b"%.2f keys per slot on average.\n\0" as *const u8 as *const libc::c_char,
        keys_per_slot as libc::c_double,
    );
}
unsafe extern "C" fn clusterManagerAddSlots(
    mut node: *mut clusterManagerNode,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut _reply: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut argc: libc::c_int = (*node).slots_count + 2 as libc::c_int;
    let mut argv: *mut hisds = zmalloc(
        (argc as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<hisds>() as libc::c_ulong),
    ) as *mut hisds;
    let mut argvlen: *mut size_t = zmalloc(
        (argc as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    let ref mut fresh27 = *argv.offset(0 as libc::c_int as isize);
    *fresh27 = b"CLUSTER\0" as *const u8 as *const libc::c_char as hisds;
    let ref mut fresh28 = *argv.offset(1 as libc::c_int as isize);
    *fresh28 = b"ADDSLOTS\0" as *const u8 as *const libc::c_char as hisds;
    *argvlen.offset(0 as libc::c_int as isize) = 7 as libc::c_int as size_t;
    *argvlen.offset(1 as libc::c_int as isize) = 8 as libc::c_int as size_t;
    *err = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut argv_idx: libc::c_int = 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16384 as libc::c_int {
        if argv_idx >= argc {
            break;
        }
        if (*node).slots[i as usize] != 0 {
            let ref mut fresh29 = *argv.offset(argv_idx as isize);
            *fresh29 = hi_sdsfromlonglong(i as libc::c_longlong);
            *argvlen
                .offset(argv_idx as isize) = hi_sdslen(*argv.offset(argv_idx as isize));
            argv_idx += 1;
        }
        i += 1;
    }
    if argv_idx == 2 as libc::c_int {
        success = 0 as libc::c_int;
    } else {
        redisAppendCommandArgv(
            (*node).context,
            argc,
            argv as *mut *const libc::c_char,
            argvlen,
        );
        if redisGetReply((*node).context, &mut _reply) != 0 as libc::c_int {
            success = 0 as libc::c_int;
        } else {
            reply = _reply as *mut redisReply;
            success = clusterManagerCheckRedisReply(node, reply, err);
        }
    }
    zfree(argvlen as *mut libc::c_void);
    if !argv.is_null() {
        i = 2 as libc::c_int;
        while i < argc {
            hi_sdsfree(*argv.offset(i as isize));
            i += 1;
        }
        zfree(argv as *mut libc::c_void);
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerGetSlotOwner(
    mut n: *mut clusterManagerNode,
    mut slot: libc::c_int,
    mut err: *mut *mut libc::c_char,
) -> *mut clusterManagerNode {
    if slot >= 0 as libc::c_int && slot < 16384 as libc::c_int {} else {
        __assert_fail(
            b"slot >= 0 && slot < CLUSTER_MANAGER_SLOTS\0" as *const u8
                as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            3936 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"clusterManagerNode *clusterManagerGetSlotOwner(clusterManagerNode *, int, char **)\0",
            ))
                .as_ptr(),
        );
    };
    let mut owner: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut reply: *mut redisReply = redisCommand(
        (*n).context,
        b"CLUSTER SLOTS\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    if clusterManagerCheckRedisReply(n, reply, err) != 0 {
        if (*reply).type_0 == 2 as libc::c_int {} else {
            __assert_fail(
                b"reply->type == REDIS_REPLY_ARRAY\0" as *const u8
                    as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                3940 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"clusterManagerNode *clusterManagerGetSlotOwner(clusterManagerNode *, int, char **)\0",
                ))
                    .as_ptr(),
            );
        };
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*reply).elements {
            let mut r: *mut redisReply = *((*reply).element).offset(i as isize);
            if (*r).type_0 == 2 as libc::c_int
                && (*r).elements >= 3 as libc::c_int as libc::c_ulong
            {} else {
                __assert_fail(
                    b"r->type == REDIS_REPLY_ARRAY && r->elements >= 3\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    3944 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"clusterManagerNode *clusterManagerGetSlotOwner(clusterManagerNode *, int, char **)\0",
                    ))
                        .as_ptr(),
                );
            };
            let mut from: libc::c_int = 0;
            let mut to: libc::c_int = 0;
            from = (**((*r).element).offset(0 as libc::c_int as isize)).integer
                as libc::c_int;
            to = (**((*r).element).offset(1 as libc::c_int as isize)).integer
                as libc::c_int;
            if !(slot < from || slot > to) {
                let mut nr: *mut redisReply = *((*r).element)
                    .offset(2 as libc::c_int as isize);
                if (*nr).type_0 == 2 as libc::c_int
                    && (*nr).elements >= 2 as libc::c_int as libc::c_ulong
                {} else {
                    __assert_fail(
                        b"nr->type == REDIS_REPLY_ARRAY && nr->elements >= 2\0"
                            as *const u8 as *const libc::c_char,
                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                        3950 as libc::c_int as libc::c_uint,
                        (*core::mem::transmute::<
                            &[u8; 83],
                            &[libc::c_char; 83],
                        >(
                            b"clusterManagerNode *clusterManagerGetSlotOwner(clusterManagerNode *, int, char **)\0",
                        ))
                            .as_ptr(),
                    );
                };
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                if (*nr).elements >= 3 as libc::c_int as libc::c_ulong {
                    name = (**((*nr).element).offset(2 as libc::c_int as isize)).str_0;
                }
                if !name.is_null() {
                    owner = clusterManagerNodeByName(name);
                } else {
                    let mut ip: *mut libc::c_char = (**((*nr).element)
                        .offset(0 as libc::c_int as isize))
                        .str_0;
                    if !ip.is_null() {} else {
                        __assert_fail(
                            b"ip != NULL\0" as *const u8 as *const libc::c_char,
                            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                            3958 as libc::c_int as libc::c_uint,
                            (*core::mem::transmute::<
                                &[u8; 83],
                                &[libc::c_char; 83],
                            >(
                                b"clusterManagerNode *clusterManagerGetSlotOwner(clusterManagerNode *, int, char **)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    let mut port: libc::c_int = (**((*nr).element)
                        .offset(1 as libc::c_int as isize))
                        .integer as libc::c_int;
                    let mut li: listIter = listIter {
                        next: 0 as *mut listNode,
                        direction: 0,
                    };
                    let mut ln: *mut listNode = 0 as *mut listNode;
                    listRewind(cluster_manager.nodes, &mut li);
                    loop {
                        ln = listNext(&mut li);
                        if ln.is_null() {
                            break;
                        }
                        let mut nd: *mut clusterManagerNode = (*ln).value
                            as *mut clusterManagerNode;
                        if !(strcmp((*nd).ip, ip) == 0 as libc::c_int
                            && port == (*nd).port)
                        {
                            continue;
                        }
                        owner = nd;
                        break;
                    }
                }
                if !owner.is_null() {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return owner;
}
unsafe extern "C" fn clusterManagerSetSlot(
    mut node1: *mut clusterManagerNode,
    mut node2: *mut clusterManagerNode,
    mut slot: libc::c_int,
    mut status: *const libc::c_char,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut reply: *mut redisReply = redisCommand(
        (*node1).context,
        b"CLUSTER SETSLOT %d %s %s\0" as *const u8 as *const libc::c_char,
        slot,
        status,
        (*node2).name,
    ) as *mut redisReply;
    if !err.is_null() {
        *err = 0 as *mut libc::c_char;
    }
    if reply.is_null() {
        if !err.is_null() {
            *err = zstrdup(
                b"CLUSTER SETSLOT failed to run\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    let mut success: libc::c_int = 1 as libc::c_int;
    if (*reply).type_0 == 6 as libc::c_int {
        success = 0 as libc::c_int;
        if !err.is_null() {
            *err = zmalloc(
                ((*reply).len)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            strcpy(*err, (*reply).str_0);
        } else {
            clusterManagerLog(
                3 as libc::c_int,
                b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                    as *const libc::c_char,
                (*node1).ip,
                (*node1).port,
                (*reply).str_0,
            );
        }
    }
    freeReplyObject(reply as *mut libc::c_void);
    return success;
}
unsafe extern "C" fn clusterManagerClearSlotStatus(
    mut node: *mut clusterManagerNode,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"CLUSTER SETSLOT %d %s\0" as *const u8 as *const libc::c_char,
        slot,
        b"STABLE\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    let mut success: libc::c_int = clusterManagerCheckRedisReply(
        node,
        reply,
        0 as *mut *mut libc::c_char,
    );
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerDelSlot(
    mut node: *mut clusterManagerNode,
    mut slot: libc::c_int,
    mut ignore_unassigned_err: libc::c_int,
) -> libc::c_int {
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"CLUSTER DELSLOTS %d\0" as *const u8 as *const libc::c_char,
        slot,
    ) as *mut redisReply;
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut success: libc::c_int = clusterManagerCheckRedisReply(node, reply, &mut err);
    if success == 0 && !reply.is_null() && (*reply).type_0 == 6 as libc::c_int
        && ignore_unassigned_err != 0
    {
        let mut get_owner_err: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut assigned_to: *mut clusterManagerNode = clusterManagerGetSlotOwner(
            node,
            slot,
            &mut get_owner_err,
        );
        if assigned_to.is_null() {
            if get_owner_err.is_null() {
                success = 1 as libc::c_int;
            } else {
                clusterManagerLog(
                    3 as libc::c_int,
                    b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                        as *const libc::c_char,
                    (*node).ip,
                    (*node).port,
                    get_owner_err,
                );
                zfree(get_owner_err as *mut libc::c_void);
            }
        }
    }
    if success == 0 && !err.is_null() {
        clusterManagerLog(
            3 as libc::c_int,
            b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                as *const libc::c_char,
            (*node).ip,
            (*node).port,
            err,
        );
        zfree(err as *mut libc::c_void);
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerAddSlot(
    mut node: *mut clusterManagerNode,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"CLUSTER ADDSLOTS %d\0" as *const u8 as *const libc::c_char,
        slot,
    ) as *mut redisReply;
    let mut success: libc::c_int = clusterManagerCheckRedisReply(
        node,
        reply,
        0 as *mut *mut libc::c_char,
    );
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerCountKeysInSlot(
    mut node: *mut clusterManagerNode,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"CLUSTER COUNTKEYSINSLOT %d\0" as *const u8 as *const libc::c_char,
        slot,
    ) as *mut redisReply;
    let mut count: libc::c_int = -(1 as libc::c_int);
    let mut success: libc::c_int = clusterManagerCheckRedisReply(
        node,
        reply,
        0 as *mut *mut libc::c_char,
    );
    if success != 0 && (*reply).type_0 == 3 as libc::c_int {
        count = (*reply).integer as libc::c_int;
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return count;
}
unsafe extern "C" fn clusterManagerBumpEpoch(
    mut node: *mut clusterManagerNode,
) -> libc::c_int {
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"CLUSTER BUMPEPOCH\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    let mut success: libc::c_int = clusterManagerCheckRedisReply(
        node,
        reply,
        0 as *mut *mut libc::c_char,
    );
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerOnSetOwnerErr(
    mut reply: *mut redisReply,
    mut n: *mut clusterManagerNode,
    mut bulk_idx: libc::c_int,
) -> libc::c_int {
    return (bulk_idx != 1 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn clusterManagerSetSlotOwner(
    mut owner: *mut clusterManagerNode,
    mut slot: libc::c_int,
    mut do_clear: libc::c_int,
) -> libc::c_int {
    let mut success: libc::c_int = clusterManagerStartTransaction(owner);
    if success == 0 {
        return 0 as libc::c_int;
    }
    clusterManagerDelSlot(owner, slot, 1 as libc::c_int);
    clusterManagerAddSlot(owner, slot);
    if do_clear != 0 {
        clusterManagerClearSlotStatus(owner, slot);
    }
    clusterManagerBumpEpoch(owner);
    success = clusterManagerExecTransaction(
        owner,
        Some(
            clusterManagerOnSetOwnerErr
                as unsafe extern "C" fn(
                    *mut redisReply,
                    *mut clusterManagerNode,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    return success;
}
unsafe extern "C" fn clusterManagerCompareKeysValues(
    mut n1: *mut clusterManagerNode,
    mut n2: *mut clusterManagerNode,
    mut keys_reply: *mut redisReply,
    mut diffs: *mut list,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut argc: size_t = ((*keys_reply).elements)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    static mut hash_zero: *const libc::c_char = b"0000000000000000000000000000000000000000\0"
        as *const u8 as *const libc::c_char;
    let mut argv: *mut *mut libc::c_char = zcalloc(
        argc.wrapping_mul(core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut argv_len: *mut size_t = zcalloc(
        argc.wrapping_mul(core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    let ref mut fresh30 = *argv.offset(0 as libc::c_int as isize);
    *fresh30 = b"DEBUG\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    *argv_len.offset(0 as libc::c_int as isize) = 5 as libc::c_int as size_t;
    let ref mut fresh31 = *argv.offset(1 as libc::c_int as isize);
    *fresh31 = b"DIGEST-VALUE\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    *argv_len.offset(1 as libc::c_int as isize) = 12 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < (*keys_reply).elements {
        let mut entry: *mut redisReply = *((*keys_reply).element).offset(i as isize);
        let mut idx: libc::c_int = i.wrapping_add(2 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        let ref mut fresh32 = *argv.offset(idx as isize);
        *fresh32 = (*entry).str_0;
        *argv_len.offset(idx as isize) = (*entry).len;
        i = i.wrapping_add(1);
    }
    let mut success: libc::c_int = 0 as libc::c_int;
    let mut _reply1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _reply2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r1: *mut redisReply = 0 as *mut redisReply;
    let mut r2: *mut redisReply = 0 as *mut redisReply;
    redisAppendCommandArgv(
        (*n1).context,
        argc as libc::c_int,
        argv as *mut *const libc::c_char,
        argv_len,
    );
    success = (redisGetReply((*n1).context, &mut _reply1) == 0 as libc::c_int)
        as libc::c_int;
    if !(success == 0) {
        r1 = _reply1 as *mut redisReply;
        redisAppendCommandArgv(
            (*n2).context,
            argc as libc::c_int,
            argv as *mut *const libc::c_char,
            argv_len,
        );
        success = (redisGetReply((*n2).context, &mut _reply2) == 0 as libc::c_int)
            as libc::c_int;
        if !(success == 0) {
            r2 = _reply2 as *mut redisReply;
            success = ((*r1).type_0 != 6 as libc::c_int
                && (*r2).type_0 != 6 as libc::c_int) as libc::c_int;
            if (*r1).type_0 == 6 as libc::c_int {
                clusterManagerLog(
                    3 as libc::c_int,
                    b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                        as *const libc::c_char,
                    (*n1).ip,
                    (*n1).port,
                    (*r1).str_0,
                );
                success = 0 as libc::c_int;
            }
            if (*r2).type_0 == 6 as libc::c_int {
                clusterManagerLog(
                    3 as libc::c_int,
                    b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                        as *const libc::c_char,
                    (*n2).ip,
                    (*n2).port,
                    (*r2).str_0,
                );
                success = 0 as libc::c_int;
            }
            if !(success == 0) {
                if (*keys_reply).elements == (*r1).elements
                    && (*keys_reply).elements == (*r2).elements
                {} else {
                    __assert_fail(
                        b"keys_reply->elements == r1->elements && keys_reply->elements == r2->elements\0"
                            as *const u8 as *const libc::c_char,
                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                        4143 as libc::c_int as libc::c_uint,
                        (*core::mem::transmute::<
                            &[u8; 102],
                            &[libc::c_char; 102],
                        >(
                            b"int clusterManagerCompareKeysValues(clusterManagerNode *, clusterManagerNode *, redisReply *, list *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                i = 0 as libc::c_int as size_t;
                while i < (*keys_reply).elements {
                    let mut key: *mut libc::c_char = (**((*keys_reply).element)
                        .offset(i as isize))
                        .str_0;
                    let mut hash1: *mut libc::c_char = (**((*r1).element)
                        .offset(i as isize))
                        .str_0;
                    let mut hash2: *mut libc::c_char = (**((*r2).element)
                        .offset(i as isize))
                        .str_0;
                    if !(strcmp(hash1, hash_zero) == 0 as libc::c_int
                        || strcmp(hash2, hash_zero) == 0 as libc::c_int)
                    {
                        if strcmp(hash1, hash2) != 0 as libc::c_int {
                            listAddNodeTail(diffs, key as *mut libc::c_void);
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
    }
    if !r1.is_null() {
        freeReplyObject(r1 as *mut libc::c_void);
    }
    if !r2.is_null() {
        freeReplyObject(r2 as *mut libc::c_void);
    }
    zfree(argv as *mut libc::c_void);
    zfree(argv_len as *mut libc::c_void);
    return success;
}
unsafe extern "C" fn clusterManagerMigrateKeysInReply(
    mut source: *mut clusterManagerNode,
    mut target: *mut clusterManagerNode,
    mut reply: *mut redisReply,
    mut replace: libc::c_int,
    mut timeout: libc::c_int,
    mut dots: *mut libc::c_char,
) -> *mut redisReply {
    let mut migrate_reply: *mut redisReply = 0 as *mut redisReply;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argv_len: *mut size_t = 0 as *mut size_t;
    let mut c: libc::c_int = if replace != 0 {
        8 as libc::c_int
    } else {
        7 as libc::c_int
    };
    if !(config.conn_info.auth).is_null() {
        c += 2 as libc::c_int;
    }
    if !(config.conn_info.user).is_null() {
        c += 1 as libc::c_int;
    }
    let mut argc: size_t = (c as libc::c_ulong).wrapping_add((*reply).elements);
    let mut i: size_t = 0;
    let mut offset: size_t = 6 as libc::c_int as size_t;
    argv = zcalloc(
        argc.wrapping_mul(core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    argv_len = zcalloc(
        argc.wrapping_mul(core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    let mut portstr: [libc::c_char; 255] = [0; 255];
    let mut timeoutstr: [libc::c_char; 255] = [0; 255];
    snprintf(
        portstr.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        (*target).port,
    );
    snprintf(
        timeoutstr.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        timeout,
    );
    let ref mut fresh33 = *argv.offset(0 as libc::c_int as isize);
    *fresh33 = b"MIGRATE\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    *argv_len.offset(0 as libc::c_int as isize) = 7 as libc::c_int as size_t;
    let ref mut fresh34 = *argv.offset(1 as libc::c_int as isize);
    *fresh34 = (*target).ip;
    *argv_len.offset(1 as libc::c_int as isize) = strlen((*target).ip);
    let ref mut fresh35 = *argv.offset(2 as libc::c_int as isize);
    *fresh35 = portstr.as_mut_ptr();
    *argv_len.offset(2 as libc::c_int as isize) = strlen(portstr.as_mut_ptr());
    let ref mut fresh36 = *argv.offset(3 as libc::c_int as isize);
    *fresh36 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    *argv_len.offset(3 as libc::c_int as isize) = 0 as libc::c_int as size_t;
    let ref mut fresh37 = *argv.offset(4 as libc::c_int as isize);
    *fresh37 = b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    *argv_len.offset(4 as libc::c_int as isize) = 1 as libc::c_int as size_t;
    let ref mut fresh38 = *argv.offset(5 as libc::c_int as isize);
    *fresh38 = timeoutstr.as_mut_ptr();
    *argv_len.offset(5 as libc::c_int as isize) = strlen(timeoutstr.as_mut_ptr());
    if replace != 0 {
        let ref mut fresh39 = *argv.offset(offset as isize);
        *fresh39 = b"REPLACE\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        *argv_len.offset(offset as isize) = 7 as libc::c_int as size_t;
        offset = offset.wrapping_add(1);
    }
    if !(config.conn_info.auth).is_null() {
        if !(config.conn_info.user).is_null() {
            let ref mut fresh40 = *argv.offset(offset as isize);
            *fresh40 = b"AUTH2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            *argv_len.offset(offset as isize) = 5 as libc::c_int as size_t;
            offset = offset.wrapping_add(1);
            let ref mut fresh41 = *argv.offset(offset as isize);
            *fresh41 = config.conn_info.user;
            *argv_len.offset(offset as isize) = strlen(config.conn_info.user);
            offset = offset.wrapping_add(1);
            let ref mut fresh42 = *argv.offset(offset as isize);
            *fresh42 = config.conn_info.auth;
            *argv_len.offset(offset as isize) = strlen(config.conn_info.auth);
            offset = offset.wrapping_add(1);
        } else {
            let ref mut fresh43 = *argv.offset(offset as isize);
            *fresh43 = b"AUTH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            *argv_len.offset(offset as isize) = 4 as libc::c_int as size_t;
            offset = offset.wrapping_add(1);
            let ref mut fresh44 = *argv.offset(offset as isize);
            *fresh44 = config.conn_info.auth;
            *argv_len.offset(offset as isize) = strlen(config.conn_info.auth);
            offset = offset.wrapping_add(1);
        }
    }
    let ref mut fresh45 = *argv.offset(offset as isize);
    *fresh45 = b"KEYS\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    *argv_len.offset(offset as isize) = 4 as libc::c_int as size_t;
    offset = offset.wrapping_add(1);
    i = 0 as libc::c_int as size_t;
    while i < (*reply).elements {
        let mut entry: *mut redisReply = *((*reply).element).offset(i as isize);
        let mut idx: size_t = i.wrapping_add(offset);
        if (*entry).type_0 == 1 as libc::c_int {} else {
            __assert_fail(
                b"entry->type == REDIS_REPLY_STRING\0" as *const u8
                    as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                4227 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 121],
                    &[libc::c_char; 121],
                >(
                    b"redisReply *clusterManagerMigrateKeysInReply(clusterManagerNode *, clusterManagerNode *, redisReply *, int, int, char *)\0",
                ))
                    .as_ptr(),
            );
        };
        let ref mut fresh46 = *argv.offset(idx as isize);
        *fresh46 = hi_sdsnewlen((*entry).str_0 as *const libc::c_void, (*entry).len);
        *argv_len.offset(idx as isize) = (*entry).len;
        if !dots.is_null() {
            *dots.offset(i as isize) = '.' as i32 as libc::c_char;
        }
        i = i.wrapping_add(1);
    }
    if !dots.is_null() {
        *dots.offset((*reply).elements as isize) = '\0' as i32 as libc::c_char;
    }
    let mut _reply: *mut libc::c_void = 0 as *mut libc::c_void;
    redisAppendCommandArgv(
        (*source).context,
        argc as libc::c_int,
        argv as *mut *const libc::c_char,
        argv_len,
    );
    let mut success: libc::c_int = (redisGetReply((*source).context, &mut _reply)
        == 0 as libc::c_int) as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*reply).elements {
        hi_sdsfree(*argv.offset(i.wrapping_add(offset) as isize));
        i = i.wrapping_add(1);
    }
    if !(success == 0) {
        migrate_reply = _reply as *mut redisReply;
    }
    zfree(argv as *mut libc::c_void);
    zfree(argv_len as *mut libc::c_void);
    return migrate_reply;
}
unsafe extern "C" fn clusterManagerMigrateKeysInSlot(
    mut source: *mut clusterManagerNode,
    mut target: *mut clusterManagerNode,
    mut slot: libc::c_int,
    mut timeout: libc::c_int,
    mut pipeline: libc::c_int,
    mut verbose: libc::c_int,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut count: size_t = 0;
    let mut current_block: u64;
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut do_fix: libc::c_int = config.cluster_manager_command.flags
        & (1 as libc::c_int) << 0 as libc::c_int;
    let mut do_replace: libc::c_int = config.cluster_manager_command.flags
        & (1 as libc::c_int) << 6 as libc::c_int;
    loop {
        let mut dots: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut reply: *mut redisReply = 0 as *mut redisReply;
        let mut migrate_reply: *mut redisReply = 0 as *mut redisReply;
        reply = redisCommand(
            (*source).context,
            b"CLUSTER GETKEYSINSLOT %d %d\0" as *const u8 as *const libc::c_char,
            slot,
            pipeline,
        ) as *mut redisReply;
        success = (reply != 0 as *mut libc::c_void as *mut redisReply) as libc::c_int;
        if success == 0 {
            return 0 as libc::c_int;
        }
        if (*reply).type_0 == 6 as libc::c_int {
            success = 0 as libc::c_int;
            if !err.is_null() {
                *err = zmalloc(
                    ((*reply).len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                strcpy(*err, (*reply).str_0);
                clusterManagerLog(
                    3 as libc::c_int,
                    b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                        as *const libc::c_char,
                    (*source).ip,
                    (*source).port,
                    *err,
                );
            }
        } else {
            if (*reply).type_0 == 2 as libc::c_int {} else {
                __assert_fail(
                    b"reply->type == REDIS_REPLY_ARRAY\0" as *const u8
                        as *const libc::c_char,
                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                    4275 as libc::c_int as libc::c_uint,
                    (*core::mem::transmute::<
                        &[u8; 109],
                        &[libc::c_char; 109],
                    >(
                        b"int clusterManagerMigrateKeysInSlot(clusterManagerNode *, clusterManagerNode *, int, int, int, int, char **)\0",
                    ))
                        .as_ptr(),
                );
            };
            count = (*reply).elements;
            if count == 0 as libc::c_int as libc::c_ulong {
                freeReplyObject(reply as *mut libc::c_void);
                break;
            } else {
                if verbose != 0 {
                    dots = zmalloc(
                        count
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_char;
                }
                migrate_reply = clusterManagerMigrateKeysInReply(
                    source,
                    target,
                    reply,
                    0 as libc::c_int,
                    timeout,
                    dots,
                );
                if !migrate_reply.is_null() {
                    if (*migrate_reply).type_0 == 6 as libc::c_int {
                        let mut is_busy: libc::c_int = (strstr(
                            (*migrate_reply).str_0,
                            b"BUSYKEY\0" as *const u8 as *const libc::c_char,
                        ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
                        let mut not_served: libc::c_int = 0 as libc::c_int;
                        if is_busy == 0 {
                            let mut get_owner_err: *mut libc::c_char = 0
                                as *mut libc::c_char;
                            let mut served_by: *mut clusterManagerNode = clusterManagerGetSlotOwner(
                                source,
                                slot,
                                &mut get_owner_err,
                            );
                            if served_by.is_null() {
                                if get_owner_err.is_null() {
                                    not_served = 1 as libc::c_int;
                                } else {
                                    clusterManagerLog(
                                        3 as libc::c_int,
                                        b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*source).ip,
                                        (*source).port,
                                        get_owner_err,
                                    );
                                    zfree(get_owner_err as *mut libc::c_void);
                                }
                            }
                        }
                        if is_busy != 0 || not_served != 0 {
                            if do_fix != 0 && not_served != 0 {
                                clusterManagerLog(
                                    2 as libc::c_int,
                                    b"*** Slot was not served, setting owner to node %s:%d.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    (*target).ip,
                                    (*target).port,
                                );
                                clusterManagerSetSlot(
                                    source,
                                    target,
                                    slot,
                                    b"node\0" as *const u8 as *const libc::c_char,
                                    0 as *mut *mut libc::c_char,
                                );
                            }
                            if is_busy != 0 {
                                clusterManagerLog(
                                    2 as libc::c_int,
                                    b"\n*** Target key exists\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                if do_replace == 0 {
                                    clusterManagerLog(
                                        2 as libc::c_int,
                                        b"*** Checking key values on both nodes...\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    let mut diffs: *mut list = listCreate();
                                    success = clusterManagerCompareKeysValues(
                                        source,
                                        target,
                                        reply,
                                        diffs,
                                    );
                                    if success == 0 {
                                        clusterManagerLog(
                                            3 as libc::c_int,
                                            b"*** Value check failed!\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        listRelease(diffs);
                                        current_block = 7487061004554222981;
                                    } else if (*diffs).len > 0 as libc::c_int as libc::c_ulong {
                                        success = 0 as libc::c_int;
                                        clusterManagerLog(
                                            3 as libc::c_int,
                                            b"*** Found %d key(s) in both source node and target node having different values.\n    Source node: %s:%d\n    Target node: %s:%d\n    Keys(s):\n\0"
                                                as *const u8 as *const libc::c_char,
                                            (*diffs).len,
                                            (*source).ip,
                                            (*source).port,
                                            (*target).ip,
                                            (*target).port,
                                        );
                                        let mut dli: listIter = listIter {
                                            next: 0 as *mut listNode,
                                            direction: 0,
                                        };
                                        let mut dln: *mut listNode = 0 as *mut listNode;
                                        listRewind(diffs, &mut dli);
                                        loop {
                                            dln = listNext(&mut dli);
                                            if dln.is_null() {
                                                break;
                                            }
                                            let mut k: *mut libc::c_char = (*dln).value
                                                as *mut libc::c_char;
                                            clusterManagerLog(
                                                3 as libc::c_int,
                                                b"    - %s\n\0" as *const u8 as *const libc::c_char,
                                                k,
                                            );
                                        }
                                        clusterManagerLog(
                                            3 as libc::c_int,
                                            b"Please fix the above key(s) manually and try again or relaunch the command \nwith --cluster-replace option to force key overriding.\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        listRelease(diffs);
                                        current_block = 7487061004554222981;
                                    } else {
                                        listRelease(diffs);
                                        current_block = 10399321362245223758;
                                    }
                                } else {
                                    current_block = 10399321362245223758;
                                }
                                match current_block {
                                    7487061004554222981 => {}
                                    _ => {
                                        clusterManagerLog(
                                            2 as libc::c_int,
                                            b"*** Replacing target keys...\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 1924505913685386279;
                                    }
                                }
                            } else {
                                current_block = 1924505913685386279;
                            }
                            match current_block {
                                7487061004554222981 => {}
                                _ => {
                                    freeReplyObject(migrate_reply as *mut libc::c_void);
                                    migrate_reply = clusterManagerMigrateKeysInReply(
                                        source,
                                        target,
                                        reply,
                                        is_busy,
                                        timeout,
                                        0 as *mut libc::c_char,
                                    );
                                    success = (!migrate_reply.is_null()
                                        && (*migrate_reply).type_0 != 6 as libc::c_int)
                                        as libc::c_int;
                                    current_block = 11777552016271000781;
                                }
                            }
                        } else {
                            success = 0 as libc::c_int;
                            current_block = 11777552016271000781;
                        }
                        match current_block {
                            7487061004554222981 => {}
                            _ => {
                                if success == 0 {
                                    if !migrate_reply.is_null() {
                                        if !err.is_null() {
                                            *err = zmalloc(
                                                ((*migrate_reply).len)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(
                                                        core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                    ),
                                            ) as *mut libc::c_char;
                                            strcpy(*err, (*migrate_reply).str_0);
                                        }
                                        printf(b"\n\0" as *const u8 as *const libc::c_char);
                                        clusterManagerLog(
                                            3 as libc::c_int,
                                            b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                                                as *const libc::c_char,
                                            (*source).ip,
                                            (*source).port,
                                            (*migrate_reply).str_0,
                                        );
                                    }
                                    current_block = 7487061004554222981;
                                } else {
                                    current_block = 17836213544692497527;
                                }
                            }
                        }
                    } else {
                        current_block = 17836213544692497527;
                    }
                    match current_block {
                        7487061004554222981 => {}
                        _ => {
                            if verbose != 0 {
                                printf(b"%s\0" as *const u8 as *const libc::c_char, dots);
                                fflush(stdout);
                            }
                        }
                    }
                }
            }
        }
        if !reply.is_null() {
            freeReplyObject(reply as *mut libc::c_void);
        }
        if !migrate_reply.is_null() {
            freeReplyObject(migrate_reply as *mut libc::c_void);
        }
        if !dots.is_null() {
            zfree(dots as *mut libc::c_void);
        }
        if success == 0 {
            break;
        }
    }
    return success;
}
unsafe extern "C" fn clusterManagerMoveSlot(
    mut source: *mut clusterManagerNode,
    mut target: *mut clusterManagerNode,
    mut slot: libc::c_int,
    mut opts: libc::c_int,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    if opts & (1 as libc::c_int) << 6 as libc::c_int == 0 {
        printf(
            b"Moving slot %d from %s:%d to %s:%d: \0" as *const u8
                as *const libc::c_char,
            slot,
            (*source).ip,
            (*source).port,
            (*target).ip,
            (*target).port,
        );
        fflush(stdout);
    }
    if !err.is_null() {
        *err = 0 as *mut libc::c_char;
    }
    let mut pipeline: libc::c_int = config.cluster_manager_command.pipeline;
    let mut timeout: libc::c_int = config.cluster_manager_command.timeout;
    let mut print_dots: libc::c_int = opts & (1 as libc::c_int) << 7 as libc::c_int;
    let mut option_cold: libc::c_int = opts & (1 as libc::c_int) << 1 as libc::c_int;
    let mut success: libc::c_int = 1 as libc::c_int;
    if option_cold == 0 {
        success = clusterManagerSetSlot(
            target,
            source,
            slot,
            b"importing\0" as *const u8 as *const libc::c_char,
            err,
        );
        if success == 0 {
            return 0 as libc::c_int;
        }
        success = clusterManagerSetSlot(
            source,
            target,
            slot,
            b"migrating\0" as *const u8 as *const libc::c_char,
            err,
        );
        if success == 0 {
            return 0 as libc::c_int;
        }
    }
    success = clusterManagerMigrateKeysInSlot(
        source,
        target,
        slot,
        timeout,
        pipeline,
        print_dots,
        err,
    );
    if opts & (1 as libc::c_int) << 6 as libc::c_int == 0 {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if success == 0 {
        return 0 as libc::c_int;
    }
    if option_cold == 0 {
        success = clusterManagerSetSlot(
            target,
            target,
            slot,
            b"node\0" as *const u8 as *const libc::c_char,
            err,
        );
        if success == 0 {
            return 0 as libc::c_int;
        }
        success = clusterManagerSetSlot(
            source,
            target,
            slot,
            b"node\0" as *const u8 as *const libc::c_char,
            err,
        );
        let mut acceptable: *const libc::c_char = b"ERR Please use SETSLOT only with masters.\0"
            as *const u8 as *const libc::c_char;
        if success == 0 && !err.is_null()
            && strncmp(*err, acceptable, strlen(acceptable)) == 0
        {
            zfree(*err as *mut libc::c_void);
            *err = 0 as *mut libc::c_char;
        } else if success == 0 && !err.is_null() {
            return 0 as libc::c_int
        }
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind(cluster_manager.nodes, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
            if n == target || n == source {
                continue;
            }
            if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                continue;
            }
            success = clusterManagerSetSlot(
                n,
                target,
                slot,
                b"node\0" as *const u8 as *const libc::c_char,
                err,
            );
            if success == 0 {
                return 0 as libc::c_int;
            }
        }
    }
    if opts & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        (*source).slots[slot as usize] = 0 as libc::c_int as uint8_t;
        (*target).slots[slot as usize] = 1 as libc::c_int as uint8_t;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn clusterManagerFlushNodeConfig(
    mut node: *mut clusterManagerNode,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    if (*node).dirty == 0 {
        return 0 as libc::c_int;
    }
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut is_err: libc::c_int = 0 as libc::c_int;
    let mut success: libc::c_int = 1 as libc::c_int;
    if !err.is_null() {
        *err = 0 as *mut libc::c_char;
    }
    if !((*node).replicate).is_null() {
        reply = redisCommand(
            (*node).context,
            b"CLUSTER REPLICATE %s\0" as *const u8 as *const libc::c_char,
            (*node).replicate,
        ) as *mut redisReply;
        if reply.is_null()
            || {
                is_err = ((*reply).type_0 == 6 as libc::c_int) as libc::c_int;
                is_err != 0
            }
        {
            if is_err != 0 && !err.is_null() {
                *err = zmalloc(
                    ((*reply).len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                strcpy(*err, (*reply).str_0);
            }
            success = 0 as libc::c_int;
            current_block = 2913733023007058749;
        } else {
            current_block = 10599921512955367680;
        }
    } else {
        let mut added: libc::c_int = clusterManagerAddSlots(node, err);
        if added == 0 || !(*err).is_null() {
            success = 0 as libc::c_int;
        }
        current_block = 10599921512955367680;
    }
    match current_block {
        10599921512955367680 => {
            (*node).dirty = 0 as libc::c_int;
        }
        _ => {}
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerWaitForClusterJoin() {
    printf(b"Waiting for the cluster to join\n\0" as *const u8 as *const libc::c_char);
    let mut counter: libc::c_int = 0 as libc::c_int;
    let mut check_after: libc::c_int = 20 as libc::c_int
        + ((*cluster_manager.nodes).len as libc::c_float * 0.15f32) as libc::c_int;
    while clusterManagerIsConfigConsistent() == 0 {
        printf(b".\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
        sleep(1 as libc::c_int as libc::c_uint);
        counter += 1;
        if counter > check_after {
            let mut status: *mut dict = clusterManagerGetLinkStatus();
            let mut iter: *mut dictIterator = 0 as *mut dictIterator;
            if !status.is_null()
                && ((*status).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*status).ht_used[1 as libc::c_int as usize])
                    > 0 as libc::c_int as libc::c_ulong
            {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
                clusterManagerLog(
                    3 as libc::c_int,
                    b"Warning: %d node(s) may be unreachable\n\0" as *const u8
                        as *const libc::c_char,
                    ((*status).ht_used[0 as libc::c_int as usize])
                        .wrapping_add((*status).ht_used[1 as libc::c_int as usize]),
                );
                iter = dictGetIterator(status);
                let mut entry: *mut dictEntry = 0 as *mut dictEntry;
                loop {
                    entry = dictNext(iter);
                    if entry.is_null() {
                        break;
                    }
                    let mut nodeaddr: hisds = (*entry).key as hisds;
                    let mut node_ip: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut node_port: libc::c_int = 0 as libc::c_int;
                    let mut node_bus_port: libc::c_int = 0 as libc::c_int;
                    let mut from: *mut list = (*entry).v.val as *mut list;
                    if parseClusterNodeAddress(
                        nodeaddr,
                        &mut node_ip,
                        &mut node_port,
                        &mut node_bus_port,
                    ) != 0 && node_bus_port != 0
                    {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b" - The port %d of node %s may be unreachable from:\n\0"
                                as *const u8 as *const libc::c_char,
                            node_bus_port,
                            node_ip,
                        );
                    } else {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b" - Node %s may be unreachable from:\n\0" as *const u8
                                as *const libc::c_char,
                            nodeaddr,
                        );
                    }
                    let mut li: listIter = listIter {
                        next: 0 as *mut listNode,
                        direction: 0,
                    };
                    let mut ln: *mut listNode = 0 as *mut listNode;
                    listRewind(from, &mut li);
                    loop {
                        ln = listNext(&mut li);
                        if ln.is_null() {
                            break;
                        }
                        let mut from_addr: hisds = (*ln).value as hisds;
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"   %s\n\0" as *const u8 as *const libc::c_char,
                            from_addr,
                        );
                        hi_sdsfree(from_addr);
                    }
                    clusterManagerLog(
                        3 as libc::c_int,
                        b"Cluster bus ports must be reachable by every node.\nRemember that cluster bus ports are different from standard instance ports.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    listEmpty(from);
                }
            }
            if !iter.is_null() {
                dictReleaseIterator(iter);
            }
            if !status.is_null() {
                dictRelease(status);
            }
            counter = 0 as libc::c_int;
        }
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn clusterManagerNodeLoadInfo(
    mut node: *mut clusterManagerNode,
    mut opts: libc::c_int,
    mut err: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut getfriends: libc::c_int = 0;
    let mut lines: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"CLUSTER NODES\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    let mut success: libc::c_int = 1 as libc::c_int;
    *err = 0 as *mut libc::c_char;
    if clusterManagerCheckRedisReply(node, reply, err) == 0 {
        success = 0 as libc::c_int;
    } else {
        getfriends = opts & (1 as libc::c_int) << 0 as libc::c_int;
        lines = (*reply).str_0;
        p = 0 as *mut libc::c_char;
        line = 0 as *mut libc::c_char;
        loop {
            p = strstr(lines, b"\n\0" as *const u8 as *const libc::c_char);
            if p.is_null() {
                break;
            }
            *p = '\0' as i32 as libc::c_char;
            line = lines;
            lines = p.offset(1 as libc::c_int as isize);
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut flags: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut master_id: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ping_sent: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ping_recv: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut config_epoch: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut link_status: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut i: libc::c_int = 0 as libc::c_int;
            loop {
                p = strchr(line, ' ' as i32);
                if p.is_null() {
                    break;
                }
                *p = '\0' as i32 as libc::c_char;
                let mut token: *mut libc::c_char = line;
                line = p.offset(1 as libc::c_int as isize);
                let fresh47 = i;
                i = i + 1;
                match fresh47 {
                    0 => {
                        name = token;
                    }
                    1 => {
                        addr = token;
                    }
                    2 => {
                        flags = token;
                    }
                    3 => {
                        master_id = token;
                    }
                    4 => {
                        ping_sent = token;
                    }
                    5 => {
                        ping_recv = token;
                    }
                    6 => {
                        config_epoch = token;
                    }
                    7 => {
                        link_status = token;
                    }
                    _ => {}
                }
                if i == 8 as libc::c_int {
                    break;
                }
            }
            if flags.is_null() {
                success = 0 as libc::c_int;
                break;
            } else {
                let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut port: libc::c_int = 0 as libc::c_int;
                let mut bus_port: libc::c_int = 0 as libc::c_int;
                if addr.is_null()
                    || parseClusterNodeAddress(addr, &mut ip, &mut port, &mut bus_port)
                        == 0
                {
                    fprintf(
                        stderr,
                        b"Error: invalid CLUSTER NODES reply\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    success = 0 as libc::c_int;
                    break;
                } else {
                    let mut myself: libc::c_int = (strstr(
                        flags,
                        b"myself\0" as *const u8 as *const libc::c_char,
                    ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
                    let mut currentNode: *mut clusterManagerNode = 0
                        as *mut clusterManagerNode;
                    if myself != 0 {
                        (*node).bus_port = bus_port;
                        (*node).flags |= (1 as libc::c_int) << 0 as libc::c_int;
                        currentNode = node;
                        clusterManagerNodeResetSlots(node);
                        if i == 8 as libc::c_int {
                            let mut remaining: libc::c_int = strlen(line) as libc::c_int;
                            while remaining > 0 as libc::c_int {
                                p = strchr(line, ' ' as i32);
                                if p.is_null() {
                                    p = line.offset(remaining as isize);
                                }
                                remaining = (remaining as libc::c_long
                                    - p.offset_from(line) as libc::c_long) as libc::c_int;
                                let mut slotsdef: *mut libc::c_char = line;
                                *p = '\0' as i32 as libc::c_char;
                                if remaining != 0 {
                                    line = p.offset(1 as libc::c_int as isize);
                                    remaining -= 1;
                                } else {
                                    line = p;
                                }
                                let mut dash: *mut libc::c_char = 0 as *mut libc::c_char;
                                if *slotsdef.offset(0 as libc::c_int as isize)
                                    as libc::c_int == '[' as i32
                                {
                                    slotsdef = slotsdef.offset(1);
                                    p = strstr(
                                        slotsdef,
                                        b"->-\0" as *const u8 as *const libc::c_char,
                                    );
                                    if !p.is_null() {
                                        *p = '\0' as i32 as libc::c_char;
                                        p = p.offset(3 as libc::c_int as isize);
                                        let mut closing_bracket: *mut libc::c_char = strchr(
                                            p,
                                            ']' as i32,
                                        );
                                        if !closing_bracket.is_null() {
                                            *closing_bracket = '\0' as i32 as libc::c_char;
                                        }
                                        let mut slot: hisds = hi_sdsnew(slotsdef);
                                        let mut dst: hisds = hi_sdsnew(p);
                                        (*node).migrating_count += 2 as libc::c_int;
                                        (*node)
                                            .migrating = zrealloc(
                                            (*node).migrating as *mut libc::c_void,
                                            ((*node).migrating_count as libc::c_ulong)
                                                .wrapping_mul(
                                                    core::mem::size_of::<hisds>() as libc::c_ulong,
                                                ),
                                        ) as *mut hisds;
                                        let ref mut fresh48 = *((*node).migrating)
                                            .offset(
                                                ((*node).migrating_count - 2 as libc::c_int) as isize,
                                            );
                                        *fresh48 = slot;
                                        let ref mut fresh49 = *((*node).migrating)
                                            .offset(
                                                ((*node).migrating_count - 1 as libc::c_int) as isize,
                                            );
                                        *fresh49 = dst;
                                    } else {
                                        p = strstr(
                                            slotsdef,
                                            b"-<-\0" as *const u8 as *const libc::c_char,
                                        );
                                        if !p.is_null() {
                                            *p = '\0' as i32 as libc::c_char;
                                            p = p.offset(3 as libc::c_int as isize);
                                            let mut closing_bracket_0: *mut libc::c_char = strchr(
                                                p,
                                                ']' as i32,
                                            );
                                            if !closing_bracket_0.is_null() {
                                                *closing_bracket_0 = '\0' as i32 as libc::c_char;
                                            }
                                            let mut slot_0: hisds = hi_sdsnew(slotsdef);
                                            let mut src: hisds = hi_sdsnew(p);
                                            (*node).importing_count += 2 as libc::c_int;
                                            (*node)
                                                .importing = zrealloc(
                                                (*node).importing as *mut libc::c_void,
                                                ((*node).importing_count as libc::c_ulong)
                                                    .wrapping_mul(
                                                        core::mem::size_of::<hisds>() as libc::c_ulong,
                                                    ),
                                            ) as *mut hisds;
                                            let ref mut fresh50 = *((*node).importing)
                                                .offset(
                                                    ((*node).importing_count - 2 as libc::c_int) as isize,
                                                );
                                            *fresh50 = slot_0;
                                            let ref mut fresh51 = *((*node).importing)
                                                .offset(
                                                    ((*node).importing_count - 1 as libc::c_int) as isize,
                                                );
                                            *fresh51 = src;
                                        }
                                    }
                                } else {
                                    dash = strchr(slotsdef, '-' as i32);
                                    if !dash.is_null() {
                                        p = dash;
                                        let mut start: libc::c_int = 0;
                                        let mut stop: libc::c_int = 0;
                                        *p = '\0' as i32 as libc::c_char;
                                        start = atoi(slotsdef);
                                        stop = atoi(p.offset(1 as libc::c_int as isize));
                                        (*node).slots_count += stop - (start - 1 as libc::c_int);
                                        while start <= stop {
                                            let fresh52 = start;
                                            start = start + 1;
                                            (*node)
                                                .slots[fresh52 as usize] = 1 as libc::c_int as uint8_t;
                                        }
                                    } else if p > slotsdef {
                                        (*node)
                                            .slots[atoi(slotsdef)
                                            as usize] = 1 as libc::c_int as uint8_t;
                                        (*node).slots_count += 1;
                                    }
                                }
                            }
                        }
                        (*node).dirty = 0 as libc::c_int;
                    } else if getfriends == 0 {
                        if (*node).flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        currentNode = clusterManagerNewNode(
                            hi_sdsnew(ip),
                            port,
                            bus_port,
                        );
                        (*currentNode).flags |= (1 as libc::c_int) << 2 as libc::c_int;
                        if ((*node).friends).is_null() {
                            (*node).friends = listCreate();
                        }
                        listAddNodeTail(
                            (*node).friends,
                            currentNode as *mut libc::c_void,
                        );
                    }
                    if !name.is_null() {
                        if !((*currentNode).name).is_null() {
                            hi_sdsfree((*currentNode).name);
                        }
                        (*currentNode).name = hi_sdsnew(name);
                    }
                    if !((*currentNode).flags_str).is_null() {
                        freeClusterManagerNodeFlags((*currentNode).flags_str);
                    }
                    (*currentNode).flags_str = listCreate();
                    let mut flag_len: libc::c_int = 0;
                    loop {
                        flag_len = strlen(flags) as libc::c_int;
                        if !(flag_len > 0 as libc::c_int) {
                            break;
                        }
                        let mut flag: hisds = 0 as hisds;
                        let mut fp: *mut libc::c_char = strchr(flags, ',' as i32);
                        if !fp.is_null() {
                            *fp = '\0' as i32 as libc::c_char;
                            flag = hi_sdsnew(flags);
                            flags = fp.offset(1 as libc::c_int as isize);
                        } else {
                            flag = hi_sdsnew(flags);
                            flags = flags.offset(flag_len as isize);
                        }
                        if strcmp(
                            flag as *const libc::c_char,
                            b"noaddr\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            (*currentNode).flags
                                |= (1 as libc::c_int) << 3 as libc::c_int;
                        } else if strcmp(
                            flag as *const libc::c_char,
                            b"disconnected\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            (*currentNode).flags
                                |= (1 as libc::c_int) << 4 as libc::c_int;
                        } else if strcmp(
                            flag as *const libc::c_char,
                            b"fail\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            (*currentNode).flags
                                |= (1 as libc::c_int) << 5 as libc::c_int;
                        } else if strcmp(
                            flag as *const libc::c_char,
                            b"slave\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            (*currentNode).flags
                                |= (1 as libc::c_int) << 1 as libc::c_int;
                            if !master_id.is_null() {
                                if !((*currentNode).replicate).is_null() {
                                    hi_sdsfree((*currentNode).replicate);
                                }
                                (*currentNode).replicate = hi_sdsnew(master_id);
                            }
                        }
                        listAddNodeTail(
                            (*currentNode).flags_str,
                            flag as *mut libc::c_void,
                        );
                    }
                    if !config_epoch.is_null() {
                        (*currentNode).current_epoch = atoll(config_epoch) as uint64_t;
                    }
                    if !ping_sent.is_null() {
                        (*currentNode).ping_sent = atoll(ping_sent) as time_t;
                    }
                    if !ping_recv.is_null() {
                        (*currentNode).ping_recv = atoll(ping_recv) as time_t;
                    }
                    if getfriends == 0 && myself != 0 {
                        break;
                    }
                }
            }
        }
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return success;
}
unsafe extern "C" fn clusterManagerLoadInfoFromNode(
    mut node: *mut clusterManagerNode,
) -> libc::c_int {
    if ((*node).context).is_null() && clusterManagerNodeConnect(node) == 0 {
        freeClusterManagerNode(node);
        return 0 as libc::c_int;
    }
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    if clusterManagerNodeIsCluster(node, &mut e) == 0 {
        clusterManagerPrintNotClusterNodeError(node, e);
        if !e.is_null() {
            zfree(e as *mut libc::c_void);
        }
        freeClusterManagerNode(node);
        return 0 as libc::c_int;
    }
    e = 0 as *mut libc::c_char;
    if clusterManagerNodeLoadInfo(node, (1 as libc::c_int) << 0 as libc::c_int, &mut e)
        == 0
    {
        if !e.is_null() {
            clusterManagerLog(
                3 as libc::c_int,
                b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                    as *const libc::c_char,
                (*node).ip,
                (*node).port,
                e,
            );
            zfree(e as *mut libc::c_void);
        }
        freeClusterManagerNode(node);
        return 0 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    if !(cluster_manager.nodes).is_null() {
        listRewind(cluster_manager.nodes, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            freeClusterManagerNode((*ln).value as *mut clusterManagerNode);
        }
        listRelease(cluster_manager.nodes);
    }
    cluster_manager.nodes = listCreate();
    listAddNodeTail(cluster_manager.nodes, node as *mut libc::c_void);
    if !((*node).friends).is_null() {
        listRewind((*node).friends, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut friend: *mut clusterManagerNode = (*ln).value
                as *mut clusterManagerNode;
            if !(((*friend).ip).is_null() || (*friend).port == 0) {
                if !(((*friend).context).is_null()
                    && clusterManagerNodeConnect(friend) == 0)
                {
                    e = 0 as *mut libc::c_char;
                    if clusterManagerNodeLoadInfo(friend, 0 as libc::c_int, &mut e) != 0
                    {
                        if !((*friend).flags
                            & ((1 as libc::c_int) << 3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 5 as libc::c_int) != 0)
                        {
                            listAddNodeTail(
                                cluster_manager.nodes,
                                friend as *mut libc::c_void,
                            );
                            continue;
                        }
                    } else {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"[ERR] Unable to load info for node %s:%d\n\0" as *const u8
                                as *const libc::c_char,
                            (*friend).ip,
                            (*friend).port,
                        );
                    }
                }
            }
            if (*friend).flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                cluster_manager.unreachable_masters += 1;
            }
            freeClusterManagerNode(friend);
        }
        listRelease((*node).friends);
        (*node).friends = 0 as *mut list;
    }
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if !((*n).replicate).is_null() {
            let mut master: *mut clusterManagerNode = clusterManagerNodeByName(
                (*n).replicate as *const libc::c_char,
            );
            if master.is_null() {
                clusterManagerLog(
                    2 as libc::c_int,
                    b"*** WARNING: %s:%d claims to be slave of unknown node ID %s.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*n).ip,
                    (*n).port,
                    (*n).replicate,
                );
            } else {
                (*master).replicas_count += 1;
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterManagerSlotCompare(
    mut slot1: *const libc::c_void,
    mut slot2: *const libc::c_void,
) -> libc::c_int {
    let mut i1: *mut *const libc::c_char = slot1 as *mut *const libc::c_char;
    let mut i2: *mut *const libc::c_char = slot2 as *mut *const libc::c_char;
    return strcmp(*i1, *i2);
}
#[no_mangle]
pub unsafe extern "C" fn clusterManagerSlotCountCompareDesc(
    mut n1: *const libc::c_void,
    mut n2: *const libc::c_void,
) -> libc::c_int {
    let mut node1: *mut clusterManagerNode = *(n1 as *mut *mut clusterManagerNode);
    let mut node2: *mut clusterManagerNode = *(n2 as *mut *mut clusterManagerNode);
    return (*node2).slots_count - (*node1).slots_count;
}
#[no_mangle]
pub unsafe extern "C" fn clusterManagerCompareNodeBalance(
    mut n1: *const libc::c_void,
    mut n2: *const libc::c_void,
) -> libc::c_int {
    let mut node1: *mut clusterManagerNode = *(n1 as *mut *mut clusterManagerNode);
    let mut node2: *mut clusterManagerNode = *(n2 as *mut *mut clusterManagerNode);
    return (*node1).balance - (*node2).balance;
}
unsafe extern "C" fn clusterManagerGetConfigSignature(
    mut node: *mut clusterManagerNode,
) -> hisds {
    let mut lines: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut signature: hisds = 0 as hisds;
    let mut node_count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut name_len: libc::c_int = 0 as libc::c_int;
    let mut node_configs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"CLUSTER NODES\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    if !(reply.is_null() || (*reply).type_0 == 6 as libc::c_int) {
        lines = (*reply).str_0;
        p = 0 as *mut libc::c_char;
        line = 0 as *mut libc::c_char;
        loop {
            p = strstr(lines, b"\n\0" as *const u8 as *const libc::c_char);
            if p.is_null() {
                break;
            }
            i = 0 as libc::c_int;
            *p = '\0' as i32 as libc::c_char;
            line = lines;
            lines = p.offset(1 as libc::c_int as isize);
            let mut nodename: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut tot_size: libc::c_int = 0 as libc::c_int;
            loop {
                p = strchr(line, ' ' as i32);
                if p.is_null() {
                    break;
                }
                *p = '\0' as i32 as libc::c_char;
                let mut token: *mut libc::c_char = line;
                line = p.offset(1 as libc::c_int as isize);
                if i == 0 as libc::c_int {
                    nodename = token;
                    tot_size = p.offset_from(token) as libc::c_long as libc::c_int;
                    let fresh53 = tot_size;
                    tot_size = tot_size + 1;
                    name_len = fresh53;
                }
                i += 1;
                if i == 8 as libc::c_int {
                    break;
                }
            }
            if i != 8 as libc::c_int {
                continue;
            }
            if nodename.is_null() {
                continue;
            }
            let mut remaining: libc::c_int = strlen(line) as libc::c_int;
            if remaining == 0 as libc::c_int {
                continue;
            }
            let mut slots: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut c: libc::c_int = 0 as libc::c_int;
            while remaining > 0 as libc::c_int {
                p = strchr(line, ' ' as i32);
                if p.is_null() {
                    p = line.offset(remaining as isize);
                }
                let mut size: libc::c_int = p.offset_from(line) as libc::c_long
                    as libc::c_int;
                remaining -= size;
                tot_size += size;
                let mut slotsdef: *mut libc::c_char = line;
                *p = '\0' as i32 as libc::c_char;
                if remaining != 0 {
                    line = p.offset(1 as libc::c_int as isize);
                    remaining -= 1;
                } else {
                    line = p;
                }
                if *slotsdef.offset(0 as libc::c_int as isize) as libc::c_int
                    != '[' as i32
                {
                    c += 1;
                    slots = zrealloc(
                        slots as *mut libc::c_void,
                        (c as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut *mut libc::c_char;
                    let ref mut fresh54 = *slots.offset((c - 1 as libc::c_int) as isize);
                    *fresh54 = slotsdef;
                }
            }
            if c > 0 as libc::c_int {
                if c > 1 as libc::c_int {
                    qsort(
                        slots as *mut libc::c_void,
                        c as size_t,
                        core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        Some(
                            clusterManagerSlotCompare
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                        ),
                    );
                }
                node_count += 1;
                node_configs = zrealloc(
                    node_configs as *mut libc::c_void,
                    (node_count as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                tot_size = (tot_size as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul((c - 1 as libc::c_int) as libc::c_ulong),
                    ) as libc::c_int as libc::c_int;
                let mut cfg: *mut libc::c_char = zmalloc(
                    (core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(tot_size as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                memcpy(
                    cfg as *mut libc::c_void,
                    nodename as *const libc::c_void,
                    name_len as libc::c_ulong,
                );
                let mut sp: *mut libc::c_char = cfg.offset(name_len as isize);
                let fresh55 = sp;
                sp = sp.offset(1);
                *fresh55 = ':' as i32 as libc::c_char;
                i = 0 as libc::c_int;
                while i < c {
                    if i > 0 as libc::c_int {
                        let fresh56 = sp;
                        sp = sp.offset(1);
                        *fresh56 = ',' as i32 as libc::c_char;
                    }
                    let mut slen: libc::c_int = strlen(*slots.offset(i as isize))
                        as libc::c_int;
                    memcpy(
                        sp as *mut libc::c_void,
                        *slots.offset(i as isize) as *const libc::c_void,
                        slen as libc::c_ulong,
                    );
                    sp = sp.offset(slen as isize);
                    i += 1;
                }
                let fresh57 = sp;
                sp = sp.offset(1);
                *fresh57 = '\0' as i32 as libc::c_char;
                let ref mut fresh58 = *node_configs
                    .offset((node_count - 1 as libc::c_int) as isize);
                *fresh58 = cfg;
            }
            zfree(slots as *mut libc::c_void);
        }
        if node_count > 0 as libc::c_int {
            if node_count > 1 as libc::c_int {
                qsort(
                    node_configs as *mut libc::c_void,
                    node_count as size_t,
                    core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    Some(
                        clusterManagerSlotCompare
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
            }
            signature = hi_sdsempty();
            i = 0 as libc::c_int;
            while i < node_count {
                if i > 0 as libc::c_int {
                    signature = hi_sdscatprintf(
                        signature,
                        b"%c\0" as *const u8 as *const libc::c_char,
                        '|' as i32,
                    );
                }
                signature = hi_sdscatfmt(
                    signature,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    *node_configs.offset(i as isize),
                );
                i += 1;
            }
        }
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    if !node_configs.is_null() {
        i = 0 as libc::c_int;
        while i < node_count {
            zfree(*node_configs.offset(i as isize) as *mut libc::c_void);
            i += 1;
        }
        zfree(node_configs as *mut libc::c_void);
    }
    return signature;
}
unsafe extern "C" fn clusterManagerIsConfigConsistent() -> libc::c_int {
    if (cluster_manager.nodes).is_null() {
        return 0 as libc::c_int;
    }
    let mut consistent: libc::c_int = ((*cluster_manager.nodes).len
        <= 1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if consistent != 0 {
        return 1 as libc::c_int;
    }
    let mut first_cfg: hisds = 0 as hisds;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut node: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        let mut cfg: hisds = clusterManagerGetConfigSignature(node);
        if cfg.is_null() {
            consistent = 0 as libc::c_int;
            break;
        } else if first_cfg.is_null() {
            first_cfg = cfg;
        } else {
            consistent = (hi_sdscmp(first_cfg, cfg) == 0) as libc::c_int;
            hi_sdsfree(cfg);
            if consistent == 0 {
                break;
            }
        }
    }
    if !first_cfg.is_null() {
        hi_sdsfree(first_cfg);
    }
    return consistent;
}
unsafe extern "C" fn clusterManagerGetDisconnectedLinks(
    mut node: *mut clusterManagerNode,
) -> *mut list {
    let mut lines: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut links: *mut list = 0 as *mut list;
    let mut reply: *mut redisReply = redisCommand(
        (*node).context,
        b"CLUSTER NODES\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    if !(clusterManagerCheckRedisReply(node, reply, 0 as *mut *mut libc::c_char) == 0) {
        links = listCreate();
        lines = (*reply).str_0;
        p = 0 as *mut libc::c_char;
        line = 0 as *mut libc::c_char;
        loop {
            p = strstr(lines, b"\n\0" as *const u8 as *const libc::c_char);
            if p.is_null() {
                break;
            }
            let mut i: libc::c_int = 0 as libc::c_int;
            *p = '\0' as i32 as libc::c_char;
            line = lines;
            lines = p.offset(1 as libc::c_int as isize);
            let mut nodename: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut flags: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut link_status: *mut libc::c_char = 0 as *mut libc::c_char;
            loop {
                p = strchr(line, ' ' as i32);
                if p.is_null() {
                    break;
                }
                *p = '\0' as i32 as libc::c_char;
                let mut token: *mut libc::c_char = line;
                line = p.offset(1 as libc::c_int as isize);
                if i == 0 as libc::c_int {
                    nodename = token;
                } else if i == 1 as libc::c_int {
                    addr = token;
                } else if i == 2 as libc::c_int {
                    flags = token;
                } else if i == 7 as libc::c_int {
                    link_status = token;
                } else if i == 8 as libc::c_int {
                    break;
                }
                i += 1;
            }
            if i == 7 as libc::c_int {
                link_status = line;
            }
            if nodename.is_null() || addr.is_null() || flags.is_null()
                || link_status.is_null()
            {
                continue;
            }
            if !(strstr(flags, b"myself\0" as *const u8 as *const libc::c_char))
                .is_null()
            {
                continue;
            }
            let mut disconnected: libc::c_int = (!(strstr(
                flags,
                b"disconnected\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
                || !(strstr(
                    link_status,
                    b"disconnected\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()) as libc::c_int;
            let mut handshaking: libc::c_int = (strstr(
                flags,
                b"handshake\0" as *const u8 as *const libc::c_char,
            ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
            if disconnected != 0 || handshaking != 0 {
                let mut link: *mut clusterManagerLink = zmalloc(
                    core::mem::size_of::<clusterManagerLink>() as libc::c_ulong,
                ) as *mut clusterManagerLink;
                (*link).node_name = hi_sdsnew(nodename);
                (*link).node_addr = hi_sdsnew(addr);
                (*link).connected = 0 as libc::c_int;
                (*link).handshaking = handshaking;
                listAddNodeTail(links, link as *mut libc::c_void);
            }
        }
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return links;
}
unsafe extern "C" fn clusterManagerGetLinkStatus() -> *mut dict {
    if (cluster_manager.nodes).is_null() {
        return 0 as *mut dict;
    }
    let mut status: *mut dict = dictCreate(&mut clusterManagerLinkDictType);
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut node: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        let mut links: *mut list = clusterManagerGetDisconnectedLinks(node);
        if !links.is_null() {
            let mut lli: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            let mut lln: *mut listNode = 0 as *mut listNode;
            listRewind(links, &mut lli);
            loop {
                lln = listNext(&mut lli);
                if lln.is_null() {
                    break;
                }
                let mut link: *mut clusterManagerLink = (*lln).value
                    as *mut clusterManagerLink;
                let mut from: *mut list = 0 as *mut list;
                let mut entry: *mut dictEntry = dictFind(
                    status,
                    (*link).node_addr as *const libc::c_void,
                );
                if !entry.is_null() {
                    from = (*entry).v.val as *mut list;
                } else {
                    from = listCreate();
                    dictAdd(
                        status,
                        hi_sdsdup((*link).node_addr) as *mut libc::c_void,
                        from as *mut libc::c_void,
                    );
                }
                let mut myaddr: hisds = hi_sdsempty();
                myaddr = hi_sdscatfmt(
                    myaddr,
                    b"%s:%u\0" as *const u8 as *const libc::c_char,
                    (*node).ip,
                    (*node).port,
                );
                listAddNodeTail(from, myaddr as *mut libc::c_void);
                hi_sdsfree((*link).node_name);
                hi_sdsfree((*link).node_addr);
                zfree(link as *mut libc::c_void);
            }
            listRelease(links);
        }
    }
    return status;
}
unsafe extern "C" fn clusterManagerOnError(mut err: hisds) {
    if (cluster_manager.errors).is_null() {
        cluster_manager.errors = listCreate();
    }
    listAddNodeTail(cluster_manager.errors, err as *mut libc::c_void);
    clusterManagerLog(
        3 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        err,
    );
}
unsafe extern "C" fn clusterManagerGetCoveredSlots(
    mut all_slots: *mut libc::c_char,
) -> libc::c_int {
    if (cluster_manager.nodes).is_null() {
        return 0 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    let mut totslots: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut node: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        i = 0 as libc::c_int;
        while i < 16384 as libc::c_int {
            if (*node).slots[i as usize] as libc::c_int != 0
                && *all_slots.offset(i as isize) == 0
            {
                *all_slots.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                totslots += 1;
            }
            i += 1;
        }
    }
    return totslots;
}
unsafe extern "C" fn clusterManagerPrintSlotsList(mut slots: *mut list) {
    let mut n: clusterManagerNode = {
        let mut init = clusterManagerNode {
            context: 0 as *mut redisContext,
            name: 0 as *mut libc::c_char,
            ip: 0 as *mut libc::c_char,
            port: 0,
            bus_port: 0,
            current_epoch: 0,
            ping_sent: 0,
            ping_recv: 0,
            flags: 0,
            flags_str: 0 as *mut list,
            replicate: 0 as *mut libc::c_char,
            dirty: 0,
            slots: [0; 16384],
            slots_count: 0,
            replicas_count: 0,
            friends: 0 as *mut list,
            migrating: 0 as *mut hisds,
            importing: 0 as *mut hisds,
            migrating_count: 0,
            importing_count: 0,
            weight: 0.,
            balance: 0,
        };
        init
    };
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(slots, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut slot: libc::c_int = atoi((*ln).value as *const libc::c_char);
        if slot >= 0 as libc::c_int && slot < 16384 as libc::c_int {
            n.slots[slot as usize] = 1 as libc::c_int as uint8_t;
        }
    }
    let mut nodeslist: hisds = clusterManagerNodeSlotsString(&mut n);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, nodeslist);
    hi_sdsfree(nodeslist);
}
unsafe extern "C" fn clusterManagerGetNodeWithMostKeysInSlot(
    mut nodes: *mut list,
    mut slot: libc::c_int,
    mut err: *mut *mut libc::c_char,
) -> *mut clusterManagerNode {
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut numkeys: libc::c_int = 0 as libc::c_int;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(nodes, &mut li);
    if !err.is_null() {
        *err = 0 as *mut libc::c_char;
    }
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
            || !((*n).replicate).is_null()
        {
            continue;
        }
        let mut r: *mut redisReply = redisCommand(
            (*n).context,
            b"CLUSTER COUNTKEYSINSLOT %d\0" as *const u8 as *const libc::c_char,
            slot,
        ) as *mut redisReply;
        let mut success: libc::c_int = clusterManagerCheckRedisReply(n, r, err);
        if success != 0 {
            if (*r).integer > numkeys as libc::c_longlong || node.is_null() {
                numkeys = (*r).integer as libc::c_int;
                node = n;
            }
        }
        if !r.is_null() {
            freeReplyObject(r as *mut libc::c_void);
        }
        if !(success == 0) {
            continue;
        }
        if !err.is_null() && !(*err).is_null() {
            clusterManagerLog(
                3 as libc::c_int,
                b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                    as *const libc::c_char,
                (*n).ip,
                (*n).port,
                err,
            );
        }
        node = 0 as *mut clusterManagerNode;
        break;
    }
    return node;
}
unsafe extern "C" fn clusterManagerNodeWithLeastReplicas() -> *mut clusterManagerNode {
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut lowest_count: libc::c_int = 0 as libc::c_int;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            continue;
        }
        if node.is_null() || (*n).replicas_count < lowest_count {
            node = n;
            lowest_count = (*n).replicas_count;
        }
    }
    return node;
}
unsafe extern "C" fn clusterManagerNodeMasterRandom() -> *mut clusterManagerNode {
    let mut master_count: libc::c_int = 0 as libc::c_int;
    let mut idx: libc::c_int = 0;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            continue;
        }
        master_count += 1;
    }
    if master_count > 0 as libc::c_int {} else {
        __assert_fail(
            b"master_count > 0\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            5170 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"clusterManagerNode *clusterManagerNodeMasterRandom()\0"))
                .as_ptr(),
        );
    };
    srand(time(0 as *mut time_t) as libc::c_uint);
    idx = rand() % master_count;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n_0: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if (*n_0).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            continue;
        }
        let fresh59 = idx;
        idx = idx - 1;
        if fresh59 == 0 {
            return n_0;
        }
    }
    if 0 as libc::c_int != 0 {} else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            5182 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"clusterManagerNode *clusterManagerNodeMasterRandom()\0"))
                .as_ptr(),
        );
    };
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn clusterManagerFixSlotsCoverage(
    mut all_slots: *mut libc::c_char,
) -> libc::c_int {
    let mut iter: *mut dictIterator = 0 as *mut dictIterator;
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut ignore_force: libc::c_int = 0;
    let mut current_block: u64;
    let mut force_fix: libc::c_int = config.cluster_manager_command.flags
        & (1 as libc::c_int) << 10 as libc::c_int;
    if cluster_manager.unreachable_masters > 0 as libc::c_int && force_fix == 0 {
        clusterManagerLog(
            2 as libc::c_int,
            b"*** Fixing slots coverage with %d unreachable masters is dangerous: redis-cli will assume that slots about masters that are not reachable are not covered, and will try to reassign them to the reachable nodes. This can cause data loss and is rarely what you want to do. If you really want to proceed use the --cluster-fix-with-unreachable-masters option.\n\0"
                as *const u8 as *const libc::c_char,
            cluster_manager.unreachable_masters,
        );
        exit(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0;
    let mut fixed: libc::c_int = 0 as libc::c_int;
    let mut none: *mut list = 0 as *mut list;
    let mut single: *mut list = 0 as *mut list;
    let mut multi: *mut list = 0 as *mut list;
    clusterManagerLog(
        1 as libc::c_int,
        b">>> Fixing slots coverage...\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    's_24: loop {
        if !(i < 16384 as libc::c_int) {
            current_block = 9828876828309294594;
            break;
        }
        let mut covered: libc::c_int = *all_slots.offset(i as isize) as libc::c_int;
        if covered == 0 {
            let mut slot: hisds = hi_sdsfromlonglong(i as libc::c_longlong);
            let mut slot_nodes: *mut list = listCreate();
            let mut slot_nodes_str: hisds = hi_sdsempty();
            let mut li: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            let mut ln: *mut listNode = 0 as *mut listNode;
            listRewind(cluster_manager.nodes, &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut n: *mut clusterManagerNode = (*ln).value
                    as *mut clusterManagerNode;
                if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
                    || !((*n).replicate).is_null()
                {
                    continue;
                }
                let mut reply: *mut redisReply = redisCommand(
                    (*n).context,
                    b"CLUSTER GETKEYSINSLOT %d %d\0" as *const u8 as *const libc::c_char,
                    i,
                    1 as libc::c_int,
                ) as *mut redisReply;
                if clusterManagerCheckRedisReply(n, reply, 0 as *mut *mut libc::c_char)
                    == 0
                {
                    fixed = -(1 as libc::c_int);
                    if !reply.is_null() {
                        freeReplyObject(reply as *mut libc::c_void);
                    }
                    current_block = 3891736614445177135;
                    break 's_24;
                } else {
                    if (*reply).type_0 == 2 as libc::c_int {} else {
                        __assert_fail(
                            b"reply->type == REDIS_REPLY_ARRAY\0" as *const u8
                                as *const libc::c_char,
                            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                            5217 as libc::c_int as libc::c_uint,
                            (*core::mem::transmute::<
                                &[u8; 43],
                                &[libc::c_char; 43],
                            >(b"int clusterManagerFixSlotsCoverage(char *)\0"))
                                .as_ptr(),
                        );
                    };
                    if (*reply).elements > 0 as libc::c_int as libc::c_ulong {
                        listAddNodeTail(slot_nodes, n as *mut libc::c_void);
                        if (*slot_nodes).len > 1 as libc::c_int as libc::c_ulong {
                            slot_nodes_str = hi_sdscat(
                                slot_nodes_str,
                                b", \0" as *const u8 as *const libc::c_char,
                            );
                        }
                        slot_nodes_str = hi_sdscatfmt(
                            slot_nodes_str,
                            b"%s:%u\0" as *const u8 as *const libc::c_char,
                            (*n).ip,
                            (*n).port,
                        );
                    }
                    freeReplyObject(reply as *mut libc::c_void);
                }
            }
            hi_sdsfree(slot_nodes_str);
            dictAdd(
                clusterManagerUncoveredSlots,
                slot as *mut libc::c_void,
                slot_nodes as *mut libc::c_void,
            );
        }
        i += 1;
    }
    match current_block {
        9828876828309294594 => {
            none = listCreate();
            single = listCreate();
            multi = listCreate();
            iter = dictGetIterator(clusterManagerUncoveredSlots);
            entry = 0 as *mut dictEntry;
            loop {
                entry = dictNext(iter);
                if entry.is_null() {
                    break;
                }
                let mut slot_0: hisds = (*entry).key as hisds;
                let mut nodes: *mut list = (*entry).v.val as *mut list;
                match (*nodes).len {
                    0 => {
                        listAddNodeTail(none, slot_0 as *mut libc::c_void);
                    }
                    1 => {
                        listAddNodeTail(single, slot_0 as *mut libc::c_void);
                    }
                    _ => {
                        listAddNodeTail(multi, slot_0 as *mut libc::c_void);
                    }
                }
            }
            dictReleaseIterator(iter);
            ignore_force = 1 as libc::c_int;
            if (*none).len > 0 as libc::c_int as libc::c_ulong {
                printf(
                    b"The following uncovered slots have no keys across the cluster:\n\0"
                        as *const u8 as *const libc::c_char,
                );
                clusterManagerPrintSlotsList(none);
                if confirmWithYes(
                    b"Fix these slots by covering with a random node?\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    ignore_force,
                ) != 0
                {
                    let mut li_0: listIter = listIter {
                        next: 0 as *mut listNode,
                        direction: 0,
                    };
                    let mut ln_0: *mut listNode = 0 as *mut listNode;
                    listRewind(none, &mut li_0);
                    loop {
                        ln_0 = listNext(&mut li_0);
                        if ln_0.is_null() {
                            current_block = 16738040538446813684;
                            break;
                        }
                        let mut slot_1: hisds = (*ln_0).value as hisds;
                        let mut s: libc::c_int = atoi(slot_1 as *const libc::c_char);
                        let mut n_0: *mut clusterManagerNode = clusterManagerNodeMasterRandom();
                        clusterManagerLog(
                            1 as libc::c_int,
                            b">>> Covering slot %s with %s:%d\n\0" as *const u8
                                as *const libc::c_char,
                            slot_1,
                            (*n_0).ip,
                            (*n_0).port,
                        );
                        if clusterManagerSetSlotOwner(n_0, s, 0 as libc::c_int) == 0 {
                            fixed = -(1 as libc::c_int);
                            current_block = 3891736614445177135;
                            break;
                        } else {
                            (*n_0).slots[s as usize] = 1 as libc::c_int as uint8_t;
                            fixed += 1;
                        }
                    }
                } else {
                    current_block = 16738040538446813684;
                }
            } else {
                current_block = 16738040538446813684;
            }
            match current_block {
                3891736614445177135 => {}
                _ => {
                    if (*single).len > 0 as libc::c_int as libc::c_ulong {
                        printf(
                            b"The following uncovered slots have keys in just one node:\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        clusterManagerPrintSlotsList(single);
                        if confirmWithYes(
                            b"Fix these slots by covering with those nodes?\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            ignore_force,
                        ) != 0
                        {
                            let mut li_1: listIter = listIter {
                                next: 0 as *mut listNode,
                                direction: 0,
                            };
                            let mut ln_1: *mut listNode = 0 as *mut listNode;
                            listRewind(single, &mut li_1);
                            loop {
                                ln_1 = listNext(&mut li_1);
                                if ln_1.is_null() {
                                    current_block = 14001958660280927786;
                                    break;
                                }
                                let mut slot_2: hisds = (*ln_1).value as hisds;
                                let mut s_0: libc::c_int = atoi(
                                    slot_2 as *const libc::c_char,
                                );
                                let mut entry_0: *mut dictEntry = dictFind(
                                    clusterManagerUncoveredSlots,
                                    slot_2 as *const libc::c_void,
                                );
                                if !entry_0.is_null() {} else {
                                    __assert_fail(
                                        b"entry != NULL\0" as *const u8 as *const libc::c_char,
                                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                        5296 as libc::c_int as libc::c_uint,
                                        (*core::mem::transmute::<
                                            &[u8; 43],
                                            &[libc::c_char; 43],
                                        >(b"int clusterManagerFixSlotsCoverage(char *)\0"))
                                            .as_ptr(),
                                    );
                                };
                                let mut nodes_0: *mut list = (*entry_0).v.val as *mut list;
                                let mut fn_0: *mut listNode = (*nodes_0).head;
                                if !fn_0.is_null() {} else {
                                    __assert_fail(
                                        b"fn != NULL\0" as *const u8 as *const libc::c_char,
                                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                        5299 as libc::c_int as libc::c_uint,
                                        (*core::mem::transmute::<
                                            &[u8; 43],
                                            &[libc::c_char; 43],
                                        >(b"int clusterManagerFixSlotsCoverage(char *)\0"))
                                            .as_ptr(),
                                    );
                                };
                                let mut n_1: *mut clusterManagerNode = (*fn_0).value
                                    as *mut clusterManagerNode;
                                clusterManagerLog(
                                    1 as libc::c_int,
                                    b">>> Covering slot %s with %s:%d\n\0" as *const u8
                                        as *const libc::c_char,
                                    slot_2,
                                    (*n_1).ip,
                                    (*n_1).port,
                                );
                                if clusterManagerSetSlotOwner(n_1, s_0, 0 as libc::c_int)
                                    == 0
                                {
                                    fixed = -(1 as libc::c_int);
                                    current_block = 3891736614445177135;
                                    break;
                                } else {
                                    (*n_1)
                                        .slots[atoi(slot_2 as *const libc::c_char)
                                        as usize] = 1 as libc::c_int as uint8_t;
                                    fixed += 1;
                                }
                            }
                        } else {
                            current_block = 14001958660280927786;
                        }
                    } else {
                        current_block = 14001958660280927786;
                    }
                    match current_block {
                        3891736614445177135 => {}
                        _ => {
                            if (*multi).len > 0 as libc::c_int as libc::c_ulong {
                                printf(
                                    b"The following uncovered slots have keys in multiple nodes:\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                clusterManagerPrintSlotsList(multi);
                                if confirmWithYes(
                                    b"Fix these slots by moving keys into a single node?\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    ignore_force,
                                ) != 0
                                {
                                    let mut li_2: listIter = listIter {
                                        next: 0 as *mut listNode,
                                        direction: 0,
                                    };
                                    let mut ln_2: *mut listNode = 0 as *mut listNode;
                                    listRewind(multi, &mut li_2);
                                    's_338: loop {
                                        ln_2 = listNext(&mut li_2);
                                        if ln_2.is_null() {
                                            break;
                                        }
                                        let mut slot_3: hisds = (*ln_2).value as hisds;
                                        let mut entry_1: *mut dictEntry = dictFind(
                                            clusterManagerUncoveredSlots,
                                            slot_3 as *const libc::c_void,
                                        );
                                        if !entry_1.is_null() {} else {
                                            __assert_fail(
                                                b"entry != NULL\0" as *const u8 as *const libc::c_char,
                                                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                                5327 as libc::c_int as libc::c_uint,
                                                (*core::mem::transmute::<
                                                    &[u8; 43],
                                                    &[libc::c_char; 43],
                                                >(b"int clusterManagerFixSlotsCoverage(char *)\0"))
                                                    .as_ptr(),
                                            );
                                        };
                                        let mut nodes_1: *mut list = (*entry_1).v.val as *mut list;
                                        let mut s_1: libc::c_int = atoi(
                                            slot_3 as *const libc::c_char,
                                        );
                                        let mut target: *mut clusterManagerNode = clusterManagerGetNodeWithMostKeysInSlot(
                                            nodes_1,
                                            s_1,
                                            0 as *mut *mut libc::c_char,
                                        );
                                        if target.is_null() {
                                            fixed = -(1 as libc::c_int);
                                            break;
                                        } else {
                                            clusterManagerLog(
                                                1 as libc::c_int,
                                                b">>> Covering slot %s moving keys to %s:%d\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                slot_3,
                                                (*target).ip,
                                                (*target).port,
                                            );
                                            if clusterManagerSetSlotOwner(target, s_1, 1 as libc::c_int)
                                                == 0
                                            {
                                                fixed = -(1 as libc::c_int);
                                                break;
                                            } else {
                                                (*target)
                                                    .slots[atoi(slot_3 as *const libc::c_char)
                                                    as usize] = 1 as libc::c_int as uint8_t;
                                                let mut nli: listIter = listIter {
                                                    next: 0 as *mut listNode,
                                                    direction: 0,
                                                };
                                                let mut nln: *mut listNode = 0 as *mut listNode;
                                                listRewind(nodes_1, &mut nli);
                                                loop {
                                                    nln = listNext(&mut nli);
                                                    if nln.is_null() {
                                                        break;
                                                    }
                                                    let mut src: *mut clusterManagerNode = (*nln).value
                                                        as *mut clusterManagerNode;
                                                    if src == target {
                                                        continue;
                                                    }
                                                    if clusterManagerSetSlot(
                                                        src,
                                                        target,
                                                        s_1,
                                                        b"NODE\0" as *const u8 as *const libc::c_char,
                                                        0 as *mut *mut libc::c_char,
                                                    ) == 0
                                                    {
                                                        fixed = -(1 as libc::c_int);
                                                    }
                                                    if fixed < 0 as libc::c_int {
                                                        break 's_338;
                                                    }
                                                    if clusterManagerSetSlot(
                                                        src,
                                                        target,
                                                        s_1,
                                                        b"IMPORTING\0" as *const u8 as *const libc::c_char,
                                                        0 as *mut *mut libc::c_char,
                                                    ) == 0
                                                    {
                                                        fixed = -(1 as libc::c_int);
                                                    }
                                                    if fixed < 0 as libc::c_int {
                                                        break 's_338;
                                                    }
                                                    let mut opts: libc::c_int = (1 as libc::c_int)
                                                        << 7 as libc::c_int
                                                        | (1 as libc::c_int) << 1 as libc::c_int;
                                                    if clusterManagerMoveSlot(
                                                        src,
                                                        target,
                                                        s_1,
                                                        opts,
                                                        0 as *mut *mut libc::c_char,
                                                    ) == 0
                                                    {
                                                        fixed = -(1 as libc::c_int);
                                                        break 's_338;
                                                    } else {
                                                        if clusterManagerClearSlotStatus(src, s_1) == 0 {
                                                            fixed = -(1 as libc::c_int);
                                                        }
                                                        if fixed < 0 as libc::c_int {
                                                            break 's_338;
                                                        }
                                                    }
                                                }
                                                fixed += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !none.is_null() {
        listRelease(none);
    }
    if !single.is_null() {
        listRelease(single);
    }
    if !multi.is_null() {
        listRelease(multi);
    }
    return fixed;
}
unsafe extern "C" fn clusterManagerFixOpenSlot(mut slot: libc::c_int) -> libc::c_int {
    let mut try_to_close_slot: libc::c_int = 0;
    let mut move_opts: libc::c_int = 0;
    let mut current_block: u64;
    let mut force_fix: libc::c_int = config.cluster_manager_command.flags
        & (1 as libc::c_int) << 10 as libc::c_int;
    if cluster_manager.unreachable_masters > 0 as libc::c_int && force_fix == 0 {
        clusterManagerLog(
            2 as libc::c_int,
            b"*** Fixing open slots with %d unreachable masters is dangerous: redis-cli will assume that slots about masters that are not reachable are not covered, and will try to reassign them to the reachable nodes. This can cause data loss and is rarely what you want to do. If you really want to proceed use the --cluster-fix-with-unreachable-masters option.\n\0"
                as *const u8 as *const libc::c_char,
            cluster_manager.unreachable_masters,
        );
        exit(1 as libc::c_int);
    }
    clusterManagerLog(
        1 as libc::c_int,
        b">>> Fixing open slot %d\n\0" as *const u8 as *const libc::c_char,
        slot,
    );
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut owners: *mut list = listCreate();
    let mut migrating: *mut list = listCreate();
    let mut importing: *mut list = listCreate();
    let mut migrating_str: hisds = hi_sdsempty();
    let mut importing_str: hisds = hi_sdsempty();
    let mut owner: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            current_block = 4808432441040389987;
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            continue;
        }
        if (*n).slots[slot as usize] != 0 {
            listAddNodeTail(owners, n as *mut libc::c_void);
        } else {
            let mut r: *mut redisReply = redisCommand(
                (*n).context,
                b"CLUSTER COUNTKEYSINSLOT %d\0" as *const u8 as *const libc::c_char,
                slot,
            ) as *mut redisReply;
            success = clusterManagerCheckRedisReply(n, r, 0 as *mut *mut libc::c_char);
            if success != 0 && (*r).integer > 0 as libc::c_int as libc::c_longlong {
                clusterManagerLog(
                    2 as libc::c_int,
                    b"*** Found keys about slot %d in non-owner node %s:%d!\n\0"
                        as *const u8 as *const libc::c_char,
                    slot,
                    (*n).ip,
                    (*n).port,
                );
                listAddNodeTail(owners, n as *mut libc::c_void);
            }
            if !r.is_null() {
                freeReplyObject(r as *mut libc::c_void);
            }
            if success == 0 {
                current_block = 10287032174686567;
                break;
            }
        }
    }
    match current_block {
        4808432441040389987 => {
            if (*owners).len == 1 as libc::c_int as libc::c_ulong {
                owner = (*(*owners).head).value as *mut clusterManagerNode;
            }
            listRewind(cluster_manager.nodes, &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    current_block = 10095721787123848864;
                    break;
                }
                let mut n_0: *mut clusterManagerNode = (*ln).value
                    as *mut clusterManagerNode;
                if (*n_0).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                    continue;
                }
                let mut is_migrating: libc::c_int = 0 as libc::c_int;
                let mut is_importing: libc::c_int = 0 as libc::c_int;
                if !((*n_0).migrating).is_null() {
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i < (*n_0).migrating_count {
                        let mut migrating_slot: hisds = *((*n_0).migrating)
                            .offset(i as isize);
                        if atoi(migrating_slot as *const libc::c_char) == slot {
                            let mut sep: *mut libc::c_char = (if (*migrating).len
                                == 0 as libc::c_int as libc::c_ulong
                            {
                                b"\0" as *const u8 as *const libc::c_char
                            } else {
                                b",\0" as *const u8 as *const libc::c_char
                            }) as *mut libc::c_char;
                            migrating_str = hi_sdscatfmt(
                                migrating_str,
                                b"%s%s:%u\0" as *const u8 as *const libc::c_char,
                                sep,
                                (*n_0).ip,
                                (*n_0).port,
                            );
                            listAddNodeTail(migrating, n_0 as *mut libc::c_void);
                            is_migrating = 1 as libc::c_int;
                            break;
                        } else {
                            i += 2 as libc::c_int;
                        }
                    }
                }
                if is_migrating == 0 && !((*n_0).importing).is_null() {
                    let mut i_0: libc::c_int = 0 as libc::c_int;
                    while i_0 < (*n_0).importing_count {
                        let mut importing_slot: hisds = *((*n_0).importing)
                            .offset(i_0 as isize);
                        if atoi(importing_slot as *const libc::c_char) == slot {
                            let mut sep_0: *mut libc::c_char = (if (*importing).len
                                == 0 as libc::c_int as libc::c_ulong
                            {
                                b"\0" as *const u8 as *const libc::c_char
                            } else {
                                b",\0" as *const u8 as *const libc::c_char
                            }) as *mut libc::c_char;
                            importing_str = hi_sdscatfmt(
                                importing_str,
                                b"%s%s:%u\0" as *const u8 as *const libc::c_char,
                                sep_0,
                                (*n_0).ip,
                                (*n_0).port,
                            );
                            listAddNodeTail(importing, n_0 as *mut libc::c_void);
                            is_importing = 1 as libc::c_int;
                            break;
                        } else {
                            i_0 += 2 as libc::c_int;
                        }
                    }
                }
                if !(is_migrating == 0 && is_importing == 0 && n_0 != owner) {
                    continue;
                }
                let mut r_0: *mut redisReply = redisCommand(
                    (*n_0).context,
                    b"CLUSTER COUNTKEYSINSLOT %d\0" as *const u8 as *const libc::c_char,
                    slot,
                ) as *mut redisReply;
                success = clusterManagerCheckRedisReply(
                    n_0,
                    r_0,
                    0 as *mut *mut libc::c_char,
                );
                if success != 0 && (*r_0).integer > 0 as libc::c_int as libc::c_longlong
                {
                    clusterManagerLog(
                        2 as libc::c_int,
                        b"*** Found keys about slot %d in node %s:%d!\n\0" as *const u8
                            as *const libc::c_char,
                        slot,
                        (*n_0).ip,
                        (*n_0).port,
                    );
                    let mut sep_1: *mut libc::c_char = (if (*importing).len
                        == 0 as libc::c_int as libc::c_ulong
                    {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b",\0" as *const u8 as *const libc::c_char
                    }) as *mut libc::c_char;
                    importing_str = hi_sdscatfmt(
                        importing_str,
                        b"%s%s:%u\0" as *const u8 as *const libc::c_char,
                        sep_1,
                        (*n_0).ip,
                        (*n_0).port,
                    );
                    listAddNodeTail(importing, n_0 as *mut libc::c_void);
                }
                if !r_0.is_null() {
                    freeReplyObject(r_0 as *mut libc::c_void);
                }
                if success == 0 {
                    current_block = 10287032174686567;
                    break;
                }
            }
            match current_block {
                10287032174686567 => {}
                _ => {
                    if hi_sdslen(migrating_str) > 0 as libc::c_int as libc::c_ulong {
                        printf(
                            b"Set as migrating in: %s\n\0" as *const u8
                                as *const libc::c_char,
                            migrating_str,
                        );
                    }
                    if hi_sdslen(importing_str) > 0 as libc::c_int as libc::c_ulong {
                        printf(
                            b"Set as importing in: %s\n\0" as *const u8
                                as *const libc::c_char,
                            importing_str,
                        );
                    }
                    if owner.is_null() {
                        clusterManagerLog(
                            1 as libc::c_int,
                            b">>> No single clear owner for the slot, selecting an owner by # of keys...\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        owner = clusterManagerGetNodeWithMostKeysInSlot(
                            cluster_manager.nodes,
                            slot,
                            0 as *mut *mut libc::c_char,
                        );
                        if owner.is_null() {
                            clusterManagerLog(
                                3 as libc::c_int,
                                b"[ERR] Can't select a slot owner. Impossible to fix.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            success = 0 as libc::c_int;
                            current_block = 10287032174686567;
                        } else {
                            clusterManagerLog(
                                2 as libc::c_int,
                                b"*** Configuring %s:%d as the slot owner\n\0" as *const u8
                                    as *const libc::c_char,
                                (*owner).ip,
                                (*owner).port,
                            );
                            success = clusterManagerClearSlotStatus(owner, slot);
                            if success == 0 {
                                current_block = 10287032174686567;
                            } else {
                                success = clusterManagerSetSlotOwner(
                                    owner,
                                    slot,
                                    0 as libc::c_int,
                                );
                                if success == 0 {
                                    current_block = 10287032174686567;
                                } else {
                                    (*owner).slots[slot as usize] = 1 as libc::c_int as uint8_t;
                                    clusterManagerRemoveNodeFromList(migrating, owner);
                                    clusterManagerRemoveNodeFromList(importing, owner);
                                    current_block = 981995395831942902;
                                }
                            }
                        }
                    } else {
                        current_block = 981995395831942902;
                    }
                    match current_block {
                        10287032174686567 => {}
                        _ => {
                            if (*owners).len > 1 as libc::c_int as libc::c_ulong {
                                if !owner.is_null() {} else {
                                    __assert_fail(
                                        b"owner != NULL\0" as *const u8 as *const libc::c_char,
                                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                        5540 as libc::c_int as libc::c_uint,
                                        (*core::mem::transmute::<
                                            &[u8; 35],
                                            &[libc::c_char; 35],
                                        >(b"int clusterManagerFixOpenSlot(int)\0"))
                                            .as_ptr(),
                                    );
                                };
                                listRewind(owners, &mut li);
                                loop {
                                    ln = listNext(&mut li);
                                    if ln.is_null() {
                                        current_block = 1352918242886884122;
                                        break;
                                    }
                                    let mut n_1: *mut clusterManagerNode = (*ln).value
                                        as *mut clusterManagerNode;
                                    if n_1 == owner {
                                        continue;
                                    }
                                    success = clusterManagerDelSlot(
                                        n_1,
                                        slot,
                                        1 as libc::c_int,
                                    );
                                    if success == 0 {
                                        current_block = 10287032174686567;
                                        break;
                                    }
                                    (*n_1).slots[slot as usize] = 0 as libc::c_int as uint8_t;
                                    success = clusterManagerSetSlot(
                                        n_1,
                                        owner,
                                        slot,
                                        b"node\0" as *const u8 as *const libc::c_char,
                                        0 as *mut *mut libc::c_char,
                                    );
                                    if success == 0 {
                                        current_block = 10287032174686567;
                                        break;
                                    }
                                    success = clusterManagerSetSlot(
                                        n_1,
                                        owner,
                                        slot,
                                        b"importing\0" as *const u8 as *const libc::c_char,
                                        0 as *mut *mut libc::c_char,
                                    );
                                    if success == 0 {
                                        current_block = 10287032174686567;
                                        break;
                                    }
                                    clusterManagerRemoveNodeFromList(importing, n_1);
                                    listAddNodeTail(importing, n_1 as *mut libc::c_void);
                                    clusterManagerRemoveNodeFromList(migrating, n_1);
                                }
                            } else {
                                current_block = 1352918242886884122;
                            }
                            match current_block {
                                10287032174686567 => {}
                                _ => {
                                    move_opts = (1 as libc::c_int) << 7 as libc::c_int;
                                    if (*migrating).len == 1 as libc::c_int as libc::c_ulong
                                        && (*importing).len == 1 as libc::c_int as libc::c_ulong
                                    {
                                        let mut src: *mut clusterManagerNode = (*(*migrating).head)
                                            .value as *mut clusterManagerNode;
                                        let mut dst: *mut clusterManagerNode = (*(*importing).head)
                                            .value as *mut clusterManagerNode;
                                        clusterManagerLog(
                                            1 as libc::c_int,
                                            b">>> Case 1: Moving slot %d from %s:%d to %s:%d\n\0"
                                                as *const u8 as *const libc::c_char,
                                            slot,
                                            (*src).ip,
                                            (*src).port,
                                            (*dst).ip,
                                            (*dst).port,
                                        );
                                        move_opts |= (1 as libc::c_int) << 2 as libc::c_int;
                                        success = clusterManagerMoveSlot(
                                            src,
                                            dst,
                                            slot,
                                            move_opts,
                                            0 as *mut *mut libc::c_char,
                                        );
                                    } else if (*migrating).len
                                        == 0 as libc::c_int as libc::c_ulong
                                        && (*importing).len > 0 as libc::c_int as libc::c_ulong
                                    {
                                        clusterManagerLog(
                                            1 as libc::c_int,
                                            b">>> Case 2: Moving all the %d slot keys to its owner %s:%d\n\0"
                                                as *const u8 as *const libc::c_char,
                                            slot,
                                            (*owner).ip,
                                            (*owner).port,
                                        );
                                        move_opts |= (1 as libc::c_int) << 1 as libc::c_int;
                                        listRewind(importing, &mut li);
                                        loop {
                                            ln = listNext(&mut li);
                                            if ln.is_null() {
                                                current_block = 5687667889785024198;
                                                break;
                                            }
                                            let mut n_2: *mut clusterManagerNode = (*ln).value
                                                as *mut clusterManagerNode;
                                            if n_2 == owner {
                                                continue;
                                            }
                                            success = clusterManagerMoveSlot(
                                                n_2,
                                                owner,
                                                slot,
                                                move_opts,
                                                0 as *mut *mut libc::c_char,
                                            );
                                            if success == 0 {
                                                current_block = 10287032174686567;
                                                break;
                                            }
                                            clusterManagerLog(
                                                1 as libc::c_int,
                                                b">>> Setting %d as STABLE in %s:%d\n\0" as *const u8
                                                    as *const libc::c_char,
                                                slot,
                                                (*n_2).ip,
                                                (*n_2).port,
                                            );
                                            success = clusterManagerClearSlotStatus(n_2, slot);
                                            if success == 0 {
                                                current_block = 10287032174686567;
                                                break;
                                            }
                                        }
                                        match current_block {
                                            10287032174686567 => {}
                                            _ => {
                                                listRewind(cluster_manager.nodes, &mut li);
                                                loop {
                                                    ln = listNext(&mut li);
                                                    if ln.is_null() {
                                                        break;
                                                    }
                                                    let mut n_3: *mut clusterManagerNode = (*ln).value
                                                        as *mut clusterManagerNode;
                                                    if n_3 == owner {
                                                        continue;
                                                    }
                                                    if (*n_3).flags & (1 as libc::c_int) << 1 as libc::c_int
                                                        != 0
                                                    {
                                                        continue;
                                                    }
                                                    success = clusterManagerSetSlot(
                                                        n_3,
                                                        owner,
                                                        slot,
                                                        b"NODE\0" as *const u8 as *const libc::c_char,
                                                        0 as *mut *mut libc::c_char,
                                                    );
                                                    if success == 0 {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (*migrating).len == 1 as libc::c_int as libc::c_ulong
                                            && (*importing).len > 1 as libc::c_int as libc::c_ulong
                                        {
                                            let mut try_to_fix: libc::c_int = 1 as libc::c_int;
                                            let mut src_0: *mut clusterManagerNode = (*(*migrating)
                                                .head)
                                                .value as *mut clusterManagerNode;
                                            let mut dst_0: *mut clusterManagerNode = 0
                                                as *mut clusterManagerNode;
                                            let mut target_id: hisds = 0 as hisds;
                                            let mut i_1: libc::c_int = 0 as libc::c_int;
                                            while i_1 < (*src_0).migrating_count {
                                                let mut migrating_slot_0: hisds = *((*src_0).migrating)
                                                    .offset(i_1 as isize);
                                                if atoi(migrating_slot_0 as *const libc::c_char) == slot {
                                                    target_id = *((*src_0).migrating)
                                                        .offset((i_1 + 1 as libc::c_int) as isize);
                                                    break;
                                                } else {
                                                    i_1 += 2 as libc::c_int;
                                                }
                                            }
                                            if !target_id.is_null() {} else {
                                                __assert_fail(
                                                    b"target_id != NULL\0" as *const u8 as *const libc::c_char,
                                                    b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                                    5626 as libc::c_int as libc::c_uint,
                                                    (*core::mem::transmute::<
                                                        &[u8; 35],
                                                        &[libc::c_char; 35],
                                                    >(b"int clusterManagerFixOpenSlot(int)\0"))
                                                        .as_ptr(),
                                                );
                                            };
                                            let mut li_0: listIter = listIter {
                                                next: 0 as *mut listNode,
                                                direction: 0,
                                            };
                                            let mut ln_0: *mut listNode = 0 as *mut listNode;
                                            listRewind(importing, &mut li_0);
                                            loop {
                                                ln_0 = listNext(&mut li_0);
                                                if ln_0.is_null() {
                                                    break;
                                                }
                                                let mut n_4: *mut clusterManagerNode = (*ln_0).value
                                                    as *mut clusterManagerNode;
                                                let mut count: libc::c_int = clusterManagerCountKeysInSlot(
                                                    n_4,
                                                    slot,
                                                );
                                                if count > 0 as libc::c_int {
                                                    try_to_fix = 0 as libc::c_int;
                                                    break;
                                                } else if strcmp(
                                                    (*n_4).name as *const libc::c_char,
                                                    target_id as *const libc::c_char,
                                                ) == 0 as libc::c_int
                                                {
                                                    dst_0 = n_4;
                                                }
                                            }
                                            if try_to_fix == 0 {
                                                current_block = 12137613406420127454;
                                            } else {
                                                if !dst_0.is_null() {
                                                    clusterManagerLog(
                                                        1 as libc::c_int,
                                                        b">>> Case 3: Moving slot %d from %s:%d to %s:%d and closing it on all the other importing nodes.\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        slot,
                                                        (*src_0).ip,
                                                        (*src_0).port,
                                                        (*dst_0).ip,
                                                        (*dst_0).port,
                                                    );
                                                    success = clusterManagerMoveSlot(
                                                        src_0,
                                                        dst_0,
                                                        slot,
                                                        move_opts,
                                                        0 as *mut *mut libc::c_char,
                                                    );
                                                    if !(success == 0) {
                                                        listRewind(importing, &mut li_0);
                                                        loop {
                                                            ln_0 = listNext(&mut li_0);
                                                            if ln_0.is_null() {
                                                                break;
                                                            }
                                                            let mut n_5: *mut clusterManagerNode = (*ln_0).value
                                                                as *mut clusterManagerNode;
                                                            if dst_0 == n_5 {
                                                                continue;
                                                            }
                                                            success = clusterManagerClearSlotStatus(n_5, slot);
                                                            if success == 0 {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    clusterManagerLog(
                                                        1 as libc::c_int,
                                                        b">>> Case 3: Closing slot %d on both migrating and importing nodes.\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        slot,
                                                    );
                                                    success = clusterManagerClearSlotStatus(src_0, slot);
                                                    if !(success == 0) {
                                                        listRewind(importing, &mut li_0);
                                                        loop {
                                                            ln_0 = listNext(&mut li_0);
                                                            if ln_0.is_null() {
                                                                break;
                                                            }
                                                            let mut n_6: *mut clusterManagerNode = (*ln_0).value
                                                                as *mut clusterManagerNode;
                                                            success = clusterManagerClearSlotStatus(n_6, slot);
                                                            if success == 0 {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                current_block = 10287032174686567;
                                            }
                                        } else {
                                            try_to_close_slot = ((*importing).len
                                                == 0 as libc::c_int as libc::c_ulong
                                                && (*migrating).len == 1 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            if try_to_close_slot != 0 {
                                                let mut n_7: *mut clusterManagerNode = (*(*migrating).head)
                                                    .value as *mut clusterManagerNode;
                                                if owner.is_null() || owner != n_7 {
                                                    let mut r_1: *mut redisReply = redisCommand(
                                                        (*n_7).context,
                                                        b"CLUSTER GETKEYSINSLOT %d %d\0" as *const u8
                                                            as *const libc::c_char,
                                                        slot,
                                                        10 as libc::c_int,
                                                    ) as *mut redisReply;
                                                    success = clusterManagerCheckRedisReply(
                                                        n_7,
                                                        r_1,
                                                        0 as *mut *mut libc::c_char,
                                                    );
                                                    if !r_1.is_null() {
                                                        if success != 0 {
                                                            try_to_close_slot = ((*r_1).elements
                                                                == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
                                                        }
                                                        freeReplyObject(r_1 as *mut libc::c_void);
                                                    }
                                                    if success == 0 {
                                                        current_block = 10287032174686567;
                                                    } else {
                                                        current_block = 16077153431071379266;
                                                    }
                                                } else {
                                                    current_block = 16077153431071379266;
                                                }
                                            } else {
                                                current_block = 16077153431071379266;
                                            }
                                            match current_block {
                                                10287032174686567 => {}
                                                _ => {
                                                    if try_to_close_slot != 0 {
                                                        let mut n_8: *mut clusterManagerNode = (*(*migrating).head)
                                                            .value as *mut clusterManagerNode;
                                                        clusterManagerLog(
                                                            1 as libc::c_int,
                                                            b">>> Case 4: Closing slot %d on %s:%d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            slot,
                                                            (*n_8).ip,
                                                            (*n_8).port,
                                                        );
                                                        let mut r_2: *mut redisReply = redisCommand(
                                                            (*n_8).context,
                                                            b"CLUSTER SETSLOT %d %s\0" as *const u8
                                                                as *const libc::c_char,
                                                            slot,
                                                            b"STABLE\0" as *const u8 as *const libc::c_char,
                                                        ) as *mut redisReply;
                                                        success = clusterManagerCheckRedisReply(
                                                            n_8,
                                                            r_2,
                                                            0 as *mut *mut libc::c_char,
                                                        );
                                                        if !r_2.is_null() {
                                                            freeReplyObject(r_2 as *mut libc::c_void);
                                                        }
                                                        success == 0;
                                                        current_block = 10287032174686567;
                                                    } else {
                                                        current_block = 12137613406420127454;
                                                    }
                                                }
                                            }
                                        }
                                        match current_block {
                                            10287032174686567 => {}
                                            _ => {
                                                success = 0 as libc::c_int;
                                                clusterManagerLog(
                                                    3 as libc::c_int,
                                                    b"[ERR] Sorry, redis-cli can't fix this slot yet (work in progress). Slot is set as migrating in %s, as importing in %s, owner is %s:%d\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    migrating_str,
                                                    importing_str,
                                                    (*owner).ip,
                                                    (*owner).port,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    listRelease(owners);
    listRelease(migrating);
    listRelease(importing);
    hi_sdsfree(migrating_str);
    hi_sdsfree(importing_str);
    return success;
}
unsafe extern "C" fn clusterManagerFixMultipleSlotOwners(
    mut slot: libc::c_int,
    mut owners: *mut list,
) -> libc::c_int {
    clusterManagerLog(
        1 as libc::c_int,
        b">>> Fixing multiple owners for slot %d...\n\0" as *const u8
            as *const libc::c_char,
        slot,
    );
    let mut success: libc::c_int = 0 as libc::c_int;
    if (*owners).len > 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"listLength(owners) > 1\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            5722 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"int clusterManagerFixMultipleSlotOwners(int, list *)\0"))
                .as_ptr(),
        );
    };
    let mut owner: *mut clusterManagerNode = clusterManagerGetNodeWithMostKeysInSlot(
        owners,
        slot,
        0 as *mut *mut libc::c_char,
    );
    if owner.is_null() {
        owner = (*(*owners).head).value as *mut clusterManagerNode;
    }
    clusterManagerLog(
        1 as libc::c_int,
        b">>> Setting slot %d owner: %s:%d\n\0" as *const u8 as *const libc::c_char,
        slot,
        (*owner).ip,
        (*owner).port,
    );
    if clusterManagerSetSlotOwner(owner, slot, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if n == owner {
            continue;
        }
        if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            continue;
        }
        let mut count: libc::c_int = clusterManagerCountKeysInSlot(n, slot);
        success = (count >= 0 as libc::c_int) as libc::c_int;
        if success == 0 {
            break;
        }
        clusterManagerDelSlot(n, slot, 1 as libc::c_int);
        if clusterManagerSetSlot(
            n,
            owner,
            slot,
            b"node\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut libc::c_char,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if !(count > 0 as libc::c_int) {
            continue;
        }
        let mut opts: libc::c_int = (1 as libc::c_int) << 7 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int;
        success = clusterManagerMoveSlot(
            n,
            owner,
            slot,
            opts,
            0 as *mut *mut libc::c_char,
        );
        if success == 0 {
            break;
        }
    }
    return success;
}
unsafe extern "C" fn clusterManagerCheckCluster(mut quiet: libc::c_int) -> libc::c_int {
    let mut ln: *mut listNode = (*cluster_manager.nodes).head;
    if ln.is_null() {
        return 0 as libc::c_int;
    }
    let mut node: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
    clusterManagerLog(
        1 as libc::c_int,
        b">>> Performing Cluster Check (using node %s:%d)\n\0" as *const u8
            as *const libc::c_char,
        (*node).ip,
        (*node).port,
    );
    let mut result: libc::c_int = 1 as libc::c_int;
    let mut consistent: libc::c_int = 0 as libc::c_int;
    let mut do_fix: libc::c_int = config.cluster_manager_command.flags
        & (1 as libc::c_int) << 0 as libc::c_int;
    if quiet == 0 {
        clusterManagerShowNodes();
    }
    consistent = clusterManagerIsConfigConsistent();
    if consistent == 0 {
        let mut err: hisds = hi_sdsnew(
            b"[ERR] Nodes don't agree about configuration!\0" as *const u8
                as *const libc::c_char,
        );
        clusterManagerOnError(err);
        result = 0 as libc::c_int;
    } else {
        clusterManagerLog(
            4 as libc::c_int,
            b"[OK] All nodes agree about slots configuration.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    clusterManagerLog(
        1 as libc::c_int,
        b">>> Check for open slots...\n\0" as *const u8 as *const libc::c_char,
    );
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewind(cluster_manager.nodes, &mut li);
    let mut i: libc::c_int = 0;
    let mut open_slots: *mut dict = 0 as *mut dict;
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        if !((*n).migrating).is_null() {
            if open_slots.is_null() {
                open_slots = dictCreate(&mut clusterManagerDictType);
            }
            let mut errstr: hisds = hi_sdsempty();
            errstr = hi_sdscatprintf(
                errstr,
                b"[WARNING] Node %s:%d has slots in migrating state \0" as *const u8
                    as *const libc::c_char,
                (*n).ip,
                (*n).port,
            );
            i = 0 as libc::c_int;
            while i < (*n).migrating_count {
                let mut slot: hisds = *((*n).migrating).offset(i as isize);
                dictReplace(
                    open_slots,
                    slot as *mut libc::c_void,
                    hi_sdsdup(*((*n).migrating).offset((i + 1 as libc::c_int) as isize))
                        as *mut libc::c_void,
                );
                let mut fmt: *mut libc::c_char = (if i > 0 as libc::c_int {
                    b",%S\0" as *const u8 as *const libc::c_char
                } else {
                    b"%S\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char;
                errstr = hi_sdscatfmt(errstr, fmt, slot);
                i += 2 as libc::c_int;
            }
            errstr = hi_sdscat(errstr, b".\0" as *const u8 as *const libc::c_char);
            clusterManagerOnError(errstr);
        }
        if !((*n).importing).is_null() {
            if open_slots.is_null() {
                open_slots = dictCreate(&mut clusterManagerDictType);
            }
            let mut errstr_0: hisds = hi_sdsempty();
            errstr_0 = hi_sdscatprintf(
                errstr_0,
                b"[WARNING] Node %s:%d has slots in importing state \0" as *const u8
                    as *const libc::c_char,
                (*n).ip,
                (*n).port,
            );
            i = 0 as libc::c_int;
            while i < (*n).importing_count {
                let mut slot_0: hisds = *((*n).importing).offset(i as isize);
                dictReplace(
                    open_slots,
                    slot_0 as *mut libc::c_void,
                    hi_sdsdup(*((*n).importing).offset((i + 1 as libc::c_int) as isize))
                        as *mut libc::c_void,
                );
                let mut fmt_0: *mut libc::c_char = (if i > 0 as libc::c_int {
                    b",%S\0" as *const u8 as *const libc::c_char
                } else {
                    b"%S\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char;
                errstr_0 = hi_sdscatfmt(errstr_0, fmt_0, slot_0);
                i += 2 as libc::c_int;
            }
            errstr_0 = hi_sdscat(errstr_0, b".\0" as *const u8 as *const libc::c_char);
            clusterManagerOnError(errstr_0);
        }
    }
    if !open_slots.is_null() {
        result = 0 as libc::c_int;
        let mut iter: *mut dictIterator = dictGetIterator(open_slots);
        let mut entry: *mut dictEntry = 0 as *mut dictEntry;
        let mut errstr_1: hisds = hi_sdsnew(
            b"[WARNING] The following slots are open: \0" as *const u8
                as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        loop {
            entry = dictNext(iter);
            if entry.is_null() {
                break;
            }
            let mut slot_1: hisds = (*entry).key as hisds;
            let fresh60 = i;
            i = i + 1;
            let mut fmt_1: *mut libc::c_char = (if fresh60 > 0 as libc::c_int {
                b",%S\0" as *const u8 as *const libc::c_char
            } else {
                b"%S\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
            errstr_1 = hi_sdscatfmt(errstr_1, fmt_1, slot_1);
        }
        clusterManagerLog(
            3 as libc::c_int,
            b"%s.\n\0" as *const u8 as *const libc::c_char,
            errstr_1,
        );
        hi_sdsfree(errstr_1);
        if do_fix != 0 {
            dictReleaseIterator(iter);
            iter = dictGetIterator(open_slots);
            loop {
                entry = dictNext(iter);
                if entry.is_null() {
                    break;
                }
                let mut slot_2: hisds = (*entry).key as hisds;
                result = clusterManagerFixOpenSlot(atoi(slot_2 as *const libc::c_char));
                if result == 0 {
                    break;
                }
            }
        }
        dictReleaseIterator(iter);
        dictRelease(open_slots);
    }
    clusterManagerLog(
        1 as libc::c_int,
        b">>> Check slots coverage...\n\0" as *const u8 as *const libc::c_char,
    );
    let mut slots: [libc::c_char; 16384] = [0; 16384];
    memset(
        slots.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16384 as libc::c_int as libc::c_ulong,
    );
    let mut coverage: libc::c_int = clusterManagerGetCoveredSlots(slots.as_mut_ptr());
    if coverage == 16384 as libc::c_int {
        clusterManagerLog(
            4 as libc::c_int,
            b"[OK] All %d slots covered.\n\0" as *const u8 as *const libc::c_char,
            16384 as libc::c_int,
        );
    } else {
        let mut err_0: hisds = hi_sdsempty();
        err_0 = hi_sdscatprintf(
            err_0,
            b"[ERR] Not all %d slots are covered by nodes.\n\0" as *const u8
                as *const libc::c_char,
            16384 as libc::c_int,
        );
        clusterManagerOnError(err_0);
        result = 0 as libc::c_int;
        if do_fix != 0 {
            let mut dtype: dictType = clusterManagerDictType;
            dtype
                .keyDestructor = Some(
                dictSdsDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            );
            dtype
                .valDestructor = Some(
                dictListDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            );
            clusterManagerUncoveredSlots = dictCreate(&mut dtype);
            let mut fixed: libc::c_int = clusterManagerFixSlotsCoverage(
                slots.as_mut_ptr(),
            );
            if fixed > 0 as libc::c_int {
                result = 1 as libc::c_int;
            }
        }
    }
    let mut search_multiple_owners: libc::c_int = config.cluster_manager_command.flags
        & (1 as libc::c_int) << 9 as libc::c_int;
    if search_multiple_owners != 0 {
        clusterManagerLog(
            1 as libc::c_int,
            b">>> Check for multiple slot owners...\n\0" as *const u8
                as *const libc::c_char,
        );
        let mut slot_3: libc::c_int = 0 as libc::c_int;
        let mut slots_with_multiple_owners: libc::c_int = 0 as libc::c_int;
        while slot_3 < 16384 as libc::c_int {
            let mut li_0: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            let mut ln_0: *mut listNode = 0 as *mut listNode;
            listRewind(cluster_manager.nodes, &mut li_0);
            let mut owners: *mut list = listCreate();
            loop {
                ln_0 = listNext(&mut li_0);
                if ln_0.is_null() {
                    break;
                }
                let mut n_0: *mut clusterManagerNode = (*ln_0).value
                    as *mut clusterManagerNode;
                if (*n_0).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                    continue;
                }
                if (*n_0).slots[slot_3 as usize] != 0 {
                    listAddNodeTail(owners, n_0 as *mut libc::c_void);
                } else {
                    let mut count: libc::c_int = clusterManagerCountKeysInSlot(
                        n_0,
                        slot_3,
                    );
                    if count > 0 as libc::c_int {
                        listAddNodeTail(owners, n_0 as *mut libc::c_void);
                    }
                }
            }
            if (*owners).len > 1 as libc::c_int as libc::c_ulong {
                result = 0 as libc::c_int;
                clusterManagerLog(
                    3 as libc::c_int,
                    b"[WARNING] Slot %d has %d owners:\n\0" as *const u8
                        as *const libc::c_char,
                    slot_3,
                    (*owners).len,
                );
                listRewind(owners, &mut li_0);
                loop {
                    ln_0 = listNext(&mut li_0);
                    if ln_0.is_null() {
                        break;
                    }
                    let mut n_1: *mut clusterManagerNode = (*ln_0).value
                        as *mut clusterManagerNode;
                    clusterManagerLog(
                        3 as libc::c_int,
                        b"    %s:%d\n\0" as *const u8 as *const libc::c_char,
                        (*n_1).ip,
                        (*n_1).port,
                    );
                }
                slots_with_multiple_owners += 1;
                if do_fix != 0 {
                    result = clusterManagerFixMultipleSlotOwners(slot_3, owners);
                    if result == 0 {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"Failed to fix multiple owners for slot %d\n\0" as *const u8
                                as *const libc::c_char,
                            slot_3,
                        );
                        listRelease(owners);
                        break;
                    } else {
                        slots_with_multiple_owners -= 1;
                    }
                }
            }
            listRelease(owners);
            slot_3 += 1;
        }
        if slots_with_multiple_owners == 0 as libc::c_int {
            clusterManagerLog(
                4 as libc::c_int,
                b"[OK] No multiple owners found.\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return result;
}
unsafe extern "C" fn clusterNodeForResharding(
    mut id: *mut libc::c_char,
    mut target: *mut clusterManagerNode,
    mut raise_err: *mut libc::c_int,
) -> *mut clusterManagerNode {
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut invalid_node_msg: *const libc::c_char = b"*** The specified node (%s) is not known or not a master, please retry.\n\0"
        as *const u8 as *const libc::c_char;
    node = clusterManagerNodeByName(id);
    *raise_err = 0 as libc::c_int;
    if node.is_null() || (*node).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        clusterManagerLog(3 as libc::c_int, invalid_node_msg, id);
        *raise_err = 1 as libc::c_int;
        return 0 as *mut clusterManagerNode;
    } else {
        if !target.is_null() {
            if strcmp(
                (*node).name as *const libc::c_char,
                (*target).name as *const libc::c_char,
            ) == 0
            {
                clusterManagerLog(
                    3 as libc::c_int,
                    b"*** It is not possible to use the target node as source node.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return 0 as *mut clusterManagerNode;
            }
        }
    }
    return node;
}
unsafe extern "C" fn clusterManagerComputeReshardTable(
    mut sources: *mut list,
    mut numslots: libc::c_int,
) -> *mut list {
    let mut moved: *mut list = listCreate();
    let mut src_count: libc::c_int = (*sources).len as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut tot_slots: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut sorted: *mut *mut clusterManagerNode = zmalloc(
        (src_count as libc::c_ulong)
            .wrapping_mul(
                core::mem::size_of::<*mut clusterManagerNode>() as libc::c_ulong,
            ),
    ) as *mut *mut clusterManagerNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(sources, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut node: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        tot_slots += (*node).slots_count;
        let fresh61 = i;
        i = i + 1;
        let ref mut fresh62 = *sorted.offset(fresh61 as isize);
        *fresh62 = node;
    }
    qsort(
        sorted as *mut libc::c_void,
        src_count as size_t,
        core::mem::size_of::<*mut clusterManagerNode>() as libc::c_ulong,
        Some(
            clusterManagerSlotCountCompareDesc
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < src_count {
        let mut node_0: *mut clusterManagerNode = *sorted.offset(i as isize);
        let mut n: libc::c_float = numslots as libc::c_float / tot_slots as libc::c_float
            * (*node_0).slots_count as libc::c_float;
        if i == 0 as libc::c_int {
            n = ceil(n as libc::c_double) as libc::c_float;
        } else {
            n = floor(n as libc::c_double) as libc::c_float;
        }
        let mut max: libc::c_int = n as libc::c_int;
        let mut count: libc::c_int = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 16384 as libc::c_int {
            let mut slot: libc::c_int = (*node_0).slots[j as usize] as libc::c_int;
            if !(slot == 0) {
                if count >= max || (*moved).len as libc::c_int >= numslots {
                    break;
                }
                let mut item: *mut clusterManagerReshardTableItem = zmalloc(
                    core::mem::size_of::<clusterManagerReshardTableItem>()
                        as libc::c_ulong,
                ) as *mut clusterManagerReshardTableItem;
                (*item).source = node_0;
                (*item).slot = j;
                listAddNodeTail(moved, item as *mut libc::c_void);
                count += 1;
            }
            j += 1;
        }
        i += 1;
    }
    zfree(sorted as *mut libc::c_void);
    return moved;
}
unsafe extern "C" fn clusterManagerShowReshardTable(mut table: *mut list) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(table, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut item: *mut clusterManagerReshardTableItem = (*ln).value
            as *mut clusterManagerReshardTableItem;
        let mut n: *mut clusterManagerNode = (*item).source;
        printf(
            b"    Moving slot %d from %s\n\0" as *const u8 as *const libc::c_char,
            (*item).slot,
            (*n).name,
        );
    };
}
unsafe extern "C" fn clusterManagerReleaseReshardTable(mut table: *mut list) {
    if !table.is_null() {
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind(table, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut item: *mut clusterManagerReshardTableItem = (*ln).value
                as *mut clusterManagerReshardTableItem;
            zfree(item as *mut libc::c_void);
        }
        listRelease(table);
    }
}
unsafe extern "C" fn clusterManagerLog(
    mut level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut use_colors: libc::c_int = config.cluster_manager_command.flags
        & (1 as libc::c_int) << 8 as libc::c_int;
    if use_colors != 0 {
        printf(b"\x1B[\0" as *const u8 as *const libc::c_char);
        match level {
            1 => {
                printf(b"29;1m\0" as *const u8 as *const libc::c_char);
            }
            2 => {
                printf(b"33;1m\0" as *const u8 as *const libc::c_char);
            }
            3 => {
                printf(b"31;1m\0" as *const u8 as *const libc::c_char);
            }
            4 => {
                printf(b"32;1m\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                printf(b"0m\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    if use_colors != 0 {
        printf(b"\x1B[0m\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn clusterManagerNodeArrayInit(
    mut array: *mut clusterManagerNodeArray,
    mut alloc_len: libc::c_int,
) {
    (*array)
        .nodes = zcalloc(
        (alloc_len as libc::c_ulong)
            .wrapping_mul(
                core::mem::size_of::<*mut clusterManagerNode>() as libc::c_ulong,
            ),
    ) as *mut *mut clusterManagerNode;
    (*array).alloc = (*array).nodes;
    (*array).len = alloc_len;
    (*array).count = 0 as libc::c_int;
}
unsafe extern "C" fn clusterManagerNodeArrayReset(
    mut array: *mut clusterManagerNodeArray,
) {
    if (*array).nodes > (*array).alloc {
        (*array)
            .len = ((*array).nodes).offset_from((*array).alloc) as libc::c_long
            as libc::c_int;
        (*array).nodes = (*array).alloc;
        (*array).count = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*array).len {
            if !(*((*array).nodes).offset(i as isize)).is_null() {
                (*array).count += 1;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn clusterManagerNodeArrayShift(
    mut array: *mut clusterManagerNodeArray,
    mut nodeptr: *mut *mut clusterManagerNode,
) {
    if (*array).len > 0 as libc::c_int {} else {
        __assert_fail(
            b"array->len > 0\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            6050 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"void clusterManagerNodeArrayShift(clusterManagerNodeArray *, clusterManagerNode **)\0",
            ))
                .as_ptr(),
        );
    };
    if !(*(*array).nodes).is_null() {
        (*array).count -= 1;
    }
    *nodeptr = *(*array).nodes;
    (*array).nodes = ((*array).nodes).offset(1);
    (*array).len -= 1;
}
unsafe extern "C" fn clusterManagerNodeArrayAdd(
    mut array: *mut clusterManagerNodeArray,
    mut node: *mut clusterManagerNode,
) {
    if (*array).len > 0 as libc::c_int {} else {
        __assert_fail(
            b"array->len > 0\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            6063 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"void clusterManagerNodeArrayAdd(clusterManagerNodeArray *, clusterManagerNode *)\0",
            ))
                .as_ptr(),
        );
    };
    if !node.is_null() {} else {
        __assert_fail(
            b"node != NULL\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            6064 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"void clusterManagerNodeArrayAdd(clusterManagerNodeArray *, clusterManagerNode *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*array).count < (*array).len {} else {
        __assert_fail(
            b"array->count < array->len\0" as *const u8 as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            6065 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"void clusterManagerNodeArrayAdd(clusterManagerNodeArray *, clusterManagerNode *)\0",
            ))
                .as_ptr(),
        );
    };
    let fresh63 = (*array).count;
    (*array).count = (*array).count + 1;
    let ref mut fresh64 = *((*array).nodes).offset(fresh63 as isize);
    *fresh64 = node;
}
unsafe extern "C" fn clusterManagerPrintNotEmptyNodeError(
    mut node: *mut clusterManagerNode,
    mut err: *mut libc::c_char,
) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if !err.is_null() {
        msg = err;
    } else {
        msg = b"is not empty. Either the node already knows other nodes (check with CLUSTER NODES) or contains some key in database 0.\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    clusterManagerLog(
        3 as libc::c_int,
        b"[ERR] Node %s:%d %s\n\0" as *const u8 as *const libc::c_char,
        (*node).ip,
        (*node).port,
        msg,
    );
}
unsafe extern "C" fn clusterManagerPrintNotClusterNodeError(
    mut node: *mut clusterManagerNode,
    mut err: *mut libc::c_char,
) {
    let mut msg: *mut libc::c_char = (if !err.is_null() {
        err as *const libc::c_char
    } else {
        b"is not configured as a cluster node.\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    clusterManagerLog(
        3 as libc::c_int,
        b"[ERR] Node %s:%d %s\n\0" as *const u8 as *const libc::c_char,
        (*node).ip,
        (*node).port,
        msg,
    );
}
unsafe extern "C" fn clusterManagerMode(
    mut proc_0: Option::<clusterManagerCommandProc>,
) {
    let mut argc: libc::c_int = config.cluster_manager_command.argc;
    let mut argv: *mut *mut libc::c_char = config.cluster_manager_command.argv;
    cluster_manager.nodes = 0 as *mut list;
    let mut success: libc::c_int = proc_0
        .expect("non-null function pointer")(argc, argv);
    if config.stdin_lastarg != 0 {
        zfree(config.cluster_manager_command.argv as *mut libc::c_void);
        hi_sdsfree(config.cluster_manager_command.stdin_arg);
    } else if config.stdin_tag_arg != 0 {
        hi_sdsfree(config.cluster_manager_command.stdin_arg);
    }
    freeClusterManager();
    exit(if success != 0 { 0 as libc::c_int } else { 1 as libc::c_int });
}
unsafe extern "C" fn clusterManagerCommandCreate(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut success: libc::c_int = 1 as libc::c_int;
    cluster_manager.nodes = listCreate();
    i = 0 as libc::c_int;
    while i < argc {
        let mut addr: *mut libc::c_char = *argv.offset(i as isize);
        let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut port: libc::c_int = 0 as libc::c_int;
        if parseClusterNodeAddress(addr, &mut ip, &mut port, 0 as *mut libc::c_int) == 0
        {
            fprintf(
                stderr,
                b"Invalid address format: %s\n\0" as *const u8 as *const libc::c_char,
                addr,
            );
            return 0 as libc::c_int;
        }
        let mut node: *mut clusterManagerNode = clusterManagerNewNode(
            ip,
            port,
            0 as libc::c_int,
        );
        if clusterManagerNodeConnect(node) == 0 {
            freeClusterManagerNode(node);
            return 0 as libc::c_int;
        }
        let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
        if clusterManagerNodeIsCluster(node, &mut err) == 0 {
            clusterManagerPrintNotClusterNodeError(node, err);
            if !err.is_null() {
                zfree(err as *mut libc::c_void);
            }
            freeClusterManagerNode(node);
            return 0 as libc::c_int;
        }
        err = 0 as *mut libc::c_char;
        if clusterManagerNodeLoadInfo(node, 0 as libc::c_int, &mut err) == 0 {
            if !err.is_null() {
                clusterManagerLog(
                    3 as libc::c_int,
                    b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                        as *const libc::c_char,
                    (*node).ip,
                    (*node).port,
                    err,
                );
                zfree(err as *mut libc::c_void);
            }
            freeClusterManagerNode(node);
            return 0 as libc::c_int;
        }
        err = 0 as *mut libc::c_char;
        if clusterManagerNodeIsEmpty(node, &mut err) == 0 {
            clusterManagerPrintNotEmptyNodeError(node, err);
            if !err.is_null() {
                zfree(err as *mut libc::c_void);
            }
            freeClusterManagerNode(node);
            return 0 as libc::c_int;
        }
        listAddNodeTail(cluster_manager.nodes, node as *mut libc::c_void);
        i += 1;
    }
    let mut node_len: libc::c_int = (*cluster_manager.nodes).len as libc::c_int;
    let mut replicas: libc::c_int = config.cluster_manager_command.replicas;
    let mut masters_count: libc::c_int = node_len / (replicas + 1 as libc::c_int);
    if masters_count < 3 as libc::c_int {
        clusterManagerLog(
            3 as libc::c_int,
            b"*** ERROR: Invalid configuration for cluster creation.\n*** Redis Cluster requires at least 3 master nodes.\n*** This is not possible with %d nodes and %d replicas per node.\0"
                as *const u8 as *const libc::c_char,
            node_len,
            replicas,
        );
        clusterManagerLog(
            3 as libc::c_int,
            b"\n*** At least %d nodes are required.\n\0" as *const u8
                as *const libc::c_char,
            3 as libc::c_int * (replicas + 1 as libc::c_int),
        );
        return 0 as libc::c_int;
    }
    clusterManagerLog(
        1 as libc::c_int,
        b">>> Performing hash slots allocation on %d nodes...\n\0" as *const u8
            as *const libc::c_char,
        node_len,
    );
    let mut interleaved_len: libc::c_int = 0 as libc::c_int;
    let mut ip_count: libc::c_int = 0 as libc::c_int;
    let mut interleaved: *mut *mut clusterManagerNode = zcalloc(
        (node_len as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<clusterManagerNode>() as libc::c_ulong),
    ) as *mut *mut clusterManagerNode;
    let mut ips: *mut *mut libc::c_char = zcalloc(
        (node_len as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut ip_nodes: *mut clusterManagerNodeArray = zcalloc(
        (node_len as libc::c_ulong)
            .wrapping_mul(
                core::mem::size_of::<clusterManagerNodeArray>() as libc::c_ulong,
            ),
    ) as *mut clusterManagerNodeArray;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(cluster_manager.nodes, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
        let mut found: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ip_count {
            let mut ip_0: *mut libc::c_char = *ips.offset(i as isize);
            if strcmp(ip_0, (*n).ip) == 0 {
                found = 1 as libc::c_int;
                break;
            } else {
                i += 1;
            }
        }
        if found == 0 {
            let fresh65 = ip_count;
            ip_count = ip_count + 1;
            let ref mut fresh66 = *ips.offset(fresh65 as isize);
            *fresh66 = (*n).ip;
        }
        let mut node_array: *mut clusterManagerNodeArray = &mut *ip_nodes
            .offset(i as isize) as *mut clusterManagerNodeArray;
        if ((*node_array).nodes).is_null() {
            clusterManagerNodeArrayInit(node_array, node_len);
        }
        clusterManagerNodeArrayAdd(node_array, n);
    }
    while interleaved_len < node_len {
        i = 0 as libc::c_int;
        while i < ip_count {
            let mut node_array_0: *mut clusterManagerNodeArray = &mut *ip_nodes
                .offset(i as isize) as *mut clusterManagerNodeArray;
            if (*node_array_0).count > 0 as libc::c_int {
                let mut n_0: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
                clusterManagerNodeArrayShift(node_array_0, &mut n_0);
                let fresh67 = interleaved_len;
                interleaved_len = interleaved_len + 1;
                let ref mut fresh68 = *interleaved.offset(fresh67 as isize);
                *fresh68 = n_0;
            }
            i += 1;
        }
    }
    let mut masters: *mut *mut clusterManagerNode = interleaved;
    interleaved = interleaved.offset(masters_count as isize);
    interleaved_len -= masters_count;
    let mut slots_per_node: libc::c_float = 16384 as libc::c_int as libc::c_float
        / masters_count as libc::c_float;
    let mut first: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut cursor: libc::c_float = 0.0f32;
    i = 0 as libc::c_int;
    while i < masters_count {
        let mut master: *mut clusterManagerNode = *masters.offset(i as isize);
        let mut last: libc::c_long = lround(
            (cursor + slots_per_node - 1 as libc::c_int as libc::c_float)
                as libc::c_double,
        );
        if last > 16384 as libc::c_int as libc::c_long
            || i == masters_count - 1 as libc::c_int
        {
            last = (16384 as libc::c_int - 1 as libc::c_int) as libc::c_long;
        }
        if last < first {
            last = first;
        }
        printf(
            b"Master[%d] -> Slots %ld - %ld\n\0" as *const u8 as *const libc::c_char,
            i,
            first,
            last,
        );
        (*master).slots_count = 0 as libc::c_int;
        j = first as libc::c_int;
        while j as libc::c_long <= last {
            (*master).slots[j as usize] = 1 as libc::c_int as uint8_t;
            (*master).slots_count += 1;
            j += 1;
        }
        (*master).dirty = 1 as libc::c_int;
        first = last + 1 as libc::c_int as libc::c_long;
        cursor += slots_per_node;
        i += 1;
    }
    let mut first_node: *mut clusterManagerNode = *interleaved
        .offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < interleaved_len - 1 as libc::c_int {
        let ref mut fresh69 = *interleaved.offset(i as isize);
        *fresh69 = *interleaved.offset((i + 1 as libc::c_int) as isize);
        i += 1;
    }
    let ref mut fresh70 = *interleaved
        .offset((interleaved_len - 1 as libc::c_int) as isize);
    *fresh70 = first_node;
    let mut assign_unused: libc::c_int = 0 as libc::c_int;
    let mut available_count: libc::c_int = interleaved_len;
    loop {
        i = 0 as libc::c_int;
        while i < masters_count {
            let mut master_0: *mut clusterManagerNode = *masters.offset(i as isize);
            let mut assigned_replicas: libc::c_int = 0 as libc::c_int;
            while assigned_replicas < replicas {
                if available_count == 0 as libc::c_int {
                    break;
                }
                let mut found_0: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
                let mut slave: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
                let mut firstNodeIdx: libc::c_int = -(1 as libc::c_int);
                j = 0 as libc::c_int;
                while j < interleaved_len {
                    let mut n_1: *mut clusterManagerNode = *interleaved
                        .offset(j as isize);
                    if !n_1.is_null() {
                        if strcmp((*n_1).ip, (*master_0).ip) != 0 {
                            found_0 = n_1;
                            let ref mut fresh71 = *interleaved.offset(j as isize);
                            *fresh71 = 0 as *mut clusterManagerNode;
                            break;
                        } else if firstNodeIdx < 0 as libc::c_int {
                            firstNodeIdx = j;
                        }
                    }
                    j += 1;
                }
                if !found_0.is_null() {
                    slave = found_0;
                } else if firstNodeIdx >= 0 as libc::c_int {
                    slave = *interleaved.offset(firstNodeIdx as isize);
                    interleaved_len -= firstNodeIdx + 1 as libc::c_int;
                    interleaved = interleaved
                        .offset((firstNodeIdx + 1 as libc::c_int) as isize);
                }
                if slave.is_null() {
                    break;
                }
                assigned_replicas += 1;
                available_count -= 1;
                if !((*slave).replicate).is_null() {
                    hi_sdsfree((*slave).replicate);
                }
                (*slave).replicate = hi_sdsnew((*master_0).name as *const libc::c_char);
                (*slave).dirty = 1 as libc::c_int;
                printf(
                    b"Adding replica %s:%d to %s:%d\n\0" as *const u8
                        as *const libc::c_char,
                    (*slave).ip,
                    (*slave).port,
                    (*master_0).ip,
                    (*master_0).port,
                );
                if assign_unused != 0 {
                    break;
                }
            }
            i += 1;
        }
        if !(assign_unused == 0 && available_count > 0 as libc::c_int) {
            break;
        }
        assign_unused = 1 as libc::c_int;
        printf(b"Adding extra replicas...\n\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < ip_count {
        let mut node_array_1: *mut clusterManagerNodeArray = ip_nodes.offset(i as isize);
        clusterManagerNodeArrayReset(node_array_1);
        i += 1;
    }
    clusterManagerOptimizeAntiAffinity(ip_nodes, ip_count);
    clusterManagerShowNodes();
    let mut ignore_force: libc::c_int = 0 as libc::c_int;
    if confirmWithYes(
        b"Can I set the above configuration?\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ignore_force,
    ) != 0
    {
        listRewind(cluster_manager.nodes, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                current_block = 6988365858197790817;
                break;
            }
            let mut node_0: *mut clusterManagerNode = (*ln).value
                as *mut clusterManagerNode;
            let mut err_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut flushed: libc::c_int = clusterManagerFlushNodeConfig(
                node_0,
                &mut err_0,
            );
            if flushed == 0 && (*node_0).dirty != 0 && ((*node_0).replicate).is_null() {
                if !err_0.is_null() {
                    clusterManagerLog(
                        3 as libc::c_int,
                        b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                            as *const libc::c_char,
                        (*node_0).ip,
                        (*node_0).port,
                        err_0,
                    );
                    zfree(err_0 as *mut libc::c_void);
                }
                success = 0 as libc::c_int;
                current_block = 4878356980533367297;
                break;
            } else if !err_0.is_null() {
                zfree(err_0 as *mut libc::c_void);
            }
        }
        match current_block {
            4878356980533367297 => {}
            _ => {
                clusterManagerLog(
                    1 as libc::c_int,
                    b">>> Nodes configuration updated\n\0" as *const u8
                        as *const libc::c_char,
                );
                clusterManagerLog(
                    1 as libc::c_int,
                    b">>> Assign a different config epoch to each node\n\0" as *const u8
                        as *const libc::c_char,
                );
                let mut config_epoch: libc::c_int = 1 as libc::c_int;
                listRewind(cluster_manager.nodes, &mut li);
                loop {
                    ln = listNext(&mut li);
                    if ln.is_null() {
                        break;
                    }
                    let mut node_1: *mut clusterManagerNode = (*ln).value
                        as *mut clusterManagerNode;
                    let mut reply: *mut redisReply = 0 as *mut redisReply;
                    let fresh72 = config_epoch;
                    config_epoch = config_epoch + 1;
                    reply = redisCommand(
                        (*node_1).context,
                        b"cluster set-config-epoch %d\0" as *const u8
                            as *const libc::c_char,
                        fresh72,
                    ) as *mut redisReply;
                    if !reply.is_null() {
                        freeReplyObject(reply as *mut libc::c_void);
                    }
                }
                clusterManagerLog(
                    1 as libc::c_int,
                    b">>> Sending CLUSTER MEET messages to join the cluster\n\0"
                        as *const u8 as *const libc::c_char,
                );
                let mut first_0: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
                let mut first_ip: [libc::c_char; 46] = [0; 46];
                listRewind(cluster_manager.nodes, &mut li);
                loop {
                    ln = listNext(&mut li);
                    if ln.is_null() {
                        current_block = 12129449210080749085;
                        break;
                    }
                    let mut node_2: *mut clusterManagerNode = (*ln).value
                        as *mut clusterManagerNode;
                    if first_0.is_null() {
                        first_0 = node_2;
                        if !(anetResolve(
                            0 as *mut libc::c_char,
                            (*first_0).ip,
                            first_ip.as_mut_ptr(),
                            core::mem::size_of::<[libc::c_char; 46]>()
                                as libc::c_ulong,
                            0 as libc::c_int,
                        ) == -(1 as libc::c_int))
                        {
                            continue;
                        }
                        fprintf(
                            stderr,
                            b"Invalid IP address or hostname specified: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            (*first_0).ip,
                        );
                        success = 0 as libc::c_int;
                        current_block = 4878356980533367297;
                        break;
                    } else {
                        let mut reply_0: *mut redisReply = 0 as *mut redisReply;
                        if (*first_0).bus_port == 0 as libc::c_int
                            || (*first_0).bus_port
                                == (*first_0).port + 10000 as libc::c_int
                        {
                            reply_0 = redisCommand(
                                (*node_2).context,
                                b"cluster meet %s %d\0" as *const u8 as *const libc::c_char,
                                first_ip.as_mut_ptr(),
                                (*first_0).port,
                            ) as *mut redisReply;
                        } else {
                            reply_0 = redisCommand(
                                (*node_2).context,
                                b"cluster meet %s %d %d\0" as *const u8
                                    as *const libc::c_char,
                                first_ip.as_mut_ptr(),
                                (*first_0).port,
                                (*first_0).bus_port,
                            ) as *mut redisReply;
                        }
                        let mut is_err: libc::c_int = 0 as libc::c_int;
                        if !reply_0.is_null() {
                            is_err = ((*reply_0).type_0 == 6 as libc::c_int)
                                as libc::c_int;
                            if is_err != 0 {
                                clusterManagerLog(
                                    3 as libc::c_int,
                                    b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                                        as *const libc::c_char,
                                    (*node_2).ip,
                                    (*node_2).port,
                                    (*reply_0).str_0,
                                );
                            }
                            freeReplyObject(reply_0 as *mut libc::c_void);
                        } else {
                            is_err = 1 as libc::c_int;
                            fprintf(
                                stderr,
                                b"Failed to send CLUSTER MEET command.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if !(is_err != 0) {
                            continue;
                        }
                        success = 0 as libc::c_int;
                        current_block = 4878356980533367297;
                        break;
                    }
                }
                match current_block {
                    4878356980533367297 => {}
                    _ => {
                        sleep(1 as libc::c_int as libc::c_uint);
                        clusterManagerWaitForClusterJoin();
                        listRewind(cluster_manager.nodes, &mut li);
                        loop {
                            ln = listNext(&mut li);
                            if ln.is_null() {
                                current_block = 6880299496751257707;
                                break;
                            }
                            let mut node_3: *mut clusterManagerNode = (*ln).value
                                as *mut clusterManagerNode;
                            if (*node_3).dirty == 0 {
                                continue;
                            }
                            let mut err_1: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut flushed_0: libc::c_int = clusterManagerFlushNodeConfig(
                                node_3,
                                &mut err_1,
                            );
                            if flushed_0 == 0 && ((*node_3).replicate).is_null() {
                                if !err_1.is_null() {
                                    clusterManagerLog(
                                        3 as libc::c_int,
                                        b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*node_3).ip,
                                        (*node_3).port,
                                        err_1,
                                    );
                                    zfree(err_1 as *mut libc::c_void);
                                }
                                success = 0 as libc::c_int;
                                current_block = 4878356980533367297;
                                break;
                            } else if !err_1.is_null() {
                                zfree(err_1 as *mut libc::c_void);
                            }
                        }
                        match current_block {
                            4878356980533367297 => {}
                            _ => {
                                listRewind(cluster_manager.nodes, &mut li);
                                let mut first_node_0: *mut clusterManagerNode = 0
                                    as *mut clusterManagerNode;
                                loop {
                                    ln = listNext(&mut li);
                                    if ln.is_null() {
                                        break;
                                    }
                                    let mut node_4: *mut clusterManagerNode = (*ln).value
                                        as *mut clusterManagerNode;
                                    if first_node_0.is_null() {
                                        first_node_0 = node_4;
                                    } else {
                                        freeClusterManagerNode(node_4);
                                    }
                                }
                                listEmpty(cluster_manager.nodes);
                                if clusterManagerLoadInfoFromNode(first_node_0) == 0 {
                                    success = 0 as libc::c_int;
                                } else {
                                    clusterManagerCheckCluster(0 as libc::c_int);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    zfree(masters as *mut libc::c_void);
    zfree(ips as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < node_len {
        let mut node_array_2: *mut clusterManagerNodeArray = ip_nodes.offset(i as isize);
        zfree((*node_array_2).alloc as *mut libc::c_void);
        i += 1;
    }
    zfree(ip_nodes as *mut libc::c_void);
    return success;
}
unsafe extern "C" fn clusterManagerCommandAddNode(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut first: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut first_ip: [libc::c_char; 46] = [0; 46];
    let mut refnode: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut master_node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut new_node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut added: libc::c_int = 0;
    let mut current_block: u64;
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut function_restore_reply: *mut redisReply = 0 as *mut redisReply;
    let mut function_list_reply: *mut redisReply = 0 as *mut redisReply;
    let mut ref_ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ref_port: libc::c_int = 0 as libc::c_int;
    let mut port: libc::c_int = 0 as libc::c_int;
    if !(getClusterHostFromCmdArgs(
        argc - 1 as libc::c_int,
        argv.offset(1 as libc::c_int as isize),
        &mut ref_ip,
        &mut ref_port,
    ) == 0)
    {
        if !(getClusterHostFromCmdArgs(1 as libc::c_int, argv, &mut ip, &mut port) == 0)
        {
            clusterManagerLog(
                1 as libc::c_int,
                b">>> Adding node %s:%d to cluster %s:%d\n\0" as *const u8
                    as *const libc::c_char,
                ip,
                port,
                ref_ip,
                ref_port,
            );
            refnode = clusterManagerNewNode(ref_ip, ref_port, 0 as libc::c_int);
            if clusterManagerLoadInfoFromNode(refnode) == 0 {
                return 0 as libc::c_int;
            }
            if clusterManagerCheckCluster(0 as libc::c_int) == 0 {
                return 0 as libc::c_int;
            }
            master_node = 0 as *mut clusterManagerNode;
            if config.cluster_manager_command.flags
                & (1 as libc::c_int) << 1 as libc::c_int != 0
            {
                let mut master_id: *mut libc::c_char = config
                    .cluster_manager_command
                    .master_id;
                if !master_id.is_null() {
                    master_node = clusterManagerNodeByName(master_id);
                    if master_node.is_null() {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"[ERR] No such master ID %s\n\0" as *const u8
                                as *const libc::c_char,
                            master_id,
                        );
                        return 0 as libc::c_int;
                    }
                } else {
                    master_node = clusterManagerNodeWithLeastReplicas();
                    if !master_node.is_null() {} else {
                        __assert_fail(
                            b"master_node != NULL\0" as *const u8 as *const libc::c_char,
                            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                            6434 as libc::c_int as libc::c_uint,
                            (*core::mem::transmute::<
                                &[u8; 47],
                                &[libc::c_char; 47],
                            >(b"int clusterManagerCommandAddNode(int, char **)\0"))
                                .as_ptr(),
                        );
                    };
                    printf(
                        b"Automatically selected master %s:%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*master_node).ip,
                        (*master_node).port,
                    );
                }
            }
            new_node = clusterManagerNewNode(ip, port, 0 as libc::c_int);
            added = 0 as libc::c_int;
            if clusterManagerNodeConnect(new_node) == 0 {
                clusterManagerLog(
                    3 as libc::c_int,
                    b"[ERR] Sorry, can't connect to node %s:%d\n\0" as *const u8
                        as *const libc::c_char,
                    ip,
                    port,
                );
                success = 0 as libc::c_int;
            } else {
                err = 0 as *mut libc::c_char;
                success = clusterManagerNodeIsCluster(new_node, &mut err);
                if success == 0 {
                    clusterManagerPrintNotClusterNodeError(new_node, err);
                    if !err.is_null() {
                        zfree(err as *mut libc::c_void);
                    }
                } else if clusterManagerNodeLoadInfo(
                    new_node,
                    0 as libc::c_int,
                    &mut err,
                ) == 0
                {
                    if !err.is_null() {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"Node %s:%d replied with error:\n%s\n\0" as *const u8
                                as *const libc::c_char,
                            (*new_node).ip,
                            (*new_node).port,
                            err,
                        );
                        zfree(err as *mut libc::c_void);
                    }
                    success = 0 as libc::c_int;
                } else {
                    success = clusterManagerNodeIsEmpty(new_node, &mut err);
                    if success == 0 {
                        clusterManagerPrintNotEmptyNodeError(new_node, err);
                        if !err.is_null() {
                            zfree(err as *mut libc::c_void);
                        }
                    } else {
                        first = (*(*cluster_manager.nodes).head).value
                            as *mut clusterManagerNode;
                        listAddNodeTail(
                            cluster_manager.nodes,
                            new_node as *mut libc::c_void,
                        );
                        added = 1 as libc::c_int;
                        if master_node.is_null() {
                            clusterManagerLog(
                                1 as libc::c_int,
                                b">>> Getting functions from cluster\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            reply = redisCommand(
                                (*refnode).context,
                                b"FUNCTION DUMP\0" as *const u8 as *const libc::c_char,
                            ) as *mut redisReply;
                            if clusterManagerCheckRedisReply(refnode, reply, &mut err)
                                == 0
                            {
                                clusterManagerLog(
                                    1 as libc::c_int,
                                    b">>> Failed retrieving Functions from the cluster, skip this step as Redis version do not support function command (error = '%s')\n\0"
                                        as *const u8 as *const libc::c_char,
                                    if !err.is_null() {
                                        err as *const libc::c_char
                                    } else {
                                        b"NULL reply\0" as *const u8 as *const libc::c_char
                                    },
                                );
                                if !err.is_null() {
                                    zfree(err as *mut libc::c_void);
                                }
                                current_block = 6560072651652764009;
                            } else {
                                if (*reply).type_0 == 1 as libc::c_int {} else {
                                    __assert_fail(
                                        b"reply->type == REDIS_REPLY_STRING\0" as *const u8
                                            as *const libc::c_char,
                                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                        6481 as libc::c_int as libc::c_uint,
                                        (*core::mem::transmute::<
                                            &[u8; 47],
                                            &[libc::c_char; 47],
                                        >(b"int clusterManagerCommandAddNode(int, char **)\0"))
                                            .as_ptr(),
                                    );
                                };
                                clusterManagerLog(
                                    1 as libc::c_int,
                                    b">>> Send FUNCTION LIST to %s:%d to verify there is no functions in it\n\0"
                                        as *const u8 as *const libc::c_char,
                                    ip,
                                    port,
                                );
                                function_list_reply = redisCommand(
                                    (*new_node).context,
                                    b"FUNCTION LIST\0" as *const u8 as *const libc::c_char,
                                ) as *mut redisReply;
                                if clusterManagerCheckRedisReply(
                                    new_node,
                                    function_list_reply,
                                    &mut err,
                                ) == 0
                                {
                                    clusterManagerLog(
                                        3 as libc::c_int,
                                        b">>> Failed on CLUSTER LIST (error = '%s')\r\n\0"
                                            as *const u8 as *const libc::c_char,
                                        if !err.is_null() {
                                            err as *const libc::c_char
                                        } else {
                                            b"NULL reply\0" as *const u8 as *const libc::c_char
                                        },
                                    );
                                    if !err.is_null() {
                                        zfree(err as *mut libc::c_void);
                                    }
                                    success = 0 as libc::c_int;
                                    current_block = 231212672767030328;
                                } else {
                                    if (*function_list_reply).type_0 == 2 as libc::c_int
                                    {} else {
                                        __assert_fail(
                                            b"function_list_reply->type == REDIS_REPLY_ARRAY\0"
                                                as *const u8 as *const libc::c_char,
                                            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                            6490 as libc::c_int as libc::c_uint,
                                            (*core::mem::transmute::<
                                                &[u8; 47],
                                                &[libc::c_char; 47],
                                            >(b"int clusterManagerCommandAddNode(int, char **)\0"))
                                                .as_ptr(),
                                        );
                                    };
                                    if (*function_list_reply).elements
                                        > 0 as libc::c_int as libc::c_ulong
                                    {
                                        clusterManagerLog(
                                            3 as libc::c_int,
                                            b">>> New node already contains functions and can not be added to the cluster. Use FUNCTION FLUSH and try again.\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        success = 0 as libc::c_int;
                                        current_block = 231212672767030328;
                                    } else {
                                        clusterManagerLog(
                                            1 as libc::c_int,
                                            b">>> Send FUNCTION RESTORE to %s:%d\n\0" as *const u8
                                                as *const libc::c_char,
                                            ip,
                                            port,
                                        );
                                        function_restore_reply = redisCommand(
                                            (*new_node).context,
                                            b"FUNCTION RESTORE %b\0" as *const u8
                                                as *const libc::c_char,
                                            (*reply).str_0,
                                            (*reply).len,
                                        ) as *mut redisReply;
                                        if clusterManagerCheckRedisReply(
                                            new_node,
                                            function_restore_reply,
                                            &mut err,
                                        ) == 0
                                        {
                                            clusterManagerLog(
                                                3 as libc::c_int,
                                                b">>> Failed loading functions to the new node (error = '%s')\r\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                if !err.is_null() {
                                                    err as *const libc::c_char
                                                } else {
                                                    b"NULL reply\0" as *const u8 as *const libc::c_char
                                                },
                                            );
                                            if !err.is_null() {
                                                zfree(err as *mut libc::c_void);
                                            }
                                            success = 0 as libc::c_int;
                                            current_block = 231212672767030328;
                                        } else {
                                            current_block = 6560072651652764009;
                                        }
                                    }
                                }
                            }
                        } else {
                            current_block = 6560072651652764009;
                        }
                        match current_block {
                            231212672767030328 => {}
                            _ => {
                                if !reply.is_null() {
                                    freeReplyObject(reply as *mut libc::c_void);
                                }
                                clusterManagerLog(
                                    1 as libc::c_int,
                                    b">>> Send CLUSTER MEET to node %s:%d to make it join the cluster.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    ip,
                                    port,
                                );
                                first_ip = [0; 46];
                                if anetResolve(
                                    0 as *mut libc::c_char,
                                    (*first).ip,
                                    first_ip.as_mut_ptr(),
                                    core::mem::size_of::<[libc::c_char; 46]>()
                                        as libc::c_ulong,
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                                {
                                    fprintf(
                                        stderr,
                                        b"Invalid IP address or hostname specified: %s\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*first).ip,
                                    );
                                    success = 0 as libc::c_int;
                                } else {
                                    if (*first).bus_port == 0 as libc::c_int
                                        || (*first).bus_port == (*first).port + 10000 as libc::c_int
                                    {
                                        reply = redisCommand(
                                            (*new_node).context,
                                            b"CLUSTER MEET %s %d\0" as *const u8 as *const libc::c_char,
                                            first_ip.as_mut_ptr(),
                                            (*first).port,
                                        ) as *mut redisReply;
                                    } else {
                                        reply = redisCommand(
                                            (*new_node).context,
                                            b"CLUSTER MEET %s %d %d\0" as *const u8
                                                as *const libc::c_char,
                                            first_ip.as_mut_ptr(),
                                            (*first).port,
                                            (*first).bus_port,
                                        ) as *mut redisReply;
                                    }
                                    success = clusterManagerCheckRedisReply(
                                        new_node,
                                        reply,
                                        0 as *mut *mut libc::c_char,
                                    );
                                    if !(success == 0) {
                                        if !master_node.is_null() {
                                            sleep(1 as libc::c_int as libc::c_uint);
                                            clusterManagerWaitForClusterJoin();
                                            clusterManagerLog(
                                                1 as libc::c_int,
                                                b">>> Configure node as replica of %s:%d.\n\0" as *const u8
                                                    as *const libc::c_char,
                                                (*master_node).ip,
                                                (*master_node).port,
                                            );
                                            freeReplyObject(reply as *mut libc::c_void);
                                            reply = redisCommand(
                                                (*new_node).context,
                                                b"CLUSTER REPLICATE %s\0" as *const u8
                                                    as *const libc::c_char,
                                                (*master_node).name,
                                            ) as *mut redisReply;
                                            success = clusterManagerCheckRedisReply(
                                                new_node,
                                                reply,
                                                0 as *mut *mut libc::c_char,
                                            );
                                            if success == 0 {
                                                current_block = 231212672767030328;
                                            } else {
                                                current_block = 3229571381435211107;
                                            }
                                        } else {
                                            current_block = 3229571381435211107;
                                        }
                                        match current_block {
                                            231212672767030328 => {}
                                            _ => {
                                                clusterManagerLog(
                                                    4 as libc::c_int,
                                                    b"[OK] New node added correctly.\n\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if added == 0 && !new_node.is_null() {
                freeClusterManagerNode(new_node);
            }
            if !reply.is_null() {
                freeReplyObject(reply as *mut libc::c_void);
            }
            if !function_restore_reply.is_null() {
                freeReplyObject(function_restore_reply as *mut libc::c_void);
            }
            if !function_list_reply.is_null() {
                freeReplyObject(function_list_reply as *mut libc::c_void);
            }
            return success;
        }
    }
    fprintf(
        stderr,
        b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
            as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn clusterManagerCommandDeleteNode(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut node_id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ref_node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut r_1: *mut redisReply = 0 as *mut redisReply;
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    if getClusterHostFromCmdArgs(1 as libc::c_int, argv, &mut ip, &mut port) == 0 {
        fprintf(
            stderr,
            b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        node_id = *argv.offset(1 as libc::c_int as isize);
        clusterManagerLog(
            1 as libc::c_int,
            b">>> Removing node %s from cluster %s:%d\n\0" as *const u8
                as *const libc::c_char,
            node_id,
            ip,
            port,
        );
        ref_node = clusterManagerNewNode(ip, port, 0 as libc::c_int);
        node = 0 as *mut clusterManagerNode;
        if clusterManagerLoadInfoFromNode(ref_node) == 0 {
            return 0 as libc::c_int;
        }
        node = clusterManagerNodeByName(node_id);
        if node.is_null() {
            clusterManagerLog(
                3 as libc::c_int,
                b"[ERR] No such node ID %s\n\0" as *const u8 as *const libc::c_char,
                node_id,
            );
            return 0 as libc::c_int;
        }
        if (*node).slots_count != 0 as libc::c_int {
            clusterManagerLog(
                3 as libc::c_int,
                b"[ERR] Node %s:%d is not empty! Reshard data away and try again.\n\0"
                    as *const u8 as *const libc::c_char,
                (*node).ip,
                (*node).port,
            );
            return 0 as libc::c_int;
        }
        clusterManagerLog(
            1 as libc::c_int,
            b">>> Sending CLUSTER FORGET messages to the cluster...\n\0" as *const u8
                as *const libc::c_char,
        );
        li = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        ln = 0 as *mut listNode;
        listRewind(cluster_manager.nodes, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
            if n == node {
                continue;
            }
            if !((*n).replicate).is_null()
                && strcasecmp((*n).replicate as *const libc::c_char, node_id) == 0
            {
                let mut master: *mut clusterManagerNode = clusterManagerNodeWithLeastReplicas();
                if !master.is_null() {} else {
                    __assert_fail(
                        b"master != NULL\0" as *const u8 as *const libc::c_char,
                        b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                        6597 as libc::c_int as libc::c_uint,
                        (*core::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"int clusterManagerCommandDeleteNode(int, char **)\0"))
                            .as_ptr(),
                    );
                };
                clusterManagerLog(
                    1 as libc::c_int,
                    b">>> %s:%d as replica of %s:%d\n\0" as *const u8
                        as *const libc::c_char,
                    (*n).ip,
                    (*n).port,
                    (*master).ip,
                    (*master).port,
                );
                let mut r: *mut redisReply = redisCommand(
                    (*n).context,
                    b"CLUSTER REPLICATE %s\0" as *const u8 as *const libc::c_char,
                    (*master).name,
                ) as *mut redisReply;
                success = clusterManagerCheckRedisReply(
                    n,
                    r,
                    0 as *mut *mut libc::c_char,
                );
                if !r.is_null() {
                    freeReplyObject(r as *mut libc::c_void);
                }
                if success == 0 {
                    return 0 as libc::c_int;
                }
            }
            let mut r_0: *mut redisReply = redisCommand(
                (*n).context,
                b"CLUSTER FORGET %s\0" as *const u8 as *const libc::c_char,
                node_id,
            ) as *mut redisReply;
            success = clusterManagerCheckRedisReply(n, r_0, 0 as *mut *mut libc::c_char);
            if !r_0.is_null() {
                freeReplyObject(r_0 as *mut libc::c_void);
            }
            if success == 0 {
                return 0 as libc::c_int;
            }
        }
        clusterManagerLog(
            1 as libc::c_int,
            b">>> Sending CLUSTER RESET SOFT to the deleted node.\n\0" as *const u8
                as *const libc::c_char,
        );
        r_1 = redisCommand(
            (*node).context,
            b"CLUSTER RESET %s\0" as *const u8 as *const libc::c_char,
            b"SOFT\0" as *const u8 as *const libc::c_char,
        ) as *mut redisReply;
        success = clusterManagerCheckRedisReply(node, r_1, 0 as *mut *mut libc::c_char);
        if !r_1.is_null() {
            freeReplyObject(r_1 as *mut libc::c_void);
        }
        return success;
    };
}
unsafe extern "C" fn clusterManagerCommandInfo(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    if getClusterHostFromCmdArgs(argc, argv, &mut ip, &mut port) == 0 {
        fprintf(
            stderr,
            b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        node = clusterManagerNewNode(ip, port, 0 as libc::c_int);
        if clusterManagerLoadInfoFromNode(node) == 0 {
            return 0 as libc::c_int;
        }
        clusterManagerShowClusterInfo();
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn clusterManagerCommandCheck(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    if getClusterHostFromCmdArgs(argc, argv, &mut ip, &mut port) == 0 {
        fprintf(
            stderr,
            b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        node = clusterManagerNewNode(ip, port, 0 as libc::c_int);
        if clusterManagerLoadInfoFromNode(node) == 0 {
            return 0 as libc::c_int;
        }
        clusterManagerShowClusterInfo();
        return clusterManagerCheckCluster(0 as libc::c_int);
    };
}
unsafe extern "C" fn clusterManagerCommandFix(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    config.cluster_manager_command.flags |= (1 as libc::c_int) << 0 as libc::c_int;
    return clusterManagerCommandCheck(argc, argv);
}
unsafe extern "C" fn clusterManagerCommandReshard(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut info_0: hisds = 0 as *mut libc::c_char;
    let mut opts: libc::c_int = 0;
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut slots: libc::c_int = 0;
    let mut buf_0: [libc::c_char; 255] = [0; 255];
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut raise_err: libc::c_int = 0;
    let mut target: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut sources: *mut list = 0 as *mut list;
    let mut table: *mut list = 0 as *mut list;
    let mut all: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut current_block: u64;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    if getClusterHostFromCmdArgs(argc, argv, &mut ip, &mut port) == 0 {
        fprintf(
            stderr,
            b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        node = clusterManagerNewNode(ip, port, 0 as libc::c_int);
        if clusterManagerLoadInfoFromNode(node) == 0 {
            return 0 as libc::c_int;
        }
        clusterManagerCheckCluster(0 as libc::c_int);
        if !(cluster_manager.errors).is_null()
            && (*cluster_manager.errors).len > 0 as libc::c_int as libc::c_ulong
        {
            fflush(stdout);
            fprintf(
                stderr,
                b"*** Please fix your cluster problems before resharding\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        slots = config.cluster_manager_command.slots;
        if slots == 0 {
            while slots <= 0 as libc::c_int || slots > 16384 as libc::c_int {
                printf(
                    b"How many slots do you want to move (from 1 to %d)? \0" as *const u8
                        as *const libc::c_char,
                    16384 as libc::c_int,
                );
                fflush(stdout);
                let mut buf: [libc::c_char; 6] = [0; 6];
                let mut nread: libc::c_int = read(
                    fileno(stdin),
                    buf.as_mut_ptr() as *mut libc::c_void,
                    6 as libc::c_int as size_t,
                ) as libc::c_int;
                if nread <= 0 as libc::c_int {
                    continue;
                }
                let mut last_idx: libc::c_int = nread - 1 as libc::c_int;
                if buf[last_idx as usize] as libc::c_int != '\n' as i32 {
                    let mut ch: libc::c_int = 0;
                    loop {
                        ch = getchar();
                        if !(ch != '\n' as i32 && ch != -(1 as libc::c_int)) {
                            break;
                        }
                    }
                }
                buf[last_idx as usize] = '\0' as i32 as libc::c_char;
                slots = atoi(buf.as_mut_ptr());
            }
        }
        buf_0 = [0; 255];
        to = config.cluster_manager_command.to;
        from = config.cluster_manager_command.from;
        while to.is_null() {
            printf(
                b"What is the receiving node ID? \0" as *const u8 as *const libc::c_char,
            );
            fflush(stdout);
            let mut nread_0: libc::c_int = read(
                fileno(stdin),
                buf_0.as_mut_ptr() as *mut libc::c_void,
                255 as libc::c_int as size_t,
            ) as libc::c_int;
            if nread_0 <= 0 as libc::c_int {
                continue;
            }
            let mut last_idx_0: libc::c_int = nread_0 - 1 as libc::c_int;
            if buf_0[last_idx_0 as usize] as libc::c_int != '\n' as i32 {
                let mut ch_0: libc::c_int = 0;
                loop {
                    ch_0 = getchar();
                    if !(ch_0 != '\n' as i32 && ch_0 != -(1 as libc::c_int)) {
                        break;
                    }
                }
            }
            buf_0[last_idx_0 as usize] = '\0' as i32 as libc::c_char;
            if strlen(buf_0.as_mut_ptr()) > 0 as libc::c_int as libc::c_ulong {
                to = buf_0.as_mut_ptr();
            }
        }
        raise_err = 0 as libc::c_int;
        target = clusterNodeForResharding(
            to,
            0 as *mut clusterManagerNode,
            &mut raise_err,
        );
        if target.is_null() {
            return 0 as libc::c_int;
        }
        sources = listCreate();
        table = 0 as *mut list;
        all = 0 as libc::c_int;
        result = 1 as libc::c_int;
        if from.is_null() {
            printf(
                b"Please enter all the source node IDs.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"  Type 'all' to use all the nodes as source nodes for the hash slots.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            printf(
                b"  Type 'done' once you entered all the source nodes IDs.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            loop {
                printf(
                    b"Source node #%lu: \0" as *const u8 as *const libc::c_char,
                    ((*sources).len).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                fflush(stdout);
                let mut nread_1: libc::c_int = read(
                    fileno(stdin),
                    buf_0.as_mut_ptr() as *mut libc::c_void,
                    255 as libc::c_int as size_t,
                ) as libc::c_int;
                if nread_1 <= 0 as libc::c_int {
                    continue;
                }
                let mut last_idx_1: libc::c_int = nread_1 - 1 as libc::c_int;
                if buf_0[last_idx_1 as usize] as libc::c_int != '\n' as i32 {
                    let mut ch_1: libc::c_int = 0;
                    loop {
                        ch_1 = getchar();
                        if !(ch_1 != '\n' as i32 && ch_1 != -(1 as libc::c_int)) {
                            break;
                        }
                    }
                }
                buf_0[last_idx_1 as usize] = '\0' as i32 as libc::c_char;
                if strcmp(
                    buf_0.as_mut_ptr(),
                    b"done\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    current_block = 2168227384378665163;
                    break;
                }
                if strcmp(
                    buf_0.as_mut_ptr(),
                    b"all\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    all = 1 as libc::c_int;
                    current_block = 2168227384378665163;
                    break;
                } else {
                    let mut src: *mut clusterManagerNode = clusterNodeForResharding(
                        buf_0.as_mut_ptr(),
                        target,
                        &mut raise_err,
                    );
                    if !src.is_null() {
                        listAddNodeTail(sources, src as *mut libc::c_void);
                    } else {
                        if !(raise_err != 0) {
                            continue;
                        }
                        result = 0 as libc::c_int;
                        current_block = 7153694847983639079;
                        break;
                    }
                }
            }
        } else {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            loop {
                p = strchr(from, ',' as i32);
                if p.is_null() {
                    current_block = 7494008139977416618;
                    break;
                }
                *p = '\0' as i32 as libc::c_char;
                if strcmp(from, b"all\0" as *const u8 as *const libc::c_char) == 0 {
                    all = 1 as libc::c_int;
                    current_block = 7494008139977416618;
                    break;
                } else {
                    let mut src_0: *mut clusterManagerNode = clusterNodeForResharding(
                        from,
                        target,
                        &mut raise_err,
                    );
                    if !src_0.is_null() {
                        listAddNodeTail(sources, src_0 as *mut libc::c_void);
                    } else if raise_err != 0 {
                        result = 0 as libc::c_int;
                        current_block = 7153694847983639079;
                        break;
                    }
                    from = p.offset(1 as libc::c_int as isize);
                }
            }
            match current_block {
                7153694847983639079 => {}
                _ => {
                    if all == 0 && strlen(from) > 0 as libc::c_int as libc::c_ulong {
                        if strcmp(from, b"all\0" as *const u8 as *const libc::c_char)
                            == 0
                        {
                            all = 1 as libc::c_int;
                        }
                        if all == 0 {
                            let mut src_1: *mut clusterManagerNode = clusterNodeForResharding(
                                from,
                                target,
                                &mut raise_err,
                            );
                            if !src_1.is_null() {
                                listAddNodeTail(sources, src_1 as *mut libc::c_void);
                                current_block = 2168227384378665163;
                            } else if raise_err != 0 {
                                result = 0 as libc::c_int;
                                current_block = 7153694847983639079;
                            } else {
                                current_block = 2168227384378665163;
                            }
                        } else {
                            current_block = 2168227384378665163;
                        }
                    } else {
                        current_block = 2168227384378665163;
                    }
                }
            }
        }
        match current_block {
            2168227384378665163 => {
                li = listIter {
                    next: 0 as *mut listNode,
                    direction: 0,
                };
                ln = 0 as *mut listNode;
                if all != 0 {
                    listEmpty(sources);
                    listRewind(cluster_manager.nodes, &mut li);
                    loop {
                        ln = listNext(&mut li);
                        if ln.is_null() {
                            break;
                        }
                        let mut n: *mut clusterManagerNode = (*ln).value
                            as *mut clusterManagerNode;
                        if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
                            || !((*n).replicate).is_null()
                        {
                            continue;
                        }
                        if hi_sdscmp((*n).name, (*target).name) == 0 {
                            continue;
                        }
                        listAddNodeTail(sources, n as *mut libc::c_void);
                    }
                }
                if (*sources).len == 0 as libc::c_int as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"*** No source nodes given, operation aborted.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    result = 0 as libc::c_int;
                } else {
                    printf(
                        b"\nReady to move %d slots.\n\0" as *const u8
                            as *const libc::c_char,
                        slots,
                    );
                    printf(b"  Source nodes:\n\0" as *const u8 as *const libc::c_char);
                    listRewind(sources, &mut li);
                    loop {
                        ln = listNext(&mut li);
                        if ln.is_null() {
                            break;
                        }
                        let mut src_2: *mut clusterManagerNode = (*ln).value
                            as *mut clusterManagerNode;
                        let mut info: hisds = clusterManagerNodeInfo(
                            src_2,
                            4 as libc::c_int,
                        );
                        printf(b"%s\n\0" as *const u8 as *const libc::c_char, info);
                        hi_sdsfree(info);
                    }
                    printf(
                        b"  Destination node:\n\0" as *const u8 as *const libc::c_char,
                    );
                    info_0 = clusterManagerNodeInfo(target, 4 as libc::c_int);
                    printf(b"%s\n\0" as *const u8 as *const libc::c_char, info_0);
                    hi_sdsfree(info_0);
                    table = clusterManagerComputeReshardTable(sources, slots);
                    printf(
                        b"  Resharding plan:\n\0" as *const u8 as *const libc::c_char,
                    );
                    clusterManagerShowReshardTable(table);
                    if config.cluster_manager_command.flags
                        & (1 as libc::c_int) << 2 as libc::c_int == 0
                    {
                        printf(
                            b"Do you want to proceed with the proposed reshard plan (yes/no)? \0"
                                as *const u8 as *const libc::c_char,
                        );
                        fflush(stdout);
                        let mut buf_1: [libc::c_char; 4] = [0; 4];
                        let mut nread_2: libc::c_int = read(
                            fileno(stdin),
                            buf_1.as_mut_ptr() as *mut libc::c_void,
                            4 as libc::c_int as size_t,
                        ) as libc::c_int;
                        buf_1[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        if nread_2 <= 0 as libc::c_int
                            || strcmp(
                                b"yes\0" as *const u8 as *const libc::c_char,
                                buf_1.as_mut_ptr(),
                            ) != 0 as libc::c_int
                        {
                            result = 0 as libc::c_int;
                            current_block = 7153694847983639079;
                        } else {
                            current_block = 722119776535234387;
                        }
                    } else {
                        current_block = 722119776535234387;
                    }
                    match current_block {
                        7153694847983639079 => {}
                        _ => {
                            opts = (1 as libc::c_int) << 7 as libc::c_int;
                            listRewind(table, &mut li);
                            loop {
                                ln = listNext(&mut li);
                                if ln.is_null() {
                                    break;
                                }
                                let mut item: *mut clusterManagerReshardTableItem = (*ln)
                                    .value as *mut clusterManagerReshardTableItem;
                                let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
                                result = clusterManagerMoveSlot(
                                    (*item).source,
                                    target,
                                    (*item).slot,
                                    opts,
                                    &mut err,
                                );
                                if !(result == 0) {
                                    continue;
                                }
                                if !err.is_null() {
                                    clusterManagerLog(
                                        3 as libc::c_int,
                                        b"clusterManagerMoveSlot failed: %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        err,
                                    );
                                    zfree(err as *mut libc::c_void);
                                }
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        listRelease(sources);
        clusterManagerReleaseReshardTable(table);
        return result;
    };
}
unsafe extern "C" fn clusterManagerCommandRebalance(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut total_weight: libc::c_float = 0.;
    let mut nodes_involved: libc::c_int = 0;
    let mut use_empty: libc::c_int = 0;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut threshold_reached: libc::c_int = 0;
    let mut total_balance: libc::c_int = 0;
    let mut threshold: libc::c_float = 0.;
    let mut dst_idx: libc::c_int = 0;
    let mut src_idx: libc::c_int = 0;
    let mut simulate: libc::c_int = 0;
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut result: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut current_block: u64;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut weightedNodes: *mut *mut clusterManagerNode = 0
        as *mut *mut clusterManagerNode;
    let mut involved: *mut list = 0 as *mut list;
    if getClusterHostFromCmdArgs(argc, argv, &mut ip, &mut port) == 0 {
        fprintf(
            stderr,
            b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        node = clusterManagerNewNode(ip, port, 0 as libc::c_int);
        if clusterManagerLoadInfoFromNode(node) == 0 {
            return 0 as libc::c_int;
        }
        result = 1 as libc::c_int;
        i = 0;
        if !(config.cluster_manager_command.weight).is_null() {
            i = 0 as libc::c_int;
            loop {
                if !(i < config.cluster_manager_command.weight_argc) {
                    current_block = 7149356873433890176;
                    break;
                }
                let mut name: *mut libc::c_char = *(config
                    .cluster_manager_command
                    .weight)
                    .offset(i as isize);
                let mut p: *mut libc::c_char = strchr(name, '=' as i32);
                if p.is_null() {
                    clusterManagerLog(
                        3 as libc::c_int,
                        b"*** invalid input %s\n\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                    result = 0 as libc::c_int;
                    current_block = 2461195999744060749;
                    break;
                } else {
                    *p = '\0' as i32 as libc::c_char;
                    p = p.offset(1);
                    let mut w: libc::c_float = atof(p) as libc::c_float;
                    let mut n: *mut clusterManagerNode = clusterManagerNodeByAbbreviatedName(
                        name,
                    );
                    if n.is_null() {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"*** No such master node %s\n\0" as *const u8
                                as *const libc::c_char,
                            name,
                        );
                        result = 0 as libc::c_int;
                        current_block = 2461195999744060749;
                        break;
                    } else {
                        (*n).weight = w;
                        i += 1;
                    }
                }
            }
        } else {
            current_block = 7149356873433890176;
        }
        match current_block {
            7149356873433890176 => {
                total_weight = 0 as libc::c_int as libc::c_float;
                nodes_involved = 0 as libc::c_int;
                use_empty = config.cluster_manager_command.flags
                    & (1 as libc::c_int) << 4 as libc::c_int;
                involved = listCreate();
                li = listIter {
                    next: 0 as *mut listNode,
                    direction: 0,
                };
                ln = 0 as *mut listNode;
                listRewind(cluster_manager.nodes, &mut li);
                loop {
                    ln = listNext(&mut li);
                    if ln.is_null() {
                        break;
                    }
                    let mut n_0: *mut clusterManagerNode = (*ln).value
                        as *mut clusterManagerNode;
                    if (*n_0).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
                        || !((*n_0).replicate).is_null()
                    {
                        continue;
                    }
                    if use_empty == 0 && (*n_0).slots_count == 0 as libc::c_int {
                        (*n_0).weight = 0 as libc::c_int as libc::c_float;
                    } else {
                        total_weight += (*n_0).weight;
                        nodes_involved += 1;
                        listAddNodeTail(involved, n_0 as *mut libc::c_void);
                    }
                }
                weightedNodes = zmalloc(
                    (nodes_involved as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<*mut clusterManagerNode>()
                                as libc::c_ulong,
                        ),
                ) as *mut *mut clusterManagerNode;
                if !weightedNodes.is_null() {
                    clusterManagerCheckCluster(1 as libc::c_int);
                    if !(cluster_manager.errors).is_null()
                        && (*cluster_manager.errors).len
                            > 0 as libc::c_int as libc::c_ulong
                    {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"*** Please fix your cluster problems before rebalancing\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        result = 0 as libc::c_int;
                    } else {
                        threshold_reached = 0 as libc::c_int;
                        total_balance = 0 as libc::c_int;
                        threshold = config.cluster_manager_command.threshold;
                        i = 0 as libc::c_int;
                        listRewind(involved, &mut li);
                        loop {
                            ln = listNext(&mut li);
                            if ln.is_null() {
                                break;
                            }
                            let mut n_1: *mut clusterManagerNode = (*ln).value
                                as *mut clusterManagerNode;
                            let fresh73 = i;
                            i = i + 1;
                            let ref mut fresh74 = *weightedNodes
                                .offset(fresh73 as isize);
                            *fresh74 = n_1;
                            let mut expected: libc::c_int = (16384 as libc::c_int
                                as libc::c_float / total_weight * (*n_1).weight)
                                as libc::c_int;
                            (*n_1).balance = (*n_1).slots_count - expected;
                            total_balance += (*n_1).balance;
                            let mut over_threshold: libc::c_int = 0 as libc::c_int;
                            if threshold > 0 as libc::c_int as libc::c_float {
                                if (*n_1).slots_count > 0 as libc::c_int {
                                    let mut err_perc: libc::c_float = fabs(
                                        100 as libc::c_int as libc::c_double
                                            - 100.0f64 * expected as libc::c_double
                                                / (*n_1).slots_count as libc::c_double,
                                    ) as libc::c_float;
                                    if err_perc > threshold {
                                        over_threshold = 1 as libc::c_int;
                                    }
                                } else if expected > 1 as libc::c_int {
                                    over_threshold = 1 as libc::c_int;
                                }
                            }
                            if over_threshold != 0 {
                                threshold_reached = 1 as libc::c_int;
                            }
                        }
                        if threshold_reached == 0 {
                            clusterManagerLog(
                                2 as libc::c_int,
                                b"*** No rebalancing needed! All nodes are within the %.2f%% threshold.\n\0"
                                    as *const u8 as *const libc::c_char,
                                config.cluster_manager_command.threshold as libc::c_double,
                            );
                        } else {
                            while total_balance > 0 as libc::c_int {
                                listRewind(involved, &mut li);
                                loop {
                                    ln = listNext(&mut li);
                                    if ln.is_null() {
                                        break;
                                    }
                                    let mut n_2: *mut clusterManagerNode = (*ln).value
                                        as *mut clusterManagerNode;
                                    if (*n_2).balance <= 0 as libc::c_int
                                        && total_balance > 0 as libc::c_int
                                    {
                                        (*n_2).balance -= 1;
                                        total_balance -= 1;
                                    }
                                }
                            }
                            qsort(
                                weightedNodes as *mut libc::c_void,
                                nodes_involved as size_t,
                                core::mem::size_of::<*mut clusterManagerNode>()
                                    as libc::c_ulong,
                                Some(
                                    clusterManagerCompareNodeBalance
                                        as unsafe extern "C" fn(
                                            *const libc::c_void,
                                            *const libc::c_void,
                                        ) -> libc::c_int,
                                ),
                            );
                            clusterManagerLog(
                                1 as libc::c_int,
                                b">>> Rebalancing across %d nodes. Total weight = %.2f\n\0"
                                    as *const u8 as *const libc::c_char,
                                nodes_involved,
                                total_weight as libc::c_double,
                            );
                            if config.verbose != 0 {
                                i = 0 as libc::c_int;
                                while i < nodes_involved {
                                    let mut n_3: *mut clusterManagerNode = *weightedNodes
                                        .offset(i as isize);
                                    printf(
                                        b"%s:%d balance is %d slots\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*n_3).ip,
                                        (*n_3).port,
                                        (*n_3).balance,
                                    );
                                    i += 1;
                                }
                            }
                            dst_idx = 0 as libc::c_int;
                            src_idx = nodes_involved - 1 as libc::c_int;
                            simulate = config.cluster_manager_command.flags
                                & (1 as libc::c_int) << 5 as libc::c_int;
                            while dst_idx < src_idx {
                                let mut dst: *mut clusterManagerNode = *weightedNodes
                                    .offset(dst_idx as isize);
                                let mut src: *mut clusterManagerNode = *weightedNodes
                                    .offset(src_idx as isize);
                                let mut db: libc::c_int = abs((*dst).balance);
                                let mut sb: libc::c_int = abs((*src).balance);
                                let mut numslots: libc::c_int = if db < sb {
                                    db
                                } else {
                                    sb
                                };
                                if numslots > 0 as libc::c_int {
                                    printf(
                                        b"Moving %d slots from %s:%d to %s:%d\n\0" as *const u8
                                            as *const libc::c_char,
                                        numslots,
                                        (*src).ip,
                                        (*src).port,
                                        (*dst).ip,
                                        (*dst).port,
                                    );
                                    let mut lsrc: *mut list = listCreate();
                                    let mut table: *mut list = 0 as *mut list;
                                    listAddNodeTail(lsrc, src as *mut libc::c_void);
                                    table = clusterManagerComputeReshardTable(lsrc, numslots);
                                    listRelease(lsrc);
                                    let mut table_len: libc::c_int = (*table).len
                                        as libc::c_int;
                                    if table.is_null() || table_len != numslots {
                                        clusterManagerLog(
                                            3 as libc::c_int,
                                            b"*** Assertion failed: Reshard table != number of slots\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        result = 0 as libc::c_int;
                                    } else {
                                        if simulate != 0 {
                                            i = 0 as libc::c_int;
                                            while i < table_len {
                                                printf(b"#\0" as *const u8 as *const libc::c_char);
                                                i += 1;
                                            }
                                            current_block = 1852451392920375136;
                                        } else {
                                            let mut opts: libc::c_int = (1 as libc::c_int)
                                                << 6 as libc::c_int
                                                | (1 as libc::c_int) << 2 as libc::c_int;
                                            listRewind(table, &mut li);
                                            loop {
                                                ln = listNext(&mut li);
                                                if ln.is_null() {
                                                    current_block = 1852451392920375136;
                                                    break;
                                                }
                                                let mut item: *mut clusterManagerReshardTableItem = (*ln)
                                                    .value as *mut clusterManagerReshardTableItem;
                                                let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
                                                result = clusterManagerMoveSlot(
                                                    (*item).source,
                                                    dst,
                                                    (*item).slot,
                                                    opts,
                                                    &mut err,
                                                );
                                                if result == 0 {
                                                    clusterManagerLog(
                                                        3 as libc::c_int,
                                                        b"*** clusterManagerMoveSlot: %s\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        err,
                                                    );
                                                    zfree(err as *mut libc::c_void);
                                                    current_block = 11547696158181946830;
                                                    break;
                                                } else {
                                                    printf(b"#\0" as *const u8 as *const libc::c_char);
                                                    fflush(stdout);
                                                }
                                            }
                                        }
                                        match current_block {
                                            11547696158181946830 => {}
                                            _ => {
                                                printf(b"\n\0" as *const u8 as *const libc::c_char);
                                            }
                                        }
                                    }
                                    clusterManagerReleaseReshardTable(table);
                                    if result == 0 {
                                        break;
                                    }
                                }
                                (*dst).balance += numslots;
                                (*src).balance -= numslots;
                                if (*dst).balance == 0 as libc::c_int {
                                    dst_idx += 1;
                                }
                                if (*src).balance == 0 as libc::c_int {
                                    src_idx -= 1;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        if !involved.is_null() {
            listRelease(involved);
        }
        if !weightedNodes.is_null() {
            zfree(weightedNodes as *mut libc::c_void);
        }
        return result;
    };
}
unsafe extern "C" fn clusterManagerCommandSetTimeout(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ok: libc::c_int = 0;
    let mut timeout: libc::c_int = 0;
    let mut node: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut ok_count: libc::c_int = 0;
    let mut err_count: libc::c_int = 0;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    if getClusterHostFromCmdArgs(1 as libc::c_int, argv, &mut ip, &mut port) == 0 {
        fprintf(
            stderr,
            b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        timeout = atoi(*argv.offset(1 as libc::c_int as isize));
        if timeout < 100 as libc::c_int {
            fprintf(
                stderr,
                b"Setting a node timeout of less than 100 milliseconds is a bad idea.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        node = clusterManagerNewNode(ip, port, 0 as libc::c_int);
        if clusterManagerLoadInfoFromNode(node) == 0 {
            return 0 as libc::c_int;
        }
        ok_count = 0 as libc::c_int;
        err_count = 0 as libc::c_int;
        clusterManagerLog(
            1 as libc::c_int,
            b">>> Reconfiguring node timeout in every cluster node...\n\0" as *const u8
                as *const libc::c_char,
        );
        li = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        ln = 0 as *mut listNode;
        listRewind(cluster_manager.nodes, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
            let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut reply: *mut redisReply = redisCommand(
                (*n).context,
                b"CONFIG %s %s %d\0" as *const u8 as *const libc::c_char,
                b"SET\0" as *const u8 as *const libc::c_char,
                b"cluster-node-timeout\0" as *const u8 as *const libc::c_char,
                timeout,
            ) as *mut redisReply;
            if !reply.is_null() {
                ok = clusterManagerCheckRedisReply(n, reply, &mut err);
                freeReplyObject(reply as *mut libc::c_void);
                if !(ok == 0) {
                    reply = redisCommand(
                        (*n).context,
                        b"CONFIG %s\0" as *const u8 as *const libc::c_char,
                        b"REWRITE\0" as *const u8 as *const libc::c_char,
                    ) as *mut redisReply;
                    if !reply.is_null() {
                        ok = clusterManagerCheckRedisReply(n, reply, &mut err);
                        freeReplyObject(reply as *mut libc::c_void);
                        if !(ok == 0) {
                            clusterManagerLog(
                                2 as libc::c_int,
                                b"*** New timeout set for %s:%d\n\0" as *const u8
                                    as *const libc::c_char,
                                (*n).ip,
                                (*n).port,
                            );
                            ok_count += 1;
                            continue;
                        }
                    }
                }
            }
            let mut need_free: libc::c_int = 0 as libc::c_int;
            if err.is_null() {
                err = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            } else {
                need_free = 1 as libc::c_int;
            }
            clusterManagerLog(
                3 as libc::c_int,
                b"ERR setting node-timeout for %s:%d: %s\n\0" as *const u8
                    as *const libc::c_char,
                (*n).ip,
                (*n).port,
                err,
            );
            if need_free != 0 {
                zfree(err as *mut libc::c_void);
            }
            err_count += 1;
        }
        clusterManagerLog(
            1 as libc::c_int,
            b">>> New node timeout set. %d OK, %d ERR.\n\0" as *const u8
                as *const libc::c_char,
            ok_count,
            err_count,
        );
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn clusterManagerCommandImport(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut from_user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from_pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut slots_map: [*mut clusterManagerNode; 16384] = [0
        as *mut clusterManagerNode; 16384];
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut cursor: libc::c_int = 0;
    let mut timeout: libc::c_int = 0;
    let mut src_host: [*mut libc::c_char; 1] = [0 as *mut libc::c_char; 1];
    let mut refnode: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut reply_err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src_reply: *mut redisReply = 0 as *mut redisReply;
    let mut src_ctx: *mut redisContext = 0 as *mut redisContext;
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut src_port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src_ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut invalid_args_msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmdfmt: hisds = 0 as hisds;
    if getClusterHostFromCmdArgs(argc, argv, &mut ip, &mut port) == 0 {
        invalid_args_msg = b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if (config.cluster_manager_command.from).is_null() {
        invalid_args_msg = b"[ERR] Option '--cluster-from' is required for subcommand 'import'.\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        src_host = [config.cluster_manager_command.from];
        if getClusterHostFromCmdArgs(
            1 as libc::c_int,
            src_host.as_mut_ptr(),
            &mut src_ip,
            &mut src_port,
        ) == 0
        {
            invalid_args_msg = b"[ERR] Invalid --cluster-from host. You need to pass a valid address (ie. 120.0.0.1:7000).\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            clusterManagerLog(
                1 as libc::c_int,
                b">>> Importing data from %s:%d to cluster %s:%d\n\0" as *const u8
                    as *const libc::c_char,
                src_ip,
                src_port,
                ip,
                port,
            );
            refnode = clusterManagerNewNode(ip, port, 0 as libc::c_int);
            if clusterManagerLoadInfoFromNode(refnode) == 0 {
                return 0 as libc::c_int;
            }
            if clusterManagerCheckCluster(0 as libc::c_int) == 0 {
                return 0 as libc::c_int;
            }
            reply_err = 0 as *mut libc::c_char;
            src_reply = 0 as *mut redisReply;
            src_ctx = redisConnect(src_ip, src_port);
            if (*src_ctx).err != 0 {
                success = 0 as libc::c_int;
                fprintf(
                    stderr,
                    b"Could not connect to Redis at %s:%d: %s.\n\0" as *const u8
                        as *const libc::c_char,
                    src_ip,
                    src_port,
                    ((*src_ctx).errstr).as_mut_ptr(),
                );
            } else {
                from_user = config.cluster_manager_command.from_user;
                from_pass = config.cluster_manager_command.from_pass;
                if cliAuth(src_ctx, from_user, from_pass) == -(1 as libc::c_int) {
                    success = 0 as libc::c_int;
                } else {
                    src_reply = reconnectingRedisCommand(
                        src_ctx,
                        b"INFO\0" as *const u8 as *const libc::c_char,
                    );
                    if src_reply.is_null() || (*src_reply).type_0 == 6 as libc::c_int {
                        if !src_reply.is_null() && !((*src_reply).str_0).is_null() {
                            reply_err = (*src_reply).str_0;
                        }
                        success = 0 as libc::c_int;
                    } else if getLongInfoField(
                        (*src_reply).str_0,
                        b"cluster_enabled\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) != 0
                    {
                        clusterManagerLog(
                            3 as libc::c_int,
                            b"[ERR] The source node should not be a cluster node.\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        success = 0 as libc::c_int;
                    } else {
                        freeReplyObject(src_reply as *mut libc::c_void);
                        src_reply = reconnectingRedisCommand(
                            src_ctx,
                            b"DBSIZE\0" as *const u8 as *const libc::c_char,
                        );
                        if src_reply.is_null() || (*src_reply).type_0 == 6 as libc::c_int
                        {
                            if !src_reply.is_null() && !((*src_reply).str_0).is_null() {
                                reply_err = (*src_reply).str_0;
                            }
                            success = 0 as libc::c_int;
                        } else {
                            size = (*src_reply).integer as libc::c_int;
                            i = 0;
                            clusterManagerLog(
                                2 as libc::c_int,
                                b"*** Importing %d keys from DB 0\n\0" as *const u8
                                    as *const libc::c_char,
                                size,
                            );
                            slots_map = [0 as *mut clusterManagerNode; 16384];
                            memset(
                                slots_map.as_mut_ptr() as *mut libc::c_void,
                                0 as libc::c_int,
                                core::mem::size_of::<[*mut clusterManagerNode; 16384]>()
                                    as libc::c_ulong,
                            );
                            li = listIter {
                                next: 0 as *mut listNode,
                                direction: 0,
                            };
                            ln = 0 as *mut listNode;
                            i = 0 as libc::c_int;
                            while i < 16384 as libc::c_int {
                                listRewind(cluster_manager.nodes, &mut li);
                                loop {
                                    ln = listNext(&mut li);
                                    if ln.is_null() {
                                        break;
                                    }
                                    let mut n: *mut clusterManagerNode = (*ln).value
                                        as *mut clusterManagerNode;
                                    if (*n).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
                                    {
                                        continue;
                                    }
                                    if (*n).slots_count == 0 as libc::c_int {
                                        continue;
                                    }
                                    if !((*n).slots[i as usize] != 0) {
                                        continue;
                                    }
                                    slots_map[i as usize] = n;
                                    break;
                                }
                                i += 1;
                            }
                            cmdfmt = hi_sdsnew(
                                b"MIGRATE %s %d %s %d %d\0" as *const u8
                                    as *const libc::c_char,
                            );
                            if !(config.conn_info.auth).is_null() {
                                if !(config.conn_info.user).is_null() {
                                    cmdfmt = hi_sdscatfmt(
                                        cmdfmt,
                                        b" AUTH2 %s %s\0" as *const u8 as *const libc::c_char,
                                        config.conn_info.user,
                                        config.conn_info.auth,
                                    );
                                } else {
                                    cmdfmt = hi_sdscatfmt(
                                        cmdfmt,
                                        b" AUTH %s\0" as *const u8 as *const libc::c_char,
                                        config.conn_info.auth,
                                    );
                                }
                            }
                            if config.cluster_manager_command.flags
                                & (1 as libc::c_int) << 7 as libc::c_int != 0
                            {
                                cmdfmt = hi_sdscat(
                                    cmdfmt,
                                    b" COPY\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if config.cluster_manager_command.flags
                                & (1 as libc::c_int) << 6 as libc::c_int != 0
                            {
                                cmdfmt = hi_sdscat(
                                    cmdfmt,
                                    b" REPLACE\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            cursor = -(999 as libc::c_int);
                            timeout = config.cluster_manager_command.timeout;
                            's_266: while cursor != 0 as libc::c_int {
                                if cursor < 0 as libc::c_int {
                                    cursor = 0 as libc::c_int;
                                }
                                freeReplyObject(src_reply as *mut libc::c_void);
                                src_reply = reconnectingRedisCommand(
                                    src_ctx,
                                    b"SCAN %d COUNT %d\0" as *const u8 as *const libc::c_char,
                                    cursor,
                                    1000 as libc::c_int,
                                );
                                if src_reply.is_null()
                                    || (*src_reply).type_0 == 6 as libc::c_int
                                {
                                    if !src_reply.is_null() && !((*src_reply).str_0).is_null() {
                                        reply_err = (*src_reply).str_0;
                                    }
                                    success = 0 as libc::c_int;
                                    break;
                                } else {
                                    if (*src_reply).type_0 == 2 as libc::c_int {} else {
                                        __assert_fail(
                                            b"src_reply->type == REDIS_REPLY_ARRAY\0" as *const u8
                                                as *const libc::c_char,
                                            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                            7207 as libc::c_int as libc::c_uint,
                                            (*core::mem::transmute::<
                                                &[u8; 46],
                                                &[libc::c_char; 46],
                                            >(b"int clusterManagerCommandImport(int, char **)\0"))
                                                .as_ptr(),
                                        );
                                    };
                                    if (*src_reply).elements
                                        >= 2 as libc::c_int as libc::c_ulong
                                    {} else {
                                        __assert_fail(
                                            b"src_reply->elements >= 2\0" as *const u8
                                                as *const libc::c_char,
                                            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                            7208 as libc::c_int as libc::c_uint,
                                            (*core::mem::transmute::<
                                                &[u8; 46],
                                                &[libc::c_char; 46],
                                            >(b"int clusterManagerCommandImport(int, char **)\0"))
                                                .as_ptr(),
                                        );
                                    };
                                    if (**((*src_reply).element)
                                        .offset(1 as libc::c_int as isize))
                                        .type_0 == 2 as libc::c_int
                                    {} else {
                                        __assert_fail(
                                            b"src_reply->element[1]->type == REDIS_REPLY_ARRAY\0"
                                                as *const u8 as *const libc::c_char,
                                            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                            7209 as libc::c_int as libc::c_uint,
                                            (*core::mem::transmute::<
                                                &[u8; 46],
                                                &[libc::c_char; 46],
                                            >(b"int clusterManagerCommandImport(int, char **)\0"))
                                                .as_ptr(),
                                        );
                                    };
                                    if (**((*src_reply).element)
                                        .offset(0 as libc::c_int as isize))
                                        .type_0 == 1 as libc::c_int
                                    {
                                        cursor = atoi(
                                            (**((*src_reply).element).offset(0 as libc::c_int as isize))
                                                .str_0,
                                        );
                                    } else if (**((*src_reply).element)
                                        .offset(0 as libc::c_int as isize))
                                        .type_0 == 3 as libc::c_int
                                    {
                                        cursor = (**((*src_reply).element)
                                            .offset(0 as libc::c_int as isize))
                                            .integer as libc::c_int;
                                    }
                                    let mut keycount: libc::c_int = (**((*src_reply).element)
                                        .offset(1 as libc::c_int as isize))
                                        .elements as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < keycount {
                                        let mut kr: *mut redisReply = *((**((*src_reply).element)
                                            .offset(1 as libc::c_int as isize))
                                            .element)
                                            .offset(i as isize);
                                        if (*kr).type_0 == 1 as libc::c_int {} else {
                                            __assert_fail(
                                                b"kr->type == REDIS_REPLY_STRING\0" as *const u8
                                                    as *const libc::c_char,
                                                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                                                7217 as libc::c_int as libc::c_uint,
                                                (*core::mem::transmute::<
                                                    &[u8; 46],
                                                    &[libc::c_char; 46],
                                                >(b"int clusterManagerCommandImport(int, char **)\0"))
                                                    .as_ptr(),
                                            );
                                        };
                                        let mut key: *mut libc::c_char = (*kr).str_0;
                                        let mut slot: uint16_t = clusterManagerKeyHashSlot(
                                            key,
                                            (*kr).len as libc::c_int,
                                        ) as uint16_t;
                                        let mut target: *mut clusterManagerNode = slots_map[slot
                                            as usize];
                                        printf(
                                            b"Migrating %s to %s:%d: \0" as *const u8
                                                as *const libc::c_char,
                                            key,
                                            (*target).ip,
                                            (*target).port,
                                        );
                                        let mut r: *mut redisReply = reconnectingRedisCommand(
                                            src_ctx,
                                            cmdfmt as *const libc::c_char,
                                            (*target).ip,
                                            (*target).port,
                                            key,
                                            0 as libc::c_int,
                                            timeout,
                                        );
                                        if r.is_null() || (*r).type_0 == 6 as libc::c_int {
                                            if !r.is_null() && !((*r).str_0).is_null() {
                                                clusterManagerLog(
                                                    3 as libc::c_int,
                                                    b"Source %s:%d replied with error:\n%s\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    src_ip,
                                                    src_port,
                                                    (*r).str_0,
                                                );
                                            }
                                            success = 0 as libc::c_int;
                                        }
                                        freeReplyObject(r as *mut libc::c_void);
                                        if success == 0 {
                                            break 's_266;
                                        }
                                        clusterManagerLog(
                                            4 as libc::c_int,
                                            b"OK\n\0" as *const u8 as *const libc::c_char,
                                        );
                                        i += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !reply_err.is_null() {
                clusterManagerLog(
                    3 as libc::c_int,
                    b"Source %s:%d replied with error:\n%s\n\0" as *const u8
                        as *const libc::c_char,
                    src_ip,
                    src_port,
                    reply_err,
                );
            }
            if !src_ctx.is_null() {
                redisFree(src_ctx);
            }
            if !src_reply.is_null() {
                freeReplyObject(src_reply as *mut libc::c_void);
            }
            if !cmdfmt.is_null() {
                hi_sdsfree(cmdfmt);
            }
            return success;
        }
    }
    fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, invalid_args_msg);
    return 0 as libc::c_int;
}
unsafe extern "C" fn clusterManagerCommandCall(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut refnode: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut argvlen: *mut size_t = 0 as *mut size_t;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    if getClusterHostFromCmdArgs(1 as libc::c_int, argv, &mut ip, &mut port) == 0 {
        fprintf(
            stderr,
            b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        refnode = clusterManagerNewNode(ip, port, 0 as libc::c_int);
        if clusterManagerLoadInfoFromNode(refnode) == 0 {
            return 0 as libc::c_int;
        }
        argc -= 1;
        argv = argv.offset(1);
        argvlen = zmalloc(
            (argc as libc::c_ulong)
                .wrapping_mul(core::mem::size_of::<size_t>() as libc::c_ulong),
        ) as *mut size_t;
        clusterManagerLog(
            1 as libc::c_int,
            b">>> Calling\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < argc {
            *argvlen.offset(i as isize) = strlen(*argv.offset(i as isize));
            printf(
                b" %s\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            );
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        li = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        ln = 0 as *mut listNode;
        listRewind(cluster_manager.nodes, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut n: *mut clusterManagerNode = (*ln).value as *mut clusterManagerNode;
            if config.cluster_manager_command.flags
                & (1 as libc::c_int) << 11 as libc::c_int != 0
                && !((*n).replicate).is_null()
            {
                continue;
            }
            if config.cluster_manager_command.flags
                & (1 as libc::c_int) << 12 as libc::c_int != 0
                && ((*n).replicate).is_null()
            {
                continue;
            }
            if ((*n).context).is_null() && clusterManagerNodeConnect(n) == 0 {
                continue;
            }
            let mut reply: *mut redisReply = 0 as *mut redisReply;
            redisAppendCommandArgv(
                (*n).context,
                argc,
                argv as *mut *const libc::c_char,
                argvlen,
            );
            let mut status: libc::c_int = redisGetReply(
                (*n).context,
                &mut reply as *mut *mut redisReply as *mut *mut libc::c_void,
            );
            if status != 0 as libc::c_int || reply.is_null() {
                printf(
                    b"%s:%d: Failed!\n\0" as *const u8 as *const libc::c_char,
                    (*n).ip,
                    (*n).port,
                );
            } else {
                let mut formatted_reply: hisds = cliFormatReplyRaw(reply);
                printf(
                    b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                    (*n).ip,
                    (*n).port,
                    formatted_reply,
                );
                hi_sdsfree(formatted_reply);
            }
            if !reply.is_null() {
                freeReplyObject(reply as *mut libc::c_void);
            }
        }
        zfree(argvlen as *mut libc::c_void);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn clusterManagerCommandBackup(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut refnode: *mut clusterManagerNode = 0 as *mut clusterManagerNode;
    let mut no_issues: libc::c_int = 0;
    let mut cluster_errors_count: libc::c_int = 0;
    let mut json: hisds = 0 as *mut libc::c_char;
    let mut first_node: libc::c_int = 0;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut jsonpath: hisds = 0 as *mut libc::c_char;
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut success: libc::c_int = 1 as libc::c_int;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    if getClusterHostFromCmdArgs(1 as libc::c_int, argv, &mut ip, &mut port) == 0 {
        fprintf(
            stderr,
            b"[ERR] Invalid arguments: you need to pass either a valid address (ie. 120.0.0.1:7000) or space separated IP and port (ie. 120.0.0.1 7000)\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        refnode = clusterManagerNewNode(ip, port, 0 as libc::c_int);
        if clusterManagerLoadInfoFromNode(refnode) == 0 {
            return 0 as libc::c_int;
        }
        no_issues = clusterManagerCheckCluster(0 as libc::c_int);
        cluster_errors_count = (if no_issues != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            (*cluster_manager.errors).len
        }) as libc::c_int;
        config
            .cluster_manager_command
            .backup_dir = *argv.offset(1 as libc::c_int as isize);
        json = hi_sdsnew(b"[\n\0" as *const u8 as *const libc::c_char);
        first_node = 0 as libc::c_int;
        li = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        ln = 0 as *mut listNode;
        listRewind(cluster_manager.nodes, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            if first_node == 0 {
                first_node = 1 as libc::c_int;
            } else {
                json = hi_sdscat(json, b",\n\0" as *const u8 as *const libc::c_char);
            }
            let mut node: *mut clusterManagerNode = (*ln).value
                as *mut clusterManagerNode;
            let mut node_json: hisds = clusterManagerNodeGetJSON(
                node,
                cluster_errors_count as libc::c_ulong,
            );
            json = hi_sdscat(json, node_json as *const libc::c_char);
            hi_sdsfree(node_json);
            if !((*node).replicate).is_null() {
                continue;
            }
            clusterManagerLog(
                1 as libc::c_int,
                b">>> Node %s:%d -> Saving RDB...\n\0" as *const u8
                    as *const libc::c_char,
                (*node).ip,
                (*node).port,
            );
            fflush(stdout);
            getRDB(node);
        }
        json = hi_sdscat(json, b"\n]\0" as *const u8 as *const libc::c_char);
        jsonpath = hi_sdsnew(config.cluster_manager_command.backup_dir);
        if *jsonpath
            .offset(
                (hi_sdslen(jsonpath)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as libc::c_int != '/' as i32
        {
            jsonpath = hi_sdscat(jsonpath, b"/\0" as *const u8 as *const libc::c_char);
        }
        jsonpath = hi_sdscat(
            jsonpath,
            b"nodes.json\0" as *const u8 as *const libc::c_char,
        );
        fflush(stdout);
        clusterManagerLog(
            1 as libc::c_int,
            b"Saving cluster configuration to: %s\n\0" as *const u8
                as *const libc::c_char,
            jsonpath,
        );
        out = fopen(
            jsonpath as *const libc::c_char,
            b"w+\0" as *const u8 as *const libc::c_char,
        );
        if out.is_null() {
            clusterManagerLog(
                3 as libc::c_int,
                b"Could not save nodes to: %s\n\0" as *const u8 as *const libc::c_char,
                jsonpath,
            );
            success = 0 as libc::c_int;
        } else {
            fputs(json as *const libc::c_char, out);
            fclose(out);
        }
        hi_sdsfree(json);
        hi_sdsfree(jsonpath);
        if success != 0 {
            if no_issues == 0 {
                clusterManagerLog(
                    2 as libc::c_int,
                    b"*** Cluster seems to have some problems, please be aware of it if you're going to restore this backup.\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            clusterManagerLog(
                4 as libc::c_int,
                b"[OK] Backup created into: %s\n\0" as *const u8 as *const libc::c_char,
                config.cluster_manager_command.backup_dir,
            );
        } else {
            clusterManagerLog(
                4 as libc::c_int,
                b"[ERR] Failed to back cluster!\n\0" as *const u8 as *const libc::c_char,
            );
        }
        return success;
    };
}
unsafe extern "C" fn clusterManagerCommandHelp(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut commands_count: libc::c_int = (core::mem::size_of::<
        [clusterManagerCommandDef; 13],
    >() as libc::c_ulong)
        .wrapping_div(
            core::mem::size_of::<clusterManagerCommandDef>() as libc::c_ulong,
        ) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    fprintf(
        stdout,
        b"Cluster Manager Commands:\n\0" as *const u8 as *const libc::c_char,
    );
    let mut padding: libc::c_int = 15 as libc::c_int;
    while i < commands_count {
        let mut def: *mut clusterManagerCommandDef = &mut *clusterManagerCommands
            .as_mut_ptr()
            .offset(i as isize) as *mut clusterManagerCommandDef;
        let mut namelen: libc::c_int = strlen((*def).name) as libc::c_int;
        let mut padlen: libc::c_int = padding - namelen;
        fprintf(stdout, b"  %s\0" as *const u8 as *const libc::c_char, (*def).name);
        j = 0 as libc::c_int;
        while j < padlen {
            fprintf(stdout, b" \0" as *const u8 as *const libc::c_char);
            j += 1;
        }
        fprintf(
            stdout,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            if !((*def).args).is_null() {
                (*def).args as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        if !((*def).options).is_null() {
            let mut optslen: libc::c_int = strlen((*def).options) as libc::c_int;
            let mut p: *mut libc::c_char = (*def).options;
            let mut eos: *mut libc::c_char = p.offset(optslen as isize);
            let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
            loop {
                comma = strchr(p, ',' as i32);
                if comma.is_null() {
                    break;
                }
                let mut deflen: libc::c_int = comma.offset_from(p) as libc::c_long
                    as libc::c_int;
                let mut buf: [libc::c_char; 255] = [0; 255];
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    p as *const libc::c_void,
                    deflen as libc::c_ulong,
                );
                buf[deflen as usize] = '\0' as i32 as libc::c_char;
                j = 0 as libc::c_int;
                while j < padding {
                    fprintf(stdout, b" \0" as *const u8 as *const libc::c_char);
                    j += 1;
                }
                fprintf(
                    stdout,
                    b"  --cluster-%s\n\0" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr(),
                );
                p = comma.offset(1 as libc::c_int as isize);
                if p >= eos {
                    break;
                }
            }
            if p < eos {
                j = 0 as libc::c_int;
                while j < padding {
                    fprintf(stdout, b" \0" as *const u8 as *const libc::c_char);
                    j += 1;
                }
                fprintf(
                    stdout,
                    b"  --cluster-%s\n\0" as *const u8 as *const libc::c_char,
                    p,
                );
            }
        }
        i += 1;
    }
    fprintf(
        stdout,
        b"\nFor check, fix, reshard, del-node, set-timeout, info, rebalance, call, import, backup you can specify the host and port of any working node in the cluster.\n\0"
            as *const u8 as *const libc::c_char,
    );
    let mut options_count: libc::c_int = (core::mem::size_of::<
        [clusterManagerOptionDef; 1],
    >() as libc::c_ulong)
        .wrapping_div(core::mem::size_of::<clusterManagerOptionDef>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    fprintf(
        stdout,
        b"\nCluster Manager Options:\n\0" as *const u8 as *const libc::c_char,
    );
    while i < options_count {
        let mut def_0: *mut clusterManagerOptionDef = &mut *clusterManagerOptions
            .as_mut_ptr()
            .offset(i as isize) as *mut clusterManagerOptionDef;
        let mut namelen_0: libc::c_int = strlen((*def_0).name) as libc::c_int;
        let mut padlen_0: libc::c_int = padding - namelen_0;
        fprintf(stdout, b"  %s\0" as *const u8 as *const libc::c_char, (*def_0).name);
        j = 0 as libc::c_int;
        while j < padlen_0 {
            fprintf(stdout, b" \0" as *const u8 as *const libc::c_char);
            j += 1;
        }
        fprintf(stdout, b"%s\n\0" as *const u8 as *const libc::c_char, (*def_0).desc);
        i += 1;
    }
    fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn latencyModePrint(
    mut min: libc::c_longlong,
    mut max: libc::c_longlong,
    mut avg: libc::c_double,
    mut count: libc::c_longlong,
) {
    if config.output == 0 as libc::c_int {
        printf(
            b"min: %lld, max: %lld, avg: %.2f (%lld samples)\0" as *const u8
                as *const libc::c_char,
            min,
            max,
            avg,
            count,
        );
        fflush(stdout);
    } else if config.output == 2 as libc::c_int {
        printf(
            b"%lld,%lld,%.2f,%lld\n\0" as *const u8 as *const libc::c_char,
            min,
            max,
            avg,
            count,
        );
    } else if config.output == 1 as libc::c_int {
        printf(
            b"%lld %lld %.2f %lld\n\0" as *const u8 as *const libc::c_char,
            min,
            max,
            avg,
            count,
        );
    } else if config.output == 3 as libc::c_int {
        printf(
            b"{\"min\": %lld, \"max\": %lld, \"avg\": %.2f, \"count\": %lld}\n\0"
                as *const u8 as *const libc::c_char,
            min,
            max,
            avg,
            count,
        );
    }
}
unsafe extern "C" fn latencyMode() {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut start: libc::c_longlong = 0;
    let mut latency: libc::c_longlong = 0;
    let mut min: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut max: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut tot: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut history_interval: libc::c_longlong = (if config.interval != 0 {
        config.interval / 1000 as libc::c_int as libc::c_long
    } else {
        15000 as libc::c_int as libc::c_long
    }) as libc::c_longlong;
    let mut avg: libc::c_double = 0.;
    let mut history_start: libc::c_longlong = mstime();
    if config.interval == 0 as libc::c_int as libc::c_long {
        config.interval = 1000 as libc::c_int as libc::c_long;
    } else {
        config.interval /= 1000 as libc::c_int as libc::c_long;
    }
    if context.is_null() {
        exit(1 as libc::c_int);
    }
    loop {
        start = mstime();
        reply = reconnectingRedisCommand(
            context,
            b"PING\0" as *const u8 as *const libc::c_char,
        );
        if reply.is_null() {
            fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        latency = mstime() - start;
        freeReplyObject(reply as *mut libc::c_void);
        count += 1;
        if count == 1 as libc::c_int as libc::c_longlong {
            tot = latency;
            max = tot;
            min = max;
            avg = latency as libc::c_double;
        } else {
            if latency < min {
                min = latency;
            }
            if latency > max {
                max = latency;
            }
            tot += latency;
            avg = tot as libc::c_double / count as libc::c_double;
        }
        if config.output == 0 as libc::c_int {
            printf(b"\x1B[0G\x1B[2K\0" as *const u8 as *const libc::c_char);
            latencyModePrint(min, max, avg, count);
        } else if config.latency_history != 0 {
            latencyModePrint(min, max, avg, count);
        } else if mstime() - history_start > config.interval as libc::c_longlong {
            latencyModePrint(min, max, avg, count);
            exit(0 as libc::c_int);
        }
        if config.latency_history != 0 && mstime() - history_start > history_interval {
            printf(
                b" -- %.2f seconds range\n\0" as *const u8 as *const libc::c_char,
                ((mstime() - history_start) as libc::c_float
                    / 1000 as libc::c_int as libc::c_float) as libc::c_double,
            );
            history_start = mstime();
            count = 0 as libc::c_int as libc::c_longlong;
            tot = count;
            max = tot;
            min = max;
        }
        usleep((10 as libc::c_int * 1000 as libc::c_int) as __useconds_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn showLatencyDistSamples(
    mut samples: *mut distsamples,
    mut tot: libc::c_longlong,
) {
    let mut j: libc::c_int = 0;
    printf(b"\x1B[38;5;0m\0" as *const u8 as *const libc::c_char);
    j = 0 as libc::c_int;
    loop {
        let mut coloridx: libc::c_int = ceil(
            (*samples.offset(j as isize)).count as libc::c_double / tot as libc::c_double
                * (spectrum_palette_size - 1 as libc::c_int) as libc::c_double,
        ) as libc::c_int;
        let mut color: libc::c_int = *spectrum_palette.offset(coloridx as isize);
        printf(
            b"\x1B[48;5;%dm%c\0" as *const u8 as *const libc::c_char,
            color,
            (*samples.offset(j as isize)).character,
        );
        (*samples.offset(j as isize)).count = 0 as libc::c_int as libc::c_longlong;
        if (*samples.offset(j as isize)).max == 0 as libc::c_int as libc::c_longlong {
            break;
        }
        j += 1;
    }
    printf(b"\x1B[0m\n\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn showLatencyDistLegend() {
    let mut j: libc::c_int = 0;
    printf(
        b"---------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b". - * #          .01 .125 .25 .5 milliseconds\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"1,2,3,...,9      from 1 to 9     milliseconds\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"A,B,C,D,E        10,20,30,40,50  milliseconds\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"F,G,H,I,J        .1,.2,.3,.4,.5       seconds\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"K,L,M,N,O,P,Q,?  1,2,4,8,16,30,60,>60 seconds\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"From 0 to 100%%: \0" as *const u8 as *const libc::c_char);
    j = 0 as libc::c_int;
    while j < spectrum_palette_size {
        printf(
            b"\x1B[48;5;%dm \0" as *const u8 as *const libc::c_char,
            *spectrum_palette.offset(j as isize),
        );
        j += 1;
    }
    printf(b"\x1B[0m\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"---------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn latencyDistMode() {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut start: libc::c_longlong = 0;
    let mut latency: libc::c_longlong = 0;
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut history_interval: libc::c_longlong = (if config.interval != 0 {
        config.interval / 1000 as libc::c_int as libc::c_long
    } else {
        1000 as libc::c_int as libc::c_long
    }) as libc::c_longlong;
    let mut history_start: libc::c_longlong = ustime();
    let mut j: libc::c_int = 0;
    let mut outputs: libc::c_int = 0 as libc::c_int;
    let mut samples: [distsamples; 31] = [
        {
            let mut init = distsamples {
                max: 10 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '.' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 125 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '-' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 250 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '*' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 500 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '#' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 1000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '1' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 2000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '2' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 3000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '3' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 4000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '4' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 5000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '5' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 6000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '6' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 7000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '7' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 8000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '8' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 9000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '9' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 10000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'A' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 20000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'B' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 30000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'C' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 40000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'D' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 50000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'E' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 100000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'F' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 200000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'G' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 300000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'H' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 400000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'I' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 500000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'J' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 1000000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'K' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 2000000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'L' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 4000000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'M' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 8000000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'N' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 16000000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'O' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 30000000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'P' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 60000000 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: 'Q' as i32,
            };
            init
        },
        {
            let mut init = distsamples {
                max: 0 as libc::c_int as libc::c_longlong,
                count: 0 as libc::c_int as libc::c_longlong,
                character: '?' as i32,
            };
            init
        },
    ];
    if context.is_null() {
        exit(1 as libc::c_int);
    }
    loop {
        start = ustime();
        reply = reconnectingRedisCommand(
            context,
            b"PING\0" as *const u8 as *const libc::c_char,
        );
        if reply.is_null() {
            fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        latency = ustime() - start;
        freeReplyObject(reply as *mut libc::c_void);
        count += 1;
        j = 0 as libc::c_int;
        loop {
            if samples[j as usize].max == 0 as libc::c_int as libc::c_longlong
                || latency <= samples[j as usize].max
            {
                samples[j as usize].count += 1;
                break;
            } else {
                j += 1;
            }
        }
        if count != 0
            && (ustime() - history_start) / 1000 as libc::c_int as libc::c_longlong
                > history_interval
        {
            let fresh75 = outputs;
            outputs = outputs + 1;
            if fresh75 % 20 as libc::c_int == 0 as libc::c_int {
                showLatencyDistLegend();
            }
            showLatencyDistSamples(samples.as_mut_ptr(), count);
            history_start = ustime();
            count = 0 as libc::c_int as libc::c_longlong;
        }
        usleep((10 as libc::c_int * 1000 as libc::c_int) as __useconds_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sendReplconf(
    mut arg1: *const libc::c_char,
    mut arg2: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 1 as libc::c_int;
    fprintf(
        stderr,
        b"sending REPLCONF %s %s\n\0" as *const u8 as *const libc::c_char,
        arg1,
        arg2,
    );
    let mut reply: *mut redisReply = redisCommand(
        context,
        b"REPLCONF %s %s\0" as *const u8 as *const libc::c_char,
        arg1,
        arg2,
    ) as *mut redisReply;
    if reply.is_null() {
        fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    } else {
        if (*reply).type_0 == 6 as libc::c_int {
            fprintf(
                stderr,
                b"REPLCONF %s error: %s\n\0" as *const u8 as *const libc::c_char,
                arg1,
                (*reply).str_0,
            );
            res = 0 as libc::c_int;
        }
    }
    freeReplyObject(reply as *mut libc::c_void);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn sendCapa() {
    sendReplconf(
        b"capa\0" as *const u8 as *const libc::c_char,
        b"eof\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sendRdbOnly() {
    sendReplconf(
        b"rdb-only\0" as *const u8 as *const libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn readConn(
    mut c: *mut redisContext,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> ssize_t {
    return ((*(*c).funcs).read).expect("non-null function pointer")(c, buf, len);
}
#[no_mangle]
pub unsafe extern "C" fn sendSync(
    mut c: *mut redisContext,
    mut out_eof: *mut libc::c_char,
) -> libc::c_ulonglong {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nread: ssize_t = 0;
    if cliWriteConn(
        c,
        b"SYNC\r\n\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    ) != 6 as libc::c_int as libc::c_long
    {
        fprintf(
            stderr,
            b"Error writing to master\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    p = buf.as_mut_ptr();
    loop {
        nread = readConn(c, p, 1 as libc::c_int as size_t);
        if nread <= 0 as libc::c_int as libc::c_long {
            fprintf(
                stderr,
                b"Error reading bulk length while SYNCing\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if *p as libc::c_int == '\n' as i32 && p != buf.as_mut_ptr() {
            break;
        }
        if *p as libc::c_int != '\n' as i32 {
            p = p.offset(1);
        }
    }
    *p = '\0' as i32 as libc::c_char;
    if buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
        fprintf(
            stderr,
            b"SYNC with master failed: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    if strncmp(
        buf.as_mut_ptr().offset(1 as libc::c_int as isize),
        b"EOF:\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && strlen(buf.as_mut_ptr().offset(5 as libc::c_int as isize))
            >= 40 as libc::c_int as libc::c_ulong
    {
        memcpy(
            out_eof as *mut libc::c_void,
            buf.as_mut_ptr().offset(5 as libc::c_int as isize) as *const libc::c_void,
            40 as libc::c_int as libc::c_ulong,
        );
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    return strtoull(
        buf.as_mut_ptr().offset(1 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
unsafe extern "C" fn slaveMode() {
    static mut eofmark: [libc::c_char; 40] = [0; 40];
    static mut lastbytes: [libc::c_char; 40] = [0; 40];
    static mut usemark: libc::c_int = 0 as libc::c_int;
    let mut payload: libc::c_ulonglong = sendSync(context, eofmark.as_mut_ptr());
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut original_output: libc::c_int = config.output;
    if payload == 0 as libc::c_int as libc::c_ulonglong {
        payload = (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong);
        memset(
            lastbytes.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            40 as libc::c_int as libc::c_ulong,
        );
        usemark = 1 as libc::c_int;
        fprintf(
            stderr,
            b"SYNC with master, discarding bytes of bulk transfer until EOF marker...\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            stderr,
            b"SYNC with master, discarding %llu bytes of bulk transfer...\n\0"
                as *const u8 as *const libc::c_char,
            payload,
        );
    }
    while payload != 0 {
        let mut nread: ssize_t = 0;
        nread = readConn(
            context,
            buf.as_mut_ptr(),
            (if payload
                > core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_ulonglong
            {
                core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_ulonglong
            } else {
                payload
            }) as size_t,
        );
        if nread <= 0 as libc::c_int as libc::c_long {
            fprintf(
                stderr,
                b"Error reading RDB payload while SYNCing\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        payload = payload.wrapping_sub(nread as libc::c_ulonglong);
        if !(usemark != 0) {
            continue;
        }
        if nread >= 40 as libc::c_int as libc::c_long {
            memcpy(
                lastbytes.as_mut_ptr() as *mut libc::c_void,
                buf
                    .as_mut_ptr()
                    .offset(nread as isize)
                    .offset(-(40 as libc::c_int as isize)) as *const libc::c_void,
                40 as libc::c_int as libc::c_ulong,
            );
        } else {
            let mut rem: libc::c_int = (40 as libc::c_int as libc::c_long - nread)
                as libc::c_int;
            memmove(
                lastbytes.as_mut_ptr() as *mut libc::c_void,
                lastbytes.as_mut_ptr().offset(nread as isize) as *const libc::c_void,
                rem as libc::c_ulong,
            );
            memcpy(
                lastbytes.as_mut_ptr().offset(rem as isize) as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                nread as libc::c_ulong,
            );
        }
        if memcmp(
            lastbytes.as_mut_ptr() as *const libc::c_void,
            eofmark.as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            break;
        }
    }
    if usemark != 0 {
        let mut offset: libc::c_ulonglong = (9223372036854775807 as libc::c_longlong
            as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_sub(payload);
        fprintf(
            stderr,
            b"SYNC done after %llu bytes. Logging commands from master.\n\0" as *const u8
                as *const libc::c_char,
            offset,
        );
        sleep(1 as libc::c_int as libc::c_uint);
        sendReplconf(
            b"ACK\0" as *const u8 as *const libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            stderr,
            b"SYNC done. Logging commands from master.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    config.output = 2 as libc::c_int;
    while cliReadReply(0 as libc::c_int) == 0 as libc::c_int {}
    config.output = original_output;
}
unsafe extern "C" fn getRDB(mut node: *mut clusterManagerNode) {
    let mut fd: libc::c_int = 0;
    let mut s: *mut redisContext = 0 as *mut redisContext;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if !node.is_null() {
        if !((*node).context).is_null() {} else {
            __assert_fail(
                b"node->context\0" as *const u8 as *const libc::c_char,
                b"redis-cli.c\0" as *const u8 as *const libc::c_char,
                7787 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void getRDB(clusterManagerNode *)\0"))
                    .as_ptr(),
            );
        };
        s = (*node).context;
        filename = clusterManagerGetNodeRDBFilename(node);
    } else {
        s = context;
        filename = config.rdb_filename;
    }
    static mut eofmark: [libc::c_char; 40] = [0; 40];
    static mut lastbytes: [libc::c_char; 40] = [0; 40];
    static mut usemark: libc::c_int = 0 as libc::c_int;
    let mut payload: libc::c_ulonglong = sendSync(s, eofmark.as_mut_ptr());
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    if payload == 0 as libc::c_int as libc::c_ulonglong {
        payload = (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong);
        memset(
            lastbytes.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            40 as libc::c_int as libc::c_ulong,
        );
        usemark = 1 as libc::c_int;
        fprintf(
            stderr,
            b"SYNC sent to master, writing bytes of bulk transfer until EOF marker to '%s'\n\0"
                as *const u8 as *const libc::c_char,
            filename,
        );
    } else {
        fprintf(
            stderr,
            b"SYNC sent to master, writing %llu bytes to '%s'\n\0" as *const u8
                as *const libc::c_char,
            payload,
            filename,
        );
    }
    let mut write_to_stdout: libc::c_int = (strcmp(
        filename,
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0) as libc::c_int;
    if write_to_stdout != 0 {
        fd = 1 as libc::c_int;
    } else {
        fd = open(
            filename,
            0o100 as libc::c_int | 0o1 as libc::c_int,
            0o644 as libc::c_int,
        );
        if fd == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"Error opening '%s': %s\n\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
            exit(1 as libc::c_int);
        }
    }
    while payload != 0 {
        let mut nread: ssize_t = 0;
        let mut nwritten: ssize_t = 0;
        nread = readConn(
            s,
            buf.as_mut_ptr(),
            (if payload
                > core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_ulonglong
            {
                core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_ulonglong
            } else {
                payload
            }) as size_t,
        );
        if nread <= 0 as libc::c_int as libc::c_long {
            fprintf(
                stderr,
                b"I/O Error reading RDB payload from socket\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        nwritten = write(fd, buf.as_mut_ptr() as *const libc::c_void, nread as size_t);
        if nwritten != nread {
            fprintf(
                stderr,
                b"Error writing data to file: %s\n\0" as *const u8
                    as *const libc::c_char,
                if nwritten == -(1 as libc::c_int) as libc::c_long {
                    strerror(*__errno_location()) as *const libc::c_char
                } else {
                    b"short write\0" as *const u8 as *const libc::c_char
                },
            );
            exit(1 as libc::c_int);
        }
        payload = payload.wrapping_sub(nread as libc::c_ulonglong);
        if !(usemark != 0) {
            continue;
        }
        if nread >= 40 as libc::c_int as libc::c_long {
            memcpy(
                lastbytes.as_mut_ptr() as *mut libc::c_void,
                buf
                    .as_mut_ptr()
                    .offset(nread as isize)
                    .offset(-(40 as libc::c_int as isize)) as *const libc::c_void,
                40 as libc::c_int as libc::c_ulong,
            );
        } else {
            let mut rem: libc::c_int = (40 as libc::c_int as libc::c_long - nread)
                as libc::c_int;
            memmove(
                lastbytes.as_mut_ptr() as *mut libc::c_void,
                lastbytes.as_mut_ptr().offset(nread as isize) as *const libc::c_void,
                rem as libc::c_ulong,
            );
            memcpy(
                lastbytes.as_mut_ptr().offset(rem as isize) as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                nread as libc::c_ulong,
            );
        }
        if memcmp(
            lastbytes.as_mut_ptr() as *const libc::c_void,
            eofmark.as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            break;
        }
    }
    if usemark != 0 {
        payload = (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_sub(payload)
            .wrapping_sub(40 as libc::c_int as libc::c_ulonglong);
        if write_to_stdout == 0
            && ftruncate(fd, payload as __off64_t) == -(1 as libc::c_int)
        {
            fprintf(
                stderr,
                b"ftruncate failed: %s.\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        fprintf(
            stderr,
            b"Transfer finished with success after %llu bytes\n\0" as *const u8
                as *const libc::c_char,
            payload,
        );
    } else {
        fprintf(
            stderr,
            b"Transfer finished with success.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    redisFree(s);
    if !node.is_null() {
        (*node).context = 0 as *mut redisContext;
    }
    if write_to_stdout == 0 && fsync(fd) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"Fail to fsync '%s': %s\n\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    close(fd);
    if !node.is_null() {
        hi_sdsfree(filename);
        return;
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn pipeMode() {
    let mut errors: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut replies: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut obuf_len: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut obuf_pos: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut obuf: [libc::c_char; 16384] = [0; 16384];
    let mut aneterr: [libc::c_char; 256] = [0; 256];
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut eof: libc::c_int = 0 as libc::c_int;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut magic: [libc::c_char; 20] = [0; 20];
    let mut last_read_time: time_t = time(0 as *mut time_t);
    srand(time(0 as *mut time_t) as libc::c_uint);
    if anetNonBlock(aneterr.as_mut_ptr(), (*context).fd) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"Can't set the socket in non blocking mode: %s\n\0" as *const u8
                as *const libc::c_char,
            aneterr.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    (*context).flags &= !(0x1 as libc::c_int);
    while done == 0 {
        let mut mask: libc::c_int = 1 as libc::c_int;
        if eof == 0 || obuf_len != 0 as libc::c_int as libc::c_longlong {
            mask |= 2 as libc::c_int;
        }
        mask = aeWait((*context).fd, mask, 1000 as libc::c_int as libc::c_longlong);
        if mask & 1 as libc::c_int != 0 {
            let mut read_error: libc::c_int = 0 as libc::c_int;
            loop {
                if read_error == 0 && redisBufferRead(context) == -(1 as libc::c_int) {
                    read_error = 1 as libc::c_int;
                }
                reply = 0 as *mut redisReply;
                if redisGetReply(
                    context,
                    &mut reply as *mut *mut redisReply as *mut *mut libc::c_void,
                ) == -(1 as libc::c_int)
                {
                    fprintf(
                        stderr,
                        b"Error reading replies from server\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                if !reply.is_null() {
                    last_read_time = time(0 as *mut time_t);
                    if (*reply).type_0 == 6 as libc::c_int {
                        fprintf(
                            stderr,
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            (*reply).str_0,
                        );
                        errors += 1;
                    } else if eof != 0 && (*reply).type_0 == 1 as libc::c_int
                        && (*reply).len == 20 as libc::c_int as libc::c_ulong
                    {
                        if memcmp(
                            (*reply).str_0 as *const libc::c_void,
                            magic.as_mut_ptr() as *const libc::c_void,
                            20 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            printf(
                                b"Last reply received from server.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            done = 1 as libc::c_int;
                            replies -= 1;
                        }
                    }
                    replies += 1;
                    freeReplyObject(reply as *mut libc::c_void);
                }
                if reply.is_null() {
                    break;
                }
            }
            if read_error != 0 {
                exit(1 as libc::c_int);
            }
        }
        if mask & 2 as libc::c_int != 0 {
            let mut loop_nwritten: ssize_t = 0 as libc::c_int as ssize_t;
            loop {
                if obuf_len != 0 as libc::c_int as libc::c_longlong {
                    let mut nwritten: ssize_t = cliWriteConn(
                        context,
                        obuf.as_mut_ptr().offset(obuf_pos as isize),
                        obuf_len as size_t,
                    );
                    if nwritten == -(1 as libc::c_int) as libc::c_long {
                        if *__errno_location() != 11 as libc::c_int
                            && *__errno_location() != 4 as libc::c_int
                        {
                            fprintf(
                                stderr,
                                b"Error writing to the server: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                strerror(*__errno_location()),
                            );
                            exit(1 as libc::c_int);
                        } else {
                            nwritten = 0 as libc::c_int as ssize_t;
                        }
                    }
                    obuf_len -= nwritten as libc::c_longlong;
                    obuf_pos += nwritten as libc::c_longlong;
                    loop_nwritten += nwritten;
                    if obuf_len != 0 as libc::c_int as libc::c_longlong {
                        break;
                    }
                }
                if (*context).err != 0 {
                    fprintf(
                        stderr,
                        b"Server I/O Error: %s\n\0" as *const u8 as *const libc::c_char,
                        ((*context).errstr).as_mut_ptr(),
                    );
                    exit(1 as libc::c_int);
                }
                if obuf_len == 0 as libc::c_int as libc::c_longlong && eof == 0 {
                    let mut nread: ssize_t = read(
                        0 as libc::c_int,
                        obuf.as_mut_ptr() as *mut libc::c_void,
                        core::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
                    );
                    if nread == 0 as libc::c_int as libc::c_long {
                        let mut echo: [libc::c_char; 44] = {
                            let echo_bytes: &[u8; 44] = b"\r\n*2\r\n$4\r\nECHO\r\n$20\r\n01234567890123456789\r\n\0";
                            let echo_chars: &mut [libc::c_char; 44] = unsafe {
                                &mut *(&echo_bytes as *const _ as *mut [libc::c_char; 44])
                            };
                            let echo_cell = UnsafeCell::new(echo_chars);
                            unsafe { **echo_cell.get() }
                        };

                        let mut j: libc::c_int = 0;
                        eof = 1 as libc::c_int;
                        j = 0 as libc::c_int;
                        while j < 20 as libc::c_int {
                            magic[j
                                as usize] = (rand() & 0xff as libc::c_int) as libc::c_char;
                            j += 1;
                        }
                        memcpy(
                            echo.as_mut_ptr().offset(21 as libc::c_int as isize)
                                as *mut libc::c_void,
                            magic.as_mut_ptr() as *const libc::c_void,
                            20 as libc::c_int as libc::c_ulong,
                        );
                        memcpy(
                            obuf.as_mut_ptr() as *mut libc::c_void,
                            echo.as_mut_ptr() as *const libc::c_void,
                            (core::mem::size_of::<[libc::c_char; 44]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        obuf_len = (core::mem::size_of::<[libc::c_char; 44]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as libc::c_longlong;
                        obuf_pos = 0 as libc::c_int as libc::c_longlong;
                        printf(
                            b"All data transferred. Waiting for the last reply...\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    } else if nread == -(1 as libc::c_int) as libc::c_long {
                        fprintf(
                            stderr,
                            b"Error reading from stdin: %s\n\0" as *const u8
                                as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                        exit(1 as libc::c_int);
                    } else {
                        obuf_len = nread as libc::c_longlong;
                        obuf_pos = 0 as libc::c_int as libc::c_longlong;
                    }
                }
                if obuf_len == 0 as libc::c_int as libc::c_longlong && eof != 0
                    || loop_nwritten
                        > (128 as libc::c_int * 1024 as libc::c_int) as libc::c_long
                {
                    break;
                }
            }
        }
        if !(eof != 0 && config.pipe_timeout > 0 as libc::c_int
            && time(0 as *mut time_t) - last_read_time
                > config.pipe_timeout as libc::c_long)
        {
            continue;
        }
        fprintf(
            stderr,
            b"No replies for %d seconds: exiting.\n\0" as *const u8
                as *const libc::c_char,
            config.pipe_timeout,
        );
        errors += 1;
        break;
    }
    printf(
        b"errors: %lld, replies: %lld\n\0" as *const u8 as *const libc::c_char,
        errors,
        replies,
    );
    if errors != 0 {
        exit(1 as libc::c_int);
    } else {
        exit(0 as libc::c_int);
    };
}
unsafe extern "C" fn sendScan(mut it: *mut libc::c_ulonglong) -> *mut redisReply {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    if !(config.pattern).is_null() {
        reply = redisCommand(
            context,
            b"SCAN %llu MATCH %b\0" as *const u8 as *const libc::c_char,
            *it,
            config.pattern,
            hi_sdslen(config.pattern),
        ) as *mut redisReply;
    } else {
        reply = redisCommand(
            context,
            b"SCAN %llu\0" as *const u8 as *const libc::c_char,
            *it,
        ) as *mut redisReply;
    }
    if reply.is_null() {
        fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    } else {
        if (*reply).type_0 == 6 as libc::c_int {
            fprintf(
                stderr,
                b"SCAN error: %s\n\0" as *const u8 as *const libc::c_char,
                (*reply).str_0,
            );
            exit(1 as libc::c_int);
        } else {
            if (*reply).type_0 != 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"Non ARRAY response from SCAN!\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            } else {
                if (*reply).elements != 2 as libc::c_int as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"Invalid element count from SCAN!\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
    }
    if (**((*reply).element).offset(0 as libc::c_int as isize)).type_0
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"reply->element[0]->type == REDIS_REPLY_STRING\0" as *const u8
                as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            8064 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"redisReply *sendScan(unsigned long long *)\0"))
                .as_ptr(),
        );
    };
    if (**((*reply).element).offset(1 as libc::c_int as isize)).type_0
        == 2 as libc::c_int
    {} else {
        __assert_fail(
            b"reply->element[1]->type == REDIS_REPLY_ARRAY\0" as *const u8
                as *const libc::c_char,
            b"redis-cli.c\0" as *const u8 as *const libc::c_char,
            8065 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"redisReply *sendScan(unsigned long long *)\0"))
                .as_ptr(),
        );
    };
    *it = strtoull(
        (**((*reply).element).offset(0 as libc::c_int as isize)).str_0,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    return reply;
}
unsafe extern "C" fn getDbSize() -> libc::c_int {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut size: libc::c_int = 0;
    reply = redisCommand(context, b"DBSIZE\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if reply.is_null() {
        fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    } else {
        if (*reply).type_0 == 6 as libc::c_int {
            fprintf(
                stderr,
                b"Couldn't determine DBSIZE: %s\n\0" as *const u8 as *const libc::c_char,
                (*reply).str_0,
            );
            exit(1 as libc::c_int);
        } else {
            if (*reply).type_0 != 3 as libc::c_int {
                fprintf(
                    stderr,
                    b"Non INTEGER response from DBSIZE!\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    size = (*reply).integer as libc::c_int;
    freeReplyObject(reply as *mut libc::c_void);
    return size;
}
#[no_mangle]
pub static mut type_string: typeinfo = {
    let mut init = typeinfo {
        name: b"string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizecmd: b"STRLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizeunit: b"bytes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        biggest: 0,
        count: 0,
        totalsize: 0,
        biggest_key: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
#[no_mangle]
pub static mut type_list: typeinfo = {
    let mut init = typeinfo {
        name: b"list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizecmd: b"LLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizeunit: b"items\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        biggest: 0,
        count: 0,
        totalsize: 0,
        biggest_key: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
#[no_mangle]
pub static mut type_set: typeinfo = {
    let mut init = typeinfo {
        name: b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizecmd: b"SCARD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizeunit: b"members\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        biggest: 0,
        count: 0,
        totalsize: 0,
        biggest_key: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
#[no_mangle]
pub static mut type_hash: typeinfo = {
    let mut init = typeinfo {
        name: b"hash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizecmd: b"HLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizeunit: b"fields\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        biggest: 0,
        count: 0,
        totalsize: 0,
        biggest_key: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
#[no_mangle]
pub static mut type_zset: typeinfo = {
    let mut init = typeinfo {
        name: b"zset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizecmd: b"ZCARD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizeunit: b"members\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        biggest: 0,
        count: 0,
        totalsize: 0,
        biggest_key: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
#[no_mangle]
pub static mut type_stream: typeinfo = {
    let mut init = typeinfo {
        name: b"stream\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizecmd: b"XLEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizeunit: b"entries\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        biggest: 0,
        count: 0,
        totalsize: 0,
        biggest_key: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
#[no_mangle]
pub static mut type_other: typeinfo = {
    let mut init = typeinfo {
        name: b"other\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sizecmd: 0 as *const libc::c_char as *mut libc::c_char,
        sizeunit: b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        biggest: 0,
        count: 0,
        totalsize: 0,
        biggest_key: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
unsafe extern "C" fn typeinfo_add(
    mut types: *mut dict,
    mut name: *mut libc::c_char,
    mut type_template: *mut typeinfo,
) -> *mut typeinfo {
    let mut info: *mut typeinfo = zmalloc(
        core::mem::size_of::<typeinfo>() as libc::c_ulong,
    ) as *mut typeinfo;
    *info = *type_template;
    (*info).name = hi_sdsnew(name);
    dictAdd(types, (*info).name as *mut libc::c_void, info as *mut libc::c_void);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn type_free(mut d: *mut dict, mut val: *mut libc::c_void) {
    let mut info: *mut typeinfo = val as *mut typeinfo;
    if !((*info).biggest_key).is_null() {
        hi_sdsfree((*info).biggest_key);
    }
    hi_sdsfree((*info).name);
    zfree(info as *mut libc::c_void);
}
static mut typeinfoDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: None,
            valDestructor: Some(
                type_free as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
unsafe extern "C" fn getKeyTypes(
    mut types_dict: *mut dict,
    mut keys: *mut redisReply,
    mut types: *mut *mut typeinfo,
) {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*keys).elements {
        let mut argv: [*const libc::c_char; 2] = [
            b"TYPE\0" as *const u8 as *const libc::c_char,
            (**((*keys).element).offset(i as isize)).str_0 as *const libc::c_char,
        ];
        let mut lens: [size_t; 2] = [
            4 as libc::c_int as size_t,
            (**((*keys).element).offset(i as isize)).len,
        ];
        redisAppendCommandArgv(
            context,
            2 as libc::c_int,
            argv.as_mut_ptr(),
            lens.as_mut_ptr(),
        );
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*keys).elements {
        if redisGetReply(
            context,
            &mut reply as *mut *mut redisReply as *mut *mut libc::c_void,
        ) != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"Error getting type for key '%s' (%d: %s)\n\0" as *const u8
                    as *const libc::c_char,
                (**((*keys).element).offset(i as isize)).str_0,
                (*context).err,
                ((*context).errstr).as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        } else {
            if (*reply).type_0 != 5 as libc::c_int {
                if (*reply).type_0 == 6 as libc::c_int {
                    fprintf(
                        stderr,
                        b"TYPE returned an error: %s\n\0" as *const u8
                            as *const libc::c_char,
                        (*reply).str_0,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"Invalid reply type (%d) for TYPE on key '%s'!\n\0" as *const u8
                            as *const libc::c_char,
                        (*reply).type_0,
                        (**((*keys).element).offset(i as isize)).str_0,
                    );
                }
                exit(1 as libc::c_int);
            }
        }
        let mut typereply: hisds = hi_sdsnew((*reply).str_0);
        let mut de: *mut dictEntry = dictFind(
            types_dict,
            typereply as *const libc::c_void,
        );
        hi_sdsfree(typereply);
        let mut type_0: *mut typeinfo = 0 as *mut typeinfo;
        if !de.is_null() {
            type_0 = (*de).v.val as *mut typeinfo;
        } else if strcmp((*reply).str_0, b"none\0" as *const u8 as *const libc::c_char)
            != 0
        {
            type_0 = typeinfo_add(types_dict, (*reply).str_0, &mut type_other);
        }
        let ref mut fresh76 = *types.offset(i as isize);
        *fresh76 = type_0;
        freeReplyObject(reply as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn getKeySizes(
    mut keys: *mut redisReply,
    mut types: *mut *mut typeinfo,
    mut sizes: *mut libc::c_ulonglong,
    mut memkeys: libc::c_int,
    mut memkeys_samples: libc::c_uint,
) {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*keys).elements {
        if !((*types.offset(i as isize)).is_null()
            || ((**types.offset(i as isize)).sizecmd).is_null() && memkeys == 0)
        {
            if memkeys == 0 {
                let mut argv: [*const libc::c_char; 2] = [
                    (**types.offset(i as isize)).sizecmd as *const libc::c_char,
                    (**((*keys).element).offset(i as isize)).str_0 as *const libc::c_char,
                ];
                let mut lens: [size_t; 2] = [
                    strlen((**types.offset(i as isize)).sizecmd),
                    (**((*keys).element).offset(i as isize)).len,
                ];
                redisAppendCommandArgv(
                    context,
                    2 as libc::c_int,
                    argv.as_mut_ptr(),
                    lens.as_mut_ptr(),
                );
            } else if memkeys_samples == 0 as libc::c_int as libc::c_uint {
                let mut argv_0: [*const libc::c_char; 3] = [
                    b"MEMORY\0" as *const u8 as *const libc::c_char,
                    b"USAGE\0" as *const u8 as *const libc::c_char,
                    (**((*keys).element).offset(i as isize)).str_0 as *const libc::c_char,
                ];
                let mut lens_0: [size_t; 3] = [
                    6 as libc::c_int as size_t,
                    5 as libc::c_int as size_t,
                    (**((*keys).element).offset(i as isize)).len,
                ];
                redisAppendCommandArgv(
                    context,
                    3 as libc::c_int,
                    argv_0.as_mut_ptr(),
                    lens_0.as_mut_ptr(),
                );
            } else {
                let mut samplesstr: hisds = hi_sdsfromlonglong(
                    memkeys_samples as libc::c_longlong,
                );
                let mut argv_1: [*const libc::c_char; 5] = [
                    b"MEMORY\0" as *const u8 as *const libc::c_char,
                    b"USAGE\0" as *const u8 as *const libc::c_char,
                    (**((*keys).element).offset(i as isize)).str_0
                        as *const libc::c_char,
                    b"SAMPLES\0" as *const u8 as *const libc::c_char,
                    samplesstr as *const libc::c_char,
                ];
                let mut lens_1: [size_t; 5] = [
                    6 as libc::c_int as size_t,
                    5 as libc::c_int as size_t,
                    (**((*keys).element).offset(i as isize)).len,
                    7 as libc::c_int as size_t,
                    hi_sdslen(samplesstr),
                ];
                redisAppendCommandArgv(
                    context,
                    5 as libc::c_int,
                    argv_1.as_mut_ptr(),
                    lens_1.as_mut_ptr(),
                );
                hi_sdsfree(samplesstr);
            }
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*keys).elements {
        if (*types.offset(i as isize)).is_null()
            || ((**types.offset(i as isize)).sizecmd).is_null() && memkeys == 0
        {
            *sizes.offset(i as isize) = 0 as libc::c_int as libc::c_ulonglong;
        } else {
            if redisGetReply(
                context,
                &mut reply as *mut *mut redisReply as *mut *mut libc::c_void,
            ) != 0 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"Error getting size for key '%s' (%d: %s)\n\0" as *const u8
                        as *const libc::c_char,
                    (**((*keys).element).offset(i as isize)).str_0,
                    (*context).err,
                    ((*context).errstr).as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            } else {
                if (*reply).type_0 != 3 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Warning:  %s on '%s' failed (may have changed type)\n\0"
                            as *const u8 as *const libc::c_char,
                        if memkeys == 0 {
                            (**types.offset(i as isize)).sizecmd as *const libc::c_char
                        } else {
                            b"MEMORY USAGE\0" as *const u8 as *const libc::c_char
                        },
                        (**((*keys).element).offset(i as isize)).str_0,
                    );
                    *sizes.offset(i as isize) = 0 as libc::c_int as libc::c_ulonglong;
                } else {
                    *sizes.offset(i as isize) = (*reply).integer as libc::c_ulonglong;
                }
            }
            freeReplyObject(reply as *mut libc::c_void);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn longStatLoopModeStop(mut s: libc::c_int) {
    core::ptr::write_volatile(
        &mut force_cancel_loop as *mut sig_atomic_t,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn findBigKeys(
    mut memkeys: libc::c_int,
    mut memkeys_samples: libc::c_uint,
) {
    let mut sampled: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut total_keys: libc::c_ulonglong = 0;
    let mut totlen: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut sizes: *mut libc::c_ulonglong = 0 as *mut libc::c_ulonglong;
    let mut it: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut scan_loops: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut keys: *mut redisReply = 0 as *mut redisReply;
    let mut arrsize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut types: *mut *mut typeinfo = 0 as *mut *mut typeinfo;
    let mut pct: libc::c_double = 0.;
    let mut types_dict: *mut dict = dictCreate(&mut typeinfoDictType);
    typeinfo_add(
        types_dict,
        b"string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut type_string,
    );
    typeinfo_add(
        types_dict,
        b"list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut type_list,
    );
    typeinfo_add(
        types_dict,
        b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut type_set,
    );
    typeinfo_add(
        types_dict,
        b"hash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut type_hash,
    );
    typeinfo_add(
        types_dict,
        b"zset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut type_zset,
    );
    typeinfo_add(
        types_dict,
        b"stream\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut type_stream,
    );
    signal(
        2 as libc::c_int,
        Some(longStatLoopModeStop as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    total_keys = getDbSize() as libc::c_ulonglong;
    printf(
        b"\n# Scanning the entire keyspace to find biggest keys as well as\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"# average sizes per key type.  You can use -i 0.1 to sleep 0.1 sec\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"# per 100 SCAN commands (not usually needed).\n\n\0" as *const u8
            as *const libc::c_char,
    );
    loop {
        pct = 100 as libc::c_int as libc::c_double * sampled as libc::c_double
            / total_keys as libc::c_double;
        reply = sendScan(&mut it);
        scan_loops = scan_loops.wrapping_add(1);
        keys = *((*reply).element).offset(1 as libc::c_int as isize);
        if (*keys).elements > arrsize as libc::c_ulong {
            types = zrealloc(
                types as *mut libc::c_void,
                (core::mem::size_of::<*mut typeinfo>() as libc::c_ulong)
                    .wrapping_mul((*keys).elements),
            ) as *mut *mut typeinfo;
            sizes = zrealloc(
                sizes as *mut libc::c_void,
                (core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
                    .wrapping_mul((*keys).elements),
            ) as *mut libc::c_ulonglong;
            if types.is_null() || sizes.is_null() {
                fprintf(
                    stderr,
                    b"Failed to allocate storage for keys!\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            arrsize = (*keys).elements as libc::c_uint;
        }
        getKeyTypes(types_dict, keys, types);
        getKeySizes(keys, types, sizes, memkeys, memkeys_samples);
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*keys).elements {
            let mut type_0: *mut typeinfo = *types.offset(i as isize);
            if !type_0.is_null() {
                (*type_0)
                    .totalsize = ((*type_0).totalsize)
                    .wrapping_add(*sizes.offset(i as isize));
                (*type_0).count = ((*type_0).count).wrapping_add(1);
                totlen = totlen
                    .wrapping_add(
                        (**((*keys).element).offset(i as isize)).len as libc::c_ulonglong,
                    );
                sampled = sampled.wrapping_add(1);
                if (*type_0).biggest < *sizes.offset(i as isize) {
                    if !((*type_0).biggest_key).is_null() {
                        hi_sdsfree((*type_0).biggest_key);
                    }
                    (*type_0)
                        .biggest_key = hi_sdscatrepr(
                        hi_sdsempty(),
                        (**((*keys).element).offset(i as isize)).str_0,
                        (**((*keys).element).offset(i as isize)).len,
                    );
                    if ((*type_0).biggest_key).is_null() {
                        fprintf(
                            stderr,
                            b"Failed to allocate memory for key!\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    printf(
                        b"[%05.2f%%] Biggest %-6s found so far '%s' with %llu %s\n\0"
                            as *const u8 as *const libc::c_char,
                        pct,
                        (*type_0).name,
                        (*type_0).biggest_key,
                        *sizes.offset(i as isize),
                        if memkeys == 0 {
                            (*type_0).sizeunit as *const libc::c_char
                        } else {
                            b"bytes\0" as *const u8 as *const libc::c_char
                        },
                    );
                    (*type_0).biggest = *sizes.offset(i as isize);
                }
                if sampled.wrapping_rem(1000000 as libc::c_int as libc::c_ulonglong)
                    == 0 as libc::c_int as libc::c_ulonglong
                {
                    printf(
                        b"[%05.2f%%] Sampled %llu keys so far\n\0" as *const u8
                            as *const libc::c_char,
                        pct,
                        sampled,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
        if config.interval != 0
            && scan_loops.wrapping_rem(100 as libc::c_int as libc::c_ulonglong)
                == 0 as libc::c_int as libc::c_ulonglong
        {
            usleep(config.interval as __useconds_t);
        }
        freeReplyObject(reply as *mut libc::c_void);
        if !(force_cancel_loop == 0 as libc::c_int
            && it != 0 as libc::c_int as libc::c_ulonglong)
        {
            break;
        }
    }
    if !types.is_null() {
        zfree(types as *mut libc::c_void);
    }
    if !sizes.is_null() {
        zfree(sizes as *mut libc::c_void);
    }
    printf(b"\n-------- summary -------\n\n\0" as *const u8 as *const libc::c_char);
    if force_cancel_loop != 0 {
        printf(b"[%05.2f%%] \0" as *const u8 as *const libc::c_char, pct);
    }
    printf(
        b"Sampled %llu keys in the keyspace!\n\0" as *const u8 as *const libc::c_char,
        sampled,
    );
    printf(
        b"Total key length in bytes is %llu (avg len %.2f)\n\n\0" as *const u8
            as *const libc::c_char,
        totlen,
        if totlen != 0 {
            totlen as libc::c_double / sampled as libc::c_double
        } else {
            0 as libc::c_int as libc::c_double
        },
    );
    di = dictGetIterator(types_dict);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut type_1: *mut typeinfo = (*de).v.val as *mut typeinfo;
        if !((*type_1).biggest_key).is_null() {
            printf(
                b"Biggest %6s found '%s' has %llu %s\n\0" as *const u8
                    as *const libc::c_char,
                (*type_1).name,
                (*type_1).biggest_key,
                (*type_1).biggest,
                if memkeys == 0 {
                    (*type_1).sizeunit as *const libc::c_char
                } else {
                    b"bytes\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    dictReleaseIterator(di);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    di = dictGetIterator(types_dict);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut type_2: *mut typeinfo = (*de).v.val as *mut typeinfo;
        printf(
            b"%llu %ss with %llu %s (%05.2f%% of keys, avg size %.2f)\n\0" as *const u8
                as *const libc::c_char,
            (*type_2).count,
            (*type_2).name,
            (*type_2).totalsize,
            if memkeys == 0 {
                (*type_2).sizeunit as *const libc::c_char
            } else {
                b"bytes\0" as *const u8 as *const libc::c_char
            },
            if sampled != 0 {
                100 as libc::c_int as libc::c_double * (*type_2).count as libc::c_double
                    / sampled as libc::c_double
            } else {
                0 as libc::c_int as libc::c_double
            },
            if (*type_2).count != 0 {
                (*type_2).totalsize as libc::c_double / (*type_2).count as libc::c_double
            } else {
                0 as libc::c_int as libc::c_double
            },
        );
    }
    dictReleaseIterator(di);
    dictRelease(types_dict);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn getKeyFreqs(
    mut keys: *mut redisReply,
    mut freqs: *mut libc::c_ulonglong,
) {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*keys).elements {
        let mut argv: [*const libc::c_char; 3] = [
            b"OBJECT\0" as *const u8 as *const libc::c_char,
            b"FREQ\0" as *const u8 as *const libc::c_char,
            (**((*keys).element).offset(i as isize)).str_0 as *const libc::c_char,
        ];
        let mut lens: [size_t; 3] = [
            6 as libc::c_int as size_t,
            4 as libc::c_int as size_t,
            (**((*keys).element).offset(i as isize)).len,
        ];
        redisAppendCommandArgv(
            context,
            3 as libc::c_int,
            argv.as_mut_ptr(),
            lens.as_mut_ptr(),
        );
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*keys).elements {
        if redisGetReply(
            context,
            &mut reply as *mut *mut redisReply as *mut *mut libc::c_void,
        ) != 0 as libc::c_int
        {
            let mut keyname: hisds = hi_sdscatrepr(
                hi_sdsempty(),
                (**((*keys).element).offset(i as isize)).str_0,
                (**((*keys).element).offset(i as isize)).len,
            );
            fprintf(
                stderr,
                b"Error getting freq for key '%s' (%d: %s)\n\0" as *const u8
                    as *const libc::c_char,
                keyname,
                (*context).err,
                ((*context).errstr).as_mut_ptr(),
            );
            hi_sdsfree(keyname);
            exit(1 as libc::c_int);
        } else {
            if (*reply).type_0 != 3 as libc::c_int {
                if (*reply).type_0 == 6 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Error: %s\n\0" as *const u8 as *const libc::c_char,
                        (*reply).str_0,
                    );
                    exit(1 as libc::c_int);
                } else {
                    let mut keyname_0: hisds = hi_sdscatrepr(
                        hi_sdsempty(),
                        (**((*keys).element).offset(i as isize)).str_0,
                        (**((*keys).element).offset(i as isize)).len,
                    );
                    fprintf(
                        stderr,
                        b"Warning: OBJECT freq on '%s' failed (may have been deleted)\n\0"
                            as *const u8 as *const libc::c_char,
                        keyname_0,
                    );
                    hi_sdsfree(keyname_0);
                    *freqs.offset(i as isize) = 0 as libc::c_int as libc::c_ulonglong;
                }
            } else {
                *freqs.offset(i as isize) = (*reply).integer as libc::c_ulonglong;
            }
        }
        freeReplyObject(reply as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn findHotKeys() {
    let mut keys: *mut redisReply = 0 as *mut redisReply;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut counters: [libc::c_ulonglong; 16] = [
        0 as libc::c_int as libc::c_ulonglong,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut hotkeys: [hisds; 16] = [
        0 as hisds,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut sampled: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut total_keys: libc::c_ulonglong = 0;
    let mut freqs: *mut libc::c_ulonglong = 0 as *mut libc::c_ulonglong;
    let mut it: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut scan_loops: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut arrsize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut pct: libc::c_double = 0.;
    signal(
        2 as libc::c_int,
        Some(longStatLoopModeStop as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    total_keys = getDbSize() as libc::c_ulonglong;
    printf(
        b"\n# Scanning the entire keyspace to find hot keys as well as\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"# average sizes per key type.  You can use -i 0.1 to sleep 0.1 sec\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"# per 100 SCAN commands (not usually needed).\n\n\0" as *const u8
            as *const libc::c_char,
    );
    loop {
        pct = 100 as libc::c_int as libc::c_double * sampled as libc::c_double
            / total_keys as libc::c_double;
        reply = sendScan(&mut it);
        scan_loops = scan_loops.wrapping_add(1);
        keys = *((*reply).element).offset(1 as libc::c_int as isize);
        if (*keys).elements > arrsize as libc::c_ulong {
            freqs = zrealloc(
                freqs as *mut libc::c_void,
                (core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
                    .wrapping_mul((*keys).elements),
            ) as *mut libc::c_ulonglong;
            if freqs.is_null() {
                fprintf(
                    stderr,
                    b"Failed to allocate storage for keys!\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            arrsize = (*keys).elements as libc::c_uint;
        }
        getKeyFreqs(keys, freqs);
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*keys).elements {
            sampled = sampled.wrapping_add(1);
            if sampled.wrapping_rem(1000000 as libc::c_int as libc::c_ulonglong)
                == 0 as libc::c_int as libc::c_ulonglong
            {
                printf(
                    b"[%05.2f%%] Sampled %llu keys so far\n\0" as *const u8
                        as *const libc::c_char,
                    pct,
                    sampled,
                );
            }
            k = 0 as libc::c_int as libc::c_uint;
            while k < 16 as libc::c_int as libc::c_uint
                && *freqs.offset(i as isize) > counters[k as usize]
            {
                k = k.wrapping_add(1);
            }
            if !(k == 0 as libc::c_int as libc::c_uint) {
                k = k.wrapping_sub(1);
                if k == 0 as libc::c_int as libc::c_uint
                    || counters[k as usize] == 0 as libc::c_int as libc::c_ulonglong
                {
                    hi_sdsfree(hotkeys[k as usize]);
                } else {
                    hi_sdsfree(hotkeys[0 as libc::c_int as usize]);
                    memmove(
                        counters.as_mut_ptr() as *mut libc::c_void,
                        counters.as_mut_ptr().offset(1 as libc::c_int as isize)
                            as *const libc::c_void,
                        (core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
                            .wrapping_mul(k as libc::c_ulong),
                    );
                    memmove(
                        hotkeys.as_mut_ptr() as *mut libc::c_void,
                        hotkeys.as_mut_ptr().offset(1 as libc::c_int as isize)
                            as *const libc::c_void,
                        (core::mem::size_of::<hisds>() as libc::c_ulong)
                            .wrapping_mul(k as libc::c_ulong),
                    );
                }
                counters[k as usize] = *freqs.offset(i as isize);
                hotkeys[k
                    as usize] = hi_sdscatrepr(
                    hi_sdsempty(),
                    (**((*keys).element).offset(i as isize)).str_0,
                    (**((*keys).element).offset(i as isize)).len,
                );
                printf(
                    b"[%05.2f%%] Hot key '%s' found so far with counter %llu\n\0"
                        as *const u8 as *const libc::c_char,
                    pct,
                    hotkeys[k as usize],
                    *freqs.offset(i as isize),
                );
            }
            i = i.wrapping_add(1);
        }
        if config.interval != 0
            && scan_loops.wrapping_rem(100 as libc::c_int as libc::c_ulonglong)
                == 0 as libc::c_int as libc::c_ulonglong
        {
            usleep(config.interval as __useconds_t);
        }
        freeReplyObject(reply as *mut libc::c_void);
        if !(force_cancel_loop == 0 as libc::c_int
            && it != 0 as libc::c_int as libc::c_ulonglong)
        {
            break;
        }
    }
    if !freqs.is_null() {
        zfree(freqs as *mut libc::c_void);
    }
    printf(b"\n-------- summary -------\n\n\0" as *const u8 as *const libc::c_char);
    if force_cancel_loop != 0 {
        printf(b"[%05.2f%%] \0" as *const u8 as *const libc::c_char, pct);
    }
    printf(
        b"Sampled %llu keys in the keyspace!\n\0" as *const u8 as *const libc::c_char,
        sampled,
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i <= 16 as libc::c_int as libc::c_uint {
        k = (16 as libc::c_int as libc::c_uint).wrapping_sub(i);
        if counters[k as usize] > 0 as libc::c_int as libc::c_ulonglong {
            printf(
                b"hot key found with counter: %llu\tkeyname: %s\n\0" as *const u8
                    as *const libc::c_char,
                counters[k as usize],
                hotkeys[k as usize],
            );
            hi_sdsfree(hotkeys[k as usize]);
        }
        i = i.wrapping_add(1);
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn getInfoField(
    mut info: *mut libc::c_char,
    mut field: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strstr(info, field);
    let mut n1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    p = p
        .offset(
            (strlen(field)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        );
    n1 = strchr(p, '\r' as i32);
    n2 = strchr(p, ',' as i32);
    if !n2.is_null() && n2 < n1 {
        n1 = n2;
    }
    result = zmalloc(
        (core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(n1.offset_from(p) as libc::c_long as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        result as *mut libc::c_void,
        p as *const libc::c_void,
        n1.offset_from(p) as libc::c_long as libc::c_ulong,
    );
    *result
        .offset(
            n1.offset_from(p) as libc::c_long as isize,
        ) = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn getLongInfoField(
    mut info: *mut libc::c_char,
    mut field: *mut libc::c_char,
) -> libc::c_long {
    let mut value: *mut libc::c_char = getInfoField(info, field);
    let mut l: libc::c_long = 0;
    if value.is_null() {
        return -(9223372036854775807 as libc::c_long) - 1 as libc::c_long;
    }
    l = strtol(value, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    zfree(value as *mut libc::c_void);
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn bytesToHuman(
    mut s: *mut libc::c_char,
    mut n: libc::c_longlong,
) {
    let mut d: libc::c_double = 0.;
    if n < 0 as libc::c_int as libc::c_longlong {
        *s = '-' as i32 as libc::c_char;
        s = s.offset(1);
        n = -n;
    }
    if n < 1024 as libc::c_int as libc::c_longlong {
        sprintf(s, b"%lldB\0" as *const u8 as *const libc::c_char, n);
        return;
    } else {
        if n < (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_longlong {
            d = n as libc::c_double / 1024 as libc::c_int as libc::c_double;
            sprintf(s, b"%.2fK\0" as *const u8 as *const libc::c_char, d);
        } else if n
            < 1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong
        {
            d = n as libc::c_double
                / (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double;
            sprintf(s, b"%.2fM\0" as *const u8 as *const libc::c_char, d);
        } else if n
            < 1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong
        {
            d = n as libc::c_double
                / (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                    * 1024 as libc::c_int as libc::c_longlong) as libc::c_double;
            sprintf(s, b"%.2fG\0" as *const u8 as *const libc::c_char, d);
        }
    };
}
unsafe extern "C" fn statMode() {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut aux: libc::c_long = 0;
    let mut requests: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        let mut buf: [libc::c_char; 64] = [0; 64];
        let mut j: libc::c_int = 0;
        reply = reconnectingRedisCommand(
            context,
            b"INFO\0" as *const u8 as *const libc::c_char,
        );
        if reply.is_null() {
            fprintf(stderr, b"\nI/O error\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        } else {
            if (*reply).type_0 == 6 as libc::c_int {
                fprintf(
                    stderr,
                    b"ERROR: %s\n\0" as *const u8 as *const libc::c_char,
                    (*reply).str_0,
                );
                exit(1 as libc::c_int);
            }
        }
        let fresh77 = i;
        i = i + 1;
        if fresh77 % 20 as libc::c_int == 0 as libc::c_int {
            printf(
                b"------- data ------ --------------------- load -------------------- - child -\nkeys       mem      clients blocked requests            connections          \n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        aux = 0 as libc::c_int as libc::c_long;
        j = 0 as libc::c_int;
        while j < 20 as libc::c_int {
            let mut k: libc::c_long = 0;
            sprintf(
                buf.as_mut_ptr(),
                b"db%d:keys\0" as *const u8 as *const libc::c_char,
                j,
            );
            k = getLongInfoField((*reply).str_0, buf.as_mut_ptr());
            if !(k == -(9223372036854775807 as libc::c_long) - 1 as libc::c_long) {
                aux += k;
            }
            j += 1;
        }
        sprintf(buf.as_mut_ptr(), b"%ld\0" as *const u8 as *const libc::c_char, aux);
        printf(b"%-11s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        aux = getLongInfoField(
            (*reply).str_0,
            b"used_memory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        bytesToHuman(buf.as_mut_ptr(), aux as libc::c_longlong);
        printf(b"%-8s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        aux = getLongInfoField(
            (*reply).str_0,
            b"connected_clients\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        sprintf(buf.as_mut_ptr(), b"%ld\0" as *const u8 as *const libc::c_char, aux);
        printf(b" %-8s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        aux = getLongInfoField(
            (*reply).str_0,
            b"blocked_clients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        sprintf(buf.as_mut_ptr(), b"%ld\0" as *const u8 as *const libc::c_char, aux);
        printf(b"%-8s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        aux = getLongInfoField(
            (*reply).str_0,
            b"total_commands_processed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        sprintf(
            buf.as_mut_ptr(),
            b"%ld (+%ld)\0" as *const u8 as *const libc::c_char,
            aux,
            if requests == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int as libc::c_long
            } else {
                aux - requests
            },
        );
        printf(b"%-19s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        requests = aux;
        aux = getLongInfoField(
            (*reply).str_0,
            b"total_connections_received\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        sprintf(buf.as_mut_ptr(), b"%ld\0" as *const u8 as *const libc::c_char, aux);
        printf(b" %-12s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        aux = getLongInfoField(
            (*reply).str_0,
            b"bgsave_in_progress\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        aux
            |= getLongInfoField(
                (*reply).str_0,
                b"aof_rewrite_in_progress\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) << 1 as libc::c_int;
        aux
            |= getLongInfoField(
                (*reply).str_0,
                b"loading\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) << 2 as libc::c_int;
        match aux {
            1 => {
                printf(b"SAVE\0" as *const u8 as *const libc::c_char);
            }
            2 => {
                printf(b"AOF\0" as *const u8 as *const libc::c_char);
            }
            3 => {
                printf(b"SAVE+AOF\0" as *const u8 as *const libc::c_char);
            }
            4 => {
                printf(b"LOAD\0" as *const u8 as *const libc::c_char);
            }
            0 | _ => {}
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        freeReplyObject(reply as *mut libc::c_void);
        usleep(config.interval as __useconds_t);
    };
}
unsafe extern "C" fn scanMode() {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut cur: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    signal(
        2 as libc::c_int,
        Some(longStatLoopModeStop as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    loop {
        reply = sendScan(&mut cur);
        let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (j as libc::c_ulong)
            < (**((*reply).element).offset(1 as libc::c_int as isize)).elements
        {
            if config.output == 0 as libc::c_int {
                let mut out: hisds = hi_sdscatrepr(
                    hi_sdsempty(),
                    (**((**((*reply).element).offset(1 as libc::c_int as isize)).element)
                        .offset(j as isize))
                        .str_0,
                    (**((**((*reply).element).offset(1 as libc::c_int as isize)).element)
                        .offset(j as isize))
                        .len,
                );
                printf(b"%s\n\0" as *const u8 as *const libc::c_char, out);
                hi_sdsfree(out);
            } else {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    (**((**((*reply).element).offset(1 as libc::c_int as isize)).element)
                        .offset(j as isize))
                        .str_0,
                );
            }
            j = j.wrapping_add(1);
        }
        freeReplyObject(reply as *mut libc::c_void);
        if config.interval != 0 {
            usleep(config.interval as __useconds_t);
        }
        if !(force_cancel_loop == 0 as libc::c_int
            && cur != 0 as libc::c_int as libc::c_ulonglong)
        {
            break;
        }
    }
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn powerLawRand(
    mut min: libc::c_longlong,
    mut max: libc::c_longlong,
    mut alpha: libc::c_double,
) -> libc::c_longlong {
    let mut pl: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    max += 1 as libc::c_int as libc::c_longlong;
    r = rand() as libc::c_double / 2147483647 as libc::c_int as libc::c_double;
    pl = pow(
        (pow(max as libc::c_double, alpha + 1 as libc::c_int as libc::c_double)
            - pow(min as libc::c_double, alpha + 1 as libc::c_int as libc::c_double)) * r
            + pow(min as libc::c_double, alpha + 1 as libc::c_int as libc::c_double),
        1.0f64 / (alpha + 1 as libc::c_int as libc::c_double),
    );
    return max - 1 as libc::c_int as libc::c_longlong - pl as libc::c_longlong + min;
}
#[no_mangle]
pub unsafe extern "C" fn LRUTestGenKey(mut buf: *mut libc::c_char, mut buflen: size_t) {
    snprintf(
        buf,
        buflen,
        b"lru:%lld\0" as *const u8 as *const libc::c_char,
        powerLawRand(
            1 as libc::c_int as libc::c_longlong,
            config.lru_test_sample_size,
            6.2f64,
        ),
    );
}
unsafe extern "C" fn LRUTestMode() {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut key: [libc::c_char; 128] = [0; 128];
    let mut start_cycle: libc::c_longlong = 0;
    let mut j: libc::c_int = 0;
    srand((time(0 as *mut time_t) ^ getpid() as libc::c_long) as libc::c_uint);
    loop {
        start_cycle = mstime();
        let mut hits: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        let mut misses: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        while mstime() - start_cycle < 1000 as libc::c_int as libc::c_longlong {
            j = 0 as libc::c_int;
            while j < 250 as libc::c_int {
                let mut val: [libc::c_char; 6] = [0; 6];
                val[5 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < 5 as libc::c_int {
                    val[i
                        as usize] = ('A' as i32 + rand() % ('z' as i32 - 'A' as i32))
                        as libc::c_char;
                    i += 1;
                }
                LRUTestGenKey(
                    key.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
                redisAppendCommand(
                    context,
                    b"SET %s %s\0" as *const u8 as *const libc::c_char,
                    key.as_mut_ptr(),
                    val.as_mut_ptr(),
                );
                j += 1;
            }
            j = 0 as libc::c_int;
            while j < 250 as libc::c_int {
                redisGetReply(
                    context,
                    &mut reply as *mut *mut redisReply as *mut *mut libc::c_void,
                );
                j += 1;
            }
            j = 0 as libc::c_int;
            while j < 250 as libc::c_int {
                LRUTestGenKey(
                    key.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
                redisAppendCommand(
                    context,
                    b"GET %s\0" as *const u8 as *const libc::c_char,
                    key.as_mut_ptr(),
                );
                j += 1;
            }
            j = 0 as libc::c_int;
            while j < 250 as libc::c_int {
                if redisGetReply(
                    context,
                    &mut reply as *mut *mut redisReply as *mut *mut libc::c_void,
                ) == 0 as libc::c_int
                {
                    match (*reply).type_0 {
                        6 => {
                            fprintf(
                                stderr,
                                b"%s\n\0" as *const u8 as *const libc::c_char,
                                (*reply).str_0,
                            );
                        }
                        4 => {
                            misses += 1;
                        }
                        _ => {
                            hits += 1;
                        }
                    }
                }
                j += 1;
            }
            if (*context).err != 0 {
                fprintf(
                    stderr,
                    b"I/O error during LRU test\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        printf(
            b"%lld Gets/sec | Hits: %lld (%.2f%%) | Misses: %lld (%.2f%%)\n\0"
                as *const u8 as *const libc::c_char,
            hits + misses,
            hits,
            hits as libc::c_double / (hits + misses) as libc::c_double
                * 100 as libc::c_int as libc::c_double,
            misses,
            misses as libc::c_double / (hits + misses) as libc::c_double
                * 100 as libc::c_int as libc::c_double,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_something_fast() -> libc::c_ulong {
    let mut s: [libc::c_uchar; 256] = [0; 256];
    let mut i: libc::c_uchar = 0;
    let mut j: libc::c_uchar = 0;
    let mut t: libc::c_uchar = 0;
    let mut count: libc::c_int = 1000 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut output: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    k = 0 as libc::c_int;
    while k < 256 as libc::c_int {
        s[k as usize] = k as libc::c_uchar;
        k += 1;
    }
    i = 0 as libc::c_int as libc::c_uchar;
    j = 0 as libc::c_int as libc::c_uchar;
    loop {
        let fresh78 = count;
        count = count - 1;
        if !(fresh78 != 0) {
            break;
        }
        i = i.wrapping_add(1);
        j = (j as libc::c_int + s[i as usize] as libc::c_int) as libc::c_uchar;
        t = s[i as usize];
        s[i as usize] = s[j as usize];
        s[j as usize] = t;
        output = output
            .wrapping_add(
                s[(s[i as usize] as libc::c_int + s[j as usize] as libc::c_int
                    & 255 as libc::c_int) as usize] as libc::c_ulong,
            );
    }
    return output;
}
unsafe extern "C" fn sigIntHandler(mut s: libc::c_int) {
    if config.monitor_mode != 0 || config.pubsub_mode != 0 {
        close((*context).fd);
        (*context).fd = -(1 as libc::c_int);
        config.blocking_state_aborted = 1 as libc::c_int;
    } else {
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn intrinsicLatencyMode() {
    let mut test_end: libc::c_longlong = 0;
    let mut run_time: libc::c_longlong = 0;
    let mut max_latency: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut runs: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    run_time = config.intrinsic_latency_duration as libc::c_longlong
        * 1000000 as libc::c_int as libc::c_longlong;
    test_end = ustime() + run_time;
    signal(
        2 as libc::c_int,
        Some(longStatLoopModeStop as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    loop {
        let mut start: libc::c_longlong = 0;
        let mut end: libc::c_longlong = 0;
        let mut latency: libc::c_longlong = 0;
        start = ustime();
        compute_something_fast();
        end = ustime();
        latency = end - start;
        runs += 1;
        if latency <= 0 as libc::c_int as libc::c_longlong {
            continue;
        }
        if latency > max_latency {
            max_latency = latency;
            printf(
                b"Max latency so far: %lld microseconds.\n\0" as *const u8
                    as *const libc::c_char,
                max_latency,
            );
        }
        let mut avg_us: libc::c_double = run_time as libc::c_double
            / runs as libc::c_double;
        let mut avg_ns: libc::c_double = avg_us * 1e3f64;
        if force_cancel_loop != 0 || end > test_end {
            printf(
                b"\n%lld total runs (avg latency: %.4f microseconds / %.2f nanoseconds per run).\n\0"
                    as *const u8 as *const libc::c_char,
                runs,
                avg_us,
                avg_ns,
            );
            printf(
                b"Worst run took %.0fx longer than the average latency.\n\0" as *const u8
                    as *const libc::c_char,
                max_latency as libc::c_double / avg_us,
            );
            exit(0 as libc::c_int);
        }
    };
}
unsafe extern "C" fn askPassword(mut msg: *const libc::c_char) -> hisds {
    linenoiseMaskModeEnable();
    let mut auth: hisds = linenoise(msg);
    linenoiseMaskModeDisable();
    return auth;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut firstarg: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    memset(
        &mut config.sslconfig as *mut cliSSLconfig as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<cliSSLconfig>() as libc::c_ulong,
    );
    config
        .conn_info
        .hostip = hi_sdsnew(b"127.0.0.1\0" as *const u8 as *const libc::c_char);
    config.conn_info.hostport = 6379 as libc::c_int;
    config.hostsocket = 0 as *mut libc::c_char;
    config.repeat = 1 as libc::c_int as libc::c_long;
    config.interval = 0 as libc::c_int as libc::c_long;
    config.dbnum = 0 as libc::c_int;
    config.conn_info.input_dbnum = 0 as libc::c_int;
    config.interactive = 0 as libc::c_int;
    config.shutdown = 0 as libc::c_int;
    config.monitor_mode = 0 as libc::c_int;
    config.pubsub_mode = 0 as libc::c_int;
    config.blocking_state_aborted = 0 as libc::c_int;
    config.latency_mode = 0 as libc::c_int;
    config.latency_dist_mode = 0 as libc::c_int;
    config.latency_history = 0 as libc::c_int;
    config.lru_test_mode = 0 as libc::c_int;
    config.lru_test_sample_size = 0 as libc::c_int as libc::c_longlong;
    config.cluster_mode = 0 as libc::c_int;
    config.cluster_send_asking = 0 as libc::c_int;
    config.slave_mode = 0 as libc::c_int;
    config.getrdb_mode = 0 as libc::c_int;
    config.get_functions_rdb_mode = 0 as libc::c_int;
    config.stat_mode = 0 as libc::c_int;
    config.scan_mode = 0 as libc::c_int;
    config.intrinsic_latency_mode = 0 as libc::c_int;
    config.pattern = 0 as hisds;
    config.rdb_filename = 0 as *mut libc::c_char;
    config.pipe_mode = 0 as libc::c_int;
    config.pipe_timeout = 30 as libc::c_int;
    config.bigkeys = 0 as libc::c_int;
    config.hotkeys = 0 as libc::c_int;
    config.stdin_lastarg = 0 as libc::c_int;
    config.stdin_tag_arg = 0 as libc::c_int;
    config.stdin_tag_name = 0 as *mut libc::c_char;
    config.conn_info.auth = 0 as *mut libc::c_char;
    config.askpass = 0 as libc::c_int;
    config.conn_info.user = 0 as *mut libc::c_char;
    config.eval = 0 as *mut libc::c_char;
    config.eval_ldb = 0 as libc::c_int;
    config.eval_ldb_end = 0 as libc::c_int;
    config.eval_ldb_sync = 0 as libc::c_int;
    config.enable_ldb_on_eval = 0 as libc::c_int;
    config.last_cmd_type = -(1 as libc::c_int);
    config.verbose = 0 as libc::c_int;
    config.set_errcode = 0 as libc::c_int;
    config.no_auth_warning = 0 as libc::c_int;
    config.in_multi = 0 as libc::c_int;
    config.cluster_manager_command.name = 0 as *mut libc::c_char;
    config.cluster_manager_command.argc = 0 as libc::c_int;
    config.cluster_manager_command.argv = 0 as *mut *mut libc::c_char;
    config.cluster_manager_command.stdin_arg = 0 as hisds;
    config.cluster_manager_command.flags = 0 as libc::c_int;
    config.cluster_manager_command.replicas = 0 as libc::c_int;
    config.cluster_manager_command.from = 0 as *mut libc::c_char;
    config.cluster_manager_command.to = 0 as *mut libc::c_char;
    config.cluster_manager_command.from_user = 0 as *mut libc::c_char;
    config.cluster_manager_command.from_pass = 0 as *mut libc::c_char;
    config.cluster_manager_command.from_askpass = 0 as libc::c_int;
    config.cluster_manager_command.weight = 0 as *mut *mut libc::c_char;
    config.cluster_manager_command.weight_argc = 0 as libc::c_int;
    config.cluster_manager_command.slots = 0 as libc::c_int;
    config.cluster_manager_command.timeout = 60000 as libc::c_int;
    config.cluster_manager_command.pipeline = 10 as libc::c_int;
    config.cluster_manager_command.threshold = 2 as libc::c_int as libc::c_float;
    config.cluster_manager_command.backup_dir = 0 as *mut libc::c_char;
    pref.hints = 1 as libc::c_int;
    spectrum_palette = spectrum_palette_color.as_mut_ptr();
    spectrum_palette_size = spectrum_palette_color_size;
    if isatty(fileno(stdout)) == 0
        && (getenv(b"FAKETTY\0" as *const u8 as *const libc::c_char)).is_null()
    {
        config.output = 1 as libc::c_int;
        config.push_output = 0 as libc::c_int;
    } else {
        config.output = 0 as libc::c_int;
        config.push_output = 1 as libc::c_int;
    }
    config.mb_delim = hi_sdsnew(b"\n\0" as *const u8 as *const libc::c_char);
    config.cmd_delim = hi_sdsnew(b"\n\0" as *const u8 as *const libc::c_char);
    firstarg = parseOptions(argc, argv);
    argc -= firstarg;
    argv = argv.offset(firstarg as isize);
    parseEnv();
    if config.askpass != 0 {
        config
            .conn_info
            .auth = askPassword(
            b"Please input password: \0" as *const u8 as *const libc::c_char,
        );
    }
    if config.cluster_manager_command.from_askpass != 0 {
        config
            .cluster_manager_command
            .from_pass = askPassword(
            b"Please input import source node password: \0" as *const u8
                as *const libc::c_char,
        );
    }
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    init_genrand64(
        (tv.tv_sec as libc::c_longlong * 1000000 as libc::c_int as libc::c_longlong
            + tv.tv_usec as libc::c_longlong ^ getpid() as libc::c_longlong)
            as libc::c_ulonglong,
    );
    if !(config.cluster_manager_command.name).is_null() {
        let mut proc_0: Option::<clusterManagerCommandProc> = validateClusterManagerCommand();
        if proc_0.is_none() {
            exit(1 as libc::c_int);
        }
        clusterManagerMode(proc_0);
    }
    if config.latency_mode != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        latencyMode();
    }
    if config.latency_dist_mode != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        latencyDistMode();
    }
    if config.slave_mode != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        sendCapa();
        sendReplconf(
            b"rdb-filter-only\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        slaveMode();
    }
    if config.getrdb_mode != 0 || config.get_functions_rdb_mode != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        sendCapa();
        sendRdbOnly();
        if config.get_functions_rdb_mode != 0
            && sendReplconf(
                b"rdb-filter-only\0" as *const u8 as *const libc::c_char,
                b"functions\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            fprintf(
                stderr,
                b"Failed requesting functions only RDB from server, aborting\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        getRDB(0 as *mut clusterManagerNode);
    }
    if config.pipe_mode != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        pipeMode();
    }
    if config.bigkeys != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        findBigKeys(0 as libc::c_int, 0 as libc::c_int as libc::c_uint);
    }
    if config.memkeys != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        findBigKeys(1 as libc::c_int, config.memkeys_samples);
    }
    if config.hotkeys != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        findHotKeys();
    }
    if config.stat_mode != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        if config.interval == 0 as libc::c_int as libc::c_long {
            config.interval = 1000000 as libc::c_int as libc::c_long;
        }
        statMode();
    }
    if config.scan_mode != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        scanMode();
    }
    if config.lru_test_mode != 0 {
        if cliConnect(0 as libc::c_int) == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
        LRUTestMode();
    }
    if config.intrinsic_latency_mode != 0 {
        intrinsicLatencyMode();
    }
    if argc == 0 as libc::c_int && (config.eval).is_null() {
        signal(
            13 as libc::c_int,
            core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        signal(
            2 as libc::c_int,
            Some(sigIntHandler as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        cliConnect(0 as libc::c_int);
        repl();
    }
    if !(config.eval).is_null() {
        if cliConnect(0 as libc::c_int) != 0 as libc::c_int {
            exit(1 as libc::c_int);
        }
        return evalMode(argc, argv);
    } else {
        cliConnect((1 as libc::c_int) << 1 as libc::c_int);
        return noninteractive(argc, argv);
    };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
