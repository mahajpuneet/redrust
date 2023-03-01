extern crate c2rust_bitfields;
extern crate libc;
extern crate core;

extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    static mut SDS_NOINIT: *const libc::c_char;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn __errno_location() -> *mut libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn ztrymalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn string2ll(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_longlong,
    ) -> libc::c_int;
    fn string2ull(s: *const libc::c_char, value: *mut libc::c_ulonglong) -> libc::c_int;
    static mut raxNotFound: *mut libc::c_void;
    fn raxNew() -> *mut rax;
    fn raxInsert(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        data: *mut libc::c_void,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxTryInsert(
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
        free_callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn raxStart(it: *mut raxIterator, rt: *mut rax);
    fn raxSeek(
        it: *mut raxIterator,
        op: *const libc::c_char,
        ele: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn raxNext(it: *mut raxIterator) -> libc::c_int;
    fn raxPrev(it: *mut raxIterator) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    fn raxEOF(it: *mut raxIterator) -> libc::c_int;
    fn raxSize(rax: *mut rax) -> uint64_t;
    fn intrev64(v: uint64_t) -> uint64_t;
    fn lpNew(capacity: size_t) -> *mut libc::c_uchar;
    fn lpFree(lp: *mut libc::c_uchar);
    fn lpShrinkToFit(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpAppend(
        lp: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> *mut libc::c_uchar;
    fn lpAppendInteger(
        lp: *mut libc::c_uchar,
        lval: libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpReplaceInteger(
        lp: *mut libc::c_uchar,
        p: *mut *mut libc::c_uchar,
        lval: libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpGet(
        p: *mut libc::c_uchar,
        count: *mut int64_t,
        intbuf: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpFirst(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpLast(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpNext(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpPrev(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpBytes(lp: *mut libc::c_uchar) -> size_t;
    fn lpValidateIntegrity(
        lp: *mut libc::c_uchar,
        size: size_t,
        deep: libc::c_int,
        entry_cb: listpackValidateEntryCB,
        cb_userdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn lpValidateFirst(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpValidateNext(
        lp: *mut libc::c_uchar,
        pp: *mut *mut libc::c_uchar,
        lpbytes: size_t,
    ) -> libc::c_int;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn decrRefCount(o: *mut robj);
    fn alsoPropagate(
        dbid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
        target: libc::c_int,
    );
    fn createStringObjectFromLongLong(value: libc::c_longlong) -> *mut robj;
    static mut shared: sharedObjectsStruct;
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn mstime() -> libc::c_longlong;
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn addReplyNullArray(c: *mut client);
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    static mut server: redisServer;
    fn createStreamObject() -> *mut robj;
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn setDeferredMapLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn addReplyNull(c: *mut client);
    fn addReplyBulk(c: *mut client, obj: *mut robj);
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReply(c: *mut client, obj: *mut robj);
    fn setDeferredReplyBulkSds(c: *mut client, node: *mut libc::c_void, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyErrorArity(c: *mut client);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn rewriteClientCommandArgument(c: *mut client, i: libc::c_int, newval: *mut robj);
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn incrRefCount(o: *mut robj);
    fn getRangeLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        min: libc::c_long,
        max: libc::c_long,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn checkType(c: *mut client, o: *mut robj, type_0: libc::c_int) -> libc::c_int;
    fn getLongLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_longlong,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn preventCommandPropagation(c: *mut client);
    fn mustObeyClient(c: *mut client) -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn lookupKeyRead(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyWrite(db: *mut redisDb, key: *mut robj) -> *mut robj;
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
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn getTimeoutFromObjectOrReply(
        c: *mut client,
        object: *mut robj,
        timeout: *mut mstime_t,
        unit: libc::c_int,
    ) -> libc::c_int;
    fn signalKeyAsReady(db: *mut redisDb, key: *mut robj, type_0: libc::c_int);
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
pub type intptr_t = libc::c_long;
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
pub type listpackValidateEntryCB = Option::<
    unsafe extern "C" fn(
        *mut libc::c_uchar,
        libc::c_uint,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
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
pub struct streamIterator {
    pub stream: *mut stream,
    pub master_id: streamID,
    pub master_fields_count: uint64_t,
    pub master_fields_start: *mut libc::c_uchar,
    pub master_fields_ptr: *mut libc::c_uchar,
    pub entry_flags: libc::c_int,
    pub rev: libc::c_int,
    pub skip_tombstones: libc::c_int,
    pub start_key: [uint64_t; 2],
    pub end_key: [uint64_t; 2],
    pub ri: raxIterator,
    pub lp: *mut libc::c_uchar,
    pub lp_ele: *mut libc::c_uchar,
    pub lp_flags: *mut libc::c_uchar,
    pub field_buf: [libc::c_uchar; 21],
    pub value_buf: [libc::c_uchar; 21],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamPropInfo {
    pub keyname: *mut robj,
    pub groupname: *mut robj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamAddTrimArgs {
    pub id: streamID,
    pub id_given: libc::c_int,
    pub seq_given: libc::c_int,
    pub no_mkstream: libc::c_int,
    pub trim_strategy: libc::c_int,
    pub trim_strategy_arg_idx: libc::c_int,
    pub approx_trim: libc::c_int,
    pub limit: libc::c_longlong,
    pub maxlen: libc::c_longlong,
    pub minid: streamID,
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
#[no_mangle]
pub unsafe extern "C" fn streamNew() -> *mut stream {
    let mut s: *mut stream = zmalloc(core::mem::size_of::<stream>() as libc::c_ulong)
        as *mut stream;
    (*s).rax = raxNew();
    (*s).length = 0 as libc::c_int as uint64_t;
    (*s).first_id.ms = 0 as libc::c_int as uint64_t;
    (*s).first_id.seq = 0 as libc::c_int as uint64_t;
    (*s).last_id.ms = 0 as libc::c_int as uint64_t;
    (*s).last_id.seq = 0 as libc::c_int as uint64_t;
    (*s).max_deleted_entry_id.seq = 0 as libc::c_int as uint64_t;
    (*s).max_deleted_entry_id.ms = 0 as libc::c_int as uint64_t;
    (*s).entries_added = 0 as libc::c_int as uint64_t;
    (*s).cgroups = 0 as *mut rax;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn freeStream(mut s: *mut stream) {
    raxFreeWithCallback(
        (*s).rax,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_uchar) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(lpFree as unsafe extern "C" fn(*mut libc::c_uchar) -> ())),
    );
    if !((*s).cgroups).is_null() {
        raxFreeWithCallback(
            (*s).cgroups,
            core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut streamCG) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(Some(streamFreeCG as unsafe extern "C" fn(*mut streamCG) -> ())),
        );
    }
    zfree(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn streamLength(mut subject: *const robj) -> libc::c_ulong {
    let mut s: *mut stream = (*subject).ptr as *mut stream;
    return (*s).length;
}
#[no_mangle]
pub unsafe extern "C" fn streamIncrID(mut id: *mut streamID) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*id).seq == 18446744073709551615 as libc::c_ulong {
        if (*id).ms == 18446744073709551615 as libc::c_ulong {
            (*id).seq = 0 as libc::c_int as uint64_t;
            (*id).ms = (*id).seq;
            ret = -(1 as libc::c_int);
        } else {
            (*id).ms = ((*id).ms).wrapping_add(1);
            (*id).seq = 0 as libc::c_int as uint64_t;
        }
    } else {
        (*id).seq = ((*id).seq).wrapping_add(1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn streamDecrID(mut id: *mut streamID) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*id).seq == 0 as libc::c_int as libc::c_ulong {
        if (*id).ms == 0 as libc::c_int as libc::c_ulong {
            (*id).seq = 18446744073709551615 as libc::c_ulong;
            (*id).ms = (*id).seq;
            ret = -(1 as libc::c_int);
        } else {
            (*id).ms = ((*id).ms).wrapping_sub(1);
            (*id).seq = 18446744073709551615 as libc::c_ulong;
        }
    } else {
        (*id).seq = ((*id).seq).wrapping_sub(1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn streamNextID(
    mut last_id: *mut streamID,
    mut new_id: *mut streamID,
) {
    let mut ms: uint64_t = mstime() as uint64_t;
    if ms > (*last_id).ms {
        (*new_id).ms = ms;
        (*new_id).seq = 0 as libc::c_int as uint64_t;
    } else {
        *new_id = *last_id;
        streamIncrID(new_id);
    };
}
#[no_mangle]
pub unsafe extern "C" fn streamDup(mut o: *mut robj) -> *mut robj {
    let mut sobj: *mut robj = 0 as *mut robj;
    if (*o).type_0() as libc::c_int == 6 as libc::c_int {} else {
        _serverAssert(
            b"o->type == OBJ_STREAM\0" as *const u8 as *const libc::c_char,
            b"t_stream.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int,
        );
        unreachable!();
    };
    match (*o).encoding() as libc::c_int {
        10 => {
            sobj = createStreamObject();
        }
        _ => {
            _serverPanic(
                b"t_stream.c\0" as *const u8 as *const libc::c_char,
                166 as libc::c_int,
                b"Wrong encoding.\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    let mut s: *mut stream = 0 as *mut stream;
    let mut new_s: *mut stream = 0 as *mut stream;
    s = (*o).ptr as *mut stream;
    new_s = (*sobj).ptr as *mut stream;
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
    let mut rax_key: [uint64_t; 2] = [0; 2];
    raxStart(&mut ri, (*s).rax);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    let mut lp_bytes: size_t = 0 as libc::c_int as size_t;
    let mut lp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    while raxNext(&mut ri) != 0 {
        lp = ri.data as *mut libc::c_uchar;
        lp_bytes = lpBytes(lp);
        let mut new_lp: *mut libc::c_uchar = zmalloc(lp_bytes) as *mut libc::c_uchar;
        memcpy(new_lp as *mut libc::c_void, lp as *const libc::c_void, lp_bytes);
        memcpy(
            rax_key.as_mut_ptr() as *mut libc::c_void,
            ri.key as *const libc::c_void,
            core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
        );
        raxInsert(
            (*new_s).rax,
            &mut rax_key as *mut [uint64_t; 2] as *mut libc::c_uchar,
            core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
            new_lp as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    (*new_s).length = (*s).length;
    (*new_s).first_id = (*s).first_id;
    (*new_s).last_id = (*s).last_id;
    (*new_s).max_deleted_entry_id = (*s).max_deleted_entry_id;
    (*new_s).entries_added = (*s).entries_added;
    raxStop(&mut ri);
    if ((*s).cgroups).is_null() {
        return sobj;
    }
    let mut ri_cgroups: raxIterator = raxIterator {
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
    raxStart(&mut ri_cgroups, (*s).cgroups);
    raxSeek(
        &mut ri_cgroups,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri_cgroups) != 0 {
        let mut cg: *mut streamCG = ri_cgroups.data as *mut streamCG;
        let mut new_cg: *mut streamCG = streamCreateCG(
            new_s,
            ri_cgroups.key as *mut libc::c_char,
            ri_cgroups.key_len,
            &mut (*cg).last_id,
            (*cg).entries_read,
        );
        if !new_cg.is_null() {} else {
            _serverAssert(
                b"new_cg != NULL\0" as *const u8 as *const libc::c_char,
                b"t_stream.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int,
            );
            unreachable!();
        };
        let mut ri_cg_pel: raxIterator = raxIterator {
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
        raxStart(&mut ri_cg_pel, (*cg).pel);
        raxSeek(
            &mut ri_cg_pel,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        while raxNext(&mut ri_cg_pel) != 0 {
            let mut nack: *mut streamNACK = ri_cg_pel.data as *mut streamNACK;
            let mut new_nack: *mut streamNACK = streamCreateNACK(
                0 as *mut streamConsumer,
            );
            (*new_nack).delivery_time = (*nack).delivery_time;
            (*new_nack).delivery_count = (*nack).delivery_count;
            raxInsert(
                (*new_cg).pel,
                ri_cg_pel.key,
                core::mem::size_of::<streamID>() as libc::c_ulong,
                new_nack as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
        }
        raxStop(&mut ri_cg_pel);
        let mut ri_consumers: raxIterator = raxIterator {
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
        raxStart(&mut ri_consumers, (*cg).consumers);
        raxSeek(
            &mut ri_consumers,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        while raxNext(&mut ri_consumers) != 0 {
            let mut consumer: *mut streamConsumer = ri_consumers.data
                as *mut streamConsumer;
            let mut new_consumer: *mut streamConsumer = 0 as *mut streamConsumer;
            new_consumer = zmalloc(
                core::mem::size_of::<streamConsumer>() as libc::c_ulong,
            ) as *mut streamConsumer;
            (*new_consumer).name = sdsdup((*consumer).name);
            (*new_consumer).pel = raxNew();
            raxInsert(
                (*new_cg).consumers,
                (*new_consumer).name as *mut libc::c_uchar,
                sdslen((*new_consumer).name),
                new_consumer as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            (*new_consumer).seen_time = (*consumer).seen_time;
            let mut ri_cpel: raxIterator = raxIterator {
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
            raxStart(&mut ri_cpel, (*consumer).pel);
            raxSeek(
                &mut ri_cpel,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri_cpel) != 0 {
                let mut new_nack_0: *mut streamNACK = raxFind(
                    (*new_cg).pel,
                    ri_cpel.key,
                    core::mem::size_of::<streamID>() as libc::c_ulong,
                ) as *mut streamNACK;
                if new_nack_0 != raxNotFound as *mut streamNACK {} else {
                    _serverAssert(
                        b"new_nack != raxNotFound\0" as *const u8 as *const libc::c_char,
                        b"t_stream.c\0" as *const u8 as *const libc::c_char,
                        246 as libc::c_int,
                    );
                    unreachable!();
                };
                (*new_nack_0).consumer = new_consumer;
                raxInsert(
                    (*new_consumer).pel,
                    ri_cpel.key,
                    core::mem::size_of::<streamID>() as libc::c_ulong,
                    new_nack_0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            raxStop(&mut ri_cpel);
        }
        raxStop(&mut ri_consumers);
    }
    raxStop(&mut ri_cgroups);
    return sobj;
}
#[inline]
unsafe extern "C" fn lpGetIntegerIfValid(
    mut ele: *mut libc::c_uchar,
    mut valid: *mut libc::c_int,
) -> int64_t {
    let mut v: int64_t = 0;
    let mut e: *mut libc::c_uchar = lpGet(ele, &mut v, 0 as *mut libc::c_uchar);
    if e.is_null() {
        if !valid.is_null() {
            *valid = 1 as libc::c_int;
        }
        return v;
    }
    let mut ll: libc::c_longlong = 0;
    let mut ret: libc::c_int = string2ll(e as *mut libc::c_char, v as size_t, &mut ll);
    if !valid.is_null() {
        *valid = ret;
    } else {
        if ret != 0 as libc::c_int {} else {
            _serverAssert(
                b"ret != 0\0" as *const u8 as *const libc::c_char,
                b"t_stream.c\0" as *const u8 as *const libc::c_char,
                281 as libc::c_int,
            );
            unreachable!();
        };
    }
    v = ll as int64_t;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn lpGetEdgeStreamID(
    mut lp: *mut libc::c_uchar,
    mut first: libc::c_int,
    mut master_id: *mut streamID,
    mut edge_id: *mut streamID,
) -> libc::c_int {
    if lp.is_null() {
        return 0 as libc::c_int;
    }
    let mut lp_ele: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if first != 0 {
        lp_ele = lpFirst(lp);
        lp_ele = lpNext(lp, lp_ele);
        lp_ele = lpNext(lp, lp_ele);
        let mut master_fields_count: int64_t = lpGetIntegerIfValid(
            lp_ele,
            0 as *mut libc::c_int,
        );
        lp_ele = lpNext(lp, lp_ele);
        let mut i: int64_t = 0 as libc::c_int as int64_t;
        while i < master_fields_count {
            lp_ele = lpNext(lp, lp_ele);
            i += 1;
        }
        lp_ele = lpNext(lp, lp_ele);
        if lp_ele.is_null() {
            return 0 as libc::c_int;
        }
    } else {
        lp_ele = lpLast(lp);
        let mut lp_count: int64_t = lpGetIntegerIfValid(lp_ele, 0 as *mut libc::c_int);
        if lp_count == 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        loop {
            let fresh0 = lp_count;
            lp_count = lp_count - 1;
            if !(fresh0 != 0) {
                break;
            }
            lp_ele = lpPrev(lp, lp_ele);
        }
    }
    lp_ele = lpNext(lp, lp_ele);
    let mut id: streamID = *master_id;
    id
        .ms = (id.ms as libc::c_ulong)
        .wrapping_add(
            lpGetIntegerIfValid(lp_ele, 0 as *mut libc::c_int) as libc::c_ulong,
        ) as uint64_t as uint64_t;
    lp_ele = lpNext(lp, lp_ele);
    id
        .seq = (id.seq as libc::c_ulong)
        .wrapping_add(
            lpGetIntegerIfValid(lp_ele, 0 as *mut libc::c_int) as libc::c_ulong,
        ) as uint64_t as uint64_t;
    *edge_id = id;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn streamLogListpackContent(mut lp: *mut libc::c_uchar) {
    let mut p: *mut libc::c_uchar = lpFirst(lp);
    while !p.is_null() {
        let mut buf: [libc::c_uchar; 21] = [0; 21];
        let mut v: int64_t = 0;
        let mut ele: *mut libc::c_uchar = lpGet(p, &mut v, buf.as_mut_ptr());
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"- [%d] '%.*s'\0" as *const u8 as *const libc::c_char,
                v as libc::c_int,
                v as libc::c_int,
                ele,
            );
        }
        p = lpNext(lp, p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn streamEncodeID(
    mut buf: *mut libc::c_void,
    mut id: *mut streamID,
) {
    let mut e: [uint64_t; 2] = [0; 2];
    e[0 as libc::c_int as usize] = intrev64((*id).ms);
    e[1 as libc::c_int as usize] = intrev64((*id).seq);
    memcpy(
        buf,
        e.as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn streamDecodeID(
    mut buf: *mut libc::c_void,
    mut id: *mut streamID,
) {
    let mut e: [uint64_t; 2] = [0; 2];
    memcpy(
        e.as_mut_ptr() as *mut libc::c_void,
        buf,
        core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
    );
    (*id).ms = intrev64(e[0 as libc::c_int as usize]);
    (*id).seq = intrev64(e[1 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn streamCompareID(
    mut a: *mut streamID,
    mut b: *mut streamID,
) -> libc::c_int {
    if (*a).ms > (*b).ms {
        return 1 as libc::c_int
    } else {
        if (*a).ms < (*b).ms {
            return -(1 as libc::c_int)
        } else {
            if (*a).seq > (*b).seq {
                return 1 as libc::c_int
            } else {
                if (*a).seq < (*b).seq {
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn streamGetEdgeID(
    mut s: *mut stream,
    mut first: libc::c_int,
    mut skip_tombstones: libc::c_int,
    mut edge_id: *mut streamID,
) {
    let mut si: streamIterator = streamIterator {
        stream: 0 as *mut stream,
        master_id: streamID { ms: 0, seq: 0 },
        master_fields_count: 0,
        master_fields_start: 0 as *mut libc::c_uchar,
        master_fields_ptr: 0 as *mut libc::c_uchar,
        entry_flags: 0,
        rev: 0,
        skip_tombstones: 0,
        start_key: [0; 2],
        end_key: [0; 2],
        ri: raxIterator {
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
        },
        lp: 0 as *mut libc::c_uchar,
        lp_ele: 0 as *mut libc::c_uchar,
        lp_flags: 0 as *mut libc::c_uchar,
        field_buf: [0; 21],
        value_buf: [0; 21],
    };
    let mut numfields: int64_t = 0;
    streamIteratorStart(
        &mut si,
        s,
        0 as *mut streamID,
        0 as *mut streamID,
        (first == 0) as libc::c_int,
    );
    si.skip_tombstones = skip_tombstones;
    let mut found: libc::c_int = streamIteratorGetID(&mut si, edge_id, &mut numfields);
    if found == 0 {
        let mut min_id: streamID = {
            let mut init = streamID {
                ms: 0 as libc::c_int as uint64_t,
                seq: 0 as libc::c_int as uint64_t,
            };
            init
        };
        let mut max_id: streamID = {
            let mut init = streamID {
                ms: 18446744073709551615 as libc::c_ulong,
                seq: 18446744073709551615 as libc::c_ulong,
            };
            init
        };
        *edge_id = if first != 0 { max_id } else { min_id };
    }
    streamIteratorStop(&mut si);
}
#[no_mangle]
pub unsafe extern "C" fn streamAppendItem(
    mut s: *mut stream,
    mut argv: *mut *mut robj,
    mut numfields: int64_t,
    mut added_id: *mut streamID,
    mut use_id: *mut streamID,
    mut seq_given: libc::c_int,
) -> libc::c_int {
    let mut id: streamID = streamID { ms: 0, seq: 0 };
    if !use_id.is_null() {
        if seq_given != 0 {
            id = *use_id;
        } else if (*s).last_id.ms == (*use_id).ms {
            if (*s).last_id.seq == 18446744073709551615 as libc::c_ulong {
                return -(1 as libc::c_int);
            }
            id = (*s).last_id;
            id.seq = (id.seq).wrapping_add(1);
        } else {
            id = *use_id;
        }
    } else {
        streamNextID(&mut (*s).last_id, &mut id);
    }
    if streamCompareID(&mut id, &mut (*s).last_id) <= 0 as libc::c_int {
        *__errno_location() = 33 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut totelelen: size_t = 0 as libc::c_int as size_t;
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < numfields * 2 as libc::c_int as libc::c_long {
        let mut ele: sds = (**argv.offset(i as isize)).ptr as sds;
        totelelen = (totelelen as libc::c_ulong).wrapping_add(sdslen(ele)) as size_t
            as size_t;
        i += 1;
    }
    if totelelen > ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong {
        *__errno_location() = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
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
        b"$\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    let mut lp_bytes: size_t = 0 as libc::c_int as size_t;
    let mut lp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if raxEOF(&mut ri) == 0 {
        lp = ri.data as *mut libc::c_uchar;
        lp_bytes = lpBytes(lp);
    }
    raxStop(&mut ri);
    let mut rax_key: [uint64_t; 2] = [0; 2];
    let mut master_id: streamID = streamID { ms: 0, seq: 0 };
    if !lp.is_null() {
        let mut node_max_bytes: size_t = server.stream_node_max_bytes;
        if node_max_bytes == 0 as libc::c_int as libc::c_ulong
            || node_max_bytes
                > ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong
        {
            node_max_bytes = ((1 as libc::c_int) << 30 as libc::c_int) as size_t;
        }
        if lp_bytes.wrapping_add(totelelen) >= node_max_bytes {
            lp = 0 as *mut libc::c_uchar;
        } else if server.stream_node_max_entries != 0 {
            let mut lp_ele: *mut libc::c_uchar = lpFirst(lp);
            let mut count: int64_t = lpGetIntegerIfValid(lp_ele, 0 as *mut libc::c_int)
                + lpGetIntegerIfValid(lpNext(lp, lp_ele), 0 as *mut libc::c_int);
            if count as libc::c_longlong >= server.stream_node_max_entries {
                lp = lpShrinkToFit(lp);
                if ri.data != lp as *mut libc::c_void {
                    raxInsert(
                        (*s).rax,
                        ri.key,
                        ri.key_len,
                        lp as *mut libc::c_void,
                        0 as *mut *mut libc::c_void,
                    );
                }
                lp = 0 as *mut libc::c_uchar;
            }
        }
    }
    let mut flags: libc::c_int = 0 as libc::c_int;
    if lp.is_null() {
        master_id = id;
        streamEncodeID(rax_key.as_mut_ptr() as *mut libc::c_void, &mut id);
        let mut prealloc: size_t = 4096 as libc::c_int as size_t;
        if server.stream_node_max_bytes > 0 as libc::c_int as libc::c_ulong
            && server.stream_node_max_bytes < prealloc
        {
            prealloc = server.stream_node_max_bytes;
        }
        lp = lpNew(prealloc);
        lp = lpAppendInteger(lp, 1 as libc::c_int as libc::c_longlong);
        lp = lpAppendInteger(lp, 0 as libc::c_int as libc::c_longlong);
        lp = lpAppendInteger(lp, numfields as libc::c_longlong);
        let mut i_0: int64_t = 0 as libc::c_int as int64_t;
        while i_0 < numfields {
            let mut field: sds = (**argv
                .offset((i_0 * 2 as libc::c_int as libc::c_long) as isize))
                .ptr as sds;
            lp = lpAppend(lp, field as *mut libc::c_uchar, sdslen(field) as uint32_t);
            i_0 += 1;
        }
        lp = lpAppendInteger(lp, 0 as libc::c_int as libc::c_longlong);
        raxInsert(
            (*s).rax,
            &mut rax_key as *mut [uint64_t; 2] as *mut libc::c_uchar,
            core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
            lp as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        flags |= (1 as libc::c_int) << 1 as libc::c_int;
    } else {
        if ri.key_len == core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong
        {} else {
            _serverAssert(
                b"ri.key_len == sizeof(rax_key)\0" as *const u8 as *const libc::c_char,
                b"t_stream.c\0" as *const u8 as *const libc::c_char,
                578 as libc::c_int,
            );
            unreachable!();
        };
        memcpy(
            rax_key.as_mut_ptr() as *mut libc::c_void,
            ri.key as *const libc::c_void,
            core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
        );
        streamDecodeID(rax_key.as_mut_ptr() as *mut libc::c_void, &mut master_id);
        let mut lp_ele_0: *mut libc::c_uchar = lpFirst(lp);
        let mut count_0: int64_t = lpGetIntegerIfValid(lp_ele_0, 0 as *mut libc::c_int);
        lp = lpReplaceInteger(
            lp,
            &mut lp_ele_0,
            (count_0 + 1 as libc::c_int as libc::c_long) as libc::c_longlong,
        );
        lp_ele_0 = lpNext(lp, lp_ele_0);
        lp_ele_0 = lpNext(lp, lp_ele_0);
        let mut master_fields_count: int64_t = lpGetIntegerIfValid(
            lp_ele_0,
            0 as *mut libc::c_int,
        );
        lp_ele_0 = lpNext(lp, lp_ele_0);
        if numfields == master_fields_count {
            let mut i_1: int64_t = 0;
            i_1 = 0 as libc::c_int as int64_t;
            while i_1 < master_fields_count {
                let mut field_0: sds = (**argv
                    .offset((i_1 * 2 as libc::c_int as libc::c_long) as isize))
                    .ptr as sds;
                let mut e_len: int64_t = 0;
                let mut buf: [libc::c_uchar; 21] = [0; 21];
                let mut e: *mut libc::c_uchar = lpGet(
                    lp_ele_0,
                    &mut e_len,
                    buf.as_mut_ptr(),
                );
                if sdslen(field_0) != e_len as size_t
                    || memcmp(
                        e as *const libc::c_void,
                        field_0 as *const libc::c_void,
                        e_len as libc::c_ulong,
                    ) != 0 as libc::c_int
                {
                    break;
                }
                lp_ele_0 = lpNext(lp, lp_ele_0);
                i_1 += 1;
            }
            if i_1 == master_fields_count {
                flags |= (1 as libc::c_int) << 1 as libc::c_int;
            }
        }
    }
    lp = lpAppendInteger(lp, flags as libc::c_longlong);
    lp = lpAppendInteger(lp, (id.ms).wrapping_sub(master_id.ms) as libc::c_longlong);
    lp = lpAppendInteger(lp, (id.seq).wrapping_sub(master_id.seq) as libc::c_longlong);
    if flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        lp = lpAppendInteger(lp, numfields as libc::c_longlong);
    }
    let mut i_2: int64_t = 0 as libc::c_int as int64_t;
    while i_2 < numfields {
        let mut field_1: sds = (**argv
            .offset((i_2 * 2 as libc::c_int as libc::c_long) as isize))
            .ptr as sds;
        let mut value: sds = (**argv
            .offset(
                (i_2 * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as isize,
            ))
            .ptr as sds;
        if flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            lp = lpAppend(
                lp,
                field_1 as *mut libc::c_uchar,
                sdslen(field_1) as uint32_t,
            );
        }
        lp = lpAppend(lp, value as *mut libc::c_uchar, sdslen(value) as uint32_t);
        i_2 += 1;
    }
    let mut lp_count: int64_t = numfields;
    lp_count += 3 as libc::c_int as libc::c_long;
    if flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        lp_count += numfields + 1 as libc::c_int as libc::c_long;
    }
    lp = lpAppendInteger(lp, lp_count as libc::c_longlong);
    if ri.data != lp as *mut libc::c_void {
        raxInsert(
            (*s).rax,
            &mut rax_key as *mut [uint64_t; 2] as *mut libc::c_uchar,
            core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
            lp as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    (*s).length = ((*s).length).wrapping_add(1);
    (*s).entries_added = ((*s).entries_added).wrapping_add(1);
    (*s).last_id = id;
    if (*s).length == 1 as libc::c_int as libc::c_ulong {
        (*s).first_id = id;
    }
    if !added_id.is_null() {
        *added_id = id;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn streamTrim(
    mut s: *mut stream,
    mut args: *mut streamAddTrimArgs,
) -> int64_t {
    let mut maxlen: size_t = (*args).maxlen as size_t;
    let mut id: *mut streamID = &mut (*args).minid;
    let mut approx: libc::c_int = (*args).approx_trim;
    let mut limit: int64_t = (*args).limit as int64_t;
    let mut trim_strategy: libc::c_int = (*args).trim_strategy;
    if trim_strategy == 0 as libc::c_int {
        return 0 as libc::c_int as int64_t;
    }
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
    let mut deleted: int64_t = 0 as libc::c_int as int64_t;
    while raxNext(&mut ri) != 0 {
        if trim_strategy == 1 as libc::c_int && (*s).length <= maxlen {
            break;
        }
        let mut lp: *mut libc::c_uchar = ri.data as *mut libc::c_uchar;
        let mut p: *mut libc::c_uchar = lpFirst(lp);
        let mut entries: int64_t = lpGetIntegerIfValid(p, 0 as *mut libc::c_int);
        if limit != 0 && deleted + entries > limit {
            break;
        }
        let mut remove_node: libc::c_int = 0;
        let mut master_id: streamID = {
            let mut init = streamID {
                ms: 0 as libc::c_int as uint64_t,
                seq: 0,
            };
            init
        };
        if trim_strategy == 1 as libc::c_int {
            remove_node = (((*s).length).wrapping_sub(entries as libc::c_ulong)
                >= maxlen) as libc::c_int;
        } else {
            streamDecodeID(ri.key as *mut libc::c_void, &mut master_id);
            let mut last_id: streamID = streamID { ms: 0, seq: 0 };
            lpGetEdgeStreamID(lp, 0 as libc::c_int, &mut master_id, &mut last_id);
            remove_node = (streamCompareID(&mut last_id, id) < 0 as libc::c_int)
                as libc::c_int;
        }
        if remove_node != 0 {
            lpFree(lp);
            raxRemove((*s).rax, ri.key, ri.key_len, 0 as *mut *mut libc::c_void);
            raxSeek(
                &mut ri,
                b">=\0" as *const u8 as *const libc::c_char,
                ri.key,
                ri.key_len,
            );
            (*s)
                .length = ((*s).length as libc::c_ulong)
                .wrapping_sub(entries as libc::c_ulong) as uint64_t as uint64_t;
            deleted += entries;
        } else {
            if approx != 0 {
                break;
            }
            let mut deleted_from_lp: int64_t = 0 as libc::c_int as int64_t;
            p = lpNext(lp, p);
            p = lpNext(lp, p);
            let mut master_fields_count: int64_t = lpGetIntegerIfValid(
                p,
                0 as *mut libc::c_int,
            );
            p = lpNext(lp, p);
            let mut j: int64_t = 0 as libc::c_int as int64_t;
            while j < master_fields_count {
                p = lpNext(lp, p);
                j += 1;
            }
            p = lpNext(lp, p);
            while !p.is_null() {
                let mut pcopy: *mut libc::c_uchar = p;
                let mut flags: int64_t = lpGetIntegerIfValid(p, 0 as *mut libc::c_int);
                p = lpNext(lp, p);
                let mut to_skip: int64_t = 0;
                let mut ms_delta: int64_t = lpGetIntegerIfValid(
                    p,
                    0 as *mut libc::c_int,
                );
                p = lpNext(lp, p);
                let mut seq_delta: int64_t = lpGetIntegerIfValid(
                    p,
                    0 as *mut libc::c_int,
                );
                p = lpNext(lp, p);
                let mut currid: streamID = {
                    let mut init = streamID {
                        ms: 0 as libc::c_int as uint64_t,
                        seq: 0,
                    };
                    init
                };
                if trim_strategy == 2 as libc::c_int {
                    currid.ms = (master_id.ms).wrapping_add(ms_delta as libc::c_ulong);
                    currid
                        .seq = (master_id.seq).wrapping_add(seq_delta as libc::c_ulong);
                }
                let mut stop: libc::c_int = 0;
                if trim_strategy == 1 as libc::c_int {
                    stop = ((*s).length <= maxlen) as libc::c_int;
                } else {
                    stop = (streamCompareID(&mut currid, id) >= 0 as libc::c_int)
                        as libc::c_int;
                }
                if stop != 0 {
                    break;
                }
                if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_long != 0
                {
                    to_skip = master_fields_count;
                } else {
                    to_skip = lpGetIntegerIfValid(p, 0 as *mut libc::c_int);
                    p = lpNext(lp, p);
                    to_skip *= 2 as libc::c_int as libc::c_long;
                }
                loop {
                    let fresh1 = to_skip;
                    to_skip = to_skip - 1;
                    if !(fresh1 != 0) {
                        break;
                    }
                    p = lpNext(lp, p);
                }
                p = lpNext(lp, p);
                if flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_long == 0
                {
                    let mut delta: intptr_t = p.offset_from(lp) as libc::c_long;
                    flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_long;
                    lp = lpReplaceInteger(lp, &mut pcopy, flags as libc::c_longlong);
                    deleted_from_lp += 1;
                    (*s).length = ((*s).length).wrapping_sub(1);
                    p = lp.offset(delta as isize);
                }
            }
            deleted += deleted_from_lp;
            p = lpFirst(lp);
            lp = lpReplaceInteger(
                lp,
                &mut p,
                (entries - deleted_from_lp) as libc::c_longlong,
            );
            p = lpNext(lp, p);
            let mut marked_deleted: int64_t = lpGetIntegerIfValid(
                p,
                0 as *mut libc::c_int,
            );
            lp = lpReplaceInteger(
                lp,
                &mut p,
                (marked_deleted + deleted_from_lp) as libc::c_longlong,
            );
            p = lpNext(lp, p);
            entries -= deleted_from_lp;
            marked_deleted += deleted_from_lp;
            entries + marked_deleted > 10 as libc::c_int as libc::c_long
                && marked_deleted > entries / 2 as libc::c_int as libc::c_long;
            raxInsert(
                (*s).rax,
                ri.key,
                ri.key_len,
                lp as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            break;
        }
    }
    raxStop(&mut ri);
    if (*s).length == 0 as libc::c_int as libc::c_ulong {
        (*s).first_id.ms = 0 as libc::c_int as uint64_t;
        (*s).first_id.seq = 0 as libc::c_int as uint64_t;
    } else if deleted != 0 {
        streamGetEdgeID(s, 1 as libc::c_int, 1 as libc::c_int, &mut (*s).first_id);
    }
    return deleted;
}
#[no_mangle]
pub unsafe extern "C" fn streamTrimByLength(
    mut s: *mut stream,
    mut maxlen: libc::c_longlong,
    mut approx: libc::c_int,
) -> int64_t {
    let mut args: streamAddTrimArgs = {
        let mut init = streamAddTrimArgs {
            id: streamID { ms: 0, seq: 0 },
            id_given: 0,
            seq_given: 0,
            no_mkstream: 0,
            trim_strategy: 1 as libc::c_int,
            trim_strategy_arg_idx: 0,
            approx_trim: approx,
            limit: if approx != 0 {
                100 as libc::c_int as libc::c_longlong * server.stream_node_max_entries
            } else {
                0 as libc::c_int as libc::c_longlong
            },
            maxlen: maxlen,
            minid: streamID { ms: 0, seq: 0 },
        };
        init
    };
    return streamTrim(s, &mut args);
}
#[no_mangle]
pub unsafe extern "C" fn streamTrimByID(
    mut s: *mut stream,
    mut minid: streamID,
    mut approx: libc::c_int,
) -> int64_t {
    let mut args: streamAddTrimArgs = {
        let mut init = streamAddTrimArgs {
            id: streamID { ms: 0, seq: 0 },
            id_given: 0,
            seq_given: 0,
            no_mkstream: 0,
            trim_strategy: 2 as libc::c_int,
            trim_strategy_arg_idx: 0,
            approx_trim: approx,
            limit: if approx != 0 {
                100 as libc::c_int as libc::c_longlong * server.stream_node_max_entries
            } else {
                0 as libc::c_int as libc::c_longlong
            },
            maxlen: 0,
            minid: minid,
        };
        init
    };
    return streamTrim(s, &mut args);
}
unsafe extern "C" fn streamParseAddOrTrimArgsOrReply(
    mut c: *mut client,
    mut args: *mut streamAddTrimArgs,
    mut xadd: libc::c_int,
) -> libc::c_int {
    memset(
        args as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<streamAddTrimArgs>() as libc::c_ulong,
    );
    let mut i: libc::c_int = 2 as libc::c_int;
    let mut limit_given: libc::c_int = 0 as libc::c_int;
    while i < (*c).argc {
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - i;
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(i as isize)).ptr
            as *mut libc::c_char;
        if xadd != 0
            && *opt.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
            && *opt.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            break;
        }
        if strcasecmp(opt, b"maxlen\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            if (*args).trim_strategy != 0 as libc::c_int {
                addReplyError(
                    c,
                    b"syntax error, MAXLEN and MINID options at the same time are not compatible\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            (*args).approx_trim = 0 as libc::c_int;
            let mut next: *mut libc::c_char = (**((*c).argv)
                .offset((i + 1 as libc::c_int) as isize))
                .ptr as *mut libc::c_char;
            if moreargs >= 2 as libc::c_int
                && *next.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
                && *next.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                (*args).approx_trim = 1 as libc::c_int;
                i += 1;
            } else if moreargs >= 2 as libc::c_int
                && *next.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
                && *next.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                i += 1;
            }
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset((i + 1 as libc::c_int) as isize),
                &mut (*args).maxlen,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if (*args).maxlen < 0 as libc::c_int as libc::c_longlong {
                addReplyError(
                    c,
                    b"The MAXLEN argument must be >= 0.\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            i += 1;
            (*args).trim_strategy = 1 as libc::c_int;
            (*args).trim_strategy_arg_idx = i;
        } else if strcasecmp(opt, b"minid\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            if (*args).trim_strategy != 0 as libc::c_int {
                addReplyError(
                    c,
                    b"syntax error, MAXLEN and MINID options at the same time are not compatible\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            (*args).approx_trim = 0 as libc::c_int;
            let mut next_0: *mut libc::c_char = (**((*c).argv)
                .offset((i + 1 as libc::c_int) as isize))
                .ptr as *mut libc::c_char;
            if moreargs >= 2 as libc::c_int
                && *next_0.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
                && *next_0.offset(1 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
            {
                (*args).approx_trim = 1 as libc::c_int;
                i += 1;
            } else if moreargs >= 2 as libc::c_int
                && *next_0.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
                && *next_0.offset(1 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
            {
                i += 1;
            }
            if streamParseStrictIDOrReply(
                c,
                *((*c).argv).offset((i + 1 as libc::c_int) as isize),
                &mut (*args).minid,
                0 as libc::c_int as uint64_t,
                0 as *mut libc::c_int,
            ) != 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            i += 1;
            (*args).trim_strategy = 2 as libc::c_int;
            (*args).trim_strategy_arg_idx = i;
        } else if strcasecmp(opt, b"limit\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset((i + 1 as libc::c_int) as isize),
                &mut (*args).limit,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if (*args).limit < 0 as libc::c_int as libc::c_longlong {
                addReplyError(
                    c,
                    b"The LIMIT argument must be >= 0.\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            limit_given = 1 as libc::c_int;
            i += 1;
        } else if xadd != 0
            && strcasecmp(opt, b"nomkstream\0" as *const u8 as *const libc::c_char) == 0
        {
            (*args).no_mkstream = 1 as libc::c_int;
        } else if xadd != 0 {
            if streamParseStrictIDOrReply(
                c,
                *((*c).argv).offset(i as isize),
                &mut (*args).id,
                0 as libc::c_int as uint64_t,
                &mut (*args).seq_given,
            ) != 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            (*args).id_given = 1 as libc::c_int;
            break;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    if (*args).limit != 0 && (*args).trim_strategy == 0 as libc::c_int {
        addReplyError(
            c,
            b"syntax error, LIMIT cannot be used without specifying a trimming strategy\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if xadd == 0 && (*args).trim_strategy == 0 as libc::c_int {
        addReplyError(
            c,
            b"syntax error, XTRIM must be called with a trimming strategy\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if mustObeyClient(c) != 0 {
        (*args).limit = 0 as libc::c_int as libc::c_longlong;
    } else if limit_given != 0 {
        if (*args).approx_trim == 0 {
            addReplyError(
                c,
                b"syntax error, LIMIT cannot be used without the special ~ option\0"
                    as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if (*args).approx_trim != 0 {
        (*args)
            .limit = 100 as libc::c_int as libc::c_longlong
            * server.stream_node_max_entries;
        if (*args).limit <= 0 as libc::c_int as libc::c_longlong {
            (*args).limit = 10000 as libc::c_int as libc::c_longlong;
        }
        if (*args).limit > 1000000 as libc::c_int as libc::c_longlong {
            (*args).limit = 1000000 as libc::c_int as libc::c_longlong;
        }
    } else {
        (*args).limit = 0 as libc::c_int as libc::c_longlong;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn streamIteratorStart(
    mut si: *mut streamIterator,
    mut s: *mut stream,
    mut start: *mut streamID,
    mut end: *mut streamID,
    mut rev: libc::c_int,
) {
    if !start.is_null() {
        streamEncodeID(((*si).start_key).as_mut_ptr() as *mut libc::c_void, start);
    } else {
        (*si).start_key[0 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
        (*si).start_key[1 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    }
    if !end.is_null() {
        streamEncodeID(((*si).end_key).as_mut_ptr() as *mut libc::c_void, end);
    } else {
        (*si).end_key[0 as libc::c_int as usize] = 18446744073709551615 as libc::c_ulong;
        (*si).end_key[1 as libc::c_int as usize] = 18446744073709551615 as libc::c_ulong;
    }
    raxStart(&mut (*si).ri, (*s).rax);
    if rev == 0 {
        if !start.is_null() && ((*start).ms != 0 || (*start).seq != 0) {
            raxSeek(
                &mut (*si).ri,
                b"<=\0" as *const u8 as *const libc::c_char,
                ((*si).start_key).as_mut_ptr() as *mut libc::c_uchar,
                core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
            );
            if raxEOF(&mut (*si).ri) != 0 {
                raxSeek(
                    &mut (*si).ri,
                    b"^\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_uchar,
                    0 as libc::c_int as size_t,
                );
            }
        } else {
            raxSeek(
                &mut (*si).ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
        }
    } else if !end.is_null() && ((*end).ms != 0 || (*end).seq != 0) {
        raxSeek(
            &mut (*si).ri,
            b"<=\0" as *const u8 as *const libc::c_char,
            ((*si).end_key).as_mut_ptr() as *mut libc::c_uchar,
            core::mem::size_of::<[uint64_t; 2]>() as libc::c_ulong,
        );
        if raxEOF(&mut (*si).ri) != 0 {
            raxSeek(
                &mut (*si).ri,
                b"$\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
        }
    } else {
        raxSeek(
            &mut (*si).ri,
            b"$\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
    }
    (*si).stream = s;
    (*si).lp = 0 as *mut libc::c_uchar;
    (*si).lp_ele = 0 as *mut libc::c_uchar;
    (*si).rev = rev;
    (*si).skip_tombstones = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn streamIteratorGetID(
    mut si: *mut streamIterator,
    mut id: *mut streamID,
    mut numfields: *mut int64_t,
) -> libc::c_int {
    loop {
        if ((*si).lp).is_null() || ((*si).lp_ele).is_null() {
            if (*si).rev == 0 && raxNext(&mut (*si).ri) == 0 {
                return 0 as libc::c_int
            } else {
                if (*si).rev != 0 && raxPrev(&mut (*si).ri) == 0 {
                    return 0 as libc::c_int;
                }
            }
            if (*si).ri.key_len == core::mem::size_of::<streamID>() as libc::c_ulong
            {} else {
                _serverAssert(
                    b"si->ri.key_len == sizeof(streamID)\0" as *const u8
                        as *const libc::c_char,
                    b"t_stream.c\0" as *const u8 as *const libc::c_char,
                    1112 as libc::c_int,
                );
                unreachable!();
            };
            streamDecodeID((*si).ri.key as *mut libc::c_void, &mut (*si).master_id);
            (*si).lp = (*si).ri.data as *mut libc::c_uchar;
            (*si).lp_ele = lpFirst((*si).lp);
            (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
            (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
            (*si)
                .master_fields_count = lpGetIntegerIfValid(
                (*si).lp_ele,
                0 as *mut libc::c_int,
            ) as uint64_t;
            (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
            (*si).master_fields_start = (*si).lp_ele;
            if (*si).rev == 0 {
                let mut i: uint64_t = 0 as libc::c_int as uint64_t;
                while i < (*si).master_fields_count {
                    (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
                    i = i.wrapping_add(1);
                }
            } else {
                (*si).lp_ele = lpLast((*si).lp);
            }
        } else if (*si).rev != 0 {
            let mut lp_count: int64_t = lpGetIntegerIfValid(
                (*si).lp_ele,
                0 as *mut libc::c_int,
            );
            loop {
                let fresh2 = lp_count;
                lp_count = lp_count - 1;
                if !(fresh2 != 0) {
                    break;
                }
                (*si).lp_ele = lpPrev((*si).lp, (*si).lp_ele);
            }
            (*si).lp_ele = lpPrev((*si).lp, (*si).lp_ele);
        }
        loop {
            if (*si).rev == 0 {
                (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
                if ((*si).lp_ele).is_null() {
                    break;
                }
            } else {
                let mut lp_count_0: int64_t = lpGetIntegerIfValid(
                    (*si).lp_ele,
                    0 as *mut libc::c_int,
                );
                if lp_count_0 == 0 as libc::c_int as libc::c_long {
                    (*si).lp = 0 as *mut libc::c_uchar;
                    (*si).lp_ele = 0 as *mut libc::c_uchar;
                    break;
                } else {
                    loop {
                        let fresh3 = lp_count_0;
                        lp_count_0 = lp_count_0 - 1;
                        if !(fresh3 != 0) {
                            break;
                        }
                        (*si).lp_ele = lpPrev((*si).lp, (*si).lp_ele);
                    }
                }
            }
            (*si).lp_flags = (*si).lp_ele;
            let mut flags: int64_t = lpGetIntegerIfValid(
                (*si).lp_ele,
                0 as *mut libc::c_int,
            );
            (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
            *id = (*si).master_id;
            (*id)
                .ms = ((*id).ms as libc::c_ulong)
                .wrapping_add(
                    lpGetIntegerIfValid((*si).lp_ele, 0 as *mut libc::c_int)
                        as libc::c_ulong,
                ) as uint64_t as uint64_t;
            (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
            (*id)
                .seq = ((*id).seq as libc::c_ulong)
                .wrapping_add(
                    lpGetIntegerIfValid((*si).lp_ele, 0 as *mut libc::c_int)
                        as libc::c_ulong,
                ) as uint64_t as uint64_t;
            (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
            let mut buf: [libc::c_uchar; 16] = [0; 16];
            streamEncodeID(buf.as_mut_ptr() as *mut libc::c_void, id);
            if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_long != 0 {
                *numfields = (*si).master_fields_count as int64_t;
            } else {
                *numfields = lpGetIntegerIfValid((*si).lp_ele, 0 as *mut libc::c_int);
                (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
            }
            if *numfields >= 0 as libc::c_int as libc::c_long {} else {
                _serverAssert(
                    b"*numfields>=0\0" as *const u8 as *const libc::c_char,
                    b"t_stream.c\0" as *const u8 as *const libc::c_char,
                    1193 as libc::c_int,
                );
                unreachable!();
            };
            if (*si).rev == 0 {
                if memcmp(
                    buf.as_mut_ptr() as *const libc::c_void,
                    ((*si).start_key).as_mut_ptr() as *const libc::c_void,
                    core::mem::size_of::<streamID>() as libc::c_ulong,
                ) >= 0 as libc::c_int
                    && ((*si).skip_tombstones == 0
                        || flags
                            & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_long
                            == 0)
                {
                    if memcmp(
                        buf.as_mut_ptr() as *const libc::c_void,
                        ((*si).end_key).as_mut_ptr() as *const libc::c_void,
                        core::mem::size_of::<streamID>() as libc::c_ulong,
                    ) > 0 as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*si).entry_flags = flags as libc::c_int;
                    if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_long
                        != 0
                    {
                        (*si).master_fields_ptr = (*si).master_fields_start;
                    }
                    return 1 as libc::c_int;
                }
            } else if memcmp(
                buf.as_mut_ptr() as *const libc::c_void,
                ((*si).end_key).as_mut_ptr() as *const libc::c_void,
                core::mem::size_of::<streamID>() as libc::c_ulong,
            ) <= 0 as libc::c_int
                && ((*si).skip_tombstones == 0
                    || flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_long
                        == 0)
            {
                if memcmp(
                    buf.as_mut_ptr() as *const libc::c_void,
                    ((*si).start_key).as_mut_ptr() as *const libc::c_void,
                    core::mem::size_of::<streamID>() as libc::c_ulong,
                ) < 0 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                (*si).entry_flags = flags as libc::c_int;
                if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_long != 0
                {
                    (*si).master_fields_ptr = (*si).master_fields_start;
                }
                return 1 as libc::c_int;
            }
            if (*si).rev == 0 {
                let mut to_discard: int64_t = if flags
                    & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_long != 0
                {
                    *numfields
                } else {
                    *numfields * 2 as libc::c_int as libc::c_long
                };
                let mut i_0: int64_t = 0 as libc::c_int as int64_t;
                while i_0 < to_discard {
                    (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
                    i_0 += 1;
                }
            } else {
                let mut prev_times: int64_t = 4 as libc::c_int as int64_t;
                if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_long == 0
                {
                    prev_times += 1;
                }
                loop {
                    let fresh4 = prev_times;
                    prev_times = prev_times - 1;
                    if !(fresh4 != 0) {
                        break;
                    }
                    (*si).lp_ele = lpPrev((*si).lp, (*si).lp_ele);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn streamIteratorGetField(
    mut si: *mut streamIterator,
    mut fieldptr: *mut *mut libc::c_uchar,
    mut valueptr: *mut *mut libc::c_uchar,
    mut fieldlen: *mut int64_t,
    mut valuelen: *mut int64_t,
) {
    if (*si).entry_flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        *fieldptr = lpGet(
            (*si).master_fields_ptr,
            fieldlen,
            ((*si).field_buf).as_mut_ptr(),
        );
        (*si).master_fields_ptr = lpNext((*si).lp, (*si).master_fields_ptr);
    } else {
        *fieldptr = lpGet((*si).lp_ele, fieldlen, ((*si).field_buf).as_mut_ptr());
        (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
    }
    *valueptr = lpGet((*si).lp_ele, valuelen, ((*si).value_buf).as_mut_ptr());
    (*si).lp_ele = lpNext((*si).lp, (*si).lp_ele);
}
#[no_mangle]
pub unsafe extern "C" fn streamIteratorRemoveEntry(
    mut si: *mut streamIterator,
    mut current: *mut streamID,
) {
    let mut lp: *mut libc::c_uchar = (*si).lp;
    let mut aux: int64_t = 0;
    let mut flags: int64_t = lpGetIntegerIfValid((*si).lp_flags, 0 as *mut libc::c_int);
    flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_long;
    lp = lpReplaceInteger(lp, &mut (*si).lp_flags, flags as libc::c_longlong);
    let mut p: *mut libc::c_uchar = lpFirst(lp);
    aux = lpGetIntegerIfValid(p, 0 as *mut libc::c_int);
    if aux == 1 as libc::c_int as libc::c_long {
        lpFree(lp);
        raxRemove(
            (*(*si).stream).rax,
            (*si).ri.key,
            (*si).ri.key_len,
            0 as *mut *mut libc::c_void,
        );
    } else {
        lp = lpReplaceInteger(
            lp,
            &mut p,
            (aux - 1 as libc::c_int as libc::c_long) as libc::c_longlong,
        );
        p = lpNext(lp, p);
        aux = lpGetIntegerIfValid(p, 0 as *mut libc::c_int);
        lp = lpReplaceInteger(
            lp,
            &mut p,
            (aux + 1 as libc::c_int as libc::c_long) as libc::c_longlong,
        );
        if (*si).lp != lp {
            raxInsert(
                (*(*si).stream).rax,
                (*si).ri.key,
                (*si).ri.key_len,
                lp as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
        }
    }
    (*(*si).stream).length = ((*(*si).stream).length).wrapping_sub(1);
    let mut start: streamID = streamID { ms: 0, seq: 0 };
    let mut end: streamID = streamID { ms: 0, seq: 0 };
    if (*si).rev != 0 {
        streamDecodeID(((*si).start_key).as_mut_ptr() as *mut libc::c_void, &mut start);
        end = *current;
    } else {
        start = *current;
        streamDecodeID(((*si).end_key).as_mut_ptr() as *mut libc::c_void, &mut end);
    }
    streamIteratorStop(si);
    streamIteratorStart(si, (*si).stream, &mut start, &mut end, (*si).rev);
}
#[no_mangle]
pub unsafe extern "C" fn streamIteratorStop(mut si: *mut streamIterator) {
    raxStop(&mut (*si).ri);
}
#[no_mangle]
pub unsafe extern "C" fn streamEntryExists(
    mut s: *mut stream,
    mut id: *mut streamID,
) -> libc::c_int {
    let mut si: streamIterator = streamIterator {
        stream: 0 as *mut stream,
        master_id: streamID { ms: 0, seq: 0 },
        master_fields_count: 0,
        master_fields_start: 0 as *mut libc::c_uchar,
        master_fields_ptr: 0 as *mut libc::c_uchar,
        entry_flags: 0,
        rev: 0,
        skip_tombstones: 0,
        start_key: [0; 2],
        end_key: [0; 2],
        ri: raxIterator {
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
        },
        lp: 0 as *mut libc::c_uchar,
        lp_ele: 0 as *mut libc::c_uchar,
        lp_flags: 0 as *mut libc::c_uchar,
        field_buf: [0; 21],
        value_buf: [0; 21],
    };
    streamIteratorStart(&mut si, s, id, id, 0 as libc::c_int);
    let mut myid: streamID = streamID { ms: 0, seq: 0 };
    let mut numfields: int64_t = 0;
    let mut found: libc::c_int = streamIteratorGetID(&mut si, &mut myid, &mut numfields);
    streamIteratorStop(&mut si);
    if found == 0 {
        return 0 as libc::c_int;
    }
    if streamCompareID(id, &mut myid) == 0 as libc::c_int {} else {
        _serverAssert(
            b"streamCompareID(id,&myid) == 0\0" as *const u8 as *const libc::c_char,
            b"t_stream.c\0" as *const u8 as *const libc::c_char,
            1342 as libc::c_int,
        );
        unreachable!();
    };
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn streamDeleteItem(
    mut s: *mut stream,
    mut id: *mut streamID,
) -> libc::c_int {
    let mut deleted: libc::c_int = 0 as libc::c_int;
    let mut si: streamIterator = streamIterator {
        stream: 0 as *mut stream,
        master_id: streamID { ms: 0, seq: 0 },
        master_fields_count: 0,
        master_fields_start: 0 as *mut libc::c_uchar,
        master_fields_ptr: 0 as *mut libc::c_uchar,
        entry_flags: 0,
        rev: 0,
        skip_tombstones: 0,
        start_key: [0; 2],
        end_key: [0; 2],
        ri: raxIterator {
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
        },
        lp: 0 as *mut libc::c_uchar,
        lp_ele: 0 as *mut libc::c_uchar,
        lp_flags: 0 as *mut libc::c_uchar,
        field_buf: [0; 21],
        value_buf: [0; 21],
    };
    streamIteratorStart(&mut si, s, id, id, 0 as libc::c_int);
    let mut myid: streamID = streamID { ms: 0, seq: 0 };
    let mut numfields: int64_t = 0;
    if streamIteratorGetID(&mut si, &mut myid, &mut numfields) != 0 {
        streamIteratorRemoveEntry(&mut si, &mut myid);
        deleted = 1 as libc::c_int;
    }
    streamIteratorStop(&mut si);
    return deleted;
}
#[no_mangle]
pub unsafe extern "C" fn streamLastValidID(
    mut s: *mut stream,
    mut maxid: *mut streamID,
) {
    let mut si: streamIterator = streamIterator {
        stream: 0 as *mut stream,
        master_id: streamID { ms: 0, seq: 0 },
        master_fields_count: 0,
        master_fields_start: 0 as *mut libc::c_uchar,
        master_fields_ptr: 0 as *mut libc::c_uchar,
        entry_flags: 0,
        rev: 0,
        skip_tombstones: 0,
        start_key: [0; 2],
        end_key: [0; 2],
        ri: raxIterator {
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
        },
        lp: 0 as *mut libc::c_uchar,
        lp_ele: 0 as *mut libc::c_uchar,
        lp_flags: 0 as *mut libc::c_uchar,
        field_buf: [0; 21],
        value_buf: [0; 21],
    };
    streamIteratorStart(
        &mut si,
        s,
        0 as *mut streamID,
        0 as *mut streamID,
        1 as libc::c_int,
    );
    let mut numfields: int64_t = 0;
    if streamIteratorGetID(&mut si, maxid, &mut numfields) == 0 && (*s).length != 0 {
        _serverPanic(
            b"t_stream.c\0" as *const u8 as *const libc::c_char,
            1369 as libc::c_int,
            b"Corrupt stream, length is %llu, but no max id\0" as *const u8
                as *const libc::c_char,
            (*s).length as libc::c_ulonglong,
        );
        unreachable!();
    }
    streamIteratorStop(&mut si);
}
#[no_mangle]
pub unsafe extern "C" fn createStreamIDString(mut id: *mut streamID) -> sds {
    let mut str: sds = sdsnewlen(
        SDS_NOINIT as *const libc::c_void,
        44 as libc::c_int as size_t,
    );
    sdssetlen(str, 0 as libc::c_int as size_t);
    return sdscatfmt(
        str,
        b"%U-%U\0" as *const u8 as *const libc::c_char,
        (*id).ms,
        (*id).seq,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addReplyStreamID(mut c: *mut client, mut id: *mut streamID) {
    addReplyBulkSds(c, createStreamIDString(id));
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredReplyStreamID(
    mut c: *mut client,
    mut dr: *mut libc::c_void,
    mut id: *mut streamID,
) {
    setDeferredReplyBulkSds(c, dr, createStreamIDString(id));
}
#[no_mangle]
pub unsafe extern "C" fn createObjectFromStreamID(mut id: *mut streamID) -> *mut robj {
    return createObject(0 as libc::c_int, createStreamIDString(id) as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn streamIDEqZero(mut id: *mut streamID) -> libc::c_int {
    return !((*id).ms != 0 || (*id).seq != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn streamRangeHasTombstones(
    mut s: *mut stream,
    mut start: *mut streamID,
    mut end: *mut streamID,
) -> libc::c_int {
    let mut start_id: streamID = streamID { ms: 0, seq: 0 };
    let mut end_id: streamID = streamID { ms: 0, seq: 0 };
    if (*s).length == 0 || streamIDEqZero(&mut (*s).max_deleted_entry_id) != 0 {
        return 0 as libc::c_int;
    }
    if streamCompareID(&mut (*s).first_id, &mut (*s).max_deleted_entry_id)
        > 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !start.is_null() {
        start_id = *start;
    } else {
        start_id.ms = 0 as libc::c_int as uint64_t;
        start_id.seq = 0 as libc::c_int as uint64_t;
    }
    if !end.is_null() {
        end_id = *end;
    } else {
        end_id.ms = 18446744073709551615 as libc::c_ulong;
        end_id.seq = 18446744073709551615 as libc::c_ulong;
    }
    if streamCompareID(&mut start_id, &mut (*s).max_deleted_entry_id) <= 0 as libc::c_int
        && streamCompareID(&mut (*s).max_deleted_entry_id, &mut end_id)
            <= 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn streamReplyWithCGLag(
    mut c: *mut client,
    mut s: *mut stream,
    mut cg: *mut streamCG,
) {
    let mut valid: libc::c_int = 0 as libc::c_int;
    let mut lag: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if (*s).entries_added == 0 {
        lag = 0 as libc::c_int as libc::c_longlong;
        valid = 1 as libc::c_int;
    } else if (*cg).entries_read != -(1 as libc::c_int) as libc::c_longlong
        && streamRangeHasTombstones(s, &mut (*cg).last_id, 0 as *mut streamID) == 0
    {
        lag = (*s).entries_added as libc::c_longlong - (*cg).entries_read;
        valid = 1 as libc::c_int;
    } else {
        let mut entries_read: libc::c_longlong = streamEstimateDistanceFromFirstEverEntry(
            s,
            &mut (*cg).last_id,
        );
        if entries_read != -(1 as libc::c_int) as libc::c_longlong {
            lag = (*s).entries_added as libc::c_longlong - entries_read;
            valid = 1 as libc::c_int;
        }
    }
    if valid != 0 {
        addReplyLongLong(c, lag);
    } else {
        addReplyNull(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn streamEstimateDistanceFromFirstEverEntry(
    mut s: *mut stream,
    mut id: *mut streamID,
) -> libc::c_longlong {
    if (*s).entries_added == 0 {
        return 0 as libc::c_int as libc::c_longlong;
    }
    if (*s).length == 0 && streamCompareID(id, &mut (*s).last_id) < 1 as libc::c_int {
        return (*s).entries_added as libc::c_longlong;
    }
    let mut cmp_last: libc::c_int = streamCompareID(id, &mut (*s).last_id);
    if cmp_last == 0 as libc::c_int {
        return (*s).entries_added as libc::c_longlong
    } else {
        if cmp_last > 0 as libc::c_int {
            return -(1 as libc::c_int) as libc::c_longlong;
        }
    }
    let mut cmp_id_first: libc::c_int = streamCompareID(id, &mut (*s).first_id);
    let mut cmp_xdel_first: libc::c_int = streamCompareID(
        &mut (*s).max_deleted_entry_id,
        &mut (*s).first_id,
    );
    if streamIDEqZero(&mut (*s).max_deleted_entry_id) != 0
        || cmp_xdel_first < 0 as libc::c_int
    {
        if cmp_id_first < 0 as libc::c_int {
            return ((*s).entries_added).wrapping_sub((*s).length) as libc::c_longlong
        } else {
            if cmp_id_first == 0 as libc::c_int {
                return ((*s).entries_added)
                    .wrapping_sub((*s).length)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_longlong;
            }
        }
    }
    return -(1 as libc::c_int) as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn streamPropagateXCLAIM(
    mut c: *mut client,
    mut key: *mut robj,
    mut group: *mut streamCG,
    mut groupname: *mut robj,
    mut id: *mut robj,
    mut nack: *mut streamNACK,
) {
    let mut argv: [*mut robj; 14] = [0 as *mut robj; 14];
    argv[0 as libc::c_int as usize] = shared.xclaim;
    argv[1 as libc::c_int as usize] = key;
    argv[2 as libc::c_int as usize] = groupname;
    argv[3 as libc::c_int
        as usize] = createStringObject(
        (*(*nack).consumer).name as *const libc::c_char,
        sdslen((*(*nack).consumer).name),
    );
    argv[4 as libc::c_int as usize] = shared.integers[0 as libc::c_int as usize];
    argv[5 as libc::c_int as usize] = id;
    argv[6 as libc::c_int as usize] = shared.time;
    argv[7 as libc::c_int
        as usize] = createStringObjectFromLongLong((*nack).delivery_time);
    argv[8 as libc::c_int as usize] = shared.retrycount;
    argv[9 as libc::c_int
        as usize] = createStringObjectFromLongLong(
        (*nack).delivery_count as libc::c_longlong,
    );
    argv[10 as libc::c_int as usize] = shared.force;
    argv[11 as libc::c_int as usize] = shared.justid;
    argv[12 as libc::c_int as usize] = shared.lastid;
    argv[13 as libc::c_int as usize] = createObjectFromStreamID(&mut (*group).last_id);
    alsoPropagate(
        (*(*c).db).id,
        argv.as_mut_ptr(),
        14 as libc::c_int,
        1 as libc::c_int | 2 as libc::c_int,
    );
    decrRefCount(argv[3 as libc::c_int as usize]);
    decrRefCount(argv[7 as libc::c_int as usize]);
    decrRefCount(argv[9 as libc::c_int as usize]);
    decrRefCount(argv[13 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn streamPropagateGroupID(
    mut c: *mut client,
    mut key: *mut robj,
    mut group: *mut streamCG,
    mut groupname: *mut robj,
) {
    let mut argv: [*mut robj; 7] = [0 as *mut robj; 7];
    argv[0 as libc::c_int as usize] = shared.xgroup;
    argv[1 as libc::c_int as usize] = shared.setid;
    argv[2 as libc::c_int as usize] = key;
    argv[3 as libc::c_int as usize] = groupname;
    argv[4 as libc::c_int as usize] = createObjectFromStreamID(&mut (*group).last_id);
    argv[5 as libc::c_int as usize] = shared.entriesread;
    argv[6 as libc::c_int
        as usize] = createStringObjectFromLongLong((*group).entries_read);
    alsoPropagate(
        (*(*c).db).id,
        argv.as_mut_ptr(),
        7 as libc::c_int,
        1 as libc::c_int | 2 as libc::c_int,
    );
    decrRefCount(argv[4 as libc::c_int as usize]);
    decrRefCount(argv[6 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn streamPropagateConsumerCreation(
    mut c: *mut client,
    mut key: *mut robj,
    mut groupname: *mut robj,
    mut consumername: sds,
) {
    let mut argv: [*mut robj; 5] = [0 as *mut robj; 5];
    argv[0 as libc::c_int as usize] = shared.xgroup;
    argv[1 as libc::c_int as usize] = shared.createconsumer;
    argv[2 as libc::c_int as usize] = key;
    argv[3 as libc::c_int as usize] = groupname;
    argv[4 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsdup(consumername) as *mut libc::c_void,
    );
    alsoPropagate(
        (*(*c).db).id,
        argv.as_mut_ptr(),
        5 as libc::c_int,
        1 as libc::c_int | 2 as libc::c_int,
    );
    decrRefCount(argv[4 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn streamReplyWithRange(
    mut c: *mut client,
    mut s: *mut stream,
    mut start: *mut streamID,
    mut end: *mut streamID,
    mut count: size_t,
    mut rev: libc::c_int,
    mut group: *mut streamCG,
    mut consumer: *mut streamConsumer,
    mut flags: libc::c_int,
    mut spi: *mut streamPropInfo,
) -> size_t {
    let mut arraylen_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut arraylen: size_t = 0 as libc::c_int as size_t;
    let mut si: streamIterator = streamIterator {
        stream: 0 as *mut stream,
        master_id: streamID { ms: 0, seq: 0 },
        master_fields_count: 0,
        master_fields_start: 0 as *mut libc::c_uchar,
        master_fields_ptr: 0 as *mut libc::c_uchar,
        entry_flags: 0,
        rev: 0,
        skip_tombstones: 0,
        start_key: [0; 2],
        end_key: [0; 2],
        ri: raxIterator {
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
        },
        lp: 0 as *mut libc::c_uchar,
        lp_ele: 0 as *mut libc::c_uchar,
        lp_flags: 0 as *mut libc::c_uchar,
        field_buf: [0; 21],
        value_buf: [0; 21],
    };
    let mut numfields: int64_t = 0;
    let mut id: streamID = streamID { ms: 0, seq: 0 };
    let mut propagate_last_id: libc::c_int = 0 as libc::c_int;
    let mut noack: libc::c_int = flags & (1 as libc::c_int) << 0 as libc::c_int;
    if !group.is_null() && flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return streamReplyWithRangeFromConsumerPEL(c, s, start, end, count, consumer);
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        arraylen_ptr = addReplyDeferredLen(c);
    }
    streamIteratorStart(&mut si, s, start, end, rev);
    while streamIteratorGetID(&mut si, &mut id, &mut numfields) != 0 {
        if !group.is_null()
            && streamCompareID(&mut id, &mut (*group).last_id) > 0 as libc::c_int
        {
            if (*group).entries_read != -(1 as libc::c_int) as libc::c_longlong
                && streamRangeHasTombstones(s, &mut id, 0 as *mut streamID) == 0
            {
                (*group).entries_read += 1;
            } else if (*s).entries_added != 0 {
                (*group)
                    .entries_read = streamEstimateDistanceFromFirstEverEntry(s, &mut id);
            }
            (*group).last_id = id;
            if noack != 0 {
                propagate_last_id = 1 as libc::c_int;
            }
        }
        addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
        addReplyStreamID(c, &mut id);
        addReplyArrayLen(c, numfields * 2 as libc::c_int as libc::c_long);
        loop {
            let fresh5 = numfields;
            numfields = numfields - 1;
            if !(fresh5 != 0) {
                break;
            }
            let mut key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut value: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut key_len: int64_t = 0;
            let mut value_len: int64_t = 0;
            streamIteratorGetField(
                &mut si,
                &mut key,
                &mut value,
                &mut key_len,
                &mut value_len,
            );
            addReplyBulkCBuffer(c, key as *const libc::c_void, key_len as size_t);
            addReplyBulkCBuffer(c, value as *const libc::c_void, value_len as size_t);
        }
        if !group.is_null() && noack == 0 {
            let mut buf: [libc::c_uchar; 16] = [0; 16];
            streamEncodeID(buf.as_mut_ptr() as *mut libc::c_void, &mut id);
            let mut nack: *mut streamNACK = streamCreateNACK(consumer);
            let mut group_inserted: libc::c_int = raxTryInsert(
                (*group).pel,
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                nack as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            let mut consumer_inserted: libc::c_int = raxTryInsert(
                (*consumer).pel,
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                nack as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            if group_inserted == 0 as libc::c_int {
                streamFreeNACK(nack);
                nack = raxFind(
                    (*group).pel,
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                ) as *mut streamNACK;
                if nack != raxNotFound as *mut streamNACK {} else {
                    _serverAssert(
                        b"nack != raxNotFound\0" as *const u8 as *const libc::c_char,
                        b"t_stream.c\0" as *const u8 as *const libc::c_char,
                        1759 as libc::c_int,
                    );
                    unreachable!();
                };
                raxRemove(
                    (*(*nack).consumer).pel,
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                    0 as *mut *mut libc::c_void,
                );
                (*nack).consumer = consumer;
                (*nack).delivery_time = mstime();
                (*nack).delivery_count = 1 as libc::c_int as uint64_t;
                raxInsert(
                    (*consumer).pel,
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                    nack as *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            } else if group_inserted == 1 as libc::c_int
                && consumer_inserted == 0 as libc::c_int
            {
                _serverPanic(
                    b"t_stream.c\0" as *const u8 as *const libc::c_char,
                    1768 as libc::c_int,
                    b"NACK half-created. Should not be possible.\0" as *const u8
                        as *const libc::c_char,
                );
                unreachable!();
            }
            if !spi.is_null() {
                let mut idarg: *mut robj = createObjectFromStreamID(&mut id);
                streamPropagateXCLAIM(
                    c,
                    (*spi).keyname,
                    group,
                    (*spi).groupname,
                    idarg,
                    nack,
                );
                decrRefCount(idarg);
            }
        }
        arraylen = arraylen.wrapping_add(1);
        if count != 0 && count == arraylen {
            break;
        }
    }
    if !spi.is_null() && propagate_last_id != 0 {
        streamPropagateGroupID(c, (*spi).keyname, group, (*spi).groupname);
    }
    streamIteratorStop(&mut si);
    if !arraylen_ptr.is_null() {
        setDeferredArrayLen(c, arraylen_ptr, arraylen as libc::c_long);
    }
    return arraylen;
}
#[no_mangle]
pub unsafe extern "C" fn streamReplyWithRangeFromConsumerPEL(
    mut c: *mut client,
    mut s: *mut stream,
    mut start: *mut streamID,
    mut end: *mut streamID,
    mut count: size_t,
    mut consumer: *mut streamConsumer,
) -> size_t {
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
    let mut startkey: [libc::c_uchar; 16] = [0; 16];
    let mut endkey: [libc::c_uchar; 16] = [0; 16];
    streamEncodeID(startkey.as_mut_ptr() as *mut libc::c_void, start);
    if !end.is_null() {
        streamEncodeID(endkey.as_mut_ptr() as *mut libc::c_void, end);
    }
    let mut arraylen: size_t = 0 as libc::c_int as size_t;
    let mut arraylen_ptr: *mut libc::c_void = addReplyDeferredLen(c);
    raxStart(&mut ri, (*consumer).pel);
    raxSeek(
        &mut ri,
        b">=\0" as *const u8 as *const libc::c_char,
        startkey.as_mut_ptr(),
        core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
    );
    while raxNext(&mut ri) != 0 && (count == 0 || arraylen < count) {
        if !end.is_null()
            && memcmp(
                ri.key as *const libc::c_void,
                end as *const libc::c_void,
                ri.key_len,
            ) > 0 as libc::c_int
        {
            break;
        }
        let mut thisid: streamID = streamID { ms: 0, seq: 0 };
        streamDecodeID(ri.key as *mut libc::c_void, &mut thisid);
        if streamReplyWithRange(
            c,
            s,
            &mut thisid,
            &mut thisid,
            1 as libc::c_int as size_t,
            0 as libc::c_int,
            0 as *mut streamCG,
            0 as *mut streamConsumer,
            (1 as libc::c_int) << 1 as libc::c_int,
            0 as *mut streamPropInfo,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
            addReplyStreamID(c, &mut thisid);
            addReplyNullArray(c);
        } else {
            let mut nack: *mut streamNACK = ri.data as *mut streamNACK;
            (*nack).delivery_time = mstime();
            (*nack).delivery_count = ((*nack).delivery_count).wrapping_add(1);
        }
        arraylen = arraylen.wrapping_add(1);
    }
    raxStop(&mut ri);
    setDeferredArrayLen(c, arraylen_ptr, arraylen as libc::c_long);
    return arraylen;
}
#[no_mangle]
pub unsafe extern "C" fn streamTypeLookupWriteOrCreate(
    mut c: *mut client,
    mut key: *mut robj,
    mut no_create: libc::c_int,
) -> *mut robj {
    let mut o: *mut robj = lookupKeyWrite((*c).db, key);
    if checkType(c, o, 6 as libc::c_int) != 0 {
        return 0 as *mut robj;
    }
    if o.is_null() {
        if no_create != 0 {
            addReplyNull(c);
            return 0 as *mut robj;
        }
        o = createStreamObject();
        dbAdd((*c).db, key, o);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn streamGenericParseIDOrReply(
    mut c: *mut client,
    mut o: *const robj,
    mut id: *mut streamID,
    mut missing_seq: uint64_t,
    mut strict: libc::c_int,
    mut seq_given: *mut libc::c_int,
) -> libc::c_int {
    let mut ms: libc::c_ulonglong = 0;
    let mut seq: libc::c_ulonglong = 0;
    let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut buf: [libc::c_char; 128] = [0; 128];
    if !(sdslen((*o).ptr as sds)
        > (core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
    {
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            (*o).ptr,
            (sdslen((*o).ptr as sds)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if !(strict != 0
            && (buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32
                || buf[0 as libc::c_int as usize] as libc::c_int == '+' as i32)
            && buf[1 as libc::c_int as usize] as libc::c_int == '\0' as i32)
        {
            if !seq_given.is_null() {
                *seq_given = 1 as libc::c_int;
            }
            if buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32
                && buf[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
            {
                (*id).ms = 0 as libc::c_int as uint64_t;
                (*id).seq = 0 as libc::c_int as uint64_t;
                return 0 as libc::c_int;
            } else {
                if buf[0 as libc::c_int as usize] as libc::c_int == '+' as i32
                    && buf[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
                {
                    (*id).ms = 18446744073709551615 as libc::c_ulong;
                    (*id).seq = 18446744073709551615 as libc::c_ulong;
                    return 0 as libc::c_int;
                }
            }
            ms = 0;
            seq = 0;
            dot = strchr(buf.as_mut_ptr(), '-' as i32);
            if !dot.is_null() {
                *dot = '\0' as i32 as libc::c_char;
            }
            if !(string2ull(buf.as_mut_ptr(), &mut ms) == 0 as libc::c_int) {
                if !dot.is_null() {
                    let mut seqlen: size_t = strlen(
                        dot.offset(1 as libc::c_int as isize),
                    );
                    if !seq_given.is_null()
                        && seqlen == 1 as libc::c_int as libc::c_ulong
                        && *dot.offset(1 as libc::c_int as isize) as libc::c_int
                            == '*' as i32
                    {
                        seq = 0 as libc::c_int as libc::c_ulonglong;
                        *seq_given = 0 as libc::c_int;
                        current_block = 14401909646449704462;
                    } else if string2ull(dot.offset(1 as libc::c_int as isize), &mut seq)
                        == 0 as libc::c_int
                    {
                        current_block = 5263947611583184802;
                    } else {
                        current_block = 14401909646449704462;
                    }
                } else {
                    seq = missing_seq as libc::c_ulonglong;
                    current_block = 14401909646449704462;
                }
                match current_block {
                    5263947611583184802 => {}
                    _ => {
                        (*id).ms = ms as uint64_t;
                        (*id).seq = seq as uint64_t;
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    if !c.is_null() {
        addReplyError(
            c,
            b"Invalid stream ID specified as stream command argument\0" as *const u8
                as *const libc::c_char,
        );
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn streamParseID(
    mut o: *const robj,
    mut id: *mut streamID,
) -> libc::c_int {
    return streamGenericParseIDOrReply(
        0 as *mut client,
        o,
        id,
        0 as libc::c_int as uint64_t,
        0 as libc::c_int,
        0 as *mut libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn streamParseIDOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut id: *mut streamID,
    mut missing_seq: uint64_t,
) -> libc::c_int {
    return streamGenericParseIDOrReply(
        c,
        o,
        id,
        missing_seq,
        0 as libc::c_int,
        0 as *mut libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn streamParseStrictIDOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut id: *mut streamID,
    mut missing_seq: uint64_t,
    mut seq_given: *mut libc::c_int,
) -> libc::c_int {
    return streamGenericParseIDOrReply(
        c,
        o,
        id,
        missing_seq,
        1 as libc::c_int,
        seq_given,
    );
}
#[no_mangle]
pub unsafe extern "C" fn streamParseIntervalIDOrReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut id: *mut streamID,
    mut exclude: *mut libc::c_int,
    mut missing_seq: uint64_t,
) -> libc::c_int {
    let mut p: *mut libc::c_char = (*o).ptr as *mut libc::c_char;
    let mut len: size_t = sdslen(p);
    let mut invalid: libc::c_int = 0 as libc::c_int;
    if !exclude.is_null() {
        *exclude = (len > 1 as libc::c_int as libc::c_ulong
            && *p.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32)
            as libc::c_int;
    }
    if !exclude.is_null() && *exclude != 0 {
        let mut t: *mut robj = createStringObject(
            p.offset(1 as libc::c_int as isize),
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        invalid = (streamParseStrictIDOrReply(
            c,
            t,
            id,
            missing_seq,
            0 as *mut libc::c_int,
        ) == -(1 as libc::c_int)) as libc::c_int;
        decrRefCount(t);
    } else {
        invalid = (streamParseIDOrReply(c, o, id, missing_seq) == -(1 as libc::c_int))
            as libc::c_int;
    }
    if invalid != 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn streamRewriteApproxSpecifier(
    mut c: *mut client,
    mut idx: libc::c_int,
) {
    rewriteClientCommandArgument(c, idx, shared.special_equals);
}
#[no_mangle]
pub unsafe extern "C" fn streamRewriteTrimArgument(
    mut c: *mut client,
    mut s: *mut stream,
    mut trim_strategy: libc::c_int,
    mut idx: libc::c_int,
) {
    let mut arg: *mut robj = 0 as *mut robj;
    if trim_strategy == 1 as libc::c_int {
        arg = createStringObjectFromLongLong((*s).length as libc::c_longlong);
    } else {
        let mut first_id: streamID = streamID { ms: 0, seq: 0 };
        streamGetEdgeID(s, 1 as libc::c_int, 0 as libc::c_int, &mut first_id);
        arg = createObjectFromStreamID(&mut first_id);
    }
    rewriteClientCommandArgument(c, idx, arg);
    decrRefCount(arg);
}
#[no_mangle]
pub unsafe extern "C" fn xaddCommand(mut c: *mut client) {
    let mut parsed_args: streamAddTrimArgs = streamAddTrimArgs {
        id: streamID { ms: 0, seq: 0 },
        id_given: 0,
        seq_given: 0,
        no_mkstream: 0,
        trim_strategy: 0,
        trim_strategy_arg_idx: 0,
        approx_trim: 0,
        limit: 0,
        maxlen: 0,
        minid: streamID { ms: 0, seq: 0 },
    };
    let mut idpos: libc::c_int = streamParseAddOrTrimArgsOrReply(
        c,
        &mut parsed_args,
        1 as libc::c_int,
    );
    if idpos < 0 as libc::c_int {
        return;
    }
    let mut field_pos: libc::c_int = idpos + 1 as libc::c_int;
    if (*c).argc - field_pos < 2 as libc::c_int
        || ((*c).argc - field_pos) % 2 as libc::c_int == 1 as libc::c_int
    {
        addReplyErrorArity(c);
        return;
    }
    if parsed_args.id_given != 0 && parsed_args.seq_given != 0
        && parsed_args.id.ms == 0 as libc::c_int as libc::c_ulong
        && parsed_args.id.seq == 0 as libc::c_int as libc::c_ulong
    {
        addReplyError(
            c,
            b"The ID specified in XADD must be greater than 0-0\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    let mut o: *mut robj = 0 as *mut robj;
    let mut s: *mut stream = 0 as *mut stream;
    o = streamTypeLookupWriteOrCreate(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        parsed_args.no_mkstream,
    );
    if o.is_null() {
        return;
    }
    s = (*o).ptr as *mut stream;
    if (*s).last_id.ms == 18446744073709551615 as libc::c_ulong
        && (*s).last_id.seq == 18446744073709551615 as libc::c_ulong
    {
        addReplyError(
            c,
            b"The stream has exhausted the last possible ID, unable to add more items\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut id: streamID = streamID { ms: 0, seq: 0 };
    if streamAppendItem(
        s,
        ((*c).argv).offset(field_pos as isize),
        (((*c).argc - field_pos) / 2 as libc::c_int) as int64_t,
        &mut id,
        (if parsed_args.id_given != 0 {
            &mut parsed_args.id
        } else {
            0 as *mut streamID
        }),
        parsed_args.seq_given,
    ) == -(1 as libc::c_int)
    {
        if *__errno_location() == 33 as libc::c_int {
            addReplyError(
                c,
                b"The ID specified in XADD is equal or smaller than the target stream top item\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            addReplyError(
                c,
                b"Elements are too large to be stored\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return;
    }
    let mut replyid: sds = createStreamIDString(&mut id);
    addReplyBulkCBuffer(c, replyid as *const libc::c_void, sdslen(replyid));
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"xadd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
    if parsed_args.trim_strategy != 0 as libc::c_int {
        if streamTrim(s, &mut parsed_args) != 0 {
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 10 as libc::c_int,
                b"xtrim\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                *((*c).argv).offset(1 as libc::c_int as isize),
                (*(*c).db).id,
            );
        }
        if parsed_args.approx_trim != 0 {
            streamRewriteApproxSpecifier(
                c,
                parsed_args.trim_strategy_arg_idx - 1 as libc::c_int,
            );
            streamRewriteTrimArgument(
                c,
                s,
                parsed_args.trim_strategy,
                parsed_args.trim_strategy_arg_idx,
            );
        }
    }
    if parsed_args.id_given == 0 || parsed_args.seq_given == 0 {
        let mut idarg: *mut robj = createObject(
            0 as libc::c_int,
            replyid as *mut libc::c_void,
        );
        rewriteClientCommandArgument(c, idpos, idarg);
        decrRefCount(idarg);
    } else {
        sdsfree(replyid);
    }
    signalKeyAsReady(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
        6 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xrangeGenericCommand(mut c: *mut client, mut rev: libc::c_int) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut s: *mut stream = 0 as *mut stream;
    let mut startid: streamID = streamID { ms: 0, seq: 0 };
    let mut endid: streamID = streamID { ms: 0, seq: 0 };
    let mut count: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    let mut startarg: *mut robj = if rev != 0 {
        *((*c).argv).offset(3 as libc::c_int as isize)
    } else {
        *((*c).argv).offset(2 as libc::c_int as isize)
    };
    let mut endarg: *mut robj = if rev != 0 {
        *((*c).argv).offset(2 as libc::c_int as isize)
    } else {
        *((*c).argv).offset(3 as libc::c_int as isize)
    };
    let mut startex: libc::c_int = 0 as libc::c_int;
    let mut endex: libc::c_int = 0 as libc::c_int;
    if streamParseIntervalIDOrReply(
        c,
        startarg,
        &mut startid,
        &mut startex,
        0 as libc::c_int as uint64_t,
    ) != 0 as libc::c_int
    {
        return;
    }
    if startex != 0 && streamIncrID(&mut startid) != 0 as libc::c_int {
        addReplyError(
            c,
            b"invalid start ID for the interval\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if streamParseIntervalIDOrReply(
        c,
        endarg,
        &mut endid,
        &mut endex,
        18446744073709551615 as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return;
    }
    if endex != 0 && streamDecrID(&mut endid) != 0 as libc::c_int {
        addReplyError(
            c,
            b"invalid end ID for the interval\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*c).argc > 4 as libc::c_int {
        let mut j: libc::c_int = 4 as libc::c_int;
        while j < (*c).argc {
            let mut additional: libc::c_int = (*c).argc - j - 1 as libc::c_int;
            if strcasecmp(
                (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                b"COUNT\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int && additional >= 1 as libc::c_int
            {
                if getLongLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                    &mut count,
                    0 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    return;
                }
                if count < 0 as libc::c_int as libc::c_longlong {
                    count = 0 as libc::c_int as libc::c_longlong;
                }
                j += 1;
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
            j += 1;
        }
    }
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.emptyarray,
    );
    if o.is_null() || checkType(c, o, 6 as libc::c_int) != 0 {
        return;
    }
    s = (*o).ptr as *mut stream;
    if count == 0 as libc::c_int as libc::c_longlong {
        addReplyNullArray(c);
    } else {
        if count == -(1 as libc::c_int) as libc::c_longlong {
            count = 0 as libc::c_int as libc::c_longlong;
        }
        streamReplyWithRange(
            c,
            s,
            &mut startid,
            &mut endid,
            count as size_t,
            rev,
            0 as *mut streamCG,
            0 as *mut streamConsumer,
            0 as libc::c_int,
            0 as *mut streamPropInfo,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn xrangeCommand(mut c: *mut client) {
    xrangeGenericCommand(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xrevrangeCommand(mut c: *mut client) {
    xrangeGenericCommand(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xlenCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 6 as libc::c_int) != 0 {
        return;
    }
    let mut s: *mut stream = (*o).ptr as *mut stream;
    addReplyLongLong(c, (*s).length as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn xreadCommand(mut c: *mut client) {
    let mut arraylen: size_t = 0;
    let mut arraylen_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut current_block: u64;
    let mut timeout: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut streams_count: libc::c_int = 0 as libc::c_int;
    let mut streams_arg: libc::c_int = 0 as libc::c_int;
    let mut noack: libc::c_int = 0 as libc::c_int;
    let mut static_ids: [streamID; 8] = [streamID { ms: 0, seq: 0 }; 8];
    let mut ids: *mut streamID = static_ids.as_mut_ptr();
    let mut groups: *mut *mut streamCG = 0 as *mut *mut streamCG;
    let mut xreadgroup: libc::c_int = (sdslen(
        (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as sds,
    ) == 10 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut groupname: *mut robj = 0 as *mut robj;
    let mut consumername: *mut robj = 0 as *mut robj;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < (*c).argc {
        let mut moreargs: libc::c_int = (*c).argc - i - 1 as libc::c_int;
        let mut o: *mut libc::c_char = (**((*c).argv).offset(i as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(o, b"BLOCK\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            if (*c).flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong
                != 0
            {
                addReplyErrorFormat(
                    c,
                    b"%s command is not allowed with BLOCK option from scripts\0"
                        as *const u8 as *const libc::c_char,
                    (**((*c).argv).offset(0 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                );
                return;
            }
            i += 1;
            if getTimeoutFromObjectOrReply(
                c,
                *((*c).argv).offset(i as isize),
                &mut timeout,
                1 as libc::c_int,
            ) != 0 as libc::c_int
            {
                return;
            }
        } else if strcasecmp(o, b"COUNT\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            i += 1;
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset(i as isize),
                &mut count,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            if count < 0 as libc::c_int as libc::c_longlong {
                count = 0 as libc::c_int as libc::c_longlong;
            }
        } else if strcasecmp(o, b"STREAMS\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            streams_arg = i + 1 as libc::c_int;
            streams_count = (*c).argc - streams_arg;
            if streams_count % 2 as libc::c_int != 0 as libc::c_int {
                addReplyError(
                    c,
                    b"Unbalanced XREAD list of streams: for each stream key an ID or '$' must be specified.\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
            streams_count /= 2 as libc::c_int;
            break;
        } else if strcasecmp(o, b"GROUP\0" as *const u8 as *const libc::c_char) == 0
            && moreargs >= 2 as libc::c_int
        {
            if xreadgroup == 0 {
                addReplyError(
                    c,
                    b"The GROUP option is only supported by XREADGROUP. You called XREAD instead.\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
            groupname = *((*c).argv).offset((i + 1 as libc::c_int) as isize);
            consumername = *((*c).argv).offset((i + 2 as libc::c_int) as isize);
            i += 2 as libc::c_int;
        } else if strcasecmp(o, b"NOACK\0" as *const u8 as *const libc::c_char) == 0 {
            if xreadgroup == 0 {
                addReplyError(
                    c,
                    b"The NOACK option is only supported by XREADGROUP. You called XREAD instead.\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
            noack = 1 as libc::c_int;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        i += 1;
    }
    if streams_arg == 0 as libc::c_int {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    if xreadgroup != 0 && groupname.is_null() {
        addReplyError(
            c,
            b"Missing GROUP option for XREADGROUP\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if streams_count > 8 as libc::c_int {
        ids = zmalloc(
            (core::mem::size_of::<streamID>() as libc::c_ulong)
                .wrapping_mul(streams_count as libc::c_ulong),
        ) as *mut streamID;
    }
    if !groupname.is_null() {
        groups = zmalloc(
            (core::mem::size_of::<*mut streamCG>() as libc::c_ulong)
                .wrapping_mul(streams_count as libc::c_ulong),
        ) as *mut *mut streamCG;
    }
    let mut i_0: libc::c_int = streams_arg + streams_count;
    loop {
        if !(i_0 < (*c).argc) {
            current_block = 7330218953828964527;
            break;
        }
        let mut id_idx: libc::c_int = i_0 - streams_arg - streams_count;
        let mut key: *mut robj = *((*c).argv).offset((i_0 - streams_count) as isize);
        let mut o_0: *mut robj = lookupKeyRead((*c).db, key);
        if checkType(c, o_0, 6 as libc::c_int) != 0 {
            current_block = 15441252349031572992;
            break;
        }
        let mut group: *mut streamCG = 0 as *mut streamCG;
        if !groupname.is_null() {
            if o_0.is_null()
                || {
                    group = streamLookupCG(
                        (*o_0).ptr as *mut stream,
                        (*groupname).ptr as sds,
                    );
                    group.is_null()
                }
            {
                addReplyErrorFormat(
                    c,
                    b"-NOGROUP No such key '%s' or consumer group '%s' in XREADGROUP with GROUP option\0"
                        as *const u8 as *const libc::c_char,
                    (*key).ptr as *mut libc::c_char,
                    (*groupname).ptr as *mut libc::c_char,
                );
                current_block = 15441252349031572992;
                break;
            } else {
                let ref mut fresh6 = *groups.offset(id_idx as isize);
                *fresh6 = group;
            }
        }
        if strcmp(
            (**((*c).argv).offset(i_0 as isize)).ptr as *const libc::c_char,
            b"$\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if xreadgroup != 0 {
                addReplyError(
                    c,
                    b"The $ ID is meaningless in the context of XREADGROUP: you want to read the history of this consumer by specifying a proper ID, or use the > ID to get new messages. The $ ID would just return an empty result set.\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block = 15441252349031572992;
                break;
            } else if !o_0.is_null() {
                let mut s: *mut stream = (*o_0).ptr as *mut stream;
                *ids.offset(id_idx as isize) = (*s).last_id;
            } else {
                (*ids.offset(id_idx as isize)).ms = 0 as libc::c_int as uint64_t;
                (*ids.offset(id_idx as isize)).seq = 0 as libc::c_int as uint64_t;
            }
        } else if strcmp(
            (**((*c).argv).offset(i_0 as isize)).ptr as *const libc::c_char,
            b">\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if xreadgroup == 0 {
                addReplyError(
                    c,
                    b"The > ID can be specified only when calling XREADGROUP using the GROUP <group> <consumer> option.\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block = 15441252349031572992;
                break;
            } else {
                (*ids.offset(id_idx as isize))
                    .ms = 18446744073709551615 as libc::c_ulong;
                (*ids.offset(id_idx as isize))
                    .seq = 18446744073709551615 as libc::c_ulong;
            }
        } else if streamParseStrictIDOrReply(
            c,
            *((*c).argv).offset(i_0 as isize),
            ids.offset(id_idx as isize),
            0 as libc::c_int as uint64_t,
            0 as *mut libc::c_int,
        ) != 0 as libc::c_int
        {
            current_block = 15441252349031572992;
            break;
        }
        i_0 += 1;
    }
    match current_block {
        7330218953828964527 => {
            arraylen = 0 as libc::c_int as size_t;
            arraylen_ptr = 0 as *mut libc::c_void;
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < streams_count {
                let mut o_1: *mut robj = lookupKeyRead(
                    (*c).db,
                    *((*c).argv).offset((streams_arg + i_1) as isize),
                );
                if !o_1.is_null() {
                    let mut s_0: *mut stream = (*o_1).ptr as *mut stream;
                    let mut gt: *mut streamID = ids.offset(i_1 as isize);
                    let mut serve_synchronously: libc::c_int = 0 as libc::c_int;
                    let mut serve_history: libc::c_int = 0 as libc::c_int;
                    if !groups.is_null() {
                        if (*gt).ms != 18446744073709551615 as libc::c_ulong
                            || (*gt).seq != 18446744073709551615 as libc::c_ulong
                        {
                            serve_synchronously = 1 as libc::c_int;
                            serve_history = 1 as libc::c_int;
                        } else if (*s_0).length != 0 {
                            let mut maxid: streamID = streamID { ms: 0, seq: 0 };
                            let mut last: *mut streamID = &mut (**groups
                                .offset(i_1 as isize))
                                .last_id;
                            streamLastValidID(s_0, &mut maxid);
                            if streamCompareID(&mut maxid, last) > 0 as libc::c_int {
                                serve_synchronously = 1 as libc::c_int;
                                *gt = *last;
                            }
                        }
                    } else if (*s_0).length != 0 {
                        let mut maxid_0: streamID = streamID { ms: 0, seq: 0 };
                        streamLastValidID(s_0, &mut maxid_0);
                        if streamCompareID(&mut maxid_0, gt) > 0 as libc::c_int {
                            serve_synchronously = 1 as libc::c_int;
                        }
                    }
                    if serve_synchronously != 0 {
                        arraylen = arraylen.wrapping_add(1);
                        if arraylen == 1 as libc::c_int as libc::c_ulong {
                            arraylen_ptr = addReplyDeferredLen(c);
                        }
                        let mut start: streamID = *gt;
                        streamIncrID(&mut start);
                        if (*c).resp == 2 as libc::c_int {
                            addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                        }
                        addReplyBulk(
                            c,
                            *((*c).argv).offset((streams_arg + i_1) as isize),
                        );
                        let mut consumer: *mut streamConsumer = 0 as *mut streamConsumer;
                        let mut spi: streamPropInfo = {
                            let mut init = streamPropInfo {
                                keyname: *((*c).argv).offset((i_1 + streams_arg) as isize),
                                groupname: groupname,
                            };
                            init
                        };
                        if !groups.is_null() {
                            consumer = streamLookupConsumer(
                                *groups.offset(i_1 as isize),
                                (*consumername).ptr as sds,
                                0 as libc::c_int,
                            );
                            if consumer.is_null() {
                                consumer = streamCreateConsumer(
                                    *groups.offset(i_1 as isize),
                                    (*consumername).ptr as sds,
                                    *((*c).argv).offset((streams_arg + i_1) as isize),
                                    (*(*c).db).id,
                                    0 as libc::c_int,
                                );
                                if noack != 0 {
                                    streamPropagateConsumerCreation(
                                        c,
                                        spi.keyname,
                                        spi.groupname,
                                        (*consumer).name,
                                    );
                                }
                            }
                        }
                        let mut flags: libc::c_int = 0 as libc::c_int;
                        if noack != 0 {
                            flags |= (1 as libc::c_int) << 0 as libc::c_int;
                        }
                        if serve_history != 0 {
                            flags |= (1 as libc::c_int) << 2 as libc::c_int;
                        }
                        streamReplyWithRange(
                            c,
                            s_0,
                            &mut start,
                            0 as *mut streamID,
                            count as size_t,
                            0 as libc::c_int,
                            if !groups.is_null() {
                                *groups.offset(i_1 as isize)
                            } else {
                                0 as *mut streamCG
                            },
                            consumer,
                            flags,
                            &mut spi,
                        );
                        if !groups.is_null() {
                            server.dirty += 1;
                        }
                    }
                }
                i_1 += 1;
            }
            if arraylen != 0 {
                if (*c).resp == 2 as libc::c_int {
                    setDeferredArrayLen(c, arraylen_ptr, arraylen as libc::c_long);
                } else {
                    setDeferredMapLen(c, arraylen_ptr, arraylen as libc::c_long);
                }
            } else if timeout != -(1 as libc::c_int) as libc::c_longlong {
                if (*c).flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 41 as libc::c_int != 0
                {
                    addReplyNullArray(c);
                } else {
                    blockForKeys(
                        c,
                        4 as libc::c_int,
                        ((*c).argv).offset(streams_arg as isize),
                        streams_count,
                        -(1 as libc::c_int) as libc::c_long,
                        timeout,
                        0 as *mut robj,
                        0 as *mut blockPos,
                        ids,
                    );
                    (*c)
                        .bpop
                        .xread_count = (if count != 0 {
                        count
                    } else {
                        1000 as libc::c_int as libc::c_longlong
                    }) as size_t;
                    if !groupname.is_null() {
                        incrRefCount(groupname);
                        incrRefCount(consumername);
                        (*c).bpop.xread_group = groupname;
                        (*c).bpop.xread_consumer = consumername;
                        (*c).bpop.xread_group_noack = noack;
                    } else {
                        (*c).bpop.xread_group = 0 as *mut robj;
                        (*c).bpop.xread_consumer = 0 as *mut robj;
                    }
                }
            } else {
                addReplyNullArray(c);
            }
        }
        _ => {}
    }
    preventCommandPropagation(c);
    if ids != static_ids.as_mut_ptr() {
        zfree(ids as *mut libc::c_void);
    }
    zfree(groups as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn streamCreateNACK(
    mut consumer: *mut streamConsumer,
) -> *mut streamNACK {
    let mut nack: *mut streamNACK = zmalloc(
        core::mem::size_of::<streamNACK>() as libc::c_ulong,
    ) as *mut streamNACK;
    (*nack).delivery_time = mstime();
    (*nack).delivery_count = 1 as libc::c_int as uint64_t;
    (*nack).consumer = consumer;
    return nack;
}
#[no_mangle]
pub unsafe extern "C" fn streamFreeNACK(mut na: *mut streamNACK) {
    zfree(na as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn streamFreeConsumer(mut sc: *mut streamConsumer) {
    raxFree((*sc).pel);
    sdsfree((*sc).name);
    zfree(sc as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn streamCreateCG(
    mut s: *mut stream,
    mut name: *mut libc::c_char,
    mut namelen: size_t,
    mut id: *mut streamID,
    mut entries_read: libc::c_longlong,
) -> *mut streamCG {
    if ((*s).cgroups).is_null() {
        (*s).cgroups = raxNew();
    }
    if raxFind((*s).cgroups, name as *mut libc::c_uchar, namelen) != raxNotFound {
        return 0 as *mut streamCG;
    }
    let mut cg: *mut streamCG = zmalloc(
        core::mem::size_of::<streamCG>() as libc::c_ulong,
    ) as *mut streamCG;
    (*cg).pel = raxNew();
    (*cg).consumers = raxNew();
    (*cg).last_id = *id;
    (*cg).entries_read = entries_read;
    raxInsert(
        (*s).cgroups,
        name as *mut libc::c_uchar,
        namelen,
        cg as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return cg;
}
#[no_mangle]
pub unsafe extern "C" fn streamFreeCG(mut cg: *mut streamCG) {
    raxFreeWithCallback(
        (*cg).pel,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut streamNACK) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(streamFreeNACK as unsafe extern "C" fn(*mut streamNACK) -> ())),
    );
    raxFreeWithCallback(
        (*cg).consumers,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut streamConsumer) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(streamFreeConsumer as unsafe extern "C" fn(*mut streamConsumer) -> ())),
    );
    zfree(cg as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn streamLookupCG(
    mut s: *mut stream,
    mut groupname: sds,
) -> *mut streamCG {
    if ((*s).cgroups).is_null() {
        return 0 as *mut streamCG;
    }
    let mut cg: *mut streamCG = raxFind(
        (*s).cgroups,
        groupname as *mut libc::c_uchar,
        sdslen(groupname),
    ) as *mut streamCG;
    return if cg == raxNotFound as *mut streamCG { 0 as *mut streamCG } else { cg };
}
#[no_mangle]
pub unsafe extern "C" fn streamCreateConsumer(
    mut cg: *mut streamCG,
    mut name: sds,
    mut key: *mut robj,
    mut dbid: libc::c_int,
    mut flags: libc::c_int,
) -> *mut streamConsumer {
    if cg.is_null() {
        return 0 as *mut streamConsumer;
    }
    let mut notify: libc::c_int = (flags & (1 as libc::c_int) << 0 as libc::c_int == 0)
        as libc::c_int;
    let mut dirty: libc::c_int = (flags & (1 as libc::c_int) << 1 as libc::c_int == 0)
        as libc::c_int;
    let mut consumer: *mut streamConsumer = zmalloc(
        core::mem::size_of::<streamConsumer>() as libc::c_ulong,
    ) as *mut streamConsumer;
    let mut success: libc::c_int = raxTryInsert(
        (*cg).consumers,
        name as *mut libc::c_uchar,
        sdslen(name),
        consumer as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    if success == 0 {
        zfree(consumer as *mut libc::c_void);
        return 0 as *mut streamConsumer;
    }
    (*consumer).name = sdsdup(name);
    (*consumer).pel = raxNew();
    (*consumer).seen_time = mstime();
    if dirty != 0 {
        server.dirty += 1;
    }
    if notify != 0 {
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 10 as libc::c_int,
            b"xgroup-createconsumer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            key,
            dbid,
        );
    }
    return consumer;
}
#[no_mangle]
pub unsafe extern "C" fn streamLookupConsumer(
    mut cg: *mut streamCG,
    mut name: sds,
    mut flags: libc::c_int,
) -> *mut streamConsumer {
    if cg.is_null() {
        return 0 as *mut streamConsumer;
    }
    let mut refresh: libc::c_int = (flags & (1 as libc::c_int) << 0 as libc::c_int == 0)
        as libc::c_int;
    let mut consumer: *mut streamConsumer = raxFind(
        (*cg).consumers,
        name as *mut libc::c_uchar,
        sdslen(name),
    ) as *mut streamConsumer;
    if consumer == raxNotFound as *mut streamConsumer {
        return 0 as *mut streamConsumer;
    }
    if refresh != 0 {
        (*consumer).seen_time = mstime();
    }
    return consumer;
}
#[no_mangle]
pub unsafe extern "C" fn streamDelConsumer(
    mut cg: *mut streamCG,
    mut consumer: *mut streamConsumer,
) {
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
    raxStart(&mut ri, (*consumer).pel);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        let mut nack: *mut streamNACK = ri.data as *mut streamNACK;
        raxRemove((*cg).pel, ri.key, ri.key_len, 0 as *mut *mut libc::c_void);
        streamFreeNACK(nack);
    }
    raxStop(&mut ri);
    raxRemove(
        (*cg).consumers,
        (*consumer).name as *mut libc::c_uchar,
        sdslen((*consumer).name),
        0 as *mut *mut libc::c_void,
    );
    streamFreeConsumer(consumer);
}
#[no_mangle]
pub unsafe extern "C" fn xgroupCommand(mut c: *mut client) {
    let mut s: *mut stream = 0 as *mut stream;
    let mut grpname: sds = 0 as sds;
    let mut cg: *mut streamCG = 0 as *mut streamCG;
    let mut opt: *mut libc::c_char = (**((*c).argv).offset(1 as libc::c_int as isize))
        .ptr as *mut libc::c_char;
    let mut mkstream: libc::c_int = 0 as libc::c_int;
    let mut entries_read: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    let mut o: *mut robj = 0 as *mut robj;
    if (*c).argc >= 4 as libc::c_int {
        let mut i: libc::c_int = 5 as libc::c_int;
        let mut create_subcmd: libc::c_int = (strcasecmp(
            opt,
            b"CREATE\0" as *const u8 as *const libc::c_char,
        ) == 0) as libc::c_int;
        let mut setid_subcmd: libc::c_int = (strcasecmp(
            opt,
            b"SETID\0" as *const u8 as *const libc::c_char,
        ) == 0) as libc::c_int;
        while i < (*c).argc {
            if create_subcmd != 0
                && strcasecmp(
                    (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
                    b"MKSTREAM\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                mkstream = 1 as libc::c_int;
                i += 1;
            } else if (create_subcmd != 0 || setid_subcmd != 0)
                && strcasecmp(
                    (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
                    b"ENTRIESREAD\0" as *const u8 as *const libc::c_char,
                ) == 0 && (i + 1 as libc::c_int) < (*c).argc
            {
                if getLongLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset((i + 1 as libc::c_int) as isize),
                    &mut entries_read,
                    0 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    return;
                }
                if entries_read < 0 as libc::c_int as libc::c_longlong
                    && entries_read != -(1 as libc::c_int) as libc::c_longlong
                {
                    addReplyError(
                        c,
                        b"value for ENTRIESREAD must be positive or -1\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                i += 2 as libc::c_int;
            } else {
                addReplySubcommandSyntaxError(c);
                return;
            }
        }
        o = lookupKeyWrite((*c).db, *((*c).argv).offset(2 as libc::c_int as isize));
        if !o.is_null() {
            if checkType(c, o, 6 as libc::c_int) != 0 {
                return;
            }
            s = (*o).ptr as *mut stream;
        }
        grpname = (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds;
    }
    if (*c).argc >= 4 as libc::c_int && mkstream == 0 {
        if s.is_null() {
            addReplyError(
                c,
                b"The XGROUP subcommand requires the key to exist. Note that for CREATE you may want to use the MKSTREAM option to create an empty stream automatically.\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        cg = streamLookupCG(s, grpname);
        if cg.is_null()
            && (strcasecmp(opt, b"SETID\0" as *const u8 as *const libc::c_char) == 0
                || strcasecmp(
                    opt,
                    b"CREATECONSUMER\0" as *const u8 as *const libc::c_char,
                ) == 0
                || strcasecmp(opt, b"DELCONSUMER\0" as *const u8 as *const libc::c_char)
                    == 0)
        {
            addReplyErrorFormat(
                c,
                b"-NOGROUP No such consumer group '%s' for key name '%s'\0" as *const u8
                    as *const libc::c_char,
                grpname,
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
            return;
        }
    }
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(opt, b"HELP\0" as *const u8 as *const libc::c_char) == 0
    {
        let mut help: [*const libc::c_char; 15] = [
            b"CREATE <key> <groupname> <id|$> [option]\0" as *const u8
                as *const libc::c_char,
            b"    Create a new consumer group. Options are:\0" as *const u8
                as *const libc::c_char,
            b"    * MKSTREAM\0" as *const u8 as *const libc::c_char,
            b"      Create the empty stream if it does not exist.\0" as *const u8
                as *const libc::c_char,
            b"    * ENTRIESREAD entries_read\0" as *const u8 as *const libc::c_char,
            b"      Set the group's entries_read counter (internal use).\0" as *const u8
                as *const libc::c_char,
            b"CREATECONSUMER <key> <groupname> <consumer>\0" as *const u8
                as *const libc::c_char,
            b"    Create a new consumer in the specified group.\0" as *const u8
                as *const libc::c_char,
            b"DELCONSUMER <key> <groupname> <consumer>\0" as *const u8
                as *const libc::c_char,
            b"    Remove the specified consumer.\0" as *const u8 as *const libc::c_char,
            b"DESTROY <key> <groupname>\0" as *const u8 as *const libc::c_char,
            b"    Remove the specified group.\0" as *const u8 as *const libc::c_char,
            b"SETID <key> <groupname> <id|$> [ENTRIESREAD entries_read]\0" as *const u8
                as *const libc::c_char,
            b"    Set the current group ID and entries_read counter.\0" as *const u8
                as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if strcasecmp(opt, b"CREATE\0" as *const u8 as *const libc::c_char) == 0
        && ((*c).argc >= 5 as libc::c_int && (*c).argc <= 8 as libc::c_int)
    {
        let mut id: streamID = streamID { ms: 0, seq: 0 };
        if strcmp(
            (**((*c).argv).offset(4 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"$\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !s.is_null() {
                id = (*s).last_id;
            } else {
                id.ms = 0 as libc::c_int as uint64_t;
                id.seq = 0 as libc::c_int as uint64_t;
            }
        } else if streamParseStrictIDOrReply(
            c,
            *((*c).argv).offset(4 as libc::c_int as isize),
            &mut id,
            0 as libc::c_int as uint64_t,
            0 as *mut libc::c_int,
        ) != 0 as libc::c_int
        {
            return
        }
        if s.is_null() {
            if mkstream != 0 {} else {
                _serverAssert(
                    b"mkstream\0" as *const u8 as *const libc::c_char,
                    b"t_stream.c\0" as *const u8 as *const libc::c_char,
                    2674 as libc::c_int,
                );
                unreachable!();
            };
            o = createStreamObject();
            dbAdd((*c).db, *((*c).argv).offset(2 as libc::c_int as isize), o);
            s = (*o).ptr as *mut stream;
            signalModifiedKey(
                c,
                (*c).db,
                *((*c).argv).offset(2 as libc::c_int as isize),
            );
        }
        let mut cg_0: *mut streamCG = streamCreateCG(
            s,
            grpname,
            sdslen(grpname),
            &mut id,
            entries_read,
        );
        if !cg_0.is_null() {
            addReply(c, shared.ok);
            server.dirty += 1;
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 10 as libc::c_int,
                b"xgroup-create\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                *((*c).argv).offset(2 as libc::c_int as isize),
                (*(*c).db).id,
            );
        } else {
            addReplyError(
                c,
                b"-BUSYGROUP Consumer Group name already exists\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if strcasecmp(opt, b"SETID\0" as *const u8 as *const libc::c_char) == 0
        && ((*c).argc == 5 as libc::c_int || (*c).argc == 7 as libc::c_int)
    {
        let mut id_0: streamID = streamID { ms: 0, seq: 0 };
        if strcmp(
            (**((*c).argv).offset(4 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"$\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            id_0 = (*s).last_id;
        } else if streamParseIDOrReply(
            c,
            *((*c).argv).offset(4 as libc::c_int as isize),
            &mut id_0,
            0 as libc::c_int as uint64_t,
        ) != 0 as libc::c_int
        {
            return
        }
        (*cg).last_id = id_0;
        (*cg).entries_read = entries_read;
        addReply(c, shared.ok);
        server.dirty += 1;
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 10 as libc::c_int,
            b"xgroup-setid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(2 as libc::c_int as isize),
            (*(*c).db).id,
        );
    } else if strcasecmp(opt, b"DESTROY\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc == 4 as libc::c_int
    {
        if !cg.is_null() {
            raxRemove(
                (*s).cgroups,
                grpname as *mut libc::c_uchar,
                sdslen(grpname),
                0 as *mut *mut libc::c_void,
            );
            streamFreeCG(cg);
            addReply(c, shared.cone);
            server.dirty += 1;
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 10 as libc::c_int,
                b"xgroup-destroy\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                *((*c).argv).offset(2 as libc::c_int as isize),
                (*(*c).db).id,
            );
            signalKeyAsReady(
                (*c).db,
                *((*c).argv).offset(2 as libc::c_int as isize),
                6 as libc::c_int,
            );
        } else {
            addReply(c, shared.czero);
        }
    } else if strcasecmp(opt, b"CREATECONSUMER\0" as *const u8 as *const libc::c_char)
        == 0 && (*c).argc == 5 as libc::c_int
    {
        let mut created: *mut streamConsumer = streamCreateConsumer(
            cg,
            (**((*c).argv).offset(4 as libc::c_int as isize)).ptr as sds,
            *((*c).argv).offset(2 as libc::c_int as isize),
            (*(*c).db).id,
            0 as libc::c_int,
        );
        addReplyLongLong(
            c,
            (if !created.is_null() { 1 as libc::c_int } else { 0 as libc::c_int })
                as libc::c_longlong,
        );
    } else if strcasecmp(opt, b"DELCONSUMER\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc == 5 as libc::c_int
    {
        let mut pending: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        let mut consumer: *mut streamConsumer = streamLookupConsumer(
            cg,
            (**((*c).argv).offset(4 as libc::c_int as isize)).ptr as sds,
            (1 as libc::c_int) << 0 as libc::c_int,
        );
        if !consumer.is_null() {
            pending = raxSize((*consumer).pel) as libc::c_longlong;
            streamDelConsumer(cg, consumer);
            server.dirty += 1;
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 10 as libc::c_int,
                b"xgroup-delconsumer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                *((*c).argv).offset(2 as libc::c_int as isize),
                (*(*c).db).id,
            );
        }
        addReplyLongLong(c, pending);
    } else {
        addReplySubcommandSyntaxError(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xsetidCommand(mut c: *mut client) {
    let mut id: streamID = streamID { ms: 0, seq: 0 };
    let mut max_xdel_id: streamID = {
        let mut init = streamID {
            ms: 0 as libc::c_int as uint64_t,
            seq: 0 as libc::c_int as uint64_t,
        };
        init
    };
    let mut entries_added: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    if streamParseStrictIDOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut id,
        0 as libc::c_int as uint64_t,
        0 as *mut libc::c_int,
    ) != 0 as libc::c_int
    {
        return;
    }
    let mut i: libc::c_int = 3 as libc::c_int;
    while i < (*c).argc {
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - i;
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(i as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(opt, b"ENTRIESADDED\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset((i + 1 as libc::c_int) as isize),
                &mut entries_added,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return
            } else {
                if entries_added < 0 as libc::c_int as libc::c_longlong {
                    addReplyError(
                        c,
                        b"entries_added must be positive\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
            }
            i += 2 as libc::c_int;
        } else if strcasecmp(opt, b"MAXDELETEDID\0" as *const u8 as *const libc::c_char)
            == 0 && moreargs != 0
        {
            if streamParseStrictIDOrReply(
                c,
                *((*c).argv).offset((i + 1 as libc::c_int) as isize),
                &mut max_xdel_id,
                0 as libc::c_int as uint64_t,
                0 as *mut libc::c_int,
            ) != 0 as libc::c_int
            {
                return
            } else {
                if streamCompareID(&mut id, &mut max_xdel_id) < 0 as libc::c_int {
                    addReplyError(
                        c,
                        b"The ID specified in XSETID is smaller than the provided max_deleted_entry_id\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return;
                }
            }
            i += 2 as libc::c_int;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
    }
    let mut o: *mut robj = lookupKeyWriteOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.nokeyerr,
    );
    if o.is_null() || checkType(c, o, 6 as libc::c_int) != 0 {
        return;
    }
    let mut s: *mut stream = (*o).ptr as *mut stream;
    if (*s).length > 0 as libc::c_int as libc::c_ulong {
        let mut maxid: streamID = streamID { ms: 0, seq: 0 };
        streamLastValidID(s, &mut maxid);
        if streamCompareID(&mut id, &mut maxid) < 0 as libc::c_int {
            addReplyError(
                c,
                b"The ID specified in XSETID is smaller than the target stream top item\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        if entries_added != -(1 as libc::c_int) as libc::c_longlong
            && (*s).length > entries_added as uint64_t
        {
            addReplyError(
                c,
                b"The entries_added specified in XSETID is smaller than the target stream length\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    (*s).last_id = id;
    if entries_added != -(1 as libc::c_int) as libc::c_longlong {
        (*s).entries_added = entries_added as uint64_t;
    }
    if streamIDEqZero(&mut max_xdel_id) == 0 {
        (*s).max_deleted_entry_id = max_xdel_id;
    }
    addReply(c, shared.ok);
    server.dirty += 1;
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"xsetid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xackCommand(mut c: *mut client) {
    let mut acknowledged: libc::c_int = 0;
    let mut current_block: u64;
    let mut group: *mut streamCG = 0 as *mut streamCG;
    let mut o: *mut robj = lookupKeyRead(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    if !o.is_null() {
        if checkType(c, o, 6 as libc::c_int) != 0 {
            return;
        }
        group = streamLookupCG(
            (*o).ptr as *mut stream,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        );
    }
    if o.is_null() || group.is_null() {
        addReply(c, shared.czero);
        return;
    }
    let mut static_ids: [streamID; 8] = [streamID { ms: 0, seq: 0 }; 8];
    let mut ids: *mut streamID = static_ids.as_mut_ptr();
    let mut id_count: libc::c_int = (*c).argc - 3 as libc::c_int;
    if id_count > 8 as libc::c_int {
        ids = zmalloc(
            (core::mem::size_of::<streamID>() as libc::c_ulong)
                .wrapping_mul(id_count as libc::c_ulong),
        ) as *mut streamID;
    }
    let mut j: libc::c_int = 3 as libc::c_int;
    loop {
        if !(j < (*c).argc) {
            current_block = 13586036798005543211;
            break;
        }
        if streamParseStrictIDOrReply(
            c,
            *((*c).argv).offset(j as isize),
            &mut *ids.offset((j - 3 as libc::c_int) as isize),
            0 as libc::c_int as uint64_t,
            0 as *mut libc::c_int,
        ) != 0 as libc::c_int
        {
            current_block = 12863602657406568938;
            break;
        }
        j += 1;
    }
    match current_block {
        13586036798005543211 => {
            acknowledged = 0 as libc::c_int;
            let mut j_0: libc::c_int = 3 as libc::c_int;
            while j_0 < (*c).argc {
                let mut buf: [libc::c_uchar; 16] = [0; 16];
                streamEncodeID(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    &mut *ids.offset((j_0 - 3 as libc::c_int) as isize),
                );
                let mut nack: *mut streamNACK = raxFind(
                    (*group).pel,
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                ) as *mut streamNACK;
                if nack != raxNotFound as *mut streamNACK {
                    raxRemove(
                        (*group).pel,
                        buf.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                        0 as *mut *mut libc::c_void,
                    );
                    raxRemove(
                        (*(*nack).consumer).pel,
                        buf.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                        0 as *mut *mut libc::c_void,
                    );
                    streamFreeNACK(nack);
                    acknowledged += 1;
                    server.dirty += 1;
                }
                j_0 += 1;
            }
            addReplyLongLong(c, acknowledged as libc::c_longlong);
        }
        _ => {}
    }
    if ids != static_ids.as_mut_ptr() {
        zfree(ids as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xpendingCommand(mut c: *mut client) {
    let mut justinfo: libc::c_int = ((*c).argc == 3 as libc::c_int) as libc::c_int;
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    let mut groupname: *mut robj = *((*c).argv).offset(2 as libc::c_int as isize);
    let mut consumername: *mut robj = 0 as *mut robj;
    let mut startid: streamID = streamID { ms: 0, seq: 0 };
    let mut endid: streamID = streamID { ms: 0, seq: 0 };
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut minidle: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut startex: libc::c_int = 0 as libc::c_int;
    let mut endex: libc::c_int = 0 as libc::c_int;
    if (*c).argc != 3 as libc::c_int
        && ((*c).argc < 6 as libc::c_int || (*c).argc > 9 as libc::c_int)
    {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    if (*c).argc >= 6 as libc::c_int {
        let mut startidx: libc::c_int = 3 as libc::c_int;
        if strcasecmp(
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"IDLE\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset(4 as libc::c_int as isize),
                &mut minidle,
                0 as *const libc::c_char,
            ) == -(1 as libc::c_int)
            {
                return;
            }
            if (*c).argc < 8 as libc::c_int {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
            startidx += 2 as libc::c_int;
        }
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset((startidx + 2 as libc::c_int) as isize),
            &mut count,
            0 as *const libc::c_char,
        ) == -(1 as libc::c_int)
        {
            return;
        }
        if count < 0 as libc::c_int as libc::c_longlong {
            count = 0 as libc::c_int as libc::c_longlong;
        }
        if streamParseIntervalIDOrReply(
            c,
            *((*c).argv).offset(startidx as isize),
            &mut startid,
            &mut startex,
            0 as libc::c_int as uint64_t,
        ) != 0 as libc::c_int
        {
            return;
        }
        if startex != 0 && streamIncrID(&mut startid) != 0 as libc::c_int {
            addReplyError(
                c,
                b"invalid start ID for the interval\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if streamParseIntervalIDOrReply(
            c,
            *((*c).argv).offset((startidx + 1 as libc::c_int) as isize),
            &mut endid,
            &mut endex,
            18446744073709551615 as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            return;
        }
        if endex != 0 && streamDecrID(&mut endid) != 0 as libc::c_int {
            addReplyError(
                c,
                b"invalid end ID for the interval\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        if (startidx + 3 as libc::c_int) < (*c).argc {
            consumername = *((*c).argv).offset((startidx + 3 as libc::c_int) as isize);
        }
    }
    let mut o: *mut robj = lookupKeyRead(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    let mut group: *mut streamCG = 0 as *mut streamCG;
    if checkType(c, o, 6 as libc::c_int) != 0 {
        return;
    }
    if o.is_null()
        || {
            group = streamLookupCG((*o).ptr as *mut stream, (*groupname).ptr as sds);
            group.is_null()
        }
    {
        addReplyErrorFormat(
            c,
            b"-NOGROUP No such key '%s' or consumer group '%s'\0" as *const u8
                as *const libc::c_char,
            (*key).ptr as *mut libc::c_char,
            (*groupname).ptr as *mut libc::c_char,
        );
        return;
    }
    if justinfo != 0 {
        addReplyArrayLen(c, 4 as libc::c_int as libc::c_long);
        addReplyLongLong(c, raxSize((*group).pel) as libc::c_longlong);
        if raxSize((*group).pel) == 0 as libc::c_int as libc::c_ulong {
            addReplyNull(c);
            addReplyNull(c);
            addReplyNullArray(c);
        } else {
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
            raxStart(&mut ri, (*group).pel);
            raxSeek(
                &mut ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            raxNext(&mut ri);
            streamDecodeID(ri.key as *mut libc::c_void, &mut startid);
            addReplyStreamID(c, &mut startid);
            raxSeek(
                &mut ri,
                b"$\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            raxNext(&mut ri);
            streamDecodeID(ri.key as *mut libc::c_void, &mut endid);
            addReplyStreamID(c, &mut endid);
            raxStop(&mut ri);
            raxStart(&mut ri, (*group).consumers);
            raxSeek(
                &mut ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            let mut arraylen_ptr: *mut libc::c_void = addReplyDeferredLen(c);
            let mut arraylen: size_t = 0 as libc::c_int as size_t;
            while raxNext(&mut ri) != 0 {
                let mut consumer: *mut streamConsumer = ri.data as *mut streamConsumer;
                if raxSize((*consumer).pel) == 0 as libc::c_int as libc::c_ulong {
                    continue;
                }
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCBuffer(c, ri.key as *const libc::c_void, ri.key_len);
                addReplyBulkLongLong(c, raxSize((*consumer).pel) as libc::c_longlong);
                arraylen = arraylen.wrapping_add(1);
            }
            setDeferredArrayLen(c, arraylen_ptr, arraylen as libc::c_long);
            raxStop(&mut ri);
        }
    } else {
        let mut consumer_0: *mut streamConsumer = 0 as *mut streamConsumer;
        if !consumername.is_null() {
            consumer_0 = streamLookupConsumer(
                group,
                (*consumername).ptr as sds,
                (1 as libc::c_int) << 0 as libc::c_int,
            );
            if consumer_0.is_null() {
                addReplyArrayLen(c, 0 as libc::c_int as libc::c_long);
                return;
            }
        }
        let mut pel: *mut rax = if !consumer_0.is_null() {
            (*consumer_0).pel
        } else {
            (*group).pel
        };
        let mut startkey: [libc::c_uchar; 16] = [0; 16];
        let mut endkey: [libc::c_uchar; 16] = [0; 16];
        let mut ri_0: raxIterator = raxIterator {
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
        let mut now: mstime_t = mstime();
        streamEncodeID(startkey.as_mut_ptr() as *mut libc::c_void, &mut startid);
        streamEncodeID(endkey.as_mut_ptr() as *mut libc::c_void, &mut endid);
        raxStart(&mut ri_0, pel);
        raxSeek(
            &mut ri_0,
            b">=\0" as *const u8 as *const libc::c_char,
            startkey.as_mut_ptr(),
            core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
        );
        let mut arraylen_ptr_0: *mut libc::c_void = addReplyDeferredLen(c);
        let mut arraylen_0: size_t = 0 as libc::c_int as size_t;
        while count != 0 && raxNext(&mut ri_0) != 0
            && memcmp(
                ri_0.key as *const libc::c_void,
                endkey.as_mut_ptr() as *const libc::c_void,
                ri_0.key_len,
            ) <= 0 as libc::c_int
        {
            let mut nack: *mut streamNACK = ri_0.data as *mut streamNACK;
            if minidle != 0 {
                let mut this_idle: mstime_t = now - (*nack).delivery_time;
                if this_idle < minidle {
                    continue;
                }
            }
            arraylen_0 = arraylen_0.wrapping_add(1);
            count -= 1;
            addReplyArrayLen(c, 4 as libc::c_int as libc::c_long);
            let mut id: streamID = streamID { ms: 0, seq: 0 };
            streamDecodeID(ri_0.key as *mut libc::c_void, &mut id);
            addReplyStreamID(c, &mut id);
            addReplyBulkCBuffer(
                c,
                (*(*nack).consumer).name as *const libc::c_void,
                sdslen((*(*nack).consumer).name),
            );
            let mut elapsed: mstime_t = now - (*nack).delivery_time;
            if elapsed < 0 as libc::c_int as libc::c_longlong {
                elapsed = 0 as libc::c_int as mstime_t;
            }
            addReplyLongLong(c, elapsed);
            addReplyLongLong(c, (*nack).delivery_count as libc::c_longlong);
        }
        raxStop(&mut ri_0);
        setDeferredArrayLen(c, arraylen_ptr_0, arraylen_0 as libc::c_long);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xclaimCommand(mut c: *mut client) {
    let mut consumer: *mut streamConsumer = 0 as *mut streamConsumer;
    let mut arraylenptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut arraylen: size_t = 0;
    let mut name: sds = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut group: *mut streamCG = 0 as *mut streamCG;
    let mut o: *mut robj = lookupKeyRead(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    let mut minidle: libc::c_longlong = 0;
    let mut retrycount: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    let mut deliverytime: mstime_t = -(1 as libc::c_int) as mstime_t;
    let mut force: libc::c_int = 0 as libc::c_int;
    let mut justid: libc::c_int = 0 as libc::c_int;
    if !o.is_null() {
        if checkType(c, o, 6 as libc::c_int) != 0 {
            return;
        }
        group = streamLookupCG(
            (*o).ptr as *mut stream,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        );
    }
    if o.is_null() || group.is_null() {
        addReplyErrorFormat(
            c,
            b"-NOGROUP No such key '%s' or consumer group '%s'\0" as *const u8
                as *const libc::c_char,
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *mut libc::c_char,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *mut libc::c_char,
        );
        return;
    }
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(4 as libc::c_int as isize),
        &mut minidle,
        b"Invalid min-idle-time argument for XCLAIM\0" as *const u8
            as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if minidle < 0 as libc::c_int as libc::c_longlong {
        minidle = 0 as libc::c_int as libc::c_longlong;
    }
    let mut j: libc::c_int = 0;
    let mut static_ids: [streamID; 8] = [streamID { ms: 0, seq: 0 }; 8];
    let mut ids: *mut streamID = static_ids.as_mut_ptr();
    let mut id_count: libc::c_int = (*c).argc - 5 as libc::c_int;
    if id_count > 8 as libc::c_int {
        ids = zmalloc(
            (core::mem::size_of::<streamID>() as libc::c_ulong)
                .wrapping_mul(id_count as libc::c_ulong),
        ) as *mut streamID;
    }
    j = 5 as libc::c_int;
    while j < (*c).argc {
        if streamParseStrictIDOrReply(
            0 as *mut client,
            *((*c).argv).offset(j as isize),
            &mut *ids.offset((j - 5 as libc::c_int) as isize),
            0 as libc::c_int as uint64_t,
            0 as *mut libc::c_int,
        ) != 0 as libc::c_int
        {
            break;
        }
        j += 1;
    }
    let mut last_id_arg: libc::c_int = j - 1 as libc::c_int;
    let mut now: mstime_t = mstime();
    let mut last_id: streamID = {
        let mut init = streamID {
            ms: 0 as libc::c_int as uint64_t,
            seq: 0 as libc::c_int as uint64_t,
        };
        init
    };
    let mut propagate_last_id: libc::c_int = 0 as libc::c_int;
    loop {
        if !(j < (*c).argc) {
            current_block = 10758786907990354186;
            break;
        }
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - j;
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(opt, b"FORCE\0" as *const u8 as *const libc::c_char) == 0 {
            force = 1 as libc::c_int;
        } else if strcasecmp(opt, b"JUSTID\0" as *const u8 as *const libc::c_char) == 0 {
            justid = 1 as libc::c_int;
        } else if strcasecmp(opt, b"IDLE\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            j += 1;
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset(j as isize),
                &mut deliverytime,
                b"Invalid IDLE option argument for XCLAIM\0" as *const u8
                    as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                current_block = 1213296160947575575;
                break;
            }
            deliverytime = now - deliverytime;
        } else if strcasecmp(opt, b"TIME\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            j += 1;
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset(j as isize),
                &mut deliverytime,
                b"Invalid TIME option argument for XCLAIM\0" as *const u8
                    as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                current_block = 1213296160947575575;
                break;
            }
        } else if strcasecmp(opt, b"RETRYCOUNT\0" as *const u8 as *const libc::c_char)
            == 0 && moreargs != 0
        {
            j += 1;
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset(j as isize),
                &mut retrycount,
                b"Invalid RETRYCOUNT option argument for XCLAIM\0" as *const u8
                    as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                current_block = 1213296160947575575;
                break;
            }
        } else if strcasecmp(opt, b"LASTID\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            j += 1;
            if streamParseStrictIDOrReply(
                c,
                *((*c).argv).offset(j as isize),
                &mut last_id,
                0 as libc::c_int as uint64_t,
                0 as *mut libc::c_int,
            ) != 0 as libc::c_int
            {
                current_block = 1213296160947575575;
                break;
            }
        } else {
            addReplyErrorFormat(
                c,
                b"Unrecognized XCLAIM option '%s'\0" as *const u8 as *const libc::c_char,
                opt,
            );
            current_block = 1213296160947575575;
            break;
        }
        j += 1;
    }
    match current_block {
        10758786907990354186 => {
            if streamCompareID(&mut last_id, &mut (*group).last_id) > 0 as libc::c_int {
                (*group).last_id = last_id;
                propagate_last_id = 1 as libc::c_int;
            }
            if deliverytime != -(1 as libc::c_int) as libc::c_longlong {
                if deliverytime < 0 as libc::c_int as libc::c_longlong
                    || deliverytime > now
                {
                    deliverytime = now;
                }
            } else {
                deliverytime = now;
            }
            consumer = 0 as *mut streamConsumer;
            arraylenptr = addReplyDeferredLen(c);
            arraylen = 0 as libc::c_int as size_t;
            name = (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds;
            let mut current_block_76: u64;
            let mut j_0: libc::c_int = 5 as libc::c_int;
            while j_0 <= last_id_arg {
                let mut id: streamID = *ids.offset((j_0 - 5 as libc::c_int) as isize);
                let mut buf: [libc::c_uchar; 16] = [0; 16];
                streamEncodeID(buf.as_mut_ptr() as *mut libc::c_void, &mut id);
                let mut nack: *mut streamNACK = raxFind(
                    (*group).pel,
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                ) as *mut streamNACK;
                if streamEntryExists((*o).ptr as *mut stream, &mut id) == 0 {
                    if nack != raxNotFound as *mut streamNACK {
                        streamPropagateXCLAIM(
                            c,
                            *((*c).argv).offset(1 as libc::c_int as isize),
                            group,
                            *((*c).argv).offset(2 as libc::c_int as isize),
                            *((*c).argv).offset(j_0 as isize),
                            nack,
                        );
                        propagate_last_id = 0 as libc::c_int;
                        server.dirty += 1;
                        raxRemove(
                            (*group).pel,
                            buf.as_mut_ptr(),
                            core::mem::size_of::<[libc::c_uchar; 16]>()
                                as libc::c_ulong,
                            0 as *mut *mut libc::c_void,
                        );
                        raxRemove(
                            (*(*nack).consumer).pel,
                            buf.as_mut_ptr(),
                            core::mem::size_of::<[libc::c_uchar; 16]>()
                                as libc::c_ulong,
                            0 as *mut *mut libc::c_void,
                        );
                        streamFreeNACK(nack);
                    }
                } else {
                    if force != 0 && nack == raxNotFound as *mut streamNACK {
                        nack = streamCreateNACK(0 as *mut streamConsumer);
                        raxInsert(
                            (*group).pel,
                            buf.as_mut_ptr(),
                            core::mem::size_of::<[libc::c_uchar; 16]>()
                                as libc::c_ulong,
                            nack as *mut libc::c_void,
                            0 as *mut *mut libc::c_void,
                        );
                    }
                    if nack != raxNotFound as *mut streamNACK {
                        if !((*nack).consumer).is_null() && minidle != 0 {
                            let mut this_idle: mstime_t = now - (*nack).delivery_time;
                            if this_idle < minidle {
                                current_block_76 = 17747245473264231573;
                            } else {
                                current_block_76 = 10853015579903106591;
                            }
                        } else {
                            current_block_76 = 10853015579903106591;
                        }
                        match current_block_76 {
                            17747245473264231573 => {}
                            _ => {
                                if consumer.is_null()
                                    && {
                                        consumer = streamLookupConsumer(
                                            group,
                                            name,
                                            0 as libc::c_int,
                                        );
                                        consumer.is_null()
                                    }
                                {
                                    consumer = streamCreateConsumer(
                                        group,
                                        name,
                                        *((*c).argv).offset(1 as libc::c_int as isize),
                                        (*(*c).db).id,
                                        0 as libc::c_int,
                                    );
                                }
                                if (*nack).consumer != consumer {
                                    if !((*nack).consumer).is_null() {
                                        raxRemove(
                                            (*(*nack).consumer).pel,
                                            buf.as_mut_ptr(),
                                            core::mem::size_of::<[libc::c_uchar; 16]>()
                                                as libc::c_ulong,
                                            0 as *mut *mut libc::c_void,
                                        );
                                    }
                                }
                                (*nack).delivery_time = deliverytime;
                                if retrycount >= 0 as libc::c_int as libc::c_longlong {
                                    (*nack).delivery_count = retrycount as uint64_t;
                                } else if justid == 0 {
                                    (*nack)
                                        .delivery_count = ((*nack).delivery_count).wrapping_add(1);
                                }
                                if (*nack).consumer != consumer {
                                    raxInsert(
                                        (*consumer).pel,
                                        buf.as_mut_ptr(),
                                        core::mem::size_of::<[libc::c_uchar; 16]>()
                                            as libc::c_ulong,
                                        nack as *mut libc::c_void,
                                        0 as *mut *mut libc::c_void,
                                    );
                                    (*nack).consumer = consumer;
                                }
                                if justid != 0 {
                                    addReplyStreamID(c, &mut id);
                                } else {
                                    if streamReplyWithRange(
                                        c,
                                        (*o).ptr as *mut stream,
                                        &mut id,
                                        &mut id,
                                        1 as libc::c_int as size_t,
                                        0 as libc::c_int,
                                        0 as *mut streamCG,
                                        0 as *mut streamConsumer,
                                        (1 as libc::c_int) << 1 as libc::c_int,
                                        0 as *mut streamPropInfo,
                                    ) == 1 as libc::c_int as libc::c_ulong
                                    {} else {
                                        _serverAssert(
                                            b"streamReplyWithRange(c,o->ptr,&id,&id,1,0,NULL,NULL,STREAM_RWR_RAWENTRIES,NULL) == 1\0"
                                                as *const u8 as *const libc::c_char,
                                            b"t_stream.c\0" as *const u8 as *const libc::c_char,
                                            3296 as libc::c_int,
                                        );
                                        unreachable!();
                                    };
                                }
                                arraylen = arraylen.wrapping_add(1);
                                streamPropagateXCLAIM(
                                    c,
                                    *((*c).argv).offset(1 as libc::c_int as isize),
                                    group,
                                    *((*c).argv).offset(2 as libc::c_int as isize),
                                    *((*c).argv).offset(j_0 as isize),
                                    nack,
                                );
                                propagate_last_id = 0 as libc::c_int;
                                server.dirty += 1;
                            }
                        }
                    }
                }
                j_0 += 1;
            }
            if propagate_last_id != 0 {
                streamPropagateGroupID(
                    c,
                    *((*c).argv).offset(1 as libc::c_int as isize),
                    group,
                    *((*c).argv).offset(2 as libc::c_int as isize),
                );
                server.dirty += 1;
            }
            setDeferredArrayLen(c, arraylenptr, arraylen as libc::c_long);
            preventCommandPropagation(c);
        }
        _ => {}
    }
    if ids != static_ids.as_mut_ptr() {
        zfree(ids as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xautoclaimCommand(mut c: *mut client) {
    let mut group: *mut streamCG = 0 as *mut streamCG;
    let mut o: *mut robj = lookupKeyRead(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    let mut minidle: libc::c_longlong = 0;
    let mut count: libc::c_long = 100 as libc::c_int as libc::c_long;
    let attempts_factor: libc::c_uint = 10 as libc::c_int as libc::c_uint;
    let mut startid: streamID = streamID { ms: 0, seq: 0 };
    let mut startex: libc::c_int = 0;
    let mut justid: libc::c_int = 0 as libc::c_int;
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(4 as libc::c_int as isize),
        &mut minidle,
        b"Invalid min-idle-time argument for XAUTOCLAIM\0" as *const u8
            as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if minidle < 0 as libc::c_int as libc::c_longlong {
        minidle = 0 as libc::c_int as libc::c_longlong;
    }
    if streamParseIntervalIDOrReply(
        c,
        *((*c).argv).offset(5 as libc::c_int as isize),
        &mut startid,
        &mut startex,
        0 as libc::c_int as uint64_t,
    ) != 0 as libc::c_int
    {
        return;
    }
    if startex != 0 && streamIncrID(&mut startid) != 0 as libc::c_int {
        addReplyError(
            c,
            b"invalid start ID for the interval\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut j: libc::c_int = 6 as libc::c_int;
    while j < (*c).argc {
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - j;
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(opt, b"COUNT\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            let mut max_count: libc::c_long = (9223372036854775807 as libc::c_long
                as libc::c_ulong)
                .wrapping_div(
                    (if core::mem::size_of::<streamID>() as libc::c_ulong
                        > attempts_factor as libc::c_ulong
                    {
                        core::mem::size_of::<streamID>() as libc::c_ulong
                    } else {
                        attempts_factor as libc::c_ulong
                    }),
                ) as libc::c_long;
            if getRangeLongFromObjectOrReply(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                1 as libc::c_int as libc::c_long,
                max_count,
                &mut count,
                b"COUNT must be > 0\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            j += 1;
        } else if strcasecmp(opt, b"JUSTID\0" as *const u8 as *const libc::c_char) == 0 {
            justid = 1 as libc::c_int;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        j += 1;
    }
    if !o.is_null() {
        if checkType(c, o, 6 as libc::c_int) != 0 {
            return;
        }
        group = streamLookupCG(
            (*o).ptr as *mut stream,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        );
    }
    if o.is_null() || group.is_null() {
        addReplyErrorFormat(
            c,
            b"-NOGROUP No such key '%s' or consumer group '%s'\0" as *const u8
                as *const libc::c_char,
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *mut libc::c_char,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *mut libc::c_char,
        );
        return;
    }
    let mut deleted_ids: *mut streamID = ztrymalloc(
        (count as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<streamID>() as libc::c_ulong),
    ) as *mut streamID;
    if deleted_ids.is_null() {
        addReplyError(
            c,
            b"Insufficient memory, failed allocating transient memory, COUNT too high.\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut consumer: *mut streamConsumer = 0 as *mut streamConsumer;
    let mut attempts: libc::c_longlong = (count * attempts_factor as libc::c_long)
        as libc::c_longlong;
    addReplyArrayLen(c, 3 as libc::c_int as libc::c_long);
    let mut endidptr: *mut libc::c_void = addReplyDeferredLen(c);
    let mut arraylenptr: *mut libc::c_void = addReplyDeferredLen(c);
    let mut startkey: [libc::c_uchar; 16] = [0; 16];
    streamEncodeID(startkey.as_mut_ptr() as *mut libc::c_void, &mut startid);
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
    raxStart(&mut ri, (*group).pel);
    raxSeek(
        &mut ri,
        b">=\0" as *const u8 as *const libc::c_char,
        startkey.as_mut_ptr(),
        core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
    );
    let mut arraylen: size_t = 0 as libc::c_int as size_t;
    let mut now: mstime_t = mstime();
    let mut name: sds = (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds;
    let mut deleted_id_num: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh7 = attempts;
        attempts = attempts - 1;
        if !(fresh7 != 0 && count != 0 && raxNext(&mut ri) != 0) {
            break;
        }
        let mut nack: *mut streamNACK = ri.data as *mut streamNACK;
        let mut id: streamID = streamID { ms: 0, seq: 0 };
        streamDecodeID(ri.key as *mut libc::c_void, &mut id);
        if streamEntryExists((*o).ptr as *mut stream, &mut id) == 0 {
            let mut idstr: *mut robj = createObjectFromStreamID(&mut id);
            streamPropagateXCLAIM(
                c,
                *((*c).argv).offset(1 as libc::c_int as isize),
                group,
                *((*c).argv).offset(2 as libc::c_int as isize),
                idstr,
                nack,
            );
            decrRefCount(idstr);
            server.dirty += 1;
            raxRemove((*group).pel, ri.key, ri.key_len, 0 as *mut *mut libc::c_void);
            raxRemove(
                (*(*nack).consumer).pel,
                ri.key,
                ri.key_len,
                0 as *mut *mut libc::c_void,
            );
            streamFreeNACK(nack);
            let fresh8 = deleted_id_num;
            deleted_id_num = deleted_id_num + 1;
            *deleted_ids.offset(fresh8 as isize) = id;
            raxSeek(
                &mut ri,
                b">=\0" as *const u8 as *const libc::c_char,
                ri.key,
                ri.key_len,
            );
            count -= 1;
        } else {
            if minidle != 0 {
                let mut this_idle: mstime_t = now - (*nack).delivery_time;
                if this_idle < minidle {
                    continue;
                }
            }
            if consumer.is_null()
                && {
                    consumer = streamLookupConsumer(group, name, 0 as libc::c_int);
                    consumer.is_null()
                }
            {
                consumer = streamCreateConsumer(
                    group,
                    name,
                    *((*c).argv).offset(1 as libc::c_int as isize),
                    (*(*c).db).id,
                    0 as libc::c_int,
                );
            }
            if (*nack).consumer != consumer {
                if !((*nack).consumer).is_null() {
                    raxRemove(
                        (*(*nack).consumer).pel,
                        ri.key,
                        ri.key_len,
                        0 as *mut *mut libc::c_void,
                    );
                }
            }
            (*nack).delivery_time = now;
            if justid == 0 {
                (*nack).delivery_count = ((*nack).delivery_count).wrapping_add(1);
            }
            if (*nack).consumer != consumer {
                raxInsert(
                    (*consumer).pel,
                    ri.key,
                    ri.key_len,
                    nack as *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                (*nack).consumer = consumer;
            }
            if justid != 0 {
                addReplyStreamID(c, &mut id);
            } else {
                if streamReplyWithRange(
                    c,
                    (*o).ptr as *mut stream,
                    &mut id,
                    &mut id,
                    1 as libc::c_int as size_t,
                    0 as libc::c_int,
                    0 as *mut streamCG,
                    0 as *mut streamConsumer,
                    (1 as libc::c_int) << 1 as libc::c_int,
                    0 as *mut streamPropInfo,
                ) == 1 as libc::c_int as libc::c_ulong
                {} else {
                    _serverAssert(
                        b"streamReplyWithRange(c,o->ptr,&id,&id,1,0,NULL,NULL,STREAM_RWR_RAWENTRIES,NULL) == 1\0"
                            as *const u8 as *const libc::c_char,
                        b"t_stream.c\0" as *const u8 as *const libc::c_char,
                        3470 as libc::c_int,
                    );
                    unreachable!();
                };
            }
            arraylen = arraylen.wrapping_add(1);
            count -= 1;
            let mut idstr_0: *mut robj = createObjectFromStreamID(&mut id);
            streamPropagateXCLAIM(
                c,
                *((*c).argv).offset(1 as libc::c_int as isize),
                group,
                *((*c).argv).offset(2 as libc::c_int as isize),
                idstr_0,
                nack,
            );
            decrRefCount(idstr_0);
            server.dirty += 1;
        }
    }
    raxNext(&mut ri);
    let mut endid: streamID = streamID { ms: 0, seq: 0 };
    if raxEOF(&mut ri) != 0 {
        endid.seq = 0 as libc::c_int as uint64_t;
        endid.ms = endid.seq;
    } else {
        streamDecodeID(ri.key as *mut libc::c_void, &mut endid);
    }
    raxStop(&mut ri);
    setDeferredArrayLen(c, arraylenptr, arraylen as libc::c_long);
    setDeferredReplyStreamID(c, endidptr, &mut endid);
    addReplyArrayLen(c, deleted_id_num as libc::c_long);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < deleted_id_num {
        addReplyStreamID(c, &mut *deleted_ids.offset(i as isize));
        i += 1;
    }
    zfree(deleted_ids as *mut libc::c_void);
    preventCommandPropagation(c);
}
#[no_mangle]
pub unsafe extern "C" fn xdelCommand(mut c: *mut client) {
    let mut deleted: libc::c_int = 0;
    let mut first_entry: libc::c_int = 0;
    let mut current_block: u64;
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyWriteOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 6 as libc::c_int) != 0 {
        return;
    }
    let mut s: *mut stream = (*o).ptr as *mut stream;
    let mut static_ids: [streamID; 8] = [streamID { ms: 0, seq: 0 }; 8];
    let mut ids: *mut streamID = static_ids.as_mut_ptr();
    let mut id_count: libc::c_int = (*c).argc - 2 as libc::c_int;
    if id_count > 8 as libc::c_int {
        ids = zmalloc(
            (core::mem::size_of::<streamID>() as libc::c_ulong)
                .wrapping_mul(id_count as libc::c_ulong),
        ) as *mut streamID;
    }
    let mut j: libc::c_int = 2 as libc::c_int;
    loop {
        if !(j < (*c).argc) {
            current_block = 14523784380283086299;
            break;
        }
        if streamParseStrictIDOrReply(
            c,
            *((*c).argv).offset(j as isize),
            &mut *ids.offset((j - 2 as libc::c_int) as isize),
            0 as libc::c_int as uint64_t,
            0 as *mut libc::c_int,
        ) != 0 as libc::c_int
        {
            current_block = 184023078884431501;
            break;
        }
        j += 1;
    }
    match current_block {
        14523784380283086299 => {
            deleted = 0 as libc::c_int;
            first_entry = 0 as libc::c_int;
            let mut j_0: libc::c_int = 2 as libc::c_int;
            while j_0 < (*c).argc {
                let mut id: *mut streamID = &mut *ids
                    .offset((j_0 - 2 as libc::c_int) as isize) as *mut streamID;
                if streamDeleteItem(s, id) != 0 {
                    if streamCompareID(id, &mut (*s).first_id) == 0 as libc::c_int {
                        first_entry = 1 as libc::c_int;
                    }
                    if streamCompareID(id, &mut (*s).max_deleted_entry_id)
                        > 0 as libc::c_int
                    {
                        (*s).max_deleted_entry_id = *id;
                    }
                    deleted += 1;
                }
                j_0 += 1;
            }
            if deleted != 0 {
                if (*s).length == 0 as libc::c_int as libc::c_ulong {
                    (*s).first_id.ms = 0 as libc::c_int as uint64_t;
                    (*s).first_id.seq = 0 as libc::c_int as uint64_t;
                } else if first_entry != 0 {
                    streamGetEdgeID(
                        s,
                        1 as libc::c_int,
                        1 as libc::c_int,
                        &mut (*s).first_id,
                    );
                }
            }
            if deleted != 0 {
                signalModifiedKey(
                    c,
                    (*c).db,
                    *((*c).argv).offset(1 as libc::c_int as isize),
                );
                notifyKeyspaceEvent(
                    (1 as libc::c_int) << 10 as libc::c_int,
                    b"xdel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *((*c).argv).offset(1 as libc::c_int as isize),
                    (*(*c).db).id,
                );
                server.dirty += deleted as libc::c_longlong;
            }
            addReplyLongLong(c, deleted as libc::c_longlong);
        }
        _ => {}
    }
    if ids != static_ids.as_mut_ptr() {
        zfree(ids as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xtrimCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut parsed_args: streamAddTrimArgs = streamAddTrimArgs {
        id: streamID { ms: 0, seq: 0 },
        id_given: 0,
        seq_given: 0,
        no_mkstream: 0,
        trim_strategy: 0,
        trim_strategy_arg_idx: 0,
        approx_trim: 0,
        limit: 0,
        maxlen: 0,
        minid: streamID { ms: 0, seq: 0 },
    };
    if streamParseAddOrTrimArgsOrReply(c, &mut parsed_args, 0 as libc::c_int)
        < 0 as libc::c_int
    {
        return;
    }
    o = lookupKeyWriteOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 6 as libc::c_int) != 0 {
        return;
    }
    let mut s: *mut stream = (*o).ptr as *mut stream;
    let mut deleted: int64_t = streamTrim(s, &mut parsed_args);
    if deleted != 0 {
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 10 as libc::c_int,
            b"xtrim\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        if parsed_args.approx_trim != 0 {
            streamRewriteApproxSpecifier(
                c,
                parsed_args.trim_strategy_arg_idx - 1 as libc::c_int,
            );
            streamRewriteTrimArgument(
                c,
                s,
                parsed_args.trim_strategy,
                parsed_args.trim_strategy_arg_idx,
            );
        }
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        server.dirty += deleted as libc::c_longlong;
    }
    addReplyLongLong(c, deleted as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn xinfoReplyWithStreamInfo(
    mut c: *mut client,
    mut s: *mut stream,
) {
    let mut full: libc::c_int = 1 as libc::c_int;
    let mut count: libc::c_longlong = 10 as libc::c_int as libc::c_longlong;
    let mut optv: *mut *mut robj = ((*c).argv).offset(3 as libc::c_int as isize);
    let mut optc: libc::c_int = (*c).argc - 3 as libc::c_int;
    if optc == 0 as libc::c_int {
        full = 0 as libc::c_int;
    } else {
        if optc != 1 as libc::c_int && optc != 3 as libc::c_int {
            addReplySubcommandSyntaxError(c);
            return;
        }
        if strcasecmp(
            (**optv.offset(0 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"full\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            addReplySubcommandSyntaxError(c);
            return;
        }
        if optc == 3 as libc::c_int {
            if strcasecmp(
                (**optv.offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
                b"count\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                addReplySubcommandSyntaxError(c);
                return;
            }
            if getLongLongFromObjectOrReply(
                c,
                *optv.offset(2 as libc::c_int as isize),
                &mut count,
                0 as *const libc::c_char,
            ) == -(1 as libc::c_int)
            {
                return;
            }
            if count < 0 as libc::c_int as libc::c_longlong {
                count = 10 as libc::c_int as libc::c_longlong;
            }
        }
    }
    addReplyMapLen(
        c,
        (if full != 0 { 9 as libc::c_int } else { 10 as libc::c_int }) as libc::c_long,
    );
    addReplyBulkCString(c, b"length\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, (*s).length as libc::c_longlong);
    addReplyBulkCString(c, b"radix-tree-keys\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, raxSize((*s).rax) as libc::c_longlong);
    addReplyBulkCString(c, b"radix-tree-nodes\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, (*(*s).rax).numnodes as libc::c_longlong);
    addReplyBulkCString(c, b"last-generated-id\0" as *const u8 as *const libc::c_char);
    addReplyStreamID(c, &mut (*s).last_id);
    addReplyBulkCString(
        c,
        b"max-deleted-entry-id\0" as *const u8 as *const libc::c_char,
    );
    addReplyStreamID(c, &mut (*s).max_deleted_entry_id);
    addReplyBulkCString(c, b"entries-added\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, (*s).entries_added as libc::c_longlong);
    addReplyBulkCString(
        c,
        b"recorded-first-entry-id\0" as *const u8 as *const libc::c_char,
    );
    addReplyStreamID(c, &mut (*s).first_id);
    if full == 0 {
        addReplyBulkCString(c, b"groups\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(
            c,
            (if !((*s).cgroups).is_null() {
                raxSize((*s).cgroups)
            } else {
                0 as libc::c_int as libc::c_ulong
            }) as libc::c_longlong,
        );
        let mut emitted: libc::c_int = 0;
        let mut start: streamID = streamID { ms: 0, seq: 0 };
        let mut end: streamID = streamID { ms: 0, seq: 0 };
        start.seq = 0 as libc::c_int as uint64_t;
        start.ms = start.seq;
        end.seq = 18446744073709551615 as libc::c_ulong;
        end.ms = end.seq;
        addReplyBulkCString(c, b"first-entry\0" as *const u8 as *const libc::c_char);
        emitted = streamReplyWithRange(
            c,
            s,
            &mut start,
            &mut end,
            1 as libc::c_int as size_t,
            0 as libc::c_int,
            0 as *mut streamCG,
            0 as *mut streamConsumer,
            (1 as libc::c_int) << 1 as libc::c_int,
            0 as *mut streamPropInfo,
        ) as libc::c_int;
        if emitted == 0 {
            addReplyNull(c);
        }
        addReplyBulkCString(c, b"last-entry\0" as *const u8 as *const libc::c_char);
        emitted = streamReplyWithRange(
            c,
            s,
            &mut start,
            &mut end,
            1 as libc::c_int as size_t,
            1 as libc::c_int,
            0 as *mut streamCG,
            0 as *mut streamConsumer,
            (1 as libc::c_int) << 1 as libc::c_int,
            0 as *mut streamPropInfo,
        ) as libc::c_int;
        if emitted == 0 {
            addReplyNull(c);
        }
    } else {
        addReplyBulkCString(c, b"entries\0" as *const u8 as *const libc::c_char);
        streamReplyWithRange(
            c,
            s,
            0 as *mut streamID,
            0 as *mut streamID,
            count as size_t,
            0 as libc::c_int,
            0 as *mut streamCG,
            0 as *mut streamConsumer,
            0 as libc::c_int,
            0 as *mut streamPropInfo,
        );
        addReplyBulkCString(c, b"groups\0" as *const u8 as *const libc::c_char);
        if ((*s).cgroups).is_null() {
            addReplyArrayLen(c, 0 as libc::c_int as libc::c_long);
        } else {
            addReplyArrayLen(c, raxSize((*s).cgroups) as libc::c_long);
            let mut ri_cgroups: raxIterator = raxIterator {
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
            raxStart(&mut ri_cgroups, (*s).cgroups);
            raxSeek(
                &mut ri_cgroups,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri_cgroups) != 0 {
                let mut cg: *mut streamCG = ri_cgroups.data as *mut streamCG;
                addReplyMapLen(c, 7 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"name\0" as *const u8 as *const libc::c_char);
                addReplyBulkCBuffer(
                    c,
                    ri_cgroups.key as *const libc::c_void,
                    ri_cgroups.key_len,
                );
                addReplyBulkCString(
                    c,
                    b"last-delivered-id\0" as *const u8 as *const libc::c_char,
                );
                addReplyStreamID(c, &mut (*cg).last_id);
                addReplyBulkCString(
                    c,
                    b"entries-read\0" as *const u8 as *const libc::c_char,
                );
                if (*cg).entries_read != -(1 as libc::c_int) as libc::c_longlong {
                    addReplyLongLong(c, (*cg).entries_read);
                } else {
                    addReplyNull(c);
                }
                addReplyBulkCString(c, b"lag\0" as *const u8 as *const libc::c_char);
                streamReplyWithCGLag(c, s, cg);
                addReplyBulkCString(
                    c,
                    b"pel-count\0" as *const u8 as *const libc::c_char,
                );
                addReplyLongLong(c, raxSize((*cg).pel) as libc::c_longlong);
                addReplyBulkCString(c, b"pending\0" as *const u8 as *const libc::c_char);
                let mut arraylen_cg_pel: libc::c_longlong = 0 as libc::c_int
                    as libc::c_longlong;
                let mut arrayptr_cg_pel: *mut libc::c_void = addReplyDeferredLen(c);
                let mut ri_cg_pel: raxIterator = raxIterator {
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
                raxStart(&mut ri_cg_pel, (*cg).pel);
                raxSeek(
                    &mut ri_cg_pel,
                    b"^\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_uchar,
                    0 as libc::c_int as size_t,
                );
                while raxNext(&mut ri_cg_pel) != 0
                    && (count == 0 || arraylen_cg_pel < count)
                {
                    let mut nack: *mut streamNACK = ri_cg_pel.data as *mut streamNACK;
                    addReplyArrayLen(c, 4 as libc::c_int as libc::c_long);
                    let mut id: streamID = streamID { ms: 0, seq: 0 };
                    streamDecodeID(ri_cg_pel.key as *mut libc::c_void, &mut id);
                    addReplyStreamID(c, &mut id);
                    if !((*nack).consumer).is_null() {} else {
                        _serverAssert(
                            b"nack->consumer\0" as *const u8 as *const libc::c_char,
                            b"t_stream.c\0" as *const u8 as *const libc::c_char,
                            3759 as libc::c_int,
                        );
                        unreachable!();
                    };
                    addReplyBulkCBuffer(
                        c,
                        (*(*nack).consumer).name as *const libc::c_void,
                        sdslen((*(*nack).consumer).name),
                    );
                    addReplyLongLong(c, (*nack).delivery_time);
                    addReplyLongLong(c, (*nack).delivery_count as libc::c_longlong);
                    arraylen_cg_pel += 1;
                }
                setDeferredArrayLen(c, arrayptr_cg_pel, arraylen_cg_pel as libc::c_long);
                raxStop(&mut ri_cg_pel);
                addReplyBulkCString(
                    c,
                    b"consumers\0" as *const u8 as *const libc::c_char,
                );
                addReplyArrayLen(c, raxSize((*cg).consumers) as libc::c_long);
                let mut ri_consumers: raxIterator = raxIterator {
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
                raxStart(&mut ri_consumers, (*cg).consumers);
                raxSeek(
                    &mut ri_consumers,
                    b"^\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_uchar,
                    0 as libc::c_int as size_t,
                );
                while raxNext(&mut ri_consumers) != 0 {
                    let mut consumer: *mut streamConsumer = ri_consumers.data
                        as *mut streamConsumer;
                    addReplyMapLen(c, 4 as libc::c_int as libc::c_long);
                    addReplyBulkCString(
                        c,
                        b"name\0" as *const u8 as *const libc::c_char,
                    );
                    addReplyBulkCBuffer(
                        c,
                        (*consumer).name as *const libc::c_void,
                        sdslen((*consumer).name),
                    );
                    addReplyBulkCString(
                        c,
                        b"seen-time\0" as *const u8 as *const libc::c_char,
                    );
                    addReplyLongLong(c, (*consumer).seen_time);
                    addReplyBulkCString(
                        c,
                        b"pel-count\0" as *const u8 as *const libc::c_char,
                    );
                    addReplyLongLong(c, raxSize((*consumer).pel) as libc::c_longlong);
                    addReplyBulkCString(
                        c,
                        b"pending\0" as *const u8 as *const libc::c_char,
                    );
                    let mut arraylen_cpel: libc::c_longlong = 0 as libc::c_int
                        as libc::c_longlong;
                    let mut arrayptr_cpel: *mut libc::c_void = addReplyDeferredLen(c);
                    let mut ri_cpel: raxIterator = raxIterator {
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
                    raxStart(&mut ri_cpel, (*consumer).pel);
                    raxSeek(
                        &mut ri_cpel,
                        b"^\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_uchar,
                        0 as libc::c_int as size_t,
                    );
                    while raxNext(&mut ri_cpel) != 0
                        && (count == 0 || arraylen_cpel < count)
                    {
                        let mut nack_0: *mut streamNACK = ri_cpel.data
                            as *mut streamNACK;
                        addReplyArrayLen(c, 3 as libc::c_int as libc::c_long);
                        let mut id_0: streamID = streamID { ms: 0, seq: 0 };
                        streamDecodeID(ri_cpel.key as *mut libc::c_void, &mut id_0);
                        addReplyStreamID(c, &mut id_0);
                        addReplyLongLong(c, (*nack_0).delivery_time);
                        addReplyLongLong(
                            c,
                            (*nack_0).delivery_count as libc::c_longlong,
                        );
                        arraylen_cpel += 1;
                    }
                    setDeferredArrayLen(c, arrayptr_cpel, arraylen_cpel as libc::c_long);
                    raxStop(&mut ri_cpel);
                }
                raxStop(&mut ri_consumers);
            }
            raxStop(&mut ri_cgroups);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xinfoCommand(mut c: *mut client) {
    let mut s: *mut stream = 0 as *mut stream;
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut robj = 0 as *mut robj;
    if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"HELP\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if (*c).argc != 2 as libc::c_int {
            addReplySubcommandSyntaxError(c);
            return;
        }
        let mut help: [*const libc::c_char; 7] = [
            b"CONSUMERS <key> <groupname>\0" as *const u8 as *const libc::c_char,
            b"    Show consumers of <groupname>.\0" as *const u8 as *const libc::c_char,
            b"GROUPS <key>\0" as *const u8 as *const libc::c_char,
            b"    Show the stream consumer groups.\0" as *const u8
                as *const libc::c_char,
            b"STREAM <key> [FULL [COUNT <count>]\0" as *const u8 as *const libc::c_char,
            b"    Show information about the stream.\0" as *const u8
                as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
        return;
    } else {
        if (*c).argc < 3 as libc::c_int {
            addReplySubcommandSyntaxError(c);
            return;
        }
    }
    opt = (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *mut libc::c_char;
    key = *((*c).argv).offset(2 as libc::c_int as isize);
    let mut o: *mut robj = lookupKeyReadOrReply(c, key, shared.nokeyerr);
    if o.is_null() || checkType(c, o, 6 as libc::c_int) != 0 {
        return;
    }
    s = (*o).ptr as *mut stream;
    if strcasecmp(opt, b"CONSUMERS\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc == 4 as libc::c_int
    {
        let mut cg: *mut streamCG = streamLookupCG(
            s,
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds,
        );
        if cg.is_null() {
            addReplyErrorFormat(
                c,
                b"-NOGROUP No such consumer group '%s' for key name '%s'\0" as *const u8
                    as *const libc::c_char,
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
                (*key).ptr as *mut libc::c_char,
            );
            return;
        }
        addReplyArrayLen(c, raxSize((*cg).consumers) as libc::c_long);
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
        raxStart(&mut ri, (*cg).consumers);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        let mut now: mstime_t = mstime();
        while raxNext(&mut ri) != 0 {
            let mut consumer: *mut streamConsumer = ri.data as *mut streamConsumer;
            let mut idle: mstime_t = now - (*consumer).seen_time;
            if idle < 0 as libc::c_int as libc::c_longlong {
                idle = 0 as libc::c_int as mstime_t;
            }
            addReplyMapLen(c, 3 as libc::c_int as libc::c_long);
            addReplyBulkCString(c, b"name\0" as *const u8 as *const libc::c_char);
            addReplyBulkCBuffer(
                c,
                (*consumer).name as *const libc::c_void,
                sdslen((*consumer).name),
            );
            addReplyBulkCString(c, b"pending\0" as *const u8 as *const libc::c_char);
            addReplyLongLong(c, raxSize((*consumer).pel) as libc::c_longlong);
            addReplyBulkCString(c, b"idle\0" as *const u8 as *const libc::c_char);
            addReplyLongLong(c, idle);
        }
        raxStop(&mut ri);
    } else if strcasecmp(opt, b"GROUPS\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc == 3 as libc::c_int
    {
        if ((*s).cgroups).is_null() {
            addReplyArrayLen(c, 0 as libc::c_int as libc::c_long);
            return;
        }
        addReplyArrayLen(c, raxSize((*s).cgroups) as libc::c_long);
        let mut ri_0: raxIterator = raxIterator {
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
        raxStart(&mut ri_0, (*s).cgroups);
        raxSeek(
            &mut ri_0,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        while raxNext(&mut ri_0) != 0 {
            let mut cg_0: *mut streamCG = ri_0.data as *mut streamCG;
            addReplyMapLen(c, 6 as libc::c_int as libc::c_long);
            addReplyBulkCString(c, b"name\0" as *const u8 as *const libc::c_char);
            addReplyBulkCBuffer(c, ri_0.key as *const libc::c_void, ri_0.key_len);
            addReplyBulkCString(c, b"consumers\0" as *const u8 as *const libc::c_char);
            addReplyLongLong(c, raxSize((*cg_0).consumers) as libc::c_longlong);
            addReplyBulkCString(c, b"pending\0" as *const u8 as *const libc::c_char);
            addReplyLongLong(c, raxSize((*cg_0).pel) as libc::c_longlong);
            addReplyBulkCString(
                c,
                b"last-delivered-id\0" as *const u8 as *const libc::c_char,
            );
            addReplyStreamID(c, &mut (*cg_0).last_id);
            addReplyBulkCString(
                c,
                b"entries-read\0" as *const u8 as *const libc::c_char,
            );
            if (*cg_0).entries_read != -(1 as libc::c_int) as libc::c_longlong {
                addReplyLongLong(c, (*cg_0).entries_read);
            } else {
                addReplyNull(c);
            }
            addReplyBulkCString(c, b"lag\0" as *const u8 as *const libc::c_char);
            streamReplyWithCGLag(c, s, cg_0);
        }
        raxStop(&mut ri_0);
    } else if strcasecmp(opt, b"STREAM\0" as *const u8 as *const libc::c_char) == 0 {
        xinfoReplyWithStreamInfo(c, s);
    } else {
        addReplySubcommandSyntaxError(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn streamValidateListpackIntegrity(
    mut lp: *mut libc::c_uchar,
    mut size: size_t,
    mut deep: libc::c_int,
) -> libc::c_int {
    let mut valid_record: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if lpValidateIntegrity(lp, size, 0 as libc::c_int, None, 0 as *mut libc::c_void) == 0
    {
        return 0 as libc::c_int;
    }
    if deep == 0 {
        return 1 as libc::c_int;
    }
    p = lpValidateFirst(lp);
    next = p;
    if lpValidateNext(lp, &mut next, size) == 0 {
        return 0 as libc::c_int;
    }
    if p.is_null() {
        return 0 as libc::c_int;
    }
    let mut entry_count: int64_t = lpGetIntegerIfValid(p, &mut valid_record);
    if valid_record == 0 {
        return 0 as libc::c_int;
    }
    p = next;
    if lpValidateNext(lp, &mut next, size) == 0 {
        return 0 as libc::c_int;
    }
    let mut deleted_count: int64_t = lpGetIntegerIfValid(p, &mut valid_record);
    if valid_record == 0 {
        return 0 as libc::c_int;
    }
    p = next;
    if lpValidateNext(lp, &mut next, size) == 0 {
        return 0 as libc::c_int;
    }
    let mut master_fields: int64_t = lpGetIntegerIfValid(p, &mut valid_record);
    if valid_record == 0 {
        return 0 as libc::c_int;
    }
    p = next;
    if lpValidateNext(lp, &mut next, size) == 0 {
        return 0 as libc::c_int;
    }
    let mut j: int64_t = 0 as libc::c_int as int64_t;
    while j < master_fields {
        p = next;
        if lpValidateNext(lp, &mut next, size) == 0 {
            return 0 as libc::c_int;
        }
        j += 1;
    }
    let mut zero: int64_t = lpGetIntegerIfValid(p, &mut valid_record);
    if valid_record == 0 || zero != 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    p = next;
    if lpValidateNext(lp, &mut next, size) == 0 {
        return 0 as libc::c_int;
    }
    entry_count += deleted_count;
    loop {
        let fresh9 = entry_count;
        entry_count = entry_count - 1;
        if !(fresh9 != 0) {
            break;
        }
        if p.is_null() {
            return 0 as libc::c_int;
        }
        let mut fields: int64_t = master_fields;
        let mut extra_fields: int64_t = 3 as libc::c_int as int64_t;
        let mut flags: int64_t = lpGetIntegerIfValid(p, &mut valid_record);
        if valid_record == 0 {
            return 0 as libc::c_int;
        }
        p = next;
        if lpValidateNext(lp, &mut next, size) == 0 {
            return 0 as libc::c_int;
        }
        lpGetIntegerIfValid(p, &mut valid_record);
        if valid_record == 0 {
            return 0 as libc::c_int;
        }
        p = next;
        if lpValidateNext(lp, &mut next, size) == 0 {
            return 0 as libc::c_int;
        }
        lpGetIntegerIfValid(p, &mut valid_record);
        if valid_record == 0 {
            return 0 as libc::c_int;
        }
        p = next;
        if lpValidateNext(lp, &mut next, size) == 0 {
            return 0 as libc::c_int;
        }
        if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_long == 0 {
            fields = lpGetIntegerIfValid(p, &mut valid_record);
            if valid_record == 0 {
                return 0 as libc::c_int;
            }
            p = next;
            if lpValidateNext(lp, &mut next, size) == 0 {
                return 0 as libc::c_int;
            }
            let mut j_0: int64_t = 0 as libc::c_int as int64_t;
            while j_0 < fields {
                p = next;
                if lpValidateNext(lp, &mut next, size) == 0 {
                    return 0 as libc::c_int;
                }
                j_0 += 1;
            }
            extra_fields += fields + 1 as libc::c_int as libc::c_long;
        }
        let mut j_1: int64_t = 0 as libc::c_int as int64_t;
        while j_1 < fields {
            p = next;
            if lpValidateNext(lp, &mut next, size) == 0 {
                return 0 as libc::c_int;
            }
            j_1 += 1;
        }
        let mut lp_count: int64_t = lpGetIntegerIfValid(p, &mut valid_record);
        if valid_record == 0 {
            return 0 as libc::c_int;
        }
        if lp_count != fields + extra_fields {
            return 0 as libc::c_int;
        }
        p = next;
        if lpValidateNext(lp, &mut next, size) == 0 {
            return 0 as libc::c_int;
        }
    }
    if !next.is_null() {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
