extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    static mut server: redisServer;
    fn mstime() -> libc::c_longlong;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdsAllocSize(s: sds) -> size_t;
    fn rand() -> libc::c_int;
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
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    static mut getMonotonicUs: Option::<unsafe extern "C" fn() -> monotime>;
    fn aeCreateTimeEvent(
        eventLoop: *mut aeEventLoop,
        milliseconds: libc::c_longlong,
        proc_0: Option::<aeTimeProc>,
        clientData: *mut libc::c_void,
        finalizerProc: Option::<aeEventFinalizerProc>,
    ) -> libc::c_longlong;
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictGetRandomKey(d: *mut dict) -> *mut dictEntry;
    fn dictGetSomeKeys(
        d: *mut dict,
        des: *mut *mut dictEntry,
        count: libc::c_uint,
    ) -> libc::c_uint;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zmalloc_used_memory() -> size_t;
    fn latencyAddSample(event: *const libc::c_char, latency: mstime_t);
    fn flushSlavesOutputBuffers();
    fn checkClientPauseTimeoutAndReturnIfPaused() -> libc::c_int;
    fn decrRefCount(o: *mut robj);
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn propagatePendingCommands();
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn propagateDeletion(db: *mut redisDb, key: *mut robj, lazy: libc::c_int);
    fn dbSyncDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn dbAsyncDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn isInsideYieldingLongCommand() -> libc::c_int;
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
    fn bioPendingJobsOfType(type_0: libc::c_int) -> libc::c_ulonglong;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub type __useconds_t = libc::c_uint;
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
pub type memory_order = libc::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
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
pub struct evictionPoolEntry {
    pub idle: libc::c_ulonglong,
    pub key: sds,
    pub cached: sds,
    pub dbid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replBufBlock {
    pub refcount: libc::c_int,
    pub id: libc::c_longlong,
    pub repl_offset: libc::c_longlong,
    pub size: size_t,
    pub used: size_t,
    pub buf: [libc::c_char; 0],
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
unsafe extern "C" fn elapsedStart(mut start_time: *mut monotime) {
    *start_time = getMonotonicUs.expect("non-null function pointer")();
}
#[inline]
unsafe extern "C" fn elapsedUs(mut start_time: monotime) -> uint64_t {
    return (getMonotonicUs.expect("non-null function pointer")())
        .wrapping_sub(start_time);
}
static mut EvictionPoolLRU: *mut evictionPoolEntry = 0 as *const evictionPoolEntry
    as *mut evictionPoolEntry;
#[no_mangle]
pub unsafe extern "C" fn getLRUClock() -> libc::c_uint {
    return (mstime() / 1000 as libc::c_int as libc::c_longlong
        & (((1 as libc::c_int) << 24 as libc::c_int) - 1 as libc::c_int)
            as libc::c_longlong) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn LRU_CLOCK() -> libc::c_uint {
    let mut lruclock: libc::c_uint = 0;
    if 1000 as libc::c_int / server.hz <= 1000 as libc::c_int {
        lruclock = core::intrinsics::atomic_load_relaxed(&mut server.lruclock);
    } else {
        lruclock = getLRUClock();
    }
    return lruclock;
}
#[no_mangle]
pub unsafe extern "C" fn estimateObjectIdleTime(mut o: *mut robj) -> libc::c_ulonglong {
    let mut lruclock: libc::c_ulonglong = LRU_CLOCK() as libc::c_ulonglong;
    if lruclock >= (*o).lru() as libc::c_ulonglong {
        return lruclock
            .wrapping_sub((*o).lru() as libc::c_ulonglong)
            .wrapping_mul(1000 as libc::c_int as libc::c_ulonglong)
    } else {
        return lruclock
            .wrapping_add(
                (((1 as libc::c_int) << 24 as libc::c_int) - 1 as libc::c_int
                    - (*o).lru() as libc::c_int) as libc::c_ulonglong,
            )
            .wrapping_mul(1000 as libc::c_int as libc::c_ulonglong)
    };
}
#[no_mangle]
pub unsafe extern "C" fn evictionPoolAlloc() {
    let mut ep: *mut evictionPoolEntry = 0 as *mut evictionPoolEntry;
    let mut j: libc::c_int = 0;
    ep = zmalloc(
        (core::mem::size_of::<evictionPoolEntry>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong),
    ) as *mut evictionPoolEntry;
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        (*ep.offset(j as isize)).idle = 0 as libc::c_int as libc::c_ulonglong;
        let ref mut fresh0 = (*ep.offset(j as isize)).key;
        *fresh0 = 0 as sds;
        let ref mut fresh1 = (*ep.offset(j as isize)).cached;
        *fresh1 = sdsnewlen(0 as *const libc::c_void, 255 as libc::c_int as size_t);
        (*ep.offset(j as isize)).dbid = 0 as libc::c_int;
        j += 1;
    }
    EvictionPoolLRU = ep;
}
#[no_mangle]
pub unsafe extern "C" fn evictionPoolPopulate(
    mut dbid: libc::c_int,
    mut sampledict: *mut dict,
    mut keydict: *mut dict,
    mut pool: *mut evictionPoolEntry,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let vla = server.maxmemory_samples as usize;
    let mut samples: Vec::<*mut dictEntry> = ::std::vec::from_elem(
        0 as *mut dictEntry,
        vla,
    );
    count = dictGetSomeKeys(
        sampledict,
        samples.as_mut_ptr(),
        server.maxmemory_samples as libc::c_uint,
    ) as libc::c_int;
    j = 0 as libc::c_int;
    while j < count {
        let mut idle: libc::c_ulonglong = 0;
        let mut key: sds = 0 as *mut libc::c_char;
        let mut o: *mut robj = 0 as *mut robj;
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        de = *samples.as_mut_ptr().offset(j as isize);
        key = (*de).key as sds;
        if server.maxmemory_policy != (2 as libc::c_int) << 8 as libc::c_int {
            if sampledict != keydict {
                de = dictFind(keydict, key as *const libc::c_void);
            }
            o = (*de).v.val as *mut robj;
        }
        if server.maxmemory_policy & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            idle = estimateObjectIdleTime(o);
        } else if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            idle = (255 as libc::c_int as libc::c_ulong)
                .wrapping_sub(LFUDecrAndReturn(o)) as libc::c_ulonglong;
        } else if server.maxmemory_policy == (2 as libc::c_int) << 8 as libc::c_int {
            idle = (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_sub((*de).v.val as libc::c_long as libc::c_ulonglong);
        } else {
            _serverPanic(
                b"evict.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_int,
                b"Unknown eviction policy in evictionPoolPopulate()\0" as *const u8
                    as *const libc::c_char,
            );
            unreachable!();
        }
        k = 0 as libc::c_int;
        while k < 16 as libc::c_int && !((*pool.offset(k as isize)).key).is_null()
            && (*pool.offset(k as isize)).idle < idle
        {
            k += 1;
        }
        if !(k == 0 as libc::c_int
            && !((*pool.offset((16 as libc::c_int - 1 as libc::c_int) as isize)).key)
                .is_null())
        {
            if !(k < 16 as libc::c_int && ((*pool.offset(k as isize)).key).is_null()) {
                if ((*pool.offset((16 as libc::c_int - 1 as libc::c_int) as isize)).key)
                    .is_null()
                {
                    let mut cached: sds = (*pool
                        .offset((16 as libc::c_int - 1 as libc::c_int) as isize))
                        .cached;
                    memmove(
                        pool.offset(k as isize).offset(1 as libc::c_int as isize)
                            as *mut libc::c_void,
                        pool.offset(k as isize) as *const libc::c_void,
                        (core::mem::size_of::<evictionPoolEntry>() as libc::c_ulong)
                            .wrapping_mul(
                                (16 as libc::c_int - k - 1 as libc::c_int) as libc::c_ulong,
                            ),
                    );
                    let ref mut fresh2 = (*pool.offset(k as isize)).cached;
                    *fresh2 = cached;
                } else {
                    k -= 1;
                    let mut cached_0: sds = (*pool.offset(0 as libc::c_int as isize))
                        .cached;
                    if (*pool.offset(0 as libc::c_int as isize)).key
                        != (*pool.offset(0 as libc::c_int as isize)).cached
                    {
                        sdsfree((*pool.offset(0 as libc::c_int as isize)).key);
                    }
                    memmove(
                        pool as *mut libc::c_void,
                        pool.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        (core::mem::size_of::<evictionPoolEntry>() as libc::c_ulong)
                            .wrapping_mul(k as libc::c_ulong),
                    );
                    let ref mut fresh3 = (*pool.offset(k as isize)).cached;
                    *fresh3 = cached_0;
                }
            }
            let mut klen: libc::c_int = sdslen(key) as libc::c_int;
            if klen > 255 as libc::c_int {
                let ref mut fresh4 = (*pool.offset(k as isize)).key;
                *fresh4 = sdsdup(key);
            } else {
                memcpy(
                    (*pool.offset(k as isize)).cached as *mut libc::c_void,
                    key as *const libc::c_void,
                    (klen + 1 as libc::c_int) as libc::c_ulong,
                );
                sdssetlen((*pool.offset(k as isize)).cached, klen as size_t);
                let ref mut fresh5 = (*pool.offset(k as isize)).key;
                *fresh5 = (*pool.offset(k as isize)).cached;
            }
            (*pool.offset(k as isize)).idle = idle;
            (*pool.offset(k as isize)).dbid = dbid;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn LFUGetTimeInMinutes() -> libc::c_ulong {
    return (server.unixtime / 60 as libc::c_int & 65535 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn LFUTimeElapsed(mut ldt: libc::c_ulong) -> libc::c_ulong {
    let mut now: libc::c_ulong = LFUGetTimeInMinutes();
    if now >= ldt {
        return now.wrapping_sub(ldt);
    }
    return (65535 as libc::c_int as libc::c_ulong).wrapping_sub(ldt).wrapping_add(now);
}
#[no_mangle]
pub unsafe extern "C" fn LFULogIncr(mut counter: uint8_t) -> uint8_t {
    if counter as libc::c_int == 255 as libc::c_int {
        return 255 as libc::c_int as uint8_t;
    }
    let mut r: libc::c_double = rand() as libc::c_double
        / 2147483647 as libc::c_int as libc::c_double;
    let mut baseval: libc::c_double = (counter as libc::c_int - 5 as libc::c_int)
        as libc::c_double;
    if baseval < 0 as libc::c_int as libc::c_double {
        baseval = 0 as libc::c_int as libc::c_double;
    }
    let mut p: libc::c_double = 1.0f64
        / (baseval * server.lfu_log_factor as libc::c_double
            + 1 as libc::c_int as libc::c_double);
    if r < p {
        counter = counter.wrapping_add(1);
    }
    return counter;
}
#[no_mangle]
pub unsafe extern "C" fn LFUDecrAndReturn(mut o: *mut robj) -> libc::c_ulong {
    let mut ldt: libc::c_ulong = ((*o).lru() as libc::c_int >> 8 as libc::c_int)
        as libc::c_ulong;
    let mut counter: libc::c_ulong = ((*o).lru() as libc::c_int & 255 as libc::c_int)
        as libc::c_ulong;
    let mut num_periods: libc::c_ulong = if server.lfu_decay_time != 0 {
        (LFUTimeElapsed(ldt)).wrapping_div(server.lfu_decay_time as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if num_periods != 0 {
        counter = if num_periods > counter {
            0 as libc::c_int as libc::c_ulong
        } else {
            counter.wrapping_sub(num_periods)
        };
    }
    return counter;
}
#[no_mangle]
pub unsafe extern "C" fn freeMemoryGetNotCountedMemory() -> size_t {
    let mut overhead: size_t = 0 as libc::c_int as size_t;
    if server.repl_buffer_mem as libc::c_longlong > server.repl_backlog_size {
        let mut extra_approx_size: size_t = ((server.repl_backlog_size
            / (16 as libc::c_int * 1024 as libc::c_int) as libc::c_longlong
            + 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong)
            .wrapping_mul(
                (core::mem::size_of::<replBufBlock>() as libc::c_ulong)
                    .wrapping_add(core::mem::size_of::<listNode>() as libc::c_ulong)
                    as libc::c_ulonglong,
            ) as size_t;
        let mut counted_mem: size_t = (server.repl_backlog_size as libc::c_ulonglong)
            .wrapping_add(extra_approx_size as libc::c_ulonglong) as size_t;
        if server.repl_buffer_mem > counted_mem {
            overhead = (overhead as libc::c_ulong)
                .wrapping_add((server.repl_buffer_mem).wrapping_sub(counted_mem))
                as size_t as size_t;
        }
    }
    if server.aof_state != 0 as libc::c_int {
        overhead = (overhead as libc::c_ulong).wrapping_add(sdsAllocSize(server.aof_buf))
            as size_t as size_t;
    }
    return overhead;
}
#[no_mangle]
pub unsafe extern "C" fn getMaxmemoryState(
    mut total: *mut size_t,
    mut logical: *mut size_t,
    mut tofree: *mut size_t,
    mut level: *mut libc::c_float,
) -> libc::c_int {
    let mut mem_reported: size_t = 0;
    let mut mem_used: size_t = 0;
    let mut mem_tofree: size_t = 0;
    mem_reported = zmalloc_used_memory();
    if !total.is_null() {
        *total = mem_reported;
    }
    if server.maxmemory == 0 {
        if !level.is_null() {
            *level = 0 as libc::c_int as libc::c_float;
        }
        return 0 as libc::c_int;
    }
    if mem_reported as libc::c_ulonglong <= server.maxmemory && level.is_null() {
        return 0 as libc::c_int;
    }
    mem_used = mem_reported;
    let mut overhead: size_t = freeMemoryGetNotCountedMemory();
    mem_used = if mem_used > overhead {
        mem_used.wrapping_sub(overhead)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if !level.is_null() {
        *level = mem_used as libc::c_float / server.maxmemory as libc::c_float;
    }
    if mem_reported as libc::c_ulonglong <= server.maxmemory {
        return 0 as libc::c_int;
    }
    if mem_used as libc::c_ulonglong <= server.maxmemory {
        return 0 as libc::c_int;
    }
    mem_tofree = (mem_used as libc::c_ulonglong).wrapping_sub(server.maxmemory)
        as size_t;
    if !logical.is_null() {
        *logical = mem_used;
    }
    if !tofree.is_null() {
        *tofree = mem_tofree;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn overMaxmemoryAfterAlloc(mut moremem: size_t) -> libc::c_int {
    if server.maxmemory == 0 {
        return 0 as libc::c_int;
    }
    let mut mem_used: size_t = zmalloc_used_memory();
    if mem_used.wrapping_add(moremem) as libc::c_ulonglong <= server.maxmemory {
        return 0 as libc::c_int;
    }
    let mut overhead: size_t = freeMemoryGetNotCountedMemory();
    mem_used = if mem_used > overhead {
        mem_used.wrapping_sub(overhead)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    return (mem_used.wrapping_add(moremem) as libc::c_ulonglong > server.maxmemory)
        as libc::c_int;
}
static mut isEvictionProcRunning: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn evictionTimeProc(
    mut eventLoop: *mut aeEventLoop,
    mut id: libc::c_longlong,
    mut clientData: *mut libc::c_void,
) -> libc::c_int {
    if performEvictions() == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    isEvictionProcRunning = 0 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn startEvictionTimeProc() {
    if isEvictionProcRunning == 0 {
        isEvictionProcRunning = 1 as libc::c_int;
        aeCreateTimeEvent(
            server.el,
            0 as libc::c_int as libc::c_longlong,
            Some(
                evictionTimeProc
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_longlong,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
            None,
        );
    }
}
unsafe extern "C" fn isSafeToPerformEvictions() -> libc::c_int {
    if isInsideYieldingLongCommand() != 0 || server.loading != 0 {
        return 0 as libc::c_int;
    }
    if !(server.masterhost).is_null() && server.repl_slave_ignore_maxmemory != 0 {
        return 0 as libc::c_int;
    }
    if checkClientPauseTimeoutAndReturnIfPaused() != 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn evictionTimeLimitUs() -> libc::c_ulong {
    if server.maxmemory_eviction_tenacity >= 0 as libc::c_int {} else {
        _serverAssert(
            b"server.maxmemory_eviction_tenacity >= 0\0" as *const u8
                as *const libc::c_char,
            b"evict.c\0" as *const u8 as *const libc::c_char,
            500 as libc::c_int,
        );
        unreachable!();
    };
    if server.maxmemory_eviction_tenacity <= 100 as libc::c_int {} else {
        _serverAssert(
            b"server.maxmemory_eviction_tenacity <= 100\0" as *const u8
                as *const libc::c_char,
            b"evict.c\0" as *const u8 as *const libc::c_char,
            501 as libc::c_int,
        );
        unreachable!();
    };
    if server.maxmemory_eviction_tenacity <= 10 as libc::c_int {
        return (50 as libc::c_ulong)
            .wrapping_mul(server.maxmemory_eviction_tenacity as libc::c_ulong);
    }
    if server.maxmemory_eviction_tenacity < 100 as libc::c_int {
        return (500.0f64
            * pow(
                1.15f64,
                server.maxmemory_eviction_tenacity as libc::c_double - 10.0f64,
            )) as libc::c_ulong;
    }
    return (9223372036854775807 as libc::c_long as libc::c_ulong)
        .wrapping_mul(2 as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn performEvictions() -> libc::c_int {
    let mut eviction_time_limit_us: libc::c_ulong = 0;
    let mut evictionTimer: monotime = 0;
    let mut prev_core_propagates: libc::c_int = 0;
    let mut current_block: u64;
    if isSafeToPerformEvictions() == 0 {
        return 0 as libc::c_int;
    }
    let mut keys_freed: libc::c_int = 0 as libc::c_int;
    let mut mem_reported: size_t = 0;
    let mut mem_tofree: size_t = 0;
    let mut mem_freed: libc::c_longlong = 0;
    let mut latency: mstime_t = 0;
    let mut eviction_latency: mstime_t = 0;
    let mut delta: libc::c_longlong = 0;
    let mut slaves: libc::c_int = (*server.slaves).len as libc::c_int;
    let mut result: libc::c_int = 2 as libc::c_int;
    if getMaxmemoryState(
        &mut mem_reported,
        0 as *mut size_t,
        &mut mem_tofree,
        0 as *mut libc::c_float,
    ) == 0 as libc::c_int
    {
        result = 0 as libc::c_int;
    } else if server.maxmemory_policy == (7 as libc::c_int) << 8 as libc::c_int {
        result = 2 as libc::c_int;
    } else {
        eviction_time_limit_us = evictionTimeLimitUs();
        mem_freed = 0 as libc::c_int as libc::c_longlong;
        if server.latency_monitor_threshold != 0 {
            latency = mstime();
        } else {
            latency = 0 as libc::c_int as mstime_t;
        }
        evictionTimer = 0;
        elapsedStart(&mut evictionTimer);
        prev_core_propagates = server.core_propagates;
        if server.also_propagate.numops == 0 as libc::c_int {} else {
            _serverAssert(
                b"server.also_propagate.numops == 0\0" as *const u8
                    as *const libc::c_char,
                b"evict.c\0" as *const u8 as *const libc::c_char,
                575 as libc::c_int,
            );
            unreachable!();
        };
        server.core_propagates = 1 as libc::c_int;
        server.propagate_no_multi = 1 as libc::c_int;
        loop {
            if !(mem_freed < mem_tofree as libc::c_longlong) {
                current_block = 10213293998891106930;
                break;
            }
            let mut j: libc::c_int = 0;
            let mut k: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            static mut next_db: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut bestkey: sds = 0 as sds;
            let mut bestdbid: libc::c_int = 0;
            let mut db: *mut redisDb = 0 as *mut redisDb;
            let mut dict: *mut dict = 0 as *mut dict;
            let mut de: *mut dictEntry = 0 as *mut dictEntry;
            if server.maxmemory_policy
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
                || server.maxmemory_policy == (2 as libc::c_int) << 8 as libc::c_int
            {
                let mut pool: *mut evictionPoolEntry = EvictionPoolLRU;
                while bestkey.is_null() {
                    let mut total_keys: libc::c_ulong = 0 as libc::c_int
                        as libc::c_ulong;
                    let mut keys: libc::c_ulong = 0;
                    i = 0 as libc::c_int;
                    while i < server.dbnum {
                        db = (server.db).offset(i as isize);
                        dict = if server.maxmemory_policy
                            & (1 as libc::c_int) << 2 as libc::c_int != 0
                        {
                            (*db).dict
                        } else {
                            (*db).expires
                        };
                        keys = ((*dict).ht_used[0 as libc::c_int as usize])
                            .wrapping_add((*dict).ht_used[1 as libc::c_int as usize]);
                        if keys != 0 as libc::c_int as libc::c_ulong {
                            evictionPoolPopulate(i, dict, (*db).dict, pool);
                            total_keys = total_keys.wrapping_add(keys);
                        }
                        i += 1;
                    }
                    if total_keys == 0 {
                        break;
                    }
                    k = 16 as libc::c_int - 1 as libc::c_int;
                    while k >= 0 as libc::c_int {
                        if !((*pool.offset(k as isize)).key).is_null() {
                            bestdbid = (*pool.offset(k as isize)).dbid;
                            if server.maxmemory_policy
                                & (1 as libc::c_int) << 2 as libc::c_int != 0
                            {
                                de = dictFind(
                                    (*(server.db).offset(bestdbid as isize)).dict,
                                    (*pool.offset(k as isize)).key as *const libc::c_void,
                                );
                            } else {
                                de = dictFind(
                                    (*(server.db).offset(bestdbid as isize)).expires,
                                    (*pool.offset(k as isize)).key as *const libc::c_void,
                                );
                            }
                            if (*pool.offset(k as isize)).key
                                != (*pool.offset(k as isize)).cached
                            {
                                sdsfree((*pool.offset(k as isize)).key);
                            }
                            let ref mut fresh6 = (*pool.offset(k as isize)).key;
                            *fresh6 = 0 as sds;
                            (*pool.offset(k as isize))
                                .idle = 0 as libc::c_int as libc::c_ulonglong;
                            if !de.is_null() {
                                bestkey = (*de).key as sds;
                                break;
                            }
                        }
                        k -= 1;
                    }
                }
            } else if server.maxmemory_policy
                == (6 as libc::c_int) << 8 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                || server.maxmemory_policy == (3 as libc::c_int) << 8 as libc::c_int
            {
                i = 0 as libc::c_int;
                while i < server.dbnum {
                    next_db = next_db.wrapping_add(1);
                    j = next_db.wrapping_rem(server.dbnum as libc::c_uint)
                        as libc::c_int;
                    db = (server.db).offset(j as isize);
                    dict = if server.maxmemory_policy
                        == (6 as libc::c_int) << 8 as libc::c_int
                            | (1 as libc::c_int) << 2 as libc::c_int
                    {
                        (*db).dict
                    } else {
                        (*db).expires
                    };
                    if ((*dict).ht_used[0 as libc::c_int as usize])
                        .wrapping_add((*dict).ht_used[1 as libc::c_int as usize])
                        != 0 as libc::c_int as libc::c_ulong
                    {
                        de = dictGetRandomKey(dict);
                        bestkey = (*de).key as sds;
                        bestdbid = j;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
            if bestkey.is_null() {
                current_block = 316881304435480400;
                break;
            }
            db = (server.db).offset(bestdbid as isize);
            let mut keyobj: *mut robj = createStringObject(
                bestkey as *const libc::c_char,
                sdslen(bestkey),
            );
            delta = zmalloc_used_memory() as libc::c_longlong;
            if server.latency_monitor_threshold != 0 {
                eviction_latency = mstime();
            } else {
                eviction_latency = 0 as libc::c_int as mstime_t;
            }
            if server.lazyfree_lazy_eviction != 0 {
                dbAsyncDelete(db, keyobj);
            } else {
                dbSyncDelete(db, keyobj);
            }
            if server.latency_monitor_threshold != 0 {
                eviction_latency = mstime() - eviction_latency;
            }
            if server.latency_monitor_threshold != 0
                && eviction_latency >= server.latency_monitor_threshold
            {
                latencyAddSample(
                    b"eviction-del\0" as *const u8 as *const libc::c_char,
                    eviction_latency,
                );
            }
            delta -= zmalloc_used_memory() as libc::c_longlong;
            mem_freed += delta;
            server.stat_evictedkeys += 1;
            signalModifiedKey(0 as *mut client, db, keyobj);
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 9 as libc::c_int,
                b"evicted\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                keyobj,
                (*db).id,
            );
            propagateDeletion(db, keyobj, server.lazyfree_lazy_eviction);
            decrRefCount(keyobj);
            keys_freed += 1;
            if !(keys_freed % 16 as libc::c_int == 0 as libc::c_int) {
                continue;
            }
            if slaves != 0 {
                flushSlavesOutputBuffers();
            }
            if server.lazyfree_lazy_eviction != 0 {
                if getMaxmemoryState(
                    0 as *mut size_t,
                    0 as *mut size_t,
                    0 as *mut size_t,
                    0 as *mut libc::c_float,
                ) == 0 as libc::c_int
                {
                    current_block = 10213293998891106930;
                    break;
                }
            }
            if !(elapsedUs(evictionTimer) > eviction_time_limit_us) {
                continue;
            }
            startEvictionTimeProc();
            current_block = 10213293998891106930;
            break;
        }
        match current_block {
            10213293998891106930 => {
                result = if isEvictionProcRunning != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
            }
            _ => {}
        }
        if result == 2 as libc::c_int {
            let mut lazyfree_latency: mstime_t = 0;
            if server.latency_monitor_threshold != 0 {
                lazyfree_latency = mstime();
            } else {
                lazyfree_latency = 0 as libc::c_int as mstime_t;
            }
            while bioPendingJobsOfType(2 as libc::c_int) != 0
                && elapsedUs(evictionTimer) < eviction_time_limit_us
            {
                if getMaxmemoryState(
                    0 as *mut size_t,
                    0 as *mut size_t,
                    0 as *mut size_t,
                    0 as *mut libc::c_float,
                ) == 0 as libc::c_int
                {
                    result = 0 as libc::c_int;
                    break;
                } else {
                    usleep(
                        (if eviction_time_limit_us < 1000 as libc::c_int as libc::c_ulong
                        {
                            eviction_time_limit_us
                        } else {
                            1000 as libc::c_int as libc::c_ulong
                        }) as __useconds_t,
                    );
                }
            }
            if server.latency_monitor_threshold != 0 {
                lazyfree_latency = mstime() - lazyfree_latency;
            }
            if server.latency_monitor_threshold != 0
                && lazyfree_latency >= server.latency_monitor_threshold
            {
                latencyAddSample(
                    b"eviction-lazyfree\0" as *const u8 as *const libc::c_char,
                    lazyfree_latency,
                );
            }
        }
        if server.core_propagates != 0 {} else {
            _serverAssert(
                b"server.core_propagates\0" as *const u8 as *const libc::c_char,
                b"evict.c\0" as *const u8 as *const libc::c_char,
                749 as libc::c_int,
            );
            unreachable!();
        };
        propagatePendingCommands();
        server.core_propagates = prev_core_propagates;
        server.propagate_no_multi = 0 as libc::c_int;
        if server.latency_monitor_threshold != 0 {
            latency = mstime() - latency;
        }
        if server.latency_monitor_threshold != 0
            && latency >= server.latency_monitor_threshold
        {
            latencyAddSample(
                b"eviction-cycle\0" as *const u8 as *const libc::c_char,
                latency,
            );
        }
    }
    if result == 1 as libc::c_int || result == 2 as libc::c_int {
        if server.stat_last_eviction_exceeded_time == 0 as libc::c_int as libc::c_ulong {
            elapsedStart(&mut server.stat_last_eviction_exceeded_time);
        }
    } else if result == 0 as libc::c_int {
        if server.stat_last_eviction_exceeded_time != 0 as libc::c_int as libc::c_ulong {
            server
                .stat_total_eviction_exceeded_time = (server
                .stat_total_eviction_exceeded_time as libc::c_ulonglong)
                .wrapping_add(
                    elapsedUs(server.stat_last_eviction_exceeded_time)
                        as libc::c_ulonglong,
                ) as libc::c_longlong as libc::c_longlong;
            server.stat_last_eviction_exceeded_time = 0 as libc::c_int as monotime;
        }
    }
    return result;
}
