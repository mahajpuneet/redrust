extern crate c2rust_bitfields;
extern crate libc;
extern crate core;

extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut hashDictType: dictType;
    static mut sdsReplyDictType: dictType;
    fn addReplyNull(c: *mut client);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyBulkLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyErrorArity(c: *mut client);
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdsfromlonglong(value: libc::c_longlong) -> sds;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictExpand(d: *mut dict, size: libc::c_ulong) -> libc::c_int;
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
    fn dictResize(d: *mut dict) -> libc::c_int;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictGetFairRandomKey(d: *mut dict) -> *mut dictEntry;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn sdigits10(v: int64_t) -> uint32_t;
    fn string2ll(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_longlong,
    ) -> libc::c_int;
    fn string2ld(
        s: *const libc::c_char,
        slen: size_t,
        dp: *mut f64,
    ) -> libc::c_int;
    fn ld2string(
        buf: *mut libc::c_char,
        len: size_t,
        value: f64,
        mode: ld2string_mode,
    ) -> libc::c_int;
    fn lpAppend(
        lp: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> *mut libc::c_uchar;
    fn lpReplace(
        lp: *mut libc::c_uchar,
        p: *mut *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> *mut libc::c_uchar;
    fn lpDeleteRangeWithEntry(
        lp: *mut libc::c_uchar,
        p: *mut *mut libc::c_uchar,
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
    fn lpBytes(lp: *mut libc::c_uchar) -> size_t;
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
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn rewriteClientCommandArgument(c: *mut client, i: libc::c_int, newval: *mut robj);
    fn decrRefCount(o: *mut robj);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn createRawStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn createStringObjectFromLongLong(value: libc::c_longlong) -> *mut robj;
    fn createHashObject() -> *mut robj;
    fn getLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
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
    fn getLongDoubleFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut f64,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn htNeedsResize(dict: *mut dict) -> libc::c_int;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn serverLogHexDump(
        level: libc::c_int,
        descr: *mut libc::c_char,
        value: *mut libc::c_void,
        len: size_t,
    );
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn lookupKeyWrite(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn dbAdd(db: *mut redisDb, key: *mut robj, val: *mut robj);
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
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
    fn dbDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn scanGenericCommand(c: *mut client, o: *mut robj, cursor: libc::c_ulong);
    fn parseScanCursorOrReply(
        c: *mut client,
        o: *mut robj,
        cursor: *mut libc::c_ulong,
    ) -> libc::c_int;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
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
pub type ld2string_mode = libc::c_uint;
pub const LD_STR_HEX: ld2string_mode = 2;
pub const LD_STR_HUMAN: ld2string_mode = 1;
pub const LD_STR_AUTO: ld2string_mode = 0;
#[derive(Copy, Clone, c2rust_bitfields:: BitfieldStruct)]
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
pub struct hashTypeIterator {
    pub subject: *mut robj,
    pub encoding: libc::c_int,
    pub fptr: *mut libc::c_uchar,
    pub vptr: *mut libc::c_uchar,
    pub di: *mut dictIterator,
    pub de: *mut dictEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listpackEntry {
    pub sval: *mut libc::c_uchar,
    pub slen: uint32_t,
    pub lval: libc::c_longlong,
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
pub unsafe extern "C" fn hashTypeTryConversion(
    mut o: *mut robj,
    mut argv: *mut *mut robj,
    mut start: libc::c_int,
    mut end: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut sum: size_t = 0 as libc::c_int as size_t;
    if (*o).encoding() as libc::c_int != 11 as libc::c_int {
        return;
    }
    i = start;
    while i <= end {
        if (**argv.offset(i as isize)).encoding() as libc::c_int == 0 as libc::c_int
            || (**argv.offset(i as isize)).encoding() as libc::c_int == 8 as libc::c_int
        {
            let mut len: size_t = sdslen((**argv.offset(i as isize)).ptr as sds);
            if len > server.hash_max_listpack_value {
                hashTypeConvert(o, 2 as libc::c_int);
                return;
            }
            sum = (sum as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        }
        i += 1;
    }
    if lpSafeToAdd((*o).ptr as *mut libc::c_uchar, sum) == 0 {
        hashTypeConvert(o, 2 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeGetFromListpack(
    mut o: *mut robj,
    mut field: sds,
    mut vstr: *mut *mut libc::c_uchar,
    mut vlen: *mut libc::c_uint,
    mut vll: *mut libc::c_longlong,
) -> libc::c_int {
    let mut zl: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut fptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {} else {
        _serverAssert(
            b"o->encoding == OBJ_ENCODING_LISTPACK\0" as *const u8
                as *const libc::c_char,
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
        );
        unreachable!();
    };
    zl = (*o).ptr as *mut libc::c_uchar;
    fptr = lpFirst(zl);
    if !fptr.is_null() {
        fptr = lpFind(
            zl,
            fptr,
            field as *mut libc::c_uchar,
            sdslen(field) as uint32_t,
            1 as libc::c_int as libc::c_uint,
        );
        if !fptr.is_null() {
            vptr = lpNext(zl, fptr);
            if !vptr.is_null() {} else {
                _serverAssert(
                    b"vptr != NULL\0" as *const u8 as *const libc::c_char,
                    b"t_hash.c\0" as *const u8 as *const libc::c_char,
                    78 as libc::c_int,
                );
                unreachable!();
            };
        }
    }
    if !vptr.is_null() {
        *vstr = lpGetValue(vptr, vlen, vll);
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeGetFromHashTable(
    mut o: *mut robj,
    mut field: sds,
) -> sds {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    if (*o).encoding() as libc::c_int == 2 as libc::c_int {} else {
        _serverAssert(
            b"o->encoding == OBJ_ENCODING_HT\0" as *const u8 as *const libc::c_char,
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
        );
        unreachable!();
    };
    de = dictFind((*o).ptr as *mut dict, field as *const libc::c_void);
    if de.is_null() {
        return 0 as sds;
    }
    return (*de).v.val as sds;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeGetValue(
    mut o: *mut robj,
    mut field: sds,
    mut vstr: *mut *mut libc::c_uchar,
    mut vlen: *mut libc::c_uint,
    mut vll: *mut libc::c_longlong,
) -> libc::c_int {
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        *vstr = 0 as *mut libc::c_uchar;
        if hashTypeGetFromListpack(o, field, vstr, vlen, vll) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        let mut value: sds = 0 as *mut libc::c_char;
        value = hashTypeGetFromHashTable(o, field);
        if !value.is_null() {
            *vstr = value as *mut libc::c_uchar;
            *vlen = sdslen(value) as libc::c_uint;
            return 0 as libc::c_int;
        }
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeGetValueObject(
    mut o: *mut robj,
    mut field: sds,
) -> *mut robj {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = 0;
    let mut vll: libc::c_longlong = 0;
    if hashTypeGetValue(o, field, &mut vstr, &mut vlen, &mut vll) == -(1 as libc::c_int)
    {
        return 0 as *mut robj;
    }
    if !vstr.is_null() {
        return createStringObject(vstr as *mut libc::c_char, vlen as size_t)
    } else {
        return createStringObjectFromLongLong(vll)
    };
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeGetValueLength(
    mut o: *mut robj,
    mut field: sds,
) -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    let mut vll: libc::c_longlong = 9223372036854775807 as libc::c_longlong;
    if hashTypeGetValue(o, field, &mut vstr, &mut vlen, &mut vll) == 0 as libc::c_int {
        len = (if !vstr.is_null() { vlen } else { sdigits10(vll as int64_t) }) as size_t;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeExists(
    mut o: *mut robj,
    mut field: sds,
) -> libc::c_int {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    let mut vll: libc::c_longlong = 9223372036854775807 as libc::c_longlong;
    return (hashTypeGetValue(o, field, &mut vstr, &mut vlen, &mut vll)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeSet(
    mut o: *mut robj,
    mut field: sds,
    mut value: sds,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut update: libc::c_int = 0 as libc::c_int;
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        if sdslen(field) > server.hash_max_listpack_value
            || sdslen(value) > server.hash_max_listpack_value
        {
            hashTypeConvert(o, 2 as libc::c_int);
        }
    }
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut fptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        zl = (*o).ptr as *mut libc::c_uchar;
        fptr = lpFirst(zl);
        if !fptr.is_null() {
            fptr = lpFind(
                zl,
                fptr,
                field as *mut libc::c_uchar,
                sdslen(field) as uint32_t,
                1 as libc::c_int as libc::c_uint,
            );
            if !fptr.is_null() {
                vptr = lpNext(zl, fptr);
                if !vptr.is_null() {} else {
                    _serverAssert(
                        b"vptr != NULL\0" as *const u8 as *const libc::c_char,
                        b"t_hash.c\0" as *const u8 as *const libc::c_char,
                        211 as libc::c_int,
                    );
                    unreachable!();
                };
                update = 1 as libc::c_int;
                zl = lpReplace(
                    zl,
                    &mut vptr,
                    value as *mut libc::c_uchar,
                    sdslen(value) as uint32_t,
                );
            }
        }
        if update == 0 {
            zl = lpAppend(zl, field as *mut libc::c_uchar, sdslen(field) as uint32_t);
            zl = lpAppend(zl, value as *mut libc::c_uchar, sdslen(value) as uint32_t);
        }
        (*o).ptr = zl as *mut libc::c_void;
        if hashTypeLength(o) > server.hash_max_listpack_entries {
            hashTypeConvert(o, 2 as libc::c_int);
        }
    } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        let mut de: *mut dictEntry = dictFind(
            (*o).ptr as *mut dict,
            field as *const libc::c_void,
        );
        if !de.is_null() {
            sdsfree((*de).v.val as sds);
            if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                (*de).v.val = value as *mut libc::c_void;
                value = 0 as sds;
            } else {
                (*de).v.val = sdsdup(value) as *mut libc::c_void;
            }
            update = 1 as libc::c_int;
        } else {
            let mut f: sds = 0 as *mut libc::c_char;
            let mut v: sds = 0 as *mut libc::c_char;
            if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                f = field;
                field = 0 as sds;
            } else {
                f = sdsdup(field);
            }
            if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                v = value;
                value = 0 as sds;
            } else {
                v = sdsdup(value);
            }
            dictAdd(
                (*o).ptr as *mut dict,
                f as *mut libc::c_void,
                v as *mut libc::c_void,
            );
        }
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 && !field.is_null() {
        sdsfree(field);
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 && !value.is_null() {
        sdsfree(value);
    }
    return update;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeDelete(
    mut o: *mut robj,
    mut field: sds,
) -> libc::c_int {
    let mut deleted: libc::c_int = 0 as libc::c_int;
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut fptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        zl = (*o).ptr as *mut libc::c_uchar;
        fptr = lpFirst(zl);
        if !fptr.is_null() {
            fptr = lpFind(
                zl,
                fptr,
                field as *mut libc::c_uchar,
                sdslen(field) as uint32_t,
                1 as libc::c_int as libc::c_uint,
            );
            if !fptr.is_null() {
                zl = lpDeleteRangeWithEntry(
                    zl,
                    &mut fptr,
                    2 as libc::c_int as libc::c_ulong,
                );
                (*o).ptr = zl as *mut libc::c_void;
                deleted = 1 as libc::c_int;
            }
        }
    } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        if dictDelete((*o).ptr as *mut dict, field as *const libc::c_void)
            == 0 as libc::c_int
        {
            deleted = 1 as libc::c_int;
            if htNeedsResize((*o).ptr as *mut dict) != 0 {
                dictResize((*o).ptr as *mut dict);
            }
        }
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return deleted;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeLength(mut o: *const robj) -> libc::c_ulong {
    let mut length: libc::c_ulong = (9223372036854775807 as libc::c_long
        as libc::c_ulong)
        .wrapping_mul(2 as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong);
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        length = (lpLength((*o).ptr as *mut libc::c_uchar))
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
    } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        length = ((*((*o).ptr as *const dict)).ht_used[0 as libc::c_int as usize])
            .wrapping_add(
                (*((*o).ptr as *const dict)).ht_used[1 as libc::c_int as usize],
            );
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeInitIterator(
    mut subject: *mut robj,
) -> *mut hashTypeIterator {
    let mut hi: *mut hashTypeIterator = zmalloc(
        core::mem::size_of::<hashTypeIterator>() as libc::c_ulong,
    ) as *mut hashTypeIterator;
    (*hi).subject = subject;
    (*hi).encoding = (*subject).encoding() as libc::c_int;
    if (*hi).encoding == 11 as libc::c_int {
        (*hi).fptr = 0 as *mut libc::c_uchar;
        (*hi).vptr = 0 as *mut libc::c_uchar;
    } else if (*hi).encoding == 2 as libc::c_int {
        (*hi).di = dictGetIterator((*subject).ptr as *mut dict);
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            325 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return hi;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeReleaseIterator(mut hi: *mut hashTypeIterator) {
    if (*hi).encoding == 2 as libc::c_int {
        dictReleaseIterator((*hi).di);
    }
    zfree(hi as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeNext(mut hi: *mut hashTypeIterator) -> libc::c_int {
    if (*hi).encoding == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut fptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        zl = (*(*hi).subject).ptr as *mut libc::c_uchar;
        fptr = (*hi).fptr;
        vptr = (*hi).vptr;
        if fptr.is_null() {
            if vptr.is_null() {} else {
                _serverAssert(
                    b"vptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"t_hash.c\0" as *const u8 as *const libc::c_char,
                    349 as libc::c_int,
                );
                unreachable!();
            };
            fptr = lpFirst(zl);
        } else {
            if !vptr.is_null() {} else {
                _serverAssert(
                    b"vptr != NULL\0" as *const u8 as *const libc::c_char,
                    b"t_hash.c\0" as *const u8 as *const libc::c_char,
                    353 as libc::c_int,
                );
                unreachable!();
            };
            fptr = lpNext(zl, vptr);
        }
        if fptr.is_null() {
            return -(1 as libc::c_int);
        }
        vptr = lpNext(zl, fptr);
        if !vptr.is_null() {} else {
            _serverAssert(
                b"vptr != NULL\0" as *const u8 as *const libc::c_char,
                b"t_hash.c\0" as *const u8 as *const libc::c_char,
                360 as libc::c_int,
            );
            unreachable!();
        };
        (*hi).fptr = fptr;
        (*hi).vptr = vptr;
    } else if (*hi).encoding == 2 as libc::c_int {
        (*hi).de = dictNext((*hi).di);
        if ((*hi).de).is_null() {
            return -(1 as libc::c_int);
        }
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            368 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeCurrentFromListpack(
    mut hi: *mut hashTypeIterator,
    mut what: libc::c_int,
    mut vstr: *mut *mut libc::c_uchar,
    mut vlen: *mut libc::c_uint,
    mut vll: *mut libc::c_longlong,
) {
    if (*hi).encoding == 11 as libc::c_int {} else {
        _serverAssert(
            b"hi->encoding == OBJ_ENCODING_LISTPACK\0" as *const u8
                as *const libc::c_char,
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            380 as libc::c_int,
        );
        unreachable!();
    };
    if what & 1 as libc::c_int != 0 {
        *vstr = lpGetValue((*hi).fptr, vlen, vll);
    } else {
        *vstr = lpGetValue((*hi).vptr, vlen, vll);
    };
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeCurrentFromHashTable(
    mut hi: *mut hashTypeIterator,
    mut what: libc::c_int,
) -> sds {
    if (*hi).encoding == 2 as libc::c_int {} else {
        _serverAssert(
            b"hi->encoding == OBJ_ENCODING_HT\0" as *const u8 as *const libc::c_char,
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
        );
        unreachable!();
    };
    if what & 1 as libc::c_int != 0 {
        return (*(*hi).de).key as sds
    } else {
        return (*(*hi).de).v.val as sds
    };
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeCurrentObject(
    mut hi: *mut hashTypeIterator,
    mut what: libc::c_int,
    mut vstr: *mut *mut libc::c_uchar,
    mut vlen: *mut libc::c_uint,
    mut vll: *mut libc::c_longlong,
) {
    if (*hi).encoding == 11 as libc::c_int {
        *vstr = 0 as *mut libc::c_uchar;
        hashTypeCurrentFromListpack(hi, what, vstr, vlen, vll);
    } else if (*hi).encoding == 2 as libc::c_int {
        let mut ele: sds = hashTypeCurrentFromHashTable(hi, what);
        *vstr = ele as *mut libc::c_uchar;
        *vlen = sdslen(ele) as libc::c_uint;
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeCurrentObjectNewSds(
    mut hi: *mut hashTypeIterator,
    mut what: libc::c_int,
) -> sds {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = 0;
    let mut vll: libc::c_longlong = 0;
    hashTypeCurrentObject(hi, what, &mut vstr, &mut vlen, &mut vll);
    if !vstr.is_null() {
        return sdsnewlen(vstr as *const libc::c_void, vlen as size_t);
    }
    return sdsfromlonglong(vll);
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeLookupWriteOrCreate(
    mut c: *mut client,
    mut key: *mut robj,
) -> *mut robj {
    let mut o: *mut robj = lookupKeyWrite((*c).db, key);
    if checkType(c, o, 4 as libc::c_int) != 0 {
        return 0 as *mut robj;
    }
    if o.is_null() {
        o = createHashObject();
        dbAdd((*c).db, key, o);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeConvertListpack(
    mut o: *mut robj,
    mut enc: libc::c_int,
) {
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {} else {
        _serverAssert(
            b"o->encoding == OBJ_ENCODING_LISTPACK\0" as *const u8
                as *const libc::c_char,
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            450 as libc::c_int,
        );
        unreachable!();
    };
    if !(enc == 11 as libc::c_int) {
        if enc == 2 as libc::c_int {
            let mut hi: *mut hashTypeIterator = 0 as *mut hashTypeIterator;
            let mut dict: *mut dict = 0 as *mut dict;
            let mut ret: libc::c_int = 0;
            hi = hashTypeInitIterator(o);
            dict = dictCreate(&mut hashDictType);
            dictExpand(dict, hashTypeLength(o));
            while hashTypeNext(hi) != -(1 as libc::c_int) {
                let mut key: sds = 0 as *mut libc::c_char;
                let mut value: sds = 0 as *mut libc::c_char;
                key = hashTypeCurrentObjectNewSds(hi, 1 as libc::c_int);
                value = hashTypeCurrentObjectNewSds(hi, 2 as libc::c_int);
                ret = dictAdd(
                    dict,
                    key as *mut libc::c_void,
                    value as *mut libc::c_void,
                );
                if ret != 0 as libc::c_int {
                    sdsfree(key);
                    sdsfree(value);
                    hashTypeReleaseIterator(hi);
                    serverLogHexDump(
                        3 as libc::c_int,
                        b"listpack with dup elements dump\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*o).ptr,
                        lpBytes((*o).ptr as *mut libc::c_uchar),
                    );
                    _serverPanic(
                        b"t_hash.c\0" as *const u8 as *const libc::c_char,
                        477 as libc::c_int,
                        b"Listpack corruption detected\0" as *const u8
                            as *const libc::c_char,
                    );
                    unreachable!();
                }
            }
            hashTypeReleaseIterator(hi);
            zfree((*o).ptr);
            (*o).set_encoding(2 as libc::c_int as libc::c_uint);
            (*o).ptr = dict as *mut libc::c_void;
        } else {
            _serverPanic(
                b"t_hash.c\0" as *const u8 as *const libc::c_char,
                485 as libc::c_int,
                b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeConvert(mut o: *mut robj, mut enc: libc::c_int) {
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        hashTypeConvertListpack(o, enc);
    } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            493 as libc::c_int,
            b"Not implemented\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            495 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn hashTypeDup(mut o: *mut robj) -> *mut robj {
    let mut hobj: *mut robj = 0 as *mut robj;
    let mut hi: *mut hashTypeIterator = 0 as *mut hashTypeIterator;
    if (*o).type_0() as libc::c_int == 4 as libc::c_int {} else {
        _serverAssert(
            b"o->type == OBJ_HASH\0" as *const u8 as *const libc::c_char,
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            508 as libc::c_int,
        );
        unreachable!();
    };
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*o).ptr as *mut libc::c_uchar;
        let mut sz: size_t = lpBytes(zl);
        let mut new_zl: *mut libc::c_uchar = zmalloc(sz) as *mut libc::c_uchar;
        memcpy(new_zl as *mut libc::c_void, zl as *const libc::c_void, sz);
        hobj = createObject(4 as libc::c_int, new_zl as *mut libc::c_void);
        (*hobj).set_encoding(11 as libc::c_int as libc::c_uint);
    } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        let mut d: *mut dict = dictCreate(&mut hashDictType);
        dictExpand(
            d,
            ((*((*o).ptr as *const dict)).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*((*o).ptr as *const dict)).ht_used[1 as libc::c_int as usize],
                ),
        );
        hi = hashTypeInitIterator(o);
        while hashTypeNext(hi) != -(1 as libc::c_int) {
            let mut field: sds = 0 as *mut libc::c_char;
            let mut value: sds = 0 as *mut libc::c_char;
            let mut newfield: sds = 0 as *mut libc::c_char;
            let mut newvalue: sds = 0 as *mut libc::c_char;
            field = hashTypeCurrentFromHashTable(hi, 1 as libc::c_int);
            value = hashTypeCurrentFromHashTable(hi, 2 as libc::c_int);
            newfield = sdsdup(field);
            newvalue = sdsdup(value);
            dictAdd(d, newfield as *mut libc::c_void, newvalue as *mut libc::c_void);
        }
        hashTypeReleaseIterator(hi);
        hobj = createObject(4 as libc::c_int, d as *mut libc::c_void);
        (*hobj).set_encoding(2 as libc::c_int as libc::c_uint);
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            539 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return hobj;
}
#[no_mangle]
pub unsafe extern "C" fn hashSdsFromListpackEntry(mut e: *mut listpackEntry) -> sds {
    return if !((*e).sval).is_null() {
        sdsnewlen((*e).sval as *const libc::c_void, (*e).slen as size_t)
    } else {
        sdsfromlonglong((*e).lval)
    };
}
#[no_mangle]
pub unsafe extern "C" fn hashReplyFromListpackEntry(
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
pub unsafe extern "C" fn hashTypeRandomElement(
    mut hashobj: *mut robj,
    mut hashsize: libc::c_ulong,
    mut key: *mut listpackEntry,
    mut val: *mut listpackEntry,
) {
    if (*hashobj).encoding() as libc::c_int == 2 as libc::c_int {
        let mut de: *mut dictEntry = dictGetFairRandomKey((*hashobj).ptr as *mut dict);
        let mut s: sds = (*de).key as sds;
        (*key).sval = s as *mut libc::c_uchar;
        (*key).slen = sdslen(s) as uint32_t;
        if !val.is_null() {
            let mut s_0: sds = (*de).v.val as sds;
            (*val).sval = s_0 as *mut libc::c_uchar;
            (*val).slen = sdslen(s_0) as uint32_t;
        }
    } else if (*hashobj).encoding() as libc::c_int == 11 as libc::c_int {
        lpRandomPair((*hashobj).ptr as *mut libc::c_uchar, hashsize, key, val);
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            575 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn hsetnxCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    o = hashTypeLookupWriteOrCreate(c, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        return;
    }
    if hashTypeExists(o, (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds)
        != 0
    {
        addReply(c, shared.czero);
    } else {
        hashTypeTryConversion(o, (*c).argv, 2 as libc::c_int, 3 as libc::c_int);
        hashTypeSet(
            o,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds,
            0 as libc::c_int,
        );
        addReply(c, shared.cone);
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 6 as libc::c_int,
            b"hset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        server.dirty += 1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn hsetCommand(mut c: *mut client) {
    let mut i: libc::c_int = 0;
    let mut created: libc::c_int = 0 as libc::c_int;
    let mut o: *mut robj = 0 as *mut robj;
    if (*c).argc % 2 as libc::c_int == 1 as libc::c_int {
        addReplyErrorArity(c);
        return;
    }
    o = hashTypeLookupWriteOrCreate(c, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        return;
    }
    hashTypeTryConversion(o, (*c).argv, 2 as libc::c_int, (*c).argc - 1 as libc::c_int);
    i = 2 as libc::c_int;
    while i < (*c).argc {
        created
            += (hashTypeSet(
                o,
                (**((*c).argv).offset(i as isize)).ptr as sds,
                (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr as sds,
                0 as libc::c_int,
            ) == 0) as libc::c_int;
        i += 2 as libc::c_int;
    }
    let mut cmdname: *mut libc::c_char = (**((*c).argv)
        .offset(0 as libc::c_int as isize))
        .ptr as *mut libc::c_char;
    if *cmdname.offset(1 as libc::c_int as isize) as libc::c_int == 's' as i32
        || *cmdname.offset(1 as libc::c_int as isize) as libc::c_int == 'S' as i32
    {
        addReplyLongLong(c, created as libc::c_longlong);
    } else {
        addReply(c, shared.ok);
    }
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 6 as libc::c_int,
        b"hset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty
        += (((*c).argc - 2 as libc::c_int) / 2 as libc::c_int) as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn hincrbyCommand(mut c: *mut client) {
    let mut value: libc::c_longlong = 0;
    let mut incr: libc::c_longlong = 0;
    let mut oldvalue: libc::c_longlong = 0;
    let mut o: *mut robj = 0 as *mut robj;
    let mut new: sds = 0 as *mut libc::c_char;
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = 0;
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(3 as libc::c_int as isize),
        &mut incr,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    o = hashTypeLookupWriteOrCreate(c, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        return;
    }
    if hashTypeGetValue(
        o,
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        &mut vstr,
        &mut vlen,
        &mut value,
    ) == 0 as libc::c_int
    {
        if !vstr.is_null() {
            if string2ll(vstr as *mut libc::c_char, vlen as size_t, &mut value)
                == 0 as libc::c_int
            {
                addReplyError(
                    c,
                    b"hash value is not an integer\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
        }
    } else {
        value = 0 as libc::c_int as libc::c_longlong;
    }
    oldvalue = value;
    if incr < 0 as libc::c_int as libc::c_longlong
        && oldvalue < 0 as libc::c_int as libc::c_longlong
        && incr
            < -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                - oldvalue
        || incr > 0 as libc::c_int as libc::c_longlong
            && oldvalue > 0 as libc::c_int as libc::c_longlong
            && incr > 9223372036854775807 as libc::c_longlong - oldvalue
    {
        addReplyError(
            c,
            b"increment or decrement would overflow\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    value += incr;
    new = sdsfromlonglong(value);
    hashTypeSet(
        o,
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        new,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
    addReplyLongLong(c, value);
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 6 as libc::c_int,
        b"hincrby\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
}
#[no_mangle]
pub unsafe extern "C" fn hincrbyfloatCommand(mut c: *mut client) {
    let mut value: f64 = 0 as f64;
    let mut incr: f64 = 0 as f64;
    let mut ll: libc::c_longlong = 0;
    let mut o: *mut robj = 0 as *mut robj;
    let mut new: sds = 0 as *mut libc::c_char;
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = 0;
    if getLongDoubleFromObjectOrReply(
        c,
        *((*c).argv).offset(3 as libc::c_int as isize),
        &mut incr,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    o = hashTypeLookupWriteOrCreate(c, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        return;
    }
    if hashTypeGetValue(
        o,
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        &mut vstr,
        &mut vlen,
        &mut ll,
    ) == 0 as libc::c_int
    {
        if !vstr.is_null() {
            if string2ld(vstr as *mut libc::c_char, vlen as size_t, &mut value)
                == 0 as libc::c_int
            {
                addReplyError(
                    c,
                    b"hash value is not a float\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
        } else {
            value = ll as f64;
        }
    } else {
        value = (0 as libc::c_int) as f64;
    }
    value += incr;
    if value.is_nan() as i32 != 0
        || if value.is_infinite() {
            if value.is_sign_positive() { 1 } else { -1 }
        } else {
            0
        } != 0
    {
        addReplyError(
            c,
            b"increment would produce NaN or Infinity\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    let mut buf: [libc::c_char; 5120] = [0; 5120];
    let mut len: libc::c_int = ld2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 5120]>() as libc::c_ulong,
        value,
        LD_STR_HUMAN,
    );
    new = sdsnewlen(buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    hashTypeSet(
        o,
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        new,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
    addReplyBulkCBuffer(c, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 6 as libc::c_int,
        b"hincrbyfloat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
    let mut newobj: *mut robj = 0 as *mut robj;
    newobj = createRawStringObject(buf.as_mut_ptr(), len as size_t);
    rewriteClientCommandArgument(c, 0 as libc::c_int, shared.hset);
    rewriteClientCommandArgument(c, 3 as libc::c_int, newobj);
    decrRefCount(newobj);
}
unsafe extern "C" fn addHashFieldToReply(
    mut c: *mut client,
    mut o: *mut robj,
    mut field: sds,
) {
    if o.is_null() {
        addReplyNull(c);
        return;
    }
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    let mut vll: libc::c_longlong = 9223372036854775807 as libc::c_longlong;
    if hashTypeGetValue(o, field, &mut vstr, &mut vlen, &mut vll) == 0 as libc::c_int {
        if !vstr.is_null() {
            addReplyBulkCBuffer(c, vstr as *const libc::c_void, vlen as size_t);
        } else {
            addReplyBulkLongLong(c, vll);
        }
    } else {
        addReplyNull(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn hgetCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.null[(*c).resp as usize],
    );
    if o.is_null() || checkType(c, o, 4 as libc::c_int) != 0 {
        return;
    }
    addHashFieldToReply(
        c,
        o,
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
    );
}
#[no_mangle]
pub unsafe extern "C" fn hmgetCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut i: libc::c_int = 0;
    o = lookupKeyRead((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if checkType(c, o, 4 as libc::c_int) != 0 {
        return;
    }
    addReplyArrayLen(c, ((*c).argc - 2 as libc::c_int) as libc::c_long);
    i = 2 as libc::c_int;
    while i < (*c).argc {
        addHashFieldToReply(c, o, (**((*c).argv).offset(i as isize)).ptr as sds);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn hdelCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut j: libc::c_int = 0;
    let mut deleted: libc::c_int = 0 as libc::c_int;
    let mut keyremoved: libc::c_int = 0 as libc::c_int;
    o = lookupKeyWriteOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 4 as libc::c_int) != 0 {
        return;
    }
    j = 2 as libc::c_int;
    while j < (*c).argc {
        if hashTypeDelete(o, (**((*c).argv).offset(j as isize)).ptr as sds) != 0 {
            deleted += 1;
            if hashTypeLength(o) == 0 as libc::c_int as libc::c_ulong {
                dbDelete((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
                keyremoved = 1 as libc::c_int;
                break;
            }
        }
        j += 1;
    }
    if deleted != 0 {
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 6 as libc::c_int,
            b"hdel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        if keyremoved != 0 {
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 2 as libc::c_int,
                b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                *((*c).argv).offset(1 as libc::c_int as isize),
                (*(*c).db).id,
            );
        }
        server.dirty += deleted as libc::c_longlong;
    }
    addReplyLongLong(c, deleted as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn hlenCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 4 as libc::c_int) != 0 {
        return;
    }
    addReplyLongLong(c, hashTypeLength(o) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn hstrlenCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 4 as libc::c_int) != 0 {
        return;
    }
    addReplyLongLong(
        c,
        hashTypeGetValueLength(
            o,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        ) as libc::c_longlong,
    );
}
unsafe extern "C" fn addHashIteratorCursorToReply(
    mut c: *mut client,
    mut hi: *mut hashTypeIterator,
    mut what: libc::c_int,
) {
    if (*hi).encoding == 11 as libc::c_int {
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint);
        let mut vll: libc::c_longlong = 9223372036854775807 as libc::c_longlong;
        hashTypeCurrentFromListpack(hi, what, &mut vstr, &mut vlen, &mut vll);
        if !vstr.is_null() {
            addReplyBulkCBuffer(c, vstr as *const libc::c_void, vlen as size_t);
        } else {
            addReplyBulkLongLong(c, vll);
        }
    } else if (*hi).encoding == 2 as libc::c_int {
        let mut value: sds = hashTypeCurrentFromHashTable(hi, what);
        addReplyBulkCBuffer(c, value as *const libc::c_void, sdslen(value));
    } else {
        _serverPanic(
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            817 as libc::c_int,
            b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn genericHgetallCommand(
    mut c: *mut client,
    mut flags: libc::c_int,
) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut hi: *mut hashTypeIterator = 0 as *mut hashTypeIterator;
    let mut length: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut emptyResp: *mut robj = if flags & 1 as libc::c_int != 0
        && flags & 2 as libc::c_int != 0
    {
        shared.emptymap[(*c).resp as usize]
    } else {
        shared.emptyarray
    };
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        emptyResp,
    );
    if o.is_null() || checkType(c, o, 4 as libc::c_int) != 0 {
        return;
    }
    length = hashTypeLength(o) as libc::c_int;
    if flags & 1 as libc::c_int != 0 && flags & 2 as libc::c_int != 0 {
        addReplyMapLen(c, length as libc::c_long);
    } else {
        addReplyArrayLen(c, length as libc::c_long);
    }
    hi = hashTypeInitIterator(o);
    while hashTypeNext(hi) != -(1 as libc::c_int) {
        if flags & 1 as libc::c_int != 0 {
            addHashIteratorCursorToReply(c, hi, 1 as libc::c_int);
            count += 1;
        }
        if flags & 2 as libc::c_int != 0 {
            addHashIteratorCursorToReply(c, hi, 2 as libc::c_int);
            count += 1;
        }
    }
    hashTypeReleaseIterator(hi);
    if flags & 1 as libc::c_int != 0 && flags & 2 as libc::c_int != 0 {
        count /= 2 as libc::c_int;
    }
    if count == length {} else {
        _serverAssert(
            b"count == length\0" as *const u8 as *const libc::c_char,
            b"t_hash.c\0" as *const u8 as *const libc::c_char,
            856 as libc::c_int,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn hkeysCommand(mut c: *mut client) {
    genericHgetallCommand(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn hvalsCommand(mut c: *mut client) {
    genericHgetallCommand(c, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn hgetallCommand(mut c: *mut client) {
    genericHgetallCommand(c, 1 as libc::c_int | 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn hexistsCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 4 as libc::c_int) != 0 {
        return;
    }
    addReply(
        c,
        if hashTypeExists(
            o,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        ) != 0
        {
            shared.cone
        } else {
            shared.czero
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn hscanCommand(mut c: *mut client) {
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
    if o.is_null() || checkType(c, o, 4 as libc::c_int) != 0 {
        return;
    }
    scanGenericCommand(c, o, cursor);
}
unsafe extern "C" fn hrandfieldReplyWithListpack(
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
                addReplyBulkCBuffer(
                    c,
                    (*vals.offset(i as isize)).sval as *const libc::c_void,
                    (*vals.offset(i as isize)).slen as size_t,
                );
            } else {
                addReplyBulkLongLong(c, (*vals.offset(i as isize)).lval);
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn hrandfieldWithCountCommand(
    mut c: *mut client,
    mut l: libc::c_long,
    mut withvalues: libc::c_int,
) {
    let mut count: libc::c_ulong = 0;
    let mut size: libc::c_ulong = 0;
    let mut uniq: libc::c_int = 1 as libc::c_int;
    let mut hash: *mut robj = 0 as *mut robj;
    hash = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.emptyarray,
    );
    if hash.is_null() || checkType(c, hash, 4 as libc::c_int) != 0 {
        return;
    }
    size = hashTypeLength(hash);
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
        if withvalues != 0 && (*c).resp == 2 as libc::c_int {
            addReplyArrayLen(
                c,
                count.wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_long,
            );
        } else {
            addReplyArrayLen(c, count as libc::c_long);
        }
        if (*hash).encoding() as libc::c_int == 2 as libc::c_int {
            let mut key: sds = 0 as *mut libc::c_char;
            let mut value: sds = 0 as *mut libc::c_char;
            loop {
                let fresh0 = count;
                count = count.wrapping_sub(1);
                if !(fresh0 != 0) {
                    break;
                }
                let mut de: *mut dictEntry = dictGetFairRandomKey(
                    (*hash).ptr as *mut dict,
                );
                key = (*de).key as sds;
                value = (*de).v.val as sds;
                if withvalues != 0 && (*c).resp > 2 as libc::c_int {
                    addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                }
                addReplyBulkCBuffer(c, key as *const libc::c_void, sdslen(key));
                if withvalues != 0 {
                    addReplyBulkCBuffer(c, value as *const libc::c_void, sdslen(value));
                }
                if (*c).flags
                    & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0
                {
                    break;
                }
            }
        } else if (*hash).encoding() as libc::c_int == 11 as libc::c_int {
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
            if withvalues != 0 {
                vals = zmalloc(
                    (core::mem::size_of::<listpackEntry>() as libc::c_ulong)
                        .wrapping_mul(limit),
                ) as *mut listpackEntry;
            }
            while count != 0 {
                sample_count = if count > limit { limit } else { count };
                count = count.wrapping_sub(sample_count);
                lpRandomPairs(
                    (*hash).ptr as *mut libc::c_uchar,
                    sample_count as libc::c_uint,
                    keys,
                    vals,
                );
                hrandfieldReplyWithListpack(c, sample_count as libc::c_uint, keys, vals);
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
    let mut reply_size: libc::c_long = (if count < size { count } else { size })
        as libc::c_long;
    if withvalues != 0 && (*c).resp == 2 as libc::c_int {
        addReplyArrayLen(c, reply_size * 2 as libc::c_int as libc::c_long);
    } else {
        addReplyArrayLen(c, reply_size);
    }
    if count >= size {
        let mut hi: *mut hashTypeIterator = hashTypeInitIterator(hash);
        while hashTypeNext(hi) != -(1 as libc::c_int) {
            if withvalues != 0 && (*c).resp > 2 as libc::c_int {
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
            }
            addHashIteratorCursorToReply(c, hi, 1 as libc::c_int);
            if withvalues != 0 {
                addHashIteratorCursorToReply(c, hi, 2 as libc::c_int);
            }
        }
        hashTypeReleaseIterator(hi);
        return;
    }
    if count.wrapping_mul(3 as libc::c_int as libc::c_ulong) > size {
        let mut d: *mut dict = dictCreate(&mut sdsReplyDictType);
        dictExpand(d, size);
        let mut hi_0: *mut hashTypeIterator = hashTypeInitIterator(hash);
        while hashTypeNext(hi_0) != -(1 as libc::c_int) {
            let mut ret: libc::c_int = 1 as libc::c_int;
            let mut key_0: sds = 0 as *mut libc::c_char;
            let mut value_0: sds = 0 as sds;
            key_0 = hashTypeCurrentObjectNewSds(hi_0, 1 as libc::c_int);
            if withvalues != 0 {
                value_0 = hashTypeCurrentObjectNewSds(hi_0, 2 as libc::c_int);
            }
            ret = dictAdd(d, key_0 as *mut libc::c_void, value_0 as *mut libc::c_void);
            if ret == 0 as libc::c_int {} else {
                _serverAssert(
                    b"ret == DICT_OK\0" as *const u8 as *const libc::c_char,
                    b"t_hash.c\0" as *const u8 as *const libc::c_char,
                    1031 as libc::c_int,
                );
                unreachable!();
            };
        }
        if ((*d).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*d).ht_used[1 as libc::c_int as usize]) == size
        {} else {
            _serverAssert(
                b"dictSize(d) == size\0" as *const u8 as *const libc::c_char,
                b"t_hash.c\0" as *const u8 as *const libc::c_char,
                1033 as libc::c_int,
            );
            unreachable!();
        };
        hashTypeReleaseIterator(hi_0);
        while size > count {
            let mut de_0: *mut dictEntry = 0 as *mut dictEntry;
            de_0 = dictGetFairRandomKey(d);
            dictUnlink(d, (*de_0).key);
            sdsfree((*de_0).key as sds);
            sdsfree((*de_0).v.val as sds);
            dictFreeUnlinkedEntry(d, de_0);
            size = size.wrapping_sub(1);
        }
        let mut di: *mut dictIterator = 0 as *mut dictIterator;
        let mut de_1: *mut dictEntry = 0 as *mut dictEntry;
        di = dictGetIterator(d);
        loop {
            de_1 = dictNext(di);
            if de_1.is_null() {
                break;
            }
            let mut key_1: sds = (*de_1).key as sds;
            let mut value_1: sds = (*de_1).v.val as sds;
            if withvalues != 0 && (*c).resp > 2 as libc::c_int {
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
            }
            addReplyBulkSds(c, key_1);
            if withvalues != 0 {
                addReplyBulkSds(c, value_1);
            }
        }
        dictReleaseIterator(di);
        dictRelease(d);
    } else {
        if (*hash).encoding() as libc::c_int == 11 as libc::c_int {
            let mut keys_0: *mut listpackEntry = 0 as *mut listpackEntry;
            let mut vals_0: *mut listpackEntry = 0 as *mut listpackEntry;
            keys_0 = zmalloc(
                (core::mem::size_of::<listpackEntry>() as libc::c_ulong)
                    .wrapping_mul(count),
            ) as *mut listpackEntry;
            if withvalues != 0 {
                vals_0 = zmalloc(
                    (core::mem::size_of::<listpackEntry>() as libc::c_ulong)
                        .wrapping_mul(count),
                ) as *mut listpackEntry;
            }
            if lpRandomPairsUnique(
                (*hash).ptr as *mut libc::c_uchar,
                count as libc::c_uint,
                keys_0,
                vals_0,
            ) as libc::c_ulong == count
            {} else {
                _serverAssert(
                    b"lpRandomPairsUnique(hash->ptr, count, keys, vals) == count\0"
                        as *const u8 as *const libc::c_char,
                    b"t_hash.c\0" as *const u8 as *const libc::c_char,
                    1077 as libc::c_int,
                );
                unreachable!();
            };
            hrandfieldReplyWithListpack(c, count as libc::c_uint, keys_0, vals_0);
            zfree(keys_0 as *mut libc::c_void);
            zfree(vals_0 as *mut libc::c_void);
            return;
        }
        let mut added: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut key_2: listpackEntry = listpackEntry {
            sval: 0 as *mut libc::c_uchar,
            slen: 0,
            lval: 0,
        };
        let mut value_2: listpackEntry = listpackEntry {
            sval: 0 as *mut libc::c_uchar,
            slen: 0,
            lval: 0,
        };
        let mut d_0: *mut dict = dictCreate(&mut hashDictType);
        dictExpand(d_0, count);
        while added < count {
            hashTypeRandomElement(
                hash,
                size,
                &mut key_2,
                if withvalues != 0 { &mut value_2 } else { 0 as *mut listpackEntry },
            );
            let mut skey: sds = hashSdsFromListpackEntry(&mut key_2);
            if dictAdd(d_0, skey as *mut libc::c_void, 0 as *mut libc::c_void)
                != 0 as libc::c_int
            {
                sdsfree(skey);
            } else {
                added = added.wrapping_add(1);
                if withvalues != 0 && (*c).resp > 2 as libc::c_int {
                    addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                }
                hashReplyFromListpackEntry(c, &mut key_2);
                if withvalues != 0 {
                    hashReplyFromListpackEntry(c, &mut value_2);
                }
            }
        }
        dictRelease(d_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn hrandfieldCommand(mut c: *mut client) {
    let mut l: libc::c_long = 0;
    let mut withvalues: libc::c_int = 0 as libc::c_int;
    let mut hash: *mut robj = 0 as *mut robj;
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
                    b"withvalues\0" as *const u8 as *const libc::c_char,
                ) != 0
        {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        } else {
            if (*c).argc == 4 as libc::c_int {
                withvalues = 1 as libc::c_int;
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
        hrandfieldWithCountCommand(c, l, withvalues);
        return;
    }
    hash = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.null[(*c).resp as usize],
    );
    if hash.is_null() || checkType(c, hash, 4 as libc::c_int) != 0 {
        return;
    }
    hashTypeRandomElement(hash, hashTypeLength(hash), &mut ele, 0 as *mut listpackEntry);
    hashReplyFromListpackEntry(c, &mut ele);
}
