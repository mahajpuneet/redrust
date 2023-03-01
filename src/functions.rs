extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn dictUnlink(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFreeUnlinkedEntry(d: *mut dict, he: *mut dictEntry);
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFetchValue(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn rioInitWithBuffer(r: *mut rio, s: sds);
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn dictEmpty(
        d: *mut dict,
        callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
    );
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn stringmatchlen(
        p: *const libc::c_char,
        plen: libc::c_int,
        s: *const libc::c_char,
        slen: libc::c_int,
        nocase: libc::c_int,
    ) -> libc::c_int;
    fn crc64(crc: uint64_t, s: *const libc::c_uchar, l: uint64_t) -> uint64_t;
    fn luaEngineInitEngine() -> libc::c_int;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn createClient(conn: *mut connection) -> *mut client;
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn addReplyNull(c: *mut client);
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyErrorSds(c: *mut client, err: sds);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyStatus(c: *mut client, status: *const libc::c_char);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplySetLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn sdsZmallocSize(s: sds) -> size_t;
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn getLongLongFromObject(o: *mut robj, target: *mut libc::c_longlong) -> libc::c_int;
    fn replicationFeedMonitors(
        c: *mut client,
        monitors: *mut list,
        dictid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
    );
    fn rdbLoadType(rdb: *mut rio) -> libc::c_int;
    fn rdbFunctionLoad(
        rdb: *mut rio,
        ver: libc::c_int,
        lib_ctx: *mut functionsLibCtx,
        type_0: libc::c_int,
        rdbflags: libc::c_int,
        err: *mut sds,
    ) -> libc::c_int;
    fn rdbSaveFunctions(rdb: *mut rio) -> ssize_t;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn freeFunctionsAsync(lib_ctx: *mut functionsLibCtx);
    fn scriptFlagsToCmdFlags(cmd_flags: uint64_t, script_flags: uint64_t) -> uint64_t;
    fn dictSdsHash(key: *const libc::c_void) -> uint64_t;
    fn dictSdsCaseHash(key: *const libc::c_void) -> uint64_t;
    fn dictSdsKeyCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn dictSdsKeyCaseCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn dictSdsDestructor(d: *mut dict, val: *mut libc::c_void);
    fn dictSdsDup(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn verifyDumpPayload(
        p: *mut libc::c_uchar,
        len: size_t,
        rdbver_ptr: *mut uint16_t,
    ) -> libc::c_int;
    fn scriptResetRun(r_ctx: *mut scriptRunCtx);
    fn scriptPrepareForRun(
        r_ctx: *mut scriptRunCtx,
        engine_client: *mut client,
        caller: *mut client,
        funcname: *const libc::c_char,
        script_flags: uint64_t,
        ro: libc::c_int,
    ) -> libc::c_int;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn scriptKill(c: *mut client, is_eval: libc::c_int);
    fn scriptRunDuration() -> libc::c_longlong;
    fn scriptGetCaller() -> *mut client;
    fn scriptCurrFunction() -> *const libc::c_char;
    fn scriptIsRunning() -> libc::c_int;
    fn scriptIsEval() -> libc::c_int;
    static mut scripts_flags_def: [scriptFlag; 0];
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
pub type __off_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rio {
    pub read: Option::<
        unsafe extern "C" fn(*mut _rio, *mut libc::c_void, size_t) -> size_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut _rio, *const libc::c_void, size_t) -> size_t,
    >,
    pub tell: Option::<unsafe extern "C" fn(*mut _rio) -> off_t>,
    pub flush: Option::<unsafe extern "C" fn(*mut _rio) -> libc::c_int>,
    pub update_cksum: Option::<
        unsafe extern "C" fn(*mut _rio, *const libc::c_void, size_t) -> (),
    >,
    pub cksum: uint64_t,
    pub flags: uint64_t,
    pub processed_bytes: size_t,
    pub max_processing_chunk: size_t,
    pub io: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub buffer: C2RustUnnamed_3,
    pub file: C2RustUnnamed_2,
    pub conn: C2RustUnnamed_1,
    pub fd: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub fd: libc::c_int,
    pub pos: off_t,
    pub buf: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub conn: *mut connection,
    pub pos: off_t,
    pub buf: sds,
    pub read_limit: size_t,
    pub read_so_far: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fp: *mut FILE,
    pub buffered: off_t,
    pub autosync: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ptr: sds,
    pub pos: off_t,
}
pub type rio = _rio;
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
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
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
    pub v: C2RustUnnamed_4,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
pub struct functionsLibCtx {
    pub libraries: *mut dict,
    pub functions: *mut dict,
    pub cache_memory: size_t,
    pub engines_stats: *mut dict,
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
    pub bs: C2RustUnnamed_8,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub range: C2RustUnnamed_7,
    pub keynum: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub union C2RustUnnamed_8 {
    pub index: C2RustUnnamed_10,
    pub keyword: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub inst_metric: [C2RustUnnamed_11; 5],
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
pub struct C2RustUnnamed_11 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct functionInfo {
    pub name: sds,
    pub function: *mut libc::c_void,
    pub li: *mut functionLibInfo,
    pub desc: sds,
    pub f_flags: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct functionLibInfo {
    pub name: sds,
    pub functions: *mut dict,
    pub ei: *mut engineInfo,
    pub code: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct engineInfo {
    pub name: sds,
    pub engine: *mut engine,
    pub c: *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct engine {
    pub engine_ctx: *mut libc::c_void,
    pub create: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut functionLibInfo,
            sds,
            *mut sds,
        ) -> libc::c_int,
    >,
    pub call: Option::<
        unsafe extern "C" fn(
            *mut scriptRunCtx,
            *mut libc::c_void,
            *mut libc::c_void,
            *mut *mut robj,
            size_t,
            *mut *mut robj,
            size_t,
        ) -> (),
    >,
    pub get_used_memory: Option::<unsafe extern "C" fn(*mut libc::c_void) -> size_t>,
    pub get_function_memory_overhead: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> size_t,
    >,
    pub get_engine_memory_overhead: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> size_t,
    >,
    pub free_function: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
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
pub struct functionsLibMataData {
    pub engine: sds,
    pub name: sds,
    pub code: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct functionsLibEngineStats {
    pub n_lib: size_t,
    pub n_functions: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scriptFlag {
    pub flag: uint64_t,
    pub str_0: *const libc::c_char,
}
pub const restorePolicy_Replace: restorePolicy = 2;
pub type restorePolicy = libc::c_uint;
pub const restorePolicy_Append: restorePolicy = 1;
pub const restorePolicy_Flush: restorePolicy = 0;
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
static mut engine_cache_memory: size_t = 0 as libc::c_int as size_t;
#[no_mangle]
pub static mut engineDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsCaseHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: Some(
                dictSdsDup
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut functionDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsCaseHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: Some(
                dictSdsDup
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut engineStatsDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsCaseHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: Some(
                dictSdsDup
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
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
                engineStatsDispose
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut libraryFunctionDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: Some(
                dictSdsDup
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
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
                engineFunctionDispose
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut librariesDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: Some(
                dictSdsDup
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
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
                engineLibraryDispose
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
static mut engines: *mut dict = 0 as *const dict as *mut dict;
static mut curr_functions_lib_ctx: *mut functionsLibCtx = 0 as *const functionsLibCtx
    as *mut functionsLibCtx;
unsafe extern "C" fn functionMallocSize(mut fi: *mut functionInfo) -> size_t {
    return (je_malloc_usable_size(fi as *mut libc::c_void))
        .wrapping_add(sdsZmallocSize((*fi).name))
        .wrapping_add(
            (if !((*fi).desc).is_null() {
                sdsZmallocSize((*fi).desc)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        )
        .wrapping_add(
            ((*(*(*(*fi).li).ei).engine).get_function_memory_overhead)
                .expect("non-null function pointer")((*fi).function),
        );
}
unsafe extern "C" fn libraryMallocSize(mut li: *mut functionLibInfo) -> size_t {
    return (je_malloc_usable_size(li as *mut libc::c_void))
        .wrapping_add(sdsZmallocSize((*li).name))
        .wrapping_add(sdsZmallocSize((*li).code));
}
unsafe extern "C" fn engineStatsDispose(mut d: *mut dict, mut obj: *mut libc::c_void) {
    let mut stats: *mut functionsLibEngineStats = obj as *mut functionsLibEngineStats;
    zfree(stats as *mut libc::c_void);
}
unsafe extern "C" fn engineFunctionDispose(
    mut d: *mut dict,
    mut obj: *mut libc::c_void,
) {
    if obj.is_null() {
        return;
    }
    let mut fi: *mut functionInfo = obj as *mut functionInfo;
    sdsfree((*fi).name);
    if !((*fi).desc).is_null() {
        sdsfree((*fi).desc);
    }
    let mut engine: *mut engine = (*(*(*fi).li).ei).engine;
    ((*engine).free_function)
        .expect("non-null function pointer")((*engine).engine_ctx, (*fi).function);
    zfree(fi as *mut libc::c_void);
}
unsafe extern "C" fn engineLibraryFree(mut li: *mut functionLibInfo) {
    if li.is_null() {
        return;
    }
    dictRelease((*li).functions);
    sdsfree((*li).name);
    sdsfree((*li).code);
    zfree(li as *mut libc::c_void);
}
unsafe extern "C" fn engineLibraryDispose(mut d: *mut dict, mut obj: *mut libc::c_void) {
    engineLibraryFree(obj as *mut functionLibInfo);
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibCtxClear(mut lib_ctx: *mut functionsLibCtx) {
    dictEmpty((*lib_ctx).functions, None);
    dictEmpty((*lib_ctx).libraries, None);
    let mut iter: *mut dictIterator = dictGetIterator((*lib_ctx).engines_stats);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            break;
        }
        let mut stats: *mut functionsLibEngineStats = (*entry).v.val
            as *mut functionsLibEngineStats;
        (*stats).n_functions = 0 as libc::c_int as size_t;
        (*stats).n_lib = 0 as libc::c_int as size_t;
    }
    dictReleaseIterator(iter);
    (*curr_functions_lib_ctx).cache_memory = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibCtxClearCurrent(mut async_0: libc::c_int) {
    if async_0 != 0 {
        let mut old_l_ctx: *mut functionsLibCtx = curr_functions_lib_ctx;
        curr_functions_lib_ctx = functionsLibCtxCreate();
        freeFunctionsAsync(old_l_ctx);
    } else {
        functionsLibCtxClear(curr_functions_lib_ctx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibCtxFree(
    mut functions_lib_ctx: *mut functionsLibCtx,
) {
    functionsLibCtxClear(functions_lib_ctx);
    dictRelease((*functions_lib_ctx).functions);
    dictRelease((*functions_lib_ctx).libraries);
    dictRelease((*functions_lib_ctx).engines_stats);
    zfree(functions_lib_ctx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibCtxSwapWithCurrent(
    mut new_lib_ctx: *mut functionsLibCtx,
) {
    functionsLibCtxFree(curr_functions_lib_ctx);
    curr_functions_lib_ctx = new_lib_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibCtxGetCurrent() -> *mut functionsLibCtx {
    return curr_functions_lib_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibCtxCreate() -> *mut functionsLibCtx {
    let mut ret: *mut functionsLibCtx = zmalloc(
        core::mem::size_of::<functionsLibCtx>() as libc::c_ulong,
    ) as *mut functionsLibCtx;
    (*ret).libraries = dictCreate(&mut librariesDictType);
    (*ret).functions = dictCreate(&mut functionDictType);
    (*ret).engines_stats = dictCreate(&mut engineStatsDictType);
    let mut iter: *mut dictIterator = dictGetIterator(engines);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            break;
        }
        let mut ei: *mut engineInfo = (*entry).v.val as *mut engineInfo;
        let mut stats: *mut functionsLibEngineStats = zcalloc(
            core::mem::size_of::<functionsLibEngineStats>() as libc::c_ulong,
        ) as *mut functionsLibEngineStats;
        dictAdd(
            (*ret).engines_stats,
            (*ei).name as *mut libc::c_void,
            stats as *mut libc::c_void,
        );
    }
    dictReleaseIterator(iter);
    (*ret).cache_memory = 0 as libc::c_int as size_t;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn functionLibCreateFunction(
    mut name: sds,
    mut function: *mut libc::c_void,
    mut li: *mut functionLibInfo,
    mut desc: sds,
    mut f_flags: uint64_t,
    mut err: *mut sds,
) -> libc::c_int {
    if functionsVerifyName(name) != 0 as libc::c_int {
        *err = sdsnew(
            b"Library names can only contain letters, numbers, or underscores(_) and must be at least one character long\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !(dictFetchValue((*li).functions, name as *const libc::c_void)).is_null() {
        *err = sdsnew(
            b"Function already exists in the library\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut fi: *mut functionInfo = zmalloc(
        core::mem::size_of::<functionInfo>() as libc::c_ulong,
    ) as *mut functionInfo;
    *fi = {
        let mut init = functionInfo {
            name: name,
            function: function,
            li: li,
            desc: desc,
            f_flags: f_flags,
        };
        init
    };
    let mut res: libc::c_int = dictAdd(
        (*li).functions,
        (*fi).name as *mut libc::c_void,
        fi as *mut libc::c_void,
    );
    if res == 0 as libc::c_int {} else {
        _serverAssert(
            b"res == DICT_OK\0" as *const u8 as *const libc::c_char,
            b"functions.c\0" as *const u8 as *const libc::c_char,
            267 as libc::c_int,
        );
        unreachable!();
    };
    return 0 as libc::c_int;
}
unsafe extern "C" fn engineLibraryCreate(
    mut name: sds,
    mut ei: *mut engineInfo,
    mut code: sds,
) -> *mut functionLibInfo {
    let mut li: *mut functionLibInfo = zmalloc(
        core::mem::size_of::<functionLibInfo>() as libc::c_ulong,
    ) as *mut functionLibInfo;
    *li = {
        let mut init = functionLibInfo {
            name: sdsdup(name),
            functions: dictCreate(&mut libraryFunctionDictType),
            ei: ei,
            code: sdsdup(code),
        };
        init
    };
    return li;
}
unsafe extern "C" fn libraryUnlink(
    mut lib_ctx: *mut functionsLibCtx,
    mut li: *mut functionLibInfo,
) {
    let mut iter: *mut dictIterator = dictGetIterator((*li).functions);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            break;
        }
        let mut fi: *mut functionInfo = (*entry).v.val as *mut functionInfo;
        let mut ret: libc::c_int = dictDelete(
            (*lib_ctx).functions,
            (*fi).name as *const libc::c_void,
        );
        if ret == 0 as libc::c_int {} else {
            _serverAssert(
                b"ret == DICT_OK\0" as *const u8 as *const libc::c_char,
                b"functions.c\0" as *const u8 as *const libc::c_char,
                289 as libc::c_int,
            );
            unreachable!();
        };
        (*lib_ctx)
            .cache_memory = ((*lib_ctx).cache_memory as libc::c_ulong)
            .wrapping_sub(functionMallocSize(fi)) as size_t as size_t;
    }
    dictReleaseIterator(iter);
    entry = dictUnlink((*lib_ctx).libraries, (*li).name as *const libc::c_void);
    if ((*(*(*lib_ctx).libraries).type_0).valDup).is_some() {
        (*entry)
            .v
            .val = ((*(*(*lib_ctx).libraries).type_0).valDup)
            .expect(
                "non-null function pointer",
            )((*lib_ctx).libraries, 0 as *const libc::c_void);
    } else {
        (*entry).v.val = 0 as *mut libc::c_void;
    }
    dictFreeUnlinkedEntry((*lib_ctx).libraries, entry);
    (*lib_ctx)
        .cache_memory = ((*lib_ctx).cache_memory as libc::c_ulong)
        .wrapping_sub(libraryMallocSize(li)) as size_t as size_t;
    let mut stats: *mut functionsLibEngineStats = dictFetchValue(
        (*lib_ctx).engines_stats,
        (*(*li).ei).name as *const libc::c_void,
    ) as *mut functionsLibEngineStats;
    if !stats.is_null() {} else {
        _serverAssert(
            b"stats\0" as *const u8 as *const libc::c_char,
            b"functions.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int,
        );
        unreachable!();
    };
    (*stats).n_lib = ((*stats).n_lib).wrapping_sub(1);
    (*stats)
        .n_functions = ((*stats).n_functions as libc::c_ulong)
        .wrapping_sub(
            ((*(*li).functions).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*li).functions).ht_used[1 as libc::c_int as usize]),
        ) as size_t as size_t;
}
unsafe extern "C" fn libraryLink(
    mut lib_ctx: *mut functionsLibCtx,
    mut li: *mut functionLibInfo,
) {
    let mut iter: *mut dictIterator = dictGetIterator((*li).functions);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            break;
        }
        let mut fi: *mut functionInfo = (*entry).v.val as *mut functionInfo;
        dictAdd(
            (*lib_ctx).functions,
            (*fi).name as *mut libc::c_void,
            fi as *mut libc::c_void,
        );
        (*lib_ctx)
            .cache_memory = ((*lib_ctx).cache_memory as libc::c_ulong)
            .wrapping_add(functionMallocSize(fi)) as size_t as size_t;
    }
    dictReleaseIterator(iter);
    dictAdd(
        (*lib_ctx).libraries,
        (*li).name as *mut libc::c_void,
        li as *mut libc::c_void,
    );
    (*lib_ctx)
        .cache_memory = ((*lib_ctx).cache_memory as libc::c_ulong)
        .wrapping_add(libraryMallocSize(li)) as size_t as size_t;
    let mut stats: *mut functionsLibEngineStats = dictFetchValue(
        (*lib_ctx).engines_stats,
        (*(*li).ei).name as *const libc::c_void,
    ) as *mut functionsLibEngineStats;
    if !stats.is_null() {} else {
        _serverAssert(
            b"stats\0" as *const u8 as *const libc::c_char,
            b"functions.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int,
        );
        unreachable!();
    };
    (*stats).n_lib = ((*stats).n_lib).wrapping_add(1);
    (*stats)
        .n_functions = ((*stats).n_functions as libc::c_ulong)
        .wrapping_add(
            ((*(*li).functions).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*li).functions).ht_used[1 as libc::c_int as usize]),
        ) as size_t as size_t;
}
unsafe extern "C" fn libraryJoin(
    mut functions_lib_ctx_dst: *mut functionsLibCtx,
    mut functions_lib_ctx_src: *mut functionsLibCtx,
    mut replace: libc::c_int,
    mut err: *mut sds,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut iter: *mut dictIterator = 0 as *mut dictIterator;
    let mut old_libraries_list: *mut list = 0 as *mut list;
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    iter = dictGetIterator((*functions_lib_ctx_src).libraries);
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            current_block = 11050875288958768710;
            break;
        }
        let mut li: *mut functionLibInfo = (*entry).v.val as *mut functionLibInfo;
        let mut old_li: *mut functionLibInfo = dictFetchValue(
            (*functions_lib_ctx_dst).libraries,
            (*li).name as *const libc::c_void,
        ) as *mut functionLibInfo;
        if old_li.is_null() {
            continue;
        }
        if replace == 0 {
            *err = sdscatfmt(
                sdsempty(),
                b"Library %s already exists\0" as *const u8 as *const libc::c_char,
                (*li).name,
            );
            current_block = 12349063433418891401;
            break;
        } else {
            if old_libraries_list.is_null() {
                old_libraries_list = listCreate();
                (*old_libraries_list)
                    .free = core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut functionLibInfo) -> ()>,
                    Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                >(
                    Some(
                        engineLibraryFree
                            as unsafe extern "C" fn(*mut functionLibInfo) -> (),
                    ),
                );
            }
            libraryUnlink(functions_lib_ctx_dst, old_li);
            listAddNodeTail(old_libraries_list, old_li as *mut libc::c_void);
        }
    }
    match current_block {
        11050875288958768710 => {
            dictReleaseIterator(iter);
            iter = 0 as *mut dictIterator;
            iter = dictGetIterator((*functions_lib_ctx_src).functions);
            loop {
                entry = dictNext(iter);
                if entry.is_null() {
                    current_block = 6057473163062296781;
                    break;
                }
                let mut fi: *mut functionInfo = (*entry).v.val as *mut functionInfo;
                if (dictFetchValue(
                    (*functions_lib_ctx_dst).functions,
                    (*fi).name as *const libc::c_void,
                ))
                    .is_null()
                {
                    continue;
                }
                *err = sdscatfmt(
                    sdsempty(),
                    b"Function %s already exists\0" as *const u8 as *const libc::c_char,
                    (*fi).name,
                );
                current_block = 12349063433418891401;
                break;
            }
            match current_block {
                12349063433418891401 => {}
                _ => {
                    dictReleaseIterator(iter);
                    iter = 0 as *mut dictIterator;
                    iter = dictGetIterator((*functions_lib_ctx_src).libraries);
                    loop {
                        entry = dictNext(iter);
                        if entry.is_null() {
                            break;
                        }
                        let mut li_0: *mut functionLibInfo = (*entry).v.val
                            as *mut functionLibInfo;
                        libraryLink(functions_lib_ctx_dst, li_0);
                        if ((*(*(*functions_lib_ctx_src).libraries).type_0).valDup)
                            .is_some()
                        {
                            (*entry)
                                .v
                                .val = ((*(*(*functions_lib_ctx_src).libraries).type_0)
                                .valDup)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*functions_lib_ctx_src).libraries,
                                0 as *const libc::c_void,
                            );
                        } else {
                            (*entry).v.val = 0 as *mut libc::c_void;
                        }
                    }
                    dictReleaseIterator(iter);
                    iter = 0 as *mut dictIterator;
                    functionsLibCtxClear(functions_lib_ctx_src);
                    if !old_libraries_list.is_null() {
                        listRelease(old_libraries_list);
                        old_libraries_list = 0 as *mut list;
                    }
                    ret = 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    if !iter.is_null() {
        dictReleaseIterator(iter);
    }
    if !old_libraries_list.is_null() {
        while (*old_libraries_list).len > 0 as libc::c_int as libc::c_ulong {
            let mut head: *mut listNode = (*old_libraries_list).head;
            let mut li_1: *mut functionLibInfo = (*head).value as *mut functionLibInfo;
            (*head).value = 0 as *mut libc::c_void;
            libraryLink(functions_lib_ctx_dst, li_1);
            listDelNode(old_libraries_list, head);
        }
        listRelease(old_libraries_list);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn functionsRegisterEngine(
    mut engine_name: *const libc::c_char,
    mut engine: *mut engine,
) -> libc::c_int {
    let mut engine_name_sds: sds = sdsnew(engine_name);
    if !(dictFetchValue(engines, engine_name_sds as *const libc::c_void)).is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Same engine was registered twice\0" as *const u8 as *const libc::c_char,
            );
        }
        sdsfree(engine_name_sds);
        return -(1 as libc::c_int);
    }
    let mut c: *mut client = createClient(0 as *mut connection);
    (*c)
        .flags = ((*c).flags as libc::c_ulonglong
        | ((1 as libc::c_ulonglong) << 41 as libc::c_int
            | ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulonglong))
        as uint64_t;
    let mut ei: *mut engineInfo = zmalloc(
        core::mem::size_of::<engineInfo>() as libc::c_ulong,
    ) as *mut engineInfo;
    *ei = {
        let mut init = engineInfo {
            name: engine_name_sds,
            engine: engine,
            c: c,
        };
        init
    };
    dictAdd(engines, engine_name_sds as *mut libc::c_void, ei as *mut libc::c_void);
    engine_cache_memory = (engine_cache_memory as libc::c_ulong)
        .wrapping_add(
            (je_malloc_usable_size(ei as *mut libc::c_void))
                .wrapping_add(sdsZmallocSize((*ei).name))
                .wrapping_add(je_malloc_usable_size(engine as *mut libc::c_void))
                .wrapping_add(
                    ((*engine).get_engine_memory_overhead)
                        .expect("non-null function pointer")((*engine).engine_ctx),
                ),
        ) as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn functionStatsCommand(mut c: *mut client) {
    if scriptIsRunning() != 0 && scriptIsEval() != 0 {
        addReplyErrorObject(c, shared.slowevalerr);
        return;
    }
    addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
    addReplyBulkCString(c, b"running_script\0" as *const u8 as *const libc::c_char);
    if scriptIsRunning() == 0 {
        addReplyNull(c);
    } else {
        addReplyMapLen(c, 3 as libc::c_int as libc::c_long);
        addReplyBulkCString(c, b"name\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, scriptCurrFunction());
        addReplyBulkCString(c, b"command\0" as *const u8 as *const libc::c_char);
        let mut script_client: *mut client = scriptGetCaller();
        addReplyArrayLen(c, (*script_client).argc as libc::c_long);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*script_client).argc {
            addReplyBulkCBuffer(
                c,
                (**((*script_client).argv).offset(i as isize)).ptr,
                sdslen((**((*script_client).argv).offset(i as isize)).ptr as sds),
            );
            i += 1;
        }
        addReplyBulkCString(c, b"duration_ms\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, scriptRunDuration());
    }
    addReplyBulkCString(c, b"engines\0" as *const u8 as *const libc::c_char);
    addReplyMapLen(
        c,
        ((*engines).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*engines).ht_used[1 as libc::c_int as usize]) as libc::c_long,
    );
    let mut iter: *mut dictIterator = dictGetIterator(engines);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            break;
        }
        let mut ei: *mut engineInfo = (*entry).v.val as *mut engineInfo;
        addReplyBulkCString(c, (*ei).name as *const libc::c_char);
        addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
        let mut e_stats: *mut functionsLibEngineStats = dictFetchValue(
            (*curr_functions_lib_ctx).engines_stats,
            (*ei).name as *const libc::c_void,
        ) as *mut functionsLibEngineStats;
        addReplyBulkCString(c, b"libraries_count\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*e_stats).n_lib as libc::c_longlong);
        addReplyBulkCString(c, b"functions_count\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*e_stats).n_functions as libc::c_longlong);
    }
    dictReleaseIterator(iter);
}
unsafe extern "C" fn functionListReplyFlags(
    mut c: *mut client,
    mut fi: *mut functionInfo,
) {
    let mut flagcount: libc::c_int = 0 as libc::c_int;
    let mut flag: *mut scriptFlag = scripts_flags_def.as_mut_ptr();
    while !((*flag).str_0).is_null() {
        if (*fi).f_flags & (*flag).flag != 0 {
            flagcount += 1;
        }
        flag = flag.offset(1);
    }
    addReplySetLen(c, flagcount as libc::c_long);
    let mut flag_0: *mut scriptFlag = scripts_flags_def.as_mut_ptr();
    while !((*flag_0).str_0).is_null() {
        if (*fi).f_flags & (*flag_0).flag != 0 {
            addReplyStatus(c, (*flag_0).str_0);
        }
        flag_0 = flag_0.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn functionListCommand(mut c: *mut client) {
    let mut with_code: libc::c_int = 0 as libc::c_int;
    let mut library_name: sds = 0 as sds;
    let mut i: libc::c_int = 2 as libc::c_int;
    while i < (*c).argc {
        let mut next_arg: *mut robj = *((*c).argv).offset(i as isize);
        if with_code == 0
            && strcasecmp(
                (*next_arg).ptr as *const libc::c_char,
                b"withcode\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            with_code = 1 as libc::c_int;
        } else if library_name.is_null()
            && strcasecmp(
                (*next_arg).ptr as *const libc::c_char,
                b"libraryname\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            if i >= (*c).argc - 1 as libc::c_int {
                addReplyError(
                    c,
                    b"library name argument was not given\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            i += 1;
            library_name = (**((*c).argv).offset(i as isize)).ptr as sds;
        } else {
            addReplyErrorSds(
                c,
                sdscatfmt(
                    sdsempty(),
                    b"Unknown argument %s\0" as *const u8 as *const libc::c_char,
                    (*next_arg).ptr,
                ),
            );
            return;
        }
        i += 1;
    }
    let mut reply_len: size_t = 0 as libc::c_int as size_t;
    let mut len_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if !library_name.is_null() {
        len_ptr = addReplyDeferredLen(c);
    } else {
        addReplyArrayLen(
            c,
            ((*(*curr_functions_lib_ctx).libraries).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*(*curr_functions_lib_ctx).libraries)
                        .ht_used[1 as libc::c_int as usize],
                ) as libc::c_long,
        );
    }
    let mut iter: *mut dictIterator = dictGetIterator(
        (*curr_functions_lib_ctx).libraries,
    );
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            break;
        }
        let mut li: *mut functionLibInfo = (*entry).v.val as *mut functionLibInfo;
        if !library_name.is_null() {
            if stringmatchlen(
                library_name as *const libc::c_char,
                sdslen(library_name) as libc::c_int,
                (*li).name as *const libc::c_char,
                sdslen((*li).name) as libc::c_int,
                1 as libc::c_int,
            ) == 0
            {
                continue;
            }
        }
        reply_len = reply_len.wrapping_add(1);
        addReplyMapLen(
            c,
            (if with_code != 0 { 4 as libc::c_int } else { 3 as libc::c_int })
                as libc::c_long,
        );
        addReplyBulkCString(c, b"library_name\0" as *const u8 as *const libc::c_char);
        addReplyBulkCBuffer(c, (*li).name as *const libc::c_void, sdslen((*li).name));
        addReplyBulkCString(c, b"engine\0" as *const u8 as *const libc::c_char);
        addReplyBulkCBuffer(
            c,
            (*(*li).ei).name as *const libc::c_void,
            sdslen((*(*li).ei).name),
        );
        addReplyBulkCString(c, b"functions\0" as *const u8 as *const libc::c_char);
        addReplyArrayLen(
            c,
            ((*(*li).functions).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*li).functions).ht_used[1 as libc::c_int as usize])
                as libc::c_long,
        );
        let mut functions_iter: *mut dictIterator = dictGetIterator((*li).functions);
        let mut function_entry: *mut dictEntry = 0 as *mut dictEntry;
        loop {
            function_entry = dictNext(functions_iter);
            if function_entry.is_null() {
                break;
            }
            let mut fi: *mut functionInfo = (*function_entry).v.val as *mut functionInfo;
            addReplyMapLen(c, 3 as libc::c_int as libc::c_long);
            addReplyBulkCString(c, b"name\0" as *const u8 as *const libc::c_char);
            addReplyBulkCBuffer(
                c,
                (*fi).name as *const libc::c_void,
                sdslen((*fi).name),
            );
            addReplyBulkCString(c, b"description\0" as *const u8 as *const libc::c_char);
            if !((*fi).desc).is_null() {
                addReplyBulkCBuffer(
                    c,
                    (*fi).desc as *const libc::c_void,
                    sdslen((*fi).desc),
                );
            } else {
                addReplyNull(c);
            }
            addReplyBulkCString(c, b"flags\0" as *const u8 as *const libc::c_char);
            functionListReplyFlags(c, fi);
        }
        dictReleaseIterator(functions_iter);
        if with_code != 0 {
            addReplyBulkCString(
                c,
                b"library_code\0" as *const u8 as *const libc::c_char,
            );
            addReplyBulkCBuffer(
                c,
                (*li).code as *const libc::c_void,
                sdslen((*li).code),
            );
        }
    }
    dictReleaseIterator(iter);
    if !len_ptr.is_null() {
        setDeferredArrayLen(c, len_ptr, reply_len as libc::c_long);
    }
}
#[no_mangle]
pub unsafe extern "C" fn functionDeleteCommand(mut c: *mut client) {
    let mut function_name: *mut robj = *((*c).argv).offset(2 as libc::c_int as isize);
    let mut li: *mut functionLibInfo = dictFetchValue(
        (*curr_functions_lib_ctx).libraries,
        (*function_name).ptr,
    ) as *mut functionLibInfo;
    if li.is_null() {
        addReplyError(c, b"Library not found\0" as *const u8 as *const libc::c_char);
        return;
    }
    libraryUnlink(curr_functions_lib_ctx, li);
    engineLibraryFree(li);
    server.dirty += 1;
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn functionKillCommand(mut c: *mut client) {
    scriptKill(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fcallGetCommandFlags(
    mut c: *mut client,
    mut cmd_flags: uint64_t,
) -> uint64_t {
    let mut function_name: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    (*c)
        .cur_script = dictFind(
        (*curr_functions_lib_ctx).functions,
        (*function_name).ptr,
    );
    if ((*c).cur_script).is_null() {
        return cmd_flags;
    }
    let mut fi: *mut functionInfo = (*(*c).cur_script).v.val as *mut functionInfo;
    let mut script_flags: uint64_t = (*fi).f_flags;
    return scriptFlagsToCmdFlags(cmd_flags, script_flags);
}
unsafe extern "C" fn fcallCommandGeneric(mut c: *mut client, mut ro: libc::c_int) {
    replicationFeedMonitors(c, server.monitors, (*(*c).db).id, (*c).argv, (*c).argc);
    let mut function_name: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut de: *mut dictEntry = (*c).cur_script;
    if de.is_null() {
        de = dictFind((*curr_functions_lib_ctx).functions, (*function_name).ptr);
    }
    if de.is_null() {
        addReplyError(c, b"Function not found\0" as *const u8 as *const libc::c_char);
        return;
    }
    let mut fi: *mut functionInfo = (*de).v.val as *mut functionInfo;
    let mut engine: *mut engine = (*(*(*fi).li).ei).engine;
    let mut numkeys: libc::c_longlong = 0;
    if getLongLongFromObject(
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut numkeys,
    ) != 0 as libc::c_int
    {
        addReplyError(
            c,
            b"Bad number of keys provided\0" as *const u8 as *const libc::c_char,
        );
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
    let mut run_ctx: scriptRunCtx = scriptRunCtx {
        funcname: 0 as *const libc::c_char,
        c: 0 as *mut client,
        original_client: 0 as *mut client,
        flags: 0,
        repl_flags: 0,
        start_time: 0,
        snapshot_time: 0,
    };
    if scriptPrepareForRun(
        &mut run_ctx,
        (*(*(*fi).li).ei).c,
        c,
        (*fi).name as *const libc::c_char,
        (*fi).f_flags,
        ro,
    ) != 0 as libc::c_int
    {
        return;
    }
    ((*engine).call)
        .expect(
            "non-null function pointer",
        )(
        &mut run_ctx,
        (*engine).engine_ctx,
        (*fi).function,
        ((*c).argv).offset(3 as libc::c_int as isize),
        numkeys as size_t,
        ((*c).argv).offset(3 as libc::c_int as isize).offset(numkeys as isize),
        (((*c).argc - 3 as libc::c_int) as libc::c_longlong - numkeys) as size_t,
    );
    scriptResetRun(&mut run_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn fcallCommand(mut c: *mut client) {
    fcallCommandGeneric(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fcallroCommand(mut c: *mut client) {
    fcallCommandGeneric(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn functionDumpCommand(mut c: *mut client) {
    let mut buf: [libc::c_uchar; 2] = [0; 2];
    let mut crc: uint64_t = 0;
    let mut payload: rio = rio {
        read: None,
        write: None,
        tell: None,
        flush: None,
        update_cksum: None,
        cksum: 0,
        flags: 0,
        processed_bytes: 0,
        max_processing_chunk: 0,
        io: C2RustUnnamed {
            buffer: C2RustUnnamed_3 {
                ptr: 0 as *mut libc::c_char,
                pos: 0,
            },
        },
    };
    rioInitWithBuffer(&mut payload, sdsempty());
    rdbSaveFunctions(&mut payload);
    buf[0 as libc::c_int
        as usize] = (10 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    buf[1 as libc::c_int
        as usize] = (10 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    payload
        .io
        .buffer
        .ptr = sdscatlen(
        payload.io.buffer.ptr,
        buf.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    crc = crc64(
        0 as libc::c_int as uint64_t,
        payload.io.buffer.ptr as *mut libc::c_uchar,
        sdslen(payload.io.buffer.ptr),
    );
    payload
        .io
        .buffer
        .ptr = sdscatlen(
        payload.io.buffer.ptr,
        &mut crc as *mut uint64_t as *const libc::c_void,
        8 as libc::c_int as size_t,
    );
    addReplyBulkSds(c, payload.io.buffer.ptr);
}
#[no_mangle]
pub unsafe extern "C" fn functionRestoreCommand(mut c: *mut client) {
    let mut current_block: u64;
    if (*c).argc > 4 as libc::c_int {
        addReplySubcommandSyntaxError(c);
        return;
    }
    let mut restore_replicy: restorePolicy = restorePolicy_Append;
    let mut data: sds = (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds;
    let mut data_len: size_t = sdslen(data);
    let mut payload: rio = rio {
        read: None,
        write: None,
        tell: None,
        flush: None,
        update_cksum: None,
        cksum: 0,
        flags: 0,
        processed_bytes: 0,
        max_processing_chunk: 0,
        io: C2RustUnnamed {
            buffer: C2RustUnnamed_3 {
                ptr: 0 as *mut libc::c_char,
                pos: 0,
            },
        },
    };
    let mut err: sds = 0 as sds;
    if (*c).argc == 4 as libc::c_int {
        let mut restore_policy_str: *const libc::c_char = (**((*c).argv)
            .offset(3 as libc::c_int as isize))
            .ptr as *const libc::c_char;
        if strcasecmp(
            restore_policy_str,
            b"append\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            restore_replicy = restorePolicy_Append;
        } else if strcasecmp(
            restore_policy_str,
            b"replace\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            restore_replicy = restorePolicy_Replace;
        } else if strcasecmp(
            restore_policy_str,
            b"flush\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            restore_replicy = restorePolicy_Flush;
        } else {
            addReplyError(
                c,
                b"Wrong restore policy given, value should be either FLUSH, APPEND or REPLACE.\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    let mut rdbver: uint16_t = 0;
    if verifyDumpPayload(data as *mut libc::c_uchar, data_len, &mut rdbver)
        != 0 as libc::c_int
    {
        addReplyError(
            c,
            b"DUMP payload version or checksum are wrong\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    let mut functions_lib_ctx: *mut functionsLibCtx = functionsLibCtxCreate();
    rioInitWithBuffer(&mut payload, data);
    loop {
        if !(data_len.wrapping_sub(payload.io.buffer.pos as libc::c_ulong)
            > 10 as libc::c_int as libc::c_ulong)
        {
            current_block = 5689316957504528238;
            break;
        }
        let mut type_0: libc::c_int = 0;
        type_0 = rdbLoadType(&mut payload);
        if type_0 == -(1 as libc::c_int) {
            err = sdsnew(
                b"can not read data type\0" as *const u8 as *const libc::c_char,
            );
            current_block = 4908586452986928802;
            break;
        } else if type_0 != 246 as libc::c_int && type_0 != 245 as libc::c_int {
            err = sdsnew(
                b"given type is not a function\0" as *const u8 as *const libc::c_char,
            );
            current_block = 4908586452986928802;
            break;
        } else {
            if !(rdbFunctionLoad(
                &mut payload,
                rdbver as libc::c_int,
                functions_lib_ctx,
                type_0,
                0 as libc::c_int,
                &mut err,
            ) != 0 as libc::c_int)
            {
                continue;
            }
            if err.is_null() {
                err = sdsnew(
                    b"failed loading the given functions payload\0" as *const u8
                        as *const libc::c_char,
                );
            }
            current_block = 4908586452986928802;
            break;
        }
    }
    match current_block {
        5689316957504528238 => {
            if restore_replicy as libc::c_uint
                == restorePolicy_Flush as libc::c_int as libc::c_uint
            {
                functionsLibCtxSwapWithCurrent(functions_lib_ctx);
                functions_lib_ctx = 0 as *mut functionsLibCtx;
                current_block = 9520865839495247062;
            } else if libraryJoin(
                curr_functions_lib_ctx,
                functions_lib_ctx,
                (restore_replicy as libc::c_uint
                    == restorePolicy_Replace as libc::c_int as libc::c_uint)
                    as libc::c_int,
                &mut err,
            ) != 0 as libc::c_int
            {
                current_block = 4908586452986928802;
            } else {
                current_block = 9520865839495247062;
            }
            match current_block {
                4908586452986928802 => {}
                _ => {
                    server.dirty += 1;
                }
            }
        }
        _ => {}
    }
    if !err.is_null() {
        addReplyErrorSds(c, err);
    } else {
        addReply(c, shared.ok);
    }
    if !functions_lib_ctx.is_null() {
        functionsLibCtxFree(functions_lib_ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn functionFlushCommand(mut c: *mut client) {
    if (*c).argc > 3 as libc::c_int {
        addReplySubcommandSyntaxError(c);
        return;
    }
    let mut async_0: libc::c_int = 0 as libc::c_int;
    if (*c).argc == 3 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"sync\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        async_0 = 0 as libc::c_int;
    } else if (*c).argc == 3 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
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
            b"FUNCTION FLUSH only supports SYNC|ASYNC option\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    functionsLibCtxClearCurrent(async_0);
    server.dirty += 1;
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn functionHelpCommand(mut c: *mut client) {
    let mut help: [*const libc::c_char; 38] = [
        b"LOAD [REPLACE] <FUNCTION CODE>\0" as *const u8 as *const libc::c_char,
        b"    Create a new library with the given library name and code.\0" as *const u8
            as *const libc::c_char,
        b"DELETE <LIBRARY NAME>\0" as *const u8 as *const libc::c_char,
        b"    Delete the given library.\0" as *const u8 as *const libc::c_char,
        b"LIST [LIBRARYNAME PATTERN] [WITHCODE]\0" as *const u8 as *const libc::c_char,
        b"    Return general information on all the libraries:\0" as *const u8
            as *const libc::c_char,
        b"    * Library name\0" as *const u8 as *const libc::c_char,
        b"    * The engine used to run the Library\0" as *const u8
            as *const libc::c_char,
        b"    * Library description\0" as *const u8 as *const libc::c_char,
        b"    * Functions list\0" as *const u8 as *const libc::c_char,
        b"    * Library code (if WITHCODE is given)\0" as *const u8
            as *const libc::c_char,
        b"    It also possible to get only function that matches a pattern using LIBRARYNAME argument.\0"
            as *const u8 as *const libc::c_char,
        b"STATS\0" as *const u8 as *const libc::c_char,
        b"    Return information about the current function running:\0" as *const u8
            as *const libc::c_char,
        b"    * Function name\0" as *const u8 as *const libc::c_char,
        b"    * Command used to run the function\0" as *const u8 as *const libc::c_char,
        b"    * Duration in MS that the function is running\0" as *const u8
            as *const libc::c_char,
        b"    If no function is running, return nil\0" as *const u8
            as *const libc::c_char,
        b"    In addition, returns a list of available engines.\0" as *const u8
            as *const libc::c_char,
        b"KILL\0" as *const u8 as *const libc::c_char,
        b"    Kill the current running function.\0" as *const u8 as *const libc::c_char,
        b"FLUSH [ASYNC|SYNC]\0" as *const u8 as *const libc::c_char,
        b"    Delete all the libraries.\0" as *const u8 as *const libc::c_char,
        b"    When called without the optional mode argument, the behavior is determined by the\0"
            as *const u8 as *const libc::c_char,
        b"    lazyfree-lazy-user-flush configuration directive. Valid modes are:\0"
            as *const u8 as *const libc::c_char,
        b"    * ASYNC: Asynchronously flush the libraries.\0" as *const u8
            as *const libc::c_char,
        b"    * SYNC: Synchronously flush the libraries.\0" as *const u8
            as *const libc::c_char,
        b"DUMP\0" as *const u8 as *const libc::c_char,
        b"    Return a serialized payload representing the current libraries, can be restored using FUNCTION RESTORE command\0"
            as *const u8 as *const libc::c_char,
        b"RESTORE <PAYLOAD> [FLUSH|APPEND|REPLACE]\0" as *const u8
            as *const libc::c_char,
        b"    Restore the libraries represented by the given payload, it is possible to give a restore policy to\0"
            as *const u8 as *const libc::c_char,
        b"    control how to handle existing libraries (default APPEND):\0" as *const u8
            as *const libc::c_char,
        b"    * FLUSH: delete all existing libraries.\0" as *const u8
            as *const libc::c_char,
        b"    * APPEND: appends the restored libraries to the existing libraries. On collision, abort.\0"
            as *const u8 as *const libc::c_char,
        b"    * REPLACE: appends the restored libraries to the existing libraries, On collision, replace the old\0"
            as *const u8 as *const libc::c_char,
        b"      libraries with the new libraries (notice that even on this option there is a chance of failure\0"
            as *const u8 as *const libc::c_char,
        b"      in case of functions name collision with another library).\0"
            as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    addReplyHelp(c, help.as_mut_ptr());
}
unsafe extern "C" fn functionsVerifyName(mut name: sds) -> libc::c_int {
    if sdslen(name) == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < sdslen(name) {
        let mut curr_char: libc::c_char = *name.offset(i as isize);
        if curr_char as libc::c_int >= 'a' as i32
            && curr_char as libc::c_int <= 'z' as i32
            || curr_char as libc::c_int >= 'A' as i32
                && curr_char as libc::c_int <= 'Z' as i32
            || curr_char as libc::c_int >= '0' as i32
                && curr_char as libc::c_int <= '9' as i32
            || curr_char as libc::c_int == '_' as i32
        {
            i = i.wrapping_add(1);
        } else {
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn functionExtractLibMetaData(
    mut payload: sds,
    mut md: *mut functionsLibMataData,
    mut err: *mut sds,
) -> libc::c_int {
    let mut current_block: u64;
    let mut name: sds = 0 as sds;
    let mut desc: sds = 0 as sds;
    let mut engine: sds = 0 as sds;
    let mut code: sds = 0 as sds;
    if strncmp(
        payload as *const libc::c_char,
        b"#!\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        *err = sdsnew(b"Missing library metadata\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    let mut shebang_end: *mut libc::c_char = strchr(
        payload as *const libc::c_char,
        '\n' as i32,
    );
    if shebang_end.is_null() {
        *err = sdsnew(b"Invalid library metadata\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    let mut shebang_len: size_t = shebang_end.offset_from(payload) as libc::c_long
        as size_t;
    let mut shebang: sds = sdsnewlen(payload as *const libc::c_void, shebang_len);
    let mut numparts: libc::c_int = 0;
    let mut parts: *mut sds = sdssplitargs(
        shebang as *const libc::c_char,
        &mut numparts,
    );
    sdsfree(shebang);
    if parts.is_null() || numparts == 0 as libc::c_int {
        *err = sdsnew(b"Invalid library metadata\0" as *const u8 as *const libc::c_char);
        sdsfreesplitres(parts, numparts);
        return -(1 as libc::c_int);
    }
    engine = sdsdup(*parts.offset(0 as libc::c_int as isize));
    sdsrange(engine, 2 as libc::c_int as ssize_t, -(1 as libc::c_int) as ssize_t);
    let mut i: libc::c_int = 1 as libc::c_int;
    loop {
        if !(i < numparts) {
            current_block = 15089075282327824602;
            break;
        }
        let mut part: sds = *parts.offset(i as isize);
        if strncasecmp(
            part as *const libc::c_char,
            b"name=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ) == 0 as libc::c_int
        {
            if !name.is_null() {
                *err = sdscatfmt(
                    sdsempty(),
                    b"Invalid metadata value, name argument was given multiple times\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block = 17467102364019782755;
                break;
            } else {
                name = sdsdup(part);
                sdsrange(
                    name,
                    5 as libc::c_int as ssize_t,
                    -(1 as libc::c_int) as ssize_t,
                );
                i += 1;
            }
        } else {
            *err = sdscatfmt(
                sdsempty(),
                b"Invalid metadata value given: %s\0" as *const u8
                    as *const libc::c_char,
                part,
            );
            current_block = 17467102364019782755;
            break;
        }
    }
    match current_block {
        15089075282327824602 => {
            if name.is_null() {
                *err = sdsnew(
                    b"Library name was not given\0" as *const u8 as *const libc::c_char,
                );
            } else {
                sdsfreesplitres(parts, numparts);
                (*md).name = name;
                (*md)
                    .code = sdsnewlen(
                    shebang_end as *const libc::c_void,
                    (sdslen(payload)).wrapping_sub(shebang_len),
                );
                (*md).engine = engine;
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    if !name.is_null() {
        sdsfree(name);
    }
    if !desc.is_null() {
        sdsfree(desc);
    }
    if !engine.is_null() {
        sdsfree(engine);
    }
    if !code.is_null() {
        sdsfree(code);
    }
    sdsfreesplitres(parts, numparts);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn functionFreeLibMetaData(mut md: *mut functionsLibMataData) {
    if !((*md).code).is_null() {
        sdsfree((*md).code);
    }
    if !((*md).name).is_null() {
        sdsfree((*md).name);
    }
    if !((*md).engine).is_null() {
        sdsfree((*md).engine);
    }
}
#[no_mangle]
pub unsafe extern "C" fn functionsCreateWithLibraryCtx(
    mut code: sds,
    mut replace: libc::c_int,
    mut err: *mut sds,
    mut lib_ctx: *mut functionsLibCtx,
) -> sds {
    let mut ei: *mut engineInfo = 0 as *mut engineInfo;
    let mut engine: *mut engine = 0 as *mut engine;
    let mut loaded_lib_name: sds = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut iter: *mut dictIterator = 0 as *mut dictIterator;
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut new_li: *mut functionLibInfo = 0 as *mut functionLibInfo;
    let mut old_li: *mut functionLibInfo = 0 as *mut functionLibInfo;
    let mut md: functionsLibMataData = {
        let mut init = functionsLibMataData {
            engine: 0 as sds,
            name: 0 as *mut libc::c_char,
            code: 0 as *mut libc::c_char,
        };
        init
    };
    if functionExtractLibMetaData(code, &mut md, err) != 0 as libc::c_int {
        return 0 as sds;
    }
    if functionsVerifyName(md.name) != 0 {
        *err = sdsnew(
            b"Library names can only contain letters, numbers, or underscores(_) and must be at least one character long\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        ei = dictFetchValue(engines, md.engine as *const libc::c_void)
            as *mut engineInfo;
        if ei.is_null() {
            *err = sdscatfmt(
                sdsempty(),
                b"Engine '%S' not found\0" as *const u8 as *const libc::c_char,
                md.engine,
            );
        } else {
            engine = (*ei).engine;
            old_li = dictFetchValue((*lib_ctx).libraries, md.name as *const libc::c_void)
                as *mut functionLibInfo;
            if !old_li.is_null() && replace == 0 {
                old_li = 0 as *mut functionLibInfo;
                *err = sdscatfmt(
                    sdsempty(),
                    b"Library '%S' already exists\0" as *const u8 as *const libc::c_char,
                    md.name,
                );
            } else {
                if !old_li.is_null() {
                    libraryUnlink(lib_ctx, old_li);
                }
                new_li = engineLibraryCreate(md.name, ei, code);
                if !(((*engine).create)
                    .expect(
                        "non-null function pointer",
                    )((*engine).engine_ctx, new_li, md.code, err) != 0 as libc::c_int)
                {
                    if ((*(*new_li).functions).ht_used[0 as libc::c_int as usize])
                        .wrapping_add(
                            (*(*new_li).functions).ht_used[1 as libc::c_int as usize],
                        ) == 0 as libc::c_int as libc::c_ulong
                    {
                        *err = sdsnew(
                            b"No functions registered\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        iter = dictGetIterator((*new_li).functions);
                        loop {
                            entry = dictNext(iter);
                            if entry.is_null() {
                                current_block = 1109700713171191020;
                                break;
                            }
                            let mut fi: *mut functionInfo = (*entry).v.val
                                as *mut functionInfo;
                            if (dictFetchValue(
                                (*lib_ctx).functions,
                                (*fi).name as *const libc::c_void,
                            ))
                                .is_null()
                            {
                                continue;
                            }
                            *err = sdscatfmt(
                                sdsempty(),
                                b"Function %s already exists\0" as *const u8
                                    as *const libc::c_char,
                                (*fi).name,
                            );
                            current_block = 12566960168453152444;
                            break;
                        }
                        match current_block {
                            12566960168453152444 => {}
                            _ => {
                                dictReleaseIterator(iter);
                                iter = 0 as *mut dictIterator;
                                libraryLink(lib_ctx, new_li);
                                if !old_li.is_null() {
                                    engineLibraryFree(old_li);
                                }
                                loaded_lib_name = md.name;
                                md.name = 0 as sds;
                                functionFreeLibMetaData(&mut md);
                                return loaded_lib_name;
                            }
                        }
                    }
                }
            }
        }
    }
    if !iter.is_null() {
        dictReleaseIterator(iter);
    }
    if !new_li.is_null() {
        engineLibraryFree(new_li);
    }
    if !old_li.is_null() {
        libraryLink(lib_ctx, old_li);
    }
    functionFreeLibMetaData(&mut md);
    return 0 as sds;
}
#[no_mangle]
pub unsafe extern "C" fn functionLoadCommand(mut c: *mut client) {
    let mut replace: libc::c_int = 0 as libc::c_int;
    let mut argc_pos: libc::c_int = 2 as libc::c_int;
    while argc_pos < (*c).argc - 1 as libc::c_int {
        let fresh0 = argc_pos;
        argc_pos = argc_pos + 1;
        let mut next_arg: *mut robj = *((*c).argv).offset(fresh0 as isize);
        if strcasecmp(
            (*next_arg).ptr as *const libc::c_char,
            b"replace\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            replace = 1 as libc::c_int;
        } else {
            addReplyErrorFormat(
                c,
                b"Unknown option given: %s\0" as *const u8 as *const libc::c_char,
                (*next_arg).ptr as *mut libc::c_char,
            );
            return;
        }
    }
    if argc_pos >= (*c).argc {
        addReplyError(
            c,
            b"Function code is missing\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut code: *mut robj = *((*c).argv).offset(argc_pos as isize);
    let mut err: sds = 0 as sds;
    let mut library_name: sds = 0 as sds;
    library_name = functionsCreateWithLibraryCtx(
        (*code).ptr as sds,
        replace,
        &mut err,
        curr_functions_lib_ctx,
    );
    if library_name.is_null() {
        addReplyErrorSds(c, err);
        return;
    }
    server.dirty += 1;
    addReplyBulkSds(c, library_name);
}
#[no_mangle]
pub unsafe extern "C" fn functionsMemory() -> libc::c_ulong {
    let mut iter: *mut dictIterator = dictGetIterator(engines);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut engines_nemory: size_t = 0 as libc::c_int as size_t;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            break;
        }
        let mut ei: *mut engineInfo = (*entry).v.val as *mut engineInfo;
        let mut engine: *mut engine = (*ei).engine;
        engines_nemory = (engines_nemory as libc::c_ulong)
            .wrapping_add(
                ((*engine).get_used_memory)
                    .expect("non-null function pointer")((*engine).engine_ctx),
            ) as size_t as size_t;
    }
    dictReleaseIterator(iter);
    return engines_nemory;
}
#[no_mangle]
pub unsafe extern "C" fn functionsMemoryOverhead() -> libc::c_ulong {
    let mut memory_overhead: size_t = ((*engines).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*engines).ht_used[1 as libc::c_int as usize])
        .wrapping_mul(core::mem::size_of::<dictEntry>() as libc::c_ulong)
        .wrapping_add(
            (if (*engines).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*engines).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_add(
                    (if (*engines).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                        == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*engines).ht_size_exp[1 as libc::c_int as usize]
                                as libc::c_int
                    }),
                )
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        );
    memory_overhead = (memory_overhead as libc::c_ulong)
        .wrapping_add(
            ((*(*curr_functions_lib_ctx).functions).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*(*curr_functions_lib_ctx).functions)
                        .ht_used[1 as libc::c_int as usize],
                )
                .wrapping_mul(core::mem::size_of::<dictEntry>() as libc::c_ulong)
                .wrapping_add(
                    (if (*(*curr_functions_lib_ctx).functions)
                        .ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                        == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*(*curr_functions_lib_ctx).functions)
                                .ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                    })
                        .wrapping_add(
                            (if (*(*curr_functions_lib_ctx).functions)
                                .ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                                == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*(*curr_functions_lib_ctx).functions)
                                        .ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                            }),
                        )
                        .wrapping_mul(
                            core::mem::size_of::<*mut dictEntry>() as libc::c_ulong,
                        ),
                )
                .wrapping_add(core::mem::size_of::<functionsLibCtx>() as libc::c_ulong),
        ) as size_t as size_t;
    memory_overhead = (memory_overhead as libc::c_ulong)
        .wrapping_add((*curr_functions_lib_ctx).cache_memory) as size_t as size_t;
    memory_overhead = (memory_overhead as libc::c_ulong)
        .wrapping_add(engine_cache_memory) as size_t as size_t;
    return memory_overhead;
}
#[no_mangle]
pub unsafe extern "C" fn functionsNum() -> libc::c_ulong {
    return ((*(*curr_functions_lib_ctx).functions).ht_used[0 as libc::c_int as usize])
        .wrapping_add(
            (*(*curr_functions_lib_ctx).functions).ht_used[1 as libc::c_int as usize],
        );
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibNum() -> libc::c_ulong {
    return ((*(*curr_functions_lib_ctx).libraries).ht_used[0 as libc::c_int as usize])
        .wrapping_add(
            (*(*curr_functions_lib_ctx).libraries).ht_used[1 as libc::c_int as usize],
        );
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibGet() -> *mut dict {
    return (*curr_functions_lib_ctx).libraries;
}
#[no_mangle]
pub unsafe extern "C" fn functionsLibCtxfunctionsLen(
    mut functions_ctx: *mut functionsLibCtx,
) -> size_t {
    return ((*(*functions_ctx).functions).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*functions_ctx).functions).ht_used[1 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn functionsInit() -> libc::c_int {
    engines = dictCreate(&mut engineDictType);
    if luaEngineInitEngine() != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    curr_functions_lib_ctx = functionsLibCtxCreate();
    return 0 as libc::c_int;
}
