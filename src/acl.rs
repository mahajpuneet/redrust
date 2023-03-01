
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
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdscmp(s1: sds, s2: sds) -> libc::c_int;
    fn sdssplitlen(
        s: *const libc::c_char,
        len: ssize_t,
        sep: *const libc::c_char,
        seplen: libc::c_int,
        count: *mut libc::c_int,
    ) -> *mut sds;
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdstolower(s: sds);
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listEmpty(list: *mut list);
    fn listAddNodeHead(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listDup(orig: *mut list) -> *mut list;
    fn listSearchKey(list: *mut list, key: *mut libc::c_void) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn stringmatchlen(
        p: *const libc::c_char,
        plen: libc::c_int,
        s: *const libc::c_char,
        slen: libc::c_int,
        nocase: libc::c_int,
    ) -> libc::c_int;
    static mut raxNotFound: *mut libc::c_void;
    fn raxNew() -> *mut rax;
    fn raxInsert(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        data: *mut libc::c_void,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxRemove(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxFind(rax: *mut rax, s: *mut libc::c_uchar, len: size_t) -> *mut libc::c_void;
    fn raxFree(rax: *mut rax);
    fn raxFreeWithCallback(
        rax: *mut rax,
        free_callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn raxStart(it: *mut raxIterator, rt: *mut rax);
    fn raxSeek(
        it: *mut raxIterator,
        op: *const libc::c_char,
        ele: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn raxNext(it: *mut raxIterator) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    fn raxSize(rax: *mut rax) -> uint64_t;
    pub static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn moduleNotifyUserChanged(c: *mut client);
    fn mstime() -> libc::c_longlong;
    fn getRandomHexChars(p: *mut libc::c_char, len: size_t);
    fn freeClient(c: *mut client);
    fn freeClientAsync(c: *mut client);
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn setDeferredMapLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn setDeferredSetLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn addReplyNull(c: *mut client);
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyErrorSdsSafe(c: *mut client, err: sds);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyDouble(c: *mut client, d: libc::c_double);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn catClientInfoString(s: sds, client: *mut client) -> sds;
    fn redactClientCommandArgument(c: *mut client, argc: libc::c_int);
    fn getClientType(c: *mut client) -> libc::c_int;
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn addReplyStatusFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn decrRefCount(o: *mut robj);
    fn incrRefCount(o: *mut robj);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn getLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn _serverAssert(estr: *const libc::c_char, file: *const libc::c_char, line: libc::c_int);
    fn lookupCommandBySdsLogic(commands: *mut dict, s: sds) -> *mut redisCommand;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn getKeysFreeResult(result: *mut getKeysResult);
    fn getChannelsFromCommand(
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        result: *mut getKeysResult,
    ) -> libc::c_int;
    fn doesCommandHaveChannelsWithFlags(cmd: *mut redisCommand, flags: libc::c_int) -> libc::c_int;
    fn getKeysFromCommandWithSpecs(
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        search_flags: libc::c_int,
        result: *mut getKeysResult,
    ) -> libc::c_int;
    fn doesCommandHaveKeys(cmd: *mut redisCommand) -> libc::c_int;
    fn _serverPanic(file: *const libc::c_char, line: libc::c_int, msg: *const libc::c_char, _: ...);
    fn lookupCommand(argv: *mut *mut robj, argc: libc::c_int) -> *mut redisCommand;
    fn exit(_: libc::c_int) -> !;
    fn sha256_init(ctx: *mut SHA256_CTX);
    fn sha256_update(ctx: *mut SHA256_CTX, data: *const BYTE, len: size_t);
    fn sha256_final(ctx: *mut SHA256_CTX, hash: *mut BYTE);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
    pub beforesleep: Option<aeBeforeSleepProc>,
    pub aftersleep: Option<aeBeforeSleepProc>,
    pub flags: libc::c_int,
}
pub type aeBeforeSleepProc = unsafe extern "C" fn(*mut aeEventLoop) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeTimeEvent {
    pub id: libc::c_longlong,
    pub when: monotime,
    pub timeProc: Option<aeTimeProc>,
    pub finalizerProc: Option<aeEventFinalizerProc>,
    pub clientData: *mut libc::c_void,
    pub prev: *mut aeTimeEvent,
    pub next: *mut aeTimeEvent,
    pub refcount: libc::c_int,
}
pub type aeEventFinalizerProc = unsafe extern "C" fn(*mut aeEventLoop, *mut libc::c_void) -> ();
pub type aeTimeProc =
    unsafe extern "C" fn(*mut aeEventLoop, libc::c_longlong, *mut libc::c_void) -> libc::c_int;
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
    pub rfileProc: Option<aeFileProc>,
    pub wfileProc: Option<aeFileProc>,
    pub clientData: *mut libc::c_void,
}
pub type aeFileProc =
    unsafe extern "C" fn(*mut aeEventLoop, libc::c_int, *mut libc::c_void, libc::c_int) -> ();
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
pub type ConnectionCallbackFunc = Option<unsafe extern "C" fn(*mut connection) -> ()>;
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
    pub ae_handler: Option<
        unsafe extern "C" fn(*mut aeEventLoop, libc::c_int, *mut libc::c_void, libc::c_int) -> (),
    >,
    pub connect: Option<
        unsafe extern "C" fn(
            *mut connection,
            *const libc::c_char,
            libc::c_int,
            *const libc::c_char,
            ConnectionCallbackFunc,
        ) -> libc::c_int,
    >,
    pub write:
        Option<unsafe extern "C" fn(*mut connection, *const libc::c_void, size_t) -> libc::c_int>,
    pub writev:
        Option<unsafe extern "C" fn(*mut connection, *const iovec, libc::c_int) -> libc::c_int>,
    pub read:
        Option<unsafe extern "C" fn(*mut connection, *mut libc::c_void, size_t) -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut connection) -> ()>,
    pub accept:
        Option<unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc) -> libc::c_int>,
    pub set_write_handler: Option<
        unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc, libc::c_int) -> libc::c_int,
    >,
    pub set_read_handler:
        Option<unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc) -> libc::c_int>,
    pub get_last_error: Option<unsafe extern "C" fn(*mut connection) -> *const libc::c_char>,
    pub blocking_connect: Option<
        unsafe extern "C" fn(
            *mut connection,
            *const libc::c_char,
            libc::c_int,
            libc::c_longlong,
        ) -> libc::c_int,
    >,
    pub sync_write: Option<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub sync_read: Option<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub sync_readline: Option<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub get_type: Option<unsafe extern "C" fn(*mut connection) -> libc::c_int>,
}
#[derive(Copy, Clone, c2rust_bitfields::BitfieldStruct)]
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
    pub hashFunction: Option<unsafe extern "C" fn(*const libc::c_void) -> uint64_t>,
    pub keyDup: Option<unsafe extern "C" fn(*mut dict, *const libc::c_void) -> *mut libc::c_void>,
    pub valDup: Option<unsafe extern "C" fn(*mut dict, *const libc::c_void) -> *mut libc::c_void>,
    pub keyCompare: Option<
        unsafe extern "C" fn(*mut dict, *const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    pub keyDestructor: Option<unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> ()>,
    pub valDestructor: Option<unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> ()>,
    pub expandAllowed: Option<unsafe extern "C" fn(size_t, libc::c_double) -> libc::c_int>,
    pub dictEntryMetadataBytes: Option<unsafe extern "C" fn(*mut dict) -> size_t>,
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
    pub dup: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub match_0: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>,
    pub len: libc::c_ulong,
}
#[derive(Copy, Clone, c2rust_bitfields::BitfieldStruct)]
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
pub struct raxStack {
    pub stack: *mut *mut libc::c_void,
    pub items: size_t,
    pub maxitems: size_t,
    pub static_items: [*mut libc::c_void; 32],
    pub oom: libc::c_int,
}
pub type raxNodeCallback = Option<unsafe extern "C" fn(*mut *mut raxNode) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raxIterator {
    pub flags: libc::c_int,
    pub rt: *mut rax,
    pub key: *mut libc::c_uchar,
    pub data: *mut libc::c_void,
    pub key_len: size_t,
    pub key_max: size_t,
    pub key_static_string: [libc::c_uchar; 128],
    pub node: *mut raxNode,
    pub stack: raxStack,
    pub node_cb: raxNodeCallback,
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
pub type RedisModuleUserChangedFunc =
    Option<unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> ()>;
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
    pub proc_0: Option<redisCommandProc>,
    pub arity: libc::c_int,
    pub flags: uint64_t,
    pub acl_categories: uint64_t,
    pub key_specs_static: [keySpec; 4],
    pub getkeys_proc: Option<redisGetKeysProc>,
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
pub struct C2RustUnnamed_6 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aclSelector {
    pub flags: uint32_t,
    pub allowed_commands: [uint64_t; 16],
    pub allowed_firstargs: *mut *mut sds,
    pub patterns: *mut list,
    pub channels: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyPattern {
    pub flags: libc::c_int,
    pub pattern: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ACLCategoryItem {
    pub name: *const libc::c_char,
    pub flag: uint64_t,
}
pub const _ISspace: C2RustUnnamed_7 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256_CTX {
    pub data: [BYTE; 64],
    pub datalen: WORD,
    pub bitlen: libc::c_ulonglong,
    pub state: [WORD; 8],
}
pub type WORD = uint32_t;
pub type BYTE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ACLLogEntry {
    pub count: uint64_t,
    pub reason: libc::c_int,
    pub context: libc::c_int,
    pub object: sds,
    pub username: sds,
    pub ctime: mstime_t,
    pub cinfo: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aclKeyResultCache {
    pub keys_init: libc::c_int,
    pub keys: getKeysResult,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ACLUserFlag {
    pub name: *const libc::c_char,
    pub flag: uint64_t,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_7 = 8;
pub const _ISpunct: C2RustUnnamed_7 = 4;
pub const _IScntrl: C2RustUnnamed_7 = 2;
pub const _ISblank: C2RustUnnamed_7 = 1;
pub const _ISgraph: C2RustUnnamed_7 = 32768;
pub const _ISprint: C2RustUnnamed_7 = 16384;
pub const _ISxdigit: C2RustUnnamed_7 = 4096;
pub const _ISdigit: C2RustUnnamed_7 = 2048;
pub const _ISalpha: C2RustUnnamed_7 = 1024;
pub const _ISlower: C2RustUnnamed_7 = 512;
pub const _ISupper: C2RustUnnamed_7 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ACLSelectorFlags {
    pub name: *const libc::c_char,
    pub flag: uint64_t,
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize) as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s.offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s.offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s.offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s.offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub static mut Users: *mut rax = 0 as *const rax as *mut rax;
#[no_mangle]
pub static mut DefaultUser: *mut user = 0 as *const user as *mut user;
#[no_mangle]
pub static mut UsersToLoad: *mut list = 0 as *const list as *mut list;
#[no_mangle]
pub static mut ACLLog: *mut list = 0 as *const list as *mut list;
static mut commandId: *mut rax = 0 as *const rax as *mut rax;
static mut nextid: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
#[no_mangle]
pub static mut ACLCommandCategories: [ACLCategoryItem; 22] = [
    {
        let mut init = ACLCategoryItem {
            name: b"keyspace\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"read\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 1 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"write\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 2 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"set\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 3 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"sortedset\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 4 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"list\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 5 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"hash\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 6 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"string\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 7 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"bitmap\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 8 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"hyperloglog\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 9 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"geo\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 10 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"stream\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 11 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"pubsub\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 12 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"admin\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 13 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"fast\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 14 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"slow\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 15 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"blocking\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 16 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"dangerous\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 17 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"connection\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 18 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"transaction\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 19 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: b"scripting\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_ulonglong) << 20 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLCategoryItem {
            name: 0 as *const libc::c_char,
            flag: 0 as libc::c_int as uint64_t,
        };
        init
    },
];
#[no_mangle]
pub static mut ACLUserFlags: [ACLUserFlag; 6] = [
    {
        let mut init = ACLUserFlag {
            name: b"on\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_int) << 0 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLUserFlag {
            name: b"off\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_int) << 1 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLUserFlag {
            name: b"nopass\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLUserFlag {
            name: b"skip-sanitize-payload\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_int) << 4 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLUserFlag {
            name: b"sanitize-payload\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_int) << 3 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLUserFlag {
            name: 0 as *const libc::c_char,
            flag: 0 as libc::c_int as uint64_t,
        };
        init
    },
];
#[no_mangle]
pub static mut ACLSelectorFlags: [ACLSelectorFlags; 4] = [
    {
        let mut init = ACLSelectorFlags {
            name: b"allkeys\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_int) << 1 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLSelectorFlags {
            name: b"allchannels\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_int) << 3 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLSelectorFlags {
            name: b"allcommands\0" as *const u8 as *const libc::c_char,
            flag: ((1 as libc::c_int) << 2 as libc::c_int) as uint64_t,
        };
        init
    },
    {
        let mut init = ACLSelectorFlags {
            name: 0 as *const libc::c_char,
            flag: 0 as libc::c_int as uint64_t,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn time_independent_strcmp(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut diff: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < len {
        diff |= *a.offset(j as isize) as libc::c_int ^ *b.offset(j as isize) as libc::c_int;
        j += 1;
    }
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn ACLHashPassword(
    mut cleartext: *mut libc::c_uchar,
    mut len: size_t,
) -> sds {
    let mut ctx: SHA256_CTX = SHA256_CTX {
        data: [0; 64],
        datalen: 0,
        bitlen: 0,
        state: [0; 8],
    };
    let mut hash: [libc::c_uchar; 32] = [0; 32];
    let mut hex: [libc::c_char; 64] = [0; 64];
    let mut cset: *mut libc::c_char =
        b"0123456789abcdef\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    sha256_init(&mut ctx);
    sha256_update(&mut ctx, cleartext as *const BYTE, len);
    sha256_final(&mut ctx, hash.as_mut_ptr());
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 32 as libc::c_int {
        hex[(j * 2 as libc::c_int) as usize] = *cset.offset(
            ((hash[j as usize] as libc::c_int & 0xf0 as libc::c_int) >> 4 as libc::c_int) as isize,
        );
        hex[(j * 2 as libc::c_int + 1 as libc::c_int) as usize] =
            *cset.offset((hash[j as usize] as libc::c_int & 0xf as libc::c_int) as isize);
        j += 1;
    }
    return sdsnewlen(
        hex.as_mut_ptr() as *const libc::c_void,
        (32 as libc::c_int * 2 as libc::c_int) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ACLCheckPasswordHash(
    mut hash: *mut libc::c_uchar,
    mut hashlen: libc::c_int,
) -> libc::c_int {
    if hashlen != 32 as libc::c_int * 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 32 as libc::c_int * 2 as libc::c_int {
        let mut c: libc::c_char = *hash.offset(i as isize) as libc::c_char;
        if ((c as libc::c_int) < 'a' as i32 || c as libc::c_int > 'f' as i32)
            && ((c as libc::c_int) < '0' as i32 || c as libc::c_int > '9' as i32)
        {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLStringHasSpaces(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        if *(*__ctype_b_loc()).offset(*s.offset(i as isize) as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || *s.offset(i as isize) as libc::c_int == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLGetCommandCategoryFlagByName(
    mut name: *const libc::c_char,
) -> uint64_t {
    let mut j: libc::c_int = 0 as libc::c_int;
    while ACLCommandCategories[j as usize].flag != 0 as libc::c_int as libc::c_ulong {
        if strcasecmp(name, ACLCommandCategories[j as usize].name) == 0 {
            return ACLCommandCategories[j as usize].flag;
        }
        j += 1;
    }
    return 0 as libc::c_int as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn ACLListMatchLoadedUser(
    mut definition: *mut libc::c_void,
    mut user: *mut libc::c_void,
) -> libc::c_int {
    let mut user_definition: *mut sds = definition as *mut sds;
    return (sdscmp(
        *user_definition.offset(0 as libc::c_int as isize),
        user as sds,
    ) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLListMatchSds(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
) -> libc::c_int {
    return (sdscmp(a as sds, b as sds) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLListFreeSds(mut item: *mut libc::c_void) {
    sdsfree(item as sds);
}
#[no_mangle]
pub unsafe extern "C" fn ACLListDupSds(mut item: *mut libc::c_void) -> *mut libc::c_void {
    return sdsdup(item as sds) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ACLKeyPatternCreate(
    mut pattern: sds,
    mut flags: libc::c_int,
) -> *mut keyPattern {
    let mut new: *mut keyPattern =
        zmalloc(core::mem::size_of::<keyPattern>() as libc::c_ulong) as *mut keyPattern;
    (*new).pattern = pattern;
    (*new).flags = flags;
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn ACLKeyPatternFree(mut pattern: *mut keyPattern) {
    sdsfree((*pattern).pattern);
    zfree(pattern as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ACLListMatchKeyPattern(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
) -> libc::c_int {
    return (sdscmp(
        (*(a as *mut keyPattern)).pattern,
        (*(b as *mut keyPattern)).pattern,
    ) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLListFreeKeyPattern(mut item: *mut libc::c_void) {
    ACLKeyPatternFree(item as *mut keyPattern);
}
#[no_mangle]
pub unsafe extern "C" fn ACLListDupKeyPattern(mut item: *mut libc::c_void) -> *mut libc::c_void {
    let mut old: *mut keyPattern = item as *mut keyPattern;
    return ACLKeyPatternCreate(sdsdup((*old).pattern), (*old).flags) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn sdsCatPatternString(mut base: sds, mut pat: *mut keyPattern) -> sds {
    if (*pat).flags
        == (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
    {
        base = sdscatlen(
            base,
            b"~\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    } else if (*pat).flags == (1 as libc::c_int) << 0 as libc::c_int {
        base = sdscatlen(
            base,
            b"%R~\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
    } else if (*pat).flags == (1 as libc::c_int) << 1 as libc::c_int {
        base = sdscatlen(
            base,
            b"%W~\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
    } else {
        _serverPanic(
            b"acl.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
            b"Invalid key pattern flag detected\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return sdscatsds(base, (*pat).pattern);
}
#[no_mangle]
pub unsafe extern "C" fn ACLCreateSelector(mut flags: libc::c_int) -> *mut aclSelector {
    let mut selector: *mut aclSelector =
        zmalloc(core::mem::size_of::<aclSelector>() as libc::c_ulong) as *mut aclSelector;
    (*selector).flags = (flags | server.acl_pubsub_default) as uint32_t;
    (*selector).patterns = listCreate();
    (*selector).channels = listCreate();
    (*selector).allowed_firstargs = 0 as *mut *mut sds;
    (*(*selector).patterns).match_0 = Some(
        ACLListMatchKeyPattern
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    );
    (*(*selector).patterns).free =
        Some(ACLListFreeKeyPattern as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*(*selector).patterns).dup =
        Some(ACLListDupKeyPattern as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void);
    (*(*selector).channels).match_0 = Some(
        ACLListMatchSds
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    );
    (*(*selector).channels).free =
        Some(ACLListFreeSds as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*(*selector).channels).dup =
        Some(ACLListDupSds as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void);
    memset(
        ((*selector).allowed_commands).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[uint64_t; 16]>() as libc::c_ulong,
    );
    return selector;
}
#[no_mangle]
pub unsafe extern "C" fn ACLFreeSelector(mut selector: *mut aclSelector) {
    listRelease((*selector).patterns);
    listRelease((*selector).channels);
    ACLResetFirstArgs(selector);
    zfree(selector as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ACLCopySelector(mut src: *mut aclSelector) -> *mut aclSelector {
    let mut dst: *mut aclSelector =
        zmalloc(core::mem::size_of::<aclSelector>() as libc::c_ulong) as *mut aclSelector;
    (*dst).flags = (*src).flags;
    (*dst).patterns = listDup((*src).patterns);
    (*dst).channels = listDup((*src).channels);
    memcpy(
        ((*dst).allowed_commands).as_mut_ptr() as *mut libc::c_void,
        ((*src).allowed_commands).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[uint64_t; 16]>() as libc::c_ulong,
    );
    (*dst).allowed_firstargs = 0 as *mut *mut sds;
    if !((*src).allowed_firstargs).is_null() {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 1024 as libc::c_int {
            if !(*((*src).allowed_firstargs).offset(j as isize)).is_null() {
                let mut i: libc::c_int = 0 as libc::c_int;
                while !(*(*((*src).allowed_firstargs).offset(j as isize)).offset(i as isize))
                    .is_null()
                {
                    ACLAddAllowedFirstArg(
                        dst,
                        j as libc::c_ulong,
                        *(*((*src).allowed_firstargs).offset(j as isize)).offset(i as isize)
                            as *const libc::c_char,
                    );
                    i += 1;
                }
            }
            j += 1;
        }
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn ACLListFreeSelector(mut a: *mut libc::c_void) {
    ACLFreeSelector(a as *mut aclSelector);
}
#[no_mangle]
pub unsafe extern "C" fn ACLListDuplicateSelector(mut src: *mut libc::c_void) -> *mut libc::c_void {
    return ACLCopySelector(src as *mut aclSelector) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ACLUserGetRootSelector(mut u: *mut user) -> *mut aclSelector {
    if (*(*u).selectors).len != 0 {
    } else {
        _serverAssert(
            b"listLength(u->selectors)\0" as *const u8 as *const libc::c_char,
            b"acl.c\0" as *const u8 as *const libc::c_char,
            371 as libc::c_int,
        );
        unreachable!();
    };
    let mut s: *mut aclSelector = (*(*(*u).selectors).head).value as *mut aclSelector;
    if (*s).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
    } else {
        _serverAssert(
            b"s->flags & SELECTOR_FLAG_ROOT\0" as *const u8 as *const libc::c_char,
            b"acl.c\0" as *const u8 as *const libc::c_char,
            373 as libc::c_int,
        );
        unreachable!();
    };
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn ACLCreateUser(
    mut name: *const libc::c_char,
    mut namelen: size_t,
) -> *mut user {
    if raxFind(Users, name as *mut libc::c_uchar, namelen) != raxNotFound {
        return 0 as *mut user;
    }
    let mut u: *mut user = zmalloc(core::mem::size_of::<user>() as libc::c_ulong) as *mut user;
    (*u).name = sdsnewlen(name as *const libc::c_void, namelen);
    (*u).flags = ((1 as libc::c_int) << 1 as libc::c_int) as uint32_t;
    (*u).passwords = listCreate();
    (*u).acl_string = 0 as *mut robj;
    (*(*u).passwords).match_0 = Some(
        ACLListMatchSds
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    );
    (*(*u).passwords).free = Some(ACLListFreeSds as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*(*u).passwords).dup =
        Some(ACLListDupSds as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void);
    (*u).selectors = listCreate();
    (*(*u).selectors).free =
        Some(ACLListFreeSelector as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*(*u).selectors).dup = Some(
        ACLListDuplicateSelector as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    );
    let mut s: *mut aclSelector = ACLCreateSelector((1 as libc::c_int) << 0 as libc::c_int);
    listAddNodeHead((*u).selectors, s as *mut libc::c_void);
    raxInsert(
        Users,
        name as *mut libc::c_uchar,
        namelen,
        u as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return u;
}
#[no_mangle]
pub unsafe extern "C" fn ACLCreateUnlinkedUser() -> *mut user {
    let mut username: [libc::c_char; 64] = [0; 64];
    let mut j: libc::c_int = 0 as libc::c_int;
    loop {
        snprintf(
            username.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"__fakeuser:%d__\0" as *const u8 as *const libc::c_char,
            j,
        );
        let mut fakeuser: *mut user =
            ACLCreateUser(username.as_mut_ptr(), strlen(username.as_mut_ptr()));
        if fakeuser.is_null() {
            j += 1;
        } else {
            let mut retval: libc::c_int = raxRemove(
                Users,
                username.as_mut_ptr() as *mut libc::c_uchar,
                strlen(username.as_mut_ptr()),
                0 as *mut *mut libc::c_void,
            );
            if retval != 0 as libc::c_int {
            } else {
                _serverAssert(
                    b"retval != 0\0" as *const u8 as *const libc::c_char,
                    b"acl.c\0" as *const u8 as *const libc::c_char,
                    417 as libc::c_int,
                );
                unreachable!();
            };
            return fakeuser;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ACLFreeUser(mut u: *mut user) {
    sdsfree((*u).name);
    if !((*u).acl_string).is_null() {
        decrRefCount((*u).acl_string);
        (*u).acl_string = 0 as *mut robj;
    }
    listRelease((*u).passwords);
    listRelease((*u).selectors);
    zfree(u as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ACLFreeUserAndKillClients(mut u: *mut user) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.clients, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        if (*c).user == u {
            (*c).user = DefaultUser;
            (*c).authenticated = 0 as libc::c_int;
            if c == server.current_client {
                (*c).flags = ((*c).flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 40 as libc::c_int)
                    as uint64_t;
            } else {
                freeClientAsync(c);
            }
        }
    }
    ACLFreeUser(u);
}
#[no_mangle]
pub unsafe extern "C" fn ACLCopyUser(mut dst: *mut user, mut src: *mut user) {
    listRelease((*dst).passwords);
    listRelease((*dst).selectors);
    (*dst).passwords = listDup((*src).passwords);
    (*dst).selectors = listDup((*src).selectors);
    (*dst).flags = (*src).flags;
    if !((*dst).acl_string).is_null() {
        decrRefCount((*dst).acl_string);
    }
    (*dst).acl_string = (*src).acl_string;
    if !((*dst).acl_string).is_null() {
        incrRefCount((*dst).acl_string);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ACLFreeUsersSet(mut users: *mut rax) {
    raxFreeWithCallback(
        users,
        core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut user) -> ()>,
            Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(
            ACLFreeUserAndKillClients as unsafe extern "C" fn(*mut user) -> (),
        )),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ACLGetCommandBitCoordinates(
    mut id: uint64_t,
    mut word: *mut uint64_t,
    mut bit: *mut uint64_t,
) -> libc::c_int {
    if id >= 1024 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    *word = id
        .wrapping_div(core::mem::size_of::<uint64_t>() as libc::c_ulong)
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
    *bit = ((1 as libc::c_ulonglong)
        << id.wrapping_rem(
            (core::mem::size_of::<uint64_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )) as uint64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLGetSelectorCommandBit(
    mut selector: *const aclSelector,
    mut id: libc::c_ulong,
) -> libc::c_int {
    let mut word: uint64_t = 0;
    let mut bit: uint64_t = 0;
    if ACLGetCommandBitCoordinates(id, &mut word, &mut bit) == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    return ((*selector).allowed_commands[word as usize] & bit != 0 as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLSelectorCanExecuteFutureCommands(
    mut selector: *mut aclSelector,
) -> libc::c_int {
    return ACLGetSelectorCommandBit(
        selector,
        (1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ACLSetSelectorCommandBit(
    mut selector: *mut aclSelector,
    mut id: libc::c_ulong,
    mut value: libc::c_int,
) {
    let mut word: uint64_t = 0;
    let mut bit: uint64_t = 0;
    if ACLGetCommandBitCoordinates(id, &mut word, &mut bit) == -(1 as libc::c_int) {
        return;
    }
    if value != 0 {
        (*selector).allowed_commands[word as usize] |= bit;
    } else {
        (*selector).allowed_commands[word as usize] &= !bit;
        (*selector).flags &= !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ACLChangeSelectorPerm(
    mut selector: *mut aclSelector,
    mut cmd: *mut redisCommand,
    mut allow: libc::c_int,
) {
    let mut id: libc::c_ulong = (*cmd).id as libc::c_ulong;
    ACLSetSelectorCommandBit(selector, id, allow);
    ACLResetFirstArgsForCommand(selector, id);
    if !((*cmd).subcommands_dict).is_null() {
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        let mut di: *mut dictIterator = dictGetSafeIterator((*cmd).subcommands_dict);
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut sub: *mut redisCommand = (*de).v.val as *mut redisCommand;
            ACLSetSelectorCommandBit(selector, (*sub).id as libc::c_ulong, allow);
        }
        dictReleaseIterator(di);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ACLSetSelectorCommandBitsForCategoryLogic(
    mut commands: *mut dict,
    mut selector: *mut aclSelector,
    mut cflag: uint64_t,
    mut value: libc::c_int,
) {
    let mut di: *mut dictIterator = dictGetIterator(commands);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cmd: *mut redisCommand = (*de).v.val as *mut redisCommand;
        if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0 {
            continue;
        }
        if (*cmd).acl_categories & cflag != 0 {
            ACLChangeSelectorPerm(selector, cmd, value);
        }
        if !((*cmd).subcommands_dict).is_null() {
            ACLSetSelectorCommandBitsForCategoryLogic(
                (*cmd).subcommands_dict,
                selector,
                cflag,
                value,
            );
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn ACLSetSelectorCommandBitsForCategory(
    mut selector: *mut aclSelector,
    mut category: *const libc::c_char,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut cflag: uint64_t = ACLGetCommandCategoryFlagByName(category);
    if cflag == 0 {
        return -(1 as libc::c_int);
    }
    ACLSetSelectorCommandBitsForCategoryLogic(server.orig_commands, selector, cflag, value);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLCountCategoryBitsForCommands(
    mut commands: *mut dict,
    mut selector: *mut aclSelector,
    mut on: *mut libc::c_ulong,
    mut off: *mut libc::c_ulong,
    mut cflag: uint64_t,
) {
    let mut di: *mut dictIterator = dictGetIterator(commands);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cmd: *mut redisCommand = (*de).v.val as *mut redisCommand;
        if (*cmd).acl_categories & cflag != 0 {
            if ACLGetSelectorCommandBit(selector, (*cmd).id as libc::c_ulong) != 0 {
                *on = (*on).wrapping_add(1);
            } else {
                *off = (*off).wrapping_add(1);
            }
        }
        if !((*cmd).subcommands_dict).is_null() {
            ACLCountCategoryBitsForCommands((*cmd).subcommands_dict, selector, on, off, cflag);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn ACLCountCategoryBitsForSelector(
    mut selector: *mut aclSelector,
    mut on: *mut libc::c_ulong,
    mut off: *mut libc::c_ulong,
    mut category: *const libc::c_char,
) -> libc::c_int {
    let mut cflag: uint64_t = ACLGetCommandCategoryFlagByName(category);
    if cflag == 0 {
        return -(1 as libc::c_int);
    }
    *off = 0 as libc::c_int as libc::c_ulong;
    *on = *off;
    ACLCountCategoryBitsForCommands(server.orig_commands, selector, on, off, cflag);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLDescribeSelectorCommandRulesSingleCommands(
    mut selector: *mut aclSelector,
    mut fake_selector: *mut aclSelector,
    mut rules: sds,
    mut commands: *mut dict,
) -> sds {
    let mut di: *mut dictIterator = dictGetIterator(commands);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cmd: *mut redisCommand = (*de).v.val as *mut redisCommand;
        let mut userbit: libc::c_int =
            ACLGetSelectorCommandBit(selector, (*cmd).id as libc::c_ulong);
        let mut fakebit: libc::c_int =
            ACLGetSelectorCommandBit(fake_selector, (*cmd).id as libc::c_ulong);
        if userbit != fakebit {
            rules = sdscatlen(
                rules,
                (if userbit != 0 {
                    b"+\0" as *const u8 as *const libc::c_char
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                }) as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            rules = sdscatsds(rules, (*cmd).fullname);
            rules = sdscatlen(
                rules,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            ACLChangeSelectorPerm(fake_selector, cmd, userbit);
        }
        if !((*cmd).subcommands_dict).is_null() {
            rules = ACLDescribeSelectorCommandRulesSingleCommands(
                selector,
                fake_selector,
                rules,
                (*cmd).subcommands_dict,
            );
        }
        if userbit == 0 as libc::c_int
            && !((*selector).allowed_firstargs).is_null()
            && !(*((*selector).allowed_firstargs).offset((*cmd).id as isize)).is_null()
        {
            let mut j: libc::c_int = 0 as libc::c_int;
            while !(*(*((*selector).allowed_firstargs).offset((*cmd).id as isize))
                .offset(j as isize))
            .is_null()
            {
                rules = sdscatlen(
                    rules,
                    b"+\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
                rules = sdscatsds(rules, (*cmd).fullname);
                rules = sdscatlen(
                    rules,
                    b"|\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
                rules = sdscatsds(
                    rules,
                    *(*((*selector).allowed_firstargs).offset((*cmd).id as isize))
                        .offset(j as isize),
                );
                rules = sdscatlen(
                    rules,
                    b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
                j += 1;
            }
        }
    }
    dictReleaseIterator(di);
    return rules;
}
#[no_mangle]
pub unsafe extern "C" fn ACLDescribeSelectorCommandRules(mut selector: *mut aclSelector) -> sds {
    let mut rules: sds = sdsempty();
    let mut additive: libc::c_int = 0;
    let mut fs: aclSelector = {
        let mut init = aclSelector {
            flags: 0 as libc::c_int as uint32_t,
            allowed_commands: [0; 16],
            allowed_firstargs: 0 as *mut *mut sds,
            patterns: 0 as *mut list,
            channels: 0 as *mut list,
        };
        init
    };
    let mut fake_selector: *mut aclSelector = &mut fs;
    if ACLSelectorCanExecuteFutureCommands(selector) != 0 {
        additive = 0 as libc::c_int;
        rules = sdscat(rules, b"+@all \0" as *const u8 as *const libc::c_char);
        ACLSetSelector(
            fake_selector,
            b"+@all\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int) as size_t,
        );
    } else {
        additive = 1 as libc::c_int;
        rules = sdscat(rules, b"-@all \0" as *const u8 as *const libc::c_char);
        ACLSetSelector(
            fake_selector,
            b"-@all\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int) as size_t,
        );
    }
    let mut ts: aclSelector = {
        let mut init = aclSelector {
            flags: 0 as libc::c_int as uint32_t,
            allowed_commands: [0; 16],
            allowed_firstargs: 0 as *mut *mut sds,
            patterns: 0 as *mut list,
            channels: 0 as *mut list,
        };
        init
    };
    let mut temp_selector: *mut aclSelector = &mut ts;
    let mut applied: [libc::c_char; 22] = [0; 22];
    memset(
        applied.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
    );
    memcpy(
        ((*temp_selector).allowed_commands).as_mut_ptr() as *mut libc::c_void,
        ((*selector).allowed_commands).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[uint64_t; 16]>() as libc::c_ulong,
    );
    loop {
        let mut best: libc::c_int = -(1 as libc::c_int);
        let mut mindiff: libc::c_ulong = 2147483647 as libc::c_int as libc::c_ulong;
        let mut maxsame: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut j: libc::c_int = 0 as libc::c_int;
        while ACLCommandCategories[j as usize].flag != 0 as libc::c_int as libc::c_ulong {
            if !(applied[j as usize] != 0) {
                let mut on: libc::c_ulong = 0;
                let mut off: libc::c_ulong = 0;
                let mut diff: libc::c_ulong = 0;
                let mut same: libc::c_ulong = 0;
                ACLCountCategoryBitsForSelector(
                    temp_selector,
                    &mut on,
                    &mut off,
                    ACLCommandCategories[j as usize].name,
                );
                diff = if additive != 0 { off } else { on };
                same = if additive != 0 { on } else { off };
                if same > diff && (diff < mindiff || diff == mindiff && same > maxsame) {
                    best = j;
                    mindiff = diff;
                    maxsame = same;
                }
            }
            j += 1;
        }
        if best == -(1 as libc::c_int) {
            break;
        }
        let mut op: sds = sdsnewlen(
            (if additive != 0 {
                b"+@\0" as *const u8 as *const libc::c_char
            } else {
                b"-@\0" as *const u8 as *const libc::c_char
            }) as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        op = sdscat(op, ACLCommandCategories[best as usize].name);
        ACLSetSelector(
            fake_selector,
            op as *const libc::c_char,
            -(1 as libc::c_int) as size_t,
        );
        let mut invop: sds = sdsnewlen(
            (if additive != 0 {
                b"-@\0" as *const u8 as *const libc::c_char
            } else {
                b"+@\0" as *const u8 as *const libc::c_char
            }) as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        invop = sdscat(invop, ACLCommandCategories[best as usize].name);
        ACLSetSelector(
            temp_selector,
            invop as *const libc::c_char,
            -(1 as libc::c_int) as size_t,
        );
        rules = sdscatsds(rules, op);
        rules = sdscatlen(
            rules,
            b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        sdsfree(op);
        sdsfree(invop);
        applied[best as usize] = 1 as libc::c_int as libc::c_char;
    }
    rules = ACLDescribeSelectorCommandRulesSingleCommands(
        selector,
        fake_selector,
        rules,
        server.orig_commands,
    );
    sdsrange(
        rules,
        0 as libc::c_int as ssize_t,
        -(2 as libc::c_int) as ssize_t,
    );
    if memcmp(
        ((*fake_selector).allowed_commands).as_mut_ptr() as *const libc::c_void,
        ((*selector).allowed_commands).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[uint64_t; 16]>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"CRITICAL ERROR: User ACLs don't match final bitmap: '%s'\0" as *const u8
                    as *const libc::c_char,
                rules,
            );
        }
        _serverPanic(
            b"acl.c\0" as *const u8 as *const libc::c_char,
            770 as libc::c_int,
            b"No bitmap match in ACLDescribeSelectorCommandRules()\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    return rules;
}
#[no_mangle]
pub unsafe extern "C" fn ACLDescribeSelector(mut selector: *mut aclSelector) -> sds {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut res: sds = sdsempty();
    if (*selector).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        res = sdscatlen(
            res,
            b"~* \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
    } else {
        listRewind((*selector).patterns, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut thispat: *mut keyPattern = (*ln).value as *mut keyPattern;
            res = sdsCatPatternString(res, thispat);
            res = sdscatlen(
                res,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
    if (*selector).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
        res = sdscatlen(
            res,
            b"&* \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
    } else {
        res = sdscatlen(
            res,
            b"resetchannels \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            14 as libc::c_int as size_t,
        );
        listRewind((*selector).channels, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut thispat_0: sds = (*ln).value as sds;
            res = sdscatlen(
                res,
                b"&\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            res = sdscatsds(res, thispat_0);
            res = sdscatlen(
                res,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
    let mut rules: sds = ACLDescribeSelectorCommandRules(selector);
    res = sdscatsds(res, rules);
    sdsfree(rules);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn ACLDescribeUser(mut u: *mut user) -> *mut robj {
    if !((*u).acl_string).is_null() {
        incrRefCount((*u).acl_string);
        return (*u).acl_string;
    }
    let mut res: sds = sdsempty();
    let mut j: libc::c_int = 0 as libc::c_int;
    while ACLUserFlags[j as usize].flag != 0 {
        if (*u).flags as libc::c_ulong & ACLUserFlags[j as usize].flag != 0 {
            res = sdscat(res, ACLUserFlags[j as usize].name);
            res = sdscatlen(
                res,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind((*u).passwords, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut thispass: sds = (*ln).value as sds;
        res = sdscatlen(
            res,
            b"#\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        res = sdscatsds(res, thispass);
        res = sdscatlen(
            res,
            b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    listRewind((*u).selectors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut selector: *mut aclSelector = (*ln).value as *mut aclSelector;
        let mut default_perm: sds = ACLDescribeSelector(selector);
        if (*selector).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
            res = sdscatfmt(
                res,
                b"%s\0" as *const u8 as *const libc::c_char,
                default_perm,
            );
        } else {
            res = sdscatfmt(
                res,
                b" (%s)\0" as *const u8 as *const libc::c_char,
                default_perm,
            );
        }
        sdsfree(default_perm);
    }
    (*u).acl_string = createObject(0 as libc::c_int, res as *mut libc::c_void);
    incrRefCount((*u).acl_string);
    return (*u).acl_string;
}
#[no_mangle]
pub unsafe extern "C" fn ACLLookupCommand(mut name: *const libc::c_char) -> *mut redisCommand {
    let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
    let mut sdsname: sds = sdsnew(name);
    cmd = lookupCommandBySdsLogic(server.orig_commands, sdsname);
    sdsfree(sdsname);
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn ACLResetFirstArgsForCommand(
    mut selector: *mut aclSelector,
    mut id: libc::c_ulong,
) {
    if !((*selector).allowed_firstargs).is_null()
        && !(*((*selector).allowed_firstargs).offset(id as isize)).is_null()
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        while !(*(*((*selector).allowed_firstargs).offset(id as isize)).offset(i as isize))
            .is_null()
        {
            sdsfree(*(*((*selector).allowed_firstargs).offset(id as isize)).offset(i as isize));
            i += 1;
        }
        zfree(*((*selector).allowed_firstargs).offset(id as isize) as *mut libc::c_void);
        let ref mut fresh0 = *((*selector).allowed_firstargs).offset(id as isize);
        *fresh0 = 0 as *mut sds;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ACLResetFirstArgs(mut selector: *mut aclSelector) {
    if ((*selector).allowed_firstargs).is_null() {
        return;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 1024 as libc::c_int {
        if !(*((*selector).allowed_firstargs).offset(j as isize)).is_null() {
            let mut i: libc::c_int = 0 as libc::c_int;
            while !(*(*((*selector).allowed_firstargs).offset(j as isize)).offset(i as isize))
                .is_null()
            {
                sdsfree(*(*((*selector).allowed_firstargs).offset(j as isize)).offset(i as isize));
                i += 1;
            }
            zfree(*((*selector).allowed_firstargs).offset(j as isize) as *mut libc::c_void);
        }
        j += 1;
    }
    zfree((*selector).allowed_firstargs as *mut libc::c_void);
    (*selector).allowed_firstargs = 0 as *mut *mut sds;
}
#[no_mangle]
pub unsafe extern "C" fn ACLAddAllowedFirstArg(
    mut selector: *mut aclSelector,
    mut id: libc::c_ulong,
    mut sub: *const libc::c_char,
) {
    if ((*selector).allowed_firstargs).is_null() {
        (*selector).allowed_firstargs = zcalloc(
            (1024 as libc::c_int as libc::c_ulong)
                .wrapping_mul(core::mem::size_of::<*mut sds>() as libc::c_ulong),
        ) as *mut *mut sds;
    }
    let mut items: libc::c_long = 0 as libc::c_int as libc::c_long;
    if !(*((*selector).allowed_firstargs).offset(id as isize)).is_null() {
        while !(*(*((*selector).allowed_firstargs).offset(id as isize)).offset(items as isize))
            .is_null()
        {
            if strcasecmp(
                *(*((*selector).allowed_firstargs).offset(id as isize)).offset(items as isize)
                    as *const libc::c_char,
                sub,
            ) == 0
            {
                return;
            }
            items += 1;
        }
    }
    items += 2 as libc::c_int as libc::c_long;
    let ref mut fresh1 = *((*selector).allowed_firstargs).offset(id as isize);
    *fresh1 = zrealloc(
        *((*selector).allowed_firstargs).offset(id as isize) as *mut libc::c_void,
        (core::mem::size_of::<sds>() as libc::c_ulong).wrapping_mul(items as libc::c_ulong),
    ) as *mut sds;
    let ref mut fresh2 = *(*((*selector).allowed_firstargs).offset(id as isize))
        .offset((items - 2 as libc::c_int as libc::c_long) as isize);
    *fresh2 = sdsnew(sub);
    let ref mut fresh3 = *(*((*selector).allowed_firstargs).offset(id as isize))
        .offset((items - 1 as libc::c_int as libc::c_long) as isize);
    *fresh3 = 0 as sds;
}
#[no_mangle]
pub unsafe extern "C" fn aclCreateSelectorFromOpSet(
    mut opset: *const libc::c_char,
    mut opsetlen: size_t,
) -> *mut aclSelector {
    if *opset.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32
        && *opset.offset(opsetlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == ')' as i32
    {
    } else {
        _serverAssert(
            b"opset[0] == '(' && opset[opsetlen - 1] == ')'\0" as *const u8 as *const libc::c_char,
            b"acl.c\0" as *const u8 as *const libc::c_char,
            940 as libc::c_int,
        );
        unreachable!();
    };
    let mut s: *mut aclSelector = ACLCreateSelector(0 as libc::c_int);
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut trimmed: sds = sdsnewlen(
        opset.offset(1 as libc::c_int as isize) as *const libc::c_void,
        opsetlen.wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    let mut argv: *mut sds = sdssplitargs(trimmed as *const libc::c_char, &mut argc);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc {
        if ACLSetSelector(
            s,
            *argv.offset(i as isize) as *const libc::c_char,
            sdslen(*argv.offset(i as isize)),
        ) == -(1 as libc::c_int)
        {
            ACLFreeSelector(s);
            s = 0 as *mut aclSelector;
            break;
        } else {
            i += 1;
        }
    }
    sdsfreesplitres(argv, argc);
    sdsfree(trimmed);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn ACLSetSelector(
    mut selector: *mut aclSelector,
    mut op: *const libc::c_char,
    mut oplen: size_t,
) -> libc::c_int {
    if strcasecmp(op, b"allkeys\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(op, b"~*\0" as *const u8 as *const libc::c_char) == 0
    {
        (*selector).flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        listEmpty((*selector).patterns);
    } else if strcasecmp(op, b"resetkeys\0" as *const u8 as *const libc::c_char) == 0 {
        (*selector).flags &= !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        listEmpty((*selector).patterns);
    } else if strcasecmp(op, b"allchannels\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(op, b"&*\0" as *const u8 as *const libc::c_char) == 0
    {
        (*selector).flags |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
        listEmpty((*selector).channels);
    } else if strcasecmp(op, b"resetchannels\0" as *const u8 as *const libc::c_char) == 0 {
        (*selector).flags &= !((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
        listEmpty((*selector).channels);
    } else if strcasecmp(op, b"allcommands\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(op, b"+@all\0" as *const u8 as *const libc::c_char) == 0
    {
        memset(
            ((*selector).allowed_commands).as_mut_ptr() as *mut libc::c_void,
            255 as libc::c_int,
            core::mem::size_of::<[uint64_t; 16]>() as libc::c_ulong,
        );
        (*selector).flags |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        ACLResetFirstArgs(selector);
    } else if strcasecmp(op, b"nocommands\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(op, b"-@all\0" as *const u8 as *const libc::c_char) == 0
    {
        memset(
            ((*selector).allowed_commands).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            core::mem::size_of::<[uint64_t; 16]>() as libc::c_ulong,
        );
        (*selector).flags &= !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        ACLResetFirstArgs(selector);
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
        || *op.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
    {
        if (*selector).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
            *__errno_location() = 17 as libc::c_int;
            return -(1 as libc::c_int);
        }
        let mut flags: libc::c_int = 0 as libc::c_int;
        let mut offset: size_t = 1 as libc::c_int as size_t;
        if *op.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
            while offset < oplen {
                if ({
                    let mut __res: libc::c_int = 0;
                    if core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *op.offset(offset as isize) as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(*op.offset(offset as isize) as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*op.offset(offset as isize) as libc::c_int as isize);
                    }
                    __res
                }) == 'R' as i32
                    && flags & (1 as libc::c_int) << 0 as libc::c_int == 0
                {
                    flags |= (1 as libc::c_int) << 0 as libc::c_int;
                } else if ({
                    let mut __res: libc::c_int = 0;
                    if core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *op.offset(offset as isize) as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(*op.offset(offset as isize) as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*op.offset(offset as isize) as libc::c_int as isize);
                    }
                    __res
                }) == 'W' as i32
                    && flags & (1 as libc::c_int) << 1 as libc::c_int == 0
                {
                    flags |= (1 as libc::c_int) << 1 as libc::c_int;
                } else if *op.offset(offset as isize) as libc::c_int == '~' as i32 {
                    offset = offset.wrapping_add(1);
                    break;
                } else {
                    *__errno_location() = 22 as libc::c_int;
                    return -(1 as libc::c_int);
                }
                offset = offset.wrapping_add(1);
            }
        } else {
            flags = (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int;
        }
        if ACLStringHasSpaces(op.offset(offset as isize), oplen.wrapping_sub(offset)) != 0 {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        let mut newpat: *mut keyPattern = ACLKeyPatternCreate(
            sdsnewlen(
                op.offset(offset as isize) as *const libc::c_void,
                oplen.wrapping_sub(offset),
            ),
            flags,
        );
        let mut ln: *mut listNode =
            listSearchKey((*selector).patterns, newpat as *mut libc::c_void);
        if ln.is_null() {
            listAddNodeTail((*selector).patterns, newpat as *mut libc::c_void);
        } else {
            (*((*ln).value as *mut keyPattern)).flags |= flags;
            ACLKeyPatternFree(newpat);
        }
        (*selector).flags &= !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '&' as i32 {
        if (*selector).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            *__errno_location() = 21 as libc::c_int;
            return -(1 as libc::c_int);
        }
        if ACLStringHasSpaces(
            op.offset(1 as libc::c_int as isize),
            oplen.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        let mut newpat_0: sds = sdsnewlen(
            op.offset(1 as libc::c_int as isize) as *const libc::c_void,
            oplen.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut ln_0: *mut listNode =
            listSearchKey((*selector).channels, newpat_0 as *mut libc::c_void);
        if ln_0.is_null() {
            listAddNodeTail((*selector).channels, newpat_0 as *mut libc::c_void);
        } else {
            sdsfree(newpat_0);
        }
        (*selector).flags &= !((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
        && *op.offset(1 as libc::c_int as isize) as libc::c_int != '@' as i32
    {
        if (strrchr(op, '|' as i32)).is_null() {
            let mut cmd: *mut redisCommand = ACLLookupCommand(op.offset(1 as libc::c_int as isize));
            if cmd.is_null() {
                *__errno_location() = 2 as libc::c_int;
                return -(1 as libc::c_int);
            }
            ACLChangeSelectorPerm(selector, cmd, 1 as libc::c_int);
        } else {
            let mut copy: *mut libc::c_char = zstrdup(op.offset(1 as libc::c_int as isize));
            let mut sub: *mut libc::c_char = strrchr(copy, '|' as i32);
            *sub.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            sub = sub.offset(1);
            let mut cmd_0: *mut redisCommand = ACLLookupCommand(copy);
            if cmd_0.is_null() {
                zfree(copy as *mut libc::c_void);
                *__errno_location() = 2 as libc::c_int;
                return -(1 as libc::c_int);
            }
            if !((*cmd_0).parent).is_null() {
                zfree(copy as *mut libc::c_void);
                *__errno_location() = 10 as libc::c_int;
                return -(1 as libc::c_int);
            }
            if strlen(sub) == 0 as libc::c_int as libc::c_ulong {
                zfree(copy as *mut libc::c_void);
                *__errno_location() = 22 as libc::c_int;
                return -(1 as libc::c_int);
            }
            if !((*cmd_0).subcommands_dict).is_null() {
                cmd_0 = ACLLookupCommand(op.offset(1 as libc::c_int as isize));
                if cmd_0.is_null() {
                    zfree(copy as *mut libc::c_void);
                    *__errno_location() = 2 as libc::c_int;
                    return -(1 as libc::c_int);
                }
                ACLChangeSelectorPerm(selector, cmd_0, 1 as libc::c_int);
            } else {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Deprecation warning: Allowing a first arg of an otherwise blocked command is a misuse of ACL and may get disabled in the future (offender: +%s)\0"
                            as *const u8 as *const libc::c_char,
                        op.offset(1 as libc::c_int as isize),
                    );
                }
                ACLAddAllowedFirstArg(selector, (*cmd_0).id as libc::c_ulong, sub);
            }
            zfree(copy as *mut libc::c_void);
        }
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        && *op.offset(1 as libc::c_int as isize) as libc::c_int != '@' as i32
    {
        let mut cmd_1: *mut redisCommand = ACLLookupCommand(op.offset(1 as libc::c_int as isize));
        if cmd_1.is_null() {
            *__errno_location() = 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ACLChangeSelectorPerm(selector, cmd_1, 0 as libc::c_int);
    } else if (*op.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
        || *op.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
        && *op.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
    {
        let mut bitval: libc::c_int =
            if *op.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        if ACLSetSelectorCommandBitsForCategory(
            selector,
            op.offset(2 as libc::c_int as isize),
            bitval,
        ) == -(1 as libc::c_int)
        {
            *__errno_location() = 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLSetUser(
    mut u: *mut user,
    mut op: *const libc::c_char,
    mut oplen: ssize_t,
) -> libc::c_int {
    if !((*u).acl_string).is_null() {
        decrRefCount((*u).acl_string);
        (*u).acl_string = 0 as *mut robj;
    }
    if oplen == -(1 as libc::c_int) as libc::c_long {
        oplen = strlen(op) as ssize_t;
    }
    if oplen == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    if strcasecmp(op, b"on\0" as *const u8 as *const libc::c_char) == 0 {
        (*u).flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*u).flags &= !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
    } else if strcasecmp(op, b"off\0" as *const u8 as *const libc::c_char) == 0 {
        (*u).flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        (*u).flags &= !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    } else if strcasecmp(
        op,
        b"skip-sanitize-payload\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        (*u).flags |= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        (*u).flags &= !((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    } else if strcasecmp(
        op,
        b"sanitize-payload\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        (*u).flags &= !((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        (*u).flags |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    } else if strcasecmp(op, b"nopass\0" as *const u8 as *const libc::c_char) == 0 {
        (*u).flags |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        listEmpty((*u).passwords);
    } else if strcasecmp(op, b"resetpass\0" as *const u8 as *const libc::c_char) == 0 {
        (*u).flags &= !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        listEmpty((*u).passwords);
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32
        || *op.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
    {
        let mut newpass: sds = 0 as *mut libc::c_char;
        if *op.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32 {
            newpass = ACLHashPassword(
                (op as *mut libc::c_uchar).offset(1 as libc::c_int as isize),
                (oplen - 1 as libc::c_int as libc::c_long) as size_t,
            );
        } else {
            if ACLCheckPasswordHash(
                (op as *mut libc::c_uchar).offset(1 as libc::c_int as isize),
                (oplen - 1 as libc::c_int as libc::c_long) as libc::c_int,
            ) == -(1 as libc::c_int)
            {
                *__errno_location() = 74 as libc::c_int;
                return -(1 as libc::c_int);
            }
            newpass = sdsnewlen(
                op.offset(1 as libc::c_int as isize) as *const libc::c_void,
                (oplen - 1 as libc::c_int as libc::c_long) as size_t,
            );
        }
        let mut ln: *mut listNode = listSearchKey((*u).passwords, newpass as *mut libc::c_void);
        if ln.is_null() {
            listAddNodeTail((*u).passwords, newpass as *mut libc::c_void);
        } else {
            sdsfree(newpass);
        }
        (*u).flags &= !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32
        || *op.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32
    {
        let mut delpass: sds = 0 as *mut libc::c_char;
        if *op.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
            delpass = ACLHashPassword(
                (op as *mut libc::c_uchar).offset(1 as libc::c_int as isize),
                (oplen - 1 as libc::c_int as libc::c_long) as size_t,
            );
        } else {
            if ACLCheckPasswordHash(
                (op as *mut libc::c_uchar).offset(1 as libc::c_int as isize),
                (oplen - 1 as libc::c_int as libc::c_long) as libc::c_int,
            ) == -(1 as libc::c_int)
            {
                *__errno_location() = 74 as libc::c_int;
                return -(1 as libc::c_int);
            }
            delpass = sdsnewlen(
                op.offset(1 as libc::c_int as isize) as *const libc::c_void,
                (oplen - 1 as libc::c_int as libc::c_long) as size_t,
            );
        }
        let mut ln_0: *mut listNode = listSearchKey((*u).passwords, delpass as *mut libc::c_void);
        sdsfree(delpass);
        if !ln_0.is_null() {
            listDelNode((*u).passwords, ln_0);
        } else {
            *__errno_location() = 19 as libc::c_int;
            return -(1 as libc::c_int);
        }
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32
        && *op.offset((oplen - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
            == ')' as i32
    {
        let mut selector: *mut aclSelector = aclCreateSelectorFromOpSet(op, oplen as size_t);
        if selector.is_null() {
            return -(1 as libc::c_int);
        }
        listAddNodeTail((*u).selectors, selector as *mut libc::c_void);
        return 0 as libc::c_int;
    } else {
        if strcasecmp(op, b"clearselectors\0" as *const u8 as *const libc::c_char) == 0 {
            let mut li: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            let mut ln_1: *mut listNode = 0 as *mut listNode;
            listRewind((*u).selectors, &mut li);
            if !(listNext(&mut li)).is_null() {
            } else {
                _serverAssert(
                    b"listNext(&li)\0" as *const u8 as *const libc::c_char,
                    b"acl.c\0" as *const u8 as *const libc::c_char,
                    1311 as libc::c_int,
                );
                unreachable!();
            };
            loop {
                ln_1 = listNext(&mut li);
                if ln_1.is_null() {
                    break;
                }
                listDelNode((*u).selectors, ln_1);
            }
            return 0 as libc::c_int;
        } else {
            if strcasecmp(op, b"reset\0" as *const u8 as *const libc::c_char) == 0 {
                if ACLSetUser(
                    u,
                    b"resetpass\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int) as ssize_t,
                ) == 0 as libc::c_int
                {
                } else {
                    _serverAssert(
                        b"ACLSetUser(u,\"resetpass\",-1) == C_OK\0" as *const u8
                            as *const libc::c_char,
                        b"acl.c\0" as *const u8 as *const libc::c_char,
                        1317 as libc::c_int,
                    );
                    unreachable!();
                };
                if ACLSetUser(
                    u,
                    b"resetkeys\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int) as ssize_t,
                ) == 0 as libc::c_int
                {
                } else {
                    _serverAssert(
                        b"ACLSetUser(u,\"resetkeys\",-1) == C_OK\0" as *const u8
                            as *const libc::c_char,
                        b"acl.c\0" as *const u8 as *const libc::c_char,
                        1318 as libc::c_int,
                    );
                    unreachable!();
                };
                if ACLSetUser(
                    u,
                    b"resetchannels\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int) as ssize_t,
                ) == 0 as libc::c_int
                {
                } else {
                    _serverAssert(
                        b"ACLSetUser(u,\"resetchannels\",-1) == C_OK\0" as *const u8
                            as *const libc::c_char,
                        b"acl.c\0" as *const u8 as *const libc::c_char,
                        1319 as libc::c_int,
                    );
                    unreachable!();
                };
                if server.acl_pubsub_default & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                    if ACLSetUser(
                        u,
                        b"allchannels\0" as *const u8 as *const libc::c_char,
                        -(1 as libc::c_int) as ssize_t,
                    ) == 0 as libc::c_int
                    {
                    } else {
                        _serverAssert(
                            b"ACLSetUser(u,\"allchannels\",-1) == C_OK\0" as *const u8
                                as *const libc::c_char,
                            b"acl.c\0" as *const u8 as *const libc::c_char,
                            1321 as libc::c_int,
                        );
                        unreachable!();
                    };
                }
                if ACLSetUser(
                    u,
                    b"off\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int) as ssize_t,
                ) == 0 as libc::c_int
                {
                } else {
                    _serverAssert(
                        b"ACLSetUser(u,\"off\",-1) == C_OK\0" as *const u8 as *const libc::c_char,
                        b"acl.c\0" as *const u8 as *const libc::c_char,
                        1322 as libc::c_int,
                    );
                    unreachable!();
                };
                if ACLSetUser(
                    u,
                    b"sanitize-payload\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int) as ssize_t,
                ) == 0 as libc::c_int
                {
                } else {
                    _serverAssert(
                        b"ACLSetUser(u,\"sanitize-payload\",-1) == C_OK\0" as *const u8
                            as *const libc::c_char,
                        b"acl.c\0" as *const u8 as *const libc::c_char,
                        1323 as libc::c_int,
                    );
                    unreachable!();
                };
                if ACLSetUser(
                    u,
                    b"clearselectors\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int) as ssize_t,
                ) == 0 as libc::c_int
                {
                } else {
                    _serverAssert(
                        b"ACLSetUser(u,\"clearselectors\",-1) == C_OK\0" as *const u8
                            as *const libc::c_char,
                        b"acl.c\0" as *const u8 as *const libc::c_char,
                        1324 as libc::c_int,
                    );
                    unreachable!();
                };
                if ACLSetUser(
                    u,
                    b"-@all\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int) as ssize_t,
                ) == 0 as libc::c_int
                {
                } else {
                    _serverAssert(
                        b"ACLSetUser(u,\"-@all\",-1) == C_OK\0" as *const u8 as *const libc::c_char,
                        b"acl.c\0" as *const u8 as *const libc::c_char,
                        1325 as libc::c_int,
                    );
                    unreachable!();
                };
            } else {
                let mut selector_0: *mut aclSelector = ACLUserGetRootSelector(u);
                if ACLSetSelector(selector_0, op, oplen as size_t) == -(1 as libc::c_int) {
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLSetUserStringError() -> *const libc::c_char {
    let mut errmsg: *const libc::c_char = b"Wrong format\0" as *const u8 as *const libc::c_char;
    if *__errno_location() == 2 as libc::c_int {
        errmsg = b"Unknown command or category name in ACL\0" as *const u8 as *const libc::c_char;
    } else if *__errno_location() == 22 as libc::c_int {
        errmsg = b"Syntax error\0" as *const u8 as *const libc::c_char;
    } else if *__errno_location() == 17 as libc::c_int {
        errmsg = b"Adding a pattern after the * pattern (or the 'allkeys' flag) is not valid and does not have any effect. Try 'resetkeys' to start with an empty list of patterns\0"
            as *const u8 as *const libc::c_char;
    } else if *__errno_location() == 21 as libc::c_int {
        errmsg = b"Adding a pattern after the * pattern (or the 'allchannels' flag) is not valid and does not have any effect. Try 'resetchannels' to start with an empty list of channels\0"
            as *const u8 as *const libc::c_char;
    } else if *__errno_location() == 19 as libc::c_int {
        errmsg = b"The password you are trying to remove from the user does not exist\0"
            as *const u8 as *const libc::c_char;
    } else if *__errno_location() == 74 as libc::c_int {
        errmsg = b"The password hash must be exactly 64 characters and contain only lowercase hexadecimal characters\0"
            as *const u8 as *const libc::c_char;
    } else if *__errno_location() == 114 as libc::c_int {
        errmsg = b"Duplicate user found. A user can only be defined once in config files\0"
            as *const u8 as *const libc::c_char;
    } else if *__errno_location() == 10 as libc::c_int {
        errmsg = b"Allowing first-arg of a subcommand is not supported\0" as *const u8
            as *const libc::c_char;
    }
    return errmsg;
}
#[no_mangle]
pub unsafe extern "C" fn ACLCreateDefaultUser() -> *mut user {
    let mut new: *mut user = ACLCreateUser(
        b"default\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    ACLSetUser(
        new,
        b"+@all\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int) as ssize_t,
    );
    ACLSetUser(
        new,
        b"~*\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int) as ssize_t,
    );
    ACLSetUser(
        new,
        b"&*\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int) as ssize_t,
    );
    ACLSetUser(
        new,
        b"on\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int) as ssize_t,
    );
    ACLSetUser(
        new,
        b"nopass\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int) as ssize_t,
    );
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn ACLInit() {
    Users = raxNew();
    UsersToLoad = listCreate();
    (*UsersToLoad).match_0 = Some(
        ACLListMatchLoadedUser
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    );
    ACLLog = listCreate();
    DefaultUser = ACLCreateDefaultUser();
}
#[no_mangle]
pub unsafe extern "C" fn ACLCheckUserCredentials(
    mut username: *mut robj,
    mut password: *mut robj,
) -> libc::c_int {
    let mut u: *mut user = ACLGetUserByName(
        (*username).ptr as *const libc::c_char,
        sdslen((*username).ptr as sds),
    );
    if u.is_null() {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (*u).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (*u).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        return 0 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind((*u).passwords, &mut li);
    let mut hashed: sds = ACLHashPassword(
        (*password).ptr as *mut libc::c_uchar,
        sdslen((*password).ptr as sds),
    );
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut thispass: sds = (*ln).value as sds;
        if time_independent_strcmp(hashed, thispass, 32 as libc::c_int * 2 as libc::c_int) == 0 {
            sdsfree(hashed);
            return 0 as libc::c_int;
        }
    }
    sdsfree(hashed);
    *__errno_location() = 22 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ACLAuthenticateUser(
    mut c: *mut client,
    mut username: *mut robj,
    mut password: *mut robj,
) -> libc::c_int {
    if ACLCheckUserCredentials(username, password) == 0 as libc::c_int {
        (*c).authenticated = 1 as libc::c_int;
        (*c).user = ACLGetUserByName(
            (*username).ptr as *const libc::c_char,
            sdslen((*username).ptr as sds),
        );
        moduleNotifyUserChanged(c);
        return 0 as libc::c_int;
    } else {
        addACLLogEntry(
            c,
            3 as libc::c_int,
            if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0 {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            },
            0 as libc::c_int,
            (*username).ptr as sds,
            0 as sds,
        );
        return -(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ACLGetCommandID(mut cmdname: sds) -> libc::c_ulong {
    let mut lowername: sds = sdsdup(cmdname);
    sdstolower(lowername);
    if commandId.is_null() {
        commandId = raxNew();
    }
    let mut id: *mut libc::c_void = raxFind(
        commandId,
        lowername as *mut libc::c_uchar,
        sdslen(lowername),
    );
    if id != raxNotFound {
        sdsfree(lowername);
        return id as libc::c_ulong;
    }
    raxInsert(
        commandId,
        lowername as *mut libc::c_uchar,
        strlen(lowername as *const libc::c_char),
        nextid as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    sdsfree(lowername);
    let mut thisid: libc::c_ulong = nextid;
    nextid = nextid.wrapping_add(1);
    if nextid == (1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        nextid = nextid.wrapping_add(1);
    }
    return thisid;
}
#[no_mangle]
pub unsafe extern "C" fn ACLClearCommandID() {
    if !commandId.is_null() {
        raxFree(commandId);
    }
    commandId = 0 as *mut rax;
    nextid = 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ACLGetUserByName(
    mut name: *const libc::c_char,
    mut namelen: size_t,
) -> *mut user {
    let mut myuser: *mut libc::c_void = raxFind(Users, name as *mut libc::c_uchar, namelen);
    if myuser == raxNotFound {
        return 0 as *mut user;
    }
    return myuser as *mut user;
}
unsafe extern "C" fn ACLSelectorCheckKey(
    mut selector: *mut aclSelector,
    mut key: *const libc::c_char,
    mut keylen: libc::c_int,
    mut keyspec_flags: libc::c_int,
) -> libc::c_int {
    if (*selector).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        return 0 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind((*selector).patterns, &mut li);
    let mut key_flags: libc::c_int = 0 as libc::c_int;
    if keyspec_flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 4 as libc::c_int != 0 {
        key_flags |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if keyspec_flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 6 as libc::c_int != 0 {
        key_flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if keyspec_flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0 {
        key_flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if keyspec_flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 5 as libc::c_int != 0 {
        key_flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut pattern: *mut keyPattern = (*ln).value as *mut keyPattern;
        if (*pattern).flags & key_flags != key_flags {
            continue;
        }
        let mut plen: size_t = sdslen((*pattern).pattern);
        if stringmatchlen(
            (*pattern).pattern as *const libc::c_char,
            plen as libc::c_int,
            key,
            keylen,
            0 as libc::c_int,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    return 2 as libc::c_int;
}
unsafe extern "C" fn ACLSelectorHasUnrestrictedKeyAccess(
    mut selector: *mut aclSelector,
    mut flags: libc::c_int,
) -> libc::c_int {
    if (*selector).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        return 1 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind((*selector).patterns, &mut li);
    let mut access_flags: libc::c_int = 0 as libc::c_int;
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 4 as libc::c_int != 0 {
        access_flags |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 6 as libc::c_int != 0 {
        access_flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0 {
        access_flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 5 as libc::c_int != 0 {
        access_flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut pattern: *mut keyPattern = (*ln).value as *mut keyPattern;
        if (*pattern).flags & access_flags != access_flags {
            continue;
        }
        if strcmp(
            (*pattern).pattern as *const libc::c_char,
            b"*\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ACLCheckChannelAgainstList(
    mut reference: *mut list,
    mut channel: *const libc::c_char,
    mut channellen: libc::c_int,
    mut is_pattern: libc::c_int,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(reference, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut pattern: sds = (*ln).value as sds;
        let mut plen: size_t = sdslen(pattern);
        if is_pattern != 0 && strcmp(pattern as *const libc::c_char, channel) == 0
            || is_pattern == 0
                && stringmatchlen(
                    pattern as *const libc::c_char,
                    plen as libc::c_int,
                    channel,
                    channellen,
                    0 as libc::c_int,
                ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    return 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn initACLKeyResultCache(mut cache: *mut aclKeyResultCache) {
    (*cache).keys_init = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cleanupACLKeyResultCache(mut cache: *mut aclKeyResultCache) {
    if (*cache).keys_init != 0 {
        getKeysFreeResult(&mut (*cache).keys);
    }
}
unsafe extern "C" fn ACLSelectorCheckCmd(
    mut selector: *mut aclSelector,
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut keyidxptr: *mut libc::c_int,
    mut cache: *mut aclKeyResultCache,
) -> libc::c_int {
    let mut id: uint64_t = (*cmd).id as uint64_t;
    let mut ret: libc::c_int = 0;
    if (*selector).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint == 0
        && (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 15 as libc::c_int == 0
    {
        if ACLGetSelectorCommandBit(selector, id) == 0 as libc::c_int {
            if argc < 2 as libc::c_int
                || ((*selector).allowed_firstargs).is_null()
                || (*((*selector).allowed_firstargs).offset(id as isize)).is_null()
            {
                return 1 as libc::c_int;
            }
            let mut subid: libc::c_long = 0 as libc::c_int as libc::c_long;
            loop {
                if (*(*((*selector).allowed_firstargs).offset(id as isize)).offset(subid as isize))
                    .is_null()
                {
                    return 1 as libc::c_int;
                }
                let mut idx: libc::c_int = if !((*cmd).parent).is_null() {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                if strcasecmp(
                    (**argv.offset(idx as isize)).ptr as *const libc::c_char,
                    *(*((*selector).allowed_firstargs).offset(id as isize)).offset(subid as isize)
                        as *const libc::c_char,
                ) == 0
                {
                    break;
                }
                subid += 1;
            }
        }
    }
    if (*selector).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint == 0
        && doesCommandHaveKeys(cmd) != 0
    {
        if (*cache).keys_init == 0 {
            (*cache).keys = {
                let mut init = getKeysResult {
                    keysbuf: [
                        {
                            let mut init = keyReference {
                                pos: 0 as libc::c_int,
                                flags: 0,
                            };
                            init
                        },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                        keyReference { pos: 0, flags: 0 },
                    ],
                    keys: 0 as *mut keyReference,
                    numkeys: 0 as libc::c_int,
                    size: 256 as libc::c_int,
                };
                init
            };
            getKeysFromCommandWithSpecs(cmd, argv, argc, 0 as libc::c_int, &mut (*cache).keys);
            (*cache).keys_init = 1 as libc::c_int;
        }
        let mut result: *mut getKeysResult = &mut (*cache).keys;
        let mut resultidx: *mut keyReference = (*result).keys;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < (*result).numkeys {
            let mut idx_0: libc::c_int = (*resultidx.offset(j as isize)).pos;
            ret = ACLSelectorCheckKey(
                selector,
                (**argv.offset(idx_0 as isize)).ptr as *const libc::c_char,
                sdslen((**argv.offset(idx_0 as isize)).ptr as sds) as libc::c_int,
                (*resultidx.offset(j as isize)).flags,
            );
            if ret != 0 as libc::c_int {
                if !keyidxptr.is_null() {
                    *keyidxptr = (*resultidx.offset(j as isize)).pos;
                }
                return ret;
            }
            j += 1;
        }
    }
    let channel_flags: libc::c_int = ((1 as libc::c_ulonglong) << 14 as libc::c_int
        | (1 as libc::c_ulonglong) << 12 as libc::c_int)
        as libc::c_int;
    if (*selector).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint == 0
        && doesCommandHaveChannelsWithFlags(cmd, channel_flags) != 0
    {
        let mut channels: getKeysResult = {
            let mut init = getKeysResult {
                keysbuf: [
                    {
                        let mut init = keyReference {
                            pos: 0 as libc::c_int,
                            flags: 0,
                        };
                        init
                    },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                ],
                keys: 0 as *mut keyReference,
                numkeys: 0 as libc::c_int,
                size: 256 as libc::c_int,
            };
            init
        };
        getChannelsFromCommand(cmd, argv, argc, &mut channels);
        let mut channelref: *mut keyReference = channels.keys;
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < channels.numkeys {
            let mut idx_1: libc::c_int = (*channelref.offset(j_0 as isize)).pos;
            if !((*channelref.offset(j_0 as isize)).flags & channel_flags == 0) {
                let mut is_pattern: libc::c_int = ((*channelref.offset(j_0 as isize)).flags
                    as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 11 as libc::c_int)
                    as libc::c_int;
                let mut ret_0: libc::c_int = ACLCheckChannelAgainstList(
                    (*selector).channels,
                    (**argv.offset(idx_1 as isize)).ptr as *const libc::c_char,
                    sdslen((**argv.offset(idx_1 as isize)).ptr as sds) as libc::c_int,
                    is_pattern,
                );
                if ret_0 != 0 as libc::c_int {
                    if !keyidxptr.is_null() {
                        *keyidxptr = (*channelref.offset(j_0 as isize)).pos;
                    }
                    getKeysFreeResult(&mut channels);
                    return ret_0;
                }
            }
            j_0 += 1;
        }
        getKeysFreeResult(&mut channels);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLUserCheckKeyPerm(
    mut u: *mut user,
    mut key: *const libc::c_char,
    mut keylen: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    if u.is_null() {
        return 0 as libc::c_int;
    }
    listRewind((*u).selectors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut s: *mut aclSelector = (*ln).value as *mut aclSelector;
        if ACLSelectorCheckKey(s, key, keylen, flags) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLUserCheckCmdWithUnrestrictedKeyAccess(
    mut u: *mut user,
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut local_idxptr: libc::c_int = 0;
    if u.is_null() {
        return 1 as libc::c_int;
    }
    let mut cache: aclKeyResultCache = aclKeyResultCache {
        keys_init: 0,
        keys: getKeysResult {
            keysbuf: [keyReference { pos: 0, flags: 0 }; 256],
            keys: 0 as *mut keyReference,
            numkeys: 0,
            size: 0,
        },
    };
    initACLKeyResultCache(&mut cache);
    listRewind((*u).selectors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut s: *mut aclSelector = (*ln).value as *mut aclSelector;
        let mut acl_retval: libc::c_int =
            ACLSelectorCheckCmd(s, cmd, argv, argc, &mut local_idxptr, &mut cache);
        if acl_retval == 0 as libc::c_int && ACLSelectorHasUnrestrictedKeyAccess(s, flags) != 0 {
            cleanupACLKeyResultCache(&mut cache);
            return 1 as libc::c_int;
        }
    }
    cleanupACLKeyResultCache(&mut cache);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLUserCheckChannelPerm(
    mut u: *mut user,
    mut channel: sds,
    mut is_pattern: libc::c_int,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    if u.is_null() {
        return 0 as libc::c_int;
    }
    listRewind((*u).selectors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut s: *mut aclSelector = (*ln).value as *mut aclSelector;
        if (*s).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            return 0 as libc::c_int;
        }
        if ACLCheckChannelAgainstList(
            (*s).channels,
            channel as *const libc::c_char,
            sdslen(channel) as libc::c_int,
            is_pattern,
        ) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    return 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLCheckAllUserCommandPerm(
    mut u: *mut user,
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut idxptr: *mut libc::c_int,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    if u.is_null() {
        return 0 as libc::c_int;
    }
    let mut relevant_error: libc::c_int = 1 as libc::c_int;
    let mut local_idxptr: libc::c_int = 0 as libc::c_int;
    let mut last_idx: libc::c_int = 0 as libc::c_int;
    let mut cache: aclKeyResultCache = aclKeyResultCache {
        keys_init: 0,
        keys: getKeysResult {
            keysbuf: [keyReference { pos: 0, flags: 0 }; 256],
            keys: 0 as *mut keyReference,
            numkeys: 0,
            size: 0,
        },
    };
    initACLKeyResultCache(&mut cache);
    listRewind((*u).selectors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut s: *mut aclSelector = (*ln).value as *mut aclSelector;
        let mut acl_retval: libc::c_int =
            ACLSelectorCheckCmd(s, cmd, argv, argc, &mut local_idxptr, &mut cache);
        if acl_retval == 0 as libc::c_int {
            cleanupACLKeyResultCache(&mut cache);
            return 0 as libc::c_int;
        }
        if acl_retval > relevant_error || acl_retval == relevant_error && local_idxptr > last_idx {
            relevant_error = acl_retval;
            last_idx = local_idxptr;
        }
    }
    *idxptr = last_idx;
    cleanupACLKeyResultCache(&mut cache);
    return relevant_error;
}
#[no_mangle]
pub unsafe extern "C" fn ACLCheckAllPerm(
    mut c: *mut client,
    mut idxptr: *mut libc::c_int,
) -> libc::c_int {
    return ACLCheckAllUserCommandPerm((*c).user, (*c).cmd, (*c).argv, (*c).argc, idxptr);
}
#[no_mangle]
pub unsafe extern "C" fn ACLKillPubsubClientsIfNeeded(mut new: *mut user, mut original: *mut user) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut lpi: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut lpn: *mut listNode = 0 as *mut listNode;
    let mut o: *mut robj = 0 as *mut robj;
    let mut kill: libc::c_int = 0 as libc::c_int;
    listRewind((*new).selectors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut s: *mut aclSelector = (*ln).value as *mut aclSelector;
        if (*s).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            return;
        }
    }
    let mut upcoming: *mut list = listCreate();
    listRewind((*new).selectors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut s_0: *mut aclSelector = (*ln).value as *mut aclSelector;
        listRewind((*s_0).channels, &mut lpi);
        loop {
            lpn = listNext(&mut lpi);
            if lpn.is_null() {
                break;
            }
            listAddNodeTail(upcoming, (*lpn).value);
        }
    }
    let mut match_0: libc::c_int = 1 as libc::c_int;
    listRewind((*original).selectors, &mut li);
    loop {
        ln = listNext(&mut li);
        if !(!ln.is_null() && match_0 != 0) {
            break;
        }
        let mut s_1: *mut aclSelector = (*ln).value as *mut aclSelector;
        listRewind((*s_1).channels, &mut lpi);
        loop {
            lpn = listNext(&mut lpi);
            if !(!lpn.is_null() && match_0 != 0) {
                break;
            }
            if !(listSearchKey(upcoming, (*lpn).value)).is_null() {
                continue;
            }
            match_0 = 0 as libc::c_int;
            break;
        }
    }
    if match_0 != 0 {
        listRelease(upcoming);
        return;
    }
    listRewind(server.clients, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        kill = 0 as libc::c_int;
        if (*c).user == original && getClientType(c) == 2 as libc::c_int {
            listRewind((*c).pubsub_patterns, &mut lpi);
            while kill == 0 && {
                lpn = listNext(&mut lpi);
                !lpn.is_null()
            } {
                o = (*lpn).value as *mut robj;
                let mut res: libc::c_int = ACLCheckChannelAgainstList(
                    upcoming,
                    (*o).ptr as *const libc::c_char,
                    sdslen((*o).ptr as sds) as libc::c_int,
                    1 as libc::c_int,
                );
                kill = (res == 4 as libc::c_int) as libc::c_int;
            }
            if kill == 0 {
                let mut di: *mut dictIterator = dictGetIterator((*c).pubsub_channels);
                let mut de: *mut dictEntry = 0 as *mut dictEntry;
                while kill == 0 && {
                    de = dictNext(di);
                    !de.is_null()
                } {
                    o = (*de).key as *mut robj;
                    let mut res_0: libc::c_int = ACLCheckChannelAgainstList(
                        upcoming,
                        (*o).ptr as *const libc::c_char,
                        sdslen((*o).ptr as sds) as libc::c_int,
                        0 as libc::c_int,
                    );
                    kill = (res_0 == 4 as libc::c_int) as libc::c_int;
                }
                dictReleaseIterator(di);
                di = dictGetIterator((*c).pubsubshard_channels);
                while kill == 0 && {
                    de = dictNext(di);
                    !de.is_null()
                } {
                    o = (*de).key as *mut robj;
                    let mut res_1: libc::c_int = ACLCheckChannelAgainstList(
                        upcoming,
                        (*o).ptr as *const libc::c_char,
                        sdslen((*o).ptr as sds) as libc::c_int,
                        0 as libc::c_int,
                    );
                    kill = (res_1 == 4 as libc::c_int) as libc::c_int;
                }
                dictReleaseIterator(di);
            }
            if kill != 0 {
                freeClient(c);
            }
        }
    }
    listRelease(upcoming);
}
#[no_mangle]
pub unsafe extern "C" fn ACLMergeSelectorArguments(
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut merged_argc: *mut libc::c_int,
    mut invalid_idx: *mut libc::c_int,
) -> *mut sds {
    *merged_argc = 0 as libc::c_int;
    let mut open_bracket_start: libc::c_int = -(1 as libc::c_int);
    let mut acl_args: *mut sds = zmalloc(
        (core::mem::size_of::<sds>() as libc::c_ulong).wrapping_mul(argc as libc::c_ulong),
    ) as *mut sds;
    let mut selector: sds = 0 as sds;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < argc {
        let mut op: *mut libc::c_char = *argv.offset(j as isize);
        if *op.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32
            && *op.offset((sdslen(op)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != ')' as i32
        {
            selector = sdsdup(*argv.offset(j as isize));
            open_bracket_start = j;
        } else if open_bracket_start != -(1 as libc::c_int) {
            selector = sdscatfmt(selector, b" %s\0" as *const u8 as *const libc::c_char, op);
            if *op.offset((sdslen(op)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == ')' as i32
            {
                open_bracket_start = -(1 as libc::c_int);
                let ref mut fresh4 = *acl_args.offset(*merged_argc as isize);
                *fresh4 = selector;
                *merged_argc += 1;
            }
        } else {
            let ref mut fresh5 = *acl_args.offset(*merged_argc as isize);
            *fresh5 = sdsdup(*argv.offset(j as isize));
            *merged_argc += 1;
        }
        j += 1;
    }
    if open_bracket_start != -(1 as libc::c_int) {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < *merged_argc {
            sdsfree(*acl_args.offset(i as isize));
            i += 1;
        }
        zfree(acl_args as *mut libc::c_void);
        sdsfree(selector);
        if !invalid_idx.is_null() {
            *invalid_idx = open_bracket_start;
        }
        return 0 as *mut sds;
    }
    return acl_args;
}
#[no_mangle]
pub unsafe extern "C" fn ACLStringSetUser(
    mut u: *mut user,
    mut username: sds,
    mut argv: *mut sds,
    mut argc: libc::c_int,
) -> sds {
    let mut current_block: u64;
    if !u.is_null() || !username.is_null() {
    } else {
        _serverAssert(
            b"u != NULL || username != NULL\0" as *const u8 as *const libc::c_char,
            b"acl.c\0" as *const u8 as *const libc::c_char,
            1975 as libc::c_int,
        );
        unreachable!();
    };
    let mut error: sds = 0 as sds;
    let mut merged_argc: libc::c_int = 0 as libc::c_int;
    let mut invalid_idx: libc::c_int = 0 as libc::c_int;
    let mut acl_args: *mut sds =
        ACLMergeSelectorArguments(argv, argc, &mut merged_argc, &mut invalid_idx);
    if acl_args.is_null() {
        return sdscatfmt(
            sdsempty(),
            b"Unmatched parenthesis in acl selector starting at '%s'.\0" as *const u8
                as *const libc::c_char,
            *argv.offset(invalid_idx as isize),
        );
    }
    let mut tempu: *mut user = ACLCreateUnlinkedUser();
    if !u.is_null() {
        ACLCopyUser(tempu, u);
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    loop {
        if !(j < merged_argc) {
            current_block = 17965632435239708295;
            break;
        }
        if ACLSetUser(
            tempu,
            *acl_args.offset(j as isize) as *const libc::c_char,
            sdslen(*acl_args.offset(j as isize)) as ssize_t,
        ) != 0 as libc::c_int
        {
            let mut errmsg: *const libc::c_char = ACLSetUserStringError();
            error = sdscatfmt(
                sdsempty(),
                b"Error in ACL SETUSER modifier '%s': %s\0" as *const u8 as *const libc::c_char,
                *acl_args.offset(j as isize),
                errmsg,
            );
            current_block = 3700939349979128670;
            break;
        } else {
            j += 1;
        }
    }
    match current_block {
        17965632435239708295 => {
            if !u.is_null() {
                ACLKillPubsubClientsIfNeeded(tempu, u);
            }
            if u.is_null() {
                u = ACLCreateUser(username as *const libc::c_char, sdslen(username));
            }
            if !u.is_null() {
            } else {
                _serverAssert(
                    b"u != NULL\0" as *const u8 as *const libc::c_char,
                    b"acl.c\0" as *const u8 as *const libc::c_char,
                    2017 as libc::c_int,
                );
                unreachable!();
            };
            ACLCopyUser(u, tempu);
        }
        _ => {}
    }
    ACLFreeUser(tempu);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < merged_argc {
        sdsfree(*acl_args.offset(i as isize));
        i += 1;
    }
    zfree(acl_args as *mut libc::c_void);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn ACLAppendUserForLoading(
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut argc_err: *mut libc::c_int,
) -> libc::c_int {
    if argc < 2 as libc::c_int
        || strcasecmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"user\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        if !argc_err.is_null() {
            *argc_err = 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if !(listSearchKey(
        UsersToLoad,
        *argv.offset(1 as libc::c_int as isize) as *mut libc::c_void,
    ))
    .is_null()
    {
        if !argc_err.is_null() {
            *argc_err = 1 as libc::c_int;
        }
        *__errno_location() = 114 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut fakeuser: *mut user = ACLCreateUnlinkedUser();
    let mut merged_argc: libc::c_int = 0;
    let mut acl_args: *mut sds = ACLMergeSelectorArguments(
        argv.offset(2 as libc::c_int as isize),
        argc - 2 as libc::c_int,
        &mut merged_argc,
        argc_err,
    );
    if acl_args.is_null() {
        return -(1 as libc::c_int);
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < merged_argc {
        if ACLSetUser(
            fakeuser,
            *acl_args.offset(j as isize) as *const libc::c_char,
            sdslen(*acl_args.offset(j as isize)) as ssize_t,
        ) == -(1 as libc::c_int)
        {
            if *__errno_location() != 2 as libc::c_int {
                ACLFreeUser(fakeuser);
                if !argc_err.is_null() {
                    *argc_err = j;
                }
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < merged_argc {
                    sdsfree(*acl_args.offset(i as isize));
                    i += 1;
                }
                zfree(acl_args as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
        }
        j += 1;
    }
    let mut copy: *mut sds = zmalloc(
        (core::mem::size_of::<sds>() as libc::c_ulong)
            .wrapping_mul((merged_argc + 2 as libc::c_int) as libc::c_ulong),
    ) as *mut sds;
    let ref mut fresh6 = *copy.offset(0 as libc::c_int as isize);
    *fresh6 = sdsdup(*argv.offset(1 as libc::c_int as isize));
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < merged_argc {
        let ref mut fresh7 = *copy.offset((j_0 + 1 as libc::c_int) as isize);
        *fresh7 = sdsdup(*acl_args.offset(j_0 as isize));
        j_0 += 1;
    }
    let ref mut fresh8 = *copy.offset((merged_argc + 1 as libc::c_int) as isize);
    *fresh8 = 0 as sds;
    listAddNodeTail(UsersToLoad, copy as *mut libc::c_void);
    ACLFreeUser(fakeuser);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < merged_argc {
        sdsfree(*acl_args.offset(i_0 as isize));
        i_0 += 1;
    }
    zfree(acl_args as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLLoadConfiguredUsers() -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(UsersToLoad, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut aclrules: *mut sds = (*ln).value as *mut sds;
        let mut username: sds = *aclrules.offset(0 as libc::c_int as isize);
        if ACLStringHasSpaces(
            *aclrules.offset(0 as libc::c_int as isize) as *const libc::c_char,
            sdslen(*aclrules.offset(0 as libc::c_int as isize)),
        ) != 0
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Spaces not allowed in ACL usernames\0" as *const u8 as *const libc::c_char,
                );
            }
            return -(1 as libc::c_int);
        }
        let mut u: *mut user = ACLCreateUser(username as *const libc::c_char, sdslen(username));
        if u.is_null() {
            if strcmp(
                username as *const libc::c_char,
                b"default\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
            } else {
                _serverAssert(
                    b"!strcmp(username, \"default\")\0" as *const u8 as *const libc::c_char,
                    b"acl.c\0" as *const u8 as *const libc::c_char,
                    2116 as libc::c_int,
                );
                unreachable!();
            };
            u = ACLGetUserByName(
                b"default\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as size_t,
            );
            ACLSetUser(
                u,
                b"reset\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int) as ssize_t,
            );
        }
        let mut j: libc::c_int = 1 as libc::c_int;
        while !(*aclrules.offset(j as isize)).is_null() {
            if ACLSetUser(
                u,
                *aclrules.offset(j as isize) as *const libc::c_char,
                sdslen(*aclrules.offset(j as isize)) as ssize_t,
            ) != 0 as libc::c_int
            {
                let mut errmsg: *const libc::c_char = ACLSetUserStringError();
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Error loading ACL rule '%s' for the user named '%s': %s\0" as *const u8
                            as *const libc::c_char,
                        *aclrules.offset(j as isize),
                        *aclrules.offset(0 as libc::c_int as isize),
                        errmsg,
                    );
                }
                return -(1 as libc::c_int);
            }
            j += 1;
        }
        if (*u).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"The user '%s' is disabled (there is no 'on' modifier in the user description). Make sure this is not a configuration error.\0"
                        as *const u8 as *const libc::c_char,
                    *aclrules.offset(0 as libc::c_int as isize),
                );
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLLoadFromFile(mut filename: *const libc::c_char) -> sds {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        let mut errors: sds = sdscatprintf(
            sdsempty(),
            b"Error loading ACLs, opening file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
        return errors;
    }
    let mut acls: sds = sdsempty();
    while !(fgets(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
    .is_null()
    {
        acls = sdscat(acls, buf.as_mut_ptr());
    }
    fclose(fp);
    let mut totlines: libc::c_int = 0;
    let mut lines: *mut sds = 0 as *mut sds;
    let mut errors_0: sds = sdsempty();
    lines = sdssplitlen(
        acls as *const libc::c_char,
        strlen(acls as *const libc::c_char) as ssize_t,
        b"\n\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        &mut totlines,
    );
    sdsfree(acls);
    let mut old_users: *mut rax = Users;
    Users = raxNew();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < totlines {
        let mut argv: *mut sds = 0 as *mut sds;
        let mut argc: libc::c_int = 0;
        let mut linenum: libc::c_int = i + 1 as libc::c_int;
        let ref mut fresh9 = *lines.offset(i as isize);
        *fresh9 = sdstrim(
            *lines.offset(i as isize),
            b" \t\r\n\0" as *const u8 as *const libc::c_char,
        );
        if !(*(*lines.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32)
        {
            argv = sdssplitlen(
                *lines.offset(i as isize) as *const libc::c_char,
                sdslen(*lines.offset(i as isize)) as ssize_t,
                b" \0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                &mut argc,
            );
            if argv.is_null() {
                errors_0 = sdscatprintf(
                    errors_0,
                    b"%s:%d: unbalanced quotes in acl line. \0" as *const u8 as *const libc::c_char,
                    server.acl_filename,
                    linenum,
                );
            } else if argc == 0 as libc::c_int {
                sdsfreesplitres(argv, argc);
            } else if strcmp(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                b"user\0" as *const u8 as *const libc::c_char,
            ) != 0
                || argc < 2 as libc::c_int
            {
                errors_0 = sdscatprintf(
                    errors_0,
                    b"%s:%d should start with user keyword followed by the username. \0"
                        as *const u8 as *const libc::c_char,
                    server.acl_filename,
                    linenum,
                );
                sdsfreesplitres(argv, argc);
            } else if ACLStringHasSpaces(
                *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                sdslen(*argv.offset(1 as libc::c_int as isize)),
            ) != 0
            {
                errors_0 = sdscatprintf(
                    errors_0,
                    b"'%s:%d: username '%s' contains invalid characters. \0" as *const u8
                        as *const libc::c_char,
                    server.acl_filename,
                    linenum,
                    *argv.offset(1 as libc::c_int as isize),
                );
                sdsfreesplitres(argv, argc);
            } else {
                let mut u: *mut user = ACLCreateUser(
                    *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                    sdslen(*argv.offset(1 as libc::c_int as isize)),
                );
                if u.is_null() {
                    errors_0 = sdscatprintf(
                        errors_0,
                        b"WARNING: Duplicate user '%s' found on line %d. \0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(1 as libc::c_int as isize),
                        linenum,
                    );
                    sdsfreesplitres(argv, argc);
                } else {
                    let mut merged_argc: libc::c_int = 0;
                    let mut acl_args: *mut sds = ACLMergeSelectorArguments(
                        argv.offset(2 as libc::c_int as isize),
                        argc - 2 as libc::c_int,
                        &mut merged_argc,
                        0 as *mut libc::c_int,
                    );
                    if acl_args.is_null() {
                        errors_0 = sdscatprintf(
                            errors_0,
                            b"%s:%d: Unmatched parenthesis in selector definition.\0" as *const u8
                                as *const libc::c_char,
                            server.acl_filename,
                            linenum,
                        );
                    }
                    let mut syntax_error: libc::c_int = 0 as libc::c_int;
                    let mut j: libc::c_int = 0 as libc::c_int;
                    while j < merged_argc {
                        let ref mut fresh10 = *acl_args.offset(j as isize);
                        *fresh10 = sdstrim(
                            *acl_args.offset(j as isize),
                            b"\t\r\n\0" as *const u8 as *const libc::c_char,
                        );
                        if ACLSetUser(
                            u,
                            *acl_args.offset(j as isize) as *const libc::c_char,
                            sdslen(*acl_args.offset(j as isize)) as ssize_t,
                        ) != 0 as libc::c_int
                        {
                            let mut errmsg: *const libc::c_char = ACLSetUserStringError();
                            if *__errno_location() == 2 as libc::c_int {
                                errors_0 = sdscatprintf(
                                    errors_0,
                                    b"%s:%d: Error in applying operation '%s': %s. \0" as *const u8
                                        as *const libc::c_char,
                                    server.acl_filename,
                                    linenum,
                                    *acl_args.offset(j as isize),
                                    errmsg,
                                );
                            } else if syntax_error == 0 as libc::c_int {
                                errors_0 = sdscatprintf(
                                    errors_0,
                                    b"%s:%d: %s. \0" as *const u8 as *const libc::c_char,
                                    server.acl_filename,
                                    linenum,
                                    errmsg,
                                );
                                syntax_error = 1 as libc::c_int;
                            }
                        }
                        j += 1;
                    }
                    let mut i_0: libc::c_int = 0 as libc::c_int;
                    while i_0 < merged_argc {
                        sdsfree(*acl_args.offset(i_0 as isize));
                        i_0 += 1;
                    }
                    zfree(acl_args as *mut libc::c_void);
                    if sdslen(errors_0) != 0 as libc::c_int as libc::c_ulong {
                        sdsfreesplitres(argv, argc);
                    } else {
                        sdsfreesplitres(argv, argc);
                    }
                }
            }
        }
        i += 1;
    }
    sdsfreesplitres(lines, totlines);
    if sdslen(errors_0) == 0 as libc::c_int as libc::c_ulong {
        let mut new_default: *mut user = ACLGetUserByName(
            b"default\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        if new_default.is_null() {
            new_default = ACLCreateDefaultUser();
        }
        ACLCopyUser(DefaultUser, new_default);
        ACLFreeUser(new_default);
        raxInsert(
            Users,
            b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
            7 as libc::c_int as size_t,
            DefaultUser as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        raxRemove(
            old_users,
            b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
            7 as libc::c_int as size_t,
            0 as *mut *mut libc::c_void,
        );
        ACLFreeUsersSet(old_users);
        sdsfree(errors_0);
        return 0 as sds;
    } else {
        ACLFreeUsersSet(Users);
        Users = old_users;
        errors_0 = sdscat(
            errors_0,
            b"WARNING: ACL errors detected, no change to the previously active ACL rules was performed\0"
                as *const u8 as *const libc::c_char,
        );
        return errors_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ACLSaveToFile(mut filename: *const libc::c_char) -> libc::c_int {
    let mut acl: sds = sdsempty();
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut tmpfilename: sds = 0 as sds;
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut ri: raxIterator = raxIterator {
        flags: 0,
        rt: 0 as *mut rax,
        key: 0 as *mut libc::c_uchar,
        data: 0 as *mut libc::c_void,
        key_len: 0,
        key_max: 0,
        key_static_string: [0; 128],
        node: 0 as *mut raxNode,
        stack: raxStack {
            stack: 0 as *mut *mut libc::c_void,
            items: 0,
            maxitems: 0,
            static_items: [0 as *mut libc::c_void; 32],
            oom: 0,
        },
        node_cb: None,
    };
    raxStart(&mut ri, Users);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        let mut u: *mut user = ri.data as *mut user;
        let mut user: sds = sdsnew(b"user \0" as *const u8 as *const libc::c_char);
        user = sdscatsds(user, (*u).name);
        user = sdscatlen(
            user,
            b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        let mut descr: *mut robj = ACLDescribeUser(u);
        user = sdscatsds(user, (*descr).ptr as sds);
        decrRefCount(descr);
        acl = sdscatsds(acl, user);
        acl = sdscatlen(
            acl,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        sdsfree(user);
    }
    raxStop(&mut ri);
    tmpfilename = sdsnew(filename);
    tmpfilename = sdscatfmt(
        tmpfilename,
        b".tmp-%i-%I\0" as *const u8 as *const libc::c_char,
        getpid(),
        mstime() as libc::c_int,
    );
    fd = open(
        tmpfilename as *const libc::c_char,
        0o1 as libc::c_int | 0o100 as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Opening temp ACL file for ACL SAVE: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    } else if write(fd, acl as *const libc::c_void, sdslen(acl)) != sdslen(acl) as ssize_t {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Writing ACL file for ACL SAVE: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    } else {
        close(fd);
        fd = -(1 as libc::c_int);
        if rename(tmpfilename as *const libc::c_char, filename) == -(1 as libc::c_int) {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Renaming ACL file for ACL SAVE: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
        } else {
            sdsfree(tmpfilename);
            tmpfilename = 0 as sds;
            retval = 0 as libc::c_int;
        }
    }
    if fd != -(1 as libc::c_int) {
        close(fd);
    }
    if !tmpfilename.is_null() {
        unlink(tmpfilename as *const libc::c_char);
    }
    sdsfree(tmpfilename);
    sdsfree(acl);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn ACLLoadUsersAtStartup() {
    if *(server.acl_filename).offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        && (*UsersToLoad).len != 0 as libc::c_int as libc::c_ulong
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Configuring Redis with users defined in redis.conf and at the same setting an ACL file path is invalid. This setup is very likely to lead to configuration errors and security holes, please define either an ACL file or declare users directly in your redis.conf, but not both.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    if ACLLoadConfiguredUsers() == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Critical error while loading ACLs. Exiting.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    if *(server.acl_filename).offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        let mut errors: sds = ACLLoadFromFile(server.acl_filename);
        if !errors.is_null() {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Aborting Redis startup because of ACL errors: %s\0" as *const u8
                        as *const libc::c_char,
                    errors,
                );
            }
            sdsfree(errors);
            exit(1 as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ACLLogMatchEntry(
    mut a: *mut ACLLogEntry,
    mut b: *mut ACLLogEntry,
) -> libc::c_int {
    if (*a).reason != (*b).reason {
        return 0 as libc::c_int;
    }
    if (*a).context != (*b).context {
        return 0 as libc::c_int;
    }
    let mut delta: mstime_t = (*a).ctime - (*b).ctime;
    if delta < 0 as libc::c_int as libc::c_longlong {
        delta = -delta;
    }
    if delta > 60000 as libc::c_int as libc::c_longlong {
        return 0 as libc::c_int;
    }
    if sdscmp((*a).object, (*b).object) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if sdscmp((*a).username, (*b).username) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ACLFreeLogEntry(mut leptr: *mut libc::c_void) {
    let mut le: *mut ACLLogEntry = leptr as *mut ACLLogEntry;
    sdsfree((*le).object);
    sdsfree((*le).username);
    sdsfree((*le).cinfo);
    zfree(le as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn addACLLogEntry(
    mut c: *mut client,
    mut reason: libc::c_int,
    mut context: libc::c_int,
    mut argpos: libc::c_int,
    mut username: sds,
    mut object: sds,
) {
    let mut le: *mut ACLLogEntry =
        zmalloc(core::mem::size_of::<ACLLogEntry>() as libc::c_ulong) as *mut ACLLogEntry;
    (*le).count = 1 as libc::c_int as uint64_t;
    (*le).reason = reason;
    (*le).username = sdsdup(if !username.is_null() {
        username
    } else {
        (*(*c).user).name
    });
    (*le).ctime = mstime();
    if !object.is_null() {
        (*le).object = object;
    } else {
        match reason {
            1 => {
                (*le).object = sdsdup((*(*c).cmd).fullname);
            }
            2 => {
                (*le).object = sdsdup((**((*c).argv).offset(argpos as isize)).ptr as sds);
            }
            4 => {
                (*le).object = sdsdup((**((*c).argv).offset(argpos as isize)).ptr as sds);
            }
            3 => {
                (*le).object = sdsdup((**((*c).argv).offset(0 as libc::c_int as isize)).ptr as sds);
            }
            _ => {
                (*le).object = sdsempty();
            }
        }
    }
    let mut realclient: *mut client = c;
    if (*realclient).flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0 {
        realclient = server.script_caller;
    }
    (*le).cinfo = catClientInfoString(sdsempty(), realclient);
    (*le).context = context;
    let mut toscan: libc::c_long = 10 as libc::c_int as libc::c_long;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(ACLLog, &mut li);
    let mut match_0: *mut ACLLogEntry = 0 as *mut ACLLogEntry;
    loop {
        let fresh11 = toscan;
        toscan = toscan - 1;
        if !(fresh11 != 0 && {
            ln = listNext(&mut li);
            !ln.is_null()
        }) {
            break;
        }
        let mut current: *mut ACLLogEntry = (*ln).value as *mut ACLLogEntry;
        if !(ACLLogMatchEntry(current, le) != 0) {
            continue;
        }
        match_0 = current;
        listDelNode(ACLLog, ln);
        listAddNodeHead(ACLLog, current as *mut libc::c_void);
        break;
    }
    if !match_0.is_null() {
        sdsfree((*match_0).cinfo);
        (*match_0).cinfo = (*le).cinfo;
        (*match_0).ctime = (*le).ctime;
        (*match_0).count = ((*match_0).count).wrapping_add(1);
        (*le).cinfo = 0 as sds;
        ACLFreeLogEntry(le as *mut libc::c_void);
    } else {
        listAddNodeHead(ACLLog, le as *mut libc::c_void);
        while (*ACLLog).len > server.acllog_max_len {
            let mut ln_0: *mut listNode = (*ACLLog).tail;
            let mut le_0: *mut ACLLogEntry = (*ln_0).value as *mut ACLLogEntry;
            ACLFreeLogEntry(le_0 as *mut libc::c_void);
            listDelNode(ACLLog, ln_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn getAclErrorMessage(mut acl_res: libc::c_int) -> *const libc::c_char {
    match acl_res {
        1 => {
            return b"can't run this command or subcommand\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            return b"can't access at least one of the keys mentioned in the command arguments\0"
                as *const u8 as *const libc::c_char;
        }
        4 => {
            return b"can't publish to the channel mentioned in the command\0" as *const u8
                as *const libc::c_char;
        }
        _ => {
            return b"lacking the permissions for the command\0" as *const u8
                as *const libc::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn aclCatWithFlags(
    mut c: *mut client,
    mut commands: *mut dict,
    mut cflag: uint64_t,
    mut arraylen: *mut libc::c_int,
) {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = dictGetIterator(commands);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cmd: *mut redisCommand = (*de).v.val as *mut redisCommand;
        if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0 {
            continue;
        }
        if (*cmd).acl_categories & cflag != 0 {
            addReplyBulkCBuffer(
                c,
                (*cmd).fullname as *const libc::c_void,
                sdslen((*cmd).fullname),
            );
            *arraylen += 1;
        }
        if !((*cmd).subcommands_dict).is_null() {
            aclCatWithFlags(c, (*cmd).subcommands_dict, cflag, arraylen);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn aclAddReplySelectorDescription(
    mut c: *mut client,
    mut s: *mut aclSelector,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    addReplyBulkCString(c, b"commands\0" as *const u8 as *const libc::c_char);
    let mut cmddescr: sds = ACLDescribeSelectorCommandRules(s);
    addReplyBulkSds(c, cmddescr);
    addReplyBulkCString(c, b"keys\0" as *const u8 as *const libc::c_char);
    if (*s).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        addReplyBulkCBuffer(
            c,
            b"~*\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
    } else {
        let mut dsl: sds = sdsempty();
        listRewind((*s).patterns, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut thispat: *mut keyPattern = (*ln).value as *mut keyPattern;
            if ln != (*(*s).patterns).head {
                dsl = sdscat(dsl, b" \0" as *const u8 as *const libc::c_char);
            }
            dsl = sdsCatPatternString(dsl, thispat);
        }
        addReplyBulkSds(c, dsl);
    }
    addReplyBulkCString(c, b"channels\0" as *const u8 as *const libc::c_char);
    if (*s).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
        addReplyBulkCBuffer(
            c,
            b"&*\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
    } else {
        let mut dsl_0: sds = sdsempty();
        listRewind((*s).channels, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut thispat_0: sds = (*ln).value as sds;
            if ln != (*(*s).channels).head {
                dsl_0 = sdscat(dsl_0, b" \0" as *const u8 as *const libc::c_char);
            }
            dsl_0 = sdscatfmt(
                dsl_0,
                b"&%S\0" as *const u8 as *const libc::c_char,
                thispat_0,
            );
        }
        addReplyBulkSds(c, dsl_0);
    }
    return 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aclCommand(mut c: *mut client) {
    let mut sub: *mut libc::c_char =
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *mut libc::c_char;
    if strcasecmp(sub, b"setuser\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc >= 3 as libc::c_int
    {
        let mut j: libc::c_int = 2 as libc::c_int;
        while j < (*c).argc {
            redactClientCommandArgument(c, j);
            j += 1;
        }
        let mut username: sds = (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds;
        if ACLStringHasSpaces(username as *const libc::c_char, sdslen(username)) != 0 {
            addReplyErrorFormat(
                c,
                b"Usernames can't contain spaces or null characters\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        let mut u: *mut user = ACLGetUserByName(username as *const libc::c_char, sdslen(username));
        let mut temp_argv: *mut sds = zmalloc(
            ((*c).argc as libc::c_ulong)
                .wrapping_mul(core::mem::size_of::<sds>() as libc::c_ulong),
        ) as *mut sds;
        let mut i: libc::c_int = 3 as libc::c_int;
        while i < (*c).argc {
            let ref mut fresh12 = *temp_argv.offset((i - 3 as libc::c_int) as isize);
            *fresh12 = (**((*c).argv).offset(i as isize)).ptr as sds;
            i += 1;
        }
        let mut error: sds = ACLStringSetUser(u, username, temp_argv, (*c).argc - 3 as libc::c_int);
        zfree(temp_argv as *mut libc::c_void);
        if error.is_null() {
            addReply(c, shared.ok);
        } else {
            addReplyErrorSdsSafe(c, error);
        }
        return;
    } else {
        if strcasecmp(sub, b"deluser\0" as *const u8 as *const libc::c_char) == 0
            && (*c).argc >= 3 as libc::c_int
        {
            let mut deleted: libc::c_int = 0 as libc::c_int;
            let mut j_0: libc::c_int = 2 as libc::c_int;
            while j_0 < (*c).argc {
                let mut username_0: sds = (**((*c).argv).offset(j_0 as isize)).ptr as sds;
                if strcmp(
                    username_0 as *const libc::c_char,
                    b"default\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    addReplyError(
                        c,
                        b"The 'default' user cannot be removed\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                j_0 += 1;
            }
            let mut j_1: libc::c_int = 2 as libc::c_int;
            while j_1 < (*c).argc {
                let mut username_1: sds = (**((*c).argv).offset(j_1 as isize)).ptr as sds;
                let mut u_0: *mut user = 0 as *mut user;
                if raxRemove(
                    Users,
                    username_1 as *mut libc::c_uchar,
                    sdslen(username_1),
                    &mut u_0 as *mut *mut user as *mut *mut libc::c_void,
                ) != 0
                {
                    ACLFreeUserAndKillClients(u_0);
                    deleted += 1;
                }
                j_1 += 1;
            }
            addReplyLongLong(c, deleted as libc::c_longlong);
        } else if strcasecmp(sub, b"getuser\0" as *const u8 as *const libc::c_char) == 0
            && (*c).argc == 3 as libc::c_int
        {
            let mut u_1: *mut user = ACLGetUserByName(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
                sdslen((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds),
            );
            if u_1.is_null() {
                addReplyNull(c);
                return;
            }
            let mut ufields: *mut libc::c_void = addReplyDeferredLen(c);
            let mut fields: libc::c_int = 3 as libc::c_int;
            addReplyBulkCString(c, b"flags\0" as *const u8 as *const libc::c_char);
            let mut deflen: *mut libc::c_void = addReplyDeferredLen(c);
            let mut numflags: libc::c_int = 0 as libc::c_int;
            let mut j_2: libc::c_int = 0 as libc::c_int;
            while ACLUserFlags[j_2 as usize].flag != 0 {
                if (*u_1).flags as libc::c_ulong & ACLUserFlags[j_2 as usize].flag != 0 {
                    addReplyBulkCString(c, ACLUserFlags[j_2 as usize].name);
                    numflags += 1;
                }
                j_2 += 1;
            }
            setDeferredSetLen(c, deflen, numflags as libc::c_long);
            addReplyBulkCString(c, b"passwords\0" as *const u8 as *const libc::c_char);
            addReplyArrayLen(c, (*(*u_1).passwords).len as libc::c_long);
            let mut li: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            let mut ln: *mut listNode = 0 as *mut listNode;
            listRewind((*u_1).passwords, &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut thispass: sds = (*ln).value as sds;
                addReplyBulkCBuffer(c, thispass as *const libc::c_void, sdslen(thispass));
            }
            fields += aclAddReplySelectorDescription(c, ACLUserGetRootSelector(u_1));
            addReplyBulkCString(c, b"selectors\0" as *const u8 as *const libc::c_char);
            addReplyArrayLen(
                c,
                ((*(*u_1).selectors).len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_long,
            );
            listRewind((*u_1).selectors, &mut li);
            if !(listNext(&mut li)).is_null() {
            } else {
                _serverAssert(
                    b"listNext(&li)\0" as *const u8 as *const libc::c_char,
                    b"acl.c\0" as *const u8 as *const libc::c_char,
                    2739 as libc::c_int,
                );
                unreachable!();
            };
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut slen: *mut libc::c_void = addReplyDeferredLen(c);
                let mut sfields: libc::c_int =
                    aclAddReplySelectorDescription(c, (*ln).value as *mut aclSelector);
                setDeferredMapLen(c, slen, sfields as libc::c_long);
            }
            setDeferredMapLen(c, ufields, fields as libc::c_long);
        } else if (strcasecmp(sub, b"list\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(sub, b"users\0" as *const u8 as *const libc::c_char) == 0)
            && (*c).argc == 2 as libc::c_int
        {
            let mut justnames: libc::c_int =
                (strcasecmp(sub, b"users\0" as *const u8 as *const libc::c_char) == 0)
                    as libc::c_int;
            addReplyArrayLen(c, raxSize(Users) as libc::c_long);
            let mut ri: raxIterator = raxIterator {
                flags: 0,
                rt: 0 as *mut rax,
                key: 0 as *mut libc::c_uchar,
                data: 0 as *mut libc::c_void,
                key_len: 0,
                key_max: 0,
                key_static_string: [0; 128],
                node: 0 as *mut raxNode,
                stack: raxStack {
                    stack: 0 as *mut *mut libc::c_void,
                    items: 0,
                    maxitems: 0,
                    static_items: [0 as *mut libc::c_void; 32],
                    oom: 0,
                },
                node_cb: None,
            };
            raxStart(&mut ri, Users);
            raxSeek(
                &mut ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri) != 0 {
                let mut u_2: *mut user = ri.data as *mut user;
                if justnames != 0 {
                    addReplyBulkCBuffer(c, (*u_2).name as *const libc::c_void, sdslen((*u_2).name));
                } else {
                    let mut config: sds = sdsnew(b"user \0" as *const u8 as *const libc::c_char);
                    config = sdscatsds(config, (*u_2).name);
                    config = sdscatlen(
                        config,
                        b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                        1 as libc::c_int as size_t,
                    );
                    let mut descr: *mut robj = ACLDescribeUser(u_2);
                    config = sdscatsds(config, (*descr).ptr as sds);
                    decrRefCount(descr);
                    addReplyBulkSds(c, config);
                }
            }
            raxStop(&mut ri);
        } else if strcasecmp(sub, b"whoami\0" as *const u8 as *const libc::c_char) == 0
            && (*c).argc == 2 as libc::c_int
        {
            if !((*c).user).is_null() {
                addReplyBulkCBuffer(
                    c,
                    (*(*c).user).name as *const libc::c_void,
                    sdslen((*(*c).user).name),
                );
            } else {
                addReplyNull(c);
            }
        } else if *(server.acl_filename).offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
            && (strcasecmp(sub, b"load\0" as *const u8 as *const libc::c_char) == 0
                || strcasecmp(sub, b"save\0" as *const u8 as *const libc::c_char) == 0)
        {
            addReplyError(
                c,
                b"This Redis instance is not configured to use an ACL file. You may want to specify users via the ACL SETUSER command and then issue a CONFIG REWRITE (assuming you have a Redis configuration file set) in order to store users in the Redis configuration.\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        } else {
            if strcasecmp(sub, b"load\0" as *const u8 as *const libc::c_char) == 0
                && (*c).argc == 2 as libc::c_int
            {
                let mut errors: sds = ACLLoadFromFile(server.acl_filename);
                if errors.is_null() {
                    addReply(c, shared.ok);
                } else {
                    addReplyError(c, errors as *const libc::c_char);
                    sdsfree(errors);
                }
            } else if strcasecmp(sub, b"save\0" as *const u8 as *const libc::c_char) == 0
                && (*c).argc == 2 as libc::c_int
            {
                if ACLSaveToFile(server.acl_filename) == 0 as libc::c_int {
                    addReply(c, shared.ok);
                } else {
                    addReplyError(
                        c,
                        b"There was an error trying to save the ACLs. Please check the server logs for more information\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            } else if strcasecmp(sub, b"cat\0" as *const u8 as *const libc::c_char) == 0
                && (*c).argc == 2 as libc::c_int
            {
                let mut dl: *mut libc::c_void = addReplyDeferredLen(c);
                let mut j_3: libc::c_int = 0;
                j_3 = 0 as libc::c_int;
                while ACLCommandCategories[j_3 as usize].flag != 0 as libc::c_int as libc::c_ulong {
                    addReplyBulkCString(c, ACLCommandCategories[j_3 as usize].name);
                    j_3 += 1;
                }
                setDeferredArrayLen(c, dl, j_3 as libc::c_long);
            } else if strcasecmp(sub, b"cat\0" as *const u8 as *const libc::c_char) == 0
                && (*c).argc == 3 as libc::c_int
            {
                let mut cflag: uint64_t = ACLGetCommandCategoryFlagByName(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
                );
                if cflag == 0 as libc::c_int as libc::c_ulong {
                    addReplyErrorFormat(
                        c,
                        b"Unknown category '%.128s'\0" as *const u8 as *const libc::c_char,
                        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *mut libc::c_char,
                    );
                    return;
                }
                let mut arraylen: libc::c_int = 0 as libc::c_int;
                let mut dl_0: *mut libc::c_void = addReplyDeferredLen(c);
                aclCatWithFlags(c, server.orig_commands, cflag, &mut arraylen);
                setDeferredArrayLen(c, dl_0, arraylen as libc::c_long);
            } else if strcasecmp(sub, b"genpass\0" as *const u8 as *const libc::c_char) == 0
                && ((*c).argc == 2 as libc::c_int || (*c).argc == 3 as libc::c_int)
            {
                let mut pass: [libc::c_char; 1024] = [0; 1024];
                let mut bits: libc::c_long = 256 as libc::c_int as libc::c_long;
                if (*c).argc == 3 as libc::c_int
                    && getLongFromObjectOrReply(
                        c,
                        *((*c).argv).offset(2 as libc::c_int as isize),
                        &mut bits,
                        0 as *const libc::c_char,
                    ) != 0 as libc::c_int
                {
                    return;
                }
                if bits <= 0 as libc::c_int as libc::c_long
                    || bits > 4096 as libc::c_int as libc::c_long
                {
                    addReplyErrorFormat(
                        c,
                        b"ACL GENPASS argument must be the number of bits for the output password, a positive number up to %d\0"
                            as *const u8 as *const libc::c_char,
                        4096 as libc::c_int,
                    );
                    return;
                }
                let mut chars: libc::c_long =
                    (bits + 3 as libc::c_int as libc::c_long) / 4 as libc::c_int as libc::c_long;
                getRandomHexChars(pass.as_mut_ptr(), chars as size_t);
                addReplyBulkCBuffer(c, pass.as_mut_ptr() as *const libc::c_void, chars as size_t);
            } else if strcasecmp(sub, b"log\0" as *const u8 as *const libc::c_char) == 0
                && ((*c).argc == 2 as libc::c_int || (*c).argc == 3 as libc::c_int)
            {
                let mut count: libc::c_long = 10 as libc::c_int as libc::c_long;
                if (*c).argc == 3 as libc::c_int {
                    if strcasecmp(
                        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                            as *const libc::c_char,
                        b"reset\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*ACLLog).free =
                            Some(ACLFreeLogEntry as unsafe extern "C" fn(*mut libc::c_void) -> ());
                        listEmpty(ACLLog);
                        (*ACLLog).free = None;
                        addReply(c, shared.ok);
                        return;
                    } else {
                        if getLongFromObjectOrReply(
                            c,
                            *((*c).argv).offset(2 as libc::c_int as isize),
                            &mut count,
                            0 as *const libc::c_char,
                        ) != 0 as libc::c_int
                        {
                            return;
                        }
                    }
                    if count < 0 as libc::c_int as libc::c_long {
                        count = 0 as libc::c_int as libc::c_long;
                    }
                }
                if count as size_t > (*ACLLog).len {
                    count = (*ACLLog).len as libc::c_long;
                }
                addReplyArrayLen(c, count);
                let mut li_0: listIter = listIter {
                    next: 0 as *mut listNode,
                    direction: 0,
                };
                let mut ln_0: *mut listNode = 0 as *mut listNode;
                listRewind(ACLLog, &mut li_0);
                let mut now: mstime_t = mstime();
                loop {
                    let fresh13 = count;
                    count = count - 1;
                    if !(fresh13 != 0 && {
                        ln_0 = listNext(&mut li_0);
                        !ln_0.is_null()
                    }) {
                        break;
                    }
                    let mut le: *mut ACLLogEntry = (*ln_0).value as *mut ACLLogEntry;
                    addReplyMapLen(c, 7 as libc::c_int as libc::c_long);
                    addReplyBulkCString(c, b"count\0" as *const u8 as *const libc::c_char);
                    addReplyLongLong(c, (*le).count as libc::c_longlong);
                    addReplyBulkCString(c, b"reason\0" as *const u8 as *const libc::c_char);
                    let mut reasonstr: *mut libc::c_char = 0 as *mut libc::c_char;
                    match (*le).reason {
                        1 => {
                            reasonstr = b"command\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        2 => {
                            reasonstr =
                                b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        }
                        4 => {
                            reasonstr = b"channel\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        3 => {
                            reasonstr =
                                b"auth\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        }
                        _ => {
                            reasonstr = b"unknown\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                    }
                    addReplyBulkCString(c, reasonstr);
                    addReplyBulkCString(c, b"context\0" as *const u8 as *const libc::c_char);
                    let mut ctxstr: *mut libc::c_char = 0 as *mut libc::c_char;
                    match (*le).context {
                        0 => {
                            ctxstr = b"toplevel\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        2 => {
                            ctxstr =
                                b"multi\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        }
                        1 => {
                            ctxstr =
                                b"lua\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        }
                        3 => {
                            ctxstr = b"module\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        _ => {
                            ctxstr = b"unknown\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                    }
                    addReplyBulkCString(c, ctxstr);
                    addReplyBulkCString(c, b"object\0" as *const u8 as *const libc::c_char);
                    addReplyBulkCBuffer(
                        c,
                        (*le).object as *const libc::c_void,
                        sdslen((*le).object),
                    );
                    addReplyBulkCString(c, b"username\0" as *const u8 as *const libc::c_char);
                    addReplyBulkCBuffer(
                        c,
                        (*le).username as *const libc::c_void,
                        sdslen((*le).username),
                    );
                    addReplyBulkCString(c, b"age-seconds\0" as *const u8 as *const libc::c_char);
                    let mut age: libc::c_double = (now - (*le).ctime) as libc::c_double
                        / 1000 as libc::c_int as libc::c_double;
                    addReplyDouble(c, age);
                    addReplyBulkCString(c, b"client-info\0" as *const u8 as *const libc::c_char);
                    addReplyBulkCBuffer(c, (*le).cinfo as *const libc::c_void, sdslen((*le).cinfo));
                }
            } else if strcasecmp(sub, b"dryrun\0" as *const u8 as *const libc::c_char) == 0
                && (*c).argc >= 4 as libc::c_int
            {
                let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
                let mut u_3: *mut user = ACLGetUserByName(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
                    sdslen((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds),
                );
                if u_3.is_null() {
                    addReplyErrorFormat(
                        c,
                        b"User '%s' not found\0" as *const u8 as *const libc::c_char,
                        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *mut libc::c_char,
                    );
                    return;
                }
                cmd = lookupCommand(
                    ((*c).argv).offset(3 as libc::c_int as isize),
                    (*c).argc - 3 as libc::c_int,
                );
                if cmd.is_null() {
                    addReplyErrorFormat(
                        c,
                        b"Command '%s' not found\0" as *const u8 as *const libc::c_char,
                        (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as *mut libc::c_char,
                    );
                    return;
                }
                if (*cmd).arity > 0 as libc::c_int && (*cmd).arity != (*c).argc - 3 as libc::c_int
                    || ((*c).argc - 3 as libc::c_int) < -(*cmd).arity
                {
                    addReplyErrorFormat(
                        c,
                        b"wrong number of arguments for '%s' command\0" as *const u8
                            as *const libc::c_char,
                        (*cmd).fullname,
                    );
                    return;
                }
                let mut idx: libc::c_int = 0;
                let mut result: libc::c_int = ACLCheckAllUserCommandPerm(
                    u_3,
                    cmd,
                    ((*c).argv).offset(3 as libc::c_int as isize),
                    (*c).argc - 3 as libc::c_int,
                    &mut idx,
                );
                if result != 0 as libc::c_int {
                    let mut err: sds = sdsempty();
                    if result == 1 as libc::c_int {
                        err = sdscatfmt(
                            err,
                            b"This user has no permissions to run the '%s' command\0" as *const u8
                                as *const libc::c_char,
                            (*cmd).fullname,
                        );
                    } else if result == 2 as libc::c_int {
                        err = sdscatfmt(
                            err,
                            b"This user has no permissions to access the '%s' key\0" as *const u8
                                as *const libc::c_char,
                            (**((*c).argv).offset((idx + 3 as libc::c_int) as isize)).ptr,
                        );
                    } else if result == 4 as libc::c_int {
                        err = sdscatfmt(
                            err,
                            b"This user has no permissions to access the '%s' channel\0"
                                as *const u8 as *const libc::c_char,
                            (**((*c).argv).offset((idx + 3 as libc::c_int) as isize)).ptr,
                        );
                    } else {
                        _serverPanic(
                            b"acl.c\0" as *const u8 as *const libc::c_char,
                            2936 as libc::c_int,
                            b"Invalid permission result\0" as *const u8 as *const libc::c_char,
                        );
                        unreachable!();
                    }
                    addReplyBulkSds(c, err);
                    return;
                }
                addReply(c, shared.ok);
            } else if (*c).argc == 2 as libc::c_int
                && strcasecmp(sub, b"help\0" as *const u8 as *const libc::c_char) == 0
            {
                let mut help: [*const libc::c_char; 27] = [
                    b"CAT [<category>]\0" as *const u8 as *const libc::c_char,
                    b"    List all commands that belong to <category>, or all command categories\0"
                        as *const u8 as *const libc::c_char,
                    b"    when no category is specified.\0" as *const u8
                        as *const libc::c_char,
                    b"DELUSER <username> [<username> ...]\0" as *const u8
                        as *const libc::c_char,
                    b"    Delete a list of users.\0" as *const u8 as *const libc::c_char,
                    b"DRYRUN <username> <command> [<arg> ...]\0" as *const u8
                        as *const libc::c_char,
                    b"    Returns whether the user can execute the given command without executing the command.\0"
                        as *const u8 as *const libc::c_char,
                    b"GETUSER <username>\0" as *const u8 as *const libc::c_char,
                    b"    Get the user's details.\0" as *const u8 as *const libc::c_char,
                    b"GENPASS [<bits>]\0" as *const u8 as *const libc::c_char,
                    b"    Generate a secure 256-bit user password. The optional `bits` argument can\0"
                        as *const u8 as *const libc::c_char,
                    b"    be used to specify a different size.\0" as *const u8
                        as *const libc::c_char,
                    b"LIST\0" as *const u8 as *const libc::c_char,
                    b"    Show users details in config file format.\0" as *const u8
                        as *const libc::c_char,
                    b"LOAD\0" as *const u8 as *const libc::c_char,
                    b"    Reload users from the ACL file.\0" as *const u8
                        as *const libc::c_char,
                    b"LOG [<count> | RESET]\0" as *const u8 as *const libc::c_char,
                    b"    Show the ACL log entries.\0" as *const u8
                        as *const libc::c_char,
                    b"SAVE\0" as *const u8 as *const libc::c_char,
                    b"    Save the current config to the ACL file.\0" as *const u8
                        as *const libc::c_char,
                    b"SETUSER <username> <attribute> [<attribute> ...]\0" as *const u8
                        as *const libc::c_char,
                    b"    Create or modify a user with the specified attributes.\0"
                        as *const u8 as *const libc::c_char,
                    b"USERS\0" as *const u8 as *const libc::c_char,
                    b"    List all the registered usernames.\0" as *const u8
                        as *const libc::c_char,
                    b"WHOAMI\0" as *const u8 as *const libc::c_char,
                    b"    Return the current connection username.\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                ];
                addReplyHelp(c, help.as_mut_ptr());
            } else {
                addReplySubcommandSyntaxError(c);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandCategories(mut c: *mut client, mut cmd: *mut redisCommand) {
    let mut flagcount: libc::c_int = 0 as libc::c_int;
    let mut flaglen: *mut libc::c_void = addReplyDeferredLen(c);
    let mut j: libc::c_int = 0 as libc::c_int;
    while ACLCommandCategories[j as usize].flag != 0 as libc::c_int as libc::c_ulong {
        if (*cmd).acl_categories & ACLCommandCategories[j as usize].flag != 0 {
            addReplyStatusFormat(
                c,
                b"@%s\0" as *const u8 as *const libc::c_char,
                ACLCommandCategories[j as usize].name,
            );
            flagcount += 1;
        }
        j += 1;
    }
    setDeferredSetLen(c, flaglen, flagcount as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn authCommand(mut c: *mut client) {
    if (*c).argc > 3 as libc::c_int {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    redactClientCommandArgument(c, 1 as libc::c_int);
    let mut username: *mut robj = 0 as *mut robj;
    let mut password: *mut robj = 0 as *mut robj;
    if (*c).argc == 2 as libc::c_int {
        if (*DefaultUser).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            addReplyError(
                c,
                b"AUTH <password> called without any password configured for the default user. Are you sure your configuration is correct?\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        username = shared.default_username;
        password = *((*c).argv).offset(1 as libc::c_int as isize);
    } else {
        username = *((*c).argv).offset(1 as libc::c_int as isize);
        password = *((*c).argv).offset(2 as libc::c_int as isize);
        redactClientCommandArgument(c, 2 as libc::c_int);
    }
    if ACLAuthenticateUser(c, username, password) == 0 as libc::c_int {
        addReply(c, shared.ok);
    } else {
        addReplyError(
            c,
            b"-WRONGPASS invalid username-password pair or user is disabled.\0" as *const u8
                as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ACLUpdateDefaultUserPassword(mut password: sds) {
    ACLSetUser(
        DefaultUser,
        b"resetpass\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int) as ssize_t,
    );
    if !password.is_null() {
        let mut aclop: sds = sdscatlen(
            sdsnew(b">\0" as *const u8 as *const libc::c_char),
            password as *const libc::c_void,
            sdslen(password),
        );
        ACLSetUser(
            DefaultUser,
            aclop as *const libc::c_char,
            sdslen(aclop) as ssize_t,
        );
        sdsfree(aclop);
    } else {
        ACLSetUser(
            DefaultUser,
            b"nopass\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int) as ssize_t,
        );
    };
}
