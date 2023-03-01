extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type lua_State;
    pub type RedisModuleCommand;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn freeClientArgv(c: *mut client);
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn setDeferredMapLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn setDeferredSetLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn addReplyNull(c: *mut client);
    fn addReplyBool(c: *mut client, b: libc::c_int);
    fn addReplyVerbatim(
        c: *mut client,
        s: *const libc::c_char,
        len: size_t,
        ext: *const libc::c_char,
    );
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyStatusLength(c: *mut client, s: *const libc::c_char, len: size_t);
    fn addReplyErrorSdsEx(c: *mut client, err: sds, flags: libc::c_int);
    fn addReplyDouble(c: *mut client, d: libc::c_double);
    fn addReplyBigNum(c: *mut client, num: *const libc::c_char, len: size_t);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyErrorFormatEx(
        c: *mut client,
        flags: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn decrRefCount(o: *mut robj);
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn sdsmapchars(
        s: sds,
        from: *const libc::c_char,
        to: *const libc::c_char,
        setlen: size_t,
    ) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_remove(L: *mut lua_State, idx: libc::c_int);
    fn lua_insert(L: *mut lua_State, idx: libc::c_int);
    fn lua_replace(L: *mut lua_State, idx: libc::c_int);
    fn lua_checkstack(L: *mut lua_State, sz: libc::c_int) -> libc::c_int;
    fn lua_isnumber(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_isstring(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tonumber(L: *mut lua_State, idx: libc::c_int) -> lua_Number;
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_objlen(L: *mut lua_State, idx: libc::c_int) -> size_t;
    fn lua_topointer(L: *mut lua_State, idx: libc::c_int) -> *const libc::c_void;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, l: size_t);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void);
    fn lua_gettable(L: *mut lua_State, idx: libc::c_int);
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawget(L: *mut lua_State, idx: libc::c_int);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_settable(L: *mut lua_State, idx: libc::c_int);
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawset(L: *mut lua_State, idx: libc::c_int);
    fn lua_rawseti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int);
    fn lua_pcall(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
    ) -> libc::c_int;
    fn lua_gc(L: *mut lua_State, what: libc::c_int, data: libc::c_int) -> libc::c_int;
    fn lua_error(L: *mut lua_State) -> libc::c_int;
    fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_sethook(
        L: *mut lua_State,
        func: lua_Hook,
        mask: libc::c_int,
        count: libc::c_int,
    ) -> libc::c_int;
    fn lua_enablereadonlytable(
        L: *mut lua_State,
        index: libc::c_int,
        enabled: libc::c_int,
    );
    fn lua_isreadonlytable(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdsfree(s: sds);
    fn sdsempty() -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn ACLCheckAllUserCommandPerm(
        u: *mut user,
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        idxptr: *mut libc::c_int,
    ) -> libc::c_int;
    fn lookupCommand(argv: *mut *mut robj, argc: libc::c_int) -> *mut redisCommand;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn serverLogRaw(level: libc::c_int, msg: *const libc::c_char);
    fn luaLdbLineHook(lua: *mut lua_State, ar: *mut lua_Debug);
    fn ldbIsEnabled() -> libc::c_int;
    fn ldbLog(entry: sds);
    fn ldbLogRedisReply(reply: *mut libc::c_char);
    fn sha1hex(digest: *mut libc::c_char, script: *mut libc::c_char, len: size_t);
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn scriptSetResp(r_ctx: *mut scriptRunCtx, resp: libc::c_int) -> libc::c_int;
    fn scriptSetRepl(r_ctx: *mut scriptRunCtx, repl: libc::c_int) -> libc::c_int;
    fn scriptCall(r_ctx: *mut scriptRunCtx, err: *mut sds);
    fn scriptInterrupt(r_ctx: *mut scriptRunCtx) -> libc::c_int;
    fn luaL_argerror(
        L: *mut lua_State,
        numarg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_checkinteger(L: *mut lua_State, numArg: libc::c_int) -> lua_Integer;
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaopen_base(L: *mut lua_State) -> libc::c_int;
    fn luaopen_table(L: *mut lua_State) -> libc::c_int;
    fn luaopen_string(L: *mut lua_State) -> libc::c_int;
    fn luaopen_math(L: *mut lua_State) -> libc::c_int;
    fn luaopen_debug(L: *mut lua_State) -> libc::c_int;
    fn redisLrand48() -> int32_t;
    fn redisSrand48(seedval: int32_t);
    fn parseReply(parser: *mut ReplyParser, p_ctx: *mut libc::c_void) -> libc::c_int;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn luaopen_cjson(L: *mut lua_State) -> libc::c_int;
    fn luaopen_struct(L: *mut lua_State) -> libc::c_int;
    fn luaopen_cmsgpack(L: *mut lua_State) -> libc::c_int;
    fn luaopen_bit(L: *mut lua_State) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uint_least64_t = __uint64_t;
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
#[repr(C)]
pub struct redisObject {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "4..=7")]
    #[bitfield(name = "lru", ty = "libc::c_uint", bits = "8..=31")]
    pub type_0_encoding_lru: [u8; 4],
    pub refcount: libc::c_int,
    pub ptr: *mut libc::c_void,
}
pub type ptrdiff_t = libc::c_long;
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = libc::c_double;
pub type lua_Integer = ptrdiff_t;
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
pub type lua_Hook = Option::<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
pub type sig_atomic_t = __sig_atomic_t;
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
    pub v: C2RustUnnamed,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
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
pub struct clientReplyBlock {
    pub size: size_t,
    pub used: size_t,
    pub buf: [libc::c_char; 0],
}
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
    pub bs: C2RustUnnamed_3,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub range: C2RustUnnamed_2,
    pub keynum: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub union C2RustUnnamed_3 {
    pub index: C2RustUnnamed_5,
    pub keyword: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub inst_metric: [C2RustUnnamed_6; 5],
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
pub struct C2RustUnnamed_6 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errorInfo {
    pub msg: sds,
    pub source: sds,
    pub line: sds,
    pub ignore_err_stats_update: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReplyParser {
    pub curr_location: *const libc::c_char,
    pub callbacks: ReplyParserCallbacks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReplyParserCallbacks {
    pub null_array_callback: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, size_t) -> (),
    >,
    pub null_bulk_string_callback: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, size_t) -> (),
    >,
    pub bulk_string_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub error_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub simple_str_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub long_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_longlong,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub array_callback: Option::<
        unsafe extern "C" fn(
            *mut ReplyParser,
            *mut libc::c_void,
            size_t,
            *const libc::c_char,
        ) -> (),
    >,
    pub set_callback: Option::<
        unsafe extern "C" fn(
            *mut ReplyParser,
            *mut libc::c_void,
            size_t,
            *const libc::c_char,
        ) -> (),
    >,
    pub map_callback: Option::<
        unsafe extern "C" fn(
            *mut ReplyParser,
            *mut libc::c_void,
            size_t,
            *const libc::c_char,
        ) -> (),
    >,
    pub bool_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub double_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub big_number_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub verbatim_string_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub attribute_callback: Option::<
        unsafe extern "C" fn(
            *mut ReplyParser,
            *mut libc::c_void,
            size_t,
            *const libc::c_char,
        ) -> (),
    >,
    pub null_callback: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, size_t) -> (),
    >,
    pub error: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[inline]
unsafe extern "C" fn sdsalloc(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .alloc as size_t;
        }
        2 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .alloc as size_t;
        }
        3 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .alloc as size_t;
        }
        4 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .alloc;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
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
static mut libraries_allow_list: [*mut libc::c_char; 8] = [
    b"string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cjson\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cmsgpack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"math\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"table\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"struct\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut redis_api_allow_list: [*mut libc::c_char; 3] = [
    b"redis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"__redis__err__handler\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut lua_builtins_allow_list: [*mut libc::c_char; 27] = [
    b"xpcall\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tostring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"getfenv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"setmetatable\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"next\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"assert\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tonumber\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rawequal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"collectgarbage\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"getmetatable\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rawset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pcall\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"coroutine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"_G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"select\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"unpack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gcinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pairs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rawget\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"loadstring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ipairs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"_VERSION\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"setfenv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"load\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut lua_builtins_not_documented_allow_list: [*mut libc::c_char; 2] = [
    b"newproxy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut lua_builtins_removed_after_initialization_allow_list: [*mut libc::c_char; 2] = [
    b"debug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut allow_lists: [*mut *mut libc::c_char; 6] = unsafe {
    [
        libraries_allow_list.as_ptr() as *mut _,
        redis_api_allow_list.as_ptr() as *mut _,
        lua_builtins_allow_list.as_ptr() as *mut _,
        lua_builtins_not_documented_allow_list.as_ptr() as *mut _,
        lua_builtins_removed_after_initialization_allow_list.as_ptr() as *mut _,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    ]
};
static mut deny_list: [*mut libc::c_char; 4] = [
    b"dofile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"loadfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"print\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn luaSaveOnRegistry(
    mut lua: *mut lua_State,
    mut name: *const libc::c_char,
    mut ptr: *mut libc::c_void,
) {
    lua_pushstring(lua, name);
    if !ptr.is_null() {
        lua_pushlightuserdata(lua, ptr);
    } else {
        lua_pushnil(lua);
    }
    lua_settable(lua, -(10000 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn luaGetFromRegistry(
    mut lua: *mut lua_State,
    mut name: *const libc::c_char,
) -> *mut libc::c_void {
    lua_pushstring(lua, name);
    lua_gettable(lua, -(10000 as libc::c_int));
    if lua_type(lua, -(1 as libc::c_int)) == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if lua_type(lua, -(1 as libc::c_int)) == 2 as libc::c_int {} else {
        _serverAssert(
            b"lua_islightuserdata(lua, -1)\0" as *const u8 as *const libc::c_char,
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
        );
        unreachable!();
    };
    let mut ptr: *mut libc::c_void = lua_topointer(lua, -(1 as libc::c_int))
        as *mut libc::c_void;
    if !ptr.is_null() {} else {
        _serverAssert(
            b"ptr\0" as *const u8 as *const libc::c_char,
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
        );
        unreachable!();
    };
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    return ptr;
}
static mut DefaultLuaTypeParserCallbacks: ReplyParserCallbacks = unsafe {
    {
        let mut init = ReplyParserCallbacks {
            null_array_callback: Some(
                redisProtocolToLuaType_NullArray
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            null_bulk_string_callback: Some(
                redisProtocolToLuaType_NullBulkString
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            bulk_string_callback: Some(
                redisProtocolToLuaType_BulkString
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            error_callback: Some(
                redisProtocolToLuaType_Error
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            simple_str_callback: Some(
                redisProtocolToLuaType_Status
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            long_callback: Some(
                redisProtocolToLuaType_Int
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_longlong,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            array_callback: Some(
                redisProtocolToLuaType_Array
                    as unsafe extern "C" fn(
                        *mut ReplyParser,
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_char,
                    ) -> (),
            ),
            set_callback: Some(
                redisProtocolToLuaType_Set
                    as unsafe extern "C" fn(
                        *mut ReplyParser,
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_char,
                    ) -> (),
            ),
            map_callback: Some(
                redisProtocolToLuaType_Map
                    as unsafe extern "C" fn(
                        *mut ReplyParser,
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_char,
                    ) -> (),
            ),
            bool_callback: Some(
                redisProtocolToLuaType_Bool
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            double_callback: Some(
                redisProtocolToLuaType_Double
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_double,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            big_number_callback: Some(
                redisProtocolToLuaType_BigNumber
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            verbatim_string_callback: Some(
                redisProtocolToLuaType_VerbatimString
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            attribute_callback: Some(
                redisProtocolToLuaType_Attribute
                    as unsafe extern "C" fn(
                        *mut ReplyParser,
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_char,
                    ) -> (),
            ),
            null_callback: Some(
                redisProtocolToLuaType_Null
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            error: None,
        };
        init
    }
};
unsafe extern "C" fn redisProtocolToLuaType(
    mut lua: *mut lua_State,
    mut reply: *mut libc::c_char,
) {
    let mut parser: ReplyParser = {
        let mut init = ReplyParser {
            curr_location: reply,
            callbacks: DefaultLuaTypeParserCallbacks,
        };
        init
    };
    parseReply(&mut parser, lua as *mut libc::c_void);
}
unsafe extern "C" fn redisProtocolToLuaType_Int(
    mut ctx: *mut libc::c_void,
    mut val: libc::c_longlong,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 1 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_pushnumber(lua, val as lua_Number);
}
unsafe extern "C" fn redisProtocolToLuaType_NullBulkString(
    mut ctx: *mut libc::c_void,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 1 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_pushboolean(lua, 0 as libc::c_int);
}
unsafe extern "C" fn redisProtocolToLuaType_NullArray(
    mut ctx: *mut libc::c_void,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 1 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            278 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_pushboolean(lua, 0 as libc::c_int);
}
unsafe extern "C" fn redisProtocolToLuaType_BulkString(
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 1 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_pushlstring(lua, str, len);
}
unsafe extern "C" fn redisProtocolToLuaType_Status(
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 3 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(lua, b"ok\0" as *const u8 as *const libc::c_char);
    lua_pushlstring(lua, str, len);
    lua_settable(lua, -(3 as libc::c_int));
}
unsafe extern "C" fn redisProtocolToLuaType_Error(
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 3 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    let mut err_msg: sds = sdscatlen(
        sdsnew(b"-\0" as *const u8 as *const libc::c_char),
        str as *const libc::c_void,
        len,
    );
    luaPushErrorBuff(lua, err_msg);
    lua_pushstring(
        lua,
        b"ignore_error_stats_update\0" as *const u8 as *const libc::c_char,
    );
    lua_pushboolean(lua, 1 as libc::c_int);
    lua_settable(lua, -(3 as libc::c_int));
}
unsafe extern "C" fn redisProtocolToLuaType_Map(
    mut parser: *mut ReplyParser,
    mut ctx: *mut libc::c_void,
    mut len: size_t,
    mut proto: *const libc::c_char,
) {
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if !lua.is_null() {
        if lua_checkstack(lua, 3 as libc::c_int) == 0 {
            _serverPanic(
                b"script_lua.c\0" as *const u8 as *const libc::c_char,
                348 as libc::c_int,
                b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                    as *const libc::c_char,
            );
            unreachable!();
        }
        lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
        lua_pushstring(lua, b"map\0" as *const u8 as *const libc::c_char);
        lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    }
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < len {
        parseReply(parser, lua as *mut libc::c_void);
        parseReply(parser, lua as *mut libc::c_void);
        if !lua.is_null() {
            lua_settable(lua, -(3 as libc::c_int));
        }
        j = j.wrapping_add(1);
    }
    if !lua.is_null() {
        lua_settable(lua, -(3 as libc::c_int));
    }
}
unsafe extern "C" fn redisProtocolToLuaType_Set(
    mut parser: *mut ReplyParser,
    mut ctx: *mut libc::c_void,
    mut len: size_t,
    mut proto: *const libc::c_char,
) {
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if !lua.is_null() {
        if lua_checkstack(lua, 3 as libc::c_int) == 0 {
            _serverPanic(
                b"script_lua.c\0" as *const u8 as *const libc::c_char,
                370 as libc::c_int,
                b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                    as *const libc::c_char,
            );
            unreachable!();
        }
        lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
        lua_pushstring(lua, b"set\0" as *const u8 as *const libc::c_char);
        lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    }
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < len {
        parseReply(parser, lua as *mut libc::c_void);
        if !lua.is_null() {
            if lua_checkstack(lua, 1 as libc::c_int) == 0 {
                _serverPanic(
                    b"script_lua.c\0" as *const u8 as *const libc::c_char,
                    384 as libc::c_int,
                    b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                        as *const libc::c_char,
                );
                unreachable!();
            }
            lua_pushboolean(lua, 1 as libc::c_int);
            lua_settable(lua, -(3 as libc::c_int));
        }
        j = j.wrapping_add(1);
    }
    if !lua.is_null() {
        lua_settable(lua, -(3 as libc::c_int));
    }
}
unsafe extern "C" fn redisProtocolToLuaType_Array(
    mut parser: *mut ReplyParser,
    mut ctx: *mut libc::c_void,
    mut len: size_t,
    mut proto: *const libc::c_char,
) {
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if !lua.is_null() {
        if lua_checkstack(lua, 2 as libc::c_int) == 0 {
            _serverPanic(
                b"script_lua.c\0" as *const u8 as *const libc::c_char,
                401 as libc::c_int,
                b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                    as *const libc::c_char,
            );
            unreachable!();
        }
        lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    }
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < len {
        if !lua.is_null() {
            lua_pushnumber(
                lua,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as lua_Number,
            );
        }
        parseReply(parser, lua as *mut libc::c_void);
        if !lua.is_null() {
            lua_settable(lua, -(3 as libc::c_int));
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn redisProtocolToLuaType_Attribute(
    mut parser: *mut ReplyParser,
    mut ctx: *mut libc::c_void,
    mut len: size_t,
    mut proto: *const libc::c_char,
) {
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < len {
        parseReply(parser, 0 as *mut libc::c_void);
        parseReply(parser, 0 as *mut libc::c_void);
        j = j.wrapping_add(1);
    }
    parseReply(parser, ctx);
}
unsafe extern "C" fn redisProtocolToLuaType_VerbatimString(
    mut ctx: *mut libc::c_void,
    mut format: *const libc::c_char,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 5 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(lua, b"verbatim_string\0" as *const u8 as *const libc::c_char);
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(lua, b"string\0" as *const u8 as *const libc::c_char);
    lua_pushlstring(lua, str, len);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"format\0" as *const u8 as *const libc::c_char);
    lua_pushlstring(lua, format, 3 as libc::c_int as size_t);
    lua_settable(lua, -(3 as libc::c_int));
    lua_settable(lua, -(3 as libc::c_int));
}
unsafe extern "C" fn redisProtocolToLuaType_BigNumber(
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 3 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            464 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(lua, b"big_number\0" as *const u8 as *const libc::c_char);
    lua_pushlstring(lua, str, len);
    lua_settable(lua, -(3 as libc::c_int));
}
unsafe extern "C" fn redisProtocolToLuaType_Null(
    mut ctx: *mut libc::c_void,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 1 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_pushnil(lua);
}
unsafe extern "C" fn redisProtocolToLuaType_Bool(
    mut ctx: *mut libc::c_void,
    mut val: libc::c_int,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 1 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            499 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_pushboolean(lua, val);
}
unsafe extern "C" fn redisProtocolToLuaType_Double(
    mut ctx: *mut libc::c_void,
    mut d: libc::c_double,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut lua: *mut lua_State = ctx as *mut lua_State;
    if lua_checkstack(lua, 3 as libc::c_int) == 0 {
        _serverPanic(
            b"script_lua.c\0" as *const u8 as *const libc::c_char,
            515 as libc::c_int,
            b"lua stack limit reach when parsing redis.call reply\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(lua, b"double\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, d);
    lua_settable(lua, -(3 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn luaPushErrorBuff(mut lua: *mut lua_State, mut err_buffer: sds) {
    let mut msg: sds = 0 as *mut libc::c_char;
    let mut error_code: sds = 0 as *mut libc::c_char;
    if ldbIsEnabled() != 0 {
        ldbLog(
            sdscatprintf(
                sdsempty(),
                b"<error> %s\0" as *const u8 as *const libc::c_char,
                err_buffer,
            ),
        );
    }
    if *err_buffer.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        let mut err_msg: *mut libc::c_char = strstr(
            err_buffer as *const libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
        );
        if err_msg.is_null() {
            msg = sdsnew(
                err_buffer.offset(1 as libc::c_int as isize) as *const libc::c_char,
            );
            error_code = sdsnew(b"ERR\0" as *const u8 as *const libc::c_char);
        } else {
            *err_msg = '\0' as i32 as libc::c_char;
            msg = sdsnew(err_msg.offset(1 as libc::c_int as isize));
            error_code = sdsnew(
                err_buffer.offset(1 as libc::c_int as isize) as *const libc::c_char,
            );
        }
        sdsfree(err_buffer);
    } else {
        msg = err_buffer;
        error_code = sdsnew(b"ERR\0" as *const u8 as *const libc::c_char);
    }
    msg = sdstrim(msg, b"\r\n\0" as *const u8 as *const libc::c_char);
    let mut final_msg: sds = sdscatfmt(
        error_code,
        b" %s\0" as *const u8 as *const libc::c_char,
        msg,
    );
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(lua, b"err\0" as *const u8 as *const libc::c_char);
    lua_pushstring(lua, final_msg as *const libc::c_char);
    lua_settable(lua, -(3 as libc::c_int));
    sdsfree(msg);
    sdsfree(final_msg);
}
#[no_mangle]
pub unsafe extern "C" fn luaPushError(
    mut lua: *mut lua_State,
    mut error: *const libc::c_char,
) {
    luaPushErrorBuff(lua, sdsnew(error));
}
#[no_mangle]
pub unsafe extern "C" fn luaError(mut lua: *mut lua_State) -> libc::c_int {
    return lua_error(lua);
}
unsafe extern "C" fn luaReplyToRedisReply(
    mut c: *mut client,
    mut script_client: *mut client,
    mut lua: *mut lua_State,
) {
    let mut t: libc::c_int = lua_type(lua, -(1 as libc::c_int));
    if lua_checkstack(lua, 4 as libc::c_int) == 0 {
        addReplyErrorFormat(
            c,
            b"reached lua stack limit\0" as *const u8 as *const libc::c_char,
        );
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
        return;
    }
    let mut replylen_1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j: libc::c_int = 0;
    let mut mbulklen: libc::c_int = 0;
    match t {
        4 => {
            addReplyBulkCBuffer(
                c,
                lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t)
                    as *mut libc::c_char as *const libc::c_void,
                lua_objlen(lua, -(1 as libc::c_int)),
            );
        }
        1 => {
            if (*script_client).resp == 2 as libc::c_int {
                addReply(
                    c,
                    if lua_toboolean(lua, -(1 as libc::c_int)) != 0 {
                        shared.cone
                    } else {
                        shared.null[(*c).resp as usize]
                    },
                );
            } else {
                addReplyBool(c, lua_toboolean(lua, -(1 as libc::c_int)));
            }
        }
        3 => {
            addReplyLongLong(
                c,
                lua_tonumber(lua, -(1 as libc::c_int)) as libc::c_longlong,
            );
        }
        5 => {
            lua_pushstring(lua, b"err\0" as *const u8 as *const libc::c_char);
            lua_rawget(lua, -(2 as libc::c_int));
            t = lua_type(lua, -(1 as libc::c_int));
            if t == 4 as libc::c_int {
                lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
                let mut err_info: errorInfo = {
                    let mut init = errorInfo {
                        msg: 0 as sds,
                        source: 0 as *mut libc::c_char,
                        line: 0 as *mut libc::c_char,
                        ignore_err_stats_update: 0,
                    };
                    init
                };
                luaExtractErrorInformation(lua, &mut err_info);
                addReplyErrorFormatEx(
                    c,
                    (if err_info.ignore_err_stats_update != 0 {
                        (1 as libc::c_ulonglong) << 0 as libc::c_int
                    } else {
                        0 as libc::c_int as libc::c_ulonglong
                    }) as libc::c_int,
                    b"-%s\0" as *const u8 as *const libc::c_char,
                    err_info.msg,
                );
                luaErrorInformationDiscard(&mut err_info);
                lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
                return;
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            lua_pushstring(lua, b"ok\0" as *const u8 as *const libc::c_char);
            lua_rawget(lua, -(2 as libc::c_int));
            t = lua_type(lua, -(1 as libc::c_int));
            if t == 4 as libc::c_int {
                let mut ok: sds = sdsnew(
                    lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t),
                );
                sdsmapchars(
                    ok,
                    b"\r\n\0" as *const u8 as *const libc::c_char,
                    b"  \0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as size_t,
                );
                addReplyStatusLength(c, ok as *const libc::c_char, sdslen(ok));
                sdsfree(ok);
                lua_settop(lua, -(2 as libc::c_int) - 1 as libc::c_int);
                return;
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            lua_pushstring(lua, b"double\0" as *const u8 as *const libc::c_char);
            lua_rawget(lua, -(2 as libc::c_int));
            t = lua_type(lua, -(1 as libc::c_int));
            if t == 3 as libc::c_int {
                addReplyDouble(c, lua_tonumber(lua, -(1 as libc::c_int)));
                lua_settop(lua, -(2 as libc::c_int) - 1 as libc::c_int);
                return;
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            lua_pushstring(lua, b"big_number\0" as *const u8 as *const libc::c_char);
            lua_rawget(lua, -(2 as libc::c_int));
            t = lua_type(lua, -(1 as libc::c_int));
            if t == 4 as libc::c_int {
                let mut big_num: sds = sdsnewlen(
                    lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t)
                        as *const libc::c_void,
                    lua_objlen(lua, -(1 as libc::c_int)),
                );
                sdsmapchars(
                    big_num,
                    b"\r\n\0" as *const u8 as *const libc::c_char,
                    b"  \0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as size_t,
                );
                addReplyBigNum(c, big_num as *const libc::c_char, sdslen(big_num));
                sdsfree(big_num);
                lua_settop(lua, -(2 as libc::c_int) - 1 as libc::c_int);
                return;
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            lua_pushstring(
                lua,
                b"verbatim_string\0" as *const u8 as *const libc::c_char,
            );
            lua_rawget(lua, -(2 as libc::c_int));
            t = lua_type(lua, -(1 as libc::c_int));
            if t == 5 as libc::c_int {
                lua_pushstring(lua, b"format\0" as *const u8 as *const libc::c_char);
                lua_rawget(lua, -(2 as libc::c_int));
                t = lua_type(lua, -(1 as libc::c_int));
                if t == 4 as libc::c_int {
                    let mut format: *mut libc::c_char = lua_tolstring(
                        lua,
                        -(1 as libc::c_int),
                        0 as *mut size_t,
                    ) as *mut libc::c_char;
                    lua_pushstring(lua, b"string\0" as *const u8 as *const libc::c_char);
                    lua_rawget(lua, -(3 as libc::c_int));
                    t = lua_type(lua, -(1 as libc::c_int));
                    if t == 4 as libc::c_int {
                        let mut len: size_t = 0;
                        let mut str: *mut libc::c_char = lua_tolstring(
                            lua,
                            -(1 as libc::c_int),
                            &mut len,
                        ) as *mut libc::c_char;
                        addReplyVerbatim(c, str, len, format);
                        lua_settop(lua, -(4 as libc::c_int) - 1 as libc::c_int);
                        return;
                    }
                    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
                }
                lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            lua_pushstring(lua, b"map\0" as *const u8 as *const libc::c_char);
            lua_rawget(lua, -(2 as libc::c_int));
            t = lua_type(lua, -(1 as libc::c_int));
            if t == 5 as libc::c_int {
                let mut maplen: libc::c_int = 0 as libc::c_int;
                let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
                lua_pushnil(lua);
                while lua_next(lua, -(2 as libc::c_int)) != 0 {
                    lua_pushvalue(lua, -(2 as libc::c_int));
                    luaReplyToRedisReply(c, script_client, lua);
                    luaReplyToRedisReply(c, script_client, lua);
                    maplen += 1;
                }
                setDeferredMapLen(c, replylen, maplen as libc::c_long);
                lua_settop(lua, -(2 as libc::c_int) - 1 as libc::c_int);
                return;
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            lua_pushstring(lua, b"set\0" as *const u8 as *const libc::c_char);
            lua_rawget(lua, -(2 as libc::c_int));
            t = lua_type(lua, -(1 as libc::c_int));
            if t == 5 as libc::c_int {
                let mut setlen: libc::c_int = 0 as libc::c_int;
                let mut replylen_0: *mut libc::c_void = addReplyDeferredLen(c);
                lua_pushnil(lua);
                while lua_next(lua, -(2 as libc::c_int)) != 0 {
                    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
                    lua_pushvalue(lua, -(1 as libc::c_int));
                    luaReplyToRedisReply(c, script_client, lua);
                    setlen += 1;
                }
                setDeferredSetLen(c, replylen_0, setlen as libc::c_long);
                lua_settop(lua, -(2 as libc::c_int) - 1 as libc::c_int);
                return;
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            replylen_1 = addReplyDeferredLen(c);
            j = 1 as libc::c_int;
            mbulklen = 0 as libc::c_int;
            loop {
                let fresh0 = j;
                j = j + 1;
                lua_pushnumber(lua, fresh0 as lua_Number);
                lua_rawget(lua, -(2 as libc::c_int));
                t = lua_type(lua, -(1 as libc::c_int));
                if t == 0 as libc::c_int {
                    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
                    break;
                } else {
                    luaReplyToRedisReply(c, script_client, lua);
                    mbulklen += 1;
                }
            }
            setDeferredArrayLen(c, replylen_1, mbulklen as libc::c_long);
        }
        _ => {
            addReplyNull(c);
        }
    }
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
}
static mut lua_argv: *mut *mut robj = 0 as *const *mut robj as *mut *mut robj;
static mut lua_argv_size: libc::c_int = 0 as libc::c_int;
static mut lua_args_cached_objects: [*mut robj; 32] = [0 as *const robj
    as *mut robj; 32];
static mut lua_args_cached_objects_len: [size_t; 32] = [0; 32];
unsafe extern "C" fn luaArgsToRedisArgv(
    mut lua: *mut lua_State,
    mut argc: *mut libc::c_int,
    mut argv_len: *mut libc::c_int,
) -> *mut *mut robj {
    let mut j: libc::c_int = 0;
    *argc = lua_gettop(lua);
    if *argc == 0 as libc::c_int {
        luaPushError(
            lua,
            b"Please specify at least one argument for this redis lib call\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut *mut robj;
    }
    if lua_argv_size < *argc {
        lua_argv = zrealloc(
            lua_argv as *mut libc::c_void,
            (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                .wrapping_mul(*argc as libc::c_ulong),
        ) as *mut *mut robj;
        lua_argv_size = *argc;
    }
    *argv_len = lua_argv_size;
    j = 0 as libc::c_int;
    while j < *argc {
        let mut obj_s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut obj_len: size_t = 0;
        let mut dbuf: [libc::c_char; 64] = [0; 64];
        if lua_type(lua, j + 1 as libc::c_int) == 3 as libc::c_int {
            let mut num: lua_Number = lua_tonumber(lua, j + 1 as libc::c_int);
            obj_len = snprintf(
                dbuf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"%.17g\0" as *const u8 as *const libc::c_char,
                num,
            ) as size_t;
            obj_s = dbuf.as_mut_ptr();
        } else {
            obj_s = lua_tolstring(lua, j + 1 as libc::c_int, &mut obj_len)
                as *mut libc::c_char;
            if obj_s.is_null() {
                break;
            }
        }
        if j < 32 as libc::c_int && !(lua_args_cached_objects[j as usize]).is_null()
            && lua_args_cached_objects_len[j as usize] >= obj_len
        {
            let mut s: sds = (*lua_args_cached_objects[j as usize]).ptr as sds;
            let ref mut fresh1 = *lua_argv.offset(j as isize);
            *fresh1 = lua_args_cached_objects[j as usize];
            lua_args_cached_objects[j as usize] = 0 as *mut robj;
            memcpy(
                s as *mut libc::c_void,
                obj_s as *const libc::c_void,
                obj_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            sdssetlen(s, obj_len);
        } else {
            let ref mut fresh2 = *lua_argv.offset(j as isize);
            *fresh2 = createStringObject(obj_s, obj_len);
        }
        j += 1;
    }
    lua_settop(lua, -*argc - 1 as libc::c_int);
    if j != *argc {
        freeLuaRedisArgv(lua_argv, j, lua_argv_size);
        luaPushError(
            lua,
            b"Lua redis lib command arguments must be strings or integers\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut *mut robj;
    }
    return lua_argv;
}
#[no_mangle]
pub unsafe extern "C" fn freeLuaRedisArgv(
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut argv_len: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < argc {
        let mut o: *mut robj = *argv.offset(j as isize);
        if j < 32 as libc::c_int && (*o).refcount == 1 as libc::c_int
            && ((*o).encoding() as libc::c_int == 0 as libc::c_int
                || (*o).encoding() as libc::c_int == 8 as libc::c_int)
            && sdslen((*o).ptr as sds) <= 64 as libc::c_int as libc::c_ulong
        {
            let mut s: sds = (*o).ptr as sds;
            if !(lua_args_cached_objects[j as usize]).is_null() {
                decrRefCount(lua_args_cached_objects[j as usize]);
            }
            lua_args_cached_objects[j as usize] = o;
            lua_args_cached_objects_len[j as usize] = sdsalloc(s);
        } else {
            decrRefCount(o);
        }
        j += 1;
    }
    if argv != lua_argv || argv_len != lua_argv_size {
        zfree(argv as *mut libc::c_void);
        lua_argv = 0 as *mut *mut robj;
        lua_argv_size = 0 as libc::c_int;
    }
}
unsafe extern "C" fn luaRedisGenericCommand(
    mut lua: *mut lua_State,
    mut raise_error: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut rctx: *mut scriptRunCtx = luaGetFromRegistry(
        lua,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut scriptRunCtx;
    if rctx.is_null() {
        luaPushError(
            lua,
            b"redis.call/pcall can only be called inside a script invocation\0"
                as *const u8 as *const libc::c_char,
        );
        return luaError(lua);
    }
    let mut err: sds = 0 as sds;
    let mut c: *mut client = (*rctx).c;
    let mut reply: sds = 0 as *mut libc::c_char;
    (*c).argv = luaArgsToRedisArgv(lua, &mut (*c).argc, &mut (*c).argv_len);
    if ((*c).argv).is_null() {
        return if raise_error != 0 { luaError(lua) } else { 1 as libc::c_int };
    }
    static mut inuse: libc::c_int = 0 as libc::c_int;
    if inuse != 0 {
        let mut recursion_warning: *mut libc::c_char = b"luaRedisGenericCommand() recursive call detected. Are you doing funny stuff with Lua debug hooks?\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                recursion_warning,
            );
        }
        luaPushError(lua, recursion_warning);
        return 1 as libc::c_int;
    }
    inuse += 1;
    if ldbIsEnabled() != 0 {
        let mut cmdlog: sds = sdsnew(b"<redis>\0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < (*c).argc {
            if j == 10 as libc::c_int {
                cmdlog = sdscatprintf(
                    cmdlog,
                    b" ... (%d more)\0" as *const u8 as *const libc::c_char,
                    (*c).argc - j - 1 as libc::c_int,
                );
                break;
            } else {
                cmdlog = sdscatlen(
                    cmdlog,
                    b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
                cmdlog = sdscatsds(
                    cmdlog,
                    (**((*c).argv).offset(j as isize)).ptr as sds,
                );
                j += 1;
            }
        }
        ldbLog(cmdlog);
    }
    scriptCall(rctx, &mut err);
    if !err.is_null() {
        luaPushError(lua, err as *const libc::c_char);
        sdsfree(err);
        lua_pushstring(
            lua,
            b"ignore_error_stats_update\0" as *const u8 as *const libc::c_char,
        );
        lua_pushboolean(lua, 1 as libc::c_int);
        lua_settable(lua, -(3 as libc::c_int));
    } else {
        if (*(*c).reply).len == 0 as libc::c_int as libc::c_ulong
            && ((*c).bufpos as size_t) < (*c).buf_usable_size
        {
            *((*c).buf).offset((*c).bufpos as isize) = '\0' as i32 as libc::c_char;
            reply = (*c).buf;
            (*c).bufpos = 0 as libc::c_int;
        } else {
            reply = sdsnewlen((*c).buf as *const libc::c_void, (*c).bufpos as size_t);
            (*c).bufpos = 0 as libc::c_int;
            while (*(*c).reply).len != 0 {
                let mut o: *mut clientReplyBlock = (*(*(*c).reply).head).value
                    as *mut clientReplyBlock;
                reply = sdscatlen(
                    reply,
                    ((*o).buf).as_mut_ptr() as *const libc::c_void,
                    (*o).used,
                );
                listDelNode((*c).reply, (*(*c).reply).head);
            }
        }
        if raise_error != 0
            && *reply.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
        {
            raise_error = 0 as libc::c_int;
        }
        redisProtocolToLuaType(lua, reply);
        if ldbIsEnabled() != 0 {
            ldbLogRedisReply(reply);
        }
        if reply != (*c).buf {
            sdsfree(reply);
        }
        (*c).reply_bytes = 0 as libc::c_int as libc::c_ulonglong;
    }
    freeLuaRedisArgv((*c).argv, (*c).argc, (*c).argv_len);
    (*c).argv_len = 0 as libc::c_int;
    (*c).argc = (*c).argv_len;
    (*c).user = 0 as *mut user;
    (*c).argv = 0 as *mut *mut robj;
    freeClientArgv(c);
    inuse -= 1;
    if raise_error != 0 {
        return luaError(lua);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaRedisPcall(mut lua: *mut lua_State) -> libc::c_int {
    let mut argc: libc::c_int = lua_gettop(lua);
    lua_pushboolean(lua, 1 as libc::c_int);
    lua_insert(lua, 1 as libc::c_int);
    if lua_pcall(lua, argc - 1 as libc::c_int, -(1 as libc::c_int), 0 as libc::c_int)
        != 0
    {
        lua_remove(lua, 1 as libc::c_int);
        if lua_type(lua, -(1 as libc::c_int)) == 5 as libc::c_int {
            lua_getfield(
                lua,
                -(1 as libc::c_int),
                b"err\0" as *const u8 as *const libc::c_char,
            );
            if lua_isstring(lua, -(1 as libc::c_int)) != 0 {
                lua_replace(lua, -(2 as libc::c_int));
            }
        }
        lua_pushboolean(lua, 0 as libc::c_int);
        lua_insert(lua, 1 as libc::c_int);
    }
    return lua_gettop(lua);
}
unsafe extern "C" fn luaRedisCallCommand(mut lua: *mut lua_State) -> libc::c_int {
    return luaRedisGenericCommand(lua, 1 as libc::c_int);
}
unsafe extern "C" fn luaRedisPCallCommand(mut lua: *mut lua_State) -> libc::c_int {
    return luaRedisGenericCommand(lua, 0 as libc::c_int);
}
unsafe extern "C" fn luaRedisSha1hexCommand(mut lua: *mut lua_State) -> libc::c_int {
    let mut argc: libc::c_int = lua_gettop(lua);
    let mut digest: [libc::c_char; 41] = [0; 41];
    let mut len: size_t = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc != 1 as libc::c_int {
        luaPushError(
            lua,
            b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
        );
        return luaError(lua);
    }
    s = lua_tolstring(lua, 1 as libc::c_int, &mut len) as *mut libc::c_char;
    sha1hex(digest.as_mut_ptr(), s, len);
    lua_pushstring(lua, digest.as_mut_ptr());
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaRedisReturnSingleFieldTable(
    mut lua: *mut lua_State,
    mut field: *mut libc::c_char,
) -> libc::c_int {
    if lua_gettop(lua) != 1 as libc::c_int
        || lua_type(lua, -(1 as libc::c_int)) != 4 as libc::c_int
    {
        luaPushError(
            lua,
            b"wrong number or type of arguments\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(lua, field);
    lua_pushvalue(lua, -(3 as libc::c_int));
    lua_settable(lua, -(3 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaRedisErrorReplyCommand(mut lua: *mut lua_State) -> libc::c_int {
    if lua_gettop(lua) != 1 as libc::c_int
        || lua_type(lua, -(1 as libc::c_int)) != 4 as libc::c_int
    {
        luaPushError(
            lua,
            b"wrong number or type of arguments\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut err: *const libc::c_char = lua_tolstring(
        lua,
        -(1 as libc::c_int),
        0 as *mut size_t,
    );
    let mut err_buff: sds = 0 as sds;
    if *err.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
        err_buff = sdscatfmt(
            sdsempty(),
            b"-%s\0" as *const u8 as *const libc::c_char,
            err,
        );
    } else {
        err_buff = sdsnew(err);
    }
    luaPushErrorBuff(lua, err_buff);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaRedisStatusReplyCommand(mut lua: *mut lua_State) -> libc::c_int {
    return luaRedisReturnSingleFieldTable(
        lua,
        b"ok\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn luaRedisSetReplCommand(mut lua: *mut lua_State) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    let mut argc: libc::c_int = lua_gettop(lua);
    let mut rctx: *mut scriptRunCtx = luaGetFromRegistry(
        lua,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut scriptRunCtx;
    if rctx.is_null() {
        luaPushError(
            lua,
            b"redis.set_repl can only be called inside a script invocation\0"
                as *const u8 as *const libc::c_char,
        );
        return luaError(lua);
    }
    if argc != 1 as libc::c_int {
        luaPushError(
            lua,
            b"redis.set_repl() requires two arguments.\0" as *const u8
                as *const libc::c_char,
        );
        return luaError(lua);
    }
    flags = lua_tonumber(lua, -(1 as libc::c_int)) as libc::c_int;
    if flags & !(1 as libc::c_int | 2 as libc::c_int) != 0 as libc::c_int {
        luaPushError(
            lua,
            b"Invalid replication flags. Use REPL_AOF, REPL_REPLICA, REPL_ALL or REPL_NONE.\0"
                as *const u8 as *const libc::c_char,
        );
        return luaError(lua);
    }
    scriptSetRepl(rctx, flags);
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaRedisAclCheckCmdPermissionsCommand(
    mut lua: *mut lua_State,
) -> libc::c_int {
    let mut rctx: *mut scriptRunCtx = luaGetFromRegistry(
        lua,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut scriptRunCtx;
    if rctx.is_null() {
        luaPushError(
            lua,
            b"redis.acl_check_cmd can only be called inside a script invocation\0"
                as *const u8 as *const libc::c_char,
        );
        return luaError(lua);
    }
    let mut raise_error: libc::c_int = 0 as libc::c_int;
    let mut argc: libc::c_int = 0;
    let mut argv_len: libc::c_int = 0;
    let mut argv: *mut *mut robj = luaArgsToRedisArgv(lua, &mut argc, &mut argv_len);
    if argv.is_null() {
        return luaError(lua);
    }
    let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
    cmd = lookupCommand(argv, argc);
    if cmd.is_null() {
        luaPushError(
            lua,
            b"Invalid command passed to redis.acl_check_cmd()\0" as *const u8
                as *const libc::c_char,
        );
        raise_error = 1 as libc::c_int;
    } else {
        let mut keyidxptr: libc::c_int = 0;
        if ACLCheckAllUserCommandPerm(
            (*(*rctx).original_client).user,
            cmd,
            argv,
            argc,
            &mut keyidxptr,
        ) != 0 as libc::c_int
        {
            lua_pushboolean(lua, 0 as libc::c_int);
        } else {
            lua_pushboolean(lua, 1 as libc::c_int);
        }
    }
    freeLuaRedisArgv(argv, argc, argv_len);
    if raise_error != 0 { return luaError(lua) } else { return 1 as libc::c_int };
}
unsafe extern "C" fn luaLogCommand(mut lua: *mut lua_State) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut argc: libc::c_int = lua_gettop(lua);
    let mut level: libc::c_int = 0;
    let mut log: sds = 0 as *mut libc::c_char;
    if argc < 2 as libc::c_int {
        luaPushError(
            lua,
            b"redis.log() requires two arguments or more.\0" as *const u8
                as *const libc::c_char,
        );
        return luaError(lua);
    } else {
        if lua_isnumber(lua, -argc) == 0 {
            luaPushError(
                lua,
                b"First argument must be a number (log level).\0" as *const u8
                    as *const libc::c_char,
            );
            return luaError(lua);
        }
    }
    level = lua_tonumber(lua, -argc) as libc::c_int;
    if level < 0 as libc::c_int || level > 3 as libc::c_int {
        luaPushError(lua, b"Invalid debug level.\0" as *const u8 as *const libc::c_char);
        return luaError(lua);
    }
    if level < server.verbosity {
        return 0 as libc::c_int;
    }
    log = sdsempty();
    j = 1 as libc::c_int;
    while j < argc {
        let mut len: size_t = 0;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = lua_tolstring(lua, -argc + j, &mut len) as *mut libc::c_char;
        if !s.is_null() {
            if j != 1 as libc::c_int {
                log = sdscatlen(
                    log,
                    b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            }
            log = sdscatlen(log, s as *const libc::c_void, len);
        }
        j += 1;
    }
    serverLogRaw(level, log as *const libc::c_char);
    sdsfree(log);
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaSetResp(mut lua: *mut lua_State) -> libc::c_int {
    let mut rctx: *mut scriptRunCtx = luaGetFromRegistry(
        lua,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut scriptRunCtx;
    if rctx.is_null() {
        luaPushError(
            lua,
            b"redis.setresp can only be called inside a script invocation\0" as *const u8
                as *const libc::c_char,
        );
        return luaError(lua);
    }
    let mut argc: libc::c_int = lua_gettop(lua);
    if argc != 1 as libc::c_int {
        luaPushError(
            lua,
            b"redis.setresp() requires one argument.\0" as *const u8
                as *const libc::c_char,
        );
        return luaError(lua);
    }
    let mut resp: libc::c_int = lua_tonumber(lua, -argc) as libc::c_int;
    if resp != 2 as libc::c_int && resp != 3 as libc::c_int {
        luaPushError(
            lua,
            b"RESP version must be 2 or 3.\0" as *const u8 as *const libc::c_char,
        );
        return luaError(lua);
    }
    scriptSetResp(rctx, resp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaLoadLib(
    mut lua: *mut lua_State,
    mut libname: *const libc::c_char,
    mut luafunc: lua_CFunction,
) {
    lua_pushcclosure(lua, luafunc, 0 as libc::c_int);
    lua_pushstring(lua, libname);
    lua_call(lua, 1 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn luaLoadLibraries(mut lua: *mut lua_State) {
    luaLoadLib(
        lua,
        b"\0" as *const u8 as *const libc::c_char,
        Some(luaopen_base as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    luaLoadLib(
        lua,
        b"table\0" as *const u8 as *const libc::c_char,
        Some(luaopen_table as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    luaLoadLib(
        lua,
        b"string\0" as *const u8 as *const libc::c_char,
        Some(luaopen_string as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    luaLoadLib(
        lua,
        b"math\0" as *const u8 as *const libc::c_char,
        Some(luaopen_math as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    luaLoadLib(
        lua,
        b"debug\0" as *const u8 as *const libc::c_char,
        Some(luaopen_debug as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    luaLoadLib(
        lua,
        b"cjson\0" as *const u8 as *const libc::c_char,
        Some(luaopen_cjson as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    luaLoadLib(
        lua,
        b"struct\0" as *const u8 as *const libc::c_char,
        Some(luaopen_struct as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    luaLoadLib(
        lua,
        b"cmsgpack\0" as *const u8 as *const libc::c_char,
        Some(luaopen_cmsgpack as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    luaLoadLib(
        lua,
        b"bit\0" as *const u8 as *const libc::c_char,
        Some(luaopen_bit as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaGetStringSds(
    mut lua: *mut lua_State,
    mut index: libc::c_int,
) -> sds {
    if lua_isstring(lua, index) == 0 {
        return 0 as sds;
    }
    let mut len: size_t = 0;
    let mut str: *const libc::c_char = lua_tolstring(lua, index, &mut len);
    let mut str_sds: sds = sdsnewlen(str as *const libc::c_void, len);
    return str_sds;
}
unsafe extern "C" fn luaProtectedTableError(mut lua: *mut lua_State) -> libc::c_int {
    let mut argc: libc::c_int = lua_gettop(lua);
    if argc != 2 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"malicious code trying to call luaProtectedTableError with wrong arguments\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        luaL_error(
            lua,
            b"Wrong number of arguments to luaProtectedTableError\0" as *const u8
                as *const libc::c_char,
        );
    }
    if lua_isstring(lua, -(1 as libc::c_int)) == 0
        && lua_isnumber(lua, -(1 as libc::c_int)) == 0
    {
        luaL_error(
            lua,
            b"Second argument to luaProtectedTableError must be a string or number\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut variable_name: *const libc::c_char = lua_tolstring(
        lua,
        -(1 as libc::c_int),
        0 as *mut size_t,
    );
    luaL_error(
        lua,
        b"Script attempted to access nonexistent global variable '%s'\0" as *const u8
            as *const libc::c_char,
        variable_name,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaSetErrorMetatable(mut lua: *mut lua_State) {
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushcclosure(
        lua,
        Some(
            luaProtectedTableError as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_setfield(
        lua,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(lua, -(2 as libc::c_int));
}
unsafe extern "C" fn luaNewIndexAllowList(mut lua: *mut lua_State) -> libc::c_int {
    let mut argc: libc::c_int = lua_gettop(lua);
    if argc != 3 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"malicious code trying to call luaProtectedTableError with wrong arguments\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        luaL_error(
            lua,
            b"Wrong number of arguments to luaNewIndexAllowList\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(lua_type(lua, -(3 as libc::c_int)) == 5 as libc::c_int) {
        luaL_error(
            lua,
            b"first argument to luaNewIndexAllowList must be a table\0" as *const u8
                as *const libc::c_char,
        );
    }
    if lua_isstring(lua, -(2 as libc::c_int)) == 0
        && lua_isnumber(lua, -(2 as libc::c_int)) == 0
    {
        luaL_error(
            lua,
            b"Second argument to luaNewIndexAllowList must be a string or number\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut variable_name: *const libc::c_char = lua_tolstring(
        lua,
        -(2 as libc::c_int),
        0 as *mut size_t,
    );
    let mut allow_l: *mut *mut *mut libc::c_char = allow_lists.as_mut_ptr();
    while !(*allow_l).is_null() {
        let mut c: *mut *mut libc::c_char = *allow_l;
        while !(*c).is_null() {
            if strcmp(*c, variable_name) == 0 as libc::c_int {
                break;
            }
            c = c.offset(1);
        }
        if !(*c).is_null() {
            break;
        }
        allow_l = allow_l.offset(1);
    }
    if (*allow_l).is_null() {
        let mut c_0: *mut *mut libc::c_char = deny_list.as_mut_ptr();
        while !(*c_0).is_null() {
            if strcmp(*c_0, variable_name) == 0 as libc::c_int {
                break;
            }
            c_0 = c_0.offset(1);
        }
        if (*c_0).is_null() {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"A key '%s' was added to Lua globals which is not on the globals allow list nor listed on the deny list.\0"
                        as *const u8 as *const libc::c_char,
                    variable_name,
                );
            }
        }
    } else {
        lua_rawset(lua, -(3 as libc::c_int));
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaSetAllowListProtection(mut lua: *mut lua_State) {
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushcclosure(
        lua,
        Some(
            luaNewIndexAllowList as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_setfield(
        lua,
        -(2 as libc::c_int),
        b"__newindex\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(lua, -(2 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn luaSetTableProtectionRecursively(mut lua: *mut lua_State) {
    if lua_isreadonlytable(lua, -(1 as libc::c_int)) != 0 {
        return;
    }
    lua_enablereadonlytable(lua, -(1 as libc::c_int), 1 as libc::c_int);
    lua_checkstack(lua, 2 as libc::c_int);
    lua_pushnil(lua);
    while lua_next(lua, -(2 as libc::c_int)) != 0 {
        if lua_type(lua, -(1 as libc::c_int)) == 5 as libc::c_int {
            luaSetTableProtectionRecursively(lua);
        }
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    if lua_getmetatable(lua, -(1 as libc::c_int)) != 0 {
        luaSetTableProtectionRecursively(lua);
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaRegisterVersion(mut lua: *mut lua_State) {
    lua_pushstring(lua, b"REDIS_VERSION_NUM\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 0x70008 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"REDIS_VERSION\0" as *const u8 as *const libc::c_char);
    lua_pushstring(lua, b"7.0.8\0" as *const u8 as *const libc::c_char);
    lua_settable(lua, -(3 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn luaRegisterLogFunction(mut lua: *mut lua_State) {
    lua_pushstring(lua, b"log\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(luaLogCommand as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"LOG_DEBUG\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 0 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"LOG_VERBOSE\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 1 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"LOG_NOTICE\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 2 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"LOG_WARNING\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 3 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn luaRegisterRedisAPI(mut lua: *mut lua_State) {
    lua_pushvalue(lua, -(10002 as libc::c_int));
    luaSetAllowListProtection(lua);
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    luaLoadLibraries(lua);
    lua_pushcclosure(
        lua,
        Some(luaRedisPcall as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        lua,
        -(10002 as libc::c_int),
        b"pcall\0" as *const u8 as *const libc::c_char,
    );
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(lua, b"call\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(luaRedisCallCommand as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"pcall\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisPCallCommand as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    luaRegisterLogFunction(lua);
    luaRegisterVersion(lua);
    lua_pushstring(lua, b"setresp\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(luaSetResp as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"sha1hex\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisSha1hexCommand as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"error_reply\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisErrorReplyCommand
                as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"status_reply\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisStatusReplyCommand
                as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"set_repl\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisSetReplCommand as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"REPL_NONE\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 0 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"REPL_AOF\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 1 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"REPL_SLAVE\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 2 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"REPL_REPLICA\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, 2 as libc::c_int as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"REPL_ALL\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(lua, (1 as libc::c_int | 2 as libc::c_int) as lua_Number);
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"acl_check_cmd\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            luaRedisAclCheckCmdPermissionsCommand
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
    lua_getfield(
        lua,
        -(10002 as libc::c_int),
        b"math\0" as *const u8 as *const libc::c_char,
    );
    lua_pushstring(lua, b"random\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(redis_math_random as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_pushstring(lua, b"randomseed\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        lua,
        Some(
            redis_math_randomseed as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
        ),
        0 as libc::c_int,
    );
    lua_settable(lua, -(3 as libc::c_int));
    lua_setfield(
        lua,
        -(10002 as libc::c_int),
        b"math\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn luaCreateArray(
    mut lua: *mut lua_State,
    mut elev: *mut *mut robj,
    mut elec: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    lua_createtable(lua, 0 as libc::c_int, 0 as libc::c_int);
    j = 0 as libc::c_int;
    while j < elec {
        lua_pushlstring(
            lua,
            (**elev.offset(j as isize)).ptr as *mut libc::c_char,
            sdslen((**elev.offset(j as isize)).ptr as sds),
        );
        lua_rawseti(lua, -(2 as libc::c_int), j + 1 as libc::c_int);
        j += 1;
    }
}
unsafe extern "C" fn redis_math_random(mut L: *mut lua_State) -> libc::c_int {
    let mut rctx: *mut scriptRunCtx = luaGetFromRegistry(
        L,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut scriptRunCtx;
    if rctx.is_null() {
        return luaL_error(
            L,
            b"math.random can only be called inside a script invocation\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut r: lua_Number = (redisLrand48() % 2147483647 as libc::c_int) as lua_Number
        / 2147483647 as libc::c_int as lua_Number;
    match lua_gettop(L) {
        0 => {
            lua_pushnumber(L, r);
        }
        1 => {
            let mut u: libc::c_int = luaL_checkinteger(L, 1 as libc::c_int)
                as libc::c_int;
            (1 as libc::c_int <= u
                || luaL_argerror(
                    L,
                    1 as libc::c_int,
                    b"interval is empty\0" as *const u8 as *const libc::c_char,
                ) != 0) as libc::c_int;
            lua_pushnumber(
                L,
                floor(r * u as libc::c_double) + 1 as libc::c_int as libc::c_double,
            );
        }
        2 => {
            let mut l: libc::c_int = luaL_checkinteger(L, 1 as libc::c_int)
                as libc::c_int;
            let mut u_0: libc::c_int = luaL_checkinteger(L, 2 as libc::c_int)
                as libc::c_int;
            (l <= u_0
                || luaL_argerror(
                    L,
                    2 as libc::c_int,
                    b"interval is empty\0" as *const u8 as *const libc::c_char,
                ) != 0) as libc::c_int;
            lua_pushnumber(
                L,
                floor(r * (u_0 - l + 1 as libc::c_int) as libc::c_double)
                    + l as libc::c_double,
            );
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn redis_math_randomseed(mut L: *mut lua_State) -> libc::c_int {
    let mut rctx: *mut scriptRunCtx = luaGetFromRegistry(
        L,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut scriptRunCtx;
    if rctx.is_null() {
        return luaL_error(
            L,
            b"math.randomseed can only be called inside a script invocation\0"
                as *const u8 as *const libc::c_char,
        );
    }
    redisSrand48(luaL_checkinteger(L, 1 as libc::c_int) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaMaskCountHook(mut lua: *mut lua_State, mut ar: *mut lua_Debug) {
    let mut rctx: *mut scriptRunCtx = luaGetFromRegistry(
        lua,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut scriptRunCtx;
    if scriptInterrupt(rctx) == 1 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Lua script killed by user with SCRIPT KILL.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        lua_sethook(
            lua,
            Some(
                luaMaskCountHook
                    as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> (),
            ),
            (1 as libc::c_int) << 2 as libc::c_int,
            0 as libc::c_int,
        );
        luaPushError(
            lua,
            b"Script killed by user with SCRIPT KILL...\0" as *const u8
                as *const libc::c_char,
        );
        luaError(lua);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaErrorInformationDiscard(mut err_info: *mut errorInfo) {
    if !((*err_info).msg).is_null() {
        sdsfree((*err_info).msg);
    }
    if !((*err_info).source).is_null() {
        sdsfree((*err_info).source);
    }
    if !((*err_info).line).is_null() {
        sdsfree((*err_info).line);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaExtractErrorInformation(
    mut lua: *mut lua_State,
    mut err_info: *mut errorInfo,
) {
    if lua_isstring(lua, -(1 as libc::c_int)) != 0 {
        (*err_info)
            .msg = sdscatfmt(
            sdsempty(),
            b"ERR %s\0" as *const u8 as *const libc::c_char,
            lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t),
        );
        (*err_info).line = 0 as sds;
        (*err_info).source = 0 as sds;
        (*err_info).ignore_err_stats_update = 0 as libc::c_int;
    }
    lua_getfield(lua, -(1 as libc::c_int), b"err\0" as *const u8 as *const libc::c_char);
    if lua_isstring(lua, -(1 as libc::c_int)) != 0 {
        (*err_info)
            .msg = sdsnew(lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t));
    }
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_getfield(
        lua,
        -(1 as libc::c_int),
        b"source\0" as *const u8 as *const libc::c_char,
    );
    if lua_isstring(lua, -(1 as libc::c_int)) != 0 {
        (*err_info)
            .source = sdsnew(lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t));
    }
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_getfield(
        lua,
        -(1 as libc::c_int),
        b"line\0" as *const u8 as *const libc::c_char,
    );
    if lua_isstring(lua, -(1 as libc::c_int)) != 0 {
        (*err_info)
            .line = sdsnew(lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t));
    }
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_getfield(
        lua,
        -(1 as libc::c_int),
        b"ignore_error_stats_update\0" as *const u8 as *const libc::c_char,
    );
    if lua_type(lua, -(1 as libc::c_int)) == 1 as libc::c_int {
        (*err_info).ignore_err_stats_update = lua_toboolean(lua, -(1 as libc::c_int));
    }
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaCallFunction(
    mut run_ctx: *mut scriptRunCtx,
    mut lua: *mut lua_State,
    mut keys: *mut *mut robj,
    mut nkeys: size_t,
    mut args: *mut *mut robj,
    mut nargs: size_t,
    mut debug_enabled: libc::c_int,
) {
    let mut c: *mut client = (*run_ctx).original_client;
    let mut delhook: libc::c_int = 0 as libc::c_int;
    luaSaveOnRegistry(
        lua,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
        run_ctx as *mut libc::c_void,
    );
    if server.busy_reply_threshold > 0 as libc::c_int as libc::c_longlong
        && debug_enabled == 0
    {
        lua_sethook(
            lua,
            Some(
                luaMaskCountHook
                    as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> (),
            ),
            (1 as libc::c_int) << 3 as libc::c_int,
            100000 as libc::c_int,
        );
        delhook = 1 as libc::c_int;
    } else if debug_enabled != 0 {
        lua_sethook(
            lua,
            Some(
                luaLdbLineHook
                    as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> (),
            ),
            (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
            100000 as libc::c_int,
        );
        delhook = 1 as libc::c_int;
    }
    luaCreateArray(lua, keys, nkeys as libc::c_int);
    if (*run_ctx).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0
    {
        lua_enablereadonlytable(lua, -(10002 as libc::c_int), 0 as libc::c_int);
        lua_setfield(
            lua,
            -(10002 as libc::c_int),
            b"KEYS\0" as *const u8 as *const libc::c_char,
        );
        lua_enablereadonlytable(lua, -(10002 as libc::c_int), 1 as libc::c_int);
    }
    luaCreateArray(lua, args, nargs as libc::c_int);
    if (*run_ctx).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0
    {
        lua_enablereadonlytable(lua, -(10002 as libc::c_int), 0 as libc::c_int);
        lua_setfield(
            lua,
            -(10002 as libc::c_int),
            b"ARGV\0" as *const u8 as *const libc::c_char,
        );
        lua_enablereadonlytable(lua, -(10002 as libc::c_int), 1 as libc::c_int);
    }
    let mut err: libc::c_int = 0;
    if (*run_ctx).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0
    {
        err = lua_pcall(lua, 0 as libc::c_int, 1 as libc::c_int, -(2 as libc::c_int));
    } else {
        err = lua_pcall(lua, 2 as libc::c_int, 1 as libc::c_int, -(4 as libc::c_int));
    }
    static mut gc_count: libc::c_long = 0 as libc::c_int as libc::c_long;
    gc_count += 1;
    if gc_count == 50 as libc::c_int as libc::c_long {
        lua_gc(lua, 5 as libc::c_int, 50 as libc::c_int);
        gc_count = 0 as libc::c_int as libc::c_long;
    }
    if err != 0 {
        if !(lua_type(lua, -(1 as libc::c_int)) == 5 as libc::c_int) {
            let mut msg: *const libc::c_char = b"execution failure\0" as *const u8
                as *const libc::c_char;
            if lua_isstring(lua, -(1 as libc::c_int)) != 0 {
                msg = lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t);
            }
            addReplyErrorFormat(
                c,
                b"Error running script %s, %.100s\n\0" as *const u8
                    as *const libc::c_char,
                (*run_ctx).funcname,
                msg,
            );
        } else {
            let mut err_info: errorInfo = {
                let mut init = errorInfo {
                    msg: 0 as sds,
                    source: 0 as *mut libc::c_char,
                    line: 0 as *mut libc::c_char,
                    ignore_err_stats_update: 0,
                };
                init
            };
            let mut final_msg: sds = sdsempty();
            luaExtractErrorInformation(lua, &mut err_info);
            final_msg = sdscatfmt(
                final_msg,
                b"-%s\0" as *const u8 as *const libc::c_char,
                err_info.msg,
            );
            if !(err_info.line).is_null() && !(err_info.source).is_null() {
                final_msg = sdscatfmt(
                    final_msg,
                    b" script: %s, on %s:%s.\0" as *const u8 as *const libc::c_char,
                    (*run_ctx).funcname,
                    err_info.source,
                    err_info.line,
                );
            }
            addReplyErrorSdsEx(
                c,
                final_msg,
                (if err_info.ignore_err_stats_update != 0 {
                    (1 as libc::c_ulonglong) << 0 as libc::c_int
                } else {
                    0 as libc::c_int as libc::c_ulonglong
                }) as libc::c_int,
            );
            luaErrorInformationDiscard(&mut err_info);
        }
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    } else {
        luaReplyToRedisReply(c, (*run_ctx).c, lua);
    }
    if delhook != 0 {
        lua_sethook(lua, None, 0 as libc::c_int, 0 as libc::c_int);
    }
    luaSaveOnRegistry(
        lua,
        b"__RUN_CTX__\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaMemory(mut lua: *mut lua_State) -> libc::c_ulong {
    return (lua_gc(lua, 3 as libc::c_int, 0 as libc::c_int) as libc::c_longlong
        * 1024 as libc::c_longlong) as libc::c_ulong;
}
