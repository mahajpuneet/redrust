extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsgrowzero(s: sds, len: size_t) -> sds;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
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
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn addReplyNull(c: *mut client);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn getLongLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_longlong,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn checkType(c: *mut client, o: *mut robj, type_0: libc::c_int) -> libc::c_int;
    fn getLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn mustObeyClient(c: *mut client) -> libc::c_int;
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn lookupKeyRead(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyWrite(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyReadOrReply(
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
    fn dbDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn dbUnshareStringValue(db: *mut redisDb, key: *mut robj, o: *mut robj) -> *mut robj;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn decrRefCount(o: *mut robj);
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
pub struct bitfieldOp {
    pub offset: uint64_t,
    pub i64_0: int64_t,
    pub opcode: libc::c_int,
    pub owtype: libc::c_int,
    pub bits: libc::c_int,
    pub sign: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub u: uint64_t,
    pub i: int64_t,
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
pub unsafe extern "C" fn redisPopcount(
    mut s: *mut libc::c_void,
    mut count: libc::c_long,
) -> libc::c_longlong {
    let mut bits: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut p: *mut libc::c_uchar = s as *mut libc::c_uchar;
    let mut p4: *mut uint32_t = 0 as *mut uint32_t;
    static mut bitsinbyte: [libc::c_uchar; 256] = [
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
    ];
    while p as libc::c_ulong & 3 as libc::c_int as libc::c_ulong != 0 && count != 0 {
        let fresh0 = p;
        p = p.offset(1);
        bits += bitsinbyte[*fresh0 as usize] as libc::c_longlong;
        count -= 1;
    }
    p4 = p as *mut uint32_t;
    while count >= 28 as libc::c_int as libc::c_long {
        let mut aux1: uint32_t = 0;
        let mut aux2: uint32_t = 0;
        let mut aux3: uint32_t = 0;
        let mut aux4: uint32_t = 0;
        let mut aux5: uint32_t = 0;
        let mut aux6: uint32_t = 0;
        let mut aux7: uint32_t = 0;
        let fresh1 = p4;
        p4 = p4.offset(1);
        aux1 = *fresh1;
        let fresh2 = p4;
        p4 = p4.offset(1);
        aux2 = *fresh2;
        let fresh3 = p4;
        p4 = p4.offset(1);
        aux3 = *fresh3;
        let fresh4 = p4;
        p4 = p4.offset(1);
        aux4 = *fresh4;
        let fresh5 = p4;
        p4 = p4.offset(1);
        aux5 = *fresh5;
        let fresh6 = p4;
        p4 = p4.offset(1);
        aux6 = *fresh6;
        let fresh7 = p4;
        p4 = p4.offset(1);
        aux7 = *fresh7;
        count -= 28 as libc::c_int as libc::c_long;
        aux1 = aux1
            .wrapping_sub(
                aux1 >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint,
            );
        aux1 = (aux1 & 0x33333333 as libc::c_int as libc::c_uint)
            .wrapping_add(
                aux1 >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint,
            );
        aux2 = aux2
            .wrapping_sub(
                aux2 >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint,
            );
        aux2 = (aux2 & 0x33333333 as libc::c_int as libc::c_uint)
            .wrapping_add(
                aux2 >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint,
            );
        aux3 = aux3
            .wrapping_sub(
                aux3 >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint,
            );
        aux3 = (aux3 & 0x33333333 as libc::c_int as libc::c_uint)
            .wrapping_add(
                aux3 >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint,
            );
        aux4 = aux4
            .wrapping_sub(
                aux4 >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint,
            );
        aux4 = (aux4 & 0x33333333 as libc::c_int as libc::c_uint)
            .wrapping_add(
                aux4 >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint,
            );
        aux5 = aux5
            .wrapping_sub(
                aux5 >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint,
            );
        aux5 = (aux5 & 0x33333333 as libc::c_int as libc::c_uint)
            .wrapping_add(
                aux5 >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint,
            );
        aux6 = aux6
            .wrapping_sub(
                aux6 >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint,
            );
        aux6 = (aux6 & 0x33333333 as libc::c_int as libc::c_uint)
            .wrapping_add(
                aux6 >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint,
            );
        aux7 = aux7
            .wrapping_sub(
                aux7 >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint,
            );
        aux7 = (aux7 & 0x33333333 as libc::c_int as libc::c_uint)
            .wrapping_add(
                aux7 >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint,
            );
        bits
            += ((aux1.wrapping_add(aux1 >> 4 as libc::c_int)
                & 0xf0f0f0f as libc::c_int as libc::c_uint)
                .wrapping_add(
                    aux2.wrapping_add(aux2 >> 4 as libc::c_int)
                        & 0xf0f0f0f as libc::c_int as libc::c_uint,
                )
                .wrapping_add(
                    aux3.wrapping_add(aux3 >> 4 as libc::c_int)
                        & 0xf0f0f0f as libc::c_int as libc::c_uint,
                )
                .wrapping_add(
                    aux4.wrapping_add(aux4 >> 4 as libc::c_int)
                        & 0xf0f0f0f as libc::c_int as libc::c_uint,
                )
                .wrapping_add(
                    aux5.wrapping_add(aux5 >> 4 as libc::c_int)
                        & 0xf0f0f0f as libc::c_int as libc::c_uint,
                )
                .wrapping_add(
                    aux6.wrapping_add(aux6 >> 4 as libc::c_int)
                        & 0xf0f0f0f as libc::c_int as libc::c_uint,
                )
                .wrapping_add(
                    aux7.wrapping_add(aux7 >> 4 as libc::c_int)
                        & 0xf0f0f0f as libc::c_int as libc::c_uint,
                )
                .wrapping_mul(0x1010101 as libc::c_int as libc::c_uint)
                >> 24 as libc::c_int) as libc::c_longlong;
    }
    p = p4 as *mut libc::c_uchar;
    loop {
        let fresh8 = count;
        count = count - 1;
        if !(fresh8 != 0) {
            break;
        }
        let fresh9 = p;
        p = p.offset(1);
        bits += bitsinbyte[*fresh9 as usize] as libc::c_longlong;
    }
    return bits;
}
#[no_mangle]
pub unsafe extern "C" fn redisBitpos(
    mut s: *mut libc::c_void,
    mut count: libc::c_ulong,
    mut bit: libc::c_int,
) -> libc::c_longlong {
    let mut l: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut skipval: libc::c_ulong = 0;
    let mut word: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut one: libc::c_ulong = 0;
    let mut pos: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut j: libc::c_ulong = 0;
    let mut found: libc::c_int = 0;
    skipval = (if bit != 0 {
        0 as libc::c_int
    } else {
        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
    }) as libc::c_ulong;
    c = s as *mut libc::c_uchar;
    found = 0 as libc::c_int;
    while c as libc::c_ulong
        & (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0 && count != 0
    {
        if *c as libc::c_ulong != skipval {
            found = 1 as libc::c_int;
            break;
        } else {
            c = c.offset(1);
            count = count.wrapping_sub(1);
            pos += 8 as libc::c_int as libc::c_longlong;
        }
    }
    l = c as *mut libc::c_ulong;
    if found == 0 {
        skipval = if bit != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
        };
        while count >= core::mem::size_of::<libc::c_ulong>() as libc::c_ulong {
            if *l != skipval {
                break;
            }
            l = l.offset(1);
            count = count
                .wrapping_sub(core::mem::size_of::<libc::c_ulong>() as libc::c_ulong);
            pos = (pos as libc::c_ulonglong)
                .wrapping_add(
                    (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        as libc::c_ulonglong,
                ) as libc::c_longlong as libc::c_longlong;
        }
    }
    c = l as *mut libc::c_uchar;
    j = 0 as libc::c_int as libc::c_ulong;
    while j < core::mem::size_of::<libc::c_ulong>() as libc::c_ulong {
        word <<= 8 as libc::c_int;
        if count != 0 {
            word |= *c as libc::c_ulong;
            c = c.offset(1);
            count = count.wrapping_sub(1);
        }
        j = j.wrapping_add(1);
    }
    if bit == 1 as libc::c_int && word == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    one = (9223372036854775807 as libc::c_long as libc::c_ulong)
        .wrapping_mul(2 as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong);
    one >>= 1 as libc::c_int;
    one = !one;
    while one != 0 {
        if (one & word != 0 as libc::c_int as libc::c_ulong) as libc::c_int == bit {
            return pos;
        }
        pos += 1;
        one >>= 1 as libc::c_int;
    }
    _serverPanic(
        b"bitops.c\0" as *const u8 as *const libc::c_char,
        184 as libc::c_int,
        b"End of redisBitpos() reached.\0" as *const u8 as *const libc::c_char,
    );
    unreachable!();
    return 0 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn setUnsignedBitfield(
    mut p: *mut libc::c_uchar,
    mut offset: uint64_t,
    mut bits: uint64_t,
    mut value: uint64_t,
) {
    let mut byte: uint64_t = 0;
    let mut bit: uint64_t = 0;
    let mut byteval: uint64_t = 0;
    let mut bitval: uint64_t = 0;
    let mut j: uint64_t = 0;
    j = 0 as libc::c_int as uint64_t;
    while j < bits {
        bitval = (value
            & (1 as libc::c_int as uint64_t)
                << bits.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_sub(j)
            != 0 as libc::c_int as libc::c_ulong) as libc::c_int as uint64_t;
        byte = offset >> 3 as libc::c_int;
        bit = (7 as libc::c_int as libc::c_ulong)
            .wrapping_sub(offset & 0x7 as libc::c_int as libc::c_ulong);
        byteval = *p.offset(byte as isize) as uint64_t;
        byteval &= !((1 as libc::c_int) << bit) as libc::c_ulong;
        byteval |= bitval << bit;
        *p
            .offset(
                byte as isize,
            ) = (byteval & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        offset = offset.wrapping_add(1);
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn setSignedBitfield(
    mut p: *mut libc::c_uchar,
    mut offset: uint64_t,
    mut bits: uint64_t,
    mut value: int64_t,
) {
    let mut uv: uint64_t = value as uint64_t;
    setUnsignedBitfield(p, offset, bits, uv);
}
#[no_mangle]
pub unsafe extern "C" fn getUnsignedBitfield(
    mut p: *mut libc::c_uchar,
    mut offset: uint64_t,
    mut bits: uint64_t,
) -> uint64_t {
    let mut byte: uint64_t = 0;
    let mut bit: uint64_t = 0;
    let mut byteval: uint64_t = 0;
    let mut bitval: uint64_t = 0;
    let mut j: uint64_t = 0;
    let mut value: uint64_t = 0 as libc::c_int as uint64_t;
    j = 0 as libc::c_int as uint64_t;
    while j < bits {
        byte = offset >> 3 as libc::c_int;
        bit = (7 as libc::c_int as libc::c_ulong)
            .wrapping_sub(offset & 0x7 as libc::c_int as libc::c_ulong);
        byteval = *p.offset(byte as isize) as uint64_t;
        bitval = byteval >> bit & 1 as libc::c_int as libc::c_ulong;
        value = value << 1 as libc::c_int | bitval;
        offset = offset.wrapping_add(1);
        j = j.wrapping_add(1);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn getSignedBitfield(
    mut p: *mut libc::c_uchar,
    mut offset: uint64_t,
    mut bits: uint64_t,
) -> int64_t {
    let mut value: int64_t = 0;
    let mut conv: C2RustUnnamed_7 = C2RustUnnamed_7 { u: 0 };
    conv.u = getUnsignedBitfield(p, offset, bits);
    value = conv.i;
    if bits < 64 as libc::c_int as libc::c_ulong
        && value as libc::c_ulong
            & (1 as libc::c_int as uint64_t)
                << bits.wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        value = (value as libc::c_ulong | (-(1 as libc::c_int) as uint64_t) << bits)
            as int64_t;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn checkUnsignedBitfieldOverflow(
    mut value: uint64_t,
    mut incr: int64_t,
    mut bits: uint64_t,
    mut owtype: libc::c_int,
    mut limit: *mut uint64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut max: uint64_t = if bits == 64 as libc::c_int as libc::c_ulong {
        18446744073709551615 as libc::c_ulong
    } else {
        ((1 as libc::c_int as uint64_t) << bits)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    };
    let mut maxincr: int64_t = max.wrapping_sub(value) as int64_t;
    let mut minincr: int64_t = value.wrapping_neg() as int64_t;
    if value > max || incr > 0 as libc::c_int as libc::c_long && incr > maxincr {
        if !limit.is_null() {
            if owtype == 0 as libc::c_int {
                current_block = 1855159724523092465;
            } else {
                if owtype == 1 as libc::c_int {
                    *limit = max;
                }
                current_block = 11006700562992250127;
            }
        } else {
            current_block = 11006700562992250127;
        }
        match current_block {
            1855159724523092465 => {}
            _ => return 1 as libc::c_int,
        }
    } else if incr < 0 as libc::c_int as libc::c_long && incr < minincr {
        if !limit.is_null() {
            if owtype == 0 as libc::c_int {
                current_block = 1855159724523092465;
            } else {
                if owtype == 1 as libc::c_int {
                    *limit = 0 as libc::c_int as uint64_t;
                }
                current_block = 4166486009154926805;
            }
        } else {
            current_block = 4166486009154926805;
        }
        match current_block {
            1855159724523092465 => {}
            _ => return -(1 as libc::c_int),
        }
    } else {
        return 0 as libc::c_int
    }
    let mut mask: uint64_t = (-(1 as libc::c_int) as uint64_t) << bits;
    let mut res: uint64_t = value.wrapping_add(incr as libc::c_ulong);
    res &= !mask;
    *limit = res;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkSignedBitfieldOverflow(
    mut value: int64_t,
    mut incr: int64_t,
    mut bits: uint64_t,
    mut owtype: libc::c_int,
    mut limit: *mut int64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut max: int64_t = if bits == 64 as libc::c_int as libc::c_ulong {
        9223372036854775807 as libc::c_long
    } else {
        ((1 as libc::c_int as int64_t)
            << bits.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long
    };
    let mut min: int64_t = -max - 1 as libc::c_int as libc::c_long;
    let mut maxincr: int64_t = (max as uint64_t).wrapping_sub(value as libc::c_ulong)
        as int64_t;
    let mut minincr: int64_t = min - value;
    if value > max || bits != 64 as libc::c_int as libc::c_ulong && incr > maxincr
        || value >= 0 as libc::c_int as libc::c_long
            && incr > 0 as libc::c_int as libc::c_long && incr > maxincr
    {
        if !limit.is_null() {
            if owtype == 0 as libc::c_int {
                current_block = 90460017421671848;
            } else {
                if owtype == 1 as libc::c_int {
                    *limit = max;
                }
                current_block = 7815301370352969686;
            }
        } else {
            current_block = 7815301370352969686;
        }
        match current_block {
            90460017421671848 => {}
            _ => return 1 as libc::c_int,
        }
    } else if value < min || bits != 64 as libc::c_int as libc::c_ulong && incr < minincr
        || value < 0 as libc::c_int as libc::c_long
            && incr < 0 as libc::c_int as libc::c_long && incr < minincr
    {
        if !limit.is_null() {
            if owtype == 0 as libc::c_int {
                current_block = 90460017421671848;
            } else {
                if owtype == 1 as libc::c_int {
                    *limit = min;
                }
                current_block = 12599329904712511516;
            }
        } else {
            current_block = 12599329904712511516;
        }
        match current_block {
            90460017421671848 => {}
            _ => return -(1 as libc::c_int),
        }
    } else {
        return 0 as libc::c_int
    }
    let mut msb: uint64_t = (1 as libc::c_int as uint64_t)
        << bits.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut a: uint64_t = value as uint64_t;
    let mut b: uint64_t = incr as uint64_t;
    let mut c: uint64_t = 0;
    c = a.wrapping_add(b);
    if bits < 64 as libc::c_int as libc::c_ulong {
        let mut mask: uint64_t = (-(1 as libc::c_int) as uint64_t) << bits;
        if c & msb != 0 {
            c |= mask;
        } else {
            c &= !mask;
        }
    }
    *limit = c as int64_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printBits(mut p: *mut libc::c_uchar, mut count: libc::c_ulong) {
    let mut j: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut byte: libc::c_ulong = 0;
    j = 0 as libc::c_int as libc::c_ulong;
    while j < count {
        byte = *p.offset(j as isize) as libc::c_ulong;
        i = 0x80 as libc::c_int as libc::c_ulong;
        while i > 0 as libc::c_int as libc::c_ulong {
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                if byte & i != 0 { '1' as i32 } else { '0' as i32 },
            );
            i = i.wrapping_div(2 as libc::c_int as libc::c_ulong);
        }
        printf(b"|\0" as *const u8 as *const libc::c_char);
        j = j.wrapping_add(1);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn getBitOffsetFromArgument(
    mut c: *mut client,
    mut o: *mut robj,
    mut offset: *mut uint64_t,
    mut hash: libc::c_int,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut loffset: libc::c_longlong = 0;
    let mut err: *mut libc::c_char = b"bit offset is not an integer or out of range\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut p: *mut libc::c_char = (*o).ptr as *mut libc::c_char;
    let mut plen: size_t = sdslen(p);
    let mut usehash: libc::c_int = 0 as libc::c_int;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 && hash != 0
        && bits > 0 as libc::c_int
    {
        usehash = 1 as libc::c_int;
    }
    if string2ll(
        p.offset(usehash as isize),
        plen.wrapping_sub(usehash as libc::c_ulong),
        &mut loffset,
    ) == 0 as libc::c_int
    {
        addReplyError(c, err);
        return -(1 as libc::c_int);
    }
    if usehash != 0 {
        loffset *= bits as libc::c_longlong;
    }
    if loffset < 0 as libc::c_int as libc::c_longlong
        || mustObeyClient(c) == 0
            && loffset >> 3 as libc::c_int >= server.proto_max_bulk_len
    {
        addReplyError(c, err);
        return -(1 as libc::c_int);
    }
    *offset = loffset as uint64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getBitfieldTypeFromArgument(
    mut c: *mut client,
    mut o: *mut robj,
    mut sign: *mut libc::c_int,
    mut bits: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = (*o).ptr as *mut libc::c_char;
    let mut err: *mut libc::c_char = b"Invalid bitfield type. Use something like i16 u8. Note that u64 is not supported but i64 is.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut llbits: libc::c_longlong = 0;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'i' as i32 {
        *sign = 1 as libc::c_int;
    } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'u' as i32 {
        *sign = 0 as libc::c_int;
    } else {
        addReplyError(c, err);
        return -(1 as libc::c_int);
    }
    if string2ll(
        p.offset(1 as libc::c_int as isize),
        strlen(p.offset(1 as libc::c_int as isize)),
        &mut llbits,
    ) == 0 as libc::c_int || llbits < 1 as libc::c_int as libc::c_longlong
        || *sign == 1 as libc::c_int && llbits > 64 as libc::c_int as libc::c_longlong
        || *sign == 0 as libc::c_int && llbits > 63 as libc::c_int as libc::c_longlong
    {
        addReplyError(c, err);
        return -(1 as libc::c_int);
    }
    *bits = llbits as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lookupStringForBitCommand(
    mut c: *mut client,
    mut maxbit: uint64_t,
    mut dirty: *mut libc::c_int,
) -> *mut robj {
    let mut byte: size_t = maxbit >> 3 as libc::c_int;
    let mut o: *mut robj = lookupKeyWrite(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    if checkType(c, o, 0 as libc::c_int) != 0 {
        return 0 as *mut robj;
    }
    if !dirty.is_null() {
        *dirty = 0 as libc::c_int;
    }
    if o.is_null() {
        o = createObject(
            0 as libc::c_int,
            sdsnewlen(
                0 as *const libc::c_void,
                byte.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_void,
        );
        dbAdd((*c).db, *((*c).argv).offset(1 as libc::c_int as isize), o);
        if !dirty.is_null() {
            *dirty = 1 as libc::c_int;
        }
    } else {
        o = dbUnshareStringValue(
            (*c).db,
            *((*c).argv).offset(1 as libc::c_int as isize),
            o,
        );
        let mut oldlen: size_t = sdslen((*o).ptr as sds);
        (*o)
            .ptr = sdsgrowzero(
            (*o).ptr as sds,
            byte.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_void;
        if !dirty.is_null() && oldlen != sdslen((*o).ptr as sds) {
            *dirty = 1 as libc::c_int;
        }
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn getObjectReadOnlyString(
    mut o: *mut robj,
    mut len: *mut libc::c_long,
    mut llbuf: *mut libc::c_char,
) -> *mut libc::c_uchar {
    if o.is_null() || (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssert(
            b"!o || o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"bitops.c\0" as *const u8 as *const libc::c_char,
            514 as libc::c_int,
        );
        unreachable!();
    };
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !o.is_null() && (*o).encoding() as libc::c_int == 1 as libc::c_int {
        p = llbuf as *mut libc::c_uchar;
        if !len.is_null() {
            *len = ll2string(
                llbuf,
                21 as libc::c_int as size_t,
                (*o).ptr as libc::c_long as libc::c_longlong,
            ) as libc::c_long;
        }
    } else if !o.is_null() {
        p = (*o).ptr as *mut libc::c_uchar;
        if !len.is_null() {
            *len = sdslen((*o).ptr as sds) as libc::c_long;
        }
    } else if !len.is_null() {
        *len = 0 as libc::c_int as libc::c_long;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn setbitCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut err: *mut libc::c_char = b"bit is not an integer or out of range\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut bitoffset: uint64_t = 0;
    let mut byte: ssize_t = 0;
    let mut bit: ssize_t = 0;
    let mut byteval: libc::c_int = 0;
    let mut bitval: libc::c_int = 0;
    let mut on: libc::c_long = 0;
    if getBitOffsetFromArgument(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut bitoffset,
        0 as libc::c_int,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return;
    }
    if getLongFromObjectOrReply(
        c,
        *((*c).argv).offset(3 as libc::c_int as isize),
        &mut on,
        err,
    ) != 0 as libc::c_int
    {
        return;
    }
    if on & !(1 as libc::c_int) as libc::c_long != 0 {
        addReplyError(c, err);
        return;
    }
    let mut dirty: libc::c_int = 0;
    o = lookupStringForBitCommand(c, bitoffset, &mut dirty);
    if o.is_null() {
        return;
    }
    byte = (bitoffset >> 3 as libc::c_int) as ssize_t;
    byteval = *((*o).ptr as *mut uint8_t).offset(byte as isize) as libc::c_int;
    bit = (7 as libc::c_int as libc::c_ulong)
        .wrapping_sub(bitoffset & 0x7 as libc::c_int as libc::c_ulong) as ssize_t;
    bitval = byteval & (1 as libc::c_int) << bit;
    if dirty != 0 || (bitval != 0) as libc::c_int as libc::c_long != on {
        byteval &= !((1 as libc::c_int) << bit);
        byteval = (byteval as libc::c_long
            | (on & 0x1 as libc::c_int as libc::c_long) << bit) as libc::c_int;
        *((*o).ptr as *mut uint8_t).offset(byte as isize) = byteval as uint8_t;
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 3 as libc::c_int,
            b"setbit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        server.dirty += 1;
    }
    addReply(c, if bitval != 0 { shared.cone } else { shared.czero });
}
#[no_mangle]
pub unsafe extern "C" fn getbitCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut llbuf: [libc::c_char; 32] = [0; 32];
    let mut bitoffset: uint64_t = 0;
    let mut byte: size_t = 0;
    let mut bit: size_t = 0;
    let mut bitval: size_t = 0 as libc::c_int as size_t;
    if getBitOffsetFromArgument(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut bitoffset,
        0 as libc::c_int,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return;
    }
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 0 as libc::c_int) != 0 {
        return;
    }
    byte = bitoffset >> 3 as libc::c_int;
    bit = (7 as libc::c_int as libc::c_ulong)
        .wrapping_sub(bitoffset & 0x7 as libc::c_int as libc::c_ulong);
    if (*o).encoding() as libc::c_int == 0 as libc::c_int
        || (*o).encoding() as libc::c_int == 8 as libc::c_int
    {
        if byte < sdslen((*o).ptr as sds) {
            bitval = (*((*o).ptr as *mut uint8_t).offset(byte as isize) as libc::c_int
                & (1 as libc::c_int) << bit) as size_t;
        }
    } else if byte
        < ll2string(
            llbuf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            (*o).ptr as libc::c_long as libc::c_longlong,
        ) as size_t
    {
        bitval = (llbuf[byte as usize] as libc::c_int & (1 as libc::c_int) << bit)
            as size_t;
    }
    addReply(c, if bitval != 0 { shared.cone } else { shared.czero });
}
#[no_mangle]
pub unsafe extern "C" fn bitopCommand(mut c: *mut client) {
    let mut opname: *mut libc::c_char = (**((*c).argv).offset(1 as libc::c_int as isize))
        .ptr as *mut libc::c_char;
    let mut o: *mut robj = 0 as *mut robj;
    let mut targetkey: *mut robj = *((*c).argv).offset(2 as libc::c_int as isize);
    let mut op: libc::c_ulong = 0;
    let mut j: libc::c_ulong = 0;
    let mut numkeys: libc::c_ulong = 0;
    let mut objects: *mut *mut robj = 0 as *mut *mut robj;
    let mut src: *mut *mut libc::c_uchar = 0 as *mut *mut libc::c_uchar;
    let mut len: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    let mut maxlen: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut minlen: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut res: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*opname.offset(0 as libc::c_int as isize) as libc::c_int == 'a' as i32
        || *opname.offset(0 as libc::c_int as isize) as libc::c_int == 'A' as i32)
        && strcasecmp(opname, b"and\0" as *const u8 as *const libc::c_char) == 0
    {
        op = 0 as libc::c_int as libc::c_ulong;
    } else if (*opname.offset(0 as libc::c_int as isize) as libc::c_int == 'o' as i32
        || *opname.offset(0 as libc::c_int as isize) as libc::c_int == 'O' as i32)
        && strcasecmp(opname, b"or\0" as *const u8 as *const libc::c_char) == 0
    {
        op = 1 as libc::c_int as libc::c_ulong;
    } else if (*opname.offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32
        || *opname.offset(0 as libc::c_int as isize) as libc::c_int == 'X' as i32)
        && strcasecmp(opname, b"xor\0" as *const u8 as *const libc::c_char) == 0
    {
        op = 2 as libc::c_int as libc::c_ulong;
    } else if (*opname.offset(0 as libc::c_int as isize) as libc::c_int == 'n' as i32
        || *opname.offset(0 as libc::c_int as isize) as libc::c_int == 'N' as i32)
        && strcasecmp(opname, b"not\0" as *const u8 as *const libc::c_char) == 0
    {
        op = 3 as libc::c_int as libc::c_ulong;
    } else {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    if op == 3 as libc::c_int as libc::c_ulong && (*c).argc != 4 as libc::c_int {
        addReplyError(
            c,
            b"BITOP NOT must be called with a single source key.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    numkeys = ((*c).argc - 3 as libc::c_int) as libc::c_ulong;
    src = zmalloc(
        (core::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(numkeys),
    ) as *mut *mut libc::c_uchar;
    len = zmalloc(
        (core::mem::size_of::<libc::c_long>() as libc::c_ulong).wrapping_mul(numkeys),
    ) as *mut libc::c_ulong;
    objects = zmalloc(
        (core::mem::size_of::<*mut robj>() as libc::c_ulong).wrapping_mul(numkeys),
    ) as *mut *mut robj;
    j = 0 as libc::c_int as libc::c_ulong;
    while j < numkeys {
        o = lookupKeyRead(
            (*c).db,
            *((*c).argv)
                .offset(j.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize),
        );
        if o.is_null() {
            let ref mut fresh10 = *objects.offset(j as isize);
            *fresh10 = 0 as *mut robj;
            let ref mut fresh11 = *src.offset(j as isize);
            *fresh11 = 0 as *mut libc::c_uchar;
            *len.offset(j as isize) = 0 as libc::c_int as libc::c_ulong;
            minlen = 0 as libc::c_int as libc::c_ulong;
        } else {
            if checkType(c, o, 0 as libc::c_int) != 0 {
                let mut i: libc::c_ulong = 0;
                i = 0 as libc::c_int as libc::c_ulong;
                while i < j {
                    if !(*objects.offset(i as isize)).is_null() {
                        decrRefCount(*objects.offset(i as isize));
                    }
                    i = i.wrapping_add(1);
                }
                zfree(src as *mut libc::c_void);
                zfree(len as *mut libc::c_void);
                zfree(objects as *mut libc::c_void);
                return;
            }
            let ref mut fresh12 = *objects.offset(j as isize);
            *fresh12 = getDecodedObject(o);
            let ref mut fresh13 = *src.offset(j as isize);
            *fresh13 = (**objects.offset(j as isize)).ptr as *mut libc::c_uchar;
            *len.offset(j as isize) = sdslen((**objects.offset(j as isize)).ptr as sds);
            if *len.offset(j as isize) > maxlen {
                maxlen = *len.offset(j as isize);
            }
            if j == 0 as libc::c_int as libc::c_ulong || *len.offset(j as isize) < minlen
            {
                minlen = *len.offset(j as isize);
            }
        }
        j = j.wrapping_add(1);
    }
    if maxlen != 0 {
        res = sdsnewlen(0 as *const libc::c_void, maxlen) as *mut libc::c_uchar;
        let mut output: libc::c_uchar = 0;
        let mut byte: libc::c_uchar = 0;
        let mut i_0: libc::c_ulong = 0;
        j = 0 as libc::c_int as libc::c_ulong;
        if minlen
            >= (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            && numkeys <= 16 as libc::c_int as libc::c_ulong
        {
            let mut lp: [*mut libc::c_ulong; 16] = [0 as *mut libc::c_ulong; 16];
            let mut lres: *mut libc::c_ulong = res as *mut libc::c_ulong;
            memcpy(
                lp.as_mut_ptr() as *mut libc::c_void,
                src as *const libc::c_void,
                (core::mem::size_of::<*mut libc::c_ulong>() as libc::c_ulong)
                    .wrapping_mul(numkeys),
            );
            memcpy(
                res as *mut libc::c_void,
                *src.offset(0 as libc::c_int as isize) as *const libc::c_void,
                minlen,
            );
            if op == 0 as libc::c_int as libc::c_ulong {
                while minlen
                    >= (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                {
                    i_0 = 1 as libc::c_int as libc::c_ulong;
                    while i_0 < numkeys {
                        *lres.offset(0 as libc::c_int as isize)
                            &= *(lp[i_0 as usize]).offset(0 as libc::c_int as isize);
                        *lres.offset(1 as libc::c_int as isize)
                            &= *(lp[i_0 as usize]).offset(1 as libc::c_int as isize);
                        *lres.offset(2 as libc::c_int as isize)
                            &= *(lp[i_0 as usize]).offset(2 as libc::c_int as isize);
                        *lres.offset(3 as libc::c_int as isize)
                            &= *(lp[i_0 as usize]).offset(3 as libc::c_int as isize);
                        lp[i_0
                            as usize] = (lp[i_0 as usize])
                            .offset(4 as libc::c_int as isize);
                        i_0 = i_0.wrapping_add(1);
                    }
                    lres = lres.offset(4 as libc::c_int as isize);
                    j = j
                        .wrapping_add(
                            (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        );
                    minlen = minlen
                        .wrapping_sub(
                            (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        );
                }
            } else if op == 1 as libc::c_int as libc::c_ulong {
                while minlen
                    >= (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                {
                    i_0 = 1 as libc::c_int as libc::c_ulong;
                    while i_0 < numkeys {
                        *lres.offset(0 as libc::c_int as isize)
                            |= *(lp[i_0 as usize]).offset(0 as libc::c_int as isize);
                        *lres.offset(1 as libc::c_int as isize)
                            |= *(lp[i_0 as usize]).offset(1 as libc::c_int as isize);
                        *lres.offset(2 as libc::c_int as isize)
                            |= *(lp[i_0 as usize]).offset(2 as libc::c_int as isize);
                        *lres.offset(3 as libc::c_int as isize)
                            |= *(lp[i_0 as usize]).offset(3 as libc::c_int as isize);
                        lp[i_0
                            as usize] = (lp[i_0 as usize])
                            .offset(4 as libc::c_int as isize);
                        i_0 = i_0.wrapping_add(1);
                    }
                    lres = lres.offset(4 as libc::c_int as isize);
                    j = j
                        .wrapping_add(
                            (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        );
                    minlen = minlen
                        .wrapping_sub(
                            (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        );
                }
            } else if op == 2 as libc::c_int as libc::c_ulong {
                while minlen
                    >= (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                {
                    i_0 = 1 as libc::c_int as libc::c_ulong;
                    while i_0 < numkeys {
                        *lres.offset(0 as libc::c_int as isize)
                            ^= *(lp[i_0 as usize]).offset(0 as libc::c_int as isize);
                        *lres.offset(1 as libc::c_int as isize)
                            ^= *(lp[i_0 as usize]).offset(1 as libc::c_int as isize);
                        *lres.offset(2 as libc::c_int as isize)
                            ^= *(lp[i_0 as usize]).offset(2 as libc::c_int as isize);
                        *lres.offset(3 as libc::c_int as isize)
                            ^= *(lp[i_0 as usize]).offset(3 as libc::c_int as isize);
                        lp[i_0
                            as usize] = (lp[i_0 as usize])
                            .offset(4 as libc::c_int as isize);
                        i_0 = i_0.wrapping_add(1);
                    }
                    lres = lres.offset(4 as libc::c_int as isize);
                    j = j
                        .wrapping_add(
                            (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        );
                    minlen = minlen
                        .wrapping_sub(
                            (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        );
                }
            } else if op == 3 as libc::c_int as libc::c_ulong {
                while minlen
                    >= (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                {
                    *lres
                        .offset(
                            0 as libc::c_int as isize,
                        ) = !*lres.offset(0 as libc::c_int as isize);
                    *lres
                        .offset(
                            1 as libc::c_int as isize,
                        ) = !*lres.offset(1 as libc::c_int as isize);
                    *lres
                        .offset(
                            2 as libc::c_int as isize,
                        ) = !*lres.offset(2 as libc::c_int as isize);
                    *lres
                        .offset(
                            3 as libc::c_int as isize,
                        ) = !*lres.offset(3 as libc::c_int as isize);
                    lres = lres.offset(4 as libc::c_int as isize);
                    j = j
                        .wrapping_add(
                            (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        );
                    minlen = minlen
                        .wrapping_sub(
                            (core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        );
                }
            }
        }
        while j < maxlen {
            output = (if *len.offset(0 as libc::c_int as isize) <= j {
                0 as libc::c_int
            } else {
                *(*src.offset(0 as libc::c_int as isize)).offset(j as isize)
                    as libc::c_int
            }) as libc::c_uchar;
            if op == 3 as libc::c_int as libc::c_ulong {
                output = !(output as libc::c_int) as libc::c_uchar;
            }
            i_0 = 1 as libc::c_int as libc::c_ulong;
            while i_0 < numkeys {
                let mut skip: libc::c_int = 0 as libc::c_int;
                byte = (if *len.offset(i_0 as isize) <= j {
                    0 as libc::c_int
                } else {
                    *(*src.offset(i_0 as isize)).offset(j as isize) as libc::c_int
                }) as libc::c_uchar;
                match op {
                    0 => {
                        output = (output as libc::c_int & byte as libc::c_int)
                            as libc::c_uchar;
                        skip = (output as libc::c_int == 0 as libc::c_int)
                            as libc::c_int;
                    }
                    1 => {
                        output = (output as libc::c_int | byte as libc::c_int)
                            as libc::c_uchar;
                        skip = (output as libc::c_int == 0xff as libc::c_int)
                            as libc::c_int;
                    }
                    2 => {
                        output = (output as libc::c_int ^ byte as libc::c_int)
                            as libc::c_uchar;
                    }
                    _ => {}
                }
                if skip != 0 {
                    break;
                }
                i_0 = i_0.wrapping_add(1);
            }
            *res.offset(j as isize) = output;
            j = j.wrapping_add(1);
        }
    }
    j = 0 as libc::c_int as libc::c_ulong;
    while j < numkeys {
        if !(*objects.offset(j as isize)).is_null() {
            decrRefCount(*objects.offset(j as isize));
        }
        j = j.wrapping_add(1);
    }
    zfree(src as *mut libc::c_void);
    zfree(len as *mut libc::c_void);
    zfree(objects as *mut libc::c_void);
    if maxlen != 0 {
        o = createObject(0 as libc::c_int, res as *mut libc::c_void);
        setKey(c, (*c).db, targetkey, o, 0 as libc::c_int);
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 3 as libc::c_int,
            b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            targetkey,
            (*(*c).db).id,
        );
        decrRefCount(o);
        server.dirty += 1;
    } else if dbDelete((*c).db, targetkey) != 0 {
        signalModifiedKey(c, (*c).db, targetkey);
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 2 as libc::c_int,
            b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            targetkey,
            (*(*c).db).id,
        );
        server.dirty += 1;
    }
    addReplyLongLong(c, maxlen as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn bitcountCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut start: libc::c_longlong = 0;
    let mut end: libc::c_longlong = 0;
    let mut strlen_0: libc::c_long = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut llbuf: [libc::c_char; 21] = [0; 21];
    let mut isbit: libc::c_int = 0 as libc::c_int;
    let mut first_byte_neg_mask: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut last_byte_neg_mask: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    o = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.czero,
    );
    if o.is_null() || checkType(c, o, 0 as libc::c_int) != 0 {
        return;
    }
    p = getObjectReadOnlyString(o, &mut strlen_0, llbuf.as_mut_ptr());
    if (*c).argc == 4 as libc::c_int || (*c).argc == 5 as libc::c_int {
        let mut totlen: libc::c_longlong = strlen_0 as libc::c_longlong;
        if totlen <= 9223372036854775807 as libc::c_longlong >> 3 as libc::c_int
        {} else {
            _serverAssert(
                b"totlen <= LLONG_MAX >> 3\0" as *const u8 as *const libc::c_char,
                b"bitops.c\0" as *const u8 as *const libc::c_char,
                814 as libc::c_int,
            );
            unreachable!();
        };
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
        if start < 0 as libc::c_int as libc::c_longlong
            && end < 0 as libc::c_int as libc::c_longlong && start > end
        {
            addReply(c, shared.czero);
            return;
        }
        if (*c).argc == 5 as libc::c_int {
            if strcasecmp(
                (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"bit\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                isbit = 1 as libc::c_int;
            } else if strcasecmp(
                (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"byte\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                isbit = 0 as libc::c_int;
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
        }
        if isbit != 0 {
            totlen <<= 3 as libc::c_int;
        }
        if start < 0 as libc::c_int as libc::c_longlong {
            start = totlen + start;
        }
        if end < 0 as libc::c_int as libc::c_longlong {
            end = totlen + end;
        }
        if start < 0 as libc::c_int as libc::c_longlong {
            start = 0 as libc::c_int as libc::c_longlong;
        }
        if end < 0 as libc::c_int as libc::c_longlong {
            end = 0 as libc::c_int as libc::c_longlong;
        }
        if end >= totlen {
            end = totlen - 1 as libc::c_int as libc::c_longlong;
        }
        if isbit != 0 && start <= end {
            first_byte_neg_mask = (!(((1 as libc::c_int)
                << 8 as libc::c_int as libc::c_longlong
                    - (start & 7 as libc::c_int as libc::c_longlong)) - 1 as libc::c_int)
                & 0xff as libc::c_int) as libc::c_uchar;
            last_byte_neg_mask = (((1 as libc::c_int)
                << 7 as libc::c_int as libc::c_longlong
                    - (end & 7 as libc::c_int as libc::c_longlong)) - 1 as libc::c_int)
                as libc::c_uchar;
            start >>= 3 as libc::c_int;
            end >>= 3 as libc::c_int;
        }
    } else if (*c).argc == 2 as libc::c_int {
        start = 0 as libc::c_int as libc::c_longlong;
        end = (strlen_0 - 1 as libc::c_int as libc::c_long) as libc::c_longlong;
    } else {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    if start > end {
        addReply(c, shared.czero);
    } else {
        let mut bytes: libc::c_long = (end - start
            + 1 as libc::c_int as libc::c_longlong) as libc::c_long;
        let mut count: libc::c_longlong = redisPopcount(
            p.offset(start as isize) as *mut libc::c_void,
            bytes,
        );
        if first_byte_neg_mask as libc::c_int != 0 as libc::c_int
            || last_byte_neg_mask as libc::c_int != 0 as libc::c_int
        {
            let mut firstlast: [libc::c_uchar; 2] = [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ];
            if first_byte_neg_mask as libc::c_int != 0 as libc::c_int {
                firstlast[0 as libc::c_int
                    as usize] = (*p.offset(start as isize) as libc::c_int
                    & first_byte_neg_mask as libc::c_int) as libc::c_uchar;
            }
            if last_byte_neg_mask as libc::c_int != 0 as libc::c_int {
                firstlast[1 as libc::c_int
                    as usize] = (*p.offset(end as isize) as libc::c_int
                    & last_byte_neg_mask as libc::c_int) as libc::c_uchar;
            }
            count
                -= redisPopcount(
                    firstlast.as_mut_ptr() as *mut libc::c_void,
                    2 as libc::c_int as libc::c_long,
                );
        }
        addReplyLongLong(c, count);
    };
}
#[no_mangle]
pub unsafe extern "C" fn bitposCommand(mut c: *mut client) {
    let mut curbytes: libc::c_long = 0;
    let mut current_block: u64;
    let mut o: *mut robj = 0 as *mut robj;
    let mut start: libc::c_longlong = 0;
    let mut end: libc::c_longlong = 0;
    let mut bit: libc::c_long = 0;
    let mut strlen_0: libc::c_long = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut llbuf: [libc::c_char; 21] = [0; 21];
    let mut isbit: libc::c_int = 0 as libc::c_int;
    let mut end_given: libc::c_int = 0 as libc::c_int;
    let mut first_byte_neg_mask: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut last_byte_neg_mask: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    if getLongFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut bit,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if bit != 0 as libc::c_int as libc::c_long && bit != 1 as libc::c_int as libc::c_long
    {
        addReplyError(
            c,
            b"The bit argument must be 1 or 0.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    o = lookupKeyRead((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        addReplyLongLong(
            c,
            (if bit != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int })
                as libc::c_longlong,
        );
        return;
    }
    if checkType(c, o, 0 as libc::c_int) != 0 {
        return;
    }
    p = getObjectReadOnlyString(o, &mut strlen_0, llbuf.as_mut_ptr());
    if (*c).argc == 4 as libc::c_int || (*c).argc == 5 as libc::c_int
        || (*c).argc == 6 as libc::c_int
    {
        let mut totlen: libc::c_longlong = strlen_0 as libc::c_longlong;
        if totlen <= 9223372036854775807 as libc::c_longlong >> 3 as libc::c_int
        {} else {
            _serverAssert(
                b"totlen <= LLONG_MAX >> 3\0" as *const u8 as *const libc::c_char,
                b"bitops.c\0" as *const u8 as *const libc::c_char,
                909 as libc::c_int,
            );
            unreachable!();
        };
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset(3 as libc::c_int as isize),
            &mut start,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        if (*c).argc == 6 as libc::c_int {
            if strcasecmp(
                (**((*c).argv).offset(5 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"bit\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                isbit = 1 as libc::c_int;
            } else if strcasecmp(
                (**((*c).argv).offset(5 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"byte\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                isbit = 0 as libc::c_int;
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
        }
        if (*c).argc >= 5 as libc::c_int {
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset(4 as libc::c_int as isize),
                &mut end,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            end_given = 1 as libc::c_int;
        } else if isbit != 0 {
            end = (totlen << 3 as libc::c_int) + 7 as libc::c_int as libc::c_longlong;
        } else {
            end = totlen - 1 as libc::c_int as libc::c_longlong;
        }
        if isbit != 0 {
            totlen <<= 3 as libc::c_int;
        }
        if start < 0 as libc::c_int as libc::c_longlong {
            start = totlen + start;
        }
        if end < 0 as libc::c_int as libc::c_longlong {
            end = totlen + end;
        }
        if start < 0 as libc::c_int as libc::c_longlong {
            start = 0 as libc::c_int as libc::c_longlong;
        }
        if end < 0 as libc::c_int as libc::c_longlong {
            end = 0 as libc::c_int as libc::c_longlong;
        }
        if end >= totlen {
            end = totlen - 1 as libc::c_int as libc::c_longlong;
        }
        if isbit != 0 && start <= end {
            first_byte_neg_mask = (!(((1 as libc::c_int)
                << 8 as libc::c_int as libc::c_longlong
                    - (start & 7 as libc::c_int as libc::c_longlong)) - 1 as libc::c_int)
                & 0xff as libc::c_int) as libc::c_uchar;
            last_byte_neg_mask = (((1 as libc::c_int)
                << 7 as libc::c_int as libc::c_longlong
                    - (end & 7 as libc::c_int as libc::c_longlong)) - 1 as libc::c_int)
                as libc::c_uchar;
            start >>= 3 as libc::c_int;
            end >>= 3 as libc::c_int;
        }
    } else if (*c).argc == 3 as libc::c_int {
        start = 0 as libc::c_int as libc::c_longlong;
        end = (strlen_0 - 1 as libc::c_int as libc::c_long) as libc::c_longlong;
    } else {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    if start > end {
        addReplyLongLong(c, -(1 as libc::c_int) as libc::c_longlong);
    } else {
        let mut bytes: libc::c_long = (end - start
            + 1 as libc::c_int as libc::c_longlong) as libc::c_long;
        let mut pos: libc::c_longlong = 0;
        let mut tmpchar: libc::c_uchar = 0;
        if first_byte_neg_mask != 0 {
            if bit != 0 {
                tmpchar = (*p.offset(start as isize) as libc::c_int
                    & !(first_byte_neg_mask as libc::c_int)) as libc::c_uchar;
            } else {
                tmpchar = (*p.offset(start as isize) as libc::c_int
                    | first_byte_neg_mask as libc::c_int) as libc::c_uchar;
            }
            if last_byte_neg_mask as libc::c_int != 0
                && bytes == 1 as libc::c_int as libc::c_long
            {
                if bit != 0 {
                    tmpchar = (tmpchar as libc::c_int
                        & !(last_byte_neg_mask as libc::c_int)) as libc::c_uchar;
                } else {
                    tmpchar = (tmpchar as libc::c_int
                        | last_byte_neg_mask as libc::c_int) as libc::c_uchar;
                }
            }
            pos = redisBitpos(
                &mut tmpchar as *mut libc::c_uchar as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                bit as libc::c_int,
            );
            if bytes == 1 as libc::c_int as libc::c_long
                || pos != -(1 as libc::c_int) as libc::c_longlong
                    && pos != 8 as libc::c_int as libc::c_longlong
            {
                current_block = 288816968937153605;
            } else {
                start += 1;
                bytes -= 1;
                current_block = 4888910987971495881;
            }
        } else {
            current_block = 4888910987971495881;
        }
        match current_block {
            4888910987971495881 => {
                curbytes = bytes
                    - (if last_byte_neg_mask as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_long;
                if curbytes > 0 as libc::c_int as libc::c_long {
                    pos = redisBitpos(
                        p.offset(start as isize) as *mut libc::c_void,
                        curbytes as libc::c_ulong,
                        bit as libc::c_int,
                    );
                    if bytes == curbytes
                        || pos != -(1 as libc::c_int) as libc::c_longlong
                            && pos != (curbytes as libc::c_longlong) << 3 as libc::c_int
                    {
                        current_block = 288816968937153605;
                    } else {
                        start += curbytes as libc::c_longlong;
                        bytes -= curbytes;
                        current_block = 15669289850109000831;
                    }
                } else {
                    current_block = 15669289850109000831;
                }
                match current_block {
                    288816968937153605 => {}
                    _ => {
                        if bit != 0 {
                            tmpchar = (*p.offset(end as isize) as libc::c_int
                                & !(last_byte_neg_mask as libc::c_int)) as libc::c_uchar;
                        } else {
                            tmpchar = (*p.offset(end as isize) as libc::c_int
                                | last_byte_neg_mask as libc::c_int) as libc::c_uchar;
                        }
                        pos = redisBitpos(
                            &mut tmpchar as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            bit as libc::c_int,
                        );
                    }
                }
            }
            _ => {}
        }
        if end_given != 0 && bit == 0 as libc::c_int as libc::c_long
            && pos == (bytes as libc::c_longlong) << 3 as libc::c_int
        {
            addReplyLongLong(c, -(1 as libc::c_int) as libc::c_longlong);
            return;
        }
        if pos != -(1 as libc::c_int) as libc::c_longlong {
            pos += start << 3 as libc::c_int;
        }
        addReplyLongLong(c, pos);
    };
}
#[no_mangle]
pub unsafe extern "C" fn bitfieldGeneric(mut c: *mut client, mut flags: libc::c_int) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut bitoffset: uint64_t = 0;
    let mut j: libc::c_int = 0;
    let mut numops: libc::c_int = 0 as libc::c_int;
    let mut changes: libc::c_int = 0 as libc::c_int;
    let mut dirty: libc::c_int = 0 as libc::c_int;
    let mut ops: *mut bitfieldOp = 0 as *mut bitfieldOp;
    let mut owtype: libc::c_int = 0 as libc::c_int;
    let mut readonly: libc::c_int = 1 as libc::c_int;
    let mut highest_write_offset: uint64_t = 0 as libc::c_int as uint64_t;
    let mut current_block_44: u64;
    j = 2 as libc::c_int;
    while j < (*c).argc {
        let mut remargs: libc::c_int = (*c).argc - j - 1 as libc::c_int;
        let mut subcmd: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
            as *mut libc::c_char;
        let mut opcode: libc::c_int = 0;
        let mut i64: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        let mut sign: libc::c_int = 0 as libc::c_int;
        let mut bits: libc::c_int = 0 as libc::c_int;
        if strcasecmp(subcmd, b"get\0" as *const u8 as *const libc::c_char) == 0
            && remargs >= 2 as libc::c_int
        {
            opcode = 0 as libc::c_int;
            current_block_44 = 11194104282611034094;
        } else if strcasecmp(subcmd, b"set\0" as *const u8 as *const libc::c_char) == 0
            && remargs >= 3 as libc::c_int
        {
            opcode = 1 as libc::c_int;
            current_block_44 = 11194104282611034094;
        } else if strcasecmp(subcmd, b"incrby\0" as *const u8 as *const libc::c_char)
            == 0 && remargs >= 3 as libc::c_int
        {
            opcode = 2 as libc::c_int;
            current_block_44 = 11194104282611034094;
        } else {
            if strcasecmp(subcmd, b"overflow\0" as *const u8 as *const libc::c_char) == 0
                && remargs >= 1 as libc::c_int
            {
                let mut owtypename: *mut libc::c_char = (**((*c).argv)
                    .offset((j + 1 as libc::c_int) as isize))
                    .ptr as *mut libc::c_char;
                j += 1;
                if strcasecmp(owtypename, b"wrap\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                    owtype = 0 as libc::c_int;
                } else if strcasecmp(
                    owtypename,
                    b"sat\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    owtype = 1 as libc::c_int;
                } else if strcasecmp(
                    owtypename,
                    b"fail\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    owtype = 2 as libc::c_int;
                } else {
                    addReplyError(
                        c,
                        b"Invalid OVERFLOW type specified\0" as *const u8
                            as *const libc::c_char,
                    );
                    zfree(ops as *mut libc::c_void);
                    return;
                }
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                zfree(ops as *mut libc::c_void);
                return;
            }
            current_block_44 = 16658872821858055392;
        }
        match current_block_44 {
            11194104282611034094 => {
                if getBitfieldTypeFromArgument(
                    c,
                    *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                    &mut sign,
                    &mut bits,
                ) != 0 as libc::c_int
                {
                    zfree(ops as *mut libc::c_void);
                    return;
                }
                if getBitOffsetFromArgument(
                    c,
                    *((*c).argv).offset((j + 2 as libc::c_int) as isize),
                    &mut bitoffset,
                    1 as libc::c_int,
                    bits,
                ) != 0 as libc::c_int
                {
                    zfree(ops as *mut libc::c_void);
                    return;
                }
                if opcode != 0 as libc::c_int {
                    readonly = 0 as libc::c_int;
                    if highest_write_offset
                        < bitoffset
                            .wrapping_add(bits as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        highest_write_offset = bitoffset
                            .wrapping_add(bits as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    }
                    if getLongLongFromObjectOrReply(
                        c,
                        *((*c).argv).offset((j + 3 as libc::c_int) as isize),
                        &mut i64,
                        0 as *const libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        zfree(ops as *mut libc::c_void);
                        return;
                    }
                }
                ops = zrealloc(
                    ops as *mut libc::c_void,
                    (core::mem::size_of::<bitfieldOp>() as libc::c_ulong)
                        .wrapping_mul((numops + 1 as libc::c_int) as libc::c_ulong),
                ) as *mut bitfieldOp;
                (*ops.offset(numops as isize)).offset = bitoffset;
                (*ops.offset(numops as isize)).i64_0 = i64 as int64_t;
                (*ops.offset(numops as isize)).opcode = opcode;
                (*ops.offset(numops as isize)).owtype = owtype;
                (*ops.offset(numops as isize)).bits = bits;
                (*ops.offset(numops as isize)).sign = sign;
                numops += 1;
                j += 3 as libc::c_int - (opcode == 0 as libc::c_int) as libc::c_int;
            }
            _ => {}
        }
        j += 1;
    }
    if readonly != 0 {
        o = lookupKeyRead((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        if !o.is_null() && checkType(c, o, 0 as libc::c_int) != 0 {
            zfree(ops as *mut libc::c_void);
            return;
        }
    } else {
        if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            zfree(ops as *mut libc::c_void);
            addReplyError(
                c,
                b"BITFIELD_RO only supports the GET subcommand\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        o = lookupStringForBitCommand(c, highest_write_offset, &mut dirty);
        if o.is_null() {
            zfree(ops as *mut libc::c_void);
            return;
        }
    }
    addReplyArrayLen(c, numops as libc::c_long);
    j = 0 as libc::c_int;
    while j < numops {
        let mut thisop: *mut bitfieldOp = ops.offset(j as isize);
        if (*thisop).opcode == 1 as libc::c_int || (*thisop).opcode == 2 as libc::c_int {
            if (*thisop).sign != 0 {
                let mut oldval: int64_t = 0;
                let mut newval: int64_t = 0;
                let mut wrapped: int64_t = 0;
                let mut retval: int64_t = 0;
                let mut overflow: libc::c_int = 0;
                oldval = getSignedBitfield(
                    (*o).ptr as *mut libc::c_uchar,
                    (*thisop).offset,
                    (*thisop).bits as uint64_t,
                );
                if (*thisop).opcode == 2 as libc::c_int {
                    overflow = checkSignedBitfieldOverflow(
                        oldval,
                        (*thisop).i64_0,
                        (*thisop).bits as uint64_t,
                        (*thisop).owtype,
                        &mut wrapped,
                    );
                    newval = if overflow != 0 {
                        wrapped
                    } else {
                        oldval + (*thisop).i64_0
                    };
                    retval = newval;
                } else {
                    newval = (*thisop).i64_0;
                    overflow = checkSignedBitfieldOverflow(
                        newval,
                        0 as libc::c_int as int64_t,
                        (*thisop).bits as uint64_t,
                        (*thisop).owtype,
                        &mut wrapped,
                    );
                    if overflow != 0 {
                        newval = wrapped;
                    }
                    retval = oldval;
                }
                if !(overflow != 0 && (*thisop).owtype == 2 as libc::c_int) {
                    addReplyLongLong(c, retval as libc::c_longlong);
                    setSignedBitfield(
                        (*o).ptr as *mut libc::c_uchar,
                        (*thisop).offset,
                        (*thisop).bits as uint64_t,
                        newval,
                    );
                    if dirty != 0 || oldval != newval {
                        changes += 1;
                    }
                } else {
                    addReplyNull(c);
                }
            } else {
                let mut oldval_0: uint64_t = 0;
                let mut newval_0: uint64_t = 0;
                let mut wrapped_0: uint64_t = 0;
                let mut retval_0: uint64_t = 0;
                let mut overflow_0: libc::c_int = 0;
                oldval_0 = getUnsignedBitfield(
                    (*o).ptr as *mut libc::c_uchar,
                    (*thisop).offset,
                    (*thisop).bits as uint64_t,
                );
                if (*thisop).opcode == 2 as libc::c_int {
                    newval_0 = oldval_0.wrapping_add((*thisop).i64_0 as libc::c_ulong);
                    overflow_0 = checkUnsignedBitfieldOverflow(
                        oldval_0,
                        (*thisop).i64_0,
                        (*thisop).bits as uint64_t,
                        (*thisop).owtype,
                        &mut wrapped_0,
                    );
                    if overflow_0 != 0 {
                        newval_0 = wrapped_0;
                    }
                    retval_0 = newval_0;
                } else {
                    newval_0 = (*thisop).i64_0 as uint64_t;
                    overflow_0 = checkUnsignedBitfieldOverflow(
                        newval_0,
                        0 as libc::c_int as int64_t,
                        (*thisop).bits as uint64_t,
                        (*thisop).owtype,
                        &mut wrapped_0,
                    );
                    if overflow_0 != 0 {
                        newval_0 = wrapped_0;
                    }
                    retval_0 = oldval_0;
                }
                if !(overflow_0 != 0 && (*thisop).owtype == 2 as libc::c_int) {
                    addReplyLongLong(c, retval_0 as libc::c_longlong);
                    setUnsignedBitfield(
                        (*o).ptr as *mut libc::c_uchar,
                        (*thisop).offset,
                        (*thisop).bits as uint64_t,
                        newval_0,
                    );
                    if dirty != 0 || oldval_0 != newval_0 {
                        changes += 1;
                    }
                } else {
                    addReplyNull(c);
                }
            }
        } else {
            let mut buf: [libc::c_uchar; 9] = [0; 9];
            let mut strlen_0: libc::c_long = 0 as libc::c_int as libc::c_long;
            let mut src: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut llbuf: [libc::c_char; 21] = [0; 21];
            if !o.is_null() {
                src = getObjectReadOnlyString(o, &mut strlen_0, llbuf.as_mut_ptr());
            }
            memset(
                buf.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                9 as libc::c_int as libc::c_ulong,
            );
            let mut i: libc::c_int = 0;
            let mut byte: uint64_t = (*thisop).offset >> 3 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 9 as libc::c_int {
                if src.is_null()
                    || (i as libc::c_ulong).wrapping_add(byte) >= strlen_0 as uint64_t
                {
                    break;
                }
                buf[i
                    as usize] = *src
                    .offset((i as libc::c_ulong).wrapping_add(byte) as isize);
                i += 1;
            }
            if (*thisop).sign != 0 {
                let mut val: int64_t = getSignedBitfield(
                    buf.as_mut_ptr(),
                    ((*thisop).offset)
                        .wrapping_sub(
                            byte.wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ),
                    (*thisop).bits as uint64_t,
                );
                addReplyLongLong(c, val as libc::c_longlong);
            } else {
                let mut val_0: uint64_t = getUnsignedBitfield(
                    buf.as_mut_ptr(),
                    ((*thisop).offset)
                        .wrapping_sub(
                            byte.wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ),
                    (*thisop).bits as uint64_t,
                );
                addReplyLongLong(c, val_0 as libc::c_longlong);
            }
        }
        j += 1;
    }
    if changes != 0 {
        signalModifiedKey(c, (*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
        notifyKeyspaceEvent(
            (1 as libc::c_int) << 3 as libc::c_int,
            b"setbit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *((*c).argv).offset(1 as libc::c_int as isize),
            (*(*c).db).id,
        );
        server.dirty += changes as libc::c_longlong;
    }
    zfree(ops as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn bitfieldCommand(mut c: *mut client) {
    bitfieldGeneric(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bitfieldroCommand(mut c: *mut client) {
    bitfieldGeneric(c, (1 as libc::c_int) << 0 as libc::c_int);
}
