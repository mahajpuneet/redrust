extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    fn incrRefCount(o: *mut robj);
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdsMakeRoomFor(s: sds, addlen: size_t) -> sds;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn listEmpty(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
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
    fn raxRandomWalk(it: *mut raxIterator, steps: size_t) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    fn raxEOF(it: *mut raxIterator) -> libc::c_int;
    fn raxSize(rax: *mut rax) -> uint64_t;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn addReplyProto(c: *mut client, s: *const libc::c_char, len: size_t);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyPushLen(c: *mut client, length: libc::c_long);
    fn updateClientMemUsageAndBucket(c: *mut client) -> libc::c_int;
    fn lookupClientByID(id: uint64_t) -> *mut client;
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn decrRefCount(o: *mut robj);
    fn freeTrackingRadixTreeAsync(rt: *mut rax);
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn addReplyPubsubMessage(
        c: *mut client,
        channel: *mut robj,
        msg: *mut robj,
        message_bulk: *mut robj,
    );
    fn getKeysFromCommand(
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        result: *mut getKeysResult,
    ) -> libc::c_int;
    fn getKeysFreeResult(result: *mut getKeysResult);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bcastState {
    pub keys: *mut rax,
    pub clients: *mut rax,
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
pub static mut TrackingTable: *mut rax = 0 as *const rax as *mut rax;
#[no_mangle]
pub static mut PrefixTable: *mut rax = 0 as *const rax as *mut rax;
#[no_mangle]
pub static mut TrackingTableTotalItems: uint64_t = 0 as libc::c_int as uint64_t;
#[no_mangle]
pub static mut TrackingChannelName: *mut robj = 0 as *const robj as *mut robj;
#[no_mangle]
pub unsafe extern "C" fn disableTracking(mut c: *mut client) {
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 33 as libc::c_int
        != 0
    {
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
        raxStart(&mut ri, (*c).client_tracking_prefixes);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        while raxNext(&mut ri) != 0 {
            let mut bs: *mut bcastState = raxFind(PrefixTable, ri.key, ri.key_len)
                as *mut bcastState;
            if bs != raxNotFound as *mut bcastState {} else {
                _serverAssert(
                    b"bs != raxNotFound\0" as *const u8 as *const libc::c_char,
                    b"tracking.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                );
                unreachable!();
            };
            raxRemove(
                (*bs).clients,
                &mut c as *mut *mut client as *mut libc::c_uchar,
                core::mem::size_of::<*mut client>() as libc::c_ulong,
                0 as *mut *mut libc::c_void,
            );
            if raxSize((*bs).clients) == 0 as libc::c_int as libc::c_ulong {
                raxFree((*bs).clients);
                raxFree((*bs).keys);
                zfree(bs as *mut libc::c_void);
                raxRemove(PrefixTable, ri.key, ri.key_len, 0 as *mut *mut libc::c_void);
            }
        }
        raxStop(&mut ri);
        raxFree((*c).client_tracking_prefixes);
        (*c).client_tracking_prefixes = 0 as *mut rax;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 31 as libc::c_int
        != 0
    {
        server.tracking_clients = (server.tracking_clients).wrapping_sub(1);
        (*c)
            .flags = ((*c).flags as libc::c_ulonglong
            & !((1 as libc::c_ulonglong) << 31 as libc::c_int
                | (1 as libc::c_ulonglong) << 32 as libc::c_int
                | (1 as libc::c_ulonglong) << 33 as libc::c_int
                | (1 as libc::c_ulonglong) << 34 as libc::c_int
                | (1 as libc::c_ulonglong) << 35 as libc::c_int
                | (1 as libc::c_ulonglong) << 36 as libc::c_int
                | (1 as libc::c_ulonglong) << 37 as libc::c_int)) as uint64_t;
    }
}
unsafe extern "C" fn stringCheckPrefix(
    mut s1: *mut libc::c_uchar,
    mut s1_len: size_t,
    mut s2: *mut libc::c_uchar,
    mut s2_len: size_t,
) -> libc::c_int {
    let mut min_length: size_t = if s1_len < s2_len { s1_len } else { s2_len };
    return (memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, min_length)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkPrefixCollisionsOrReply(
    mut c: *mut client,
    mut prefixes: *mut *mut robj,
    mut numprefix: size_t,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < numprefix {
        if !((*c).client_tracking_prefixes).is_null() {
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
            raxStart(&mut ri, (*c).client_tracking_prefixes);
            raxSeek(
                &mut ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri) != 0 {
                if stringCheckPrefix(
                    ri.key,
                    ri.key_len,
                    (**prefixes.offset(i as isize)).ptr as *mut libc::c_uchar,
                    sdslen((**prefixes.offset(i as isize)).ptr as sds),
                ) != 0
                {
                    let mut collision: sds = sdsnewlen(
                        ri.key as *const libc::c_void,
                        ri.key_len,
                    );
                    addReplyErrorFormat(
                        c,
                        b"Prefix '%s' overlaps with an existing prefix '%s'. Prefixes for a single client must not overlap.\0"
                            as *const u8 as *const libc::c_char,
                        (**prefixes.offset(i as isize)).ptr as *mut libc::c_uchar,
                        collision as *mut libc::c_uchar,
                    );
                    sdsfree(collision);
                    raxStop(&mut ri);
                    return 0 as libc::c_int;
                }
            }
            raxStop(&mut ri);
        }
        let mut j: size_t = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < numprefix {
            if stringCheckPrefix(
                (**prefixes.offset(i as isize)).ptr as *mut libc::c_uchar,
                sdslen((**prefixes.offset(i as isize)).ptr as sds),
                (**prefixes.offset(j as isize)).ptr as *mut libc::c_uchar,
                sdslen((**prefixes.offset(j as isize)).ptr as sds),
            ) != 0
            {
                addReplyErrorFormat(
                    c,
                    b"Prefix '%s' overlaps with another provided prefix '%s'. Prefixes for a single client must not overlap.\0"
                        as *const u8 as *const libc::c_char,
                    (**prefixes.offset(i as isize)).ptr as *mut libc::c_uchar,
                    (**prefixes.offset(j as isize)).ptr as *mut libc::c_uchar,
                );
                return i as libc::c_int;
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn enableBcastTrackingForPrefix(
    mut c: *mut client,
    mut prefix: *mut libc::c_char,
    mut plen: size_t,
) {
    let mut bs: *mut bcastState = raxFind(
        PrefixTable,
        prefix as *mut libc::c_uchar,
        plen,
    ) as *mut bcastState;
    if bs == raxNotFound as *mut bcastState {
        bs = zmalloc(core::mem::size_of::<bcastState>() as libc::c_ulong)
            as *mut bcastState;
        (*bs).keys = raxNew();
        (*bs).clients = raxNew();
        raxInsert(
            PrefixTable,
            prefix as *mut libc::c_uchar,
            plen,
            bs as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    if raxTryInsert(
        (*bs).clients,
        &mut c as *mut *mut client as *mut libc::c_uchar,
        core::mem::size_of::<*mut client>() as libc::c_ulong,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    ) != 0
    {
        if ((*c).client_tracking_prefixes).is_null() {
            (*c).client_tracking_prefixes = raxNew();
        }
        raxInsert(
            (*c).client_tracking_prefixes,
            prefix as *mut libc::c_uchar,
            plen,
            0 as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn enableTracking(
    mut c: *mut client,
    mut redirect_to: uint64_t,
    mut options: uint64_t,
    mut prefix: *mut *mut robj,
    mut numprefix: size_t,
) {
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 31 as libc::c_int
        == 0
    {
        server.tracking_clients = (server.tracking_clients).wrapping_add(1);
    }
    (*c)
        .flags = ((*c).flags as libc::c_ulonglong
        | (1 as libc::c_ulonglong) << 31 as libc::c_int) as uint64_t;
    (*c)
        .flags = ((*c).flags as libc::c_ulonglong
        & !((1 as libc::c_ulonglong) << 32 as libc::c_int
            | (1 as libc::c_ulonglong) << 33 as libc::c_int
            | (1 as libc::c_ulonglong) << 34 as libc::c_int
            | (1 as libc::c_ulonglong) << 35 as libc::c_int
            | (1 as libc::c_ulonglong) << 37 as libc::c_int)) as uint64_t;
    (*c).client_tracking_redirection = redirect_to;
    if TrackingTable.is_null() {
        TrackingTable = raxNew();
        PrefixTable = raxNew();
        TrackingChannelName = createStringObject(
            b"__redis__:invalidate\0" as *const u8 as *const libc::c_char,
            20 as libc::c_int as size_t,
        );
    }
    if options as libc::c_ulonglong & (1 as libc::c_ulonglong) << 33 as libc::c_int != 0
    {
        (*c)
            .flags = ((*c).flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 33 as libc::c_int) as uint64_t;
        if numprefix == 0 as libc::c_int as libc::c_ulong {
            enableBcastTrackingForPrefix(
                c,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < numprefix {
            let mut sdsprefix: sds = (**prefix.offset(j as isize)).ptr as sds;
            enableBcastTrackingForPrefix(c, sdsprefix, sdslen(sdsprefix));
            j = j.wrapping_add(1);
        }
    }
    (*c)
        .flags = ((*c).flags as libc::c_ulonglong
        | options as libc::c_ulonglong
            & ((1 as libc::c_ulonglong) << 34 as libc::c_int
                | (1 as libc::c_ulonglong) << 35 as libc::c_int
                | (1 as libc::c_ulonglong) << 37 as libc::c_int)) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn trackingRememberKeys(mut c: *mut client) {
    let mut optin: uint64_t = ((*c).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 34 as libc::c_int) as uint64_t;
    let mut optout: uint64_t = ((*c).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 35 as libc::c_int) as uint64_t;
    let mut caching_given: uint64_t = ((*c).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 36 as libc::c_int) as uint64_t;
    if optin != 0 && caching_given == 0 || optout != 0 && caching_given != 0 {
        return;
    }
    let mut result: getKeysResult = {
        let mut init = getKeysResult {
            keysbuf: [
                {
                    let mut init = keyReference {
                        pos: 0 as libc::c_int,
                        flags: 0,
                    };
                    init
                },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
            ],
            keys: 0 as *mut keyReference,
            numkeys: 0 as libc::c_int,
            size: 256 as libc::c_int,
        };
        init
    };
    let mut numkeys: libc::c_int = getKeysFromCommand(
        (*c).cmd,
        (*c).argv,
        (*c).argc,
        &mut result,
    );
    if numkeys == 0 {
        getKeysFreeResult(&mut result);
        return;
    }
    if (*(*c).cmd).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 5 as libc::c_int != 0
    {
        return;
    }
    let mut keys: *mut keyReference = result.keys;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < numkeys {
        let mut idx: libc::c_int = (*keys.offset(j as isize)).pos;
        let mut sdskey: sds = (**((*c).argv).offset(idx as isize)).ptr as sds;
        let mut ids: *mut rax = raxFind(
            TrackingTable,
            sdskey as *mut libc::c_uchar,
            sdslen(sdskey),
        ) as *mut rax;
        if ids == raxNotFound as *mut rax {
            ids = raxNew();
            let mut inserted: libc::c_int = raxTryInsert(
                TrackingTable,
                sdskey as *mut libc::c_uchar,
                sdslen(sdskey),
                ids as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            if inserted == 1 as libc::c_int {} else {
                _serverAssert(
                    b"inserted == 1\0" as *const u8 as *const libc::c_char,
                    b"tracking.c\0" as *const u8 as *const libc::c_char,
                    248 as libc::c_int,
                );
                unreachable!();
            };
        }
        if raxTryInsert(
            ids,
            &mut (*c).id as *mut uint64_t as *mut libc::c_uchar,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
            0 as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        ) != 0
        {
            TrackingTableTotalItems = TrackingTableTotalItems.wrapping_add(1);
        }
        j += 1;
    }
    getKeysFreeResult(&mut result);
}
#[no_mangle]
pub unsafe extern "C" fn sendTrackingMessage(
    mut c: *mut client,
    mut keyname: *mut libc::c_char,
    mut keylen: size_t,
    mut proto: libc::c_int,
) {
    let mut using_redirection: libc::c_int = 0 as libc::c_int;
    if (*c).client_tracking_redirection != 0 {
        let mut redir: *mut client = lookupClientByID((*c).client_tracking_redirection);
        if redir.is_null() {
            (*c)
                .flags = ((*c).flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 32 as libc::c_int) as uint64_t;
            if (*c).resp > 2 as libc::c_int {
                addReplyPushLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCBuffer(
                    c,
                    b"tracking-redir-broken\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    21 as libc::c_int as size_t,
                );
                addReplyLongLong(
                    c,
                    (*c).client_tracking_redirection as libc::c_longlong,
                );
            }
            return;
        }
        c = redir;
        using_redirection = 1 as libc::c_int;
    }
    if (*c).resp > 2 as libc::c_int {
        addReplyPushLen(c, 2 as libc::c_int as libc::c_long);
        addReplyBulkCBuffer(
            c,
            b"invalidate\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            10 as libc::c_int as size_t,
        );
    } else if using_redirection != 0
        && (*c).flags & ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0
    {
        addReplyPubsubMessage(
            c,
            TrackingChannelName,
            0 as *mut robj,
            shared.messagebulk,
        );
    } else {
        return
    }
    if proto != 0 {
        addReplyProto(c, keyname, keylen);
    } else {
        addReplyArrayLen(c, 1 as libc::c_int as libc::c_long);
        addReplyBulkCBuffer(c, keyname as *const libc::c_void, keylen);
    }
    updateClientMemUsageAndBucket(c);
}
#[no_mangle]
pub unsafe extern "C" fn trackingRememberKeyToBroadcast(
    mut c: *mut client,
    mut keyname: *mut libc::c_char,
    mut keylen: size_t,
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
    raxStart(&mut ri, PrefixTable);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        if ri.key_len > keylen {
            continue;
        }
        if ri.key_len != 0 as libc::c_int as libc::c_ulong
            && memcmp(
                ri.key as *const libc::c_void,
                keyname as *const libc::c_void,
                ri.key_len,
            ) != 0 as libc::c_int
        {
            continue;
        }
        let mut bs: *mut bcastState = ri.data as *mut bcastState;
        raxInsert(
            (*bs).keys,
            keyname as *mut libc::c_uchar,
            keylen,
            c as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    raxStop(&mut ri);
}
#[no_mangle]
pub unsafe extern "C" fn trackingInvalidateKey(
    mut c: *mut client,
    mut keyobj: *mut robj,
    mut bcast: libc::c_int,
) {
    if TrackingTable.is_null() {
        return;
    }
    let mut key: *mut libc::c_uchar = (*keyobj).ptr as *mut libc::c_uchar;
    let mut keylen: size_t = sdslen((*keyobj).ptr as sds);
    if bcast != 0 && raxSize(PrefixTable) > 0 as libc::c_int as libc::c_ulong {
        trackingRememberKeyToBroadcast(c, key as *mut libc::c_char, keylen);
    }
    let mut ids: *mut rax = raxFind(TrackingTable, key, keylen) as *mut rax;
    if ids == raxNotFound as *mut rax {
        return;
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
    raxStart(&mut ri, ids);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        let mut id: uint64_t = 0;
        memcpy(
            &mut id as *mut uint64_t as *mut libc::c_void,
            ri.key as *const libc::c_void,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
        );
        let mut target: *mut client = lookupClientByID(id);
        if target.is_null()
            || (*target).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 31 as libc::c_int == 0
            || (*target).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 33 as libc::c_int != 0
        {
            continue;
        }
        if (*target).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 37 as libc::c_int != 0
            && target == server.current_client
        {
            continue;
        }
        if target == server.current_client {
            incrRefCount(keyobj);
            listAddNodeTail(server.tracking_pending_keys, keyobj as *mut libc::c_void);
        } else {
            sendTrackingMessage(
                target,
                (*keyobj).ptr as *mut libc::c_char,
                sdslen((*keyobj).ptr as sds),
                0 as libc::c_int,
            );
        }
    }
    raxStop(&mut ri);
    TrackingTableTotalItems = (TrackingTableTotalItems as libc::c_ulong)
        .wrapping_sub(raxSize(ids)) as uint64_t as uint64_t;
    raxFree(ids);
    raxRemove(TrackingTable, key, keylen, 0 as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn trackingHandlePendingKeyInvalidations() {
    if (*server.tracking_pending_keys).len == 0 {
        return;
    }
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewind(server.tracking_pending_keys, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut key: *mut robj = (*ln).value as *mut robj;
        if !(server.current_client).is_null() {
            if !key.is_null() {
                sendTrackingMessage(
                    server.current_client,
                    (*key).ptr as *mut libc::c_char,
                    sdslen((*key).ptr as sds),
                    0 as libc::c_int,
                );
            } else {
                sendTrackingMessage(
                    server.current_client,
                    (*shared.null[(*server.current_client).resp as usize]).ptr
                        as *mut libc::c_char,
                    sdslen(
                        (*shared.null[(*server.current_client).resp as usize]).ptr as sds,
                    ),
                    1 as libc::c_int,
                );
            }
        }
        if !key.is_null() {
            decrRefCount(key);
        }
    }
    listEmpty(server.tracking_pending_keys);
}
#[no_mangle]
pub unsafe extern "C" fn freeTrackingRadixTreeCallback(mut rt: *mut libc::c_void) {
    raxFree(rt as *mut rax);
}
#[no_mangle]
pub unsafe extern "C" fn freeTrackingRadixTree(mut rt: *mut rax) {
    raxFreeWithCallback(
        rt,
        Some(
            freeTrackingRadixTreeCallback
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn trackingInvalidateKeysOnFlush(mut async_0: libc::c_int) {
    if server.tracking_clients != 0 {
        let mut ln: *mut listNode = 0 as *mut listNode;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        listRewind(server.clients, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut c: *mut client = (*ln).value as *mut client;
            if (*c).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
            {
                if c == server.current_client {
                    listAddNodeTail(
                        server.tracking_pending_keys,
                        0 as *mut libc::c_void,
                    );
                } else {
                    sendTrackingMessage(
                        c,
                        (*shared.null[(*c).resp as usize]).ptr as *mut libc::c_char,
                        sdslen((*shared.null[(*c).resp as usize]).ptr as sds),
                        1 as libc::c_int,
                    );
                }
            }
        }
    }
    if !TrackingTable.is_null() {
        if async_0 != 0 {
            freeTrackingRadixTreeAsync(TrackingTable);
        } else {
            freeTrackingRadixTree(TrackingTable);
        }
        TrackingTable = raxNew();
        TrackingTableTotalItems = 0 as libc::c_int as uint64_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn trackingLimitUsedSlots() {
    static mut timeout_counter: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if TrackingTable.is_null() {
        return;
    }
    if server.tracking_table_max_keys == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut max_keys: size_t = server.tracking_table_max_keys;
    if raxSize(TrackingTable) <= max_keys {
        timeout_counter = 0 as libc::c_int as libc::c_uint;
        return;
    }
    let mut effort: libc::c_int = (100 as libc::c_int as libc::c_uint)
        .wrapping_mul(timeout_counter.wrapping_add(1 as libc::c_int as libc::c_uint))
        as libc::c_int;
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
    raxStart(&mut ri, TrackingTable);
    while effort > 0 as libc::c_int {
        effort -= 1;
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        raxRandomWalk(&mut ri, 0 as libc::c_int as size_t);
        if raxEOF(&mut ri) != 0 {
            break;
        }
        let mut keyobj: *mut robj = createStringObject(
            ri.key as *mut libc::c_char,
            ri.key_len,
        );
        trackingInvalidateKey(0 as *mut client, keyobj, 0 as libc::c_int);
        decrRefCount(keyobj);
        if raxSize(TrackingTable) <= max_keys {
            timeout_counter = 0 as libc::c_int as libc::c_uint;
            raxStop(&mut ri);
            return;
        }
    }
    raxStop(&mut ri);
    timeout_counter = timeout_counter.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn trackingBuildBroadcastReply(
    mut c: *mut client,
    mut keys: *mut rax,
) -> sds {
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
    let mut count: uint64_t = 0;
    if c.is_null() {
        count = raxSize(keys);
    } else {
        count = 0 as libc::c_int as uint64_t;
        raxStart(&mut ri, keys);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        while raxNext(&mut ri) != 0 {
            if ri.data != c as *mut libc::c_void {
                count = count.wrapping_add(1);
            }
        }
        raxStop(&mut ri);
        if count == 0 as libc::c_int as libc::c_ulong {
            return 0 as sds;
        }
    }
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut len: size_t = ll2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        count as libc::c_longlong,
    ) as size_t;
    let mut proto: sds = sdsempty();
    proto = sdsMakeRoomFor(
        proto,
        count.wrapping_mul(15 as libc::c_int as libc::c_ulong),
    );
    proto = sdscatlen(
        proto,
        b"*\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    proto = sdscatlen(proto, buf.as_mut_ptr() as *const libc::c_void, len);
    proto = sdscatlen(
        proto,
        b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    raxStart(&mut ri, keys);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        if !c.is_null() && ri.data == c as *mut libc::c_void {
            continue;
        }
        len = ll2string(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            ri.key_len as libc::c_longlong,
        ) as size_t;
        proto = sdscatlen(
            proto,
            b"$\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        proto = sdscatlen(proto, buf.as_mut_ptr() as *const libc::c_void, len);
        proto = sdscatlen(
            proto,
            b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        proto = sdscatlen(proto, ri.key as *const libc::c_void, ri.key_len);
        proto = sdscatlen(
            proto,
            b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
    }
    raxStop(&mut ri);
    return proto;
}
#[no_mangle]
pub unsafe extern "C" fn trackingBroadcastInvalidationMessages() {
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
    let mut ri2: raxIterator = raxIterator {
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
    if TrackingTable.is_null() || server.tracking_clients == 0 {
        return;
    }
    raxStart(&mut ri, PrefixTable);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        let mut bs: *mut bcastState = ri.data as *mut bcastState;
        if raxSize((*bs).keys) != 0 {
            let mut proto: sds = trackingBuildBroadcastReply(
                0 as *mut client,
                (*bs).keys,
            );
            raxStart(&mut ri2, (*bs).clients);
            raxSeek(
                &mut ri2,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri2) != 0 {
                let mut c: *mut client = 0 as *mut client;
                memcpy(
                    &mut c as *mut *mut client as *mut libc::c_void,
                    ri2.key as *const libc::c_void,
                    core::mem::size_of::<*mut client>() as libc::c_ulong,
                );
                if (*c).flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 37 as libc::c_int != 0
                {
                    let mut adhoc: sds = trackingBuildBroadcastReply(c, (*bs).keys);
                    if !adhoc.is_null() {
                        sendTrackingMessage(c, adhoc, sdslen(adhoc), 1 as libc::c_int);
                        sdsfree(adhoc);
                    }
                } else {
                    sendTrackingMessage(c, proto, sdslen(proto), 1 as libc::c_int);
                }
            }
            raxStop(&mut ri2);
            sdsfree(proto);
        }
        raxFree((*bs).keys);
        (*bs).keys = raxNew();
    }
    raxStop(&mut ri);
}
#[no_mangle]
pub unsafe extern "C" fn trackingGetTotalItems() -> uint64_t {
    return TrackingTableTotalItems;
}
#[no_mangle]
pub unsafe extern "C" fn trackingGetTotalKeys() -> uint64_t {
    if TrackingTable.is_null() {
        return 0 as libc::c_int as uint64_t;
    }
    return raxSize(TrackingTable);
}
#[no_mangle]
pub unsafe extern "C" fn trackingGetTotalPrefixes() -> uint64_t {
    if PrefixTable.is_null() {
        return 0 as libc::c_int as uint64_t;
    }
    return raxSize(PrefixTable);
}
