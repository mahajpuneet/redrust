extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type RedisModuleCommand;
    fn sdsdup(s: sds) -> sds;
    fn sdscmp(s1: sds, s2: sds) -> libc::c_int;
    fn sdsAllocPtr(s: sds) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut getMonotonicUs: Option::<unsafe extern "C" fn() -> monotime>;
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictScan(
        d: *mut dict,
        v: libc::c_ulong,
        fn_0: Option::<dictScanFunction>,
        bucketfn: Option::<dictScanBucketFunction>,
        privdata: *mut libc::c_void,
    ) -> libc::c_ulong;
    fn dictGetHash(d: *mut dict, key: *const libc::c_void) -> uint64_t;
    fn dictFindEntryRefByPtrAndHash(
        d: *mut dict,
        oldptr: *const libc::c_void,
        hash: uint64_t,
    ) -> *mut *mut dictEntry;
    fn listEmpty(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
    fn zmalloc_get_allocator_info(
        allocated: *mut size_t,
        active: *mut size_t,
        resident: *mut size_t,
    ) -> libc::c_int;
    fn zfree_no_tcache(ptr: *mut libc::c_void);
    fn zmalloc_no_tcache(size: size_t) -> *mut libc::c_void;
    fn latencyAddSample(event: *const libc::c_char, latency: mstime_t);
    fn quicklistBookmarkCreate(
        ql_ref: *mut *mut quicklist,
        name: *const libc::c_char,
        node: *mut quicklistNode,
    ) -> libc::c_int;
    fn quicklistBookmarkDelete(
        ql: *mut quicklist,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn quicklistBookmarkFind(
        ql: *mut quicklist,
        name: *const libc::c_char,
    ) -> *mut quicklistNode;
    fn raxInsert(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        data: *mut libc::c_void,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxStart(it: *mut raxIterator, rt: *mut rax);
    fn raxSeek(
        it: *mut raxIterator,
        op: *const libc::c_char,
        ele: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn raxNext(it: *mut raxIterator) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    fn raxSize(rax: *mut rax) -> uint64_t;
    fn raxSetData(n: *mut raxNode, data: *mut libc::c_void);
    static mut server: redisServer;
    fn moduleDefragValue(
        key: *mut robj,
        obj: *mut robj,
        defragged: *mut libc::c_long,
        dbid: libc::c_int,
    ) -> libc::c_int;
    fn moduleLateDefrag(
        key: *mut robj,
        value: *mut robj,
        cursor: *mut libc::c_ulong,
        endtime: libc::c_longlong,
        defragged: *mut libc::c_longlong,
        dbid: libc::c_int,
    ) -> libc::c_int;
    fn moduleDefragGlobals() -> libc::c_long;
    fn ustime() -> libc::c_longlong;
    fn mstime() -> libc::c_longlong;
    fn hasActiveChildProcess() -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
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
    fn evalScriptsDict() -> *mut dict;
    fn slotToKeyReplaceEntry(entry: *mut dictEntry, db: *mut redisDb);
    fn je_get_defrag_hint(ptr: *mut libc::c_void) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uint_least64_t = __uint_least64_t;
pub type intptr_t = libc::c_long;
pub type sds = *mut libc::c_char;
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
pub struct quicklistNode {
    pub prev: *mut quicklistNode,
    pub next: *mut quicklistNode,
    pub entry: *mut libc::c_uchar,
    pub sz: size_t,
    #[bitfield(name = "count", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "16..=17")]
    #[bitfield(name = "container", ty = "libc::c_uint", bits = "18..=19")]
    #[bitfield(name = "recompress", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "attempted_compress", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "dont_compress", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "extra", ty = "libc::c_uint", bits = "23..=31")]
    pub count_encoding_container_recompress_attempted_compress_dont_compress_extra: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistBookmark {
    pub node: *mut quicklistNode,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct quicklist {
    pub head: *mut quicklistNode,
    pub tail: *mut quicklistNode,
    pub count: libc::c_ulong,
    pub len: libc::c_ulong,
    #[bitfield(name = "fill", ty = "libc::c_int", bits = "0..=15")]
    #[bitfield(name = "compress", ty = "libc::c_uint", bits = "16..=31")]
    #[bitfield(name = "bookmark_count", ty = "libc::c_uint", bits = "32..=35")]
    pub fill_compress_bookmark_count: [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub bookmarks: [quicklistBookmark; 0],
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
pub type raxDefragFunction = unsafe extern "C" fn(
    *mut raxIterator,
    *mut libc::c_void,
    *mut libc::c_long,
) -> *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PendingEntryContext {
    pub cg: *mut streamCG,
    pub c: *mut streamConsumer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaScript {
    pub flags: uint64_t,
    pub body: *mut robj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scanLaterZsetData {
    pub zs: *mut zset,
    pub defragged: libc::c_long,
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
#[no_mangle]
pub unsafe extern "C" fn activeDefragAlloc(
    mut ptr: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut size: size_t = 0;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if je_get_defrag_hint(ptr) == 0 {
        server.stat_active_defrag_misses += 1;
        return 0 as *mut libc::c_void;
    }
    size = je_malloc_usable_size(ptr);
    newptr = zmalloc_no_tcache(size);
    memcpy(newptr, ptr, size);
    zfree_no_tcache(ptr);
    return newptr;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragSds(mut sdsptr: sds) -> sds {
    let mut ptr: *mut libc::c_void = sdsAllocPtr(sdsptr);
    let mut newptr: *mut libc::c_void = activeDefragAlloc(ptr);
    if !newptr.is_null() {
        let mut offset: size_t = sdsptr.offset_from(ptr as *mut libc::c_char)
            as libc::c_long as size_t;
        sdsptr = (newptr as *mut libc::c_char).offset(offset as isize);
        return sdsptr;
    }
    return 0 as sds;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragStringOb(
    mut ob: *mut robj,
    mut defragged: *mut libc::c_long,
) -> *mut robj {
    let mut ret: *mut robj = 0 as *mut robj;
    if (*ob).refcount != 1 as libc::c_int {
        return 0 as *mut robj;
    }
    if (*ob).type_0() as libc::c_int != 0 as libc::c_int
        || (*ob).encoding() as libc::c_int != 8 as libc::c_int
    {
        ret = activeDefragAlloc(ob as *mut libc::c_void) as *mut robj;
        if !ret.is_null() {
            ob = ret;
            *defragged += 1;
        }
    }
    if (*ob).type_0() as libc::c_int == 0 as libc::c_int {
        if (*ob).encoding() as libc::c_int == 0 as libc::c_int {
            let mut newsds: sds = activeDefragSds((*ob).ptr as sds);
            if !newsds.is_null() {
                (*ob).ptr = newsds as *mut libc::c_void;
                *defragged += 1;
            }
        } else if (*ob).encoding() as libc::c_int == 8 as libc::c_int {
            let mut ofs: libc::c_long = (*ob).ptr as intptr_t - ob as intptr_t;
            ret = activeDefragAlloc(ob as *mut libc::c_void) as *mut robj;
            if !ret.is_null() {
                (*ret).ptr = (ret as intptr_t + ofs) as *mut libc::c_void;
                *defragged += 1;
            }
        } else if (*ob).encoding() as libc::c_int != 1 as libc::c_int {
            _serverPanic(
                b"defrag.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                b"Unknown string encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragLuaScript(
    mut script: *mut luaScript,
    mut defragged: *mut libc::c_long,
) -> *mut luaScript {
    let mut ret: *mut luaScript = 0 as *mut luaScript;
    ret = activeDefragAlloc(script as *mut libc::c_void) as *mut luaScript;
    if !ret.is_null() {
        script = ret;
        *defragged += 1;
    }
    let mut ob: *mut robj = activeDefragStringOb((*script).body, defragged);
    if !ob.is_null() {
        (*script).body = ob;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn dictIterDefragEntry(
    mut iter: *mut dictIterator,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    if !((*iter).nextEntry).is_null() {
        let mut newde: *mut dictEntry = activeDefragAlloc(
            (*iter).nextEntry as *mut libc::c_void,
        ) as *mut dictEntry;
        if !newde.is_null() {
            defragged += 1;
            (*iter).nextEntry = newde;
            (*(*iter).entry).next = newde;
        }
    }
    if *((*(*iter).d).ht_table[(*iter).table as usize]).offset((*iter).index as isize)
        == (*iter).entry
    {
        let mut newde_0: *mut dictEntry = activeDefragAlloc(
            (*iter).entry as *mut libc::c_void,
        ) as *mut dictEntry;
        if !newde_0.is_null() {
            (*iter).entry = newde_0;
            let ref mut fresh0 = *((*(*iter).d).ht_table[(*iter).table as usize])
                .offset((*iter).index as isize);
            *fresh0 = newde_0;
            defragged += 1;
        }
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn dictDefragTables(mut d: *mut dict) -> libc::c_long {
    let mut newtable: *mut *mut dictEntry = 0 as *mut *mut dictEntry;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    newtable = activeDefragAlloc(
        (*d).ht_table[0 as libc::c_int as usize] as *mut libc::c_void,
    ) as *mut *mut dictEntry;
    if !newtable.is_null() {
        defragged += 1;
        (*d).ht_table[0 as libc::c_int as usize] = newtable;
    }
    if !((*d).ht_table[1 as libc::c_int as usize]).is_null() {
        newtable = activeDefragAlloc(
            (*d).ht_table[1 as libc::c_int as usize] as *mut libc::c_void,
        ) as *mut *mut dictEntry;
        if !newtable.is_null() {
            defragged += 1;
            (*d).ht_table[1 as libc::c_int as usize] = newtable;
        }
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn zslUpdateNode(
    mut zsl: *mut zskiplist,
    mut oldnode: *mut zskiplistNode,
    mut newnode: *mut zskiplistNode,
    mut update: *mut *mut zskiplistNode,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*zsl).level {
        if (*((**update.offset(i as isize)).level).as_mut_ptr().offset(i as isize))
            .forward == oldnode
        {
            let ref mut fresh1 = (*((**update.offset(i as isize)).level)
                .as_mut_ptr()
                .offset(i as isize))
                .forward;
            *fresh1 = newnode;
        }
        i += 1;
    }
    if (*zsl).header != oldnode {} else {
        _serverAssert(
            b"zsl->header!=oldnode\0" as *const u8 as *const libc::c_char,
            b"defrag.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int,
        );
        unreachable!();
    };
    if !((*((*newnode).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward)
        .is_null()
    {
        if (*(*((*newnode).level).as_mut_ptr().offset(0 as libc::c_int as isize))
            .forward)
            .backward == oldnode
        {} else {
            _serverAssert(
                b"newnode->level[0].forward->backward==oldnode\0" as *const u8
                    as *const libc::c_char,
                b"defrag.c\0" as *const u8 as *const libc::c_char,
                209 as libc::c_int,
            );
            unreachable!();
        };
        let ref mut fresh2 = (*(*((*newnode).level)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .forward)
            .backward;
        *fresh2 = newnode;
    } else {
        if (*zsl).tail == oldnode {} else {
            _serverAssert(
                b"zsl->tail==oldnode\0" as *const u8 as *const libc::c_char,
                b"defrag.c\0" as *const u8 as *const libc::c_char,
                212 as libc::c_int,
            );
            unreachable!();
        };
        (*zsl).tail = newnode;
    };
}
#[no_mangle]
pub unsafe extern "C" fn zslDefrag(
    mut zsl: *mut zskiplist,
    mut score: libc::c_double,
    mut oldele: sds,
    mut newele: sds,
) -> *mut libc::c_double {
    let mut update: [*mut zskiplistNode; 32] = [0 as *mut zskiplistNode; 32];
    let mut x: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut newx: *mut zskiplistNode = 0 as *mut zskiplistNode;
    let mut i: libc::c_int = 0;
    let mut ele: sds = if !newele.is_null() { newele } else { oldele };
    x = (*zsl).header;
    i = (*zsl).level - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        while !((*((*x).level).as_mut_ptr().offset(i as isize)).forward).is_null()
            && (*(*((*x).level).as_mut_ptr().offset(i as isize)).forward).ele != oldele
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
    if !x.is_null() && score == (*x).score && (*x).ele == oldele {} else {
        _serverAssert(
            b"x && score == x->score && x->ele==oldele\0" as *const u8
                as *const libc::c_char,
            b"defrag.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
        );
        unreachable!();
    };
    if !newele.is_null() {
        (*x).ele = newele;
    }
    newx = activeDefragAlloc(x as *mut libc::c_void) as *mut zskiplistNode;
    if !newx.is_null() {
        zslUpdateNode(zsl, x, newx, update.as_mut_ptr());
        return &mut (*newx).score;
    }
    return 0 as *mut libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragZsetEntry(
    mut zs: *mut zset,
    mut de: *mut dictEntry,
) -> libc::c_long {
    let mut newsds: sds = 0 as *mut libc::c_char;
    let mut newscore: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut sdsele: sds = (*de).key as sds;
    newsds = activeDefragSds(sdsele);
    if !newsds.is_null() {
        defragged += 1;
        (*de).key = newsds as *mut libc::c_void;
    }
    newscore = zslDefrag(
        (*zs).zsl,
        *((*de).v.val as *mut libc::c_double),
        sdsele,
        newsds,
    );
    if !newscore.is_null() {
        if ((*(*(*zs).dict).type_0).valDup).is_some() {
            (*de)
                .v
                .val = ((*(*(*zs).dict).type_0).valDup)
                .expect(
                    "non-null function pointer",
                )((*zs).dict, newscore as *const libc::c_void);
        } else {
            (*de).v.val = newscore as *mut libc::c_void;
        }
        defragged += 1;
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragSdsDict(
    mut d: *mut dict,
    mut val_type: libc::c_int,
) -> libc::c_long {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    di = dictGetIterator(d);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut sdsele: sds = (*de).key as sds;
        let mut newsds: sds = 0 as *mut libc::c_char;
        newsds = activeDefragSds(sdsele);
        if !newsds.is_null() {
            (*de).key = newsds as *mut libc::c_void;
            defragged += 1;
        }
        if val_type == 1 as libc::c_int {
            sdsele = (*de).v.val as sds;
            newsds = activeDefragSds(sdsele);
            if !newsds.is_null() {
                (*de).v.val = newsds as *mut libc::c_void;
                defragged += 1;
            }
        } else if val_type == 2 as libc::c_int {
            let mut newele: *mut robj = 0 as *mut robj;
            let mut ele: *mut robj = (*de).v.val as *mut robj;
            newele = activeDefragStringOb(ele, &mut defragged);
            if !newele.is_null() {
                (*de).v.val = newele as *mut libc::c_void;
            }
        } else if val_type == 3 as libc::c_int {
            let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut ptr: *mut libc::c_void = (*de).v.val;
            newptr = activeDefragAlloc(ptr);
            if !newptr.is_null() {
                (*de).v.val = newptr;
                defragged += 1;
            }
        } else if val_type == 4 as libc::c_int {
            let mut newptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut ptr_0: *mut libc::c_void = (*de).v.val;
            newptr_0 = activeDefragLuaScript(ptr_0 as *mut luaScript, &mut defragged)
                as *mut libc::c_void;
            if !newptr_0.is_null() {
                (*de).v.val = newptr_0;
            }
        }
        defragged += dictIterDefragEntry(di);
    }
    dictReleaseIterator(di);
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragList(
    mut l: *mut list,
    mut val_type: libc::c_int,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut newln: *mut listNode = 0 as *mut listNode;
    ln = (*l).head;
    while !ln.is_null() {
        newln = activeDefragAlloc(ln as *mut libc::c_void) as *mut listNode;
        if !newln.is_null() {
            if !((*newln).prev).is_null() {
                (*(*newln).prev).next = newln;
            } else {
                (*l).head = newln;
            }
            if !((*newln).next).is_null() {
                (*(*newln).next).prev = newln;
            } else {
                (*l).tail = newln;
            }
            ln = newln;
            defragged += 1;
        }
        if val_type == 1 as libc::c_int {
            let mut newsds: sds = 0 as *mut libc::c_char;
            let mut sdsele: sds = (*ln).value as sds;
            newsds = activeDefragSds(sdsele);
            if !newsds.is_null() {
                (*ln).value = newsds as *mut libc::c_void;
                defragged += 1;
            }
        } else if val_type == 2 as libc::c_int {
            let mut newele: *mut robj = 0 as *mut robj;
            let mut ele: *mut robj = (*ln).value as *mut robj;
            newele = activeDefragStringOb(ele, &mut defragged);
            if !newele.is_null() {
                (*ln).value = newele as *mut libc::c_void;
            }
        } else if val_type == 3 as libc::c_int {
            let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut ptr: *mut libc::c_void = (*ln).value;
            newptr = activeDefragAlloc(ptr);
            if !newptr.is_null() {
                (*ln).value = newptr;
                defragged += 1;
            }
        }
        ln = (*ln).next;
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragSdsListAndDict(
    mut l: *mut list,
    mut d: *mut dict,
    mut dict_val_type: libc::c_int,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut newsds: sds = 0 as *mut libc::c_char;
    let mut sdsele: sds = 0 as *mut libc::c_char;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut newln: *mut listNode = 0 as *mut listNode;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    ln = (*l).head;
    while !ln.is_null() {
        newln = activeDefragAlloc(ln as *mut libc::c_void) as *mut listNode;
        if !newln.is_null() {
            if !((*newln).prev).is_null() {
                (*(*newln).prev).next = newln;
            } else {
                (*l).head = newln;
            }
            if !((*newln).next).is_null() {
                (*(*newln).next).prev = newln;
            } else {
                (*l).tail = newln;
            }
            ln = newln;
            defragged += 1;
        }
        sdsele = (*ln).value as sds;
        newsds = activeDefragSds(sdsele);
        if !newsds.is_null() {
            let mut hash: uint64_t = dictGetHash(d, newsds as *const libc::c_void);
            let mut deref: *mut *mut dictEntry = dictFindEntryRefByPtrAndHash(
                d,
                sdsele as *const libc::c_void,
                hash,
            );
            if !deref.is_null() {
                (**deref).key = newsds as *mut libc::c_void;
            }
            (*ln).value = newsds as *mut libc::c_void;
            defragged += 1;
        }
        ln = (*ln).next;
    }
    di = dictGetIterator(d);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        if dict_val_type == 1 as libc::c_int {
            let mut newsds_0: sds = 0 as *mut libc::c_char;
            let mut sdsele_0: sds = (*de).v.val as sds;
            newsds_0 = activeDefragSds(sdsele_0);
            if !newsds_0.is_null() {
                (*de).v.val = newsds_0 as *mut libc::c_void;
                defragged += 1;
            }
        } else if dict_val_type == 2 as libc::c_int {
            let mut newele: *mut robj = 0 as *mut robj;
            let mut ele: *mut robj = (*de).v.val as *mut robj;
            newele = activeDefragStringOb(ele, &mut defragged);
            if !newele.is_null() {
                (*de).v.val = newele as *mut libc::c_void;
            }
        } else if dict_val_type == 3 as libc::c_int {
            let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut ptr: *mut libc::c_void = (*de).v.val;
            newptr = activeDefragAlloc(ptr);
            if !newptr.is_null() {
                (*de).v.val = newptr;
                defragged += 1;
            }
        }
        defragged += dictIterDefragEntry(di);
    }
    dictReleaseIterator(di);
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn replaceSatelliteDictKeyPtrAndOrDefragDictEntry(
    mut d: *mut dict,
    mut oldkey: sds,
    mut newkey: sds,
    mut hash: uint64_t,
    mut defragged: *mut libc::c_long,
) -> *mut dictEntry {
    let mut deref: *mut *mut dictEntry = dictFindEntryRefByPtrAndHash(
        d,
        oldkey as *const libc::c_void,
        hash,
    );
    if !deref.is_null() {
        let mut de: *mut dictEntry = *deref;
        let mut newde: *mut dictEntry = activeDefragAlloc(de as *mut libc::c_void)
            as *mut dictEntry;
        if !newde.is_null() {
            *deref = newde;
            de = *deref;
            *defragged += 1;
        }
        if !newkey.is_null() {
            (*de).key = newkey as *mut libc::c_void;
        }
        return de;
    }
    return 0 as *mut dictEntry;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragQuickListNode(
    mut ql: *mut quicklist,
    mut node_ref: *mut *mut quicklistNode,
) -> libc::c_long {
    let mut newnode: *mut quicklistNode = 0 as *mut quicklistNode;
    let mut node: *mut quicklistNode = *node_ref;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut newzl: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    newnode = activeDefragAlloc(node as *mut libc::c_void) as *mut quicklistNode;
    if !newnode.is_null() {
        if !((*newnode).prev).is_null() {
            (*(*newnode).prev).next = newnode;
        } else {
            (*ql).head = newnode;
        }
        if !((*newnode).next).is_null() {
            (*(*newnode).next).prev = newnode;
        } else {
            (*ql).tail = newnode;
        }
        node = newnode;
        *node_ref = node;
        defragged += 1;
    }
    newzl = activeDefragAlloc((*node).entry as *mut libc::c_void) as *mut libc::c_uchar;
    if !newzl.is_null() {
        defragged += 1;
        (*node).entry = newzl;
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragQuickListNodes(
    mut ql: *mut quicklist,
) -> libc::c_long {
    let mut node: *mut quicklistNode = (*ql).head;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    while !node.is_null() {
        defragged += activeDefragQuickListNode(ql, &mut node);
        node = (*node).next;
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragLater(mut db: *mut redisDb, mut kde: *mut dictEntry) {
    let mut key: sds = sdsdup((*kde).key as sds);
    listAddNodeTail((*db).defrag_later, key as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn scanLaterList(
    mut ob: *mut robj,
    mut cursor: *mut libc::c_ulong,
    mut endtime: libc::c_longlong,
    mut defragged: *mut libc::c_longlong,
) -> libc::c_long {
    let mut ql: *mut quicklist = (*ob).ptr as *mut quicklist;
    let mut node: *mut quicklistNode = 0 as *mut quicklistNode;
    let mut iterations: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut bookmark_failed: libc::c_int = 0 as libc::c_int;
    if (*ob).type_0() as libc::c_int != 1 as libc::c_int
        || (*ob).encoding() as libc::c_int != 9 as libc::c_int
    {
        return 0 as libc::c_int as libc::c_long;
    }
    if *cursor == 0 as libc::c_int as libc::c_ulong {
        node = (*ql).head;
    } else {
        node = quicklistBookmarkFind(ql, b"_AD\0" as *const u8 as *const libc::c_char);
        if node.is_null() {
            *cursor = 0 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int as libc::c_long;
        }
        node = (*node).next;
    }
    *cursor = (*cursor).wrapping_add(1);
    while !node.is_null() {
        *defragged += activeDefragQuickListNode(ql, &mut node) as libc::c_longlong;
        server.stat_active_defrag_scanned += 1;
        iterations += 1;
        if iterations > 128 as libc::c_int as libc::c_long && bookmark_failed == 0 {
            if ustime() > endtime {
                if quicklistBookmarkCreate(
                    &mut ql,
                    b"_AD\0" as *const u8 as *const libc::c_char,
                    node,
                ) == 0
                {
                    bookmark_failed = 1 as libc::c_int;
                } else {
                    (*ob).ptr = ql as *mut libc::c_void;
                    return 1 as libc::c_int as libc::c_long;
                }
            }
            iterations = 0 as libc::c_int as libc::c_long;
        }
        node = (*node).next;
    }
    quicklistBookmarkDelete(ql, b"_AD\0" as *const u8 as *const libc::c_char);
    *cursor = 0 as libc::c_int as libc::c_ulong;
    return (if bookmark_failed != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
        as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn scanLaterZsetCallback(
    mut privdata: *mut libc::c_void,
    mut _de: *const dictEntry,
) {
    let mut de: *mut dictEntry = _de as *mut dictEntry;
    let mut data: *mut scanLaterZsetData = privdata as *mut scanLaterZsetData;
    (*data).defragged += activeDefragZsetEntry((*data).zs, de);
    server.stat_active_defrag_scanned += 1;
}
#[no_mangle]
pub unsafe extern "C" fn scanLaterZset(
    mut ob: *mut robj,
    mut cursor: *mut libc::c_ulong,
) -> libc::c_long {
    if (*ob).type_0() as libc::c_int != 3 as libc::c_int
        || (*ob).encoding() as libc::c_int != 7 as libc::c_int
    {
        return 0 as libc::c_int as libc::c_long;
    }
    let mut zs: *mut zset = (*ob).ptr as *mut zset;
    let mut d: *mut dict = (*zs).dict;
    let mut data: scanLaterZsetData = {
        let mut init = scanLaterZsetData {
            zs: zs,
            defragged: 0 as libc::c_int as libc::c_long,
        };
        init
    };
    *cursor = dictScan(
        d,
        *cursor,
        Some(
            scanLaterZsetCallback
                as unsafe extern "C" fn(*mut libc::c_void, *const dictEntry) -> (),
        ),
        Some(
            defragDictBucketCallback
                as unsafe extern "C" fn(*mut dict, *mut *mut dictEntry) -> (),
        ),
        &mut data as *mut scanLaterZsetData as *mut libc::c_void,
    );
    return data.defragged;
}
#[no_mangle]
pub unsafe extern "C" fn scanLaterSetCallback(
    mut privdata: *mut libc::c_void,
    mut _de: *const dictEntry,
) {
    let mut de: *mut dictEntry = _de as *mut dictEntry;
    let mut defragged: *mut libc::c_long = privdata as *mut libc::c_long;
    let mut sdsele: sds = (*de).key as sds;
    let mut newsds: sds = 0 as *mut libc::c_char;
    newsds = activeDefragSds(sdsele);
    if !newsds.is_null() {
        *defragged += 1;
        (*de).key = newsds as *mut libc::c_void;
    }
    server.stat_active_defrag_scanned += 1;
}
#[no_mangle]
pub unsafe extern "C" fn scanLaterSet(
    mut ob: *mut robj,
    mut cursor: *mut libc::c_ulong,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    if (*ob).type_0() as libc::c_int != 2 as libc::c_int
        || (*ob).encoding() as libc::c_int != 2 as libc::c_int
    {
        return 0 as libc::c_int as libc::c_long;
    }
    let mut d: *mut dict = (*ob).ptr as *mut dict;
    *cursor = dictScan(
        d,
        *cursor,
        Some(
            scanLaterSetCallback
                as unsafe extern "C" fn(*mut libc::c_void, *const dictEntry) -> (),
        ),
        Some(
            defragDictBucketCallback
                as unsafe extern "C" fn(*mut dict, *mut *mut dictEntry) -> (),
        ),
        &mut defragged as *mut libc::c_long as *mut libc::c_void,
    );
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn scanLaterHashCallback(
    mut privdata: *mut libc::c_void,
    mut _de: *const dictEntry,
) {
    let mut de: *mut dictEntry = _de as *mut dictEntry;
    let mut defragged: *mut libc::c_long = privdata as *mut libc::c_long;
    let mut sdsele: sds = (*de).key as sds;
    let mut newsds: sds = 0 as *mut libc::c_char;
    newsds = activeDefragSds(sdsele);
    if !newsds.is_null() {
        *defragged += 1;
        (*de).key = newsds as *mut libc::c_void;
    }
    sdsele = (*de).v.val as sds;
    newsds = activeDefragSds(sdsele);
    if !newsds.is_null() {
        *defragged += 1;
        (*de).v.val = newsds as *mut libc::c_void;
    }
    server.stat_active_defrag_scanned += 1;
}
#[no_mangle]
pub unsafe extern "C" fn scanLaterHash(
    mut ob: *mut robj,
    mut cursor: *mut libc::c_ulong,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    if (*ob).type_0() as libc::c_int != 4 as libc::c_int
        || (*ob).encoding() as libc::c_int != 2 as libc::c_int
    {
        return 0 as libc::c_int as libc::c_long;
    }
    let mut d: *mut dict = (*ob).ptr as *mut dict;
    *cursor = dictScan(
        d,
        *cursor,
        Some(
            scanLaterHashCallback
                as unsafe extern "C" fn(*mut libc::c_void, *const dictEntry) -> (),
        ),
        Some(
            defragDictBucketCallback
                as unsafe extern "C" fn(*mut dict, *mut *mut dictEntry) -> (),
        ),
        &mut defragged as *mut libc::c_long as *mut libc::c_void,
    );
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragQuicklist(
    mut db: *mut redisDb,
    mut kde: *mut dictEntry,
) -> libc::c_long {
    let mut ob: *mut robj = (*kde).v.val as *mut robj;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut ql: *mut quicklist = (*ob).ptr as *mut quicklist;
    let mut newql: *mut quicklist = 0 as *mut quicklist;
    if (*ob).type_0() as libc::c_int == 1 as libc::c_int
        && (*ob).encoding() as libc::c_int == 9 as libc::c_int
    {} else {
        _serverAssert(
            b"ob->type == OBJ_LIST && ob->encoding == OBJ_ENCODING_QUICKLIST\0"
                as *const u8 as *const libc::c_char,
            b"defrag.c\0" as *const u8 as *const libc::c_char,
            577 as libc::c_int,
        );
        unreachable!();
    };
    newql = activeDefragAlloc(ql as *mut libc::c_void) as *mut quicklist;
    if !newql.is_null() {
        defragged += 1;
        ql = newql;
        (*ob).ptr = ql as *mut libc::c_void;
    }
    if (*ql).len > server.active_defrag_max_scan_fields {
        defragLater(db, kde);
    } else {
        defragged += activeDefragQuickListNodes(ql);
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragZsetSkiplist(
    mut db: *mut redisDb,
    mut kde: *mut dictEntry,
) -> libc::c_long {
    let mut ob: *mut robj = (*kde).v.val as *mut robj;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut zs: *mut zset = (*ob).ptr as *mut zset;
    let mut newzs: *mut zset = 0 as *mut zset;
    let mut newzsl: *mut zskiplist = 0 as *mut zskiplist;
    let mut newdict: *mut dict = 0 as *mut dict;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut newheader: *mut zskiplistNode = 0 as *mut zskiplistNode;
    if (*ob).type_0() as libc::c_int == 3 as libc::c_int
        && (*ob).encoding() as libc::c_int == 7 as libc::c_int
    {} else {
        _serverAssert(
            b"ob->type == OBJ_ZSET && ob->encoding == OBJ_ENCODING_SKIPLIST\0"
                as *const u8 as *const libc::c_char,
            b"defrag.c\0" as *const u8 as *const libc::c_char,
            596 as libc::c_int,
        );
        unreachable!();
    };
    newzs = activeDefragAlloc(zs as *mut libc::c_void) as *mut zset;
    if !newzs.is_null() {
        defragged += 1;
        zs = newzs;
        (*ob).ptr = zs as *mut libc::c_void;
    }
    newzsl = activeDefragAlloc((*zs).zsl as *mut libc::c_void) as *mut zskiplist;
    if !newzsl.is_null() {
        defragged += 1;
        (*zs).zsl = newzsl;
    }
    newheader = activeDefragAlloc((*(*zs).zsl).header as *mut libc::c_void)
        as *mut zskiplistNode;
    if !newheader.is_null() {
        defragged += 1;
        (*(*zs).zsl).header = newheader;
    }
    if ((*(*zs).dict).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*zs).dict).ht_used[1 as libc::c_int as usize])
        > server.active_defrag_max_scan_fields
    {
        defragLater(db, kde);
    } else {
        let mut di: *mut dictIterator = dictGetIterator((*zs).dict);
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            defragged += activeDefragZsetEntry(zs, de);
        }
        dictReleaseIterator(di);
    }
    newdict = activeDefragAlloc((*zs).dict as *mut libc::c_void) as *mut dict;
    if !newdict.is_null() {
        defragged += 1;
        (*zs).dict = newdict;
    }
    defragged += dictDefragTables((*zs).dict);
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragHash(
    mut db: *mut redisDb,
    mut kde: *mut dictEntry,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut ob: *mut robj = (*kde).v.val as *mut robj;
    let mut d: *mut dict = 0 as *mut dict;
    let mut newd: *mut dict = 0 as *mut dict;
    if (*ob).type_0() as libc::c_int == 4 as libc::c_int
        && (*ob).encoding() as libc::c_int == 2 as libc::c_int
    {} else {
        _serverAssert(
            b"ob->type == OBJ_HASH && ob->encoding == OBJ_ENCODING_HT\0" as *const u8
                as *const libc::c_char,
            b"defrag.c\0" as *const u8 as *const libc::c_char,
            624 as libc::c_int,
        );
        unreachable!();
    };
    d = (*ob).ptr as *mut dict;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
        > server.active_defrag_max_scan_fields
    {
        defragLater(db, kde);
    } else {
        defragged += activeDefragSdsDict(d, 1 as libc::c_int);
    }
    newd = activeDefragAlloc((*ob).ptr) as *mut dict;
    if !newd.is_null() {
        defragged += 1;
        (*ob).ptr = newd as *mut libc::c_void;
    }
    defragged += dictDefragTables((*ob).ptr as *mut dict);
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragSet(
    mut db: *mut redisDb,
    mut kde: *mut dictEntry,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut ob: *mut robj = (*kde).v.val as *mut robj;
    let mut d: *mut dict = 0 as *mut dict;
    let mut newd: *mut dict = 0 as *mut dict;
    if (*ob).type_0() as libc::c_int == 2 as libc::c_int
        && (*ob).encoding() as libc::c_int == 2 as libc::c_int
    {} else {
        _serverAssert(
            b"ob->type == OBJ_SET && ob->encoding == OBJ_ENCODING_HT\0" as *const u8
                as *const libc::c_char,
            b"defrag.c\0" as *const u8 as *const libc::c_char,
            642 as libc::c_int,
        );
        unreachable!();
    };
    d = (*ob).ptr as *mut dict;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
        > server.active_defrag_max_scan_fields
    {
        defragLater(db, kde);
    } else {
        defragged += activeDefragSdsDict(d, 0 as libc::c_int);
    }
    newd = activeDefragAlloc((*ob).ptr) as *mut dict;
    if !newd.is_null() {
        defragged += 1;
        (*ob).ptr = newd as *mut libc::c_void;
    }
    defragged += dictDefragTables((*ob).ptr as *mut dict);
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragRaxNode(mut noderef: *mut *mut raxNode) -> libc::c_int {
    let mut newnode: *mut raxNode = activeDefragAlloc(*noderef as *mut libc::c_void)
        as *mut raxNode;
    if !newnode.is_null() {
        *noderef = newnode;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scanLaterStreamListpacks(
    mut ob: *mut robj,
    mut cursor: *mut libc::c_ulong,
    mut endtime: libc::c_longlong,
    mut defragged: *mut libc::c_longlong,
) -> libc::c_int {
    static mut last: [libc::c_uchar; 16] = [0; 16];
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
    let mut iterations: libc::c_long = 0 as libc::c_int as libc::c_long;
    if (*ob).type_0() as libc::c_int != 6 as libc::c_int
        || (*ob).encoding() as libc::c_int != 10 as libc::c_int
    {
        *cursor = 0 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    let mut s: *mut stream = (*ob).ptr as *mut stream;
    raxStart(&mut ri, (*s).rax);
    if *cursor == 0 as libc::c_int as libc::c_ulong {
        defragRaxNode(&mut (*(*s).rax).head);
        ri
            .node_cb = Some(
            defragRaxNode as unsafe extern "C" fn(*mut *mut raxNode) -> libc::c_int,
        );
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
    } else {
        if raxSeek(
            &mut ri,
            b">\0" as *const u8 as *const libc::c_char,
            last.as_mut_ptr(),
            core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
        ) == 0
        {
            *cursor = 0 as libc::c_int as libc::c_ulong;
            raxStop(&mut ri);
            return 0 as libc::c_int;
        }
        ri
            .node_cb = Some(
            defragRaxNode as unsafe extern "C" fn(*mut *mut raxNode) -> libc::c_int,
        );
    }
    *cursor = (*cursor).wrapping_add(1);
    while raxNext(&mut ri) != 0 {
        let mut newdata: *mut libc::c_void = activeDefragAlloc(ri.data);
        if !newdata.is_null() {
            ri.data = newdata;
            raxSetData(ri.node, ri.data);
            *defragged += 1;
        }
        server.stat_active_defrag_scanned += 1;
        iterations += 1;
        if iterations > 128 as libc::c_int as libc::c_long {
            if ustime() > endtime {
                if ri.key_len
                    == core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong
                {} else {
                    _serverAssert(
                        b"ri.key_len==sizeof(last)\0" as *const u8
                            as *const libc::c_char,
                        b"defrag.c\0" as *const u8 as *const libc::c_char,
                        706 as libc::c_int,
                    );
                    unreachable!();
                };
                memcpy(
                    last.as_mut_ptr() as *mut libc::c_void,
                    ri.key as *const libc::c_void,
                    ri.key_len,
                );
                raxStop(&mut ri);
                return 1 as libc::c_int;
            }
            iterations = 0 as libc::c_int as libc::c_long;
        }
    }
    raxStop(&mut ri);
    *cursor = 0 as libc::c_int as libc::c_ulong;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn defragRadixTree(
    mut raxref: *mut *mut rax,
    mut defrag_data: libc::c_int,
    mut element_cb: Option::<raxDefragFunction>,
    mut element_cb_data: *mut libc::c_void,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
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
    let mut rax: *mut rax = 0 as *mut rax;
    rax = activeDefragAlloc(*raxref as *mut libc::c_void) as *mut rax;
    if !rax.is_null() {
        defragged += 1;
        *raxref = rax;
    }
    rax = *raxref;
    raxStart(&mut ri, rax);
    ri
        .node_cb = Some(
        defragRaxNode as unsafe extern "C" fn(*mut *mut raxNode) -> libc::c_int,
    );
    defragRaxNode(&mut (*rax).head);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        let mut newdata: *mut libc::c_void = 0 as *mut libc::c_void;
        if element_cb.is_some() {
            newdata = element_cb
                .expect(
                    "non-null function pointer",
                )(&mut ri, element_cb_data, &mut defragged);
        }
        if defrag_data != 0 && newdata.is_null() {
            newdata = activeDefragAlloc(ri.data);
        }
        if !newdata.is_null() {
            ri.data = newdata;
            raxSetData(ri.node, ri.data);
            defragged += 1;
        }
    }
    raxStop(&mut ri);
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragStreamConsumerPendingEntry(
    mut ri: *mut raxIterator,
    mut privdata: *mut libc::c_void,
    mut defragged: *mut libc::c_long,
) -> *mut libc::c_void {
    let mut ctx: *mut PendingEntryContext = privdata as *mut PendingEntryContext;
    let mut nack: *mut streamNACK = (*ri).data as *mut streamNACK;
    let mut newnack: *mut streamNACK = 0 as *mut streamNACK;
    (*nack).consumer = (*ctx).c;
    newnack = activeDefragAlloc(nack as *mut libc::c_void) as *mut streamNACK;
    if !newnack.is_null() {
        let mut prev: *mut libc::c_void = 0 as *mut libc::c_void;
        raxInsert(
            (*(*ctx).cg).pel,
            (*ri).key,
            (*ri).key_len,
            newnack as *mut libc::c_void,
            &mut prev,
        );
        if prev == nack as *mut libc::c_void {} else {
            _serverAssert(
                b"prev==nack\0" as *const u8 as *const libc::c_char,
                b"defrag.c\0" as *const u8 as *const libc::c_char,
                766 as libc::c_int,
            );
            unreachable!();
        };
    }
    return newnack as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn defragStreamConsumer(
    mut ri: *mut raxIterator,
    mut privdata: *mut libc::c_void,
    mut defragged: *mut libc::c_long,
) -> *mut libc::c_void {
    let mut c: *mut streamConsumer = (*ri).data as *mut streamConsumer;
    let mut cg: *mut streamCG = privdata as *mut streamCG;
    let mut newc: *mut libc::c_void = activeDefragAlloc(c as *mut libc::c_void);
    if !newc.is_null() {
        c = newc as *mut streamConsumer;
    }
    let mut newsds: sds = activeDefragSds((*c).name);
    if !newsds.is_null() {
        *defragged += 1;
        (*c).name = newsds;
    }
    if !((*c).pel).is_null() {
        let mut pel_ctx: PendingEntryContext = {
            let mut init = PendingEntryContext {
                cg: cg,
                c: c,
            };
            init
        };
        *defragged
            += defragRadixTree(
                &mut (*c).pel,
                0 as libc::c_int,
                Some(
                    defragStreamConsumerPendingEntry
                        as unsafe extern "C" fn(
                            *mut raxIterator,
                            *mut libc::c_void,
                            *mut libc::c_long,
                        ) -> *mut libc::c_void,
                ),
                &mut pel_ctx as *mut PendingEntryContext as *mut libc::c_void,
            );
    }
    return newc;
}
#[no_mangle]
pub unsafe extern "C" fn defragStreamConsumerGroup(
    mut ri: *mut raxIterator,
    mut privdata: *mut libc::c_void,
    mut defragged: *mut libc::c_long,
) -> *mut libc::c_void {
    let mut cg: *mut streamCG = (*ri).data as *mut streamCG;
    if !((*cg).consumers).is_null() {
        *defragged
            += defragRadixTree(
                &mut (*cg).consumers,
                0 as libc::c_int,
                Some(
                    defragStreamConsumer
                        as unsafe extern "C" fn(
                            *mut raxIterator,
                            *mut libc::c_void,
                            *mut libc::c_long,
                        ) -> *mut libc::c_void,
                ),
                cg as *mut libc::c_void,
            );
    }
    if !((*cg).pel).is_null() {
        *defragged
            += defragRadixTree(
                &mut (*cg).pel,
                0 as libc::c_int,
                None,
                0 as *mut libc::c_void,
            );
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn defragStream(
    mut db: *mut redisDb,
    mut kde: *mut dictEntry,
) -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut ob: *mut robj = (*kde).v.val as *mut robj;
    if (*ob).type_0() as libc::c_int == 6 as libc::c_int
        && (*ob).encoding() as libc::c_int == 10 as libc::c_int
    {} else {
        _serverAssert(
            b"ob->type == OBJ_STREAM && ob->encoding == OBJ_ENCODING_STREAM\0"
                as *const u8 as *const libc::c_char,
            b"defrag.c\0" as *const u8 as *const libc::c_char,
            803 as libc::c_int,
        );
        unreachable!();
    };
    let mut s: *mut stream = (*ob).ptr as *mut stream;
    let mut news: *mut stream = 0 as *mut stream;
    news = activeDefragAlloc(s as *mut libc::c_void) as *mut stream;
    if !news.is_null() {
        defragged += 1;
        s = news;
        (*ob).ptr = s as *mut libc::c_void;
    }
    if raxSize((*s).rax) > server.active_defrag_max_scan_fields {
        let mut newrax: *mut rax = activeDefragAlloc((*s).rax as *mut libc::c_void)
            as *mut rax;
        if !newrax.is_null() {
            defragged += 1;
            (*s).rax = newrax;
        }
        defragLater(db, kde);
    } else {
        defragged
            += defragRadixTree(
                &mut (*s).rax,
                1 as libc::c_int,
                None,
                0 as *mut libc::c_void,
            );
    }
    if !((*s).cgroups).is_null() {
        defragged
            += defragRadixTree(
                &mut (*s).cgroups,
                1 as libc::c_int,
                Some(
                    defragStreamConsumerGroup
                        as unsafe extern "C" fn(
                            *mut raxIterator,
                            *mut libc::c_void,
                            *mut libc::c_long,
                        ) -> *mut libc::c_void,
                ),
                0 as *mut libc::c_void,
            );
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragModule(
    mut db: *mut redisDb,
    mut kde: *mut dictEntry,
) -> libc::c_long {
    let mut obj: *mut robj = (*kde).v.val as *mut robj;
    if (*obj).type_0() as libc::c_int == 5 as libc::c_int {} else {
        _serverAssert(
            b"obj->type == OBJ_MODULE\0" as *const u8 as *const libc::c_char,
            b"defrag.c\0" as *const u8 as *const libc::c_char,
            828 as libc::c_int,
        );
        unreachable!();
    };
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    if moduleDefragValue((*kde).key as *mut robj, obj, &mut defragged, (*db).id) == 0 {
        defragLater(db, kde);
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragKey(
    mut db: *mut redisDb,
    mut de: *mut dictEntry,
) -> libc::c_long {
    let mut keysds: sds = (*de).key as sds;
    let mut newob: *mut robj = 0 as *mut robj;
    let mut ob: *mut robj = 0 as *mut robj;
    let mut newzl: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut newsds: sds = 0 as *mut libc::c_char;
    newsds = activeDefragSds(keysds);
    if !newsds.is_null() {
        defragged += 1;
        (*de).key = newsds as *mut libc::c_void;
    }
    if ((*(*db).expires).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*db).expires).ht_used[1 as libc::c_int as usize]) != 0
    {
        let mut hash: uint64_t = dictGetHash((*db).dict, (*de).key);
        replaceSatelliteDictKeyPtrAndOrDefragDictEntry(
            (*db).expires,
            keysds,
            newsds,
            hash,
            &mut defragged,
        );
    }
    ob = (*de).v.val as *mut robj;
    newob = activeDefragStringOb(ob, &mut defragged);
    if !newob.is_null() {
        (*de).v.val = newob as *mut libc::c_void;
        ob = newob;
    }
    if !((*ob).type_0() as libc::c_int == 0 as libc::c_int) {
        if (*ob).type_0() as libc::c_int == 1 as libc::c_int {
            if (*ob).encoding() as libc::c_int == 9 as libc::c_int {
                defragged += defragQuicklist(db, de);
            } else {
                _serverPanic(
                    b"defrag.c\0" as *const u8 as *const libc::c_char,
                    872 as libc::c_int,
                    b"Unknown list encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        } else if (*ob).type_0() as libc::c_int == 2 as libc::c_int {
            if (*ob).encoding() as libc::c_int == 2 as libc::c_int {
                defragged += defragSet(db, de);
            } else if (*ob).encoding() as libc::c_int == 6 as libc::c_int {
                let mut newis: *mut intset = 0 as *mut intset;
                let mut is: *mut intset = (*ob).ptr as *mut intset;
                newis = activeDefragAlloc(is as *mut libc::c_void) as *mut intset;
                if !newis.is_null() {
                    defragged += 1;
                    (*ob).ptr = newis as *mut libc::c_void;
                }
            } else {
                _serverPanic(
                    b"defrag.c\0" as *const u8 as *const libc::c_char,
                    882 as libc::c_int,
                    b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        } else if (*ob).type_0() as libc::c_int == 3 as libc::c_int {
            if (*ob).encoding() as libc::c_int == 11 as libc::c_int {
                newzl = activeDefragAlloc((*ob).ptr) as *mut libc::c_uchar;
                if !newzl.is_null() {
                    defragged += 1;
                    (*ob).ptr = newzl as *mut libc::c_void;
                }
            } else if (*ob).encoding() as libc::c_int == 7 as libc::c_int {
                defragged += defragZsetSkiplist(db, de);
            } else {
                _serverPanic(
                    b"defrag.c\0" as *const u8 as *const libc::c_char,
                    891 as libc::c_int,
                    b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        } else if (*ob).type_0() as libc::c_int == 4 as libc::c_int {
            if (*ob).encoding() as libc::c_int == 11 as libc::c_int {
                newzl = activeDefragAlloc((*ob).ptr) as *mut libc::c_uchar;
                if !newzl.is_null() {
                    defragged += 1;
                    (*ob).ptr = newzl as *mut libc::c_void;
                }
            } else if (*ob).encoding() as libc::c_int == 2 as libc::c_int {
                defragged += defragHash(db, de);
            } else {
                _serverPanic(
                    b"defrag.c\0" as *const u8 as *const libc::c_char,
                    900 as libc::c_int,
                    b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        } else if (*ob).type_0() as libc::c_int == 6 as libc::c_int {
            defragged += defragStream(db, de);
        } else if (*ob).type_0() as libc::c_int == 5 as libc::c_int {
            defragged += defragModule(db, de);
        } else {
            _serverPanic(
                b"defrag.c\0" as *const u8 as *const libc::c_char,
                907 as libc::c_int,
                b"Unknown object type\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragScanCallback(
    mut privdata: *mut libc::c_void,
    mut de: *const dictEntry,
) {
    let mut defragged: libc::c_long = defragKey(
        privdata as *mut redisDb,
        de as *mut dictEntry,
    );
    server.stat_active_defrag_hits += defragged as libc::c_longlong;
    if defragged != 0 {
        server.stat_active_defrag_key_hits += 1;
    } else {
        server.stat_active_defrag_key_misses += 1;
    }
    server.stat_active_defrag_scanned += 1;
}
#[no_mangle]
pub unsafe extern "C" fn defragDictBucketCallback(
    mut d: *mut dict,
    mut bucketref: *mut *mut dictEntry,
) {
    while !(*bucketref).is_null() {
        let mut de: *mut dictEntry = *bucketref;
        let mut newde: *mut dictEntry = 0 as *mut dictEntry;
        newde = activeDefragAlloc(de as *mut libc::c_void) as *mut dictEntry;
        if !newde.is_null() {
            *bucketref = newde;
            if server.cluster_enabled != 0
                && d == (*(server.db).offset(0 as libc::c_int as isize)).dict
            {
                slotToKeyReplaceEntry(newde, server.db);
            }
        }
        bucketref = &mut (**bucketref).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getAllocatorFragmentation(
    mut out_frag_bytes: *mut size_t,
) -> libc::c_float {
    let mut resident: size_t = 0;
    let mut active: size_t = 0;
    let mut allocated: size_t = 0;
    zmalloc_get_allocator_info(&mut allocated, &mut active, &mut resident);
    let mut frag_pct: libc::c_float = active as libc::c_float
        / allocated as libc::c_float * 100 as libc::c_int as libc::c_float
        - 100 as libc::c_int as libc::c_float;
    let mut frag_bytes: size_t = active.wrapping_sub(allocated);
    let mut rss_pct: libc::c_float = resident as libc::c_float
        / allocated as libc::c_float * 100 as libc::c_int as libc::c_float
        - 100 as libc::c_int as libc::c_float;
    let mut rss_bytes: size_t = resident.wrapping_sub(allocated);
    if !out_frag_bytes.is_null() {
        *out_frag_bytes = frag_bytes;
    }
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"allocated=%zu, active=%zu, resident=%zu, frag=%.0f%% (%.0f%% rss), frag_bytes=%zu (%zu rss)\0"
                as *const u8 as *const libc::c_char,
            allocated,
            active,
            resident,
            frag_pct as libc::c_double,
            rss_pct as libc::c_double,
            frag_bytes,
            rss_bytes,
        );
    }
    return frag_pct;
}
#[no_mangle]
pub unsafe extern "C" fn defragOtherGlobals() -> libc::c_long {
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    defragged += activeDefragSdsDict(evalScriptsDict(), 4 as libc::c_int);
    defragged += moduleDefragGlobals();
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn defragLaterItem(
    mut de: *mut dictEntry,
    mut cursor: *mut libc::c_ulong,
    mut endtime: libc::c_longlong,
    mut dbid: libc::c_int,
) -> libc::c_int {
    if !de.is_null() {
        let mut ob: *mut robj = (*de).v.val as *mut robj;
        if (*ob).type_0() as libc::c_int == 1 as libc::c_int {
            return scanLaterList(
                ob,
                cursor,
                endtime,
                &mut server.stat_active_defrag_hits,
            ) as libc::c_int
        } else {
            if (*ob).type_0() as libc::c_int == 2 as libc::c_int {
                server.stat_active_defrag_hits
                    += scanLaterSet(ob, cursor) as libc::c_longlong;
            } else if (*ob).type_0() as libc::c_int == 3 as libc::c_int {
                server.stat_active_defrag_hits
                    += scanLaterZset(ob, cursor) as libc::c_longlong;
            } else if (*ob).type_0() as libc::c_int == 4 as libc::c_int {
                server.stat_active_defrag_hits
                    += scanLaterHash(ob, cursor) as libc::c_longlong;
            } else if (*ob).type_0() as libc::c_int == 6 as libc::c_int {
                return scanLaterStreamListpacks(
                    ob,
                    cursor,
                    endtime,
                    &mut server.stat_active_defrag_hits,
                )
            } else {
                if (*ob).type_0() as libc::c_int == 5 as libc::c_int {
                    return moduleLateDefrag(
                        (*de).key as *mut robj,
                        ob,
                        cursor,
                        endtime,
                        &mut server.stat_active_defrag_hits,
                        dbid,
                    )
                } else {
                    *cursor = 0 as libc::c_int as libc::c_ulong;
                }
            }
        }
    } else {
        *cursor = 0 as libc::c_int as libc::c_ulong;
    }
    return 0 as libc::c_int;
}
static mut defrag_later_current_key: sds = 0 as *const libc::c_char as sds;
static mut defrag_later_cursor: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn defragLaterStep(
    mut db: *mut redisDb,
    mut endtime: libc::c_longlong,
) -> libc::c_int {
    let mut iterations: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut prev_defragged: libc::c_ulonglong = server.stat_active_defrag_hits
        as libc::c_ulonglong;
    let mut prev_scanned: libc::c_ulonglong = server.stat_active_defrag_scanned
        as libc::c_ulonglong;
    let mut key_defragged: libc::c_longlong = 0;
    loop {
        if defrag_later_cursor == 0 {
            let mut head: *mut listNode = (*(*db).defrag_later).head;
            if !defrag_later_current_key.is_null() {
                if defrag_later_current_key == (*head).value as sds {} else {
                    _serverAssert(
                        b"defrag_later_current_key == head->value\0" as *const u8
                            as *const libc::c_char,
                        b"defrag.c\0" as *const u8 as *const libc::c_char,
                        1017 as libc::c_int,
                    );
                    unreachable!();
                };
                listDelNode((*db).defrag_later, head);
                defrag_later_cursor = 0 as libc::c_int as libc::c_ulong;
                defrag_later_current_key = 0 as sds;
            }
            head = (*(*db).defrag_later).head;
            if head.is_null() {
                return 0 as libc::c_int;
            }
            defrag_later_current_key = (*head).value as sds;
            defrag_later_cursor = 0 as libc::c_int as libc::c_ulong;
        }
        let mut de: *mut dictEntry = dictFind(
            (*db).dict,
            defrag_later_current_key as *const libc::c_void,
        );
        key_defragged = server.stat_active_defrag_hits;
        loop {
            let mut quit: libc::c_int = 0 as libc::c_int;
            if defragLaterItem(de, &mut defrag_later_cursor, endtime, (*db).id) != 0 {
                quit = 1 as libc::c_int;
            }
            if quit != 0
                || {
                    iterations = iterations.wrapping_add(1);
                    iterations > 16 as libc::c_int as libc::c_uint
                        || (server.stat_active_defrag_hits as libc::c_ulonglong)
                            .wrapping_sub(prev_defragged)
                            > 512 as libc::c_int as libc::c_ulonglong
                        || (server.stat_active_defrag_scanned as libc::c_ulonglong)
                            .wrapping_sub(prev_scanned)
                            > 64 as libc::c_int as libc::c_ulonglong
                }
            {
                if quit != 0 || ustime() > endtime {
                    if key_defragged != server.stat_active_defrag_hits {
                        server.stat_active_defrag_key_hits += 1;
                    } else {
                        server.stat_active_defrag_key_misses += 1;
                    }
                    return 1 as libc::c_int;
                }
                iterations = 0 as libc::c_int as libc::c_uint;
                prev_defragged = server.stat_active_defrag_hits as libc::c_ulonglong;
                prev_scanned = server.stat_active_defrag_scanned as libc::c_ulonglong;
            }
            if !(defrag_later_cursor != 0) {
                break;
            }
        }
        if key_defragged != server.stat_active_defrag_hits {
            server.stat_active_defrag_key_hits += 1;
        } else {
            server.stat_active_defrag_key_misses += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn computeDefragCycles() {
    let mut frag_bytes: size_t = 0;
    let mut frag_pct: libc::c_float = getAllocatorFragmentation(&mut frag_bytes);
    if server.active_defrag_running == 0 {
        if frag_pct < server.active_defrag_threshold_lower as libc::c_float
            || frag_bytes < server.active_defrag_ignore_bytes
        {
            return;
        }
    }
    let mut cpu_pct: libc::c_int = (server.active_defrag_cycle_min as libc::c_float
        + (frag_pct - server.active_defrag_threshold_lower as libc::c_float)
            * (server.active_defrag_cycle_max - server.active_defrag_cycle_min)
                as libc::c_float
            / (server.active_defrag_threshold_upper
                - server.active_defrag_threshold_lower) as libc::c_float) as libc::c_int;
    cpu_pct = if cpu_pct < server.active_defrag_cycle_min {
        server.active_defrag_cycle_min
    } else if cpu_pct > server.active_defrag_cycle_max {
        server.active_defrag_cycle_max
    } else {
        cpu_pct
    };
    if cpu_pct > server.active_defrag_running {
        server.active_defrag_running = cpu_pct;
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Starting active defrag, frag=%.0f%%, frag_bytes=%zu, cpu=%d%%\0"
                    as *const u8 as *const libc::c_char,
                frag_pct as libc::c_double,
                frag_bytes,
                cpu_pct,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn activeDefragCycle() {
    static mut current_db: libc::c_int = -(1 as libc::c_int);
    static mut cursor: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    static mut db: *mut redisDb = 0 as *const redisDb as *mut redisDb;
    static mut start_scan: libc::c_longlong = 0;
    static mut start_stat: libc::c_longlong = 0;
    let mut iterations: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut prev_defragged: libc::c_ulonglong = server.stat_active_defrag_hits
        as libc::c_ulonglong;
    let mut prev_scanned: libc::c_ulonglong = server.stat_active_defrag_scanned
        as libc::c_ulonglong;
    let mut start: libc::c_longlong = 0;
    let mut timelimit: libc::c_longlong = 0;
    let mut endtime: libc::c_longlong = 0;
    let mut latency: mstime_t = 0;
    let mut quit: libc::c_int = 0 as libc::c_int;
    if server.active_defrag_enabled == 0 {
        if server.active_defrag_running != 0 {
            server.active_defrag_running = 0 as libc::c_int;
            if !db.is_null() {
                listEmpty((*db).defrag_later);
            }
            defrag_later_current_key = 0 as sds;
            defrag_later_cursor = 0 as libc::c_int as libc::c_ulong;
            current_db = -(1 as libc::c_int);
            cursor = 0 as libc::c_int as libc::c_ulong;
            db = 0 as *mut redisDb;
        } else {
            return
        }
    } else {
        if hasActiveChildProcess() != 0 {
            return;
        }
        if 1000 as libc::c_int <= 1000 as libc::c_int / server.hz
            || server.cronloops
                % (1000 as libc::c_int / (1000 as libc::c_int / server.hz)) == 0
        {
            computeDefragCycles();
        }
        if server.active_defrag_running == 0 {
            return;
        }
        start = ustime();
        timelimit = (1000000 as libc::c_int * server.active_defrag_running / server.hz
            / 100 as libc::c_int) as libc::c_longlong;
        if timelimit <= 0 as libc::c_int as libc::c_longlong {
            timelimit = 1 as libc::c_int as libc::c_longlong;
        }
        endtime = start + timelimit;
        if server.latency_monitor_threshold != 0 {
            latency = mstime();
        } else {
            latency = 0 as libc::c_int as mstime_t;
        }
        let mut current_block_49: u64;
        loop {
            if cursor == 0 {
                if !db.is_null() && defragLaterStep(db, endtime) != 0 {
                    quit = 1 as libc::c_int;
                    break;
                } else {
                    current_db += 1;
                    if current_db >= server.dbnum {
                        server.stat_active_defrag_hits
                            += defragOtherGlobals() as libc::c_longlong;
                        let mut now: libc::c_longlong = ustime();
                        let mut frag_bytes: size_t = 0;
                        let mut frag_pct: libc::c_float = getAllocatorFragmentation(
                            &mut frag_bytes,
                        );
                        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                1 as libc::c_int,
                                b"Active defrag done in %dms, reallocated=%d, frag=%.0f%%, frag_bytes=%zu\0"
                                    as *const u8 as *const libc::c_char,
                                ((now - start_scan)
                                    / 1000 as libc::c_int as libc::c_longlong) as libc::c_int,
                                (server.stat_active_defrag_hits - start_stat)
                                    as libc::c_int,
                                frag_pct as libc::c_double,
                                frag_bytes,
                            );
                        }
                        start_scan = now;
                        current_db = -(1 as libc::c_int);
                        cursor = 0 as libc::c_int as libc::c_ulong;
                        db = 0 as *mut redisDb;
                        server.active_defrag_running = 0 as libc::c_int;
                        computeDefragCycles();
                        if !(server.active_defrag_running != 0 as libc::c_int
                            && ustime() < endtime)
                        {
                            break;
                        }
                        current_block_49 = 7172762164747879670;
                    } else {
                        if current_db == 0 as libc::c_int {
                            start_scan = ustime();
                            start_stat = server.stat_active_defrag_hits;
                        }
                        db = &mut *(server.db).offset(current_db as isize)
                            as *mut redisDb;
                        cursor = 0 as libc::c_int as libc::c_ulong;
                        current_block_49 = 9441801433784995173;
                    }
                }
            } else {
                current_block_49 = 9441801433784995173;
            }
            match current_block_49 {
                9441801433784995173 => {
                    loop {
                        if defragLaterStep(db, endtime) != 0 {
                            quit = 1 as libc::c_int;
                            break;
                        } else {
                            cursor = dictScan(
                                (*db).dict,
                                cursor,
                                Some(
                                    defragScanCallback
                                        as unsafe extern "C" fn(
                                            *mut libc::c_void,
                                            *const dictEntry,
                                        ) -> (),
                                ),
                                Some(
                                    defragDictBucketCallback
                                        as unsafe extern "C" fn(
                                            *mut dict,
                                            *mut *mut dictEntry,
                                        ) -> (),
                                ),
                                db as *mut libc::c_void,
                            );
                            if cursor == 0
                                || {
                                    iterations = iterations.wrapping_add(1);
                                    iterations > 16 as libc::c_int as libc::c_uint
                                        || (server.stat_active_defrag_hits as libc::c_ulonglong)
                                            .wrapping_sub(prev_defragged)
                                            > 512 as libc::c_int as libc::c_ulonglong
                                        || (server.stat_active_defrag_scanned as libc::c_ulonglong)
                                            .wrapping_sub(prev_scanned)
                                            > 64 as libc::c_int as libc::c_ulonglong
                                }
                            {
                                if cursor == 0 || ustime() > endtime {
                                    quit = 1 as libc::c_int;
                                    break;
                                } else {
                                    iterations = 0 as libc::c_int as libc::c_uint;
                                    prev_defragged = server.stat_active_defrag_hits
                                        as libc::c_ulonglong;
                                    prev_scanned = server.stat_active_defrag_scanned
                                        as libc::c_ulonglong;
                                }
                            }
                            if !(cursor != 0 && quit == 0) {
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
            if !(quit == 0) {
                break;
            }
        }
        if server.latency_monitor_threshold != 0 {
            latency = mstime() - latency;
        }
        if server.latency_monitor_threshold != 0
            && latency >= server.latency_monitor_threshold
        {
            latencyAddSample(
                b"active-defrag-cycle\0" as *const u8 as *const libc::c_char,
                latency,
            );
        }
    }
    if server.active_defrag_running > 0 as libc::c_int {
        if server.stat_last_active_defrag_time == 0 as libc::c_int as libc::c_ulong {
            elapsedStart(&mut server.stat_last_active_defrag_time);
        }
    } else if server.stat_last_active_defrag_time != 0 as libc::c_int as libc::c_ulong {
        server
            .stat_total_active_defrag_time = (server.stat_total_active_defrag_time
            as libc::c_ulonglong)
            .wrapping_add(
                elapsedUs(server.stat_last_active_defrag_time) as libc::c_ulonglong,
            ) as libc::c_longlong as libc::c_longlong;
        server.stat_last_active_defrag_time = 0 as libc::c_int as monotime;
    }
}
