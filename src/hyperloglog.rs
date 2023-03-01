extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReply(c: *mut client, obj: *mut robj);
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn sdsMakeRoomFor(s: sds, addlen: size_t) -> sds;
    fn sdsIncrLen(s: sds, incr: ssize_t);
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyStatus(c: *mut client, status: *const libc::c_char);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn decrRefCount(o: *mut robj);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn stringObjectLen(o: *mut robj) -> size_t;
    fn checkType(c: *mut client, o: *mut robj, type_0: libc::c_int) -> libc::c_int;
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn lookupKeyRead(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyWrite(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn dbAdd(db: *mut redisDb, key: *mut robj, val: *mut robj);
    fn dbUnshareStringValue(db: *mut redisDb, key: *mut robj, o: *mut robj) -> *mut robj;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
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
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn llroundl(_: f64) -> libc::c_longlong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hllhdr {
    pub magic: [libc::c_char; 4],
    pub encoding: uint8_t,
    pub notused: [uint8_t; 3],
    pub card: [uint8_t; 8],
    pub registers: [uint8_t; 0],
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
static mut invalid_hll_err: *mut libc::c_char = b"-INVALIDOBJ Corrupted HLL object detected\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn MurmurHash64A(
    mut key: *const libc::c_void,
    mut len: libc::c_int,
    mut seed: libc::c_uint,
) -> uint64_t {
    let m: uint64_t = 0xc6a4a7935bd1e995 as libc::c_ulong;
    let r: libc::c_int = 47 as libc::c_int;
    let mut h: uint64_t = seed as libc::c_ulong ^ (len as libc::c_ulong).wrapping_mul(m);
    let mut data: *const uint8_t = key as *const uint8_t;
    let mut end: *const uint8_t = data.offset((len - (len & 7 as libc::c_int)) as isize);
    while data != end {
        let mut k: uint64_t = 0;
        k = *(data as *mut uint64_t);
        k = (k as libc::c_ulong).wrapping_mul(m) as uint64_t as uint64_t;
        k ^= k >> r;
        k = (k as libc::c_ulong).wrapping_mul(m) as uint64_t as uint64_t;
        h ^= k;
        h = (h as libc::c_ulong).wrapping_mul(m) as uint64_t as uint64_t;
        data = data.offset(8 as libc::c_int as isize);
    }
    let mut current_block_16: u64;
    match len & 7 as libc::c_int {
        7 => {
            h
                ^= (*data.offset(6 as libc::c_int as isize) as uint64_t)
                    << 48 as libc::c_int;
            current_block_16 = 16156281075998952011;
        }
        6 => {
            current_block_16 = 16156281075998952011;
        }
        5 => {
            current_block_16 = 11813332777664204542;
        }
        4 => {
            current_block_16 = 3600228491966162135;
        }
        3 => {
            current_block_16 = 4519919386293895278;
        }
        2 => {
            current_block_16 = 13565725371418141234;
        }
        1 => {
            current_block_16 = 10324593184274379554;
        }
        _ => {
            current_block_16 = 4956146061682418353;
        }
    }
    match current_block_16 {
        16156281075998952011 => {
            h
                ^= (*data.offset(5 as libc::c_int as isize) as uint64_t)
                    << 40 as libc::c_int;
            current_block_16 = 11813332777664204542;
        }
        _ => {}
    }
    match current_block_16 {
        11813332777664204542 => {
            h
                ^= (*data.offset(4 as libc::c_int as isize) as uint64_t)
                    << 32 as libc::c_int;
            current_block_16 = 3600228491966162135;
        }
        _ => {}
    }
    match current_block_16 {
        3600228491966162135 => {
            h
                ^= (*data.offset(3 as libc::c_int as isize) as uint64_t)
                    << 24 as libc::c_int;
            current_block_16 = 4519919386293895278;
        }
        _ => {}
    }
    match current_block_16 {
        4519919386293895278 => {
            h
                ^= (*data.offset(2 as libc::c_int as isize) as uint64_t)
                    << 16 as libc::c_int;
            current_block_16 = 13565725371418141234;
        }
        _ => {}
    }
    match current_block_16 {
        13565725371418141234 => {
            h
                ^= (*data.offset(1 as libc::c_int as isize) as uint64_t)
                    << 8 as libc::c_int;
            current_block_16 = 10324593184274379554;
        }
        _ => {}
    }
    match current_block_16 {
        10324593184274379554 => {
            h ^= *data.offset(0 as libc::c_int as isize) as uint64_t;
            h = (h as libc::c_ulong).wrapping_mul(m) as uint64_t as uint64_t;
        }
        _ => {}
    }
    h ^= h >> r;
    h = (h as libc::c_ulong).wrapping_mul(m) as uint64_t as uint64_t;
    h ^= h >> r;
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn hllPatLen(
    mut ele: *mut libc::c_uchar,
    mut elesize: size_t,
    mut regp: *mut libc::c_long,
) -> libc::c_int {
    let mut hash: uint64_t = 0;
    let mut bit: uint64_t = 0;
    let mut index: uint64_t = 0;
    let mut count: libc::c_int = 0;
    hash = MurmurHash64A(
        ele as *const libc::c_void,
        elesize as libc::c_int,
        0xadc83b19 as libc::c_ulonglong as libc::c_uint,
    );
    index = hash
        & (((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int)
            as libc::c_ulong;
    hash >>= 14 as libc::c_int;
    hash |= (1 as libc::c_int as uint64_t) << 64 as libc::c_int - 14 as libc::c_int;
    bit = 1 as libc::c_int as uint64_t;
    count = 1 as libc::c_int;
    while hash & bit == 0 as libc::c_int as libc::c_ulong {
        count += 1;
        bit <<= 1 as libc::c_int;
    }
    *regp = index as libc::c_int as libc::c_long;
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn hllDenseSet(
    mut registers: *mut uint8_t,
    mut index: libc::c_long,
    mut count: uint8_t,
) -> libc::c_int {
    let mut oldcount: uint8_t = 0;
    let mut _p: *mut uint8_t = registers;
    let mut _byte: libc::c_ulong = (index * 6 as libc::c_int as libc::c_long
        / 8 as libc::c_int as libc::c_long) as libc::c_ulong;
    let mut _fb: libc::c_ulong = (index * 6 as libc::c_int as libc::c_long
        & 7 as libc::c_int as libc::c_long) as libc::c_ulong;
    let mut _fb8: libc::c_ulong = (8 as libc::c_int as libc::c_ulong).wrapping_sub(_fb);
    let mut b0: libc::c_ulong = *_p.offset(_byte as isize) as libc::c_ulong;
    let mut b1: libc::c_ulong = *_p
        .offset(_byte.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_ulong;
    oldcount = ((b0 >> _fb | b1 << _fb8)
        & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
        as uint8_t;
    if count as libc::c_int > oldcount as libc::c_int {
        let mut _p_0: *mut uint8_t = registers;
        let mut _byte_0: libc::c_ulong = (index * 6 as libc::c_int as libc::c_long
            / 8 as libc::c_int as libc::c_long) as libc::c_ulong;
        let mut _fb_0: libc::c_ulong = (index * 6 as libc::c_int as libc::c_long
            & 7 as libc::c_int as libc::c_long) as libc::c_ulong;
        let mut _fb8_0: libc::c_ulong = (8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(_fb_0);
        let mut _v: libc::c_ulong = count as libc::c_ulong;
        let ref mut fresh0 = *_p_0.offset(_byte_0 as isize);
        *fresh0 = (*fresh0 as libc::c_int
            & !((((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) << _fb_0))
            as uint8_t;
        let ref mut fresh1 = *_p_0.offset(_byte_0 as isize);
        *fresh1 = (*fresh1 as libc::c_ulong | _v << _fb_0) as uint8_t;
        let ref mut fresh2 = *_p_0
            .offset(_byte_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        *fresh2 = (*fresh2 as libc::c_int
            & !(((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int >> _fb8_0))
            as uint8_t;
        let ref mut fresh3 = *_p_0
            .offset(_byte_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        *fresh3 = (*fresh3 as libc::c_ulong | _v >> _fb8_0) as uint8_t;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn hllDenseAdd(
    mut registers: *mut uint8_t,
    mut ele: *mut libc::c_uchar,
    mut elesize: size_t,
) -> libc::c_int {
    let mut index: libc::c_long = 0;
    let mut count: uint8_t = hllPatLen(ele, elesize, &mut index) as uint8_t;
    return hllDenseSet(registers, index, count);
}
#[no_mangle]
pub unsafe extern "C" fn hllDenseRegHisto(
    mut registers: *mut uint8_t,
    mut reghisto: *mut libc::c_int,
) {
    let mut j: libc::c_int = 0;
    if (1 as libc::c_int) << 14 as libc::c_int == 16384 as libc::c_int
        && 6 as libc::c_int == 6 as libc::c_int
    {
        let mut r: *mut uint8_t = registers;
        let mut r0: libc::c_ulong = 0;
        let mut r1: libc::c_ulong = 0;
        let mut r2: libc::c_ulong = 0;
        let mut r3: libc::c_ulong = 0;
        let mut r4: libc::c_ulong = 0;
        let mut r5: libc::c_ulong = 0;
        let mut r6: libc::c_ulong = 0;
        let mut r7: libc::c_ulong = 0;
        let mut r8: libc::c_ulong = 0;
        let mut r9: libc::c_ulong = 0;
        let mut r10: libc::c_ulong = 0;
        let mut r11: libc::c_ulong = 0;
        let mut r12: libc::c_ulong = 0;
        let mut r13: libc::c_ulong = 0;
        let mut r14: libc::c_ulong = 0;
        let mut r15: libc::c_ulong = 0;
        j = 0 as libc::c_int;
        while j < 1024 as libc::c_int {
            r0 = (*r.offset(0 as libc::c_int as isize) as libc::c_int
                & 63 as libc::c_int) as libc::c_ulong;
            r1 = ((*r.offset(0 as libc::c_int as isize) as libc::c_int
                >> 6 as libc::c_int
                | (*r.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 2 as libc::c_int) & 63 as libc::c_int) as libc::c_ulong;
            r2 = ((*r.offset(1 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int
                | (*r.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 4 as libc::c_int) & 63 as libc::c_int) as libc::c_ulong;
            r3 = (*r.offset(2 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int
                & 63 as libc::c_int) as libc::c_ulong;
            r4 = (*r.offset(3 as libc::c_int as isize) as libc::c_int
                & 63 as libc::c_int) as libc::c_ulong;
            r5 = ((*r.offset(3 as libc::c_int as isize) as libc::c_int
                >> 6 as libc::c_int
                | (*r.offset(4 as libc::c_int as isize) as libc::c_int)
                    << 2 as libc::c_int) & 63 as libc::c_int) as libc::c_ulong;
            r6 = ((*r.offset(4 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int
                | (*r.offset(5 as libc::c_int as isize) as libc::c_int)
                    << 4 as libc::c_int) & 63 as libc::c_int) as libc::c_ulong;
            r7 = (*r.offset(5 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int
                & 63 as libc::c_int) as libc::c_ulong;
            r8 = (*r.offset(6 as libc::c_int as isize) as libc::c_int
                & 63 as libc::c_int) as libc::c_ulong;
            r9 = ((*r.offset(6 as libc::c_int as isize) as libc::c_int
                >> 6 as libc::c_int
                | (*r.offset(7 as libc::c_int as isize) as libc::c_int)
                    << 2 as libc::c_int) & 63 as libc::c_int) as libc::c_ulong;
            r10 = ((*r.offset(7 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int
                | (*r.offset(8 as libc::c_int as isize) as libc::c_int)
                    << 4 as libc::c_int) & 63 as libc::c_int) as libc::c_ulong;
            r11 = (*r.offset(8 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int & 63 as libc::c_int) as libc::c_ulong;
            r12 = (*r.offset(9 as libc::c_int as isize) as libc::c_int
                & 63 as libc::c_int) as libc::c_ulong;
            r13 = ((*r.offset(9 as libc::c_int as isize) as libc::c_int
                >> 6 as libc::c_int
                | (*r.offset(10 as libc::c_int as isize) as libc::c_int)
                    << 2 as libc::c_int) & 63 as libc::c_int) as libc::c_ulong;
            r14 = ((*r.offset(10 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int
                | (*r.offset(11 as libc::c_int as isize) as libc::c_int)
                    << 4 as libc::c_int) & 63 as libc::c_int) as libc::c_ulong;
            r15 = (*r.offset(11 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int & 63 as libc::c_int) as libc::c_ulong;
            let ref mut fresh4 = *reghisto.offset(r0 as isize);
            *fresh4 += 1;
            let ref mut fresh5 = *reghisto.offset(r1 as isize);
            *fresh5 += 1;
            let ref mut fresh6 = *reghisto.offset(r2 as isize);
            *fresh6 += 1;
            let ref mut fresh7 = *reghisto.offset(r3 as isize);
            *fresh7 += 1;
            let ref mut fresh8 = *reghisto.offset(r4 as isize);
            *fresh8 += 1;
            let ref mut fresh9 = *reghisto.offset(r5 as isize);
            *fresh9 += 1;
            let ref mut fresh10 = *reghisto.offset(r6 as isize);
            *fresh10 += 1;
            let ref mut fresh11 = *reghisto.offset(r7 as isize);
            *fresh11 += 1;
            let ref mut fresh12 = *reghisto.offset(r8 as isize);
            *fresh12 += 1;
            let ref mut fresh13 = *reghisto.offset(r9 as isize);
            *fresh13 += 1;
            let ref mut fresh14 = *reghisto.offset(r10 as isize);
            *fresh14 += 1;
            let ref mut fresh15 = *reghisto.offset(r11 as isize);
            *fresh15 += 1;
            let ref mut fresh16 = *reghisto.offset(r12 as isize);
            *fresh16 += 1;
            let ref mut fresh17 = *reghisto.offset(r13 as isize);
            *fresh17 += 1;
            let ref mut fresh18 = *reghisto.offset(r14 as isize);
            *fresh18 += 1;
            let ref mut fresh19 = *reghisto.offset(r15 as isize);
            *fresh19 += 1;
            r = r.offset(12 as libc::c_int as isize);
            j += 1;
        }
    } else {
        j = 0 as libc::c_int;
        while j < (1 as libc::c_int) << 14 as libc::c_int {
            let mut reg: libc::c_ulong = 0;
            let mut _p: *mut uint8_t = registers;
            let mut _byte: libc::c_ulong = (j * 6 as libc::c_int / 8 as libc::c_int)
                as libc::c_ulong;
            let mut _fb: libc::c_ulong = (j * 6 as libc::c_int & 7 as libc::c_int)
                as libc::c_ulong;
            let mut _fb8: libc::c_ulong = (8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(_fb);
            let mut b0: libc::c_ulong = *_p.offset(_byte as isize) as libc::c_ulong;
            let mut b1: libc::c_ulong = *_p
                .offset(_byte.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_ulong;
            reg = (b0 >> _fb | b1 << _fb8)
                & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong;
            let ref mut fresh20 = *reghisto.offset(reg as isize);
            *fresh20 += 1;
            j += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn hllSparseToDense(mut o: *mut robj) -> libc::c_int {
    let mut sparse: sds = (*o).ptr as sds;
    let mut dense: sds = 0 as *mut libc::c_char;
    let mut hdr: *mut hllhdr = 0 as *mut hllhdr;
    let mut oldhdr: *mut hllhdr = sparse as *mut hllhdr;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut runlen: libc::c_int = 0;
    let mut regval: libc::c_int = 0;
    let mut p: *mut uint8_t = sparse as *mut uint8_t;
    let mut end: *mut uint8_t = p.offset(sdslen(sparse) as isize);
    hdr = sparse as *mut hllhdr;
    if (*hdr).encoding as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    dense = sdsnewlen(
        0 as *const libc::c_void,
        (core::mem::size_of::<hllhdr>() as libc::c_ulong)
            .wrapping_add(
                ((((1 as libc::c_int) << 14 as libc::c_int) * 6 as libc::c_int
                    + 7 as libc::c_int) / 8 as libc::c_int) as libc::c_ulong,
            ),
    );
    hdr = dense as *mut hllhdr;
    *hdr = *oldhdr;
    (*hdr).encoding = 0 as libc::c_int as uint8_t;
    p = p.offset(core::mem::size_of::<hllhdr>() as libc::c_ulong as isize);
    while p < end {
        if *p as libc::c_int & 0xc0 as libc::c_int == 0 as libc::c_int {
            runlen = (*p as libc::c_int & 0x3f as libc::c_int) + 1 as libc::c_int;
            idx += runlen;
            p = p.offset(1);
        } else if *p as libc::c_int & 0xc0 as libc::c_int == 0x40 as libc::c_int {
            runlen = ((*p as libc::c_int & 0x3f as libc::c_int) << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int)
                + 1 as libc::c_int;
            idx += runlen;
            p = p.offset(2 as libc::c_int as isize);
        } else {
            runlen = (*p as libc::c_int & 0x3 as libc::c_int) + 1 as libc::c_int;
            regval = (*p as libc::c_int >> 2 as libc::c_int & 0x1f as libc::c_int)
                + 1 as libc::c_int;
            if runlen + idx > (1 as libc::c_int) << 14 as libc::c_int {
                break;
            }
            loop {
                let fresh21 = runlen;
                runlen = runlen - 1;
                if !(fresh21 != 0) {
                    break;
                }
                let mut _p: *mut uint8_t = ((*hdr).registers).as_mut_ptr();
                let mut _byte: libc::c_ulong = (idx * 6 as libc::c_int
                    / 8 as libc::c_int) as libc::c_ulong;
                let mut _fb: libc::c_ulong = (idx * 6 as libc::c_int & 7 as libc::c_int)
                    as libc::c_ulong;
                let mut _fb8: libc::c_ulong = (8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(_fb);
                let mut _v: libc::c_ulong = regval as libc::c_ulong;
                let ref mut fresh22 = *_p.offset(_byte as isize);
                *fresh22 = (*fresh22 as libc::c_int
                    & !((((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                        << _fb)) as uint8_t;
                let ref mut fresh23 = *_p.offset(_byte as isize);
                *fresh23 = (*fresh23 as libc::c_ulong | _v << _fb) as uint8_t;
                let ref mut fresh24 = *_p
                    .offset(
                        _byte.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *fresh24 = (*fresh24 as libc::c_int
                    & !(((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int
                        >> _fb8)) as uint8_t;
                let ref mut fresh25 = *_p
                    .offset(
                        _byte.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *fresh25 = (*fresh25 as libc::c_ulong | _v >> _fb8) as uint8_t;
                idx += 1;
            }
            p = p.offset(1);
        }
    }
    if idx != (1 as libc::c_int) << 14 as libc::c_int {
        sdsfree(dense);
        return -(1 as libc::c_int);
    }
    sdsfree((*o).ptr as sds);
    (*o).ptr = dense as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hllSparseSet(
    mut o: *mut robj,
    mut index: libc::c_long,
    mut count: uint8_t,
) -> libc::c_int {
    let mut seq: [uint8_t; 5] = [0; 5];
    let mut n: *mut uint8_t = 0 as *mut uint8_t;
    let mut last: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut seqlen: libc::c_int = 0;
    let mut oldlen: libc::c_int = 0;
    let mut deltalen: libc::c_int = 0;
    let mut scanlen: libc::c_int = 0;
    let mut current_block: u64;
    let mut hdr: *mut hllhdr = 0 as *mut hllhdr;
    let mut oldcount: uint8_t = 0;
    let mut sparse: *mut uint8_t = 0 as *mut uint8_t;
    let mut end: *mut uint8_t = 0 as *mut uint8_t;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut prev: *mut uint8_t = 0 as *mut uint8_t;
    let mut next: *mut uint8_t = 0 as *mut uint8_t;
    let mut first: libc::c_long = 0;
    let mut span: libc::c_long = 0;
    let mut is_zero: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut is_xzero: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut is_val: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut runlen: libc::c_long = 0 as libc::c_int as libc::c_long;
    if !(count as libc::c_int > 32 as libc::c_int) {
        (*o)
            .ptr = sdsMakeRoomFor((*o).ptr as sds, 3 as libc::c_int as size_t)
            as *mut libc::c_void;
        p = ((*o).ptr as *mut uint8_t)
            .offset(core::mem::size_of::<hllhdr>() as libc::c_ulong as isize);
        sparse = p;
        end = p
            .offset(sdslen((*o).ptr as sds) as isize)
            .offset(-(core::mem::size_of::<hllhdr>() as libc::c_ulong as isize));
        first = 0 as libc::c_int as libc::c_long;
        prev = 0 as *mut uint8_t;
        next = 0 as *mut uint8_t;
        span = 0 as libc::c_int as libc::c_long;
        while p < end {
            let mut oplen: libc::c_long = 0;
            oplen = 1 as libc::c_int as libc::c_long;
            if *p as libc::c_int & 0xc0 as libc::c_int == 0 as libc::c_int {
                span = ((*p as libc::c_int & 0x3f as libc::c_int) + 1 as libc::c_int)
                    as libc::c_long;
            } else if *p as libc::c_int & 0x80 as libc::c_int != 0 {
                span = ((*p as libc::c_int & 0x3 as libc::c_int) + 1 as libc::c_int)
                    as libc::c_long;
            } else {
                span = (((*p as libc::c_int & 0x3f as libc::c_int) << 8 as libc::c_int
                    | *p.offset(1 as libc::c_int as isize) as libc::c_int)
                    + 1 as libc::c_int) as libc::c_long;
                oplen = 2 as libc::c_int as libc::c_long;
            }
            if index <= first + span - 1 as libc::c_int as libc::c_long {
                break;
            }
            prev = p;
            p = p.offset(oplen as isize);
            first += span;
        }
        if span == 0 as libc::c_int as libc::c_long || p >= end {
            return -(1 as libc::c_int);
        }
        next = if *p as libc::c_int & 0xc0 as libc::c_int == 0x40 as libc::c_int {
            p.offset(2 as libc::c_int as isize)
        } else {
            p.offset(1 as libc::c_int as isize)
        };
        if next >= end {
            next = 0 as *mut uint8_t;
        }
        if *p as libc::c_int & 0xc0 as libc::c_int == 0 as libc::c_int {
            is_zero = 1 as libc::c_int as libc::c_long;
            runlen = ((*p as libc::c_int & 0x3f as libc::c_int) + 1 as libc::c_int)
                as libc::c_long;
        } else if *p as libc::c_int & 0xc0 as libc::c_int == 0x40 as libc::c_int {
            is_xzero = 1 as libc::c_int as libc::c_long;
            runlen = (((*p as libc::c_int & 0x3f as libc::c_int) << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int)
                + 1 as libc::c_int) as libc::c_long;
        } else {
            is_val = 1 as libc::c_int as libc::c_long;
            runlen = ((*p as libc::c_int & 0x3 as libc::c_int) + 1 as libc::c_int)
                as libc::c_long;
        }
        if is_val != 0 {
            oldcount = ((*p as libc::c_int >> 2 as libc::c_int & 0x1f as libc::c_int)
                + 1 as libc::c_int) as uint8_t;
            if oldcount as libc::c_int >= count as libc::c_int {
                return 0 as libc::c_int;
            }
            if runlen == 1 as libc::c_int as libc::c_long {
                *p = ((count as libc::c_int - 1 as libc::c_int) << 2 as libc::c_int
                    | 1 as libc::c_int - 1 as libc::c_int | 0x80 as libc::c_int)
                    as uint8_t;
                current_block = 1772724598971987819;
            } else {
                current_block = 7746103178988627676;
            }
        } else {
            current_block = 7746103178988627676;
        }
        match current_block {
            7746103178988627676 => {
                if is_zero != 0 && runlen == 1 as libc::c_int as libc::c_long {
                    *p = ((count as libc::c_int - 1 as libc::c_int) << 2 as libc::c_int
                        | 1 as libc::c_int - 1 as libc::c_int | 0x80 as libc::c_int)
                        as uint8_t;
                    current_block = 1772724598971987819;
                } else {
                    seq = [0; 5];
                    n = seq.as_mut_ptr();
                    last = (first + span - 1 as libc::c_int as libc::c_long)
                        as libc::c_int;
                    len = 0;
                    if is_zero != 0 || is_xzero != 0 {
                        if index != first {
                            len = (index - first) as libc::c_int;
                            if len > 64 as libc::c_int {
                                let mut _l: libc::c_int = len - 1 as libc::c_int;
                                *n = (_l >> 8 as libc::c_int | 0x40 as libc::c_int)
                                    as uint8_t;
                                *n
                                    .offset(
                                        1 as libc::c_int as isize,
                                    ) = (_l & 0xff as libc::c_int) as uint8_t;
                                n = n.offset(2 as libc::c_int as isize);
                            } else {
                                *n = (len - 1 as libc::c_int) as uint8_t;
                                n = n.offset(1);
                            }
                        }
                        *n = ((count as libc::c_int - 1 as libc::c_int)
                            << 2 as libc::c_int | 1 as libc::c_int - 1 as libc::c_int
                            | 0x80 as libc::c_int) as uint8_t;
                        n = n.offset(1);
                        if index != last as libc::c_long {
                            len = (last as libc::c_long - index) as libc::c_int;
                            if len > 64 as libc::c_int {
                                let mut _l_0: libc::c_int = len - 1 as libc::c_int;
                                *n = (_l_0 >> 8 as libc::c_int | 0x40 as libc::c_int)
                                    as uint8_t;
                                *n
                                    .offset(
                                        1 as libc::c_int as isize,
                                    ) = (_l_0 & 0xff as libc::c_int) as uint8_t;
                                n = n.offset(2 as libc::c_int as isize);
                            } else {
                                *n = (len - 1 as libc::c_int) as uint8_t;
                                n = n.offset(1);
                            }
                        }
                    } else {
                        let mut curval: libc::c_int = (*p as libc::c_int
                            >> 2 as libc::c_int & 0x1f as libc::c_int)
                            + 1 as libc::c_int;
                        if index != first {
                            len = (index - first) as libc::c_int;
                            *n = ((curval - 1 as libc::c_int) << 2 as libc::c_int
                                | len - 1 as libc::c_int | 0x80 as libc::c_int) as uint8_t;
                            n = n.offset(1);
                        }
                        *n = ((count as libc::c_int - 1 as libc::c_int)
                            << 2 as libc::c_int | 1 as libc::c_int - 1 as libc::c_int
                            | 0x80 as libc::c_int) as uint8_t;
                        n = n.offset(1);
                        if index != last as libc::c_long {
                            len = (last as libc::c_long - index) as libc::c_int;
                            *n = ((curval - 1 as libc::c_int) << 2 as libc::c_int
                                | len - 1 as libc::c_int | 0x80 as libc::c_int) as uint8_t;
                            n = n.offset(1);
                        }
                    }
                    seqlen = n.offset_from(seq.as_mut_ptr()) as libc::c_long
                        as libc::c_int;
                    oldlen = if is_xzero != 0 {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    deltalen = seqlen - oldlen;
                    if deltalen > 0 as libc::c_int
                        && (sdslen((*o).ptr as sds))
                            .wrapping_add(deltalen as libc::c_ulong)
                            > server.hll_sparse_max_bytes
                    {
                        current_block = 18064604767964208725;
                    } else {
                        if deltalen != 0 && !next.is_null() {
                            memmove(
                                next.offset(deltalen as isize) as *mut libc::c_void,
                                next as *const libc::c_void,
                                end.offset_from(next) as libc::c_long as libc::c_ulong,
                            );
                        }
                        sdsIncrLen((*o).ptr as sds, deltalen as ssize_t);
                        memcpy(
                            p as *mut libc::c_void,
                            seq.as_mut_ptr() as *const libc::c_void,
                            seqlen as libc::c_ulong,
                        );
                        end = end.offset(deltalen as isize);
                        current_block = 1772724598971987819;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            18064604767964208725 => {}
            _ => {
                p = if !prev.is_null() { prev } else { sparse };
                scanlen = 5 as libc::c_int;
                while p < end
                    && {
                        let fresh26 = scanlen;
                        scanlen = scanlen - 1;
                        fresh26 != 0
                    }
                {
                    if *p as libc::c_int & 0xc0 as libc::c_int == 0x40 as libc::c_int {
                        p = p.offset(2 as libc::c_int as isize);
                    } else if *p as libc::c_int & 0xc0 as libc::c_int == 0 as libc::c_int
                    {
                        p = p.offset(1);
                    } else {
                        if p.offset(1 as libc::c_int as isize) < end
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x80 as libc::c_int != 0
                        {
                            let mut v1: libc::c_int = (*p as libc::c_int
                                >> 2 as libc::c_int & 0x1f as libc::c_int)
                                + 1 as libc::c_int;
                            let mut v2: libc::c_int = (*p
                                .offset(1 as libc::c_int as isize) as libc::c_int
                                >> 2 as libc::c_int & 0x1f as libc::c_int)
                                + 1 as libc::c_int;
                            if v1 == v2 {
                                let mut len_0: libc::c_int = (*p as libc::c_int
                                    & 0x3 as libc::c_int) + 1 as libc::c_int
                                    + ((*p.offset(1 as libc::c_int as isize) as libc::c_int
                                        & 0x3 as libc::c_int) + 1 as libc::c_int);
                                if len_0 <= 4 as libc::c_int {
                                    *p
                                        .offset(
                                            1 as libc::c_int as isize,
                                        ) = ((v1 - 1 as libc::c_int) << 2 as libc::c_int
                                        | len_0 - 1 as libc::c_int | 0x80 as libc::c_int)
                                        as uint8_t;
                                    memmove(
                                        p as *mut libc::c_void,
                                        p.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        end.offset_from(p) as libc::c_long as libc::c_ulong,
                                    );
                                    sdsIncrLen((*o).ptr as sds, -(1 as libc::c_int) as ssize_t);
                                    end = end.offset(-1);
                                    continue;
                                }
                            }
                        }
                        p = p.offset(1);
                    }
                }
                hdr = (*o).ptr as *mut hllhdr;
                (*hdr)
                    .card[7 as libc::c_int
                    as usize] = ((*hdr).card[7 as libc::c_int as usize] as libc::c_int
                    | (1 as libc::c_int) << 7 as libc::c_int) as uint8_t;
                return 1 as libc::c_int;
            }
        }
    }
    if hllSparseToDense(o) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    hdr = (*o).ptr as *mut hllhdr;
    let mut dense_retval: libc::c_int = hllDenseSet(
        ((*hdr).registers).as_mut_ptr(),
        index,
        count,
    );
    if dense_retval == 1 as libc::c_int {} else {
        _serverAssert(
            b"dense_retval == 1\0" as *const u8 as *const libc::c_char,
            b"hyperloglog.c\0" as *const u8 as *const libc::c_char,
            894 as libc::c_int,
        );
        unreachable!();
    };
    return dense_retval;
}
#[no_mangle]
pub unsafe extern "C" fn hllSparseAdd(
    mut o: *mut robj,
    mut ele: *mut libc::c_uchar,
    mut elesize: size_t,
) -> libc::c_int {
    let mut index: libc::c_long = 0;
    let mut count: uint8_t = hllPatLen(ele, elesize, &mut index) as uint8_t;
    return hllSparseSet(o, index, count);
}
#[no_mangle]
pub unsafe extern "C" fn hllSparseRegHisto(
    mut sparse: *mut uint8_t,
    mut sparselen: libc::c_int,
    mut invalid: *mut libc::c_int,
    mut reghisto: *mut libc::c_int,
) {
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut runlen: libc::c_int = 0;
    let mut regval: libc::c_int = 0;
    let mut end: *mut uint8_t = sparse.offset(sparselen as isize);
    let mut p: *mut uint8_t = sparse;
    while p < end {
        if *p as libc::c_int & 0xc0 as libc::c_int == 0 as libc::c_int {
            runlen = (*p as libc::c_int & 0x3f as libc::c_int) + 1 as libc::c_int;
            idx += runlen;
            *reghisto.offset(0 as libc::c_int as isize) += runlen;
            p = p.offset(1);
        } else if *p as libc::c_int & 0xc0 as libc::c_int == 0x40 as libc::c_int {
            runlen = ((*p as libc::c_int & 0x3f as libc::c_int) << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int)
                + 1 as libc::c_int;
            idx += runlen;
            *reghisto.offset(0 as libc::c_int as isize) += runlen;
            p = p.offset(2 as libc::c_int as isize);
        } else {
            runlen = (*p as libc::c_int & 0x3 as libc::c_int) + 1 as libc::c_int;
            regval = (*p as libc::c_int >> 2 as libc::c_int & 0x1f as libc::c_int)
                + 1 as libc::c_int;
            idx += runlen;
            *reghisto.offset(regval as isize) += runlen;
            p = p.offset(1);
        }
    }
    if idx != (1 as libc::c_int) << 14 as libc::c_int && !invalid.is_null() {
        *invalid = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn hllRawRegHisto(
    mut registers: *mut uint8_t,
    mut reghisto: *mut libc::c_int,
) {
    let mut word: *mut uint64_t = registers as *mut uint64_t;
    let mut bytes: *mut uint8_t = 0 as *mut uint8_t;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < ((1 as libc::c_int) << 14 as libc::c_int) / 8 as libc::c_int {
        if *word == 0 as libc::c_int as libc::c_ulong {
            *reghisto.offset(0 as libc::c_int as isize) += 8 as libc::c_int;
        } else {
            bytes = word as *mut uint8_t;
            let ref mut fresh27 = *reghisto
                .offset(*bytes.offset(0 as libc::c_int as isize) as isize);
            *fresh27 += 1;
            let ref mut fresh28 = *reghisto
                .offset(*bytes.offset(1 as libc::c_int as isize) as isize);
            *fresh28 += 1;
            let ref mut fresh29 = *reghisto
                .offset(*bytes.offset(2 as libc::c_int as isize) as isize);
            *fresh29 += 1;
            let ref mut fresh30 = *reghisto
                .offset(*bytes.offset(3 as libc::c_int as isize) as isize);
            *fresh30 += 1;
            let ref mut fresh31 = *reghisto
                .offset(*bytes.offset(4 as libc::c_int as isize) as isize);
            *fresh31 += 1;
            let ref mut fresh32 = *reghisto
                .offset(*bytes.offset(5 as libc::c_int as isize) as isize);
            *fresh32 += 1;
            let ref mut fresh33 = *reghisto
                .offset(*bytes.offset(6 as libc::c_int as isize) as isize);
            *fresh33 += 1;
            let ref mut fresh34 = *reghisto
                .offset(*bytes.offset(7 as libc::c_int as isize) as isize);
            *fresh34 += 1;
        }
        word = word.offset(1);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn hllSigma(mut x: libc::c_double) -> libc::c_double {
    if x == 1.0f64 {
        return core::f32::INFINITY as libc::c_double;
    }
    let mut zPrime: libc::c_double = 0.;
    let mut y: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut z: libc::c_double = x;
    loop {
        x *= x;
        zPrime = z;
        z += x * y;
        y += y;
        if !(zPrime != z) {
            break;
        }
    }
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn hllTau(mut x: libc::c_double) -> libc::c_double {
    if x == 0.0f64 || x == 1.0f64 {
        return 0.0f64;
    }
    let mut zPrime: libc::c_double = 0.;
    let mut y: libc::c_double = 1.0f64;
    let mut z: libc::c_double = 1 as libc::c_int as libc::c_double - x;
    loop {
        x = sqrt(x);
        zPrime = z;
        y *= 0.5f64;
        z
            -= pow(
                1 as libc::c_int as libc::c_double - x,
                2 as libc::c_int as libc::c_double,
            ) * y;
        if !(zPrime != z) {
            break;
        }
    }
    return z / 3 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn hllCount(
    mut hdr: *mut hllhdr,
    mut invalid: *mut libc::c_int,
) -> uint64_t {
    let mut m: libc::c_double = ((1 as libc::c_int) << 14 as libc::c_int)
        as libc::c_double;
    let mut E: libc::c_double = 0.;
    let mut j: libc::c_int = 0;
    let mut reghisto: [libc::c_int; 64] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if (*hdr).encoding as libc::c_int == 0 as libc::c_int {
        hllDenseRegHisto(((*hdr).registers).as_mut_ptr(), reghisto.as_mut_ptr());
    } else if (*hdr).encoding as libc::c_int == 1 as libc::c_int {
        hllSparseRegHisto(
            ((*hdr).registers).as_mut_ptr(),
            (sdslen(hdr as sds))
                .wrapping_sub(core::mem::size_of::<hllhdr>() as libc::c_ulong)
                as libc::c_int,
            invalid,
            reghisto.as_mut_ptr(),
        );
    } else if (*hdr).encoding as libc::c_int == 255 as libc::c_int {
        hllRawRegHisto(((*hdr).registers).as_mut_ptr(), reghisto.as_mut_ptr());
    } else {
        _serverPanic(
            b"hyperloglog.c\0" as *const u8 as *const libc::c_char,
            1034 as libc::c_int,
            b"Unknown HyperLogLog encoding in hllCount()\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    let mut z: libc::c_double = m
        * hllTau(
            (m
                - reghisto[(64 as libc::c_int - 14 as libc::c_int + 1 as libc::c_int)
                    as usize] as libc::c_double) / m,
        );
    j = 64 as libc::c_int - 14 as libc::c_int;
    while j >= 1 as libc::c_int {
        z += reghisto[j as usize] as libc::c_double;
        z *= 0.5f64;
        j -= 1;
    }
    z += m * hllSigma(reghisto[0 as libc::c_int as usize] as libc::c_double / m);
    E = llroundl((0.721347520444481703680f64 * m * m / z) as f64)
        as libc::c_double;
    return E as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn hllAdd(
    mut o: *mut robj,
    mut ele: *mut libc::c_uchar,
    mut elesize: size_t,
) -> libc::c_int {
    let mut hdr: *mut hllhdr = (*o).ptr as *mut hllhdr;
    match (*hdr).encoding as libc::c_int {
        0 => return hllDenseAdd(((*hdr).registers).as_mut_ptr(), ele, elesize),
        1 => return hllSparseAdd(o, ele, elesize),
        _ => return -(1 as libc::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn hllMerge(
    mut max: *mut uint8_t,
    mut hll: *mut robj,
) -> libc::c_int {
    let mut hdr: *mut hllhdr = (*hll).ptr as *mut hllhdr;
    let mut i: libc::c_int = 0;
    if (*hdr).encoding as libc::c_int == 0 as libc::c_int {
        let mut val: uint8_t = 0;
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << 14 as libc::c_int {
            let mut _p: *mut uint8_t = ((*hdr).registers).as_mut_ptr();
            let mut _byte: libc::c_ulong = (i * 6 as libc::c_int / 8 as libc::c_int)
                as libc::c_ulong;
            let mut _fb: libc::c_ulong = (i * 6 as libc::c_int & 7 as libc::c_int)
                as libc::c_ulong;
            let mut _fb8: libc::c_ulong = (8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(_fb);
            let mut b0: libc::c_ulong = *_p.offset(_byte as isize) as libc::c_ulong;
            let mut b1: libc::c_ulong = *_p
                .offset(_byte.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_ulong;
            val = ((b0 >> _fb | b1 << _fb8)
                & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong) as uint8_t;
            if val as libc::c_int > *max.offset(i as isize) as libc::c_int {
                *max.offset(i as isize) = val;
            }
            i += 1;
        }
    } else {
        let mut p: *mut uint8_t = (*hll).ptr as *mut uint8_t;
        let mut end: *mut uint8_t = p.offset(sdslen((*hll).ptr as sds) as isize);
        let mut runlen: libc::c_long = 0;
        let mut regval: libc::c_long = 0;
        p = p.offset(core::mem::size_of::<hllhdr>() as libc::c_ulong as isize);
        i = 0 as libc::c_int;
        while p < end {
            if *p as libc::c_int & 0xc0 as libc::c_int == 0 as libc::c_int {
                runlen = ((*p as libc::c_int & 0x3f as libc::c_int) + 1 as libc::c_int)
                    as libc::c_long;
                i = (i as libc::c_long + runlen) as libc::c_int;
                p = p.offset(1);
            } else if *p as libc::c_int & 0xc0 as libc::c_int == 0x40 as libc::c_int {
                runlen = (((*p as libc::c_int & 0x3f as libc::c_int) << 8 as libc::c_int
                    | *p.offset(1 as libc::c_int as isize) as libc::c_int)
                    + 1 as libc::c_int) as libc::c_long;
                i = (i as libc::c_long + runlen) as libc::c_int;
                p = p.offset(2 as libc::c_int as isize);
            } else {
                runlen = ((*p as libc::c_int & 0x3 as libc::c_int) + 1 as libc::c_int)
                    as libc::c_long;
                regval = ((*p as libc::c_int >> 2 as libc::c_int & 0x1f as libc::c_int)
                    + 1 as libc::c_int) as libc::c_long;
                if runlen + i as libc::c_long
                    > ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_long
                {
                    break;
                }
                loop {
                    let fresh35 = runlen;
                    runlen = runlen - 1;
                    if !(fresh35 != 0) {
                        break;
                    }
                    if regval > *max.offset(i as isize) as libc::c_long {
                        *max.offset(i as isize) = regval as uint8_t;
                    }
                    i += 1;
                }
                p = p.offset(1);
            }
        }
        if i != (1 as libc::c_int) << 14 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn createHLLObject() -> *mut robj {
    let mut o: *mut robj = 0 as *mut robj;
    let mut hdr: *mut hllhdr = 0 as *mut hllhdr;
    let mut s: sds = 0 as *mut libc::c_char;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut sparselen: libc::c_int = (core::mem::size_of::<hllhdr>() as libc::c_ulong)
        .wrapping_add(
            ((((1 as libc::c_int) << 14 as libc::c_int)
                + (16384 as libc::c_int - 1 as libc::c_int)) / 16384 as libc::c_int
                * 2 as libc::c_int) as libc::c_ulong,
        ) as libc::c_int;
    let mut aux: libc::c_int = 0;
    aux = (1 as libc::c_int) << 14 as libc::c_int;
    s = sdsnewlen(0 as *const libc::c_void, sparselen as size_t);
    p = (s as *mut uint8_t)
        .offset(core::mem::size_of::<hllhdr>() as libc::c_ulong as isize);
    while aux != 0 {
        let mut xzero: libc::c_int = 16384 as libc::c_int;
        if xzero > aux {
            xzero = aux;
        }
        let mut _l: libc::c_int = xzero - 1 as libc::c_int;
        *p = (_l >> 8 as libc::c_int | 0x40 as libc::c_int) as uint8_t;
        *p.offset(1 as libc::c_int as isize) = (_l & 0xff as libc::c_int) as uint8_t;
        p = p.offset(2 as libc::c_int as isize);
        aux -= xzero;
    }
    if p.offset_from(s as *mut uint8_t) as libc::c_long == sparselen as libc::c_long
    {} else {
        _serverAssert(
            b"(p-(uint8_t*)s) == sparselen\0" as *const u8 as *const libc::c_char,
            b"hyperloglog.c\0" as *const u8 as *const libc::c_char,
            1137 as libc::c_int,
        );
        unreachable!();
    };
    o = createObject(0 as libc::c_int, s as *mut libc::c_void);
    hdr = (*o).ptr as *mut hllhdr;
    memcpy(
        ((*hdr).magic).as_mut_ptr() as *mut libc::c_void,
        b"HYLL\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    (*hdr).encoding = 1 as libc::c_int as uint8_t;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn isHLLObjectOrReply(
    mut c: *mut client,
    mut o: *mut robj,
) -> libc::c_int {
    let mut hdr: *mut hllhdr = 0 as *mut hllhdr;
    if checkType(c, o, 0 as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    if (*o).encoding() as libc::c_int == 0 as libc::c_int
        || (*o).encoding() as libc::c_int == 8 as libc::c_int
    {
        if !(stringObjectLen(o) < core::mem::size_of::<hllhdr>() as libc::c_ulong) {
            hdr = (*o).ptr as *mut hllhdr;
            if !((*hdr).magic[0 as libc::c_int as usize] as libc::c_int != 'H' as i32
                || (*hdr).magic[1 as libc::c_int as usize] as libc::c_int != 'Y' as i32
                || (*hdr).magic[2 as libc::c_int as usize] as libc::c_int != 'L' as i32
                || (*hdr).magic[3 as libc::c_int as usize] as libc::c_int != 'L' as i32)
            {
                if !((*hdr).encoding as libc::c_int > 1 as libc::c_int) {
                    if !((*hdr).encoding as libc::c_int == 0 as libc::c_int
                        && stringObjectLen(o)
                            != (core::mem::size_of::<hllhdr>() as libc::c_ulong)
                                .wrapping_add(
                                    ((((1 as libc::c_int) << 14 as libc::c_int)
                                        * 6 as libc::c_int + 7 as libc::c_int) / 8 as libc::c_int)
                                        as libc::c_ulong,
                                ))
                    {
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    addReplyError(
        c,
        b"-WRONGTYPE Key is not a valid HyperLogLog string value.\0" as *const u8
            as *const libc::c_char,
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pfaddCommand(mut c: *mut client) {
    let mut o: *mut robj = lookupKeyWrite(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    let mut hdr: *mut hllhdr = 0 as *mut hllhdr;
    let mut updated: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    if o.is_null() {
        o = createHLLObject();
        dbAdd((*c).db, *((*c).argv).offset(1 as libc::c_int as isize), o);
        updated += 1;
    } else {
        if isHLLObjectOrReply(c, o) != 0 as libc::c_int {
            return;
        }
        o = dbUnshareStringValue(
            (*c).db,
            *((*c).argv).offset(1 as libc::c_int as isize),
            o,
        );
    }
    j = 2 as libc::c_int;
    while j < (*c).argc {
        let mut retval: libc::c_int = hllAdd(
            o,
            (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_uchar,
            sdslen((**((*c).argv).offset(j as isize)).ptr as sds),
        );
        match retval {
            1 => {
                updated += 1;
            }
            -1 => {
                addReplyError(c, invalid_hll_err);
                return;
            }
            _ => {}
        }
        j += 1;
    }
    hdr = (*o).ptr as *mut hllhdr;
    if updated != 0 {
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 3 as libc::c_int,
            b"pfadd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        server.dirty += updated as libc::c_longlong;
        (*hdr)
            .card[7 as libc::c_int
            as usize] = ((*hdr).card[7 as libc::c_int as usize] as libc::c_int
            | (1 as libc::c_int) << 7 as libc::c_int) as uint8_t;
    }
    addReply(c, if updated != 0 { shared.cone } else { shared.czero });
}
#[no_mangle]
pub unsafe extern "C" fn pfcountCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut hdr: *mut hllhdr = 0 as *mut hllhdr;
    let mut card: uint64_t = 0;
    if (*c).argc > 2 as libc::c_int {
        let mut max: [uint8_t; 16400] = [0; 16400];
        let mut registers: *mut uint8_t = 0 as *mut uint8_t;
        let mut j: libc::c_int = 0;
        memset(
            max.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            core::mem::size_of::<[uint8_t; 16400]>() as libc::c_ulong,
        );
        hdr = max.as_mut_ptr() as *mut hllhdr;
        (*hdr).encoding = 255 as libc::c_int as uint8_t;
        registers = max
            .as_mut_ptr()
            .offset(core::mem::size_of::<hllhdr>() as libc::c_ulong as isize);
        j = 1 as libc::c_int;
        while j < (*c).argc {
            let mut o_0: *mut robj = lookupKeyRead(
                (*c).db,
                *((*c).argv).offset(j as isize),
            );
            if !o_0.is_null() {
                if isHLLObjectOrReply(c, o_0) != 0 as libc::c_int {
                    return;
                }
                if hllMerge(registers, o_0) == -(1 as libc::c_int) {
                    addReplyError(c, invalid_hll_err);
                    return;
                }
            }
            j += 1;
        }
        addReplyLongLong(c, hllCount(hdr, 0 as *mut libc::c_int) as libc::c_longlong);
        return;
    }
    o = lookupKeyRead((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        addReply(c, shared.czero);
    } else {
        if isHLLObjectOrReply(c, o) != 0 as libc::c_int {
            return;
        }
        o = dbUnshareStringValue(
            (*c).db,
            *((*c).argv).offset(1 as libc::c_int as isize),
            o,
        );
        hdr = (*o).ptr as *mut hllhdr;
        if (*hdr).card[7 as libc::c_int as usize] as libc::c_int
            & (1 as libc::c_int) << 7 as libc::c_int == 0 as libc::c_int
        {
            card = (*hdr).card[0 as libc::c_int as usize] as uint64_t;
            card
                |= ((*hdr).card[1 as libc::c_int as usize] as uint64_t)
                    << 8 as libc::c_int;
            card
                |= ((*hdr).card[2 as libc::c_int as usize] as uint64_t)
                    << 16 as libc::c_int;
            card
                |= ((*hdr).card[3 as libc::c_int as usize] as uint64_t)
                    << 24 as libc::c_int;
            card
                |= ((*hdr).card[4 as libc::c_int as usize] as uint64_t)
                    << 32 as libc::c_int;
            card
                |= ((*hdr).card[5 as libc::c_int as usize] as uint64_t)
                    << 40 as libc::c_int;
            card
                |= ((*hdr).card[6 as libc::c_int as usize] as uint64_t)
                    << 48 as libc::c_int;
            card
                |= ((*hdr).card[7 as libc::c_int as usize] as uint64_t)
                    << 56 as libc::c_int;
        } else {
            let mut invalid: libc::c_int = 0 as libc::c_int;
            card = hllCount(hdr, &mut invalid);
            if invalid != 0 {
                addReplyError(c, invalid_hll_err);
                return;
            }
            (*hdr)
                .card[0 as libc::c_int
                as usize] = (card & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            (*hdr)
                .card[1 as libc::c_int
                as usize] = (card >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            (*hdr)
                .card[2 as libc::c_int
                as usize] = (card >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            (*hdr)
                .card[3 as libc::c_int
                as usize] = (card >> 24 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            (*hdr)
                .card[4 as libc::c_int
                as usize] = (card >> 32 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            (*hdr)
                .card[5 as libc::c_int
                as usize] = (card >> 40 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            (*hdr)
                .card[6 as libc::c_int
                as usize] = (card >> 48 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            (*hdr)
                .card[7 as libc::c_int
                as usize] = (card >> 56 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
            signalModifiedKey(
                c,
                (*c).db,
                *((*c).argv).offset(1 as libc::c_int as isize),
            );
            server.dirty += 1;
        }
        addReplyLongLong(c, card as libc::c_longlong);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pfmergeCommand(mut c: *mut client) {
    let mut max: [uint8_t; 16384] = [0; 16384];
    let mut hdr: *mut hllhdr = 0 as *mut hllhdr;
    let mut j: libc::c_int = 0;
    let mut use_dense: libc::c_int = 0 as libc::c_int;
    memset(
        max.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[uint8_t; 16384]>() as libc::c_ulong,
    );
    j = 1 as libc::c_int;
    while j < (*c).argc {
        let mut o: *mut robj = lookupKeyRead((*c).db, *((*c).argv).offset(j as isize));
        if !o.is_null() {
            if isHLLObjectOrReply(c, o) != 0 as libc::c_int {
                return;
            }
            hdr = (*o).ptr as *mut hllhdr;
            if (*hdr).encoding as libc::c_int == 0 as libc::c_int {
                use_dense = 1 as libc::c_int;
            }
            if hllMerge(max.as_mut_ptr(), o) == -(1 as libc::c_int) {
                addReplyError(c, invalid_hll_err);
                return;
            }
        }
        j += 1;
    }
    let mut o_0: *mut robj = lookupKeyWrite(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    if o_0.is_null() {
        o_0 = createHLLObject();
        dbAdd((*c).db, *((*c).argv).offset(1 as libc::c_int as isize), o_0);
    } else {
        o_0 = dbUnshareStringValue(
            (*c).db,
            *((*c).argv).offset(1 as libc::c_int as isize),
            o_0,
        );
    }
    if use_dense != 0 && hllSparseToDense(o_0) == -(1 as libc::c_int) {
        addReplyError(c, invalid_hll_err);
        return;
    }
    j = 0 as libc::c_int;
    while j < (1 as libc::c_int) << 14 as libc::c_int {
        if !(max[j as usize] as libc::c_int == 0 as libc::c_int) {
            hdr = (*o_0).ptr as *mut hllhdr;
            match (*hdr).encoding as libc::c_int {
                0 => {
                    hllDenseSet(
                        ((*hdr).registers).as_mut_ptr(),
                        j as libc::c_long,
                        max[j as usize],
                    );
                }
                1 => {
                    hllSparseSet(o_0, j as libc::c_long, max[j as usize]);
                }
                _ => {}
            }
        }
        j += 1;
    }
    hdr = (*o_0).ptr as *mut hllhdr;
    (*hdr)
        .card[7 as libc::c_int
        as usize] = ((*hdr).card[7 as libc::c_int as usize] as libc::c_int
        | (1 as libc::c_int) << 7 as libc::c_int) as uint8_t;
    signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 3 as libc::c_int,
        b"pfadd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    server.dirty += 1;
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn pfselftestCommand(mut c: *mut client) {
    let mut relerr: libc::c_double = 0.;
    let mut checkpoint: int64_t = 0;
    let mut seed: uint64_t = 0;
    let mut ele: uint64_t = 0;
    let mut current_block: u64;
    let mut j: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut bitcounters: sds = sdsnewlen(
        0 as *const libc::c_void,
        (core::mem::size_of::<hllhdr>() as libc::c_ulong)
            .wrapping_add(
                ((((1 as libc::c_int) << 14 as libc::c_int) * 6 as libc::c_int
                    + 7 as libc::c_int) / 8 as libc::c_int) as libc::c_ulong,
            ),
    );
    let mut hdr: *mut hllhdr = bitcounters as *mut hllhdr;
    let mut hdr2: *mut hllhdr = 0 as *mut hllhdr;
    let mut o: *mut robj = 0 as *mut robj;
    let mut bytecounters: [uint8_t; 16384] = [0; 16384];
    j = 0 as libc::c_int as libc::c_uint;
    's_12: loop {
        if !(j < 1000 as libc::c_int as libc::c_uint) {
            current_block = 2719512138335094285;
            break;
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint {
            let mut r: libc::c_uint = (rand()
                & ((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint;
            bytecounters[i as usize] = r as uint8_t;
            let mut _p: *mut uint8_t = ((*hdr).registers).as_mut_ptr();
            let mut _byte: libc::c_ulong = i
                .wrapping_mul(6 as libc::c_int as libc::c_uint)
                .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong;
            let mut _fb: libc::c_ulong = (i
                .wrapping_mul(6 as libc::c_int as libc::c_uint)
                & 7 as libc::c_int as libc::c_uint) as libc::c_ulong;
            let mut _fb8: libc::c_ulong = (8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(_fb);
            let mut _v: libc::c_ulong = r as libc::c_ulong;
            let ref mut fresh36 = *_p.offset(_byte as isize);
            *fresh36 = (*fresh36 as libc::c_int
                & !((((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                    << _fb)) as uint8_t;
            let ref mut fresh37 = *_p.offset(_byte as isize);
            *fresh37 = (*fresh37 as libc::c_ulong | _v << _fb) as uint8_t;
            let ref mut fresh38 = *_p
                .offset(_byte.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *fresh38 = (*fresh38 as libc::c_int
                & !(((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int >> _fb8))
                as uint8_t;
            let ref mut fresh39 = *_p
                .offset(_byte.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *fresh39 = (*fresh39 as libc::c_ulong | _v >> _fb8) as uint8_t;
            i = i.wrapping_add(1);
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint {
            let mut val: libc::c_uint = 0;
            let mut _p_0: *mut uint8_t = ((*hdr).registers).as_mut_ptr();
            let mut _byte_0: libc::c_ulong = i
                .wrapping_mul(6 as libc::c_int as libc::c_uint)
                .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong;
            let mut _fb_0: libc::c_ulong = (i
                .wrapping_mul(6 as libc::c_int as libc::c_uint)
                & 7 as libc::c_int as libc::c_uint) as libc::c_ulong;
            let mut _fb8_0: libc::c_ulong = (8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(_fb_0);
            let mut b0: libc::c_ulong = *_p_0.offset(_byte_0 as isize) as libc::c_ulong;
            let mut b1: libc::c_ulong = *_p_0
                .offset(_byte_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_ulong;
            val = ((b0 >> _fb_0 | b1 << _fb8_0)
                & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong) as libc::c_uint;
            if val != bytecounters[i as usize] as libc::c_uint {
                addReplyErrorFormat(
                    c,
                    b"TESTFAILED Register %d should be %d but is %d\0" as *const u8
                        as *const libc::c_char,
                    i,
                    bytecounters[i as usize] as libc::c_int,
                    val as libc::c_int,
                );
                current_block = 4808091333740361217;
                break 's_12;
            } else {
                i = i.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
    match current_block {
        2719512138335094285 => {
            memset(
                ((*hdr).registers).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (core::mem::size_of::<hllhdr>() as libc::c_ulong)
                    .wrapping_add(
                        ((((1 as libc::c_int) << 14 as libc::c_int) * 6 as libc::c_int
                            + 7 as libc::c_int) / 8 as libc::c_int) as libc::c_ulong,
                    )
                    .wrapping_sub(core::mem::size_of::<hllhdr>() as libc::c_ulong),
            );
            o = createHLLObject();
            relerr = 1.04f64
                / sqrt(((1 as libc::c_int) << 14 as libc::c_int) as libc::c_double);
            checkpoint = 1 as libc::c_int as int64_t;
            seed = rand() as uint64_t | (rand() as uint64_t) << 32 as libc::c_int;
            ele = 0;
            j = 1 as libc::c_int as libc::c_uint;
            loop {
                if !(j <= 10000000 as libc::c_int as libc::c_uint) {
                    current_block = 12381812505308290051;
                    break;
                }
                ele = j as libc::c_ulong ^ seed;
                hllDenseAdd(
                    ((*hdr).registers).as_mut_ptr(),
                    &mut ele as *mut uint64_t as *mut libc::c_uchar,
                    core::mem::size_of::<uint64_t>() as libc::c_ulong,
                );
                hllAdd(
                    o,
                    &mut ele as *mut uint64_t as *mut libc::c_uchar,
                    core::mem::size_of::<uint64_t>() as libc::c_ulong,
                );
                if j as libc::c_long == checkpoint
                    && (j as libc::c_ulong)
                        < (server.hll_sparse_max_bytes)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                {
                    hdr2 = (*o).ptr as *mut hllhdr;
                    if (*hdr2).encoding as libc::c_int != 1 as libc::c_int {
                        addReplyError(
                            c,
                            b"TESTFAILED sparse encoding not used\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block = 4808091333740361217;
                        break;
                    }
                }
                if j as libc::c_long == checkpoint
                    && hllCount(hdr, 0 as *mut libc::c_int)
                        != hllCount((*o).ptr as *mut hllhdr, 0 as *mut libc::c_int)
                {
                    addReplyError(
                        c,
                        b"TESTFAILED dense/sparse disagree\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block = 4808091333740361217;
                    break;
                } else {
                    if j as libc::c_long == checkpoint {
                        let mut abserr: int64_t = checkpoint
                            - hllCount(hdr, 0 as *mut libc::c_int) as int64_t;
                        let mut maxerr: uint64_t = ceil(
                            relerr * 6 as libc::c_int as libc::c_double
                                * checkpoint as libc::c_double,
                        ) as uint64_t;
                        if j == 10 as libc::c_int as libc::c_uint {
                            maxerr = 1 as libc::c_int as uint64_t;
                        }
                        if abserr < 0 as libc::c_int as libc::c_long {
                            abserr = -abserr;
                        }
                        if abserr > maxerr as int64_t {
                            addReplyErrorFormat(
                                c,
                                b"TESTFAILED Too big error. card:%llu abserr:%llu\0"
                                    as *const u8 as *const libc::c_char,
                                checkpoint as libc::c_ulonglong,
                                abserr as libc::c_ulonglong,
                            );
                            current_block = 4808091333740361217;
                            break;
                        } else {
                            checkpoint *= 10 as libc::c_int as libc::c_long;
                        }
                    }
                    j = j.wrapping_add(1);
                }
            }
            match current_block {
                4808091333740361217 => {}
                _ => {
                    addReply(c, shared.ok);
                }
            }
        }
        _ => {}
    }
    sdsfree(bitcounters);
    if !o.is_null() {
        decrRefCount(o);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pfdebugCommand(mut c: *mut client) {
    let mut current_block: u64;
    let mut cmd: *mut libc::c_char = (**((*c).argv).offset(1 as libc::c_int as isize))
        .ptr as *mut libc::c_char;
    let mut hdr: *mut hllhdr = 0 as *mut hllhdr;
    let mut o: *mut robj = 0 as *mut robj;
    let mut j: libc::c_int = 0;
    o = lookupKeyWrite((*c).db, *((*c).argv).offset(2 as libc::c_int as isize));
    if o.is_null() {
        addReplyError(
            c,
            b"The specified key does not exist\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if isHLLObjectOrReply(c, o) != 0 as libc::c_int {
        return;
    }
    o = dbUnshareStringValue((*c).db, *((*c).argv).offset(2 as libc::c_int as isize), o);
    hdr = (*o).ptr as *mut hllhdr;
    if strcasecmp(cmd, b"getreg\0" as *const u8 as *const libc::c_char) == 0 {
        if (*c).argc != 3 as libc::c_int {
            current_block = 7351741740727689750;
        } else {
            if (*hdr).encoding as libc::c_int == 1 as libc::c_int {
                if hllSparseToDense(o) == -(1 as libc::c_int) {
                    addReplyError(c, invalid_hll_err);
                    return;
                }
                server.dirty += 1;
            }
            hdr = (*o).ptr as *mut hllhdr;
            addReplyArrayLen(
                c,
                ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_long,
            );
            j = 0 as libc::c_int;
            while j < (1 as libc::c_int) << 14 as libc::c_int {
                let mut val: uint8_t = 0;
                let mut _p: *mut uint8_t = ((*hdr).registers).as_mut_ptr();
                let mut _byte: libc::c_ulong = (j * 6 as libc::c_int / 8 as libc::c_int)
                    as libc::c_ulong;
                let mut _fb: libc::c_ulong = (j * 6 as libc::c_int & 7 as libc::c_int)
                    as libc::c_ulong;
                let mut _fb8: libc::c_ulong = (8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(_fb);
                let mut b0: libc::c_ulong = *_p.offset(_byte as isize) as libc::c_ulong;
                let mut b1: libc::c_ulong = *_p
                    .offset(
                        _byte.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_ulong;
                val = ((b0 >> _fb | b1 << _fb8)
                    & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_ulong) as uint8_t;
                addReplyLongLong(c, val as libc::c_longlong);
                j += 1;
            }
            current_block = 16203797167131938757;
        }
    } else if strcasecmp(cmd, b"decode\0" as *const u8 as *const libc::c_char) == 0 {
        if (*c).argc != 3 as libc::c_int {
            current_block = 7351741740727689750;
        } else {
            let mut p: *mut uint8_t = (*o).ptr as *mut uint8_t;
            let mut end: *mut uint8_t = p.offset(sdslen((*o).ptr as sds) as isize);
            let mut decoded: sds = sdsempty();
            if (*hdr).encoding as libc::c_int != 1 as libc::c_int {
                sdsfree(decoded);
                addReplyError(
                    c,
                    b"HLL encoding is not sparse\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            p = p.offset(core::mem::size_of::<hllhdr>() as libc::c_ulong as isize);
            while p < end {
                let mut runlen: libc::c_int = 0;
                let mut regval: libc::c_int = 0;
                if *p as libc::c_int & 0xc0 as libc::c_int == 0 as libc::c_int {
                    runlen = (*p as libc::c_int & 0x3f as libc::c_int)
                        + 1 as libc::c_int;
                    p = p.offset(1);
                    decoded = sdscatprintf(
                        decoded,
                        b"z:%d \0" as *const u8 as *const libc::c_char,
                        runlen,
                    );
                } else if *p as libc::c_int & 0xc0 as libc::c_int == 0x40 as libc::c_int
                {
                    runlen = ((*p as libc::c_int & 0x3f as libc::c_int)
                        << 8 as libc::c_int
                        | *p.offset(1 as libc::c_int as isize) as libc::c_int)
                        + 1 as libc::c_int;
                    p = p.offset(2 as libc::c_int as isize);
                    decoded = sdscatprintf(
                        decoded,
                        b"Z:%d \0" as *const u8 as *const libc::c_char,
                        runlen,
                    );
                } else {
                    runlen = (*p as libc::c_int & 0x3 as libc::c_int) + 1 as libc::c_int;
                    regval = (*p as libc::c_int >> 2 as libc::c_int
                        & 0x1f as libc::c_int) + 1 as libc::c_int;
                    p = p.offset(1);
                    decoded = sdscatprintf(
                        decoded,
                        b"v:%d,%d \0" as *const u8 as *const libc::c_char,
                        regval,
                        runlen,
                    );
                }
            }
            decoded = sdstrim(decoded, b" \0" as *const u8 as *const libc::c_char);
            addReplyBulkCBuffer(c, decoded as *const libc::c_void, sdslen(decoded));
            sdsfree(decoded);
            current_block = 16203797167131938757;
        }
    } else if strcasecmp(cmd, b"encoding\0" as *const u8 as *const libc::c_char) == 0 {
        let mut encodingstr: [*mut libc::c_char; 2] = [
            b"dense\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"sparse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ];
        if (*c).argc != 3 as libc::c_int {
            current_block = 7351741740727689750;
        } else {
            addReplyStatus(c, encodingstr[(*hdr).encoding as usize]);
            current_block = 16203797167131938757;
        }
    } else if strcasecmp(cmd, b"todense\0" as *const u8 as *const libc::c_char) == 0 {
        let mut conv: libc::c_int = 0 as libc::c_int;
        if (*c).argc != 3 as libc::c_int {
            current_block = 7351741740727689750;
        } else {
            if (*hdr).encoding as libc::c_int == 1 as libc::c_int {
                if hllSparseToDense(o) == -(1 as libc::c_int) {
                    addReplyError(c, invalid_hll_err);
                    return;
                }
                conv = 1 as libc::c_int;
                server.dirty += 1;
            }
            addReply(c, if conv != 0 { shared.cone } else { shared.czero });
            current_block = 16203797167131938757;
        }
    } else {
        addReplyErrorFormat(
            c,
            b"Unknown PFDEBUG subcommand '%s'\0" as *const u8 as *const libc::c_char,
            cmd,
        );
        current_block = 16203797167131938757;
    }
    match current_block {
        16203797167131938757 => return,
        _ => {
            addReplyErrorFormat(
                c,
                b"Wrong number of arguments for the '%s' subcommand\0" as *const u8
                    as *const libc::c_char,
                cmd,
            );
            return;
        }
    };
}
