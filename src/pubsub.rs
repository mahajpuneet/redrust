extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type RedisModuleCommand;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFetchValue(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn listCreate() -> *mut list;
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listSearchKey(list: *mut list, key: *mut libc::c_void) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn stringmatchlen(
        p: *const libc::c_char,
        plen: libc::c_int,
        s: *const libc::c_char,
        slen: libc::c_int,
        nocase: libc::c_int,
    ) -> libc::c_int;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn addReplyNull(c: *mut client);
    fn addReplyBulk(c: *mut client, obj: *mut robj);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyPushLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn updateClientMemUsageAndBucket(c: *mut client) -> libc::c_int;
    fn decrRefCount(o: *mut robj);
    fn incrRefCount(o: *mut robj);
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn forceCommandPropagation(c: *mut client, flags: libc::c_int);
    fn _serverAssertWithInfo(
        c: *const client,
        o: *const robj,
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn sentinelPublishCommand(c: *mut client);
    fn clusterPropagatePublish(
        channel: *mut robj,
        message: *mut robj,
        sharded: libc::c_int,
    );
    fn slotToChannelAdd(channel: sds);
    fn slotToChannelDel(channel: sds);
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
pub struct listIter {
    pub next: *mut listNode,
    pub direction: libc::c_int,
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
pub struct C2RustUnnamed_6 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pubsubtype {
    pub shard: libc::c_int,
    pub clientPubSubChannels: Option::<unsafe extern "C" fn(*mut client) -> *mut dict>,
    pub subscriptionCount: Option::<unsafe extern "C" fn(*mut client) -> libc::c_int>,
    pub serverPubSubChannels: *mut *mut dict,
    pub subscribeMsg: *mut *mut robj,
    pub unsubscribeMsg: *mut *mut robj,
    pub messageBulk: *mut *mut robj,
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
pub static mut pubSubType: pubsubtype = pubsubtype {
    shard: 0,
    clientPubSubChannels: None,
    subscriptionCount: None,
    serverPubSubChannels: 0 as *const *mut dict as *mut *mut dict,
    subscribeMsg: 0 as *const *mut robj as *mut *mut robj,
    unsubscribeMsg: 0 as *const *mut robj as *mut *mut robj,
    messageBulk: 0 as *const *mut robj as *mut *mut robj,
};
#[no_mangle]
pub static mut pubSubShardType: pubsubtype = pubsubtype {
    shard: 0,
    clientPubSubChannels: None,
    subscriptionCount: None,
    serverPubSubChannels: 0 as *const *mut dict as *mut *mut dict,
    subscribeMsg: 0 as *const *mut robj as *mut *mut robj,
    unsubscribeMsg: 0 as *const *mut robj as *mut *mut robj,
    messageBulk: 0 as *const *mut robj as *mut *mut robj,
};
#[no_mangle]
pub unsafe extern "C" fn addReplyPubsubMessage(
    mut c: *mut client,
    mut channel: *mut robj,
    mut msg: *mut robj,
    mut message_bulk: *mut robj,
) {
    if (*c).resp == 2 as libc::c_int {
        addReply(c, shared.mbulkhdr[3 as libc::c_int as usize]);
    } else {
        addReplyPushLen(c, 3 as libc::c_int as libc::c_long);
    }
    addReply(c, message_bulk);
    addReplyBulk(c, channel);
    if !msg.is_null() {
        addReplyBulk(c, msg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn addReplyPubsubPatMessage(
    mut c: *mut client,
    mut pat: *mut robj,
    mut channel: *mut robj,
    mut msg: *mut robj,
) {
    if (*c).resp == 2 as libc::c_int {
        addReply(c, shared.mbulkhdr[4 as libc::c_int as usize]);
    } else {
        addReplyPushLen(c, 4 as libc::c_int as libc::c_long);
    }
    addReply(c, shared.pmessagebulk);
    addReplyBulk(c, pat);
    addReplyBulk(c, channel);
    addReplyBulk(c, msg);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyPubsubSubscribed(
    mut c: *mut client,
    mut channel: *mut robj,
    mut type_0: pubsubtype,
) {
    if (*c).resp == 2 as libc::c_int {
        addReply(c, shared.mbulkhdr[3 as libc::c_int as usize]);
    } else {
        addReplyPushLen(c, 3 as libc::c_int as libc::c_long);
    }
    addReply(c, *type_0.subscribeMsg);
    addReplyBulk(c, channel);
    addReplyLongLong(
        c,
        (type_0.subscriptionCount).expect("non-null function pointer")(c)
            as libc::c_longlong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addReplyPubsubUnsubscribed(
    mut c: *mut client,
    mut channel: *mut robj,
    mut type_0: pubsubtype,
) {
    if (*c).resp == 2 as libc::c_int {
        addReply(c, shared.mbulkhdr[3 as libc::c_int as usize]);
    } else {
        addReplyPushLen(c, 3 as libc::c_int as libc::c_long);
    }
    addReply(c, *type_0.unsubscribeMsg);
    if !channel.is_null() {
        addReplyBulk(c, channel);
    } else {
        addReplyNull(c);
    }
    addReplyLongLong(
        c,
        (type_0.subscriptionCount).expect("non-null function pointer")(c)
            as libc::c_longlong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addReplyPubsubPatSubscribed(
    mut c: *mut client,
    mut pattern: *mut robj,
) {
    if (*c).resp == 2 as libc::c_int {
        addReply(c, shared.mbulkhdr[3 as libc::c_int as usize]);
    } else {
        addReplyPushLen(c, 3 as libc::c_int as libc::c_long);
    }
    addReply(c, shared.psubscribebulk);
    addReplyBulk(c, pattern);
    addReplyLongLong(c, clientSubscriptionsCount(c) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyPubsubPatUnsubscribed(
    mut c: *mut client,
    mut pattern: *mut robj,
) {
    if (*c).resp == 2 as libc::c_int {
        addReply(c, shared.mbulkhdr[3 as libc::c_int as usize]);
    } else {
        addReplyPushLen(c, 3 as libc::c_int as libc::c_long);
    }
    addReply(c, shared.punsubscribebulk);
    if !pattern.is_null() {
        addReplyBulk(c, pattern);
    } else {
        addReplyNull(c);
    }
    addReplyLongLong(c, clientSubscriptionsCount(c) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn serverPubsubSubscriptionCount() -> libc::c_int {
    return ((*server.pubsub_channels).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*server.pubsub_channels).ht_used[1 as libc::c_int as usize])
        .wrapping_add(
            ((*server.pubsub_patterns).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*server.pubsub_patterns).ht_used[1 as libc::c_int as usize],
                ),
        ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn serverPubsubShardSubscriptionCount() -> libc::c_int {
    return ((*server.pubsubshard_channels).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*server.pubsubshard_channels).ht_used[1 as libc::c_int as usize])
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clientSubscriptionsCount(mut c: *mut client) -> libc::c_int {
    return ((*(*c).pubsub_channels).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*c).pubsub_channels).ht_used[1 as libc::c_int as usize])
        .wrapping_add((*(*c).pubsub_patterns).len) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clientShardSubscriptionsCount(
    mut c: *mut client,
) -> libc::c_int {
    return ((*(*c).pubsubshard_channels).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*c).pubsubshard_channels).ht_used[1 as libc::c_int as usize])
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getClientPubSubChannels(mut c: *mut client) -> *mut dict {
    return (*c).pubsub_channels;
}
#[no_mangle]
pub unsafe extern "C" fn getClientPubSubShardChannels(mut c: *mut client) -> *mut dict {
    return (*c).pubsubshard_channels;
}
#[no_mangle]
pub unsafe extern "C" fn clientTotalPubSubSubscriptionCount(
    mut c: *mut client,
) -> libc::c_int {
    return clientSubscriptionsCount(c) + clientShardSubscriptionsCount(c);
}
#[no_mangle]
pub unsafe extern "C" fn pubsubSubscribeChannel(
    mut c: *mut client,
    mut channel: *mut robj,
    mut type_0: pubsubtype,
) -> libc::c_int {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut clients: *mut list = 0 as *mut list;
    let mut retval: libc::c_int = 0 as libc::c_int;
    if dictAdd(
        (type_0.clientPubSubChannels).expect("non-null function pointer")(c),
        channel as *mut libc::c_void,
        0 as *mut libc::c_void,
    ) == 0 as libc::c_int
    {
        retval = 1 as libc::c_int;
        incrRefCount(channel);
        de = dictFind(*type_0.serverPubSubChannels, channel as *const libc::c_void);
        if de.is_null() {
            clients = listCreate();
            dictAdd(
                *type_0.serverPubSubChannels,
                channel as *mut libc::c_void,
                clients as *mut libc::c_void,
            );
            incrRefCount(channel);
        } else {
            clients = (*de).v.val as *mut list;
        }
        listAddNodeTail(clients, c as *mut libc::c_void);
    }
    addReplyPubsubSubscribed(c, channel, type_0);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubUnsubscribeChannel(
    mut c: *mut client,
    mut channel: *mut robj,
    mut notify: libc::c_int,
    mut type_0: pubsubtype,
) -> libc::c_int {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut clients: *mut list = 0 as *mut list;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut retval: libc::c_int = 0 as libc::c_int;
    incrRefCount(channel);
    if dictDelete(
        (type_0.clientPubSubChannels).expect("non-null function pointer")(c),
        channel as *const libc::c_void,
    ) == 0 as libc::c_int
    {
        retval = 1 as libc::c_int;
        de = dictFind(*type_0.serverPubSubChannels, channel as *const libc::c_void);
        if !de.is_null() {} else {
            _serverAssertWithInfo(
                c,
                0 as *const robj,
                b"de != NULL\0" as *const u8 as *const libc::c_char,
                b"pubsub.c\0" as *const u8 as *const libc::c_char,
                268 as libc::c_int,
            );
            unreachable!();
        };
        clients = (*de).v.val as *mut list;
        ln = listSearchKey(clients, c as *mut libc::c_void);
        if !ln.is_null() {} else {
            _serverAssertWithInfo(
                c,
                0 as *const robj,
                b"ln != NULL\0" as *const u8 as *const libc::c_char,
                b"pubsub.c\0" as *const u8 as *const libc::c_char,
                271 as libc::c_int,
            );
            unreachable!();
        };
        listDelNode(clients, ln);
        if (*clients).len == 0 as libc::c_int as libc::c_ulong {
            dictDelete(*type_0.serverPubSubChannels, channel as *const libc::c_void);
            if server.cluster_enabled & type_0.shard != 0 {
                slotToChannelDel((*channel).ptr as sds);
            }
        }
    }
    if notify != 0 {
        addReplyPubsubUnsubscribed(c, channel, type_0);
    }
    decrRefCount(channel);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubShardUnsubscribeAllClients(mut channel: *mut robj) {
    let mut retval: libc::c_int = 0;
    let mut de: *mut dictEntry = dictFind(
        server.pubsubshard_channels,
        channel as *const libc::c_void,
    );
    if !de.is_null() {} else {
        _serverAssertWithInfo(
            0 as *const client,
            channel,
            b"de != NULL\0" as *const u8 as *const libc::c_char,
            b"pubsub.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int,
        );
        unreachable!();
    };
    let mut clients: *mut list = (*de).v.val as *mut list;
    if (*clients).len > 0 as libc::c_int as libc::c_ulong {
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind(clients, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut c: *mut client = (*ln).value as *mut client;
            retval = dictDelete(
                (*c).pubsubshard_channels,
                channel as *const libc::c_void,
            );
            if retval == 0 as libc::c_int {} else {
                _serverAssertWithInfo(
                    c,
                    channel,
                    b"retval == DICT_OK\0" as *const u8 as *const libc::c_char,
                    b"pubsub.c\0" as *const u8 as *const libc::c_char,
                    306 as libc::c_int,
                );
                unreachable!();
            };
            addReplyPubsubUnsubscribed(c, channel, pubSubShardType);
            if clientTotalPubSubSubscriptionCount(c) == 0 as libc::c_int {
                (*c).flags
                    &= !((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong;
            }
        }
    }
    retval = dictDelete(server.pubsubshard_channels, channel as *const libc::c_void);
    slotToChannelDel((*channel).ptr as sds);
    if retval == 0 as libc::c_int {} else {
        _serverAssertWithInfo(
            0 as *const client,
            channel,
            b"retval == DICT_OK\0" as *const u8 as *const libc::c_char,
            b"pubsub.c\0" as *const u8 as *const libc::c_char,
            319 as libc::c_int,
        );
        unreachable!();
    };
    decrRefCount(channel);
}
#[no_mangle]
pub unsafe extern "C" fn pubsubSubscribePattern(
    mut c: *mut client,
    mut pattern: *mut robj,
) -> libc::c_int {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut clients: *mut list = 0 as *mut list;
    let mut retval: libc::c_int = 0 as libc::c_int;
    if (listSearchKey((*c).pubsub_patterns, pattern as *mut libc::c_void)).is_null() {
        retval = 1 as libc::c_int;
        listAddNodeTail((*c).pubsub_patterns, pattern as *mut libc::c_void);
        incrRefCount(pattern);
        de = dictFind(server.pubsub_patterns, pattern as *const libc::c_void);
        if de.is_null() {
            clients = listCreate();
            dictAdd(
                server.pubsub_patterns,
                pattern as *mut libc::c_void,
                clients as *mut libc::c_void,
            );
            incrRefCount(pattern);
        } else {
            clients = (*de).v.val as *mut list;
        }
        listAddNodeTail(clients, c as *mut libc::c_void);
    }
    addReplyPubsubPatSubscribed(c, pattern);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubUnsubscribePattern(
    mut c: *mut client,
    mut pattern: *mut robj,
    mut notify: libc::c_int,
) -> libc::c_int {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut clients: *mut list = 0 as *mut list;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut retval: libc::c_int = 0 as libc::c_int;
    incrRefCount(pattern);
    ln = listSearchKey((*c).pubsub_patterns, pattern as *mut libc::c_void);
    if !ln.is_null() {
        retval = 1 as libc::c_int;
        listDelNode((*c).pubsub_patterns, ln);
        de = dictFind(server.pubsub_patterns, pattern as *const libc::c_void);
        if !de.is_null() {} else {
            _serverAssertWithInfo(
                c,
                0 as *const robj,
                b"de != NULL\0" as *const u8 as *const libc::c_char,
                b"pubsub.c\0" as *const u8 as *const libc::c_char,
                364 as libc::c_int,
            );
            unreachable!();
        };
        clients = (*de).v.val as *mut list;
        ln = listSearchKey(clients, c as *mut libc::c_void);
        if !ln.is_null() {} else {
            _serverAssertWithInfo(
                c,
                0 as *const robj,
                b"ln != NULL\0" as *const u8 as *const libc::c_char,
                b"pubsub.c\0" as *const u8 as *const libc::c_char,
                367 as libc::c_int,
            );
            unreachable!();
        };
        listDelNode(clients, ln);
        if (*clients).len == 0 as libc::c_int as libc::c_ulong {
            dictDelete(server.pubsub_patterns, pattern as *const libc::c_void);
        }
    }
    if notify != 0 {
        addReplyPubsubPatUnsubscribed(c, pattern);
    }
    decrRefCount(pattern);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubUnsubscribeAllChannelsInternal(
    mut c: *mut client,
    mut notify: libc::c_int,
    mut type_0: pubsubtype,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    if ((*(type_0.clientPubSubChannels).expect("non-null function pointer")(c))
        .ht_used[0 as libc::c_int as usize])
        .wrapping_add(
            (*(type_0.clientPubSubChannels).expect("non-null function pointer")(c))
                .ht_used[1 as libc::c_int as usize],
        ) > 0 as libc::c_int as libc::c_ulong
    {
        let mut di: *mut dictIterator = dictGetSafeIterator(
            (type_0.clientPubSubChannels).expect("non-null function pointer")(c),
        );
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut channel: *mut robj = (*de).key as *mut robj;
            count += pubsubUnsubscribeChannel(c, channel, notify, type_0);
        }
        dictReleaseIterator(di);
    }
    if notify != 0 && count == 0 as libc::c_int {
        addReplyPubsubUnsubscribed(c, 0 as *mut robj, type_0);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubUnsubscribeAllChannels(
    mut c: *mut client,
    mut notify: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = pubsubUnsubscribeAllChannelsInternal(
        c,
        notify,
        pubSubType,
    );
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubUnsubscribeShardAllChannels(
    mut c: *mut client,
    mut notify: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = pubsubUnsubscribeAllChannelsInternal(
        c,
        notify,
        pubSubShardType,
    );
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubUnsubscribeShardChannels(
    mut channels: *mut *mut robj,
    mut count: libc::c_uint,
) {
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while j < count {
        pubsubShardUnsubscribeAllClients(*channels.offset(j as isize));
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pubsubUnsubscribeAllPatterns(
    mut c: *mut client,
    mut notify: libc::c_int,
) -> libc::c_int {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut count: libc::c_int = 0 as libc::c_int;
    listRewind((*c).pubsub_patterns, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut pattern: *mut robj = (*ln).value as *mut robj;
        count += pubsubUnsubscribePattern(c, pattern, notify);
    }
    if notify != 0 && count == 0 as libc::c_int {
        addReplyPubsubPatUnsubscribed(c, 0 as *mut robj);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubPublishMessageInternal(
    mut channel: *mut robj,
    mut message: *mut robj,
    mut type_0: pubsubtype,
) -> libc::c_int {
    let mut receivers: libc::c_int = 0 as libc::c_int;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    de = dictFind(*type_0.serverPubSubChannels, channel as *const libc::c_void);
    if !de.is_null() {
        let mut list: *mut list = (*de).v.val as *mut list;
        let mut ln_0: *mut listNode = 0 as *mut listNode;
        let mut li_0: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        listRewind(list, &mut li_0);
        loop {
            ln_0 = listNext(&mut li_0);
            if ln_0.is_null() {
                break;
            }
            let mut c: *mut client = (*ln_0).value as *mut client;
            addReplyPubsubMessage(c, channel, message, *type_0.messageBulk);
            updateClientMemUsageAndBucket(c);
            receivers += 1;
        }
    }
    if type_0.shard != 0 {
        return receivers;
    }
    di = dictGetIterator(server.pubsub_patterns);
    if !di.is_null() {
        channel = getDecodedObject(channel);
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut pattern: *mut robj = (*de).key as *mut robj;
            let mut clients: *mut list = (*de).v.val as *mut list;
            if stringmatchlen(
                (*pattern).ptr as *mut libc::c_char,
                sdslen((*pattern).ptr as sds) as libc::c_int,
                (*channel).ptr as *mut libc::c_char,
                sdslen((*channel).ptr as sds) as libc::c_int,
                0 as libc::c_int,
            ) == 0
            {
                continue;
            }
            listRewind(clients, &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut c_0: *mut client = (*ln).value as *mut client;
                addReplyPubsubPatMessage(c_0, pattern, channel, message);
                updateClientMemUsageAndBucket(c_0);
                receivers += 1;
            }
        }
        decrRefCount(channel);
        dictReleaseIterator(di);
    }
    return receivers;
}
#[no_mangle]
pub unsafe extern "C" fn pubsubPublishMessage(
    mut channel: *mut robj,
    mut message: *mut robj,
    mut sharded: libc::c_int,
) -> libc::c_int {
    return pubsubPublishMessageInternal(
        channel,
        message,
        if sharded != 0 { pubSubShardType } else { pubSubType },
    );
}
#[no_mangle]
pub unsafe extern "C" fn subscribeCommand(mut c: *mut client) {
    let mut j: libc::c_int = 0;
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 41 as libc::c_int
        != 0
        && (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong == 0
    {
        addReplyError(
            c,
            b"SUBSCRIBE isn't allowed for a DENY BLOCKING client\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    j = 1 as libc::c_int;
    while j < (*c).argc {
        pubsubSubscribeChannel(c, *((*c).argv).offset(j as isize), pubSubType);
        j += 1;
    }
    (*c).flags |= ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn unsubscribeCommand(mut c: *mut client) {
    if (*c).argc == 1 as libc::c_int {
        pubsubUnsubscribeAllChannels(c, 1 as libc::c_int);
    } else {
        let mut j: libc::c_int = 0;
        j = 1 as libc::c_int;
        while j < (*c).argc {
            pubsubUnsubscribeChannel(
                c,
                *((*c).argv).offset(j as isize),
                1 as libc::c_int,
                pubSubType,
            );
            j += 1;
        }
    }
    if clientTotalPubSubSubscriptionCount(c) == 0 as libc::c_int {
        (*c).flags &= !((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn psubscribeCommand(mut c: *mut client) {
    let mut j: libc::c_int = 0;
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 41 as libc::c_int
        != 0
        && (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong == 0
    {
        addReplyError(
            c,
            b"PSUBSCRIBE isn't allowed for a DENY BLOCKING client\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    j = 1 as libc::c_int;
    while j < (*c).argc {
        pubsubSubscribePattern(c, *((*c).argv).offset(j as isize));
        j += 1;
    }
    (*c).flags |= ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn punsubscribeCommand(mut c: *mut client) {
    if (*c).argc == 1 as libc::c_int {
        pubsubUnsubscribeAllPatterns(c, 1 as libc::c_int);
    } else {
        let mut j: libc::c_int = 0;
        j = 1 as libc::c_int;
        while j < (*c).argc {
            pubsubUnsubscribePattern(
                c,
                *((*c).argv).offset(j as isize),
                1 as libc::c_int,
            );
            j += 1;
        }
    }
    if clientTotalPubSubSubscriptionCount(c) == 0 as libc::c_int {
        (*c).flags &= !((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pubsubPublishMessageAndPropagateToCluster(
    mut channel: *mut robj,
    mut message: *mut robj,
    mut sharded: libc::c_int,
) -> libc::c_int {
    let mut receivers: libc::c_int = pubsubPublishMessage(channel, message, sharded);
    if server.cluster_enabled != 0 {
        clusterPropagatePublish(channel, message, sharded);
    }
    return receivers;
}
#[no_mangle]
pub unsafe extern "C" fn publishCommand(mut c: *mut client) {
    if server.sentinel_mode != 0 {
        sentinelPublishCommand(c);
        return;
    }
    let mut receivers: libc::c_int = pubsubPublishMessageAndPropagateToCluster(
        *((*c).argv).offset(1 as libc::c_int as isize),
        *((*c).argv).offset(2 as libc::c_int as isize),
        0 as libc::c_int,
    );
    if server.cluster_enabled == 0 {
        forceCommandPropagation(c, 2 as libc::c_int);
    }
    addReplyLongLong(c, receivers as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn pubsubCommand(mut c: *mut client) {
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut help: [*const libc::c_char; 12] = [
            b"CHANNELS [<pattern>]\0" as *const u8 as *const libc::c_char,
            b"    Return the currently active channels matching a <pattern> (default: '*').\0"
                as *const u8 as *const libc::c_char,
            b"NUMPAT\0" as *const u8 as *const libc::c_char,
            b"    Return number of subscriptions to patterns.\0" as *const u8
                as *const libc::c_char,
            b"NUMSUB [<channel> ...]\0" as *const u8 as *const libc::c_char,
            b"    Return the number of subscribers for the specified channels, excluding\0"
                as *const u8 as *const libc::c_char,
            b"    pattern subscriptions(default: no channels).\0" as *const u8
                as *const libc::c_char,
            b"SHARDCHANNELS [<pattern>]\0" as *const u8 as *const libc::c_char,
            b"    Return the currently active shard level channels matching a <pattern> (default: '*').\0"
                as *const u8 as *const libc::c_char,
            b"SHARDNUMSUB [<shardchannel> ...]\0" as *const u8 as *const libc::c_char,
            b"    Return the number of subscribers for the specified shard level channel(s)\0"
                as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"channels\0" as *const u8 as *const libc::c_char,
    ) == 0 && ((*c).argc == 2 as libc::c_int || (*c).argc == 3 as libc::c_int)
    {
        let mut pat: sds = (if (*c).argc == 2 as libc::c_int {
            0 as *mut libc::c_void
        } else {
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
        }) as sds;
        channelList(c, pat, server.pubsub_channels);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"numsub\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 2 as libc::c_int
    {
        let mut j: libc::c_int = 0;
        addReplyArrayLen(
            c,
            (((*c).argc - 2 as libc::c_int) * 2 as libc::c_int) as libc::c_long,
        );
        j = 2 as libc::c_int;
        while j < (*c).argc {
            let mut l: *mut list = dictFetchValue(
                server.pubsub_channels,
                *((*c).argv).offset(j as isize) as *const libc::c_void,
            ) as *mut list;
            addReplyBulk(c, *((*c).argv).offset(j as isize));
            addReplyLongLong(
                c,
                (if !l.is_null() { (*l).len } else { 0 as libc::c_int as libc::c_ulong })
                    as libc::c_longlong,
            );
            j += 1;
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"numpat\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        addReplyLongLong(
            c,
            ((*server.pubsub_patterns).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*server.pubsub_patterns).ht_used[1 as libc::c_int as usize],
                ) as libc::c_longlong,
        );
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"shardchannels\0" as *const u8 as *const libc::c_char,
    ) == 0 && ((*c).argc == 2 as libc::c_int || (*c).argc == 3 as libc::c_int)
    {
        let mut pat_0: sds = (if (*c).argc == 2 as libc::c_int {
            0 as *mut libc::c_void
        } else {
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
        }) as sds;
        channelList(c, pat_0, server.pubsubshard_channels);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"shardnumsub\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 2 as libc::c_int
    {
        let mut j_0: libc::c_int = 0;
        addReplyArrayLen(
            c,
            (((*c).argc - 2 as libc::c_int) * 2 as libc::c_int) as libc::c_long,
        );
        j_0 = 2 as libc::c_int;
        while j_0 < (*c).argc {
            let mut l_0: *mut list = dictFetchValue(
                server.pubsubshard_channels,
                *((*c).argv).offset(j_0 as isize) as *const libc::c_void,
            ) as *mut list;
            addReplyBulk(c, *((*c).argv).offset(j_0 as isize));
            addReplyLongLong(
                c,
                (if !l_0.is_null() {
                    (*l_0).len
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as libc::c_longlong,
            );
            j_0 += 1;
        }
    } else {
        addReplySubcommandSyntaxError(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn channelList(
    mut c: *mut client,
    mut pat: sds,
    mut pubsub_channels: *mut dict,
) {
    let mut di: *mut dictIterator = dictGetIterator(pubsub_channels);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut mblen: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut replylen: *mut libc::c_void = 0 as *mut libc::c_void;
    replylen = addReplyDeferredLen(c);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cobj: *mut robj = (*de).key as *mut robj;
        let mut channel: sds = (*cobj).ptr as sds;
        if pat.is_null()
            || stringmatchlen(
                pat as *const libc::c_char,
                sdslen(pat) as libc::c_int,
                channel as *const libc::c_char,
                sdslen(channel) as libc::c_int,
                0 as libc::c_int,
            ) != 0
        {
            addReplyBulk(c, cobj);
            mblen += 1;
        }
    }
    dictReleaseIterator(di);
    setDeferredArrayLen(c, replylen, mblen);
}
#[no_mangle]
pub unsafe extern "C" fn spublishCommand(mut c: *mut client) {
    let mut receivers: libc::c_int = pubsubPublishMessageAndPropagateToCluster(
        *((*c).argv).offset(1 as libc::c_int as isize),
        *((*c).argv).offset(2 as libc::c_int as isize),
        1 as libc::c_int,
    );
    if server.cluster_enabled == 0 {
        forceCommandPropagation(c, 2 as libc::c_int);
    }
    addReplyLongLong(c, receivers as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn ssubscribeCommand(mut c: *mut client) {
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 41 as libc::c_int
        != 0
    {
        addReplyError(
            c,
            b"SSUBSCRIBE isn't allowed for a DENY BLOCKING client\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < (*c).argc {
        if server.cluster_enabled
            & (dictFind(
                *pubSubShardType.serverPubSubChannels,
                *((*c).argv).offset(j as isize) as *const libc::c_void,
            ) == 0 as *mut libc::c_void as *mut dictEntry) as libc::c_int != 0
        {
            slotToChannelAdd((**((*c).argv).offset(j as isize)).ptr as sds);
        }
        pubsubSubscribeChannel(c, *((*c).argv).offset(j as isize), pubSubShardType);
        j += 1;
    }
    (*c).flags |= ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn sunsubscribeCommand(mut c: *mut client) {
    if (*c).argc == 1 as libc::c_int {
        pubsubUnsubscribeShardAllChannels(c, 1 as libc::c_int);
    } else {
        let mut j: libc::c_int = 1 as libc::c_int;
        while j < (*c).argc {
            pubsubUnsubscribeChannel(
                c,
                *((*c).argv).offset(j as isize),
                1 as libc::c_int,
                pubSubShardType,
            );
            j += 1;
        }
    }
    if clientTotalPubSubSubscriptionCount(c) == 0 as libc::c_int {
        (*c).flags &= !((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pubsubMemOverhead(mut c: *mut client) -> size_t {
    let mut mem: size_t = ((*(*c).pubsub_patterns).len)
        .wrapping_mul(core::mem::size_of::<listNode>() as libc::c_ulong);
    mem = (mem as libc::c_ulong)
        .wrapping_add(
            ((*(*c).pubsub_channels).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*c).pubsub_channels).ht_used[1 as libc::c_int as usize])
                .wrapping_mul(core::mem::size_of::<dictEntry>() as libc::c_ulong)
                .wrapping_add(
                    (if (*(*c).pubsub_channels).ht_size_exp[0 as libc::c_int as usize]
                        as libc::c_int == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*(*c).pubsub_channels)
                                .ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                    })
                        .wrapping_add(
                            (if (*(*c).pubsub_channels)
                                .ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                                == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*(*c).pubsub_channels)
                                        .ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                            }),
                        )
                        .wrapping_mul(
                            core::mem::size_of::<*mut dictEntry>() as libc::c_ulong,
                        ),
                ),
        ) as size_t as size_t;
    mem = (mem as libc::c_ulong)
        .wrapping_add(
            ((*(*c).pubsubshard_channels).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*(*c).pubsubshard_channels).ht_used[1 as libc::c_int as usize],
                )
                .wrapping_mul(core::mem::size_of::<dictEntry>() as libc::c_ulong)
                .wrapping_add(
                    (if (*(*c).pubsubshard_channels)
                        .ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                        == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*(*c).pubsubshard_channels)
                                .ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                    })
                        .wrapping_add(
                            (if (*(*c).pubsubshard_channels)
                                .ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                                == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*(*c).pubsubshard_channels)
                                        .ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                            }),
                        )
                        .wrapping_mul(
                            core::mem::size_of::<*mut dictEntry>() as libc::c_ulong,
                        ),
                ),
        ) as size_t as size_t;
    return mem;
}
unsafe extern "C" fn run_static_initializers() {
    pubSubType = {
        let mut init = pubsubtype {
            shard: 0 as libc::c_int,
            clientPubSubChannels: Some(
                getClientPubSubChannels as unsafe extern "C" fn(*mut client) -> *mut dict,
            ),
            subscriptionCount: Some(
                clientSubscriptionsCount
                    as unsafe extern "C" fn(*mut client) -> libc::c_int,
            ),
            serverPubSubChannels: &mut server.pubsub_channels,
            subscribeMsg: &mut shared.subscribebulk,
            unsubscribeMsg: &mut shared.unsubscribebulk,
            messageBulk: &mut shared.messagebulk,
        };
        init
    };
    pubSubShardType = {
        let mut init = pubsubtype {
            shard: 1 as libc::c_int,
            clientPubSubChannels: Some(
                getClientPubSubShardChannels
                    as unsafe extern "C" fn(*mut client) -> *mut dict,
            ),
            subscriptionCount: Some(
                clientShardSubscriptionsCount
                    as unsafe extern "C" fn(*mut client) -> libc::c_int,
            ),
            serverPubSubChannels: &mut server.pubsubshard_channels,
            subscribeMsg: &mut shared.ssubscribebulk,
            unsubscribeMsg: &mut shared.sunsubscribebulk,
            messageBulk: &mut shared.smessagebulk,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
