extern crate c2rust_bitfields;
extern crate libc;
extern crate core;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleCtx;
    pub type RedisModuleDefragCtx;
    pub type RedisModuleInfoCtx;
    pub type RedisModuleKeyOptCtx;
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut setDictType: dictType;
    static mut zsetDictType: dictType;
    fn moduleGetMemUsage(
        key: *mut robj,
        val: *mut robj,
        sample_size: size_t,
        dbid: libc::c_int,
    ) -> size_t;
    fn addReplyNull(c: *mut client);
    fn addReplyVerbatim(
        c: *mut client,
        s: *const libc::c_char,
        len: size_t,
        ext: *const libc::c_char,
    );
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyOrErrorObject(c: *mut client, reply: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyDouble(c: *mut client, d: libc::c_double);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn sdsZmallocSize(s: sds) -> size_t;
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn estimateObjectIdleTime(o: *mut robj) -> libc::c_ulonglong;
    fn memcpy(
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
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn je_malloc_stats_print(
        write_cb: Option::<
            unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> (),
        >,
        je_cbopaque: *mut libc::c_void,
        opts: *const libc::c_char,
    );
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zmalloc_used_memory() -> size_t;
    fn jemalloc_purge() -> libc::c_int;
    fn intsetNew() -> *mut intset;
    fn intsetBlobLen(is: *mut intset) -> size_t;
    fn sdigits10(v: int64_t) -> uint32_t;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
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
    fn string2ld(
        s: *const libc::c_char,
        slen: size_t,
        dp: *mut f64,
    ) -> libc::c_int;
    fn string2d(
        s: *const libc::c_char,
        slen: size_t,
        dp: *mut libc::c_double,
    ) -> libc::c_int;
    fn ld2string(
        buf: *mut libc::c_char,
        len: size_t,
        value: f64,
        mode: ld2string_mode,
    ) -> libc::c_int;
    fn quicklistCreate() -> *mut quicklist;
    fn quicklistRelease(quicklist: *mut quicklist);
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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn lpNew(capacity: size_t) -> *mut libc::c_uchar;
    fn lpFree(lp: *mut libc::c_uchar);
    fn lpBytes(lp: *mut libc::c_uchar) -> size_t;
    fn streamNew() -> *mut stream;
    fn freeStream(s: *mut stream);
    fn sdsAllocPtr(s: sds) -> *mut libc::c_void;
    fn sdsAllocSize(s: sds) -> size_t;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn zslFree(zsl: *mut zskiplist);
    fn sdsRemoveFreeSpace(s: sds) -> sds;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn dismissMemory(ptr: *mut libc::c_void, size_hint: size_t);
    fn LRU_CLOCK() -> libc::c_uint;
    fn LFUGetTimeInMinutes() -> libc::c_ulong;
    fn sdsfromlonglong(value: libc::c_longlong) -> sds;
    fn _serverAssertWithInfo(
        c: *const client,
        o: *const robj,
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdsfree(s: sds);
    fn sdsempty() -> sds;
    fn zslCreate() -> *mut zskiplist;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdstrynewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    static mut SDS_NOINIT: *const libc::c_char;
    fn evalScriptsMemory() -> libc::c_ulong;
    fn lookupKeyReadWithFlags(
        db: *mut redisDb,
        key: *mut robj,
        flags: libc::c_int,
    ) -> *mut robj;
    fn evalScriptsDict() -> *mut dict;
    fn LFUDecrAndReturn(o: *mut robj) -> libc::c_ulong;
    fn functionsMemoryOverhead() -> libc::c_ulong;
}
pub type __int8_t = libc::c_schar;
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
pub type int8_t = __int8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intset {
    pub encoding: uint32_t,
    pub length: uint32_t,
    pub contents: [int8_t; 0],
}
pub type ld2string_mode = libc::c_uint;
pub const LD_STR_HEX: ld2string_mode = 2;
pub const LD_STR_HUMAN: ld2string_mode = 1;
pub const LD_STR_AUTO: ld2string_mode = 0;
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
#[repr(C)]
pub struct quicklistNode {
    pub prev: *mut quicklistNode,
    pub next: *mut quicklistNode,
    pub entry: *mut libc::c_uchar,
    pub sz: size_t,
    #[bitfield(name = "count", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "16..=17")]
    #[bitfield(name = "container", ty = "libc::c_uint", bits = "18..=19")]
    #[bitfield(name = "recompress", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "attempted_compress", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "dont_compress", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "extra", ty = "libc::c_uint", bits = "23..=31")]
    pub count_encoding_container_recompress_attempted_compress_dont_compress_extra: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistLZF {
    pub sz: size_t,
    pub compressed: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistBookmark {
    pub node: *mut quicklistNode,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
#[repr(C)]
pub struct quicklist {
    pub head: *mut quicklistNode,
    pub tail: *mut quicklistNode,
    pub count: libc::c_ulong,
    pub len: libc::c_ulong,
    #[bitfield(name = "fill", ty = "libc::c_int", bits = "0..=15")]
    #[bitfield(name = "compress", ty = "libc::c_uint", bits = "16..=31")]
    #[bitfield(name = "bookmark_count", ty = "libc::c_uint", bits = "32..=35")]
    pub fill_compress_bookmark_count: [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub bookmarks: [quicklistBookmark; 0],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raxStack {
    pub stack: *mut *mut libc::c_void,
    pub items: size_t,
    pub maxitems: size_t,
    pub static_items: [*mut libc::c_void; 32],
    pub oom: libc::c_int,
}
pub type raxNodeCallback = Option::<
    unsafe extern "C" fn(*mut *mut raxNode) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModule {
    pub handle: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub ver: libc::c_int,
    pub apiver: libc::c_int,
    pub types: *mut list,
    pub usedby: *mut list,
    pub using: *mut list,
    pub filters: *mut list,
    pub module_configs: *mut list,
    pub configs_initialized: libc::c_int,
    pub in_call: libc::c_int,
    pub in_hook: libc::c_int,
    pub options: libc::c_int,
    pub blocked_clients: libc::c_int,
    pub info_cb: RedisModuleInfoFunc,
    pub defrag_cb: RedisModuleDefragFunc,
    pub loadmod: *mut moduleLoadQueueEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moduleLoadQueueEntry {
    pub path: sds,
    pub argc: libc::c_int,
    pub argv: *mut *mut robj,
}
pub type robj = redisObject;
pub type RedisModuleDefragFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> (),
>;
pub type RedisModuleInfoFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleInfoCtx, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleIO {
    pub bytes: size_t,
    pub rio: *mut rio,
    pub type_0: *mut moduleType,
    pub error: libc::c_int,
    pub ver: libc::c_int,
    pub ctx: *mut RedisModuleCtx,
    pub key: *mut redisObject,
    pub dbid: libc::c_int,
}
pub type moduleType = RedisModuleType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleType {
    pub id: uint64_t,
    pub module: *mut RedisModule,
    pub rdb_load: moduleTypeLoadFunc,
    pub rdb_save: moduleTypeSaveFunc,
    pub aof_rewrite: moduleTypeRewriteFunc,
    pub mem_usage: moduleTypeMemUsageFunc,
    pub digest: moduleTypeDigestFunc,
    pub free: moduleTypeFreeFunc,
    pub free_effort: moduleTypeFreeEffortFunc,
    pub unlink: moduleTypeUnlinkFunc,
    pub copy: moduleTypeCopyFunc,
    pub defrag: moduleTypeDefragFunc,
    pub aux_load: moduleTypeAuxLoadFunc,
    pub aux_save: moduleTypeAuxSaveFunc,
    pub mem_usage2: moduleTypeMemUsageFunc2,
    pub free_effort2: moduleTypeFreeEffortFunc2,
    pub unlink2: moduleTypeUnlinkFunc2,
    pub copy2: moduleTypeCopyFunc2,
    pub aux_save_triggers: libc::c_int,
    pub name: [libc::c_char; 10],
}
pub type moduleTypeCopyFunc2 = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKeyOptCtx,
        *const libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type moduleTypeUnlinkFunc2 = Option::<
    unsafe extern "C" fn(*mut RedisModuleKeyOptCtx, *mut libc::c_void) -> (),
>;
pub type moduleTypeFreeEffortFunc2 = Option::<
    unsafe extern "C" fn(*mut RedisModuleKeyOptCtx, *const libc::c_void) -> size_t,
>;
pub type moduleTypeMemUsageFunc2 = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKeyOptCtx,
        *const libc::c_void,
        size_t,
    ) -> size_t,
>;
pub type moduleTypeAuxSaveFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> (),
>;
pub type moduleTypeAuxLoadFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int, libc::c_int) -> libc::c_int,
>;
pub type moduleTypeDefragFunc = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleDefragCtx,
        *mut redisObject,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type moduleTypeCopyFunc = Option::<
    unsafe extern "C" fn(
        *mut redisObject,
        *mut redisObject,
        *const libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type moduleTypeUnlinkFunc = Option::<
    unsafe extern "C" fn(*mut redisObject, *mut libc::c_void) -> (),
>;
pub type moduleTypeFreeEffortFunc = Option::<
    unsafe extern "C" fn(*mut redisObject, *const libc::c_void) -> size_t,
>;
pub type moduleTypeFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type moduleTypeDigestFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleDigest, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleDigest {
    pub o: [libc::c_uchar; 20],
    pub x: [libc::c_uchar; 20],
    pub key: *mut redisObject,
    pub dbid: libc::c_int,
}
pub type moduleTypeMemUsageFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> size_t,
>;
pub type moduleTypeRewriteFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut redisObject, *mut libc::c_void) -> (),
>;
pub type moduleTypeSaveFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut libc::c_void) -> (),
>;
pub type moduleTypeLoadFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> *mut libc::c_void,
>;
pub type RedisModuleUserChangedFunc = Option::<
    unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moduleValue {
    pub type_0: *mut moduleType,
    pub value: *mut libc::c_void,
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
pub struct zskiplistNode {
    pub ele: sds,
    pub score: libc::c_double,
    pub backward: *mut zskiplistNode,
    pub level: [zskiplistLevel; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zskiplistLevel {
    pub forward: *mut zskiplistNode,
    pub span: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zskiplist {
    pub header: *mut zskiplistNode,
    pub tail: *mut zskiplistNode,
    pub length: libc::c_ulong,
    pub level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zset {
    pub dict: *mut dict,
    pub zsl: *mut zskiplist,
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
pub struct redisMemOverhead {
    pub peak_allocated: size_t,
    pub total_allocated: size_t,
    pub startup_allocated: size_t,
    pub repl_backlog: size_t,
    pub clients_slaves: size_t,
    pub clients_normal: size_t,
    pub cluster_links: size_t,
    pub aof_buffer: size_t,
    pub lua_caches: size_t,
    pub functions_caches: size_t,
    pub overhead_total: size_t,
    pub dataset: size_t,
    pub total_keys: size_t,
    pub bytes_per_key: size_t,
    pub dataset_perc: libc::c_float,
    pub peak_perc: libc::c_float,
    pub total_frag: libc::c_float,
    pub total_frag_bytes: ssize_t,
    pub allocator_frag: libc::c_float,
    pub allocator_frag_bytes: ssize_t,
    pub allocator_rss: libc::c_float,
    pub allocator_rss_bytes: ssize_t,
    pub rss_extra: libc::c_float,
    pub rss_extra_bytes: size_t,
    pub num_dbs: size_t,
    pub db: *mut C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub dbid: size_t,
    pub overhead_ht_main: size_t,
    pub overhead_ht_expires: size_t,
    pub overhead_ht_slot_to_keys: size_t,
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
    pub inst_metric: [C2RustUnnamed_12; 5],
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
pub struct C2RustUnnamed_12 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamID {
    pub ms: uint64_t,
    pub seq: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream {
    pub rax: *mut rax,
    pub length: uint64_t,
    pub last_id: streamID,
    pub first_id: streamID,
    pub max_deleted_entry_id: streamID,
    pub entries_added: uint64_t,
    pub cgroups: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamCG {
    pub last_id: streamID,
    pub entries_read: libc::c_longlong,
    pub pel: *mut rax,
    pub consumers: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamConsumer {
    pub seen_time: mstime_t,
    pub name: sds,
    pub pel: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamNACK {
    pub delivery_time: mstime_t,
    pub delivery_count: uint64_t,
    pub consumer: *mut streamConsumer,
}
#[inline]
unsafe extern "C" fn sdsavail(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return 0 as libc::c_int as size_t,
        1 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr8;
            return ((*sh).alloc as libc::c_int - (*sh).len as libc::c_int) as size_t;
        }
        2 => {
            let mut sh_0: *mut sdshdr16 = s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr16;
            return ((*sh_0).alloc as libc::c_int - (*sh_0).len as libc::c_int) as size_t;
        }
        3 => {
            let mut sh_1: *mut sdshdr32 = s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr32;
            return ((*sh_1).alloc).wrapping_sub((*sh_1).len) as size_t;
        }
        4 => {
            let mut sh_2: *mut sdshdr64 = s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr64;
            return ((*sh_2).alloc).wrapping_sub((*sh_2).len);
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
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
#[no_mangle]
pub unsafe extern "C" fn createObject(
    mut type_0: libc::c_int,
    mut ptr: *mut libc::c_void,
) -> *mut robj {
    let mut o: *mut robj = zmalloc(core::mem::size_of::<robj>() as libc::c_ulong)
        as *mut robj;
    (*o).set_type_0(type_0 as libc::c_uint);
    (*o).set_encoding(0 as libc::c_int as libc::c_uint);
    (*o).ptr = ptr;
    (*o).refcount = 1 as libc::c_int;
    if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*o)
            .set_lru(
                (LFUGetTimeInMinutes() << 8 as libc::c_int
                    | 5 as libc::c_int as libc::c_ulong) as libc::c_uint,
            );
    } else {
        (*o).set_lru(LRU_CLOCK());
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn makeObjectShared(mut o: *mut robj) -> *mut robj {
    if (*o).refcount == 1 as libc::c_int {} else {
        _serverAssert(
            b"o->refcount == 1\0" as *const u8 as *const libc::c_char,
            b"object.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
        );
        unreachable!();
    };
    (*o).refcount = 2147483647 as libc::c_int;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createRawStringObject(
    mut ptr: *const libc::c_char,
    mut len: size_t,
) -> *mut robj {
    return createObject(
        0 as libc::c_int,
        sdsnewlen(ptr as *const libc::c_void, len) as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn createEmbeddedStringObject(
    mut ptr: *const libc::c_char,
    mut len: size_t,
) -> *mut robj {
    let mut o: *mut robj = zmalloc(
        (core::mem::size_of::<robj>() as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<sdshdr8>() as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut robj;
    let mut sh: *mut sdshdr8 = o.offset(1 as libc::c_int as isize) as *mut libc::c_void
        as *mut sdshdr8;
    (*o).set_type_0(0 as libc::c_int as libc::c_uint);
    (*o).set_encoding(8 as libc::c_int as libc::c_uint);
    (*o).ptr = sh.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    (*o).refcount = 1 as libc::c_int;
    if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*o)
            .set_lru(
                (LFUGetTimeInMinutes() << 8 as libc::c_int
                    | 5 as libc::c_int as libc::c_ulong) as libc::c_uint,
            );
    } else {
        (*o).set_lru(LRU_CLOCK());
    }
    (*sh).len = len as uint8_t;
    (*sh).alloc = len as uint8_t;
    (*sh).flags = 1 as libc::c_int as libc::c_uchar;
    if ptr == SDS_NOINIT {
        *((*sh).buf).as_mut_ptr().offset(len as isize) = '\0' as i32 as libc::c_char;
    } else if !ptr.is_null() {
        memcpy(
            ((*sh).buf).as_mut_ptr() as *mut libc::c_void,
            ptr as *const libc::c_void,
            len,
        );
        *((*sh).buf).as_mut_ptr().offset(len as isize) = '\0' as i32 as libc::c_char;
    } else {
        memset(
            ((*sh).buf).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createStringObject(
    mut ptr: *const libc::c_char,
    mut len: size_t,
) -> *mut robj {
    if len <= 44 as libc::c_int as libc::c_ulong {
        return createEmbeddedStringObject(ptr, len)
    } else {
        return createRawStringObject(ptr, len)
    };
}
#[no_mangle]
pub unsafe extern "C" fn tryCreateRawStringObject(
    mut ptr: *const libc::c_char,
    mut len: size_t,
) -> *mut robj {
    let mut str: sds = sdstrynewlen(ptr as *const libc::c_void, len);
    if str.is_null() {
        return 0 as *mut robj;
    }
    return createObject(0 as libc::c_int, str as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tryCreateStringObject(
    mut ptr: *const libc::c_char,
    mut len: size_t,
) -> *mut robj {
    if len <= 44 as libc::c_int as libc::c_ulong {
        return createEmbeddedStringObject(ptr, len)
    } else {
        return tryCreateRawStringObject(ptr, len)
    };
}
#[no_mangle]
pub unsafe extern "C" fn createStringObjectFromLongLongWithOptions(
    mut value: libc::c_longlong,
    mut valueobj: libc::c_int,
) -> *mut robj {
    let mut o: *mut robj = 0 as *mut robj;
    if server.maxmemory == 0 as libc::c_int as libc::c_ulonglong
        || server.maxmemory_policy
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) == 0
    {
        valueobj = 0 as libc::c_int;
    }
    if value >= 0 as libc::c_int as libc::c_longlong
        && value < 10000 as libc::c_int as libc::c_longlong
        && valueobj == 0 as libc::c_int
    {
        incrRefCount(shared.integers[value as usize]);
        o = shared.integers[value as usize];
    } else if value
        >= (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
            as libc::c_longlong
        && value <= 9223372036854775807 as libc::c_long as libc::c_longlong
    {
        o = createObject(0 as libc::c_int, 0 as *mut libc::c_void);
        (*o).set_encoding(1 as libc::c_int as libc::c_uint);
        (*o).ptr = value as libc::c_long as *mut libc::c_void;
    } else {
        o = createObject(0 as libc::c_int, sdsfromlonglong(value) as *mut libc::c_void);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createStringObjectFromLongLong(
    mut value: libc::c_longlong,
) -> *mut robj {
    return createStringObjectFromLongLongWithOptions(value, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn createStringObjectFromLongLongForValue(
    mut value: libc::c_longlong,
) -> *mut robj {
    return createStringObjectFromLongLongWithOptions(value, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn createStringObjectFromLongDouble(
    mut value: f64,
    mut humanfriendly: libc::c_int,
) -> *mut robj {
    let mut buf: [libc::c_char; 5120] = [0; 5120];
    let mut len: libc::c_int = ld2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 5120]>() as libc::c_ulong,
        value,
        (if humanfriendly != 0 {
            LD_STR_HUMAN as libc::c_int
        } else {
            LD_STR_AUTO as libc::c_int
        }) as ld2string_mode,
    );
    return createStringObject(buf.as_mut_ptr(), len as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn dupStringObject(mut o: *const robj) -> *mut robj {
    let mut d: *mut robj = 0 as *mut robj;
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssert(
            b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"object.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
        );
        unreachable!();
    };
    match (*o).encoding() as libc::c_int {
        0 => {
            return createRawStringObject(
                (*o).ptr as *const libc::c_char,
                sdslen((*o).ptr as sds),
            );
        }
        8 => {
            return createEmbeddedStringObject(
                (*o).ptr as *const libc::c_char,
                sdslen((*o).ptr as sds),
            );
        }
        1 => {
            d = createObject(0 as libc::c_int, 0 as *mut libc::c_void);
            (*d).set_encoding(1 as libc::c_int as libc::c_uint);
            (*d).ptr = (*o).ptr;
            return d;
        }
        _ => {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int,
                b"Wrong encoding.\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn createQuicklistObject() -> *mut robj {
    let mut l: *mut quicklist = quicklistCreate();
    let mut o: *mut robj = createObject(1 as libc::c_int, l as *mut libc::c_void);
    (*o).set_encoding(9 as libc::c_int as libc::c_uint);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createSetObject() -> *mut robj {
    let mut d: *mut dict = dictCreate(&mut setDictType);
    let mut o: *mut robj = createObject(2 as libc::c_int, d as *mut libc::c_void);
    (*o).set_encoding(2 as libc::c_int as libc::c_uint);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createIntsetObject() -> *mut robj {
    let mut is: *mut intset = intsetNew();
    let mut o: *mut robj = createObject(2 as libc::c_int, is as *mut libc::c_void);
    (*o).set_encoding(6 as libc::c_int as libc::c_uint);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createHashObject() -> *mut robj {
    let mut zl: *mut libc::c_uchar = lpNew(0 as libc::c_int as size_t);
    let mut o: *mut robj = createObject(4 as libc::c_int, zl as *mut libc::c_void);
    (*o).set_encoding(11 as libc::c_int as libc::c_uint);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createZsetObject() -> *mut robj {
    let mut zs: *mut zset = zmalloc(core::mem::size_of::<zset>() as libc::c_ulong)
        as *mut zset;
    let mut o: *mut robj = 0 as *mut robj;
    (*zs).dict = dictCreate(&mut zsetDictType);
    (*zs).zsl = zslCreate();
    o = createObject(3 as libc::c_int, zs as *mut libc::c_void);
    (*o).set_encoding(7 as libc::c_int as libc::c_uint);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createZsetListpackObject() -> *mut robj {
    let mut lp: *mut libc::c_uchar = lpNew(0 as libc::c_int as size_t);
    let mut o: *mut robj = createObject(3 as libc::c_int, lp as *mut libc::c_void);
    (*o).set_encoding(11 as libc::c_int as libc::c_uint);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createStreamObject() -> *mut robj {
    let mut s: *mut stream = streamNew();
    let mut o: *mut robj = createObject(6 as libc::c_int, s as *mut libc::c_void);
    (*o).set_encoding(10 as libc::c_int as libc::c_uint);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn createModuleObject(
    mut mt: *mut moduleType,
    mut value: *mut libc::c_void,
) -> *mut robj {
    let mut mv: *mut moduleValue = zmalloc(
        core::mem::size_of::<moduleValue>() as libc::c_ulong,
    ) as *mut moduleValue;
    (*mv).type_0 = mt;
    (*mv).value = value;
    return createObject(5 as libc::c_int, mv as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn freeStringObject(mut o: *mut robj) {
    if (*o).encoding() as libc::c_int == 0 as libc::c_int {
        sdsfree((*o).ptr as sds);
    }
}
#[no_mangle]
pub unsafe extern "C" fn freeListObject(mut o: *mut robj) {
    if (*o).encoding() as libc::c_int == 9 as libc::c_int {
        quicklistRelease((*o).ptr as *mut quicklist);
    } else {
        _serverPanic(
            b"object.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int,
            b"Unknown list encoding type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeSetObject(mut o: *mut robj) {
    match (*o).encoding() as libc::c_int {
        2 => {
            dictRelease((*o).ptr as *mut dict);
        }
        6 => {
            zfree((*o).ptr);
        }
        _ => {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                313 as libc::c_int,
                b"Unknown set encoding type\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeZsetObject(mut o: *mut robj) {
    let mut zs: *mut zset = 0 as *mut zset;
    match (*o).encoding() as libc::c_int {
        7 => {
            zs = (*o).ptr as *mut zset;
            dictRelease((*zs).dict);
            zslFree((*zs).zsl);
            zfree(zs as *mut libc::c_void);
        }
        11 => {
            zfree((*o).ptr);
        }
        _ => {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                330 as libc::c_int,
                b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeHashObject(mut o: *mut robj) {
    match (*o).encoding() as libc::c_int {
        2 => {
            dictRelease((*o).ptr as *mut dict);
        }
        11 => {
            lpFree((*o).ptr as *mut libc::c_uchar);
        }
        _ => {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                343 as libc::c_int,
                b"Unknown hash encoding type\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeModuleObject(mut o: *mut robj) {
    let mut mv: *mut moduleValue = (*o).ptr as *mut moduleValue;
    ((*(*mv).type_0).free).expect("non-null function pointer")((*mv).value);
    zfree(mv as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn freeStreamObject(mut o: *mut robj) {
    freeStream((*o).ptr as *mut stream);
}
#[no_mangle]
pub unsafe extern "C" fn incrRefCount(mut o: *mut robj) {
    if (*o).refcount < 2147483647 as libc::c_int - 1 as libc::c_int {
        (*o).refcount += 1;
    } else if !((*o).refcount == 2147483647 as libc::c_int) {
        if (*o).refcount == 2147483647 as libc::c_int - 1 as libc::c_int {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                365 as libc::c_int,
                b"You tried to retain an object allocated in the stack\0" as *const u8
                    as *const libc::c_char,
            );
            unreachable!();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn decrRefCount(mut o: *mut robj) {
    if (*o).refcount == 1 as libc::c_int {
        match (*o).type_0() as libc::c_int {
            0 => {
                freeStringObject(o);
            }
            1 => {
                freeListObject(o);
            }
            2 => {
                freeSetObject(o);
            }
            3 => {
                freeZsetObject(o);
            }
            4 => {
                freeHashObject(o);
            }
            5 => {
                freeModuleObject(o);
            }
            6 => {
                freeStreamObject(o);
            }
            _ => {
                _serverPanic(
                    b"object.c\0" as *const u8 as *const libc::c_char,
                    380 as libc::c_int,
                    b"Unknown object type\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
        zfree(o as *mut libc::c_void);
    } else {
        if (*o).refcount <= 0 as libc::c_int {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                384 as libc::c_int,
                b"decrRefCount against refcount <= 0\0" as *const u8
                    as *const libc::c_char,
            );
            unreachable!();
        }
        if (*o).refcount != 2147483647 as libc::c_int {
            (*o).refcount -= 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dismissSds(mut s: sds) {
    dismissMemory(sdsAllocPtr(s), sdsAllocSize(s));
}
#[no_mangle]
pub unsafe extern "C" fn dismissStringObject(mut o: *mut robj) {
    if (*o).encoding() as libc::c_int == 0 as libc::c_int {
        dismissSds((*o).ptr as sds);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dismissListObject(mut o: *mut robj, mut size_hint: size_t) {
    if (*o).encoding() as libc::c_int == 9 as libc::c_int {
        let mut ql: *mut quicklist = (*o).ptr as *mut quicklist;
        if (*ql).len != 0 as libc::c_int as libc::c_ulong {} else {
            _serverAssert(
                b"ql->len != 0\0" as *const u8 as *const libc::c_char,
                b"object.c\0" as *const u8 as *const libc::c_char,
                405 as libc::c_int,
            );
            unreachable!();
        };
        if size_hint.wrapping_div((*ql).len) >= server.page_size {
            let mut node: *mut quicklistNode = (*ql).head;
            while !node.is_null() {
                if (*node).encoding() as libc::c_int == 2 as libc::c_int {
                    dismissMemory(
                        (*node).entry as *mut libc::c_void,
                        (*((*node).entry as *mut quicklistLZF)).sz,
                    );
                } else {
                    dismissMemory((*node).entry as *mut libc::c_void, (*node).sz);
                }
                node = (*node).next;
            }
        }
    } else {
        _serverPanic(
            b"object.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_int,
            b"Unknown list encoding type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn dismissSetObject(mut o: *mut robj, mut size_hint: size_t) {
    if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        let mut set: *mut dict = (*o).ptr as *mut dict;
        if ((*set).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*set).ht_used[1 as libc::c_int as usize])
            != 0 as libc::c_int as libc::c_ulong
        {} else {
            _serverAssert(
                b"dictSize(set) != 0\0" as *const u8 as *const libc::c_char,
                b"object.c\0" as *const u8 as *const libc::c_char,
                428 as libc::c_int,
            );
            unreachable!();
        };
        if size_hint
            .wrapping_div(
                ((*set).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*set).ht_used[1 as libc::c_int as usize]),
            ) >= server.page_size
        {
            let mut de: *mut dictEntry = 0 as *mut dictEntry;
            let mut di: *mut dictIterator = dictGetIterator(set);
            loop {
                de = dictNext(di);
                if de.is_null() {
                    break;
                }
                dismissSds((*de).key as sds);
            }
            dictReleaseIterator(di);
        }
        dismissMemory(
            (*set).ht_table[0 as libc::c_int as usize] as *mut libc::c_void,
            (if (*set).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*set).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        );
        dismissMemory(
            (*set).ht_table[1 as libc::c_int as usize] as *mut libc::c_void,
            (if (*set).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*set).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        );
    } else if (*o).encoding() as libc::c_int == 6 as libc::c_int {
        dismissMemory((*o).ptr, intsetBlobLen((*o).ptr as *mut intset));
    } else {
        _serverPanic(
            b"object.c\0" as *const u8 as *const libc::c_char,
            446 as libc::c_int,
            b"Unknown set encoding type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn dismissZsetObject(mut o: *mut robj, mut size_hint: size_t) {
    if (*o).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*o).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        if (*zsl).length != 0 as libc::c_int as libc::c_ulong {} else {
            _serverAssert(
                b"zsl->length != 0\0" as *const u8 as *const libc::c_char,
                b"object.c\0" as *const u8 as *const libc::c_char,
                455 as libc::c_int,
            );
            unreachable!();
        };
        if size_hint.wrapping_div((*zsl).length) >= server.page_size {
            let mut zn: *mut zskiplistNode = (*zsl).tail;
            while !zn.is_null() {
                dismissSds((*zn).ele);
                zn = (*zn).backward;
            }
        }
        let mut d: *mut dict = (*zs).dict;
        dismissMemory(
            (*d).ht_table[0 as libc::c_int as usize] as *mut libc::c_void,
            (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        );
        dismissMemory(
            (*d).ht_table[1 as libc::c_int as usize] as *mut libc::c_void,
            (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        );
    } else if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        dismissMemory((*o).ptr, lpBytes((*o).ptr as *mut libc::c_uchar));
    } else {
        _serverPanic(
            b"object.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int,
            b"Unknown zset encoding type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn dismissHashObject(mut o: *mut robj, mut size_hint: size_t) {
    if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        let mut d: *mut dict = (*o).ptr as *mut dict;
        if ((*d).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
            != 0 as libc::c_int as libc::c_ulong
        {} else {
            _serverAssert(
                b"dictSize(d) != 0\0" as *const u8 as *const libc::c_char,
                b"object.c\0" as *const u8 as *const libc::c_char,
                481 as libc::c_int,
            );
            unreachable!();
        };
        if size_hint
            .wrapping_div(
                ((*d).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*d).ht_used[1 as libc::c_int as usize]),
            ) >= server.page_size
        {
            let mut de: *mut dictEntry = 0 as *mut dictEntry;
            let mut di: *mut dictIterator = dictGetIterator(d);
            loop {
                de = dictNext(di);
                if de.is_null() {
                    break;
                }
                dismissSds((*de).v.val as sds);
            }
            dictReleaseIterator(di);
        }
        dismissMemory(
            (*d).ht_table[0 as libc::c_int as usize] as *mut libc::c_void,
            (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        );
        dismissMemory(
            (*d).ht_table[1 as libc::c_int as usize] as *mut libc::c_void,
            (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        );
    } else if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        dismissMemory((*o).ptr, lpBytes((*o).ptr as *mut libc::c_uchar));
    } else {
        _serverPanic(
            b"object.c\0" as *const u8 as *const libc::c_char,
            501 as libc::c_int,
            b"Unknown hash encoding type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn dismissStreamObject(mut o: *mut robj, mut size_hint: size_t) {
    let mut s: *mut stream = (*o).ptr as *mut stream;
    let mut rax: *mut rax = (*s).rax;
    if raxSize(rax) == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if size_hint.wrapping_div(raxSize(rax)) >= server.page_size {
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
        raxStart(&mut ri, rax);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        while raxNext(&mut ri) != 0 {
            dismissMemory(ri.data, lpBytes(ri.data as *mut libc::c_uchar));
        }
        raxStop(&mut ri);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dismissObject(mut o: *mut robj, mut size_hint: size_t) {
    if server.thp_enabled != 0 {
        return;
    }
    if (*o).refcount != 1 as libc::c_int {
        return;
    }
    match (*o).type_0() as libc::c_int {
        0 => {
            dismissStringObject(o);
        }
        1 => {
            dismissListObject(o, size_hint);
        }
        2 => {
            dismissSetObject(o, size_hint);
        }
        3 => {
            dismissZsetObject(o, size_hint);
        }
        4 => {
            dismissHashObject(o, size_hint);
        }
        6 => {
            dismissStreamObject(o, size_hint);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn decrRefCountVoid(mut o: *mut libc::c_void) {
    decrRefCount(o as *mut robj);
}
#[no_mangle]
pub unsafe extern "C" fn checkType(
    mut c: *mut client,
    mut o: *mut robj,
    mut type_0: libc::c_int,
) -> libc::c_int {
    if !o.is_null() && (*o).type_0() as libc::c_int != type_0 {
        addReplyErrorObject(c, shared.wrongtypeerr);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isSdsRepresentableAsLongLong(
    mut s: sds,
    mut llval: *mut libc::c_longlong,
) -> libc::c_int {
    return if string2ll(s as *const libc::c_char, sdslen(s), llval) != 0 {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn isObjectRepresentableAsLongLong(
    mut o: *mut robj,
    mut llval: *mut libc::c_longlong,
) -> libc::c_int {
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssertWithInfo(
            0 as *const client,
            o,
            b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"object.c\0" as *const u8 as *const libc::c_char,
            581 as libc::c_int,
        );
        unreachable!();
    };
    if (*o).encoding() as libc::c_int == 1 as libc::c_int {
        if !llval.is_null() {
            *llval = (*o).ptr as libc::c_long as libc::c_longlong;
        }
        return 0 as libc::c_int;
    } else {
        return isSdsRepresentableAsLongLong((*o).ptr as sds, llval)
    };
}
#[no_mangle]
pub unsafe extern "C" fn trimStringObjectIfNeeded(mut o: *mut robj) {
    if (*o).encoding() as libc::c_int == 0 as libc::c_int
        && sdsavail((*o).ptr as sds)
            > (sdslen((*o).ptr as sds)).wrapping_div(10 as libc::c_int as libc::c_ulong)
    {
        (*o).ptr = sdsRemoveFreeSpace((*o).ptr as sds) as *mut libc::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tryObjectEncoding(mut o: *mut robj) -> *mut robj {
    let mut value: libc::c_long = 0;
    let mut s: sds = (*o).ptr as sds;
    let mut len: size_t = 0;
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssertWithInfo(
            0 as *const client,
            o,
            b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"object.c\0" as *const u8 as *const libc::c_char,
            612 as libc::c_int,
        );
        unreachable!();
    };
    if !((*o).encoding() as libc::c_int == 0 as libc::c_int
        || (*o).encoding() as libc::c_int == 8 as libc::c_int)
    {
        return o;
    }
    if (*o).refcount > 1 as libc::c_int {
        return o;
    }
    len = sdslen(s);
    if len <= 20 as libc::c_int as libc::c_ulong
        && string2l(s as *const libc::c_char, len, &mut value) != 0
    {
        if (server.maxmemory == 0 as libc::c_int as libc::c_ulonglong
            || server.maxmemory_policy
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) == 0)
            && value >= 0 as libc::c_int as libc::c_long
            && value < 10000 as libc::c_int as libc::c_long
        {
            decrRefCount(o);
            incrRefCount(shared.integers[value as usize]);
            return shared.integers[value as usize];
        } else {
            if (*o).encoding() as libc::c_int == 0 as libc::c_int {
                sdsfree((*o).ptr as sds);
                (*o).set_encoding(1 as libc::c_int as libc::c_uint);
                (*o).ptr = value as *mut libc::c_void;
                return o;
            } else {
                if (*o).encoding() as libc::c_int == 8 as libc::c_int {
                    decrRefCount(o);
                    return createStringObjectFromLongLongForValue(
                        value as libc::c_longlong,
                    );
                }
            }
        }
    }
    if len <= 44 as libc::c_int as libc::c_ulong {
        let mut emb: *mut robj = 0 as *mut robj;
        if (*o).encoding() as libc::c_int == 8 as libc::c_int {
            return o;
        }
        emb = createEmbeddedStringObject(s as *const libc::c_char, sdslen(s));
        decrRefCount(o);
        return emb;
    }
    trimStringObjectIfNeeded(o);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn getDecodedObject(mut o: *mut robj) -> *mut robj {
    let mut dec: *mut robj = 0 as *mut robj;
    if (*o).encoding() as libc::c_int == 0 as libc::c_int
        || (*o).encoding() as libc::c_int == 8 as libc::c_int
    {
        incrRefCount(o);
        return o;
    }
    if (*o).type_0() as libc::c_int == 0 as libc::c_int
        && (*o).encoding() as libc::c_int == 1 as libc::c_int
    {
        let mut buf: [libc::c_char; 32] = [0; 32];
        ll2string(
            buf.as_mut_ptr(),
            32 as libc::c_int as size_t,
            (*o).ptr as libc::c_long as libc::c_longlong,
        );
        dec = createStringObject(buf.as_mut_ptr(), strlen(buf.as_mut_ptr()));
        return dec;
    } else {
        _serverPanic(
            b"object.c\0" as *const u8 as *const libc::c_char,
            698 as libc::c_int,
            b"Unknown encoding type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn compareStringObjectsWithFlags(
    mut a: *mut robj,
    mut b: *mut robj,
    mut flags: libc::c_int,
) -> libc::c_int {
    if (*a).type_0() as libc::c_int == 0 as libc::c_int
        && (*b).type_0() as libc::c_int == 0 as libc::c_int
    {} else {
        _serverAssertWithInfo(
            0 as *const client,
            a,
            b"a->type == OBJ_STRING && b->type == OBJ_STRING\0" as *const u8
                as *const libc::c_char,
            b"object.c\0" as *const u8 as *const libc::c_char,
            714 as libc::c_int,
        );
        unreachable!();
    };
    let mut bufa: [libc::c_char; 128] = [0; 128];
    let mut bufb: [libc::c_char; 128] = [0; 128];
    let mut astr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alen: size_t = 0;
    let mut blen: size_t = 0;
    let mut minlen: size_t = 0;
    if a == b {
        return 0 as libc::c_int;
    }
    if (*a).encoding() as libc::c_int == 0 as libc::c_int
        || (*a).encoding() as libc::c_int == 8 as libc::c_int
    {
        astr = (*a).ptr as *mut libc::c_char;
        alen = sdslen(astr);
    } else {
        alen = ll2string(
            bufa.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            (*a).ptr as libc::c_long as libc::c_longlong,
        ) as size_t;
        astr = bufa.as_mut_ptr();
    }
    if (*b).encoding() as libc::c_int == 0 as libc::c_int
        || (*b).encoding() as libc::c_int == 8 as libc::c_int
    {
        bstr = (*b).ptr as *mut libc::c_char;
        blen = sdslen(bstr);
    } else {
        blen = ll2string(
            bufb.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            (*b).ptr as libc::c_long as libc::c_longlong,
        ) as size_t;
        bstr = bufb.as_mut_ptr();
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return strcoll(astr, bstr)
    } else {
        let mut cmp: libc::c_int = 0;
        minlen = if alen < blen { alen } else { blen };
        cmp = memcmp(astr as *const libc::c_void, bstr as *const libc::c_void, minlen);
        if cmp == 0 as libc::c_int {
            return alen.wrapping_sub(blen) as libc::c_int;
        }
        return cmp;
    };
}
#[no_mangle]
pub unsafe extern "C" fn compareStringObjects(
    mut a: *mut robj,
    mut b: *mut robj,
) -> libc::c_int {
    return compareStringObjectsWithFlags(a, b, (1 as libc::c_int) << 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn collateStringObjects(
    mut a: *mut robj,
    mut b: *mut robj,
) -> libc::c_int {
    return compareStringObjectsWithFlags(a, b, (1 as libc::c_int) << 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn equalStringObjects(
    mut a: *mut robj,
    mut b: *mut robj,
) -> libc::c_int {
    if (*a).encoding() as libc::c_int == 1 as libc::c_int
        && (*b).encoding() as libc::c_int == 1 as libc::c_int
    {
        return ((*a).ptr == (*b).ptr) as libc::c_int
    } else {
        return (compareStringObjects(a, b) == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn stringObjectLen(mut o: *mut robj) -> size_t {
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssertWithInfo(
            0 as *const client,
            o,
            b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"object.c\0" as *const u8 as *const libc::c_char,
            771 as libc::c_int,
        );
        unreachable!();
    };
    if (*o).encoding() as libc::c_int == 0 as libc::c_int
        || (*o).encoding() as libc::c_int == 8 as libc::c_int
    {
        return sdslen((*o).ptr as sds)
    } else {
        return sdigits10((*o).ptr as libc::c_long) as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn getDoubleFromObject(
    mut o: *const robj,
    mut target: *mut libc::c_double,
) -> libc::c_int {
    let mut value: libc::c_double = 0.;
    if o.is_null() {
        value = 0 as libc::c_int as libc::c_double;
    } else {
        if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
            _serverAssertWithInfo(
                0 as *const client,
                o,
                b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
                b"object.c\0" as *const u8 as *const libc::c_char,
                785 as libc::c_int,
            );
            unreachable!();
        };
        if (*o).encoding() as libc::c_int == 0 as libc::c_int
            || (*o).encoding() as libc::c_int == 8 as libc::c_int
        {
            if string2d(
                (*o).ptr as *const libc::c_char,
                sdslen((*o).ptr as sds),
                &mut value,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
        } else if (*o).encoding() as libc::c_int == 1 as libc::c_int {
            value = (*o).ptr as libc::c_long as libc::c_double;
        } else {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                792 as libc::c_int,
                b"Unknown string encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    *target = value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getDoubleFromObjectOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut target: *mut libc::c_double,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    let mut value: libc::c_double = 0.;
    if getDoubleFromObject(o, &mut value) != 0 as libc::c_int {
        if !msg.is_null() {
            addReplyError(c, msg as *mut libc::c_char);
        } else {
            addReplyError(
                c,
                b"value is not a valid float\0" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    *target = value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getLongDoubleFromObject(
    mut o: *mut robj,
    mut target: *mut f64,
) -> libc::c_int {
    let mut value: f64 = 0 as f64;
    if o.is_null() {
        value = (0 as libc::c_int) as f64;
    } else {
        if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
            _serverAssertWithInfo(
                0 as *const client,
                o,
                b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
                b"object.c\0" as *const u8 as *const libc::c_char,
                819 as libc::c_int,
            );
            unreachable!();
        };
        if (*o).encoding() as libc::c_int == 0 as libc::c_int
            || (*o).encoding() as libc::c_int == 8 as libc::c_int
        {
            if string2ld(
                (*o).ptr as *const libc::c_char,
                sdslen((*o).ptr as sds),
                &mut value,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
        } else if (*o).encoding() as libc::c_int == 1 as libc::c_int {
            value = ((*o).ptr as libc::c_long) as f64;
        } else {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                826 as libc::c_int,
                b"Unknown string encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    *target = value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getLongDoubleFromObjectOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut target: *mut f64,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    let mut value: f64 = 0 as f64;
    if getLongDoubleFromObject(o, &mut value) != 0 as libc::c_int {
        if !msg.is_null() {
            addReplyError(c, msg as *mut libc::c_char);
        } else {
            addReplyError(
                c,
                b"value is not a valid float\0" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    *target = value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getLongLongFromObject(
    mut o: *mut robj,
    mut target: *mut libc::c_longlong,
) -> libc::c_int {
    let mut value: libc::c_longlong = 0;
    if o.is_null() {
        value = 0 as libc::c_int as libc::c_longlong;
    } else {
        if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
            _serverAssertWithInfo(
                0 as *const client,
                o,
                b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
                b"object.c\0" as *const u8 as *const libc::c_char,
                853 as libc::c_int,
            );
            unreachable!();
        };
        if (*o).encoding() as libc::c_int == 0 as libc::c_int
            || (*o).encoding() as libc::c_int == 8 as libc::c_int
        {
            if string2ll(
                (*o).ptr as *const libc::c_char,
                sdslen((*o).ptr as sds),
                &mut value,
            ) == 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        } else if (*o).encoding() as libc::c_int == 1 as libc::c_int {
            value = (*o).ptr as libc::c_long as libc::c_longlong;
        } else {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                859 as libc::c_int,
                b"Unknown string encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    if !target.is_null() {
        *target = value;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getLongLongFromObjectOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut target: *mut libc::c_longlong,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    let mut value: libc::c_longlong = 0;
    if getLongLongFromObject(o, &mut value) != 0 as libc::c_int {
        if !msg.is_null() {
            addReplyError(c, msg as *mut libc::c_char);
        } else {
            addReplyError(
                c,
                b"value is not an integer or out of range\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    *target = value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getLongFromObjectOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut target: *mut libc::c_long,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    let mut value: libc::c_longlong = 0;
    if getLongLongFromObjectOrReply(c, o, &mut value, msg) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if value
        < (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
            as libc::c_longlong
        || value > 9223372036854775807 as libc::c_long as libc::c_longlong
    {
        if !msg.is_null() {
            addReplyError(c, msg as *mut libc::c_char);
        } else {
            addReplyError(
                c,
                b"value is out of range\0" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    *target = value as libc::c_long;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getRangeLongFromObjectOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut min: libc::c_long,
    mut max: libc::c_long,
    mut target: *mut libc::c_long,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    if getLongFromObjectOrReply(c, o, target, msg) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if *target < min || *target > max {
        if !msg.is_null() {
            addReplyError(c, msg as *mut libc::c_char);
        } else {
            addReplyErrorFormat(
                c,
                b"value is out of range, value must between %ld and %ld\0" as *const u8
                    as *const libc::c_char,
                min,
                max,
            );
        }
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getPositiveLongFromObjectOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut target: *mut libc::c_long,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    if !msg.is_null() {
        return getRangeLongFromObjectOrReply(
            c,
            o,
            0 as libc::c_int as libc::c_long,
            9223372036854775807 as libc::c_long,
            target,
            msg,
        )
    } else {
        return getRangeLongFromObjectOrReply(
            c,
            o,
            0 as libc::c_int as libc::c_long,
            9223372036854775807 as libc::c_long,
            target,
            b"value is out of range, must be positive\0" as *const u8
                as *const libc::c_char,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn getIntFromObjectOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut target: *mut libc::c_int,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    let mut value: libc::c_long = 0;
    if getRangeLongFromObjectOrReply(
        c,
        o,
        (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long,
        2147483647 as libc::c_int as libc::c_long,
        &mut value,
        msg,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    *target = value as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strEncoding(mut encoding: libc::c_int) -> *mut libc::c_char {
    match encoding {
        0 => return b"raw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 => return b"int\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => {
            return b"hashtable\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        9 => {
            return b"quicklist\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        11 => {
            return b"listpack\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        6 => return b"intset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 => {
            return b"skiplist\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        8 => return b"embstr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 => return b"stream\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => return b"unknown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn streamRadixTreeMemoryUsage(mut rax: *mut rax) -> size_t {
    let mut size: size_t = core::mem::size_of::<rax>() as libc::c_ulong;
    size = ((*rax).numele)
        .wrapping_mul(core::mem::size_of::<streamID>() as libc::c_ulong);
    size = (size as libc::c_ulong)
        .wrapping_add(
            ((*rax).numnodes)
                .wrapping_mul(core::mem::size_of::<raxNode>() as libc::c_ulong),
        ) as size_t as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            ((*rax).numnodes)
                .wrapping_mul(core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_mul(30 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn objectComputeSize(
    mut key: *mut robj,
    mut o: *mut robj,
    mut sample_size: size_t,
    mut dbid: libc::c_int,
) -> size_t {
    let mut ele: sds = 0 as *mut libc::c_char;
    let mut ele2: sds = 0 as *mut libc::c_char;
    let mut d: *mut dict = 0 as *mut dict;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut asize: size_t = 0 as libc::c_int as size_t;
    let mut elesize: size_t = 0 as libc::c_int as size_t;
    let mut samples: size_t = 0 as libc::c_int as size_t;
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {
        if (*o).encoding() as libc::c_int == 1 as libc::c_int {
            asize = core::mem::size_of::<robj>() as libc::c_ulong;
        } else if (*o).encoding() as libc::c_int == 0 as libc::c_int {
            asize = (sdsZmallocSize((*o).ptr as sds))
                .wrapping_add(core::mem::size_of::<robj>() as libc::c_ulong);
        } else if (*o).encoding() as libc::c_int == 8 as libc::c_int {
            asize = je_malloc_usable_size(o as *mut libc::c_void);
        } else {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                988 as libc::c_int,
                b"Unknown string encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 1 as libc::c_int {
        if (*o).encoding() as libc::c_int == 9 as libc::c_int {
            let mut ql: *mut quicklist = (*o).ptr as *mut quicklist;
            let mut node: *mut quicklistNode = (*ql).head;
            asize = (core::mem::size_of::<robj>() as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<quicklist>() as libc::c_ulong);
            loop {
                elesize = (elesize as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<quicklistNode>() as libc::c_ulong)
                            .wrapping_add(
                                je_malloc_usable_size((*node).entry as *mut libc::c_void),
                            ),
                    ) as size_t as size_t;
                samples = samples.wrapping_add(1);
                node = (*node).next;
                if !(!node.is_null() && samples < sample_size) {
                    break;
                }
            }
            asize = (asize as libc::c_double
                + elesize as libc::c_double / samples as libc::c_double
                    * (*ql).len as libc::c_double) as size_t;
        } else {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                1001 as libc::c_int,
                b"Unknown list encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 2 as libc::c_int {
        if (*o).encoding() as libc::c_int == 2 as libc::c_int {
            d = (*o).ptr as *mut dict;
            di = dictGetIterator(d);
            asize = (core::mem::size_of::<robj>() as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<dict>() as libc::c_ulong)
                .wrapping_add(
                    (core::mem::size_of::<*mut dictEntry>() as libc::c_ulong)
                        .wrapping_mul(
                            (if (*d).ht_size_exp[0 as libc::c_int as usize]
                                as libc::c_int == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*d).ht_size_exp[0 as libc::c_int as usize]
                                        as libc::c_int
                            })
                                .wrapping_add(
                                    (if (*d).ht_size_exp[1 as libc::c_int as usize]
                                        as libc::c_int == -(1 as libc::c_int)
                                    {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (1 as libc::c_int as libc::c_ulong)
                                            << (*d).ht_size_exp[1 as libc::c_int as usize]
                                                as libc::c_int
                                    }),
                                ),
                        ),
                );
            loop {
                de = dictNext(di);
                if !(!de.is_null() && samples < sample_size) {
                    break;
                }
                ele = (*de).key as sds;
                elesize = (elesize as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<dictEntry>() as libc::c_ulong)
                            .wrapping_add(sdsZmallocSize(ele)),
                    ) as size_t as size_t;
                samples = samples.wrapping_add(1);
            }
            dictReleaseIterator(di);
            if samples != 0 {
                asize = (asize as libc::c_double
                    + elesize as libc::c_double / samples as libc::c_double
                        * ((*d).ht_used[0 as libc::c_int as usize])
                            .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
                            as libc::c_double) as size_t;
            }
        } else if (*o).encoding() as libc::c_int == 6 as libc::c_int {
            asize = (core::mem::size_of::<robj>() as libc::c_ulong)
                .wrapping_add(je_malloc_usable_size((*o).ptr));
        } else {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                1018 as libc::c_int,
                b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 3 as libc::c_int {
        if (*o).encoding() as libc::c_int == 11 as libc::c_int {
            asize = (core::mem::size_of::<robj>() as libc::c_ulong)
                .wrapping_add(je_malloc_usable_size((*o).ptr));
        } else if (*o).encoding() as libc::c_int == 7 as libc::c_int {
            d = (*((*o).ptr as *mut zset)).dict;
            let mut zsl: *mut zskiplist = (*((*o).ptr as *mut zset)).zsl;
            let mut znode: *mut zskiplistNode = (*((*(*zsl).header).level)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .forward;
            asize = (core::mem::size_of::<robj>() as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<zset>() as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<zskiplist>() as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<dict>() as libc::c_ulong)
                .wrapping_add(
                    (core::mem::size_of::<*mut dictEntry>() as libc::c_ulong)
                        .wrapping_mul(
                            (if (*d).ht_size_exp[0 as libc::c_int as usize]
                                as libc::c_int == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*d).ht_size_exp[0 as libc::c_int as usize]
                                        as libc::c_int
                            })
                                .wrapping_add(
                                    (if (*d).ht_size_exp[1 as libc::c_int as usize]
                                        as libc::c_int == -(1 as libc::c_int)
                                    {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (1 as libc::c_int as libc::c_ulong)
                                            << (*d).ht_size_exp[1 as libc::c_int as usize]
                                                as libc::c_int
                                    }),
                                ),
                        ),
                )
                .wrapping_add(je_malloc_usable_size((*zsl).header as *mut libc::c_void));
            while !znode.is_null() && samples < sample_size {
                elesize = (elesize as libc::c_ulong)
                    .wrapping_add(sdsZmallocSize((*znode).ele)) as size_t as size_t;
                elesize = (elesize as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<dictEntry>() as libc::c_ulong)
                            .wrapping_add(
                                je_malloc_usable_size(znode as *mut libc::c_void),
                            ),
                    ) as size_t as size_t;
                samples = samples.wrapping_add(1);
                znode = (*((*znode).level)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .forward;
            }
            if samples != 0 {
                asize = (asize as libc::c_double
                    + elesize as libc::c_double / samples as libc::c_double
                        * ((*d).ht_used[0 as libc::c_int as usize])
                            .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
                            as libc::c_double) as size_t;
            }
        } else {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                1038 as libc::c_int,
                b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 4 as libc::c_int {
        if (*o).encoding() as libc::c_int == 11 as libc::c_int {
            asize = (core::mem::size_of::<robj>() as libc::c_ulong)
                .wrapping_add(je_malloc_usable_size((*o).ptr));
        } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
            d = (*o).ptr as *mut dict;
            di = dictGetIterator(d);
            asize = (core::mem::size_of::<robj>() as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<dict>() as libc::c_ulong)
                .wrapping_add(
                    (core::mem::size_of::<*mut dictEntry>() as libc::c_ulong)
                        .wrapping_mul(
                            (if (*d).ht_size_exp[0 as libc::c_int as usize]
                                as libc::c_int == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*d).ht_size_exp[0 as libc::c_int as usize]
                                        as libc::c_int
                            })
                                .wrapping_add(
                                    (if (*d).ht_size_exp[1 as libc::c_int as usize]
                                        as libc::c_int == -(1 as libc::c_int)
                                    {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (1 as libc::c_int as libc::c_ulong)
                                            << (*d).ht_size_exp[1 as libc::c_int as usize]
                                                as libc::c_int
                                    }),
                                ),
                        ),
                );
            loop {
                de = dictNext(di);
                if !(!de.is_null() && samples < sample_size) {
                    break;
                }
                ele = (*de).key as sds;
                ele2 = (*de).v.val as sds;
                elesize = (elesize as libc::c_ulong)
                    .wrapping_add(
                        (sdsZmallocSize(ele)).wrapping_add(sdsZmallocSize(ele2)),
                    ) as size_t as size_t;
                elesize = (elesize as libc::c_ulong)
                    .wrapping_add(core::mem::size_of::<dictEntry>() as libc::c_ulong)
                    as size_t as size_t;
                samples = samples.wrapping_add(1);
            }
            dictReleaseIterator(di);
            if samples != 0 {
                asize = (asize as libc::c_double
                    + elesize as libc::c_double / samples as libc::c_double
                        * ((*d).ht_used[0 as libc::c_int as usize])
                            .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
                            as libc::c_double) as size_t;
            }
        } else {
            _serverPanic(
                b"object.c\0" as *const u8 as *const libc::c_char,
                1057 as libc::c_int,
                b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 6 as libc::c_int {
        let mut s: *mut stream = (*o).ptr as *mut stream;
        asize = (core::mem::size_of::<robj>() as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<stream>() as libc::c_ulong);
        asize = (asize as libc::c_ulong)
            .wrapping_add(streamRadixTreeMemoryUsage((*s).rax)) as size_t as size_t;
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
        raxStart(&mut ri, (*s).rax);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        let mut lpsize: size_t = 0 as libc::c_int as size_t;
        let mut samples_0: size_t = 0 as libc::c_int as size_t;
        while samples_0 < sample_size && raxNext(&mut ri) != 0 {
            let mut lp: *mut libc::c_uchar = ri.data as *mut libc::c_uchar;
            lpsize = (lpsize as libc::c_ulong).wrapping_add(lpBytes(lp)) as size_t
                as size_t;
            samples_0 = samples_0.wrapping_add(1);
        }
        if (*(*s).rax).numele <= samples_0 {
            asize = (asize as libc::c_ulong).wrapping_add(lpsize) as size_t as size_t;
        } else {
            if samples_0 != 0 {
                lpsize = (lpsize as libc::c_ulong).wrapping_div(samples_0) as size_t
                    as size_t;
            }
            asize = (asize as libc::c_ulong)
                .wrapping_add(
                    lpsize
                        .wrapping_mul(
                            ((*(*s).rax).numele)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ),
                ) as size_t as size_t;
            raxSeek(
                &mut ri,
                b"$\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            raxNext(&mut ri);
            asize = (asize as libc::c_ulong)
                .wrapping_add(lpBytes(ri.data as *mut libc::c_uchar)) as size_t
                as size_t;
        }
        raxStop(&mut ri);
        if !((*s).cgroups).is_null() {
            raxStart(&mut ri, (*s).cgroups);
            raxSeek(
                &mut ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri) != 0 {
                let mut cg: *mut streamCG = ri.data as *mut streamCG;
                asize = (asize as libc::c_ulong)
                    .wrapping_add(core::mem::size_of::<streamCG>() as libc::c_ulong)
                    as size_t as size_t;
                asize = (asize as libc::c_ulong)
                    .wrapping_add(streamRadixTreeMemoryUsage((*cg).pel)) as size_t
                    as size_t;
                asize = (asize as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<streamNACK>() as libc::c_ulong)
                            .wrapping_mul(raxSize((*cg).pel)),
                    ) as size_t as size_t;
                let mut cri: raxIterator = raxIterator {
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
                raxStart(&mut cri, (*cg).consumers);
                raxSeek(
                    &mut cri,
                    b"^\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_uchar,
                    0 as libc::c_int as size_t,
                );
                while raxNext(&mut cri) != 0 {
                    let mut consumer: *mut streamConsumer = cri.data
                        as *mut streamConsumer;
                    asize = (asize as libc::c_ulong)
                        .wrapping_add(
                            core::mem::size_of::<streamConsumer>() as libc::c_ulong,
                        ) as size_t as size_t;
                    asize = (asize as libc::c_ulong)
                        .wrapping_add(sdslen((*consumer).name)) as size_t as size_t;
                    asize = (asize as libc::c_ulong)
                        .wrapping_add(streamRadixTreeMemoryUsage((*consumer).pel))
                        as size_t as size_t;
                }
                raxStop(&mut cri);
            }
            raxStop(&mut ri);
        }
    } else if (*o).type_0() as libc::c_int == 5 as libc::c_int {
        asize = moduleGetMemUsage(key, o, sample_size, dbid);
    } else {
        _serverPanic(
            b"object.c\0" as *const u8 as *const libc::c_char,
            1123 as libc::c_int,
            b"Unknown object type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return asize;
}
#[no_mangle]
pub unsafe extern "C" fn freeMemoryOverheadData(mut mh: *mut redisMemOverhead) {
    zfree((*mh).db as *mut libc::c_void);
    zfree(mh as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn getMemoryOverheadData() -> *mut redisMemOverhead {
    let mut j: libc::c_int = 0;
    let mut mem_total: size_t = 0 as libc::c_int as size_t;
    let mut mem: size_t = 0 as libc::c_int as size_t;
    let mut zmalloc_used: size_t = zmalloc_used_memory();
    let mut mh: *mut redisMemOverhead = zcalloc(
        core::mem::size_of::<redisMemOverhead>() as libc::c_ulong,
    ) as *mut redisMemOverhead;
    (*mh).total_allocated = zmalloc_used;
    (*mh).startup_allocated = server.initial_memory_usage;
    (*mh).peak_allocated = server.stat_peak_memory;
    (*mh)
        .total_frag = server.cron_malloc_stats.process_rss as libc::c_float
        / server.cron_malloc_stats.zmalloc_used as libc::c_float;
    (*mh)
        .total_frag_bytes = (server.cron_malloc_stats.process_rss)
        .wrapping_sub(server.cron_malloc_stats.zmalloc_used) as ssize_t;
    (*mh)
        .allocator_frag = server.cron_malloc_stats.allocator_active as libc::c_float
        / server.cron_malloc_stats.allocator_allocated as libc::c_float;
    (*mh)
        .allocator_frag_bytes = (server.cron_malloc_stats.allocator_active)
        .wrapping_sub(server.cron_malloc_stats.allocator_allocated) as ssize_t;
    (*mh)
        .allocator_rss = server.cron_malloc_stats.allocator_resident as libc::c_float
        / server.cron_malloc_stats.allocator_active as libc::c_float;
    (*mh)
        .allocator_rss_bytes = (server.cron_malloc_stats.allocator_resident)
        .wrapping_sub(server.cron_malloc_stats.allocator_active) as ssize_t;
    (*mh)
        .rss_extra = server.cron_malloc_stats.process_rss as libc::c_float
        / server.cron_malloc_stats.allocator_resident as libc::c_float;
    (*mh)
        .rss_extra_bytes = (server.cron_malloc_stats.process_rss)
        .wrapping_sub(server.cron_malloc_stats.allocator_resident);
    mem_total = (mem_total as libc::c_ulong).wrapping_add(server.initial_memory_usage)
        as size_t as size_t;
    if (*server.slaves).len != 0
        && server.repl_buffer_mem as libc::c_longlong > server.repl_backlog_size
    {
        (*mh)
            .clients_slaves = (server.repl_buffer_mem as libc::c_ulonglong)
            .wrapping_sub(server.repl_backlog_size as libc::c_ulonglong) as size_t;
        (*mh).repl_backlog = server.repl_backlog_size as size_t;
    } else {
        (*mh).clients_slaves = 0 as libc::c_int as size_t;
        (*mh).repl_backlog = server.repl_buffer_mem;
    }
    if !(server.repl_backlog).is_null() {
        (*mh)
            .repl_backlog = ((*mh).repl_backlog as libc::c_ulong)
            .wrapping_add(
                ((*(*server.repl_backlog).blocks_index).numnodes)
                    .wrapping_mul(core::mem::size_of::<raxNode>() as libc::c_ulong)
                    .wrapping_add(
                        (raxSize((*server.repl_backlog).blocks_index))
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    ),
            ) as size_t as size_t;
    }
    mem_total = (mem_total as libc::c_ulong).wrapping_add((*mh).repl_backlog) as size_t
        as size_t;
    mem_total = (mem_total as libc::c_ulong).wrapping_add((*mh).clients_slaves) as size_t
        as size_t;
    (*mh)
        .clients_normal = (server.stat_clients_type_memory[3 as libc::c_int as usize])
        .wrapping_add(server.stat_clients_type_memory[2 as libc::c_int as usize])
        .wrapping_add(server.stat_clients_type_memory[0 as libc::c_int as usize]);
    mem_total = (mem_total as libc::c_ulong).wrapping_add((*mh).clients_normal) as size_t
        as size_t;
    (*mh).cluster_links = server.stat_cluster_links_memory;
    mem_total = (mem_total as libc::c_ulong).wrapping_add((*mh).cluster_links) as size_t
        as size_t;
    mem = 0 as libc::c_int as size_t;
    if server.aof_state != 0 as libc::c_int {
        mem = (mem as libc::c_ulong).wrapping_add(sdsZmallocSize(server.aof_buf))
            as size_t as size_t;
    }
    (*mh).aof_buffer = mem;
    mem_total = (mem_total as libc::c_ulong).wrapping_add(mem) as size_t as size_t;
    mem = evalScriptsMemory();
    (*mh).lua_caches = mem;
    mem_total = (mem_total as libc::c_ulong).wrapping_add(mem) as size_t as size_t;
    (*mh).functions_caches = functionsMemoryOverhead();
    mem_total = (mem_total as libc::c_ulong).wrapping_add((*mh).functions_caches)
        as size_t as size_t;
    j = 0 as libc::c_int;
    while j < server.dbnum {
        let mut db: *mut redisDb = (server.db).offset(j as isize);
        let mut keyscount: libc::c_longlong = ((*(*db).dict)
            .ht_used[0 as libc::c_int as usize])
            .wrapping_add((*(*db).dict).ht_used[1 as libc::c_int as usize])
            as libc::c_longlong;
        if !(keyscount == 0 as libc::c_int as libc::c_longlong) {
            (*mh)
                .total_keys = ((*mh).total_keys as libc::c_ulonglong)
                .wrapping_add(keyscount as libc::c_ulonglong) as size_t as size_t;
            (*mh)
                .db = zrealloc(
                (*mh).db as *mut libc::c_void,
                (core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*mh).num_dbs).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut C2RustUnnamed_11;
            (*((*mh).db).offset((*mh).num_dbs as isize)).dbid = j as size_t;
            mem = ((*(*db).dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*db).dict).ht_used[1 as libc::c_int as usize])
                .wrapping_mul(core::mem::size_of::<dictEntry>() as libc::c_ulong)
                .wrapping_add(
                    (if (*(*db).dict).ht_size_exp[0 as libc::c_int as usize]
                        as libc::c_int == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*(*db).dict).ht_size_exp[0 as libc::c_int as usize]
                                as libc::c_int
                    })
                        .wrapping_add(
                            (if (*(*db).dict).ht_size_exp[1 as libc::c_int as usize]
                                as libc::c_int == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*(*db).dict).ht_size_exp[1 as libc::c_int as usize]
                                        as libc::c_int
                            }),
                        )
                        .wrapping_mul(
                            core::mem::size_of::<*mut dictEntry>() as libc::c_ulong,
                        ),
                )
                .wrapping_add(
                    ((*(*db).dict).ht_used[0 as libc::c_int as usize])
                        .wrapping_add((*(*db).dict).ht_used[1 as libc::c_int as usize])
                        .wrapping_mul(core::mem::size_of::<robj>() as libc::c_ulong),
                );
            (*((*mh).db).offset((*mh).num_dbs as isize)).overhead_ht_main = mem;
            mem_total = (mem_total as libc::c_ulong).wrapping_add(mem) as size_t
                as size_t;
            mem = ((*(*db).expires).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*db).expires).ht_used[1 as libc::c_int as usize])
                .wrapping_mul(core::mem::size_of::<dictEntry>() as libc::c_ulong)
                .wrapping_add(
                    (if (*(*db).expires).ht_size_exp[0 as libc::c_int as usize]
                        as libc::c_int == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*(*db).expires).ht_size_exp[0 as libc::c_int as usize]
                                as libc::c_int
                    })
                        .wrapping_add(
                            (if (*(*db).expires).ht_size_exp[1 as libc::c_int as usize]
                                as libc::c_int == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*(*db).expires).ht_size_exp[1 as libc::c_int as usize]
                                        as libc::c_int
                            }),
                        )
                        .wrapping_mul(
                            core::mem::size_of::<*mut dictEntry>() as libc::c_ulong,
                        ),
                );
            (*((*mh).db).offset((*mh).num_dbs as isize)).overhead_ht_expires = mem;
            mem_total = (mem_total as libc::c_ulong).wrapping_add(mem) as size_t
                as size_t;
            mem = ((*(*db).dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*db).dict).ht_used[1 as libc::c_int as usize])
                .wrapping_mul(
                    (if ((*(*(*db).dict).type_0).dictEntryMetadataBytes).is_some() {
                        ((*(*(*db).dict).type_0).dictEntryMetadataBytes)
                            .expect("non-null function pointer")((*db).dict)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }),
                );
            (*((*mh).db).offset((*mh).num_dbs as isize)).overhead_ht_slot_to_keys = mem;
            mem_total = (mem_total as libc::c_ulong).wrapping_add(mem) as size_t
                as size_t;
            (*mh).num_dbs = ((*mh).num_dbs).wrapping_add(1);
        }
        j += 1;
    }
    (*mh).overhead_total = mem_total;
    (*mh).dataset = zmalloc_used.wrapping_sub(mem_total);
    (*mh)
        .peak_perc = zmalloc_used as libc::c_float * 100 as libc::c_int as libc::c_float
        / (*mh).peak_allocated as libc::c_float;
    let mut net_usage: size_t = 1 as libc::c_int as size_t;
    if zmalloc_used > (*mh).startup_allocated {
        net_usage = zmalloc_used.wrapping_sub((*mh).startup_allocated);
    }
    (*mh)
        .dataset_perc = (*mh).dataset as libc::c_float
        * 100 as libc::c_int as libc::c_float / net_usage as libc::c_float;
    (*mh)
        .bytes_per_key = if (*mh).total_keys != 0 {
        net_usage.wrapping_div((*mh).total_keys)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    return mh;
}
#[no_mangle]
pub unsafe extern "C" fn inputCatSds(
    mut result: *mut libc::c_void,
    mut str: *const libc::c_char,
) {
    let mut info: *mut sds = result as *mut sds;
    *info = sdscat(*info, str);
}
#[no_mangle]
pub unsafe extern "C" fn getMemoryDoctorReport() -> sds {
    let mut empty: libc::c_int = 0 as libc::c_int;
    let mut big_peak: libc::c_int = 0 as libc::c_int;
    let mut high_frag: libc::c_int = 0 as libc::c_int;
    let mut high_alloc_frag: libc::c_int = 0 as libc::c_int;
    let mut high_proc_rss: libc::c_int = 0 as libc::c_int;
    let mut high_alloc_rss: libc::c_int = 0 as libc::c_int;
    let mut big_slave_buf: libc::c_int = 0 as libc::c_int;
    let mut big_client_buf: libc::c_int = 0 as libc::c_int;
    let mut many_scripts: libc::c_int = 0 as libc::c_int;
    let mut num_reports: libc::c_int = 0 as libc::c_int;
    let mut mh: *mut redisMemOverhead = getMemoryOverheadData();
    if (*mh).total_allocated
        < (1024 as libc::c_int * 1024 as libc::c_int * 5 as libc::c_int) as libc::c_ulong
    {
        empty = 1 as libc::c_int;
        num_reports += 1;
    } else {
        if ((*mh).peak_allocated as libc::c_float
            / (*mh).total_allocated as libc::c_float) as libc::c_double > 1.5f64
        {
            big_peak = 1 as libc::c_int;
            num_reports += 1;
        }
        if (*mh).total_frag as libc::c_double > 1.4f64
            && (*mh).total_frag_bytes
                > ((10 as libc::c_int) << 20 as libc::c_int) as libc::c_long
        {
            high_frag = 1 as libc::c_int;
            num_reports += 1;
        }
        if (*mh).allocator_frag as libc::c_double > 1.1f64
            && (*mh).allocator_frag_bytes
                > ((10 as libc::c_int) << 20 as libc::c_int) as libc::c_long
        {
            high_alloc_frag = 1 as libc::c_int;
            num_reports += 1;
        }
        if (*mh).allocator_rss as libc::c_double > 1.1f64
            && (*mh).allocator_rss_bytes
                > ((10 as libc::c_int) << 20 as libc::c_int) as libc::c_long
        {
            high_alloc_rss = 1 as libc::c_int;
            num_reports += 1;
        }
        if (*mh).rss_extra as libc::c_double > 1.1f64
            && (*mh).rss_extra_bytes
                > ((10 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong
        {
            high_proc_rss = 1 as libc::c_int;
            num_reports += 1;
        }
        let mut numslaves: libc::c_long = (*server.slaves).len as libc::c_long;
        let mut numclients: libc::c_long = ((*server.clients).len)
            .wrapping_sub(numslaves as libc::c_ulong) as libc::c_long;
        if ((*mh).clients_normal).wrapping_div(numclients as libc::c_ulong)
            > (1024 as libc::c_int * 200 as libc::c_int) as libc::c_ulong
        {
            big_client_buf = 1 as libc::c_int;
            num_reports += 1;
        }
        if numslaves > 0 as libc::c_int as libc::c_long
            && (*mh).clients_slaves
                > (1024 as libc::c_int * 1024 as libc::c_int * 10 as libc::c_int)
                    as libc::c_ulong
        {
            big_slave_buf = 1 as libc::c_int;
            num_reports += 1;
        }
        if ((*evalScriptsDict()).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*evalScriptsDict()).ht_used[1 as libc::c_int as usize])
            > 1000 as libc::c_int as libc::c_ulong
        {
            many_scripts = 1 as libc::c_int;
            num_reports += 1;
        }
    }
    let mut s: sds = 0 as *mut libc::c_char;
    if num_reports == 0 as libc::c_int {
        s = sdsnew(
            b"Hi Sam, I can't find any memory issue in your instance. I can only account for what occurs on this base.\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if empty == 1 as libc::c_int {
        s = sdsnew(
            b"Hi Sam, this instance is empty or is using very little memory, my issues detector can't be used in these conditions. Please, leave for your mission on Earth and fill it with some data. The new Sam and I will be back to our programming as soon as I finished rebooting.\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        s = sdsnew(
            b"Sam, I detected a few issues in this Redis instance memory implants:\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        if big_peak != 0 {
            s = sdscat(
                s,
                b" * Peak memory: In the past this instance used more than 150% the memory that is currently using. The allocator is normally not able to release memory after a peak, so you can expect to see a big fragmentation ratio, however this is actually harmless and is only due to the memory peak, and if the Redis instance Resident Set Size (RSS) is currently bigger than expected, the memory will be used as soon as you fill the Redis instance with more data. If the memory peak was only occasional and you want to try to reclaim memory, please try the MEMORY PURGE command, otherwise the only other option is to shutdown and restart the instance.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if high_frag != 0 {
            s = sdscatprintf(
                s,
                b" * High total RSS: This instance has a memory fragmentation and RSS overhead greater than 1.4 (this means that the Resident Set Size of the Redis process is much larger than the sum of the logical allocations Redis performed). This problem is usually due either to a large peak memory (check if there is a peak memory entry above in the report) or may result from a workload that causes the allocator to fragment memory a lot. If the problem is a large peak memory, then there is no issue. Otherwise, make sure you are using the Jemalloc allocator and not the default libc malloc. Note: The currently used allocator is \"%s\".\n\n\0"
                    as *const u8 as *const libc::c_char,
                b"jemalloc-5.2.1\0" as *const u8 as *const libc::c_char,
            );
        }
        if high_alloc_frag != 0 {
            s = sdscatprintf(
                s,
                b" * High allocator fragmentation: This instance has an allocator external fragmentation greater than 1.1. This problem is usually due either to a large peak memory (check if there is a peak memory entry above in the report) or may result from a workload that causes the allocator to fragment memory a lot. You can try enabling 'activedefrag' config option.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if high_alloc_rss != 0 {
            s = sdscatprintf(
                s,
                b" * High allocator RSS overhead: This instance has an RSS memory overhead is greater than 1.1 (this means that the Resident Set Size of the allocator is much larger than the sum what the allocator actually holds). This problem is usually due to a large peak memory (check if there is a peak memory entry above in the report), you can try the MEMORY PURGE command to reclaim it.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if high_proc_rss != 0 {
            s = sdscatprintf(
                s,
                b" * High process RSS overhead: This instance has non-allocator RSS memory overhead is greater than 1.1 (this means that the Resident Set Size of the Redis process is much larger than the RSS the allocator holds). This problem may be due to Lua scripts or Modules.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if big_slave_buf != 0 {
            s = sdscat(
                s,
                b" * Big replica buffers: The replica output buffers in this instance are greater than 10MB for each replica (on average). This likely means that there is some replica instance that is struggling receiving data, either because it is too slow or because of networking issues. As a result, data piles on the master output buffers. Please try to identify what replica is not receiving data correctly and why. You can use the INFO output in order to check the replicas delays and the CLIENT LIST command to check the output buffers of each replica.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if big_client_buf != 0 {
            s = sdscat(
                s,
                b" * Big client buffers: The clients output buffers in this instance are greater than 200K per client (on average). This may result from different causes, like Pub/Sub clients subscribed to channels bot not receiving data fast enough, so that data piles on the Redis instance output buffer, or clients sending commands with large replies or very large sequences of commands in the same pipeline. Please use the CLIENT LIST command in order to investigate the issue if it causes problems in your instance, or to understand better why certain clients are using a big amount of memory.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if many_scripts != 0 {
            s = sdscat(
                s,
                b" * Many scripts: There seem to be many cached scripts in this instance (more than 1000). This may be because scripts are generated and `EVAL`ed, instead of being parameterized (with KEYS and ARGV), `SCRIPT LOAD`ed and `EVALSHA`ed. Unless `SCRIPT FLUSH` is called periodically, the scripts' caches may end up consuming most of your memory.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        s = sdscat(
            s,
            b"I'm here to keep you safe, Sam. I want to help you.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    freeMemoryOverheadData(mh);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn objectSetLRUOrLFU(
    mut val: *mut robj,
    mut lfu_freq: libc::c_longlong,
    mut lru_idle: libc::c_longlong,
    mut lru_clock: libc::c_longlong,
    mut lru_multiplier: libc::c_int,
) -> libc::c_int {
    if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if lfu_freq >= 0 as libc::c_int as libc::c_longlong {
            if lfu_freq <= 255 as libc::c_int as libc::c_longlong {} else {
                _serverAssert(
                    b"lfu_freq <= 255\0" as *const u8 as *const libc::c_char,
                    b"object.c\0" as *const u8 as *const libc::c_char,
                    1386 as libc::c_int,
                );
                unreachable!();
            };
            (*val)
                .set_lru(
                    ((LFUGetTimeInMinutes() << 8 as libc::c_int) as libc::c_ulonglong
                        | lfu_freq as libc::c_ulonglong) as libc::c_uint,
                );
            return 1 as libc::c_int;
        }
    } else if lru_idle >= 0 as libc::c_int as libc::c_longlong {
        lru_idle = lru_idle * lru_multiplier as libc::c_longlong
            / 1000 as libc::c_int as libc::c_longlong;
        let mut lru_abs: libc::c_long = (lru_clock - lru_idle) as libc::c_long;
        if lru_abs < 0 as libc::c_int as libc::c_long {
            lru_abs
                += (((1 as libc::c_int) << 24 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_long;
        }
        (*val).set_lru(lru_abs as libc::c_uint);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn objectCommandLookup(
    mut c: *mut client,
    mut key: *mut robj,
) -> *mut robj {
    return lookupKeyReadWithFlags(
        (*c).db,
        key,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn objectCommandLookupOrReply(
    mut c: *mut client,
    mut key: *mut robj,
    mut reply: *mut robj,
) -> *mut robj {
    let mut o: *mut robj = objectCommandLookup(c, key);
    if o.is_null() {
        addReplyOrErrorObject(c, reply);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn objectCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut help: [*const libc::c_char; 13] = [
            b"ENCODING <key>\0" as *const u8 as *const libc::c_char,
            b"    Return the kind of internal representation used in order to store the value\0"
                as *const u8 as *const libc::c_char,
            b"    associated with a <key>.\0" as *const u8 as *const libc::c_char,
            b"FREQ <key>\0" as *const u8 as *const libc::c_char,
            b"    Return the access frequency index of the <key>. The returned integer is\0"
                as *const u8 as *const libc::c_char,
            b"    proportional to the logarithm of the recent access frequency of the key.\0"
                as *const u8 as *const libc::c_char,
            b"IDLETIME <key>\0" as *const u8 as *const libc::c_char,
            b"    Return the idle time of the <key>, that is the approximated number of\0"
                as *const u8 as *const libc::c_char,
            b"    seconds elapsed since the last access to the key.\0" as *const u8
                as *const libc::c_char,
            b"REFCOUNT <key>\0" as *const u8 as *const libc::c_char,
            b"    Return the number of references of the value associated with the specified\0"
                as *const u8 as *const libc::c_char,
            b"    <key>.\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"refcount\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        o = objectCommandLookupOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            shared.null[(*c).resp as usize],
        );
        if o.is_null() {
            return;
        }
        addReplyLongLong(c, (*o).refcount as libc::c_longlong);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"encoding\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        o = objectCommandLookupOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            shared.null[(*c).resp as usize],
        );
        if o.is_null() {
            return;
        }
        addReplyBulkCString(c, strEncoding((*o).encoding() as libc::c_int));
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"idletime\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        o = objectCommandLookupOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            shared.null[(*c).resp as usize],
        );
        if o.is_null() {
            return;
        }
        if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            addReplyError(
                c,
                b"An LFU maxmemory policy is selected, idle time not tracked. Please note that when switching between policies at runtime LRU and LFU data will take some time to adjust.\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        addReplyLongLong(
            c,
            (estimateObjectIdleTime(o))
                .wrapping_div(1000 as libc::c_int as libc::c_ulonglong)
                as libc::c_longlong,
        );
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"freq\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        o = objectCommandLookupOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            shared.null[(*c).resp as usize],
        );
        if o.is_null() {
            return;
        }
        if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            addReplyError(
                c,
                b"An LFU maxmemory policy is not selected, access frequency not tracked. Please note that when switching between policies at runtime LRU and LFU data will take some time to adjust.\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        addReplyLongLong(c, LFUDecrAndReturn(o) as libc::c_longlong);
    } else {
        addReplySubcommandSyntaxError(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn memoryCommand(mut c: *mut client) {
    if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"help\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut help: [*const libc::c_char; 12] = [
            b"DOCTOR\0" as *const u8 as *const libc::c_char,
            b"    Return memory problems reports.\0" as *const u8 as *const libc::c_char,
            b"MALLOC-STATS\0" as *const u8 as *const libc::c_char,
            b"    Return internal statistics report from the memory allocator.\0"
                as *const u8 as *const libc::c_char,
            b"PURGE\0" as *const u8 as *const libc::c_char,
            b"    Attempt to purge dirty pages for reclamation by the allocator.\0"
                as *const u8 as *const libc::c_char,
            b"STATS\0" as *const u8 as *const libc::c_char,
            b"    Return information about the memory usage of the server.\0"
                as *const u8 as *const libc::c_char,
            b"USAGE <key> [SAMPLES <count>]\0" as *const u8 as *const libc::c_char,
            b"    Return memory in bytes used by <key> and its value. Nested values are\0"
                as *const u8 as *const libc::c_char,
            b"    sampled up to <count> times (default: 5, 0 means sample all).\0"
                as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"usage\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 3 as libc::c_int
    {
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        let mut samples: libc::c_longlong = 5 as libc::c_int as libc::c_longlong;
        let mut j: libc::c_int = 3 as libc::c_int;
        while j < (*c).argc {
            if strcasecmp(
                (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                b"samples\0" as *const u8 as *const libc::c_char,
            ) == 0 && (j + 1 as libc::c_int) < (*c).argc
            {
                if getLongLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                    &mut samples,
                    0 as *const libc::c_char,
                ) == -(1 as libc::c_int)
                {
                    return;
                }
                if samples < 0 as libc::c_int as libc::c_longlong {
                    addReplyErrorObject(c, shared.syntaxerr);
                    return;
                }
                if samples == 0 as libc::c_int as libc::c_longlong {
                    samples = 9223372036854775807 as libc::c_longlong;
                }
                j += 1;
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
            j += 1;
        }
        de = dictFind(
            (*(*c).db).dict,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr,
        );
        if de.is_null() {
            addReplyNull(c);
            return;
        }
        let mut usage: size_t = objectComputeSize(
            *((*c).argv).offset(2 as libc::c_int as isize),
            (*de).v.val as *mut robj,
            samples as size_t,
            (*(*c).db).id,
        );
        usage = (usage as libc::c_ulong).wrapping_add(sdsZmallocSize((*de).key as sds))
            as size_t as size_t;
        usage = (usage as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<dictEntry>() as libc::c_ulong) as size_t
            as size_t;
        usage = (usage as libc::c_ulong)
            .wrapping_add(
                if ((*(*(*(*c).db).dict).type_0).dictEntryMetadataBytes).is_some() {
                    ((*(*(*(*c).db).dict).type_0).dictEntryMetadataBytes)
                        .expect("non-null function pointer")((*(*c).db).dict)
                } else {
                    0 as libc::c_int as libc::c_ulong
                },
            ) as size_t as size_t;
        addReplyLongLong(c, usage as libc::c_longlong);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"stats\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut mh: *mut redisMemOverhead = getMemoryOverheadData();
        addReplyMapLen(
            c,
            (27 as libc::c_int as libc::c_ulong).wrapping_add((*mh).num_dbs)
                as libc::c_long,
        );
        addReplyBulkCString(c, b"peak.allocated\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).peak_allocated as libc::c_longlong);
        addReplyBulkCString(c, b"total.allocated\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).total_allocated as libc::c_longlong);
        addReplyBulkCString(
            c,
            b"startup.allocated\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(c, (*mh).startup_allocated as libc::c_longlong);
        addReplyBulkCString(
            c,
            b"replication.backlog\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(c, (*mh).repl_backlog as libc::c_longlong);
        addReplyBulkCString(c, b"clients.slaves\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).clients_slaves as libc::c_longlong);
        addReplyBulkCString(c, b"clients.normal\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).clients_normal as libc::c_longlong);
        addReplyBulkCString(c, b"cluster.links\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).cluster_links as libc::c_longlong);
        addReplyBulkCString(c, b"aof.buffer\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).aof_buffer as libc::c_longlong);
        addReplyBulkCString(c, b"lua.caches\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).lua_caches as libc::c_longlong);
        addReplyBulkCString(
            c,
            b"functions.caches\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(c, (*mh).functions_caches as libc::c_longlong);
        let mut j_0: size_t = 0 as libc::c_int as size_t;
        while j_0 < (*mh).num_dbs {
            let mut dbname: [libc::c_char; 32] = [0; 32];
            snprintf(
                dbname.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"db.%zd\0" as *const u8 as *const libc::c_char,
                (*((*mh).db).offset(j_0 as isize)).dbid,
            );
            addReplyBulkCString(c, dbname.as_mut_ptr());
            addReplyMapLen(c, 3 as libc::c_int as libc::c_long);
            addReplyBulkCString(
                c,
                b"overhead.hashtable.main\0" as *const u8 as *const libc::c_char,
            );
            addReplyLongLong(
                c,
                (*((*mh).db).offset(j_0 as isize)).overhead_ht_main as libc::c_longlong,
            );
            addReplyBulkCString(
                c,
                b"overhead.hashtable.expires\0" as *const u8 as *const libc::c_char,
            );
            addReplyLongLong(
                c,
                (*((*mh).db).offset(j_0 as isize)).overhead_ht_expires
                    as libc::c_longlong,
            );
            addReplyBulkCString(
                c,
                b"overhead.hashtable.slot-to-keys\0" as *const u8 as *const libc::c_char,
            );
            addReplyLongLong(
                c,
                (*((*mh).db).offset(j_0 as isize)).overhead_ht_slot_to_keys
                    as libc::c_longlong,
            );
            j_0 = j_0.wrapping_add(1);
        }
        addReplyBulkCString(c, b"overhead.total\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).overhead_total as libc::c_longlong);
        addReplyBulkCString(c, b"keys.count\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).total_keys as libc::c_longlong);
        addReplyBulkCString(
            c,
            b"keys.bytes-per-key\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(c, (*mh).bytes_per_key as libc::c_longlong);
        addReplyBulkCString(c, b"dataset.bytes\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*mh).dataset as libc::c_longlong);
        addReplyBulkCString(
            c,
            b"dataset.percentage\0" as *const u8 as *const libc::c_char,
        );
        addReplyDouble(c, (*mh).dataset_perc as libc::c_double);
        addReplyBulkCString(c, b"peak.percentage\0" as *const u8 as *const libc::c_char);
        addReplyDouble(c, (*mh).peak_perc as libc::c_double);
        addReplyBulkCString(
            c,
            b"allocator.allocated\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(
            c,
            server.cron_malloc_stats.allocator_allocated as libc::c_longlong,
        );
        addReplyBulkCString(
            c,
            b"allocator.active\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(
            c,
            server.cron_malloc_stats.allocator_active as libc::c_longlong,
        );
        addReplyBulkCString(
            c,
            b"allocator.resident\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(
            c,
            server.cron_malloc_stats.allocator_resident as libc::c_longlong,
        );
        addReplyBulkCString(
            c,
            b"allocator-fragmentation.ratio\0" as *const u8 as *const libc::c_char,
        );
        addReplyDouble(c, (*mh).allocator_frag as libc::c_double);
        addReplyBulkCString(
            c,
            b"allocator-fragmentation.bytes\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(c, (*mh).allocator_frag_bytes as libc::c_longlong);
        addReplyBulkCString(
            c,
            b"allocator-rss.ratio\0" as *const u8 as *const libc::c_char,
        );
        addReplyDouble(c, (*mh).allocator_rss as libc::c_double);
        addReplyBulkCString(
            c,
            b"allocator-rss.bytes\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(c, (*mh).allocator_rss_bytes as libc::c_longlong);
        addReplyBulkCString(
            c,
            b"rss-overhead.ratio\0" as *const u8 as *const libc::c_char,
        );
        addReplyDouble(c, (*mh).rss_extra as libc::c_double);
        addReplyBulkCString(
            c,
            b"rss-overhead.bytes\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(c, (*mh).rss_extra_bytes as libc::c_longlong);
        addReplyBulkCString(c, b"fragmentation\0" as *const u8 as *const libc::c_char);
        addReplyDouble(c, (*mh).total_frag as libc::c_double);
        addReplyBulkCString(
            c,
            b"fragmentation.bytes\0" as *const u8 as *const libc::c_char,
        );
        addReplyLongLong(c, (*mh).total_frag_bytes as libc::c_longlong);
        freeMemoryOverheadData(mh);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"malloc-stats\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut info: sds = sdsempty();
        je_malloc_stats_print(
            Some(
                inputCatSds
                    as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> (),
            ),
            &mut info as *mut sds as *mut libc::c_void,
            0 as *const libc::c_char,
        );
        addReplyVerbatim(
            c,
            info as *const libc::c_char,
            sdslen(info),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(info);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"doctor\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut report: sds = getMemoryDoctorReport();
        addReplyVerbatim(
            c,
            report as *const libc::c_char,
            sdslen(report),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(report);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"purge\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        if jemalloc_purge() == 0 as libc::c_int {
            addReply(c, shared.ok);
        } else {
            addReplyError(
                c,
                b"Error purging dirty pages\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        addReplySubcommandSyntaxError(c);
    };
}
