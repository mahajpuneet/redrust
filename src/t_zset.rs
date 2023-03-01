extern crate c2rust_bitfields;
extern crate libc;
extern crate core;

extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut zsetDictType: dictType;
    static mut hashDictType: dictType;
    static mut sdsReplyDictType: dictType;
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn addReplyNull(c: *mut client);
    fn addReplyNullArray(c: *mut client);
    fn addReplyBulk(c: *mut client, obj: *mut robj);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyBulkLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyDouble(c: *mut client, d: libc::c_double);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn rewriteClientCommandVector(c: *mut client, argc: libc::c_int, _: ...);
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscmp(s1: sds, s2: sds) -> libc::c_int;
    fn sdsfromlonglong(value: libc::c_longlong) -> sds;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn random() -> libc::c_long;
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
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictExpand(d: *mut dict, size: libc::c_ulong) -> libc::c_int;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictAddRaw(
        d: *mut dict,
        key: *mut libc::c_void,
        existing: *mut *mut dictEntry,
    ) -> *mut dictEntry;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn dictUnlink(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFreeUnlinkedEntry(d: *mut dict, he: *mut dictEntry);
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictResize(d: *mut dict) -> libc::c_int;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictGetFairRandomKey(d: *mut dict) -> *mut dictEntry;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn intsetFind(is: *mut intset, value: int64_t) -> uint8_t;
    fn intsetGet(is: *mut intset, pos: uint32_t, value: *mut int64_t) -> uint8_t;
    fn intsetLen(is: *const intset) -> uint32_t;
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
    fn d2string(
        buf: *mut libc::c_char,
        len: size_t,
        value: libc::c_double,
    ) -> libc::c_int;
    fn double2ll(d: libc::c_double, out: *mut libc::c_longlong) -> libc::c_int;
    fn lpNew(capacity: size_t) -> *mut libc::c_uchar;
    fn lpInsertString(
        lp: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
        p: *mut libc::c_uchar,
        where_0: libc::c_int,
        newp: *mut *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpInsertInteger(
        lp: *mut libc::c_uchar,
        lval: libc::c_longlong,
        p: *mut libc::c_uchar,
        where_0: libc::c_int,
        newp: *mut *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpAppend(
        lp: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> *mut libc::c_uchar;
    fn lpAppendInteger(
        lp: *mut libc::c_uchar,
        lval: libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpDeleteRangeWithEntry(
        lp: *mut libc::c_uchar,
        p: *mut *mut libc::c_uchar,
        num: libc::c_ulong,
    ) -> *mut libc::c_uchar;
    fn lpDeleteRange(
        lp: *mut libc::c_uchar,
        index: libc::c_long,
        num: libc::c_ulong,
    ) -> *mut libc::c_uchar;
    fn lpLength(lp: *mut libc::c_uchar) -> libc::c_ulong;
    fn lpGetValue(
        p: *mut libc::c_uchar,
        slen: *mut libc::c_uint,
        lval: *mut libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpFind(
        lp: *mut libc::c_uchar,
        p: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
        skip: libc::c_uint,
    ) -> *mut libc::c_uchar;
    fn lpFirst(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpNext(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpPrev(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpBytes(lp: *mut libc::c_uchar) -> size_t;
    fn lpSeek(lp: *mut libc::c_uchar, index: libc::c_long) -> *mut libc::c_uchar;
    fn lpCompare(
        p: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> libc::c_uint;
    fn lpRandomPair(
        lp: *mut libc::c_uchar,
        total_count: libc::c_ulong,
        key: *mut listpackEntry,
        val: *mut listpackEntry,
    );
    fn lpRandomPairs(
        lp: *mut libc::c_uchar,
        count: libc::c_uint,
        keys: *mut listpackEntry,
        vals: *mut listpackEntry,
    );
    fn lpRandomPairsUnique(
        lp: *mut libc::c_uchar,
        count: libc::c_uint,
        keys: *mut listpackEntry,
        vals: *mut listpackEntry,
    ) -> libc::c_uint;
    fn lpSafeToAdd(lp: *mut libc::c_uchar, add: size_t) -> libc::c_int;
    fn decrRefCount(o: *mut robj);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn createStringObjectFromLongLong(value: libc::c_longlong) -> *mut robj;
    fn createZsetObject() -> *mut robj;
    fn createZsetListpackObject() -> *mut robj;
    fn getLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getPositiveLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getRangeLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        min: libc::c_long,
        max: libc::c_long,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn checkType(c: *mut client, o: *mut robj, type_0: libc::c_int) -> libc::c_int;
    fn getDoubleFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_double,
        msg: *const libc::c_char,
    ) -> libc::c_int;
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
    fn htNeedsResize(dict: *mut dict) -> libc::c_int;
    fn blockForKeys(
        c: *mut client,
        btype: libc::c_int,
        keys: *mut *mut robj,
        numkeys: libc::c_int,
        count: libc::c_long,
        timeout: mstime_t,
        target: *mut robj,
        blockpos: *mut blockPos,
        ids: *mut streamID,
    );
    fn lookupKeyWrite(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn getTimeoutFromObjectOrReply(
        c: *mut client,
        object: *mut robj,
        timeout: *mut mstime_t,
        unit: libc::c_int,
    ) -> libc::c_int;
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn dbDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn lookupKeyRead(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyReadOrReply(
        c: *mut client,
        key: *mut robj,
        reply: *mut robj,
    ) -> *mut robj;
    fn lookupKeyWriteOrReply(
        c: *mut client,
        key: *mut robj,
        reply: *mut robj,
    ) -> *mut robj;
    fn dbAdd(db: *mut redisDb, key: *mut robj, val: *mut robj);
    fn setKey(
        c: *mut client,
        db: *mut redisDb,
        key: *mut robj,
        val: *mut robj,
        flags: libc::c_int,
    );
    fn scanGenericCommand(c: *mut client, o: *mut robj, cursor: libc::c_ulong);
    fn parseScanCursorOrReply(
        c: *mut client,
        o: *mut robj,
        cursor: *mut libc::c_ulong,
    ) -> libc::c_int;
    fn dictSdsHash(key: *const libc::c_void) -> uint64_t;
    fn dictSdsKeyCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct listpackEntry {
    pub sval: *mut libc::c_uchar,
    pub slen: uint32_t,
    pub lval: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamID {
    pub ms: uint64_t,
    pub seq: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zrangespec {
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub minex: libc::c_int,
    pub maxex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zlexrangespec {
    pub min: sds,
    pub max: sds,
    pub minex: libc::c_int,
    pub maxex: libc::c_int,
}
pub type zrange_direction = libc::c_uint;
pub const ZRANGE_DIRECTION_REVERSE: zrange_direction = 2;
pub const ZRANGE_DIRECTION_FORWARD: zrange_direction = 1;
pub const ZRANGE_DIRECTION_AUTO: zrange_direction = 0;
pub type zrange_type = libc::c_uint;
pub const ZRANGE_LEX: zrange_type = 3;
pub const ZRANGE_SCORE: zrange_type = 2;
pub const ZRANGE_RANK: zrange_type = 1;
pub const ZRANGE_AUTO: zrange_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zrange_result_handler {
    pub type_0: zrange_consumer_type,
    pub client: *mut client,
    pub dstkey: *mut robj,
    pub dstobj: *mut robj,
    pub userdata: *mut libc::c_void,
    pub withscores: libc::c_int,
    pub should_emit_array_length: libc::c_int,
    pub beginResultEmission: zrangeResultBeginFunction,
    pub finalizeResultEmission: zrangeResultFinalizeFunction,
    pub emitResultFromCBuffer: zrangeResultEmitCBufferFunction,
    pub emitResultFromLongLong: zrangeResultEmitLongLongFunction,
}
pub type zrangeResultEmitLongLongFunction = Option::<
    unsafe extern "C" fn(
        *mut zrange_result_handler,
        libc::c_longlong,
        libc::c_double,
    ) -> (),
>;
pub type zrangeResultEmitCBufferFunction = Option::<
    unsafe extern "C" fn(
        *mut zrange_result_handler,
        *const libc::c_void,
        size_t,
        libc::c_double,
    ) -> (),
>;
pub type zrangeResultFinalizeFunction = Option::<
    unsafe extern "C" fn(*mut zrange_result_handler, size_t) -> (),
>;
pub type zrangeResultBeginFunction = Option::<
    unsafe extern "C" fn(*mut zrange_result_handler, libc::c_long) -> (),
>;
pub type zrange_consumer_type = libc::c_uint;
pub const ZRANGE_CONSUMER_TYPE_INTERNAL: zrange_consumer_type = 1;
pub const ZRANGE_CONSUMER_TYPE_CLIENT: zrange_consumer_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zsetopsrc {
    pub subject: *mut robj,
    pub type_0: libc::c_int,
    pub encoding: libc::c_int,
    pub weight: libc::c_double,
    pub iter: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub set: _iterset,
    pub zset: _iterzset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _iterzset {
    pub zl: C2RustUnnamed_9,
    pub sl: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub zs: *mut zset,
    pub node: *mut zskiplistNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub zl: *mut libc::c_uchar,
    pub eptr: *mut libc::c_uchar,
    pub sptr: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _iterset {
    pub is: C2RustUnnamed_11,
    pub ht: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub dict: *mut dict,
    pub di: *mut dictIterator,
    pub de: *mut dictEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub is: *mut intset,
    pub ii: libc::c_int,
}
pub type iterzset = _iterzset;
pub type iterset = _iterset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zsetopval {
    pub flags: libc::c_int,
    pub _buf: [libc::c_uchar; 32],
    pub ele: sds,
    pub estr: *mut libc::c_uchar,
    pub elen: libc::c_uint,
    pub ell: libc::c_longlong,
    pub score: libc::c_double,
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
pub unsafe extern "C" fn zslCreateNode(
    mut level: libc::c_int,
    mut score: libc::c_double,
    mut ele: sds,
) -> *mut zskiplistNode {
    let mut zn: *mut zskiplistNode = zmalloc(
        (core::mem::size_of::<zskiplistNode>() as libc::c_ulong)
            .wrapping_add(
                (level as libc::c_ulong)
                    .wrapping_mul(
                        core::mem::size_of::<zskiplistLevel>() as libc::c_ulong,
                    ),
            ),
    ) as *mut zskiplistNode;
    (*zn).score = score;
    (*zn).ele = ele;
    return zn;
}
#[no_mangle]
pub unsafe extern "C" fn zslCreate() -> *mut zskiplist {
    let mut j: libc::c_int = 0;
    let mut zsl: *mut zskiplist = 0 as *mut zskiplist;
    zsl = zmalloc(core::mem::size_of::<zskiplist>() as libc::c_ulong)
        as *mut zskiplist;
    (*zsl).level = 1 as libc::c_int;
    (*zsl).length = 0 as libc::c_int as libc::c_ulong;
    (*zsl)
        .header = zslCreateNode(
        32 as libc::c_int,
        0 as libc::c_int as libc::c_double,
        0 as sds,
    );
    j = 0 as libc::c_int;
    while j < 32 as libc::c_int {
        let ref mut fresh0 = (*((*(*zsl).header).level).as_mut_ptr().offset(j as isize))
            .forward;
        *fresh0 = 0 as *mut zskiplistNode;
        (*((*(*zsl).header).level).as_mut_ptr().offset(j as isize))
            .span = 0 as libc::c_int as libc::c_ulong;
        j += 1;
    }
    (*(*zsl).header).backward = 0 as *mut zskiplistNode;
    (*zsl).tail = 0 as *mut zskiplistNode;
    return zsl;
}
#[no_mangle]
pub unsafe extern "C" fn zslFreeNode(mut node: *mut zskiplistNode) {
    sdsfree((*node).ele);
    zfree(node as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn zslFree(mut zsl: *mut zskiplist) {
    let mut node: *mut zskiplistNode = (*((*(*zsl).header).level)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .forward;
    let mut next: *mut zskiplistNode = 0 as *mut zskiplistNode;
    zfree((*zsl).header as *mut libc::c_void);
    while !node.is_null() {
        next = (*((*node).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
        zslFreeNode(node);
        node = next;
    }
    zfree(zsl as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn zslRandomLevel() -> libc::c_int {
    static mut threshold: libc::c_int = (0.25f64
        * 2147483647 as libc::c_int as libc::c_double) as libc::c_int;
    let mut level: libc::c_int = 1 as libc::c_int;
    while random() < threshold as libc::c_long {
        level += 1 as libc::c_int;
    }
    return if level < 32 as libc::c_int { level } else { 32 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn zslInsert(
    mut zsl: *mut zskiplist,
    mut score: libc::c_double,
    mut ele: sds,
) -> *mut zskiplistNode {
    let mut update: [*mut zskiplistNode; 32] = [0 as *mut zskiplistNode; 32];
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut rank: [libc::c_ulong; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    if score.is_nan() as i32 == 0 {} else {
        _serverAssert(
            b"!isnan(score)\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
        );
        unreachable!();
    };
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        rank[i
            as usize] = if i == (*zsl).level - 1 as libc::c_int {
            0 as libc::c_int as libc::c_ulong
        } else {
            rank[(i + 1 as libc::c_int) as usize]
        };
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && ((*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score < score
                || (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score
                    == score
                    && sdscmp(
                        (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).ele,
                        ele,
                    ) < 0 as libc::c_int)
        {
            rank[i
                as usize] = (rank[i as usize])
                .wrapping_add((*((*x).level).as_mut_ptr().offset(i as isize)).span);
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        update[i as usize] = x;
        i -= 1;
    }
    level = zslRandomLevel();
    if level > (*zsl).level {
        i = (*zsl).level;
        while i < level {
            rank[i as usize] = 0 as libc::c_int as libc::c_ulong;
            update[i as usize] = (*zsl).header;
            (*((*update[i as usize]).level).as_mut_ptr().offset(i as isize))
                .span = (*zsl).length;
            i += 1;
        }
        (*zsl).level = level;
    }
    x = zslCreateNode(level, score, ele);
    i = 0 as libc::c_int;
    while i < level {
        let ref mut fresh1 = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        *fresh1 = (*((*update[i as usize]).level).as_mut_ptr().offset(i as isize))
            .forward;
        let ref mut fresh2 = (*((*update[i as usize]).level)
            .as_mut_ptr()
            .offset(i as isize))
            .forward;
        *fresh2 = x;
        (*((*x).level).as_mut_ptr().offset(i as isize))
            .span = ((*((*update[i as usize]).level).as_mut_ptr().offset(i as isize))
            .span)
            .wrapping_sub(
                (rank[0 as libc::c_int as usize]).wrapping_sub(rank[i as usize]),
            );
        (*((*update[i as usize]).level).as_mut_ptr().offset(i as isize))
            .span = (rank[0 as libc::c_int as usize])
            .wrapping_sub(rank[i as usize])
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        i += 1;
    }
    i = level;
    while i < (*zsl).level {
        let ref mut fresh3 = (*((*update[i as usize]).level)
            .as_mut_ptr()
            .offset(i as isize))
            .span;
        *fresh3 = (*fresh3).wrapping_add(1);
        i += 1;
    }
    (*x)
        .backward = if update[0 as libc::c_int as usize] == (*zsl).header {
        0 as *mut zskiplistNode
    } else {
        update[0 as libc::c_int as usize]
    };
    if !((*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward)
        .is_null()
    {
        let ref mut fresh4 = (*(*((*x).level)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .forward)
            .backward;
        *fresh4 = x;
    } else {
        (*zsl).tail = x;
    }
    (*zsl).length = ((*zsl).length).wrapping_add(1);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn zslDeleteNode(
    mut zsl: *mut zskiplist,
    mut x: *mut zskiplistNode,
    mut update: *mut *mut zskiplistNode,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*zsl).level {
        if (*((**update.offset(i as isize)).level).as_mut_ptr().offset(i as isize))
            .forward == x
        {
            let ref mut fresh5 = (*((**update.offset(i as isize)).level)
                .as_mut_ptr()
                .offset(i as isize))
                .span;
            *fresh5 = (*fresh5)
                .wrapping_add(
                    ((*((*x).level).as_mut_ptr().offset(i as isize)).span)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            let ref mut fresh6 = (*((**update.offset(i as isize)).level)
                .as_mut_ptr()
                .offset(i as isize))
                .forward;
            *fresh6 = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        } else {
            let ref mut fresh7 = (*((**update.offset(i as isize)).level)
                .as_mut_ptr()
                .offset(i as isize))
                .span;
            *fresh7 = (*fresh7).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        i += 1;
    }
    if !((*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward)
        .is_null()
    {
        let ref mut fresh8 = (*(*((*x).level)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .forward)
            .backward;
        *fresh8 = (*x).backward;
    } else {
        (*zsl).tail = (*x).backward;
    }
    while (*zsl).level > 1 as libc::c_int
        && ((*((*(*zsl).header).level)
            .as_mut_ptr()
            .offset(((*zsl).level - 1 as libc::c_int) as isize))
            .forward)
            .is_null()
    {
        (*zsl).level -= 1;
    }
    (*zsl).length = ((*zsl).length).wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn zslDelete(
    mut zsl: *mut zskiplist,
    mut score: libc::c_double,
    mut ele: sds,
    mut node: *mut *mut zskiplistNode,
) -> libc::c_int {
    let mut update: [*mut zskiplistNode; 32] = [0 as *mut zskiplistNode; 32];
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut i: libc::c_int = 0;
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && ((*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score < score
                || (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score
                    == score
                    && sdscmp(
                        (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).ele,
                        ele,
                    ) < 0 as libc::c_int)
        {
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        update[i as usize] = x;
        i -= 1;
    }
    x = (*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
    if !x.is_null() && score == (*x).score && sdscmp((*x).ele, ele) == 0 as libc::c_int {
        zslDeleteNode(zsl, x, update.as_mut_ptr());
        if node.is_null() {
            zslFreeNode(x);
        } else {
            *node = x;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zslUpdateScore(
    mut zsl: *mut zskiplist,
    mut curscore: libc::c_double,
    mut ele: sds,
    mut newscore: libc::c_double,
) -> *mut zskiplistNode {
    let mut update: [*mut zskiplistNode; 32] = [0 as *mut zskiplistNode; 32];
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut i: libc::c_int = 0;
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && ((*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score
                < curscore
                || (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score
                    == curscore
                    && sdscmp(
                        (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).ele,
                        ele,
                    ) < 0 as libc::c_int)
        {
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        update[i as usize] = x;
        i -= 1;
    }
    x = (*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
    if !x.is_null() && curscore == (*x).score
        && sdscmp((*x).ele, ele) == 0 as libc::c_int
    {} else {
        _serverAssert(
            b"x && curscore == x->score && sdscmp(x->ele,ele) == 0\0" as *const u8
                as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            281 as libc::c_int,
        );
        unreachable!();
    };
    if (((*x).backward).is_null() || (*(*x).backward).score < newscore)
        && (((*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward)
            .is_null()
            || (*(*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward)
                .score > newscore)
    {
        (*x).score = newscore;
        return x;
    }
    zslDeleteNode(zsl, x, update.as_mut_ptr());
    let mut newnode: *mut zskiplistNode = zslInsert(zsl, newscore, (*x).ele);
    (*x).ele = 0 as sds;
    zslFreeNode(x);
    return newnode;
}
#[no_mangle]
pub unsafe extern "C" fn zslValueGteMin(
    mut value: libc::c_double,
    mut spec: *mut zrangespec,
) -> libc::c_int {
    return if (*spec).minex != 0 {
        (value > (*spec).min) as libc::c_int
    } else {
        (value >= (*spec).min) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn zslValueLteMax(
    mut value: libc::c_double,
    mut spec: *mut zrangespec,
) -> libc::c_int {
    return if (*spec).maxex != 0 {
        (value < (*spec).max) as libc::c_int
    } else {
        (value <= (*spec).max) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn zslIsInRange(
    mut zsl: *mut zskiplist,
    mut range: *mut zrangespec,
) -> libc::c_int {
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    if (*range).min > (*range).max
        || (*range).min == (*range).max && ((*range).minex != 0 || (*range).maxex != 0)
    {
        return 0 as libc::c_int;
    }
    x = (*zsl).tail;
    if x.is_null() || zslValueGteMin((*x).score, range) == 0 {
        return 0 as libc::c_int;
    }
    x = (*((*(*zsl).header).level).as_mut_ptr().offset(0 as libc::c_int as isize))
        .forward;
    if x.is_null() || zslValueLteMax((*x).score, range) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zslFirstInRange(
    mut zsl: *mut zskiplist,
    mut range: *mut zrangespec,
) -> *mut zskiplistNode {
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut i: libc::c_int = 0;
    if zslIsInRange(zsl, range) == 0 {
        return 0 as *mut zskiplistNode;
    }
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && zslValueGteMin(
                (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score,
                range,
            ) == 0
        {
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        i -= 1;
    }
    x = (*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
    if !x.is_null() {} else {
        _serverAssert(
            b"x != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int,
        );
        unreachable!();
    };
    if zslValueLteMax((*x).score, range) == 0 {
        return 0 as *mut zskiplistNode;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn zslLastInRange(
    mut zsl: *mut zskiplist,
    mut range: *mut zrangespec,
) -> *mut zskiplistNode {
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut i: libc::c_int = 0;
    if zslIsInRange(zsl, range) == 0 {
        return 0 as *mut zskiplistNode;
    }
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && zslValueLteMax(
                (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score,
                range,
            ) != 0
        {
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        i -= 1;
    }
    if !x.is_null() {} else {
        _serverAssert(
            b"x != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            373 as libc::c_int,
        );
        unreachable!();
    };
    if zslValueGteMin((*x).score, range) == 0 {
        return 0 as *mut zskiplistNode;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn zslDeleteRangeByScore(
    mut zsl: *mut zskiplist,
    mut range: *mut zrangespec,
    mut dict: *mut dict,
) -> libc::c_ulong {
    let mut update: [*mut zskiplistNode; 32] = [0 as *mut zskiplistNode; 32];
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut removed: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_int = 0;
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && zslValueGteMin(
                (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score,
                range,
            ) == 0
        {
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        update[i as usize] = x;
        i -= 1;
    }
    x = (*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
    while !x.is_null() && zslValueLteMax((*x).score, range) != 0 {
        let mut next: *mut zskiplistNode = (*((*x).level)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .forward;
        zslDeleteNode(zsl, x, update.as_mut_ptr());
        dictDelete(dict, (*x).ele as *const libc::c_void);
        zslFreeNode(x);
        removed = removed.wrapping_add(1);
        x = next;
    }
    return removed;
}
#[no_mangle]
pub unsafe extern "C" fn zslDeleteRangeByLex(
    mut zsl: *mut zskiplist,
    mut range: *mut zlexrangespec,
    mut dict: *mut dict,
) -> libc::c_ulong {
    let mut update: [*mut zskiplistNode; 32] = [0 as *mut zskiplistNode; 32];
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut removed: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_int = 0;
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && zslLexValueGteMin(
                (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).ele,
                range,
            ) == 0
        {
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        update[i as usize] = x;
        i -= 1;
    }
    x = (*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
    while !x.is_null() && zslLexValueLteMax((*x).ele, range) != 0 {
        let mut next: *mut zskiplistNode = (*((*x).level)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .forward;
        zslDeleteNode(zsl, x, update.as_mut_ptr());
        dictDelete(dict, (*x).ele as *const libc::c_void);
        zslFreeNode(x);
        removed = removed.wrapping_add(1);
        x = next;
    }
    return removed;
}
#[no_mangle]
pub unsafe extern "C" fn zslDeleteRangeByRank(
    mut zsl: *mut zskiplist,
    mut start: libc::c_uint,
    mut end: libc::c_uint,
    mut dict: *mut dict,
) -> libc::c_ulong {
    let mut update: [*mut zskiplistNode; 32] = [0 as *mut zskiplistNode; 32];
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut traversed: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut removed: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_int = 0;
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && traversed
                .wrapping_add((*((*x).level).as_mut_ptr().offset(i as isize)).span)
                < start as libc::c_ulong
        {
            traversed = traversed
                .wrapping_add((*((*x).level).as_mut_ptr().offset(i as isize)).span);
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        update[i as usize] = x;
        i -= 1;
    }
    traversed = traversed.wrapping_add(1);
    x = (*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
    while !x.is_null() && traversed <= end as libc::c_ulong {
        let mut next: *mut zskiplistNode = (*((*x).level)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .forward;
        zslDeleteNode(zsl, x, update.as_mut_ptr());
        dictDelete(dict, (*x).ele as *const libc::c_void);
        zslFreeNode(x);
        removed = removed.wrapping_add(1);
        traversed = traversed.wrapping_add(1);
        x = next;
    }
    return removed;
}
#[no_mangle]
pub unsafe extern "C" fn zslGetRank(
    mut zsl: *mut zskiplist,
    mut score: libc::c_double,
    mut ele: sds,
) -> libc::c_ulong {
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut rank: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_int = 0;
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && ((*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score < score
                || (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).score
                    == score
                    && sdscmp(
                        (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).ele,
                        ele,
                    ) <= 0 as libc::c_int)
        {
            rank = rank
                .wrapping_add((*((*x).level).as_mut_ptr().offset(i as isize)).span);
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        if !((*x).ele).is_null() && (*x).score == score
            && sdscmp((*x).ele, ele) == 0 as libc::c_int
        {
            return rank;
        }
        i -= 1;
    }
    return 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn zslGetElementByRank(
    mut zsl: *mut zskiplist,
    mut rank: libc::c_ulong,
) -> *mut zskiplistNode {
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut traversed: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_int = 0;
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && traversed
                .wrapping_add((*((*x).level).as_mut_ptr().offset(i as isize)).span)
                <= rank
        {
            traversed = traversed
                .wrapping_add((*((*x).level).as_mut_ptr().offset(i as isize)).span);
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        if traversed == rank {
            return x;
        }
        i -= 1;
    }
    return 0 as *mut zskiplistNode;
}
unsafe extern "C" fn zslParseRange(
    mut min: *mut robj,
    mut max: *mut robj,
    mut spec: *mut zrangespec,
) -> libc::c_int {
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    (*spec).maxex = 0 as libc::c_int;
    (*spec).minex = (*spec).maxex;
    if (*min).encoding() as libc::c_int == 1 as libc::c_int {
        (*spec).min = (*min).ptr as libc::c_long as libc::c_double;
    } else if *((*min).ptr as *mut libc::c_char).offset(0 as libc::c_int as isize)
        as libc::c_int == '(' as i32
    {
        (*spec)
            .min = strtod(
            ((*min).ptr as *mut libc::c_char).offset(1 as libc::c_int as isize),
            &mut eptr,
        );
        if *eptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            || ((*spec).min).is_nan() as i32 != 0
        {
            return -(1 as libc::c_int);
        }
        (*spec).minex = 1 as libc::c_int;
    } else {
        (*spec).min = strtod((*min).ptr as *mut libc::c_char, &mut eptr);
        if *eptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            || ((*spec).min).is_nan() as i32 != 0
        {
            return -(1 as libc::c_int);
        }
    }
    if (*max).encoding() as libc::c_int == 1 as libc::c_int {
        (*spec).max = (*max).ptr as libc::c_long as libc::c_double;
    } else if *((*max).ptr as *mut libc::c_char).offset(0 as libc::c_int as isize)
        as libc::c_int == '(' as i32
    {
        (*spec)
            .max = strtod(
            ((*max).ptr as *mut libc::c_char).offset(1 as libc::c_int as isize),
            &mut eptr,
        );
        if *eptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            || ((*spec).max).is_nan() as i32 != 0
        {
            return -(1 as libc::c_int);
        }
        (*spec).maxex = 1 as libc::c_int;
    } else {
        (*spec).max = strtod((*max).ptr as *mut libc::c_char, &mut eptr);
        if *eptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            || ((*spec).max).is_nan() as i32 != 0
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zslParseLexRangeItem(
    mut item: *mut robj,
    mut dest: *mut sds,
    mut ex: *mut libc::c_int,
) -> libc::c_int {
    let mut c: *mut libc::c_char = (*item).ptr as *mut libc::c_char;
    match *c.offset(0 as libc::c_int as isize) as libc::c_int {
        43 => {
            if *c.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                return -(1 as libc::c_int);
            }
            *ex = 1 as libc::c_int;
            *dest = shared.maxstring;
            return 0 as libc::c_int;
        }
        45 => {
            if *c.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                return -(1 as libc::c_int);
            }
            *ex = 1 as libc::c_int;
            *dest = shared.minstring;
            return 0 as libc::c_int;
        }
        40 => {
            *ex = 1 as libc::c_int;
            *dest = sdsnewlen(
                c.offset(1 as libc::c_int as isize) as *const libc::c_void,
                (sdslen(c)).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            return 0 as libc::c_int;
        }
        91 => {
            *ex = 0 as libc::c_int;
            *dest = sdsnewlen(
                c.offset(1 as libc::c_int as isize) as *const libc::c_void,
                (sdslen(c)).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            return 0 as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn zslFreeLexRange(mut spec: *mut zlexrangespec) {
    if (*spec).min != shared.minstring && (*spec).min != shared.maxstring {
        sdsfree((*spec).min);
    }
    if (*spec).max != shared.minstring && (*spec).max != shared.maxstring {
        sdsfree((*spec).max);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zslParseLexRange(
    mut min: *mut robj,
    mut max: *mut robj,
    mut spec: *mut zlexrangespec,
) -> libc::c_int {
    if (*min).encoding() as libc::c_int == 1 as libc::c_int
        || (*max).encoding() as libc::c_int == 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*spec).max = 0 as sds;
    (*spec).min = (*spec).max;
    if zslParseLexRangeItem(min, &mut (*spec).min, &mut (*spec).minex)
        == -(1 as libc::c_int)
        || zslParseLexRangeItem(max, &mut (*spec).max, &mut (*spec).maxex)
            == -(1 as libc::c_int)
    {
        zslFreeLexRange(spec);
        return -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn sdscmplex(mut a: sds, mut b: sds) -> libc::c_int {
    if a == b {
        return 0 as libc::c_int;
    }
    if a == shared.minstring || b == shared.maxstring {
        return -(1 as libc::c_int);
    }
    if a == shared.maxstring || b == shared.minstring {
        return 1 as libc::c_int;
    }
    return sdscmp(a, b);
}
#[no_mangle]
pub unsafe extern "C" fn zslLexValueGteMin(
    mut value: sds,
    mut spec: *mut zlexrangespec,
) -> libc::c_int {
    return if (*spec).minex != 0 {
        (sdscmplex(value, (*spec).min) > 0 as libc::c_int) as libc::c_int
    } else {
        (sdscmplex(value, (*spec).min) >= 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn zslLexValueLteMax(
    mut value: sds,
    mut spec: *mut zlexrangespec,
) -> libc::c_int {
    return if (*spec).maxex != 0 {
        (sdscmplex(value, (*spec).max) < 0 as libc::c_int) as libc::c_int
    } else {
        (sdscmplex(value, (*spec).max) <= 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn zslIsInLexRange(
    mut zsl: *mut zskiplist,
    mut range: *mut zlexrangespec,
) -> libc::c_int {
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut cmp: libc::c_int = sdscmplex((*range).min, (*range).max);
    if cmp > 0 as libc::c_int
        || cmp == 0 as libc::c_int && ((*range).minex != 0 || (*range).maxex != 0)
    {
        return 0 as libc::c_int;
    }
    x = (*zsl).tail;
    if x.is_null() || zslLexValueGteMin((*x).ele, range) == 0 {
        return 0 as libc::c_int;
    }
    x = (*((*(*zsl).header).level).as_mut_ptr().offset(0 as libc::c_int as isize))
        .forward;
    if x.is_null() || zslLexValueLteMax((*x).ele, range) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zslFirstInLexRange(
    mut zsl: *mut zskiplist,
    mut range: *mut zlexrangespec,
) -> *mut zskiplistNode {
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut i: libc::c_int = 0;
    if zslIsInLexRange(zsl, range) == 0 {
        return 0 as *mut zskiplistNode;
    }
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && zslLexValueGteMin(
                (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).ele,
                range,
            ) == 0
        {
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        i -= 1;
    }
    x = (*((*x).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
    if !x.is_null() {} else {
        _serverAssert(
            b"x != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            686 as libc::c_int,
        );
        unreachable!();
    };
    if zslLexValueLteMax((*x).ele, range) == 0 {
        return 0 as *mut zskiplistNode;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn zslLastInLexRange(
    mut zsl: *mut zskiplist,
    mut range: *mut zlexrangespec,
) -> *mut zskiplistNode {
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut i: libc::c_int = 0;
    if zslIsInLexRange(zsl, range) == 0 {
        return 0 as *mut zskiplistNode;
    }
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && zslLexValueLteMax(
                (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).ele,
                range,
            ) != 0
        {
            x = (*((*x).level).as_mut_ptr().offset(i as isize)).forward;
        }
        i -= 1;
    }
    if !x.is_null() {} else {
        _serverAssert(
            b"x != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            711 as libc::c_int,
        );
        unreachable!();
    };
    if zslLexValueGteMin((*x).ele, range) == 0 {
        return 0 as *mut zskiplistNode;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn zzlStrtod(
    mut vstr: *mut libc::c_uchar,
    mut vlen: libc::c_uint,
) -> libc::c_double {
    let mut buf: [libc::c_char; 128] = [0; 128];
    if vlen as libc::c_ulong
        > (core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        vlen = (core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
    }
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        vstr as *const libc::c_void,
        vlen as libc::c_ulong,
    );
    buf[vlen as usize] = '\0' as i32 as libc::c_char;
    return strtod(buf.as_mut_ptr(), 0 as *mut *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn zzlGetScore(mut sptr: *mut libc::c_uchar) -> libc::c_double {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = 0;
    let mut vlong: libc::c_longlong = 0;
    let mut score: libc::c_double = 0.;
    if !sptr.is_null() {} else {
        _serverAssert(
            b"sptr != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            737 as libc::c_int,
        );
        unreachable!();
    };
    vstr = lpGetValue(sptr, &mut vlen, &mut vlong);
    if !vstr.is_null() {
        score = zzlStrtod(vstr, vlen);
    } else {
        score = vlong as libc::c_double;
    }
    return score;
}
#[no_mangle]
pub unsafe extern "C" fn lpGetObject(mut sptr: *mut libc::c_uchar) -> sds {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = 0;
    let mut vlong: libc::c_longlong = 0;
    if !sptr.is_null() {} else {
        _serverAssert(
            b"sptr != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            755 as libc::c_int,
        );
        unreachable!();
    };
    vstr = lpGetValue(sptr, &mut vlen, &mut vlong);
    if !vstr.is_null() {
        return sdsnewlen(
            vstr as *mut libc::c_char as *const libc::c_void,
            vlen as size_t,
        )
    } else {
        return sdsfromlonglong(vlong)
    };
}
#[no_mangle]
pub unsafe extern "C" fn zzlCompareElements(
    mut eptr: *mut libc::c_uchar,
    mut cstr: *mut libc::c_uchar,
    mut clen: libc::c_uint,
) -> libc::c_int {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = 0;
    let mut vlong: libc::c_longlong = 0;
    let mut vbuf: [libc::c_uchar; 32] = [0; 32];
    let mut minlen: libc::c_int = 0;
    let mut cmp: libc::c_int = 0;
    vstr = lpGetValue(eptr, &mut vlen, &mut vlong);
    if vstr.is_null() {
        vlen = ll2string(
            vbuf.as_mut_ptr() as *mut libc::c_char,
            core::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
            vlong,
        ) as libc::c_uint;
        vstr = vbuf.as_mut_ptr();
    }
    minlen = (if vlen < clen { vlen } else { clen }) as libc::c_int;
    cmp = memcmp(
        vstr as *const libc::c_void,
        cstr as *const libc::c_void,
        minlen as libc::c_ulong,
    );
    if cmp == 0 as libc::c_int {
        return vlen.wrapping_sub(clen) as libc::c_int;
    }
    return cmp;
}
#[no_mangle]
pub unsafe extern "C" fn zzlLength(mut zl: *mut libc::c_uchar) -> libc::c_uint {
    return (lpLength(zl)).wrapping_div(2 as libc::c_int as libc::c_ulong)
        as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn zzlNext(
    mut zl: *mut libc::c_uchar,
    mut eptr: *mut *mut libc::c_uchar,
    mut sptr: *mut *mut libc::c_uchar,
) {
    let mut _eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut _sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !(*eptr).is_null() && !(*sptr).is_null() {} else {
        _serverAssert(
            b"*eptr != NULL && *sptr != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            794 as libc::c_int,
        );
        unreachable!();
    };
    _eptr = lpNext(zl, *sptr);
    if !_eptr.is_null() {
        _sptr = lpNext(zl, _eptr);
        if !_sptr.is_null() {} else {
            _serverAssert(
                b"_sptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                799 as libc::c_int,
            );
            unreachable!();
        };
    } else {
        _sptr = 0 as *mut libc::c_uchar;
    }
    *eptr = _eptr;
    *sptr = _sptr;
}
#[no_mangle]
pub unsafe extern "C" fn zzlPrev(
    mut zl: *mut libc::c_uchar,
    mut eptr: *mut *mut libc::c_uchar,
    mut sptr: *mut *mut libc::c_uchar,
) {
    let mut _eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut _sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !(*eptr).is_null() && !(*sptr).is_null() {} else {
        _serverAssert(
            b"*eptr != NULL && *sptr != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            813 as libc::c_int,
        );
        unreachable!();
    };
    _sptr = lpPrev(zl, *eptr);
    if !_sptr.is_null() {
        _eptr = lpPrev(zl, _sptr);
        if !_eptr.is_null() {} else {
            _serverAssert(
                b"_eptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                818 as libc::c_int,
            );
            unreachable!();
        };
    } else {
        _eptr = 0 as *mut libc::c_uchar;
    }
    *eptr = _eptr;
    *sptr = _sptr;
}
#[no_mangle]
pub unsafe extern "C" fn zzlIsInRange(
    mut zl: *mut libc::c_uchar,
    mut range: *mut zrangespec,
) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut score: libc::c_double = 0.;
    if (*range).min > (*range).max
        || (*range).min == (*range).max && ((*range).minex != 0 || (*range).maxex != 0)
    {
        return 0 as libc::c_int;
    }
    p = lpSeek(zl, -(1 as libc::c_int) as libc::c_long);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    score = zzlGetScore(p);
    if zslValueGteMin(score, range) == 0 {
        return 0 as libc::c_int;
    }
    p = lpSeek(zl, 1 as libc::c_int as libc::c_long);
    if !p.is_null() {} else {
        _serverAssert(
            b"p != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            846 as libc::c_int,
        );
        unreachable!();
    };
    score = zzlGetScore(p);
    if zslValueLteMax(score, range) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zzlFirstInRange(
    mut zl: *mut libc::c_uchar,
    mut range: *mut zrangespec,
) -> *mut libc::c_uchar {
    let mut eptr: *mut libc::c_uchar = lpSeek(zl, 0 as libc::c_int as libc::c_long);
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut score: libc::c_double = 0.;
    if zzlIsInRange(zl, range) == 0 {
        return 0 as *mut libc::c_uchar;
    }
    while !eptr.is_null() {
        sptr = lpNext(zl, eptr);
        if !sptr.is_null() {} else {
            _serverAssert(
                b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                865 as libc::c_int,
            );
            unreachable!();
        };
        score = zzlGetScore(sptr);
        if zslValueGteMin(score, range) != 0 {
            if zslValueLteMax(score, range) != 0 {
                return eptr;
            }
            return 0 as *mut libc::c_uchar;
        }
        eptr = lpNext(zl, sptr);
    }
    return 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn zzlLastInRange(
    mut zl: *mut libc::c_uchar,
    mut range: *mut zrangespec,
) -> *mut libc::c_uchar {
    let mut eptr: *mut libc::c_uchar = lpSeek(zl, -(2 as libc::c_int) as libc::c_long);
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut score: libc::c_double = 0.;
    if zzlIsInRange(zl, range) == 0 {
        return 0 as *mut libc::c_uchar;
    }
    while !eptr.is_null() {
        sptr = lpNext(zl, eptr);
        if !sptr.is_null() {} else {
            _serverAssert(
                b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                893 as libc::c_int,
            );
            unreachable!();
        };
        score = zzlGetScore(sptr);
        if zslValueLteMax(score, range) != 0 {
            if zslValueGteMin(score, range) != 0 {
                return eptr;
            }
            return 0 as *mut libc::c_uchar;
        }
        sptr = lpPrev(zl, eptr);
        if !sptr.is_null() {
            eptr = lpPrev(zl, sptr);
            if !eptr.is_null() {} else {
                _serverAssert(
                    b"(eptr = lpPrev(zl,sptr)) != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    907 as libc::c_int,
                );
                unreachable!();
            };
        } else {
            eptr = 0 as *mut libc::c_uchar;
        }
    }
    return 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn zzlLexValueGteMin(
    mut p: *mut libc::c_uchar,
    mut spec: *mut zlexrangespec,
) -> libc::c_int {
    let mut value: sds = lpGetObject(p);
    let mut res: libc::c_int = zslLexValueGteMin(value, spec);
    sdsfree(value);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn zzlLexValueLteMax(
    mut p: *mut libc::c_uchar,
    mut spec: *mut zlexrangespec,
) -> libc::c_int {
    let mut value: sds = lpGetObject(p);
    let mut res: libc::c_int = zslLexValueLteMax(value, spec);
    sdsfree(value);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn zzlIsInLexRange(
    mut zl: *mut libc::c_uchar,
    mut range: *mut zlexrangespec,
) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cmp: libc::c_int = sdscmplex((*range).min, (*range).max);
    if cmp > 0 as libc::c_int
        || cmp == 0 as libc::c_int && ((*range).minex != 0 || (*range).maxex != 0)
    {
        return 0 as libc::c_int;
    }
    p = lpSeek(zl, -(2 as libc::c_int) as libc::c_long);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    if zzlLexValueGteMin(p, range) == 0 {
        return 0 as libc::c_int;
    }
    p = lpSeek(zl, 0 as libc::c_int as libc::c_long);
    if !p.is_null() {} else {
        _serverAssert(
            b"p != NULL\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            945 as libc::c_int,
        );
        unreachable!();
    };
    if zzlLexValueLteMax(p, range) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zzlFirstInLexRange(
    mut zl: *mut libc::c_uchar,
    mut range: *mut zlexrangespec,
) -> *mut libc::c_uchar {
    let mut eptr: *mut libc::c_uchar = lpSeek(zl, 0 as libc::c_int as libc::c_long);
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if zzlIsInLexRange(zl, range) == 0 {
        return 0 as *mut libc::c_uchar;
    }
    while !eptr.is_null() {
        if zzlLexValueGteMin(eptr, range) != 0 {
            if zzlLexValueLteMax(eptr, range) != 0 {
                return eptr;
            }
            return 0 as *mut libc::c_uchar;
        }
        sptr = lpNext(zl, eptr);
        if !sptr.is_null() {} else {
            _serverAssert(
                b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                970 as libc::c_int,
            );
            unreachable!();
        };
        eptr = lpNext(zl, sptr);
    }
    return 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn zzlLastInLexRange(
    mut zl: *mut libc::c_uchar,
    mut range: *mut zlexrangespec,
) -> *mut libc::c_uchar {
    let mut eptr: *mut libc::c_uchar = lpSeek(zl, -(2 as libc::c_int) as libc::c_long);
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if zzlIsInLexRange(zl, range) == 0 {
        return 0 as *mut libc::c_uchar;
    }
    while !eptr.is_null() {
        if zzlLexValueLteMax(eptr, range) != 0 {
            if zzlLexValueGteMin(eptr, range) != 0 {
                return eptr;
            }
            return 0 as *mut libc::c_uchar;
        }
        sptr = lpPrev(zl, eptr);
        if !sptr.is_null() {
            eptr = lpPrev(zl, sptr);
            if !eptr.is_null() {} else {
                _serverAssert(
                    b"(eptr = lpPrev(zl,sptr)) != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    997 as libc::c_int,
                );
                unreachable!();
            };
        } else {
            eptr = 0 as *mut libc::c_uchar;
        }
    }
    return 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn zzlFind(
    mut lp: *mut libc::c_uchar,
    mut ele: sds,
    mut score: *mut libc::c_double,
) -> *mut libc::c_uchar {
    let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    eptr = lpFirst(lp);
    if eptr.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    eptr = lpFind(
        lp,
        eptr,
        ele as *mut libc::c_uchar,
        sdslen(ele) as uint32_t,
        1 as libc::c_int as libc::c_uint,
    );
    if !eptr.is_null() {
        sptr = lpNext(lp, eptr);
        if !sptr.is_null() {} else {
            _serverAssert(
                b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                1012 as libc::c_int,
            );
            unreachable!();
        };
        if !score.is_null() {
            *score = zzlGetScore(sptr);
        }
        return eptr;
    }
    return 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn zzlDelete(
    mut zl: *mut libc::c_uchar,
    mut eptr: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    return lpDeleteRangeWithEntry(zl, &mut eptr, 2 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn zzlInsertAt(
    mut zl: *mut libc::c_uchar,
    mut eptr: *mut libc::c_uchar,
    mut ele: sds,
    mut score: libc::c_double,
) -> *mut libc::c_uchar {
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut scorebuf: [libc::c_char; 128] = [0; 128];
    let mut scorelen: libc::c_int = 0;
    let mut lscore: libc::c_longlong = 0;
    let mut score_is_long: libc::c_int = double2ll(score, &mut lscore);
    if score_is_long == 0 {
        scorelen = d2string(
            scorebuf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            score,
        );
    }
    if eptr.is_null() {
        zl = lpAppend(zl, ele as *mut libc::c_uchar, sdslen(ele) as uint32_t);
        if score_is_long != 0 {
            zl = lpAppendInteger(zl, lscore);
        } else {
            zl = lpAppend(
                zl,
                scorebuf.as_mut_ptr() as *mut libc::c_uchar,
                scorelen as uint32_t,
            );
        }
    } else {
        zl = lpInsertString(
            zl,
            ele as *mut libc::c_uchar,
            sdslen(ele) as uint32_t,
            eptr,
            0 as libc::c_int,
            &mut sptr,
        );
        if score_is_long != 0 {
            zl = lpInsertInteger(
                zl,
                lscore,
                sptr,
                1 as libc::c_int,
                0 as *mut *mut libc::c_uchar,
            );
        } else {
            zl = lpInsertString(
                zl,
                scorebuf.as_mut_ptr() as *mut libc::c_uchar,
                scorelen as uint32_t,
                sptr,
                1 as libc::c_int,
                0 as *mut *mut libc::c_uchar,
            );
        }
    }
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn zzlInsert(
    mut zl: *mut libc::c_uchar,
    mut ele: sds,
    mut score: libc::c_double,
) -> *mut libc::c_uchar {
    let mut eptr: *mut libc::c_uchar = lpSeek(zl, 0 as libc::c_int as libc::c_long);
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut s: libc::c_double = 0.;
    while !eptr.is_null() {
        sptr = lpNext(zl, eptr);
        if !sptr.is_null() {} else {
            _serverAssert(
                b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                1063 as libc::c_int,
            );
            unreachable!();
        };
        s = zzlGetScore(sptr);
        if s > score {
            zl = zzlInsertAt(zl, eptr, ele, score);
            break;
        } else {
            if s == score {
                if zzlCompareElements(
                    eptr,
                    ele as *mut libc::c_uchar,
                    sdslen(ele) as libc::c_uint,
                ) > 0 as libc::c_int
                {
                    zl = zzlInsertAt(zl, eptr, ele, score);
                    break;
                }
            }
            eptr = lpNext(zl, sptr);
        }
    }
    if eptr.is_null() {
        zl = zzlInsertAt(zl, 0 as *mut libc::c_uchar, ele, score);
    }
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn zzlDeleteRangeByScore(
    mut zl: *mut libc::c_uchar,
    mut range: *mut zrangespec,
    mut deleted: *mut libc::c_ulong,
) -> *mut libc::c_uchar {
    let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut score: libc::c_double = 0.;
    let mut num: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if !deleted.is_null() {
        *deleted = 0 as libc::c_int as libc::c_ulong;
    }
    eptr = zzlFirstInRange(zl, range);
    if eptr.is_null() {
        return zl;
    }
    while !eptr.is_null()
        && {
            sptr = lpNext(zl, eptr);
            !sptr.is_null()
        }
    {
        score = zzlGetScore(sptr);
        if !(zslValueLteMax(score, range) != 0) {
            break;
        }
        zl = lpDeleteRangeWithEntry(zl, &mut eptr, 2 as libc::c_int as libc::c_ulong);
        num = num.wrapping_add(1);
    }
    if !deleted.is_null() {
        *deleted = num;
    }
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn zzlDeleteRangeByLex(
    mut zl: *mut libc::c_uchar,
    mut range: *mut zlexrangespec,
    mut deleted: *mut libc::c_ulong,
) -> *mut libc::c_uchar {
    let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut num: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if !deleted.is_null() {
        *deleted = 0 as libc::c_int as libc::c_ulong;
    }
    eptr = zzlFirstInLexRange(zl, range);
    if eptr.is_null() {
        return zl;
    }
    while !eptr.is_null()
        && {
            sptr = lpNext(zl, eptr);
            !sptr.is_null()
        }
    {
        if !(zzlLexValueLteMax(eptr, range) != 0) {
            break;
        }
        zl = lpDeleteRangeWithEntry(zl, &mut eptr, 2 as libc::c_int as libc::c_ulong);
        num = num.wrapping_add(1);
    }
    if !deleted.is_null() {
        *deleted = num;
    }
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn zzlDeleteRangeByRank(
    mut zl: *mut libc::c_uchar,
    mut start: libc::c_uint,
    mut end: libc::c_uint,
    mut deleted: *mut libc::c_ulong,
) -> *mut libc::c_uchar {
    let mut num: libc::c_uint = end
        .wrapping_sub(start)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    if !deleted.is_null() {
        *deleted = num as libc::c_ulong;
    }
    zl = lpDeleteRange(
        zl,
        (2 as libc::c_int as libc::c_uint)
            .wrapping_mul(start.wrapping_sub(1 as libc::c_int as libc::c_uint))
            as libc::c_long,
        (2 as libc::c_int as libc::c_uint).wrapping_mul(num) as libc::c_ulong,
    );
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn zsetLength(mut zobj: *const robj) -> libc::c_ulong {
    let mut length: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        length = zzlLength((*zobj).ptr as *mut libc::c_uchar) as libc::c_ulong;
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        length = (*(*((*zobj).ptr as *const zset)).zsl).length;
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1162 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn zsetConvert(mut zobj: *mut robj, mut encoding: libc::c_int) {
    let mut zs: *mut zset = 0 as *mut zset;
    let mut node: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut next: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut ele: sds = 0 as *mut libc::c_char;
    let mut score: libc::c_double = 0.;
    if (*zobj).encoding() as libc::c_int == encoding {
        return;
    }
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = 0;
        let mut vlong: libc::c_longlong = 0;
        if encoding != 7 as libc::c_int {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                1182 as libc::c_int,
                b"Unknown target encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
        zs = zmalloc(core::mem::size_of::<zset>() as libc::c_ulong) as *mut zset;
        (*zs).dict = dictCreate(&mut zsetDictType);
        (*zs).zsl = zslCreate();
        eptr = lpSeek(zl, 0 as libc::c_int as libc::c_long);
        if !eptr.is_null() {
            sptr = lpNext(zl, eptr);
            if !sptr.is_null() {} else {
                _serverAssertWithInfo(
                    0 as *const client,
                    zobj,
                    b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    1191 as libc::c_int,
                );
                unreachable!();
            };
        }
        while !eptr.is_null() {
            score = zzlGetScore(sptr);
            vstr = lpGetValue(eptr, &mut vlen, &mut vlong);
            if vstr.is_null() {
                ele = sdsfromlonglong(vlong);
            } else {
                ele = sdsnewlen(
                    vstr as *mut libc::c_char as *const libc::c_void,
                    vlen as size_t,
                );
            }
            node = zslInsert((*zs).zsl, score, ele);
            if dictAdd(
                (*zs).dict,
                ele as *mut libc::c_void,
                &mut (*node).score as *mut libc::c_double as *mut libc::c_void,
            ) == 0 as libc::c_int
            {} else {
                _serverAssert(
                    b"dictAdd(zs->dict,ele,&node->score) == DICT_OK\0" as *const u8
                        as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    1203 as libc::c_int,
                );
                unreachable!();
            };
            zzlNext(zl, &mut eptr, &mut sptr);
        }
        zfree((*zobj).ptr);
        (*zobj).ptr = zs as *mut libc::c_void;
        (*zobj).set_encoding(7 as libc::c_int as libc::c_uint);
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zl_0: *mut libc::c_uchar = lpNew(0 as libc::c_int as size_t);
        if encoding != 11 as libc::c_int {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                1214 as libc::c_int,
                b"Unknown target encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
        zs = (*zobj).ptr as *mut zset;
        dictRelease((*zs).dict);
        node = (*((*(*(*zs).zsl).header).level)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .forward;
        zfree((*(*zs).zsl).header as *mut libc::c_void);
        zfree((*zs).zsl as *mut libc::c_void);
        while !node.is_null() {
            zl_0 = zzlInsertAt(
                zl_0,
                0 as *mut libc::c_uchar,
                (*node).ele,
                (*node).score,
            );
            next = (*((*node).level).as_mut_ptr().offset(0 as libc::c_int as isize))
                .forward;
            zslFreeNode(node);
            node = next;
        }
        zfree(zs as *mut libc::c_void);
        (*zobj).ptr = zl_0 as *mut libc::c_void;
        (*zobj).set_encoding(11 as libc::c_int as libc::c_uint);
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1235 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn zsetConvertToListpackIfNeeded(
    mut zobj: *mut robj,
    mut maxelelen: size_t,
    mut totelelen: size_t,
) {
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        return;
    }
    let mut zset: *mut zset = (*zobj).ptr as *mut zset;
    if (*(*zset).zsl).length <= server.zset_max_listpack_entries
        && maxelelen <= server.zset_max_listpack_value
        && lpSafeToAdd(0 as *mut libc::c_uchar, totelelen) != 0
    {
        zsetConvert(zobj, 11 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zsetScore(
    mut zobj: *mut robj,
    mut member: sds,
    mut score: *mut libc::c_double,
) -> libc::c_int {
    if zobj.is_null() || member.is_null() {
        return -(1 as libc::c_int);
    }
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        if (zzlFind((*zobj).ptr as *mut libc::c_uchar, member, score)).is_null() {
            return -(1 as libc::c_int);
        }
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        let mut de: *mut dictEntry = dictFind((*zs).dict, member as *const libc::c_void);
        if de.is_null() {
            return -(1 as libc::c_int);
        }
        *score = *((*de).v.val as *mut libc::c_double);
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1269 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsetAdd(
    mut zobj: *mut robj,
    mut score: libc::c_double,
    mut ele: sds,
    mut in_flags: libc::c_int,
    mut out_flags: *mut libc::c_int,
    mut newscore: *mut libc::c_double,
) -> libc::c_int {
    let mut incr: libc::c_int = (in_flags & (1 as libc::c_int) << 0 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    let mut nx: libc::c_int = (in_flags & (1 as libc::c_int) << 1 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    let mut xx: libc::c_int = (in_flags & (1 as libc::c_int) << 2 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    let mut gt: libc::c_int = (in_flags & (1 as libc::c_int) << 3 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    let mut lt: libc::c_int = (in_flags & (1 as libc::c_int) << 4 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    *out_flags = 0 as libc::c_int;
    let mut curscore: libc::c_double = 0.;
    if score.is_nan() as i32 != 0 {
        *out_flags = (1 as libc::c_int) << 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        eptr = zzlFind((*zobj).ptr as *mut libc::c_uchar, ele, &mut curscore);
        if !eptr.is_null() {
            if nx != 0 {
                *out_flags |= (1 as libc::c_int) << 0 as libc::c_int;
                return 1 as libc::c_int;
            }
            if incr != 0 {
                score += curscore;
                if score.is_nan() as i32 != 0 {
                    *out_flags |= (1 as libc::c_int) << 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
            }
            if lt != 0 && score >= curscore || gt != 0 && score <= curscore {
                *out_flags |= (1 as libc::c_int) << 0 as libc::c_int;
                return 1 as libc::c_int;
            }
            if !newscore.is_null() {
                *newscore = score;
            }
            if score != curscore {
                (*zobj)
                    .ptr = zzlDelete((*zobj).ptr as *mut libc::c_uchar, eptr)
                    as *mut libc::c_void;
                (*zobj)
                    .ptr = zzlInsert((*zobj).ptr as *mut libc::c_uchar, ele, score)
                    as *mut libc::c_void;
                *out_flags |= (1 as libc::c_int) << 3 as libc::c_int;
            }
            return 1 as libc::c_int;
        } else {
            if xx == 0 {
                if (zzlLength((*zobj).ptr as *mut libc::c_uchar))
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong
                    > server.zset_max_listpack_entries
                    || sdslen(ele) > server.zset_max_listpack_value
                    || lpSafeToAdd((*zobj).ptr as *mut libc::c_uchar, sdslen(ele)) == 0
                {
                    zsetConvert(zobj, 7 as libc::c_int);
                } else {
                    (*zobj)
                        .ptr = zzlInsert((*zobj).ptr as *mut libc::c_uchar, ele, score)
                        as *mut libc::c_void;
                    if !newscore.is_null() {
                        *newscore = score;
                    }
                    *out_flags |= (1 as libc::c_int) << 2 as libc::c_int;
                    return 1 as libc::c_int;
                }
            } else {
                *out_flags |= (1 as libc::c_int) << 0 as libc::c_int;
                return 1 as libc::c_int;
            }
        }
    }
    if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        let mut znode: *mut zskiplistNode = 0 as *mut zskiplistNode;
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        de = dictFind((*zs).dict, ele as *const libc::c_void);
        if !de.is_null() {
            if nx != 0 {
                *out_flags |= (1 as libc::c_int) << 0 as libc::c_int;
                return 1 as libc::c_int;
            }
            curscore = *((*de).v.val as *mut libc::c_double);
            if incr != 0 {
                score += curscore;
                if score.is_nan() as i32 != 0 {
                    *out_flags |= (1 as libc::c_int) << 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
            }
            if lt != 0 && score >= curscore || gt != 0 && score <= curscore {
                *out_flags |= (1 as libc::c_int) << 0 as libc::c_int;
                return 1 as libc::c_int;
            }
            if !newscore.is_null() {
                *newscore = score;
            }
            if score != curscore {
                znode = zslUpdateScore((*zs).zsl, curscore, ele, score);
                (*de)
                    .v
                    .val = &mut (*znode).score as *mut libc::c_double
                    as *mut libc::c_void;
                *out_flags |= (1 as libc::c_int) << 3 as libc::c_int;
            }
            return 1 as libc::c_int;
        } else if xx == 0 {
            ele = sdsdup(ele);
            znode = zslInsert((*zs).zsl, score, ele);
            if dictAdd(
                (*zs).dict,
                ele as *mut libc::c_void,
                &mut (*znode).score as *mut libc::c_double as *mut libc::c_void,
            ) == 0 as libc::c_int
            {} else {
                _serverAssert(
                    b"dictAdd(zs->dict,ele,&znode->score) == DICT_OK\0" as *const u8
                        as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    1437 as libc::c_int,
                );
                unreachable!();
            };
            *out_flags |= (1 as libc::c_int) << 2 as libc::c_int;
            if !newscore.is_null() {
                *newscore = score;
            }
            return 1 as libc::c_int;
        } else {
            *out_flags |= (1 as libc::c_int) << 0 as libc::c_int;
            return 1 as libc::c_int;
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1446 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zsetRemoveFromSkiplist(
    mut zs: *mut zset,
    mut ele: sds,
) -> libc::c_int {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut score: libc::c_double = 0.;
    de = dictUnlink((*zs).dict, ele as *const libc::c_void);
    if !de.is_null() {
        score = *((*de).v.val as *mut libc::c_double);
        dictFreeUnlinkedEntry((*zs).dict, de);
        let mut retval: libc::c_int = zslDelete(
            (*zs).zsl,
            score,
            ele,
            0 as *mut *mut zskiplistNode,
        );
        if retval != 0 {} else {
            _serverAssert(
                b"retval\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                1473 as libc::c_int,
            );
            unreachable!();
        };
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsetDel(mut zobj: *mut robj, mut ele: sds) -> libc::c_int {
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        eptr = zzlFind((*zobj).ptr as *mut libc::c_uchar, ele, 0 as *mut libc::c_double);
        if !eptr.is_null() {
            (*zobj)
                .ptr = zzlDelete((*zobj).ptr as *mut libc::c_uchar, eptr)
                as *mut libc::c_void;
            return 1 as libc::c_int;
        }
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        if zsetRemoveFromSkiplist(zs, ele) != 0 {
            if htNeedsResize((*zs).dict) != 0 {
                dictResize((*zs).dict);
            }
            return 1 as libc::c_int;
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1498 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsetRank(
    mut zobj: *mut robj,
    mut ele: sds,
    mut reverse: libc::c_int,
) -> libc::c_long {
    let mut llen: libc::c_ulong = 0;
    let mut rank: libc::c_ulong = 0;
    llen = zsetLength(zobj);
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        eptr = lpSeek(zl, 0 as libc::c_int as libc::c_long);
        if !eptr.is_null() {} else {
            _serverAssert(
                b"eptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                1525 as libc::c_int,
            );
            unreachable!();
        };
        sptr = lpNext(zl, eptr);
        if !sptr.is_null() {} else {
            _serverAssert(
                b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                1527 as libc::c_int,
            );
            unreachable!();
        };
        rank = 1 as libc::c_int as libc::c_ulong;
        while !eptr.is_null() {
            if lpCompare(eptr, ele as *mut libc::c_uchar, sdslen(ele) as uint32_t) != 0 {
                break;
            }
            rank = rank.wrapping_add(1);
            zzlNext(zl, &mut eptr, &mut sptr);
        }
        if !eptr.is_null() {
            if reverse != 0 {
                return llen.wrapping_sub(rank) as libc::c_long
            } else {
                return rank.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_long
            }
        } else {
            return -(1 as libc::c_int) as libc::c_long
        }
    } else {
        if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
            let mut zs: *mut zset = (*zobj).ptr as *mut zset;
            let mut zsl: *mut zskiplist = (*zs).zsl;
            let mut de: *mut dictEntry = 0 as *mut dictEntry;
            let mut score: libc::c_double = 0.;
            de = dictFind((*zs).dict, ele as *const libc::c_void);
            if !de.is_null() {
                score = *((*de).v.val as *mut libc::c_double);
                rank = zslGetRank(zsl, score, ele);
                if rank != 0 as libc::c_int as libc::c_ulong {} else {
                    _serverAssert(
                        b"rank != 0\0" as *const u8 as *const libc::c_char,
                        b"t_zset.c\0" as *const u8 as *const libc::c_char,
                        1556 as libc::c_int,
                    );
                    unreachable!();
                };
                if reverse != 0 {
                    return llen.wrapping_sub(rank) as libc::c_long
                } else {
                    return rank.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_long
                }
            } else {
                return -(1 as libc::c_int) as libc::c_long
            }
        } else {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                1565 as libc::c_int,
                b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn zsetDup(mut o: *mut robj) -> *mut robj {
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut zs: *mut zset = 0 as *mut zset;
    let mut new_zs: *mut zset = 0 as *mut zset;
    if (*o).type_0() as libc::c_int == 3 as libc::c_int {} else {
        _serverAssert(
            b"o->type == OBJ_ZSET\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1579 as libc::c_int,
        );
        unreachable!();
    };
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*o).ptr as *mut libc::c_uchar;
        let mut sz: size_t = lpBytes(zl);
        let mut new_zl: *mut libc::c_uchar = zmalloc(sz) as *mut libc::c_uchar;
        memcpy(new_zl as *mut libc::c_void, zl as *const libc::c_void, sz);
        zobj = createObject(3 as libc::c_int, new_zl as *mut libc::c_void);
        (*zobj).set_encoding(11 as libc::c_int as libc::c_uint);
    } else if (*o).encoding() as libc::c_int == 7 as libc::c_int {
        zobj = createZsetObject();
        zs = (*o).ptr as *mut zset;
        new_zs = (*zobj).ptr as *mut zset;
        dictExpand(
            (*new_zs).dict,
            ((*(*zs).dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*zs).dict).ht_used[1 as libc::c_int as usize]),
        );
        let mut zsl: *mut zskiplist = (*zs).zsl;
        let mut ln: *mut zskiplistNode = 0 as *mut zskiplistNode;
        let mut ele: sds = 0 as *mut libc::c_char;
        let mut llen: libc::c_long = zsetLength(o) as libc::c_long;
        ln = (*zsl).tail;
        loop {
            let fresh9 = llen;
            llen = llen - 1;
            if !(fresh9 != 0) {
                break;
            }
            ele = (*ln).ele;
            let mut new_ele: sds = sdsdup(ele);
            let mut znode: *mut zskiplistNode = zslInsert(
                (*new_zs).zsl,
                (*ln).score,
                new_ele,
            );
            dictAdd(
                (*new_zs).dict,
                new_ele as *mut libc::c_void,
                &mut (*znode).score as *mut libc::c_double as *mut libc::c_void,
            );
            ln = (*ln).backward;
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1614 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return zobj;
}
#[no_mangle]
pub unsafe extern "C" fn zsetSdsFromListpackEntry(mut e: *mut listpackEntry) -> sds {
    return if !((*e).sval).is_null() {
        sdsnewlen((*e).sval as *const libc::c_void, (*e).slen as size_t)
    } else {
        sdsfromlonglong((*e).lval)
    };
}
#[no_mangle]
pub unsafe extern "C" fn zsetReplyFromListpackEntry(
    mut c: *mut client,
    mut e: *mut listpackEntry,
) {
    if !((*e).sval).is_null() {
        addReplyBulkCBuffer(c, (*e).sval as *const libc::c_void, (*e).slen as size_t);
    } else {
        addReplyBulkLongLong(c, (*e).lval);
    };
}
#[no_mangle]
pub unsafe extern "C" fn zsetTypeRandomElement(
    mut zsetobj: *mut robj,
    mut zsetsize: libc::c_ulong,
    mut key: *mut listpackEntry,
    mut score: *mut libc::c_double,
) {
    if (*zsetobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zsetobj).ptr as *mut zset;
        let mut de: *mut dictEntry = dictGetFairRandomKey((*zs).dict);
        let mut s: sds = (*de).key as sds;
        (*key).sval = s as *mut libc::c_uchar;
        (*key).slen = sdslen(s) as uint32_t;
        if !score.is_null() {
            *score = *((*de).v.val as *mut libc::c_double);
        }
    } else if (*zsetobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut val: listpackEntry = listpackEntry {
            sval: 0 as *mut libc::c_uchar,
            slen: 0,
            lval: 0,
        };
        lpRandomPair((*zsetobj).ptr as *mut libc::c_uchar, zsetsize, key, &mut val);
        if !score.is_null() {
            if !(val.sval).is_null() {
                *score = zzlStrtod(val.sval, val.slen);
            } else {
                *score = val.lval as libc::c_double;
            }
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1657 as libc::c_int,
            b"Unknown zset encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn zaddGenericCommand(mut c: *mut client, mut flags: libc::c_int) {
    let mut current_block: u64;
    static mut nanerr: *mut libc::c_char = b"resulting score is not a number (NaN)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut ele: sds = 0 as *mut libc::c_char;
    let mut score: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut scores: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut j: libc::c_int = 0;
    let mut elements: libc::c_int = 0;
    let mut ch: libc::c_int = 0 as libc::c_int;
    let mut scoreidx: libc::c_int = 0 as libc::c_int;
    let mut added: libc::c_int = 0 as libc::c_int;
    let mut updated: libc::c_int = 0 as libc::c_int;
    let mut processed: libc::c_int = 0 as libc::c_int;
    scoreidx = 2 as libc::c_int;
    while scoreidx < (*c).argc {
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(scoreidx as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(opt, b"nx\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 1 as libc::c_int;
        } else if strcasecmp(opt, b"xx\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 2 as libc::c_int;
        } else if strcasecmp(opt, b"ch\0" as *const u8 as *const libc::c_char) == 0 {
            ch = 1 as libc::c_int;
        } else if strcasecmp(opt, b"incr\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 0 as libc::c_int;
        } else if strcasecmp(opt, b"gt\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 3 as libc::c_int;
        } else {
            if !(strcasecmp(opt, b"lt\0" as *const u8 as *const libc::c_char) == 0) {
                break;
            }
            flags |= (1 as libc::c_int) << 4 as libc::c_int;
        }
        scoreidx += 1;
    }
    let mut incr: libc::c_int = (flags & (1 as libc::c_int) << 0 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    let mut nx: libc::c_int = (flags & (1 as libc::c_int) << 1 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    let mut xx: libc::c_int = (flags & (1 as libc::c_int) << 2 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    let mut gt: libc::c_int = (flags & (1 as libc::c_int) << 3 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    let mut lt: libc::c_int = (flags & (1 as libc::c_int) << 4 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    elements = (*c).argc - scoreidx;
    if elements % 2 as libc::c_int != 0 || elements == 0 {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    elements /= 2 as libc::c_int;
    if nx != 0 && xx != 0 {
        addReplyError(
            c,
            b"XX and NX options at the same time are not compatible\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if gt != 0 && nx != 0 || lt != 0 && nx != 0 || gt != 0 && lt != 0 {
        addReplyError(
            c,
            b"GT, LT, and/or NX options at the same time are not compatible\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if incr != 0 && elements > 1 as libc::c_int {
        addReplyError(
            c,
            b"INCR option supports a single increment-element pair\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    scores = zmalloc(
        (core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(elements as libc::c_ulong),
    ) as *mut libc::c_double;
    j = 0 as libc::c_int;
    loop {
        if !(j < elements) {
            current_block = 9520865839495247062;
            break;
        }
        if getDoubleFromObjectOrReply(
            c,
            *((*c).argv).offset((scoreidx + j * 2 as libc::c_int) as isize),
            &mut *scores.offset(j as isize),
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            current_block = 1916227487867066896;
            break;
        }
        j += 1;
    }
    match current_block {
        9520865839495247062 => {
            zobj = lookupKeyWrite((*c).db, key);
            if !(checkType(c, zobj, 3 as libc::c_int) != 0) {
                if zobj.is_null() {
                    if xx != 0 {
                        current_block = 15219312158577706460;
                    } else {
                        if server.zset_max_listpack_entries
                            == 0 as libc::c_int as libc::c_ulong
                            || server.zset_max_listpack_value
                                < sdslen(
                                    (**((*c).argv)
                                        .offset((scoreidx + 1 as libc::c_int) as isize))
                                        .ptr as sds,
                                )
                        {
                            zobj = createZsetObject();
                        } else {
                            zobj = createZsetListpackObject();
                        }
                        dbAdd((*c).db, key, zobj);
                        current_block = 790185930182612747;
                    }
                } else {
                    current_block = 790185930182612747;
                }
                match current_block {
                    790185930182612747 => {
                        j = 0 as libc::c_int;
                        loop {
                            if !(j < elements) {
                                current_block = 10753070352654377903;
                                break;
                            }
                            let mut newscore: libc::c_double = 0.;
                            score = *scores.offset(j as isize);
                            let mut retflags: libc::c_int = 0 as libc::c_int;
                            ele = (**((*c).argv)
                                .offset(
                                    (scoreidx + 1 as libc::c_int + j * 2 as libc::c_int)
                                        as isize,
                                ))
                                .ptr as sds;
                            let mut retval: libc::c_int = zsetAdd(
                                zobj,
                                score,
                                ele,
                                flags,
                                &mut retflags,
                                &mut newscore,
                            );
                            if retval == 0 as libc::c_int {
                                addReplyError(c, nanerr);
                                current_block = 1916227487867066896;
                                break;
                            } else {
                                if retflags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                                    added += 1;
                                }
                                if retflags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                                    updated += 1;
                                }
                                if retflags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
                                    processed += 1;
                                }
                                score = newscore;
                                j += 1;
                            }
                        }
                        match current_block {
                            1916227487867066896 => {}
                            _ => {
                                server.dirty += (added + updated) as libc::c_longlong;
                                current_block = 15219312158577706460;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    1916227487867066896 => {}
                    _ => {
                        if incr != 0 {
                            if processed != 0 {
                                addReplyDouble(c, score);
                            } else {
                                addReplyNull(c);
                            }
                        } else {
                            addReplyLongLong(
                                c,
                                (if ch != 0 { added + updated } else { added })
                                    as libc::c_longlong,
                            );
                        }
                    }
                }
            }
        }
        _ => {}
    }
    zfree(scores as *mut libc::c_void);
    if added != 0 || updated != 0 {
        signalModifiedKey(c, (*c).db, key);
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 7 as libc::c_int,
            (if incr != 0 {
                b"zincr\0" as *const u8 as *const libc::c_char
            } else {
                b"zadd\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            key,
            (*(*c).db).id,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn zaddCommand(mut c: *mut client) {
    zaddGenericCommand(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zincrbyCommand(mut c: *mut client) {
    zaddGenericCommand(c, (1 as libc::c_int) << 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zremCommand(mut c: *mut client) {
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut deleted: libc::c_int = 0 as libc::c_int;
    let mut keyremoved: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    zobj = lookupKeyWriteOrReply(c, key, shared.czero);
    if zobj.is_null() || checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    j = 2 as libc::c_int;
    while j < (*c).argc {
        if zsetDel(zobj, (**((*c).argv).offset(j as isize)).ptr as sds) != 0 {
            deleted += 1;
        }
        if zsetLength(zobj) == 0 as libc::c_int as libc::c_ulong {
            dbDelete((*c).db, key);
            keyremoved = 1 as libc::c_int;
            break;
        } else {
            j += 1;
        }
    }
    if deleted != 0 {
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 7 as libc::c_int,
            b"zrem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key,
            (*(*c).db).id,
        );
        if keyremoved != 0 {
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 2 as libc::c_int,
                b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                key,
                (*(*c).db).id,
            );
        }
        signalModifiedKey(c, (*c).db, key);
        server.dirty += deleted as libc::c_longlong;
    }
    addReplyLongLong(c, deleted as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn zremrangeGenericCommand(
    mut c: *mut client,
    mut rangetype: zrange_type,
) {
    let mut current_block: u64;
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut keyremoved: libc::c_int = 0 as libc::c_int;
    let mut deleted: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut range: zrangespec = zrangespec {
        min: 0.,
        max: 0.,
        minex: 0,
        maxex: 0,
    };
    let mut lexrange: zlexrangespec = zlexrangespec {
        min: 0 as *mut libc::c_char,
        max: 0 as *mut libc::c_char,
        minex: 0,
        maxex: 0,
    };
    let mut start: libc::c_long = 0;
    let mut end: libc::c_long = 0;
    let mut llen: libc::c_long = 0;
    let mut notify_type: *mut libc::c_char = 0 as *mut libc::c_char;
    if rangetype as libc::c_uint == ZRANGE_RANK as libc::c_int as libc::c_uint {
        notify_type = b"zremrangebyrank\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        if getLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut start,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
            || getLongFromObjectOrReply(
                c,
                *((*c).argv).offset(3 as libc::c_int as isize),
                &mut end,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
        {
            return;
        }
    } else if rangetype as libc::c_uint == ZRANGE_SCORE as libc::c_int as libc::c_uint {
        notify_type = b"zremrangebyscore\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        if zslParseRange(
            *((*c).argv).offset(2 as libc::c_int as isize),
            *((*c).argv).offset(3 as libc::c_int as isize),
            &mut range,
        ) != 0 as libc::c_int
        {
            addReplyError(
                c,
                b"min or max is not a float\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    } else if rangetype as libc::c_uint == ZRANGE_LEX as libc::c_int as libc::c_uint {
        notify_type = b"zremrangebylex\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        if zslParseLexRange(
            *((*c).argv).offset(2 as libc::c_int as isize),
            *((*c).argv).offset(3 as libc::c_int as isize),
            &mut lexrange,
        ) != 0 as libc::c_int
        {
            addReplyError(
                c,
                b"min or max not valid string range item\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            1866 as libc::c_int,
            b"unknown rangetype %d\0" as *const u8 as *const libc::c_char,
            rangetype as libc::c_int,
        );
        unreachable!();
    }
    zobj = lookupKeyWriteOrReply(c, key, shared.czero);
    if !(zobj.is_null() || checkType(c, zobj, 3 as libc::c_int) != 0) {
        if rangetype as libc::c_uint == ZRANGE_RANK as libc::c_int as libc::c_uint {
            llen = zsetLength(zobj) as libc::c_long;
            if start < 0 as libc::c_int as libc::c_long {
                start = llen + start;
            }
            if end < 0 as libc::c_int as libc::c_long {
                end = llen + end;
            }
            if start < 0 as libc::c_int as libc::c_long {
                start = 0 as libc::c_int as libc::c_long;
            }
            if start > end || start >= llen {
                addReply(c, shared.czero);
                current_block = 1837061184733152610;
            } else {
                if end >= llen {
                    end = llen - 1 as libc::c_int as libc::c_long;
                }
                current_block = 11932355480408055363;
            }
        } else {
            current_block = 11932355480408055363;
        }
        match current_block {
            1837061184733152610 => {}
            _ => {
                if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
                    match rangetype as libc::c_uint {
                        0 | 1 => {
                            (*zobj)
                                .ptr = zzlDeleteRangeByRank(
                                (*zobj).ptr as *mut libc::c_uchar,
                                (start + 1 as libc::c_int as libc::c_long) as libc::c_uint,
                                (end + 1 as libc::c_int as libc::c_long) as libc::c_uint,
                                &mut deleted,
                            ) as *mut libc::c_void;
                        }
                        2 => {
                            (*zobj)
                                .ptr = zzlDeleteRangeByScore(
                                (*zobj).ptr as *mut libc::c_uchar,
                                &mut range,
                                &mut deleted,
                            ) as *mut libc::c_void;
                        }
                        3 => {
                            (*zobj)
                                .ptr = zzlDeleteRangeByLex(
                                (*zobj).ptr as *mut libc::c_uchar,
                                &mut lexrange,
                                &mut deleted,
                            ) as *mut libc::c_void;
                        }
                        _ => {}
                    }
                    if zzlLength((*zobj).ptr as *mut libc::c_uchar)
                        == 0 as libc::c_int as libc::c_uint
                    {
                        dbDelete((*c).db, key);
                        keyremoved = 1 as libc::c_int;
                    }
                } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
                    let mut zs: *mut zset = (*zobj).ptr as *mut zset;
                    match rangetype as libc::c_uint {
                        0 | 1 => {
                            deleted = zslDeleteRangeByRank(
                                (*zs).zsl,
                                (start + 1 as libc::c_int as libc::c_long) as libc::c_uint,
                                (end + 1 as libc::c_int as libc::c_long) as libc::c_uint,
                                (*zs).dict,
                            );
                        }
                        2 => {
                            deleted = zslDeleteRangeByScore(
                                (*zs).zsl,
                                &mut range,
                                (*zs).dict,
                            );
                        }
                        3 => {
                            deleted = zslDeleteRangeByLex(
                                (*zs).zsl,
                                &mut lexrange,
                                (*zs).dict,
                            );
                        }
                        _ => {}
                    }
                    if htNeedsResize((*zs).dict) != 0 {
                        dictResize((*zs).dict);
                    }
                    if ((*(*zs).dict).ht_used[0 as libc::c_int as usize])
                        .wrapping_add((*(*zs).dict).ht_used[1 as libc::c_int as usize])
                        == 0 as libc::c_int as libc::c_ulong
                    {
                        dbDelete((*c).db, key);
                        keyremoved = 1 as libc::c_int;
                    }
                } else {
                    _serverPanic(
                        b"t_zset.c\0" as *const u8 as *const libc::c_char,
                        1927 as libc::c_int,
                        b"Unknown sorted set encoding\0" as *const u8
                            as *const libc::c_char,
                    );
                    unreachable!();
                }
                if deleted != 0 {
                    signalModifiedKey(c, (*c).db, key);
                    notifyKeyspaceEvent(
                        (1 as libc::c_int) << 7 as libc::c_int,
                        notify_type,
                        key,
                        (*(*c).db).id,
                    );
                    if keyremoved != 0 {
                        notifyKeyspaceEvent(
                            (1 as libc::c_int) << 2 as libc::c_int,
                            b"del\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            key,
                            (*(*c).db).id,
                        );
                    }
                }
                server
                    .dirty = (server.dirty as libc::c_ulonglong)
                    .wrapping_add(deleted as libc::c_ulonglong) as libc::c_longlong
                    as libc::c_longlong;
                addReplyLongLong(c, deleted as libc::c_longlong);
            }
        }
    }
    if rangetype as libc::c_uint == ZRANGE_LEX as libc::c_int as libc::c_uint {
        zslFreeLexRange(&mut lexrange);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zremrangebyrankCommand(mut c: *mut client) {
    zremrangeGenericCommand(c, ZRANGE_RANK);
}
#[no_mangle]
pub unsafe extern "C" fn zremrangebyscoreCommand(mut c: *mut client) {
    zremrangeGenericCommand(c, ZRANGE_SCORE);
}
#[no_mangle]
pub unsafe extern "C" fn zremrangebylexCommand(mut c: *mut client) {
    zremrangeGenericCommand(c, ZRANGE_LEX);
}
#[no_mangle]
pub unsafe extern "C" fn zuiInitIterator(mut op: *mut zsetopsrc) {
    if ((*op).subject).is_null() {
        return;
    }
    if (*op).type_0 == 2 as libc::c_int {
        let mut it: *mut iterset = &mut (*op).iter.set;
        if (*op).encoding == 6 as libc::c_int {
            (*it).is.is = (*(*op).subject).ptr as *mut intset;
            (*it).is.ii = 0 as libc::c_int;
        } else if (*op).encoding == 2 as libc::c_int {
            (*it).ht.dict = (*(*op).subject).ptr as *mut dict;
            (*it).ht.di = dictGetIterator((*(*op).subject).ptr as *mut dict);
            (*it).ht.de = dictNext((*it).ht.di);
        } else {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                2029 as libc::c_int,
                b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*op).type_0 == 3 as libc::c_int {
        let mut it_0: *mut iterzset = &mut (*op).iter.zset;
        if (*op).encoding == 11 as libc::c_int {
            (*it_0).zl.zl = (*(*op).subject).ptr as *mut libc::c_uchar;
            (*it_0).zl.eptr = lpSeek((*it_0).zl.zl, -(2 as libc::c_int) as libc::c_long);
            if !((*it_0).zl.eptr).is_null() {
                (*it_0).zl.sptr = lpNext((*it_0).zl.zl, (*it_0).zl.eptr);
                if !((*it_0).zl.sptr).is_null() {} else {
                    _serverAssert(
                        b"it->zl.sptr != NULL\0" as *const u8 as *const libc::c_char,
                        b"t_zset.c\0" as *const u8 as *const libc::c_char,
                        2041 as libc::c_int,
                    );
                    unreachable!();
                };
            }
        } else if (*op).encoding == 7 as libc::c_int {
            (*it_0).sl.zs = (*(*op).subject).ptr as *mut zset;
            (*it_0).sl.node = (*(*(*it_0).sl.zs).zsl).tail;
        } else {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                2047 as libc::c_int,
                b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2050 as libc::c_int,
            b"Unsupported type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn zuiClearIterator(mut op: *mut zsetopsrc) {
    if ((*op).subject).is_null() {
        return;
    }
    if (*op).type_0 == 2 as libc::c_int {
        let mut it: *mut iterset = &mut (*op).iter.set;
        if !((*op).encoding == 6 as libc::c_int) {
            if (*op).encoding == 2 as libc::c_int {
                dictReleaseIterator((*it).ht.di);
            } else {
                _serverPanic(
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    2065 as libc::c_int,
                    b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
    } else if (*op).type_0 == 3 as libc::c_int {
        let mut it_0: *mut iterzset = &mut (*op).iter.zset;
        if !((*op).encoding == 11 as libc::c_int) {
            if !((*op).encoding == 7 as libc::c_int) {
                _serverPanic(
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    2074 as libc::c_int,
                    b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2077 as libc::c_int,
            b"Unsupported type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn zuiDiscardDirtyValue(mut val: *mut zsetopval) {
    if (*val).flags & 1 as libc::c_int != 0 {
        sdsfree((*val).ele);
        (*val).ele = 0 as sds;
        (*val).flags &= !(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zuiLength(mut op: *mut zsetopsrc) -> libc::c_ulong {
    if ((*op).subject).is_null() {
        return 0 as libc::c_int as libc::c_ulong;
    }
    if (*op).type_0 == 2 as libc::c_int {
        if (*op).encoding == 6 as libc::c_int {
            return intsetLen((*(*op).subject).ptr as *const intset) as libc::c_ulong
        } else {
            if (*op).encoding == 2 as libc::c_int {
                let mut ht: *mut dict = (*(*op).subject).ptr as *mut dict;
                return ((*ht).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*ht).ht_used[1 as libc::c_int as usize]);
            } else {
                _serverPanic(
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    2100 as libc::c_int,
                    b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
    } else if (*op).type_0 == 3 as libc::c_int {
        if (*op).encoding == 11 as libc::c_int {
            return zzlLength((*(*op).subject).ptr as *mut libc::c_uchar) as libc::c_ulong
        } else {
            if (*op).encoding == 7 as libc::c_int {
                let mut zs: *mut zset = (*(*op).subject).ptr as *mut zset;
                return (*(*zs).zsl).length;
            } else {
                _serverPanic(
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    2109 as libc::c_int,
                    b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2112 as libc::c_int,
            b"Unsupported type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn zuiNext(
    mut op: *mut zsetopsrc,
    mut val: *mut zsetopval,
) -> libc::c_int {
    if ((*op).subject).is_null() {
        return 0 as libc::c_int;
    }
    zuiDiscardDirtyValue(val);
    memset(
        val as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<zsetopval>() as libc::c_ulong,
    );
    if (*op).type_0 == 2 as libc::c_int {
        let mut it: *mut iterset = &mut (*op).iter.set;
        if (*op).encoding == 6 as libc::c_int {
            let mut ell: int64_t = 0;
            if intsetGet((*it).is.is, (*it).is.ii as uint32_t, &mut ell) == 0 {
                return 0 as libc::c_int;
            }
            (*val).ell = ell as libc::c_longlong;
            (*val).score = 1.0f64;
            (*it).is.ii += 1;
        } else if (*op).encoding == 2 as libc::c_int {
            if ((*it).ht.de).is_null() {
                return 0 as libc::c_int;
            }
            (*val).ele = (*(*it).ht.de).key as sds;
            (*val).score = 1.0f64;
            (*it).ht.de = dictNext((*it).ht.di);
        } else {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                2148 as libc::c_int,
                b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*op).type_0 == 3 as libc::c_int {
        let mut it_0: *mut iterzset = &mut (*op).iter.zset;
        if (*op).encoding == 11 as libc::c_int {
            if ((*it_0).zl.eptr).is_null() || ((*it_0).zl.sptr).is_null() {
                return 0 as libc::c_int;
            }
            (*val).estr = lpGetValue((*it_0).zl.eptr, &mut (*val).elen, &mut (*val).ell);
            (*val).score = zzlGetScore((*it_0).zl.sptr);
            zzlPrev((*it_0).zl.zl, &mut (*it_0).zl.eptr, &mut (*it_0).zl.sptr);
        } else if (*op).encoding == 7 as libc::c_int {
            if ((*it_0).sl.node).is_null() {
                return 0 as libc::c_int;
            }
            (*val).ele = (*(*it_0).sl.node).ele;
            (*val).score = (*(*it_0).sl.node).score;
            (*it_0).sl.node = (*(*it_0).sl.node).backward;
        } else {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                2170 as libc::c_int,
                b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2173 as libc::c_int,
            b"Unsupported type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zuiLongLongFromValue(mut val: *mut zsetopval) -> libc::c_int {
    if (*val).flags & 2 as libc::c_int == 0 {
        (*val).flags |= 2 as libc::c_int;
        if !((*val).ele).is_null() {
            if string2ll(
                (*val).ele as *const libc::c_char,
                sdslen((*val).ele),
                &mut (*val).ell,
            ) != 0
            {
                (*val).flags |= 4 as libc::c_int;
            }
        } else if !((*val).estr).is_null() {
            if string2ll(
                (*val).estr as *mut libc::c_char,
                (*val).elen as size_t,
                &mut (*val).ell,
            ) != 0
            {
                (*val).flags |= 4 as libc::c_int;
            }
        } else {
            (*val).flags |= 4 as libc::c_int;
        }
    }
    return (*val).flags & 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zuiSdsFromValue(mut val: *mut zsetopval) -> sds {
    if ((*val).ele).is_null() {
        if !((*val).estr).is_null() {
            (*val)
                .ele = sdsnewlen(
                (*val).estr as *mut libc::c_char as *const libc::c_void,
                (*val).elen as size_t,
            );
        } else {
            (*val).ele = sdsfromlonglong((*val).ell);
        }
        (*val).flags |= 1 as libc::c_int;
    }
    return (*val).ele;
}
#[no_mangle]
pub unsafe extern "C" fn zuiNewSdsFromValue(mut val: *mut zsetopval) -> sds {
    if (*val).flags & 1 as libc::c_int != 0 {
        let mut ele: sds = (*val).ele;
        (*val).flags &= !(1 as libc::c_int);
        (*val).ele = 0 as sds;
        return ele;
    } else if !((*val).ele).is_null() {
        return sdsdup((*val).ele)
    } else if !((*val).estr).is_null() {
        return sdsnewlen(
            (*val).estr as *mut libc::c_char as *const libc::c_void,
            (*val).elen as size_t,
        )
    } else {
        return sdsfromlonglong((*val).ell)
    };
}
#[no_mangle]
pub unsafe extern "C" fn zuiBufferFromValue(mut val: *mut zsetopval) -> libc::c_int {
    if ((*val).estr).is_null() {
        if !((*val).ele).is_null() {
            (*val).elen = sdslen((*val).ele) as libc::c_uint;
            (*val).estr = (*val).ele as *mut libc::c_uchar;
        } else {
            (*val)
                .elen = ll2string(
                ((*val)._buf).as_mut_ptr() as *mut libc::c_char,
                core::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
                (*val).ell,
            ) as libc::c_uint;
            (*val).estr = ((*val)._buf).as_mut_ptr();
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zuiFind(
    mut op: *mut zsetopsrc,
    mut val: *mut zsetopval,
    mut score: *mut libc::c_double,
) -> libc::c_int {
    if ((*op).subject).is_null() {
        return 0 as libc::c_int;
    }
    if (*op).type_0 == 2 as libc::c_int {
        if (*op).encoding == 6 as libc::c_int {
            if zuiLongLongFromValue(val) != 0
                && intsetFind((*(*op).subject).ptr as *mut intset, (*val).ell as int64_t)
                    as libc::c_int != 0
            {
                *score = 1.0f64;
                return 1 as libc::c_int;
            } else {
                return 0 as libc::c_int
            }
        } else {
            if (*op).encoding == 2 as libc::c_int {
                let mut ht: *mut dict = (*(*op).subject).ptr as *mut dict;
                zuiSdsFromValue(val);
                if !(dictFind(ht, (*val).ele as *const libc::c_void)).is_null() {
                    *score = 1.0f64;
                    return 1 as libc::c_int;
                } else {
                    return 0 as libc::c_int
                }
            } else {
                _serverPanic(
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    2265 as libc::c_int,
                    b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
    } else if (*op).type_0 == 3 as libc::c_int {
        zuiSdsFromValue(val);
        if (*op).encoding == 11 as libc::c_int {
            if !(zzlFind((*(*op).subject).ptr as *mut libc::c_uchar, (*val).ele, score))
                .is_null()
            {
                return 1 as libc::c_int
            } else {
                return 0 as libc::c_int
            }
        } else {
            if (*op).encoding == 7 as libc::c_int {
                let mut zs: *mut zset = (*(*op).subject).ptr as *mut zset;
                let mut de: *mut dictEntry = 0 as *mut dictEntry;
                de = dictFind((*zs).dict, (*val).ele as *const libc::c_void);
                if !de.is_null() {
                    *score = *((*de).v.val as *mut libc::c_double);
                    return 1 as libc::c_int;
                } else {
                    return 0 as libc::c_int
                }
            } else {
                _serverPanic(
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    2287 as libc::c_int,
                    b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2290 as libc::c_int,
            b"Unsupported type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn zuiCompareByCardinality(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    let mut first: libc::c_ulong = zuiLength(s1 as *mut zsetopsrc);
    let mut second: libc::c_ulong = zuiLength(s2 as *mut zsetopsrc);
    if first > second {
        return 1 as libc::c_int;
    }
    if first < second {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zuiCompareByRevCardinality(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    return zuiCompareByCardinality(s1, s2) * -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn zunionInterAggregate(
    mut target: *mut libc::c_double,
    mut val: libc::c_double,
    mut aggregate: libc::c_int,
) {
    if aggregate == 1 as libc::c_int {
        *target = *target + val;
        if (*target).is_nan() as i32 != 0 {
            *target = 0.0f64;
        }
    } else if aggregate == 2 as libc::c_int {
        *target = if val < *target { val } else { *target };
    } else if aggregate == 3 as libc::c_int {
        *target = if val > *target { val } else { *target };
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2324 as libc::c_int,
            b"Unknown ZUNION/INTER aggregate type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
unsafe extern "C" fn zsetDictGetMaxElementLength(
    mut d: *mut dict,
    mut totallen: *mut size_t,
) -> size_t {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut maxelelen: size_t = 0 as libc::c_int as size_t;
    di = dictGetIterator(d);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ele: sds = (*de).key as sds;
        if sdslen(ele) > maxelelen {
            maxelelen = sdslen(ele);
        }
        if !totallen.is_null() {
            *totallen = (*totallen as libc::c_ulong).wrapping_add(sdslen(ele)) as size_t
                as size_t;
        }
    }
    dictReleaseIterator(di);
    return maxelelen;
}
unsafe extern "C" fn zdiffAlgorithm1(
    mut src: *mut zsetopsrc,
    mut setnum: libc::c_long,
    mut dstzset: *mut zset,
    mut maxelelen: *mut size_t,
    mut totelelen: *mut size_t,
) {
    let mut j: libc::c_int = 0;
    let mut zval: zsetopval = zsetopval {
        flags: 0,
        _buf: [0; 32],
        ele: 0 as *mut libc::c_char,
        estr: 0 as *mut libc::c_uchar,
        elen: 0,
        ell: 0,
        score: 0.,
    };
    let mut znode: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut tmp: sds = 0 as *mut libc::c_char;
    qsort(
        src.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (setnum - 1 as libc::c_int as libc::c_long) as size_t,
        core::mem::size_of::<zsetopsrc>() as libc::c_ulong,
        Some(
            zuiCompareByRevCardinality
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    memset(
        &mut zval as *mut zsetopval as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<zsetopval>() as libc::c_ulong,
    );
    zuiInitIterator(&mut *src.offset(0 as libc::c_int as isize));
    while zuiNext(&mut *src.offset(0 as libc::c_int as isize), &mut zval) != 0 {
        let mut value: libc::c_double = 0.;
        let mut exists: libc::c_int = 0 as libc::c_int;
        j = 1 as libc::c_int;
        while (j as libc::c_long) < setnum {
            if (*src.offset(j as isize)).subject
                == (*src.offset(0 as libc::c_int as isize)).subject
                || zuiFind(&mut *src.offset(j as isize), &mut zval, &mut value) != 0
            {
                exists = 1 as libc::c_int;
                break;
            } else {
                j += 1;
            }
        }
        if exists == 0 {
            tmp = zuiNewSdsFromValue(&mut zval);
            znode = zslInsert((*dstzset).zsl, zval.score, tmp);
            dictAdd(
                (*dstzset).dict,
                tmp as *mut libc::c_void,
                &mut (*znode).score as *mut libc::c_double as *mut libc::c_void,
            );
            if sdslen(tmp) > *maxelelen {
                *maxelelen = sdslen(tmp);
            }
            *totelelen = (*totelelen as libc::c_ulong).wrapping_add(sdslen(tmp))
                as size_t as size_t;
        }
    }
    zuiClearIterator(&mut *src.offset(0 as libc::c_int as isize));
}
unsafe extern "C" fn zdiffAlgorithm2(
    mut src: *mut zsetopsrc,
    mut setnum: libc::c_long,
    mut dstzset: *mut zset,
    mut maxelelen: *mut size_t,
    mut totelelen: *mut size_t,
) {
    let mut j: libc::c_int = 0;
    let mut cardinality: libc::c_int = 0 as libc::c_int;
    let mut zval: zsetopval = zsetopval {
        flags: 0,
        _buf: [0; 32],
        ele: 0 as *mut libc::c_char,
        estr: 0 as *mut libc::c_uchar,
        elen: 0,
        ell: 0,
        score: 0.,
    };
    let mut znode: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut tmp: sds = 0 as *mut libc::c_char;
    j = 0 as libc::c_int;
    while (j as libc::c_long) < setnum {
        if !(zuiLength(&mut *src.offset(j as isize))
            == 0 as libc::c_int as libc::c_ulong)
        {
            memset(
                &mut zval as *mut zsetopval as *mut libc::c_void,
                0 as libc::c_int,
                core::mem::size_of::<zsetopval>() as libc::c_ulong,
            );
            zuiInitIterator(&mut *src.offset(j as isize));
            while zuiNext(&mut *src.offset(j as isize), &mut zval) != 0 {
                if j == 0 as libc::c_int {
                    tmp = zuiNewSdsFromValue(&mut zval);
                    znode = zslInsert((*dstzset).zsl, zval.score, tmp);
                    dictAdd(
                        (*dstzset).dict,
                        tmp as *mut libc::c_void,
                        &mut (*znode).score as *mut libc::c_double as *mut libc::c_void,
                    );
                    cardinality += 1;
                } else {
                    tmp = zuiSdsFromValue(&mut zval);
                    if zsetRemoveFromSkiplist(dstzset, tmp) != 0 {
                        cardinality -= 1;
                    }
                }
                if cardinality == 0 as libc::c_int {
                    break;
                }
            }
            zuiClearIterator(&mut *src.offset(j as isize));
            if cardinality == 0 as libc::c_int {
                break;
            }
        }
        j += 1;
    }
    if htNeedsResize((*dstzset).dict) != 0 {
        dictResize((*dstzset).dict);
    }
    *maxelelen = zsetDictGetMaxElementLength((*dstzset).dict, totelelen);
}
unsafe extern "C" fn zsetChooseDiffAlgorithm(
    mut src: *mut zsetopsrc,
    mut setnum: libc::c_long,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut algo_one_work: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut algo_two_work: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    j = 0 as libc::c_int;
    while (j as libc::c_long) < setnum {
        if j > 0 as libc::c_int
            && (*src.offset(0 as libc::c_int as isize)).subject
                == (*src.offset(j as isize)).subject
        {
            return 0 as libc::c_int;
        }
        algo_one_work = (algo_one_work as libc::c_ulonglong)
            .wrapping_add(
                zuiLength(&mut *src.offset(0 as libc::c_int as isize))
                    as libc::c_ulonglong,
            ) as libc::c_longlong as libc::c_longlong;
        algo_two_work = (algo_two_work as libc::c_ulonglong)
            .wrapping_add(zuiLength(&mut *src.offset(j as isize)) as libc::c_ulonglong)
            as libc::c_longlong as libc::c_longlong;
        j += 1;
    }
    algo_one_work /= 2 as libc::c_int as libc::c_longlong;
    return if algo_one_work <= algo_two_work {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn zdiff(
    mut src: *mut zsetopsrc,
    mut setnum: libc::c_long,
    mut dstzset: *mut zset,
    mut maxelelen: *mut size_t,
    mut totelelen: *mut size_t,
) {
    if zuiLength(&mut *src.offset(0 as libc::c_int as isize))
        > 0 as libc::c_int as libc::c_ulong
    {
        let mut diff_algo: libc::c_int = zsetChooseDiffAlgorithm(src, setnum);
        if diff_algo == 1 as libc::c_int {
            zdiffAlgorithm1(src, setnum, dstzset, maxelelen, totelelen);
        } else if diff_algo == 2 as libc::c_int {
            zdiffAlgorithm2(src, setnum, dstzset, maxelelen, totelelen);
        } else if diff_algo != 0 as libc::c_int {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                2502 as libc::c_int,
                b"Unknown algorithm\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
}
#[no_mangle]
pub static mut setAccumulatorDictType: dictType = unsafe {
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn zunionInterDiffGenericCommand(
    mut c: *mut client,
    mut dstkey: *mut robj,
    mut numkeysIndex: libc::c_int,
    mut op: libc::c_int,
    mut cardinality_only: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut setnum: libc::c_long = 0;
    let mut aggregate: libc::c_int = 1 as libc::c_int;
    let mut src: *mut zsetopsrc = 0 as *mut zsetopsrc;
    let mut zval: zsetopval = zsetopval {
        flags: 0,
        _buf: [0; 32],
        ele: 0 as *mut libc::c_char,
        estr: 0 as *mut libc::c_uchar,
        elen: 0,
        ell: 0,
        score: 0.,
    };
    let mut tmp: sds = 0 as *mut libc::c_char;
    let mut maxelelen: size_t = 0 as libc::c_int as size_t;
    let mut totelelen: size_t = 0 as libc::c_int as size_t;
    let mut dstobj: *mut robj = 0 as *mut robj;
    let mut dstzset: *mut zset = 0 as *mut zset;
    let mut znode: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut withscores: libc::c_int = 0 as libc::c_int;
    let mut cardinality: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut limit: libc::c_long = 0 as libc::c_int as libc::c_long;
    if getLongFromObjectOrReply(
        c,
        *((*c).argv).offset(numkeysIndex as isize),
        &mut setnum,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if setnum < 1 as libc::c_int as libc::c_long {
        addReplyErrorFormat(
            c,
            b"at least 1 input key is needed for '%s' command\0" as *const u8
                as *const libc::c_char,
            (*(*c).cmd).fullname,
        );
        return;
    }
    if setnum > ((*c).argc - (numkeysIndex + 1 as libc::c_int)) as libc::c_long {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    src = zcalloc(
        (core::mem::size_of::<zsetopsrc>() as libc::c_ulong)
            .wrapping_mul(setnum as libc::c_ulong),
    ) as *mut zsetopsrc;
    i = 0 as libc::c_int;
    j = numkeysIndex + 1 as libc::c_int;
    while (i as libc::c_long) < setnum {
        let mut obj: *mut robj = lookupKeyRead((*c).db, *((*c).argv).offset(j as isize));
        if !obj.is_null() {
            if (*obj).type_0() as libc::c_int != 3 as libc::c_int
                && (*obj).type_0() as libc::c_int != 2 as libc::c_int
            {
                zfree(src as *mut libc::c_void);
                addReplyErrorObject(c, shared.wrongtypeerr);
                return;
            }
            let ref mut fresh10 = (*src.offset(i as isize)).subject;
            *fresh10 = obj;
            (*src.offset(i as isize)).type_0 = (*obj).type_0() as libc::c_int;
            (*src.offset(i as isize)).encoding = (*obj).encoding() as libc::c_int;
        } else {
            let ref mut fresh11 = (*src.offset(i as isize)).subject;
            *fresh11 = 0 as *mut robj;
        }
        (*src.offset(i as isize)).weight = 1.0f64;
        i += 1;
        j += 1;
    }
    if j < (*c).argc {
        let mut remaining: libc::c_int = (*c).argc - j;
        while remaining != 0 {
            if op != 1 as libc::c_int && cardinality_only == 0
                && remaining as libc::c_long >= setnum + 1 as libc::c_int as libc::c_long
                && strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"weights\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                j += 1;
                remaining -= 1;
                i = 0 as libc::c_int;
                while (i as libc::c_long) < setnum {
                    if getDoubleFromObjectOrReply(
                        c,
                        *((*c).argv).offset(j as isize),
                        &mut (*src.offset(i as isize)).weight,
                        b"weight value is not a float\0" as *const u8
                            as *const libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        zfree(src as *mut libc::c_void);
                        return;
                    }
                    i += 1;
                    j += 1;
                    remaining -= 1;
                }
            } else if op != 1 as libc::c_int && cardinality_only == 0
                && remaining >= 2 as libc::c_int
                && strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"aggregate\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                j += 1;
                remaining -= 1;
                if strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"sum\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    aggregate = 1 as libc::c_int;
                } else if strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"min\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    aggregate = 2 as libc::c_int;
                } else if strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"max\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    aggregate = 3 as libc::c_int;
                } else {
                    zfree(src as *mut libc::c_void);
                    addReplyErrorObject(c, shared.syntaxerr);
                    return;
                }
                j += 1;
                remaining -= 1;
            } else if remaining >= 1 as libc::c_int && dstkey.is_null()
                && cardinality_only == 0
                && strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"withscores\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                j += 1;
                remaining -= 1;
                withscores = 1 as libc::c_int;
            } else if cardinality_only != 0 && remaining >= 2 as libc::c_int
                && strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"limit\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                j += 1;
                remaining -= 1;
                if getPositiveLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset(j as isize),
                    &mut limit,
                    b"LIMIT can't be negative\0" as *const u8 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    zfree(src as *mut libc::c_void);
                    return;
                }
                j += 1;
                remaining -= 1;
            } else {
                zfree(src as *mut libc::c_void);
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
        }
    }
    if op != 1 as libc::c_int {
        qsort(
            src as *mut libc::c_void,
            setnum as size_t,
            core::mem::size_of::<zsetopsrc>() as libc::c_ulong,
            Some(
                zuiCompareByCardinality
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    dstobj = createZsetObject();
    dstzset = (*dstobj).ptr as *mut zset;
    memset(
        &mut zval as *mut zsetopval as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<zsetopval>() as libc::c_ulong,
    );
    if op == 2 as libc::c_int {
        if zuiLength(&mut *src.offset(0 as libc::c_int as isize))
            > 0 as libc::c_int as libc::c_ulong
        {
            zuiInitIterator(&mut *src.offset(0 as libc::c_int as isize));
            while zuiNext(&mut *src.offset(0 as libc::c_int as isize), &mut zval) != 0 {
                let mut score: libc::c_double = 0.;
                let mut value: libc::c_double = 0.;
                score = (*src.offset(0 as libc::c_int as isize)).weight * zval.score;
                if score.is_nan() as i32 != 0 {
                    score = 0 as libc::c_int as libc::c_double;
                }
                j = 1 as libc::c_int;
                while (j as libc::c_long) < setnum {
                    if (*src.offset(j as isize)).subject
                        == (*src.offset(0 as libc::c_int as isize)).subject
                    {
                        value = zval.score * (*src.offset(j as isize)).weight;
                        zunionInterAggregate(&mut score, value, aggregate);
                    } else {
                        if !(zuiFind(&mut *src.offset(j as isize), &mut zval, &mut value)
                            != 0)
                        {
                            break;
                        }
                        value *= (*src.offset(j as isize)).weight;
                        zunionInterAggregate(&mut score, value, aggregate);
                    }
                    j += 1;
                }
                if j as libc::c_long == setnum && cardinality_only != 0 {
                    cardinality = cardinality.wrapping_add(1);
                    if !(limit != 0 && cardinality >= limit as libc::c_ulong) {
                        continue;
                    }
                    zuiDiscardDirtyValue(&mut zval);
                    break;
                } else if j as libc::c_long == setnum {
                    tmp = zuiNewSdsFromValue(&mut zval);
                    znode = zslInsert((*dstzset).zsl, score, tmp);
                    dictAdd(
                        (*dstzset).dict,
                        tmp as *mut libc::c_void,
                        &mut (*znode).score as *mut libc::c_double as *mut libc::c_void,
                    );
                    totelelen = (totelelen as libc::c_ulong).wrapping_add(sdslen(tmp))
                        as size_t as size_t;
                    if sdslen(tmp) > maxelelen {
                        maxelelen = sdslen(tmp);
                    }
                }
            }
            zuiClearIterator(&mut *src.offset(0 as libc::c_int as isize));
        }
    } else if op == 0 as libc::c_int {
        let mut accumulator: *mut dict = dictCreate(&mut setAccumulatorDictType);
        let mut di: *mut dictIterator = 0 as *mut dictIterator;
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        let mut existing: *mut dictEntry = 0 as *mut dictEntry;
        let mut score_0: libc::c_double = 0.;
        if setnum != 0 {
            dictExpand(
                accumulator,
                zuiLength(
                    &mut *src
                        .offset((setnum - 1 as libc::c_int as libc::c_long) as isize),
                ),
            );
        }
        i = 0 as libc::c_int;
        while (i as libc::c_long) < setnum {
            if !(zuiLength(&mut *src.offset(i as isize))
                == 0 as libc::c_int as libc::c_ulong)
            {
                zuiInitIterator(&mut *src.offset(i as isize));
                while zuiNext(&mut *src.offset(i as isize), &mut zval) != 0 {
                    score_0 = (*src.offset(i as isize)).weight * zval.score;
                    if score_0.is_nan() as i32 != 0 {
                        score_0 = 0 as libc::c_int as libc::c_double;
                    }
                    de = dictAddRaw(
                        accumulator,
                        zuiSdsFromValue(&mut zval) as *mut libc::c_void,
                        &mut existing,
                    );
                    if existing.is_null() {
                        tmp = zuiNewSdsFromValue(&mut zval);
                        totelelen = (totelelen as libc::c_ulong)
                            .wrapping_add(sdslen(tmp)) as size_t as size_t;
                        if sdslen(tmp) > maxelelen {
                            maxelelen = sdslen(tmp);
                        }
                        if ((*(*accumulator).type_0).keyDup).is_some() {
                            (*de)
                                .key = ((*(*accumulator).type_0).keyDup)
                                .expect(
                                    "non-null function pointer",
                                )(accumulator, tmp as *const libc::c_void);
                        } else {
                            (*de).key = tmp as *mut libc::c_void;
                        }
                        (*de).v.d = score_0;
                    } else {
                        zunionInterAggregate(&mut (*existing).v.d, score_0, aggregate);
                    }
                }
                zuiClearIterator(&mut *src.offset(i as isize));
            }
            i += 1;
        }
        di = dictGetIterator(accumulator);
        dictExpand(
            (*dstzset).dict,
            ((*accumulator).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*accumulator).ht_used[1 as libc::c_int as usize]),
        );
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut ele: sds = (*de).key as sds;
            score_0 = (*de).v.d;
            znode = zslInsert((*dstzset).zsl, score_0, ele);
            dictAdd(
                (*dstzset).dict,
                ele as *mut libc::c_void,
                &mut (*znode).score as *mut libc::c_double as *mut libc::c_void,
            );
        }
        dictReleaseIterator(di);
        dictRelease(accumulator);
    } else if op == 1 as libc::c_int {
        zdiff(src, setnum, dstzset, &mut maxelelen, &mut totelelen);
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2767 as libc::c_int,
            b"Unknown operator\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    if !dstkey.is_null() {
        if (*(*dstzset).zsl).length != 0 {
            zsetConvertToListpackIfNeeded(dstobj, maxelelen, totelelen);
            setKey(c, (*c).db, dstkey, dstobj, 0 as libc::c_int);
            addReplyLongLong(c, zsetLength(dstobj) as libc::c_longlong);
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 7 as libc::c_int,
                (if op == 0 as libc::c_int {
                    b"zunionstore\0" as *const u8 as *const libc::c_char
                } else if op == 2 as libc::c_int {
                    b"zinterstore\0" as *const u8 as *const libc::c_char
                } else {
                    b"zdiffstore\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
                dstkey,
                (*(*c).db).id,
            );
            server.dirty += 1;
        } else {
            addReply(c, shared.czero);
            if dbDelete((*c).db, dstkey) != 0 {
                signalModifiedKey(c, (*c).db, dstkey);
                notifyKeyspaceEvent(
                    (1 as libc::c_int) << 2 as libc::c_int,
                    b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    dstkey,
                    (*(*c).db).id,
                );
                server.dirty += 1;
            }
        }
    } else if cardinality_only != 0 {
        addReplyLongLong(c, cardinality as libc::c_longlong);
    } else {
        let mut length: libc::c_ulong = (*(*dstzset).zsl).length;
        let mut zsl: *mut zskiplist = (*dstzset).zsl;
        let mut zn: *mut zskiplistNode = (*((*(*zsl).header).level)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .forward;
        if withscores != 0 && (*c).resp == 2 as libc::c_int {
            addReplyArrayLen(
                c,
                length.wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_long,
            );
        } else {
            addReplyArrayLen(c, length as libc::c_long);
        }
        while !zn.is_null() {
            if withscores != 0 && (*c).resp > 2 as libc::c_int {
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
            }
            addReplyBulkCBuffer(c, (*zn).ele as *const libc::c_void, sdslen((*zn).ele));
            if withscores != 0 {
                addReplyDouble(c, (*zn).score);
            }
            zn = (*((*zn).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
        }
    }
    decrRefCount(dstobj);
    zfree(src as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn zunionstoreCommand(mut c: *mut client) {
    zunionInterDiffGenericCommand(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zinterstoreCommand(mut c: *mut client) {
    zunionInterDiffGenericCommand(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zdiffstoreCommand(mut c: *mut client) {
    zunionInterDiffGenericCommand(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zunionCommand(mut c: *mut client) {
    zunionInterDiffGenericCommand(
        c,
        0 as *mut robj,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zinterCommand(mut c: *mut client) {
    zunionInterDiffGenericCommand(
        c,
        0 as *mut robj,
        1 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zinterCardCommand(mut c: *mut client) {
    zunionInterDiffGenericCommand(
        c,
        0 as *mut robj,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zdiffCommand(mut c: *mut client) {
    zunionInterDiffGenericCommand(
        c,
        0 as *mut robj,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn zrangeResultBeginClient(
    mut handler: *mut zrange_result_handler,
    mut length: libc::c_long,
) {
    if length > 0 as libc::c_int as libc::c_long {
        if (*handler).withscores != 0 && (*(*handler).client).resp == 2 as libc::c_int {
            length *= 2 as libc::c_int as libc::c_long;
        }
        addReplyArrayLen((*handler).client, length);
        (*handler).userdata = 0 as *mut libc::c_void;
        return;
    }
    (*handler).userdata = addReplyDeferredLen((*handler).client);
}
unsafe extern "C" fn zrangeResultEmitCBufferToClient(
    mut handler: *mut zrange_result_handler,
    mut value: *const libc::c_void,
    mut value_length_in_bytes: size_t,
    mut score: libc::c_double,
) {
    if (*handler).should_emit_array_length != 0 {
        addReplyArrayLen((*handler).client, 2 as libc::c_int as libc::c_long);
    }
    addReplyBulkCBuffer((*handler).client, value, value_length_in_bytes);
    if (*handler).withscores != 0 {
        addReplyDouble((*handler).client, score);
    }
}
unsafe extern "C" fn zrangeResultEmitLongLongToClient(
    mut handler: *mut zrange_result_handler,
    mut value: libc::c_longlong,
    mut score: libc::c_double,
) {
    if (*handler).should_emit_array_length != 0 {
        addReplyArrayLen((*handler).client, 2 as libc::c_int as libc::c_long);
    }
    addReplyBulkLongLong((*handler).client, value);
    if (*handler).withscores != 0 {
        addReplyDouble((*handler).client, score);
    }
}
unsafe extern "C" fn zrangeResultFinalizeClient(
    mut handler: *mut zrange_result_handler,
    mut result_count: size_t,
) {
    if ((*handler).userdata).is_null() {
        return;
    }
    if (*handler).withscores != 0 && (*(*handler).client).resp == 2 as libc::c_int {
        result_count = (result_count as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    setDeferredArrayLen(
        (*handler).client,
        (*handler).userdata,
        result_count as libc::c_long,
    );
}
unsafe extern "C" fn zrangeResultBeginStore(
    mut handler: *mut zrange_result_handler,
    mut length: libc::c_long,
) {
    if length > server.zset_max_listpack_entries as libc::c_long {
        (*handler).dstobj = createZsetObject();
    } else {
        (*handler).dstobj = createZsetListpackObject();
    };
}
unsafe extern "C" fn zrangeResultEmitCBufferForStore(
    mut handler: *mut zrange_result_handler,
    mut value: *const libc::c_void,
    mut value_length_in_bytes: size_t,
    mut score: libc::c_double,
) {
    let mut newscore: libc::c_double = 0.;
    let mut retflags: libc::c_int = 0 as libc::c_int;
    let mut ele: sds = sdsnewlen(value, value_length_in_bytes);
    let mut retval: libc::c_int = zsetAdd(
        (*handler).dstobj,
        score,
        ele,
        0 as libc::c_int,
        &mut retflags,
        &mut newscore,
    );
    sdsfree(ele);
    if retval != 0 {} else {
        _serverAssert(
            b"retval\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2969 as libc::c_int,
        );
        unreachable!();
    };
}
unsafe extern "C" fn zrangeResultEmitLongLongForStore(
    mut handler: *mut zrange_result_handler,
    mut value: libc::c_longlong,
    mut score: libc::c_double,
) {
    let mut newscore: libc::c_double = 0.;
    let mut retflags: libc::c_int = 0 as libc::c_int;
    let mut ele: sds = sdsfromlonglong(value);
    let mut retval: libc::c_int = zsetAdd(
        (*handler).dstobj,
        score,
        ele,
        0 as libc::c_int,
        &mut retflags,
        &mut newscore,
    );
    sdsfree(ele);
    if retval != 0 {} else {
        _serverAssert(
            b"retval\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            2980 as libc::c_int,
        );
        unreachable!();
    };
}
unsafe extern "C" fn zrangeResultFinalizeStore(
    mut handler: *mut zrange_result_handler,
    mut result_count: size_t,
) {
    if result_count != 0 {
        setKey(
            (*handler).client,
            (*(*handler).client).db,
            (*handler).dstkey,
            (*handler).dstobj,
            0 as libc::c_int,
        );
        addReplyLongLong((*handler).client, result_count as libc::c_longlong);
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 7 as libc::c_int,
            b"zrangestore\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*handler).dstkey,
            (*(*(*handler).client).db).id,
        );
        server.dirty += 1;
    } else {
        addReply((*handler).client, shared.czero);
        if dbDelete((*(*handler).client).db, (*handler).dstkey) != 0 {
            signalModifiedKey(
                (*handler).client,
                (*(*handler).client).db,
                (*handler).dstkey,
            );
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 2 as libc::c_int,
                b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*handler).dstkey,
                (*(*(*handler).client).db).id,
            );
            server.dirty += 1;
        }
    }
    decrRefCount((*handler).dstobj);
}
unsafe extern "C" fn zrangeResultHandlerInit(
    mut handler: *mut zrange_result_handler,
    mut client: *mut client,
    mut type_0: zrange_consumer_type,
) {
    memset(
        handler as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<zrange_result_handler>() as libc::c_ulong,
    );
    (*handler).client = client;
    match type_0 as libc::c_uint {
        0 => {
            (*handler)
                .beginResultEmission = Some(
                zrangeResultBeginClient
                    as unsafe extern "C" fn(
                        *mut zrange_result_handler,
                        libc::c_long,
                    ) -> (),
            );
            (*handler)
                .finalizeResultEmission = Some(
                zrangeResultFinalizeClient
                    as unsafe extern "C" fn(*mut zrange_result_handler, size_t) -> (),
            );
            (*handler)
                .emitResultFromCBuffer = Some(
                zrangeResultEmitCBufferToClient
                    as unsafe extern "C" fn(
                        *mut zrange_result_handler,
                        *const libc::c_void,
                        size_t,
                        libc::c_double,
                    ) -> (),
            );
            (*handler)
                .emitResultFromLongLong = Some(
                zrangeResultEmitLongLongToClient
                    as unsafe extern "C" fn(
                        *mut zrange_result_handler,
                        libc::c_longlong,
                        libc::c_double,
                    ) -> (),
            );
        }
        1 => {
            (*handler)
                .beginResultEmission = Some(
                zrangeResultBeginStore
                    as unsafe extern "C" fn(
                        *mut zrange_result_handler,
                        libc::c_long,
                    ) -> (),
            );
            (*handler)
                .finalizeResultEmission = Some(
                zrangeResultFinalizeStore
                    as unsafe extern "C" fn(*mut zrange_result_handler, size_t) -> (),
            );
            (*handler)
                .emitResultFromCBuffer = Some(
                zrangeResultEmitCBufferForStore
                    as unsafe extern "C" fn(
                        *mut zrange_result_handler,
                        *const libc::c_void,
                        size_t,
                        libc::c_double,
                    ) -> (),
            );
            (*handler)
                .emitResultFromLongLong = Some(
                zrangeResultEmitLongLongForStore
                    as unsafe extern "C" fn(
                        *mut zrange_result_handler,
                        libc::c_longlong,
                        libc::c_double,
                    ) -> (),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn zrangeResultHandlerScoreEmissionEnable(
    mut handler: *mut zrange_result_handler,
) {
    (*handler).withscores = 1 as libc::c_int;
    (*handler)
        .should_emit_array_length = ((*(*handler).client).resp > 2 as libc::c_int)
        as libc::c_int;
}
unsafe extern "C" fn zrangeResultHandlerDestinationKeySet(
    mut handler: *mut zrange_result_handler,
    mut dstkey: *mut robj,
) {
    (*handler).dstkey = dstkey;
}
#[no_mangle]
pub unsafe extern "C" fn genericZrangebyrankCommand(
    mut handler: *mut zrange_result_handler,
    mut zobj: *mut robj,
    mut start: libc::c_long,
    mut end: libc::c_long,
    mut withscores: libc::c_int,
    mut reverse: libc::c_int,
) {
    let mut c: *mut client = (*handler).client;
    let mut llen: libc::c_long = 0;
    let mut rangelen: libc::c_long = 0;
    let mut result_cardinality: size_t = 0;
    llen = zsetLength(zobj) as libc::c_long;
    if start < 0 as libc::c_int as libc::c_long {
        start = llen + start;
    }
    if end < 0 as libc::c_int as libc::c_long {
        end = llen + end;
    }
    if start < 0 as libc::c_int as libc::c_long {
        start = 0 as libc::c_int as libc::c_long;
    }
    if start > end || start >= llen {
        ((*handler).beginResultEmission)
            .expect(
                "non-null function pointer",
            )(handler, 0 as libc::c_int as libc::c_long);
        ((*handler).finalizeResultEmission)
            .expect("non-null function pointer")(handler, 0 as libc::c_int as size_t);
        return;
    }
    if end >= llen {
        end = llen - 1 as libc::c_int as libc::c_long;
    }
    rangelen = end - start + 1 as libc::c_int as libc::c_long;
    result_cardinality = rangelen as size_t;
    ((*handler).beginResultEmission)
        .expect("non-null function pointer")(handler, rangelen);
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = 0;
        let mut vlong: libc::c_longlong = 0;
        let mut score: libc::c_double = 0.0f64;
        if reverse != 0 {
            eptr = lpSeek(
                zl,
                -(2 as libc::c_int) as libc::c_long
                    - 2 as libc::c_int as libc::c_long * start,
            );
        } else {
            eptr = lpSeek(zl, 2 as libc::c_int as libc::c_long * start);
        }
        if !eptr.is_null() {} else {
            _serverAssertWithInfo(
                c,
                zobj,
                b"eptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                3078 as libc::c_int,
            );
            unreachable!();
        };
        sptr = lpNext(zl, eptr);
        loop {
            let fresh12 = rangelen;
            rangelen = rangelen - 1;
            if !(fresh12 != 0) {
                break;
            }
            if !eptr.is_null() && !sptr.is_null() {} else {
                _serverAssertWithInfo(
                    c,
                    zobj,
                    b"eptr != NULL && sptr != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    3082 as libc::c_int,
                );
                unreachable!();
            };
            vstr = lpGetValue(eptr, &mut vlen, &mut vlong);
            if withscores != 0 {
                score = zzlGetScore(sptr);
            }
            if vstr.is_null() {
                ((*handler).emitResultFromLongLong)
                    .expect("non-null function pointer")(handler, vlong, score);
            } else {
                ((*handler).emitResultFromCBuffer)
                    .expect(
                        "non-null function pointer",
                    )(handler, vstr as *const libc::c_void, vlen as size_t, score);
            }
            if reverse != 0 {
                zzlPrev(zl, &mut eptr, &mut sptr);
            } else {
                zzlNext(zl, &mut eptr, &mut sptr);
            }
        }
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        let mut ln: *mut zskiplistNode = 0 as *mut zskiplistNode;
        if reverse != 0 {
            ln = (*zsl).tail;
            if start > 0 as libc::c_int as libc::c_long {
                ln = zslGetElementByRank(zsl, (llen - start) as libc::c_ulong);
            }
        } else {
            ln = (*((*(*zsl).header).level)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .forward;
            if start > 0 as libc::c_int as libc::c_long {
                ln = zslGetElementByRank(
                    zsl,
                    (start + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                );
            }
        }
        loop {
            let fresh13 = rangelen;
            rangelen = rangelen - 1;
            if !(fresh13 != 0) {
                break;
            }
            if !ln.is_null() {} else {
                _serverAssertWithInfo(
                    c,
                    zobj,
                    b"ln != NULL\0" as *const u8 as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    3117 as libc::c_int,
                );
                unreachable!();
            };
            let mut ele: sds = (*ln).ele;
            ((*handler).emitResultFromCBuffer)
                .expect(
                    "non-null function pointer",
                )(handler, ele as *const libc::c_void, sdslen(ele), (*ln).score);
            ln = if reverse != 0 {
                (*ln).backward
            } else {
                (*((*ln).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward
            };
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            3123 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    ((*handler).finalizeResultEmission)
        .expect("non-null function pointer")(handler, result_cardinality);
}
#[no_mangle]
pub unsafe extern "C" fn zrangestoreCommand(mut c: *mut client) {
    let mut dstkey: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut handler: zrange_result_handler = zrange_result_handler {
        type_0: ZRANGE_CONSUMER_TYPE_CLIENT,
        client: 0 as *mut client,
        dstkey: 0 as *mut robj,
        dstobj: 0 as *mut robj,
        userdata: 0 as *mut libc::c_void,
        withscores: 0,
        should_emit_array_length: 0,
        beginResultEmission: None,
        finalizeResultEmission: None,
        emitResultFromCBuffer: None,
        emitResultFromLongLong: None,
    };
    zrangeResultHandlerInit(&mut handler, c, ZRANGE_CONSUMER_TYPE_INTERNAL);
    zrangeResultHandlerDestinationKeySet(&mut handler, dstkey);
    zrangeGenericCommand(
        &mut handler,
        2 as libc::c_int,
        1 as libc::c_int,
        ZRANGE_AUTO,
        ZRANGE_DIRECTION_AUTO,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zrangeCommand(mut c: *mut client) {
    let mut handler: zrange_result_handler = zrange_result_handler {
        type_0: ZRANGE_CONSUMER_TYPE_CLIENT,
        client: 0 as *mut client,
        dstkey: 0 as *mut robj,
        dstobj: 0 as *mut robj,
        userdata: 0 as *mut libc::c_void,
        withscores: 0,
        should_emit_array_length: 0,
        beginResultEmission: None,
        finalizeResultEmission: None,
        emitResultFromCBuffer: None,
        emitResultFromLongLong: None,
    };
    zrangeResultHandlerInit(&mut handler, c, ZRANGE_CONSUMER_TYPE_CLIENT);
    zrangeGenericCommand(
        &mut handler,
        1 as libc::c_int,
        0 as libc::c_int,
        ZRANGE_AUTO,
        ZRANGE_DIRECTION_AUTO,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zrevrangeCommand(mut c: *mut client) {
    let mut handler: zrange_result_handler = zrange_result_handler {
        type_0: ZRANGE_CONSUMER_TYPE_CLIENT,
        client: 0 as *mut client,
        dstkey: 0 as *mut robj,
        dstobj: 0 as *mut robj,
        userdata: 0 as *mut libc::c_void,
        withscores: 0,
        should_emit_array_length: 0,
        beginResultEmission: None,
        finalizeResultEmission: None,
        emitResultFromCBuffer: None,
        emitResultFromLongLong: None,
    };
    zrangeResultHandlerInit(&mut handler, c, ZRANGE_CONSUMER_TYPE_CLIENT);
    zrangeGenericCommand(
        &mut handler,
        1 as libc::c_int,
        0 as libc::c_int,
        ZRANGE_RANK,
        ZRANGE_DIRECTION_REVERSE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn genericZrangebyscoreCommand(
    mut handler: *mut zrange_result_handler,
    mut range: *mut zrangespec,
    mut zobj: *mut robj,
    mut offset: libc::c_long,
    mut limit: libc::c_long,
    mut reverse: libc::c_int,
) {
    let mut rangelen: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    ((*handler).beginResultEmission)
        .expect(
            "non-null function pointer",
        )(handler, -(1 as libc::c_int) as libc::c_long);
    if offset > 0 as libc::c_int as libc::c_long
        && offset >= zsetLength(zobj) as libc::c_long
    {
        ((*handler).finalizeResultEmission)
            .expect("non-null function pointer")(handler, 0 as libc::c_int as size_t);
        return;
    }
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = 0;
        let mut vlong: libc::c_longlong = 0;
        if reverse != 0 {
            eptr = zzlLastInRange(zl, range);
        } else {
            eptr = zzlFirstInRange(zl, range);
        }
        if !eptr.is_null() {
            sptr = lpNext(zl, eptr);
        }
        while !eptr.is_null()
            && {
                let fresh14 = offset;
                offset = offset - 1;
                fresh14 != 0
            }
        {
            if reverse != 0 {
                zzlPrev(zl, &mut eptr, &mut sptr);
            } else {
                zzlNext(zl, &mut eptr, &mut sptr);
            }
        }
        while !eptr.is_null()
            && {
                let fresh15 = limit;
                limit = limit - 1;
                fresh15 != 0
            }
        {
            let mut score: libc::c_double = zzlGetScore(sptr);
            if reverse != 0 {
                if zslValueGteMin(score, range) == 0 {
                    break;
                }
            } else if zslValueLteMax(score, range) == 0 {
                break;
            }
            vstr = lpGetValue(eptr, &mut vlen, &mut vlong);
            rangelen = rangelen.wrapping_add(1);
            if vstr.is_null() {
                ((*handler).emitResultFromLongLong)
                    .expect("non-null function pointer")(handler, vlong, score);
            } else {
                ((*handler).emitResultFromCBuffer)
                    .expect(
                        "non-null function pointer",
                    )(handler, vstr as *const libc::c_void, vlen as size_t, score);
            }
            if reverse != 0 {
                zzlPrev(zl, &mut eptr, &mut sptr);
            } else {
                zzlNext(zl, &mut eptr, &mut sptr);
            }
        }
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        let mut ln: *mut zskiplistNode = 0 as *mut zskiplistNode;
        if reverse != 0 {
            ln = zslLastInRange(zsl, range);
        } else {
            ln = zslFirstInRange(zsl, range);
        }
        while !ln.is_null()
            && {
                let fresh16 = offset;
                offset = offset - 1;
                fresh16 != 0
            }
        {
            if reverse != 0 {
                ln = (*ln).backward;
            } else {
                ln = (*((*ln).level).as_mut_ptr().offset(0 as libc::c_int as isize))
                    .forward;
            }
        }
        while !ln.is_null()
            && {
                let fresh17 = limit;
                limit = limit - 1;
                fresh17 != 0
            }
        {
            if reverse != 0 {
                if zslValueGteMin((*ln).score, range) == 0 {
                    break;
                }
            } else if zslValueLteMax((*ln).score, range) == 0 {
                break;
            }
            rangelen = rangelen.wrapping_add(1);
            ((*handler).emitResultFromCBuffer)
                .expect(
                    "non-null function pointer",
                )(
                handler,
                (*ln).ele as *const libc::c_void,
                sdslen((*ln).ele),
                (*ln).score,
            );
            if reverse != 0 {
                ln = (*ln).backward;
            } else {
                ln = (*((*ln).level).as_mut_ptr().offset(0 as libc::c_int as isize))
                    .forward;
            }
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            3260 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    ((*handler).finalizeResultEmission)
        .expect("non-null function pointer")(handler, rangelen);
}
#[no_mangle]
pub unsafe extern "C" fn zrangebyscoreCommand(mut c: *mut client) {
    let mut handler: zrange_result_handler = zrange_result_handler {
        type_0: ZRANGE_CONSUMER_TYPE_CLIENT,
        client: 0 as *mut client,
        dstkey: 0 as *mut robj,
        dstobj: 0 as *mut robj,
        userdata: 0 as *mut libc::c_void,
        withscores: 0,
        should_emit_array_length: 0,
        beginResultEmission: None,
        finalizeResultEmission: None,
        emitResultFromCBuffer: None,
        emitResultFromLongLong: None,
    };
    zrangeResultHandlerInit(&mut handler, c, ZRANGE_CONSUMER_TYPE_CLIENT);
    zrangeGenericCommand(
        &mut handler,
        1 as libc::c_int,
        0 as libc::c_int,
        ZRANGE_SCORE,
        ZRANGE_DIRECTION_FORWARD,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zrevrangebyscoreCommand(mut c: *mut client) {
    let mut handler: zrange_result_handler = zrange_result_handler {
        type_0: ZRANGE_CONSUMER_TYPE_CLIENT,
        client: 0 as *mut client,
        dstkey: 0 as *mut robj,
        dstobj: 0 as *mut robj,
        userdata: 0 as *mut libc::c_void,
        withscores: 0,
        should_emit_array_length: 0,
        beginResultEmission: None,
        finalizeResultEmission: None,
        emitResultFromCBuffer: None,
        emitResultFromLongLong: None,
    };
    zrangeResultHandlerInit(&mut handler, c, ZRANGE_CONSUMER_TYPE_CLIENT);
    zrangeGenericCommand(
        &mut handler,
        1 as libc::c_int,
        0 as libc::c_int,
        ZRANGE_SCORE,
        ZRANGE_DIRECTION_REVERSE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zcountCommand(mut c: *mut client) {
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut range: zrangespec = zrangespec {
        min: 0.,
        max: 0.,
        minex: 0,
        maxex: 0,
    };
    let mut count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if zslParseRange(
        *((*c).argv).offset(2 as libc::c_int as isize),
        *((*c).argv).offset(3 as libc::c_int as isize),
        &mut range,
    ) != 0 as libc::c_int
    {
        addReplyError(
            c,
            b"min or max is not a float\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    zobj = lookupKeyReadOrReply(c, key, shared.czero);
    if zobj.is_null() || checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut score: libc::c_double = 0.;
        eptr = zzlFirstInRange(zl, &mut range);
        if eptr.is_null() {
            addReply(c, shared.czero);
            return;
        }
        sptr = lpNext(zl, eptr);
        score = zzlGetScore(sptr);
        if zslValueLteMax(score, &mut range) != 0 {} else {
            _serverAssertWithInfo(
                c,
                zobj,
                b"zslValueLteMax(score,&range)\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                3313 as libc::c_int,
            );
            unreachable!();
        };
        while !eptr.is_null() {
            score = zzlGetScore(sptr);
            if zslValueLteMax(score, &mut range) == 0 {
                break;
            }
            count = count.wrapping_add(1);
            zzlNext(zl, &mut eptr, &mut sptr);
        }
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        let mut zn: *mut zskiplistNode = 0 as *mut zskiplistNode;
        let mut rank: libc::c_ulong = 0;
        zn = zslFirstInRange(zsl, &mut range);
        if !zn.is_null() {
            rank = zslGetRank(zsl, (*zn).score, (*zn).ele);
            count = ((*zsl).length)
                .wrapping_sub(rank.wrapping_sub(1 as libc::c_int as libc::c_ulong));
            zn = zslLastInRange(zsl, &mut range);
            if !zn.is_null() {
                rank = zslGetRank(zsl, (*zn).score, (*zn).ele);
                count = count.wrapping_sub(((*zsl).length).wrapping_sub(rank));
            }
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            3351 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    addReplyLongLong(c, count as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn zlexcountCommand(mut c: *mut client) {
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut range: zlexrangespec = zlexrangespec {
        min: 0 as *mut libc::c_char,
        max: 0 as *mut libc::c_char,
        minex: 0,
        maxex: 0,
    };
    let mut count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if zslParseLexRange(
        *((*c).argv).offset(2 as libc::c_int as isize),
        *((*c).argv).offset(3 as libc::c_int as isize),
        &mut range,
    ) != 0 as libc::c_int
    {
        addReplyError(
            c,
            b"min or max not valid string range item\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    zobj = lookupKeyReadOrReply(c, key, shared.czero);
    if zobj.is_null() || checkType(c, zobj, 3 as libc::c_int) != 0 {
        zslFreeLexRange(&mut range);
        return;
    }
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        eptr = zzlFirstInLexRange(zl, &mut range);
        if eptr.is_null() {
            zslFreeLexRange(&mut range);
            addReply(c, shared.czero);
            return;
        }
        sptr = lpNext(zl, eptr);
        if zzlLexValueLteMax(eptr, &mut range) != 0 {} else {
            _serverAssertWithInfo(
                c,
                zobj,
                b"zzlLexValueLteMax(eptr,&range)\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                3393 as libc::c_int,
            );
            unreachable!();
        };
        while !eptr.is_null() {
            if zzlLexValueLteMax(eptr, &mut range) == 0 {
                break;
            }
            count = count.wrapping_add(1);
            zzlNext(zl, &mut eptr, &mut sptr);
        }
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        let mut zn: *mut zskiplistNode = 0 as *mut zskiplistNode;
        let mut rank: libc::c_ulong = 0;
        zn = zslFirstInLexRange(zsl, &mut range);
        if !zn.is_null() {
            rank = zslGetRank(zsl, (*zn).score, (*zn).ele);
            count = ((*zsl).length)
                .wrapping_sub(rank.wrapping_sub(1 as libc::c_int as libc::c_ulong));
            zn = zslLastInLexRange(zsl, &mut range);
            if !zn.is_null() {
                rank = zslGetRank(zsl, (*zn).score, (*zn).ele);
                count = count.wrapping_sub(((*zsl).length).wrapping_sub(rank));
            }
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            3429 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    zslFreeLexRange(&mut range);
    addReplyLongLong(c, count as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn genericZrangebylexCommand(
    mut handler: *mut zrange_result_handler,
    mut range: *mut zlexrangespec,
    mut zobj: *mut robj,
    mut withscores: libc::c_int,
    mut offset: libc::c_long,
    mut limit: libc::c_long,
    mut reverse: libc::c_int,
) {
    let mut rangelen: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    ((*handler).beginResultEmission)
        .expect(
            "non-null function pointer",
        )(handler, -(1 as libc::c_int) as libc::c_long);
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = 0;
        let mut vlong: libc::c_longlong = 0;
        if reverse != 0 {
            eptr = zzlLastInLexRange(zl, range);
        } else {
            eptr = zzlFirstInLexRange(zl, range);
        }
        if !eptr.is_null() {
            sptr = lpNext(zl, eptr);
        }
        while !eptr.is_null()
            && {
                let fresh18 = offset;
                offset = offset - 1;
                fresh18 != 0
            }
        {
            if reverse != 0 {
                zzlPrev(zl, &mut eptr, &mut sptr);
            } else {
                zzlNext(zl, &mut eptr, &mut sptr);
            }
        }
        while !eptr.is_null()
            && {
                let fresh19 = limit;
                limit = limit - 1;
                fresh19 != 0
            }
        {
            let mut score: libc::c_double = 0 as libc::c_int as libc::c_double;
            if withscores != 0 {
                score = zzlGetScore(sptr);
            }
            if reverse != 0 {
                if zzlLexValueGteMin(eptr, range) == 0 {
                    break;
                }
            } else if zzlLexValueLteMax(eptr, range) == 0 {
                break;
            }
            vstr = lpGetValue(eptr, &mut vlen, &mut vlong);
            rangelen = rangelen.wrapping_add(1);
            if vstr.is_null() {
                ((*handler).emitResultFromLongLong)
                    .expect("non-null function pointer")(handler, vlong, score);
            } else {
                ((*handler).emitResultFromCBuffer)
                    .expect(
                        "non-null function pointer",
                    )(handler, vstr as *const libc::c_void, vlen as size_t, score);
            }
            if reverse != 0 {
                zzlPrev(zl, &mut eptr, &mut sptr);
            } else {
                zzlNext(zl, &mut eptr, &mut sptr);
            }
        }
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        let mut ln: *mut zskiplistNode = 0 as *mut zskiplistNode;
        if reverse != 0 {
            ln = zslLastInLexRange(zsl, range);
        } else {
            ln = zslFirstInLexRange(zsl, range);
        }
        while !ln.is_null()
            && {
                let fresh20 = offset;
                offset = offset - 1;
                fresh20 != 0
            }
        {
            if reverse != 0 {
                ln = (*ln).backward;
            } else {
                ln = (*((*ln).level).as_mut_ptr().offset(0 as libc::c_int as isize))
                    .forward;
            }
        }
        while !ln.is_null()
            && {
                let fresh21 = limit;
                limit = limit - 1;
                fresh21 != 0
            }
        {
            if reverse != 0 {
                if zslLexValueGteMin((*ln).ele, range) == 0 {
                    break;
                }
            } else if zslLexValueLteMax((*ln).ele, range) == 0 {
                break;
            }
            rangelen = rangelen.wrapping_add(1);
            ((*handler).emitResultFromCBuffer)
                .expect(
                    "non-null function pointer",
                )(
                handler,
                (*ln).ele as *const libc::c_void,
                sdslen((*ln).ele),
                (*ln).score,
            );
            if reverse != 0 {
                ln = (*ln).backward;
            } else {
                ln = (*((*ln).level).as_mut_ptr().offset(0 as libc::c_int as isize))
                    .forward;
            }
        }
    } else {
        _serverPanic(
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            3541 as libc::c_int,
            b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    ((*handler).finalizeResultEmission)
        .expect("non-null function pointer")(handler, rangelen);
}
#[no_mangle]
pub unsafe extern "C" fn zrangebylexCommand(mut c: *mut client) {
    let mut handler: zrange_result_handler = zrange_result_handler {
        type_0: ZRANGE_CONSUMER_TYPE_CLIENT,
        client: 0 as *mut client,
        dstkey: 0 as *mut robj,
        dstobj: 0 as *mut robj,
        userdata: 0 as *mut libc::c_void,
        withscores: 0,
        should_emit_array_length: 0,
        beginResultEmission: None,
        finalizeResultEmission: None,
        emitResultFromCBuffer: None,
        emitResultFromLongLong: None,
    };
    zrangeResultHandlerInit(&mut handler, c, ZRANGE_CONSUMER_TYPE_CLIENT);
    zrangeGenericCommand(
        &mut handler,
        1 as libc::c_int,
        0 as libc::c_int,
        ZRANGE_LEX,
        ZRANGE_DIRECTION_FORWARD,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zrevrangebylexCommand(mut c: *mut client) {
    let mut handler: zrange_result_handler = zrange_result_handler {
        type_0: ZRANGE_CONSUMER_TYPE_CLIENT,
        client: 0 as *mut client,
        dstkey: 0 as *mut robj,
        dstobj: 0 as *mut robj,
        userdata: 0 as *mut libc::c_void,
        withscores: 0,
        should_emit_array_length: 0,
        beginResultEmission: None,
        finalizeResultEmission: None,
        emitResultFromCBuffer: None,
        emitResultFromLongLong: None,
    };
    zrangeResultHandlerInit(&mut handler, c, ZRANGE_CONSUMER_TYPE_CLIENT);
    zrangeGenericCommand(
        &mut handler,
        1 as libc::c_int,
        0 as libc::c_int,
        ZRANGE_LEX,
        ZRANGE_DIRECTION_REVERSE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zrangeGenericCommand(
    mut handler: *mut zrange_result_handler,
    mut argc_start: libc::c_int,
    mut store: libc::c_int,
    mut rangetype: zrange_type,
    mut direction: zrange_direction,
) {
    let mut c: *mut client = (*handler).client;
    let mut key: *mut robj = *((*c).argv).offset(argc_start as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut range: zrangespec = zrangespec {
        min: 0.,
        max: 0.,
        minex: 0,
        maxex: 0,
    };
    let mut lexrange: zlexrangespec = zlexrangespec {
        min: 0 as *mut libc::c_char,
        max: 0 as *mut libc::c_char,
        minex: 0,
        maxex: 0,
    };
    let mut minidx: libc::c_int = argc_start + 1 as libc::c_int;
    let mut maxidx: libc::c_int = argc_start + 2 as libc::c_int;
    let mut opt_start: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut opt_end: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut opt_withscores: libc::c_int = 0 as libc::c_int;
    let mut opt_offset: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut opt_limit: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    let mut j: libc::c_int = argc_start + 3 as libc::c_int;
    while j < (*c).argc {
        let mut leftargs: libc::c_int = (*c).argc - j - 1 as libc::c_int;
        if store == 0
            && strcasecmp(
                (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                b"withscores\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            opt_withscores = 1 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"limit\0" as *const u8 as *const libc::c_char,
        ) == 0 && leftargs >= 2 as libc::c_int
        {
            if getLongFromObjectOrReply(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                &mut opt_offset,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
                || getLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset((j + 2 as libc::c_int) as isize),
                    &mut opt_limit,
                    0 as *const libc::c_char,
                ) != 0 as libc::c_int
            {
                return;
            }
            j += 2 as libc::c_int;
        } else if direction as libc::c_uint
            == ZRANGE_DIRECTION_AUTO as libc::c_int as libc::c_uint
            && strcasecmp(
                (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                b"rev\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            direction = ZRANGE_DIRECTION_REVERSE;
        } else if rangetype as libc::c_uint == ZRANGE_AUTO as libc::c_int as libc::c_uint
            && strcasecmp(
                (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                b"bylex\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            rangetype = ZRANGE_LEX;
        } else if rangetype as libc::c_uint == ZRANGE_AUTO as libc::c_int as libc::c_uint
            && strcasecmp(
                (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                b"byscore\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            rangetype = ZRANGE_SCORE;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        j += 1;
    }
    if direction as libc::c_uint == ZRANGE_DIRECTION_AUTO as libc::c_int as libc::c_uint
    {
        direction = ZRANGE_DIRECTION_FORWARD;
    }
    if rangetype as libc::c_uint == ZRANGE_AUTO as libc::c_int as libc::c_uint {
        rangetype = ZRANGE_RANK;
    }
    if opt_limit != -(1 as libc::c_int) as libc::c_long
        && rangetype as libc::c_uint == ZRANGE_RANK as libc::c_int as libc::c_uint
    {
        addReplyError(
            c,
            b"syntax error, LIMIT is only supported in combination with either BYSCORE or BYLEX\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if opt_withscores != 0
        && rangetype as libc::c_uint == ZRANGE_LEX as libc::c_int as libc::c_uint
    {
        addReplyError(
            c,
            b"syntax error, WITHSCORES not supported in combination with BYLEX\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if direction as libc::c_uint
        == ZRANGE_DIRECTION_REVERSE as libc::c_int as libc::c_uint
        && (ZRANGE_SCORE as libc::c_int as libc::c_uint == rangetype as libc::c_uint
            || ZRANGE_LEX as libc::c_int as libc::c_uint == rangetype as libc::c_uint)
    {
        let mut tmp: libc::c_int = maxidx;
        maxidx = minidx;
        minidx = tmp;
    }
    match rangetype as libc::c_uint {
        0 | 1 => {
            if getLongFromObjectOrReply(
                c,
                *((*c).argv).offset(minidx as isize),
                &mut opt_start,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
                || getLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset(maxidx as isize),
                    &mut opt_end,
                    0 as *const libc::c_char,
                ) != 0 as libc::c_int
            {
                return;
            }
        }
        2 => {
            if zslParseRange(
                *((*c).argv).offset(minidx as isize),
                *((*c).argv).offset(maxidx as isize),
                &mut range,
            ) != 0 as libc::c_int
            {
                addReplyError(
                    c,
                    b"min or max is not a float\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
        }
        3 => {
            if zslParseLexRange(
                *((*c).argv).offset(minidx as isize),
                *((*c).argv).offset(maxidx as isize),
                &mut lexrange,
            ) != 0 as libc::c_int
            {
                addReplyError(
                    c,
                    b"min or max not valid string range item\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
        }
        _ => {}
    }
    if opt_withscores != 0 || store != 0 {
        zrangeResultHandlerScoreEmissionEnable(handler);
    }
    zobj = lookupKeyRead((*c).db, key);
    if zobj.is_null() {
        if store != 0 {
            ((*handler).beginResultEmission)
                .expect(
                    "non-null function pointer",
                )(handler, -(1 as libc::c_int) as libc::c_long);
            ((*handler).finalizeResultEmission)
                .expect(
                    "non-null function pointer",
                )(handler, 0 as libc::c_int as size_t);
        } else {
            addReply(c, shared.emptyarray);
        }
    } else if !(checkType(c, zobj, 3 as libc::c_int) != 0) {
        match rangetype as libc::c_uint {
            0 | 1 => {
                genericZrangebyrankCommand(
                    handler,
                    zobj,
                    opt_start,
                    opt_end,
                    (opt_withscores != 0 || store != 0) as libc::c_int,
                    (direction as libc::c_uint
                        == ZRANGE_DIRECTION_REVERSE as libc::c_int as libc::c_uint)
                        as libc::c_int,
                );
            }
            2 => {
                genericZrangebyscoreCommand(
                    handler,
                    &mut range,
                    zobj,
                    opt_offset,
                    opt_limit,
                    (direction as libc::c_uint
                        == ZRANGE_DIRECTION_REVERSE as libc::c_int as libc::c_uint)
                        as libc::c_int,
                );
            }
            3 => {
                genericZrangebylexCommand(
                    handler,
                    &mut lexrange,
                    zobj,
                    (opt_withscores != 0 || store != 0) as libc::c_int,
                    opt_offset,
                    opt_limit,
                    (direction as libc::c_uint
                        == ZRANGE_DIRECTION_REVERSE as libc::c_int as libc::c_uint)
                        as libc::c_int,
                );
            }
            _ => {}
        }
    }
    if rangetype as libc::c_uint == ZRANGE_LEX as libc::c_int as libc::c_uint {
        zslFreeLexRange(&mut lexrange);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zcardCommand(mut c: *mut client) {
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    zobj = lookupKeyReadOrReply(c, key, shared.czero);
    if zobj.is_null() || checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    addReplyLongLong(c, zsetLength(zobj) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn zscoreCommand(mut c: *mut client) {
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut score: libc::c_double = 0.;
    zobj = lookupKeyReadOrReply(c, key, shared.null[(*c).resp as usize]);
    if zobj.is_null() || checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    if zsetScore(
        zobj,
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        &mut score,
    ) == -(1 as libc::c_int)
    {
        addReplyNull(c);
    } else {
        addReplyDouble(c, score);
    };
}
#[no_mangle]
pub unsafe extern "C" fn zmscoreCommand(mut c: *mut client) {
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut score: libc::c_double = 0.;
    zobj = lookupKeyRead((*c).db, key);
    if checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    addReplyArrayLen(c, ((*c).argc - 2 as libc::c_int) as libc::c_long);
    let mut j: libc::c_int = 2 as libc::c_int;
    while j < (*c).argc {
        if zobj.is_null()
            || zsetScore(zobj, (**((*c).argv).offset(j as isize)).ptr as sds, &mut score)
                == -(1 as libc::c_int)
        {
            addReplyNull(c);
        } else {
            addReplyDouble(c, score);
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn zrankGenericCommand(
    mut c: *mut client,
    mut reverse: libc::c_int,
) {
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut ele: *mut robj = *((*c).argv).offset(2 as libc::c_int as isize);
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut rank: libc::c_long = 0;
    zobj = lookupKeyReadOrReply(c, key, shared.null[(*c).resp as usize]);
    if zobj.is_null() || checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    if (*ele).encoding() as libc::c_int == 0 as libc::c_int
        || (*ele).encoding() as libc::c_int == 8 as libc::c_int
    {} else {
        _serverAssertWithInfo(
            c,
            ele,
            b"sdsEncodedObject(ele)\0" as *const u8 as *const libc::c_char,
            b"t_zset.c\0" as *const u8 as *const libc::c_char,
            3771 as libc::c_int,
        );
        unreachable!();
    };
    rank = zsetRank(zobj, (*ele).ptr as sds, reverse);
    if rank >= 0 as libc::c_int as libc::c_long {
        addReplyLongLong(c, rank as libc::c_longlong);
    } else {
        addReplyNull(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn zrankCommand(mut c: *mut client) {
    zrankGenericCommand(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zrevrankCommand(mut c: *mut client) {
    zrankGenericCommand(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zscanCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut cursor: libc::c_ulong = 0;
    if parseScanCursorOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut cursor,
    ) == -(1 as libc::c_int)
    {
        return;
    }
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.emptyscan,
    );
    if o.is_null() || checkType(c, o, 3 as libc::c_int) != 0 {
        return;
    }
    scanGenericCommand(c, o, cursor);
}
#[no_mangle]
pub unsafe extern "C" fn genericZpopCommand(
    mut c: *mut client,
    mut keyv: *mut *mut robj,
    mut keyc: libc::c_int,
    mut where_0: libc::c_int,
    mut emitkey: libc::c_int,
    mut count: libc::c_long,
    mut use_nested_array: libc::c_int,
    mut reply_nil_when_empty: libc::c_int,
    mut deleted: *mut libc::c_int,
) {
    let mut idx: libc::c_int = 0;
    let mut key: *mut robj = 0 as *mut robj;
    let mut zobj: *mut robj = 0 as *mut robj;
    let mut ele: sds = 0 as *mut libc::c_char;
    let mut score: libc::c_double = 0.;
    if !deleted.is_null() {
        *deleted = 0 as libc::c_int;
    }
    idx = 0 as libc::c_int;
    while idx < keyc {
        let fresh22 = idx;
        idx = idx + 1;
        key = *keyv.offset(fresh22 as isize);
        zobj = lookupKeyWrite((*c).db, key);
        if zobj.is_null() {
            continue;
        }
        if checkType(c, zobj, 3 as libc::c_int) != 0 {
            return;
        }
        break;
    }
    if zobj.is_null() {
        if reply_nil_when_empty != 0 {
            addReplyNullArray(c);
        } else {
            addReply(c, shared.emptyarray);
        }
        return;
    }
    if count == 0 as libc::c_int as libc::c_long {
        addReply(c, shared.emptyarray);
        return;
    }
    let mut result_count: libc::c_long = 0 as libc::c_int as libc::c_long;
    if count == -(1 as libc::c_int) as libc::c_long {
        count = 1 as libc::c_int as libc::c_long;
    }
    let mut llen: libc::c_long = zsetLength(zobj) as libc::c_long;
    let mut rangelen: libc::c_long = if count > llen { llen } else { count };
    if use_nested_array == 0 && emitkey == 0 {
        addReplyArrayLen(c, rangelen * 2 as libc::c_int as libc::c_long);
    } else if use_nested_array != 0 && emitkey == 0 {
        addReplyArrayLen(c, rangelen);
    } else if use_nested_array == 0 && emitkey != 0 {
        addReplyArrayLen(
            c,
            rangelen * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long,
        );
        addReplyBulk(c, key);
    } else if use_nested_array != 0 && emitkey != 0 {
        addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
        addReplyBulk(c, key);
        addReplyArrayLen(c, rangelen);
    }
    loop {
        if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
            let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
            let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut vlen: libc::c_uint = 0;
            let mut vlong: libc::c_longlong = 0;
            eptr = lpSeek(
                zl,
                (if where_0 == 1 as libc::c_int {
                    -(2 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as libc::c_long,
            );
            if !eptr.is_null() {} else {
                _serverAssertWithInfo(
                    c,
                    zobj,
                    b"eptr != NULL\0" as *const u8 as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    3891 as libc::c_int,
                );
                unreachable!();
            };
            vstr = lpGetValue(eptr, &mut vlen, &mut vlong);
            if vstr.is_null() {
                ele = sdsfromlonglong(vlong);
            } else {
                ele = sdsnewlen(vstr as *const libc::c_void, vlen as size_t);
            }
            sptr = lpNext(zl, eptr);
            if !sptr.is_null() {} else {
                _serverAssertWithInfo(
                    c,
                    zobj,
                    b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    3900 as libc::c_int,
                );
                unreachable!();
            };
            score = zzlGetScore(sptr);
        } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
            let mut zs: *mut zset = (*zobj).ptr as *mut zset;
            let mut zsl: *mut zskiplist = (*zs).zsl;
            let mut zln: *mut zskiplistNode = 0 as *mut zskiplistNode;
            zln = if where_0 == 1 as libc::c_int {
                (*zsl).tail
            } else {
                (*((*(*zsl).header).level)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .forward
            };
            if !zln.is_null() {} else {
                _serverAssertWithInfo(
                    c,
                    zobj,
                    b"zln != NULL\0" as *const u8 as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    3912 as libc::c_int,
                );
                unreachable!();
            };
            ele = sdsdup((*zln).ele);
            score = (*zln).score;
        } else {
            _serverPanic(
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                3916 as libc::c_int,
                b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
        if zsetDel(zobj, ele) != 0 {} else {
            _serverAssertWithInfo(
                c,
                zobj,
                b"zsetDel(zobj,ele)\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                3919 as libc::c_int,
            );
            unreachable!();
        };
        server.dirty += 1;
        if result_count == 0 as libc::c_int as libc::c_long {
            let mut events: [*mut libc::c_char; 2] = [
                b"zpopmin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"zpopmax\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ];
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 7 as libc::c_int,
                events[where_0 as usize],
                key,
                (*(*c).db).id,
            );
            signalModifiedKey(c, (*c).db, key);
        }
        if use_nested_array != 0 {
            addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
        }
        addReplyBulkCBuffer(c, ele as *const libc::c_void, sdslen(ele));
        addReplyDouble(c, score);
        sdsfree(ele);
        result_count += 1;
        rangelen -= 1;
        if !(rangelen != 0) {
            break;
        }
    }
    if zsetLength(zobj) == 0 as libc::c_int as libc::c_ulong {
        if !deleted.is_null() {
            *deleted = 1 as libc::c_int;
        }
        dbDelete((*c).db, key);
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 2 as libc::c_int,
            b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key,
            (*(*c).db).id,
        );
    }
    if (*(*c).cmd).proc_0
        == Some(zmpopCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        let mut count_obj: *mut robj = createStringObjectFromLongLong(
            (if count > llen { llen } else { count }) as libc::c_longlong,
        );
        rewriteClientCommandVector(
            c,
            3 as libc::c_int,
            if where_0 == 1 as libc::c_int { shared.zpopmax } else { shared.zpopmin },
            key,
            count_obj,
        );
        decrRefCount(count_obj);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zpopMinMaxCommand(
    mut c: *mut client,
    mut where_0: libc::c_int,
) {
    if (*c).argc > 3 as libc::c_int {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    let mut count: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    if (*c).argc == 3 as libc::c_int
        && getPositiveLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut count,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        return;
    }
    let mut use_nested_array: libc::c_int = ((*c).resp > 2 as libc::c_int
        && count != -(1 as libc::c_int) as libc::c_long) as libc::c_int;
    genericZpopCommand(
        c,
        &mut *((*c).argv).offset(1 as libc::c_int as isize),
        1 as libc::c_int,
        where_0,
        0 as libc::c_int,
        count,
        use_nested_array,
        0 as libc::c_int,
        0 as *mut libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zpopminCommand(mut c: *mut client) {
    zpopMinMaxCommand(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zpopmaxCommand(mut c: *mut client) {
    zpopMinMaxCommand(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn blockingGenericZpopCommand(
    mut c: *mut client,
    mut keys: *mut *mut robj,
    mut numkeys: libc::c_int,
    mut where_0: libc::c_int,
    mut timeout_idx: libc::c_int,
    mut count: libc::c_long,
    mut use_nested_array: libc::c_int,
    mut reply_nil_when_empty: libc::c_int,
) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut key: *mut robj = 0 as *mut robj;
    let mut timeout: mstime_t = 0;
    let mut j: libc::c_int = 0;
    if getTimeoutFromObjectOrReply(
        c,
        *((*c).argv).offset(timeout_idx as isize),
        &mut timeout,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return;
    }
    j = 0 as libc::c_int;
    while j < numkeys {
        key = *keys.offset(j as isize);
        o = lookupKeyWrite((*c).db, key);
        if !o.is_null() {
            if checkType(c, o, 3 as libc::c_int) != 0 {
                return;
            }
            let mut llen: libc::c_long = zsetLength(o) as libc::c_long;
            if !(llen == 0 as libc::c_int as libc::c_long) {
                genericZpopCommand(
                    c,
                    &mut key,
                    1 as libc::c_int,
                    where_0,
                    1 as libc::c_int,
                    count,
                    use_nested_array,
                    reply_nil_when_empty,
                    0 as *mut libc::c_int,
                );
                if count == -(1 as libc::c_int) as libc::c_long {
                    rewriteClientCommandVector(
                        c,
                        2 as libc::c_int,
                        if where_0 == 1 as libc::c_int {
                            shared.zpopmax
                        } else {
                            shared.zpopmin
                        },
                        key,
                    );
                } else {
                    let mut count_obj: *mut robj = createStringObjectFromLongLong(
                        (if count > llen { llen } else { count }) as libc::c_longlong,
                    );
                    rewriteClientCommandVector(
                        c,
                        3 as libc::c_int,
                        if where_0 == 1 as libc::c_int {
                            shared.zpopmax
                        } else {
                            shared.zpopmin
                        },
                        key,
                        count_obj,
                    );
                    decrRefCount(count_obj);
                }
                return;
            }
        }
        j += 1;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 41 as libc::c_int
        != 0
    {
        addReplyNullArray(c);
        return;
    }
    let mut pos: blockPos = {
        let mut init = blockPos {
            wherefrom: where_0,
            whereto: 0,
        };
        init
    };
    blockForKeys(
        c,
        5 as libc::c_int,
        keys,
        numkeys,
        count,
        timeout,
        0 as *mut robj,
        &mut pos,
        0 as *mut streamID,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bzpopminCommand(mut c: *mut client) {
    blockingGenericZpopCommand(
        c,
        ((*c).argv).offset(1 as libc::c_int as isize),
        (*c).argc - 2 as libc::c_int,
        0 as libc::c_int,
        (*c).argc - 1 as libc::c_int,
        -(1 as libc::c_int) as libc::c_long,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bzpopmaxCommand(mut c: *mut client) {
    blockingGenericZpopCommand(
        c,
        ((*c).argv).offset(1 as libc::c_int as isize),
        (*c).argc - 2 as libc::c_int,
        1 as libc::c_int,
        (*c).argc - 1 as libc::c_int,
        -(1 as libc::c_int) as libc::c_long,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn zrandmemberReplyWithListpack(
    mut c: *mut client,
    mut count: libc::c_uint,
    mut keys: *mut listpackEntry,
    mut vals: *mut listpackEntry,
) {
    let mut i: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while i < count as libc::c_ulong {
        if !vals.is_null() && (*c).resp > 2 as libc::c_int {
            addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
        }
        if !((*keys.offset(i as isize)).sval).is_null() {
            addReplyBulkCBuffer(
                c,
                (*keys.offset(i as isize)).sval as *const libc::c_void,
                (*keys.offset(i as isize)).slen as size_t,
            );
        } else {
            addReplyBulkLongLong(c, (*keys.offset(i as isize)).lval);
        }
        if !vals.is_null() {
            if !((*vals.offset(i as isize)).sval).is_null() {
                addReplyDouble(
                    c,
                    zzlStrtod(
                        (*vals.offset(i as isize)).sval,
                        (*vals.offset(i as isize)).slen,
                    ),
                );
            } else {
                addReplyDouble(c, (*vals.offset(i as isize)).lval as libc::c_double);
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zrandmemberWithCountCommand(
    mut c: *mut client,
    mut l: libc::c_long,
    mut withscores: libc::c_int,
) {
    let mut count: libc::c_ulong = 0;
    let mut size: libc::c_ulong = 0;
    let mut uniq: libc::c_int = 1 as libc::c_int;
    let mut zsetobj: *mut robj = 0 as *mut robj;
    zsetobj = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.emptyarray,
    );
    if zsetobj.is_null() || checkType(c, zsetobj, 3 as libc::c_int) != 0 {
        return;
    }
    size = zsetLength(zsetobj);
    if l >= 0 as libc::c_int as libc::c_long {
        count = l as libc::c_ulong;
    } else {
        count = -l as libc::c_ulong;
        uniq = 0 as libc::c_int;
    }
    if count == 0 as libc::c_int as libc::c_ulong {
        addReply(c, shared.emptyarray);
        return;
    }
    if uniq == 0 || count == 1 as libc::c_int as libc::c_ulong {
        if withscores != 0 && (*c).resp == 2 as libc::c_int {
            addReplyArrayLen(
                c,
                count.wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_long,
            );
        } else {
            addReplyArrayLen(c, count as libc::c_long);
        }
        if (*zsetobj).encoding() as libc::c_int == 7 as libc::c_int {
            let mut zs: *mut zset = (*zsetobj).ptr as *mut zset;
            loop {
                let fresh23 = count;
                count = count.wrapping_sub(1);
                if !(fresh23 != 0) {
                    break;
                }
                let mut de: *mut dictEntry = dictGetFairRandomKey((*zs).dict);
                let mut key: sds = (*de).key as sds;
                if withscores != 0 && (*c).resp > 2 as libc::c_int {
                    addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                }
                addReplyBulkCBuffer(c, key as *const libc::c_void, sdslen(key));
                if withscores != 0 {
                    addReplyDouble(c, *((*de).v.val as *mut libc::c_double));
                }
                if (*c).flags
                    & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0
                {
                    break;
                }
            }
        } else if (*zsetobj).encoding() as libc::c_int == 11 as libc::c_int {
            let mut keys: *mut listpackEntry = 0 as *mut listpackEntry;
            let mut vals: *mut listpackEntry = 0 as *mut listpackEntry;
            let mut limit: libc::c_ulong = 0;
            let mut sample_count: libc::c_ulong = 0;
            limit = if count > 1000 as libc::c_int as libc::c_ulong {
                1000 as libc::c_int as libc::c_ulong
            } else {
                count
            };
            keys = zmalloc(
                (core::mem::size_of::<listpackEntry>() as libc::c_ulong)
                    .wrapping_mul(limit),
            ) as *mut listpackEntry;
            if withscores != 0 {
                vals = zmalloc(
                    (core::mem::size_of::<listpackEntry>() as libc::c_ulong)
                        .wrapping_mul(limit),
                ) as *mut listpackEntry;
            }
            while count != 0 {
                sample_count = if count > limit { limit } else { count };
                count = count.wrapping_sub(sample_count);
                lpRandomPairs(
                    (*zsetobj).ptr as *mut libc::c_uchar,
                    sample_count as libc::c_uint,
                    keys,
                    vals,
                );
                zrandmemberReplyWithListpack(
                    c,
                    sample_count as libc::c_uint,
                    keys,
                    vals,
                );
                if (*c).flags
                    & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0
                {
                    break;
                }
            }
            zfree(keys as *mut libc::c_void);
            zfree(vals as *mut libc::c_void);
        }
        return;
    }
    let mut src: zsetopsrc = zsetopsrc {
        subject: 0 as *mut robj,
        type_0: 0,
        encoding: 0,
        weight: 0.,
        iter: C2RustUnnamed_7 {
            set: _iterset {
                is: C2RustUnnamed_11 {
                    is: 0 as *mut intset,
                    ii: 0,
                },
            },
        },
    };
    let mut zval: zsetopval = zsetopval {
        flags: 0,
        _buf: [0; 32],
        ele: 0 as *mut libc::c_char,
        estr: 0 as *mut libc::c_uchar,
        elen: 0,
        ell: 0,
        score: 0.,
    };
    src.subject = zsetobj;
    src.type_0 = (*zsetobj).type_0() as libc::c_int;
    src.encoding = (*zsetobj).encoding() as libc::c_int;
    zuiInitIterator(&mut src);
    memset(
        &mut zval as *mut zsetopval as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<zsetopval>() as libc::c_ulong,
    );
    let mut reply_size: libc::c_long = (if count < size { count } else { size })
        as libc::c_long;
    if withscores != 0 && (*c).resp == 2 as libc::c_int {
        addReplyArrayLen(c, reply_size * 2 as libc::c_int as libc::c_long);
    } else {
        addReplyArrayLen(c, reply_size);
    }
    if count >= size {
        while zuiNext(&mut src, &mut zval) != 0 {
            if withscores != 0 && (*c).resp > 2 as libc::c_int {
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
            }
            addReplyBulkSds(c, zuiNewSdsFromValue(&mut zval));
            if withscores != 0 {
                addReplyDouble(c, zval.score);
            }
        }
        zuiClearIterator(&mut src);
        return;
    }
    if count.wrapping_mul(3 as libc::c_int as libc::c_ulong) > size {
        let mut d: *mut dict = dictCreate(&mut sdsReplyDictType);
        dictExpand(d, size);
        while zuiNext(&mut src, &mut zval) != 0 {
            let mut key_0: sds = zuiNewSdsFromValue(&mut zval);
            let mut de_0: *mut dictEntry = dictAddRaw(
                d,
                key_0 as *mut libc::c_void,
                0 as *mut *mut dictEntry,
            );
            if !de_0.is_null() {} else {
                _serverAssert(
                    b"de\0" as *const u8 as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    4199 as libc::c_int,
                );
                unreachable!();
            };
            if withscores != 0 {
                (*de_0).v.d = zval.score;
            }
        }
        if ((*d).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*d).ht_used[1 as libc::c_int as usize]) == size
        {} else {
            _serverAssert(
                b"dictSize(d) == size\0" as *const u8 as *const libc::c_char,
                b"t_zset.c\0" as *const u8 as *const libc::c_char,
                4203 as libc::c_int,
            );
            unreachable!();
        };
        while size > count {
            let mut de_1: *mut dictEntry = 0 as *mut dictEntry;
            de_1 = dictGetFairRandomKey(d);
            dictUnlink(d, (*de_1).key);
            sdsfree((*de_1).key as sds);
            dictFreeUnlinkedEntry(d, de_1);
            size = size.wrapping_sub(1);
        }
        let mut di: *mut dictIterator = 0 as *mut dictIterator;
        let mut de_2: *mut dictEntry = 0 as *mut dictEntry;
        di = dictGetIterator(d);
        loop {
            de_2 = dictNext(di);
            if de_2.is_null() {
                break;
            }
            if withscores != 0 && (*c).resp > 2 as libc::c_int {
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
            }
            addReplyBulkSds(c, (*de_2).key as sds);
            if withscores != 0 {
                addReplyDouble(c, (*de_2).v.d);
            }
        }
        dictReleaseIterator(di);
        dictRelease(d);
    } else {
        if (*zsetobj).encoding() as libc::c_int == 11 as libc::c_int {
            let mut keys_0: *mut listpackEntry = 0 as *mut listpackEntry;
            let mut vals_0: *mut listpackEntry = 0 as *mut listpackEntry;
            keys_0 = zmalloc(
                (core::mem::size_of::<listpackEntry>() as libc::c_ulong)
                    .wrapping_mul(count),
            ) as *mut listpackEntry;
            if withscores != 0 {
                vals_0 = zmalloc(
                    (core::mem::size_of::<listpackEntry>() as libc::c_ulong)
                        .wrapping_mul(count),
                ) as *mut listpackEntry;
            }
            if lpRandomPairsUnique(
                (*zsetobj).ptr as *mut libc::c_uchar,
                count as libc::c_uint,
                keys_0,
                vals_0,
            ) as libc::c_ulong == count
            {} else {
                _serverAssert(
                    b"lpRandomPairsUnique(zsetobj->ptr, count, keys, vals) == count\0"
                        as *const u8 as *const libc::c_char,
                    b"t_zset.c\0" as *const u8 as *const libc::c_char,
                    4243 as libc::c_int,
                );
                unreachable!();
            };
            zrandmemberReplyWithListpack(c, count as libc::c_uint, keys_0, vals_0);
            zfree(keys_0 as *mut libc::c_void);
            zfree(vals_0 as *mut libc::c_void);
            zuiClearIterator(&mut src);
            return;
        }
        let mut added: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut d_0: *mut dict = dictCreate(&mut hashDictType);
        dictExpand(d_0, count);
        while added < count {
            let mut key_1: listpackEntry = listpackEntry {
                sval: 0 as *mut libc::c_uchar,
                slen: 0,
                lval: 0,
            };
            let mut score: libc::c_double = 0.;
            zsetTypeRandomElement(
                zsetobj,
                size,
                &mut key_1,
                if withscores != 0 { &mut score } else { 0 as *mut libc::c_double },
            );
            let mut skey: sds = zsetSdsFromListpackEntry(&mut key_1);
            if dictAdd(d_0, skey as *mut libc::c_void, 0 as *mut libc::c_void)
                != 0 as libc::c_int
            {
                sdsfree(skey);
            } else {
                added = added.wrapping_add(1);
                if withscores != 0 && (*c).resp > 2 as libc::c_int {
                    addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                }
                zsetReplyFromListpackEntry(c, &mut key_1);
                if withscores != 0 {
                    addReplyDouble(c, score);
                }
            }
        }
        dictRelease(d_0);
    }
    zuiClearIterator(&mut src);
}
#[no_mangle]
pub unsafe extern "C" fn zrandmemberCommand(mut c: *mut client) {
    let mut l: libc::c_long = 0;
    let mut withscores: libc::c_int = 0 as libc::c_int;
    let mut zset: *mut robj = 0 as *mut robj;
    let mut ele: listpackEntry = listpackEntry {
        sval: 0 as *mut libc::c_uchar,
        slen: 0,
        lval: 0,
    };
    if (*c).argc >= 3 as libc::c_int {
        if getLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut l,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        if (*c).argc > 4 as libc::c_int
            || (*c).argc == 4 as libc::c_int
                && strcasecmp(
                    (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                        as *const libc::c_char,
                    b"withscores\0" as *const u8 as *const libc::c_char,
                ) != 0
        {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        } else {
            if (*c).argc == 4 as libc::c_int {
                withscores = 1 as libc::c_int;
                if l
                    < (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        / 2 as libc::c_int as libc::c_long
                    || l
                        > 9223372036854775807 as libc::c_long
                            / 2 as libc::c_int as libc::c_long
                {
                    addReplyError(
                        c,
                        b"value is out of range\0" as *const u8 as *const libc::c_char,
                    );
                    return;
                }
            }
        }
        zrandmemberWithCountCommand(c, l, withscores);
        return;
    }
    zset = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.null[(*c).resp as usize],
    );
    if zset.is_null() || checkType(c, zset, 3 as libc::c_int) != 0 {
        return;
    }
    zsetTypeRandomElement(zset, zsetLength(zset), &mut ele, 0 as *mut libc::c_double);
    zsetReplyFromListpackEntry(c, &mut ele);
}
#[no_mangle]
pub unsafe extern "C" fn zmpopGenericCommand(
    mut c: *mut client,
    mut numkeys_idx: libc::c_int,
    mut is_block: libc::c_int,
) {
    let mut j: libc::c_long = 0;
    let mut numkeys: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut where_0: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    if getRangeLongFromObjectOrReply(
        c,
        *((*c).argv).offset(numkeys_idx as isize),
        1 as libc::c_int as libc::c_long,
        9223372036854775807 as libc::c_long,
        &mut numkeys,
        b"numkeys should be greater than 0\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    let mut where_idx: libc::c_long = numkeys_idx as libc::c_long + numkeys
        + 1 as libc::c_int as libc::c_long;
    if where_idx >= (*c).argc as libc::c_long {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    if strcasecmp(
        (**((*c).argv).offset(where_idx as isize)).ptr as *const libc::c_char,
        b"MIN\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        where_0 = 0 as libc::c_int;
    } else if strcasecmp(
        (**((*c).argv).offset(where_idx as isize)).ptr as *const libc::c_char,
        b"MAX\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        where_0 = 1 as libc::c_int;
    } else {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    j = where_idx + 1 as libc::c_int as libc::c_long;
    while j < (*c).argc as libc::c_long {
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
            as *mut libc::c_char;
        let mut moreargs: libc::c_int = (((*c).argc - 1 as libc::c_int) as libc::c_long
            - j) as libc::c_int;
        if count == -(1 as libc::c_int) as libc::c_long
            && strcasecmp(opt, b"COUNT\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            j += 1;
            if getRangeLongFromObjectOrReply(
                c,
                *((*c).argv).offset(j as isize),
                1 as libc::c_int as libc::c_long,
                9223372036854775807 as libc::c_long,
                &mut count,
                b"count should be greater than 0\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        j += 1;
    }
    if count == -(1 as libc::c_int) as libc::c_long {
        count = 1 as libc::c_int as libc::c_long;
    }
    if is_block != 0 {
        blockingGenericZpopCommand(
            c,
            ((*c).argv).offset(numkeys_idx as isize).offset(1 as libc::c_int as isize),
            numkeys as libc::c_int,
            where_0,
            1 as libc::c_int,
            count,
            1 as libc::c_int,
            1 as libc::c_int,
        );
    } else {
        genericZpopCommand(
            c,
            ((*c).argv).offset(numkeys_idx as isize).offset(1 as libc::c_int as isize),
            numkeys as libc::c_int,
            where_0,
            1 as libc::c_int,
            count,
            1 as libc::c_int,
            1 as libc::c_int,
            0 as *mut libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn zmpopCommand(mut c: *mut client) {
    zmpopGenericCommand(c, 1 as libc::c_int, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bzmpopCommand(mut c: *mut client) {
    zmpopGenericCommand(c, 2 as libc::c_int, 1 as libc::c_int);
}
