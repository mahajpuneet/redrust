extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type lua_State;
    pub type RedisModuleCommand;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn mstime() -> libc::c_longlong;
    fn exitFromChild(retcode: libc::c_int);
    fn createClient(conn: *mut connection) -> *mut client;
    fn freeClientAsync(c: *mut client);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyErrorSds(c: *mut client, err: sds);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn sdsZmallocSize(s: sds) -> size_t;
    fn getStringObjectSdsUsedMemory(o: *mut robj) -> size_t;
    fn clientHasPendingReplies(c: *mut client) -> libc::c_int;
    fn writeToClient(c: *mut client, handler_installed: libc::c_int) -> libc::c_int;
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn decrRefCount(o: *mut robj);
    fn incrRefCount(o: *mut robj);
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdssplitlen(
        s: *const libc::c_char,
        len: ssize_t,
        sep: *const libc::c_char,
        seplen: libc::c_int,
        count: *mut libc::c_int,
    ) -> *mut sds;
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdscatrepr(s: sds, p: *const libc::c_char, len: size_t) -> sds;
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn sdsmapchars(
        s: sds,
        from: *const libc::c_char,
        to: *const libc::c_char,
        setlen: size_t,
    ) -> sds;
    fn sdsjoinsds(
        argv: *mut sds,
        argc: libc::c_int,
        sep: *const libc::c_char,
        seplen: size_t,
    ) -> sds;
    fn __errno_location() -> *mut libc::c_int;
    fn connBlock(conn: *mut connection) -> libc::c_int;
    fn connNonBlock(conn: *mut connection) -> libc::c_int;
    fn connSendTimeout(conn: *mut connection, ms: libc::c_longlong) -> libc::c_int;
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn lua_close(L: *mut lua_State);
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_checkstack(L: *mut lua_State, sz: libc::c_int) -> libc::c_int;
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tonumber(L: *mut lua_State, idx: libc::c_int) -> lua_Number;
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_topointer(L: *mut lua_State, idx: libc::c_int) -> *const libc::c_void;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, l: size_t);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_gettable(L: *mut lua_State, idx: libc::c_int);
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_settable(L: *mut lua_State, idx: libc::c_int);
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_pcall(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
    ) -> libc::c_int;
    fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_getstack(
        L: *mut lua_State,
        level: libc::c_int,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    fn lua_getinfo(
        L: *mut lua_State,
        what: *const libc::c_char,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    fn lua_getlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    static mut getMonotonicUs: Option::<unsafe extern "C" fn() -> monotime>;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictGenCaseHashFunction(buf: *const libc::c_uchar, len: size_t) -> uint64_t;
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listSearchKey(list: *mut list, key: *mut libc::c_void) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn string2ll(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_longlong,
    ) -> libc::c_int;
    fn string2l(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_long,
    ) -> libc::c_int;
    fn SHA1Init(context: *mut SHA1_CTX);
    fn SHA1Update(context: *mut SHA1_CTX, data: *const libc::c_uchar, len: uint32_t);
    fn SHA1Final(digest: *mut libc::c_uchar, context: *mut SHA1_CTX);
    fn sdsfree(s: sds);
    fn sdsdup(s: sds) -> sds;
    fn sdsempty() -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn getLongLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_longlong,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn replicationFeedMonitors(
        c: *mut client,
        monitors: *mut list,
        dictid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
    );
    fn redisFork(purpose: libc::c_int) -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn selectDb(c: *mut client, id: libc::c_int) -> libc::c_int;
    fn dictSdsDestructor(d: *mut dict, val: *mut libc::c_void);
    fn dictSdsKeyCaseCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn _serverAssertWithInfo(
        c: *const client,
        o: *const robj,
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn freeLuaScriptsAsync(lua_scripts: *mut dict);
    fn scriptPrepareForRun(
        r_ctx: *mut scriptRunCtx,
        engine_client: *mut client,
        caller: *mut client,
        funcname: *const libc::c_char,
        script_flags: uint64_t,
        ro: libc::c_int,
    ) -> libc::c_int;
    fn luaPushError(lua: *mut lua_State, error: *const libc::c_char);
    fn luaError(lua: *mut lua_State) -> libc::c_int;
    fn luaGetFromRegistry(
        lua: *mut lua_State,
        name: *const libc::c_char,
    ) -> *mut libc::c_void;
    static mut scripts_flags_def: [scriptFlag; 0];
    fn luaCallFunction(
        r_ctx: *mut scriptRunCtx,
        lua: *mut lua_State,
        keys: *mut *mut robj,
        nkeys: size_t,
        args: *mut *mut robj,
        nargs: size_t,
        debug_enabled: libc::c_int,
    );
    fn luaRegisterRedisAPI(lua: *mut lua_State);
    fn scriptResetRun(r_ctx: *mut scriptRunCtx);
    fn scriptFlagsToCmdFlags(cmd_flags: uint64_t, script_flags: uint64_t) -> uint64_t;
    fn scriptKill(c: *mut client, is_eval: libc::c_int);
    fn luaSetTableProtectionRecursively(lua: *mut lua_State);
    fn luaMemory(lua: *mut lua_State) -> libc::c_ulong;
    fn luaSetErrorMetatable(lua: *mut lua_State);
    fn luaL_newstate() -> *mut lua_State;
    fn luaL_loadbuffer(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uint_least64_t = __uint64_t;
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type pthread_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uint_least64_t = __uint_least64_t;
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeEventLoop {
    pub maxfd: libc::c_int,
    pub setsize: libc::c_int,
    pub timeEventNextId: libc::c_longlong,
    pub events: *mut aeFileEvent,
    pub fired: *mut aeFiredEvent,
    pub timeEventHead: *mut aeTimeEvent,
    pub stop: libc::c_int,
    pub apidata: *mut libc::c_void,
    pub beforesleep: Option::<aeBeforeSleepProc>,
    pub aftersleep: Option::<aeBeforeSleepProc>,
    pub flags: libc::c_int,
}
pub type aeBeforeSleepProc = unsafe extern "C" fn(*mut aeEventLoop) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeTimeEvent {
    pub id: libc::c_longlong,
    pub when: monotime,
    pub timeProc: Option::<aeTimeProc>,
    pub finalizerProc: Option::<aeEventFinalizerProc>,
    pub clientData: *mut libc::c_void,
    pub prev: *mut aeTimeEvent,
    pub next: *mut aeTimeEvent,
    pub refcount: libc::c_int,
}
pub type aeEventFinalizerProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    *mut libc::c_void,
) -> ();
pub type aeTimeProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_longlong,
    *mut libc::c_void,
) -> libc::c_int;
pub type monotime = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFiredEvent {
    pub fd: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFileEvent {
    pub mask: libc::c_int,
    pub rfileProc: Option::<aeFileProc>,
    pub wfileProc: Option::<aeFileProc>,
    pub clientData: *mut libc::c_void,
}
pub type aeFileProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_int,
    *mut libc::c_void,
    libc::c_int,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub type_0: *mut ConnectionType,
    pub state: ConnectionState,
    pub flags: libc::c_short,
    pub refs: libc::c_short,
    pub last_errno: libc::c_int,
    pub private_data: *mut libc::c_void,
    pub conn_handler: ConnectionCallbackFunc,
    pub write_handler: ConnectionCallbackFunc,
    pub read_handler: ConnectionCallbackFunc,
    pub fd: libc::c_int,
}
pub type ConnectionCallbackFunc = Option::<unsafe extern "C" fn(*mut connection) -> ()>;
pub type ConnectionState = libc::c_uint;
pub const CONN_STATE_ERROR: ConnectionState = 5;
pub const CONN_STATE_CLOSED: ConnectionState = 4;
pub const CONN_STATE_CONNECTED: ConnectionState = 3;
pub const CONN_STATE_ACCEPTING: ConnectionState = 2;
pub const CONN_STATE_CONNECTING: ConnectionState = 1;
pub const CONN_STATE_NONE: ConnectionState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConnectionType {
    pub ae_handler: Option::<
        unsafe extern "C" fn(
            *mut aeEventLoop,
            libc::c_int,
            *mut libc::c_void,
            libc::c_int,
        ) -> (),
    >,
    pub connect: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *const libc::c_char,
            libc::c_int,
            *const libc::c_char,
            ConnectionCallbackFunc,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut connection, *const libc::c_void, size_t) -> libc::c_int,
    >,
    pub writev: Option::<
        unsafe extern "C" fn(*mut connection, *const iovec, libc::c_int) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void, size_t) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut connection) -> ()>,
    pub accept: Option::<
        unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc) -> libc::c_int,
    >,
    pub set_write_handler: Option::<
        unsafe extern "C" fn(
            *mut connection,
            ConnectionCallbackFunc,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub set_read_handler: Option::<
        unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc) -> libc::c_int,
    >,
    pub get_last_error: Option::<
        unsafe extern "C" fn(*mut connection) -> *const libc::c_char,
    >,
    pub blocking_connect: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *const libc::c_char,
            libc::c_int,
            libc::c_longlong,
        ) -> libc::c_int,
    >,
    pub sync_write: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub sync_read: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub sync_readline: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub get_type: Option::<unsafe extern "C" fn(*mut connection) -> libc::c_int>,
}
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct redisObject {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "4..=7")]
    #[bitfield(name = "lru", ty = "libc::c_uint", bits = "8..=31")]
    pub type_0_encoding_lru: [u8; 4],
    pub refcount: libc::c_int,
    pub ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub nups: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub short_src: [libc::c_char; 60],
    pub i_ci: libc::c_int,
}
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_histogram {
    pub lowest_discernible_value: int64_t,
    pub highest_trackable_value: int64_t,
    pub unit_magnitude: int32_t,
    pub significant_figures: int32_t,
    pub sub_bucket_half_count_magnitude: int32_t,
    pub sub_bucket_half_count: int32_t,
    pub sub_bucket_mask: int64_t,
    pub sub_bucket_count: int32_t,
    pub bucket_count: int32_t,
    pub min_value: int64_t,
    pub max_value: int64_t,
    pub normalizing_index_offset: int32_t,
    pub conversion_ratio: libc::c_double,
    pub counts_len: int32_t,
    pub total_count: int64_t,
    pub counts: *mut int64_t,
}
pub type mstime_t = libc::c_longlong;
pub type ustime_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictEntry {
    pub key: *mut libc::c_void,
    pub v: C2RustUnnamed_10,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct raxNode {
    #[bitfield(name = "iskey", ty = "uint32_t", bits = "0..=0")]
    #[bitfield(name = "isnull", ty = "uint32_t", bits = "1..=1")]
    #[bitfield(name = "iscompr", ty = "uint32_t", bits = "2..=2")]
    #[bitfield(name = "size", ty = "uint32_t", bits = "3..=31")]
    pub iskey_isnull_iscompr_size: [u8; 4],
    pub data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rax {
    pub head: *mut raxNode,
    pub numele: uint64_t,
    pub numnodes: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1_CTX {
    pub state: [uint32_t; 5],
    pub count: [uint32_t; 2],
    pub buffer: [libc::c_uchar; 64],
}
pub type pause_type = libc::c_uint;
pub const CLIENT_PAUSE_ALL: pause_type = 2;
pub const CLIENT_PAUSE_WRITE: pause_type = 1;
pub const CLIENT_PAUSE_OFF: pause_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pause_event {
    pub type_0: pause_type,
    pub end: mstime_t,
}
pub type robj = redisObject;
pub type RedisModuleUserChangedFunc = Option::<
    unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterSlotToKeyMapping {
    pub by_slot: [slotToKeys; 16384],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slotToKeys {
    pub count: uint64_t,
    pub head: *mut dictEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisDb {
    pub dict: *mut dict,
    pub expires: *mut dict,
    pub blocking_keys: *mut dict,
    pub ready_keys: *mut dict,
    pub watched_keys: *mut dict,
    pub id: libc::c_int,
    pub avg_ttl: libc::c_longlong,
    pub expires_cursor: libc::c_ulong,
    pub defrag_later: *mut list,
    pub slots_to_keys: *mut clusterSlotToKeyMapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiCmd {
    pub argv: *mut *mut robj,
    pub argv_len: libc::c_int,
    pub argc: libc::c_int,
    pub cmd: *mut redisCommand,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCommand {
    pub declared_name: *const libc::c_char,
    pub summary: *const libc::c_char,
    pub complexity: *const libc::c_char,
    pub since: *const libc::c_char,
    pub doc_flags: libc::c_int,
    pub replaced_by: *const libc::c_char,
    pub deprecated_since: *const libc::c_char,
    pub group: redisCommandGroup,
    pub history: *mut commandHistory,
    pub tips: *mut *const libc::c_char,
    pub proc_0: Option::<redisCommandProc>,
    pub arity: libc::c_int,
    pub flags: uint64_t,
    pub acl_categories: uint64_t,
    pub key_specs_static: [keySpec; 4],
    pub getkeys_proc: Option::<redisGetKeysProc>,
    pub subcommands: *mut redisCommand,
    pub args: *mut redisCommandArg,
    pub microseconds: libc::c_longlong,
    pub calls: libc::c_longlong,
    pub rejected_calls: libc::c_longlong,
    pub failed_calls: libc::c_longlong,
    pub id: libc::c_int,
    pub fullname: sds,
    pub latency_histogram: *mut hdr_histogram,
    pub key_specs: *mut keySpec,
    pub legacy_range_key_spec: keySpec,
    pub num_args: libc::c_int,
    pub num_history: libc::c_int,
    pub num_tips: libc::c_int,
    pub key_specs_num: libc::c_int,
    pub key_specs_max: libc::c_int,
    pub subcommands_dict: *mut dict,
    pub parent: *mut redisCommand,
    pub module_cmd: *mut RedisModuleCommand,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keySpec {
    pub notes: *const libc::c_char,
    pub flags: uint64_t,
    pub begin_search_type: kspec_bs_type,
    pub bs: C2RustUnnamed_14,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub range: C2RustUnnamed_13,
    pub keynum: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub lastkey: libc::c_int,
    pub keystep: libc::c_int,
    pub limit: libc::c_int,
}
pub type kspec_fk_type = libc::c_uint;
pub const KSPEC_FK_KEYNUM: kspec_fk_type = 3;
pub const KSPEC_FK_RANGE: kspec_fk_type = 2;
pub const KSPEC_FK_UNKNOWN: kspec_fk_type = 1;
pub const KSPEC_FK_INVALID: kspec_fk_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub index: C2RustUnnamed_16,
    pub keyword: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub pos: libc::c_int,
}
pub type kspec_bs_type = libc::c_uint;
pub const KSPEC_BS_KEYWORD: kspec_bs_type = 3;
pub const KSPEC_BS_INDEX: kspec_bs_type = 2;
pub const KSPEC_BS_UNKNOWN: kspec_bs_type = 1;
pub const KSPEC_BS_INVALID: kspec_bs_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCommandArg {
    pub name: *const libc::c_char,
    pub type_0: redisCommandArgType,
    pub key_spec_index: libc::c_int,
    pub token: *const libc::c_char,
    pub summary: *const libc::c_char,
    pub since: *const libc::c_char,
    pub flags: libc::c_int,
    pub deprecated_since: *const libc::c_char,
    pub subargs: *mut redisCommandArg,
    pub num_args: libc::c_int,
}
pub type redisCommandArgType = libc::c_uint;
pub const ARG_TYPE_BLOCK: redisCommandArgType = 8;
pub const ARG_TYPE_ONEOF: redisCommandArgType = 7;
pub const ARG_TYPE_PURE_TOKEN: redisCommandArgType = 6;
pub const ARG_TYPE_UNIX_TIME: redisCommandArgType = 5;
pub const ARG_TYPE_PATTERN: redisCommandArgType = 4;
pub const ARG_TYPE_KEY: redisCommandArgType = 3;
pub const ARG_TYPE_DOUBLE: redisCommandArgType = 2;
pub const ARG_TYPE_INTEGER: redisCommandArgType = 1;
pub const ARG_TYPE_STRING: redisCommandArgType = 0;
pub type redisGetKeysProc = unsafe extern "C" fn(
    *mut redisCommand,
    *mut *mut robj,
    libc::c_int,
    *mut getKeysResult,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct getKeysResult {
    pub keysbuf: [keyReference; 256],
    pub keys: *mut keyReference,
    pub numkeys: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyReference {
    pub pos: libc::c_int,
    pub flags: libc::c_int,
}
pub type redisCommandProc = unsafe extern "C" fn(*mut client) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub id: uint64_t,
    pub flags: uint64_t,
    pub conn: *mut connection,
    pub resp: libc::c_int,
    pub db: *mut redisDb,
    pub name: *mut robj,
    pub querybuf: sds,
    pub qb_pos: size_t,
    pub querybuf_peak: size_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut robj,
    pub argv_len: libc::c_int,
    pub original_argc: libc::c_int,
    pub original_argv: *mut *mut robj,
    pub argv_len_sum: size_t,
    pub cmd: *mut redisCommand,
    pub lastcmd: *mut redisCommand,
    pub realcmd: *mut redisCommand,
    pub user: *mut user,
    pub reqtype: libc::c_int,
    pub multibulklen: libc::c_int,
    pub bulklen: libc::c_long,
    pub reply: *mut list,
    pub reply_bytes: libc::c_ulonglong,
    pub deferred_reply_errors: *mut list,
    pub sentlen: size_t,
    pub ctime: time_t,
    pub duration: libc::c_long,
    pub slot: libc::c_int,
    pub cur_script: *mut dictEntry,
    pub lastinteraction: time_t,
    pub obuf_soft_limit_reached_time: time_t,
    pub authenticated: libc::c_int,
    pub replstate: libc::c_int,
    pub repl_start_cmd_stream_on_ack: libc::c_int,
    pub repldbfd: libc::c_int,
    pub repldboff: off_t,
    pub repldbsize: off_t,
    pub replpreamble: sds,
    pub read_reploff: libc::c_longlong,
    pub reploff: libc::c_longlong,
    pub repl_applied: libc::c_longlong,
    pub repl_ack_off: libc::c_longlong,
    pub repl_ack_time: libc::c_longlong,
    pub repl_last_partial_write: libc::c_longlong,
    pub psync_initial_offset: libc::c_longlong,
    pub replid: [libc::c_char; 41],
    pub slave_listening_port: libc::c_int,
    pub slave_addr: *mut libc::c_char,
    pub slave_capa: libc::c_int,
    pub slave_req: libc::c_int,
    pub mstate: multiState,
    pub btype: libc::c_int,
    pub bpop: blockingState,
    pub woff: libc::c_longlong,
    pub watched_keys: *mut list,
    pub pubsub_channels: *mut dict,
    pub pubsub_patterns: *mut list,
    pub pubsubshard_channels: *mut dict,
    pub peerid: sds,
    pub sockname: sds,
    pub client_list_node: *mut listNode,
    pub postponed_list_node: *mut listNode,
    pub pending_read_list_node: *mut listNode,
    pub auth_callback: RedisModuleUserChangedFunc,
    pub auth_callback_privdata: *mut libc::c_void,
    pub auth_module: *mut libc::c_void,
    pub client_tracking_redirection: uint64_t,
    pub client_tracking_prefixes: *mut rax,
    pub last_memory_usage: size_t,
    pub last_memory_type: libc::c_int,
    pub mem_usage_bucket_node: *mut listNode,
    pub mem_usage_bucket: *mut clientMemUsageBucket,
    pub ref_repl_buf_node: *mut listNode,
    pub ref_block_pos: size_t,
    pub buf_peak: size_t,
    pub buf_peak_last_reset_time: mstime_t,
    pub bufpos: libc::c_int,
    pub buf_usable_size: size_t,
    pub buf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientMemUsageBucket {
    pub clients: *mut list,
    pub mem_usage_sum: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockingState {
    pub count: libc::c_long,
    pub timeout: mstime_t,
    pub keys: *mut dict,
    pub target: *mut robj,
    pub blockpos: blockPos,
    pub xread_count: size_t,
    pub xread_group: *mut robj,
    pub xread_consumer: *mut robj,
    pub xread_group_noack: libc::c_int,
    pub numreplicas: libc::c_int,
    pub reploffset: libc::c_longlong,
    pub module_blocked_handle: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockPos {
    pub wherefrom: libc::c_int,
    pub whereto: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiState {
    pub commands: *mut multiCmd,
    pub count: libc::c_int,
    pub cmd_flags: libc::c_int,
    pub cmd_inv_flags: libc::c_int,
    pub argv_len_sums: size_t,
    pub alloc_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user {
    pub name: sds,
    pub flags: uint32_t,
    pub passwords: *mut list,
    pub selectors: *mut list,
    pub acl_string: *mut robj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct commandHistory {
    pub since: *const libc::c_char,
    pub changes: *const libc::c_char,
}
pub type redisCommandGroup = libc::c_uint;
pub const COMMAND_GROUP_MODULE: redisCommandGroup = 17;
pub const COMMAND_GROUP_BITMAP: redisCommandGroup = 16;
pub const COMMAND_GROUP_STREAM: redisCommandGroup = 15;
pub const COMMAND_GROUP_GEO: redisCommandGroup = 14;
pub const COMMAND_GROUP_SENTINEL: redisCommandGroup = 13;
pub const COMMAND_GROUP_CLUSTER: redisCommandGroup = 12;
pub const COMMAND_GROUP_HYPERLOGLOG: redisCommandGroup = 11;
pub const COMMAND_GROUP_SCRIPTING: redisCommandGroup = 10;
pub const COMMAND_GROUP_SERVER: redisCommandGroup = 9;
pub const COMMAND_GROUP_CONNECTION: redisCommandGroup = 8;
pub const COMMAND_GROUP_TRANSACTIONS: redisCommandGroup = 7;
pub const COMMAND_GROUP_PUBSUB: redisCommandGroup = 6;
pub const COMMAND_GROUP_HASH: redisCommandGroup = 5;
pub const COMMAND_GROUP_SORTED_SET: redisCommandGroup = 4;
pub const COMMAND_GROUP_SET: redisCommandGroup = 3;
pub const COMMAND_GROUP_LIST: redisCommandGroup = 2;
pub const COMMAND_GROUP_STRING: redisCommandGroup = 1;
pub const COMMAND_GROUP_GENERIC: redisCommandGroup = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replBacklog {
    pub ref_repl_buf_node: *mut listNode,
    pub unindexed_count: size_t,
    pub blocks_index: *mut rax,
    pub histlen: libc::c_longlong,
    pub offset: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saveparam {
    pub seconds: time_t,
    pub changes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentinelConfig {
    pub pre_monitor_cfg: *mut list,
    pub monitor_cfg: *mut list,
    pub post_monitor_cfg: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sharedObjectsStruct {
    pub crlf: *mut robj,
    pub ok: *mut robj,
    pub err: *mut robj,
    pub emptybulk: *mut robj,
    pub czero: *mut robj,
    pub cone: *mut robj,
    pub pong: *mut robj,
    pub space: *mut robj,
    pub queued: *mut robj,
    pub null: [*mut robj; 4],
    pub nullarray: [*mut robj; 4],
    pub emptymap: [*mut robj; 4],
    pub emptyset: [*mut robj; 4],
    pub emptyarray: *mut robj,
    pub wrongtypeerr: *mut robj,
    pub nokeyerr: *mut robj,
    pub syntaxerr: *mut robj,
    pub sameobjecterr: *mut robj,
    pub outofrangeerr: *mut robj,
    pub noscripterr: *mut robj,
    pub loadingerr: *mut robj,
    pub slowevalerr: *mut robj,
    pub slowscripterr: *mut robj,
    pub slowmoduleerr: *mut robj,
    pub bgsaveerr: *mut robj,
    pub masterdownerr: *mut robj,
    pub roslaveerr: *mut robj,
    pub execaborterr: *mut robj,
    pub noautherr: *mut robj,
    pub noreplicaserr: *mut robj,
    pub busykeyerr: *mut robj,
    pub oomerr: *mut robj,
    pub plus: *mut robj,
    pub messagebulk: *mut robj,
    pub pmessagebulk: *mut robj,
    pub subscribebulk: *mut robj,
    pub unsubscribebulk: *mut robj,
    pub psubscribebulk: *mut robj,
    pub punsubscribebulk: *mut robj,
    pub del: *mut robj,
    pub unlink: *mut robj,
    pub rpop: *mut robj,
    pub lpop: *mut robj,
    pub lpush: *mut robj,
    pub rpoplpush: *mut robj,
    pub lmove: *mut robj,
    pub blmove: *mut robj,
    pub zpopmin: *mut robj,
    pub zpopmax: *mut robj,
    pub emptyscan: *mut robj,
    pub multi: *mut robj,
    pub exec: *mut robj,
    pub left: *mut robj,
    pub right: *mut robj,
    pub hset: *mut robj,
    pub srem: *mut robj,
    pub xgroup: *mut robj,
    pub xclaim: *mut robj,
    pub script: *mut robj,
    pub replconf: *mut robj,
    pub eval: *mut robj,
    pub persist: *mut robj,
    pub set: *mut robj,
    pub pexpireat: *mut robj,
    pub pexpire: *mut robj,
    pub time: *mut robj,
    pub pxat: *mut robj,
    pub absttl: *mut robj,
    pub retrycount: *mut robj,
    pub force: *mut robj,
    pub justid: *mut robj,
    pub entriesread: *mut robj,
    pub lastid: *mut robj,
    pub ping: *mut robj,
    pub setid: *mut robj,
    pub keepttl: *mut robj,
    pub load: *mut robj,
    pub createconsumer: *mut robj,
    pub getack: *mut robj,
    pub special_asterick: *mut robj,
    pub special_equals: *mut robj,
    pub default_username: *mut robj,
    pub redacted: *mut robj,
    pub ssubscribebulk: *mut robj,
    pub sunsubscribebulk: *mut robj,
    pub smessagebulk: *mut robj,
    pub select: [*mut robj; 10],
    pub integers: [*mut robj; 10000],
    pub mbulkhdr: [*mut robj; 32],
    pub bulkhdr: [*mut robj; 32],
    pub maphdr: [*mut robj; 32],
    pub sethdr: [*mut robj; 32],
    pub minstring: sds,
    pub maxstring: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientBufferLimitsConfig {
    pub hard_limit_bytes: libc::c_ulonglong,
    pub soft_limit_bytes: libc::c_ulonglong,
    pub soft_limit_seconds: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisOp {
    pub argv: *mut *mut robj,
    pub argc: libc::c_int,
    pub dbid: libc::c_int,
    pub target: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisOpArray {
    pub ops: *mut redisOp,
    pub numops: libc::c_int,
    pub capacity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct malloc_stats {
    pub zmalloc_used: size_t,
    pub process_rss: size_t,
    pub allocator_allocated: size_t,
    pub allocator_active: size_t,
    pub allocator_resident: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socketFds {
    pub fd: [libc::c_int; 16],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisTLSContextConfig {
    pub cert_file: *mut libc::c_char,
    pub key_file: *mut libc::c_char,
    pub key_file_pass: *mut libc::c_char,
    pub client_cert_file: *mut libc::c_char,
    pub client_key_file: *mut libc::c_char,
    pub client_key_file_pass: *mut libc::c_char,
    pub dh_params_file: *mut libc::c_char,
    pub ca_cert_file: *mut libc::c_char,
    pub ca_cert_dir: *mut libc::c_char,
    pub protocols: *mut libc::c_char,
    pub ciphers: *mut libc::c_char,
    pub ciphersuites: *mut libc::c_char,
    pub prefer_server_ciphers: libc::c_int,
    pub session_caching: libc::c_int,
    pub session_cache_size: libc::c_int,
    pub session_cache_timeout: libc::c_int,
}
pub type aof_file_type = libc::c_uint;
pub const AOF_FILE_TYPE_INCR: aof_file_type = 105;
pub const AOF_FILE_TYPE_HIST: aof_file_type = 104;
pub const AOF_FILE_TYPE_BASE: aof_file_type = 98;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aofInfo {
    pub file_name: sds,
    pub file_seq: libc::c_longlong,
    pub file_type: aof_file_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aofManifest {
    pub base_aof_info: *mut aofInfo,
    pub incr_aof_list: *mut list,
    pub history_aof_list: *mut list,
    pub curr_base_file_seq: libc::c_longlong,
    pub curr_incr_file_seq: libc::c_longlong,
    pub dirty: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisServer {
    pub pid: pid_t,
    pub main_thread_id: pthread_t,
    pub configfile: *mut libc::c_char,
    pub executable: *mut libc::c_char,
    pub exec_argv: *mut *mut libc::c_char,
    pub dynamic_hz: libc::c_int,
    pub config_hz: libc::c_int,
    pub umask: mode_t,
    pub hz: libc::c_int,
    pub in_fork_child: libc::c_int,
    pub db: *mut redisDb,
    pub commands: *mut dict,
    pub orig_commands: *mut dict,
    pub el: *mut aeEventLoop,
    pub errors: *mut rax,
    pub lruclock: atomic_uint,
    pub shutdown_asap: sig_atomic_t,
    pub shutdown_mstime: mstime_t,
    pub last_sig_received: libc::c_int,
    pub shutdown_flags: libc::c_int,
    pub activerehashing: libc::c_int,
    pub active_defrag_running: libc::c_int,
    pub pidfile: *mut libc::c_char,
    pub arch_bits: libc::c_int,
    pub cronloops: libc::c_int,
    pub runid: [libc::c_char; 41],
    pub sentinel_mode: libc::c_int,
    pub initial_memory_usage: size_t,
    pub always_show_logo: libc::c_int,
    pub in_exec: libc::c_int,
    pub busy_module_yield_flags: libc::c_int,
    pub busy_module_yield_reply: *const libc::c_char,
    pub core_propagates: libc::c_int,
    pub propagate_no_multi: libc::c_int,
    pub module_ctx_nesting: libc::c_int,
    pub ignore_warnings: *mut libc::c_char,
    pub client_pause_in_transaction: libc::c_int,
    pub thp_enabled: libc::c_int,
    pub page_size: size_t,
    pub moduleapi: *mut dict,
    pub sharedapi: *mut dict,
    pub module_configs_queue: *mut dict,
    pub loadmodule_queue: *mut list,
    pub module_pipe: [libc::c_int; 2],
    pub child_pid: pid_t,
    pub child_type: libc::c_int,
    pub port: libc::c_int,
    pub tls_port: libc::c_int,
    pub tcp_backlog: libc::c_int,
    pub bindaddr: [*mut libc::c_char; 16],
    pub bindaddr_count: libc::c_int,
    pub bind_source_addr: *mut libc::c_char,
    pub unixsocket: *mut libc::c_char,
    pub unixsocketperm: libc::c_uint,
    pub ipfd: socketFds,
    pub tlsfd: socketFds,
    pub sofd: libc::c_int,
    pub socket_mark_id: uint32_t,
    pub cfd: socketFds,
    pub clients: *mut list,
    pub clients_to_close: *mut list,
    pub clients_pending_write: *mut list,
    pub clients_pending_read: *mut list,
    pub slaves: *mut list,
    pub monitors: *mut list,
    pub current_client: *mut client,
    pub client_mem_usage_buckets: *mut clientMemUsageBucket,
    pub clients_timeout_table: *mut rax,
    pub fixed_time_expire: libc::c_long,
    pub in_nested_call: libc::c_int,
    pub clients_index: *mut rax,
    pub client_pause_type: pause_type,
    pub postponed_clients: *mut list,
    pub client_pause_end_time: mstime_t,
    pub client_pause_per_purpose: [*mut pause_event; 3],
    pub neterr: [libc::c_char; 256],
    pub migrate_cached_sockets: *mut dict,
    pub next_client_id: uint_least64_t,
    pub protected_mode: libc::c_int,
    pub io_threads_num: libc::c_int,
    pub io_threads_do_reads: libc::c_int,
    pub io_threads_active: libc::c_int,
    pub events_processed_while_blocked: libc::c_longlong,
    pub enable_protected_configs: libc::c_int,
    pub enable_debug_cmd: libc::c_int,
    pub enable_module_cmd: libc::c_int,
    pub loading: sig_atomic_t,
    pub async_loading: sig_atomic_t,
    pub loading_total_bytes: off_t,
    pub loading_rdb_used_mem: off_t,
    pub loading_loaded_bytes: off_t,
    pub loading_start_time: time_t,
    pub loading_process_events_interval_bytes: off_t,
    pub stat_starttime: time_t,
    pub stat_numcommands: libc::c_longlong,
    pub stat_numconnections: libc::c_longlong,
    pub stat_expiredkeys: libc::c_longlong,
    pub stat_expired_stale_perc: libc::c_double,
    pub stat_expired_time_cap_reached_count: libc::c_longlong,
    pub stat_expire_cycle_time_used: libc::c_longlong,
    pub stat_evictedkeys: libc::c_longlong,
    pub stat_evictedclients: libc::c_longlong,
    pub stat_total_eviction_exceeded_time: libc::c_longlong,
    pub stat_last_eviction_exceeded_time: monotime,
    pub stat_keyspace_hits: libc::c_longlong,
    pub stat_keyspace_misses: libc::c_longlong,
    pub stat_active_defrag_hits: libc::c_longlong,
    pub stat_active_defrag_misses: libc::c_longlong,
    pub stat_active_defrag_key_hits: libc::c_longlong,
    pub stat_active_defrag_key_misses: libc::c_longlong,
    pub stat_active_defrag_scanned: libc::c_longlong,
    pub stat_total_active_defrag_time: libc::c_longlong,
    pub stat_last_active_defrag_time: monotime,
    pub stat_peak_memory: size_t,
    pub stat_aof_rewrites: libc::c_longlong,
    pub stat_aofrw_consecutive_failures: libc::c_longlong,
    pub stat_rdb_saves: libc::c_longlong,
    pub stat_fork_time: libc::c_longlong,
    pub stat_fork_rate: libc::c_double,
    pub stat_total_forks: libc::c_longlong,
    pub stat_rejected_conn: libc::c_longlong,
    pub stat_sync_full: libc::c_longlong,
    pub stat_sync_partial_ok: libc::c_longlong,
    pub stat_sync_partial_err: libc::c_longlong,
    pub slowlog: *mut list,
    pub slowlog_entry_id: libc::c_longlong,
    pub slowlog_log_slower_than: libc::c_longlong,
    pub slowlog_max_len: libc::c_ulong,
    pub cron_malloc_stats: malloc_stats,
    pub stat_net_input_bytes: atomic_llong,
    pub stat_net_output_bytes: atomic_llong,
    pub stat_net_repl_input_bytes: atomic_llong,
    pub stat_net_repl_output_bytes: atomic_llong,
    pub stat_current_cow_peak: size_t,
    pub stat_current_cow_bytes: size_t,
    pub stat_current_cow_updated: monotime,
    pub stat_current_save_keys_processed: size_t,
    pub stat_current_save_keys_total: size_t,
    pub stat_rdb_cow_bytes: size_t,
    pub stat_aof_cow_bytes: size_t,
    pub stat_module_cow_bytes: size_t,
    pub stat_module_progress: libc::c_double,
    pub stat_clients_type_memory: [size_t; 4],
    pub stat_cluster_links_memory: size_t,
    pub stat_unexpected_error_replies: libc::c_longlong,
    pub stat_total_error_replies: libc::c_longlong,
    pub stat_dump_payload_sanitizations: libc::c_longlong,
    pub stat_io_reads_processed: libc::c_longlong,
    pub stat_io_writes_processed: libc::c_longlong,
    pub stat_total_reads_processed: atomic_llong,
    pub stat_total_writes_processed: atomic_llong,
    pub inst_metric: [C2RustUnnamed_17; 5],
    pub stat_reply_buffer_shrinks: libc::c_longlong,
    pub stat_reply_buffer_expands: libc::c_longlong,
    pub verbosity: libc::c_int,
    pub maxidletime: libc::c_int,
    pub tcpkeepalive: libc::c_int,
    pub active_expire_enabled: libc::c_int,
    pub active_expire_effort: libc::c_int,
    pub active_defrag_enabled: libc::c_int,
    pub sanitize_dump_payload: libc::c_int,
    pub skip_checksum_validation: libc::c_int,
    pub jemalloc_bg_thread: libc::c_int,
    pub active_defrag_ignore_bytes: size_t,
    pub active_defrag_threshold_lower: libc::c_int,
    pub active_defrag_threshold_upper: libc::c_int,
    pub active_defrag_cycle_min: libc::c_int,
    pub active_defrag_cycle_max: libc::c_int,
    pub active_defrag_max_scan_fields: libc::c_ulong,
    pub client_max_querybuf_len: size_t,
    pub dbnum: libc::c_int,
    pub supervised: libc::c_int,
    pub supervised_mode: libc::c_int,
    pub daemonize: libc::c_int,
    pub set_proc_title: libc::c_int,
    pub proc_title_template: *mut libc::c_char,
    pub client_obuf_limits: [clientBufferLimitsConfig; 3],
    pub pause_cron: libc::c_int,
    pub latency_tracking_enabled: libc::c_int,
    pub latency_tracking_info_percentiles: *mut libc::c_double,
    pub latency_tracking_info_percentiles_len: libc::c_int,
    pub aof_enabled: libc::c_int,
    pub aof_state: libc::c_int,
    pub aof_fsync: libc::c_int,
    pub aof_filename: *mut libc::c_char,
    pub aof_dirname: *mut libc::c_char,
    pub aof_no_fsync_on_rewrite: libc::c_int,
    pub aof_rewrite_perc: libc::c_int,
    pub aof_rewrite_min_size: off_t,
    pub aof_rewrite_base_size: off_t,
    pub aof_current_size: off_t,
    pub aof_last_incr_size: off_t,
    pub aof_fsync_offset: off_t,
    pub aof_flush_sleep: libc::c_int,
    pub aof_rewrite_scheduled: libc::c_int,
    pub aof_buf: sds,
    pub aof_fd: libc::c_int,
    pub aof_selected_db: libc::c_int,
    pub aof_flush_postponed_start: time_t,
    pub aof_last_fsync: time_t,
    pub aof_rewrite_time_last: time_t,
    pub aof_rewrite_time_start: time_t,
    pub aof_cur_timestamp: time_t,
    pub aof_timestamp_enabled: libc::c_int,
    pub aof_lastbgrewrite_status: libc::c_int,
    pub aof_delayed_fsync: libc::c_ulong,
    pub aof_rewrite_incremental_fsync: libc::c_int,
    pub rdb_save_incremental_fsync: libc::c_int,
    pub aof_last_write_status: libc::c_int,
    pub aof_last_write_errno: libc::c_int,
    pub aof_load_truncated: libc::c_int,
    pub aof_use_rdb_preamble: libc::c_int,
    pub aof_bio_fsync_status: atomic_int,
    pub aof_bio_fsync_errno: atomic_int,
    pub aof_manifest: *mut aofManifest,
    pub aof_disable_auto_gc: libc::c_int,
    pub dirty: libc::c_longlong,
    pub dirty_before_bgsave: libc::c_longlong,
    pub rdb_last_load_keys_expired: libc::c_longlong,
    pub rdb_last_load_keys_loaded: libc::c_longlong,
    pub saveparams: *mut saveparam,
    pub saveparamslen: libc::c_int,
    pub rdb_filename: *mut libc::c_char,
    pub rdb_compression: libc::c_int,
    pub rdb_checksum: libc::c_int,
    pub rdb_del_sync_files: libc::c_int,
    pub lastsave: time_t,
    pub lastbgsave_try: time_t,
    pub rdb_save_time_last: time_t,
    pub rdb_save_time_start: time_t,
    pub rdb_bgsave_scheduled: libc::c_int,
    pub rdb_child_type: libc::c_int,
    pub lastbgsave_status: libc::c_int,
    pub stop_writes_on_bgsave_err: libc::c_int,
    pub rdb_pipe_read: libc::c_int,
    pub rdb_child_exit_pipe: libc::c_int,
    pub rdb_pipe_conns: *mut *mut connection,
    pub rdb_pipe_numconns: libc::c_int,
    pub rdb_pipe_numconns_writing: libc::c_int,
    pub rdb_pipe_buff: *mut libc::c_char,
    pub rdb_pipe_bufflen: libc::c_int,
    pub rdb_key_save_delay: libc::c_int,
    pub key_load_delay: libc::c_int,
    pub child_info_pipe: [libc::c_int; 2],
    pub child_info_nread: libc::c_int,
    pub also_propagate: redisOpArray,
    pub replication_allowed: libc::c_int,
    pub logfile: *mut libc::c_char,
    pub syslog_enabled: libc::c_int,
    pub syslog_ident: *mut libc::c_char,
    pub syslog_facility: libc::c_int,
    pub crashlog_enabled: libc::c_int,
    pub memcheck_enabled: libc::c_int,
    pub use_exit_on_panic: libc::c_int,
    pub shutdown_timeout: libc::c_int,
    pub shutdown_on_sigint: libc::c_int,
    pub shutdown_on_sigterm: libc::c_int,
    pub replid: [libc::c_char; 41],
    pub replid2: [libc::c_char; 41],
    pub master_repl_offset: libc::c_longlong,
    pub second_replid_offset: libc::c_longlong,
    pub slaveseldb: libc::c_int,
    pub repl_ping_slave_period: libc::c_int,
    pub repl_backlog: *mut replBacklog,
    pub repl_backlog_size: libc::c_longlong,
    pub repl_backlog_time_limit: time_t,
    pub repl_no_slaves_since: time_t,
    pub repl_min_slaves_to_write: libc::c_int,
    pub repl_min_slaves_max_lag: libc::c_int,
    pub repl_good_slaves_count: libc::c_int,
    pub repl_diskless_sync: libc::c_int,
    pub repl_diskless_load: libc::c_int,
    pub repl_diskless_sync_delay: libc::c_int,
    pub repl_diskless_sync_max_replicas: libc::c_int,
    pub repl_buffer_mem: size_t,
    pub repl_buffer_blocks: *mut list,
    pub masteruser: *mut libc::c_char,
    pub masterauth: sds,
    pub masterhost: *mut libc::c_char,
    pub masterport: libc::c_int,
    pub repl_timeout: libc::c_int,
    pub master: *mut client,
    pub cached_master: *mut client,
    pub repl_syncio_timeout: libc::c_int,
    pub repl_state: libc::c_int,
    pub repl_transfer_size: off_t,
    pub repl_transfer_read: off_t,
    pub repl_transfer_last_fsync_off: off_t,
    pub repl_transfer_s: *mut connection,
    pub repl_transfer_fd: libc::c_int,
    pub repl_transfer_tmpfile: *mut libc::c_char,
    pub repl_transfer_lastio: time_t,
    pub repl_serve_stale_data: libc::c_int,
    pub repl_slave_ro: libc::c_int,
    pub repl_slave_ignore_maxmemory: libc::c_int,
    pub repl_down_since: time_t,
    pub repl_disable_tcp_nodelay: libc::c_int,
    pub slave_priority: libc::c_int,
    pub replica_announced: libc::c_int,
    pub slave_announce_port: libc::c_int,
    pub slave_announce_ip: *mut libc::c_char,
    pub propagation_error_behavior: libc::c_int,
    pub repl_ignore_disk_write_error: libc::c_int,
    pub master_replid: [libc::c_char; 41],
    pub master_initial_offset: libc::c_longlong,
    pub repl_slave_lazy_flush: libc::c_int,
    pub clients_waiting_acks: *mut list,
    pub get_ack_from_slaves: libc::c_int,
    pub maxclients: libc::c_uint,
    pub maxmemory: libc::c_ulonglong,
    pub maxmemory_clients: ssize_t,
    pub maxmemory_policy: libc::c_int,
    pub maxmemory_samples: libc::c_int,
    pub maxmemory_eviction_tenacity: libc::c_int,
    pub lfu_log_factor: libc::c_int,
    pub lfu_decay_time: libc::c_int,
    pub proto_max_bulk_len: libc::c_longlong,
    pub oom_score_adj_values: [libc::c_int; 3],
    pub oom_score_adj: libc::c_int,
    pub disable_thp: libc::c_int,
    pub blocked_clients: libc::c_uint,
    pub blocked_clients_by_type: [libc::c_uint; 8],
    pub unblocked_clients: *mut list,
    pub ready_keys: *mut list,
    pub tracking_clients: libc::c_uint,
    pub tracking_table_max_keys: size_t,
    pub tracking_pending_keys: *mut list,
    pub sort_desc: libc::c_int,
    pub sort_alpha: libc::c_int,
    pub sort_bypattern: libc::c_int,
    pub sort_store: libc::c_int,
    pub hash_max_listpack_entries: size_t,
    pub hash_max_listpack_value: size_t,
    pub set_max_intset_entries: size_t,
    pub zset_max_listpack_entries: size_t,
    pub zset_max_listpack_value: size_t,
    pub hll_sparse_max_bytes: size_t,
    pub stream_node_max_bytes: size_t,
    pub stream_node_max_entries: libc::c_longlong,
    pub list_max_listpack_size: libc::c_int,
    pub list_compress_depth: libc::c_int,
    pub unixtime: atomic_int,
    pub timezone: time_t,
    pub daylight_active: libc::c_int,
    pub mstime: mstime_t,
    pub ustime: ustime_t,
    pub blocking_op_nesting: size_t,
    pub blocked_last_cron: libc::c_longlong,
    pub pubsub_channels: *mut dict,
    pub pubsub_patterns: *mut dict,
    pub notify_keyspace_events: libc::c_int,
    pub pubsubshard_channels: *mut dict,
    pub cluster_enabled: libc::c_int,
    pub cluster_port: libc::c_int,
    pub cluster_node_timeout: mstime_t,
    pub cluster_configfile: *mut libc::c_char,
    pub cluster: *mut clusterState,
    pub cluster_migration_barrier: libc::c_int,
    pub cluster_allow_replica_migration: libc::c_int,
    pub cluster_slave_validity_factor: libc::c_int,
    pub cluster_require_full_coverage: libc::c_int,
    pub cluster_slave_no_failover: libc::c_int,
    pub cluster_announce_ip: *mut libc::c_char,
    pub cluster_announce_hostname: *mut libc::c_char,
    pub cluster_preferred_endpoint_type: libc::c_int,
    pub cluster_announce_port: libc::c_int,
    pub cluster_announce_tls_port: libc::c_int,
    pub cluster_announce_bus_port: libc::c_int,
    pub cluster_module_flags: libc::c_int,
    pub cluster_allow_reads_when_down: libc::c_int,
    pub cluster_config_file_lock_fd: libc::c_int,
    pub cluster_link_sendbuf_limit_bytes: libc::c_ulonglong,
    pub cluster_drop_packet_filter: libc::c_int,
    pub script_caller: *mut client,
    pub busy_reply_threshold: mstime_t,
    pub pre_command_oom_state: libc::c_int,
    pub script_disable_deny_script: libc::c_int,
    pub lazyfree_lazy_eviction: libc::c_int,
    pub lazyfree_lazy_expire: libc::c_int,
    pub lazyfree_lazy_server_del: libc::c_int,
    pub lazyfree_lazy_user_del: libc::c_int,
    pub lazyfree_lazy_user_flush: libc::c_int,
    pub latency_monitor_threshold: libc::c_longlong,
    pub latency_events: *mut dict,
    pub acl_filename: *mut libc::c_char,
    pub acllog_max_len: libc::c_ulong,
    pub requirepass: sds,
    pub acl_pubsub_default: libc::c_int,
    pub watchdog_period: libc::c_int,
    pub system_memory_size: size_t,
    pub tls_cluster: libc::c_int,
    pub tls_replication: libc::c_int,
    pub tls_auth_clients: libc::c_int,
    pub tls_ctx_config: redisTLSContextConfig,
    pub server_cpulist: *mut libc::c_char,
    pub bio_cpulist: *mut libc::c_char,
    pub aof_rewrite_cpulist: *mut libc::c_char,
    pub bgsave_cpulist: *mut libc::c_char,
    pub sentinel_config: *mut sentinelConfig,
    pub failover_end_time: mstime_t,
    pub force_failover: libc::c_int,
    pub target_replica_host: *mut libc::c_char,
    pub target_replica_port: libc::c_int,
    pub failover_state: libc::c_int,
    pub cluster_allow_pubsubshard_when_down: libc::c_int,
    pub reply_buffer_peak_reset_time: libc::c_long,
    pub reply_buffer_resizing_enabled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterState {
    pub myself: *mut clusterNode,
    pub currentEpoch: uint64_t,
    pub state: libc::c_int,
    pub size: libc::c_int,
    pub nodes: *mut dict,
    pub nodes_black_list: *mut dict,
    pub migrating_slots_to: [*mut clusterNode; 16384],
    pub importing_slots_from: [*mut clusterNode; 16384],
    pub slots: [*mut clusterNode; 16384],
    pub slots_to_channels: *mut rax,
    pub failover_auth_time: mstime_t,
    pub failover_auth_count: libc::c_int,
    pub failover_auth_sent: libc::c_int,
    pub failover_auth_rank: libc::c_int,
    pub failover_auth_epoch: uint64_t,
    pub cant_failover_reason: libc::c_int,
    pub mf_end: mstime_t,
    pub mf_slave: *mut clusterNode,
    pub mf_master_offset: libc::c_longlong,
    pub mf_can_start: libc::c_int,
    pub lastVoteEpoch: uint64_t,
    pub todo_before_sleep: libc::c_int,
    pub stats_bus_messages_sent: [libc::c_longlong; 11],
    pub stats_bus_messages_received: [libc::c_longlong; 11],
    pub stats_pfail_nodes: libc::c_longlong,
    pub stat_cluster_links_buffer_limit_exceeded: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterNode {
    pub ctime: mstime_t,
    pub name: [libc::c_char; 40],
    pub flags: libc::c_int,
    pub configEpoch: uint64_t,
    pub slots: [libc::c_uchar; 2048],
    pub slot_info_pairs: *mut uint16_t,
    pub slot_info_pairs_count: libc::c_int,
    pub numslots: libc::c_int,
    pub numslaves: libc::c_int,
    pub slaves: *mut *mut clusterNode,
    pub slaveof: *mut clusterNode,
    pub last_in_ping_gossip: libc::c_ulonglong,
    pub ping_sent: mstime_t,
    pub pong_received: mstime_t,
    pub data_received: mstime_t,
    pub fail_time: mstime_t,
    pub voted_time: mstime_t,
    pub repl_offset_time: mstime_t,
    pub orphaned_time: mstime_t,
    pub repl_offset: libc::c_longlong,
    pub ip: [libc::c_char; 46],
    pub hostname: sds,
    pub port: libc::c_int,
    pub pport: libc::c_int,
    pub cport: libc::c_int,
    pub link: *mut clusterLink,
    pub inbound_link: *mut clusterLink,
    pub fail_reports: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterLink {
    pub ctime: mstime_t,
    pub conn: *mut connection,
    pub sndbuf: sds,
    pub rcvbuf: *mut libc::c_char,
    pub rcvbuf_len: size_t,
    pub rcvbuf_alloc: size_t,
    pub node: *mut clusterNode,
    pub inbound: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaCtx {
    pub lua: *mut lua_State,
    pub lua_client: *mut client,
    pub lua_scripts: *mut dict,
    pub lua_scripts_mem: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldbState {
    pub conn: *mut connection,
    pub active: libc::c_int,
    pub forked: libc::c_int,
    pub logs: *mut list,
    pub traces: *mut list,
    pub children: *mut list,
    pub bp: [libc::c_int; 64],
    pub bpcount: libc::c_int,
    pub step: libc::c_int,
    pub luabp: libc::c_int,
    pub src: *mut sds,
    pub lines: libc::c_int,
    pub currentline: libc::c_int,
    pub cbuf: sds,
    pub maxlen: size_t,
    pub maxlen_hint_sent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaScript {
    pub flags: uint64_t,
    pub body: *mut robj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scriptFlag {
    pub flag: uint64_t,
    pub str_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scriptRunCtx {
    pub funcname: *const libc::c_char,
    pub c: *mut client,
    pub original_client: *mut client,
    pub flags: libc::c_int,
    pub repl_flags: libc::c_int,
    pub start_time: monotime,
    pub snapshot_time: mstime_t,
}
#[inline]
unsafe extern "C" fn connWrite(
    mut conn: *mut connection,
    mut data: *const libc::c_void,
    mut data_len: size_t,
) -> libc::c_int {
    return ((*(*conn).type_0).write)
        .expect("non-null function pointer")(conn, data, data_len);
}
#[inline]
unsafe extern "C" fn connRead(
    mut conn: *mut connection,
    mut buf: *mut libc::c_void,
    mut buf_len: size_t,
) -> libc::c_int {
    let mut ret: libc::c_int = ((*(*conn).type_0).read)
        .expect("non-null function pointer")(conn, buf, buf_len);
    return ret;
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
unsafe extern "C" fn elapsedUs(mut start_time: monotime) -> uint64_t {
    return (getMonotonicUs.expect("non-null function pointer")())
        .wrapping_sub(start_time);
}
#[inline]
unsafe extern "C" fn elapsedMs(mut start_time: monotime) -> uint64_t {
    return (elapsedUs(start_time)).wrapping_div(1000 as libc::c_int as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn sdssetlen(mut s: sds, mut newlen: size_t) {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => {
            let mut fp: *mut libc::c_uchar = (s as *mut libc::c_uchar)
                .offset(-(1 as libc::c_int as isize));
            *fp = (0 as libc::c_int as libc::c_ulong | newlen << 3 as libc::c_int)
                as libc::c_uchar;
        }
        1 => {
            (*(s.offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len = newlen as uint8_t;
        }
        2 => {
            (*(s.offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len = newlen as uint16_t;
        }
        3 => {
            (*(s.offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len = newlen as uint32_t;
        }
        4 => {
            (*(s.offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len = newlen;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn dictLuaScriptDestructor(
    mut d: *mut dict,
    mut val: *mut libc::c_void,
) {
    if val.is_null() {
        return;
    }
    decrRefCount((*(val as *mut luaScript)).body);
    zfree(val);
}
unsafe extern "C" fn dictStrCaseHash(mut key: *const libc::c_void) -> uint64_t {
    return dictGenCaseHashFunction(
        key as *mut libc::c_uchar,
        strlen(key as *mut libc::c_char),
    );
}
#[no_mangle]
pub static mut shaScriptObjectDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictStrCaseHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCaseCompare
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
                dictLuaScriptDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut lctx: luaCtx = luaCtx {
    lua: 0 as *const lua_State as *mut lua_State,
    lua_client: 0 as *const client as *mut client,
    lua_scripts: 0 as *const dict as *mut dict,
    lua_scripts_mem: 0,
};
#[no_mangle]
pub static mut ldb: ldbState = ldbState {
    conn: 0 as *const connection as *mut connection,
    active: 0,
    forked: 0,
    logs: 0 as *const list as *mut list,
    traces: 0 as *const list as *mut list,
    children: 0 as *const list as *mut list,
    bp: [0; 64],
    bpcount: 0,
    step: 0,
    luabp: 0,
    src: 0 as *const sds as *mut sds,
    lines: 0,
    currentline: 0,
    cbuf: 0 as *const libc::c_char as *mut libc::c_char,
    maxlen: 0,
    maxlen_hint_sent: 0,
};
#[no_mangle]
pub unsafe extern "C" fn sha1hex(
    mut digest: *mut libc::c_char,
    mut script: *mut libc::c_char,
    mut len: size_t,
) {
    let mut ctx: SHA1_CTX = SHA1_CTX {
        state: [0; 5],
        count: [0; 2],
        buffer: [0; 64],
    };
    let mut hash: [libc::c_uchar; 20] = [0; 20];
    let mut cset: *mut libc::c_char = b"0123456789abcdef\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    SHA1Init(&mut ctx);
    SHA1Update(&mut ctx, script as *mut libc::c_uchar, len as uint32_t);
    SHA1Final(hash.as_mut_ptr(), &mut ctx);
    j = 0 as libc::c_int;
    while j < 20 as libc::c_int {
        *digest
            .offset(
                (j * 2 as libc::c_int) as isize,
            ) = *cset
            .offset(
                ((hash[j as usize] as libc::c_int & 0xf0 as libc::c_int)
                    >> 4 as libc::c_int) as isize,
            );
        *digest
            .offset(
                (j * 2 as libc::c_int + 1 as libc::c_int) as isize,
            ) = *cset
            .offset((hash[j as usize] as libc::c_int & 0xf as libc::c_int) as isize);
        j += 1;
    }
    *digest.offset(40 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn luaRedisBreakpointCommand(
    mut lua: *mut lua_State,
) -> libc::c_int {
    if ldb.active != 0 {
        ldb.luabp = 1 as libc::c_int;
        lua_pushboolean(lua, 1 as libc::c_int);
    } else {
        lua_pushboolean(lua, 0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaRedisDebugCommand(mut lua: *mut lua_State) -> libc::c_int {
    if ldb.active == 0 {
        return 0 as libc::c_int;
    }
    let mut argc: libc::c_int = lua_gettop(lua);
    let mut log: sds = sdscatprintf(
        sdsempty(),
        b"<debug> line %d: \0" as *const u8 as *const libc::c_char,
        ldb.currentline,
    );
    loop {
        let fresh0 = argc;
        argc = argc - 1;
        if !(fresh0 != 0) {
            break;
        }
        log = ldbCatStackValue(log, lua, -(1 as libc::c_int) - argc);
        if argc != 0 as libc::c_int {
            log = sdscatlen(
                log,
                b", \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
        }
    }
    ldbLog(log);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaRedisReplicateCommandsCommand(
    mut lua: *mut lua_State,
) -> libc::c_int {
    lua_pushboolean(lua, 1 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scriptingInit(mut setup: libc::c_int) {
    let mut lua: *mut lua_State = luaL_newstate();
    if setup != 0 {
        lctx.lua_client = 0 as *mut client;
        server.script_caller = 0 as *mut client;
        server.script_disable_deny_script = 0 as libc::c_int;
        ldbInit();
    }
    lctx.lua_scripts = dictCreate(&mut shaScriptObjectDictType);
    lctx.lua_scripts_mem = 0 as libc::c_int as libc::c_ulonglong;
    luaRegisterRedisAPI(lua);
    lua_getfield(
        lua,
        -(10002 as libc::c_int),
        b"redis\0" as *const u8 as *const libc::c_char,
    );
    lua_pushstring(lua, b"breakpoint\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisBreakpointCommand
                as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"debug\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisDebugCommand as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"replicate_commands\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisReplicateCommandsCommand
                as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_setfield(
        lua,
        -(10002 as libc::c_int),
        b"redis\0" as *const u8 as *const libc::c_char,
    );
    let mut errh_func: *mut libc::c_char = b"local dbg = debug\ndebug = nil\nfunction __redis__err__handler(err)\n  local i = dbg.getinfo(2,'nSl')\n  if i and i.what == 'C' then\n    i = dbg.getinfo(3,'nSl')\n  end\n  if type(err) ~= 'table' then\n    err = {err='ERR ' .. tostring(err)}  end  if i then\n    err['source'] = i.source\n    err['line'] = i.currentline\n  end  return err\nend\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    luaL_loadbuffer(
        lua,
        errh_func,
        strlen(errh_func),
        b"@err_handler_def\0" as *const u8 as *const libc::c_char,
    );
    lua_pcall(lua, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    if (lctx.lua_client).is_null() {
        lctx.lua_client = createClient(0 as *mut connection);
        (*lctx.lua_client).flags
            |= ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong;
        (*lctx.lua_client)
            .flags = ((*lctx.lua_client).flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 41 as libc::c_int) as uint64_t;
    }
    lua_pushvalue(lua, -(10002 as libc::c_int));
    luaSetErrorMetatable(lua);
    luaSetTableProtectionRecursively(lua);
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    lctx.lua = lua;
}
#[no_mangle]
pub unsafe extern "C" fn scriptingRelease(mut async_0: libc::c_int) {
    if async_0 != 0 {
        freeLuaScriptsAsync(lctx.lua_scripts);
    } else {
        dictRelease(lctx.lua_scripts);
    }
    lctx.lua_scripts_mem = 0 as libc::c_int as libc::c_ulonglong;
    lua_close(lctx.lua);
}
#[no_mangle]
pub unsafe extern "C" fn scriptingReset(mut async_0: libc::c_int) {
    scriptingRelease(async_0);
    scriptingInit(0 as libc::c_int);
}
unsafe extern "C" fn evalCalcFunctionName(
    mut evalsha: libc::c_int,
    mut script: sds,
    mut out_funcname: *mut libc::c_char,
) {
    *out_funcname.offset(0 as libc::c_int as isize) = 'f' as i32 as libc::c_char;
    *out_funcname.offset(1 as libc::c_int as isize) = '_' as i32 as libc::c_char;
    if evalsha == 0 {
        sha1hex(out_funcname.offset(2 as libc::c_int as isize), script, sdslen(script));
    } else {
        let mut j: libc::c_int = 0;
        let mut sha: *mut libc::c_char = script;
        j = 0 as libc::c_int;
        while j < 40 as libc::c_int {
            *out_funcname
                .offset(
                    (j + 2 as libc::c_int) as isize,
                ) = (if *sha.offset(j as isize) as libc::c_int >= 'A' as i32
                && *sha.offset(j as isize) as libc::c_int <= 'Z' as i32
            {
                *sha.offset(j as isize) as libc::c_int + ('a' as i32 - 'A' as i32)
            } else {
                *sha.offset(j as isize) as libc::c_int
            }) as libc::c_char;
            j += 1;
        }
        *out_funcname.offset(42 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn evalExtractShebangFlags(
    mut body: sds,
    mut out_flags: *mut uint64_t,
    mut out_shebang_len: *mut ssize_t,
    mut err: *mut sds,
) -> libc::c_int {
    let mut shebang_len: ssize_t = 0 as libc::c_int as ssize_t;
    let mut script_flags: uint64_t = ((1 as libc::c_ulonglong) << 4 as libc::c_int)
        as uint64_t;
    if strncmp(
        body as *const libc::c_char,
        b"#!\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        let mut numparts: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut shebang_end: *mut libc::c_char = strchr(
            body as *const libc::c_char,
            '\n' as i32,
        );
        if shebang_end.is_null() {
            if !err.is_null() {
                *err = sdsnew(
                    b"Invalid script shebang\0" as *const u8 as *const libc::c_char,
                );
            }
            return -(1 as libc::c_int);
        }
        shebang_len = shebang_end.offset_from(body) as libc::c_long;
        let mut shebang: sds = sdsnewlen(
            body as *const libc::c_void,
            shebang_len as size_t,
        );
        let mut parts: *mut sds = sdssplitargs(
            shebang as *const libc::c_char,
            &mut numparts,
        );
        sdsfree(shebang);
        if parts.is_null() || numparts == 0 as libc::c_int {
            if !err.is_null() {
                *err = sdsnew(
                    b"Invalid engine in script shebang\0" as *const u8
                        as *const libc::c_char,
                );
            }
            sdsfreesplitres(parts, numparts);
            return -(1 as libc::c_int);
        }
        if strcmp(
            *parts.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"#!lua\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            if !err.is_null() {
                *err = sdscatfmt(
                    sdsempty(),
                    b"Unexpected engine in script shebang: %s\0" as *const u8
                        as *const libc::c_char,
                    *parts.offset(0 as libc::c_int as isize),
                );
            }
            sdsfreesplitres(parts, numparts);
            return -(1 as libc::c_int);
        }
        script_flags = (script_flags as libc::c_ulonglong
            & !((1 as libc::c_ulonglong) << 4 as libc::c_int)) as uint64_t;
        j = 1 as libc::c_int;
        while j < numparts {
            if strncmp(
                *parts.offset(j as isize) as *const libc::c_char,
                b"flags=\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                sdsrange(
                    *parts.offset(j as isize),
                    6 as libc::c_int as ssize_t,
                    -(1 as libc::c_int) as ssize_t,
                );
                let mut numflags: libc::c_int = 0;
                let mut jj: libc::c_int = 0;
                let mut flags: *mut sds = sdssplitlen(
                    *parts.offset(j as isize) as *const libc::c_char,
                    sdslen(*parts.offset(j as isize)) as ssize_t,
                    b",\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                    &mut numflags,
                );
                jj = 0 as libc::c_int;
                while jj < numflags {
                    let mut sf: *mut scriptFlag = 0 as *mut scriptFlag;
                    sf = scripts_flags_def.as_mut_ptr();
                    while (*sf).flag != 0 {
                        if strcmp(
                            *flags.offset(jj as isize) as *const libc::c_char,
                            (*sf).str_0,
                        ) == 0
                        {
                            break;
                        }
                        sf = sf.offset(1);
                    }
                    if (*sf).flag == 0 {
                        if !err.is_null() {
                            *err = sdscatfmt(
                                sdsempty(),
                                b"Unexpected flag in script shebang: %s\0" as *const u8
                                    as *const libc::c_char,
                                *flags.offset(jj as isize),
                            );
                        }
                        sdsfreesplitres(flags, numflags);
                        sdsfreesplitres(parts, numparts);
                        return -(1 as libc::c_int);
                    }
                    script_flags |= (*sf).flag;
                    jj += 1;
                }
                sdsfreesplitres(flags, numflags);
            } else {
                if !err.is_null() {
                    *err = sdscatfmt(
                        sdsempty(),
                        b"Unknown lua shebang option: %s\0" as *const u8
                            as *const libc::c_char,
                        *parts.offset(j as isize),
                    );
                }
                sdsfreesplitres(parts, numparts);
                return -(1 as libc::c_int);
            }
            j += 1;
        }
        sdsfreesplitres(parts, numparts);
    }
    if !out_shebang_len.is_null() {
        *out_shebang_len = shebang_len;
    }
    *out_flags = script_flags;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn evalGetCommandFlags(
    mut c: *mut client,
    mut cmd_flags: uint64_t,
) -> uint64_t {
    let mut funcname: [libc::c_char; 43] = [0; 43];
    let mut evalsha: libc::c_int = ((*(*c).cmd).proc_0
        == Some(evalShaCommand as unsafe extern "C" fn(*mut client) -> ())
        || (*(*c).cmd).proc_0
            == Some(evalShaRoCommand as unsafe extern "C" fn(*mut client) -> ()))
        as libc::c_int;
    if evalsha != 0
        && sdslen((**((*c).argv).offset(1 as libc::c_int as isize)).ptr as sds)
            != 40 as libc::c_int as libc::c_ulong
    {
        return cmd_flags;
    }
    let mut script_flags: uint64_t = 0;
    evalCalcFunctionName(
        evalsha,
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as sds,
        funcname.as_mut_ptr(),
    );
    let mut lua_cur_script: *mut libc::c_char = funcname
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    (*c).cur_script = dictFind(lctx.lua_scripts, lua_cur_script as *const libc::c_void);
    if ((*c).cur_script).is_null() {
        if evalsha != 0 {
            return cmd_flags;
        }
        if evalExtractShebangFlags(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as sds,
            &mut script_flags,
            0 as *mut ssize_t,
            0 as *mut sds,
        ) == -(1 as libc::c_int)
        {
            return cmd_flags;
        }
    } else {
        let mut l: *mut luaScript = (*(*c).cur_script).v.val as *mut luaScript;
        script_flags = (*l).flags;
    }
    if script_flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 4 as libc::c_int
        != 0
    {
        return cmd_flags;
    }
    return scriptFlagsToCmdFlags(cmd_flags, script_flags);
}
#[no_mangle]
pub unsafe extern "C" fn luaCreateFunction(
    mut c: *mut client,
    mut body: *mut robj,
) -> sds {
    let mut funcname: [libc::c_char; 43] = [0; 43];
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut script_flags: uint64_t = 0;
    funcname[0 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    funcname[1 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    sha1hex(
        funcname.as_mut_ptr().offset(2 as libc::c_int as isize),
        (*body).ptr as *mut libc::c_char,
        sdslen((*body).ptr as sds),
    );
    de = dictFind(
        lctx.lua_scripts,
        funcname.as_mut_ptr().offset(2 as libc::c_int as isize) as *const libc::c_void,
    );
    if !de.is_null() {
        return (*de).key as sds;
    }
    let mut shebang_len: ssize_t = 0 as libc::c_int as ssize_t;
    let mut err: sds = 0 as sds;
    if evalExtractShebangFlags(
        (*body).ptr as sds,
        &mut script_flags,
        &mut shebang_len,
        &mut err,
    ) == -(1 as libc::c_int)
    {
        addReplyErrorSds(c, err);
        return 0 as sds;
    }
    if luaL_loadbuffer(
        lctx.lua,
        ((*body).ptr as *mut libc::c_char).offset(shebang_len as isize),
        (sdslen((*body).ptr as sds)).wrapping_sub(shebang_len as libc::c_ulong),
        b"@user_script\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        if !c.is_null() {
            addReplyErrorFormat(
                c,
                b"Error compiling script (new function): %s\0" as *const u8
                    as *const libc::c_char,
                lua_tolstring(lctx.lua, -(1 as libc::c_int), 0 as *mut size_t),
            );
        }
        lua_settop(lctx.lua, -(1 as libc::c_int) - 1 as libc::c_int);
        return 0 as sds;
    }
    if lua_type(lctx.lua, -(1 as libc::c_int)) == 6 as libc::c_int {} else {
        _serverAssert(
            b"lua_isfunction(lctx.lua, -1)\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            455 as libc::c_int,
        );
        unreachable!();
    };
    lua_setfield(lctx.lua, -(10000 as libc::c_int), funcname.as_mut_ptr());
    let mut l: *mut luaScript = zcalloc(
        core::mem::size_of::<luaScript>() as libc::c_ulong,
    ) as *mut luaScript;
    (*l).body = body;
    (*l).flags = script_flags;
    let mut sha: sds = sdsnewlen(
        funcname.as_mut_ptr().offset(2 as libc::c_int as isize) as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    let mut retval: libc::c_int = dictAdd(
        lctx.lua_scripts,
        sha as *mut libc::c_void,
        l as *mut libc::c_void,
    );
    if retval == 0 as libc::c_int {} else {
        _serverAssertWithInfo(
            (if !c.is_null() { c } else { lctx.lua_client }),
            0 as *const robj,
            b"retval == DICT_OK\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            467 as libc::c_int,
        );
        unreachable!();
    };
    lctx
        .lua_scripts_mem = (lctx.lua_scripts_mem)
        .wrapping_add(
            (sdsZmallocSize(sha)).wrapping_add(getStringObjectSdsUsedMemory(body))
                as libc::c_ulonglong,
        );
    incrRefCount(body);
    return sha;
}
#[no_mangle]
pub unsafe extern "C" fn prepareLuaClient() {
    selectDb(lctx.lua_client, (*(*server.script_caller).db).id);
    (*lctx.lua_client).resp = 2 as libc::c_int;
    if (*server.script_caller).flags
        & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
    {
        (*lctx.lua_client).flags
            |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn resetLuaClient() {
    (*lctx.lua_client).flags
        &= !((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn evalGenericCommand(
    mut c: *mut client,
    mut evalsha: libc::c_int,
) {
    let mut lua: *mut lua_State = lctx.lua;
    let mut funcname: [libc::c_char; 43] = [0; 43];
    let mut numkeys: libc::c_longlong = 0;
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut numkeys,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if numkeys > ((*c).argc - 3 as libc::c_int) as libc::c_longlong {
        addReplyError(
            c,
            b"Number of keys can't be greater than number of args\0" as *const u8
                as *const libc::c_char,
        );
        return;
    } else {
        if numkeys < 0 as libc::c_int as libc::c_longlong {
            addReplyError(
                c,
                b"Number of keys can't be negative\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    if !((*c).cur_script).is_null() {
        funcname[0 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
        funcname[1 as libc::c_int as usize] = '_' as i32 as libc::c_char;
        memcpy(
            funcname.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut libc::c_void,
            (*(*c).cur_script).key,
            40 as libc::c_int as libc::c_ulong,
        );
        funcname[42 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        evalCalcFunctionName(
            evalsha,
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as sds,
            funcname.as_mut_ptr(),
        );
    }
    lua_getfield(
        lua,
        -(10002 as libc::c_int),
        b"__redis__err__handler\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(lua, -(10000 as libc::c_int), funcname.as_mut_ptr());
    if lua_type(lua, -(1 as libc::c_int)) == 0 as libc::c_int {
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
        if evalsha != 0 {
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            addReplyErrorObject(c, shared.noscripterr);
            return;
        }
        if (luaCreateFunction(c, *((*c).argv).offset(1 as libc::c_int as isize)))
            .is_null()
        {
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            return;
        }
        lua_getfield(lua, -(10000 as libc::c_int), funcname.as_mut_ptr());
        if !(lua_type(lua, -(1 as libc::c_int)) == 0 as libc::c_int) {} else {
            _serverAssert(
                b"!lua_isnil(lua,-1)\0" as *const u8 as *const libc::c_char,
                b"eval.c\0" as *const u8 as *const libc::c_char,
                535 as libc::c_int,
            );
            unreachable!();
        };
    }
    let mut lua_cur_script: *mut libc::c_char = funcname
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut de: *mut dictEntry = (*c).cur_script;
    if de.is_null() {
        de = dictFind(lctx.lua_scripts, lua_cur_script as *const libc::c_void);
    }
    let mut l: *mut luaScript = (*de).v.val as *mut luaScript;
    let mut ro: libc::c_int = ((*(*c).cmd).proc_0
        == Some(evalRoCommand as unsafe extern "C" fn(*mut client) -> ())
        || (*(*c).cmd).proc_0
            == Some(evalShaRoCommand as unsafe extern "C" fn(*mut client) -> ()))
        as libc::c_int;
    let mut rctx: scriptRunCtx = scriptRunCtx {
        funcname: 0 as *const libc::c_char,
        c: 0 as *mut client,
        original_client: 0 as *mut client,
        flags: 0,
        repl_flags: 0,
        start_time: 0,
        snapshot_time: 0,
    };
    if scriptPrepareForRun(&mut rctx, lctx.lua_client, c, lua_cur_script, (*l).flags, ro)
        != 0 as libc::c_int
    {
        lua_settop(lua, -(2 as libc::c_int) - 1 as libc::c_int);
        return;
    }
    rctx
        .flags = (rctx.flags as libc::c_ulonglong
        | (1 as libc::c_ulonglong) << 7 as libc::c_int) as libc::c_int;
    luaCallFunction(
        &mut rctx,
        lua,
        ((*c).argv).offset(3 as libc::c_int as isize),
        numkeys as size_t,
        ((*c).argv).offset(3 as libc::c_int as isize).offset(numkeys as isize),
        (((*c).argc - 3 as libc::c_int) as libc::c_longlong - numkeys) as size_t,
        ldb.active,
    );
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    scriptResetRun(&mut rctx);
}
#[no_mangle]
pub unsafe extern "C" fn evalCommand(mut c: *mut client) {
    replicationFeedMonitors(c, server.monitors, (*(*c).db).id, (*c).argv, (*c).argc);
    if (*c).flags & ((1 as libc::c_int) << 25 as libc::c_int) as libc::c_ulong == 0 {
        evalGenericCommand(c, 0 as libc::c_int);
    } else {
        evalGenericCommandWithDebugging(c, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn evalRoCommand(mut c: *mut client) {
    evalCommand(c);
}
#[no_mangle]
pub unsafe extern "C" fn evalShaCommand(mut c: *mut client) {
    replicationFeedMonitors(c, server.monitors, (*(*c).db).id, (*c).argv, (*c).argc);
    if sdslen((**((*c).argv).offset(1 as libc::c_int as isize)).ptr as sds)
        != 40 as libc::c_int as libc::c_ulong
    {
        addReplyErrorObject(c, shared.noscripterr);
        return;
    }
    if (*c).flags & ((1 as libc::c_int) << 25 as libc::c_int) as libc::c_ulong == 0 {
        evalGenericCommand(c, 1 as libc::c_int);
    } else {
        addReplyError(
            c,
            b"Please use EVAL instead of EVALSHA for debugging\0" as *const u8
                as *const libc::c_char,
        );
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn evalShaRoCommand(mut c: *mut client) {
    evalShaCommand(c);
}
#[no_mangle]
pub unsafe extern "C" fn scriptCommand(mut c: *mut client) {
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut help: [*const libc::c_char; 15] = [
            b"DEBUG (YES|SYNC|NO)\0" as *const u8 as *const libc::c_char,
            b"    Set the debug mode for subsequent scripts executed.\0" as *const u8
                as *const libc::c_char,
            b"EXISTS <sha1> [<sha1> ...]\0" as *const u8 as *const libc::c_char,
            b"    Return information about the existence of the scripts in the script cache.\0"
                as *const u8 as *const libc::c_char,
            b"FLUSH [ASYNC|SYNC]\0" as *const u8 as *const libc::c_char,
            b"    Flush the Lua scripts cache. Very dangerous on replicas.\0"
                as *const u8 as *const libc::c_char,
            b"    When called without the optional mode argument, the behavior is determined by the\0"
                as *const u8 as *const libc::c_char,
            b"    lazyfree-lazy-user-flush configuration directive. Valid modes are:\0"
                as *const u8 as *const libc::c_char,
            b"    * ASYNC: Asynchronously flush the scripts cache.\0" as *const u8
                as *const libc::c_char,
            b"    * SYNC: Synchronously flush the scripts cache.\0" as *const u8
                as *const libc::c_char,
            b"KILL\0" as *const u8 as *const libc::c_char,
            b"    Kill the currently executing Lua script.\0" as *const u8
                as *const libc::c_char,
            b"LOAD <script>\0" as *const u8 as *const libc::c_char,
            b"    Load a script into the scripts cache without executing it.\0"
                as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if (*c).argc >= 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"flush\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut async_0: libc::c_int = 0 as libc::c_int;
        if (*c).argc == 3 as libc::c_int
            && strcasecmp(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"sync\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            async_0 = 0 as libc::c_int;
        } else if (*c).argc == 3 as libc::c_int
            && strcasecmp(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"async\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            async_0 = 1 as libc::c_int;
        } else if (*c).argc == 2 as libc::c_int {
            async_0 = if server.lazyfree_lazy_user_flush != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        } else {
            addReplyError(
                c,
                b"SCRIPT FLUSH only support SYNC|ASYNC option\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        scriptingReset(async_0);
        addReply(c, shared.ok);
    } else if (*c).argc >= 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"exists\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut j: libc::c_int = 0;
        addReplyArrayLen(c, ((*c).argc - 2 as libc::c_int) as libc::c_long);
        j = 2 as libc::c_int;
        while j < (*c).argc {
            if !(dictFind(lctx.lua_scripts, (**((*c).argv).offset(j as isize)).ptr))
                .is_null()
            {
                addReply(c, shared.cone);
            } else {
                addReply(c, shared.czero);
            }
            j += 1;
        }
    } else if (*c).argc == 3 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"load\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut sha: sds = luaCreateFunction(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
        );
        if sha.is_null() {
            return;
        }
        addReplyBulkCBuffer(c, sha as *const libc::c_void, 40 as libc::c_int as size_t);
    } else if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"kill\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        scriptKill(c, 1 as libc::c_int);
    } else if (*c).argc == 3 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"debug\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        if clientHasPendingReplies(c) != 0 {
            addReplyError(
                c,
                b"SCRIPT DEBUG must be called outside a pipeline\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"no\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ldbDisable(c);
            addReply(c, shared.ok);
        } else if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"yes\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ldbEnable(c);
            addReply(c, shared.ok);
        } else if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"sync\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ldbEnable(c);
            addReply(c, shared.ok);
            (*c).flags |= ((1 as libc::c_int) << 26 as libc::c_int) as libc::c_ulong;
        } else {
            addReplyError(
                c,
                b"Use SCRIPT DEBUG YES/SYNC/NO\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    } else {
        addReplySubcommandSyntaxError(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn evalMemory() -> libc::c_ulong {
    return luaMemory(lctx.lua);
}
#[no_mangle]
pub unsafe extern "C" fn evalScriptsDict() -> *mut dict {
    return lctx.lua_scripts;
}
#[no_mangle]
pub unsafe extern "C" fn evalScriptsMemory() -> libc::c_ulong {
    return (lctx.lua_scripts_mem)
        .wrapping_add(
            ((*lctx.lua_scripts).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*lctx.lua_scripts).ht_used[1 as libc::c_int as usize])
                .wrapping_mul(
                    (core::mem::size_of::<dictEntry>() as libc::c_ulong)
                        .wrapping_add(
                            core::mem::size_of::<luaScript>() as libc::c_ulong,
                        ),
                ) as libc::c_ulonglong,
        )
        .wrapping_add(
            (if (*lctx.lua_scripts).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*lctx.lua_scripts).ht_size_exp[0 as libc::c_int as usize]
                        as libc::c_int
            })
                .wrapping_add(
                    (if (*lctx.lua_scripts).ht_size_exp[1 as libc::c_int as usize]
                        as libc::c_int == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*lctx.lua_scripts).ht_size_exp[1 as libc::c_int as usize]
                                as libc::c_int
                    }),
                )
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong)
                as libc::c_ulonglong,
        ) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ldbInit() {
    ldb.conn = 0 as *mut connection;
    ldb.active = 0 as libc::c_int;
    ldb.logs = listCreate();
    (*ldb.logs)
        .free = core::mem::transmute::<
        Option::<unsafe extern "C" fn(sds) -> ()>,
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    >(Some(sdsfree as unsafe extern "C" fn(sds) -> ()));
    ldb.children = listCreate();
    ldb.src = 0 as *mut sds;
    ldb.lines = 0 as libc::c_int;
    ldb.cbuf = sdsempty();
}
#[no_mangle]
pub unsafe extern "C" fn ldbFlushLog(mut log: *mut list) {
    let mut ln: *mut listNode = 0 as *mut listNode;
    loop {
        ln = (*log).head;
        if ln.is_null() {
            break;
        }
        listDelNode(log, ln);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ldbIsEnabled() -> libc::c_int {
    return (ldb.active != 0 && ldb.step != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbEnable(mut c: *mut client) {
    (*c).flags |= ((1 as libc::c_int) << 25 as libc::c_int) as libc::c_ulong;
    ldbFlushLog(ldb.logs);
    ldb.conn = (*c).conn;
    ldb.step = 1 as libc::c_int;
    ldb.bpcount = 0 as libc::c_int;
    ldb.luabp = 0 as libc::c_int;
    sdsfree(ldb.cbuf);
    ldb.cbuf = sdsempty();
    ldb.maxlen = 256 as libc::c_int as size_t;
    ldb.maxlen_hint_sent = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbDisable(mut c: *mut client) {
    (*c).flags
        &= !((1 as libc::c_int) << 25 as libc::c_int
            | (1 as libc::c_int) << 26 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ldbLog(mut entry: sds) {
    listAddNodeTail(ldb.logs, entry as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ldbLogWithMaxLen(mut entry: sds) {
    let mut trimmed: libc::c_int = 0 as libc::c_int;
    if ldb.maxlen != 0 && sdslen(entry) > ldb.maxlen {
        sdsrange(
            entry,
            0 as libc::c_int as ssize_t,
            (ldb.maxlen).wrapping_sub(1 as libc::c_int as libc::c_ulong) as ssize_t,
        );
        entry = sdscatlen(
            entry,
            b" ...\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        trimmed = 1 as libc::c_int;
    }
    ldbLog(entry);
    if trimmed != 0 && ldb.maxlen_hint_sent == 0 as libc::c_int {
        ldb.maxlen_hint_sent = 1 as libc::c_int;
        ldbLog(
            sdsnew(
                b"<hint> The above reply was trimmed. Use 'maxlen 0' to disable trimming.\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldbSendLogs() {
    let mut proto: sds = sdsempty();
    proto = sdscatfmt(
        proto,
        b"*%i\r\n\0" as *const u8 as *const libc::c_char,
        (*ldb.logs).len as libc::c_int,
    );
    while (*ldb.logs).len != 0 {
        let mut ln: *mut listNode = (*ldb.logs).head;
        proto = sdscatlen(
            proto,
            b"+\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        sdsmapchars(
            (*ln).value as sds,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            b"  \0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        proto = sdscatsds(proto, (*ln).value as sds);
        proto = sdscatlen(
            proto,
            b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        listDelNode(ldb.logs, ln);
    }
    connWrite(ldb.conn, proto as *const libc::c_void, sdslen(proto))
        == -(1 as libc::c_int);
    sdsfree(proto);
}
#[no_mangle]
pub unsafe extern "C" fn ldbStartSession(mut c: *mut client) -> libc::c_int {
    ldb
        .forked = ((*c).flags
        & ((1 as libc::c_int) << 26 as libc::c_int) as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    if ldb.forked != 0 {
        let mut cp: pid_t = redisFork(3 as libc::c_int);
        if cp == -(1 as libc::c_int) {
            addReplyErrorFormat(
                c,
                b"Fork() failed: can't run EVAL in debugging mode: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        } else {
            if cp == 0 as libc::c_int {
                let mut act: sigaction = sigaction {
                    __sigaction_handler: C2RustUnnamed_9 {
                        sa_handler: None,
                    },
                    sa_mask: __sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    sa_restorer: None,
                };
                sigemptyset(&mut act.sa_mask);
                act.sa_flags = 0 as libc::c_int;
                act
                    .__sigaction_handler
                    .sa_handler = core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t);
                sigaction(15 as libc::c_int, &mut act, 0 as *mut sigaction);
                sigaction(2 as libc::c_int, &mut act, 0 as *mut sigaction);
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Redis forked for debugging eval\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                listAddNodeTail(ldb.children, cp as libc::c_ulong as *mut libc::c_void);
                freeClientAsync(c);
                return 0 as libc::c_int;
            }
        }
    } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Redis synchronous debugging eval session started\0" as *const u8
                as *const libc::c_char,
        );
    }
    connBlock(ldb.conn);
    connSendTimeout(ldb.conn, 5000 as libc::c_int as libc::c_longlong);
    ldb.active = 1 as libc::c_int;
    let mut srcstring: sds = sdsdup(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as sds,
    );
    let mut srclen: size_t = sdslen(srcstring);
    while srclen != 0
        && (*srcstring
            .offset(srclen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\n' as i32
            || *srcstring
                .offset(srclen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\r' as i32)
    {
        srclen = srclen.wrapping_sub(1);
        *srcstring.offset(srclen as isize) = '\0' as i32 as libc::c_char;
    }
    sdssetlen(srcstring, srclen);
    ldb
        .src = sdssplitlen(
        srcstring as *const libc::c_char,
        sdslen(srcstring) as ssize_t,
        b"\n\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        &mut ldb.lines,
    );
    sdsfree(srcstring);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbEndSession(mut c: *mut client) {
    ldbLog(sdsnew(b"<endsession>\0" as *const u8 as *const libc::c_char));
    ldbSendLogs();
    if ldb.forked != 0 {
        writeToClient(c, 0 as libc::c_int);
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Lua debugging session child exiting\0" as *const u8
                    as *const libc::c_char,
            );
        }
        exitFromChild(0 as libc::c_int);
    } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Redis synchronous debugging eval session ended\0" as *const u8
                as *const libc::c_char,
        );
    }
    connNonBlock(ldb.conn);
    connSendTimeout(ldb.conn, 0 as libc::c_int as libc::c_longlong);
    (*c).flags |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong;
    sdsfreesplitres(ldb.src, ldb.lines);
    ldb.lines = 0 as libc::c_int;
    ldb.active = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbRemoveChild(mut pid: pid_t) -> libc::c_int {
    let mut ln: *mut listNode = listSearchKey(
        ldb.children,
        pid as libc::c_ulong as *mut libc::c_void,
    );
    if !ln.is_null() {
        listDelNode(ldb.children, ln);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbPendingChildren() -> libc::c_int {
    return (*ldb.children).len as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbKillForkedSessions() {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(ldb.children, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut pid: pid_t = (*ln).value as libc::c_ulong as pid_t;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Killing debugging session %ld\0" as *const u8 as *const libc::c_char,
                pid as libc::c_long,
            );
        }
        kill(pid, 9 as libc::c_int);
    }
    listRelease(ldb.children);
    ldb.children = listCreate();
}
#[no_mangle]
pub unsafe extern "C" fn evalGenericCommandWithDebugging(
    mut c: *mut client,
    mut evalsha: libc::c_int,
) {
    if ldbStartSession(c) != 0 {
        evalGenericCommand(c, evalsha);
        ldbEndSession(c);
    } else {
        ldbDisable(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ldbGetSourceLine(mut line: libc::c_int) -> *mut libc::c_char {
    let mut idx: libc::c_int = line - 1 as libc::c_int;
    if idx < 0 as libc::c_int || idx >= ldb.lines {
        return b"<out of range source code line>\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    return *(ldb.src).offset(idx as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ldbIsBreakpoint(mut line: libc::c_int) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < ldb.bpcount {
        if ldb.bp[j as usize] == line {
            return 1 as libc::c_int;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbAddBreakpoint(mut line: libc::c_int) -> libc::c_int {
    if line <= 0 as libc::c_int || line > ldb.lines {
        return 0 as libc::c_int;
    }
    if ldbIsBreakpoint(line) == 0 && ldb.bpcount != 64 as libc::c_int {
        let fresh1 = ldb.bpcount;
        ldb.bpcount = ldb.bpcount + 1;
        ldb.bp[fresh1 as usize] = line;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbDelBreakpoint(mut line: libc::c_int) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < ldb.bpcount {
        if ldb.bp[j as usize] == line {
            ldb.bpcount -= 1;
            memmove(
                (ldb.bp).as_mut_ptr().offset(j as isize) as *mut libc::c_void,
                (ldb.bp)
                    .as_mut_ptr()
                    .offset(j as isize)
                    .offset(1 as libc::c_int as isize) as *const libc::c_void,
                (ldb.bpcount - j) as libc::c_ulong,
            );
            return 1 as libc::c_int;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldbReplParseCommand(
    mut argcp: *mut libc::c_int,
    mut err: *mut *mut libc::c_char,
) -> *mut sds {
    let mut plen: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    static mut protocol_error: *mut libc::c_char = b"protocol error\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut argv: *mut sds = 0 as *mut sds;
    let mut argc: libc::c_int = 0 as libc::c_int;
    if sdslen(ldb.cbuf) == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut sds;
    }
    let mut copy: sds = sdsdup(ldb.cbuf);
    let mut p: *mut libc::c_char = copy;
    p = strchr(p, '*' as i32);
    if p.is_null() {
        current_block = 2611887017218276167;
    } else {
        plen = p.offset(1 as libc::c_int as isize);
        p = strstr(p, b"\r\n\0" as *const u8 as *const libc::c_char);
        if p.is_null() {
            current_block = 979901658470254302;
        } else {
            *p = '\0' as i32 as libc::c_char;
            p = p.offset(2 as libc::c_int as isize);
            *argcp = atoi(plen);
            if *argcp <= 0 as libc::c_int || *argcp > 1024 as libc::c_int {
                current_block = 2611887017218276167;
            } else {
                argv = zmalloc(
                    (core::mem::size_of::<sds>() as libc::c_ulong)
                        .wrapping_mul(*argcp as libc::c_ulong),
                ) as *mut sds;
                argc = 0 as libc::c_int;
                loop {
                    if !(argc < *argcp) {
                        current_block = 14401909646449704462;
                        break;
                    }
                    if *p as libc::c_int == '\0' as i32 {
                        current_block = 979901658470254302;
                        break;
                    }
                    if *p as libc::c_int != '$' as i32 {
                        current_block = 2611887017218276167;
                        break;
                    }
                    plen = p.offset(1 as libc::c_int as isize);
                    p = strstr(p, b"\r\n\0" as *const u8 as *const libc::c_char);
                    if p.is_null() {
                        current_block = 979901658470254302;
                        break;
                    }
                    *p = '\0' as i32 as libc::c_char;
                    p = p.offset(2 as libc::c_int as isize);
                    let mut slen: libc::c_int = atoi(plen);
                    if slen <= 0 as libc::c_int || slen > 1024 as libc::c_int {
                        current_block = 2611887017218276167;
                        break;
                    }
                    if p
                        .offset(slen as isize)
                        .offset(2 as libc::c_int as isize)
                        .offset_from(copy) as libc::c_long as size_t > sdslen(copy)
                    {
                        current_block = 979901658470254302;
                        break;
                    }
                    let fresh2 = argc;
                    argc = argc + 1;
                    let ref mut fresh3 = *argv.offset(fresh2 as isize);
                    *fresh3 = sdsnewlen(p as *const libc::c_void, slen as size_t);
                    p = p.offset(slen as isize);
                    if *p.offset(0 as libc::c_int as isize) as libc::c_int != '\r' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            != '\n' as i32
                    {
                        current_block = 2611887017218276167;
                        break;
                    }
                    p = p.offset(2 as libc::c_int as isize);
                }
                match current_block {
                    2611887017218276167 => {}
                    979901658470254302 => {}
                    _ => {
                        sdsfree(copy);
                        return argv;
                    }
                }
            }
        }
    }
    match current_block {
        2611887017218276167 => {
            *err = protocol_error;
        }
        _ => {}
    }
    sdsfreesplitres(argv, argc);
    sdsfree(copy);
    return 0 as *mut sds;
}
#[no_mangle]
pub unsafe extern "C" fn ldbLogSourceLine(mut lnum: libc::c_int) {
    let mut line: *mut libc::c_char = ldbGetSourceLine(lnum);
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp: libc::c_int = ldbIsBreakpoint(lnum);
    let mut current: libc::c_int = (ldb.currentline == lnum) as libc::c_int;
    if current != 0 && bp != 0 {
        prefix = b"->#\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if current != 0 {
        prefix = b"-> \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if bp != 0 {
        prefix = b"  #\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        prefix = b"   \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    let mut thisline: sds = sdscatprintf(
        sdsempty(),
        b"%s%-3d %s\0" as *const u8 as *const libc::c_char,
        prefix,
        lnum,
        line,
    );
    ldbLog(thisline);
}
#[no_mangle]
pub unsafe extern "C" fn ldbList(mut around: libc::c_int, mut context: libc::c_int) {
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j <= ldb.lines {
        if !(around != 0 as libc::c_int && abs(around - j) > context) {
            ldbLogSourceLine(j);
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldbCatStackValueRec(
    mut s: sds,
    mut lua: *mut lua_State,
    mut idx: libc::c_int,
    mut level: libc::c_int,
) -> sds {
    let mut t: libc::c_int = lua_type(lua, idx);
    let fresh4 = level;
    level = level + 1;
    if fresh4 == 20 as libc::c_int / 2 as libc::c_int {
        return sdscat(
            s,
            b"<max recursion level reached! Nested table?>\0" as *const u8
                as *const libc::c_char,
        );
    }
    match t {
        4 => {
            let mut strl: size_t = 0;
            let mut strp: *mut libc::c_char = lua_tolstring(lua, idx, &mut strl)
                as *mut libc::c_char;
            s = sdscatrepr(s, strp, strl);
        }
        1 => {
            s = sdscat(
                s,
                if lua_toboolean(lua, idx) != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        3 => {
            s = sdscatprintf(
                s,
                b"%g\0" as *const u8 as *const libc::c_char,
                lua_tonumber(lua, idx),
            );
        }
        0 => {
            s = sdscatlen(
                s,
                b"nil\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as size_t,
            );
        }
        5 => {
            let mut expected_index: libc::c_int = 1 as libc::c_int;
            let mut is_array: libc::c_int = 1 as libc::c_int;
            let mut repr1: sds = sdsempty();
            let mut repr2: sds = sdsempty();
            lua_pushnil(lua);
            while lua_next(lua, idx - 1 as libc::c_int) != 0 {
                if is_array != 0
                    && (lua_type(lua, -(2 as libc::c_int)) != 3 as libc::c_int
                        || lua_tonumber(lua, -(2 as libc::c_int))
                            != expected_index as libc::c_double)
                {
                    is_array = 0 as libc::c_int;
                }
                repr1 = ldbCatStackValueRec(repr1, lua, -(1 as libc::c_int), level);
                repr1 = sdscatlen(
                    repr1,
                    b"; \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
                repr2 = sdscatlen(
                    repr2,
                    b"[\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
                repr2 = ldbCatStackValueRec(repr2, lua, -(2 as libc::c_int), level);
                repr2 = sdscatlen(
                    repr2,
                    b"]=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
                repr2 = ldbCatStackValueRec(repr2, lua, -(1 as libc::c_int), level);
                repr2 = sdscatlen(
                    repr2,
                    b"; \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
                lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
                expected_index += 1;
            }
            if sdslen(repr1) != 0 {
                sdsrange(
                    repr1,
                    0 as libc::c_int as ssize_t,
                    -(3 as libc::c_int) as ssize_t,
                );
            }
            if sdslen(repr2) != 0 {
                sdsrange(
                    repr2,
                    0 as libc::c_int as ssize_t,
                    -(3 as libc::c_int) as ssize_t,
                );
            }
            s = sdscatlen(
                s,
                b"{\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            s = sdscatsds(s, if is_array != 0 { repr1 } else { repr2 });
            s = sdscatlen(
                s,
                b"}\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            sdsfree(repr1);
            sdsfree(repr2);
        }
        6 | 7 | 8 | 2 => {
            let mut p: *const libc::c_void = lua_topointer(lua, idx);
            let mut typename: *mut libc::c_char = b"unknown\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
            if t == 6 as libc::c_int {
                typename = b"function\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else if t == 7 as libc::c_int {
                typename = b"userdata\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else if t == 8 as libc::c_int {
                typename = b"thread\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else if t == 2 as libc::c_int {
                typename = b"light-userdata\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            s = sdscatprintf(
                s,
                b"\"%s@%p\"\0" as *const u8 as *const libc::c_char,
                typename,
                p,
            );
        }
        _ => {
            s = sdscat(
                s,
                b"\"<unknown-lua-type>\"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn ldbCatStackValue(
    mut s: sds,
    mut lua: *mut lua_State,
    mut idx: libc::c_int,
) -> sds {
    return ldbCatStackValueRec(s, lua, idx, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ldbLogStackValue(
    mut lua: *mut lua_State,
    mut prefix: *mut libc::c_char,
) {
    let mut s: sds = sdsnew(prefix);
    s = ldbCatStackValue(s, lua, -(1 as libc::c_int));
    ldbLogWithMaxLen(s);
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = reply;
    match *p as libc::c_int {
        58 => {
            p = ldbRedisProtocolToHuman_Int(o, reply);
        }
        36 => {
            p = ldbRedisProtocolToHuman_Bulk(o, reply);
        }
        43 => {
            p = ldbRedisProtocolToHuman_Status(o, reply);
        }
        45 => {
            p = ldbRedisProtocolToHuman_Status(o, reply);
        }
        42 => {
            p = ldbRedisProtocolToHuman_MultiBulk(o, reply);
        }
        126 => {
            p = ldbRedisProtocolToHuman_Set(o, reply);
        }
        37 => {
            p = ldbRedisProtocolToHuman_Map(o, reply);
        }
        95 => {
            p = ldbRedisProtocolToHuman_Null(o, reply);
        }
        35 => {
            p = ldbRedisProtocolToHuman_Bool(o, reply);
        }
        44 => {
            p = ldbRedisProtocolToHuman_Double(o, reply);
        }
        _ => {}
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_Int(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    *o = sdscatlen(
        *o,
        reply.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (p.offset_from(reply) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    return p.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_Bulk(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut bulklen: libc::c_longlong = 0;
    string2ll(
        reply.offset(1 as libc::c_int as isize),
        (p.offset_from(reply) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut bulklen,
    );
    if bulklen == -(1 as libc::c_int) as libc::c_longlong {
        *o = sdscatlen(
            *o,
            b"NULL\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        return p.offset(2 as libc::c_int as isize);
    } else {
        *o = sdscatrepr(*o, p.offset(2 as libc::c_int as isize), bulklen as size_t);
        return p
            .offset(2 as libc::c_int as isize)
            .offset(bulklen as isize)
            .offset(2 as libc::c_int as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_Status(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    *o = sdscatrepr(*o, reply, p.offset_from(reply) as libc::c_long as size_t);
    return p.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_MultiBulk(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut mbulklen: libc::c_longlong = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    string2ll(
        reply.offset(1 as libc::c_int as isize),
        (p.offset_from(reply) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut mbulklen,
    );
    p = p.offset(2 as libc::c_int as isize);
    if mbulklen == -(1 as libc::c_int) as libc::c_longlong {
        *o = sdscatlen(
            *o,
            b"NULL\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        return p;
    }
    *o = sdscatlen(
        *o,
        b"[\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    j = 0 as libc::c_int;
    while (j as libc::c_longlong) < mbulklen {
        p = ldbRedisProtocolToHuman(o, p);
        if j as libc::c_longlong != mbulklen - 1 as libc::c_int as libc::c_longlong {
            *o = sdscatlen(
                *o,
                b",\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    *o = sdscatlen(
        *o,
        b"]\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_Set(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut mbulklen: libc::c_longlong = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    string2ll(
        reply.offset(1 as libc::c_int as isize),
        (p.offset_from(reply) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut mbulklen,
    );
    p = p.offset(2 as libc::c_int as isize);
    *o = sdscatlen(
        *o,
        b"~(\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    j = 0 as libc::c_int;
    while (j as libc::c_longlong) < mbulklen {
        p = ldbRedisProtocolToHuman(o, p);
        if j as libc::c_longlong != mbulklen - 1 as libc::c_int as libc::c_longlong {
            *o = sdscatlen(
                *o,
                b",\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    *o = sdscatlen(
        *o,
        b")\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_Map(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut mbulklen: libc::c_longlong = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    string2ll(
        reply.offset(1 as libc::c_int as isize),
        (p.offset_from(reply) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut mbulklen,
    );
    p = p.offset(2 as libc::c_int as isize);
    *o = sdscatlen(
        *o,
        b"{\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    j = 0 as libc::c_int;
    while (j as libc::c_longlong) < mbulklen {
        p = ldbRedisProtocolToHuman(o, p);
        *o = sdscatlen(
            *o,
            b" => \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        p = ldbRedisProtocolToHuman(o, p);
        if j as libc::c_longlong != mbulklen - 1 as libc::c_int as libc::c_longlong {
            *o = sdscatlen(
                *o,
                b",\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    *o = sdscatlen(
        *o,
        b"}\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_Null(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    *o = sdscatlen(
        *o,
        b"(null)\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as size_t,
    );
    return p.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_Bool(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    if *reply.offset(1 as libc::c_int as isize) as libc::c_int == 't' as i32 {
        *o = sdscatlen(
            *o,
            b"#true\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as size_t,
        );
    } else {
        *o = sdscatlen(
            *o,
            b"#false\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as size_t,
        );
    }
    return p.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedisProtocolToHuman_Double(
    mut o: *mut sds,
    mut reply: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strchr(
        reply.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    *o = sdscatlen(
        *o,
        b"(double) \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        9 as libc::c_int as size_t,
    );
    *o = sdscatlen(
        *o,
        reply.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (p.offset_from(reply) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
    );
    return p.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ldbLogRedisReply(mut reply: *mut libc::c_char) {
    let mut log: sds = sdsnew(b"<reply> \0" as *const u8 as *const libc::c_char);
    ldbRedisProtocolToHuman(&mut log, reply);
    ldbLogWithMaxLen(log);
}
#[no_mangle]
pub unsafe extern "C" fn ldbPrint(
    mut lua: *mut lua_State,
    mut varname: *mut libc::c_char,
) {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        nups: 0,
        linedefined: 0,
        lastlinedefined: 0,
        short_src: [0; 60],
        i_ci: 0,
    };
    let mut l: libc::c_int = 0 as libc::c_int;
    while lua_getstack(lua, l, &mut ar) != 0 as libc::c_int {
        l += 1;
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut i: libc::c_int = 1 as libc::c_int;
        loop {
            name = lua_getlocal(lua, &mut ar, i);
            if name.is_null() {
                break;
            }
            i += 1;
            if strcmp(varname, name) == 0 as libc::c_int {
                ldbLogStackValue(
                    lua,
                    b"<value> \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
                return;
            } else {
                lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            }
        }
    }
    if strcmp(varname, b"ARGV\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(varname, b"KEYS\0" as *const u8 as *const libc::c_char) == 0
    {
        lua_getfield(lua, -(10002 as libc::c_int), varname);
        ldbLogStackValue(
            lua,
            b"<value> \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    } else {
        ldbLog(sdsnew(b"No such variable.\0" as *const u8 as *const libc::c_char));
    };
}
#[no_mangle]
pub unsafe extern "C" fn ldbPrintAll(mut lua: *mut lua_State) {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        nups: 0,
        linedefined: 0,
        lastlinedefined: 0,
        short_src: [0; 60],
        i_ci: 0,
    };
    let mut vars: libc::c_int = 0 as libc::c_int;
    if lua_getstack(lua, 0 as libc::c_int, &mut ar) != 0 as libc::c_int {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut i: libc::c_int = 1 as libc::c_int;
        loop {
            name = lua_getlocal(lua, &mut ar, i);
            if name.is_null() {
                break;
            }
            i += 1;
            if (strstr(name, b"(*temporary)\0" as *const u8 as *const libc::c_char))
                .is_null()
            {
                let mut prefix: sds = sdscatprintf(
                    sdsempty(),
                    b"<value> %s = \0" as *const u8 as *const libc::c_char,
                    name,
                );
                ldbLogStackValue(lua, prefix);
                sdsfree(prefix);
                vars += 1;
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
        }
    }
    if vars == 0 as libc::c_int {
        ldbLog(
            sdsnew(
                b"No local variables in the current context.\0" as *const u8
                    as *const libc::c_char,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldbBreak(mut argv: *mut sds, mut argc: libc::c_int) {
    if argc == 1 as libc::c_int {
        if ldb.bpcount == 0 as libc::c_int {
            ldbLog(
                sdsnew(
                    b"No breakpoints set. Use 'b <line>' to add one.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            return;
        } else {
            ldbLog(
                sdscatfmt(
                    sdsempty(),
                    b"%i breakpoints set:\0" as *const u8 as *const libc::c_char,
                    ldb.bpcount,
                ),
            );
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < ldb.bpcount {
                ldbLogSourceLine(ldb.bp[j as usize]);
                j += 1;
            }
        }
    } else {
        let mut j_0: libc::c_int = 0;
        j_0 = 1 as libc::c_int;
        while j_0 < argc {
            let mut arg: *mut libc::c_char = *argv.offset(j_0 as isize);
            let mut line: libc::c_long = 0;
            if string2l(arg, sdslen(arg), &mut line) == 0 {
                ldbLog(
                    sdscatfmt(
                        sdsempty(),
                        b"Invalid argument:'%s'\0" as *const u8 as *const libc::c_char,
                        arg,
                    ),
                );
            } else if line == 0 as libc::c_int as libc::c_long {
                ldb.bpcount = 0 as libc::c_int;
                ldbLog(
                    sdsnew(
                        b"All breakpoints removed.\0" as *const u8 as *const libc::c_char,
                    ),
                );
            } else if line > 0 as libc::c_int as libc::c_long {
                if ldb.bpcount == 64 as libc::c_int {
                    ldbLog(
                        sdsnew(
                            b"Too many breakpoints set.\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                } else if ldbAddBreakpoint(line as libc::c_int) != 0 {
                    ldbList(line as libc::c_int, 1 as libc::c_int);
                } else {
                    ldbLog(
                        sdsnew(
                            b"Wrong line number.\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                }
            } else if line < 0 as libc::c_int as libc::c_long {
                if ldbDelBreakpoint(-line as libc::c_int) != 0 {
                    ldbLog(
                        sdsnew(
                            b"Breakpoint removed.\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {
                    ldbLog(
                        sdsnew(
                            b"No breakpoint in the specified line.\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                }
            }
            j_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ldbEval(
    mut lua: *mut lua_State,
    mut argv: *mut sds,
    mut argc: libc::c_int,
) {
    let mut code: sds = sdsjoinsds(
        argv.offset(1 as libc::c_int as isize),
        argc - 1 as libc::c_int,
        b" \0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    let mut expr: sds = sdscatsds(
        sdsnew(b"return \0" as *const u8 as *const libc::c_char),
        code,
    );
    if luaL_loadbuffer(
        lua,
        expr as *const libc::c_char,
        sdslen(expr),
        b"@ldb_eval\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
        if luaL_loadbuffer(
            lua,
            code as *const libc::c_char,
            sdslen(code),
            b"@ldb_eval\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            ldbLog(
                sdscatfmt(
                    sdsempty(),
                    b"<error> %s\0" as *const u8 as *const libc::c_char,
                    lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t),
                ),
            );
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            sdsfree(code);
            sdsfree(expr);
            return;
        }
    }
    sdsfree(code);
    sdsfree(expr);
    if lua_pcall(lua, 0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int) != 0 {
        ldbLog(
            sdscatfmt(
                sdsempty(),
                b"<error> %s\0" as *const u8 as *const libc::c_char,
                lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t),
            ),
        );
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
        return;
    }
    ldbLogStackValue(
        lua,
        b"<retval> \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ldbRedis(
    mut lua: *mut lua_State,
    mut argv: *mut sds,
    mut argc: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    if lua_checkstack(lua, argc + 1 as libc::c_int) == 0 {
        ldbLogRedisReply(
            b"max lua stack reached\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    lua_getfield(
        lua,
        -(10002 as libc::c_int),
        b"redis\0" as *const u8 as *const libc::c_char,
    );
    lua_pushstring(lua, b"call\0" as *const u8 as *const libc::c_char);
    lua_gettable(lua, -(2 as libc::c_int));
    j = 1 as libc::c_int;
    while j < argc {
        lua_pushlstring(
            lua,
            *argv.offset(j as isize) as *const libc::c_char,
            sdslen(*argv.offset(j as isize)),
        );
        j += 1;
    }
    ldb.step = 1 as libc::c_int;
    lua_pcall(lua, argc - 1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
    ldb.step = 0 as libc::c_int;
    lua_settop(lua, -(2 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ldbTrace(mut lua: *mut lua_State) {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        nups: 0,
        linedefined: 0,
        lastlinedefined: 0,
        short_src: [0; 60],
        i_ci: 0,
    };
    let mut level: libc::c_int = 0 as libc::c_int;
    while lua_getstack(lua, level, &mut ar) != 0 {
        lua_getinfo(lua, b"Snl\0" as *const u8 as *const libc::c_char, &mut ar);
        if !(strstr(
            (ar.short_src).as_mut_ptr(),
            b"user_script\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            ldbLog(
                sdscatprintf(
                    sdsempty(),
                    b"%s %s:\0" as *const u8 as *const libc::c_char,
                    if level == 0 as libc::c_int {
                        b"In\0" as *const u8 as *const libc::c_char
                    } else {
                        b"From\0" as *const u8 as *const libc::c_char
                    },
                    if !(ar.name).is_null() {
                        ar.name
                    } else {
                        b"top level\0" as *const u8 as *const libc::c_char
                    },
                ),
            );
            ldbLogSourceLine(ar.currentline);
        }
        level += 1;
    }
    if level == 0 as libc::c_int {
        ldbLog(
            sdsnew(
                b"<error> Can't retrieve Lua stack.\0" as *const u8
                    as *const libc::c_char,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldbMaxlen(mut argv: *mut sds, mut argc: libc::c_int) {
    if argc == 2 as libc::c_int {
        let mut newval: libc::c_int = atoi(
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
        );
        ldb.maxlen_hint_sent = 1 as libc::c_int;
        if newval != 0 as libc::c_int && newval <= 60 as libc::c_int {
            newval = 60 as libc::c_int;
        }
        ldb.maxlen = newval as size_t;
    }
    if ldb.maxlen != 0 {
        ldbLog(
            sdscatprintf(
                sdsempty(),
                b"<value> replies are truncated at %d bytes.\0" as *const u8
                    as *const libc::c_char,
                ldb.maxlen as libc::c_int,
            ),
        );
    } else {
        ldbLog(
            sdscatprintf(
                sdsempty(),
                b"<value> replies are unlimited.\0" as *const u8 as *const libc::c_char,
            ),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ldbRepl(mut lua: *mut lua_State) -> libc::c_int {
    let mut argv: *mut sds = 0 as *mut sds;
    let mut argc: libc::c_int = 0;
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        loop {
            argv = ldbReplParseCommand(&mut argc, &mut err);
            if !argv.is_null() {
                break;
            }
            let mut buf: [libc::c_char; 1024] = [0; 1024];
            if !err.is_null() {
                luaPushError(lua, err);
                luaError(lua);
            }
            let mut nread: libc::c_int = connRead(
                ldb.conn,
                buf.as_mut_ptr() as *mut libc::c_void,
                core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
            if nread <= 0 as libc::c_int {
                ldb.step = 0 as libc::c_int;
                ldb.bpcount = 0 as libc::c_int;
                return -(1 as libc::c_int);
            }
            ldb
                .cbuf = sdscatlen(
                ldb.cbuf,
                buf.as_mut_ptr() as *const libc::c_void,
                nread as size_t,
            );
            if sdslen(ldb.cbuf)
                > ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong
            {
                sdsfree(ldb.cbuf);
                ldb.cbuf = sdsempty();
                luaPushError(
                    lua,
                    b"max client buffer reached\0" as *const u8 as *const libc::c_char,
                );
                luaError(lua);
            }
        }
        sdsfree(ldb.cbuf);
        ldb.cbuf = sdsempty();
        if strcasecmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"h\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"help\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            ldbLog(
                sdsnew(b"Redis Lua debugger help:\0" as *const u8 as *const libc::c_char),
            );
            ldbLog(
                sdsnew(
                    b"[h]elp               Show this help.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[s]tep               Run current line and stop again.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[n]ext               Alias for step.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[c]ontinue           Run till next breakpoint.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[l]ist               List source code around current line.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[l]ist [line]        List source code around [line].\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"                     line = 0 means: current position.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[l]ist [line] [ctx]  In this form [ctx] specifies how many lines\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"                     to show before/after [line].\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[w]hole              List all source code. Alias for 'list 1 1000000'.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[p]rint              Show all the local variables.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[p]rint <var>        Show the value of the specified variable.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"                     Can also show global vars KEYS and ARGV.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[b]reak              Show all breakpoints.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[b]reak <line>       Add a breakpoint to the specified line.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[b]reak -<line>      Remove breakpoint from the specified line.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[b]reak 0            Remove all breakpoints.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[t]race              Show a backtrace.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[e]val <code>        Execute some Lua code (in a different callframe).\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[r]edis <cmd>        Execute a Redis command.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[m]axlen [len]       Trim logged Redis replies and Lua var dumps to len.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"                     Specifying zero as <len> means unlimited.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"[a]bort              Stop the execution of the script. In sync\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"                     mode dataset changes will be retained.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(sdsnew(b"\0" as *const u8 as *const libc::c_char));
            ldbLog(
                sdsnew(
                    b"Debugger functions you can call from Lua scripts:\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"redis.debug()        Produce logs in the debugger console.\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"redis.breakpoint()   Stop execution like if there was a breakpoint in the\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            ldbLog(
                sdsnew(
                    b"                     next line of code.\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            ldbSendLogs();
        } else if strcasecmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"step\0" as *const u8 as *const libc::c_char,
            ) == 0
            || strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"n\0" as *const u8 as *const libc::c_char,
            ) == 0
            || strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"next\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            ldb.step = 1 as libc::c_int;
            break;
        } else {
            if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"c\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"continue\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                break;
            }
            if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"t\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"trace\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                ldbTrace(lua);
                ldbSendLogs();
            } else if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"m\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"maxlen\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                ldbMaxlen(argv, argc);
                ldbSendLogs();
            } else if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"b\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"break\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                ldbBreak(argv, argc);
                ldbSendLogs();
            } else if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"e\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"eval\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                ldbEval(lua, argv, argc);
                ldbSendLogs();
            } else if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"a\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"abort\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                luaPushError(
                    lua,
                    b"script aborted for user request\0" as *const u8
                        as *const libc::c_char,
                );
                luaError(lua);
            } else if argc > 1 as libc::c_int
                && (strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"r\0" as *const u8 as *const libc::c_char,
                ) == 0
                    || strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"redis\0" as *const u8 as *const libc::c_char,
                    ) == 0)
            {
                ldbRedis(lua, argv, argc);
                ldbSendLogs();
            } else if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"p\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"print\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                if argc == 2 as libc::c_int {
                    ldbPrint(lua, *argv.offset(1 as libc::c_int as isize));
                } else {
                    ldbPrintAll(lua);
                }
                ldbSendLogs();
            } else if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"l\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"list\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                let mut around: libc::c_int = ldb.currentline;
                let mut ctx: libc::c_int = 5 as libc::c_int;
                if argc > 1 as libc::c_int {
                    let mut num: libc::c_int = atoi(
                        *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                    );
                    if num > 0 as libc::c_int {
                        around = num;
                    }
                }
                if argc > 2 as libc::c_int {
                    ctx = atoi(
                        *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
                    );
                }
                ldbList(around, ctx);
                ldbSendLogs();
            } else if strcasecmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcasecmp(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    b"whole\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                ldbList(1 as libc::c_int, 1000000 as libc::c_int);
                ldbSendLogs();
            } else {
                ldbLog(
                    sdsnew(
                        b"<error> Unknown Redis Lua debugger command or wrong number of arguments.\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
                ldbSendLogs();
            }
        }
        sdsfreesplitres(argv, argc);
    }
    sdsfreesplitres(argv, argc);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaLdbLineHook(
    mut lua: *mut lua_State,
    mut ar: *mut lua_Debug,
) {
    let mut rctx: *mut scriptRunCtx = luaGetFromRegistry(
        lua,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut scriptRunCtx;
    lua_getstack(lua, 0 as libc::c_int, ar);
    lua_getinfo(lua, b"Sl\0" as *const u8 as *const libc::c_char, ar);
    ldb.currentline = (*ar).currentline;
    let mut bp: libc::c_int = (ldbIsBreakpoint(ldb.currentline) != 0 || ldb.luabp != 0)
        as libc::c_int;
    let mut timeout: libc::c_int = 0 as libc::c_int;
    if (strstr(
        ((*ar).short_src).as_mut_ptr(),
        b"user_script\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        return;
    }
    if (*ar).event == 3 as libc::c_int && ldb.step == 0 as libc::c_int
        && bp == 0 as libc::c_int
    {
        let mut elapsed: mstime_t = elapsedMs((*rctx).start_time) as mstime_t;
        let mut timelimit: mstime_t = if server.busy_reply_threshold != 0 {
            server.busy_reply_threshold
        } else {
            5000 as libc::c_int as libc::c_longlong
        };
        if elapsed >= timelimit {
            timeout = 1 as libc::c_int;
            ldb.step = 1 as libc::c_int;
        } else {
            return
        }
    }
    if ldb.step != 0 || bp != 0 {
        let mut reason: *mut libc::c_char = b"step over\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        if bp != 0 {
            reason = (if ldb.luabp != 0 {
                b"redis.breakpoint() called\0" as *const u8 as *const libc::c_char
            } else {
                b"break point\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        } else if timeout != 0 {
            reason = b"timeout reached, infinite loop?\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        ldb.step = 0 as libc::c_int;
        ldb.luabp = 0 as libc::c_int;
        ldbLog(
            sdscatprintf(
                sdsempty(),
                b"* Stopped at %d, stop reason = %s\0" as *const u8
                    as *const libc::c_char,
                ldb.currentline,
                reason,
            ),
        );
        ldbLogSourceLine(ldb.currentline);
        ldbSendLogs();
        if ldbRepl(lua) == -(1 as libc::c_int) && timeout != 0 {
            luaPushError(
                lua,
                b"timeout during Lua debugging with client closing connection\0"
                    as *const u8 as *const libc::c_char,
            );
            luaError(lua);
        }
        (*rctx).start_time = getMonotonicUs.expect("non-null function pointer")();
        (*rctx).snapshot_time = mstime();
    }
}
