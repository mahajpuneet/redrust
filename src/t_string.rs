extern crate c2rust_bitfields;
extern crate libc;
extern crate core;

extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn mstime() -> libc::c_longlong;
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn addReplyNull(c: *mut client);
    fn addReplyBulk(c: *mut client, obj: *mut robj);
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    static mut SDS_NOINIT: *const libc::c_char;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsfree(s: sds);
    fn sdsgrowzero(s: sds, len: size_t) -> sds;
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn ztrymalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyErrorArity(c: *mut client);
    fn addReplyErrorExpireTime(c: *mut client);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn rewriteClientCommandVector(c: *mut client, argc: libc::c_int, _: ...);
    fn rewriteClientCommandArgument(c: *mut client, i: libc::c_int, newval: *mut robj);
    fn replaceClientCommandVector(
        c: *mut client,
        argc: libc::c_int,
        argv: *mut *mut robj,
    );
    fn decrRefCount(o: *mut robj);
    fn incrRefCount(o: *mut robj);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn tryObjectEncoding(o: *mut robj) -> *mut robj;
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn stringObjectLen(o: *mut robj) -> size_t;
    fn createStringObjectFromLongLong(value: libc::c_longlong) -> *mut robj;
    fn createStringObjectFromLongLongForValue(value: libc::c_longlong) -> *mut robj;
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
    fn mustObeyClient(c: *mut client) -> libc::c_int;
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn removeExpire(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn setExpire(
        c: *mut client,
        db: *mut redisDb,
        key: *mut robj,
        when: libc::c_longlong,
    );
    fn checkAlreadyExpired(when: libc::c_longlong) -> libc::c_int;
    fn lookupKeyRead(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyWrite(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyReadOrReply(
        c: *mut client,
        key: *mut robj,
        reply: *mut robj,
    ) -> *mut robj;
    fn dbAdd(db: *mut redisDb, key: *mut robj, val: *mut robj);
    fn dbOverwrite(db: *mut redisDb, key: *mut robj, val: *mut robj);
    fn setKey(
        c: *mut client,
        db: *mut redisDb,
        key: *mut robj,
        val: *mut robj,
        flags: libc::c_int,
    );
    fn dbSyncDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn dbUnshareStringValue(db: *mut redisDb, key: *mut robj, o: *mut robj) -> *mut robj;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn dbAsyncDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
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
unsafe extern "C" fn checkStringLength(
    mut c: *mut client,
    mut size: libc::c_longlong,
    mut append: libc::c_longlong,
) -> libc::c_int {
    if mustObeyClient(c) != 0 {
        return 0 as libc::c_int;
    }
    let mut total: libc::c_longlong = (size as uint64_t as libc::c_ulonglong)
        .wrapping_add(append as libc::c_ulonglong) as libc::c_longlong;
    if total > server.proto_max_bulk_len || total < size || total < append {
        addReplyError(
            c,
            b"string exceeds maximum allowed size (proto-max-bulk-len)\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setGenericCommand(
    mut c: *mut client,
    mut flags: libc::c_int,
    mut key: *mut robj,
    mut val: *mut robj,
    mut expire: *mut robj,
    mut unit: libc::c_int,
    mut ok_reply: *mut robj,
    mut abort_reply: *mut robj,
) {
    let mut milliseconds: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut setkey_flags: libc::c_int = 0 as libc::c_int;
    if !expire.is_null()
        && getExpireMillisecondsOrReply(c, expire, flags, unit, &mut milliseconds)
            != 0 as libc::c_int
    {
        return;
    }
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        if getGenericCommand(c) == -(1 as libc::c_int) {
            return;
        }
    }
    found = (lookupKeyWrite((*c).db, key) != 0 as *mut libc::c_void as *mut robj)
        as libc::c_int;
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 && found != 0
        || flags & (1 as libc::c_int) << 1 as libc::c_int != 0 && found == 0
    {
        if flags & (1 as libc::c_int) << 5 as libc::c_int == 0 {
            addReply(
                c,
                if !abort_reply.is_null() {
                    abort_reply
                } else {
                    shared.null[(*c).resp as usize]
                },
            );
        }
        return;
    }
    setkey_flags
        |= if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    setkey_flags |= if found != 0 { 4 as libc::c_int } else { 8 as libc::c_int };
    setKey(c, (*c).db, key, val, setkey_flags);
    server.dirty += 1;
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 3 as libc::c_int,
        b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        key,
        (*(*c).db).id,
    );
    if !expire.is_null() {
        setExpire(c, (*c).db, key, milliseconds);
        let mut milliseconds_obj: *mut robj = createStringObjectFromLongLong(
            milliseconds,
        );
        rewriteClientCommandVector(
            c,
            5 as libc::c_int,
            shared.set,
            key,
            val,
            shared.pxat,
            milliseconds_obj,
        );
        decrRefCount(milliseconds_obj);
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 2 as libc::c_int,
            b"expire\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key,
            (*(*c).db).id,
        );
    }
    if flags & (1 as libc::c_int) << 5 as libc::c_int == 0 {
        addReply(c, if !ok_reply.is_null() { ok_reply } else { shared.ok });
    }
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 && expire.is_null() {
        let mut argc: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0;
        let mut argv: *mut *mut robj = zmalloc(
            (((*c).argc - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(core::mem::size_of::<*mut robj>() as libc::c_ulong),
        ) as *mut *mut robj;
        j = 0 as libc::c_int;
        while j < (*c).argc {
            let mut a: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
                as *mut libc::c_char;
            if !(j >= 3 as libc::c_int
                && (*a.offset(0 as libc::c_int as isize) as libc::c_int == 'g' as i32
                    || *a.offset(0 as libc::c_int as isize) as libc::c_int == 'G' as i32)
                && (*a.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32
                    || *a.offset(1 as libc::c_int as isize) as libc::c_int == 'E' as i32)
                && (*a.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32
                    || *a.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32)
                && *a.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32)
            {
                let fresh0 = argc;
                argc = argc + 1;
                let ref mut fresh1 = *argv.offset(fresh0 as isize);
                *fresh1 = *((*c).argv).offset(j as isize);
                incrRefCount(*((*c).argv).offset(j as isize));
            }
            j += 1;
        }
        replaceClientCommandVector(c, argc, argv);
    }
}
unsafe extern "C" fn getExpireMillisecondsOrReply(
    mut c: *mut client,
    mut expire: *mut robj,
    mut flags: libc::c_int,
    mut unit: libc::c_int,
    mut milliseconds: *mut libc::c_longlong,
) -> libc::c_int {
    let mut ret: libc::c_int = getLongLongFromObjectOrReply(
        c,
        expire,
        milliseconds,
        0 as *const libc::c_char,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    if *milliseconds <= 0 as libc::c_int as libc::c_longlong
        || unit == 0 as libc::c_int
            && *milliseconds
                > 9223372036854775807 as libc::c_longlong
                    / 1000 as libc::c_int as libc::c_longlong
    {
        addReplyErrorExpireTime(c);
        return -(1 as libc::c_int);
    }
    if unit == 0 as libc::c_int {
        *milliseconds *= 1000 as libc::c_int as libc::c_longlong;
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0
        || flags & (1 as libc::c_int) << 2 as libc::c_int != 0
    {
        *milliseconds += mstime();
    }
    if *milliseconds <= 0 as libc::c_int as libc::c_longlong {
        addReplyErrorExpireTime(c);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parseExtendedStringArgumentsOrReply(
    mut c: *mut client,
    mut flags: *mut libc::c_int,
    mut unit: *mut libc::c_int,
    mut expire: *mut *mut robj,
    mut command_type: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = if command_type == 0 as libc::c_int {
        2 as libc::c_int
    } else {
        3 as libc::c_int
    };
    while j < (*c).argc {
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
            as *mut libc::c_char;
        let mut next: *mut robj = if j == (*c).argc - 1 as libc::c_int {
            0 as *mut robj
        } else {
            *((*c).argv).offset((j + 1 as libc::c_int) as isize)
        };
        if (*opt.offset(0 as libc::c_int as isize) as libc::c_int == 'n' as i32
            || *opt.offset(0 as libc::c_int as isize) as libc::c_int == 'N' as i32)
            && (*opt.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *opt.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
            && *opt.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && *flags & (1 as libc::c_int) << 1 as libc::c_int == 0
            && command_type == 1 as libc::c_int
        {
            *flags |= (1 as libc::c_int) << 0 as libc::c_int;
        } else if (*opt.offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *opt.offset(0 as libc::c_int as isize) as libc::c_int == 'X' as i32)
            && (*opt.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *opt.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
            && *opt.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && *flags & (1 as libc::c_int) << 0 as libc::c_int == 0
            && command_type == 1 as libc::c_int
        {
            *flags |= (1 as libc::c_int) << 1 as libc::c_int;
        } else if (*opt.offset(0 as libc::c_int as isize) as libc::c_int == 'g' as i32
            || *opt.offset(0 as libc::c_int as isize) as libc::c_int == 'G' as i32)
            && (*opt.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32
                || *opt.offset(1 as libc::c_int as isize) as libc::c_int == 'E' as i32)
            && (*opt.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32
                || *opt.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32)
            && *opt.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && command_type == 1 as libc::c_int
        {
            *flags |= (1 as libc::c_int) << 5 as libc::c_int;
        } else if strcasecmp(opt, b"KEEPTTL\0" as *const u8 as *const libc::c_char) == 0
            && *flags & (1 as libc::c_int) << 8 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 2 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 6 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 3 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 7 as libc::c_int == 0
            && command_type == 1 as libc::c_int
        {
            *flags |= (1 as libc::c_int) << 4 as libc::c_int;
        } else if strcasecmp(opt, b"PERSIST\0" as *const u8 as *const libc::c_char) == 0
            && command_type == 0 as libc::c_int
            && *flags & (1 as libc::c_int) << 2 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 6 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 3 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 7 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 4 as libc::c_int == 0
        {
            *flags |= (1 as libc::c_int) << 8 as libc::c_int;
        } else if (*opt.offset(0 as libc::c_int as isize) as libc::c_int == 'e' as i32
            || *opt.offset(0 as libc::c_int as isize) as libc::c_int == 'E' as i32)
            && (*opt.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *opt.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
            && *opt.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && *flags & (1 as libc::c_int) << 4 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 8 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 6 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 3 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 7 as libc::c_int == 0 && !next.is_null()
        {
            *flags |= (1 as libc::c_int) << 2 as libc::c_int;
            *expire = next;
            j += 1;
        } else if (*opt.offset(0 as libc::c_int as isize) as libc::c_int == 'p' as i32
            || *opt.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32)
            && (*opt.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *opt.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
            && *opt.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && *flags & (1 as libc::c_int) << 4 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 8 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 2 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 6 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 7 as libc::c_int == 0 && !next.is_null()
        {
            *flags |= (1 as libc::c_int) << 3 as libc::c_int;
            *unit = 1 as libc::c_int;
            *expire = next;
            j += 1;
        } else if (*opt.offset(0 as libc::c_int as isize) as libc::c_int == 'e' as i32
            || *opt.offset(0 as libc::c_int as isize) as libc::c_int == 'E' as i32)
            && (*opt.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *opt.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
            && (*opt.offset(2 as libc::c_int as isize) as libc::c_int == 'a' as i32
                || *opt.offset(2 as libc::c_int as isize) as libc::c_int == 'A' as i32)
            && (*opt.offset(3 as libc::c_int as isize) as libc::c_int == 't' as i32
                || *opt.offset(3 as libc::c_int as isize) as libc::c_int == 'T' as i32)
            && *opt.offset(4 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && *flags & (1 as libc::c_int) << 4 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 8 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 2 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 3 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 7 as libc::c_int == 0 && !next.is_null()
        {
            *flags |= (1 as libc::c_int) << 6 as libc::c_int;
            *expire = next;
            j += 1;
        } else if (*opt.offset(0 as libc::c_int as isize) as libc::c_int == 'p' as i32
            || *opt.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32)
            && (*opt.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *opt.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
            && (*opt.offset(2 as libc::c_int as isize) as libc::c_int == 'a' as i32
                || *opt.offset(2 as libc::c_int as isize) as libc::c_int == 'A' as i32)
            && (*opt.offset(3 as libc::c_int as isize) as libc::c_int == 't' as i32
                || *opt.offset(3 as libc::c_int as isize) as libc::c_int == 'T' as i32)
            && *opt.offset(4 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && *flags & (1 as libc::c_int) << 4 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 8 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 2 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 6 as libc::c_int == 0
            && *flags & (1 as libc::c_int) << 3 as libc::c_int == 0 && !next.is_null()
        {
            *flags |= (1 as libc::c_int) << 7 as libc::c_int;
            *unit = 1 as libc::c_int;
            *expire = next;
            j += 1;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return -(1 as libc::c_int);
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setCommand(mut c: *mut client) {
    let mut expire: *mut robj = 0 as *mut robj;
    let mut unit: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if parseExtendedStringArgumentsOrReply(
        c,
        &mut flags,
        &mut unit,
        &mut expire,
        1 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return;
    }
    let ref mut fresh2 = *((*c).argv).offset(2 as libc::c_int as isize);
    *fresh2 = tryObjectEncoding(*((*c).argv).offset(2 as libc::c_int as isize));
    setGenericCommand(
        c,
        flags,
        *((*c).argv).offset(1 as libc::c_int as isize),
        *((*c).argv).offset(2 as libc::c_int as isize),
        expire,
        unit,
        0 as *mut robj,
        0 as *mut robj,
    );
}
#[no_mangle]
pub unsafe extern "C" fn setnxCommand(mut c: *mut client) {
    let ref mut fresh3 = *((*c).argv).offset(2 as libc::c_int as isize);
    *fresh3 = tryObjectEncoding(*((*c).argv).offset(2 as libc::c_int as isize));
    setGenericCommand(
        c,
        (1 as libc::c_int) << 0 as libc::c_int,
        *((*c).argv).offset(1 as libc::c_int as isize),
        *((*c).argv).offset(2 as libc::c_int as isize),
        0 as *mut robj,
        0 as libc::c_int,
        shared.cone,
        shared.czero,
    );
}
#[no_mangle]
pub unsafe extern "C" fn setexCommand(mut c: *mut client) {
    let ref mut fresh4 = *((*c).argv).offset(3 as libc::c_int as isize);
    *fresh4 = tryObjectEncoding(*((*c).argv).offset(3 as libc::c_int as isize));
    setGenericCommand(
        c,
        (1 as libc::c_int) << 2 as libc::c_int,
        *((*c).argv).offset(1 as libc::c_int as isize),
        *((*c).argv).offset(3 as libc::c_int as isize),
        *((*c).argv).offset(2 as libc::c_int as isize),
        0 as libc::c_int,
        0 as *mut robj,
        0 as *mut robj,
    );
}
#[no_mangle]
pub unsafe extern "C" fn psetexCommand(mut c: *mut client) {
    let ref mut fresh5 = *((*c).argv).offset(3 as libc::c_int as isize);
    *fresh5 = tryObjectEncoding(*((*c).argv).offset(3 as libc::c_int as isize));
    setGenericCommand(
        c,
        (1 as libc::c_int) << 3 as libc::c_int,
        *((*c).argv).offset(1 as libc::c_int as isize),
        *((*c).argv).offset(3 as libc::c_int as isize),
        *((*c).argv).offset(2 as libc::c_int as isize),
        1 as libc::c_int,
        0 as *mut robj,
        0 as *mut robj,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getGenericCommand(mut c: *mut client) -> libc::c_int {
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.null[(*c).resp as usize],
    );
    if o.is_null() {
        return 0 as libc::c_int;
    }
    if checkType(c, o, 0 as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    addReplyBulk(c, o);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getCommand(mut c: *mut client) {
    getGenericCommand(c);
}
#[no_mangle]
pub unsafe extern "C" fn getexCommand(mut c: *mut client) {
    let mut expire: *mut robj = 0 as *mut robj;
    let mut unit: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if parseExtendedStringArgumentsOrReply(
        c,
        &mut flags,
        &mut unit,
        &mut expire,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return;
    }
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.null[(*c).resp as usize],
    );
    if o.is_null() {
        return;
    }
    if checkType(c, o, 0 as libc::c_int) != 0 {
        return;
    }
    let mut milliseconds: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if !expire.is_null()
        && getExpireMillisecondsOrReply(c, expire, flags, unit, &mut milliseconds)
            != 0 as libc::c_int
    {
        return;
    }
    addReplyBulk(c, o);
    if (flags & (1 as libc::c_int) << 7 as libc::c_int != 0
        || flags & (1 as libc::c_int) << 6 as libc::c_int != 0)
        && checkAlreadyExpired(milliseconds) != 0
    {
        let mut deleted: libc::c_int = if server.lazyfree_lazy_expire != 0 {
            dbAsyncDelete((*c).db, *((*c).argv).offset(1 as libc::c_int as isize))
        } else {
            dbSyncDelete((*c).db, *((*c).argv).offset(1 as libc::c_int as isize))
        };
        if deleted != 0 {} else {
            _serverAssert(
                b"deleted\0" as *const u8 as *const libc::c_char,
                b"t_string.c\0" as *const u8 as *const libc::c_char,
                392 as libc::c_int,
            );
            unreachable!();
        };
        let mut aux: *mut robj = if server.lazyfree_lazy_expire != 0 {
            shared.unlink
        } else {
            shared.del
        };
        rewriteClientCommandVector(
            c,
            2 as libc::c_int,
            aux,
            *((*c).argv).offset(1 as libc::c_int as isize),
        );
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 2 as libc::c_int,
            b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        server.dirty += 1;
    } else if !expire.is_null() {
        setExpire(
            c,
            (*c).db,
            *((*c).argv).offset(1 as libc::c_int as isize),
            milliseconds,
        );
        let mut milliseconds_obj: *mut robj = createStringObjectFromLongLong(
            milliseconds,
        );
        rewriteClientCommandVector(
            c,
            3 as libc::c_int,
            shared.pexpireat,
            *((*c).argv).offset(1 as libc::c_int as isize),
            milliseconds_obj,
        );
        decrRefCount(milliseconds_obj);
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 2 as libc::c_int,
            b"expire\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        server.dirty += 1;
    } else if flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        if removeExpire((*c).db, *((*c).argv).offset(1 as libc::c_int as isize)) != 0 {
            signalModifiedKey(
                c,
                (*c).db,
                *((*c).argv).offset(1 as libc::c_int as isize),
            );
            rewriteClientCommandVector(
                c,
                2 as libc::c_int,
                shared.persist,
                *((*c).argv).offset(1 as libc::c_int as isize),
            );
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 2 as libc::c_int,
                b"persist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                *((*c).argv).offset(1 as libc::c_int as isize),
                (*(*c).db).id,
            );
            server.dirty += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn getdelCommand(mut c: *mut client) {
    if getGenericCommand(c) == -(1 as libc::c_int) {
        return;
    }
    let mut deleted: libc::c_int = if server.lazyfree_lazy_user_del != 0 {
        dbAsyncDelete((*c).db, *((*c).argv).offset(1 as libc::c_int as isize))
    } else {
        dbSyncDelete((*c).db, *((*c).argv).offset(1 as libc::c_int as isize))
    };
    if deleted != 0 {
        let mut aux: *mut robj = if server.lazyfree_lazy_user_del != 0 {
            shared.unlink
        } else {
            shared.del
        };
        rewriteClientCommandVector(
            c,
            2 as libc::c_int,
            aux,
            *((*c).argv).offset(1 as libc::c_int as isize),
        );
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 2 as libc::c_int,
            b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        server.dirty += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getsetCommand(mut c: *mut client) {
    if getGenericCommand(c) == -(1 as libc::c_int) {
        return;
    }
    let ref mut fresh6 = *((*c).argv).offset(2 as libc::c_int as isize);
    *fresh6 = tryObjectEncoding(*((*c).argv).offset(2 as libc::c_int as isize));
    setKey(
        c,
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
        *((*c).argv).offset(2 as libc::c_int as isize),
        0 as libc::c_int,
    );
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 3 as libc::c_int,
        b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
    rewriteClientCommandArgument(c, 0 as libc::c_int, shared.set);
}
#[no_mangle]
pub unsafe extern "C" fn setrangeCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut offset: libc::c_long = 0;
    let mut value: sds = (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds;
    if getLongFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut offset,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if offset < 0 as libc::c_int as libc::c_long {
        addReplyError(
            c,
            b"offset is out of range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    o = lookupKeyWrite((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        if sdslen(value) == 0 as libc::c_int as libc::c_ulong {
            addReply(c, shared.czero);
            return;
        }
        if checkStringLength(
            c,
            offset as libc::c_longlong,
            sdslen(value) as libc::c_longlong,
        ) != 0 as libc::c_int
        {
            return;
        }
        o = createObject(
            0 as libc::c_int,
            sdsnewlen(
                0 as *const libc::c_void,
                (offset as libc::c_ulong).wrapping_add(sdslen(value)),
            ) as *mut libc::c_void,
        );
        dbAdd((*c).db, *((*c).argv).offset(1 as libc::c_int as isize), o);
    } else {
        let mut olen: size_t = 0;
        if checkType(c, o, 0 as libc::c_int) != 0 {
            return;
        }
        olen = stringObjectLen(o);
        if sdslen(value) == 0 as libc::c_int as libc::c_ulong {
            addReplyLongLong(c, olen as libc::c_longlong);
            return;
        }
        if checkStringLength(
            c,
            offset as libc::c_longlong,
            sdslen(value) as libc::c_longlong,
        ) != 0 as libc::c_int
        {
            return;
        }
        o = dbUnshareStringValue(
            (*c).db,
            *((*c).argv).offset(1 as libc::c_int as isize),
            o,
        );
    }
    if sdslen(value) > 0 as libc::c_int as libc::c_ulong {
        (*o)
            .ptr = sdsgrowzero(
            (*o).ptr as sds,
            (offset as libc::c_ulong).wrapping_add(sdslen(value)),
        ) as *mut libc::c_void;
        memcpy(
            ((*o).ptr as *mut libc::c_char).offset(offset as isize) as *mut libc::c_void,
            value as *const libc::c_void,
            sdslen(value),
        );
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 3 as libc::c_int,
            b"setrange\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        server.dirty += 1;
    }
    addReplyLongLong(c, sdslen((*o).ptr as sds) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn getrangeCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut start: libc::c_longlong = 0;
    let mut end: libc::c_longlong = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut llbuf: [libc::c_char; 32] = [0; 32];
    let mut strlen: size_t = 0;
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut start,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(3 as libc::c_int as isize),
        &mut end,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.emptybulk,
    );
    if o.is_null() || checkType(c, o, 0 as libc::c_int) != 0 {
        return;
    }
    if (*o).encoding() as libc::c_int == 1 as libc::c_int {
        str = llbuf.as_mut_ptr();
        strlen = ll2string(
            llbuf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            (*o).ptr as libc::c_long as libc::c_longlong,
        ) as size_t;
    } else {
        str = (*o).ptr as *mut libc::c_char;
        strlen = sdslen(str);
    }
    if start < 0 as libc::c_int as libc::c_longlong
        && end < 0 as libc::c_int as libc::c_longlong && start > end
    {
        addReply(c, shared.emptybulk);
        return;
    }
    if start < 0 as libc::c_int as libc::c_longlong {
        start = (strlen as libc::c_ulonglong).wrapping_add(start as libc::c_ulonglong)
            as libc::c_longlong;
    }
    if end < 0 as libc::c_int as libc::c_longlong {
        end = (strlen as libc::c_ulonglong).wrapping_add(end as libc::c_ulonglong)
            as libc::c_longlong;
    }
    if start < 0 as libc::c_int as libc::c_longlong {
        start = 0 as libc::c_int as libc::c_longlong;
    }
    if end < 0 as libc::c_int as libc::c_longlong {
        end = 0 as libc::c_int as libc::c_longlong;
    }
    if end as libc::c_ulonglong >= strlen as libc::c_ulonglong {
        end = strlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_longlong;
    }
    if start > end || strlen == 0 as libc::c_int as libc::c_ulong {
        addReply(c, shared.emptybulk);
    } else {
        addReplyBulkCBuffer(
            c,
            str.offset(start as isize) as *const libc::c_void,
            (end - start + 1 as libc::c_int as libc::c_longlong) as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn mgetCommand(mut c: *mut client) {
    let mut j: libc::c_int = 0;
    addReplyArrayLen(c, ((*c).argc - 1 as libc::c_int) as libc::c_long);
    j = 1 as libc::c_int;
    while j < (*c).argc {
        let mut o: *mut robj = lookupKeyRead((*c).db, *((*c).argv).offset(j as isize));
        if o.is_null() {
            addReplyNull(c);
        } else if (*o).type_0() as libc::c_int != 0 as libc::c_int {
            addReplyNull(c);
        } else {
            addReplyBulk(c, o);
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn msetGenericCommand(mut c: *mut client, mut nx: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut setkey_flags: libc::c_int = 0 as libc::c_int;
    if (*c).argc % 2 as libc::c_int == 0 as libc::c_int {
        addReplyErrorArity(c);
        return;
    }
    if nx != 0 {
        j = 1 as libc::c_int;
        while j < (*c).argc {
            if !(lookupKeyWrite((*c).db, *((*c).argv).offset(j as isize))).is_null() {
                addReply(c, shared.czero);
                return;
            }
            j += 2 as libc::c_int;
        }
        setkey_flags |= 8 as libc::c_int;
    }
    j = 1 as libc::c_int;
    while j < (*c).argc {
        let ref mut fresh7 = *((*c).argv).offset((j + 1 as libc::c_int) as isize);
        *fresh7 = tryObjectEncoding(
            *((*c).argv).offset((j + 1 as libc::c_int) as isize),
        );
        setKey(
            c,
            (*c).db,
            *((*c).argv).offset(j as isize),
            *((*c).argv).offset((j + 1 as libc::c_int) as isize),
            setkey_flags,
        );
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 3 as libc::c_int,
            b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(j as isize),
            (*(*c).db).id,
        );
        j += 2 as libc::c_int;
    }
    server.dirty
        += (((*c).argc - 1 as libc::c_int) / 2 as libc::c_int) as libc::c_longlong;
    addReply(c, if nx != 0 { shared.cone } else { shared.ok });
}
#[no_mangle]
pub unsafe extern "C" fn msetCommand(mut c: *mut client) {
    msetGenericCommand(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn msetnxCommand(mut c: *mut client) {
    msetGenericCommand(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn incrDecrCommand(
    mut c: *mut client,
    mut incr: libc::c_longlong,
) {
    let mut value: libc::c_longlong = 0;
    let mut oldvalue: libc::c_longlong = 0;
    let mut o: *mut robj = 0 as *mut robj;
    let mut new: *mut robj = 0 as *mut robj;
    o = lookupKeyWrite((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if checkType(c, o, 0 as libc::c_int) != 0 {
        return;
    }
    if getLongLongFromObjectOrReply(c, o, &mut value, 0 as *const libc::c_char)
        != 0 as libc::c_int
    {
        return;
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
    if !o.is_null() && (*o).refcount == 1 as libc::c_int
        && (*o).encoding() as libc::c_int == 1 as libc::c_int
        && (value < 0 as libc::c_int as libc::c_longlong
            || value >= 10000 as libc::c_int as libc::c_longlong)
        && value
            >= (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                as libc::c_longlong
        && value <= 9223372036854775807 as libc::c_long as libc::c_longlong
    {
        new = o;
        (*o).ptr = value as libc::c_long as *mut libc::c_void;
    } else {
        new = createStringObjectFromLongLongForValue(value);
        if !o.is_null() {
            dbOverwrite((*c).db, *((*c).argv).offset(1 as libc::c_int as isize), new);
        } else {
            dbAdd((*c).db, *((*c).argv).offset(1 as libc::c_int as isize), new);
        }
    }
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 3 as libc::c_int,
        b"incrby\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
    addReplyLongLong(c, value);
}
#[no_mangle]
pub unsafe extern "C" fn incrCommand(mut c: *mut client) {
    incrDecrCommand(c, 1 as libc::c_int as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn decrCommand(mut c: *mut client) {
    incrDecrCommand(c, -(1 as libc::c_int) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn incrbyCommand(mut c: *mut client) {
    let mut incr: libc::c_longlong = 0;
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut incr,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    incrDecrCommand(c, incr);
}
#[no_mangle]
pub unsafe extern "C" fn decrbyCommand(mut c: *mut client) {
    let mut incr: libc::c_longlong = 0;
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut incr,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if incr == -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong {
        addReplyError(
            c,
            b"decrement would overflow\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    incrDecrCommand(c, -incr);
}
#[no_mangle]
pub unsafe extern "C" fn incrbyfloatCommand(mut c: *mut client) {
    let mut incr: f64 = 0 as f64;
    let mut value: f64 = 0 as f64;
    let mut o: *mut robj = 0 as *mut robj;
    let mut new: *mut robj = 0 as *mut robj;
    o = lookupKeyWrite((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if checkType(c, o, 0 as libc::c_int) != 0 {
        return;
    }
    if getLongDoubleFromObjectOrReply(c, o, &mut value, 0 as *const libc::c_char)
        != 0 as libc::c_int
        || getLongDoubleFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut incr,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        return;
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
    new = createStringObjectFromLongDouble(value, 1 as libc::c_int);
    if !o.is_null() {
        dbOverwrite((*c).db, *((*c).argv).offset(1 as libc::c_int as isize), new);
    } else {
        dbAdd((*c).db, *((*c).argv).offset(1 as libc::c_int as isize), new);
    }
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 3 as libc::c_int,
        b"incrbyfloat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
    addReplyBulk(c, new);
    rewriteClientCommandArgument(c, 0 as libc::c_int, shared.set);
    rewriteClientCommandArgument(c, 2 as libc::c_int, new);
    rewriteClientCommandArgument(c, 3 as libc::c_int, shared.keepttl);
}
#[no_mangle]
pub unsafe extern "C" fn appendCommand(mut c: *mut client) {
    let mut totlen: size_t = 0;
    let mut o: *mut robj = 0 as *mut robj;
    let mut append: *mut robj = 0 as *mut robj;
    o = lookupKeyWrite((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        let ref mut fresh8 = *((*c).argv).offset(2 as libc::c_int as isize);
        *fresh8 = tryObjectEncoding(*((*c).argv).offset(2 as libc::c_int as isize));
        dbAdd(
            (*c).db,
            *((*c).argv).offset(1 as libc::c_int as isize),
            *((*c).argv).offset(2 as libc::c_int as isize),
        );
        incrRefCount(*((*c).argv).offset(2 as libc::c_int as isize));
        totlen = stringObjectLen(*((*c).argv).offset(2 as libc::c_int as isize));
    } else {
        if checkType(c, o, 0 as libc::c_int) != 0 {
            return;
        }
        append = *((*c).argv).offset(2 as libc::c_int as isize);
        if checkStringLength(
            c,
            stringObjectLen(o) as libc::c_longlong,
            sdslen((*append).ptr as sds) as libc::c_longlong,
        ) != 0 as libc::c_int
        {
            return;
        }
        o = dbUnshareStringValue(
            (*c).db,
            *((*c).argv).offset(1 as libc::c_int as isize),
            o,
        );
        (*o)
            .ptr = sdscatlen(
            (*o).ptr as sds,
            (*append).ptr,
            sdslen((*append).ptr as sds),
        ) as *mut libc::c_void;
        totlen = sdslen((*o).ptr as sds);
    }
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 3 as libc::c_int,
        b"append\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
    addReplyLongLong(c, totlen as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn strlenCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 0 as libc::c_int) != 0 {
        return;
    }
    addReplyLongLong(c, stringObjectLen(o) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn lcsCommand(mut c: *mut client) {
    let mut alen: uint32_t = 0;
    let mut blen: uint32_t = 0;
    let mut lcssize: libc::c_ulonglong = 0;
    let mut lcsalloc: libc::c_ulonglong = 0;
    let mut lcs: *mut uint32_t = 0 as *mut uint32_t;
    let mut idx: uint32_t = 0;
    let mut result: sds = 0 as *mut libc::c_char;
    let mut arraylenptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut arange_start: uint32_t = 0;
    let mut arange_end: uint32_t = 0;
    let mut brange_start: uint32_t = 0;
    let mut brange_end: uint32_t = 0;
    let mut computelcs: libc::c_int = 0;
    let mut arraylen: uint32_t = 0;
    let mut current_block: u64;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut minmatchlen: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut a: sds = 0 as sds;
    let mut b: sds = 0 as sds;
    let mut getlen: libc::c_int = 0 as libc::c_int;
    let mut getidx: libc::c_int = 0 as libc::c_int;
    let mut withmatchlen: libc::c_int = 0 as libc::c_int;
    let mut obja: *mut robj = 0 as *mut robj;
    let mut objb: *mut robj = 0 as *mut robj;
    obja = lookupKeyRead((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    objb = lookupKeyRead((*c).db, *((*c).argv).offset(2 as libc::c_int as isize));
    if !obja.is_null() && (*obja).type_0() as libc::c_int != 0 as libc::c_int
        || !objb.is_null() && (*objb).type_0() as libc::c_int != 0 as libc::c_int
    {
        addReplyError(
            c,
            b"The specified keys must contain string values\0" as *const u8
                as *const libc::c_char,
        );
        obja = 0 as *mut robj;
        objb = 0 as *mut robj;
    } else {
        obja = if !obja.is_null() {
            getDecodedObject(obja)
        } else {
            createStringObject(
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
            )
        };
        objb = if !objb.is_null() {
            getDecodedObject(objb)
        } else {
            createStringObject(
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
            )
        };
        a = (*obja).ptr as sds;
        b = (*objb).ptr as sds;
        j = 3 as libc::c_int as uint32_t;
        loop {
            if !(j < (*c).argc as uint32_t) {
                current_block = 9828876828309294594;
                break;
            }
            let mut opt: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
                as *mut libc::c_char;
            let mut moreargs: libc::c_int = (((*c).argc - 1 as libc::c_int)
                as libc::c_uint)
                .wrapping_sub(j) as libc::c_int;
            if strcasecmp(opt, b"IDX\0" as *const u8 as *const libc::c_char) == 0 {
                getidx = 1 as libc::c_int;
            } else if strcasecmp(opt, b"LEN\0" as *const u8 as *const libc::c_char) == 0
            {
                getlen = 1 as libc::c_int;
            } else if strcasecmp(
                opt,
                b"WITHMATCHLEN\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                withmatchlen = 1 as libc::c_int;
            } else if strcasecmp(
                opt,
                b"MINMATCHLEN\0" as *const u8 as *const libc::c_char,
            ) == 0 && moreargs != 0
            {
                if getLongLongFromObjectOrReply(
                    c,
                    *((*c).argv)
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ),
                    &mut minmatchlen,
                    0 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    current_block = 5486260907809189108;
                    break;
                }
                if minmatchlen < 0 as libc::c_int as libc::c_longlong {
                    minmatchlen = 0 as libc::c_int as libc::c_longlong;
                }
                j = j.wrapping_add(1);
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                current_block = 5486260907809189108;
                break;
            }
            j = j.wrapping_add(1);
        }
        match current_block {
            5486260907809189108 => {}
            _ => {
                if getlen != 0 && getidx != 0 {
                    addReplyError(
                        c,
                        b"If you want both the length and indexes, please just use IDX.\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else if sdslen(a)
                    >= (4294967295 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
                    || sdslen(b)
                        >= (4294967295 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong
                {
                    addReplyError(
                        c,
                        b"String too long for LCS\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    alen = sdslen(a) as uint32_t;
                    blen = sdslen(b) as uint32_t;
                    lcssize = (alen.wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulonglong)
                        .wrapping_mul(
                            blen.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as libc::c_ulonglong,
                        );
                    lcsalloc = lcssize
                        .wrapping_mul(
                            core::mem::size_of::<uint32_t>() as libc::c_ulong
                                as libc::c_ulonglong,
                        );
                    lcs = 0 as *mut uint32_t;
                    if lcsalloc
                        < 18446744073709551615 as libc::c_ulong as libc::c_ulonglong
                        && lcsalloc.wrapping_div(lcssize)
                            == core::mem::size_of::<uint32_t>() as libc::c_ulong
                                as libc::c_ulonglong
                    {
                        if lcsalloc
                            > server.proto_max_bulk_len as size_t as libc::c_ulonglong
                        {
                            addReplyError(
                                c,
                                b"Insufficient memory, transient memory for LCS exceeds proto-max-bulk-len\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            current_block = 5486260907809189108;
                        } else {
                            lcs = ztrymalloc(lcsalloc as size_t) as *mut uint32_t;
                            current_block = 2873832966593178012;
                        }
                    } else {
                        current_block = 2873832966593178012;
                    }
                    match current_block {
                        5486260907809189108 => {}
                        _ => {
                            if lcs.is_null() {
                                addReplyError(
                                    c,
                                    b"Insufficient memory, failed allocating transient memory for LCS\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            } else {
                                let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
                                while i_0 <= alen {
                                    let mut j_0: uint32_t = 0 as libc::c_int as uint32_t;
                                    while j_0 <= blen {
                                        if i_0 == 0 as libc::c_int as libc::c_uint
                                            || j_0 == 0 as libc::c_int as libc::c_uint
                                        {
                                            *lcs
                                                .offset(
                                                    j_0
                                                        .wrapping_add(
                                                            i_0
                                                                .wrapping_mul(
                                                                    blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                                ),
                                                        ) as isize,
                                                ) = 0 as libc::c_int as uint32_t;
                                        } else if *a
                                            .offset(
                                                i_0.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int
                                            == *b
                                                .offset(
                                                    j_0.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                                ) as libc::c_int
                                        {
                                            *lcs
                                                .offset(
                                                    j_0
                                                        .wrapping_add(
                                                            i_0
                                                                .wrapping_mul(
                                                                    blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                                ),
                                                        ) as isize,
                                                ) = (*lcs
                                                .offset(
                                                    j_0
                                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                        .wrapping_add(
                                                            i_0
                                                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                                .wrapping_mul(
                                                                    blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                                ),
                                                        ) as isize,
                                                ))
                                                .wrapping_add(1 as libc::c_int as libc::c_uint);
                                        } else {
                                            let mut lcs1: uint32_t = *lcs
                                                .offset(
                                                    j_0
                                                        .wrapping_add(
                                                            i_0
                                                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                                .wrapping_mul(
                                                                    blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                                ),
                                                        ) as isize,
                                                );
                                            let mut lcs2: uint32_t = *lcs
                                                .offset(
                                                    j_0
                                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                        .wrapping_add(
                                                            i_0
                                                                .wrapping_mul(
                                                                    blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                                ),
                                                        ) as isize,
                                                );
                                            *lcs
                                                .offset(
                                                    j_0
                                                        .wrapping_add(
                                                            i_0
                                                                .wrapping_mul(
                                                                    blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                                ),
                                                        ) as isize,
                                                ) = if lcs1 > lcs2 { lcs1 } else { lcs2 };
                                        }
                                        j_0 = j_0.wrapping_add(1);
                                    }
                                    i_0 = i_0.wrapping_add(1);
                                }
                                idx = *lcs
                                    .offset(
                                        blen
                                            .wrapping_add(
                                                alen
                                                    .wrapping_mul(
                                                        blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                    ),
                                            ) as isize,
                                    );
                                result = 0 as sds;
                                arraylenptr = 0 as *mut libc::c_void;
                                arange_start = alen;
                                arange_end = 0 as libc::c_int as uint32_t;
                                brange_start = 0 as libc::c_int as uint32_t;
                                brange_end = 0 as libc::c_int as uint32_t;
                                computelcs = (getidx != 0 || getlen == 0) as libc::c_int;
                                if computelcs != 0 {
                                    result = sdsnewlen(
                                        SDS_NOINIT as *const libc::c_void,
                                        idx as size_t,
                                    );
                                }
                                arraylen = 0 as libc::c_int as uint32_t;
                                if getidx != 0 {
                                    addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
                                    addReplyBulkCString(
                                        c,
                                        b"matches\0" as *const u8 as *const libc::c_char,
                                    );
                                    arraylenptr = addReplyDeferredLen(c);
                                }
                                i = alen;
                                j = blen;
                                while computelcs != 0
                                    && i > 0 as libc::c_int as libc::c_uint
                                    && j > 0 as libc::c_int as libc::c_uint
                                {
                                    let mut emit_range: libc::c_int = 0 as libc::c_int;
                                    if *a
                                        .offset(
                                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_int
                                        == *b
                                            .offset(
                                                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) as libc::c_int
                                    {
                                        *result
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            ) = *a
                                            .offset(
                                                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                            );
                                        if arange_start == alen {
                                            arange_start = i
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint);
                                            arange_end = i
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint);
                                            brange_start = j
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint);
                                            brange_end = j
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint);
                                        } else if arange_start == i && brange_start == j {
                                            arange_start = arange_start.wrapping_sub(1);
                                            brange_start = brange_start.wrapping_sub(1);
                                        } else {
                                            emit_range = 1 as libc::c_int;
                                        }
                                        if arange_start == 0 as libc::c_int as libc::c_uint
                                            || brange_start == 0 as libc::c_int as libc::c_uint
                                        {
                                            emit_range = 1 as libc::c_int;
                                        }
                                        idx = idx.wrapping_sub(1);
                                        i = i.wrapping_sub(1);
                                        j = j.wrapping_sub(1);
                                    } else {
                                        let mut lcs1_0: uint32_t = *lcs
                                            .offset(
                                                j
                                                    .wrapping_add(
                                                        i
                                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                            .wrapping_mul(
                                                                blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                            ),
                                                    ) as isize,
                                            );
                                        let mut lcs2_0: uint32_t = *lcs
                                            .offset(
                                                j
                                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                    .wrapping_add(
                                                        i
                                                            .wrapping_mul(
                                                                blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                            ),
                                                    ) as isize,
                                            );
                                        if lcs1_0 > lcs2_0 {
                                            i = i.wrapping_sub(1);
                                        } else {
                                            j = j.wrapping_sub(1);
                                        }
                                        if arange_start != alen {
                                            emit_range = 1 as libc::c_int;
                                        }
                                    }
                                    let mut match_len: uint32_t = arange_end
                                        .wrapping_sub(arange_start)
                                        .wrapping_add(1 as libc::c_int as libc::c_uint);
                                    if emit_range != 0 {
                                        if minmatchlen == 0 as libc::c_int as libc::c_longlong
                                            || match_len as libc::c_longlong >= minmatchlen
                                        {
                                            if !arraylenptr.is_null() {
                                                addReplyArrayLen(
                                                    c,
                                                    (2 as libc::c_int + withmatchlen) as libc::c_long,
                                                );
                                                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                                                addReplyLongLong(c, arange_start as libc::c_longlong);
                                                addReplyLongLong(c, arange_end as libc::c_longlong);
                                                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                                                addReplyLongLong(c, brange_start as libc::c_longlong);
                                                addReplyLongLong(c, brange_end as libc::c_longlong);
                                                if withmatchlen != 0 {
                                                    addReplyLongLong(c, match_len as libc::c_longlong);
                                                }
                                                arraylen = arraylen.wrapping_add(1);
                                            }
                                        }
                                        arange_start = alen;
                                    }
                                }
                                if !arraylenptr.is_null() {
                                    addReplyBulkCString(
                                        c,
                                        b"len\0" as *const u8 as *const libc::c_char,
                                    );
                                    addReplyLongLong(
                                        c,
                                        *lcs
                                            .offset(
                                                blen
                                                    .wrapping_add(
                                                        alen
                                                            .wrapping_mul(
                                                                blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                            ),
                                                    ) as isize,
                                            ) as libc::c_longlong,
                                    );
                                    setDeferredArrayLen(
                                        c,
                                        arraylenptr,
                                        arraylen as libc::c_long,
                                    );
                                } else if getlen != 0 {
                                    addReplyLongLong(
                                        c,
                                        *lcs
                                            .offset(
                                                blen
                                                    .wrapping_add(
                                                        alen
                                                            .wrapping_mul(
                                                                blen.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                            ),
                                                    ) as isize,
                                            ) as libc::c_longlong,
                                    );
                                } else {
                                    addReplyBulkSds(c, result);
                                    result = 0 as sds;
                                }
                                sdsfree(result);
                                zfree(lcs as *mut libc::c_void);
                            }
                        }
                    }
                }
            }
        }
    }
    if !obja.is_null() {
        decrRefCount(obja);
    }
    if !objb.is_null() {
        decrRefCount(objb);
    }
}
