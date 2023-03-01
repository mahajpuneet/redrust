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
    fn sdsdup(s: sds) -> sds;
    fn sdscmp(s1: sds, s2: sds) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictAddRaw(
        d: *mut dict,
        key: *mut libc::c_void,
        existing: *mut *mut dictEntry,
    ) -> *mut dictEntry;
    fn dictAddOrFind(d: *mut dict, key: *mut libc::c_void) -> *mut dictEntry;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn dictUnlink(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFreeUnlinkedEntry(d: *mut dict, he: *mut dictEntry);
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictGetFairRandomKey(d: *mut dict) -> *mut dictEntry;
    fn dictEmpty(
        d: *mut dict,
        callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
    );
    fn dictScan(
        d: *mut dict,
        v: libc::c_ulong,
        fn_0: Option::<dictScanFunction>,
        bucketfn: Option::<dictScanBucketFunction>,
        privdata: *mut libc::c_void,
    ) -> libc::c_ulong;
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn jemalloc_purge() -> libc::c_int;
    fn intsetGet(is: *mut intset, pos: uint32_t, value: *mut int64_t) -> uint8_t;
    fn stringmatchlen(
        p: *const libc::c_char,
        plen: libc::c_int,
        s: *const libc::c_char,
        slen: libc::c_int,
        nocase: libc::c_int,
    ) -> libc::c_int;
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
    fn latencyAddSample(event: *const libc::c_char, latency: mstime_t);
    fn lpGet(
        p: *mut libc::c_uchar,
        count: *mut int64_t,
        intbuf: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpFirst(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpNext(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn streamDup(o: *mut robj) -> *mut robj;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut dbDictType: dictType;
    static mut dbExpiresDictType: dictType;
    fn moduleGetCommandKeysViaAPI(
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        result: *mut getKeysResult,
    ) -> libc::c_int;
    fn moduleGetCommandChannelsViaAPI(
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        result: *mut getKeysResult,
    ) -> libc::c_int;
    fn moduleFireServerEvent(eid: uint64_t, subid: libc::c_int, data: *mut libc::c_void);
    fn moduleNotifyKeyUnlink(key: *mut robj, val: *mut robj, dbid: libc::c_int);
    fn moduleTypeDupOrReply(
        c: *mut client,
        fromkey: *mut robj,
        tokey: *mut robj,
        todb: libc::c_int,
        value: *mut robj,
    ) -> *mut robj;
    fn mstime() -> libc::c_longlong;
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn addReplyNull(c: *mut client);
    fn addReplyBulk(c: *mut client, obj: *mut robj);
    fn addReplyBulkLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyOrErrorObject(c: *mut client, reply: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyStatus(c: *mut client, status: *const libc::c_char);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn checkClientPauseTimeoutAndReturnIfPaused() -> libc::c_int;
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn trackingInvalidateKey(c: *mut client, keyobj: *mut robj, bcast: libc::c_int);
    fn trackingInvalidateKeysOnFlush(async_0: libc::c_int);
    fn listTypeDup(o: *mut robj) -> *mut robj;
    fn touchWatchedKey(db: *mut redisDb, key: *mut robj);
    fn touchAllWatchedKeysInDb(emptied: *mut redisDb, replaced_with: *mut redisDb);
    fn decrRefCount(o: *mut robj);
    fn decrRefCountVoid(o: *mut libc::c_void);
    fn incrRefCount(o: *mut robj);
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn createRawStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn dupStringObject(o: *const robj) -> *mut robj;
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn createStringObjectFromLongLong(value: libc::c_longlong) -> *mut robj;
    fn createStringObjectFromLongDouble(
        value: f64,
        humanfriendly: libc::c_int,
    ) -> *mut robj;
    fn getLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getIntFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_int,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn rdbSave(
        req: libc::c_int,
        filename: *mut libc::c_char,
        rsi: *mut rdbSaveInfo,
    ) -> libc::c_int;
    fn rdbPopulateSaveInfo(rsi: *mut rdbSaveInfo) -> *mut rdbSaveInfo;
    fn killRDBChild();
    fn hasActiveChildProcess() -> libc::c_int;
    fn isInsideYieldingLongCommand() -> libc::c_int;
    fn prepareForShutdown(flags: libc::c_int) -> libc::c_int;
    fn abortShutdown() -> libc::c_int;
    fn LRU_CLOCK() -> libc::c_uint;
    fn setTypeDup(o: *mut robj) -> *mut robj;
    fn hashTypeDup(o: *mut robj) -> *mut robj;
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn forceCommandPropagation(c: *mut client, flags: libc::c_int);
    fn freeObjAsync(key: *mut robj, obj: *mut robj, dbid: libc::c_int);
    fn alsoPropagate(
        dbid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
        target: libc::c_int,
    );
    fn emptyDbAsync(db: *mut redisDb);
    fn zsetDup(o: *mut robj) -> *mut robj;
    fn signalKeyAsReady(db: *mut redisDb, key: *mut robj, type_0: libc::c_int);
    fn rememberSlaveKeyWithExpire(db: *mut redisDb, key: *mut robj);
    fn flushSlaveKeysWithExpireList();
    fn LFULogIncr(value: uint8_t) -> uint8_t;
    fn blockClient(c: *mut client, btype: libc::c_int);
    fn LFUGetTimeInMinutes() -> libc::c_ulong;
    fn LFUDecrAndReturn(o: *mut robj) -> libc::c_ulong;
    fn subscribeCommand(c: *mut client);
    fn ssubscribeCommand(c: *mut client);
    fn unsubscribeCommand(c: *mut client);
    fn sunsubscribeCommand(c: *mut client);
    fn psubscribeCommand(c: *mut client);
    fn punsubscribeCommand(c: *mut client);
    fn publishCommand(c: *mut client);
    fn spublishCommand(c: *mut client);
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
    fn _serverAssertWithInfo(
        c: *const client,
        o: *const robj,
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn slotToKeyAddEntry(entry: *mut dictEntry, db: *mut redisDb);
    fn slotToKeyDelEntry(entry: *mut dictEntry, db: *mut redisDb);
    fn slotToKeyInit(db: *mut redisDb);
    fn slotToKeyFlush(db: *mut redisDb);
    fn slotToKeyDestroy(db: *mut redisDb);
    fn scriptIsEval() -> libc::c_int;
    fn scriptTimeSnapshot() -> mstime_t;
    fn functionsLibCtxClearCurrent(async_0: libc::c_int);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub type dictScanFunction = unsafe extern "C" fn(
    *mut libc::c_void,
    *const dictEntry,
) -> ();
pub type dictScanBucketFunction = unsafe extern "C" fn(
    *mut dict,
    *mut *mut dictEntry,
) -> ();
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
pub struct RedisModuleFlushInfo {
    pub version: uint64_t,
    pub sync: int32_t,
    pub dbnum: int32_t,
}
pub type RedisModuleFlushInfoV1 = RedisModuleFlushInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleSwapDbInfo {
    pub version: uint64_t,
    pub dbnum_first: int32_t,
    pub dbnum_second: int32_t,
}
pub type RedisModuleSwapDbInfoV1 = RedisModuleSwapDbInfo;
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
pub struct rdbSaveInfo {
    pub repl_stream_db: libc::c_int,
    pub repl_id_is_set: libc::c_int,
    pub repl_id: [libc::c_char; 41],
    pub repl_offset: libc::c_longlong,
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
pub struct C2RustUnnamed_11 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
pub const _ISspace: C2RustUnnamed_14 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChannelSpecs {
    pub proc_0: Option::<redisCommandProc>,
    pub flags: uint64_t,
    pub start: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub name: *mut libc::c_char,
    pub skip: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub name: *mut libc::c_char,
    pub skip: libc::c_int,
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_14 = 8;
pub const _ISpunct: C2RustUnnamed_14 = 4;
pub const _IScntrl: C2RustUnnamed_14 = 2;
pub const _ISblank: C2RustUnnamed_14 = 1;
pub const _ISgraph: C2RustUnnamed_14 = 32768;
pub const _ISprint: C2RustUnnamed_14 = 16384;
pub const _ISxdigit: C2RustUnnamed_14 = 4096;
pub const _ISdigit: C2RustUnnamed_14 = 2048;
pub const _ISalpha: C2RustUnnamed_14 = 1024;
pub const _ISlower: C2RustUnnamed_14 = 512;
pub const _ISupper: C2RustUnnamed_14 = 256;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn updateLFU(mut val: *mut robj) {
    let mut counter: libc::c_ulong = LFUDecrAndReturn(val);
    counter = LFULogIncr(counter as uint8_t) as libc::c_ulong;
    (*val)
        .set_lru((LFUGetTimeInMinutes() << 8 as libc::c_int | counter) as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lookupKey(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut flags: libc::c_int,
) -> *mut robj {
    let mut de: *mut dictEntry = dictFind((*db).dict, (*key).ptr);
    let mut val: *mut robj = 0 as *mut robj;
    if !de.is_null() {
        val = (*de).v.val as *mut robj;
        let mut is_ro_replica: libc::c_int = (!(server.masterhost).is_null()
            && server.repl_slave_ro != 0) as libc::c_int;
        let mut expire_flags: libc::c_int = 0 as libc::c_int;
        if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 && is_ro_replica == 0 {
            expire_flags |= 1 as libc::c_int;
        }
        if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            expire_flags |= 2 as libc::c_int;
        }
        if expireIfNeeded(db, key, expire_flags) != 0 {
            val = 0 as *mut robj;
        }
    }
    if !val.is_null() {
        if hasActiveChildProcess() == 0
            && flags & (1 as libc::c_int) << 0 as libc::c_int == 0
        {
            if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                updateLFU(val);
            } else {
                (*val).set_lru(LRU_CLOCK());
            }
        }
        if flags
            & ((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int) == 0
        {
            server.stat_keyspace_hits += 1;
        }
    } else {
        if flags
            & ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int) == 0
        {
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 11 as libc::c_int,
                b"keymiss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                key,
                (*db).id,
            );
        }
        if flags
            & ((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int) == 0
        {
            server.stat_keyspace_misses += 1;
        }
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lookupKeyReadWithFlags(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut flags: libc::c_int,
) -> *mut robj {
    if flags & (1 as libc::c_int) << 3 as libc::c_int == 0 {} else {
        _serverAssert(
            b"!(flags & LOOKUP_WRITE)\0" as *const u8 as *const libc::c_char,
            b"db.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
        );
        unreachable!();
    };
    return lookupKey(db, key, flags);
}
#[no_mangle]
pub unsafe extern "C" fn lookupKeyRead(
    mut db: *mut redisDb,
    mut key: *mut robj,
) -> *mut robj {
    return lookupKeyReadWithFlags(db, key, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lookupKeyWriteWithFlags(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut flags: libc::c_int,
) -> *mut robj {
    return lookupKey(db, key, flags | (1 as libc::c_int) << 3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lookupKeyWrite(
    mut db: *mut redisDb,
    mut key: *mut robj,
) -> *mut robj {
    return lookupKeyWriteWithFlags(db, key, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lookupKeyReadOrReply(
    mut c: *mut client,
    mut key: *mut robj,
    mut reply: *mut robj,
) -> *mut robj {
    let mut o: *mut robj = lookupKeyRead((*c).db, key);
    if o.is_null() {
        addReplyOrErrorObject(c, reply);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn lookupKeyWriteOrReply(
    mut c: *mut client,
    mut key: *mut robj,
    mut reply: *mut robj,
) -> *mut robj {
    let mut o: *mut robj = lookupKeyWrite((*c).db, key);
    if o.is_null() {
        addReplyOrErrorObject(c, reply);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn dbAdd(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut val: *mut robj,
) {
    let mut copy: sds = sdsdup((*key).ptr as sds);
    let mut de: *mut dictEntry = dictAddRaw(
        (*db).dict,
        copy as *mut libc::c_void,
        0 as *mut *mut dictEntry,
    );
    if !de.is_null() {} else {
        _serverAssertWithInfo(
            0 as *const client,
            key,
            b"de != NULL\0" as *const u8 as *const libc::c_char,
            b"db.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
        );
        unreachable!();
    };
    if ((*(*(*db).dict).type_0).valDup).is_some() {
        (*de)
            .v
            .val = ((*(*(*db).dict).type_0).valDup)
            .expect("non-null function pointer")((*db).dict, val as *const libc::c_void);
    } else {
        (*de).v.val = val as *mut libc::c_void;
    }
    signalKeyAsReady(db, key, (*val).type_0() as libc::c_int);
    if server.cluster_enabled != 0 {
        slotToKeyAddEntry(de, db);
    }
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 14 as libc::c_int,
        b"new\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        key,
        (*db).id,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dbAddRDBLoad(
    mut db: *mut redisDb,
    mut key: sds,
    mut val: *mut robj,
) -> libc::c_int {
    let mut de: *mut dictEntry = dictAddRaw(
        (*db).dict,
        key as *mut libc::c_void,
        0 as *mut *mut dictEntry,
    );
    if de.is_null() {
        return 0 as libc::c_int;
    }
    if ((*(*(*db).dict).type_0).valDup).is_some() {
        (*de)
            .v
            .val = ((*(*(*db).dict).type_0).valDup)
            .expect("non-null function pointer")((*db).dict, val as *const libc::c_void);
    } else {
        (*de).v.val = val as *mut libc::c_void;
    }
    if server.cluster_enabled != 0 {
        slotToKeyAddEntry(de, db);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dbOverwrite(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut val: *mut robj,
) {
    let mut de: *mut dictEntry = dictFind((*db).dict, (*key).ptr);
    if !de.is_null() {} else {
        _serverAssertWithInfo(
            0 as *const client,
            key,
            b"de != NULL\0" as *const u8 as *const libc::c_char,
            b"db.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
        );
        unreachable!();
    };
    let mut auxentry: dictEntry = *de;
    let mut old: *mut robj = (*de).v.val as *mut robj;
    if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*val).set_lru((*old).lru());
    }
    moduleNotifyKeyUnlink(key, old, (*db).id);
    if (*old).type_0() as libc::c_int == 6 as libc::c_int {
        signalKeyAsReady(db, key, (*old).type_0() as libc::c_int);
    }
    if ((*(*(*db).dict).type_0).valDup).is_some() {
        (*de)
            .v
            .val = ((*(*(*db).dict).type_0).valDup)
            .expect("non-null function pointer")((*db).dict, val as *const libc::c_void);
    } else {
        (*de).v.val = val as *mut libc::c_void;
    }
    if server.lazyfree_lazy_server_del != 0 {
        freeObjAsync(key, old, (*db).id);
        if ((*(*(*db).dict).type_0).valDup).is_some() {
            auxentry
                .v
                .val = ((*(*(*db).dict).type_0).valDup)
                .expect(
                    "non-null function pointer",
                )((*db).dict, 0 as *const libc::c_void);
        } else {
            auxentry.v.val = 0 as *mut libc::c_void;
        }
    }
    if ((*(*(*db).dict).type_0).valDestructor).is_some() {
        ((*(*(*db).dict).type_0).valDestructor)
            .expect("non-null function pointer")((*db).dict, auxentry.v.val);
    }
}
#[no_mangle]
pub unsafe extern "C" fn setKey(
    mut c: *mut client,
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut val: *mut robj,
    mut flags: libc::c_int,
) {
    let mut keyfound: libc::c_int = 0 as libc::c_int;
    if flags & 4 as libc::c_int != 0 {
        keyfound = 1 as libc::c_int;
    } else if flags & 8 as libc::c_int == 0 {
        keyfound = (lookupKeyWrite(db, key) != 0 as *mut libc::c_void as *mut robj)
            as libc::c_int;
    }
    if keyfound == 0 {
        dbAdd(db, key, val);
    } else {
        dbOverwrite(db, key, val);
    }
    incrRefCount(val);
    if flags & 1 as libc::c_int == 0 {
        removeExpire(db, key);
    }
    if flags & 2 as libc::c_int == 0 {
        signalModifiedKey(c, db, key);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dbRandomKey(mut db: *mut redisDb) -> *mut robj {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut maxtries: libc::c_int = 100 as libc::c_int;
    let mut allvolatile: libc::c_int = (((*(*db).dict)
        .ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*db).dict).ht_used[1 as libc::c_int as usize])
        == ((*(*db).expires).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*(*db).expires).ht_used[1 as libc::c_int as usize]))
        as libc::c_int;
    loop {
        let mut key: sds = 0 as *mut libc::c_char;
        let mut keyobj: *mut robj = 0 as *mut robj;
        de = dictGetFairRandomKey((*db).dict);
        if de.is_null() {
            return 0 as *mut robj;
        }
        key = (*de).key as sds;
        keyobj = createStringObject(key as *const libc::c_char, sdslen(key));
        if !(dictFind((*db).expires, key as *const libc::c_void)).is_null() {
            if allvolatile != 0 && !(server.masterhost).is_null()
                && {
                    maxtries -= 1;
                    maxtries == 0 as libc::c_int
                }
            {
                return keyobj;
            }
            if expireIfNeeded(db, keyobj, 0 as libc::c_int) != 0 {
                decrRefCount(keyobj);
                continue;
            }
        }
        return keyobj;
    };
}
unsafe extern "C" fn dbGenericDelete(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut async_0: libc::c_int,
) -> libc::c_int {
    if ((*(*db).expires).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*db).expires).ht_used[1 as libc::c_int as usize])
        > 0 as libc::c_int as libc::c_ulong
    {
        dictDelete((*db).expires, (*key).ptr);
    }
    let mut de: *mut dictEntry = dictUnlink((*db).dict, (*key).ptr);
    if !de.is_null() {
        let mut val: *mut robj = (*de).v.val as *mut robj;
        moduleNotifyKeyUnlink(key, val, (*db).id);
        if (*val).type_0() as libc::c_int == 6 as libc::c_int {
            signalKeyAsReady(db, key, (*val).type_0() as libc::c_int);
        }
        if async_0 != 0 {
            freeObjAsync(key, val, (*db).id);
            if ((*(*(*db).dict).type_0).valDup).is_some() {
                (*de)
                    .v
                    .val = ((*(*(*db).dict).type_0).valDup)
                    .expect(
                        "non-null function pointer",
                    )((*db).dict, 0 as *const libc::c_void);
            } else {
                (*de).v.val = 0 as *mut libc::c_void;
            }
        }
        if server.cluster_enabled != 0 {
            slotToKeyDelEntry(de, db);
        }
        dictFreeUnlinkedEntry((*db).dict, de);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn dbSyncDelete(
    mut db: *mut redisDb,
    mut key: *mut robj,
) -> libc::c_int {
    return dbGenericDelete(db, key, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dbAsyncDelete(
    mut db: *mut redisDb,
    mut key: *mut robj,
) -> libc::c_int {
    return dbGenericDelete(db, key, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dbDelete(
    mut db: *mut redisDb,
    mut key: *mut robj,
) -> libc::c_int {
    return dbGenericDelete(db, key, server.lazyfree_lazy_server_del);
}
#[no_mangle]
pub unsafe extern "C" fn dbUnshareStringValue(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut o: *mut robj,
) -> *mut robj {
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssert(
            b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"db.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int,
        );
        unreachable!();
    };
    if (*o).refcount != 1 as libc::c_int
        || (*o).encoding() as libc::c_int != 0 as libc::c_int
    {
        let mut decoded: *mut robj = getDecodedObject(o);
        o = createRawStringObject(
            (*decoded).ptr as *const libc::c_char,
            sdslen((*decoded).ptr as sds),
        );
        decrRefCount(decoded);
        dbOverwrite(db, key, o);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn emptyDbStructure(
    mut dbarray: *mut redisDb,
    mut dbnum: libc::c_int,
    mut async_0: libc::c_int,
    mut callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
) -> libc::c_longlong {
    let mut removed: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut startdb: libc::c_int = 0;
    let mut enddb: libc::c_int = 0;
    if dbnum == -(1 as libc::c_int) {
        startdb = 0 as libc::c_int;
        enddb = server.dbnum - 1 as libc::c_int;
    } else {
        enddb = dbnum;
        startdb = enddb;
    }
    let mut j: libc::c_int = startdb;
    while j <= enddb {
        removed = (removed as libc::c_ulonglong)
            .wrapping_add(
                ((*(*dbarray.offset(j as isize)).dict)
                    .ht_used[0 as libc::c_int as usize])
                    .wrapping_add(
                        (*(*dbarray.offset(j as isize)).dict)
                            .ht_used[1 as libc::c_int as usize],
                    ) as libc::c_ulonglong,
            ) as libc::c_longlong as libc::c_longlong;
        if async_0 != 0 {
            emptyDbAsync(&mut *dbarray.offset(j as isize));
        } else {
            dictEmpty((*dbarray.offset(j as isize)).dict, callback);
            dictEmpty((*dbarray.offset(j as isize)).expires, callback);
        }
        (*dbarray.offset(j as isize)).avg_ttl = 0 as libc::c_int as libc::c_longlong;
        (*dbarray.offset(j as isize)).expires_cursor = 0 as libc::c_int as libc::c_ulong;
        j += 1;
    }
    return removed;
}
#[no_mangle]
pub unsafe extern "C" fn emptyData(
    mut dbnum: libc::c_int,
    mut flags: libc::c_int,
    mut callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
) -> libc::c_longlong {
    let mut async_0: libc::c_int = flags & (1 as libc::c_int) << 0 as libc::c_int;
    let mut with_functions: libc::c_int = (flags & (1 as libc::c_int) << 1 as libc::c_int
        == 0) as libc::c_int;
    let mut fi: RedisModuleFlushInfoV1 = {
        let mut init = RedisModuleFlushInfo {
            version: 1 as libc::c_int as uint64_t,
            sync: (async_0 == 0) as libc::c_int,
            dbnum: dbnum,
        };
        init
    };
    let mut removed: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if dbnum < -(1 as libc::c_int) || dbnum >= server.dbnum {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    moduleFireServerEvent(
        2 as libc::c_int as uint64_t,
        0 as libc::c_int,
        &mut fi as *mut RedisModuleFlushInfoV1 as *mut libc::c_void,
    );
    signalFlushedDb(dbnum, async_0);
    removed = emptyDbStructure(server.db, dbnum, async_0, callback);
    if server.cluster_enabled != 0 {
        slotToKeyFlush(server.db);
    }
    if dbnum == -(1 as libc::c_int) {
        flushSlaveKeysWithExpireList();
    }
    if with_functions != 0 {
        if dbnum == -(1 as libc::c_int) {} else {
            _serverAssert(
                b"dbnum == -1\0" as *const u8 as *const libc::c_char,
                b"db.c\0" as *const u8 as *const libc::c_char,
                480 as libc::c_int,
            );
            unreachable!();
        };
        functionsLibCtxClearCurrent(async_0);
    }
    moduleFireServerEvent(
        2 as libc::c_int as uint64_t,
        1 as libc::c_int,
        &mut fi as *mut RedisModuleFlushInfoV1 as *mut libc::c_void,
    );
    return removed;
}
#[no_mangle]
pub unsafe extern "C" fn initTempDb() -> *mut redisDb {
    let mut tempDb: *mut redisDb = zcalloc(
        (core::mem::size_of::<redisDb>() as libc::c_ulong)
            .wrapping_mul(server.dbnum as libc::c_ulong),
    ) as *mut redisDb;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < server.dbnum {
        let ref mut fresh0 = (*tempDb.offset(i as isize)).dict;
        *fresh0 = dictCreate(&mut dbDictType);
        let ref mut fresh1 = (*tempDb.offset(i as isize)).expires;
        *fresh1 = dictCreate(&mut dbExpiresDictType);
        let ref mut fresh2 = (*tempDb.offset(i as isize)).slots_to_keys;
        *fresh2 = 0 as *mut clusterSlotToKeyMapping;
        i += 1;
    }
    if server.cluster_enabled != 0 {
        slotToKeyInit(tempDb);
    }
    return tempDb;
}
#[no_mangle]
pub unsafe extern "C" fn discardTempDb(
    mut tempDb: *mut redisDb,
    mut callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
) {
    let mut async_0: libc::c_int = 1 as libc::c_int;
    emptyDbStructure(tempDb, -(1 as libc::c_int), async_0, callback);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < server.dbnum {
        dictRelease((*tempDb.offset(i as isize)).dict);
        dictRelease((*tempDb.offset(i as isize)).expires);
        i += 1;
    }
    if server.cluster_enabled != 0 {
        slotToKeyDestroy(tempDb);
    }
    zfree(tempDb as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn selectDb(
    mut c: *mut client,
    mut id: libc::c_int,
) -> libc::c_int {
    if id < 0 as libc::c_int || id >= server.dbnum {
        return -(1 as libc::c_int);
    }
    (*c).db = &mut *(server.db).offset(id as isize) as *mut redisDb;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dbTotalServerKeyCount() -> libc::c_longlong {
    let mut total: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < server.dbnum {
        total = (total as libc::c_ulonglong)
            .wrapping_add(
                ((*(*(server.db).offset(j as isize)).dict)
                    .ht_used[0 as libc::c_int as usize])
                    .wrapping_add(
                        (*(*(server.db).offset(j as isize)).dict)
                            .ht_used[1 as libc::c_int as usize],
                    ) as libc::c_ulonglong,
            ) as libc::c_longlong as libc::c_longlong;
        j += 1;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn signalModifiedKey(
    mut c: *mut client,
    mut db: *mut redisDb,
    mut key: *mut robj,
) {
    touchWatchedKey(db, key);
    trackingInvalidateKey(c, key, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn signalFlushedDb(
    mut dbid: libc::c_int,
    mut async_0: libc::c_int,
) {
    let mut startdb: libc::c_int = 0;
    let mut enddb: libc::c_int = 0;
    if dbid == -(1 as libc::c_int) {
        startdb = 0 as libc::c_int;
        enddb = server.dbnum - 1 as libc::c_int;
    } else {
        enddb = dbid;
        startdb = enddb;
    }
    let mut j: libc::c_int = startdb;
    while j <= enddb {
        scanDatabaseForDeletedStreams(
            &mut *(server.db).offset(j as isize),
            0 as *mut redisDb,
        );
        touchAllWatchedKeysInDb(&mut *(server.db).offset(j as isize), 0 as *mut redisDb);
        j += 1;
    }
    trackingInvalidateKeysOnFlush(async_0);
}
#[no_mangle]
pub unsafe extern "C" fn getFlushCommandFlags(
    mut c: *mut client,
    mut flags: *mut libc::c_int,
) -> libc::c_int {
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"sync\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        *flags = 0 as libc::c_int;
    } else if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"async\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        *flags = (1 as libc::c_int) << 0 as libc::c_int;
    } else if (*c).argc == 1 as libc::c_int {
        *flags = if server.lazyfree_lazy_user_flush != 0 {
            (1 as libc::c_int) << 0 as libc::c_int
        } else {
            0 as libc::c_int
        };
    } else {
        addReplyErrorObject(c, shared.syntaxerr);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn flushAllDataAndResetRDB(mut flags: libc::c_int) {
    server.dirty += emptyData(-(1 as libc::c_int), flags, None);
    if server.child_type == 1 as libc::c_int {
        killRDBChild();
    }
    if server.saveparamslen > 0 as libc::c_int {
        let mut rsi: rdbSaveInfo = rdbSaveInfo {
            repl_stream_db: 0,
            repl_id_is_set: 0,
            repl_id: [0; 41],
            repl_offset: 0,
        };
        let mut rsiptr: *mut rdbSaveInfo = 0 as *mut rdbSaveInfo;
        rsiptr = rdbPopulateSaveInfo(&mut rsi);
        rdbSave(0 as libc::c_int, server.rdb_filename, rsiptr);
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        jemalloc_purge();
    }
}
#[no_mangle]
pub unsafe extern "C" fn flushdbCommand(mut c: *mut client) {
    let mut flags: libc::c_int = 0;
    if getFlushCommandFlags(c, &mut flags) == -(1 as libc::c_int) {
        return;
    }
    server.dirty
        += emptyData(
            (*(*c).db).id,
            flags | (1 as libc::c_int) << 1 as libc::c_int,
            None,
        );
    forceCommandPropagation(c, 2 as libc::c_int | 1 as libc::c_int);
    addReply(c, shared.ok);
    if flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        jemalloc_purge();
    }
}
#[no_mangle]
pub unsafe extern "C" fn flushallCommand(mut c: *mut client) {
    let mut flags: libc::c_int = 0;
    if getFlushCommandFlags(c, &mut flags) == -(1 as libc::c_int) {
        return;
    }
    flushAllDataAndResetRDB(flags | (1 as libc::c_int) << 1 as libc::c_int);
    forceCommandPropagation(c, 2 as libc::c_int | 1 as libc::c_int);
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn delGenericCommand(mut c: *mut client, mut lazy: libc::c_int) {
    let mut numdel: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j < (*c).argc {
        expireIfNeeded((*c).db, *((*c).argv).offset(j as isize), 0 as libc::c_int);
        let mut deleted: libc::c_int = if lazy != 0 {
            dbAsyncDelete((*c).db, *((*c).argv).offset(j as isize))
        } else {
            dbSyncDelete((*c).db, *((*c).argv).offset(j as isize))
        };
        if deleted != 0 {
            signalModifiedKey(c, (*c).db, *((*c).argv).offset(j as isize));
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 2 as libc::c_int,
                b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                *((*c).argv).offset(j as isize),
                (*(*c).db).id,
            );
            server.dirty += 1;
            numdel += 1;
        }
        j += 1;
    }
    addReplyLongLong(c, numdel as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn delCommand(mut c: *mut client) {
    delGenericCommand(c, server.lazyfree_lazy_user_del);
}
#[no_mangle]
pub unsafe extern "C" fn unlinkCommand(mut c: *mut client) {
    delGenericCommand(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn existsCommand(mut c: *mut client) {
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j < (*c).argc {
        if !(lookupKeyReadWithFlags(
            (*c).db,
            *((*c).argv).offset(j as isize),
            (1 as libc::c_int) << 0 as libc::c_int,
        ))
            .is_null()
        {
            count += 1;
        }
        j += 1;
    }
    addReplyLongLong(c, count);
}
#[no_mangle]
pub unsafe extern "C" fn selectCommand(mut c: *mut client) {
    let mut id: libc::c_int = 0;
    if getIntFromObjectOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        &mut id,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if server.cluster_enabled != 0 && id != 0 as libc::c_int {
        addReplyError(
            c,
            b"SELECT is not allowed in cluster mode\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if selectDb(c, id) == -(1 as libc::c_int) {
        addReplyError(
            c,
            b"DB index is out of range\0" as *const u8 as *const libc::c_char,
        );
    } else {
        addReply(c, shared.ok);
    };
}
#[no_mangle]
pub unsafe extern "C" fn randomkeyCommand(mut c: *mut client) {
    let mut key: *mut robj = 0 as *mut robj;
    key = dbRandomKey((*c).db);
    if key.is_null() {
        addReplyNull(c);
        return;
    }
    addReplyBulk(c, key);
    decrRefCount(key);
}
#[no_mangle]
pub unsafe extern "C" fn keysCommand(mut c: *mut client) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut pattern: sds = (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as sds;
    let mut plen: libc::c_int = sdslen(pattern) as libc::c_int;
    let mut allkeys: libc::c_int = 0;
    let mut numkeys: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
    di = dictGetSafeIterator((*(*c).db).dict);
    allkeys = (*pattern.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
        && plen == 1 as libc::c_int) as libc::c_int;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut key: sds = (*de).key as sds;
        let mut keyobj: *mut robj = 0 as *mut robj;
        if allkeys != 0
            || stringmatchlen(
                pattern as *const libc::c_char,
                plen,
                key as *const libc::c_char,
                sdslen(key) as libc::c_int,
                0 as libc::c_int,
            ) != 0
        {
            keyobj = createStringObject(key as *const libc::c_char, sdslen(key));
            if keyIsExpired((*c).db, keyobj) == 0 {
                addReplyBulk(c, keyobj);
                numkeys = numkeys.wrapping_add(1);
            }
            decrRefCount(keyobj);
        }
        if (*c).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0 {
            break;
        }
    }
    dictReleaseIterator(di);
    setDeferredArrayLen(c, replylen, numkeys as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn scanCallback(
    mut privdata: *mut libc::c_void,
    mut de: *const dictEntry,
) {
    let mut pd: *mut *mut libc::c_void = privdata as *mut *mut libc::c_void;
    let mut keys: *mut list = *pd.offset(0 as libc::c_int as isize) as *mut list;
    let mut o: *mut robj = *pd.offset(1 as libc::c_int as isize) as *mut robj;
    let mut key: *mut robj = 0 as *mut robj;
    let mut val: *mut robj = 0 as *mut robj;
    if o.is_null() {
        let mut sdskey: sds = (*de).key as sds;
        key = createStringObject(sdskey as *const libc::c_char, sdslen(sdskey));
    } else if (*o).type_0() as libc::c_int == 2 as libc::c_int {
        let mut keysds: sds = (*de).key as sds;
        key = createStringObject(keysds as *const libc::c_char, sdslen(keysds));
    } else if (*o).type_0() as libc::c_int == 4 as libc::c_int {
        let mut sdskey_0: sds = (*de).key as sds;
        let mut sdsval: sds = (*de).v.val as sds;
        key = createStringObject(sdskey_0 as *const libc::c_char, sdslen(sdskey_0));
        val = createStringObject(sdsval as *const libc::c_char, sdslen(sdsval));
    } else if (*o).type_0() as libc::c_int == 3 as libc::c_int {
        let mut sdskey_1: sds = (*de).key as sds;
        key = createStringObject(sdskey_1 as *const libc::c_char, sdslen(sdskey_1));
        let c_double_ptr = (*de).v.val as *mut libc::c_double;
        let new_f64_value = *c_double_ptr as f64;
        val = createStringObjectFromLongDouble(
            new_f64_value,
            0 as libc::c_int,
        );
    } else {
        _serverPanic(
            b"db.c\0" as *const u8 as *const libc::c_char,
            791 as libc::c_int,
            b"Type not handled in SCAN callback.\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    listAddNodeTail(keys, key as *mut libc::c_void);
    if !val.is_null() {
        listAddNodeTail(keys, val as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn parseScanCursorOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut cursor: *mut libc::c_ulong,
) -> libc::c_int {
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    *cursor = strtoul((*o).ptr as *const libc::c_char, &mut eptr, 10 as libc::c_int);
    if *(*__ctype_b_loc())
        .offset(
            *((*o).ptr as *mut libc::c_char).offset(0 as libc::c_int as isize)
                as libc::c_int as isize,
        ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        || *eptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        || *__errno_location() == 34 as libc::c_int
    {
        addReplyError(c, b"invalid cursor\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scanGenericCommand(
    mut c: *mut client,
    mut o: *mut robj,
    mut cursor: libc::c_ulong,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut keys: *mut list = listCreate();
    let mut node: *mut listNode = 0 as *mut listNode;
    let mut nextnode: *mut listNode = 0 as *mut listNode;
    let mut count: libc::c_long = 10 as libc::c_int as libc::c_long;
    let mut pat: sds = 0 as sds;
    let mut typename: sds = 0 as sds;
    let mut patlen: libc::c_int = 0 as libc::c_int;
    let mut use_pattern: libc::c_int = 0 as libc::c_int;
    let mut ht: *mut dict = 0 as *mut dict;
    if o.is_null() || (*o).type_0() as libc::c_int == 2 as libc::c_int
        || (*o).type_0() as libc::c_int == 4 as libc::c_int
        || (*o).type_0() as libc::c_int == 3 as libc::c_int
    {} else {
        _serverAssert(
            b"o == NULL || o->type == OBJ_SET || o->type == OBJ_HASH || o->type == OBJ_ZSET\0"
                as *const u8 as *const libc::c_char,
            b"db.c\0" as *const u8 as *const libc::c_char,
            841 as libc::c_int,
        );
        unreachable!();
    };
    i = if o.is_null() { 2 as libc::c_int } else { 3 as libc::c_int };
    loop {
        if !(i < (*c).argc) {
            current_block = 26972500619410423;
            break;
        }
        j = (*c).argc - i;
        if strcasecmp(
            (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
            b"count\0" as *const u8 as *const libc::c_char,
        ) == 0 && j >= 2 as libc::c_int
        {
            if getLongFromObjectOrReply(
                c,
                *((*c).argv).offset((i + 1 as libc::c_int) as isize),
                &mut count,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                current_block = 12433311124224288381;
                break;
            }
            if count < 1 as libc::c_int as libc::c_long {
                addReplyErrorObject(c, shared.syntaxerr);
                current_block = 12433311124224288381;
                break;
            } else {
                i += 2 as libc::c_int;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
            b"match\0" as *const u8 as *const libc::c_char,
        ) == 0 && j >= 2 as libc::c_int
        {
            pat = (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr as sds;
            patlen = sdslen(pat) as libc::c_int;
            use_pattern = !(patlen == 1 as libc::c_int
                && *pat.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32)
                as libc::c_int;
            i += 2 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
            b"type\0" as *const u8 as *const libc::c_char,
        ) == 0 && o.is_null() && j >= 2 as libc::c_int
        {
            typename = (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                as sds;
            i += 2 as libc::c_int;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            current_block = 12433311124224288381;
            break;
        }
    }
    match current_block {
        26972500619410423 => {
            ht = 0 as *mut dict;
            if o.is_null() {
                ht = (*(*c).db).dict;
            } else if (*o).type_0() as libc::c_int == 2 as libc::c_int
                && (*o).encoding() as libc::c_int == 2 as libc::c_int
            {
                ht = (*o).ptr as *mut dict;
            } else if (*o).type_0() as libc::c_int == 4 as libc::c_int
                && (*o).encoding() as libc::c_int == 2 as libc::c_int
            {
                ht = (*o).ptr as *mut dict;
                count *= 2 as libc::c_int as libc::c_long;
            } else if (*o).type_0() as libc::c_int == 3 as libc::c_int
                && (*o).encoding() as libc::c_int == 7 as libc::c_int
            {
                let mut zs: *mut zset = (*o).ptr as *mut zset;
                ht = (*zs).dict;
                count *= 2 as libc::c_int as libc::c_long;
            }
            if !ht.is_null() {
                let mut privdata: [*mut libc::c_void; 2] = [0 as *mut libc::c_void; 2];
                let mut maxiterations: libc::c_long = count
                    * 10 as libc::c_int as libc::c_long;
                privdata[0 as libc::c_int as usize] = keys as *mut libc::c_void;
                privdata[1 as libc::c_int as usize] = o as *mut libc::c_void;
                loop {
                    cursor = dictScan(
                        ht,
                        cursor,
                        Some(
                            scanCallback
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *const dictEntry,
                                ) -> (),
                        ),
                        None,
                        privdata.as_mut_ptr() as *mut libc::c_void,
                    );
                    if !(cursor != 0
                        && {
                            let fresh3 = maxiterations;
                            maxiterations = maxiterations - 1;
                            fresh3 != 0
                        } && (*keys).len < count as libc::c_ulong)
                    {
                        break;
                    }
                }
            } else if (*o).type_0() as libc::c_int == 2 as libc::c_int {
                let mut pos: libc::c_int = 0 as libc::c_int;
                let mut ll: int64_t = 0;
                loop {
                    let fresh4 = pos;
                    pos = pos + 1;
                    if !(intsetGet((*o).ptr as *mut intset, fresh4 as uint32_t, &mut ll)
                        != 0)
                    {
                        break;
                    }
                    listAddNodeTail(
                        keys,
                        createStringObjectFromLongLong(ll as libc::c_longlong)
                            as *mut libc::c_void,
                    );
                }
                cursor = 0 as libc::c_int as libc::c_ulong;
            } else if (*o).type_0() as libc::c_int == 4 as libc::c_int
                || (*o).type_0() as libc::c_int == 3 as libc::c_int
            {
                let mut p: *mut libc::c_uchar = lpFirst((*o).ptr as *mut libc::c_uchar);
                let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut vlen: int64_t = 0;
                let mut intbuf: [libc::c_uchar; 21] = [0; 21];
                while !p.is_null() {
                    vstr = lpGet(p, &mut vlen, intbuf.as_mut_ptr());
                    listAddNodeTail(
                        keys,
                        createStringObject(vstr as *mut libc::c_char, vlen as size_t)
                            as *mut libc::c_void,
                    );
                    p = lpNext((*o).ptr as *mut libc::c_uchar, p);
                }
                cursor = 0 as libc::c_int as libc::c_ulong;
            } else {
                _serverPanic(
                    b"db.c\0" as *const u8 as *const libc::c_char,
                    942 as libc::c_int,
                    b"Not handled encoding in SCAN.\0" as *const u8
                        as *const libc::c_char,
                );
                unreachable!();
            }
            node = (*keys).head;
            while !node.is_null() {
                let mut kobj: *mut robj = (*node).value as *mut robj;
                nextnode = (*node).next;
                let mut filter: libc::c_int = 0 as libc::c_int;
                if use_pattern != 0 {
                    if (*kobj).encoding() as libc::c_int == 0 as libc::c_int
                        || (*kobj).encoding() as libc::c_int == 8 as libc::c_int
                    {
                        if stringmatchlen(
                            pat as *const libc::c_char,
                            patlen,
                            (*kobj).ptr as *const libc::c_char,
                            sdslen((*kobj).ptr as sds) as libc::c_int,
                            0 as libc::c_int,
                        ) == 0
                        {
                            filter = 1 as libc::c_int;
                        }
                    } else {
                        let mut buf: [libc::c_char; 21] = [0; 21];
                        let mut len: libc::c_int = 0;
                        if (*kobj).encoding() as libc::c_int == 1 as libc::c_int
                        {} else {
                            _serverAssert(
                                b"kobj->encoding == OBJ_ENCODING_INT\0" as *const u8
                                    as *const libc::c_char,
                                b"db.c\0" as *const u8 as *const libc::c_char,
                                961 as libc::c_int,
                            );
                            unreachable!();
                        };
                        len = ll2string(
                            buf.as_mut_ptr(),
                            core::mem::size_of::<[libc::c_char; 21]>()
                                as libc::c_ulong,
                            (*kobj).ptr as libc::c_long as libc::c_longlong,
                        );
                        if stringmatchlen(
                            pat as *const libc::c_char,
                            patlen,
                            buf.as_mut_ptr(),
                            len,
                            0 as libc::c_int,
                        ) == 0
                        {
                            filter = 1 as libc::c_int;
                        }
                    }
                }
                if filter == 0 && o.is_null() && !typename.is_null() {
                    let mut typecheck: *mut robj = lookupKeyReadWithFlags(
                        (*c).db,
                        kobj,
                        (1 as libc::c_int) << 0 as libc::c_int,
                    );
                    let mut type_0: *mut libc::c_char = getObjectTypeName(typecheck);
                    if strcasecmp(typename, type_0) != 0 {
                        filter = 1 as libc::c_int;
                    }
                }
                if filter == 0 && o.is_null()
                    && expireIfNeeded((*c).db, kobj, 0 as libc::c_int) != 0
                {
                    filter = 1 as libc::c_int;
                }
                if filter != 0 {
                    decrRefCount(kobj);
                    listDelNode(keys, node);
                }
                if !o.is_null()
                    && ((*o).type_0() as libc::c_int == 3 as libc::c_int
                        || (*o).type_0() as libc::c_int == 4 as libc::c_int)
                {
                    node = nextnode;
                    if !node.is_null() {} else {
                        _serverAssert(
                            b"node\0" as *const u8 as *const libc::c_char,
                            b"db.c\0" as *const u8 as *const libc::c_char,
                            988 as libc::c_int,
                        );
                        unreachable!();
                    };
                    nextnode = (*node).next;
                    if filter != 0 {
                        kobj = (*node).value as *mut robj;
                        decrRefCount(kobj);
                        listDelNode(keys, node);
                    }
                }
                node = nextnode;
            }
            addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
            addReplyBulkLongLong(c, cursor as libc::c_longlong);
            addReplyArrayLen(c, (*keys).len as libc::c_long);
            loop {
                node = (*keys).head;
                if node.is_null() {
                    break;
                }
                let mut kobj_0: *mut robj = (*node).value as *mut robj;
                addReplyBulk(c, kobj_0);
                decrRefCount(kobj_0);
                listDelNode(keys, node);
            }
        }
        _ => {}
    }
    (*keys)
        .free = Some(decrRefCountVoid as unsafe extern "C" fn(*mut libc::c_void) -> ());
    listRelease(keys);
}
#[no_mangle]
pub unsafe extern "C" fn scanCommand(mut c: *mut client) {
    let mut cursor: libc::c_ulong = 0;
    if parseScanCursorOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        &mut cursor,
    ) == -(1 as libc::c_int)
    {
        return;
    }
    scanGenericCommand(c, 0 as *mut robj, cursor);
}
#[no_mangle]
pub unsafe extern "C" fn dbsizeCommand(mut c: *mut client) {
    addReplyLongLong(
        c,
        ((*(*(*c).db).dict).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*(*(*c).db).dict).ht_used[1 as libc::c_int as usize])
            as libc::c_longlong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lastsaveCommand(mut c: *mut client) {
    addReplyLongLong(c, server.lastsave as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn getObjectTypeName(mut o: *mut robj) -> *mut libc::c_char {
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if o.is_null() {
        type_0 = b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        match (*o).type_0() as libc::c_int {
            0 => {
                type_0 = b"string\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            1 => {
                type_0 = b"list\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            2 => {
                type_0 = b"set\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            3 => {
                type_0 = b"zset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            4 => {
                type_0 = b"hash\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            6 => {
                type_0 = b"stream\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            5 => {
                let mut mv: *mut moduleValue = (*o).ptr as *mut moduleValue;
                type_0 = ((*(*mv).type_0).name).as_mut_ptr();
            }
            _ => {
                type_0 = b"unknown\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
        }
    }
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn typeCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadWithFlags(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (1 as libc::c_int) << 0 as libc::c_int,
    );
    addReplyStatus(c, getObjectTypeName(o));
}
#[no_mangle]
pub unsafe extern "C" fn shutdownCommand(mut c: *mut client) {
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut abort: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < (*c).argc {
        if strcasecmp(
            (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
            b"nosave\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            flags |= 2 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
            b"save\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            flags |= 1 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
            b"now\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            flags |= 4 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
            b"force\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            flags |= 8 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
            b"abort\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            abort = 1 as libc::c_int;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        i += 1;
    }
    if abort != 0 && flags != 0 as libc::c_int
        || flags & 2 as libc::c_int != 0 && flags & 1 as libc::c_int != 0
    {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    if abort != 0 {
        if abortShutdown() == 0 as libc::c_int {
            addReply(c, shared.ok);
        } else {
            addReplyError(
                c,
                b"No shutdown in progress.\0" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    if flags & 4 as libc::c_int == 0
        && (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 41 as libc::c_int != 0
    {
        addReplyError(
            c,
            b"SHUTDOWN without NOW or ABORT isn't allowed for DENY BLOCKING client\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if flags & 2 as libc::c_int == 0 && isInsideYieldingLongCommand() != 0 {
        if server.busy_module_yield_flags != 0
            && !(server.busy_module_yield_reply).is_null()
        {
            addReplyErrorFormat(
                c,
                b"-BUSY %s\0" as *const u8 as *const libc::c_char,
                server.busy_module_yield_reply,
            );
        } else if server.busy_module_yield_flags != 0 {
            addReplyErrorObject(c, shared.slowmoduleerr);
        } else if scriptIsEval() != 0 {
            addReplyErrorObject(c, shared.slowevalerr);
        } else {
            addReplyErrorObject(c, shared.slowscripterr);
        }
        return;
    }
    blockClient(c, 7 as libc::c_int);
    if prepareForShutdown(flags) == 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn renameGenericCommand(mut c: *mut client, mut nx: libc::c_int) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut expire: libc::c_longlong = 0;
    let mut samekey: libc::c_int = 0 as libc::c_int;
    if sdscmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as sds,
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
    ) == 0 as libc::c_int
    {
        samekey = 1 as libc::c_int;
    }
    o = lookupKeyWriteOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.nokeyerr,
    );
    if o.is_null() {
        return;
    }
    if samekey != 0 {
        addReply(c, if nx != 0 { shared.czero } else { shared.ok });
        return;
    }
    incrRefCount(o);
    expire = getExpire((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if !(lookupKeyWrite((*c).db, *((*c).argv).offset(2 as libc::c_int as isize)))
        .is_null()
    {
        if nx != 0 {
            decrRefCount(o);
            addReply(c, shared.czero);
            return;
        }
        dbDelete((*c).db, *((*c).argv).offset(2 as libc::c_int as isize));
    }
    dbAdd((*c).db, *((*c).argv).offset(2 as libc::c_int as isize), o);
    if expire != -(1 as libc::c_int) as libc::c_longlong {
        setExpire(c, (*c).db, *((*c).argv).offset(2 as libc::c_int as isize), expire);
    }
    dbDelete((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(2 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 2 as libc::c_int,
        b"rename_from\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 2 as libc::c_int,
        b"rename_to\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(2 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
    addReply(c, if nx != 0 { shared.cone } else { shared.ok });
}
#[no_mangle]
pub unsafe extern "C" fn renameCommand(mut c: *mut client) {
    renameGenericCommand(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn renamenxCommand(mut c: *mut client) {
    renameGenericCommand(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn moveCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut src: *mut redisDb = 0 as *mut redisDb;
    let mut dst: *mut redisDb = 0 as *mut redisDb;
    let mut srcid: libc::c_int = 0;
    let mut dbid: libc::c_int = 0;
    let mut expire: libc::c_longlong = 0;
    if server.cluster_enabled != 0 {
        addReplyError(
            c,
            b"MOVE is not allowed in cluster mode\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    src = (*c).db;
    srcid = (*(*c).db).id;
    if getIntFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut dbid,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if selectDb(c, dbid) == -(1 as libc::c_int) {
        addReplyError(
            c,
            b"DB index is out of range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    dst = (*c).db;
    selectDb(c, srcid);
    if src == dst {
        addReplyErrorObject(c, shared.sameobjecterr);
        return;
    }
    o = lookupKeyWrite((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        addReply(c, shared.czero);
        return;
    }
    expire = getExpire((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if !(lookupKeyWrite(dst, *((*c).argv).offset(1 as libc::c_int as isize))).is_null() {
        addReply(c, shared.czero);
        return;
    }
    dbAdd(dst, *((*c).argv).offset(1 as libc::c_int as isize), o);
    if expire != -(1 as libc::c_int) as libc::c_longlong {
        setExpire(c, dst, *((*c).argv).offset(1 as libc::c_int as isize), expire);
    }
    incrRefCount(o);
    dbDelete(src, *((*c).argv).offset(1 as libc::c_int as isize));
    signalModifiedKey(c, src, *((*c).argv).offset(1 as libc::c_int as isize));
    signalModifiedKey(c, dst, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 2 as libc::c_int,
        b"move_from\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*src).id,
    );
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 2 as libc::c_int,
        b"move_to\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*dst).id,
    );
    server.dirty += 1;
    addReply(c, shared.cone);
}
#[no_mangle]
pub unsafe extern "C" fn copyCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut src: *mut redisDb = 0 as *mut redisDb;
    let mut dst: *mut redisDb = 0 as *mut redisDb;
    let mut srcid: libc::c_int = 0;
    let mut dbid: libc::c_int = 0;
    let mut expire: libc::c_longlong = 0;
    let mut j: libc::c_int = 0;
    let mut replace: libc::c_int = 0 as libc::c_int;
    let mut delete: libc::c_int = 0 as libc::c_int;
    src = (*c).db;
    dst = (*c).db;
    srcid = (*(*c).db).id;
    dbid = (*(*c).db).id;
    j = 3 as libc::c_int;
    while j < (*c).argc {
        let mut additional: libc::c_int = (*c).argc - j - 1 as libc::c_int;
        if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"replace\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            replace = 1 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"db\0" as *const u8 as *const libc::c_char,
        ) == 0 && additional >= 1 as libc::c_int
        {
            if getIntFromObjectOrReply(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                &mut dbid,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            if selectDb(c, dbid) == -(1 as libc::c_int) {
                addReplyError(
                    c,
                    b"DB index is out of range\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            dst = (*c).db;
            selectDb(c, srcid);
            j += 1;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        j += 1;
    }
    if server.cluster_enabled == 1 as libc::c_int
        && (srcid != 0 as libc::c_int || dbid != 0 as libc::c_int)
    {
        addReplyError(
            c,
            b"Copying to another database is not allowed in cluster mode\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut newkey: *mut robj = *((*c).argv).offset(2 as libc::c_int as isize);
    if src == dst && sdscmp((*key).ptr as sds, (*newkey).ptr as sds) == 0 as libc::c_int
    {
        addReplyErrorObject(c, shared.sameobjecterr);
        return;
    }
    o = lookupKeyRead((*c).db, key);
    if o.is_null() {
        addReply(c, shared.czero);
        return;
    }
    expire = getExpire((*c).db, key);
    if !(lookupKeyWrite(dst, newkey)).is_null() {
        if replace != 0 {
            delete = 1 as libc::c_int;
        } else {
            addReply(c, shared.czero);
            return;
        }
    }
    let mut newobj: *mut robj = 0 as *mut robj;
    match (*o).type_0() as libc::c_int {
        0 => {
            newobj = dupStringObject(o);
        }
        1 => {
            newobj = listTypeDup(o);
        }
        2 => {
            newobj = setTypeDup(o);
        }
        3 => {
            newobj = zsetDup(o);
        }
        4 => {
            newobj = hashTypeDup(o);
        }
        6 => {
            newobj = streamDup(o);
        }
        5 => {
            newobj = moduleTypeDupOrReply(c, key, newkey, (*dst).id, o);
            if newobj.is_null() {
                return;
            }
        }
        _ => {
            addReplyError(
                c,
                b"unknown type object\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    if delete != 0 {
        dbDelete(dst, newkey);
    }
    dbAdd(dst, newkey, newobj);
    if expire != -(1 as libc::c_int) as libc::c_longlong {
        setExpire(c, dst, newkey, expire);
    }
    signalModifiedKey(c, dst, *((*c).argv).offset(2 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 2 as libc::c_int,
        b"copy_to\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(2 as libc::c_int as isize),
        (*dst).id,
    );
    server.dirty += 1;
    addReply(c, shared.cone);
}
#[no_mangle]
pub unsafe extern "C" fn scanDatabaseForReadyKeys(mut db: *mut redisDb) {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = dictGetSafeIterator((*db).blocking_keys);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut key: *mut robj = (*de).key as *mut robj;
        let mut kde: *mut dictEntry = dictFind((*db).dict, (*key).ptr);
        if !kde.is_null() {
            let mut value: *mut robj = (*kde).v.val as *mut robj;
            signalKeyAsReady(db, key, (*value).type_0() as libc::c_int);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn scanDatabaseForDeletedStreams(
    mut emptied: *mut redisDb,
    mut replaced_with: *mut redisDb,
) {
    if server.blocked_clients_by_type[4 as libc::c_int as usize] == 0 {
        return;
    }
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = dictGetSafeIterator((*emptied).blocking_keys);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut key: *mut robj = (*de).key as *mut robj;
        let mut was_stream: libc::c_int = 0 as libc::c_int;
        let mut is_stream: libc::c_int = 0 as libc::c_int;
        let mut kde: *mut dictEntry = dictFind((*emptied).dict, (*key).ptr);
        if !kde.is_null() {
            let mut value: *mut robj = (*kde).v.val as *mut robj;
            was_stream = ((*value).type_0() as libc::c_int == 6 as libc::c_int)
                as libc::c_int;
        }
        if !replaced_with.is_null() {
            let mut kde_0: *mut dictEntry = dictFind((*replaced_with).dict, (*key).ptr);
            if !kde_0.is_null() {
                let mut value_0: *mut robj = (*kde_0).v.val as *mut robj;
                is_stream = ((*value_0).type_0() as libc::c_int == 6 as libc::c_int)
                    as libc::c_int;
            }
        }
        if was_stream != 0 && is_stream == 0 {
            signalKeyAsReady(emptied, key, 6 as libc::c_int);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn dbSwapDatabases(
    mut id1: libc::c_int,
    mut id2: libc::c_int,
) -> libc::c_int {
    if id1 < 0 as libc::c_int || id1 >= server.dbnum || id2 < 0 as libc::c_int
        || id2 >= server.dbnum
    {
        return -(1 as libc::c_int);
    }
    if id1 == id2 {
        return 0 as libc::c_int;
    }
    let mut aux: redisDb = *(server.db).offset(id1 as isize);
    let mut db1: *mut redisDb = &mut *(server.db).offset(id1 as isize) as *mut redisDb;
    let mut db2: *mut redisDb = &mut *(server.db).offset(id2 as isize) as *mut redisDb;
    touchAllWatchedKeysInDb(db1, db2);
    touchAllWatchedKeysInDb(db2, db1);
    scanDatabaseForDeletedStreams(db1, db2);
    scanDatabaseForDeletedStreams(db2, db1);
    (*db1).dict = (*db2).dict;
    (*db1).expires = (*db2).expires;
    (*db1).avg_ttl = (*db2).avg_ttl;
    (*db1).expires_cursor = (*db2).expires_cursor;
    (*db2).dict = aux.dict;
    (*db2).expires = aux.expires;
    (*db2).avg_ttl = aux.avg_ttl;
    (*db2).expires_cursor = aux.expires_cursor;
    scanDatabaseForReadyKeys(db1);
    scanDatabaseForReadyKeys(db2);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn swapMainDbWithTempDb(mut tempDb: *mut redisDb) {
    if server.cluster_enabled != 0 {
        let mut aux: *mut clusterSlotToKeyMapping = (*server.db).slots_to_keys;
        (*server.db).slots_to_keys = (*tempDb).slots_to_keys;
        (*tempDb).slots_to_keys = aux;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < server.dbnum {
        let mut aux_0: redisDb = *(server.db).offset(i as isize);
        let mut activedb: *mut redisDb = &mut *(server.db).offset(i as isize)
            as *mut redisDb;
        let mut newdb: *mut redisDb = &mut *tempDb.offset(i as isize) as *mut redisDb;
        touchAllWatchedKeysInDb(activedb, newdb);
        scanDatabaseForDeletedStreams(activedb, newdb);
        (*activedb).dict = (*newdb).dict;
        (*activedb).expires = (*newdb).expires;
        (*activedb).avg_ttl = (*newdb).avg_ttl;
        (*activedb).expires_cursor = (*newdb).expires_cursor;
        (*newdb).dict = aux_0.dict;
        (*newdb).expires = aux_0.expires;
        (*newdb).avg_ttl = aux_0.avg_ttl;
        (*newdb).expires_cursor = aux_0.expires_cursor;
        scanDatabaseForReadyKeys(activedb);
        i += 1;
    }
    trackingInvalidateKeysOnFlush(1 as libc::c_int);
    flushSlaveKeysWithExpireList();
}
#[no_mangle]
pub unsafe extern "C" fn swapdbCommand(mut c: *mut client) {
    let mut id1: libc::c_int = 0;
    let mut id2: libc::c_int = 0;
    if server.cluster_enabled != 0 {
        addReplyError(
            c,
            b"SWAPDB is not allowed in cluster mode\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if getIntFromObjectOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        &mut id1,
        b"invalid first DB index\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if getIntFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut id2,
        b"invalid second DB index\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if dbSwapDatabases(id1, id2) == -(1 as libc::c_int) {
        addReplyError(
            c,
            b"DB index is out of range\0" as *const u8 as *const libc::c_char,
        );
        return;
    } else {
        let mut si: RedisModuleSwapDbInfoV1 = {
            let mut init = RedisModuleSwapDbInfo {
                version: 1 as libc::c_int as uint64_t,
                dbnum_first: id1,
                dbnum_second: id2,
            };
            init
        };
        moduleFireServerEvent(
            11 as libc::c_int as uint64_t,
            0 as libc::c_int,
            &mut si as *mut RedisModuleSwapDbInfoV1 as *mut libc::c_void,
        );
        server.dirty += 1;
        addReply(c, shared.ok);
    };
}
#[no_mangle]
pub unsafe extern "C" fn removeExpire(
    mut db: *mut redisDb,
    mut key: *mut robj,
) -> libc::c_int {
    if !(dictFind((*db).dict, (*key).ptr)).is_null() {} else {
        _serverAssertWithInfo(
            0 as *const client,
            key,
            b"dictFind(db->dict,key->ptr) != NULL\0" as *const u8 as *const libc::c_char,
            b"db.c\0" as *const u8 as *const libc::c_char,
            1525 as libc::c_int,
        );
        unreachable!();
    };
    return (dictDelete((*db).expires, (*key).ptr) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setExpire(
    mut c: *mut client,
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut when: libc::c_longlong,
) {
    let mut kde: *mut dictEntry = 0 as *mut dictEntry;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    kde = dictFind((*db).dict, (*key).ptr);
    if !kde.is_null() {} else {
        _serverAssertWithInfo(
            0 as *const client,
            key,
            b"kde != NULL\0" as *const u8 as *const libc::c_char,
            b"db.c\0" as *const u8 as *const libc::c_char,
            1538 as libc::c_int,
        );
        unreachable!();
    };
    de = dictAddOrFind((*db).expires, (*kde).key);
    (*de).v.s64 = when as int64_t;
    let mut writable_slave: libc::c_int = (!(server.masterhost).is_null()
        && server.repl_slave_ro == 0 as libc::c_int) as libc::c_int;
    if !c.is_null() && writable_slave != 0
        && (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong == 0
    {
        rememberSlaveKeyWithExpire(db, key);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getExpire(
    mut db: *mut redisDb,
    mut key: *mut robj,
) -> libc::c_longlong {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    if ((*(*db).expires).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*db).expires).ht_used[1 as libc::c_int as usize])
        == 0 as libc::c_int as libc::c_ulong
        || {
            de = dictFind((*db).expires, (*key).ptr);
            de.is_null()
        }
    {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    if !(dictFind((*db).dict, (*key).ptr)).is_null() {} else {
        _serverAssertWithInfo(
            0 as *const client,
            key,
            b"dictFind(db->dict,key->ptr) != NULL\0" as *const u8 as *const libc::c_char,
            b"db.c\0" as *const u8 as *const libc::c_char,
            1558 as libc::c_int,
        );
        unreachable!();
    };
    return (*de).v.s64 as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn deleteExpiredKeyAndPropagate(
    mut db: *mut redisDb,
    mut keyobj: *mut robj,
) {
    let mut expire_latency: mstime_t = 0;
    if server.latency_monitor_threshold != 0 {
        expire_latency = mstime();
    } else {
        expire_latency = 0 as libc::c_int as mstime_t;
    }
    if server.lazyfree_lazy_expire != 0 {
        dbAsyncDelete(db, keyobj);
    } else {
        dbSyncDelete(db, keyobj);
    }
    if server.latency_monitor_threshold != 0 {
        expire_latency = mstime() - expire_latency;
    }
    if server.latency_monitor_threshold != 0
        && expire_latency >= server.latency_monitor_threshold
    {
        latencyAddSample(
            b"expire-del\0" as *const u8 as *const libc::c_char,
            expire_latency,
        );
    }
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 8 as libc::c_int,
        b"expired\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        keyobj,
        (*db).id,
    );
    signalModifiedKey(0 as *mut client, db, keyobj);
    propagateDeletion(db, keyobj, server.lazyfree_lazy_expire);
    server.stat_expiredkeys += 1;
}
#[no_mangle]
pub unsafe extern "C" fn propagateDeletion(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut lazy: libc::c_int,
) {
    let mut argv: [*mut robj; 2] = [0 as *mut robj; 2];
    argv[0 as libc::c_int as usize] = if lazy != 0 { shared.unlink } else { shared.del };
    argv[1 as libc::c_int as usize] = key;
    incrRefCount(argv[0 as libc::c_int as usize]);
    incrRefCount(argv[1 as libc::c_int as usize]);
    let mut prev_replication_allowed: libc::c_int = server.replication_allowed;
    server.replication_allowed = 1 as libc::c_int;
    alsoPropagate(
        (*db).id,
        argv.as_mut_ptr(),
        2 as libc::c_int,
        1 as libc::c_int | 2 as libc::c_int,
    );
    server.replication_allowed = prev_replication_allowed;
    decrRefCount(argv[0 as libc::c_int as usize]);
    decrRefCount(argv[1 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn keyIsExpired(
    mut db: *mut redisDb,
    mut key: *mut robj,
) -> libc::c_int {
    let mut when: mstime_t = getExpire(db, key);
    let mut now: mstime_t = 0;
    if when < 0 as libc::c_int as libc::c_longlong {
        return 0 as libc::c_int;
    }
    if server.loading != 0 {
        return 0 as libc::c_int;
    }
    if !(server.script_caller).is_null() {
        now = scriptTimeSnapshot();
    } else if server.fixed_time_expire > 0 as libc::c_int as libc::c_long {
        now = server.mstime;
    } else {
        now = mstime();
    }
    return (now > when) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn expireIfNeeded(
    mut db: *mut redisDb,
    mut key: *mut robj,
    mut flags: libc::c_int,
) -> libc::c_int {
    if keyIsExpired(db, key) == 0 {
        return 0 as libc::c_int;
    }
    if !(server.masterhost).is_null() {
        if server.current_client == server.master {
            return 0 as libc::c_int;
        }
        if flags & 1 as libc::c_int == 0 {
            return 1 as libc::c_int;
        }
    }
    if flags & 2 as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    if checkClientPauseTimeoutAndReturnIfPaused() != 0 {
        return 1 as libc::c_int;
    }
    deleteExpiredKeyAndPropagate(db, key);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getKeysPrepareResult(
    mut result: *mut getKeysResult,
    mut numkeys: libc::c_int,
) -> *mut keyReference {
    if ((*result).keys).is_null() {
        if (*result).numkeys == 0 {} else {
            _serverAssert(
                b"!result->numkeys\0" as *const u8 as *const libc::c_char,
                b"db.c\0" as *const u8 as *const libc::c_char,
                1733 as libc::c_int,
            );
            unreachable!();
        };
        (*result).keys = ((*result).keysbuf).as_mut_ptr();
    }
    if numkeys > (*result).size {
        if (*result).keys != ((*result).keysbuf).as_mut_ptr() {
            (*result)
                .keys = zrealloc(
                (*result).keys as *mut libc::c_void,
                (numkeys as libc::c_ulong)
                    .wrapping_mul(
                        core::mem::size_of::<keyReference>() as libc::c_ulong,
                    ),
            ) as *mut keyReference;
        } else {
            (*result)
                .keys = zmalloc(
                (numkeys as libc::c_ulong)
                    .wrapping_mul(
                        core::mem::size_of::<keyReference>() as libc::c_ulong,
                    ),
            ) as *mut keyReference;
            if (*result).numkeys != 0 {
                memcpy(
                    (*result).keys as *mut libc::c_void,
                    ((*result).keysbuf).as_mut_ptr() as *const libc::c_void,
                    ((*result).numkeys as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<keyReference>() as libc::c_ulong,
                        ),
                );
            }
        }
        (*result).size = numkeys;
    }
    return (*result).keys;
}
#[no_mangle]
pub unsafe extern "C" fn getAllKeySpecsFlags(
    mut cmd: *mut redisCommand,
    mut inv: libc::c_int,
) -> int64_t {
    let mut flags: int64_t = 0 as libc::c_int as int64_t;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < (*cmd).key_specs_num {
        let mut spec: *mut keySpec = ((*cmd).key_specs).offset(j as isize);
        flags = (flags as libc::c_ulong
            | if inv != 0 { !(*spec).flags } else { (*spec).flags }) as int64_t;
        j += 1;
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn getKeysUsingKeySpecs(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut search_flags: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut current_block: u64;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut last: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    j = 0 as libc::c_int;
    while j < (*cmd).key_specs_num {
        let mut spec: *mut keySpec = ((*cmd).key_specs).offset(j as isize);
        if (*spec).begin_search_type as libc::c_uint
            != KSPEC_BS_INVALID as libc::c_int as libc::c_uint
        {} else {
            _serverAssert(
                b"spec->begin_search_type != KSPEC_BS_INVALID\0" as *const u8
                    as *const libc::c_char,
                b"db.c\0" as *const u8 as *const libc::c_char,
                1778 as libc::c_int,
            );
            unreachable!();
        };
        if !((*spec).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
            && search_flags & (1 as libc::c_int) << 0 as libc::c_int == 0)
        {
            first = 0 as libc::c_int;
            if (*spec).begin_search_type as libc::c_uint
                == KSPEC_BS_INDEX as libc::c_int as libc::c_uint
            {
                first = (*spec).bs.index.pos;
                current_block = 6009453772311597924;
            } else if (*spec).begin_search_type as libc::c_uint
                == KSPEC_BS_KEYWORD as libc::c_int as libc::c_uint
            {
                let mut start_index: libc::c_int = if (*spec).bs.keyword.startfrom
                    > 0 as libc::c_int
                {
                    (*spec).bs.keyword.startfrom
                } else {
                    argc + (*spec).bs.keyword.startfrom
                };
                let mut end_index: libc::c_int = if (*spec).bs.keyword.startfrom
                    > 0 as libc::c_int
                {
                    argc - 1 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                i = start_index;
                while i != end_index {
                    if i >= argc || i < 1 as libc::c_int {
                        break;
                    }
                    if strcasecmp(
                        (**argv.offset(i as isize)).ptr as *mut libc::c_char,
                        (*spec).bs.keyword.keyword,
                    ) == 0
                    {
                        first = i + 1 as libc::c_int;
                        break;
                    } else {
                        i = if start_index <= end_index {
                            i + 1 as libc::c_int
                        } else {
                            i - 1 as libc::c_int
                        };
                    }
                }
                if first == 0 {
                    current_block = 16668937799742929182;
                } else {
                    current_block = 6009453772311597924;
                }
            } else {
                current_block = 6370670150452288782;
            }
            match current_block {
                16668937799742929182 => {}
                _ => {
                    match current_block {
                        6009453772311597924 => {
                            if (*spec).find_keys_type as libc::c_uint
                                == KSPEC_FK_RANGE as libc::c_int as libc::c_uint
                            {
                                step = (*spec).fk.range.keystep;
                                if (*spec).fk.range.lastkey >= 0 as libc::c_int {
                                    last = first + (*spec).fk.range.lastkey;
                                } else if (*spec).fk.range.limit == 0 {
                                    last = argc + (*spec).fk.range.lastkey;
                                } else {
                                    if (*spec).fk.range.lastkey == -(1 as libc::c_int) {} else {
                                        _serverAssert(
                                            b"spec->fk.range.lastkey == -1\0" as *const u8
                                                as *const libc::c_char,
                                            b"db.c\0" as *const u8 as *const libc::c_char,
                                            1815 as libc::c_int,
                                        );
                                        unreachable!();
                                    };
                                    last = first
                                        + ((argc - first) / (*spec).fk.range.limit
                                            + (*spec).fk.range.lastkey);
                                }
                                current_block = 7828949454673616476;
                            } else if (*spec).find_keys_type as libc::c_uint
                                == KSPEC_FK_KEYNUM as libc::c_int as libc::c_uint
                            {
                                step = (*spec).fk.keynum.keystep;
                                let mut numkeys: libc::c_longlong = 0;
                                if (*spec).fk.keynum.keynumidx >= argc {
                                    current_block = 6370670150452288782;
                                } else {
                                    let mut keynum_str: sds = (**argv
                                        .offset((first + (*spec).fk.keynum.keynumidx) as isize))
                                        .ptr as sds;
                                    if string2ll(
                                        keynum_str as *const libc::c_char,
                                        sdslen(keynum_str),
                                        &mut numkeys,
                                    ) == 0 || numkeys < 0 as libc::c_int as libc::c_longlong
                                    {
                                        current_block = 6370670150452288782;
                                    } else {
                                        first += (*spec).fk.keynum.firstkey;
                                        last = first + numkeys as libc::c_int - 1 as libc::c_int;
                                        current_block = 7828949454673616476;
                                    }
                                }
                            } else {
                                current_block = 6370670150452288782;
                            }
                            match current_block {
                                6370670150452288782 => {}
                                _ => {
                                    count = last - first + 1 as libc::c_int;
                                    keys = getKeysPrepareResult(result, count);
                                    if last >= argc || last < first || first >= argc {
                                        current_block = 6370670150452288782;
                                    } else {
                                        let mut current_block_28: u64;
                                        i = first;
                                        while i <= last {
                                            if i >= argc || i < first {
                                                if (*cmd).flags as libc::c_ulonglong
                                                    & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0
                                                    || (*cmd).arity < 0 as libc::c_int
                                                {
                                                    current_block_28 = 7746103178988627676;
                                                } else {
                                                    _serverPanic(
                                                        b"db.c\0" as *const u8 as *const libc::c_char,
                                                        1857 as libc::c_int,
                                                        b"Redis built-in command declared keys positions not matching the arity requirements.\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                    unreachable!();
                                                    current_block_28 = 3938820862080741272;
                                                }
                                            } else {
                                                current_block_28 = 3938820862080741272;
                                            }
                                            match current_block_28 {
                                                3938820862080741272 => {
                                                    (*keys.offset(k as isize)).pos = i;
                                                    let fresh5 = k;
                                                    k = k + 1;
                                                    (*keys.offset(fresh5 as isize))
                                                        .flags = (*spec).flags as libc::c_int;
                                                }
                                                _ => {}
                                            }
                                            i += step;
                                        }
                                        if (*spec).flags as libc::c_ulonglong
                                            & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0
                                        {
                                            current_block = 6370670150452288782;
                                        } else {
                                            current_block = 16668937799742929182;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        16668937799742929182 => {}
                        _ => {
                            if !(search_flags & (1 as libc::c_int) << 1 as libc::c_int
                                != 0)
                            {
                                (*result).numkeys = 0 as libc::c_int;
                                return -(1 as libc::c_int);
                            }
                        }
                    }
                }
            }
        }
        j += 1;
    }
    (*result).numkeys = k;
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn getKeysFromCommandWithSpecs(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut search_flags: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut has_keyspec: libc::c_int = (getAllKeySpecsFlags(cmd, 1 as libc::c_int)
        as libc::c_ulonglong & (1 as libc::c_ulonglong) << 8 as libc::c_int)
        as libc::c_int;
    let mut has_varflags: libc::c_int = (getAllKeySpecsFlags(cmd, 0 as libc::c_int)
        as libc::c_ulonglong & (1 as libc::c_ulonglong) << 10 as libc::c_int)
        as libc::c_int;
    if has_keyspec != 0 && has_varflags == 0 {
        let mut ret: libc::c_int = getKeysUsingKeySpecs(
            cmd,
            argv,
            argc,
            search_flags,
            result,
        );
        if ret >= 0 as libc::c_int {
            return ret;
        }
    }
    if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 21 as libc::c_int
        != 0
    {
        return moduleGetCommandKeysViaAPI(cmd, argv, argc, result);
    }
    if ((*cmd).getkeys_proc).is_some() {
        return ((*cmd).getkeys_proc)
            .expect("non-null function pointer")(cmd, argv, argc, result);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn doesCommandHaveKeys(mut cmd: *mut redisCommand) -> libc::c_int {
    return (((*cmd).getkeys_proc).is_some()
        || (*cmd).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 21 as libc::c_int != 0
        || getAllKeySpecsFlags(cmd, 1 as libc::c_int) as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0) as libc::c_int;
}
#[no_mangle]
pub static mut commands_with_channels: [ChannelSpecs; 9] = unsafe {
    [
        {
            let mut init = ChannelSpecs {
                proc_0: Some(
                    subscribeCommand as unsafe extern "C" fn(*mut client) -> (),
                ),
                flags: ((1 as libc::c_ulonglong) << 12 as libc::c_int) as uint64_t,
                start: 1 as libc::c_int,
                count: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = ChannelSpecs {
                proc_0: Some(
                    ssubscribeCommand as unsafe extern "C" fn(*mut client) -> (),
                ),
                flags: ((1 as libc::c_ulonglong) << 12 as libc::c_int) as uint64_t,
                start: 1 as libc::c_int,
                count: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = ChannelSpecs {
                proc_0: Some(
                    unsubscribeCommand as unsafe extern "C" fn(*mut client) -> (),
                ),
                flags: ((1 as libc::c_ulonglong) << 13 as libc::c_int) as uint64_t,
                start: 1 as libc::c_int,
                count: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = ChannelSpecs {
                proc_0: Some(
                    sunsubscribeCommand as unsafe extern "C" fn(*mut client) -> (),
                ),
                flags: ((1 as libc::c_ulonglong) << 13 as libc::c_int) as uint64_t,
                start: 1 as libc::c_int,
                count: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = ChannelSpecs {
                proc_0: Some(
                    psubscribeCommand as unsafe extern "C" fn(*mut client) -> (),
                ),
                flags: ((1 as libc::c_ulonglong) << 11 as libc::c_int
                    | (1 as libc::c_ulonglong) << 12 as libc::c_int) as uint64_t,
                start: 1 as libc::c_int,
                count: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = ChannelSpecs {
                proc_0: Some(
                    punsubscribeCommand as unsafe extern "C" fn(*mut client) -> (),
                ),
                flags: ((1 as libc::c_ulonglong) << 11 as libc::c_int
                    | (1 as libc::c_ulonglong) << 13 as libc::c_int) as uint64_t,
                start: 1 as libc::c_int,
                count: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = ChannelSpecs {
                proc_0: Some(publishCommand as unsafe extern "C" fn(*mut client) -> ()),
                flags: ((1 as libc::c_ulonglong) << 14 as libc::c_int) as uint64_t,
                start: 1 as libc::c_int,
                count: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = ChannelSpecs {
                proc_0: Some(spublishCommand as unsafe extern "C" fn(*mut client) -> ()),
                flags: ((1 as libc::c_ulonglong) << 14 as libc::c_int) as uint64_t,
                start: 1 as libc::c_int,
                count: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = ChannelSpecs {
                proc_0: None,
                flags: 0 as libc::c_int as uint64_t,
                start: 0,
                count: 0,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn doesCommandHaveChannelsWithFlags(
    mut cmd: *mut redisCommand,
    mut flags: libc::c_int,
) -> libc::c_int {
    if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 27 as libc::c_int
        != 0
    {
        return 1 as libc::c_int;
    }
    let mut spec: *mut ChannelSpecs = commands_with_channels.as_mut_ptr();
    while ((*spec).proc_0).is_some() {
        if (*cmd).proc_0 == (*spec).proc_0 {
            return ((*spec).flags & flags as libc::c_ulong != 0) as libc::c_int;
        }
        spec = spec.offset(1 as libc::c_int as isize);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getChannelsFromCommand(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 27 as libc::c_int
        != 0
    {
        return moduleGetCommandChannelsViaAPI(cmd, argv, argc, result);
    }
    let mut spec: *mut ChannelSpecs = commands_with_channels.as_mut_ptr();
    while !spec.is_null() {
        if (*cmd).proc_0 == (*spec).proc_0 {
            let mut start: libc::c_int = (*spec).start;
            let mut stop: libc::c_int = if (*spec).count == -(1 as libc::c_int) {
                argc
            } else {
                start + (*spec).count
            };
            if stop > argc {
                stop = argc;
            }
            let mut count: libc::c_int = 0 as libc::c_int;
            keys = getKeysPrepareResult(result, stop - start);
            let mut i: libc::c_int = start;
            while i < stop {
                (*keys.offset(count as isize)).pos = i;
                let fresh6 = count;
                count = count + 1;
                (*keys.offset(fresh6 as isize)).flags = (*spec).flags as libc::c_int;
                i += 1;
            }
            (*result).numkeys = count;
            return count;
        }
        spec = spec.offset(1 as libc::c_int as isize);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getKeysUsingLegacyRangeSpec(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut last: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    if (*cmd).legacy_range_key_spec.begin_search_type as libc::c_uint
        == KSPEC_BS_INVALID as libc::c_int as libc::c_uint
    {
        (*result).numkeys = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    first = (*cmd).legacy_range_key_spec.bs.index.pos;
    last = (*cmd).legacy_range_key_spec.fk.range.lastkey;
    if last >= 0 as libc::c_int {
        last += first;
    }
    step = (*cmd).legacy_range_key_spec.fk.range.keystep;
    if last < 0 as libc::c_int {
        last = argc + last;
    }
    let mut count: libc::c_int = last - first + 1 as libc::c_int;
    keys = getKeysPrepareResult(result, count);
    j = first;
    while j <= last {
        if j >= argc || j < first {
            if (*cmd).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0
                || (*cmd).arity < 0 as libc::c_int
            {
                (*result).numkeys = 0 as libc::c_int;
                return 0 as libc::c_int;
            } else {
                _serverPanic(
                    b"db.c\0" as *const u8 as *const libc::c_char,
                    2050 as libc::c_int,
                    b"Redis built-in command declared keys positions not matching the arity requirements.\0"
                        as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
        (*keys.offset(i as isize)).pos = j;
        let fresh7 = i;
        i = i + 1;
        (*keys.offset(fresh7 as isize)).flags = 0 as libc::c_int;
        j += step;
    }
    (*result).numkeys = i;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn getKeysFromCommand(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 21 as libc::c_int
        != 0
    {
        return moduleGetCommandKeysViaAPI(cmd, argv, argc, result)
    } else if ((*cmd).getkeys_proc).is_some() {
        return ((*cmd).getkeys_proc)
            .expect("non-null function pointer")(cmd, argv, argc, result)
    } else {
        return getKeysUsingLegacyRangeSpec(cmd, argv, argc, result)
    };
}
#[no_mangle]
pub unsafe extern "C" fn getKeysFreeResult(mut result: *mut getKeysResult) {
    if !result.is_null() && (*result).keys != ((*result).keysbuf).as_mut_ptr() {
        zfree((*result).keys as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn genericGetKeys(
    mut storeKeyOfs: libc::c_int,
    mut keyCountOfs: libc::c_int,
    mut firstKeyOfs: libc::c_int,
    mut keyStep: libc::c_int,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    num = atoi((**argv.offset(keyCountOfs as isize)).ptr as *const libc::c_char);
    if num < 1 as libc::c_int || num > (argc - firstKeyOfs) / keyStep {
        (*result).numkeys = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    let mut numkeys: libc::c_int = if storeKeyOfs != 0 {
        num + 1 as libc::c_int
    } else {
        num
    };
    keys = getKeysPrepareResult(result, numkeys);
    (*result).numkeys = numkeys;
    i = 0 as libc::c_int;
    while i < num {
        (*keys.offset(i as isize)).pos = firstKeyOfs + i * keyStep;
        (*keys.offset(i as isize)).flags = 0 as libc::c_int;
        i += 1;
    }
    if storeKeyOfs != 0 {
        (*keys.offset(num as isize)).pos = storeKeyOfs;
        (*keys.offset(num as isize)).flags = 0 as libc::c_int;
    }
    return (*result).numkeys;
}
#[no_mangle]
pub unsafe extern "C" fn sintercardGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zunionInterDiffStoreGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zunionInterDiffGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn evalGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        0 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn functionGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        0 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lmpopGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn blmpopGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        0 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zmpopGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bzmpopGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    return genericGetKeys(
        0 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        argv,
        argc,
        result,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sortROGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    keys = getKeysPrepareResult(result, 1 as libc::c_int);
    (*keys.offset(0 as libc::c_int as isize)).pos = 1 as libc::c_int;
    (*keys.offset(0 as libc::c_int as isize))
        .flags = ((1 as libc::c_ulonglong) << 0 as libc::c_int
        | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sortGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut found_store: libc::c_int = 0 as libc::c_int;
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    num = 0 as libc::c_int;
    keys = getKeysPrepareResult(result, 2 as libc::c_int);
    (*keys.offset(num as isize)).pos = 1 as libc::c_int;
    let fresh8 = num;
    num = num + 1;
    (*keys.offset(fresh8 as isize))
        .flags = ((1 as libc::c_ulonglong) << 0 as libc::c_int
        | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_int;
    let mut skiplist: [C2RustUnnamed_12; 4] = [
        {
            let mut init = C2RustUnnamed_12 {
                name: b"limit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                skip: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"get\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                skip: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                skip: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: 0 as *mut libc::c_char,
                skip: 0 as libc::c_int,
            };
            init
        },
    ];
    i = 2 as libc::c_int;
    while i < argc {
        j = 0 as libc::c_int;
        while !(skiplist[j as usize].name).is_null() {
            if strcasecmp(
                (**argv.offset(i as isize)).ptr as *const libc::c_char,
                skiplist[j as usize].name,
            ) == 0
            {
                i += skiplist[j as usize].skip;
                break;
            } else if strcasecmp(
                (**argv.offset(i as isize)).ptr as *const libc::c_char,
                b"store\0" as *const u8 as *const libc::c_char,
            ) == 0 && (i + 1 as libc::c_int) < argc
            {
                found_store = 1 as libc::c_int;
                (*keys.offset(num as isize)).pos = i + 1 as libc::c_int;
                (*keys.offset(num as isize))
                    .flags = ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | (1 as libc::c_ulonglong) << 5 as libc::c_int) as libc::c_int;
                break;
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    (*result).numkeys = num + found_store;
    return (*result).numkeys;
}
#[no_mangle]
pub unsafe extern "C" fn migrateGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    first = 3 as libc::c_int;
    num = 1 as libc::c_int;
    let mut skip_keywords: [C2RustUnnamed_13; 5] = [
        {
            let mut init = C2RustUnnamed_13 {
                name: b"copy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                skip: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_13 {
                name: b"replace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                skip: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_13 {
                name: b"auth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                skip: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_13 {
                name: b"auth2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                skip: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_13 {
                name: 0 as *mut libc::c_char,
                skip: 0 as libc::c_int,
            };
            init
        },
    ];
    if argc > 6 as libc::c_int {
        i = 6 as libc::c_int;
        while i < argc {
            if strcasecmp(
                (**argv.offset(i as isize)).ptr as *const libc::c_char,
                b"keys\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if sdslen((**argv.offset(3 as libc::c_int as isize)).ptr as sds)
                    > 0 as libc::c_int as libc::c_ulong
                {
                    num = 0 as libc::c_int;
                } else {
                    first = i + 1 as libc::c_int;
                    num = argc - first;
                }
                break;
            } else {
                j = 0 as libc::c_int;
                while !(skip_keywords[j as usize].name).is_null() {
                    if strcasecmp(
                        (**argv.offset(i as isize)).ptr as *const libc::c_char,
                        skip_keywords[j as usize].name,
                    ) == 0
                    {
                        i += skip_keywords[j as usize].skip;
                        break;
                    } else {
                        j += 1;
                    }
                }
                i += 1;
            }
        }
    }
    keys = getKeysPrepareResult(result, num);
    i = 0 as libc::c_int;
    while i < num {
        (*keys.offset(i as isize)).pos = first + i;
        (*keys.offset(i as isize))
            .flags = ((1 as libc::c_ulonglong) << 1 as libc::c_int
            | (1 as libc::c_ulonglong) << 4 as libc::c_int
            | (1 as libc::c_ulonglong) << 7 as libc::c_int) as libc::c_int;
        i += 1;
    }
    (*result).numkeys = num;
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn georadiusGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    let mut stored_key: libc::c_int = -(1 as libc::c_int);
    i = 5 as libc::c_int;
    while i < argc {
        let mut arg: *mut libc::c_char = (**argv.offset(i as isize)).ptr
            as *mut libc::c_char;
        if (strcasecmp(arg, b"store\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(arg, b"storedist\0" as *const u8 as *const libc::c_char) == 0)
            && (i + 1 as libc::c_int) < argc
        {
            stored_key = i + 1 as libc::c_int;
            i += 1;
        }
        i += 1;
    }
    num = 1 as libc::c_int
        + (if stored_key == -(1 as libc::c_int) {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        });
    keys = getKeysPrepareResult(result, num);
    (*keys.offset(0 as libc::c_int as isize)).pos = 1 as libc::c_int;
    (*keys.offset(0 as libc::c_int as isize)).flags = 0 as libc::c_int;
    if num > 1 as libc::c_int {
        (*keys.offset(1 as libc::c_int as isize)).pos = stored_key;
        (*keys.offset(1 as libc::c_int as isize)).flags = 0 as libc::c_int;
    }
    (*result).numkeys = num;
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn xreadGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    let mut streams_pos: libc::c_int = -(1 as libc::c_int);
    i = 1 as libc::c_int;
    while i < argc {
        let mut arg: *mut libc::c_char = (**argv.offset(i as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(arg, b"block\0" as *const u8 as *const libc::c_char) == 0 {
            i += 1;
        } else if strcasecmp(arg, b"count\0" as *const u8 as *const libc::c_char) == 0 {
            i += 1;
        } else if strcasecmp(arg, b"group\0" as *const u8 as *const libc::c_char) == 0 {
            i += 2 as libc::c_int;
        } else if !(strcasecmp(arg, b"noack\0" as *const u8 as *const libc::c_char) == 0)
        {
            if !(strcasecmp(arg, b"streams\0" as *const u8 as *const libc::c_char) == 0)
            {
                break;
            }
            streams_pos = i;
            break;
        }
        i += 1;
    }
    if streams_pos != -(1 as libc::c_int) {
        num = argc - streams_pos - 1 as libc::c_int;
    }
    if streams_pos == -(1 as libc::c_int) || num == 0 as libc::c_int
        || num % 2 as libc::c_int != 0 as libc::c_int
    {
        (*result).numkeys = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    num /= 2 as libc::c_int;
    keys = getKeysPrepareResult(result, num);
    i = streams_pos + 1 as libc::c_int;
    while i < argc - num {
        (*keys.offset((i - streams_pos - 1 as libc::c_int) as isize)).pos = i;
        (*keys.offset((i - streams_pos - 1 as libc::c_int) as isize))
            .flags = 0 as libc::c_int;
        i += 1;
    }
    (*result).numkeys = num;
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn setGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    keys = getKeysPrepareResult(result, 1 as libc::c_int);
    (*keys.offset(0 as libc::c_int as isize)).pos = 1 as libc::c_int;
    (*result).numkeys = 1 as libc::c_int;
    let mut i: libc::c_int = 3 as libc::c_int;
    while i < argc {
        let mut arg: *mut libc::c_char = (**argv.offset(i as isize)).ptr
            as *mut libc::c_char;
        if (*arg.offset(0 as libc::c_int as isize) as libc::c_int == 'g' as i32
            || *arg.offset(0 as libc::c_int as isize) as libc::c_int == 'G' as i32)
            && (*arg.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32
                || *arg.offset(1 as libc::c_int as isize) as libc::c_int == 'E' as i32)
            && (*arg.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32
                || *arg.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32)
            && *arg.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            (*keys.offset(0 as libc::c_int as isize))
                .flags = ((1 as libc::c_ulonglong) << 1 as libc::c_int
                | (1 as libc::c_ulonglong) << 4 as libc::c_int
                | (1 as libc::c_ulonglong) << 5 as libc::c_int) as libc::c_int;
            return 1 as libc::c_int;
        }
        i += 1;
    }
    (*keys.offset(0 as libc::c_int as isize))
        .flags = ((1 as libc::c_ulonglong) << 2 as libc::c_int
        | (1 as libc::c_ulonglong) << 5 as libc::c_int) as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bitfieldGetKeys(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut keys: *mut keyReference = 0 as *mut keyReference;
    let mut readonly: libc::c_int = 1 as libc::c_int;
    keys = getKeysPrepareResult(result, 1 as libc::c_int);
    (*keys.offset(0 as libc::c_int as isize)).pos = 1 as libc::c_int;
    (*result).numkeys = 1 as libc::c_int;
    let mut i: libc::c_int = 2 as libc::c_int;
    while i < argc {
        let mut remargs: libc::c_int = argc - i - 1 as libc::c_int;
        let mut arg: *mut libc::c_char = (**argv.offset(i as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(arg, b"get\0" as *const u8 as *const libc::c_char) == 0
            && remargs >= 2 as libc::c_int
        {
            i += 2 as libc::c_int;
        } else if (strcasecmp(arg, b"set\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(arg, b"incrby\0" as *const u8 as *const libc::c_char) == 0)
            && remargs >= 3 as libc::c_int
        {
            readonly = 0 as libc::c_int;
            i += 3 as libc::c_int;
            break;
        } else if strcasecmp(arg, b"overflow\0" as *const u8 as *const libc::c_char) == 0
            && remargs >= 1 as libc::c_int
        {
            i += 1 as libc::c_int;
        } else {
            readonly = 0 as libc::c_int;
            break;
        }
        i += 1;
    }
    if readonly != 0 {
        (*keys.offset(0 as libc::c_int as isize))
            .flags = ((1 as libc::c_ulonglong) << 0 as libc::c_int
            | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_int;
    } else {
        (*keys.offset(0 as libc::c_int as isize))
            .flags = ((1 as libc::c_ulonglong) << 1 as libc::c_int
            | (1 as libc::c_ulonglong) << 4 as libc::c_int
            | (1 as libc::c_ulonglong) << 5 as libc::c_int) as libc::c_int;
    }
    return 1 as libc::c_int;
}
